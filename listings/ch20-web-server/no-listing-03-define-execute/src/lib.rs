pub struct ThreadPool;

// ANCHOR: here
impl ThreadPool {
    // --省略--
    // ANCHOR_END: here
    pub fn new(size: usize) -> ThreadPool {
        ThreadPool
    }

    // ANCHOR: here
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
    }
}
// ANCHOR_END: here
