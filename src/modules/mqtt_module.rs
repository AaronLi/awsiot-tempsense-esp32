use lazy_static::lazy_static;
use esp_idf_svc::mqtt::client::{EspMqttClient, MqttClientConfiguration, MqttProtocolVersion};
use crate::config;
use crate::modules::cert_module;
use std::sync::{Arc, Mutex};
use esp_idf_svc::tls::X509;
use std::default::Default;
use core::time::Duration;
use std::ffi::CStr;

lazy_static!{
    pub static ref MQTT_CLIENT: Arc<Mutex<EspMqttClient>> = Arc::new(Mutex::new(EspMqttClient::new(dbg!(config::CONFIG.mqtt_url), &MqttClientConfiguration{
        protocol_version: Some(MqttProtocolVersion::V3_1_1),
        client_id: Some(config::CONFIG.client_id),
        crt_bundle_attach: Some(esp_idf_sys::esp_crt_bundle_attach),
        disable_clean_session: true,
        keep_alive_interval: Some(Duration::from_secs(600)),
        client_certificate: Some(X509::pem(CStr::from_bytes_with_nul(&cert_module::CLIENT_CERT).expect("Invalid device cert configuration"))),
        private_key: Some(X509::pem(CStr::from_bytes_with_nul(&cert_module::PRIVATE_KEY).expect("Invalid device cert configuration"))),
        ..Default::default()
    }, |e|{}).expect("Failed to create MQTT Client")));
}