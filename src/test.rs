use parking_lot::{ReentrantMutex, ReentrantMutexGuard};

use context::Context;

lazy_static! {
    pub static ref TEST_MUTEX: ReentrantMutex<()> = ReentrantMutex::new(());
}

pub fn test_ctx() -> (ReentrantMutexGuard<'static, ()>, Context) {
    let guard = TEST_MUTEX.lock();
    (guard, Context::create())
}
