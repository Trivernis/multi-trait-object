use std::fmt::Debug;
use crate::{__fat_pointer, impl_trait_object, IntoMultitrait, TryClone, TryPartialEq, RawClone, PartialEqAny, create_object};

#[derive(Clone, Debug, Eq, PartialEq)]
struct TestStruct {
    a: u32,
    test: String,
}

impl Default for TestStruct {
    fn default() -> Self {
        Self {
            a: 5,
            test: String::from("Hello World"),
        }
    }
}

trait ChangeStruct {
    fn update(&mut self);
}

impl ChangeStruct for TestStruct {
    fn update(&mut self) {
        self.a = 6;
    }
}

impl_trait_object!(TestStruct, dyn RawClone, dyn PartialEqAny, dyn ChangeStruct, dyn Debug);

#[test]
fn it_creates_fat_pointers() {
    let debug_vtable1 = __fat_pointer!(TestStruct as dyn Debug).vptr;
    let dclone_vtable1 = __fat_pointer!(TestStruct as dyn RawClone).vptr;
    let debug_vtable2 = __fat_pointer!(TestStruct as dyn Debug).vptr;
    assert_eq!(debug_vtable1, debug_vtable2);
    let dclone_vtable2 = __fat_pointer!(TestStruct as dyn RawClone).vptr;
    assert_eq!(dclone_vtable1, dclone_vtable2);
}

#[test]
fn it_constructs() {
    TestStruct::default().into_multitrait();
    String::from("test").into_multitrait();
    let mto = create_object!(String::from("test"), dyn Debug);
    assert!(mto.is::<String>())
}

#[test]
fn it_downcasts_traits() {
    let mto = TestStruct::default().into_multitrait();
    let debug = mto.downcast_trait::<dyn Debug>().unwrap();
    let _ = format!("{:?}", debug);
}

#[test]
fn it_downcasts_trait_mutable() {
    let mut mto = TestStruct::default().into_multitrait();
    let change_struct = mto.downcast_trait_mut::<dyn ChangeStruct>().unwrap();
    change_struct.update();
}

#[test]
fn it_downcasts_boxed_traits() {
    let mto = TestStruct::default().into_multitrait();
    let boxed = mto.downcast_trait_boxed::<dyn Debug>().unwrap();
    let _ = format!("{:?}", boxed);
}

#[test]
fn it_downcasts_to_original() {
    let mut mto = TestStruct::default().into_multitrait();
    {
        mto.downcast_ref::<TestStruct>().unwrap();
    }
    {
        mto.downcast_mut::<TestStruct>().unwrap();
    }
    let result = mto.downcast::<TestStruct>().unwrap();
    assert_eq!(result.a, 5);
    assert_eq!(result.test, String::from("Hello World"));
}

#[test]
fn it_tries_cloning() {
    let mto = TestStruct::default().into_multitrait();
    let mto2 = mto.try_clone().unwrap();
    let obj1 = mto.downcast::<TestStruct>();
    let obj2 = mto2.downcast::<TestStruct>();
    assert_eq!(obj1, obj2);
}

#[test]
fn it_returns_type_information() {
    let mto = TestStruct::default().into_multitrait();
    assert!(mto.is::<TestStruct>());
    assert!(mto.implements::<dyn Debug>());
    assert!(mto.implements::<dyn ChangeStruct>());
}

#[test]
fn it_tries_partial_eq() {
    let mto = TestStruct::default().into_multitrait();
    let mto_eq = TestStruct::default().into_multitrait();
    let mto_neq = TestStruct {test: String::from("no"), a: 6}.into_multitrait();
    assert!(mto.try_eq(&mto_eq).unwrap());
    assert_eq!(mto.try_eq(&mto_neq).unwrap(), false);
}