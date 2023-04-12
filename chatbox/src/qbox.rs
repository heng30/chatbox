pub struct QBox<T> {
    v0: *const T,
    v1: *mut T,
}

impl<T> Default for QBox<T> {
    fn default() -> QBox<T> {
        QBox {
            v0: std::ptr::null::<T>(),
            v1: std::ptr::null_mut::<T>(),
        }
    }
}

#[allow(unused)]
impl<T> QBox<T> {
    pub fn new(v: &T) -> Self {
        let v0 = v as *const T;
        let v1 = v as *const T as *mut T;
        Self { v0, v1 }
    }

    pub fn is_null(&self) -> bool {
        self.v0.is_null() || self.v1.is_null()
    }

    pub fn ptr(&self) -> &*const T {
        &self.v0
    }

    pub fn ptr_mut(&self) -> &*mut T {
        &self.v1
    }

    #[allow(clippy::should_implement_trait)]
    pub fn borrow(&self) -> &T {
        assert!(!self.is_null());
        unsafe { &*self.v0 }
    }

    #[allow(clippy::should_implement_trait)]
    #[allow(clippy::mut_from_ref)]
    pub fn borrow_mut(&self) -> &mut T {
        assert!(!self.is_null());
        unsafe { &mut *self.v1 }
    }
}

unsafe impl<T> Send for QBox<T> {}
unsafe impl<T> Sync for QBox<T> {}
impl<T> Copy for QBox<T> {}

impl<T> Clone for QBox<T> {
    fn clone(&self) -> Self {
        Self {
            v0: self.v0,
            v1: self.v1,
        }
    }
}

#[allow(unused)]
pub fn qcast_to<'a, T>(ptr: usize) -> &'a T {
    let ptr = ptr as *const T;
    unsafe { &*ptr }
}

#[allow(unused)]
pub fn qcast_to_mut<'a, T>(ptr: usize) -> &'a mut T {
    let ptr = ptr as *const T as *mut T;
    unsafe { &mut *ptr }
}
