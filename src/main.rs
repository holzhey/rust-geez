use ggez::conf::Conf;
use ggez::event::{self, EventHandler};
use ggez::graphics::{self, Color, DrawParam, Drawable, Image, ImageFormat};
use ggez::{Context, ContextBuilder, GameResult};
use rand::Rng;

fn main() -> GameResult {
    let mut conf = Conf::new();
    conf.window_setup = conf.window_setup.vsync(true).title("Starfield");
    let (mut ctx, event_loop) = ContextBuilder::new("starfield", "StarField POC")
        .default_conf(conf)
        .build()?;
    let starfield = Starfield::new(&mut ctx);
    event::run(ctx, event_loop, starfield);
}

struct Star {
    x: u32,
    y: u32,
    depth: u32,
}

impl Star {
    pub fn new(x: u32, y: u32, depth: u32) -> Self {
        Star { x, y, depth }
    }
}

struct Starfield {
    stars: Vec<Star>,
}

impl Starfield {
    pub fn new(ctx: &mut Context) -> Starfield {
        let mut rng = rand::rng();
        let mut stars: Vec<Star> = Vec::new();
        let (width, height) = Self::get_window_sizes(ctx);
        for line in 0..height {
            for _repeat in 0..2 {
                let x: u32 = rng.random_range(0..width);
                let depth: u32 = rng.random_range(1..5);
                stars.push(Star::new(x, line, depth));
            }
        }
        Starfield { stars }
    }

    fn get_pixels(&self, width: u32, height: u32) -> Vec<u8> {
        let mut pixels: Vec<u8> = vec![0; (width * height * 4) as usize];
        for star in &self.stars {
            let position = (star.y * 4 * width + star.x * 4) as usize;
            for pos in position..position + 4 {
                if let Some(pixel) = pixels.get_mut(pos) {
                    *pixel = 120 + (star.depth * 32) as u8;
                }
            }
        }
        pixels
    }

    fn get_window_sizes(ctx: &Context) -> (u32, u32) {
        (
            ctx.gfx.window().inner_size().width,
            ctx.gfx.window().inner_size().height,
        )
    }
}

impl EventHandler for Starfield {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let (width, _height) = Self::get_window_sizes(ctx);
        for star in &mut self.stars {
            star.x += star.depth;
            star.x %= width;
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);

        let (width, height) = Self::get_window_sizes(ctx);
        let pixels = self.get_pixels(width, height);

        let image = Image::from_pixels(ctx, &pixels, ImageFormat::Rgba8Unorm, width, height);
        image.draw(&mut canvas, DrawParam::default());

        canvas.finish(ctx)
    }
}
