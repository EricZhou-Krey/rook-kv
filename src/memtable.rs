use std::borrow::Borrow;
use std::marker::PhantomData;
use std::ops::Bound;
use std::sync::atomic::AtomicUsize;

#[derive(Debug, Clone, PartialEq)]
pub enum Slot<V> {
    Value(V),
    Tombstone,
}

pub trait SizeOf {
    fn size_of(&self) -> usize;
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct SkipList<K, V> {
    // TODO: head node / tower of forward pointers
    // TODO: current max level
    // TODO: element count
    // TODO: RNG state for level selection
    _marker: PhantomData<(K, V)>,
}

pub struct SkipListIter<'a, K, V> {
    // TODO: current node pointer, upper bound
    _marker: PhantomData<&'a (K, V)>,
}

impl<K: Ord, V> SkipList<K, V> {
    pub fn new() -> Self {
        todo!()
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        todo!()
    }

    pub fn get<Q>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        todo!()
    }

    pub fn contains_key<Q>(&self, key: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        todo!()
    }

    pub fn len(&self) -> usize {
        todo!()
    }

    pub fn is_empty(&self) -> bool {
        todo!()
    }

    pub fn iter(&self) -> SkipListIter<'_, K, V> {
        todo!()
    }

    pub fn range<Q>(&self, lower: Bound<&Q>, upper: Bound<&Q>) -> SkipListIter<'_, K, V>
    where
        K: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        todo!()
    }

    fn random_level(&mut self) -> usize {
        todo!()
    }

    fn find_predecessors<Q>(&self, key: &Q)
    // -> [NodeRef; SIZE]
    where
        K: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        todo!()
    }

    fn alloc_node(&mut self, key: K, value: V, height: usize) // -> NodeRef
    {
        todo!()
    }

    fn free_node(node: usize /*NodeRef*/) {
        todo!()
    }
}

impl<K, V> Drop for SkipList<K, V> {
    fn drop(&mut self) {
        todo!()
    }
}

impl<'a, K, V> Iterator for SkipListIter<'a, K, V> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct MemTable<K, V> {
    // TODO: id: usize
    // TODO: map: SkipList<K, Slot<V>>
    // TODO: approximate_size: AtomicUsize
    _marker: PhantomData<(K, V)>,
}

impl<K, V> MemTable<K, V>
where
    K: Ord + SizeOf,
    V: SizeOf,
{
    pub fn create(id: usize) -> Self {
        todo!()
    }

    pub fn id(&self) -> usize {
        todo!()
    }

    pub fn put(&self, key: K, value: V) {
        todo!()
    }

    pub fn delete(&self, key: K) {
        todo!()
    }

    pub fn get<Q>(&self, key: &Q) -> Option<Slot<V>>
    where
        K: Borrow<Q>,
        Q: Ord + ?Sized,
        V: Clone,
    {
        todo!()
    }

    pub fn scan<Q>(&self, lower: Bound<&Q>, upper: Bound<&Q>) -> impl Iterator<Item = (K, Slot<V>)>
    where
        K: Borrow<Q> + Clone,
        Q: Ord + ?Sized,
        V: Clone,
    {
        todo!();
        #[allow(unreachable_code)]
        std::iter::empty()
    }

    pub fn iter_all(&self) -> impl Iterator<Item = (K, Slot<V>)>
    where
        K: Clone,
        V: Clone,
    {
        todo!();
        #[allow(unreachable_code)]
        std::iter::empty()
    }

    pub fn approximate_size(&self) -> usize {
        todo!()
    }

    pub fn is_empty(&self) -> bool {
        todo!()
    }
}
