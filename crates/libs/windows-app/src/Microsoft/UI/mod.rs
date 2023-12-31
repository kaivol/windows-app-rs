#[cfg(feature = "Microsoft_UI_Composition")]
#[doc = "Required features: `\"Microsoft_UI_Composition\"`"]
pub mod Composition;
#[cfg(feature = "Microsoft_UI_Content")]
#[doc = "Required features: `\"Microsoft_UI_Content\"`"]
pub mod Content;
#[cfg(feature = "Microsoft_UI_Dispatching")]
#[doc = "Required features: `\"Microsoft_UI_Dispatching\"`"]
pub mod Dispatching;
#[cfg(feature = "Microsoft_UI_Input")]
#[doc = "Required features: `\"Microsoft_UI_Input\"`"]
pub mod Input;
#[cfg(feature = "Microsoft_UI_System")]
#[doc = "Required features: `\"Microsoft_UI_System\"`"]
pub mod System;
#[cfg(feature = "Microsoft_UI_Text")]
#[doc = "Required features: `\"Microsoft_UI_Text\"`"]
pub mod Text;
#[cfg(feature = "Microsoft_UI_Windowing")]
#[doc = "Required features: `\"Microsoft_UI_Windowing\"`"]
pub mod Windowing;
#[cfg(feature = "Microsoft_UI_Xaml")]
#[doc = "Required features: `\"Microsoft_UI_Xaml\"`"]
pub mod Xaml;
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IClosableNotifier(::windows_core::IUnknown);
impl IClosableNotifier {
    pub fn IsClosed(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsClosed)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn Closed<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<ClosableNotifierHandler>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Closed)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveClosed(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveClosed)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn FrameworkClosed<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<ClosableNotifierHandler>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FrameworkClosed)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveFrameworkClosed(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveFrameworkClosed)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
}
::windows_core::imp::interface_hierarchy!(
    IClosableNotifier,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::RuntimeType for IClosableNotifier {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_interface::<Self>();
}
unsafe impl ::windows_core::Interface for IClosableNotifier {
    type Vtable = IClosableNotifier_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IClosableNotifier {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x2989e93b_ed0f_5e79_90f2_eac592fc6e6a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClosableNotifier_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub IsClosed: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_Foundation")]
    pub Closed: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    Closed: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub RemoveClosed: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RemoveClosed: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub FrameworkClosed: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    FrameworkClosed: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub RemoveFrameworkClosed: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RemoveFrameworkClosed: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IColorHelper(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IColorHelper {
    type Vtable = IColorHelper_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IColorHelper {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x3adddccd_3949_585b_a566_ccb8350dd221);
}
#[repr(C)]
#[doc(hidden)]
pub struct IColorHelper_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IColorHelperStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IColorHelperStatics {
    type Vtable = IColorHelperStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IColorHelperStatics {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x1d1d85a1_eb63_538a_84f0_019210bc406b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IColorHelperStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Windows_UI")]
    pub FromArgb: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        a: u8,
        r: u8,
        g: u8,
        b: u8,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI"))]
    FromArgb: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IColors(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IColors {
    type Vtable = IColors_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IColors {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x8cf15863_8411_5afd_946c_328e04da2f2f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IColors_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IColorsStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IColorsStatics {
    type Vtable = IColorsStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IColorsStatics {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x8620a5b0_015a_57ac_a3f3_895d0b1269ae);
}
#[repr(C)]
#[doc(hidden)]
pub struct IColorsStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Windows_UI")]
    pub AliceBlue: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI"))]
    AliceBlue: usize,
    #[cfg(feature = "Windows_UI")]
    pub AntiqueWhite: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI"))]
    AntiqueWhite: usize,
    #[cfg(feature = "Windows_UI")]
    pub Aqua: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI"))]
    Aqua: usize,
    #[cfg(feature = "Windows_UI")]
    pub Aquamarine: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI"))]
    Aquamarine: usize,
    #[cfg(feature = "Windows_UI")]
    pub Azure: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI"))]
    Azure: usize,
    #[cfg(feature = "Windows_UI")]
    pub Beige: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI"))]
    Beige: usize,
    #[cfg(feature = "Windows_UI")]
    pub Bisque: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI"))]
    Bisque: usize,
    #[cfg(feature = "Windows_UI")]
    pub Black: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI"))]
    Black: usize,
    #[cfg(feature = "Windows_UI")]
    pub BlanchedAlmond: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI"))]
    BlanchedAlmond: usize,
    #[cfg(feature = "Windows_UI")]
    pub Blue: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI"))]
    Blue: usize,
    #[cfg(feature = "Windows_UI")]
    pub BlueViolet: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI"))]
    BlueViolet: usize,
    #[cfg(feature = "Windows_UI")]
    pub Brown: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI"))]
    Brown: usize,
    #[cfg(feature = "Windows_UI")]
    pub BurlyWood: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI"))]
    BurlyWood: usize,
    #[cfg(feature = "Windows_UI")]
    pub CadetBlue: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI"))]
    CadetBlue: usize,
    #[cfg(feature = "Windows_UI")]
    pub Chartreuse: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI"))]
    Chartreuse: usize,
    #[cfg(feature = "Windows_UI")]
    pub Chocolate: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI"))]
    Chocolate: usize,
    #[cfg(feature = "Windows_UI")]
    pub Coral: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI"))]
    Coral: usize,
    #[cfg(feature = "Windows_UI")]
    pub CornflowerBlue: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI"))]
    CornflowerBlue: usize,
    #[cfg(feature = "Windows_UI")]
    pub Cornsilk: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI"))]
    Cornsilk: usize,
    #[cfg(feature = "Windows_UI")]
    pub Crimson: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI"))]
    Crimson: usize,
    #[cfg(feature = "Windows_UI")]
    pub Cyan: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI"))]
    Cyan: usize,
    #[cfg(feature = "Windows_UI")]
    pub DarkBlue: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI"))]
    DarkBlue: usize,
    #[cfg(feature = "Windows_UI")]
    pub DarkCyan: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI"))]
    DarkCyan: usize,
    #[cfg(feature = "Windows_UI")]
    pub DarkGoldenrod: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI"))]
    DarkGoldenrod: usize,
    #[cfg(feature = "Windows_UI")]
    pub DarkGray: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI"))]
    DarkGray: usize,
    #[cfg(feature = "Windows_UI")]
    pub DarkGreen: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI"))]
    DarkGreen: usize,
    #[cfg(feature = "Windows_UI")]
    pub DarkKhaki: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI"))]
    DarkKhaki: usize,
    #[cfg(feature = "Windows_UI")]
    pub DarkMagenta: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI"))]
    DarkMagenta: usize,
    #[cfg(feature = "Windows_UI")]
    pub DarkOliveGreen: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI"))]
    DarkOliveGreen: usize,
    #[cfg(feature = "Windows_UI")]
    pub DarkOrange: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI"))]
    DarkOrange: usize,
    #[cfg(feature = "Windows_UI")]
    pub DarkOrchid: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI"))]
    DarkOrchid: usize,
    #[cfg(feature = "Windows_UI")]
    pub DarkRed: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI"))]
    DarkRed: usize,
    #[cfg(feature = "Windows_UI")]
    pub DarkSalmon: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI"))]
    DarkSalmon: usize,
    #[cfg(feature = "Windows_UI")]
    pub DarkSeaGreen: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI"))]
    DarkSeaGreen: usize,
    #[cfg(feature = "Windows_UI")]
    pub DarkSlateBlue: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI"))]
    DarkSlateBlue: usize,
    #[cfg(feature = "Windows_UI")]
    pub DarkSlateGray: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI"))]
    DarkSlateGray: usize,
    #[cfg(feature = "Windows_UI")]
    pub DarkTurquoise: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI"))]
    DarkTurquoise: usize,
    #[cfg(feature = "Windows_UI")]
    pub DarkViolet: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI"))]
    DarkViolet: usize,
    #[cfg(feature = "Windows_UI")]
    pub DeepPink: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI"))]
    DeepPink: usize,
    #[cfg(feature = "Windows_UI")]
    pub DeepSkyBlue: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI"))]
    DeepSkyBlue: usize,
    #[cfg(feature = "Windows_UI")]
    pub DimGray: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI"))]
    DimGray: usize,
    #[cfg(feature = "Windows_UI")]
    pub DodgerBlue: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI"))]
    DodgerBlue: usize,
    #[cfg(feature = "Windows_UI")]
    pub Firebrick: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI"))]
    Firebrick: usize,
    #[cfg(feature = "Windows_UI")]
    pub FloralWhite: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI"))]
    FloralWhite: usize,
    #[cfg(feature = "Windows_UI")]
    pub ForestGreen: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI"))]
    ForestGreen: usize,
    #[cfg(feature = "Windows_UI")]
    pub Fuchsia: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI"))]
    Fuchsia: usize,
    #[cfg(feature = "Windows_UI")]
    pub Gainsboro: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI"))]
    Gainsboro: usize,
    #[cfg(feature = "Windows_UI")]
    pub GhostWhite: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI"))]
    GhostWhite: usize,
    #[cfg(feature = "Windows_UI")]
    pub Gold: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI"))]
    Gold: usize,
    #[cfg(feature = "Windows_UI")]
    pub Goldenrod: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI"))]
    Goldenrod: usize,
    #[cfg(feature = "Windows_UI")]
    pub Gray: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI"))]
    Gray: usize,
    #[cfg(feature = "Windows_UI")]
    pub Green: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI"))]
    Green: usize,
    #[cfg(feature = "Windows_UI")]
    pub GreenYellow: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI"))]
    GreenYellow: usize,
    #[cfg(feature = "Windows_UI")]
    pub Honeydew: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI"))]
    Honeydew: usize,
    #[cfg(feature = "Windows_UI")]
    pub HotPink: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI"))]
    HotPink: usize,
    #[cfg(feature = "Windows_UI")]
    pub IndianRed: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI"))]
    IndianRed: usize,
    #[cfg(feature = "Windows_UI")]
    pub Indigo: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI"))]
    Indigo: usize,
    #[cfg(feature = "Windows_UI")]
    pub Ivory: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI"))]
    Ivory: usize,
    #[cfg(feature = "Windows_UI")]
    pub Khaki: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI"))]
    Khaki: usize,
    #[cfg(feature = "Windows_UI")]
    pub Lavender: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI"))]
    Lavender: usize,
    #[cfg(feature = "Windows_UI")]
    pub LavenderBlush: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI"))]
    LavenderBlush: usize,
    #[cfg(feature = "Windows_UI")]
    pub LawnGreen: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI"))]
    LawnGreen: usize,
    #[cfg(feature = "Windows_UI")]
    pub LemonChiffon: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI"))]
    LemonChiffon: usize,
    #[cfg(feature = "Windows_UI")]
    pub LightBlue: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI"))]
    LightBlue: usize,
    #[cfg(feature = "Windows_UI")]
    pub LightCoral: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI"))]
    LightCoral: usize,
    #[cfg(feature = "Windows_UI")]
    pub LightCyan: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI"))]
    LightCyan: usize,
    #[cfg(feature = "Windows_UI")]
    pub LightGoldenrodYellow: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI"))]
    LightGoldenrodYellow: usize,
    #[cfg(feature = "Windows_UI")]
    pub LightGreen: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI"))]
    LightGreen: usize,
    #[cfg(feature = "Windows_UI")]
    pub LightGray: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI"))]
    LightGray: usize,
    #[cfg(feature = "Windows_UI")]
    pub LightPink: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI"))]
    LightPink: usize,
    #[cfg(feature = "Windows_UI")]
    pub LightSalmon: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI"))]
    LightSalmon: usize,
    #[cfg(feature = "Windows_UI")]
    pub LightSeaGreen: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI"))]
    LightSeaGreen: usize,
    #[cfg(feature = "Windows_UI")]
    pub LightSkyBlue: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI"))]
    LightSkyBlue: usize,
    #[cfg(feature = "Windows_UI")]
    pub LightSlateGray: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI"))]
    LightSlateGray: usize,
    #[cfg(feature = "Windows_UI")]
    pub LightSteelBlue: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI"))]
    LightSteelBlue: usize,
    #[cfg(feature = "Windows_UI")]
    pub LightYellow: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI"))]
    LightYellow: usize,
    #[cfg(feature = "Windows_UI")]
    pub Lime: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI"))]
    Lime: usize,
    #[cfg(feature = "Windows_UI")]
    pub LimeGreen: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI"))]
    LimeGreen: usize,
    #[cfg(feature = "Windows_UI")]
    pub Linen: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI"))]
    Linen: usize,
    #[cfg(feature = "Windows_UI")]
    pub Magenta: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI"))]
    Magenta: usize,
    #[cfg(feature = "Windows_UI")]
    pub Maroon: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI"))]
    Maroon: usize,
    #[cfg(feature = "Windows_UI")]
    pub MediumAquamarine: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI"))]
    MediumAquamarine: usize,
    #[cfg(feature = "Windows_UI")]
    pub MediumBlue: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI"))]
    MediumBlue: usize,
    #[cfg(feature = "Windows_UI")]
    pub MediumOrchid: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI"))]
    MediumOrchid: usize,
    #[cfg(feature = "Windows_UI")]
    pub MediumPurple: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI"))]
    MediumPurple: usize,
    #[cfg(feature = "Windows_UI")]
    pub MediumSeaGreen: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI"))]
    MediumSeaGreen: usize,
    #[cfg(feature = "Windows_UI")]
    pub MediumSlateBlue: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI"))]
    MediumSlateBlue: usize,
    #[cfg(feature = "Windows_UI")]
    pub MediumSpringGreen: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI"))]
    MediumSpringGreen: usize,
    #[cfg(feature = "Windows_UI")]
    pub MediumTurquoise: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI"))]
    MediumTurquoise: usize,
    #[cfg(feature = "Windows_UI")]
    pub MediumVioletRed: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI"))]
    MediumVioletRed: usize,
    #[cfg(feature = "Windows_UI")]
    pub MidnightBlue: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI"))]
    MidnightBlue: usize,
    #[cfg(feature = "Windows_UI")]
    pub MintCream: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI"))]
    MintCream: usize,
    #[cfg(feature = "Windows_UI")]
    pub MistyRose: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI"))]
    MistyRose: usize,
    #[cfg(feature = "Windows_UI")]
    pub Moccasin: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI"))]
    Moccasin: usize,
    #[cfg(feature = "Windows_UI")]
    pub NavajoWhite: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI"))]
    NavajoWhite: usize,
    #[cfg(feature = "Windows_UI")]
    pub Navy: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI"))]
    Navy: usize,
    #[cfg(feature = "Windows_UI")]
    pub OldLace: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI"))]
    OldLace: usize,
    #[cfg(feature = "Windows_UI")]
    pub Olive: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI"))]
    Olive: usize,
    #[cfg(feature = "Windows_UI")]
    pub OliveDrab: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI"))]
    OliveDrab: usize,
    #[cfg(feature = "Windows_UI")]
    pub Orange: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI"))]
    Orange: usize,
    #[cfg(feature = "Windows_UI")]
    pub OrangeRed: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI"))]
    OrangeRed: usize,
    #[cfg(feature = "Windows_UI")]
    pub Orchid: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI"))]
    Orchid: usize,
    #[cfg(feature = "Windows_UI")]
    pub PaleGoldenrod: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI"))]
    PaleGoldenrod: usize,
    #[cfg(feature = "Windows_UI")]
    pub PaleGreen: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI"))]
    PaleGreen: usize,
    #[cfg(feature = "Windows_UI")]
    pub PaleTurquoise: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI"))]
    PaleTurquoise: usize,
    #[cfg(feature = "Windows_UI")]
    pub PaleVioletRed: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI"))]
    PaleVioletRed: usize,
    #[cfg(feature = "Windows_UI")]
    pub PapayaWhip: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI"))]
    PapayaWhip: usize,
    #[cfg(feature = "Windows_UI")]
    pub PeachPuff: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI"))]
    PeachPuff: usize,
    #[cfg(feature = "Windows_UI")]
    pub Peru: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI"))]
    Peru: usize,
    #[cfg(feature = "Windows_UI")]
    pub Pink: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI"))]
    Pink: usize,
    #[cfg(feature = "Windows_UI")]
    pub Plum: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI"))]
    Plum: usize,
    #[cfg(feature = "Windows_UI")]
    pub PowderBlue: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI"))]
    PowderBlue: usize,
    #[cfg(feature = "Windows_UI")]
    pub Purple: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI"))]
    Purple: usize,
    #[cfg(feature = "Windows_UI")]
    pub Red: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI"))]
    Red: usize,
    #[cfg(feature = "Windows_UI")]
    pub RosyBrown: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI"))]
    RosyBrown: usize,
    #[cfg(feature = "Windows_UI")]
    pub RoyalBlue: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI"))]
    RoyalBlue: usize,
    #[cfg(feature = "Windows_UI")]
    pub SaddleBrown: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI"))]
    SaddleBrown: usize,
    #[cfg(feature = "Windows_UI")]
    pub Salmon: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI"))]
    Salmon: usize,
    #[cfg(feature = "Windows_UI")]
    pub SandyBrown: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI"))]
    SandyBrown: usize,
    #[cfg(feature = "Windows_UI")]
    pub SeaGreen: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI"))]
    SeaGreen: usize,
    #[cfg(feature = "Windows_UI")]
    pub SeaShell: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI"))]
    SeaShell: usize,
    #[cfg(feature = "Windows_UI")]
    pub Sienna: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI"))]
    Sienna: usize,
    #[cfg(feature = "Windows_UI")]
    pub Silver: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI"))]
    Silver: usize,
    #[cfg(feature = "Windows_UI")]
    pub SkyBlue: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI"))]
    SkyBlue: usize,
    #[cfg(feature = "Windows_UI")]
    pub SlateBlue: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI"))]
    SlateBlue: usize,
    #[cfg(feature = "Windows_UI")]
    pub SlateGray: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI"))]
    SlateGray: usize,
    #[cfg(feature = "Windows_UI")]
    pub Snow: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI"))]
    Snow: usize,
    #[cfg(feature = "Windows_UI")]
    pub SpringGreen: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI"))]
    SpringGreen: usize,
    #[cfg(feature = "Windows_UI")]
    pub SteelBlue: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI"))]
    SteelBlue: usize,
    #[cfg(feature = "Windows_UI")]
    pub Tan: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI"))]
    Tan: usize,
    #[cfg(feature = "Windows_UI")]
    pub Teal: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI"))]
    Teal: usize,
    #[cfg(feature = "Windows_UI")]
    pub Thistle: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI"))]
    Thistle: usize,
    #[cfg(feature = "Windows_UI")]
    pub Tomato: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI"))]
    Tomato: usize,
    #[cfg(feature = "Windows_UI")]
    pub Transparent: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI"))]
    Transparent: usize,
    #[cfg(feature = "Windows_UI")]
    pub Turquoise: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI"))]
    Turquoise: usize,
    #[cfg(feature = "Windows_UI")]
    pub Violet: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI"))]
    Violet: usize,
    #[cfg(feature = "Windows_UI")]
    pub Wheat: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI"))]
    Wheat: usize,
    #[cfg(feature = "Windows_UI")]
    pub White: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI"))]
    White: usize,
    #[cfg(feature = "Windows_UI")]
    pub WhiteSmoke: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI"))]
    WhiteSmoke: usize,
    #[cfg(feature = "Windows_UI")]
    pub Yellow: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI"))]
    Yellow: usize,
    #[cfg(feature = "Windows_UI")]
    pub YellowGreen: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI"))]
    YellowGreen: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ColorHelper(::windows_core::IUnknown);
impl ColorHelper {
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn FromArgb(a: u8, r: u8, g: u8, b: u8) -> ::windows_core::Result<::windows::UI::Color> {
        Self::IColorHelperStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FromArgb)(
                ::windows_core::Interface::as_raw(this),
                a,
                r,
                g,
                b,
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IColorHelperStatics<R, F: FnOnce(&IColorHelperStatics) -> ::windows_core::Result<R>>(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<ColorHelper, IColorHelperStatics> =
            ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for ColorHelper {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for ColorHelper {
    type Vtable = IColorHelper_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ColorHelper {
    const IID: ::windows_core::GUID = <IColorHelper as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ColorHelper {
    const NAME: &'static str = "Microsoft.UI.ColorHelper";
}
::windows_core::imp::interface_hierarchy!(
    ColorHelper,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for ColorHelper {}
unsafe impl ::core::marker::Sync for ColorHelper {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct Colors(::windows_core::IUnknown);
impl Colors {
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn AliceBlue() -> ::windows_core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AliceBlue)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn AntiqueWhite() -> ::windows_core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AntiqueWhite)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn Aqua() -> ::windows_core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Aqua)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn Aquamarine() -> ::windows_core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Aquamarine)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn Azure() -> ::windows_core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Azure)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn Beige() -> ::windows_core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Beige)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn Bisque() -> ::windows_core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Bisque)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn Black() -> ::windows_core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Black)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn BlanchedAlmond() -> ::windows_core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).BlanchedAlmond)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn Blue() -> ::windows_core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Blue)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn BlueViolet() -> ::windows_core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).BlueViolet)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn Brown() -> ::windows_core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Brown)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn BurlyWood() -> ::windows_core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).BurlyWood)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn CadetBlue() -> ::windows_core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CadetBlue)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn Chartreuse() -> ::windows_core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Chartreuse)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn Chocolate() -> ::windows_core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Chocolate)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn Coral() -> ::windows_core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Coral)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn CornflowerBlue() -> ::windows_core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CornflowerBlue)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn Cornsilk() -> ::windows_core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Cornsilk)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn Crimson() -> ::windows_core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Crimson)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn Cyan() -> ::windows_core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Cyan)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn DarkBlue() -> ::windows_core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DarkBlue)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn DarkCyan() -> ::windows_core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DarkCyan)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn DarkGoldenrod() -> ::windows_core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DarkGoldenrod)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn DarkGray() -> ::windows_core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DarkGray)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn DarkGreen() -> ::windows_core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DarkGreen)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn DarkKhaki() -> ::windows_core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DarkKhaki)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn DarkMagenta() -> ::windows_core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DarkMagenta)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn DarkOliveGreen() -> ::windows_core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DarkOliveGreen)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn DarkOrange() -> ::windows_core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DarkOrange)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn DarkOrchid() -> ::windows_core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DarkOrchid)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn DarkRed() -> ::windows_core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DarkRed)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn DarkSalmon() -> ::windows_core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DarkSalmon)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn DarkSeaGreen() -> ::windows_core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DarkSeaGreen)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn DarkSlateBlue() -> ::windows_core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DarkSlateBlue)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn DarkSlateGray() -> ::windows_core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DarkSlateGray)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn DarkTurquoise() -> ::windows_core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DarkTurquoise)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn DarkViolet() -> ::windows_core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DarkViolet)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn DeepPink() -> ::windows_core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DeepPink)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn DeepSkyBlue() -> ::windows_core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DeepSkyBlue)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn DimGray() -> ::windows_core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DimGray)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn DodgerBlue() -> ::windows_core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DodgerBlue)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn Firebrick() -> ::windows_core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Firebrick)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn FloralWhite() -> ::windows_core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FloralWhite)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn ForestGreen() -> ::windows_core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ForestGreen)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn Fuchsia() -> ::windows_core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Fuchsia)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn Gainsboro() -> ::windows_core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Gainsboro)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn GhostWhite() -> ::windows_core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GhostWhite)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn Gold() -> ::windows_core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Gold)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn Goldenrod() -> ::windows_core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Goldenrod)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn Gray() -> ::windows_core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Gray)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn Green() -> ::windows_core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Green)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn GreenYellow() -> ::windows_core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GreenYellow)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn Honeydew() -> ::windows_core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Honeydew)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn HotPink() -> ::windows_core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HotPink)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn IndianRed() -> ::windows_core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IndianRed)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn Indigo() -> ::windows_core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Indigo)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn Ivory() -> ::windows_core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Ivory)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn Khaki() -> ::windows_core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Khaki)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn Lavender() -> ::windows_core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Lavender)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn LavenderBlush() -> ::windows_core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LavenderBlush)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn LawnGreen() -> ::windows_core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LawnGreen)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn LemonChiffon() -> ::windows_core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LemonChiffon)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn LightBlue() -> ::windows_core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LightBlue)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn LightCoral() -> ::windows_core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LightCoral)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn LightCyan() -> ::windows_core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LightCyan)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn LightGoldenrodYellow() -> ::windows_core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LightGoldenrodYellow)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn LightGreen() -> ::windows_core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LightGreen)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn LightGray() -> ::windows_core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LightGray)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn LightPink() -> ::windows_core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LightPink)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn LightSalmon() -> ::windows_core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LightSalmon)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn LightSeaGreen() -> ::windows_core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LightSeaGreen)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn LightSkyBlue() -> ::windows_core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LightSkyBlue)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn LightSlateGray() -> ::windows_core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LightSlateGray)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn LightSteelBlue() -> ::windows_core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LightSteelBlue)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn LightYellow() -> ::windows_core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LightYellow)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn Lime() -> ::windows_core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Lime)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn LimeGreen() -> ::windows_core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LimeGreen)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn Linen() -> ::windows_core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Linen)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn Magenta() -> ::windows_core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Magenta)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn Maroon() -> ::windows_core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Maroon)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn MediumAquamarine() -> ::windows_core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MediumAquamarine)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn MediumBlue() -> ::windows_core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MediumBlue)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn MediumOrchid() -> ::windows_core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MediumOrchid)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn MediumPurple() -> ::windows_core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MediumPurple)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn MediumSeaGreen() -> ::windows_core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MediumSeaGreen)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn MediumSlateBlue() -> ::windows_core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MediumSlateBlue)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn MediumSpringGreen() -> ::windows_core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MediumSpringGreen)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn MediumTurquoise() -> ::windows_core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MediumTurquoise)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn MediumVioletRed() -> ::windows_core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MediumVioletRed)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn MidnightBlue() -> ::windows_core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MidnightBlue)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn MintCream() -> ::windows_core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MintCream)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn MistyRose() -> ::windows_core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MistyRose)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn Moccasin() -> ::windows_core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Moccasin)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn NavajoWhite() -> ::windows_core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NavajoWhite)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn Navy() -> ::windows_core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Navy)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn OldLace() -> ::windows_core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OldLace)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn Olive() -> ::windows_core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Olive)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn OliveDrab() -> ::windows_core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OliveDrab)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn Orange() -> ::windows_core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Orange)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn OrangeRed() -> ::windows_core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OrangeRed)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn Orchid() -> ::windows_core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Orchid)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn PaleGoldenrod() -> ::windows_core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PaleGoldenrod)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn PaleGreen() -> ::windows_core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PaleGreen)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn PaleTurquoise() -> ::windows_core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PaleTurquoise)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn PaleVioletRed() -> ::windows_core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PaleVioletRed)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn PapayaWhip() -> ::windows_core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PapayaWhip)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn PeachPuff() -> ::windows_core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PeachPuff)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn Peru() -> ::windows_core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Peru)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn Pink() -> ::windows_core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Pink)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn Plum() -> ::windows_core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Plum)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn PowderBlue() -> ::windows_core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PowderBlue)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn Purple() -> ::windows_core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Purple)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn Red() -> ::windows_core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Red)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn RosyBrown() -> ::windows_core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RosyBrown)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn RoyalBlue() -> ::windows_core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RoyalBlue)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn SaddleBrown() -> ::windows_core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SaddleBrown)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn Salmon() -> ::windows_core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Salmon)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn SandyBrown() -> ::windows_core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SandyBrown)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn SeaGreen() -> ::windows_core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SeaGreen)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn SeaShell() -> ::windows_core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SeaShell)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn Sienna() -> ::windows_core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Sienna)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn Silver() -> ::windows_core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Silver)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn SkyBlue() -> ::windows_core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SkyBlue)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn SlateBlue() -> ::windows_core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SlateBlue)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn SlateGray() -> ::windows_core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SlateGray)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn Snow() -> ::windows_core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Snow)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn SpringGreen() -> ::windows_core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SpringGreen)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn SteelBlue() -> ::windows_core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SteelBlue)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn Tan() -> ::windows_core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Tan)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn Teal() -> ::windows_core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Teal)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn Thistle() -> ::windows_core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Thistle)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn Tomato() -> ::windows_core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Tomato)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn Transparent() -> ::windows_core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Transparent)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn Turquoise() -> ::windows_core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Turquoise)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn Violet() -> ::windows_core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Violet)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn Wheat() -> ::windows_core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Wheat)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn White() -> ::windows_core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).White)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn WhiteSmoke() -> ::windows_core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).WhiteSmoke)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn Yellow() -> ::windows_core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Yellow)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn YellowGreen() -> ::windows_core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).YellowGreen)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IColorsStatics<R, F: FnOnce(&IColorsStatics) -> ::windows_core::Result<R>>(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<Colors, IColorsStatics> =
            ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for Colors {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for Colors {
    type Vtable = IColors_Vtbl;
}
unsafe impl ::windows_core::ComInterface for Colors {
    const IID: ::windows_core::GUID = <IColors as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for Colors {
    const NAME: &'static str = "Microsoft.UI.Colors";
}
::windows_core::imp::interface_hierarchy!(
    Colors,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for Colors {}
unsafe impl ::core::marker::Sync for Colors {}
#[repr(C)]
pub struct DisplayId {
    pub Value: u64,
}
impl ::core::marker::Copy for DisplayId {}
impl ::core::clone::Clone for DisplayId {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DisplayId {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DisplayId").field("Value", &self.Value).finish()
    }
}
impl ::windows_core::TypeKind for DisplayId {
    type TypeKind = ::windows_core::CopyType;
}
impl ::windows_core::RuntimeType for DisplayId {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::from_slice(b"struct(Microsoft.UI.DisplayId;u8)");
}
impl ::core::cmp::PartialEq for DisplayId {
    fn eq(&self, other: &Self) -> bool {
        self.Value == other.Value
    }
}
impl ::core::cmp::Eq for DisplayId {}
impl ::core::default::Default for DisplayId {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct IconId {
    pub Value: u64,
}
impl ::core::marker::Copy for IconId {}
impl ::core::clone::Clone for IconId {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IconId {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IconId").field("Value", &self.Value).finish()
    }
}
impl ::windows_core::TypeKind for IconId {
    type TypeKind = ::windows_core::CopyType;
}
impl ::windows_core::RuntimeType for IconId {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::from_slice(b"struct(Microsoft.UI.IconId;u8)");
}
impl ::core::cmp::PartialEq for IconId {
    fn eq(&self, other: &Self) -> bool {
        self.Value == other.Value
    }
}
impl ::core::cmp::Eq for IconId {}
impl ::core::default::Default for IconId {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WindowId {
    pub Value: u64,
}
impl ::core::marker::Copy for WindowId {}
impl ::core::clone::Clone for WindowId {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WindowId {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WindowId").field("Value", &self.Value).finish()
    }
}
impl ::windows_core::TypeKind for WindowId {
    type TypeKind = ::windows_core::CopyType;
}
impl ::windows_core::RuntimeType for WindowId {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::from_slice(b"struct(Microsoft.UI.WindowId;u8)");
}
impl ::core::cmp::PartialEq for WindowId {
    fn eq(&self, other: &Self) -> bool {
        self.Value == other.Value
    }
}
impl ::core::cmp::Eq for WindowId {}
impl ::core::default::Default for WindowId {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ClosableNotifierHandler(pub ::windows_core::IUnknown);
impl ClosableNotifierHandler {
    pub fn new<F: FnMut() -> ::windows_core::Result<()> + ::core::marker::Send + 'static>(
        invoke: F,
    ) -> Self {
        let com = ClosableNotifierHandlerBox::<F> {
            vtable: &ClosableNotifierHandlerBox::<F>::VTABLE,
            count: ::windows_core::imp::RefCount::new(1),
            invoke,
        };
        unsafe { ::core::mem::transmute(::std::boxed::Box::new(com)) }
    }
    pub fn Invoke(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).Invoke)(::windows_core::Interface::as_raw(
                this,
            ))
            .ok()
        }
    }
}
#[repr(C)]
struct ClosableNotifierHandlerBox<
    F: FnMut() -> ::windows_core::Result<()> + ::core::marker::Send + 'static,
> {
    vtable: *const ClosableNotifierHandler_Vtbl,
    invoke: F,
    count: ::windows_core::imp::RefCount,
}
impl<F: FnMut() -> ::windows_core::Result<()> + ::core::marker::Send + 'static>
    ClosableNotifierHandlerBox<F>
{
    const VTABLE: ClosableNotifierHandler_Vtbl = ClosableNotifierHandler_Vtbl {
        base__: ::windows_core::IUnknown_Vtbl {
            QueryInterface: Self::QueryInterface,
            AddRef: Self::AddRef,
            Release: Self::Release,
        },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn QueryInterface(
        this: *mut ::core::ffi::c_void,
        iid: *const ::windows_core::GUID,
        interface: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        if iid.is_null() || interface.is_null() {
            return ::windows_core::HRESULT(-2147467261);
        }
        *interface = if *iid == <ClosableNotifierHandler as ::windows_core::ComInterface>::IID
            || *iid == <::windows_core::IUnknown as ::windows_core::ComInterface>::IID
            || *iid == <::windows_core::imp::IAgileObject as ::windows_core::ComInterface>::IID
        {
            &mut (*this).vtable as *mut _ as _
        } else {
            ::core::ptr::null_mut()
        };
        if (*interface).is_null() {
            ::windows_core::HRESULT(-2147467262)
        } else {
            (*this).count.add_ref();
            ::windows_core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: *mut ::core::ffi::c_void) -> u32 {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: *mut ::core::ffi::c_void) -> u32 {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            let _ = ::std::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        ((*this).invoke)().into()
    }
}
unsafe impl ::windows_core::Interface for ClosableNotifierHandler {
    type Vtable = ClosableNotifierHandler_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ClosableNotifierHandler {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x478cec68_ea8e_52fc_87e2_c819de000f92);
}
impl ::windows_core::RuntimeType for ClosableNotifierHandler {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ClosableNotifierHandler_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Invoke:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
