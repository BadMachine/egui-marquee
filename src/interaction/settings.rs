pub struct InteractionSettings<Inside, Outside> {
    pub(crate) inside_callback: Option<Inside>,
    pub(crate) outside_callback: Option<Outside>,
}

impl<Inside, Outside> InteractionSettings<Inside, Outside>
where
    Inside: FnOnce(&mut Self, Vec<egui::Id>, &mut egui::Ui),
    Outside: FnOnce(&mut Self, &mut egui::Ui),
{
    pub fn new() -> Self {
        Self {
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
