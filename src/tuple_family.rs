pub trait TupleFamily<ExtraDim> {
    type OutTuple;
    fn append_to_tuple(self, extra: ExtraDim) -> Self::OutTuple;
}

pub type RunTuple<Tuple, ExtraDim> = <Tuple as TupleFamily<ExtraDim>>::OutTuple;
pub type RunSingle<SingleType, ExtraDim> = (SingleType, ExtraDim);

macro_rules! tuple_impls {
    ( $head:ident, $( $tail:ident ),* ) => {
        impl<$( $tail, )*$head> TupleFamily<$head> for ($( $tail, )*)
        {
            type OutTuple = ($( $tail ),*,$head);
            #[allow(non_snake_case)]
            fn append_to_tuple(self, extra_dim: $head) -> Self::OutTuple {
                let ($( $tail, )*) = self;
                ($( $tail, )* extra_dim)
            }
            // interesting delegation here, as needed
        }

        tuple_impls!($( $tail ),*);
    };

    ($head:ident) => {};
}

tuple_impls!(A, B, C, D, E, F, G, H, I, J, K, L, M, O, P, Q);
