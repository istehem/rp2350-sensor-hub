use alloc::string::ToString;
use cyw43::JoinOptions;
use cyw43_pio::PioSpi;
use defmt::{debug, error, info, warn};
use embassy_executor::Spawner;
use embassy_futures::select::select;
use embassy_net::dns::DnsSocket;
use embassy_net::tcp::client::{TcpClient, TcpClientState};
use embassy_net::{Config, StackResources};
use embassy_rp::clocks::RoscRng;
use embassy_rp::gpio::Output;
use reqwless::client::HttpClient;
use reqwless::headers::ContentType;
use reqwless::request::{Method, RequestBuilder};
use reqwless::response::StatusCode;
use static_cell::StaticCell;

use crate::LedChannel;
use crate::TempHumidityChannel;
use crate::network::error::ReqwlessError;

type TcpHttpClient<'a> = HttpClient<'a, TcpClient<'a, 1, 4096, 4096>, DnsSocket<'a>>;

const WIFI_NETWORK: &str = env!("WIFI_NETWORK");
const WIFI_PASSWORD: &str = env!("WIFI_PASSWORD");
const MEASUREMENTS_ENDPOINT: &str = env!("MEASUREMENTS_ENDPOINT");

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

pub async fn run(
    spawner: &Spawner,
    power: Output<'static>,
    spi: WifiPioSpi,
    led_channel: &'static LedChannel,
    temp_humidity_channel: &'static TempHumidityChannel,
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
    // Activate the led to signal that the stack is up.
    control.gpio_set(0, true).await;

    let client_state = TcpClientState::<1, 4096, 4096>::new();
    let tcp_client = TcpClient::new(stack, &client_state);
    let dns_client = DnsSocket::new(stack);

    let mut http_client = HttpClient::new(&tcp_client, &dns_client);

    loop {
        select(
            set_led_state(&mut control, led_channel),
            post_measurement(&mut http_client, temp_humidity_channel),
        )
        .await;
    }
}

async fn post_measurement(
    http_client: &mut TcpHttpClient<'_>,
    temp_humidity_channel: &'static TempHumidityChannel,
) {
    let measurement = temp_humidity_channel.receive().await;
    let body_values = [
        r#"{"temperature":"#,
        &measurement.temperature.to_string(),
        r#","humidity":"#,
        &measurement.humidity.to_string(),
        "}",
    ];
    let body: &str = &body_values.join("");
    debug!("Going to post: {}", body);

    match http_post(http_client, MEASUREMENTS_ENDPOINT, body).await {
        Ok(status_code) => debug!(
            "Posting measurement exited with HTTP status {:?}.",
            defmt::Debug2Format(&status_code)
        ),
        Err(err) => error!("Posting measurement failed with: {}", err),
    }
}

async fn set_led_state(control: &mut cyw43::Control<'static>, led_channel: &'static LedChannel) {
    let led_state = led_channel.receive().await;
    control.gpio_set(0, led_state).await;
}

async fn http_post(
    http_client: &mut TcpHttpClient<'_>,
    url: &str,
    body: &str,
) -> Result<StatusCode, ReqwlessError> {
    let mut rx_buffer = [0; 4096];
    Ok(http_client
        .request(Method::POST, url)
        .await?
        .content_type(ContentType::ApplicationJson)
        .body(body.as_bytes())
        .send(&mut rx_buffer)
        .await?
        .status)
}

#[embassy_executor::task]
async fn cyw43_task(runner: cyw43::Runner<'static, Output<'static>, WifiPioSpi>) -> ! {
    runner.run().await
}

#[embassy_executor::task]
async fn net_task(mut runner: embassy_net::Runner<'static, cyw43::NetDriver<'static>>) -> ! {
    runner.run().await
}
