use egui::{Id, Pos2, Rect};
use std::collections::HashSet;

#[derive(Clone, Debug, Default)]
pub enum State {
    Active {
        start: Pos2,
        end: Pos2,
        components: HashSet<Id>,
    },
    Finished {
        start: Pos2,
        end: Pos2,
        components: HashSet<Id>,
    },
    #[default]
    Inactive,
}

impl State {
    pub(crate) fn components(&self) -> Option<&HashSet<Id>> {
        match self {
            State::Active { components, .. } => Some(components),
            State::Finished { .. } => None,
            State::Inactive => None,
        }
    }

    pub(crate) fn rect(&self) -> Option<Rect> {
        match self {
            State::Active { start, end, .. } => Some(Rect::from_two_pos(*start, *end)),
            State::Finished { start, end, .. } => Some(Rect::from_two_pos(*start, *end)),
            State::Inactive => None,
        }
    }
}

// #[derive(Clone, Copy, Debug, Default)]
// pub(crate) struct State {
//
//     pub(crate) start: Option<Pos2>,
//     pub(crate) end: Option<Pos2>,
// }
