#![allow(non_snake_case)]

use std::convert::TryFrom;
use std::marker::PhantomData;
use std::mem;
use std::ptr::null_mut;
use std::sync::atomic::{AtomicPtr, Ordering};

use windows::core::{Error as WinError, Result as WinResult, HRESULT, PCSTR};
use windows::Win32::Foundation::HWND;
use windows::Win32::Graphics::Gdi::HMONITOR;
use windows::Win32::System::LibraryLoader::{FreeLibrary, GetProcAddress, LoadLibraryA};
use windows::Win32::UI::WindowsAndMessaging::HICON;

use crate::Microsoft::UI::{DisplayId, IconId, WindowId};

static GetWindowIdFromWindow: FunctionCache<HWND, WindowId> = FunctionCache::new(b"Windowing_GetWindowIdFromWindow\0");
static GetWindowFromWindowId: FunctionCache<WindowId, HWND> = FunctionCache::new(b"Windowing_GetWindowFromWindowId\0");
static GetDisplayIdFromMonitor: FunctionCache<HMONITOR, DisplayId> = FunctionCache::new(b"Windowing_GetDisplayIdFromMonitor\0");
static GetMonitorFromDisplayId: FunctionCache<DisplayId, HMONITOR> = FunctionCache::new(b"Windowing_GetMonitorFromDisplayId\0");
static GetIconIdFromIcon: FunctionCache<HICON, IconId> = FunctionCache::new(b"Windowing_GetIconIdFromIcon\0");
static GetIconFromIconId: FunctionCache<IconId, HICON> = FunctionCache::new(b"Windowing_GetIconFromIconId\0");

struct FunctionCache<T1, T2: Default> {
    function: AtomicPtr<()>,
    name: &'static [u8],
    _phantom: PhantomData<(T1, T2)>,
}

impl<T1, T2: Default> FunctionCache<T1, T2> {
    const fn new(name: &'static [u8]) -> FunctionCache<T1, T2> {
        Self { function: AtomicPtr::new(null_mut()), name, _phantom: PhantomData }
    }

    fn get(&self) -> windows::core::Result<extern "system" fn(T1, *mut T2) -> HRESULT> {
        let function = self.function.load(Ordering::Acquire);
        if !(function.is_null()) {
            Ok(unsafe { mem::transmute(function) })
        } else {
            unsafe { delay_load(b"Microsoft.Internal.FrameworkUdk.dll\0", self.name).map(|f| mem::transmute(f)) }
        }
    }

    fn call(&self, value: T1) -> WinResult<T2> {
        let mut result = T2::default();
        (self.get()?)(value, &mut result).and_then(|| result)
    }
}

/// Copied from windows::core::delay_load
///
/// Load a function from a given library.
///
/// This is a small wrapper around `LoadLibrary` and `GetProcAddress`.
///
/// # Safety
///
/// * Both the library and function names must be valid PCSTR representations
unsafe fn delay_load(library: &[u8], function: &[u8]) -> WinResult<*mut core::ffi::c_void> {
    let library = LoadLibraryA(PCSTR(library.as_ptr()))?;

    if let Some(address) = GetProcAddress(library, PCSTR(function.as_ptr())) {
        Ok(address as _)
    } else {
        FreeLibrary(library);
        Err(WinError::from_win32())
    }
}

impl TryFrom<HWND> for WindowId {
    type Error = windows::core::Error;

    fn try_from(hwnd: HWND) -> Result<Self, Self::Error> {
        GetWindowIdFromWindow.call(hwnd)
    }
}

impl TryFrom<WindowId> for HWND {
    type Error = windows::core::Error;

    fn try_from(window: WindowId) -> Result<Self, Self::Error> {
        GetWindowFromWindowId.call(window)
    }
}

impl TryFrom<HICON> for IconId {
    type Error = windows::core::Error;

    fn try_from(hwnd: HICON) -> Result<Self, Self::Error> {
        GetIconIdFromIcon.call(hwnd)
    }
}

impl TryFrom<IconId> for HICON {
    type Error = windows::core::Error;

    fn try_from(window: IconId) -> Result<Self, Self::Error> {
        GetIconFromIconId.call(window)
    }
}

impl TryFrom<HMONITOR> for DisplayId {
    type Error = windows::core::Error;

    fn try_from(hwnd: HMONITOR) -> Result<Self, Self::Error> {
        GetDisplayIdFromMonitor.call(hwnd)
    }
}

impl TryFrom<DisplayId> for HMONITOR {
    type Error = windows::core::Error;

    fn try_from(window: DisplayId) -> Result<Self, Self::Error> {
        GetMonitorFromDisplayId.call(window)
    }
}
