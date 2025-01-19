#![no_std]
#![no_main]
#![cfg_attr(test, feature(custom_test_frameworks))]
#![cfg_attr(test, reexport_test_harness_main = "test_main")]
#![cfg_attr(test, test_runner(agb::test_runner::test_runner))]

use agb::input::Button;

fn rgb_to_u16(r: u8, g: u8, b: u8) -> u16 {
    ((b as u16) << 10) | ((g as u16) << 5) | r as u16
}

#[agb::entry]
fn main(mut gba: agb::Gba) -> ! {
    agb::println!("Hello World!");

    let mut input = agb::input::ButtonController::new();
    let mut screen = gba.display.video.bitmap3();

    for y in 0..160 {
        for x in 0..240 {
            screen.draw_point(x, y, rgb_to_u16((x % 32) as u8, (y % 32) as u8, 0));
        }
    }

    loop {
        agb::display::busy_wait_for_vblank();
        input.update();
    }
}
