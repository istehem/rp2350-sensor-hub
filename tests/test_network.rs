include!("common/defmt_mock.rs");

#[cfg(test)]
mod tests {
    use embassy_sync::channel::Channel;
    use reqwless::client::HttpClient;
    use rp2350_sensor_hub::network::api;
    use rp2350_sensor_hub::network::error::SendMeasurementError;
    use rp2350_sensor_hub::{Measurement, TempHumidityChannel};
    use rstest::rstest;
    use static_cell::StaticCell;
    use std::io;
    use std_embedded_nal_async::Stack;
    use wiremock::matchers::{basic_auth, body_json, header, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    static TEMP_HUMIDITY_CHANNEL_SUCCESS: StaticCell<TempHumidityChannel> = StaticCell::new();
    static TEMP_HUMIDITY_CHANNEL_RESET_ERROR: StaticCell<TempHumidityChannel> = StaticCell::new();

    const REST_USER: &str = env!("REST_USER");
    const REST_USER_PASSWORD: &str = env!("REST_USER_PASSWORD");
    const MEASUREMENTS_ENDPOINT: &str = env!("MEASUREMENTS_ENDPOINT");

    fn connection_reset_error(_request: &wiremock::Request) -> io::Error {
        io::Error::new(io::ErrorKind::ConnectionReset, "Connection reset by peer")
    }

    async fn mock_measurements_success(mock_server: &MockServer, measurement: &Measurement) {
        Mock::given(method("POST"))
            .and(basic_auth(REST_USER, REST_USER_PASSWORD))
            .and(header("Content-Type", "application/json"))
            .and(path(MEASUREMENTS_ENDPOINT))
            .and(body_json(measurement))
            .respond_with(ResponseTemplate::new(201))
            .mount(&mock_server)
            .await;
    }

    async fn mock_measurements_connection_reset(
        mock_server: &MockServer,
        measurement: &Measurement,
    ) {
        Mock::given(method("POST"))
            .and(basic_auth(REST_USER, REST_USER_PASSWORD))
            .and(header("Content-Type", "application/json"))
            .and(path(MEASUREMENTS_ENDPOINT))
            .and(body_json(measurement))
            .respond_with_err(connection_reset_error)
            .mount(&mock_server)
            .await;
    }

    #[rstest]
    #[tokio::test]
    #[test_log::test]
    async fn network_success() -> Result<(), SendMeasurementError> {
        let mock_server = MockServer::start().await;

        let measurement = Measurement {
            temperature: 25.0,
            humidity: 45.0,
        };
        mock_measurements_success(&mock_server, &measurement).await;

        let host = mock_server.uri();

        let stack = Stack::default();
        let mut client = HttpClient::new(&stack, &stack);
        let channel = TEMP_HUMIDITY_CHANNEL_SUCCESS.init(Channel::new());
        channel.send(measurement).await;
        let status_code = api::post_measurement(&mut client, &host, channel).await?;

        mock_server.verify().await;
        assert_eq!(status_code.0, 201);

        Ok(())
    }

    #[rstest]
    #[tokio::test]
    #[test_log::test]
    async fn network_error() -> Result<(), SendMeasurementError> {
        let mock_server = MockServer::start().await;

        let measurement = Measurement {
            temperature: 25.0,
            humidity: 45.0,
        };
        mock_measurements_connection_reset(&mock_server, &measurement).await;

        let host = mock_server.uri();

        let stack = Stack::default();
        let mut client = HttpClient::new(&stack, &stack);
        let channel = TEMP_HUMIDITY_CHANNEL_RESET_ERROR.init(Channel::new());
        channel.send(measurement).await;
        let result = api::post_measurement(&mut client, &host, channel).await;

        mock_server.verify().await;
        assert!(
            matches!(
                result,
                Err(SendMeasurementError::ReqwlessError(
                    reqwless::Error::ConnectionAborted
                ))
            ),
            "expected ConnectionAborted, got {:?}",
            result
        );

        Ok(())
    }
}
