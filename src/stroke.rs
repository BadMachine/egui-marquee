use egui::{Color32, Stroke};
use std::ops::Deref;

#[derive(Debug, Default)]
pub struct MarqueeStroke {
    inner: Stroke,
    pub style: StrokeStyle,
}

impl Deref for MarqueeStroke {
    type Target = Stroke;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl MarqueeStroke {
    pub fn new(inner: Option<Stroke>) -> Self {
        Self {
            inner: inner.unwrap_or_default(),
            style: Default::default(),
        }
    }

    pub fn style(mut self, style: StrokeStyle) -> Self {
        self.style = style;
        self
    }

    pub(crate) fn set_inner_stroke(&mut self, stroke: Stroke) {
        self.inner = stroke;
    }
}

#[derive(Debug, Default)]
pub enum StrokeStyle {
    #[default]
    Solid,
    Dashed {
        dash_length: f32,
        gap_length: f32,
    },
    Dotted {
        color: Color32,
        spacing: f32,
        radius: f32,
    },
}
