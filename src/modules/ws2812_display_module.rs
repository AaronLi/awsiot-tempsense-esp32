use lazy_static::lazy_static;
use ws2812_esp32_rmt_driver::lib_embedded_graphics::{LedPixelMatrix, Ws2812DrawTarget};
use crate::config;
use std::sync::{RwLock, Arc};

lazy_static!{
    pub static ref PIXELS: Arc<RwLock<Ws2812DrawTarget<LedPixelMatrix<5, 5>>>> = Arc::new(RwLock::new(Ws2812DrawTarget::new(0, config::CONFIG.led_pin).expect("invalid WS2812 configuration")));
}