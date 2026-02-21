mod naive;
mod rayon;
mod shared_queue;
use crate::Result;

pub use naive::NaiveThreadPool;
pub use rayon::RayonThreadPool;
pub use shared_queue::SharedQueueThreadPool;
pub trait ThreadPool {
    fn spawn<F>(&self, job: F)
    where
        F: FnOnce() + Send + 'static;

    fn new(threads: u32) -> Result<Self>
    where
        Self: Sized;
}
