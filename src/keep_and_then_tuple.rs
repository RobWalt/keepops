use crate::tuple_family::{RunTuple, TupleFamily};

pub trait KeepAndThenTuple<A, ExtraDim>
where
    A: TupleFamily<ExtraDim>,
{
    type SelfType<T>;
    fn keep_tuple_and_then<F>(self, f: F) -> Self::SelfType<RunTuple<A, ExtraDim>>
    where
        F: FnOnce(&A) -> Self::SelfType<ExtraDim>;
}

impl<Tuple, ExtraDim> KeepAndThenTuple<Tuple, ExtraDim> for Option<Tuple>
where
    Tuple: TupleFamily<ExtraDim>,
{
    type SelfType<T> = Option<T>;
    fn keep_tuple_and_then<F>(self, f: F) -> Self::SelfType<RunTuple<Tuple, ExtraDim>>
    where
        F: FnOnce(&Tuple) -> Self::SelfType<ExtraDim>,
    {
        self.and_then(|tuple| {
            let maybe_extra_dim = f(&tuple);
            maybe_extra_dim.map(|extra_dim| tuple.append_to_tuple(extra_dim))
        })
    }
}

impl<Tuple, ExtraDim, ErrorType> KeepAndThenTuple<Tuple, ExtraDim> for Result<Tuple, ErrorType>
where
    Tuple: TupleFamily<ExtraDim>,
{
    type SelfType<T> = Result<T, ErrorType>;
    fn keep_tuple_and_then<F>(self, f: F) -> Self::SelfType<RunTuple<Tuple, ExtraDim>>
    where
        F: FnOnce(&Tuple) -> Self::SelfType<ExtraDim>,
    {
        self.and_then(|tuple| {
            let maybe_extra_dim = f(&tuple);
            maybe_extra_dim.map(|extra_dim| tuple.append_to_tuple(extra_dim))
        })
    }
}
