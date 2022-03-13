# Multitrait Object [![](https://img.shields.io/crates/v/multi-trait-object)](https://crates.io/crates/multi-trait-object) [![](https://img.shields.io/docsrs/multi-trait-object)](https://docs.rs/multi-trait-object)

This crate provides a pointer type that allows casting into
all registered traits for a given type.
This is done by storing the pointer to the v-table for each
trait implementation on the type as well as the pointer to the
data.

## Safety

All unsafe parts are perfectly safe as far as my understanding goes.
As this crate is still in an early stage there might be some side effects that haven't
been noticed yet.

## Usage

```rust
use multi_trait_object::*;
use std::fmt::Debug;

#[derive(Debug)]
struct MyStruct {
     a: u64,
}

trait MyTrait {}
trait MyOtherTrait {}

impl MyTrait for MyStruct{}
impl MyOtherTrait for MyStruct {}

impl_trait_object!(MyStruct, dyn MyTrait, dyn MyOtherTrait, dyn Debug);

fn main() {
    let obj = MyStruct {
        a: 5
    };

    let mto = obj.into_multitrait();

    {
        let debug = mto.downcast_trait::<dyn Debug>().unwrap();
        println!("{:?}", debug);
        let my_trait = mto.downcast_trait::<dyn MyTrait>().unwrap();
    }
    
    let trait_box: Box<dyn MyTrait> = mto.downcast_trait_boxed::<dyn MyTrait>().unwrap();    
}
```

## License

Apache-2.0