#![no_std]
#![no_main]
#![cfg_attr(test, feature(custom_test_frameworks))]
#![cfg_attr(test, reexport_test_harness_main = "test_main")]
#![cfg_attr(test, test_runner(agb::test_runner::test_runner))]

use agb::input::Button;

#[agb::entry]
fn main(mut gba: agb::Gba) -> ! {
    agb::println!("Hello World!");

    let mut input = agb::input::ButtonController::new();

    loop {
        if input.is_just_pressed(Button::A) {
            agb::println!("A!");
        }

        agb::display::busy_wait_for_vblank();
        input.update();
    }
}
