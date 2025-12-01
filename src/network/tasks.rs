use cyw43_pio::PioSpi;
use defmt::info;
use embassy_executor::Spawner;
use embassy_rp::gpio::Output;
use embassy_rp::peripherals::{DMA_CH0, PIO1};
use embassy_time::{Duration, Timer};
use static_cell::StaticCell;

// Program metadata for `picotool info`.
// This isn't needed, but it's recommended to have these minimal entries.
#[unsafe(link_section = ".bi_entries")]
#[used]
pub static PICOTOOL_ENTRIES: [embassy_rp::binary_info::EntryAddr; 4] = [
    embassy_rp::binary_info::rp_program_name!(c"Blinky Example"),
    embassy_rp::binary_info::rp_program_description!(
        c"This example tests the RP Pico 2 W's onboard LED, connected to GPIO 0 of the cyw43 \
        (WiFi chip) via PIO 0 over the SPI bus."
    ),
    embassy_rp::binary_info::rp_cargo_version!(),
    embassy_rp::binary_info::rp_program_build_attribute!(),
];

static STATE: StaticCell<cyw43::State> = StaticCell::new();
type Pio = PIO1;

pub async fn spawn_tasks(
    spawner: &Spawner,
    power: Output<'static>,
    spi: PioSpi<'static, Pio, 0, DMA_CH0>,
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

    spawner.spawn(wifi_blink(control)).unwrap();
}

#[embassy_executor::task]
async fn wifi_blink(mut control: cyw43::Control<'static>) {
    let delay = Duration::from_millis(250);
    loop {
        info!("led on!");
        control.gpio_set(0, true).await;
        Timer::after(delay).await;

        info!("led off!");
        control.gpio_set(0, false).await;
        Timer::after(delay).await;
    }
}

#[embassy_executor::task]
async fn cyw43_task(
    runner: cyw43::Runner<'static, Output<'static>, PioSpi<'static, Pio, 0, DMA_CH0>>,
) -> ! {
    runner.run().await
}
