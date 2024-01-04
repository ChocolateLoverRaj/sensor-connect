use esp_idf_hal::{peripherals::Peripherals, task};
use esp_idf_svc::{
    eventloop::EspSystemEventLoop,
    nvs::EspDefaultNvsPartition,
    timer::EspTaskTimerService,
    wifi::{AsyncWifi, ClientConfiguration, Configuration, WifiDriver},
};
use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
use esp_println::println;

fn main() {
    task::block_on(main_async());
}

async fn main_async() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_sys::link_patches();

    let event_loop = EspSystemEventLoop::take().unwrap();

    let peripherals = Peripherals::take().expect("Failed to take peripherals");
    let wifi_driver = WifiDriver::new(
        peripherals.modem,
        event_loop.clone(),
        Some(EspDefaultNvsPartition::take().expect("Failed to take default nvs partition")),
    )
    .unwrap();
    let mut async_wifi = AsyncWifi::wrap(
        wifi_driver,
        event_loop.clone(),
        EspTaskTimerService::new().unwrap(),
    )
    .unwrap();
    async_wifi
        .set_configuration(&Configuration::Client(ClientConfiguration::default()))
        .unwrap();
    println!("Configured wifi");

    async_wifi.start().await.unwrap();

    println!("scanning wifi");
    let result = async_wifi.scan().await;
    println!("scan result: {:#?}", result);
}
