//! NOT implementation.
//!
//! Will be automatically included when incluing `filter::Filter`, so importing this module
//! shouldn't be necessary.
//!
use filter::Filter;

#[must_use = "filters are lazy and do nothing unless consumed"]
#[derive(Clone)]
pub struct Not<T> {
    a: T
}

impl<T> Not<T> {

    pub fn new(a: T) -> Not<T> {
        Not { a: a }
    }

}

#[cfg(not(feature = "unstable-filter-as-fn"))]
impl<I, T: Filter<I>> Filter<I> for Not<T> {

    fn filter(&self, e: &I) -> bool {
        !self.a.filter(e)
    }

}

#[cfg(feature = "unstable-filter-as-fn")]
impl<'a, T, I> FnOnce<(&'a I,)> for Not<T>
    where T: Filter<I>,
{
    type Output = bool;
    extern "rust-call" fn call_once(self, (arg,): (&'a I,)) -> Self::Output
    {
        (self)(arg)
    }
}

#[cfg(feature = "unstable-filter-as-fn")]
impl<'a, T, I> FnMut<(&'a I,)> for Not<T>
    where T: Filter<I>,
{
    extern "rust-call" fn call_mut(&mut self, (arg,): (&'a I,)) -> Self::Output
    {
        (self)(arg)
    }
}

#[cfg(feature = "unstable-filter-as-fn")]
impl<'a, T, I> Fn<(&'a I,)> for Not<T>
    where T: Filter<I>,
{
    extern "rust-call" fn call(&self, (arg,): (&'a I,)) -> Self::Output
    {
        !self.a.filter(arg)
    }
}

#[cfg(feature="unstable-filter-as-fn")]
#[test]
fn fn_not() {
    let not = Not::new(|&x: &usize| x > 3);
    for i in 0..10 {
        assert_eq!(not.filter(&i), not(&i))
    }
}
