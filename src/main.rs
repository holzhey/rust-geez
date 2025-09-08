use ggez::event::{self, EventHandler};
use ggez::graphics::{self, Color};
use ggez::mint::{Point2, Vector2};
use ggez::{Context, ContextBuilder, GameResult};
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
}

impl EventHandler for MyGame {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let width = ctx.gfx.window().inner_size().width;
        for star in &mut self.stars {
            star.x = (star.x + star.depth) % width;
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);
        let scale = Vector2::<f32> { x: 2.0, y: 2.0 };
        for star in &self.stars {
            canvas.draw(
                &graphics::Quad,
                graphics::DrawParam::new()
                    .dest(star.get_point())
                    .scale(scale)
                    .color(star.get_color()),
            );
        }
        canvas.finish(ctx)
    }
}
