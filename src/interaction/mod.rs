use crate::MARQUEE_STORAGE_NAME;
use crate::state::State;
use egui::{Id, IdMap, Response};
use std::ops::Deref;

pub(crate) mod response;
mod widget;

pub struct Noop;

pub type InsideAreaId = Id;
pub type LeftAreaId = Id;
pub type FinishAreaId = Id;

pub trait Interactive<Inside, Outside, End>: Sized {
    fn interactive<Id: Into<egui::Id>>(
        self,
        id: Id,
    ) -> InteractiveComponent<Self, Inside, Outside, End>;
}

impl<Component> Interactive<Noop, Noop, Noop> for Component {
    fn interactive<Id: Into<egui::Id>>(self, id: Id) -> InteractiveComponent<Component> {
        InteractiveComponent::<Component>::new(self, id)
    }
}

pub struct InteractiveComponent<Component, Inside = Noop, Outside = Noop, End = Noop> {
    id: egui::Id,
    inner: Component,
    inside_callback: Inside,
    outside_callback: Outside,
    end_callback: End,
}

#[derive(Default, Debug)]
pub(crate) struct InteractionCheck {
    pub(crate) inside: Vec<InsideAreaId>,
    pub(crate) left: Vec<LeftAreaId>,
    pub(crate) finished: Vec<FinishAreaId>,
}

impl<Component, Inside, Outside, Finished>
    InteractiveComponent<Component, Inside, Outside, Finished>
{
    pub(crate) fn check_intersection(
        component_id: &Id,
        ctx: &egui::Context,
        current_response: Option<&Response>,
    ) -> InteractionCheck {
        let ctx_response = ctx.read_response(*component_id);

        let response = current_response
            .map(|r| Some(r))
            .unwrap_or(ctx_response.as_ref());

        let mut check = InteractionCheck::default();

        if let Some(response) = response {
            let rect = response.interact_rect;

            // Short lock
            let mut areas =
                ctx.data(|storage| storage.get_temp::<IdMap<State>>(Id::new(MARQUEE_STORAGE_NAME)));

            // State from the previous render
            // We gonna check against previously saved components and
            // pick those who out of the intersection in this frame
            // Or those which intersected with the area for the first time
            if let Some(recent_areas) = &mut areas {
                // let prev_frame_component_state = recent_areas.get(component_id);

                for (area_id, area_state) in recent_areas {
                    match area_state {
                        State::Active {
                            start,
                            end,
                            components,
                        } => {
                            // If intersects
                            if egui::Rect::from_two_pos(*start, *end).intersects(rect) {
                                // if components.insert(*component_id) {
                                //     check.inside.push(*area_id)
                                // }
                                // Unconditionally insert so it is fresh each frame
                                check.inside.push(*area_id)

                                // If did not previously contain
                            // Does not intersect
                            } else {
                                // Drop if prev contained the component
                                // if components.remove(component_id) {
                                //     check.left.push(*area_id)
                                // }

                                // Unconditionally insert so it is fresh each frame
                                check.left.push(*area_id)
                            }
                        }
                        State::Finished {
                            start,
                            end,
                            components,
                        } => {
                            if egui::Rect::from_two_pos(*start, *end).intersects(rect) {
                                // if components.insert(*component_id) {
                                //     check.finished.push(*area_id)
                                // }
                                check.finished.push(*area_id)
                            } else {
                                if components.remove(component_id) {
                                    check.left.push(*area_id)
                                }
                            }
                        }
                        State::Inactive => {
                            // Do nothing
                        }
                    }
                }
            }

            ctx.data_mut(|storage| {
                storage.insert_temp::<IdMap<State>>(
                    Id::new(MARQUEE_STORAGE_NAME),
                    areas.unwrap_or_default(),
                )
            });
        }

        return check
    }
}

impl<Component> InteractiveComponent<Component> {
    pub fn new<I: Into<egui::Id>>(inner: Component, id: I) -> Self {
        Self {
            id: I::into(id),
            inner,
            inside_callback: Noop,
            outside_callback: Noop,
            end_callback: Noop,
        }
    }
}

impl<Component, Outside> InteractiveComponent<Component, Noop, Outside> {
    pub fn inside<Inside>(
        self,
        inside_callback: Inside,
    ) -> InteractiveComponent<Component, Inside, Outside>
    where
        Inside: for<'a> FnOnce(&mut Component, Vec<Id>),
    {
        InteractiveComponent {
            id: self.id,
            inner: self.inner,
            inside_callback,
            outside_callback: self.outside_callback,
            end_callback: self.end_callback,
        }
    }
}

impl<Component, Inside> InteractiveComponent<Component, Inside, Noop, Noop> {
    pub fn outside<Outside>(
        self,
        outside_callback: Outside,
    ) -> InteractiveComponent<Component, Inside, Outside>
    where
        Outside: for<'a> FnOnce(&mut Component),
    {
        InteractiveComponent {
            id: self.id,
            inner: self.inner,
            inside_callback: self.inside_callback,
            end_callback: self.end_callback,
            outside_callback,
        }
    }
}

impl<Component, Inside, Outside> InteractiveComponent<Component, Inside, Outside> {
    pub fn end<Finished>(
        self,
        end_callback: Finished,
    ) -> InteractiveComponent<Component, Inside, Outside, Finished>
    where
        Finished: for<'a> FnOnce(&mut Component, Vec<FinishAreaId>),
    {
        InteractiveComponent {
            id: self.id,
            inner: self.inner,
            inside_callback: self.inside_callback,
            outside_callback: self.outside_callback,
            end_callback,
        }
    }
}

impl<Component, Inside, Outside, End> Deref
    for InteractiveComponent<Component, Inside, Outside, End>
where
    Component: Interactive<Inside, Outside, End>,
{
    type Target = Component;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
