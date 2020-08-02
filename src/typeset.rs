use std::any::{Any, TypeId};
use std::collections::HashMap;

#[derive(Default)]
pub struct TypeSet {
    entries: HashMap<TypeId, Box<dyn Any>>,
}

impl TypeSet {
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
        }
    }

    pub fn get<T: Any>(&self) -> Option<&T> {
        let key = TypeId::of::<T>();
        self.entries.get(&key)?.downcast_ref::<T>()
    }

    pub fn get_mut<T: Any>(&mut self) -> Option<&mut T> {
        let key = TypeId::of::<T>();
        self.entries.get_mut(&key)?.downcast_mut::<T>()
    }

    pub fn insert<T: Any>(&mut self, value: T) -> &mut T {
        let key = TypeId::of::<T>();
        self.entries.insert(key, Box::new(value));
        self.entries
            .get_mut(&key)
            .unwrap()
            .downcast_mut::<T>()
            .unwrap()
    }

    pub fn update<T: Any, F>(&mut self, f: F)
    where
        F: FnOnce(&mut T),
    {
        self.entries
            .entry(TypeId::of::<T>())
            .and_modify(|b| f(b.downcast_mut().expect("unable to cast")));
    }

    pub fn ensure<T: Any, F>(&mut self, f: F) -> &mut T
    where
        F: FnOnce() -> T,
    {
        self.entries
            .entry(TypeId::of::<T>())
            .or_insert_with(|| Box::new(f()))
            .downcast_mut()
            .unwrap()
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
    fn it_can_insert_and_get() {
        let mut sut = TypeSet::new();

        sut.insert(sample(1));

        let item_ref = sut.get::<MyEntry>();

        assert_eq!(item_ref, Some(&sample(1)));
    }

    #[test]
    fn it_cannot_get_when_not_stored() {
        let sut = TypeSet::new();

        let item_ref = sut.get::<MyEntry>();

        assert_eq!(item_ref, None);
    }

    #[test]
    fn it_can_update() {
        let mut sut = TypeSet::new();

        sut.insert(sample(1));
        sut.update(|mr| *mr = sample(2));

        assert_eq!(sut.get::<MyEntry>(), Some(&sample(2)));
    }

    #[test]
    fn it_can_ensure() {
        let mut sut = TypeSet::new();

        sut.ensure(|| sample(1));
        assert_eq!(sut.get::<MyEntry>(), Some(&sample(1)));

        sut.ensure(|| sample(2));
        assert_eq!(sut.get::<MyEntry>(), Some(&sample(1)));
    }

    #[test]
    fn it_can_be_used_as_cache() {
        use std::cell::Cell;

        let mut sut = TypeSet::new();

        let entry = sut.ensure(|| Cell::new(sample(2)));
        entry.set(sample(1));

        assert_eq!(
            sut.get::<Cell<MyEntry>>().map(|c| c.take()),
            Some(sample(1))
        );
    }
}
