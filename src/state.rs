use egui::Pos2;

#[derive(Clone, Copy, Debug, Default)]
pub enum State {
    Active { start: Pos2, end: Pos2 },
    Finished { start: Pos2, end: Pos2 },
    #[default]
    Inactive,
}

// #[derive(Clone, Copy, Debug, Default)]
// pub(crate) struct State {
//
//     pub(crate) start: Option<Pos2>,
//     pub(crate) end: Option<Pos2>,
// }
