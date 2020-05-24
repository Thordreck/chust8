
const SYSTEM_RAM_SIZE   : usize = 4096;
const BEGIN_PROGRAM_RAM : usize = 512;
const PROGRAM_RAM_SIZE  : usize = SYSTEM_RAM_SIZE - BEGIN_PROGRAM_RAM;

type InternalStorage = [u8; SYSTEM_RAM_SIZE];

pub struct Ram
{
    data : InternalStorage,
}

// Public impl
impl Ram
{
    pub fn new() -> Self
    {
        let mut new_ram = Ram { data : [0; SYSTEM_RAM_SIZE] };
        new_ram.init_system_memory();

        return new_ram;
    }

    pub fn dump(&mut self, rom : &Vec<u8>) -> Result<(), String>
    {
        if rom.len() > PROGRAM_RAM_SIZE
        {
            return Err(format!("Cannot dump to ram: rom size
                               {} is larger than system memory {}",
                               rom.len(), PROGRAM_RAM_SIZE));
        }

        for(dst, src) in self.data.iter_mut().skip(BEGIN_PROGRAM_RAM).zip(rom)
        {
            *dst = *src;
        }

       Ok(())
    }

    pub fn peek(&self) -> &InternalStorage
    {
        &self.data
    }
}

static DEFAULT_SPRITES : [u8; (0xF + 1) * 5] =
[
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

// Private impl
impl Ram
{
    fn init_system_memory(&mut self)
    {
        for(dst, src) in self.data.iter_mut().zip(DEFAULT_SPRITES.iter())
        {
            *dst = *src;
        }
    }
}

pub struct ProgramCounter
{
    counter : usize,
}

impl ProgramCounter
{
    pub fn new() -> Self
    {
        ProgramCounter { counter : BEGIN_PROGRAM_RAM }
    }

    pub fn advance(&mut self, step : Option<usize>) -> Result<(), String>
    {
        let step_value = step.unwrap_or(1);

        if self.counter + step_value >= BEGIN_PROGRAM_RAM + PROGRAM_RAM_SIZE
        {
            return Err(String::from("Cannot advance program counter out of memory bounds"));
        }

        self.counter += step_value;
        Ok(())
    }

    pub fn value(&self) -> usize
    {
        self.counter
    }
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn ram_system_init()
    {
        let ram = Ram::new();

        // Check that the len is correct
        assert_eq!(ram.peek().len(), SYSTEM_RAM_SIZE,
                   "Ram has wrong initial size. Expected: {}. Actual {}",
                   SYSTEM_RAM_SIZE, ram.peek().len());

        // Check that the system memory has been initialized
        // with the default sprites
        assert_eq!(&ram.peek()[0..DEFAULT_SPRITES.len()], &DEFAULT_SPRITES[..],
                   "Default sprites not loaded correctly in system memory");

        // Check that the rest of the memory is zero-initialized
        assert!(ram.peek().iter().skip(DEFAULT_SPRITES.len()).all(|&value| value == 0),
                "Program memory not zero-initialized");
    }

    #[test]
    fn ram_correct_dump() -> Result<(), String>
    {
        let mut ram = Ram::new();

        // Dump to memory a vector containing consecutive numbers up to
        // the max size
        let test_rom : Vec<u8> = (1..=PROGRAM_RAM_SIZE)
                                    .map(| value | (value % std::u8::MAX as usize) as u8)
                                    .collect();


        // Safe the state of the system memory before the dump
        let mut previous_system_memory : [u8; SYSTEM_RAM_SIZE] = [0; SYSTEM_RAM_SIZE];
        previous_system_memory[..].copy_from_slice(&ram.peek()[0..SYSTEM_RAM_SIZE]);

        ram.dump(&test_rom)?;

        // Check that the system ram part has not been overwritten
        assert_eq!(&previous_system_memory[..BEGIN_PROGRAM_RAM], &ram.peek()[..BEGIN_PROGRAM_RAM],
                "System memory has been modified on rom dump");

        // Check that the vector has been dumped to memory correctly
        assert_eq!(&ram.peek()[BEGIN_PROGRAM_RAM..BEGIN_PROGRAM_RAM + test_rom.len()],
                   &test_rom[..], "Test memory not corretly dumped");

        // Check that the rest of memory remains untouched
        assert_eq!(&previous_system_memory[BEGIN_PROGRAM_RAM + test_rom.len()..],
                   &ram.peek()[BEGIN_PROGRAM_RAM + test_rom.len()..],
                   "Out of bound ram memory was modified during rom dump");

        Ok(())
    }

    #[test]
    fn ram_incorrect_dump() -> Result<(), String>
    {
        let mut ram = Ram::new();

        // Copy all the ram memory before dumping
        let previous_memory = ram.peek().clone();

        // Dump to memory a vector that exceeds the maximum ram size and
        // check that an error is returned
        let test_rom : Vec<u8> = (1..=(PROGRAM_RAM_SIZE + 1))
                                    .map(| value | (value % std::u8::MAX as usize) as u8)
                                    .collect();

        assert!(ram.dump(&test_rom).is_err(),
                "Incorrect dump do not return an error");

        // Check that the memory has been left untouched
        assert_eq!(&previous_memory[..], &ram.peek()[..],
                   "Ram memory has been modified during an invalid dump");

        Ok(())
    }

    #[test]
    fn program_counter_advance() -> Result<(), String>
    {
        let mut program_counter = ProgramCounter::new();

        // Advance up to the last valid memory position
        for x in 1..PROGRAM_RAM_SIZE
        {
            assert!(program_counter.advance(None).is_ok(),
                   "Advance loop failed at step {}", x);
        }

        // Next steps should be invalid
        assert!(program_counter.advance(None).is_err());
        assert!(program_counter.advance(Some(3)).is_err());

        // Check that we are in the last valid memory position
        assert_eq!(program_counter.value(), SYSTEM_RAM_SIZE - 1);

        Ok(())
    }
}
