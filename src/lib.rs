#![feature(generic_associated_types)]

mod keep_and_then_single;
mod keep_and_then_tuple;
mod keep_map_single;
mod keep_map_single_iterator;
mod keep_map_tuple;
mod keep_map_tuple_iterator;
mod tuple_family;

pub mod prelude {
    pub use super::keep_and_then_single::*;
    pub use super::keep_and_then_tuple::*;
    pub use super::keep_map_single::*;
    pub use super::keep_map_single_iterator::*;
    pub use super::keep_map_tuple::*;
    pub use super::keep_map_tuple_iterator::*;
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    #[derive(Debug)]
    struct WeirdStruct;

    #[test]
    fn iterator_compiles() {
        let x = Some(1).iter().keep_map(|&x| x + 1);

        let z = x
            .map(|x| (x, 2))
            .keep_tuple_map(|&(a, b)| a.0 + b)
            .keep_map(|&(a, b, c)| a.0 + b + c)
            .next();
        println!("{z:?}");

        let z = Some(WeirdStruct).iter().keep_map(|_| 1).next();
        println!("{z:?}");
    }

    #[test]
    fn map_compiles() {
        let x = Some(1);

        let z = x
            .map(|x| (x, 2))
            .keep_tuple_map(|&(a, b)| a + b)
            .keep_map(|&(a, b, c)| a + b + c);
        println!("{z:?}");

        let z = Some(WeirdStruct).keep_map(|_| 1);
        println!("{z:?}");
    }

    #[test]
    fn and_then_compiles() {
        let x = Some(1);

        let z = x
            .map(|x| (x, 2))
            .keep_tuple_and_then(|&(a, b)| "4".parse::<usize>().ok().map(|c| a + b + c))
            .keep_and_then(|&(a, b, c)| Some(a + b + c));
        println!("{z:?}");

        let z = Some(WeirdStruct).keep_and_then(|_| Some(()));
        println!("{z:?}");
    }

    #[test]
    fn example_from_readme_because_i_not_sure_if_it_works_lol() {
        #[derive(Debug)]
        pub struct Apple;
        #[derive(Debug)]
        pub struct Banana;
        #[derive(Debug)]
        pub struct Chocolate;
        #[derive(Debug)]
        pub struct AppleBananaChocolateCakeRecipe;
        #[derive(Debug)]
        pub struct AppleBananaChocolateCake;

        fn try_gather_fruits() -> Option<(Apple, Banana)> {
            Some((Apple, Banana))
        }
        fn get_some_chocolate_and_hold_onto(_apple: &Apple, _banana: &Banana) -> Chocolate {
            Chocolate
        }
        fn search_for_recipe_in_book_shelf_while_carrying(
            _apple: &Apple,
            _banana: &Banana,
            _chocolate: &Chocolate,
        ) -> Option<AppleBananaChocolateCakeRecipe> {
            Some(AppleBananaChocolateCakeRecipe)
        }
        fn bake_cake(
            _apple: Apple,
            _banana: Banana,
            _chocolate: Chocolate,
            _recipe: &AppleBananaChocolateCakeRecipe,
        ) -> AppleBananaChocolateCake {
            AppleBananaChocolateCake
        }

        let _bad_cake = try_gather_fruits()
            .map(|(apple, banana)| {
                let chocolate = get_some_chocolate_and_hold_onto(&apple, &banana);
                (apple, banana, chocolate)
            })
            .and_then(|(apple, banana, chocolate)| {
                search_for_recipe_in_book_shelf_while_carrying(&apple, &banana, &chocolate)
                    .map(|recipe| (apple, banana, chocolate, recipe))
            })
            .map(|(apple, banana, chocolate, recipe)| bake_cake(apple, banana, chocolate, &recipe));

        let _super_yummy_cake = try_gather_fruits()
            .keep_tuple_map(|(apple, banana)| get_some_chocolate_and_hold_onto(apple, banana))
            .keep_tuple_and_then(|(apple, banana, chocolate)| {
                search_for_recipe_in_book_shelf_while_carrying(apple, banana, chocolate)
            })
            .map(|(apple, banana, chocolate, recipe)| bake_cake(apple, banana, chocolate, &recipe));
    }
}
