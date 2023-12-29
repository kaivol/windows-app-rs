use std::cell::{Cell, RefCell};
use std::convert::TryFrom;
use std::mem::forget;

use windows::core::{factory, h, implement, ComInterface, IInspectable, Interface, RuntimeName, Type, PWSTR, TryIntoParam, AsImpl};
use windows::ApplicationModel::Package;
use windows::Win32::Foundation;
use windows::Win32::Foundation::{HWND, RECT};
use windows::Win32::Storage::Packaging::Appx::GetCurrentPackageFullName;
use windows::Win32::UI::HiDpi::GetDpiForWindow;
use windows::Win32::UI::WindowsAndMessaging::{
    GetSystemMetrics, GetWindowRect, SetWindowPos, SM_CXSCREEN, SM_CYSCREEN, SWP_NOMOVE, SWP_NOSIZE,
};
use windows_app::bootstrap;
use windows_app::Microsoft::UI::Xaml::Controls::{
    Button, IButtonFactory, IStackPanelFactory, StackPanel, XamlControlsResources
};
use windows_app::Microsoft::UI::Xaml::Markup::{
    IXamlMetadataProvider, IXamlMetadataProvider_Impl, XmlnsDefinition,
};
use windows_app::Microsoft::UI::Xaml::Media::SolidColorBrush;
use windows_app::Microsoft::UI::Xaml::XamlTypeInfo::XamlControlsXamlMetaDataProvider;
use windows_app::Microsoft::UI::Xaml::{Application, ApplicationInitializationCallback, HorizontalAlignment, IApplicationFactory, IApplicationOverrides, IApplicationOverrides_Impl, IWindowFactory, IWindowNative, LaunchActivatedEventArgs, ResourceDictionary, RoutedEventHandler, Window};

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
        let app = App::new()?;
        let app: IApplicationOverrides = app.into();
        let factory = factory::<Application, IApplicationFactory>().unwrap();
        let mut base: Option<IInspectable> = None;
        let mut result = std::mem::zeroed();
        let _application = (Interface::vtable(&factory).CreateInstance)(
            Interface::as_raw(&factory),
            Interface::as_raw(&app),
            &mut base as *mut _ as _,
            &mut result,
        )
        .from_abi::<Application>(result);
        let app = AsImpl::<App>::as_impl(&app);
        app.base.set(base);

        // forget(base);

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
    base: Cell<Option<IInspectable>>,
}

#[allow(non_snake_case)]
impl App {
    fn new() -> windows::core::Result<App> {
        let app = App {
            window: RefCell::new(None),
            provider: XamlControlsXamlMetaDataProvider::new()?,
            base: Cell::new(None),
        };
        Ok(app)
    }
}

impl IApplicationOverrides_Impl for App {
    fn OnLaunched(&self, _: Option<&LaunchActivatedEventArgs>) -> windows::core::Result<()> {
        let resources = XamlControlsResources::new()?;
        let resources: ResourceDictionary = resources.cast()?;
        // Application::Current()?
        //     .Resources()?
        //     .MergedDictionaries()?
        //     .Append(&resources)?;
        let app = self.base.take().unwrap();
        app.cast::<Application>().unwrap()
            .Resources()?
            .MergedDictionaries()?
            .Append(&resources)?;
        self.base.set(Some(app));
        unsafe {
            let window = new::<Window, IWindowFactory>(|f| f.CreateInstance)?;

            window.SetTitle(h!("WinUI 3 Desktop, Unpackaged (Rust)"))?;

            let mut hwnd = HWND::default();
            window.cast::<IWindowNative>()?.WindowHandle(&mut hwnd)?;

            let package = Package::Current();
            let text = if package.is_ok() {
                "package"
            } else {
                "nopackage"
            };

            let panel = new::<StackPanel, IStackPanelFactory>(|f| f.CreateInstance)?;
            // panel.SetBackground(&SolidColorBrush::CreateInstanceWithColor(
            //     windows_app::Microsoft::UI::Colors::White()?,
            // )?)?;

            let button = new::<Button, IButtonFactory>(|f| f.CreateInstance)?;
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

unsafe fn new<C: Type<C, Abi = *mut ::core::ffi::c_void> + RuntimeName, I: ComInterface>(
    create_instance: fn(
        &I::Vtable,
    ) -> unsafe extern "system" fn(
        *mut ::core::ffi::c_void,
        *mut ::core::ffi::c_void,
        *mut *mut ::core::ffi::c_void,
        *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
) -> windows::core::Result<C> {
    let factory: I = factory::<C, I>()?;

    let mut result__ = std::mem::zeroed();
    create_instance(::windows::core::Interface::vtable(&factory))(
        ::windows::core::Interface::as_raw(&factory),
        ::core::ptr::null_mut(),
        &mut ::core::option::Option::<::windows::core::IInspectable>::None as *mut _ as _,
        &mut result__,
    )
    .from_abi::<C>(result__)
}
