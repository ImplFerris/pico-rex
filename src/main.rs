#![no_std]
#![no_main]
mod game;
mod sprites;

// use core::sync::atomic::{AtomicBool, Ordering};
use embassy_executor::Spawner;
use embassy_rp::block::ImageDef;
use embassy_rp::peripherals::I2C1;
use embassy_rp::{self as hal, i2c};
use embassy_time::Timer;
use game::{Game, GameState};
use {defmt_rtt as _, panic_probe as _};

use embassy_rp::bind_interrupts;
use embassy_rp::gpio::{Input, Level, Output, Pull};
use ssd1306::mode::DisplayConfig;
use ssd1306::prelude::DisplayRotation;
use ssd1306::size::DisplaySize128x64;
use ssd1306::{I2CDisplayInterface, Ssd1306};
/// Tell the Boot ROM about our application
#[link_section = ".start_block"]
#[used]
pub static IMAGE_DEF: ImageDef = hal::block::ImageDef::secure_exe();

bind_interrupts!(struct Irqs {
    I2C1_IRQ => i2c::InterruptHandler<I2C1>;
});

// static JUMP_TRIGGERED: AtomicBool = AtomicBool::new(false);
//
// #[embassy_executor::task]
// async fn button_handler(button: Input<'static>) {
//     loop {
//         if button.is_low() {
//             JUMP_TRIGGERED.store(true, Ordering::Relaxed);
//         }
//         Timer::after_millis(20).await;
//     }
// }

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    let mut led = Output::new(p.PIN_25, Level::Low);

    // Setting up I2C send text to OLED display
    let sda = p.PIN_18;
    let scl = p.PIN_19;
    let i2c = i2c::I2c::new_async(p.I2C1, scl, sda, Irqs, i2c::Config::default());
    let interface = I2CDisplayInterface::new(i2c);

    let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();
    display.init().unwrap();
    let button = Input::new(p.PIN_15, Pull::Up);

    display.flush().unwrap();

    let mut game = Game::new(display);
    game.draw_trex().unwrap();

    // spawner.spawn(button_handler(button)).unwrap();
    let mut clicked_count = 0;
    loop {
        if game.state == GameState::GameOver {
            if button.is_low() {
                clicked_count += 1;
            }
            if clicked_count > 2 {
                game = Game::new(game.display);
                Timer::after_millis(500).await;
            }
            Timer::after_millis(50).await;
            continue;
        }

        game.clear_screen().unwrap();
        game.draw_score().unwrap();

        if button.is_low() {
            led.set_high();
            game.trex_jump();
        } else {
            led.set_low();
        }

        game.move_world().unwrap();
        game.draw_ground().unwrap();
        game.draw_trex().unwrap();

        if game.check_collison() {
            game.game_over().unwrap();
            game.display.flush().unwrap();
            Timer::after_millis(500).await;
            continue;
        }

        game.display.flush().unwrap();
        Timer::after_millis(5).await;
    }
}

// Program metadata for `picotool info`.
// This isn't needed, but it's recomended to have these minimal entries.
#[link_section = ".bi_entries"]
#[used]
pub static PICOTOOL_ENTRIES: [embassy_rp::binary_info::EntryAddr; 4] = [
    embassy_rp::binary_info::rp_program_name!(c"PicoRex"),
    embassy_rp::binary_info::rp_program_description!(
        c"Dino Game written in Rust for the Pico 2 (RP2350) with SSD1306, using the Embassy framework."
    ),
    embassy_rp::binary_info::rp_cargo_version!(),
    embassy_rp::binary_info::rp_program_build_attribute!(),
];
