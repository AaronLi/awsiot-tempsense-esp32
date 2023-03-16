use std::ops::{Add, DerefMut, Sub};
use std::thread::sleep;
use std::sync::Arc;
use std::time::{Duration, Instant, SystemTime};
use embedded_graphics::pixelcolor::Rgb888;
use embedded_svc::mqtt::client::QoS;
use esp_idf_sys as _;
use sht3x::{ClockStretch, Repeatability};

use crate::modules::{led_printer_module, mqtt_module, sht30_module, ws2812_display_module, peripherals_module, ntp_module};

mod config;
mod modules;
mod certs;

// If using the `binstart` feature of `esp-idf-sys`, always keep this module imported

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_sys::link_patches();
    peripherals_module::wifi::connect().expect("Failed to connect to wifi");
    peripherals_module::wifi::wait_for_connection(Instant::now().add(Duration::from_secs(30)));
    println!("Wifi Connected");
    ntp_module::wait_for_sync();
    println!("Clock Synced");
    let display = Arc::clone(&led_printer_module::DISPLAY);
    let mut i2c = Arc::clone(&peripherals_module::i2c::I2C_DRIVER);
    let sht30 = &sht30_module::SHT3X;
    let mqtt = Arc::clone(&mqtt_module::MQTT_CLIENT);
    println!("{:?}", SystemTime::UNIX_EPOCH.elapsed().unwrap().as_secs());
    let mut next_publish = Instant::now();
    loop {
        if let Some(_) = Instant::now().checked_duration_since(next_publish) {
            let temp = sht30.measure(i2c.lock().unwrap().deref_mut(), ClockStretch::Disabled, Repeatability::High).unwrap();
            let current_time = SystemTime::UNIX_EPOCH.elapsed().unwrap().as_secs();
            mqtt.lock().unwrap().publish("hygrometer/boortsog_tank/node0", QoS::AtMostOnce, false, format!("{{\"node\":\"{}\", \"temperature\":{}, \"humidity\":{}, \"timestamp\":{}}}", config::CONFIG.client_id, temp.temperature, temp.humidity, current_time).as_bytes());
            let display_str = format!("T:{}H:{}", temp.temperature as i32, temp.humidity as i32);
            display.lock().unwrap().display(&display_str, Rgb888::new(20, 20, 20), Rgb888::new(0, 0, 0));
            next_publish = Instant::now().add(Duration::from_secs(config::CONFIG.publish_period_secs));
        }
        ws2812_display_module::PIXELS.write().unwrap().flush();
        sleep(Duration::from_millis(25))
    }
}
