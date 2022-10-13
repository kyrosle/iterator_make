use std::{mem, ops};

use crate::iterator::Iterator;
pub trait Step: Clone + PartialOrd + Sized {
    fn steps_between(start: &Self, end: &Self) -> Option<usize>;
    fn forward_checked(start: Self, count: usize) -> Option<Self>;
    fn forward(start: Self, count: usize) -> Self {
        Step::forward_checked(start, count).expect("overflow in `Step::forward`")
    }
    unsafe fn forward_unchecked(start: Self, count: usize) -> Self {
        Step::forward(start, count)
    }
    fn backward_checked(start: Self, count: usize) -> Option<Self>;
    fn backward(start: Self, count: usize) -> Self {
        Step::backward_checked(start, count).expect("overflow in `Step::backward`")
    }
    unsafe fn backward_unchecked(start: Self, count: usize) -> Self {
        Step::backward(start, count)
    }
}

impl<A: Step> Iterator for ops::Range<A> {
    type Item = A;

    fn next(&mut self) -> Option<Self::Item> {
        self.spec_next()
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.spec_nth(n)
    }
}
trait RangeIteratorImpl {
    type Item;
    fn spec_next(&mut self) -> Option<Self::Item>;
    fn spec_nth(&mut self, n: usize) -> Option<Self::Item>;
}
impl<A: Step> RangeIteratorImpl for ops::Range<A> {
    type Item = A;

    fn spec_next(&mut self) -> Option<A> {
        if self.start < self.end {
            let n =
                Step::forward_checked(self.start.clone(), 1).expect("`Step` invariants not upheld");
            Some(mem::replace(&mut self.start, n))
        } else {
            None
        }
    }

    fn spec_nth(&mut self, n: usize) -> Option<A> {
        if let Some(plus_n) = Step::forward_checked(self.start.clone(), n) {
            if plus_n < self.end {
                self.start =
                    Step::forward_checked(plus_n.clone(), 1).expect("`Step` invariants not upheld");
                return Some(plus_n);
            }
        };
        self.start = self.end.clone();
        None
    }
}
