use crate::MARQUEE_STORAGE_NAME;
use crate::interaction::{FinishAreaId, InsideAreaId, InteractiveComponent, Noop};
use crate::state::State;
use egui::{Id, IdMap, Rect, Response, UiBuilder, Widget};

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
    pub fn check_intersection(
        &self,
        ctx: &egui::Context,
        current_response: Option<&Response>,
    ) -> (Vec<InsideAreaId>, Vec<FinishAreaId>) {
        let ctx_response = ctx.read_response(self.id);

        let response = current_response
            .map(|r| Some(r))
            .unwrap_or(ctx_response.as_ref());

        if let Some(response) = response {
            let rect = response.interact_rect;

            return ctx.data(|storage| {
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
    Inside: for<'a> FnOnce(&mut Component, Vec<InsideAreaId>),
{
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let (insides, ..) = self.check_intersection(ui.ctx(), None);

        let InteractiveComponent {
            id,
            mut inner,
            inside_callback,
            ..
        } = self;

        if !insides.is_empty() {
            inside_callback(&mut inner, insides);
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
        let (insides, ..) = self.check_intersection(ui.ctx(), None);

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
    Inside: for<'a> FnOnce(&mut Component, Vec<InsideAreaId>),
    Outside: for<'a> FnOnce(&mut Component),
{
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let (insides, ..) = self.check_intersection(ui.ctx(), None);

        let InteractiveComponent {
            id,
            mut inner,
            inside_callback,
            outside_callback,
            ..
        } = self;

        if insides.is_empty() {
            outside_callback(&mut inner);
        } else {
            inside_callback(&mut inner, insides);
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
        let (insides, finished) = self.check_intersection(ui.ctx(), None);

        let InteractiveComponent {
            id,
            mut inner,
            inside_callback,
            outside_callback,
            end_callback,
        } = self;

        if insides.is_empty() {
            outside_callback(&mut inner);
        } else {
            inside_callback(&mut inner, insides);
        }

        if !finished.is_empty() {
            end_callback(&mut inner, finished);
        }

        // use scope since we need to hold a unique Id for the widget
        // TODO: change id method to scope_id after 0.37 released
        ui.scope_builder(UiBuilder::new().id(id), |ui| inner.ui(ui))
            .inner
    }
}
