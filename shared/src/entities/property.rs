use std::{cell::RefCell, rc::Rc};

use super::entity_mutator::EntityMutator;

/// A Property of an Entity, that contains data which must be tracked for
/// updates, and synced to the Client
#[derive(Clone)]
pub struct Property<T: Clone> {
    mutator: Option<Rc<RefCell<dyn EntityMutator>>>,
    mutator_index: u8,
    pub(crate) inner: T,
}

impl<T: Clone> Property<T> {
    /// Create a new Property
    pub fn new(value: T, index: u8) -> Property<T> {
        return Property::<T> {
            inner: value,
            mutator_index: index,
            mutator: None,
        };
    }

    /// Gets a reference to the value contained by the Property
    pub fn get(&self) -> &T {
        return &self.inner;
    }

    /// Set the Property's contained value
    pub fn set(&mut self, value: T) {
        self.inner = value;
        if let Some(mutator) = &self.mutator {
            mutator.as_ref().borrow_mut().mutate(self.mutator_index);
        }
    }

    /// Set an EntityMutator object to track changes to the Property
    pub fn set_mutator(&mut self, mutator: &Rc<RefCell<dyn EntityMutator>>) {
        self.mutator = Some(mutator.clone());
    }
}
