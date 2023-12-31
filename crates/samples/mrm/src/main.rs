use ::windows::core::Result;
use ::windows_app::Microsoft::Windows::ApplicationModel::Resources::*;
use windows::core::h;
use windows_app::bootstrap;

fn main() -> Result<()> {
    bootstrap::initialize()?;
    sample_main()?;
    bootstrap::uninitialize()
}

fn sample_main() -> Result<()> {
    // ResourceLoader provides you basic access to string resources from the set of resource files,
    // referenced libraries, or other packages.
    let resource_loader =
        ResourceLoader::CreateInstance2(h!("resources.pri"), h!("Resources/Strings"))?;
    println!(
        "Simple greeting (English/Spanish system context): {}",
        resource_loader.GetString(h!("SimpleGreeting"))?
    );

    // The ResourceManager class provides additional info about resources, such as enumeration and
    // inspection. This goes beyond what the ResourceLoader class provides.
    let resource_manager = ResourceManager::CreateInstance(h!("resources.pri"))?;
    let resource_context = resource_manager.CreateResourceContext()?;
    resource_context.QualifierValues()?.Insert(h!("Language"), h!("es"))?;

    let resource_map = resource_manager.MainResourceMap()?;
    println!(
        "Longer greeting (Spanish context): {}",
        resource_map
            .GetValueWithContext(h!("Resources/Strings/LongerGreeting"), &resource_context)?
            .ValueAsString()?
    );

    Ok(())
}
