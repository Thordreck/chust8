
#[cfg(test)]
pub mod tests
{
    use lazy_static::*;
    use std::sync::Mutex;

    // This mutex is used to avoid running tests that tries
    // to call sdl2::init in parallel. Sdl2 does not allow to initialize
    // the context more than once, but rust tests are run in parallel by default.
    lazy_static!
    {
        pub static ref TEST_MUTEX: Mutex<()> = Mutex::new(());
    }

    pub fn test_lock() -> Result<std::sync::MutexGuard<'static, ()>, String>
    {
        match TEST_MUTEX.lock()
        {
            Ok(mutex) => Ok(mutex),
            Err(e)    => return Err(e.to_string()),
        }
    }
}
