[![Build Status](https://travis-ci.org/kvark/simplecs.png?branch=master)](https://travis-ci.org/kvark/simplecs)

Simple Entity-Component System in Rust:
  - minimal functionality
  - perfect data representation
  - no unsafe code

Crates:
  - `id`: provides `Id` and `Array` helpers for safer addressing
  - `ecs`: simple macro-generated ECS
  - `secs` syntax extension (struct decorator) doing the same but properly handing visibility and generics
