use crate::search::transposition_table::TtEntry;
use evmap;
use fnv;

use std::clone::Clone;
use std::sync::{Arc, Mutex};
use std::hash::BuildHasherDefault;

pub struct TranspositionTable {
    reader: evmap::ReadHandle<u64, TtEntry, (), BuildHasherDefault<fnv::FnvHasher>>,
    writer: Arc<Mutex<evmap::WriteHandle<u64, TtEntry, (), BuildHasherDefault<fnv::FnvHasher>>>>,
}

impl TranspositionTable {
    pub fn new() -> TranspositionTable {
        let opt = evmap::Options::default();
        let mp = opt.with_hasher(BuildHasherDefault::<fnv::FnvHasher>::default());
        let (r, w) = mp.construct();
        TranspositionTable {
            reader: r,
            writer: Arc::new(Mutex::new(w)),
        }
    }

    pub fn get(&self, val: &u64) -> Option<evmap::ReadGuard<TtEntry>> {
        self.reader.get_one(val)
    }

    pub fn insert(&mut self, key: u64, val: TtEntry) {
        /*
        let found_val = self.get(&key);
        match found_val {
            Some(value) if value != val => {
                let mut writer = self.writer.lock().unwrap();
                writer.insert(key, val);
                writer.refresh();
            },
            None => {
                let mut writer = self.writer.lock().unwrap();
                writer.insert(key, val);
                writer.refresh();
            }
        }
        */
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

    pub fn len(&self) -> usize {
        self.reader.len()
    }
}

impl Default for TranspositionTable {
    fn default() -> Self {
        Self::new()
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
