use ::windows::core::*;
use ::windows_app::Microsoft::Windows::ApplicationModel::Resources::*;
use ::windows_app::*;
use windows_app::Microsoft::MRM::MrmAllocateBuffer;

fn main() -> Result<()> {
    bootstrap::initialize()?;
    sample_main()?;
    bootstrap::uninitialize()
}

fn sample_main() -> Result<()> {
    // ResourceLoader provides you basic access to string resources from the set of resource files,
    // referenced libraries, or other packages.
    let resource_loader =
        ResourceLoader::CreateInstance2(w!("resources.pri"), w!("Resources/Strings"))?;
    println!(
        "Simple greeting (English/Spanish system context): {}",
        resource_loader.GetString(w!("SimpleGreeting"))?
    );

    // The ResourceManager class provides additional info about resources, such as enumeration and
    // inspection. This goes beyond what the ResourceLoader class provides.
    let resource_manager = ResourceManager::CreateInstance(w!("resources.pri"))?;
    let resource_context = resource_manager.CreateResourceContext()?;
    resource_context.QualifierValues()?.Insert(w!("Language"), w!("es"))?;

    let resource_map = resource_manager.MainResourceMap()?;
    println!(
        "Longer greeting (Spanish context): {}",
        resource_map
            .GetValueWithContext(w!("Resources/Strings/LongerGreeting"), &resource_context)?
            .ValueAsString()?
    );

    Ok(())
}
