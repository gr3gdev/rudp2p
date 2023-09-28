use cucumber::World;
use data::PeersWorld;
use report::Markdown;
use rudp2plib::configuration::Configuration;
use std::fs;
use tracing_subscriber::{
    filter::LevelFilter,
    fmt::format::{DefaultFields, Format},
    layer::SubscriberExt as _,
    Layer as _,
};

pub(crate) mod dao;
pub(crate) mod data;
pub(crate) mod report;
pub(crate) mod steps;
pub(crate) mod utils;

fn configure(port: u16) -> Configuration {
    Configuration::builder()
        .port(port)
        .share_connections(true)
        .database(rudp2plib::configuration::SqliteMode::Memory)
        .build()
}

fn main() {
    let report = "sqlite.md";
    let output = fs::File::create(report).unwrap();
    let writer = Markdown::new(output);
    futures::executor::block_on(
        PeersWorld::cucumber()
            .fail_on_skipped()
            .max_concurrent_scenarios(5)
            .after(|_feature, _rule, _scenario, _ev, world| world.unwrap().close())
            .with_writer(writer)
            .with_default_cli()
            .configure_and_init_tracing(DefaultFields::new(), Format::default().pretty(), |layer| {
                tracing_subscriber::registry().with(LevelFilter::DEBUG.and_then(layer))
            })
            .run("features"),
    );
}
