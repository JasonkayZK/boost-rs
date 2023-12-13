//! This library provides some utility traits to make working with [`Any`] smoother.
//! This crate contains similar functionality to the `downcast` crate, but simpler,
use std::any::{Any, TypeId};

/// This trait is an extension trait to [`Any`], and adds methods to retrieve a `&dyn Any`
pub trait AsAny: Any {
    fn as_any(&self) -> &dyn Any;

    fn as_any_mut(&mut self) -> &mut dyn Any;

    /// Gets the type name of `self`
    fn type_name(&self) -> TypeId;
}

impl<T: 'static> AsAny for T {
    #[inline(always)]
    fn as_any(&self) -> &dyn Any {
        self
    }

    #[inline(always)]
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    #[inline(always)]
    fn type_name(&self) -> TypeId {
        TypeId::of::<T>()
    }
}

/// This is a shim around `AaAny` to avoid some boilerplate code.
/// It is a separate trait because it is also implemented
/// on runtime polymorphic traits (which are `!Sized`).
pub trait Downcast: AsAny {
    /// Returns `true` if the boxed type is the same as `T`.
    ///
    /// Forward to the method defined on the type `Any`.
    #[inline(always)]
    fn is<T>(&self) -> bool
    where
        T: AsAny,
    {
        self.as_any().is::<T>()
    }

    /// Forward to the method defined on the type `Any`.
    #[inline(always)]
    fn downcast_ref<T>(&self) -> Option<&T>
    where
        T: AsAny,
    {
        self.as_any().downcast_ref()
    }

    /// Forward to the method defined on the type `Any`.
    #[inline(always)]
    fn downcast_mut<T>(&mut self) -> Option<&mut T>
    where
        T: AsAny,
    {
        self.as_any_mut().downcast_mut()
    }
}

impl<T: ?Sized + AsAny> Downcast for T {}

#[cfg(test)]
mod tests {
    use crate::types::as_any::{AsAny, Downcast};

    #[test]
    fn test_as_any() {
        struct Test;
        trait Custom: AsAny {
            // whatever you like to put inside of your trait
        }
        impl Custom for Test {}

        let x = Test;
        let y: &dyn Custom = &x;
        // With (extension) trait `Downcast` in scope.
        assert!((*y).downcast_ref::<Test>().is_some());
    }
}
