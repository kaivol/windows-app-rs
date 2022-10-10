//! Utilities for bootstrapping an app that uses the Windows App SDK.

use windows::core::HSTRING;
use windows::Win32::Storage::Packaging::Appx::{PACKAGE_VERSION, PACKAGE_VERSION_0};

use crate::Microsoft::WindowsAppSdk::Foundation::*;

/// Locates the Windows App SDK framework package compatible with the
/// metadata-matched versioning criteria and loads it into the current process.
///
/// If multiple packages meet the criteria, the best candidate is selected.
pub fn initialize() -> windows::core::Result<()> {
    initialize_with_options(MddBootstrapInitializeOptions_None)
}

pub fn initialize_with_options(
    options: MddBootstrapInitializeOptions,
) -> windows::core::Result<()> {
    unsafe {
        MddBootstrapInitialize2(
            WINDOWSAPPSDK_RELEASE_MAJORMINOR,
            &HSTRING::from(WINDOWSAPPSDK_RELEASE_VERSION_TAG_W),
            PACKAGE_VERSION {
                Anonymous: PACKAGE_VERSION_0 {
                    Version: WASR_VERSION_UINT64,
                },
            },
            options,
        )
    }
}

/// Undo the changes made by `initialize()`.
pub fn uninitialize() -> windows::core::Result<()> {
    unsafe { MddBootstrapShutdown() }
    Ok(())
}
