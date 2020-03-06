#![no_std]

extern crate alloc;

use alloc::alloc::Layout;
use core::{
    cmp, fmt,
    marker::PhantomData,
    mem::{self, ManuallyDrop},
    ptr,
};

#[cfg(target_pointer_width = "32")]
compile_error!("Tagged 32 bit pointers are unimplemented");

#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(transparent)]
pub struct TaggedPointer {
    ptr: usize,
}

impl TaggedPointer {
    #[inline]
    pub const fn new(ptr: usize, discriminant: u16) -> Self {
        Self {
            ptr: Self::store_discriminant(ptr, discriminant),
        }
    }

    #[inline]
    pub const fn discriminant(&self) -> u16 {
        Self::fetch_discriminant(self.ptr)
    }

    #[inline]
    pub fn as_ref<T>(&self) -> &T {
        unsafe { &*(Self::fetch_ptr(self.ptr) as *const T) }
    }

    #[inline]
    pub fn as_mut_ref<T>(&mut self) -> &mut T {
        unsafe { &mut *(Self::fetch_ptr(self.ptr) as *mut T) }
    }

    #[inline]
    #[allow(dead_code)]
    pub const fn as_ptr<T>(&self) -> *const T {
        Self::fetch_ptr(self.ptr) as *const T
    }

    #[inline]
    #[allow(dead_code)]
    pub fn as_mut_ptr<T>(&mut self) -> *mut T {
        Self::fetch_ptr(self.ptr) as *mut T
    }

    #[inline]
    const fn store_discriminant(pointer: usize, discriminant: u16) -> usize {
        const MASK: usize = !(1 << 48);

        (pointer & MASK) | ((discriminant as usize) << 56)
    }

    #[inline]
    const fn fetch_discriminant(pointer: usize) -> u16 {
        (pointer >> 56) as u16
    }

    #[inline]
    const fn fetch_ptr(pointer: usize) -> usize {
        pointer & 0xFFFF_FFFF_FFFF
    }
}

impl fmt::Pointer for TaggedPointer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Pointer::fmt(&self.as_ptr::<()>(), f)
    }
}

#[repr(transparent)]
pub struct TaggedBox<T> {
    boxed: TaggedPointer,
    _type: PhantomData<T>,
}

impl<T> TaggedBox<T> {
    #[inline]
    pub fn new<U>(val: U, discriminant: u16) -> Self {
        let ptr = if mem::size_of::<U>() == 0 {
            ptr::NonNull::dangling().as_ptr()
        } else {
            let layout = Layout::new::<U>();

            unsafe {
                let ptr = alloc::alloc::alloc(layout) as *mut U;
                assert!(ptr as usize != 0);
                ptr.write(val);

                ptr
            }
        };

        Self {
            boxed: TaggedPointer::new(ptr as usize, discriminant),
            _type: PhantomData,
        }
    }

    #[inline]
    pub fn as_ref<U>(&self) -> &U {
        self.boxed.as_ref()
    }

    #[inline]
    pub fn as_mut_ref<U>(&mut self) -> &mut U {
        self.boxed.as_mut_ref()
    }

    #[inline]
    pub fn into_inner<U>(this: Self) -> U {
        let mut this = ManuallyDrop::new(this);

        unsafe {
            let ret = this.boxed.as_mut_ptr::<U>().read();
            Self::dealloc::<U>(&mut *this);

            ret
        }
    }

    unsafe fn dealloc<U>(this: &mut Self) {
        if mem::size_of::<U>() != 0 {
            alloc::alloc::dealloc(this.boxed.as_mut_ptr(), Layout::new::<U>());
        }
    }

    #[inline]
    pub fn into_raw<U>(mut self) -> *mut U {
        self.boxed.as_mut_ptr()
    }

    #[inline]
    pub const fn discriminant(&self) -> u16 {
        self.boxed.discriminant()
    }

    #[inline]
    #[allow(dead_code)]
    pub(crate) const fn as_ptr<U>(&self) -> *const U {
        self.boxed.as_ptr() as *const U
    }

    #[inline]
    #[allow(dead_code)]
    pub(crate) fn as_mut_ptr<U>(&mut self) -> *mut U {
        self.boxed.as_mut_ptr() as *mut U
    }
}

impl<T> fmt::Debug for TaggedBox<T>
where
    T: From<TaggedBox<T>> + Into<TaggedBox<T>> + fmt::Debug + Clone,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", T::from(self.clone()))
    }
}

impl<T> fmt::Display for TaggedBox<T>
where
    T: From<TaggedBox<T>> + Into<TaggedBox<T>> + fmt::Display + Clone,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", T::from(self.clone()))
    }
}

impl<T> Clone for TaggedBox<T>
where
    T: From<TaggedBox<T>> + Into<TaggedBox<T>> + Clone,
{
    fn clone(&self) -> Self {
        T::from(TaggedBox {
            boxed: self.boxed,
            _type: PhantomData,
        })
        .clone()
        .into()
    }
}

impl<T> Copy for TaggedBox<T> where T: From<TaggedBox<T>> + Into<TaggedBox<T>> + Copy {}

