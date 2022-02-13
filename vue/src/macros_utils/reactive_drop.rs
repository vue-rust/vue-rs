use crate::Reactive;

pub struct ReactiveDrop<P>(pub Reactive<P>);

impl<P> Drop for ReactiveDrop<P> {
    fn drop(&mut self) {
        self.0.mark_dropped();
    }
}
