use crate::search::transposition_table::TtEntry;
use evmap;
use std::clone::Clone;
use std::sync::{Arc, Mutex};

pub struct TranspositionTable {
    reader: evmap::ReadHandle<u64, TtEntry>,
    writer: Arc<Mutex<evmap::WriteHandle<u64, TtEntry>>>,
}

impl TranspositionTable {
    pub fn new() -> TranspositionTable {
        let (r, w) = evmap::new::<u64, TtEntry>();
        TranspositionTable {
            reader: r,
            writer: Arc::new(Mutex::new(w)),
        }
    }

    pub fn get(&self, val: &u64) -> Option<evmap::ReadGuard<TtEntry>> {
        self.reader.get_one(val)
    }

    pub fn insert(&mut self, key: u64, val: TtEntry) {
        let mut writer = self.writer.lock().unwrap();
        writer.insert(key, val);
        writer.refresh();
    }

    pub fn insert_no_refresh(&mut self, key: u64, val: TtEntry) {
        let mut writer = self.writer.lock().unwrap();
        writer.insert(key, val);
    }

    pub fn refresh(&mut self) {
        let mut writer = self.writer.lock().unwrap();
        writer.refresh();
    }
}

impl Clone for TranspositionTable {
    fn clone(&self) -> Self {
        Self {
            writer: self.writer.clone(),
            reader: self.reader.clone(),
        }
    }
}
