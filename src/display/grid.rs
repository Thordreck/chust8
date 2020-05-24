
pub struct GridEditor
{
    grid: PixelGrid
}

impl GridEditor
{
    pub fn new() -> Self
    {
        GridEditor { grid: PixelGrid::new() }
    }

    pub fn clear(&mut self)
    {
        self.grid.data.iter_mut().for_each(|pixel| *pixel = false);
    }

    pub fn write_byte(&mut self, row: u8, col: u8, byte: u8) -> Result<(), String>
    {
        let bits = bits_big_endian(byte);

        // TODO: recheck this
        for (index, bit) in bits.iter().enumerate()
        {
            let big_endian_index = (7 - index) as usize;
            let col_index        = (col as usize + big_endian_index) % GRID_WIDTH;
            let row_index        = row as usize % GRID_HEIGHT;

            let current_value = self.grid.at(row_index, col_index)?;

            self.grid.set(row_index, col_index, current_value ^ bit)?;
        }

        Ok(())
    }

    pub fn grid(&self) -> &PixelGrid
    {
        &self.grid
    }

    pub fn mut_grid(&mut self) -> &mut PixelGrid
    {
        &mut self.grid
    }
}

// TODO: this function returns the values in little endian order actually.
// Have to fix that
// Helper function to split all the bits in a byte following a 
// big endian order
fn bits_big_endian(mut byte : u8) -> [bool; 8]
{
    let mut bits = [false; 8];

    // Iterate through all the bits in given byte
    for bit_index in 0..8
    {
        bits[bit_index] = (byte % 2) == 1;
        byte /= 2;
    }

    return bits;
}

const GRID_WIDTH  : usize = 64;
const GRID_HEIGHT : usize = 32;

type InternalStorage = [bool; GRID_WIDTH * GRID_HEIGHT];

pub struct PixelGrid
{
    data : InternalStorage,
}

// public
impl PixelGrid
{
    pub fn peek(&self) -> &InternalStorage
    {
        &self.data
    }

    pub fn at(&self, row: usize, col: usize) -> Result<bool, String>
    {
        let index = row * GRID_WIDTH + col;

        if row >= GRID_HEIGHT || col >= GRID_WIDTH || index >= self.data.len()
        {
            return Err(format!("Pixel coordinates ({}, {}) out of bounds. Grid size ({}, {})",
                               row, col, GRID_HEIGHT, GRID_WIDTH));
        }

        Ok(self.data[index])
    }

    pub fn set(&mut self, row: usize, col: usize, value: bool) -> Result<(), String>
    {
        let bit = self.at_mut(row, col)?;

        *bit = value;
        Ok(())
    }

    // TODO: should these be set in the constructor?
    pub fn width(&self) -> usize
    {
        GRID_WIDTH
    }

    pub fn height(&self) -> usize
    {
        GRID_HEIGHT
    }
}

// private
impl PixelGrid
{
    fn new() -> Self
    {
        PixelGrid { data: [false; GRID_WIDTH * GRID_HEIGHT] }
    }

    fn at_mut(&mut self, row: usize, col: usize) -> Result<&mut bool, String>
    {
        let index = row * GRID_WIDTH + col;

        if row >= GRID_HEIGHT || col >= GRID_WIDTH || index >= self.data.len()
        {
            return Err(format!("Pixel coordinates ({}, {}) out of bounds. Grid size ({}, {})",
                               row, col, GRID_HEIGHT, GRID_WIDTH));
        }

        Ok(&mut self.data[index])
    }
}

#[cfg(test)]
mod tests
{
    use super::*;
    use rand::Rng;

    #[test]
    fn grid_init()
    {
        let grid = PixelGrid::new();

        // Check that the size is correct
        assert_eq!(grid.peek().len(), GRID_WIDTH * GRID_HEIGHT,
                   "Pixel grid was not initialized with expected size.\
                    Expected {}. Actual {}", GRID_WIDTH * GRID_HEIGHT,
                    grid.peek().len());

        // Check that it's zero-initialized
        assert!(grid.peek().iter().all(|&pixel| pixel == false ),
                "Pixel grid not initialized with all pixels disabled");
    }

