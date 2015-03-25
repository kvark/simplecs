extern crate id;
#[macro_use]
extern crate ecs;

pub type SimpleComponent = i32;
world! { id (()),
    simple : SimpleComponent,
}

#[test]
fn test_component() {
    let mut hub = Components::new();
    let ent = hub.add().simple(4).entity;
    let value = hub.simple.get(ent.simple.unwrap());
    assert_eq!(*value, 4);
}

struct Sys;
impl System for Sys {
	fn process(&mut self, _param: &mut (), _data: &mut Components, _entities: &mut Vec<Entity>) {}
}

#[test]
fn test_system() {
	let mut world = World::new();
	world.add_system(Sys);
	world.update(&mut ());
}
