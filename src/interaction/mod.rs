use std::ops::Deref;

mod widget;

pub trait Interactive<Inside, Outside>: Sized {
    fn interactive<Id: Into<egui::Id>>(self, id: Id)
    -> InteractiveComponent<Self, Inside, Outside>;
}

pub struct InteractiveComponent<Component, Inside, Outside>
where
    Component: Interactive<Inside, Outside>,
{
    id: egui::Id,
    inner: Component,
    inside_callback: Option<Inside>,
    outside_callback: Option<Outside>,
}

impl<Component, Inside, Outside> InteractiveComponent<Component, Inside, Outside>
where
    Component: Interactive<Inside, Outside>,
    Inside: FnOnce(&mut Component, Vec<egui::Id>, &mut egui::Ui),
    Outside: FnOnce(&mut Component, &mut egui::Ui),
{
    pub fn new<I: Into<egui::Id>>(inner: Component, id: I) -> Self {
        Self {
            id: I::into(id),
            inner,
            inside_callback: None,
            outside_callback: None,
        }
    }

    pub fn inside(mut self, function: Inside) -> Self {
        self.inside_callback = Some(function);
        self
    }

    pub fn outside(mut self, function: Outside) -> Self {
        self.outside_callback = Some(function);
        self
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
