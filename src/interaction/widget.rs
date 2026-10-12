use crate::interaction::InteractiveComponent;
use crate::state::State;
use crate::{Interactive, MARQUEE_STORAGE_NAME};
use egui::{Id, IdMap, Rect, UiBuilder, Widget};

impl<W, Callback> Interactive<Callback> for W
where
    W: Widget,
    Callback: for<'a> FnOnce(&mut Self, Vec<Id>, &'a mut egui::Ui),
{
    fn interactive<Id: Into<egui::Id>>(
        self,
        id: Id,
        callback: Callback,
    ) -> InteractiveComponent<Callback, Self> {
        InteractiveComponent::new(id.into(), self, callback)
    }
}

impl<Callback, Component> Widget for InteractiveComponent<Callback, Component>
where
    Component: Interactive<Callback> + Widget,
    Callback: for<'a> FnOnce(&mut Component, Vec<Id>, &'a mut egui::Ui),
{
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let InteractiveComponent {
            id,
            mut inner,
            callback,
        } = self;

        let previous_response = ui.ctx().read_response(id);

        if let Some(response) = previous_response {
            let rect = response.interact_rect;

            let intersects_with: Vec<Id> = ui.data(|storage| {
                let areas = storage.get_temp::<IdMap<State>>(Id::unique(MARQUEE_STORAGE_NAME));

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

            if !intersects_with.is_empty() {
                callback(&mut inner, intersects_with, ui);
            }
        }

        // use scope since we need to hold a unique Id for the widget
        ui.scope_builder(UiBuilder::new().scope_id(id), |ui| inner.ui(ui))
            .inner
    }
}
