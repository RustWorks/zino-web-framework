use std::vec::IntoIter;

/// A static record type.
#[derive(Debug, Clone, Default)]
pub struct StaticRecord<T> {
    /// Inner container.
    inner: Vec<(&'static str, T)>,
}

impl<T> StaticRecord<T> {
    /// Creates a new instance.
    #[inline]
    pub fn new() -> Self {
        Self { inner: Vec::new() }
    }

    /// Creates a new instance with the capacity.
    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            inner: Vec::with_capacity(capacity),
        }
    }

    /// Appends an entry to the back of a collection.
    #[inline]
    pub fn add(&mut self, key: &'static str, value: T) {
        self.inner.push((key, value));
    }

    /// Searches for the key and returns its value.
    #[inline]
    pub fn find(&self, key: &str) -> Option<&T> {
        self.inner
            .iter()
            .find_map(|(field, value)| (field == &key).then_some(value))
    }

    /// Consumes `self` and returning a static reference.
    #[inline]
    pub fn leak(self) -> &'static [(&'static str, T)] {
        self.inner.leak()
    }
}

impl<T> IntoIterator for StaticRecord<T> {
    type Item = (&'static str, T);
    type IntoIter = IntoIter<Self::Item>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}
