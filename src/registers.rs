
use arrayvec::ArrayVec;

pub struct IRegister
{
    value: u16,
}

impl IRegister
{
    pub fn new() -> Self
    {
        return Self { value: 0 };
    }

    pub fn get(&self) -> u16
    {
        return self.value;
    }

    pub fn set(&mut self, new_value: u16)
    {
        self.value = new_value;
    }

    pub fn add(&mut self, value: u16) -> bool
    {
        let previous_value = self.value;

        self.value = self.value.wrapping_add(value);

        return self.value < previous_value;
    }
}

const NUM_DATA_REGISTERS : usize = 16;

type AllDataRegisters = ArrayVec::<[DataRegister; NUM_DATA_REGISTERS]>;

pub struct DataRegister
{
    value: u8,
}

impl DataRegister
{
    pub fn new() -> Self
    {
        return Self { value: 0 };
    }

    pub fn initialize_all() -> AllDataRegisters
    {
        return std::iter::repeat_with(|| DataRegister::new())
                .take(NUM_DATA_REGISTERS)
                .collect::<AllDataRegisters>();
    }

    pub fn get(&self) -> u8
    {
        return self.value;
    }

    pub fn set(&mut self, value: u8)
    {
        self.value = value;
    }

    pub fn add(&mut self, value: u8) -> bool
    {
        let previous_value = self.value;

        self.value = self.value.wrapping_add(value);

        return self.value < previous_value;
    }

    pub fn substract(&mut self, value: u8) -> bool
    {
        let previous_value = self.value;

        self.value = self.value.wrapping_sub(value);

        return self.value > previous_value;
    }

    pub fn shift_left(&mut self) -> u8
    {
        let most_significant = (self.value >> 6) & 0x1;
        self.value <<= 1;

        return most_significant;
    }

    pub fn shift_right(&mut self) -> u8
    {
        let less_significant = self.value & 0x1;
        self.value >>= 1;

        return less_significant;
    }
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_i_register()
    {
        let mut i_register = IRegister::new();
        assert_eq!(i_register.get(), 0);

        assert!(!i_register.add(1));
        assert_eq!(i_register.get(), 1);

        i_register.set(std::u16::MAX);
        assert_eq!(i_register.get(), std::u16::MAX);
        assert!(i_register.add(1));

        assert_eq!(i_register.get(), 0);
    }

    #[test]
    fn test_data_register_add_sub()
    {
        let mut data_register = DataRegister::new();
        assert_eq!(data_register.get(), 0);

        assert!(!data_register.add(1));
        assert_eq!(data_register.get(), 1);

        data_register.set(std::u8::MAX);
        assert_eq!(data_register.get(), std::u8::MAX);
        assert!(data_register.add(1));

        assert_eq!(data_register.get(), 0);

        assert!(data_register.substract(1));
        assert_eq!(data_register.get(), std::u8::MAX);


        assert!(!data_register.substract(1));
        assert_eq!(data_register.get(), std::u8::MAX - 1);
    }

    #[test]
    fn test_data_register_shift()
    {
        let mut data_register = DataRegister::new();
        assert_eq!(data_register.get(), 0);

        let test_value = 0b1001001;

        data_register.set(test_value);
        assert_eq!(data_register.get(), test_value);

        assert_eq!(data_register.shift_right(), 0b1);
        assert_eq!(data_register.get(), test_value >> 1);

        assert_eq!(data_register.shift_right(), 0b0);
        assert_eq!(data_register.get(), test_value >> 2);

        data_register.set(test_value);
        assert_eq!(data_register.get(), test_value);

        assert_eq!(data_register.shift_left(), 0b1);
        assert_eq!(data_register.get(), test_value << 1);

        assert_eq!(data_register.shift_left(), 0b0);
        assert_eq!(data_register.get(), test_value << 2);
    }
}
