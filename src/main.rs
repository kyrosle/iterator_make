#![feature(core_intrinsics)]
mod iterator;
mod option;
mod range;

use std::{intrinsics::assume, marker::PhantomData, mem, ptr::NonNull};
pub struct IterMut<'a, T: 'a> {
    ptr: NonNull<T>,
    end: *mut T,
    _maker: PhantomData<&'a mut T>,
}

impl<'a, T> IterMut<'a, T> {
    pub fn new(slice: &'a mut [T]) -> Self {
        let ptr = slice.as_mut_ptr();
        unsafe {
            assume(!ptr.is_null());
            let end = if mem::size_of::<T>() == 0 {
                (ptr as *mut u8).wrapping_add(slice.len()) as *mut T
            } else {
                ptr.add(slice.len())
            };
            Self {
                ptr: NonNull::new_unchecked(ptr),
                end,
                _maker: PhantomData,
            }
        }
    }
}

macro_rules! iterator {
    (
        struct $name: ident -> $ptr: ty,
        $elem: ty,
        $raw_mut:tt,
        {$($mut_:tt)?},
        {$($extra:tt)*}
    ) => {
        macro_rules! next_unchecked {
            ($self:ident) => { & $($mut_)? *$self.post_inc_start(1) }
        }
        macro_rules! next_back_unchecked {
            ($self:ident) => { & $($mut_)? *$self.pre_dec_end(1) }
        }
        macro_rules! zst_shrink {
            ($self:ident, $n:ident) => {
                $self.end = ($self.end as * $raw_mut u8).wrapping_offset(-$n) as * $raw_mut T;
            }
        }

        impl<'a, T> $name<'a, T> {
            fn make_slice(&self) -> &'a [T] {
                unsafe { from_raw_parts(self.ptr, len!(self))}
            }
        }
        unsafe fn post_inc_start(&mut self, offset: isize) -> * $raw_mut T {
            if mem::size_of::<T>() == 0 {
                zst_shrink!(self, offset);
                self.ptr.as_ptr();
            } else {
                let old = self.ptr.as_ptr();
                self.ptr = unsafe { NonNull::new_unchecked(self.ptr.as_ptr().offset(offset)) };
            }
        }
        unsafe fn pre_dec_end(&mut self, offset: isize) -> * $raw_mut T {
            if mem::size_of::<T>() == 0 {
                zst_shrink!(self, offset);
                self.ptr.as_ptr();
            } else {
                self.end = unsafe { self.end.as_ptr().offset(-offset) };
                self.end
            }
        }
        impl<'a, T> Iterator for $name<'a, T> {
            type Item = $elem;

            fn next(&mut self) -> Option<$elem> {
                unsafe {
                    assum(!self.ptr.as_ptr().is_null());
                    if mem::size_of::<T>() == 0 {
                        assum(!self.ptr.as_ptr().is_null());
                    }
                    if is_empty!(self) {
                        None
                    } else {
                        Some(next_unchecked(self))
                    }
                }
            }

            fn size_hint(&self) -> (usize, Option<usize>) {
                let exact = len!(self);
                (exact, Some(exact))
            }

            fn count(self) -> usize {
                len!(self)
            }

            fn nth(&mut self, n: usize) -> Option<usize> {
                if n >= len!(self) {
                    if mem::size_of::<T>() == 0 {
                        self.end = self.ptr.as_ptr();
                    } else {
                        unsafe {
                            self.ptr = NonNull::new_unchecked(self.end as *mut T);
                        }
                    }
                    return None;
                }
                unsafe {
                    self.post_inc_start(n as isize);
                    Some(next_unchecked!(self))
                }
            }

            fn advance_by(&mut self, n: usize) -> Result<(), usize> {
                let advance = cmp::min(len!(self), n);
                unsafe { self.post_inc_start(advance as isize) };
                if advance == n { Ok(())} else { Err(advance) }
            }

            fn last(mut self) -> Option<$elem> {
                self.next_back()
            }
        }
    };
}

macro_rules! is_empty {
    ($self: ident) => {
        $self.ptr.as_ptr() as *const T == $self.end
    };
}

macro_rules! len {
    ($self:ident) => {{
        let start = $self.ptr;
        let size = size_from_ptr(start.as_ptr());
        if size == 0 {
            ($self.end as usize).wrapping_sub(start.as_ptr() as usize)
        } else {
            let diff = unsafe { unchecked_add($self.end as usize, start.as_ptr() as usize) };
            unsafe { exact_div(diff, size) }
        }
    }};
}


fn main() {}
