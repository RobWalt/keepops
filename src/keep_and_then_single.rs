use crate::tuple_family::RunSingle;

pub trait KeepAndThenSingle<A, ExtraDim> {
    type SelfType<T>;

    fn keep_and_then<F>(self, f: F) -> Self::SelfType<RunSingle<A, ExtraDim>>
    where
        F: FnOnce(&A) -> Self::SelfType<ExtraDim>;
}

impl<SingleType, ExtraDim> KeepAndThenSingle<SingleType, ExtraDim> for Option<SingleType> {
    type SelfType<T> = Option<T>;
    fn keep_and_then<F>(self, f: F) -> Self::SelfType<RunSingle<SingleType, ExtraDim>>
    where
        F: FnOnce(&SingleType) -> Self::SelfType<ExtraDim>,
    {
        self.and_then(|single_type| {
            let maybe_extra_dim = f(&single_type);
            maybe_extra_dim.map(|extra_dim| (single_type, extra_dim))
        })
    }
}

impl<SingleType, ExtraDim, ErrorType> KeepAndThenSingle<SingleType, ExtraDim>
    for Result<SingleType, ErrorType>
{
    type SelfType<T> = Result<T, ErrorType>;
    fn keep_and_then<F>(self, f: F) -> Self::SelfType<RunSingle<SingleType, ExtraDim>>
    where
        F: FnOnce(&SingleType) -> Self::SelfType<ExtraDim>,
    {
        self.and_then(|single_type| {
            let maybe_extra_dim = f(&single_type);
            maybe_extra_dim.map(|extra_dim| (single_type, extra_dim))
        })
    }
}
