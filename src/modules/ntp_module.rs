use std::net::UdpSocket;
use std::ops::Add;
use std::thread::sleep;
use std::time::{Duration, Instant, SystemTime};
use lazy_static::lazy_static;
use esp_idf_svc::sntp::{EspSntp, OperatingMode, SntpConf, SyncMode, SyncStatus};
use crate::config;

lazy_static!{
    static ref NTP_CLIENT: EspSntp = EspSntp::new(&SntpConf{
        servers: [config::CONFIG.ntp_server_url],
        operating_mode: OperatingMode::Poll,
        sync_mode: SyncMode::Immediate
    }).expect("Failed to create NTP client");
}

pub fn wait_for_sync(){
    loop {
        match NTP_CLIENT.get_sync_status() {
            SyncStatus::Reset => {}
            SyncStatus::Completed => break,
            SyncStatus::InProgress => {}
        }
        sleep(Duration::from_millis(500));
    }
}