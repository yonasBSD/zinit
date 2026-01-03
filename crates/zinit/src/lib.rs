#![allow(
    clippy::pedantic,
    clippy::cargo,
    clippy::significant_drop_tightening,
    clippy::useless_let_if_seq,
    clippy::multiple_crate_versions
)]

extern crate serde;
#[macro_use]
extern crate anyhow;
#[macro_use]
extern crate log;
extern crate tokio;

pub mod app;
pub mod manager;
pub mod testapp;
pub mod zinit;
