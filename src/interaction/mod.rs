use std::ops::Deref;

mod settings;
mod widget;

pub use settings::InteractionSettings;

pub trait Interactive<Inside, Outside>: Sized {
    fn interactive<Id: Into<egui::Id>>(
        self,
        id: Id,
        settings: InteractionSettings<Inside, Outside>,
    ) -> InteractiveComponent<Self, Inside, Outside>;
}

pub struct InteractiveComponent<Component, Inside, Outside>
where
    Component: Interactive<Inside, Outside>,
{
    id: egui::Id,
    inner: Component,
    settings: InteractionSettings<Inside, Outside>,
}

impl<Component, Inside, Outside> InteractiveComponent<Component, Inside, Outside>
where
    Component: Interactive<Inside, Outside>,
{
    pub fn new<I: Into<egui::Id>>(inner: Component, id: I, settings: InteractionSettings<Inside, Outside>) -> Self {
        Self {  id: I::into(id), inner, settings }
    }
}

impl<Component, Inside, Outside> Deref for InteractiveComponent<Component, Inside, Outside>
where
    Component: Interactive<Inside, Outside>,
{
    type Target = Component;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
