use std::sync::{Arc, Mutex, RwLock};

use esp_idf_hal::i2c::{config::Config, I2cDriver};
use esp_idf_hal::prelude::Peripherals;
use esp_idf_hal::units::{Hertz, KiloHertz};
use esp_idf_svc::wifi::EspWifi;
use esp_idf_svc::nvs::{EspNvsPartition, NvsDefault};
use esp_idf_svc::eventloop::EspSystemEventLoop;
use lazy_static::lazy_static;

use crate::modules::peripherals_module;

lazy_static!{
    static ref DEVICES: (Arc<Mutex<I2cDriver<'static>>>, Arc<Mutex<EspWifi<'static>>>) = (||{
        let peripherals = Peripherals::take().unwrap();
        let i2c = Arc::new(Mutex::new(I2cDriver::new(peripherals.i2c1, peripherals.pins.gpio26, peripherals.pins.gpio32, &Config{
            baudrate: Hertz::from(KiloHertz(100)),
            sda_pullup_enabled: false,
            scl_pullup_enabled: false,
        }).expect("Failed to intialize i2c driver")));
        let wifi = Arc::new(Mutex::new(EspWifi::new(peripherals.modem, EspSystemEventLoop::take().unwrap(), EspNvsPartition::<NvsDefault>::take().ok()).expect("Failed to create EspWifi")));
        (i2c, wifi)
    })();
}

pub mod wifi {
    use std::sync::{Arc, Mutex};
    use std::thread::sleep;
    use std::time::{Duration, Instant};

    use embedded_svc::wifi::{AccessPointInfo, ClientConfiguration, Configuration, Wifi};
    use esp_idf_hal::peripherals::Peripherals;
    use esp_idf_svc::eventloop::EspSystemEventLoop;
    use esp_idf_svc::nvs::{EspNvsPartition, NvsDefault};
    use esp_idf_svc::wifi::EspWifi;
    use esp_idf_sys::EspError;
    use lazy_static::lazy_static;

    use crate::{config, modules::peripherals_module};

    lazy_static!{
        pub static ref WIFI: Arc<Mutex<EspWifi<'static>>> = Arc::clone(&super::DEVICES.deref().1);
    }

    pub fn connect() -> Result<(), EspError>{
        WIFI.lock().map(|mut w|{
            let network = w.scan().expect("Failed to scan wifi").into_iter().filter(|ap|ap.ssid.eq(config::CONFIG.wifi_ssid)).next().unwrap();
            let configuration = ClientConfiguration{
                ssid: network.ssid,
                bssid: Some(network.bssid),
                auth_method: network.auth_method,
                password: config::CONFIG.wifi_psk.into(),
                channel: Some(network.channel),
            };
            w.set_configuration(&Configuration::Client(configuration))?;
            w.start()?;
            w.connect()
        }).expect("WiFi expected to work")
    }

    pub fn wait_for_connection(wait_until: Instant) {
        while Instant::now() < wait_until {
            if WIFI.lock().unwrap().is_connected().unwrap() {
                return;
            }
            sleep(Duration::from_millis(500));
        }
    }
}

pub mod i2c {
    use std::sync::{Arc, Mutex};
    use esp_idf_hal::i2c::I2cDriver;
    use lazy_static::lazy_static;
    lazy_static!{
        pub static ref I2C_DRIVER: Arc<Mutex<I2cDriver<'static>>> = Arc::clone(&super::DEVICES.deref().0);
    }
}