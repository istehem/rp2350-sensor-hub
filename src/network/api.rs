use crate::TempHumidityChannel;
use crate::network::error::SendMeasurementError;
use defmt::{debug, error};
use embedded_nal_async::{Dns, TcpConnect};
use reqwless::client::HttpClient;
use reqwless::headers::ContentType;
use reqwless::request::{Method, RequestBuilder};
use reqwless::response::StatusCode;

const REST_USER: &str = env!("REST_USER");
const REST_USER_PASSWORD: &str = env!("REST_USER_PASSWORD");

const TCP_RX_SIZE: usize = 4096;

pub async fn post_measurement<T, D>(
    http_client: &mut HttpClient<'_, T, D>,
    endpoint: &str,
    temp_humidity_channel: &'static TempHumidityChannel,
) -> Result<StatusCode, SendMeasurementError>
where
    T: TcpConnect,
    D: Dns,
{
    let measurement = temp_humidity_channel.receive().await;
    match serde_json_core::to_string::<_, TCP_RX_SIZE>(&measurement) {
        Ok(body) => {
            debug!("Going to post: {}", body.as_str());
            http_post(http_client, endpoint, REST_USER, REST_USER_PASSWORD, &body).await
        }
        Err(err) => {
            error!(
                "Measurement serialization failed with: {:?}",
                defmt::Debug2Format(&err)
            );
            Err(SendMeasurementError::SerializationError)
        }
    }
}

async fn http_post<T, D>(
    http_client: &mut HttpClient<'_, T, D>,
    url: &str,
    user: &str,
    password: &str,
    body: &str,
) -> Result<StatusCode, SendMeasurementError>
where
    T: TcpConnect,
    D: Dns,
{
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
