use crate::tuple_family::{RunTuple, TupleFamily};

pub trait KeepMapTuple<A, ExtraDim>
where
    A: TupleFamily<ExtraDim>,
{
    type SelfType<T>;
    fn keep_tuple_map<F>(self, f: F) -> Self::SelfType<RunTuple<A, ExtraDim>>
    where
        F: FnOnce(&A) -> ExtraDim;
}

impl<Tuple, ExtraDim> KeepMapTuple<Tuple, ExtraDim> for Option<Tuple>
where
    Tuple: TupleFamily<ExtraDim>,
{
    type SelfType<T> = Option<T>;
    fn keep_tuple_map<F>(self, f: F) -> Self::SelfType<RunTuple<Tuple, ExtraDim>>
    where
        F: FnOnce(&Tuple) -> ExtraDim,
    {
        self.map(|tuple| {
            let extra_dim = f(&tuple);
            tuple.append_to_tuple(extra_dim)
        })
    }
}

impl<Tuple, ExtraDim, ErrorType> KeepMapTuple<Tuple, ExtraDim> for Result<Tuple, ErrorType>
where
    Tuple: TupleFamily<ExtraDim>,
{
    type SelfType<T> = Result<T, ErrorType>;
    fn keep_tuple_map<F>(self, f: F) -> Self::SelfType<RunTuple<Tuple, ExtraDim>>
    where
        F: FnOnce(&Tuple) -> ExtraDim,
    {
        self.map(|tuple| {
            let extra_dim = f(&tuple);
            tuple.append_to_tuple(extra_dim)
        })
    }
}
