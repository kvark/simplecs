#![crate_type = "dylib"]

#[macro_export]
macro_rules! world {
    ($space:ident ($param:ty), $($name:ident : $component:ty,)*) => {
        /// A collection of pointers to components
        #[derive(Clone, Debug, Eq, PartialEq)]
        pub struct Entity {
            $(
            pub $name: Option<$space::Id<$component>>,
            )*
        }

        impl Entity {
            pub fn new() -> Entity {
                Entity {
                    $(
                    $name: None,
                    )*
                }
            }
        }

        /// A collection of component arrays
        pub struct Components {
            $(
            pub $name: $space::Array<$component>,
            )*
        }

        /// Component add_to() wrapper
        pub struct Adder<'d> {
            pub entity: Entity,
            data: &'d mut Components,
        }
        impl<'d> Adder<'d> {
            $(
            pub fn $name(mut self, value: $component) -> Adder<'d> {
                debug_assert!(self.entity.$name.is_none());
                let id = self.data.$name.add(value);
                self.entity.$name = Some(id);
                self
            }
            )*
        }

        impl Components {
            pub fn new() -> Components {
                Components {
                $(
                    $name: $space::Array::new(),
                )*
                }
            }
            pub fn add<'d>(&'d mut self) -> Adder<'d> {
                Adder {entity: Entity::new(), data: self,}
            }
        }

        /// A system responsible for some aspect (physics, rendering, etc)
        pub trait System: Send {
            fn process(&mut self, &mut $param, &mut Components, &mut Vec<Entity>);
        }

        /// A top level union of entities, their data, and systems
        pub struct World {
            pub data: Components,
            pub entities: Vec<Entity>,
            pub systems: Vec<Box<System>>,
        }

        impl World {
            pub fn new() -> World {
                World {
                    data: Components::new(),
                    entities: Vec::new(),
                    systems: Vec::new(),
                }
            }
            pub fn add_system<S: System + 'static>(&mut self, system: S) {
                self.systems.push(Box::new(system));
            }
            pub fn update(&mut self, param: &mut $param) {
                for sys in self.systems.iter_mut() {
                    sys.process(param, &mut self.data, &mut self.entities);
                }
            }
        }
    }
}
