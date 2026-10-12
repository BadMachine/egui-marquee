use egui::Pos2;

#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct State {
    pub(crate) start: Option<Pos2>,
    pub(crate) end: Option<Pos2>,
}
