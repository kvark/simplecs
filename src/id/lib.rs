//! ID type

use std::{fmt, slice};
use std::cmp::Ordering;
use std::marker::PhantomData;

type IdType = u32;

// Deriving forces T to have the same properties, we can't afford that.
//#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Id<S>(IdType, PhantomData<S>);

impl<S> Copy for Id<S> {}

impl<S> Clone for Id<S> {
    fn clone(&self) -> Id<S> {
        Id(self.0, PhantomData)
    }
}

impl<S> Eq for Id<S> {}

impl<S> Ord for Id<S> {
    fn cmp(&self, other: &Id<S>) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl<S> PartialEq for Id<S> {
    fn eq(&self, other: &Id<S>) -> bool {
        self.0 == other.0
    }
}

impl<S> PartialOrd for Id<S> {
    fn partial_cmp(&self, other: &Id<S>) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl<S> fmt::Debug for Id<S> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Id({})", self.0)
    }
}


/// A wrapper around `Vec` that indexes with Id types
#[derive(Clone, Debug)]
pub struct Array<T>(Vec<T>);

impl<T> Array<T> {
    pub fn new() -> Array<T> {
        Array(Vec::new())
    }

    pub fn add(&mut self, value: T) -> Id<T> {
        self.0.push(value);
        Id(self.0.len() as IdType - 1, PhantomData)
    }

    pub fn get(&self, Id(i, _): Id<T>) -> &T {
        &self.0[i as usize]
    }

    pub fn get_mut(&mut self, Id(i, _): Id<T>) -> &mut T {
        self.0.get_mut(i as usize).unwrap()
    }

    pub fn find_id<F: Fn(&T,) -> bool>(&self, fun: F) -> Option<Id<T>> {
        self.0.iter().position(fun).map(|i| Id(i as IdType, PhantomData))
    }

    pub fn iter<'a>(&'a self) -> slice::Iter<'a, T> {
        self.0.iter()
    }

    pub fn mut_iter<'a>(&'a mut self) -> slice::IterMut<'a, T> {
        self.0.iter_mut()
    }
}
