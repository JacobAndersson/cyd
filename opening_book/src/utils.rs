use std::collections::HashMap;

pub type GameBook = HashMap<(u64, u16), (u64, bool)>;

pub type OpeningBook = HashMap<u64, (u16, bool)>;