    #[test]
    fn grid_valid_set_at() -> Result<(), String>
    {
        let mut grid = PixelGrid::new();

        // Check that all pixels are false initially
        // using at method
        for x in 0..GRID_HEIGHT
        {
            for y in 0..GRID_WIDTH
            {
                let bit = grid.at(x, y)?;

                assert_eq!(bit, false, "Grid pixel ({}, {}) \
                           not initialized to false", x, y);
            }
        }

        // Set random pixels with random values using set
        // and check that values are correctly updated
        let mut rng  = rand::thread_rng();
        let num_iter = 1000;

        for _ in 0..num_iter
        {
            let row   = rng.gen_range(0, GRID_HEIGHT);
            let col   = rng.gen_range(0, GRID_WIDTH);
            let value = rng.gen::<bool>();

            grid.set(row, col, value)?;
            let updated_value = grid.at(row, col)?;

            assert_eq!(updated_value, value,
                       "Grid's pixel at ({}, {}) was set to {} \
                       but it was not correctly updated", row, col, value);
        }

        Ok(())
    }

    #[test]
    fn grid_invalid_set_at() -> Result<(), String>
    {
        let mut grid = PixelGrid::new();

        // Test out of bounds operations
        let invalid_row = GRID_HEIGHT;
        let invalid_col = GRID_WIDTH;

        assert!(grid.at(invalid_row, invalid_col).is_err(),
                "Acessing a pixel with invalid coordinates does not return err");

        assert!(grid.at(0, invalid_col).is_err(),
                "Acessing a pixel with an invalid column does not return err");

        assert!(grid.at(invalid_row, 0).is_err(),
                "Acessing a pixel with an invalid row does not return err");

        assert!(grid.set(invalid_row, invalid_col, true).is_err(),
                "Setting a pixel with invalid coordinates does not return err");

        assert!(grid.set(0, invalid_col, true).is_err(),
                "Setting a pixel with an invalid column does not return err");

        assert!(grid.set(invalid_row, 0, true).is_err(),
                "Setting a pixel with an invalid row does not return err");

        Ok(())
    }

    #[test]
    fn test_bits_big_endian()
    {
        let byte = 0b11001010;
        let splitted_bits = bits_big_endian(byte);

        // for_each_bit gives the bits in a little endian order
        // so the indexes are inversed.
        assert_eq!(splitted_bits[7], true,  "Unexpected value at position 7");
        assert_eq!(splitted_bits[6], true,  "Unexpected value at position 6");
        assert_eq!(splitted_bits[5], false, "Unexpected value at position 5");
        assert_eq!(splitted_bits[4], false, "Unexpected value at position 4");
        assert_eq!(splitted_bits[3], true,  "Unexpected value at position 3");
        assert_eq!(splitted_bits[2], false, "Unexpected value at position 2");
        assert_eq!(splitted_bits[1], true,  "Unexpected value at position 1");
        assert_eq!(splitted_bits[0], false, "Unexpected value at position 0");
    }

    #[test]
    fn grid_editor_clear() -> Result<(), String>
    {
        let mut grid_editor = GridEditor::new();

        // Write into the grid
        for row in 0..GRID_HEIGHT
        {
            for col in 0..GRID_WIDTH
            {
                grid_editor.grid.set(row, col, true)?;
            }
        }

        assert!(grid_editor.grid.peek().iter().all(|pixel| *pixel == true),
                "Pixels were not update properly before clear test");

        grid_editor.clear();

        assert!(grid_editor.grid.peek().iter().all(|pixel| *pixel == false),
                "GridEditor did not disable all pixels correctly");

        Ok(())
    }

    #[test]
    fn grid_editor_write() -> Result<(), String>
    {
        Ok(())
    }
}
