use egui::Id;
use std::ops::Deref;

mod widget;

pub trait Interactive<Inside, Outside>: Sized
where
    Inside: FnOnce(&mut Self, Vec<Id>, &mut egui::Ui),
    Outside: FnOnce(&mut Self, &mut egui::Ui),
{
    fn interactive<Id: Into<egui::Id>>(
        self,
        id: Id,
        inside: Inside,
        outside: Outside,
    ) -> InteractiveComponent<Self, Inside, Outside>;
}

pub struct InteractiveComponent<Component, Inside, Outside>
where
    Component: Interactive<Inside, Outside>,
    Inside: for<'a> FnOnce(&mut Component, Vec<Id>, &'a mut egui::Ui),
    Outside: for<'a> FnOnce(&mut Component, &'a mut egui::Ui),
{
    id: Id,
    inner: Component,
    inside: Inside,
    outside: Outside,
}

impl<Inside, Outside, Component: Interactive<Inside, Outside>>
    InteractiveComponent<Component, Inside, Outside>
where
    Component: Interactive<Inside, Outside>,
    Inside: for<'a> FnOnce(&mut Component, Vec<Id>, &'a mut egui::Ui),
    Outside: for<'a> FnOnce(&mut Component, &'a mut egui::Ui),
{
    pub fn new(id: Id, inner: Component, inside: Inside, outside: Outside) -> Self {
        Self {
            id,
            inner,
            inside,
            outside,
        }
    }
}

impl<Component, Inside, Outside> Deref for InteractiveComponent<Component, Inside, Outside>
where
    Component: Interactive<Inside, Outside>,
    Inside: for<'a> FnOnce(&mut Component, Vec<Id>, &'a mut egui::Ui),
    Outside: for<'a> FnOnce(&mut Component, &'a mut egui::Ui),
{
    type Target = Component;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
