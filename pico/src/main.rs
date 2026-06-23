#![no_std]
#![no_main]

use cyw43_pio::PioSpi;
use defmt_rtt as _;
use embassy_net::tcp::client::{TcpClient, TcpClientState};
use embassy_rp::gpio::{Level, Output};
use embassy_rp::pio::Pio;
use panic_probe as _;
use reqwless::client::HttpClient;
use reqwless::request::Method;
use static_cell::StaticCell;

embassy_rp::bind_interrupts!(struct Irqs {
    PIO0_IRQ_0 => embassy_rp::pio::InterruptHandler<embassy_rp::peripherals::PIO0>;
});

#[embassy_executor::task]
async fn wifi_task(
    runner: cyw43::Runner<
        'static,
        Output<'static>,
        PioSpi<'static, embassy_rp::peripherals::PIO0, 0, embassy_rp::peripherals::DMA_CH0>,
    >,
) -> ! {
    runner.run().await
}

#[embassy_executor::task]
async fn net_task(stack: &'static embassy_net::Stack<cyw43::NetDriver<'static>>) -> ! {
    stack.run().await
}

#[embassy_executor::main]
async fn main(spawner: embassy_executor::Spawner) {
    let p = embassy_rp::init(Default::default());
    let fw = include_bytes!("../firmware/43439A0.bin");
    let clm = include_bytes!("../firmware/43439A0_clm.bin");
    let pwr = Output::new(p.PIN_23, Level::Low);
    let cs = Output::new(p.PIN_25, Level::High);
    let mut pio = Pio::new(p.PIO0, Irqs);
    let spi = PioSpi::new(
        &mut pio.common,
        pio.sm0,
        pio.irq0,
        cs,
        p.PIN_24,
        p.PIN_29,
        p.DMA_CH0,
    );
    static STATE: StaticCell<cyw43::State> = StaticCell::new();
    let state = STATE.init(cyw43::State::new());
    let (net_device, control, runner) = cyw43::new(state, pwr, spi, fw).await;
    spawner.spawn(wifi_task(runner)).unwrap();
    let mut control = control;
    control.init(clm).await;
    control
        .set_power_management(cyw43::PowerManagementMode::PowerSave)
        .await;

    let config = embassy_net::Config::dhcpv4(Default::default());

    static RESOURCES: StaticCell<embassy_net::StackResources<2>> = StaticCell::new();
    static STACK: StaticCell<embassy_net::Stack<cyw43::NetDriver<'static>>> = StaticCell::new();

    let stack = &*STACK.init(embassy_net::Stack::new(
        net_device,
        config,
        RESOURCES.init(embassy_net::StackResources::new()),
        1234,
    ));
    control
        .join_wpa2("YOUR_WIFI_SSID", "YOUR_WIFI_PASSWORD")
        .await
        .unwrap();

    spawner.spawn(net_task(stack)).unwrap();
    stack.wait_config_up().await;

    let rx_buffer = [0u8; 4096];
    let tx_buffer = [0u8; 4096];
    let dns = embassy_net::dns::DnsSocket::new(stack);
    static TCP_STATE: StaticCell<TcpClientState<1, 1024, 1024>> = StaticCell::new();
    let tcp_client = TcpClient::new(stack, TCP_STATE.init(TcpClientState::new()));
    let mut client = HttpClient::new(&tcp_client, &dns);

    loop {
        let response = client
            .request(Method::GET, "http://192.168.0.199:3000/now-playing")
            .await;
        match response {
            Ok(_resp) => {
                defmt::info!("Got response");
            }
            Err(e) => {
                defmt::info!("Error: {:?}", e);
            }
        }
    }
}
