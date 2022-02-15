use crate::macros_utils::ReactiveTy;
use std::{cell::Cell, mem::replace, rc::Rc, task::Waker};
use wasm_bindgen::JsValue;

pub struct Reactive<P>(#[doc(hidden)] Rc<Cell<Option<Inner<P>>>>);

impl<P> Reactive<P> {
    pub fn new() -> Self
    where
        P: Default,
    {
        Self(Default::default())
    }

    pub(crate) fn changed(&self) -> bool {
        self.map_inner(|inner| inner.changed)
    }

    pub(crate) fn dropped(&self) -> bool {
        self.map_inner(|inner| inner.dropped)
    }

    #[doc(hidden)]
    pub fn macro_set<S, T>(&self, setter: S, value: T, key: &str, rty: ReactiveTy)
    where
        S: FnOnce(&mut P) -> &mut T,
        T: Clone + Into<JsValue> + PartialEq,
    {
        self.map_inner(|inner| {
            set(
                setter(&mut inner.props),
                value,
                &mut inner.changed,
                &inner.target,
                key,
                rty,
                &mut inner.waker,
            )
        });
    }

    #[doc(hidden)]
    pub fn macro_set_active(&self, value: bool) {
        self.map_inner(|inner| {
            if replace(&mut inner.active, value) != value && value {
                inner.changed = true;
                wake(&mut inner.waker);
            }
        });
    }

    #[doc(hidden)]
    pub fn macro_set_target(&self, target: JsValue) {
        self.map_inner(|inner| {
            inner.changed = true;
            inner.target = target;
            wake(&mut inner.waker);
        });
    }

    pub fn map<M, R>(&mut self, mapper: M) -> R
    where
        M: FnOnce(&P) -> R,
    {
        self.map_inner(|inner| mapper(&inner.props))
    }

    fn map_inner<F, R>(&self, map: F) -> R
    where
        F: FnOnce(&mut Inner<P>) -> R,
    {
        let mut inner = self.0.take().expect("Reactive");
        let out = map(&mut inner);
        self.0.set(Some(inner));
        out
    }

    pub(crate) fn mark_dropped(&self) {
        self.map_inner(|inner| {
            inner.dropped = true;
            wake(&mut inner.waker);
        });
    }

    pub(crate) fn set_waker(&self, waker: Waker) {
        self.map_inner(|inner| {
            inner.changed = false;
            inner.waker = Some(waker);
        });
    }
}

impl<P> Clone for Reactive<P> {
    fn clone(&self) -> Self {
        Reactive(self.0.clone())
    }
}

struct Inner<P> {
    active: bool,
    changed: bool,
    dropped: bool,
    props: P,
    target: JsValue,
    waker: Option<Waker>,
}

impl<P> Drop for Inner<P> {
    fn drop(&mut self) {
        // make sure pending future are terminated
        // and the memory is reclaimed.
        wake(&mut self.waker);
    }
}

fn set<T>(
    dst: &mut T,
    value: T,
    changed: &mut bool,
    target: &JsValue,
    key: &str,
    rty: ReactiveTy,
    waker: &mut Option<Waker>,
) where
    T: Clone + Into<JsValue> + PartialEq,
{
    if *dst != value {
        *dst = value;
        *changed = true;
        wake(waker);
        update_vue(target, dst.clone().into(), key, rty);
    }
}

fn update_vue(target: &JsValue, value: JsValue, key: &str, rty: ReactiveTy) {
    if rty == ReactiveTy::Sync {
        crate::emit(target, key, value.clone());
    }

    let s;

    let key = match rty {
        ReactiveTy::Data => key,
        ReactiveTy::Prop | ReactiveTy::Sync => {
            s = format!("d{key}");
            &s
        }
    };

    crate::set(target, key, value);
}

fn wake(waker: &mut Option<Waker>) {
    if let Some(waker) = waker.take() {
        waker.wake();
    }
}
