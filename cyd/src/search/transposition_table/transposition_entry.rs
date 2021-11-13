use crate::search::transposition_table::EntryFlag;
use evmap::ShallowCopy;
use pleco::BitMove;
use std::hash::{Hash, Hasher};
use std::mem::ManuallyDrop;

#[derive(Clone, Copy)]
pub struct TtEntry {
    pub mv: BitMove,
    pub depth: u8,
    pub flag: EntryFlag,
    pub value: i64,
}

impl PartialEq for TtEntry {
    fn eq(&self, other: &Self) -> bool {
        self.mv == other.mv
            && self.depth == other.depth
            && self.flag == other.flag
            && self.value == other.value
    }
}

impl Eq for TtEntry {}

impl Hash for TtEntry {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.mv.get_raw().hash(state);
        self.depth.hash(state);
        self.flag.hash(state);
        self.value.hash(state);
    }
}

impl ShallowCopy for TtEntry {
    unsafe fn shallow_copy(&self) -> ManuallyDrop<Self> {
        ManuallyDrop::new(Self {
            mv: self.mv,
            depth: ManuallyDrop::into_inner(ShallowCopy::shallow_copy(&self.depth)),
            flag: ManuallyDrop::into_inner(ShallowCopy::shallow_copy(&self.flag)),
            value: ManuallyDrop::into_inner(ShallowCopy::shallow_copy(&self.value)),
        })
    }
}
