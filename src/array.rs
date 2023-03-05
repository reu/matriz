use core::{
    fmt::Debug,
    mem::{self, MaybeUninit},
    ptr, slice,
};

#[derive(Debug)]
pub struct ArrayBuilder<T, const N: usize> {
    items: [MaybeUninit<T>; N],
    filled: usize,
}

impl<T, const N: usize> Drop for ArrayBuilder<T, N> {
    fn drop(&mut self) {
        unsafe { ptr::drop_in_place(slice::from_raw_parts_mut(self.as_mut_ptr(), self.filled)) };
        self.filled = 0;
    }
}

impl<T, const N: usize> ArrayBuilder<T, N> {
    const UNINIT: MaybeUninit<T> = MaybeUninit::uninit();

    pub fn new() -> Self {
        Self {
            items: [Self::UNINIT; N],
            filled: 0,
        }
    }

    pub fn push(&mut self, item: T) -> Result<(), T> {
        if self.filled == N {
            return Err(item);
        }
        unsafe { ptr::write(self.as_mut_ptr().add(self.filled), item) };
        self.filled += 1;
        Ok(())
    }

    pub fn build(self) -> Result<[T; N], Self> {
        if self.filled != N {
            return Err(self);
        }
        let drop = mem::ManuallyDrop::new(self);
        unsafe { Ok(ptr::read(drop.items.as_ptr() as *const [T; N])) }
    }

    fn as_mut_ptr(&mut self) -> *mut T {
        self.items.as_mut_ptr() as *mut T
    }
}
