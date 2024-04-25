use std::net::SocketAddr;
use std::thread::sleep;
use std::time::Duration;

use linux_embedded_hal::I2cdev;
use metrics::gauge;
use metrics_exporter_prometheus::PrometheusBuilder;
use mcp9808::reg_conf::{Configuration, ShutdownMode};
use mcp9808::reg_temp_generic::ReadableTempRegister;
use mcp9808::MCP9808;
use mcp9808::reg_res::ResolutionVal;

fn main() {
    let i2c_bus = I2cdev::new("/dev/i2c-0").unwrap();
    let mut mcp9808 = MCP9808::new(i2c_bus);

    let mut conf = mcp9808.read_configuration().unwrap();
    conf.set_shutdown_mode(ShutdownMode::Shutdown);
    mcp9808.write_register(conf).unwrap();

    let builder = PrometheusBuilder::new();
    builder
        .with_http_listener(SocketAddr::from(([0, 0, 0, 0], 9003)))
        .install()
        .expect("failed to install recorder/exporter");

    let temperature_c = gauge!("temperature_celsius_indoors");
    let temperature_f = gauge!("temperature_fahrenheit_indoors");

    loop {
        let temperature = mcp9808.read_temperature().unwrap();
        let temperature = temperature.get_celsius(ResolutionVal::Deg_0_0625C);
        temperature_c.set(temperature);
        temperature_f.set((temperature * 1.8) + 32.0);
        println!("{temperature}");
        sleep(Duration::from_secs(1));
    }
}
