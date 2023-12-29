#![allow(non_snake_case)]

use std::marker::PhantomData;
use std::mem;
use std::ptr::null_mut;
use std::sync::atomic::{AtomicPtr, Ordering};

use windows::core::{s, Error as WinError, Result as WinResult, HRESULT, PCSTR};

unsafe impl<T1, T2: Default> Sync for FunctionCache<T1, T2> {}
struct FunctionCache<T1, T2: Default> {
    function: AtomicPtr<()>,
    name: PCSTR,
    _phantom: PhantomData<(T1, T2)>,
}

impl<T1, T2: Default> FunctionCache<T1, T2> {
    const fn new(name: PCSTR) -> FunctionCache<T1, T2> {
        Self {
            function: AtomicPtr::new(null_mut()),
            name,
            _phantom: PhantomData,
        }
    }

    fn call(&self, value: T1) -> WinResult<T2> {
        let function = self.function.load(Ordering::Relaxed);
        let function: extern "system" fn(T1, *mut T2) -> HRESULT = if !(function.is_null()) {
            unsafe { mem::transmute(function) }
        } else {
            unsafe {
                let function = windows::core::imp::delay_load(
                    s!("Microsoft.Internal.FrameworkUdk.dll"),
                    self.name,
                )
                .ok_or_else(|| {
                    WinError::new(
                        HRESULT(0x80004005u32 as i32),
                        format!("Could not load load function {}", self.name.display()).into(),
                    )
                })?;
                self.function.store(function as *mut _, Ordering::Relaxed);
                function
            }
        };
        let mut result = T2::default();
        function(value, &mut result).and_then(|| result)
    }
}

#[cfg(all(feature = "Microsoft_UI", feature = "Windows_Win32_Foundation"))]
mod window {
    use windows::core::Error as WinError;
    use windows::Win32::Foundation::HWND;
    use windows_core::s;

    use super::FunctionCache;
    use crate::Microsoft::UI::WindowId;

    static GetWindowIdFromWindow: FunctionCache<HWND, WindowId> =
        FunctionCache::new(s!("Windowing_GetWindowIdFromWindow"));
    static GetWindowFromWindowId: FunctionCache<WindowId, HWND> =
        FunctionCache::new(s!("Windowing_GetWindowFromWindowId"));

    impl TryFrom<HWND> for WindowId {
        type Error = WinError;

        fn try_from(hwnd: HWND) -> Result<Self, Self::Error> {
            GetWindowIdFromWindow.call(hwnd)
        }
    }

    impl TryFrom<WindowId> for HWND {
        type Error = WinError;

        fn try_from(window: WindowId) -> Result<Self, Self::Error> {
            GetWindowFromWindowId.call(window)
        }
    }
}

#[cfg(all(
    feature = "Microsoft_UI",
    feature = "Windows_Win32_UI_WindowsAndMessaging"
))]
mod icon {
    use windows::core::Error as WinError;
    use windows::Win32::UI::WindowsAndMessaging::HICON;
    use windows_core::s;

    use super::FunctionCache;
    use crate::Microsoft::UI::IconId;

    static GetIconIdFromIcon: FunctionCache<HICON, IconId> =
        FunctionCache::new(s!("Windowing_GetIconIdFromIcon"));
    static GetIconFromIconId: FunctionCache<IconId, HICON> =
        FunctionCache::new(s!("Windowing_GetIconFromIconId"));

    impl TryFrom<HICON> for IconId {
        type Error = WinError;

        fn try_from(hwnd: HICON) -> Result<Self, Self::Error> {
            GetIconIdFromIcon.call(hwnd)
        }
    }

    impl TryFrom<IconId> for HICON {
        type Error = WinError;

        fn try_from(window: IconId) -> Result<Self, Self::Error> {
            GetIconFromIconId.call(window)
        }
    }
}

#[cfg(all(feature = "Microsoft_UI", feature = "Windows_Win32_Graphics_Gdi"))]
mod monitor {
    use windows::core::Error as WinError;
    use windows::Win32::Graphics::Gdi::HMONITOR;
    use windows_core::s;

    use super::FunctionCache;
    use crate::Microsoft::UI::DisplayId;

    static GetDisplayIdFromMonitor: FunctionCache<HMONITOR, DisplayId> =
        FunctionCache::new(s!("Windowing_GetDisplayIdFromMonitor"));
    static GetMonitorFromDisplayId: FunctionCache<DisplayId, HMONITOR> =
        FunctionCache::new(s!("Windowing_GetMonitorFromDisplayId"));

    impl TryFrom<HMONITOR> for DisplayId {
        type Error = WinError;

        fn try_from(hwnd: HMONITOR) -> Result<Self, Self::Error> {
            GetDisplayIdFromMonitor.call(hwnd)
        }
    }

    impl TryFrom<DisplayId> for HMONITOR {
        type Error = WinError;

        fn try_from(window: DisplayId) -> Result<Self, Self::Error> {
            GetMonitorFromDisplayId.call(window)
        }
    }
}
