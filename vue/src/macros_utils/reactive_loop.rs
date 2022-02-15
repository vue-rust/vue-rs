use crate::Reactive;
use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

pub fn reactive_loop<F, P>(func: F, reactive: Reactive<P>)
where
    F: FnMut(Reactive<P>) + Unpin + 'static,
    P: 'static,
{
    wasm_bindgen_futures::spawn_local(Fut(func, reactive));
}

struct Fut<F, P>(F, Reactive<P>);

impl<F, P> Future for Fut<F, P>
where
    F: FnMut(Reactive<P>) + Unpin,
{
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.1.dropped() {
            return Poll::Ready(());
        } else if !self.1.changed() {
            return Poll::Pending;
        }

        let this = self.get_mut();
        (&mut this.0)(this.1.clone());
        this.1.set_waker(cx.waker().clone());

        Poll::Pending
    }
}
