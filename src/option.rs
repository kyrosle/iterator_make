use crate::iterator::{IntoIterator, Iterator};

#[derive(Clone, Debug)]
struct Item<A> {
    opt: Option<A>,
}
impl<A> Iterator for Item<A> {
    type Item = A;

    fn next(&mut self) -> Option<A> {
        self.opt.take()
    }
}
pub struct IntoIter<A> {
    inner: Item<A>,
}
impl<A> Iterator for IntoIter<A> {
    type Item = A;

    fn next(&mut self) -> Option<A> {
        self.inner.next()
    }
}
impl<T> IntoIterator for Option<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;
    fn into_iter(self) -> IntoIter<T> {
        IntoIter {
            inner: Item { opt: self },
        }
    }
}
