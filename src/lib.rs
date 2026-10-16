mod interaction;
mod response;
mod state;
mod stroke;

pub use interaction::Interactive;
pub use interaction::response::InteractiveResponse;
use std::ops::{Add, Div, Mul, Sub};

pub use response::*;

pub use stroke::*;

// egui::Id is not const compatible
pub(crate) const MARQUEE_STORAGE_NAME: &str = "__marquee::storage__";
pub(crate) const MARQUEE_STACK_TAG_KEY: &str = "marquee_stack";

use crate::state::State;
use egui::{
    Color32, Id, IdMap, PointerButton, Rect, Sense, Stroke, Ui, UiBuilder, UiKind, UiStackInfo,
};

pub struct Marquee {
    id: Id,
    button: PointerButton,
    fill: Color32,
    stroke: MarqueeStroke,
}

// Shapes
impl Marquee {
    pub fn dashed(mut self, dash_length: f32, gap_length: f32) -> Self {
        self.stroke.style = StrokeStyle::Dashed {
            dash_length,
            gap_length,
        };

        self
    }

    pub fn dotted(mut self, radius: f32, spacing: Option<f32>, color: Option<Color32>) -> Self {
        let color = color.unwrap_or(Color32::DEBUG_COLOR);

        self.stroke.style = StrokeStyle::Dotted {
            radius,
            color,
            spacing: spacing.unwrap_or(radius.mul(2.)),
        };

        self
    }
}

impl Marquee {
    pub fn new<I: Into<egui::Id>>(id: I) -> Self {
        Self {
            id: I::into(id),
            fill: Color32::TRANSPARENT,
            button: PointerButton::Primary,
            stroke: MarqueeStroke::default(),
        }
    }

    #[inline]
    pub fn button(mut self, button: PointerButton) -> Self {
        self.button = button;
        self
    }

    #[inline]
    pub fn fill(mut self, fill: Color32) -> Self {
        self.fill = fill;
        self
    }

    #[inline]
    pub fn stroke(mut self, stroke: impl Into<Stroke>) -> Self {
        self.stroke.set_inner_stroke(stroke.into());

        self
    }

    #[inline]
    pub fn stroke_style(mut self, style: StrokeStyle) -> Self {
        self.stroke.style = style;
        self
    }

    pub fn show<R>(
        self,
        ui: &mut Ui,
        add_contents: impl FnOnce(&mut Ui) -> R,
    ) -> MarqueeResponse<R> {
        let const_map_name = Id::unique(MARQUEE_STORAGE_NAME);

        let info =
            UiStackInfo::new(UiKind::GenericArea).with_tag_value(MARQUEE_STACK_TAG_KEY, self.id);

        let inner_response = ui.scope_builder(
            UiBuilder::new()
                .id_salt(self.id)
                .ui_stack_info(info)
                .sense(Sense::drag()),
            add_contents,
        );

        // Short read lock
        let mut current_state = ui.data(|d| {
            let map = d.get_temp::<IdMap<State>>(const_map_name);
            map.map(|m| m.get(&self.id).cloned()).flatten()
        });

        if inner_response.response.drag_stopped()
            || inner_response.response.drag_stopped_by(self.button)
        {
            if let Some(State::Active {
                start,
                end,
                components,
            }) = current_state
            {
                let selection = Rect::from_two_pos(start, end);

                ui.data_mut(|global_state| {
                    let marquee_map =
                        global_state.get_temp_mut_or_default::<IdMap<State>>(const_map_name);

                    // Explicitly shadow the state above
                    let current_state = State::Finished {
                        start,
                        end,
                        components,
                    };

                    marquee_map.insert(self.id, current_state);
                });

                return MarqueeResponse {
                    inner: inner_response.inner,
                    response: inner_response.response,
                    selection: Some(selection),
                };
            }
        }

        let current_pointer_pos = inner_response.response.interact_pointer_pos();

        if inner_response.response.drag_started_by(self.button)
            && let Some(start) = ui
                .input(|i| i.pointer.press_origin())
                .or_else(|| current_pointer_pos)
        {
            current_state = Some(State::Active {
                start,
                end: start,
                components: current_state
                    .map(|s| s.components().cloned())
                    .flatten()
                    .unwrap_or_default(),
            });
            // TODO: Think how to rework this one static areas are introduced
        } else if ui.input(|i| i.pointer.press_origin()).is_none() {
            current_state = Some(Default::default())
        }

        if let Some(State::Active { start, end, .. }) = &mut current_state {
            let stroke = self.stroke;

            let current_end = current_pointer_pos.unwrap_or(*start);

            if start.distance(current_end) > 0.0 {
                let rect = egui::Rect::from_two_pos(*start, current_end);
                // let painter = ui.painter().with_clip_rect(possible_area);
                let painter = ui.painter();
                painter.rect_filled(rect.shrink(stroke.width), 0., self.fill);

                if stroke.width > 0. {
                    let inset_offset: f32 = match stroke.style {
                        StrokeStyle::Dashed { .. } => stroke.width,
                        StrokeStyle::Dotted { radius, .. } => radius.div(2.),
                        _ => 0.,
                    };

                    // We need to draw the shape inside the space
                    // So thats why we are offsetting those positions
                    // Otherwise the border gets clipped
                    let left_upper_corner =
                        egui::Pos2::new(rect.min.x.add(inset_offset), rect.min.y.add(inset_offset));
                    let right_upper_corner =
                        egui::Pos2::new(rect.max.x.sub(inset_offset), rect.min.y.sub(inset_offset));
                    let right_down_corner =
                        egui::Pos2::new(rect.max.x.sub(inset_offset), rect.max.y.sub(inset_offset));
                    let left_down_corner =
                        egui::Pos2::new(rect.min.x.add(inset_offset), rect.max.y.sub(inset_offset));

                    let shapes = match stroke.style {
                        StrokeStyle::Dashed {
                            dash_length,
                            gap_length,
                        } => egui::Shape::dashed_line(
                            &[
                                left_upper_corner,
                                right_upper_corner,
                                right_down_corner,
                                left_down_corner,
                                left_upper_corner,
                            ],
                            *stroke, // for example visuals.selection.stroke
                            dash_length,
                            gap_length,
                        ),
                        StrokeStyle::Solid => {
                            vec![egui::Shape::rect_stroke(
                                rect,
                                0.,
                                *stroke, // for example visuals.window_stroke
                                egui::StrokeKind::Inside,
                            )]
                        }
                        StrokeStyle::Dotted {
                            color,
                            spacing,
                            radius,
                        } => egui::Shape::dotted_line(
                            &[
                                left_upper_corner,
                                right_upper_corner,
                                right_down_corner,
                                left_down_corner,
                                left_upper_corner,
                            ],
                            color,
                            spacing,
                            radius,
                        ),
                    };

                    painter.add(shapes);
                }
            }

            *end = current_end;
        }

        let selection = current_state.as_ref().map(|cs| cs.rect()).flatten();
        
        // Short time lock again so we insert updated state
        ui.data_mut(|global_state| {
            let marquee_map = global_state.get_temp_mut_or_default::<IdMap<State>>(const_map_name);
            marquee_map.insert(self.id, current_state.unwrap_or_default());
        });

        MarqueeResponse {
            inner: inner_response.inner,
            response: inner_response.response,
            selection,
        }
    }
}
