#![feature(custom_attribute, plugin)]
#![plugin(secs)]

use std::marker::PhantomData;
extern crate id;

#[secs(id)]
struct Proto<X: Send> {
    x: i8,
    y: PhantomData<X>,
}

#[test]
fn test_macro() {}

#[test]
fn test_world() {
    let _ = World::<i8>::new();
}
