#[cfg(test)]
mod tests {
    use reqwless::client::HttpClient;
    //use rp2350_sensor_hub::network::api;
    use rstest::rstest;
    use std_embedded_nal_async::Stack;

    #[rstest]
    #[tokio::test]
    #[test_log::test]
    async fn network() -> () {
        let stack = Stack::default();
        let _http_client = HttpClient::new(&stack, &stack);
        //api::post_measurement(http_client, temp_humidity_channel).await;
        assert!(true)
    }
}
