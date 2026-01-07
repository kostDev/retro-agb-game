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

mod game;

use game::Player;

use agb::{
    include_aseprite,
    display::object::Object,
    input::{ ButtonController },
    fixnum::{ vec2 }
};

include_aseprite!(
    mod sprites,
    "gfx/sprites.aseprite"
);

#[agb::entry]
fn main(mut gba: agb::Gba) -> ! {
    const PADDING_X: i32 = 8;
    const PADDING_Y: i32 = 24;
    // Get the graphics manager, responsible for all the graphics
    let mut gfx = gba.graphics.get();
    let mut input = ButtonController::new();
    let mut player = Player::new(
        vec2(agb::display::WIDTH/2 - PADDING_X,agb::display::HEIGHT - PADDING_Y),
        1,
        Object::new(sprites::SHIP.sprite(0))
    );
    player.update();
    let mut frame = gfx.frame();

    player.obj.show(&mut frame);
    frame.commit();

    loop {
        input.update();
        player.handle_input(input.x_tri());
        player.update();
        // prepare the frame
        let mut frame = gfx.frame();

        player.obj.show(&mut frame);
        frame.commit();
    }
}
