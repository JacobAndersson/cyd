use evmap_derive::ShallowCopy;


#[derive(PartialEq, Copy, Clone, Hash, ShallowCopy)]
pub enum EntryFlag {
    Exact,
    LowerBound,
    UpperBound,
}
