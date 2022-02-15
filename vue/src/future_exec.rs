use std::{
    cell::Cell,
    future::Future,
    pin::Pin,
    rc::{Rc, Weak},
    task::{Context, Poll},
};
use wasm_bindgen::JsValue;

pub struct FutureExec<R>(Rc<Output<R>>);

impl<R> FutureExec<R> {
    pub fn from_future<F>(f: F) -> Self
    where
        F: Future<Output = R> + 'static,
    {
        Self(Default::default())
    }
}

type Output<R> = Cell<Option<R>>;

struct Fut<F, R> {
    fut: F,
    result: Weak<Output<R>>,
}

impl<F, R> Future for Fut<F, R>
where
    F: Future<Output = R>,
{
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let result = match self.result.upgrade() {
            Some(rc) => rc,
            None => return Poll::Ready(()),
        };

        match unsafe { self.map_unchecked_mut(|f| &mut f.fut) }.poll(cx) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(r) => {
                result.set(Some(r));
                Poll::Ready(())
            }
        }
    }
}

struct 