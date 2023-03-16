use std::ffi::CStr;
use toml_cfg::toml_config;



#[toml_config]
pub(crate) struct Config {
    #[default("")]
    pub wifi_ssid: &'static str,

    #[default("")]
    pub wifi_psk: &'static str,

    #[default(27)]
    pub led_pin: u32,

    #[default(75)]
    pub display_ms_per_pixel: u16,

    #[default("")]
    pub mqtt_url: &'static str,

    #[default("")]
    pub client_id: &'static str,

    #[default("")]
    pub client_cert_path: &'static str,

    #[default("")]
    pub private_key_path: &'static str,

    #[default(30)]
    pub publish_period_secs: u64,

    #[default("ca.pool.ntp.org")]
    pub ntp_server_url: &'static str
}