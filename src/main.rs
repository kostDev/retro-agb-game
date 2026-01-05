// Games made using `agb` are no_std which means you don't have access to the standard
// rust library. This is because the game boy advance doesn't have an operating
// system, so most of the content of the standard library doesn't apply.
#![no_std]
// `agb` defines its own `main` function, so you must declare your game's main function
// using the #[agb::entry] proc macro. Failing to do so will cause failure in linking
// which won't be a particularly clear error message.
#![no_main]
// This is required to allow writing tests
#![cfg_attr(test, feature(custom_test_frameworks))]
#![cfg_attr(test, reexport_test_harness_main = "test_main")]
#![cfg_attr(test, test_runner(agb::test_runner::test_runner))]
// By default no_std crates don't get alloc, so you won't be able to use things like Vec
// until you declare the extern crate. `agb` provides an allocator so it will all work
extern crate alloc;

use agb::{
    include_aseprite,
    display::object::Object,
    input::{ ButtonController, Tri },
    fixnum::{ vec2, Vector2D }
};

include_aseprite!(
    mod sprites,
    "gfx/sprites.aseprite"
);
// The main function must take 1 arguments and never returns, and must be marked with
// the #[agb::entry] macro.
// build & run .gba game
// cargo build --release
// agb-gbafix target/thumbv4t-none-eabi/release/hero -o hero.gba

struct Player {
    // vel: Vector2D<i32>,
    pos: Vector2D<i32>,
    speed: i32,
    obj: Object,
}

#[agb::entry]
fn main(mut gba: agb::Gba) -> ! {
    // Get the graphics manager, responsible for all the graphics
    let mut gfx = gba.graphics.get();
    let mut input = ButtonController::new();
    let mut player = Player {
        pos: vec2(50,50),
        speed: 1,
        obj: Object::new(sprites::BALL.sprite(0))
    };
    player.obj.set_pos(player.pos);
    // Start a frame and add the one object to it
    let mut frame = gfx.frame();
    player.obj.show(&mut frame);
    // Until the call to `frame.commit()`, nothing will be displayed
    frame.commit();

    loop {
        input.update();

        match input.x_tri() {
            Tri::Positive => {
                player.pos.x = (
                    player.pos.x + player.speed
                ).clamp(0, agb::display::WIDTH - 16);
            },
            Tri::Negative => {
                player.pos.x = (
                    player.pos.x - player.speed
                ).clamp(0, agb::display::WIDTH - 16);
            },
            Tri::Zero => {},
        }

        match input.y_tri() {
            Tri::Positive => {
                player.pos.y = (
                    player.pos.y + player.speed
                ).clamp(0, agb::display::HEIGHT - 16);
            },
            Tri::Negative => {
                player.pos.y = (
                    player.pos.y - player.speed
                ).clamp(0, agb::display::HEIGHT - 16);
            },
            Tri::Zero => {},
        }
        // if player.pos.x == 0 || player.pos.x == agb::display::WIDTH - 16 {
        //     vel.x = -vel.x;
        // }
        // if player.pos.y == 0 || player.pos.y == agb::display::HEIGHT - 16 {
        //     vel.y = -vel.y;
        // }

        // Set the position of the ball to match our new calculated position
        player.obj.set_pos(player.pos);

        // prepare the frame
        let mut frame = gfx.frame();
        player.obj.show(&mut frame);

        frame.commit();
    }
}
