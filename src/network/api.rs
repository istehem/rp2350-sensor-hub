use crate::TempHumidityChannel;
use crate::network::error::ReqwlessError;
use defmt::{debug, error, warn};
use embassy_net::dns::DnsSocket;
use embassy_net::tcp::client::TcpClient;
use reqwless::client::HttpClient;
use reqwless::headers::ContentType;
use reqwless::request::{Method, RequestBuilder};
use reqwless::response::StatusCode;

const MEASUREMENTS_ENDPOINT: &str = env!("MEASUREMENTS_ENDPOINT");
const REST_USER: &str = env!("REST_USER");
const REST_USER_PASSWORD: &str = env!("REST_USER_PASSWORD");

const TCP_TX_SIZE: usize = 4096;
const TCP_RX_SIZE: usize = TCP_TX_SIZE;

type TcpHttpClient<'a> = HttpClient<'a, TcpClient<'a, 1, TCP_TX_SIZE, TCP_RX_SIZE>, DnsSocket<'a>>;

pub async fn post_measurement(
    http_client: &mut TcpHttpClient<'_>,
    temp_humidity_channel: &'static TempHumidityChannel,
) {
    let measurement = temp_humidity_channel.receive().await;
    match &serde_json_core::to_string::<_, TCP_RX_SIZE>(&measurement) {
        Ok(body) => {
            debug!("Going to post: {}", body.as_str());

            match http_post(
                http_client,
                MEASUREMENTS_ENDPOINT,
                REST_USER,
                REST_USER_PASSWORD,
                body,
            )
            .await
            {
                Ok(status_code) => handle_status_code(status_code),
                Err(err) => error!("Posting measurement failed with: {}", err),
            }
        }
        Err(err) => error!(
            "Measurement serialization failed with: {:?}",
            defmt::Debug2Format(err)
        ),
    }
}

async fn http_post(
    http_client: &mut TcpHttpClient<'_>,
    url: &str,
    user: &str,
    password: &str,
    body: &str,
) -> Result<StatusCode, ReqwlessError> {
    let mut rx_buffer = [0; TCP_RX_SIZE];
    Ok(http_client
        .request(Method::POST, url)
        .await?
        .content_type(ContentType::ApplicationJson)
        .basic_auth(user, password)
        .body(body.as_bytes())
        .send(&mut rx_buffer)
        .await?
        .status)
}

fn handle_status_code(status_code: StatusCode) {
    if status_code.is_successful() {
        debug!(
            "Posting measurement succeeded with http exit code: {}",
            status_code.0
        )
    } else if status_code.is_client_error() || status_code.is_server_error() {
        error!(
            "Posting measurement failed with http exit code: {}",
            status_code.0
        )
    } else {
        warn!(
            "Posting measurement exited with a non successful http exit code: {}",
            status_code.0
        )
    }
}
