use crate::Reactive;
use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

struct ReactiveFut<F, P> {
    f: F,
    reactive: Reactive<P>,
}

impl<F, P> Future for ReactiveFut<F, P>
where
    F: Future<Output = ()>,
    P: 'static,
{
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.reactive.dropped() {
            return Poll::Ready(());
        }

        let reactive = self.reactive.clone();
        let f = unsafe { self.map_unchecked_mut(|this| &mut this.f) };

        match f.poll(cx) {
            Poll::Pending => {
                reactive.set_waker(cx.waker().clone());
                Poll::Pending
            }
            Poll::Ready(()) => Poll::Ready(()),
        }
    }
}

pub fn reactive_fut<F, P>(f: F, reactive: Reactive<P>)
where
    F: Future<Output = ()> + 'static,
    P: 'static,
{
    wasm_bindgen_futures::spawn_local(ReactiveFut { f, reactive });
}
