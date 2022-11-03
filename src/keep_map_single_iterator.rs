use crate::keep_map_single::KeepMapSingle;
use crate::tuple_family::RunSingle;

#[derive(Clone)]
pub struct KeepMap<I, F> {
    pub(crate) iter: I,
    f: F,
}

impl<B, I: Iterator, F> Iterator for KeepMap<I, F>
where
    F: FnMut(&I::Item) -> B,
{
    type Item = RunSingle<I::Item, B>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().keep_map(&mut self.f)
    }
}

pub trait KeepMapSingleIteratorExtension: Iterator {
    fn keep_map<F, B>(self, f: F) -> KeepMap<Self, F>
    where
        F: FnMut(&Self::Item) -> B,
        Self: Sized,
    {
        KeepMap { iter: self, f }
    }
}

impl<I: Iterator> KeepMapSingleIteratorExtension for I {}
