#![deny(missing_debug_implementations)]

#[macro_use] extern crate futures;
extern crate futures_cpupool;
extern crate hyper;
#[macro_use] extern crate tokio_core as tokio;
#[macro_use] extern crate log;
extern crate hyper_openssl;
extern crate pretty_env_logger;

pub mod resources;