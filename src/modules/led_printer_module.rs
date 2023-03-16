use lazy_static::lazy_static;
use crate::config;
use eg_simple_status_messaging::LedPrinter;
use crate::modules::ws2812_display_module;
use std::sync::{Arc, Mutex};
use embedded_graphics::pixelcolor::Rgb888;
use ws2812_esp32_rmt_driver::lib_embedded_graphics::{LedPixelMatrix, Ws2812DrawTarget};
use ws2812_esp32_rmt_driver::Ws2812Esp32RmtDriverError;


lazy_static!{
    pub static ref DISPLAY: Arc<Mutex<LedPrinter<Rgb888, Ws2812Esp32RmtDriverError, Ws2812DrawTarget<LedPixelMatrix<5, 5>>>>> = Arc::new(Mutex::new(LedPrinter::new(Arc::clone(&ws2812_display_module::PIXELS), config::CONFIG.display_ms_per_pixel).expect("Invalid LedPrinter configuration")));
}