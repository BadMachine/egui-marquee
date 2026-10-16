use crate::interaction::{FinishAreaId, InsideAreaId, InteractiveComponent, Noop};
use egui::{UiBuilder, Widget};

impl<Component> egui::Widget for InteractiveComponent<Component, Noop, Noop, Noop>
where
    Component: egui::Widget,
{
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        Component::ui(self.inner, ui)
    }
}

// Inside
impl<Component, Inside> egui::Widget for InteractiveComponent<Component, Inside>
where
    Component: egui::Widget,
    Inside: for<'a> FnOnce(&mut Component, Vec<InsideAreaId>),
{
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let InteractiveComponent {
            id,
            mut inner,
            inside_callback,
            ..
        } = self;

        let intersection_check = Self::check_intersection(&id, ui.ctx(), None);

        if !intersection_check.inside.is_empty() {
            inside_callback(&mut inner, intersection_check.inside);
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
        let InteractiveComponent {
            id,
            mut inner,
            outside_callback,
            ..
        } = self;

        let intersection_check = Self::check_intersection(&id, ui.ctx(), None);

        if !intersection_check.left.is_empty() {
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
    Inside: for<'a> FnOnce(&mut Component, Vec<InsideAreaId>),
    Outside: for<'a> FnOnce(&mut Component),
{
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let InteractiveComponent {
            id,
            mut inner,
            inside_callback,
            outside_callback,
            ..
        } = self;

        let intersection_check = Self::check_intersection(&id, ui.ctx(), None);

        if !intersection_check.inside.is_empty() {
            inside_callback(&mut inner, intersection_check.inside);
        }

        if !intersection_check.left.is_empty() {
            outside_callback(&mut inner);
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
    Inside: for<'a> FnOnce(&mut Component, Vec<InsideAreaId>),
    Outside: for<'a> FnOnce(&mut Component),
    Finished: for<'a> FnOnce(&mut Component, Vec<FinishAreaId>),
{
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let InteractiveComponent {
            id,
            mut inner,
            inside_callback,
            outside_callback,
            end_callback,
        } = self;

        let intersection_check = Self::check_intersection(&id, ui.ctx(), None);

        if !intersection_check.inside.is_empty() {
            inside_callback(&mut inner, intersection_check.inside);
        }

        if !intersection_check.left.is_empty() {
            outside_callback(&mut inner);
        }

        if !intersection_check.finished.is_empty() {
            end_callback(&mut inner, intersection_check.finished);
        }

        // use scope since we need to hold a unique Id for the widget
        // TODO: change id method to scope_id after 0.37 released
        ui.scope_builder(UiBuilder::new().id(id), |ui| inner.ui(ui))
            .inner
    }
}
