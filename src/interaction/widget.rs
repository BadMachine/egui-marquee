use crate::interaction::{FinishAreaId, InsideAreaId, InteractiveComponent, Noop};
use crate::state::State;
use crate::{Interactive, MARQUEE_STORAGE_NAME};
use egui::{Id, IdMap, Rect, UiBuilder, Widget};

impl<Component> Interactive<Noop, Noop, Noop> for Component
where
    Component: Widget,
{
    fn interactive<Id: Into<egui::Id>>(self, id: Id) -> InteractiveComponent<Component> {
        InteractiveComponent::<Component>::new(self, id)
    }
}
impl<Component> Widget for InteractiveComponent<Component, Noop, Noop, Noop>
where
    Component: Widget,
{
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        Component::ui(self.inner, ui)
    }
}

impl<Component, Inside, Outside, Finished>
    InteractiveComponent<Component, Inside, Outside, Finished>
{
    pub fn check_intersection(&self, ui: &mut egui::Ui) -> (Vec<InsideAreaId>, Vec<FinishAreaId>) {
        let previous_response = ui.ctx().read_response(self.id);

        if let Some(response) = previous_response {
            let rect = response.interact_rect;

            return ui.data(|storage| {
                // TODO: change "new" with "unique" after 0.37 release
                let areas = storage.get_temp::<IdMap<State>>(Id::new(MARQUEE_STORAGE_NAME));

                return match areas {
                    None => (vec![], vec![]),
                    Some(areas) => {
                        areas
                            .iter()
                            .fold((vec![], vec![]), |mut accumulated, (id, state)| {
                                if let State::Active { start, end } = state
                                    && Rect::from_two_pos(*start, *end).intersects(rect)
                                {
                                    accumulated.0.push(id.to_owned());
                                } else if let State::Finished { start, end } = state
                                    && Rect::from_two_pos(*start, *end).intersects(rect)
                                {
                                    accumulated.1.push(id.to_owned());
                                }

                                return accumulated;
                            })
                    }
                };
            });
        }

        return (vec![], vec![]);
    }
}

// Inside
impl<Component, Inside> Widget for InteractiveComponent<Component, Inside>
where
    Component: Widget,
    Inside: for<'a> FnOnce(&mut Component, Vec<InsideAreaId>, &'a mut egui::Ui),
{
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let (insides, ..) = self.check_intersection(ui);

        let InteractiveComponent {
            id,
            mut inner,
            inside_callback,
            ..
        } = self;

        if !insides.is_empty() {
            inside_callback(&mut inner, insides, ui);
        }

        // use scope since we need to hold a unique Id for the widget
        // TODO: change id method to scope_id after 0.37 released
        ui.scope_builder(UiBuilder::new().id(id), |ui| inner.ui(ui))
            .inner
    }
}

// Outside
impl<Component, Outside> Widget for InteractiveComponent<Component, Noop, Outside>
where
    Component: Widget,
    Outside: for<'a> FnOnce(&mut Component, &'a mut egui::Ui),
{
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let (insides, ..) = self.check_intersection(ui);

        let InteractiveComponent {
            id,
            mut inner,
            outside_callback,
            ..
        } = self;

        if insides.is_empty() {
            outside_callback(&mut inner, ui);
        }

        // use scope since we need to hold a unique Id for the widget
        // TODO: change id method to scope_id after 0.37 released
        ui.scope_builder(UiBuilder::new().id(id), |ui| inner.ui(ui))
            .inner
    }
}

// Inside + Outside
impl<Component, Inside, Outside> Widget for InteractiveComponent<Component, Inside, Outside>
where
    Component: Widget,
    Inside: for<'a> FnOnce(&mut Component, Vec<InsideAreaId>, &'a mut egui::Ui),
    Outside: for<'a> FnOnce(&mut Component, &'a mut egui::Ui),
{
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let (insides, ..) = self.check_intersection(ui);

        let InteractiveComponent {
            id,
            mut inner,
            inside_callback,
            outside_callback,
            ..
        } = self;

        if insides.is_empty() {
            outside_callback(&mut inner, ui);
        } else {
            inside_callback(&mut inner, insides, ui);
        }

        // use scope since we need to hold a unique Id for the widget
        // TODO: change id method to scope_id after 0.37 released
        ui.scope_builder(UiBuilder::new().id(id), |ui| inner.ui(ui))
            .inner
    }
}

// Inside + Outside + Finished
impl<Component, Inside, Outside, Finished> Widget
    for InteractiveComponent<Component, Inside, Outside, Finished>
where
    Component: Widget,
    Inside: for<'a> FnOnce(&mut Component, Vec<InsideAreaId>, &'a mut egui::Ui),
    Outside: for<'a> FnOnce(&mut Component, &'a mut egui::Ui),
    Finished: for<'a> FnOnce(&mut Component, Vec<FinishAreaId>, &'a mut egui::Ui),
{
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let (insides, finished) = self.check_intersection(ui);

        let InteractiveComponent {
            id,
            mut inner,
            inside_callback,
            outside_callback,
            end_callback,
        } = self;

        if insides.is_empty() {
            outside_callback(&mut inner, ui);
        } else {
            inside_callback(&mut inner, insides, ui);
        }

        if !finished.is_empty() {
            end_callback(&mut inner, finished, ui);
        }

        // use scope since we need to hold a unique Id for the widget
        // TODO: change id method to scope_id after 0.37 released
        ui.scope_builder(UiBuilder::new().id(id), |ui| inner.ui(ui))
            .inner
    }
}
