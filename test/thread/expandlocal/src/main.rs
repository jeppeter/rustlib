///
///  this is the demonstration make for thread_local_key 
///  but it not run ok for it not call api key for TlsSetValue in windows or libc::pthread_setspecific 
///  other type is llvm to compile

use std::cell::{Cell, RefCell,UnsafeCell};
use std::error::Error;

pub struct LazyKeyInner2<T> {
    inner: UnsafeCell<Option<T>>,
}

unsafe impl<T> Sync for LazyKeyInner2<T>{}

impl<T> LazyKeyInner2<T> {
    pub const fn new() -> LazyKeyInner2<T> {
        LazyKeyInner2 { inner: UnsafeCell::new(None) }
    }

    pub unsafe fn get(&self) -> Option<&'static T> {
        unsafe { (*self.inner.get()).as_ref() }
    }

    pub unsafe fn initialize<F: FnOnce() -> T>(&self, init: F) -> &'static T {
        let value = init();
        let ptr = self.inner.get();

        unsafe {
            let _ = std::mem::replace(&mut *ptr, Some(value));
        }

        unsafe {
            match *ptr {
                Some(ref x) => x,
                None => panic!("unreachable"),
            }
        }
    }

    #[allow(unused)]
    pub unsafe fn take(&mut self) -> Option<T> {
        unsafe { (*self.inner.get()).take() }
    }
}


pub struct Key2<T> {
    inner: LazyKeyInner2<T>,
}

impl<T> std::fmt::Debug for Key2<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Key").finish_non_exhaustive()
    }
}
impl<T> Key2<T> {
    pub const fn new() -> Key2<T> {
        Key2 { inner: LazyKeyInner2::new()}
    }


    pub unsafe fn get<F: FnOnce() -> T>(&self, init: F) -> Option<&'static T> {
        unsafe {
            match self.inner.get() {
                Some(val) => Some(val),
                None => self.try_initialize(init),
            }
        }
    }

    #[inline(never)]
    unsafe fn try_initialize<F: FnOnce() -> T>(&self, init: F) -> Option<&'static T> {
        // SAFETY: See comment above (this function doc).
        if !std::mem::needs_drop::<T>()  {
            // SAFETY: See comment above (this function doc).
            Some(unsafe { self.inner.initialize(init) })
        } else {
            None
        }
    }

}


#[derive(Clone, Copy, Eq, PartialEq)]
pub struct AccessError2;

impl std::fmt::Debug for AccessError2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AccessError2").finish()
    }
}

impl std::fmt::Display for AccessError2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt("already destroyed", f)
    }
}

impl Error for AccessError2 {}


pub struct LocalKey2<T: 'static> {
    inner: unsafe fn(Option<&mut Option<T>>) -> Option<&'static T>,
}

impl<T: 'static> std::fmt::Debug for LocalKey2<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LocalKey2").finish_non_exhaustive()
    }
}




