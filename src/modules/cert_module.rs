use lazy_static::lazy_static;
use crate::certs;
use itertools::chain;
use itertools::Itertools;

lazy_static!{
    pub static ref PRIVATE_KEY: Vec<u8> = certs::PRIVATE_KEY.iter().chain([0].iter()).copied().collect::<Vec<u8>>();
    pub static ref CLIENT_CERT: Vec<u8> = certs::CLIENT_CERT.iter().chain([0].iter()).copied().collect::<Vec<u8>>();
}