extern crate sdl2;
use self::sdl2::pixels;
use self::sdl2::rect;
use self::sdl2::render::Canvas;
use self::sdl2::video::Window;

const PIXEL_SIZE : u32 = 5;
const CHIP8_WIDTH : usize = 64;
const CHIP8_HEIGHT : usize = 32;


pub struct Display {
    canvas: Canvas<Window>,
    sdl2_context: sdl2::Sdl,
    vram: [[u8; CHIP8_WIDTH]; CHIP8_HEIGHT],
    vram_changed: bool
}

#[derive(Debug)]
struct Pixel {
    colour: pixels::Color,
    representation: rect::Rect
}

#[derive(Debug)]
pub struct Sprite {
    data: Vec<u8>
}

impl Display {
    pub fn new() -> Display {
        let context = sdl2::init().unwrap();
        let mut window_builder = context.video().unwrap().window(
                                                        "Test window",
                                                        CHIP8_WIDTH as u32 * PIXEL_SIZE, 
                                                        CHIP8_HEIGHT as u32 * PIXEL_SIZE);
        let mut window = window_builder.opengl().build().unwrap();
        
        let mut canvas = window.into_canvas().present_vsync().build().unwrap();

        canvas.set_draw_color(pixels::Color::RGB(70, 80, 250));
        canvas.clear();
        canvas.present();

        Display {
            canvas,
            sdl2_context: context,
            vram: [[0u8; CHIP8_WIDTH]; CHIP8_HEIGHT],
            vram_changed: false
        }
    }

    pub fn draw_sprite(&mut self, x: u8, mut y: u8, sprite: Sprite) -> u8 {
        let mut collision = 0;
        for mut row in sprite.data {
            let mut mut_x = x;
            for _ in 0..8 {
                let bit = (row >> 7) & 1;
                if bit != 0 {
                    if self.vram[y as usize][mut_x as usize] == 1 {
                        collision = 1;
                    }
                    self.vram[y as usize][mut_x as usize] ^= bit;
                }
                row <<= 1;
                mut_x += 1;
            }
            y += 1;
        }
        self.vram_changed = true;
        collision
    }

    pub fn refresh_display(&mut self) {
        self.canvas.clear();
        for (y, row) in self.vram.iter().enumerate() {
            for (x, bit) in row.iter().enumerate() {
                let pixel = Pixel::new(x, y, *bit);
                self.canvas.set_draw_color(pixel.colour);
                self.canvas.fill_rect(pixel.representation).unwrap();
            }
        }
        self.canvas.present();
        self.vram_changed = false;
    }

    pub fn vram_changed(&self) -> bool {
        self.vram_changed
    }
}

impl Pixel {
    fn new(xpos : usize, ypos : usize, colour: u8) -> Pixel {
        Pixel {
            colour: if colour == 1 { pixels::Color::RGB(255, 255, 255) } else { pixels::Color::RGB(70, 80, 250) },
            representation: rect::Rect::new(
                                (xpos as i32 * PIXEL_SIZE as i32), 
                                (ypos as i32 * PIXEL_SIZE as i32),
                                PIXEL_SIZE, 
                                PIXEL_SIZE)
        }
    }
}

impl Sprite {
    pub fn new(data: &[u8]) -> Sprite {
        Sprite {
            data: data.to_vec()
        }
    }
}