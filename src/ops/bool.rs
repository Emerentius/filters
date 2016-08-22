//! Bool Filter implementation, so we can insert this in filter construction
//!
//! Will be automatically included when incluing `filter::Filter`, so importing this module
//! shouldn't be necessary.
//!
#[cfg(not(feature = "unstable-filter-as-fn"))]
use filter::Filter;

#[must_use = "filters are lazy and do nothing unless consumed"]
#[derive(Clone)]
pub struct Bool {
    b: bool
}

impl Bool {

    pub fn new(b: bool) -> Bool {
        Bool { b: b }
    }

}

#[cfg(not(feature = "unstable-filter-as-fn"))]
impl<I> Filter<I> for Bool {

    fn filter(&self, _: &I) -> bool {
        self.b
    }

}

impl From<bool> for Bool {

    fn from(b: bool) -> Bool {
        Bool::new(b)
    }

}

#[cfg(feature = "unstable-filter-as-fn")]
impl<'a, I> FnOnce<(&'a I,)> for Bool {
    type Output = bool;
    extern "rust-call" fn call_once(self, (arg,): (&'a I,)) -> Self::Output
    {
        (self)(arg)
    }
}


#[cfg(feature = "unstable-filter-as-fn")]
impl<'a, I> FnMut<(&'a I,)> for Bool {
    extern "rust-call" fn call_mut(&mut self, (arg,): (&'a I,)) -> Self::Output
    {
        (self)(arg)
    }
}

#[cfg(feature = "unstable-filter-as-fn")]
impl<'a, I> Fn<(&'a I,)> for Bool {
    extern "rust-call" fn call(&self, (_,): (&'a I,)) -> Self::Output
    {
        self.b
    }
}

#[cfg(feature="unstable-filter-as-fn")]
#[test]
fn fn_bool() {
    use filter::Filter;
    let b = Bool::new(true);
    assert_eq!(b.filter(&3), b(&27));
}
