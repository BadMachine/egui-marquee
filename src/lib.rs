mod interaction;
mod response;
mod state;
mod stroke;

pub use interaction::Interactive;

pub use response::*;

pub use stroke::*;

// egui::Id is not const compatible
pub(crate) const MARQUEE_STORAGE_NAME: &str = "__marquee::storage__";

use egui::{
    Color32, Id, IdMap, InnerResponse, PointerButton, Pos2, Rect, Sense, Stroke, Ui, UiBuilder,
};
use state::State;

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
            spacing: spacing.unwrap_or(radius * 2.),
        };

        self
    }
}

impl Marquee {
    pub fn new(id: Id) -> Self {
        Self {
            id,
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

        let InnerResponse { inner, response } = ui.scope_builder(
            UiBuilder::new().id_salt(self.id).sense(Sense::drag()),
            add_contents,
        );

        // Get global immutable marquee map (don't lock for too long)
        let marquee_map = ui.data(|d| d.get_temp::<IdMap<State>>(const_map_name));

        // Copy state that needs to be mutated below (we gonna insert it next time we get mutable map)
        let mut state: State = match marquee_map {
            Some(map) => map.get(&self.id).map(|s| s.clone()).unwrap_or_default(),
            None => State::default(),
        };

        let current_pointer_pos = response.interact_pointer_pos();

        if response.drag_started_by(self.button) {
            state.start = ui
                .input(|i| i.pointer.press_origin())
                .or_else(|| current_pointer_pos);
        }

        let stroke = self.stroke;

        let possible_area = response.rect.intersect(ui.clip_rect());

        let selection = match (state.start, current_pointer_pos) {
            (Some(start), Some(current))
                if response.dragged_by(self.button) || response.drag_stopped_by(self.button) =>
            {
                // Here is the tricky part: we need to preserve the order (dont create a new Rect out of those positions) because
                // we possibly need to check intersection later
                // but Rect::min/max messes stuff around
                Some((start, current.clamp(possible_area.min, possible_area.max)))
            }
            _ => None,
        };

        if let Some((start_position, end_position)) = selection {
            let rect = Rect::from_two_pos(start_position, end_position);
            let painter = ui.painter().with_clip_rect(possible_area);

            painter.rect_filled(rect.shrink(stroke.width), 0., self.fill);

            if stroke.width > 0. {
                let inset_offset: f32 = match stroke.style {
                    StrokeStyle::Dashed { .. } => stroke.width,
                    StrokeStyle::Dotted { radius, .. } => radius / 2.,
                    _ => 0.,
                };

                // We need to draw the shape inside the space
                // So thats why we are offsetting those positions
                // Otherwise the border gets clipped
                let left_upper_corner =
                    Pos2::new(rect.min.x + inset_offset, rect.min.y + inset_offset);
                let right_upper_corner =
                    Pos2::new(rect.max.x - inset_offset, rect.min.y + inset_offset);
                let right_down_corner =
                    Pos2::new(rect.max.x - inset_offset, rect.max.y - inset_offset);
                let left_down_corner =
                    Pos2::new(rect.min.x + inset_offset, rect.max.y - inset_offset);

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
                        vec![
                            egui::Shape::rect_stroke(
                                rect,
                                0.,
                                *stroke, // for example visuals.window_stroke
                                egui::StrokeKind::Inside,
                            ),
                            // egui::Shape::circle_stroke(
                            //     rect.center(),
                            //     (rect.width().min(rect.height()) / 2.) - inset_offset,
                            //     *stroke, // for example visuals.window_stroke
                            // ),
                        ]
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

            state.end = Some(end_position)
        }

        // Purge if selection ended
        if selection.is_none() || response.drag_stopped() {
            state.start = None;
            state.end = None;
        }

        // Short time lock again so we insert updated state
        ui.data_mut(|global_state| {
            let marquee_map = global_state.get_temp_mut_or_default::<IdMap<State>>(const_map_name);
            marquee_map.insert(self.id, state);
        });

        MarqueeResponse {
            inner,
            response,
            selection: selection.map(|(start, end)| Rect::from_two_pos(start, end)),
        }
    }
}
