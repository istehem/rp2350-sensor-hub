#![no_std]
#![no_main]

extern crate alloc;

use cyw43_pio::{PioSpi, RM2_CLOCK_DIVIDER};
use embassy_executor::Spawner;
use embassy_rp::{
    bind_interrupts,
    gpio::{Input, Level, Output, Pull},
    i2c::{self, Config as I2cConfig, I2c},
    peripherals::{I2C1, PIO1},
    pio::{InterruptHandler, Pio},
};
use embassy_sync::{blocking_mutex::raw::NoopRawMutex, channel::Channel};
use embedded_alloc::LlffHeap;
use static_cell::StaticCell;
use {defmt_rtt as _, panic_probe as _};

mod game {
    pub mod cache;
    pub mod entities;
    pub mod error;
    pub mod player;
    pub mod tasks;
}

mod network {
    pub mod controller;
}

#[cfg(feature = "temperature")]
mod temperature_and_humidity {
    pub mod error;
    pub mod tasks;
    pub use embassy_rp::{gpio::Flex, peripherals::PIO0};
}

#[cfg(feature = "temperature")]
pub use temperature_and_humidity::{Flex, PIO0};

type LedChannel = Channel<NoopRawMutex, bool, 4>;
static LED_CHANNEL: StaticCell<LedChannel> = StaticCell::new();

const I2C_FREQUENCY: u32 = 400_000;

#[global_allocator]
static HEAP: LlffHeap = LlffHeap::empty();

bind_interrupts!(struct Irqs {
    I2C1_IRQ => i2c::InterruptHandler<I2C1>;
    PIO1_IRQ_0 => InterruptHandler<PIO1>;
    #[cfg(feature = "temperature")]
    PIO0_IRQ_0 => InterruptHandler<PIO0>;
});

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    {
        unsafe { HEAP.init(cortex_m_rt::heap_start() as usize, 8 * 1024) }
    }
    let p = embassy_rp::init(Default::default());

    let mut config = I2cConfig::default();
    config.frequency = I2C_FREQUENCY;

    let led_channel = LED_CHANNEL.init(Channel::new());
    let sensor = Input::new(p.PIN_21, Pull::Up);
    let i2c = I2c::new_async(p.I2C1, p.PIN_7, p.PIN_6, Irqs, config);

    game::tasks::spawn_tasks(&spawner, sensor, led_channel, i2c).await;

    #[cfg(feature = "temperature")]
    {
        let pio = p.PIO0;
        let Pio {
            mut common, sm0, ..
        } = Pio::new(pio, Irqs);
        let mut pin = common.make_pio_pin(p.PIN_17);
        pin.set_pull(Pull::Up);

        temperature_and_humidity::tasks::spawn_tasks(&spawner, pin, common, sm0).await;
    }

    let power = Output::new(p.PIN_23, Level::Low);
    let chip_select = Output::new(p.PIN_25, Level::High);
    let mut pio = Pio::new(p.PIO1, Irqs);
    let spi = PioSpi::new(
        &mut pio.common,
        pio.sm0,
        // SPI communication won't work if the speed is too high, so we use a divider larger than `DEFAULT_CLOCK_DIVIDER`.
        // See: https://github.com/embassy-rs/embassy/issues/3960.
        RM2_CLOCK_DIVIDER,
        pio.irq0,
        chip_select,
        // Wireless SPI Data
        p.PIN_24,
        // Wireless SPI Clock
        p.PIN_29,
        p.DMA_CH0,
    );
    network::controller::init(&spawner, power, spi, led_channel).await;
}
