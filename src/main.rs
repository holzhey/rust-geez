use ggez::context::Has;
use ggez::event::{self, EventHandler};
use ggez::graphics::{self, Color, DrawParam, Drawable, Image, ImageFormat};
use ggez::mint::{Point2, Vector2};
use ggez::{Context, ContextBuilder, GameResult, glam};
use rand::Rng;

fn main() {
    let (mut ctx, event_loop) = ContextBuilder::new("starfield", "StarField POC")
        .build()
        .expect("aieee, could not create ggez context!");
    let my_game = MyGame::new(&mut ctx);
    event::run(ctx, event_loop, my_game);
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

    pub fn get_point(&self) -> Point2<f32> {
        Point2::<f32> {
            x: self.x as f32,
            y: self.y as f32,
        }
    }

    fn get_color(&self) -> Color {
        let lum = 255 - ((5 - self.depth as u8) * 20);
        Color::from_rgb(lum, lum, lum)
    }
}

struct MyGame {
    stars: Vec<Star>,
}

impl MyGame {
    pub fn new(ctx: &mut Context) -> MyGame {
        let mut rng = rand::rng();
        let mut stars: Vec<Star> = Vec::new();
        let width = ctx.gfx.window().inner_size().width;
        let height = ctx.gfx.window().inner_size().height;
        for line in 0..height {
            let x: u32 = rng.random_range(0..width);
            let depth: u32 = rng.random_range(1..5);
            stars.push(Star::new(x, line, depth));
        }
        MyGame { stars }
    }

    fn get_pixels(&self, width: u32, height: u32) -> Vec<u8> {
        let pixels: Vec<u8> = vec![0; (width * height * 4) as usize];

        pixels
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let width = ctx.gfx.window().inner_size().width;
        for star in &mut self.stars {
            star.x += star.depth;
            star.x %= width;
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);

        let width = ctx.gfx.window().inner_size().width;
        let height = ctx.gfx.window().inner_size().height;

        let pixels = self.get_pixels(width, height);

        let image = Image::from_pixels(ctx, &pixels, ImageFormat::Rgba8Uint, width, height);
        image.draw(&mut canvas, DrawParam::default());

        canvas.finish(ctx)
    }
}
