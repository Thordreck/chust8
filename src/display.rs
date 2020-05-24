
use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::video::{ Window, WindowContext };
use sdl2::render::{ TextureCreator, Canvas };

// TODO: this should be customizable
const DISPLAY_WIDTH  : u32 = 1280;
const DISPLAY_HEIGHT : u32 = 640;

const DISPLAY_TITLE : &'static str = "Chust8";

mod grid;
mod tileset;

use grid::{ GridEditor, PixelGrid };
use tileset::{ Tileset, TileType };

pub struct Display
{
    canvas      : Canvas<Window>,
    grid_editor : GridEditor,
    tileset     : Tileset
}

impl Display
{
    pub fn new(window: Window) -> Result<Self, String>
    {
        let canvas = match window.into_canvas().present_vsync().build()
        {
            Ok(canvas) => canvas,
            Err(error) => return Err(error.to_string()),
        };

        let grid_editor = GridEditor::new();
        let tileset     = Tileset::new(&canvas)?;

        Ok(Display { canvas, grid_editor, tileset, })
    }

    pub fn update(&mut self) -> Result<(), String>
    {
        // Compute the rect that will contain the sprites
        // according to the current window's size
        let display_cell = self.display_cell_size();

        // Get references to the textures and grid
        let texture  = self.tileset.texture();
        let on_tile  = self.tileset.tile(TileType::On);
        let off_tile = self.tileset.tile(TileType::Off);

        let grid = self.grid_editor.grid();

        // Loop through the grid and update the textures
        for col in 0..grid.width()
        {
            for row in 0..grid.height()
            {
                let pixel = grid.at(row, col)?;
                let tile  = if pixel { on_tile } else { off_tile };

                // Compute the tile where the sprite will be rended.
                // First the rescaled size is computed, and then the
                // resulting rect is deplaced to its final position.
                let mut rescaled_tile = display_cell;

                let new_x = col * rescaled_tile.width() as usize;
                let new_y = row * rescaled_tile.height() as usize;

                rescaled_tile.set_x(new_x as i32);
                rescaled_tile.set_y(new_y as i32);

                self.canvas.copy(&texture, Some(*tile), Some(rescaled_tile))?;
            }
        }

        self.canvas.present();
        Ok(())
    }

    pub fn default_window(context: &sdl2::Sdl) -> Result<Window, String>
    {
        let video_subsystem = context.video()?;

        match video_subsystem.window(DISPLAY_TITLE, DISPLAY_WIDTH, DISPLAY_HEIGHT)
                                        .position_centered().build()
        {
            Ok(window) => Ok(window),
            Err(error) => Err(error.to_string())
        }
    }

    pub fn mut_editor(&mut self) -> &mut GridEditor
    {
        &mut self.grid_editor
    }
}

// private impl
impl Display
{
    fn display_cell_size(&self) -> Rect
    {
        let viewport = self.canvas.viewport();
        let grid     = self.grid_editor.grid();

        Rect::new(0, 0, viewport.width() / grid.width() as u32,
                        viewport.height() / grid.height() as u32)
    }
}

#[cfg(test)]
mod tests
{
    use super::*;

    use rand::Rng;

    #[test]
    fn display_init() -> Result<(), String>
    {
        let context = sdl2::init()?;
        let window  = Display::default_window(&context)?;
        let display = Display::new(window)?;

        Ok(())
    }

    #[test]
    fn display_update() -> Result<(), String>
    {
        let context     = sdl2::init()?;
        let window      = Display::default_window(&context)?;
        let mut display = Display::new(window)?;

        // Set random pixels and display the result
        let mut rng  = rand::thread_rng();
        let num_iter = 1000;

        let mut grid = display.mut_editor().mut_grid();

        for _ in 0..num_iter
        {
            let row   = rng.gen_range(0, grid.height());
            let col   = rng.gen_range(0, grid.width());
            let value = rng.gen::<bool>();

            grid.set(row, col, value)?;
        }

        display.update()?;
        Ok(())
    }
}
