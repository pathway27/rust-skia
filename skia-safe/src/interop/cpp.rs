use crate::prelude::safer;
use sb::TraitObject;
use skia_bindings as sb;
use std::{marker::PhantomData, mem};

/// A sink / receiver for array data that is copied from C++ to a Rust [`Vec`].
#[derive(Debug)]
pub struct VecSink<'a, T> {
    sink: sb::VecSink<T>,
    pd: PhantomData<&'a mut Vec<T>>,
}

impl<T: 'static> VecSink<'_, T> {
    /// Create a new sink that calls back into the closure given.
    pub fn new(v: &mut dyn FnMut(&[T])) -> VecSink<T> {
        VecSink {
            sink: sb::VecSink {
                fn_trait: unsafe { mem::transmute(v) },
                set_fn: Some(Self::set_fn),
                _phantom_0: PhantomData,
            },
            pd: PhantomData,
        }
    }

    #[allow(unused)]
    pub fn new_mut(v: &mut dyn FnMut(&mut [T])) -> VecSink<T> {
        VecSink {
            sink: sb::VecSink {
                fn_trait: unsafe { mem::transmute(v) },
                set_fn: Some(Self::set_fn_mut),
                _phantom_0: PhantomData,
            },
            pd: PhantomData,
        }
    }

    pub fn native_mut(&mut self) -> &mut sb::VecSink<T> {
        &mut self.sink
    }

    unsafe extern "C" fn set_fn(ptr: *mut T, len: usize, rust_fn: TraitObject) {
        let rust_fn: &mut dyn FnMut(&[T]) = mem::transmute(rust_fn);
        (rust_fn)(safer::from_raw_parts(ptr, len));
    }

    #[allow(unused)]
    unsafe extern "C" fn set_fn_mut(ptr: *mut T, len: usize, rust_fn: TraitObject) {
        let rust_fn: &mut dyn FnMut(&mut [T]) = mem::transmute(rust_fn);
        (rust_fn)(safer::from_raw_parts_mut(ptr, len));
    }
}
