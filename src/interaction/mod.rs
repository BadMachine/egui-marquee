use std::ops::Deref;

mod widget;

pub struct Noop;

pub trait Interactive<Inside, Outside>: Sized {
    fn interactive<Id: Into<egui::Id>>(self, id: Id)
    -> InteractiveComponent<Self, Inside, Outside>;
}

pub struct InteractiveComponent<Component, Inside = Noop, Outside = Noop> {
    id: egui::Id,
    inner: Component,
    inside_callback: Inside,
    outside_callback: Outside,
}

impl<Component> InteractiveComponent<Component, Noop, Noop> {
    pub fn new<I: Into<egui::Id>>(inner: Component, id: I) -> Self {
        Self {
            id: I::into(id),
            inner,
            inside_callback: Noop,
            outside_callback: Noop,
        }
    }
}

impl<Component, Outside> InteractiveComponent<Component, Noop, Outside> {
    pub fn inside<Inside>(
        self,
        inside_callback: Inside,
    ) -> InteractiveComponent<Component, Inside, Outside>
    where
        Inside: for<'a> FnOnce(&mut Component, Vec<egui::Id>, &'a mut egui::Ui),
    {
        InteractiveComponent {
            id: self.id,
            inner: self.inner,
            inside_callback,
            outside_callback: self.outside_callback,
        }
    }
}

impl<Component, Inside> InteractiveComponent<Component, Inside, Noop> {
    pub fn outside<Outside>(
        self,
        outside_callback: Outside,
    ) -> InteractiveComponent<Component, Inside, Outside>
    where
        Outside: for<'a> FnOnce(&mut Component, &'a mut egui::Ui),
    {
        InteractiveComponent {
            id: self.id,
            inner: self.inner,
            inside_callback: self.inside_callback,
            outside_callback,
        }
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
