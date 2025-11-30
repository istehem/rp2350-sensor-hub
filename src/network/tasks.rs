use embassy_executor::Spawner;

pub async fn spawn_tasks(spawner: &Spawner) {
    spawner.spawn(wifi_blink()).unwrap();
}

#[embassy_executor::task]
async fn wifi_blink() {}
