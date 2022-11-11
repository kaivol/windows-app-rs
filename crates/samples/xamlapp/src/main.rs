use std::cell::RefCell;
use std::convert::TryFrom;

use windows::core::{implement, IInspectable, Interface, PWSTR};
use windows::h;
use windows::ApplicationModel::Package;
use windows::Win32::Foundation;
use windows::Win32::Foundation::{HWND, RECT};
use windows::Win32::Storage::Packaging::Appx::GetCurrentPackageFullName;
use windows::Win32::UI::HiDpi::GetDpiForWindow;
use windows::Win32::UI::WindowsAndMessaging::{
    GetSystemMetrics, GetWindowRect, SetWindowPos, SM_CXSCREEN, SM_CYSCREEN, SWP_NOMOVE, SWP_NOSIZE,
};
use windows_app::bootstrap;
use windows_app::UI::Xaml::Controls::{Button, StackPanel};
use windows_app::UI::Xaml::Media::SolidColorBrush;
use windows_app::UI::Xaml::{
    Application, ApplicationInitializationCallback, HorizontalAlignment, IApplicationOverrides,
    IApplicationOverrides_Impl, IWindowNative, LaunchActivatedEventArgs, RoutedEventHandler,
    Window,
};

fn main() -> windows::core::Result<()> {
    let package = unsafe {
        GetCurrentPackageFullName(&mut 0, PWSTR::null()) == Foundation::ERROR_INSUFFICIENT_BUFFER
    };
    if !package {
        bootstrap::initialize()?;
    }

    Application::Start(&ApplicationInitializationCallback::new(|_| {
        let _ = Application::compose(App::new()?)?;
        Ok(())
    }))?;

    if !package {
        bootstrap::uninitialize()?;
    }
    Ok(())
}

#[implement(IApplicationOverrides)]
struct App {
    _window: RefCell<Option<Window>>,
}

#[allow(non_snake_case)]
impl App {
    fn new() -> windows::core::Result<App> {
        let app = App {
            _window: RefCell::new(None),
        };
        Ok(app)
    }
}

impl IApplicationOverrides_Impl for App {
    fn OnLaunched(&self, _: &Option<LaunchActivatedEventArgs>) -> windows::core::Result<()> {
        let window = Window::new()?;
        window.SetTitle(h!("WinUI 3 Desktop, Unpackaged (Rust)"))?;

        let mut hwnd = HWND::default();
        unsafe {
            window.cast::<IWindowNative>()?.WindowHandle(&mut hwnd)?;
        }

        let package = Package::Current();
        let text = if package.is_ok() {
            "package"
        } else {
            "nopackage"
        };

        let panel = StackPanel::new()?;
        panel.SetBackground(&SolidColorBrush::CreateInstanceWithColor(
            windows_app::UI::Colors::Transparent()?,
        )?)?;

        let button = Button::new()?;
        button.SetContent(&IInspectable::try_from(text)?)?;
        button.SetHorizontalAlignment(HorizontalAlignment::Center)?;
        button.Click(&RoutedEventHandler::new(|sender, _args| {
            if let Some(button) = sender {
                button.cast::<Button>()?.SetContent(&IInspectable::try_from("Clicked! 🦀")?)?;
            }
            Ok(())
        }))?;
        panel.Children()?.Append(&button)?;

        window.SetContent(&panel)?;

        resize_window(hwnd, 800, 600).then(|| {
            center_window(hwnd);
        });

        let result = window.Activate();
        *self._window.borrow_mut() = Some(window);
        result
    }
}

pub fn resize_window(handle: HWND, width: u32, height: u32) -> bool {
    let scale_factor = unsafe { GetDpiForWindow(handle) } / 96;
    let width = width * scale_factor;
    let height = height * scale_factor;
    unsafe {
        SetWindowPos(
            handle,
            HWND(0),
            0, // x
            0, // y
            width as i32,
            height as i32,
            SWP_NOMOVE,
        )
        .into()
    }
}

pub fn center_window(handle: HWND) -> bool {
    let mut rect = RECT::default();
    unsafe {
        if GetWindowRect(handle, &mut rect).as_bool() {
            let screen_width = GetSystemMetrics(SM_CXSCREEN);
            let screen_height = GetSystemMetrics(SM_CYSCREEN);
            SetWindowPos(
                handle,
                HWND(0),
                (screen_width / 2) - (rect.right - rect.left) / 2,
                (screen_height / 2) - (rect.bottom - rect.top) / 2,
                0, // cx
                0, // cy
                SWP_NOSIZE,
            )
            .into()
        } else {
            false
        }
    }
}
