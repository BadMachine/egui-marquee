pub struct InteractionSettings<Inside, Outside> {
    pub(crate) inside_callback: Inside,
    pub(crate) outside_callback: Outside,
}

impl<Inside, Outside> InteractionSettings<Inside, Outside>
where
    Inside: FnOnce(&mut Self, Vec<egui::Id>, &mut egui::Ui),
    Outside: FnOnce(&mut Self, &mut egui::Ui),
{
    pub fn new(
        inside_callback: Inside,
        outside_callback: Outside,
    ) -> Self {
        Self {
            inside_callback,
            outside_callback,
        }
    }
}

// Inside: FnOnce(&mut Self, Vec<Id>, &mut egui::Ui),
// Outside: FnOnce(&mut Self, &mut egui::Ui),

// Inside: for<'a> FnOnce(&mut Component, Vec<Id>, &'a mut egui::Ui),
// Outside: for<'a> FnOnce(&mut Component, &'a mut egui::Ui),
