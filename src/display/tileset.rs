
use sdl2::rect::Rect;
use sdl2::rwops::RWops;
use sdl2::image::ImageRWops;
use sdl2::video::{ Window, WindowContext };
use sdl2::render::{ Canvas, TextureCreator, Texture };

use static_assertions::const_assert;

// The tileset png file is embedded directly into the binary, since cargo does not seem
// to package non-binary code out of the box. This is done on compilation
const TILESET_DATA : &'static [u8] = include_bytes!("../../resources/tileset.png");

// This assertion is done at compilation time.
const_assert!(TILESET_DATA.len() != 0);

pub enum TileType
{
    On,
    Off,
}

pub struct Tileset
{
    texture_creator : TextureCreator<WindowContext>,
    texture         : Texture,
    on_tile         : Rect,
    off_tile        : Rect,
}

// public impl
impl Tileset
{
    pub fn new(canvas: &Canvas<Window>) -> Result<Self, String>
    {
        let texture_creator = canvas.texture_creator();
        let texture         = load_tileset(&texture_creator)?;
        let query           = texture.query();
        let off_tile        = Rect::new(0, 0, query.width / 2, query.height);
        let on_tile         = Rect::new((query.width / 2) as i32, 0, query.width / 2, query.height);

        Ok(Tileset { texture_creator, texture, on_tile, off_tile })
    }

    pub fn texture(&self) -> &Texture
    {
        &self.texture
    }

    pub fn tile(&self, tile_type : TileType) -> &Rect
    {
        use TileType::*;

        match tile_type
        {
            On  => &self.on_tile,
            Off => &self.off_tile,
        }
    }
}

// To load the png as a texture:
//  1. Read the embedded png bytes
//  2. Save the embedded data into a RWops buffer
//  3. Process the buffer as png to get a surface
//  4. Convert it to a texture using a TextureCreator
fn load_tileset<T>(texture_creator : &TextureCreator<T>) -> Result<Texture, String>
{
    let buffer  = RWops::from_bytes(&TILESET_DATA)?;
    let surface = buffer.load_png()?;

    match texture_creator.create_texture_from_surface(surface)
    {
        Ok(texture) => Ok(texture),
        Err(error)  => return Err(error.to_string()),
    }
}

#[cfg(test)]
mod tests
{
    use super::*;
    use crate::helpers::tests::*;

    #[test]
    fn tileset_init() -> Result<(), String>
    {
        let lock = test_lock()?;

        // Generate a sdl windows to display the tileset
        let context         = sdl2::init()?;
        let video_subsystem = context.video()?;
        let window          = video_subsystem.window("Chust8", 800, 600)
                                .position_centered().build().unwrap();
        let mut canvas      = window.into_canvas().present_vsync()
                                .build().unwrap();

        let tileset = Tileset::new(&canvas)?;

        // Check that the texture is not empty
        let query = tileset.texture.query();

        assert!(query.width > 0 && query.height > 0,
                "Tileset texture was not correctly loaded");

        // Check that it's possible to draw it in a window
        canvas.copy(&tileset.texture, None, None)?;
        canvas.present();

        // Check the values returned for each tile
        let texture = tileset.texture();
        let on_tile = tileset.tile(TileType::On);

        assert_eq!(on_tile.x() as u32, query.width / 2, "Unexpected on tile x position");
        assert_eq!(on_tile.y(), 0, "Unexpected on tile y position");
        assert_eq!(on_tile.width(), query.width / 2, "Unexpected on tile width value");
        assert_eq!(on_tile.height(), query.height, "Unexpected on tile height value");

        let off_tile = tileset.tile(TileType::Off);

        assert_eq!(off_tile.x(), 0, "Unexpected off tile x position");
        assert_eq!(off_tile.y(), 0, "Unexpected off tile y position");
        assert_eq!(off_tile.width(), query.width / 2, "Unexpected off tile width value");
        assert_eq!(off_tile.height(), query.height, "Unexpected off tile height value");

        // Check that it's possible to draw both
        canvas.copy(&texture, Some(*on_tile), None)?;
        canvas.present();

        canvas.copy(&texture, Some(*off_tile), None)?;
        canvas.present();

        Ok(())
    }
}
