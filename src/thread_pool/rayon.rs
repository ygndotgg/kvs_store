use crate::Result;

pub struct RayonThreadPool {
    pool: rayon::ThreadPool,
}

impl crate::thread_pool::ThreadPool for RayonThreadPool {
    fn new(threads: u32) -> Result<Self> {
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(threads as usize)
            .build()
            .map_err(|e| failure::format_err!("Failed to create rayon thread pool:{}", e))?;
        Ok(RayonThreadPool { pool })
    }
    fn spawn<F>(&self, job: F)
    where
        F: FnOnce() + Send + 'static,
    {
        self.pool.spawn(job);
    }
}
