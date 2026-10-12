use egui::{Rect, Response};

pub struct MarqueeResponse<R> {
    pub inner: R,
    pub response: Response,
    pub selection: Option<Rect>,
}