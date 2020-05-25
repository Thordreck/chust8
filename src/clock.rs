
use std::time::{ Duration, Instant };

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Frequency { hertz: f64 }

impl Frequency
{
    pub fn new(hertz: f64) -> Self
    {
        Frequency { hertz }
    }

    pub fn period(&self) -> Duration
    {
        Duration::from_millis((1000.0 / self.hertz) as u64)
    }

    pub fn value(&self) -> f64
    {
        self.hertz
    }

    pub fn set_value(&mut self, new_value: f64)
    {
        self.hertz = new_value;
    }
}

pub const DEFAULT_CPU_FREQUENCY    : Frequency = Frequency { hertz: 500.0 };
pub const DEFAULT_TIMERS_FREQUENCY : Frequency = Frequency { hertz: 60.0 };

pub struct RateLimiter<F>
    where F: FnMut() -> Result<(), String>
{
    max_rate : Frequency,
    function : F,
    last_executed: Option<Instant>,
}

impl<F: FnMut() -> Result<(), String>> RateLimiter<F>
{
    pub fn new(function: F, max_rate: Frequency) -> Self
    {
        RateLimiter { max_rate, function, last_executed: None }
    }

    pub fn execute(&mut self) -> Result<(), String>
    {
        if self.last_executed.is_none()
        {
            self.last_executed = Some(Instant::now());
        }

        let now     = Instant::now();
        let elapsed = now.duration_since(self.last_executed.unwrap());

        if elapsed >= self.max_rate.period()
        {
            self.last_executed = Some(now);
            return (self.function)();
        }

        Ok(())
    }
}


#[cfg(test)]
mod tests
{
    use super::*;
    use std::time::SystemTime;

    #[test]
    fn frequency()
    {
        let mut frequency = Frequency::new(1.0);

        // Check that the value is correct
        assert_eq!(frequency.value(), 1.0,
                   "Frequency value was not correctly set in constructor");

        // Check period conversions
        assert_eq!(frequency.period(), Duration::from_secs(1),
                   "1 hz does not equal to a period of 1s");

        // Check setters
        frequency.set_value(30.0);
        assert_eq!(frequency.value(), 30.0,
                   "set_value() did not update value correctly");

        assert_eq!(frequency.period(), Duration::from_millis(33),
                   "15 hz does not equal to a period of 33ms");

        // Check chip8 default values
        assert_eq!(DEFAULT_CPU_FREQUENCY.value(), 500.0,
                   "Unexpected default cpu frequency value");

        assert_eq!(DEFAULT_CPU_FREQUENCY.period(), Duration::from_millis(2),
                   "Default CPU period is not equal to 2ms");

        assert_eq!(DEFAULT_TIMERS_FREQUENCY.value(), 60.0,
                   "Unexpected default timers frequency value");

        assert_eq!(DEFAULT_TIMERS_FREQUENCY.period(), Duration::from_millis(16),
                   "Default timers period is not equal to 16ms");

        // Check frequency set to zero
        frequency.set_value(0.0);
        
        assert_eq!(frequency.period(), Duration::from_millis(0),
                   "0hz period is not equal to 0ms");

    }

    #[test]
    fn rate_limiter() -> Result<(), String>
    {
        let frequency        = Frequency::new(1000.0);
        let lambda_frequency = frequency;
        let test_time        = Duration::from_millis(100);

        let mut counter = 1;

        let lambda = ||
        {
            counter += 1;
            if counter > (test_time.as_millis() / lambda_frequency.period().as_millis())
            {
                return Err(String::from("Closure was called too many times. \
                                         Max frequency was not respected"));
            }

            Ok(())
        };


        let mut rate_limiter = RateLimiter::new(lambda, frequency);

        let start_time = SystemTime::now();

        while SystemTime::now().duration_since(start_time).unwrap() <= test_time
        {
            rate_limiter.execute()?;
        }

        let expected_value = test_time.as_millis() / frequency.period().as_millis();
        assert_eq!(counter, expected_value, "Closure was not executed the expected \
                   number of times given the frequency and test time");

        Ok(())
    }
}
