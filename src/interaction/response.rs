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
        // println!("wtf------------ {:?}", self.inner.interact_rect);
    
        let (insides, finished) = self.check_intersection(&self.inner.ctx, Some(&self.inner));


        let InteractiveComponent {
            id,
            mut inner,
            inside_callback,
            outside_callback,
            end_callback,
        } = self;

        // if insides.is_empty() {
        //     outside_callback(&mut inner, ui);
        // } else {
        //     inside_callback(&mut inner, insides, ui);
        // }

        if !finished.is_empty() {
            end_callback(&mut inner, finished);
        }
    }
}
