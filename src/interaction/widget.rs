use crate::interaction::{InteractiveComponent, Noop};
use crate::state::State;
use crate::{Interactive, MARQUEE_STORAGE_NAME};
use egui::{Id, IdMap, Rect, UiBuilder, Widget};

impl<Component> Interactive<Noop, Noop> for Component
where
    Component: Widget,
{
    fn interactive<Id: Into<egui::Id>>(self, id: Id) -> InteractiveComponent<Component> {
        InteractiveComponent::<Component>::new(self, id)
    }
}
impl<Component> Widget for InteractiveComponent<Component, Noop, Noop>
where
    Component: Widget,
{
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        Component::ui(self.inner, ui)
    }
}

impl<Component, Inside, Outside> InteractiveComponent<Component, Inside, Outside> {
    pub fn check_intersection(&self, ui: &mut egui::Ui) -> Vec<Id> {
        let previous_response = ui.ctx().read_response(self.id);

        if let Some(response) = previous_response {
            let rect = response.interact_rect;

            let intersects_with: Vec<Id> = ui.data(|storage| {
                // TODO: change "new" with "unique" after 0.37 release
                let areas = storage.get_temp::<IdMap<State>>(Id::new(MARQUEE_STORAGE_NAME));

                match areas {
                    None => vec![],
                    Some(areas) => areas
                        .iter()
                        .filter_map(|(id, state)| {
                            if let (Some(start), Some(end)) = (state.start, state.end) {
                                let area_rect = Rect::from_two_pos(start, end);

                                if area_rect.intersects(rect) {
                                    return Some(id.to_owned());
                                }
                            }

                            return None;
                        })
                        .collect::<Vec<_>>(),
                }
            });

            return intersects_with;
        }

        return vec![];
    }
}

impl<Component, Inside> Widget for InteractiveComponent<Component, Inside, Noop>
where
    Component: Widget,
    Inside: for<'a> FnOnce(&mut Component, Vec<Id>, &'a mut egui::Ui),
{
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let intersects_with = self.check_intersection(ui);
        let InteractiveComponent {
            id,
            mut inner,
            inside_callback,
            ..
        } = self;

        if !intersects_with.is_empty() {
            inside_callback(&mut inner, intersects_with, ui);
        }

        // use scope since we need to hold a unique Id for the widget
        // TODO: change id method to scope_id after 0.37 released
        ui.scope_builder(UiBuilder::new().id(id), |ui| inner.ui(ui))
            .inner
    }
}

impl<Component, Outside> Widget for InteractiveComponent<Component, Noop, Outside>
where
    Component: Widget,
    Outside: for<'a> FnOnce(&mut Component, &'a mut egui::Ui),
{
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let intersects_with = self.check_intersection(ui);
        let InteractiveComponent {
            id,
            mut inner,
            outside_callback,
            ..
        } = self;

        if intersects_with.is_empty() {
            outside_callback(&mut inner, ui);
        }

        // use scope since we need to hold a unique Id for the widget
        // TODO: change id method to scope_id after 0.37 released
        ui.scope_builder(UiBuilder::new().id(id), |ui| inner.ui(ui))
            .inner
    }
}

impl<Component, Inside, Outside> Widget for InteractiveComponent<Component, Inside, Outside>
where
    Component: Widget,
    Inside: for<'a> FnOnce(&mut Component, Vec<Id>, &'a mut egui::Ui),
    Outside: for<'a> FnOnce(&mut Component, &'a mut egui::Ui),
{
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let intersects_with = self.check_intersection(ui);
        let InteractiveComponent {
            id,
            mut inner,
            inside_callback,
            outside_callback,
            ..
        } = self;

        if intersects_with.is_empty() {
            outside_callback(&mut inner, ui);
        } else {
            inside_callback(&mut inner, intersects_with, ui);
        }

        // use scope since we need to hold a unique Id for the widget
        // TODO: change id method to scope_id after 0.37 released
        ui.scope_builder(UiBuilder::new().id(id), |ui| inner.ui(ui))
            .inner
    }
}
