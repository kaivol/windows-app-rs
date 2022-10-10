#![cfg(test)]

use once_cell::sync::Lazy;
use windows::core::{Interface, Result};
use windows::w;
use windows::Win32::Foundation::BOOL;
use windows_app::bootstrap;
use windows_app::DirectWriteCore::*;

static BOOTSTRAP: Lazy<Result<()>> = Lazy::new(|| bootstrap::initialize());

// This sample will demonstrate enumerating the fonts in the system font collection, by family name.
// Based on: https://docs.microsoft.com/windows/win32/directwrite/font-enumeration

#[test]
fn main() -> Result<()> {
    Lazy::force(&BOOTSTRAP).clone()?;

    unsafe {
        // Step 1: Create DirectWrite Core factory.
        // Use the DWriteCoreCreateFactory function to create a factory object for subsequent
        // DirectWrite object creation.

        let factory = DWriteCoreCreateFactory(DWRITE_FACTORY_TYPE_SHARED, &IDWriteFactory7::IID)?
            .cast::<IDWriteFactory7>()?;

        // Step 2: Get the system font collection.
        // Use the GetSystemFontCollection method provided by the DirectWrite Core factory to return
        // an IDWriteFontCollection with all of the system fonts in it, grouped by the typographical
        // family name preferred by the font author.

        let fonts =
            factory.GetSystemFontCollection3(false, DWRITE_FONT_FAMILY_MODEL_TYPOGRAPHIC)?;

        // Step 3: Loop through system font collection.
        // Get the font family count from the font collection by using GetFontFamilyCount method.

        for i in 0..fonts.GetFontFamilyCount() {
            // Step 3.a: Get the font family.
            // Get a IDWriteFontFamily object by using the GetFontFamily method, passing it the current index.

            let family = fonts.GetFontFamily(i)?;

            // Step 3.b: Get the font family names.
            // Get the font family names by using the GetFamilyNames method. This method returns a
            // IDWriteLocalizedStrings object that can point to multiple localized versions of the family name.

            let family_names = family.GetFamilyNames()?;

            // Step 3.c: Find the localized name.
            // Get the font family name in the locale you want by using the FindLocaleName method.

            let mut name_index = 0;
            let mut name_exists = BOOL::default();
            family_names.FindLocaleName(w!("en-US"), &mut name_index, &mut name_exists)?;
            name_exists.expect("Failed to find en-US family name string");

            // Step 3.d: Get the length of the family name string
            // Get the length of the family name string by using the GetStringLength method. Use this length
            // to allocate a buffer large enough to hold the name and then get the font family name by using
            // GetString.

            let name_length = family_names.GetStringLength(name_index)?;

            let mut buffer = vec![0u16; (name_length + 1) as usize];
            family_names.GetString(name_index, &mut buffer)?;
            let _ = String::from_utf16_lossy(&buffer.split_at(buffer.len() - 1).0);
        }
    }
    Ok(())
}
