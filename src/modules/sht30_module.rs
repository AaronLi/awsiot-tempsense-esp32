use lazy_static::lazy_static;
use sht3x::{Address, Sht3x};
lazy_static!{
    pub static ref SHT3X: Sht3x = Sht3x::new(Address::Low);
}