impl<T: 'static> LocalKey2<T> {
    pub const unsafe fn new(
        inner: unsafe fn(Option<&mut Option<T>>) -> Option<&'static T>,
        ) -> LocalKey2<T> {
        LocalKey2 { inner }
    }

    pub fn with<F, R>(&'static self, f: F) -> R
    where
    F: FnOnce(&T) -> R,
    {
        self.try_with(f).expect(
            "cannot access a Thread Local Storage value \
            during or after destruction",
            )
    }

    #[inline]
    pub fn try_with<F, R>(&'static self, f: F) -> Result<R, AccessError2>
    where
    F: FnOnce(&T) -> R,
    {
        unsafe {
            let thread_local = (self.inner)(None).ok_or(AccessError2)?;
            Ok(f(thread_local))
        }
    }

    fn initialize_with<F, R>(&'static self, init: T, f: F) -> R
    where
    F: FnOnce(Option<T>, &T) -> R,
    {
        unsafe {
            let mut init = Some(init);
            let reference = (self.inner)(Some(&mut init)).expect(
                "cannot access a Thread Local Storage value \
                during or after destruction",
                );
            f(init, reference)
        }
    }
}

impl<T: 'static> LocalKey2<Cell<T>> {
    pub fn set(&'static self, value: T) {
        self.initialize_with(Cell::new(value), |value, cell| {
            if let Some(value) = value {
                cell.set(value.into_inner());
            }
        });
    }

    pub fn get(&'static self) -> T
    where
    T: Copy,
    {
        self.with(|cell| cell.get())
    }

    pub fn take(&'static self) -> T
    where
    T: Default,
    {
        self.with(|cell| cell.take())
    }

    pub fn replace(&'static self, value: T) -> T {
        self.with(|cell| cell.replace(value))
    }
}

impl<T: 'static> LocalKey2<RefCell<T>> {
    pub fn with_borrow<F, R>(&'static self, f: F) -> R
    where
    F: FnOnce(&T) -> R,
    {
        self.with(|cell| f(&cell.borrow()))
    }

    pub fn with_borrow_mut<F, R>(&'static self, f: F) -> R
    where
    F: FnOnce(&mut T) -> R,
    {
        self.with(|cell| f(&mut cell.borrow_mut()))
    }

    pub fn set(&'static self, value: T) {
        self.initialize_with(RefCell::new(value), |value, cell| {
            if let Some(value) = value {
                *cell.borrow_mut() = value.into_inner();
            }
        });
    }

    pub fn take(&'static self) -> T
    where
    T: Default,
    {
        self.with(|cell| cell.take())
    }
    pub fn replace(&'static self, value: T) -> T {
        self.with(|cell| cell.replace(value))
    }
}

/****************************************
this is not right will need 
****************************************/

pub const FOO: LocalKey2<Cell<u32>> = {
    #[inline]
    fn __init() -> Cell<u32> {
        Cell::new(1)
    }
    unsafe fn __getit(
        init: ::std::option::Option<&mut ::std::option::Option<Cell<u32>>>) -> ::std::option::Option<&'static Cell<u32>> {
        static __KEY: Key2<Cell<u32>> = Key2::new();
        unsafe {
            __KEY.get(move || {
                if let Option::Some(init) = init {
                    if let Option::Some(value) = init.take() {
                        return value;
                    }
                }
                __init()
            })
        }
    }
    unsafe { LocalKey2::new(__getit) }
};
const BAR: LocalKey2<RefCell<Vec<f32>>> = {
    #[inline]
    fn __init() -> RefCell<Vec<f32>> {
        RefCell::new(vec![1.0, 2.0])
    }
    unsafe fn __getit(
        init: ::std::option::Option<&mut ::std::option::Option<RefCell<Vec<f32>>>>,
        ) -> ::std::option::Option<&'static RefCell<Vec<f32>>> {
        static __KEY: Key2<RefCell<Vec<f32>>> = Key2::new();
        unsafe {
            __KEY
            .get(move || {
                if let ::std::option::Option::Some(init) = init {
                    if let ::std::option::Option::Some(value) = init.take() {
                        return value;
                    } else if false {
                        {
                            panic!("internal error: entered unreachable code: {0}","missing default value");
                        };
                    }
                }
                __init()
            })
        }
    }
    unsafe { LocalKey2::new(__getit) }
};
fn main() {
    println!("FOO {}", FOO.get());
    FOO.set(5);
    println!("FOO aganin {}", FOO.get());
    let c = std::thread::spawn(move || {
        {
            println!("THREAD FOO {}", FOO.get());
        };
    });
    let _ = c.join();
    return;
}

/*
#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use std::cell::{Cell, RefCell};
pub const FOO: ::std::thread::LocalKey<Cell<u32>> = {
    #[inline]
    fn __init() -> Cell<u32> {
        Cell::new(1)
    }
    unsafe fn __getit(
        init: ::std::option::Option<&mut ::std::option::Option<Cell<u32>>>,
    ) -> ::std::option::Option<&'static Cell<u32>> {
        static __KEY: ::std::thread::local_impl::Key<Cell<u32>> = ::std::thread::local_impl::Key::new();
        unsafe {
            __KEY
                .get(move || {
                    if let ::std::option::Option::Some(init) = init {
                        if let ::std::option::Option::Some(value) = init.take() {
                            return value;
                        } else if false {
                            {
                                ::core::panicking::panic_fmt(
                                    format_args!(
                                        "internal error: entered unreachable code: {0}",
                                        format_args!("missing default value"),
                                    ),
                                );
                            };
                        }
                    }
                    __init()
                })
        }
    }
    unsafe { ::std::thread::LocalKey::new(__getit) }
};
const BAR: ::std::thread::LocalKey<RefCell<Vec<f32>>> = {
    #[inline]
    fn __init() -> RefCell<Vec<f32>> {
        RefCell::new(<[_]>::into_vec(#[rustc_box] ::alloc::boxed::Box::new([1.0, 2.0])))
    }
    unsafe fn __getit(
        init: ::std::option::Option<&mut ::std::option::Option<RefCell<Vec<f32>>>>,
    ) -> ::std::option::Option<&'static RefCell<Vec<f32>>> {
        static __KEY: ::std::thread::local_impl::Key<RefCell<Vec<f32>>> = ::std::thread::local_impl::Key::new();
        unsafe {
            __KEY
                .get(move || {
                    if let ::std::option::Option::Some(init) = init {
                        if let ::std::option::Option::Some(value) = init.take() {
                            return value;
                        } else if false {
                            {
                                ::core::panicking::panic_fmt(
                                    format_args!(
                                        "internal error: entered unreachable code: {0}",
                                        format_args!("missing default value"),
                                    ),
                                );
                            };
                        }
                    }
                    __init()
                })
        }
    }
    unsafe { ::std::thread::LocalKey::new(__getit) }
};
fn main() {
    {
        ::std::io::_print(format_args!("FOO {0}\n", FOO.get()));
    };
    FOO.set(5);
    {
        ::std::io::_print(format_args!("FOO again {0}\n", FOO.get()));
    };
    let c = std::thread::spawn(move || {
        {
            ::std::io::_print(format_args!("THREAD FOO {0}\n", FOO.get()));
        };
    });
    let _ = c.join();
    return;
}
*/