use std::marker::PhantomData;

use crate::keep_map_tuple::KeepMapTuple;
use crate::tuple_family::{RunTuple, TupleFamily};

#[derive(Clone)]
pub struct KeepTupleMap<I, F, ExtraDim> {
    pub(crate) iter: I,
    f: F,
    unused: PhantomData<ExtraDim>,
}

impl<I, F, A, ExtraDim> Iterator for KeepTupleMap<I, F, ExtraDim>
where
    I: Iterator<Item = A>,
    F: FnMut(&I::Item) -> ExtraDim,
    A: TupleFamily<ExtraDim>,
{
    type Item = RunTuple<A, ExtraDim>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().keep_tuple_map(&mut self.f)
    }
}

pub trait KeepMapTupleIteratorExtension: Iterator {
    fn keep_tuple_map<F, B, ExtraDim>(self, f: F) -> KeepTupleMap<Self, F, ExtraDim>
    where
        F: FnMut(&Self::Item) -> B,
        Self: Sized,
    {
        KeepTupleMap {
            iter: self,
            f,
            unused: PhantomData::default(),
        }
    }
}

impl<I: Iterator> KeepMapTupleIteratorExtension for I {}
