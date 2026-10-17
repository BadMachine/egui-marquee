use crate::interaction::{FinishAreaId, InteractiveComponent, LeftAreaId, Noop};
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
    Finished: FnOnce(&mut Response, Vec<FinishAreaId>),
{
    fn evaluate(self) {
        let InteractiveComponent {
            id,
            mut inner,
            end_callback,
            ..
        } = self;

        let intersection_check = Self::check_intersection(&id, &inner.ctx, Some(&inner));

        if !intersection_check.finished.is_empty() {
            end_callback(&mut inner, intersection_check.finished);
        }
    }
}

impl<Outside> InteractiveResponse<Noop, Outside, Noop>
    for InteractiveComponent<Response, Noop, Outside>
where
    Outside: FnOnce(&mut Response),
{
    fn evaluate(self) {
        let InteractiveComponent {
            id,
            mut inner,
            outside_callback,
            ..
        } = self;

        let intersection_check = Self::check_intersection(&id, &inner.ctx, Some(&inner));

        // if let Some(insides) = intersection_check.inside
        //     && let Some(finished) = intersection_check.finished
        // {
        //     if insides.is_empty() && finished.is_empty() {
        //         outside_callback(&mut inner);
        //     }
        // }

        if !intersection_check.left.is_empty() {
            outside_callback(&mut inner);
        }
    }
}

impl<Outside, Finished> InteractiveResponse<Noop, Outside, Finished>
    for InteractiveComponent<Response, Noop, Outside, Finished>
where
    Outside: FnOnce(&mut Response, Vec<LeftAreaId>),
    Finished: FnOnce(&mut Response, Vec<FinishAreaId>),
{
    fn evaluate(self) {
        // if self.inner.ctx.data(|map| map.get_temp(self.id)) {}

        let InteractiveComponent {
            id,
            mut inner,
            end_callback,
            outside_callback,
            ..
        } = self;

        let intersection_check = Self::check_intersection(&id, &inner.ctx, Some(&inner));

        if !intersection_check.left.is_empty() {
            outside_callback(&mut inner, intersection_check.left);
        }

        if intersection_check.finished.is_empty() {
            end_callback(&mut inner, intersection_check.finished);
        }
    }
}