impl<T> PartialEq for TaggedBox<T>
where
    T: From<TaggedBox<T>> + Into<TaggedBox<T>> + PartialEq<T> + Clone,
{
    fn eq(&self, other: &TaggedBox<T>) -> bool {
        T::from(self.clone()) == T::from(other.clone())
    }
}

impl<T> Eq for TaggedBox<T> where T: From<TaggedBox<T>> + Into<TaggedBox<T>> + Eq + Clone {}

impl<T> PartialOrd for TaggedBox<T>
where
    T: From<TaggedBox<T>> + Into<TaggedBox<T>> + PartialOrd<T> + Clone,
{
    fn partial_cmp(&self, other: &TaggedBox<T>) -> Option<cmp::Ordering> {
        T::from(self.clone()).partial_cmp(&T::from(other.clone()))
    }
}

impl<T> Ord for TaggedBox<T>
where
    T: From<TaggedBox<T>> + Into<TaggedBox<T>> + Ord + Clone,
{
    fn cmp(&self, other: &TaggedBox<T>) -> cmp::Ordering {
        T::from(self.clone()).cmp(&T::from(other.clone()))
    }
}

#[macro_export]
macro_rules! tagged_box {
    (
        $( #[$meta:meta] )*
        $struct_vis:vis struct $struct:ident, $enum_vis:vis enum $enum:ident {
            $( $field:ident($ty:ty), )*
        }
    ) => {
        $( #[$meta] )*
        #[repr(transparent)]
        $struct_vis struct $struct {
            value: $crate::TaggedBox<$enum>,
        }

        impl $struct {
            fn new<T>(value: T, discriminant: u16) -> Self {
                Self {
                    value: $crate::TaggedBox::new(value, discriminant),
                }
            }

            pub fn into_enum(self) -> $enum {
                self.into()
            }
        }

        impl Drop for $struct {
            #[allow(unused_assignments)]
            fn drop(&mut self) {
                let mut discriminant = 0;

                $(
                    if discriminant == self.value.discriminant() {
                        unsafe {
                            core::ptr::drop_in_place(self.value.as_mut_ptr::<$ty>());
                            $crate::TaggedBox::dealloc::<$ty>(&mut self.value);
                        }
                    } else {
                        discriminant += 1;
                    }
                )*

                panic!("Attempted to drop a variant that doesn't exist");
            }
        }

        $( #[$meta] )*
        $enum_vis enum $enum {
            $( $field($ty) ),*
        }

        $(
            impl From<$ty> for $struct {
                fn from(value: $ty) -> Self {
                    Self::from($enum::$field(value))
                }
            }
        )*

        impl From<$enum> for $struct {
            fn from(value: $enum) -> Self {
                Self{
                    value: value.into(),
                }
            }
        }

        impl From<$enum> for $crate::TaggedBox<$enum> {
            #[allow(unused_assignments)]
            fn from(value: $enum)->Self{
                
                let mut discriminant = 0;

                $(
                    if let $enum::$field(value) = value {
                        return $crate::TaggedBox::new(value, discriminant);
                    } else {
                        discriminant += 1;
                    }
                )*

                panic!("Attempted to construct from a variant that doesn't exist");
            }
        }

        impl From<$struct> for $enum {
            fn from(value: $struct) -> Self {
                let mut this = core::mem::ManuallyDrop::new(value);
                unsafe { (&mut this as *mut core::mem::ManuallyDrop<$struct> as *mut $enum).read() }
            }
        }

        impl From<$crate::TaggedBox<$enum>> for $enum {
            #[allow(unused_assignments)]
            fn from(value: $crate::TaggedBox<$enum>) -> $enum {
                let mut discriminant = 0;

                $(
                    if discriminant == value.discriminant() {
                        return $enum::$field($crate::TaggedBox::into_inner::<$ty>(value));
                    } else {
                        discriminant += 1;
                    }
                )*

                panic!("Attempted to construct a variant from a discriminant that doesn't exist");
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::{vec, vec::Vec};

    #[derive(Debug, Copy, Clone, PartialEq)]
    struct CustomStruct {
        int: usize,
        boolean: bool,
    }

    tagged_box! {
        #[derive(Debug, Clone, PartialEq)]
        struct Outer, enum Inner {
            Float(f32),
            Int(i32),
            Byte(u8),
            Unit(()),
            Bool(bool),
            Array([u8; 8]),
            Vector(Vec<u8>),
            CustomStruct(CustomStruct),
        }
    }

    #[test]
    fn storage() {
        assert_eq!(Outer::from(10.0f32).into_enum(), Inner::Float(10.0));
        assert_eq!(Outer::from(100i32).into_enum(), Inner::Int(100));
        assert_eq!(Outer::from(10u8).into_enum(), Inner::Byte(10));
        assert_eq!(Outer::from(()).into_enum(), Inner::Unit(()));
        assert_eq!(Outer::from(true).into_enum(), Inner::Bool(true));
        assert_eq!(Outer::from([100; 8]).into_enum(), Inner::Array([100; 8]));
        assert_eq!(
            Outer::from(vec![100; 10]).into_enum(),
            Inner::Vector(vec![100; 10])
        );
        assert_eq!(
            Outer::from(CustomStruct {
                int: 100_000,
                boolean: false
            })
            .into_enum(),
            Inner::CustomStruct(CustomStruct {
                int: 100_000,
                boolean: false
            })
        );
    }
}











