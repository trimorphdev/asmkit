use std::{fmt::Debug, hash::Hash, marker::PhantomData};

/// An opaque reference to an entity.
/// 
/// Entity references are intended to be created *only internally*, and using manually constructed entity references (using [`EntityRef::new`]) may cause issues if you do
/// not know what you are doing.
pub trait EntityRef: Clone + Copy + Debug + Hash + PartialEq + PartialOrd {
    /// Creates an opaque reference to an entity.
    /// 
    /// Manually creating an entity reference may cause issues if you do not know what you are doing.  Entity references are unchecked for performance reasons, and your
    /// program may panic if you create an invalid entity reference.
    fn new(value: usize) -> Self;

    /// Returns the raw ID of this entity reference.
    /// 
    /// Used internally to index an entity list.
    fn as_u32(&self) -> u32;

    /// Returns the raw ID of this entity reference, as a Rust `usize`.
    /// 
    /// Used internally to index an entity list.
    fn as_usize(&self) -> usize;
}

/// A list of entities.
/// 
/// This implementation is essentially a glorified [`Vec<T>`], which returns a `Ref` when pushed to.  Of course, since it is indexed by [`EntityRef`]s, it can only index
/// [`u32::MAX`] items.  It can hold more items than this (if there is enough space in memory), but the items past the limit will be un-indexable by [`EntityRef`]s.
#[derive(Clone, PartialEq)]
pub struct EntityList<T, Ref: EntityRef> {
    /// The raw [`Vec`] that this list wraps.
    private: Vec<T>,

    /// Phantom data so the compiler isn't mad about `Ref` being unused.
    phantom: PhantomData<Ref>,
}

impl<T, Ref: EntityRef> EntityList<T, Ref> {
    /// Creates a new empty entity list.
    #[inline(always)]
    pub fn new() -> Self {
        Self { private: Vec::new(), phantom: PhantomData }
    }

    /// Pushes an item onto the end of the entity list, and returns a reference to it.
    pub fn push(&mut self, item: T) -> Ref {
        let r = Ref::new(self.private.len());
        self.private.push(item);
        r
    }

    /// Returns the item associated with the given reference.  Will panic if `item` is an invalid reference, or out of bounds.
    pub fn get(&self, item: Ref) -> &T {
        &self.private[item.as_usize()]
    }

    /// Returns a mutable reference to the provided item.
    pub fn get_mut(&mut self, item: Ref) -> &mut T {
        &mut self.private[item.as_usize()]
    }

    /// Returns the amount of items in the entity list.
    pub fn len(&self) -> usize {
        self.private.len()
    }

    /// Returns the entities which are currently stored in this entity list.
    pub fn entities(&self) -> &Vec<T> {
        &self.private
    }

    /// Returns a mutable reference to the entities which are stored in this entity list.
    pub fn entities_mut(&mut self) ->  &mut Vec<T> {
        &mut self.private
    }
}

impl<T: Debug, Ref: EntityRef> Debug for EntityList<T, Ref> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.private.fmt(f)
    }
}

/// An opaque reference to a [`Label`] in an instruction stream.
#[derive(Clone, Copy, Debug, Hash, PartialEq, PartialOrd)]
pub struct LabelRef(u32);

impl EntityRef for LabelRef {
    #[inline(always)]
    fn new(value: usize) -> Self {
        Self(value as u32)
    }

    #[inline(always)]
    fn as_u32(&self) -> u32 {
        self.0
    }

    #[inline(always)]
    fn as_usize(&self) -> usize {
        self.0 as usize
    }
}

/// A label in an instruction stream.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Label {
    /// A label attached to the provided index.
    Attached(usize),
    
    /// A label attached to no index yet.
    Unattached,
}