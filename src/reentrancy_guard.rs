thread_local! {
    static REENTRANCY_GUARD: std::cell::Cell<bool> = std::cell::Cell::new(false);
}

pub struct ReentrancyGuard;

impl ReentrancyGuard {
    pub fn new() -> Option<Self> {
        REENTRANCY_GUARD.with(|guard| {
            if guard.get() {
                None
            } else {
                guard.set(true);
                Some(Self)
            }
        })
    }
}

impl Drop for ReentrancyGuard {
    fn drop(&mut self) {
        REENTRANCY_GUARD.with(|guard| guard.set(false));
    }
}
