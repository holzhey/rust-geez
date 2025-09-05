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

struct MyGame {
    x: f32,
    y: f32,
}

impl MyGame {
    pub fn new(_ctx: &mut Context) -> MyGame {
        MyGame { x: 10.0, y: 10.0 }
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        self.x += 1.0;
        if self.x > _ctx.gfx.window().inner_size().width as f32 {
            self.x = 0.0;
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLUE);
        let p = Point2::<f32> {
            x: self.x,
            y: self.y,
        };
        let v = Vector2::<f32> { x: 2.0, y: 2.0 };
        canvas.draw(
            &graphics::Quad,
            graphics::DrawParam::new()
                .dest(p)
                .scale(v)
                .color(Color::WHITE),
        );
        canvas.finish(ctx)
    }
}
