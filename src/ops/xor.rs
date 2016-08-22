//! XOR implementation.
//!
//! Will be automatically included when incluing `filter::Filter`, so importing this module
//! shouldn't be necessary.
//!
use filter::Filter;

#[must_use = "filters are lazy and do nothing unless consumed"]
#[derive(Clone)]
pub struct XOr<T, U> {
    a: T,
    b: U
}

impl<T, U> XOr<T, U> {

    pub fn new(a: T, b: U) -> XOr<T, U> {
        XOr { a: a, b: b }
    }

}

#[cfg(not(feature = "unstable-filter-as-fn"))]
impl<I, T: Filter<I>, U: Filter<I>> Filter<I> for XOr<T, U> {

    fn filter(&self, e: &I) -> bool {
        self.a.filter(e) ^ self.b.filter(e)
    }

}

#[cfg(feature = "unstable-filter-as-fn")]
impl<'a, T, U, I> FnOnce<(&'a I,)> for XOr<T, U>
    where T: Filter<I>,
          U: Filter<I>,
{
    type Output = bool;
    extern "rust-call" fn call_once(self, (arg,): (&'a I,)) -> Self::Output
    {
        (self)(arg)
    }
}

#[cfg(feature = "unstable-filter-as-fn")]
impl<'a, T, U, I> FnMut<(&'a I,)> for XOr<T, U>
    where T: Filter<I>,
          U: Filter<I>,
{
    extern "rust-call" fn call_mut(&mut self, (arg,): (&'a I,)) -> Self::Output
    {
        (self)(arg)
    }
}

#[cfg(feature = "unstable-filter-as-fn")]
impl<'a, T, U, I> Fn<(&'a I,)> for XOr<T, U>
    where T: Filter<I>,
          U: Filter<I>,
{
    extern "rust-call" fn call(&self, (arg,): (&'a I,)) -> Self::Output
    {
        self.a.filter(arg) ^ self.b.filter(arg)
    }
}

#[cfg(feature="unstable-filter-as-fn")]
#[test]
fn fn_xor() {
    let xor = XOr::new(|&x: &usize| x > 3, |&x: &usize| x < 7);
    for i in 0..10 {
        assert_eq!(xor.filter(&i), xor(&i))
    }
}
