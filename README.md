# Keepops 

The library is pronounced "kee-pops". It is a utility library for working with
the functional-programming-esque features of rust and tuples.

# Use case 

Have you ever found yourself writing rust code like this?

```rust
fn some_fallible_function() -> Option<usize> { Some(0) }
fn some_other_fallible_function(x: usize) -> Option<usize> { Some(0) }
fn double(a: usize) -> usize { 2 * a }

some_fallible_function().map(|input| (input, double(input)));

some_fallible_function().and_then(|input| some_other_fallible_function(input).map(|result| (input, result)));

vec![1,2,3].into_iter().map(|input| (input, double(input)))
```

You: "Well, sometimes. This explicit input-to-result-tuple-passing can be a bit annoying, but it is managable :)"

Then what about this?

```rust
fn some_function_with_many_outputs() -> (usize, usize, usize) { (0, 0, 0) }
fn some_fallible_function() -> Option<usize> { Some(0) }

some_fallible_function().map(|(a,b,c)| (a, b, c, a + b + c))
```

"Yeah, ok. I see your point, but I don't think it's too ba.."

... then what about this slightly more realistic example?!?

```rust
pub struct Apple;
pub struct Banana;
pub struct Chocolate;
pub struct AppleBananaChocolateCakeRecipe;
pub struct AppleBananaChocolateCake;

fn try_gather_fruits() -> Option<(Apple, Banana)> {
  Some((Apple, Banana)) 
}
fn get_some_chocolate_and_hold_onto(apple: &Apple, banana: &Banana) -> Chocolate {
  Chocolate 
}
fn search_for_recipe_in_book_shelf_while_carrying(
  apple: &Apple,
  banana: &Banana,
  chocolate: &Chocolate
) -> Option<AppleBananaChocolateCakeRecipe> {
  Some(AppleBananaChocolateCakeRecipe) 
}
fn bake_cake(
  apple: Apple,
  banana: Banana,
  chocolate: Chocolate,
  recipe: &AppleBananaChocolateCakeRecipe
  ) -> AppleBananaChocolateCake {
  AppleBananaChocolateCake
}

try_gather_fruits()
  .map(|(apple, banana)| {
    let chocolate = get_some_chocolate_and_hold_onto(&apple, &banana);
    (apple, banana, chocolate)
  })
  .and_then(|(apple, banana, chocolate)| {
    search_for_recipe_in_book_shelf_while_carrying(&apple, &banana, &chocolate)
      .map(|recipe| (apple, banana, chocolate, recipe))
  })
  .map(|(apple, banana, chocolate, recipe)| bake_cake(apple, banana, chocolate, &recipe))
```

" ... "

As you can see, sometimes mapping can get a bit messy and verbose. But there is
a solution: We can trade in some of that explicit input passing against some
implicit input-under-the-hood-threading. With `keepops` the example above would
look like this.

```rust

try_gather_fruits()
  .keep_tuple_map(|(apple, banana)| get_some_chocolate_and_hold_onto(apple, banana))
  .keep_tuple_and_then(|(apple, banana, chocolate)| search_for_recipe_in_book_shelf_while_carrying(apple, banana, chocolate))
  .map(|(apple, banana, chocolate, recipe)| bake_cake(apple, banana, chocolate, &recipe))

```

Much smoother, am I right? 

"Well, but th..."

Shhhhhh, I know, I know. It's not perfect, but it reduces repetition and that's
the whole point of it. You don't have to use it if you don't want to.

# Limitations 

- Like most tuple based functionality in libraries, the function only work for
  tuples with finite arity. Most libraries choose an arity of 8 so I went for
  an arity 16 just for fun.
- The library uses the unstable-but-soon-stable feature of ***Generic Associated Types***
- Be aware that the functions probably come with some overhead of some sort
