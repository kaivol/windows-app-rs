#![cfg(test)]

use std::convert::TryInto;

use once_cell::sync::Lazy;
use windows::core::Result;
use windows::Win32::Foundation::{HINSTANCE, POINT};
use windows::Win32::Graphics::Gdi::{MonitorFromPoint, MONITOR_DEFAULTTOPRIMARY};
use windows::Win32::UI::WindowsAndMessaging::{GetDesktopWindow, LoadIconW, IDI_QUESTION};
use windows_app::bootstrap;
use windows_app::UI::{DisplayId, IconId, WindowId};

static BOOTSTRAP: Lazy<Result<()>> = Lazy::new(|| bootstrap::initialize());

#[test]
fn window() -> Result<()> {
    Lazy::force(&BOOTSTRAP).clone()?;

    let hwnd = unsafe { GetDesktopWindow() };
    let window_id: WindowId = hwnd.try_into()?;
    assert_eq!(hwnd, window_id.try_into()?);
    Ok(())
}

#[test]
fn icon() -> Result<()> {
    Lazy::force(&BOOTSTRAP).clone()?;

    let hicon = unsafe { LoadIconW(HINSTANCE(0), IDI_QUESTION)? };
    let icon_id: IconId = hicon.try_into()?;
    assert_eq!(hicon, icon_id.try_into()?);
    Ok(())
}

#[test]
fn monitor() -> Result<()> {
    Lazy::force(&BOOTSTRAP).clone()?;

    let hmonitor = unsafe { MonitorFromPoint(POINT { x: 0, y: 0 }, MONITOR_DEFAULTTOPRIMARY) };
    let display_id: DisplayId = hmonitor.try_into()?;
    assert_eq!(hmonitor, display_id.try_into()?);
    Ok(())
}
