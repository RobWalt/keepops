use crate::tuple_family::RunSingle;

pub trait KeepMapSingle<SingleType, ExtraDim> {
    type SelfType<T>;

    fn keep_map<F>(self, f: F) -> Self::SelfType<RunSingle<SingleType, ExtraDim>>
    where
        F: FnOnce(&SingleType) -> ExtraDim;
}

impl<SingleType, ExtraDim> KeepMapSingle<SingleType, ExtraDim> for Option<SingleType> {
    type SelfType<T> = Option<T>;
    fn keep_map<F>(self, f: F) -> Self::SelfType<RunSingle<SingleType, ExtraDim>>
    where
        F: FnOnce(&SingleType) -> ExtraDim,
    {
        self.map(|single_type| {
            let extra_dim = f(&single_type);
            (single_type, extra_dim)
        })
    }
}

impl<SingleType, ExtraDim, ErrorType> KeepMapSingle<SingleType, ExtraDim>
    for Result<SingleType, ErrorType>
{
    type SelfType<T> = Result<T, ErrorType>;
    fn keep_map<F>(self, f: F) -> Self::SelfType<RunSingle<SingleType, ExtraDim>>
    where
        F: FnOnce(&SingleType) -> ExtraDim,
    {
        self.map(|single_type| {
            let extra_dim = f(&single_type);
            (single_type, extra_dim)
        })
    }
}
