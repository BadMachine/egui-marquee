use crate::interaction::InteractiveComponent;
use crate::state::State;
use crate::{Interactive, MARQUEE_STORAGE_NAME};
use egui::{Id, IdMap, Rect, UiBuilder, Widget};

impl<W, Inside, Outside> Interactive<Inside, Outside> for W
where
    W: Widget,
    Inside: for<'a> FnOnce(&mut W, Vec<Id>, &'a mut egui::Ui),
    Outside: for<'a> FnOnce(&mut W, &'a mut egui::Ui),
{
    fn interactive<Id: Into<egui::Id>>(
        self,
        id: Id,
    ) -> InteractiveComponent<Self, Inside, Outside> {
        InteractiveComponent::<Self, Inside, Outside>::new(self, id)
    }
}

impl<Component, Inside, Outside> Widget for InteractiveComponent<Component, Inside, Outside>
where
    Component: Interactive<Inside, Outside> + Widget,
    Inside: for<'a> FnOnce(&mut Component, Vec<Id>, &'a mut egui::Ui),
    Outside: for<'a> FnOnce(&mut Component, &'a mut egui::Ui),
{
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let InteractiveComponent {
            id,
            inside_callback,
            outside_callback,
            mut inner,
        } = self;

        let previous_response = ui.ctx().read_response(id);

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

            if !intersects_with.is_empty()
                && let Some(cb) = inside_callback
            {
                cb(&mut inner, intersects_with, ui);
            } else if let Some(cb) = outside_callback {
                cb(&mut inner, ui);
            }
        }

        // use scope since we need to hold a unique Id for the widget
        // TODO: change id method to scope_id after 0.37 released
        ui.scope_builder(UiBuilder::new().id(id), |ui| inner.ui(ui))
            .inner
    }
}
