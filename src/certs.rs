use include_bytes;

pub(crate) const CLIENT_CERT: &'static [u8] = include_bytes!("../ssl/client_cert.crt");
pub(crate) const PRIVATE_KEY: &'static [u8] = include_bytes!("../ssl/private_key.key");