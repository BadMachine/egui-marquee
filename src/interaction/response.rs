use crate::interaction::{FinishAreaId, InteractiveComponent, Noop};
use egui::Response;

pub trait InteractiveResponse<Inside, Outside, Finished>
where
    Self: Sized,
{
    fn evaluate(self) {}
}

impl<Finished> InteractiveResponse<Noop, Noop, Finished>
    for InteractiveComponent<Response, Noop, Noop, Finished>
where
    Finished: for<'a> FnOnce(&mut Response, Vec<FinishAreaId>),
{
    fn evaluate(self) {
        let (insides, finished) = self.check_intersection(&self.inner.ctx, Some(&self.inner));


        let InteractiveComponent {
            mut inner,
            end_callback,
            ..
        } = self;
        if !finished.is_empty() {
            end_callback(&mut inner, finished);
        }
    }
}
