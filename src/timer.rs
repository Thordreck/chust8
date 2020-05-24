
use num_traits::sign::Unsigned;
use num_traits::identities::{ One, Zero };
use std::ops::SubAssign;

pub struct GeneralTimer<'lifetime, T: Copy + Unsigned + One + Zero + SubAssign>
{
    ticks : T,
    start_callback: Option<Box<dyn FnMut() + 'lifetime>>,
    end_callback: Option<Box<dyn FnMut() + 'lifetime>>,
}

// Public impl
impl<'lifetime, T: Unsigned + One + Zero + SubAssign + Copy> GeneralTimer<'lifetime, T>
{
    pub fn new(start_callback : Option<Box<dyn FnMut() + 'lifetime>>,
               end_callback   : Option<Box<dyn FnMut() + 'lifetime>>) -> Self
    {
        GeneralTimer { ticks : T::zero(),
                       start_callback: start_callback,
                       end_callback: end_callback,
                     }
    }

    pub fn tick(&mut self)
    {
        if self.ticks == T::zero() { return; }

        self.ticks -= T::one();

        if self.ticks == T::zero() && self.end_callback.is_some()
        {
            self.end_callback.as_mut().unwrap()();
        }
    }

    pub fn set_value(&mut self, new_value : T)
    {
        let previous_value = self.ticks;

        self.ticks = new_value;

        if self.ticks != T::zero() && self.start_callback.is_some()
        {
            self.start_callback.as_mut().unwrap()();
        }
        else if previous_value != self.ticks
                && self.ticks == T::zero()
                && self.end_callback.is_some()
        {
            self.end_callback.as_mut().unwrap()();
        }
    }

    pub fn get_value(&self) -> T
    {
        self.ticks
    }
}

pub type Timer<'lifetime> = GeneralTimer<'lifetime, u8>;

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn timer_no_callbacks()
    {
        let mut timer = Timer::new(None, None);

        // Check that the timer is zero initialized
        assert_eq!(timer.get_value(), 0);

        timer.tick();
        timer.tick();

        // Check that no ticks do nothing if the timer is already
        // at zero
        assert_eq!(timer.get_value(), 0);

        let timer_value = std::u8::MAX;

        timer.set_value(timer_value);

        assert_eq!(timer.get_value(), timer_value);

        for step in 1..=timer_value
        {
            timer.tick();
            assert_eq!(timer.get_value(), timer_value - step,
                       "Timer ticked {} times", step);
        }

        // Check that the timer has gone back to zero
        assert_eq!(timer.get_value(), 0);
    }

    #[test]
    fn timer_callbacks()
    {
        let mut start_counter = 0;
        let mut end_counter = 0;

        {
            let start_callback = Box::new(|| start_counter += 1);
            let end_callback   = Box::new(|| end_counter += 1);

            let mut timer = Timer::new(Some(start_callback), Some(end_callback));

            // Setting a timer to zero when it's already
            // at zero shouldn't do anything
            timer.set_value(0);

            // Setting it to another number should
            // activate the start_callback
            timer.set_value(2);

            // Ticking it once shouldn't affect
            // the callbacks
            timer.tick();

            // Ticking it second time should make
            // it reach zero and activate the end callback
            timer.tick();
        }

        // At the end both callbacks should be only called once
        // Checks are only done at the end so rustc does not complain
        // about multiple references to the counter variables
        assert_eq!(start_counter, 1);
        assert_eq!(end_counter, 1);
    }
}
