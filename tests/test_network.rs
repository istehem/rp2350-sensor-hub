#[cfg(test)]
mod defmt_stubs {
    #[no_mangle]
    extern "C" fn _defmt_panic() -> ! {
        panic!("defmt panic in test");
    }

    // Return u8 (0 = None, 1+ = Some(index)), not Option<u8>
    #[no_mangle]
    extern "C" fn _defmt_acquire() -> u8 {
        0 // Return 0 to indicate "no logger acquired"
    }

    // Use raw pointer and length, not &[u8]
    #[no_mangle]
    extern "C" fn _defmt_write(_bytes: *const u8, _len: usize) {
        // No-op for tests
    }

    #[no_mangle]
    extern "C" fn _defmt_timestamp() -> u64 {
        0
    }

    // Also add these if missing:
    #[no_mangle]
    extern "C" fn _defmt_flush() {
        // No-op for tests
    }

    #[no_mangle]
    extern "C" fn _defmt_release() {
        // No-op for tests
    }
}

#[cfg(test)]
mod tests {
    use embassy_sync::channel::Channel;
    use reqwless::client::HttpClient;
    use rp2350_sensor_hub::network::api;
    use rp2350_sensor_hub::network::error::SendMeasurementError;
    use rp2350_sensor_hub::{Measurement, TempHumidityChannel};
    use rstest::rstest;
    use static_cell::StaticCell;
    use std_embedded_nal_async::Stack;

    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    static TEMP_HUMIDITY_CHANNEL: StaticCell<TempHumidityChannel> = StaticCell::new();

    async fn mock_measurements(mock_server: &MockServer) {
        Mock::given(method("POST"))
            .and(path("/api/measurements"))
            .respond_with(ResponseTemplate::new(201))
            .mount(&mock_server)
            .await;
    }

    #[rstest]
    #[tokio::test]
    #[test_log::test]
    async fn network() -> Result<(), SendMeasurementError> {
        let mock_server = MockServer::start().await;
        mock_measurements(&mock_server).await;

        let endpoint = format!("{}/api/measurements", mock_server.uri());

        let stack = Stack::default();
        let mut client = HttpClient::new(&stack, &stack);
        let channel = TEMP_HUMIDITY_CHANNEL.init(Channel::new());
        let _measurement = Measurement {
            temperature: 25.0,
            humidity: 45.0,
        };
        channel.send(_measurement).await;
        let status_code = api::post_measurement(&mut client, endpoint.as_str(), channel).await?;

        mock_server.verify().await;
        assert_eq!(status_code.0, 201);

        Ok(())
    }
}
