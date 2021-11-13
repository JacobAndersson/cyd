use evmap;
use crate::search::transposition_table::TtEntry;


pub struct TranspositionTable {
    reader: evmap::ReadHandle<u64, TtEntry>,
    writer: evmap::WriteHandle<u64, TtEntry>
}

impl TranspositionTable {
    pub fn new() -> TranspositionTable {
        let (r, w) = evmap::new::<u64, TtEntry>();
        TranspositionTable {
            reader: r,
            writer: w
        }
    }

    pub fn get(&self, val: &u64) -> Option<evmap::ReadGuard<TtEntry>> {
        self.reader.get_one(val)
    }

    pub fn insert(&mut self, key: u64, val: TtEntry) {
        self.writer.insert(key, val);
        self.writer.refresh();
    }

    pub fn insert_no_refresh(&mut self, key: u64, val: TtEntry) {
        self.writer.insert(key, val);
    }

    pub fn refresh(&mut self) {
        self.writer.refresh();
    }
}
