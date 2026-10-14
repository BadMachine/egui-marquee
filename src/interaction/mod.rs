use egui::Id;
use std::ops::Deref;

mod widget;

pub struct Noop;

pub type InsideAreaId = Id;
pub type FinishAreaId = Id;

pub trait Interactive<Inside, Outside, End>: Sized {
    fn interactive<Id: Into<egui::Id>>(
        self,
        id: Id,
    ) -> InteractiveComponent<Self, Inside, Outside, End>;
}

pub struct InteractiveComponent<Component, Inside = Noop, Outside = Noop, End = Noop> {
    id: egui::Id,
    inner: Component,
    inside_callback: Inside,
    outside_callback: Outside,
    end_callback: End,
}

impl<Component> InteractiveComponent<Component> {
    pub fn new<I: Into<egui::Id>>(inner: Component, id: I) -> Self {
        Self {
            id: I::into(id),
            inner,
            inside_callback: Noop,
            outside_callback: Noop,
            end_callback: Noop,
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
            end_callback: self.end_callback,
        }
    }
}

impl<Component, Inside> InteractiveComponent<Component, Inside, Noop, Noop> {
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
            end_callback: self.end_callback,
            outside_callback,
        }
    }
}

impl<Component, Inside, Outside> InteractiveComponent<Component, Inside, Outside> {
    pub fn end<Finished>(
        self,
        end_callback: Finished,
    ) -> InteractiveComponent<Component, Inside, Outside, Finished>
    where
        Finished: for<'a> FnOnce(&mut Component, Vec<FinishAreaId>, &'a mut egui::Ui),
    {
        InteractiveComponent {
            id: self.id,
            inner: self.inner,
            inside_callback: self.inside_callback,
            outside_callback: self.outside_callback,
            end_callback,
        }
    }
}

impl<Component, Inside, Outside, End> Deref
    for InteractiveComponent<Component, Inside, Outside, End>
where
    Component: Interactive<Inside, Outside, End>,
{
    type Target = Component;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
