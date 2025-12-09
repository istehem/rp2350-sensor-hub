use cyw43::JoinOptions;
use cyw43_pio::PioSpi;
use defmt::{info, warn};
use embassy_executor::Spawner;
use embassy_net::{Config, StackResources};
use embassy_rp::clocks::RoscRng;
use embassy_rp::gpio::Output;
use embassy_sync::{blocking_mutex::raw::NoopRawMutex, channel::Channel};
use static_cell::StaticCell;

const WIFI_NETWORK: &str = env!("WIFI_NETWORK");
const WIFI_PASSWORD: &str = env!("WIFI_PASSWORD");

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

pub async fn run(
    spawner: &Spawner,
    power: Output<'static>,
    spi: WifiPioSpi,
    led_channel: &'static LedChannel,
) {
    let firmware = include_bytes!("../../cyw43-firmware/43439A0.bin");
    // Country Locale Matrix
    let clm = include_bytes!("../../cyw43-firmware/43439A0_clm.bin");

    let state = STATE.init(cyw43::State::new());
    let (net_device, mut control, runner) = cyw43::new(state, power, spi, firmware).await;

    spawner.spawn(cyw43_task(runner)).unwrap();

    control.init(clm).await;
    control
        .set_power_management(cyw43::PowerManagementMode::PowerSave)
        .await;

    let config = Config::dhcpv4(Default::default());
    let mut rng = RoscRng;
    let seed = rng.next_u64();

    // Init network stack
    static RESOURCES: StaticCell<StackResources<5>> = StaticCell::new();
    let (stack, runner) = embassy_net::new(
        net_device,
        config,
        RESOURCES.init(StackResources::new()),
        seed,
    );

    spawner.spawn(net_task(runner)).unwrap();

    while let Err(err) = control
        .join(WIFI_NETWORK, JoinOptions::new(WIFI_PASSWORD.as_bytes()))
        .await
    {
        warn!("join failed with status={}", err.status);
    }

    info!("waiting for link...");
    stack.wait_link_up().await;

    info!("waiting for DHCP...");
    stack.wait_config_up().await;

    info!("Stack is up!");

    // The driver assumes exclusive access to control so it can't be spawned into another task.
    set_led_state(control, led_channel).await;
}

async fn set_led_state(mut control: cyw43::Control<'static>, led_channel: &'static LedChannel) {
    loop {
        let led_state = led_channel.receive().await;
        control.gpio_set(0, led_state).await;
    }
}

#[embassy_executor::task]
async fn cyw43_task(runner: cyw43::Runner<'static, Output<'static>, WifiPioSpi>) -> ! {
    runner.run().await
}

#[embassy_executor::task]
async fn net_task(mut runner: embassy_net::Runner<'static, cyw43::NetDriver<'static>>) -> ! {
    runner.run().await
}
