use cyw43_pio::PioSpi;
use defmt::info;
use embassy_executor::Spawner;
use embassy_rp::gpio::Output;
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
type Pio = embassy_rp::peripherals::PIO1;
type Dma = embassy_rp::peripherals::DMA_CH0;
type WifiPioSpi = PioSpi<'static, Pio, 0, Dma>;

pub async fn spawn_tasks(spawner: &Spawner, power: Output<'static>, spi: WifiPioSpi) {
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
    wifi_blink(control).await;
}

async fn wifi_blink(mut control: cyw43::Control<'static>) {
    let delay = Duration::from_millis(2000);
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
async fn cyw43_task(runner: cyw43::Runner<'static, Output<'static>, WifiPioSpi>) -> ! {
    runner.run().await
}
