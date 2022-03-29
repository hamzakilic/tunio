use etherparse::PacketBuilder;
use log::debug;
use std::io::{Read, Write};
use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tunio::{DefaultDriver, DefaultInterface, Driver, DriverBuilder, Interface, InterfaceBuilder};

#[tokio::main]
async fn main() {
    env_logger::init();
    let driver = DefaultDriver::new(DriverBuilder::default().into()).unwrap();

    let params = InterfaceBuilder::new()
        .name("name")
        .description("description");

    let interface = DefaultInterface::new(driver, params.into()).unwrap();

    let mut stream = interface.create_stream().unwrap();

    for _ in 1..100 {
        let builder = PacketBuilder::ipv6(
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            5,
        )
        .udp(8080, 8080);

        let mut packet = Vec::with_capacity(builder.size(0));
        builder.write(&mut packet, &[]).unwrap();

        stream.write(&*packet).await;

        sleep(Duration::from_secs(1));
    }

    let mut buf = vec![0u8; 4096];
    while let Ok(n) = stream.read(buf.as_mut_slice()).await {
        println!("{buf:x?}");
    }

    tokio::signal::ctrl_c().await;
}
