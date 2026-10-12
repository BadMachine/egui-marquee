use std::ops::Deref;
use egui::Id;

mod widget;

pub trait Interactive<Callback>: Sized
where
    Callback: FnOnce(&mut Self, Vec<Id>, &mut egui::Ui),
{
    fn interactive<Id: Into<egui::Id>>(
        self,
        id: Id,
        callback: Callback,
    ) -> InteractiveComponent<Callback, Self>;
}

pub struct InteractiveComponent<Callback, Component: Interactive<Callback>>
where
    Callback: for<'a> FnOnce(&mut Component, Vec<Id>, &'a mut egui::Ui),
{
    id: Id,
    inner: Component,
    callback: Callback,
}

impl<Callback, Component: Interactive<Callback>> InteractiveComponent<Callback, Component>
where
    Callback: for<'a> FnOnce(&mut Component, Vec<Id>, &'a mut egui::Ui),
{
    pub fn new(id: Id, inner: Component, callback: Callback) -> Self {
        Self {
            id,
            inner,
            callback,
        }
    }
}

impl<Callback, Component> Deref for InteractiveComponent<Callback, Component>
where
    Component: Interactive<Callback>,
    Callback: for<'a> FnOnce(&mut Component, Vec<Id>, &'a mut egui::Ui),
{
    type Target = Component;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
