#![no_std]
extern crate alloc;

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
    pub mod error;
}

#[cfg(feature = "board")]
pub mod game {
    mod cache;
    mod entities;
    mod error;
    mod player;
    pub mod tasks;
}

#[cfg(feature = "board")]
#[cfg(feature = "temperature")]
pub mod temperature_and_humidity {
    mod error;
    pub mod tasks;
    pub use embassy_rp::peripherals::PIO0;
}
