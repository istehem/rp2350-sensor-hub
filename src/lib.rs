#![no_std]

use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use embassy_sync::channel::Channel;
use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct Measurement {
    pub humidity: f32,
    pub temperature: f32,
}
pub type TempHumidityChannel = Channel<NoopRawMutex, Measurement, 4>;

pub type LedChannel = Channel<NoopRawMutex, bool, 4>;

pub mod network {
    pub mod api;
    #[cfg(feature = "board")]
    pub mod controller;
    mod error;
}
