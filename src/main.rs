use ggez::event::{self, EventHandler};
use ggez::graphics::{self, Color};
use ggez::mint::{Point2, Vector2};
use ggez::{Context, ContextBuilder, GameResult};

fn main() {
    let (mut ctx, event_loop) = ContextBuilder::new("my_game", "Cool Game Author")
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
    pub fn new(_ctx: &mut Context) -> MyGame {
        let star = Star {
            x: 10.0,
            y: 10.0,
            ix: 1.0,
        };
        MyGame { stars: vec![star] }
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
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLUE);
        let scale = Vector2::<f32> { x: 2.0, y: 2.0 };
        for star in &self.stars {
            let p = Point2::<f32> {
                x: star.x,
                y: star.y,
            };
            canvas.draw(
                &graphics::Quad,
                graphics::DrawParam::new()
                    .dest(p)
                    .scale(scale)
                    .color(Color::WHITE),
            );
        }
        canvas.finish(ctx)
    }
}
