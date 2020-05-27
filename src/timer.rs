
use num_traits::sign::Unsigned;
use num_traits::identities::{ One, Zero };
use std::ops::SubAssign;

#[derive(Debug, PartialEq)]
pub enum TimerStatus
{
    Started,
    Running,
    End,
    Stopped,
}

pub struct GeneralTimer<T>
    where T: Copy + Unsigned + One + Zero + SubAssign
{
    ticks  : T,
    status : TimerStatus,
}

// Public impl
impl<T> GeneralTimer<T>
    where T: Copy + Unsigned + One + Zero + SubAssign
{
    pub fn new() -> Self
    {
        GeneralTimer { ticks : T::zero(),
                       status : TimerStatus::Stopped
                     }
    }

    pub fn tick(&mut self) -> TimerStatus
    {
        if self.ticks == T::zero() { return TimerStatus::Stopped; }

        self.ticks -= T::one();

        if self.ticks == T::zero()
        {
            return TimerStatus::End;
        }

        TimerStatus::Running
    }

    pub fn set_value(&mut self, new_value : T) -> TimerStatus 
    {
        let previous_value = self.ticks;
        self.ticks = new_value;

        // Note: I had to it like this since it's not possible
        // to call functions such as T::zero() inside a match.
        let value_change = (previous_value.is_zero(),
                            self.ticks.is_zero());

        use TimerStatus::*;
        match value_change
        {
            (true,  true)  => Stopped,
            (false, true)  => End,
            (true,  false) => Started,
            (false, false) => Running,
        }
    }

    pub fn get_value(&self) -> T
    {
        self.ticks
    }
}

pub type Timer = GeneralTimer<u8>;

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn timer_status()
    {
        use TimerStatus::*;

        let mut timer = Timer::new();

        // Check that the timer is zero initialized
        assert_eq!(timer.get_value(), 0);

        assert_eq!(timer.tick(), Stopped);
        assert_eq!(timer.tick(), Stopped);

        // Check that no ticks do nothing if the timer is already
        // at zero
        assert_eq!(timer.get_value(), 0);

        let timer_value = std::u8::MAX;

        assert_eq!(timer.set_value(timer_value), Started);
        assert_eq!(timer.get_value(), timer_value);

        for step in 1..timer_value
        {
            assert_eq!(timer.tick(), Running,
                      "Checking return value in iteration #{}", step);
            assert_eq!(timer.get_value(), timer_value - step,
                       "Timer ticked {} times", step);
        }

        assert_eq!(timer.get_value(), 1);

        // Next tick should set the internal counter to
        // zero and return end
        assert_eq!(timer.tick(), End);
        assert_eq!(timer.get_value(), 0);

        // Check all the return values for set_value()
        // 1. Setting a non-zero value should trigger the started
        // status only once
        assert_eq!(timer.set_value(50), Started);
        assert_eq!(timer.set_value(50), Running);
        assert_eq!(timer.set_value(59), Running);

        // 2. Setting it to zero should trigger the End status
        // only once
        assert_eq!(timer.set_value(0), End);
        assert_eq!(timer.set_value(0), Stopped);

    }
}
