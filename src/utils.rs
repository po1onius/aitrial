use std::sync::atomic::{AtomicU32, Ordering};

pub fn get_id() -> u32 {
    static ID: AtomicU32 = AtomicU32::new(0);
    ID.fetch_add(1, Ordering::SeqCst)
}
