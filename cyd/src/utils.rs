use crate::search::TtEntry;
use dashmap::DashMap;
use std::sync::Arc;

pub fn new_tt_table() -> Arc<DashMap<u64, TtEntry>> {
    Arc::new(DashMap::<u64, TtEntry>::new())
}
