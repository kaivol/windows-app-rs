use std::cell::RefCell;
use std::convert::TryFrom;

use windows::core::{h, ComInterface, IInspectable, PWSTR};
use windows::ApplicationModel::Package;
use windows::Win32::Foundation;
use windows::Win32::Foundation::{HWND, RECT};
use windows::Win32::Storage::Packaging::Appx::GetCurrentPackageFullName;
use windows::Win32::UI::HiDpi::GetDpiForWindow;
use windows::Win32::UI::WindowsAndMessaging::{
    GetSystemMetrics, GetWindowRect, SetWindowPos, SM_CXSCREEN, SM_CYSCREEN, SWP_NOMOVE, SWP_NOSIZE,
};
use windows_app::bootstrap;
use windows_app::Microsoft::UI::Xaml::Controls::{Button, StackPanel, XamlControlsResources};
use windows_app::Microsoft::UI::Xaml::Markup::{
    IXamlMetadataProvider, IXamlMetadataProvider_Impl, XmlnsDefinition,
};
use windows_app::Microsoft::UI::Xaml::XamlTypeInfo::XamlControlsXamlMetaDataProvider;
use windows_app::Microsoft::UI::Xaml::{
    Application, ApplicationInitializationCallback, HorizontalAlignment, IApplicationOverrides,
    IApplicationOverrides_Impl, IWindowNative, LaunchActivatedEventArgs, ResourceDictionary,
    RoutedEventHandler, Window,
};
use windows_implement::implement;

fn main() -> windows::core::Result<()> {
    let package = unsafe {
        match GetCurrentPackageFullName(&mut 0, PWSTR::null()) {
            Err(e) => e.code() == Foundation::ERROR_INSUFFICIENT_BUFFER.into(),
            Ok(_) => true,
        }
    };
    if !package {
        bootstrap::initialize()?;
    }

    Application::Start(&ApplicationInitializationCallback::new(|_| unsafe {
        let (app, base) = App::compose(App::new()?);
        _ = Application::CreateInstance(&app, base)?;
        Ok(())
    }))?;

    if !package {
        bootstrap::uninitialize()?;
    }
    Ok(())
}

#[implement(IApplicationOverrides, IXamlMetadataProvider)]
struct App {
    window: RefCell<Option<Window>>,
    provider: XamlControlsXamlMetaDataProvider,
}

#[allow(non_snake_case)]
impl App {
    fn new() -> windows::core::Result<App> {
        let app = App {
            window: RefCell::new(None),
            provider: XamlControlsXamlMetaDataProvider::new()?,
        };
        Ok(app)
    }
}

impl IApplicationOverrides_Impl for App {
    fn OnLaunched(&self, _: Option<&LaunchActivatedEventArgs>) -> windows::core::Result<()> {
        let resources: ResourceDictionary = XamlControlsResources::new()?.cast()?;
        Application::Current()?.Resources()?.MergedDictionaries()?.Append(&resources)?;

        let window = unsafe { Window::CreateInstance(None, &mut None)? };

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

        let panel = unsafe { StackPanel::CreateInstance(None, &mut None)? };

        let button = unsafe { Button::CreateInstance(None, &mut None)? };
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

        resize_window(hwnd, 800, 600)?;
        center_window(hwnd)?;

        let result = window.Activate();
        *self.window.borrow_mut() = Some(window);
        result
    }
}

impl IXamlMetadataProvider_Impl for App {
    fn GetXamlType(
        &self,
        r#type: &windows_app::Windows::UI::Xaml::Interop::TypeName,
    ) -> windows::core::Result<windows_app::Microsoft::UI::Xaml::Markup::IXamlType> {
        self.provider.GetXamlType(r#type)
    }

    fn GetXamlTypeByFullName(
        &self,
        fullname: &windows::core::HSTRING,
    ) -> windows::core::Result<windows_app::Microsoft::UI::Xaml::Markup::IXamlType> {
        self.provider.GetXamlTypeByFullName(fullname)
    }

    fn GetXmlnsDefinitions(&self) -> windows::core::Result<windows::core::Array<XmlnsDefinition>> {
        self.provider.GetXmlnsDefinitions()
    }
}

pub fn resize_window(handle: HWND, width: u32, height: u32) -> windows::core::Result<()> {
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
    }
}

pub fn center_window(handle: HWND) -> windows::core::Result<()> {
    let mut rect = RECT::default();
    unsafe {
        GetWindowRect(handle, &mut rect)?;
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
    }
}
