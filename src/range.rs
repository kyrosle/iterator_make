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
    fn size_hint(&self) -> (usize, Option<usize>);
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

    fn size_hint(&self) -> (usize, Option<usize>) {
        if self.start < self.end {
            let hint = Step::steps_between(&self.start, &self.end);
            (hint.unwrap_or(usize::MAX), hint)
        } else {
            (0, Some(0))
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

macro_rules! step_identical_methods {
    () => {
        unsafe fn forward_unchecked(start: Self, n: usize) -> Self {
            unsafe { start.unchecked_add(n as Self) }
        }
        unsafe fn backward_unchecked(start: Self, n: usize) -> Self {
            unsafe { start.unchecked_sub(n as Self) }
        }
        fn forward(start: Self, n: usize) -> Self {
            if Self::forward_checked(start, n).is_none() {
                let _ = Self::MAX + 1;
            }
            start.wrapping_add(n as Self)
        }
        fn backward(start: Self, n: usize) -> Self {
            if Self::backward_checked(start, n).is_none() {
                let _ = Self::MIN - 1;
            }
            start.wrapping_sub(n as Self)
        }
    };
}

macro_rules! step_integer_impls {
    {
        $([$u_narrower:ident $i_narrower:ident]),+;
        $([$u_wider:ident $i_wider:ident]),+;
    } => {
        impl Step for $u_narrower {
            step_identical_methods!();

            fn steps_between(start: &Self, end &Self) -> Option<usize> {
                if *start <= *end {
                    Some((*end - *start) as usize)
                } else {
                    None
                }
            }

            fn forward_checked(start: Self, n: usize) -> Option<Self> {
                match Self::try_from(n) {
                    Ok(n) => start.unchecked_add(n),
                    Err(_) => None,
                }
            }

            fn backward_checked(start: Self, n: usize) -> Option<Self> {
                match Self::try_from(n) {
                    Ok(n) => start.unchecked_sub(n),
                    Err(_) => None,
                }
            }
        }
    };
}
