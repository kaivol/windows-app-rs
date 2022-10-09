use ::windows_app::bootstrap;
use ::windows_app::Windows::System::Power::*;

fn main() -> ::windows::core::Result<()> {
    bootstrap::initialize()?;
    let charge = PowerManager::RemainingChargePercent()?;
    println!("Remaining charge: {charge}%");
    bootstrap::uninitialize()
}
