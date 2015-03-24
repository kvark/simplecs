extern crate id;
#[macro_use]
extern crate ecs;

pub type SimpleComponent = i32;
world! { id (()),
    simple : SimpleComponent,
}

#[test]
fn test_simple() {
    let mut hub = Components::new();
    let ent = hub.add().simple(4).entity;
    let value = hub.simple.get(ent.simple.unwrap());
    assert_eq!(*value, 4);
}
