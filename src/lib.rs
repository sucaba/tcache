pub mod typeset;

use crate::typeset::TypeSet;
use std::any::Any;
use std::cell::Cell;
use std::sync::Arc;

pub struct TypeCache {
    cell: Cell<TypeSet>,
}

impl TypeCache {
    pub fn new() -> Self {
        Self {
            cell: Cell::new(TypeSet::new()),
        }
    }

    pub fn get<T: Any>(&self) -> Option<Arc<T>> {
        let tset = self.cell.take();
        let result = tset.get::<Arc<T>>().cloned();
        self.cell.set(tset);
        result
    }

    pub fn insert_ref<T: Any>(&self, value: Arc<T>) {
        let mut tset = self.cell.take();
        tset.insert(value);
        self.cell.set(tset);
    }

    pub fn insert<T: Any>(&self, value: T) -> Arc<T> {
        let result = Arc::new(value);
        self.insert_ref(result.clone());
        result
    }

    pub fn ensure<T: Any, F>(&self, f: F) -> Arc<T>
    where
        F: FnOnce() -> T,
    {
        self.ensure_ref(|| Arc::new(f()))
    }

    pub fn ensure_ref<T: Any, F>(&self, f: F) -> Arc<T>
    where
        F: FnOnce() -> Arc<T>,
    {
        let mut tset = self.cell.take();
        let result = tset.ensure(|| f()).clone();
        self.cell.set(tset);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(PartialEq, Debug, Default)]
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
        let sut = TypeCache::new();
        let ref1 = sut.ensure(|| sample(1));
        assert_eq!(*ref1, sample(1));
        assert_eq!(sut.get::<MyEntry>().as_deref(), Some(&sample(1)));
    }

    #[test]
    fn it_should_insert_and_get() {
        let sut = TypeCache::new();
        let ref1 = sut.insert(sample(1));
        assert_eq!(*ref1, sample(1));

        let ref2 = sut.insert(sample(2));
        assert_eq!(*ref2, sample(2));
        assert_eq!(sut.get::<MyEntry>().as_deref(), Some(&sample(2)));
    }
}
