use cyw43_pio::PioSpi;
use embassy_executor::Spawner;
use embassy_rp::gpio::Output;
use embassy_sync::{blocking_mutex::raw::NoopRawMutex, channel::Channel};
use static_cell::StaticCell;

// Program metadata for `picotool info`.
// This isn't needed, but it's recommended to have these minimal entries.
#[unsafe(link_section = ".bi_entries")]
#[used]
pub static PICOTOOL_ENTRIES: [embassy_rp::binary_info::EntryAddr; 4] = [
    embassy_rp::binary_info::rp_program_name!(c"Blinky Example"),
    embassy_rp::binary_info::rp_program_description!(
        c"Wifi controler for the Pico 2 W. \
        The ounboard LED is connected to GPIO 0 of the cyw43."
    ),
    embassy_rp::binary_info::rp_cargo_version!(),
    embassy_rp::binary_info::rp_program_build_attribute!(),
];

static STATE: StaticCell<cyw43::State> = StaticCell::new();
type Pio = embassy_rp::peripherals::PIO1;
type Dma = embassy_rp::peripherals::DMA_CH0;
type WifiPioSpi = PioSpi<'static, Pio, 0, Dma>;

pub type LedChannel = Channel<NoopRawMutex, bool, 4>;

pub async fn init(
    spawner: &Spawner,
    power: Output<'static>,
    spi: WifiPioSpi,
    led_channel: &'static LedChannel,
) {
    let firmware = include_bytes!("../../cyw43-firmware/43439A0.bin");
    // Country Locale Matrix
    let clm = include_bytes!("../../cyw43-firmware/43439A0_clm.bin");

    let state = STATE.init(cyw43::State::new());
    let (_net_device, mut control, runner) = cyw43::new(state, power, spi, firmware).await;

    spawner.spawn(cyw43_task(runner)).unwrap();

    control.init(clm).await;
    control
        .set_power_management(cyw43::PowerManagementMode::PowerSave)
        .await;

    // The driver assumes exclusive access to control so it can't be spawned into another task.
    wifi_blink(control, led_channel).await;
}

async fn wifi_blink(mut control: cyw43::Control<'static>, led_channel: &'static LedChannel) {
    loop {
        let led_state = led_channel.receive().await;
        control.gpio_set(0, led_state).await;
    }
}

#[embassy_executor::task]
async fn cyw43_task(runner: cyw43::Runner<'static, Output<'static>, WifiPioSpi>) -> ! {
    runner.run().await
}
