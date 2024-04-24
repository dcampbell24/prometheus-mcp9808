use linux_embedded_hal::I2cdev;
use mcp9808::reg_conf::{Configuration, ShutdownMode};
use mcp9808::reg_temp_generic::ReadableTempRegister;
use mcp9808::MCP9808;
use mcp9808::reg_res::ResolutionVal;

fn main() {
    let i2c_bus = I2cdev::new("/dev/i2c-1").unwrap();
    let mut mcp9808 = MCP9808::new(i2c_bus);

    let mut conf = mcp9808.read_configuration().unwrap();
    conf.set_shutdown_mode(ShutdownMode::Shutdown);
    let _c = mcp9808.write_register(conf);

    let temp = mcp9808.read_temperature().unwrap();
    println!("{}", temp.get_celsius(ResolutionVal::Deg_0_0625C));
}
