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
    x: f32,
    y: f32,
    ix: f32,
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
        for num in 0..height {
            let star = Star {
                x: rng.random_range(0..width) as f32,
                y: num as f32,
                ix: rng.random_range(1..5) as f32,
            };
            stars.push(star);
        }
        MyGame { stars }
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        for star in &mut self.stars {
            star.x += star.ix;
            if star.x > _ctx.gfx.window().inner_size().width as f32 {
                star.x = 0.0;
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);
        let scale = Vector2::<f32> { x: 2.0, y: 2.0 };
        for star in &self.stars {
            let p = Point2::<f32> {
                x: star.x,
                y: star.y,
            };
            let lum = 255 - ((5 - star.ix as u8) * 20);
            canvas.draw(
                &graphics::Quad,
                graphics::DrawParam::new()
                    .dest(p)
                    .scale(scale)
                    .color(Color::from_rgb(lum, lum, lum)),
            );
        }
        canvas.finish(ctx)
    }
}
