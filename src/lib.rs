pub mod typeset;

use crate::typeset::SingletonSet;
use std::any::Any;
use std::cell::RefCell;

pub struct SingletonCache {
    cell: RefCell<SingletonSet>,
}

impl SingletonCache {
    pub fn new() -> Self {
        Self {
            cell: RefCell::new(SingletonSet::new()),
        }
    }

    pub fn get<T: Any + Clone>(&self) -> Option<T> {
        self.cell.borrow().get::<T>().cloned()
    }

    pub fn insert<T: Any>(&self, value: T) {
        self.cell.borrow_mut().insert(value);
    }

    pub fn remove<T: Any>(&self) {
        self.cell.borrow_mut().remove::<T>();
    }

    pub fn ensure<T: Any, F>(&self, f: F) -> T
    where
        F: FnOnce() -> T,
        T: Clone,
    {
        self.cell.borrow_mut().ensure(f).clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(PartialEq, Debug, Default, Clone)]
    struct MyEntry {
        pub name: String,
    }

    fn sample(n: usize) -> MyEntry {
        MyEntry {
            name: format!("cached{}", n),
        }
    }

    #[test]
    fn it_should_ensure_and_get() {
        let sut = SingletonCache::new();
        let clone1 = sut.ensure(|| sample(1));
        assert_eq!(clone1, sample(1));
        assert_eq!(sut.get::<MyEntry>(), Some(sample(1)));
    }

    #[test]
    fn it_should_insert_and_get() {
        let sut = SingletonCache::new();
        sut.insert(sample(1));
        sut.insert(sample(2));
        assert_eq!(sut.get::<MyEntry>(), Some(sample(2)));
    }
}
