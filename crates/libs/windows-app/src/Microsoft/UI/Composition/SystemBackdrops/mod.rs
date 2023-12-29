#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDesktopAcrylicController(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDesktopAcrylicController {
    type Vtable = IDesktopAcrylicController_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDesktopAcrylicController {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x7c20a6af_8eb3_5f08_bdfc_6d35e35dfe45);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDesktopAcrylicController_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Windows_UI")]
    pub FallbackColor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI"))]
    FallbackColor: usize,
    #[cfg(feature = "Windows_UI")]
    pub SetFallbackColor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::windows::UI::Color,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI"))]
    SetFallbackColor: usize,
    pub LuminosityOpacity: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f32,
    ) -> ::windows_core::HRESULT,
    pub SetLuminosityOpacity: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f32,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_UI")]
    pub TintColor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI"))]
    TintColor: usize,
    #[cfg(feature = "Windows_UI")]
    pub SetTintColor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::windows::UI::Color,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI"))]
    SetTintColor: usize,
    pub TintOpacity: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f32,
    ) -> ::windows_core::HRESULT,
    pub SetTintOpacity: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f32,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDesktopAcrylicController2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDesktopAcrylicController2 {
    type Vtable = IDesktopAcrylicController2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDesktopAcrylicController2 {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x88e0a368_dfc7_5971_a50b_40df5aa5f5c2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDesktopAcrylicController2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ResetProperties:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDesktopAcrylicController3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDesktopAcrylicController3 {
    type Vtable = IDesktopAcrylicController3_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDesktopAcrylicController3 {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x30d917e6_02d3_59ca_b440_bf9d2e7cc140);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDesktopAcrylicController3_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Kind: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut DesktopAcrylicKind,
    ) -> ::windows_core::HRESULT,
    pub SetKind: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: DesktopAcrylicKind,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDesktopAcrylicControllerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDesktopAcrylicControllerStatics {
    type Vtable = IDesktopAcrylicControllerStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDesktopAcrylicControllerStatics {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xa9e8f790_79ef_5416_9b67_6bcfe867c8b7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDesktopAcrylicControllerStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub IsSupported: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMicaController(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMicaController {
    type Vtable = IMicaController_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMicaController {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x2de996a9_0a2a_5889_a89c_1f84060a8cab);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMicaController_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Windows_UI")]
    pub FallbackColor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI"))]
    FallbackColor: usize,
    #[cfg(feature = "Windows_UI")]
    pub SetFallbackColor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::windows::UI::Color,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI"))]
    SetFallbackColor: usize,
    pub LuminosityOpacity: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f32,
    ) -> ::windows_core::HRESULT,
    pub SetLuminosityOpacity: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f32,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_UI")]
    pub TintColor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI"))]
    TintColor: usize,
    #[cfg(feature = "Windows_UI")]
    pub SetTintColor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::windows::UI::Color,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI"))]
    SetTintColor: usize,
    pub TintOpacity: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f32,
    ) -> ::windows_core::HRESULT,
    pub SetTintOpacity: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f32,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMicaController2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMicaController2 {
    type Vtable = IMicaController2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMicaController2 {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xf1ed4a52_d9ca_506e_9586_caaefd3aa971);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMicaController2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Kind: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut MicaKind,
    ) -> ::windows_core::HRESULT,
    pub SetKind: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: MicaKind,
    ) -> ::windows_core::HRESULT,
    pub ResetProperties:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMicaControllerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMicaControllerStatics {
    type Vtable = IMicaControllerStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMicaControllerStatics {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x7d85d834_d514_5250_b7c4_0b7850d1efdc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMicaControllerStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub IsSupported: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ISystemBackdropConfiguration(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISystemBackdropConfiguration {
    type Vtable = ISystemBackdropConfiguration_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ISystemBackdropConfiguration {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xebcce1b9_0e0c_5431_ab0e_00f3f0669962);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemBackdropConfiguration_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Windows_Foundation", feature = "Windows_UI"))]
    pub HighContrastBackgroundColor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Windows_Foundation", feature = "Windows_UI")))]
    HighContrastBackgroundColor: usize,
    #[cfg(all(feature = "Windows_Foundation", feature = "Windows_UI"))]
    pub SetHighContrastBackgroundColor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Windows_Foundation", feature = "Windows_UI")))]
    SetHighContrastBackgroundColor: usize,
    pub IsHighContrast: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetIsHighContrast: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
    pub IsInputActive: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetIsInputActive: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
    pub Theme: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut SystemBackdropTheme,
    ) -> ::windows_core::HRESULT,
    pub SetTheme: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: SystemBackdropTheme,
    ) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ISystemBackdropController(::windows_core::IUnknown);
impl ISystemBackdropController {
    #[doc = "Required features: `\"Windows_UI_Composition\"`"]
    #[cfg(feature = "Windows_UI_Composition")]
    pub fn SetTargetWithWindowId<P0>(
        &self,
        windowid: super::super::WindowId,
        desktopwindowtarget: P0,
    ) -> ::windows_core::Result<bool>
    where
        P0: ::windows_core::TryIntoParam<::windows::UI::Composition::CompositionTarget>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SetTargetWithWindowId)(
                ::windows_core::Interface::as_raw(this),
                windowid,
                desktopwindowtarget.try_into_param()?.abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_UI_Composition\"`, `\"Windows_UI_Core\"`"]
    #[cfg(all(feature = "Windows_UI_Composition", feature = "Windows_UI_Core"))]
    pub fn SetTargetWithCoreWindow<P0, P1>(
        &self,
        corewindow: P0,
        compositiontarget: P1,
    ) -> ::windows_core::Result<bool>
    where
        P0: ::windows_core::IntoParam<::windows::UI::Core::CoreWindow>,
        P1: ::windows_core::TryIntoParam<::windows::UI::Composition::CompositionTarget>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SetTargetWithCoreWindow)(
                ::windows_core::Interface::as_raw(this),
                corewindow.into_param().abi(),
                compositiontarget.try_into_param()?.abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<::windows::Foundation::IClosable>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this))
                .ok()
        }
    }
}
::windows_core::imp::interface_hierarchy!(
    ISystemBackdropController,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
#[cfg(feature = "Windows_Foundation")]
impl ::windows_core::CanTryInto<::windows::Foundation::IClosable> for ISystemBackdropController {}
impl ::windows_core::RuntimeType for ISystemBackdropController {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_interface::<Self>();
}
unsafe impl ::windows_core::Interface for ISystemBackdropController {
    type Vtable = ISystemBackdropController_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ISystemBackdropController {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x5632d76c_0b74_5b52_aa33_80262068aeb2);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemBackdropController_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Windows_UI_Composition")]
    pub SetTargetWithWindowId: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        windowid: super::super::WindowId,
        desktopwindowtarget: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI_Composition"))]
    SetTargetWithWindowId: usize,
    #[cfg(all(feature = "Windows_UI_Composition", feature = "Windows_UI_Core"))]
    pub SetTargetWithCoreWindow: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        corewindow: *mut ::core::ffi::c_void,
        compositiontarget: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Windows_UI_Composition", feature = "Windows_UI_Core")))]
    SetTargetWithCoreWindow: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ISystemBackdropControllerWithTargets(::windows_core::IUnknown);
impl ISystemBackdropControllerWithTargets {
    pub fn State(&self) -> ::windows_core::Result<SystemBackdropState> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).State)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn AddSystemBackdropTarget<P0>(
        &self,
        systembackdroptarget: P0,
    ) -> ::windows_core::Result<bool>
    where
        P0: ::windows_core::TryIntoParam<super::ICompositionSupportsSystemBackdrop>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AddSystemBackdropTarget)(
                ::windows_core::Interface::as_raw(this),
                systembackdroptarget.try_into_param()?.abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn RemoveAllSystemBackdropTargets(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveAllSystemBackdropTargets)(
                ::windows_core::Interface::as_raw(this),
            )
            .ok()
        }
    }
    pub fn RemoveSystemBackdropTarget<P0>(
        &self,
        systembackdroptarget: P0,
    ) -> ::windows_core::Result<bool>
    where
        P0: ::windows_core::TryIntoParam<super::ICompositionSupportsSystemBackdrop>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RemoveSystemBackdropTarget)(
                ::windows_core::Interface::as_raw(this),
                systembackdroptarget.try_into_param()?.abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetSystemBackdropConfiguration<P0>(
        &self,
        configuration: P0,
    ) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<SystemBackdropConfiguration>,
    {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetSystemBackdropConfiguration)(
                ::windows_core::Interface::as_raw(this),
                configuration.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn StateChanged<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<
                ISystemBackdropControllerWithTargets,
                ::windows_core::IInspectable,
            >,
        >,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StateChanged)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveStateChanged(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveStateChanged)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<::windows::Foundation::IClosable>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this))
                .ok()
        }
    }
    #[doc = "Required features: `\"Windows_UI_Composition\"`"]
    #[cfg(feature = "Windows_UI_Composition")]
    pub fn SetTargetWithWindowId<P0>(
        &self,
        windowid: super::super::WindowId,
        desktopwindowtarget: P0,
    ) -> ::windows_core::Result<bool>
    where
        P0: ::windows_core::TryIntoParam<::windows::UI::Composition::CompositionTarget>,
    {
        let this = &::windows_core::ComInterface::cast::<ISystemBackdropController>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SetTargetWithWindowId)(
                ::windows_core::Interface::as_raw(this),
                windowid,
                desktopwindowtarget.try_into_param()?.abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_UI_Composition\"`, `\"Windows_UI_Core\"`"]
    #[cfg(all(feature = "Windows_UI_Composition", feature = "Windows_UI_Core"))]
    pub fn SetTargetWithCoreWindow<P0, P1>(
        &self,
        corewindow: P0,
        compositiontarget: P1,
    ) -> ::windows_core::Result<bool>
    where
        P0: ::windows_core::IntoParam<::windows::UI::Core::CoreWindow>,
        P1: ::windows_core::TryIntoParam<::windows::UI::Composition::CompositionTarget>,
    {
        let this = &::windows_core::ComInterface::cast::<ISystemBackdropController>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SetTargetWithCoreWindow)(
                ::windows_core::Interface::as_raw(this),
                corewindow.into_param().abi(),
                compositiontarget.try_into_param()?.abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(
    ISystemBackdropControllerWithTargets,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
#[cfg(feature = "Windows_Foundation")]
impl ::windows_core::CanTryInto<::windows::Foundation::IClosable>
    for ISystemBackdropControllerWithTargets
{
}
impl ::windows_core::CanTryInto<ISystemBackdropController>
    for ISystemBackdropControllerWithTargets
{
}
impl ::windows_core::RuntimeType for ISystemBackdropControllerWithTargets {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_interface::<Self>();
}
unsafe impl ::windows_core::Interface for ISystemBackdropControllerWithTargets {
    type Vtable = ISystemBackdropControllerWithTargets_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ISystemBackdropControllerWithTargets {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x9c56fe7c_98eb_5f89_ad97_dad57fc30c8c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemBackdropControllerWithTargets_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub State: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut SystemBackdropState,
    ) -> ::windows_core::HRESULT,
    pub AddSystemBackdropTarget: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        systembackdroptarget: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub RemoveAllSystemBackdropTargets:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub RemoveSystemBackdropTarget: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        systembackdroptarget: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetSystemBackdropConfiguration: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        configuration: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_Foundation")]
    pub StateChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    StateChanged: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub RemoveStateChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RemoveStateChanged: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct DesktopAcrylicController(::windows_core::IUnknown);
impl DesktopAcrylicController {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<
            DesktopAcrylicController,
            ::windows_core::imp::IGenericFactory,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<::windows::Foundation::IClosable>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this))
                .ok()
        }
    }
    pub fn IsClosed(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<super::super::IClosableNotifier>(self)?;
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
        P0: ::windows_core::IntoParam<super::super::ClosableNotifierHandler>,
    {
        let this = &::windows_core::ComInterface::cast::<super::super::IClosableNotifier>(self)?;
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
        let this = &::windows_core::ComInterface::cast::<super::super::IClosableNotifier>(self)?;
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
        P0: ::windows_core::IntoParam<super::super::ClosableNotifierHandler>,
    {
        let this = &::windows_core::ComInterface::cast::<super::super::IClosableNotifier>(self)?;
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
        let this = &::windows_core::ComInterface::cast::<super::super::IClosableNotifier>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveFrameworkClosed)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn FallbackColor(&self) -> ::windows_core::Result<::windows::UI::Color> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FallbackColor)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn SetFallbackColor(&self, value: ::windows::UI::Color) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetFallbackColor)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn LuminosityOpacity(&self) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LuminosityOpacity)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetLuminosityOpacity(&self, value: f32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetLuminosityOpacity)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn TintColor(&self) -> ::windows_core::Result<::windows::UI::Color> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TintColor)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn SetTintColor(&self, value: ::windows::UI::Color) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetTintColor)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn TintOpacity(&self) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TintOpacity)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetTintOpacity(&self, value: f32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetTintOpacity)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn ResetProperties(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IDesktopAcrylicController2>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).ResetProperties)(
                ::windows_core::Interface::as_raw(this),
            )
            .ok()
        }
    }
    pub fn Kind(&self) -> ::windows_core::Result<DesktopAcrylicKind> {
        let this = &::windows_core::ComInterface::cast::<IDesktopAcrylicController3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetKind(&self, value: DesktopAcrylicKind) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IDesktopAcrylicController3>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetKind)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsSupported() -> ::windows_core::Result<bool> {
        Self::IDesktopAcrylicControllerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsSupported)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_UI_Composition\"`"]
    #[cfg(feature = "Windows_UI_Composition")]
    pub fn SetTargetWithWindowId<P0>(
        &self,
        windowid: super::super::WindowId,
        desktopwindowtarget: P0,
    ) -> ::windows_core::Result<bool>
    where
        P0: ::windows_core::TryIntoParam<::windows::UI::Composition::CompositionTarget>,
    {
        let this = &::windows_core::ComInterface::cast::<ISystemBackdropController>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SetTargetWithWindowId)(
                ::windows_core::Interface::as_raw(this),
                windowid,
                desktopwindowtarget.try_into_param()?.abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_UI_Composition\"`, `\"Windows_UI_Core\"`"]
    #[cfg(all(feature = "Windows_UI_Composition", feature = "Windows_UI_Core"))]
    pub fn SetTargetWithCoreWindow<P0, P1>(
        &self,
        corewindow: P0,
        compositiontarget: P1,
    ) -> ::windows_core::Result<bool>
    where
        P0: ::windows_core::IntoParam<::windows::UI::Core::CoreWindow>,
        P1: ::windows_core::TryIntoParam<::windows::UI::Composition::CompositionTarget>,
    {
        let this = &::windows_core::ComInterface::cast::<ISystemBackdropController>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SetTargetWithCoreWindow)(
                ::windows_core::Interface::as_raw(this),
                corewindow.into_param().abi(),
                compositiontarget.try_into_param()?.abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn State(&self) -> ::windows_core::Result<SystemBackdropState> {
        let this =
            &::windows_core::ComInterface::cast::<ISystemBackdropControllerWithTargets>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).State)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn AddSystemBackdropTarget<P0>(
        &self,
        systembackdroptarget: P0,
    ) -> ::windows_core::Result<bool>
    where
        P0: ::windows_core::TryIntoParam<super::ICompositionSupportsSystemBackdrop>,
    {
        let this =
            &::windows_core::ComInterface::cast::<ISystemBackdropControllerWithTargets>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AddSystemBackdropTarget)(
                ::windows_core::Interface::as_raw(this),
                systembackdroptarget.try_into_param()?.abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn RemoveAllSystemBackdropTargets(&self) -> ::windows_core::Result<()> {
        let this =
            &::windows_core::ComInterface::cast::<ISystemBackdropControllerWithTargets>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveAllSystemBackdropTargets)(
                ::windows_core::Interface::as_raw(this),
            )
            .ok()
        }
    }
    pub fn RemoveSystemBackdropTarget<P0>(
        &self,
        systembackdroptarget: P0,
    ) -> ::windows_core::Result<bool>
    where
        P0: ::windows_core::TryIntoParam<super::ICompositionSupportsSystemBackdrop>,
    {
        let this =
            &::windows_core::ComInterface::cast::<ISystemBackdropControllerWithTargets>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RemoveSystemBackdropTarget)(
                ::windows_core::Interface::as_raw(this),
                systembackdroptarget.try_into_param()?.abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetSystemBackdropConfiguration<P0>(
        &self,
        configuration: P0,
    ) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<SystemBackdropConfiguration>,
    {
        let this =
            &::windows_core::ComInterface::cast::<ISystemBackdropControllerWithTargets>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetSystemBackdropConfiguration)(
                ::windows_core::Interface::as_raw(this),
                configuration.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn StateChanged<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<
                ISystemBackdropControllerWithTargets,
                ::windows_core::IInspectable,
            >,
        >,
    {
        let this =
            &::windows_core::ComInterface::cast::<ISystemBackdropControllerWithTargets>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StateChanged)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveStateChanged(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this =
            &::windows_core::ComInterface::cast::<ISystemBackdropControllerWithTargets>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveStateChanged)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc(hidden)]
    pub fn IDesktopAcrylicControllerStatics<
        R,
        F: FnOnce(&IDesktopAcrylicControllerStatics) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<
            DesktopAcrylicController,
            IDesktopAcrylicControllerStatics,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for DesktopAcrylicController {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for DesktopAcrylicController {
    type Vtable = IDesktopAcrylicController_Vtbl;
}
unsafe impl ::windows_core::ComInterface for DesktopAcrylicController {
    const IID: ::windows_core::GUID =
        <IDesktopAcrylicController as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for DesktopAcrylicController {
    const NAME: &'static str = "Microsoft.UI.Composition.SystemBackdrops.DesktopAcrylicController";
}
::windows_core::imp::interface_hierarchy!(
    DesktopAcrylicController,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
#[cfg(feature = "Windows_Foundation")]
impl ::windows_core::CanTryInto<::windows::Foundation::IClosable> for DesktopAcrylicController {}
impl ::windows_core::CanTryInto<super::super::IClosableNotifier> for DesktopAcrylicController {}
impl ::windows_core::CanTryInto<ISystemBackdropController> for DesktopAcrylicController {}
impl ::windows_core::CanTryInto<ISystemBackdropControllerWithTargets> for DesktopAcrylicController {}
unsafe impl ::core::marker::Send for DesktopAcrylicController {}
unsafe impl ::core::marker::Sync for DesktopAcrylicController {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct MicaController(::windows_core::IUnknown);
impl MicaController {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<
            MicaController,
            ::windows_core::imp::IGenericFactory,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<::windows::Foundation::IClosable>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this))
                .ok()
        }
    }
    pub fn IsClosed(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<super::super::IClosableNotifier>(self)?;
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
        P0: ::windows_core::IntoParam<super::super::ClosableNotifierHandler>,
    {
        let this = &::windows_core::ComInterface::cast::<super::super::IClosableNotifier>(self)?;
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
        let this = &::windows_core::ComInterface::cast::<super::super::IClosableNotifier>(self)?;
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
        P0: ::windows_core::IntoParam<super::super::ClosableNotifierHandler>,
    {
        let this = &::windows_core::ComInterface::cast::<super::super::IClosableNotifier>(self)?;
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
        let this = &::windows_core::ComInterface::cast::<super::super::IClosableNotifier>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveFrameworkClosed)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn FallbackColor(&self) -> ::windows_core::Result<::windows::UI::Color> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FallbackColor)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn SetFallbackColor(&self, value: ::windows::UI::Color) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetFallbackColor)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn LuminosityOpacity(&self) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LuminosityOpacity)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetLuminosityOpacity(&self, value: f32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetLuminosityOpacity)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn TintColor(&self) -> ::windows_core::Result<::windows::UI::Color> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TintColor)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn SetTintColor(&self, value: ::windows::UI::Color) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetTintColor)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn TintOpacity(&self) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TintOpacity)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetTintOpacity(&self, value: f32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetTintOpacity)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn Kind(&self) -> ::windows_core::Result<MicaKind> {
        let this = &::windows_core::ComInterface::cast::<IMicaController2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetKind(&self, value: MicaKind) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IMicaController2>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetKind)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn ResetProperties(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IMicaController2>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).ResetProperties)(
                ::windows_core::Interface::as_raw(this),
            )
            .ok()
        }
    }
    pub fn IsSupported() -> ::windows_core::Result<bool> {
        Self::IMicaControllerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsSupported)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_UI_Composition\"`"]
    #[cfg(feature = "Windows_UI_Composition")]
    pub fn SetTargetWithWindowId<P0>(
        &self,
        windowid: super::super::WindowId,
        desktopwindowtarget: P0,
    ) -> ::windows_core::Result<bool>
    where
        P0: ::windows_core::TryIntoParam<::windows::UI::Composition::CompositionTarget>,
    {
        let this = &::windows_core::ComInterface::cast::<ISystemBackdropController>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SetTargetWithWindowId)(
                ::windows_core::Interface::as_raw(this),
                windowid,
                desktopwindowtarget.try_into_param()?.abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_UI_Composition\"`, `\"Windows_UI_Core\"`"]
    #[cfg(all(feature = "Windows_UI_Composition", feature = "Windows_UI_Core"))]
    pub fn SetTargetWithCoreWindow<P0, P1>(
        &self,
        corewindow: P0,
        compositiontarget: P1,
    ) -> ::windows_core::Result<bool>
    where
        P0: ::windows_core::IntoParam<::windows::UI::Core::CoreWindow>,
        P1: ::windows_core::TryIntoParam<::windows::UI::Composition::CompositionTarget>,
    {
        let this = &::windows_core::ComInterface::cast::<ISystemBackdropController>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SetTargetWithCoreWindow)(
                ::windows_core::Interface::as_raw(this),
                corewindow.into_param().abi(),
                compositiontarget.try_into_param()?.abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn State(&self) -> ::windows_core::Result<SystemBackdropState> {
        let this =
            &::windows_core::ComInterface::cast::<ISystemBackdropControllerWithTargets>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).State)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn AddSystemBackdropTarget<P0>(
        &self,
        systembackdroptarget: P0,
    ) -> ::windows_core::Result<bool>
    where
        P0: ::windows_core::TryIntoParam<super::ICompositionSupportsSystemBackdrop>,
    {
        let this =
            &::windows_core::ComInterface::cast::<ISystemBackdropControllerWithTargets>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AddSystemBackdropTarget)(
                ::windows_core::Interface::as_raw(this),
                systembackdroptarget.try_into_param()?.abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn RemoveAllSystemBackdropTargets(&self) -> ::windows_core::Result<()> {
        let this =
            &::windows_core::ComInterface::cast::<ISystemBackdropControllerWithTargets>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveAllSystemBackdropTargets)(
                ::windows_core::Interface::as_raw(this),
            )
            .ok()
        }
    }
    pub fn RemoveSystemBackdropTarget<P0>(
        &self,
        systembackdroptarget: P0,
    ) -> ::windows_core::Result<bool>
    where
        P0: ::windows_core::TryIntoParam<super::ICompositionSupportsSystemBackdrop>,
    {
        let this =
            &::windows_core::ComInterface::cast::<ISystemBackdropControllerWithTargets>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RemoveSystemBackdropTarget)(
                ::windows_core::Interface::as_raw(this),
                systembackdroptarget.try_into_param()?.abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetSystemBackdropConfiguration<P0>(
        &self,
        configuration: P0,
    ) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<SystemBackdropConfiguration>,
    {
        let this =
            &::windows_core::ComInterface::cast::<ISystemBackdropControllerWithTargets>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetSystemBackdropConfiguration)(
                ::windows_core::Interface::as_raw(this),
                configuration.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn StateChanged<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<
                ISystemBackdropControllerWithTargets,
                ::windows_core::IInspectable,
            >,
        >,
    {
        let this =
            &::windows_core::ComInterface::cast::<ISystemBackdropControllerWithTargets>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StateChanged)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveStateChanged(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this =
            &::windows_core::ComInterface::cast::<ISystemBackdropControllerWithTargets>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveStateChanged)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc(hidden)]
    pub fn IMicaControllerStatics<
        R,
        F: FnOnce(&IMicaControllerStatics) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<MicaController, IMicaControllerStatics> =
            ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for MicaController {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for MicaController {
    type Vtable = IMicaController_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MicaController {
    const IID: ::windows_core::GUID = <IMicaController as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MicaController {
    const NAME: &'static str = "Microsoft.UI.Composition.SystemBackdrops.MicaController";
}
::windows_core::imp::interface_hierarchy!(
    MicaController,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
#[cfg(feature = "Windows_Foundation")]
impl ::windows_core::CanTryInto<::windows::Foundation::IClosable> for MicaController {}
impl ::windows_core::CanTryInto<super::super::IClosableNotifier> for MicaController {}
impl ::windows_core::CanTryInto<ISystemBackdropController> for MicaController {}
impl ::windows_core::CanTryInto<ISystemBackdropControllerWithTargets> for MicaController {}
unsafe impl ::core::marker::Send for MicaController {}
unsafe impl ::core::marker::Sync for MicaController {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct SystemBackdropConfiguration(::windows_core::IUnknown);
impl SystemBackdropConfiguration {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<
            SystemBackdropConfiguration,
            ::windows_core::imp::IGenericFactory,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "Required features: `\"Windows_Foundation\"`, `\"Windows_UI\"`"]
    #[cfg(all(feature = "Windows_Foundation", feature = "Windows_UI"))]
    pub fn HighContrastBackgroundColor(
        &self,
    ) -> ::windows_core::Result<::windows::Foundation::IReference<::windows::UI::Color>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HighContrastBackgroundColor)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`, `\"Windows_UI\"`"]
    #[cfg(all(feature = "Windows_Foundation", feature = "Windows_UI"))]
    pub fn SetHighContrastBackgroundColor<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<::windows::Foundation::IReference<::windows::UI::Color>>,
    {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetHighContrastBackgroundColor)(
                ::windows_core::Interface::as_raw(this),
                value.try_into_param()?.abi(),
            )
            .ok()
        }
    }
    pub fn IsHighContrast(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsHighContrast)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetIsHighContrast(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetIsHighContrast)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsInputActive(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsInputActive)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetIsInputActive(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetIsInputActive)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn Theme(&self) -> ::windows_core::Result<SystemBackdropTheme> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Theme)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetTheme(&self, value: SystemBackdropTheme) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetTheme)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
}
impl ::windows_core::RuntimeType for SystemBackdropConfiguration {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for SystemBackdropConfiguration {
    type Vtable = ISystemBackdropConfiguration_Vtbl;
}
unsafe impl ::windows_core::ComInterface for SystemBackdropConfiguration {
    const IID: ::windows_core::GUID =
        <ISystemBackdropConfiguration as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for SystemBackdropConfiguration {
    const NAME: &'static str =
        "Microsoft.UI.Composition.SystemBackdrops.SystemBackdropConfiguration";
}
::windows_core::imp::interface_hierarchy!(
    SystemBackdropConfiguration,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for SystemBackdropConfiguration {}
unsafe impl ::core::marker::Sync for SystemBackdropConfiguration {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DesktopAcrylicKind(pub i32);
impl DesktopAcrylicKind {
    pub const Default: Self = Self(0i32);
    pub const Base: Self = Self(1i32);
    pub const Thin: Self = Self(2i32);
}
impl ::core::marker::Copy for DesktopAcrylicKind {}
impl ::core::clone::Clone for DesktopAcrylicKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DesktopAcrylicKind {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for DesktopAcrylicKind {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for DesktopAcrylicKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DesktopAcrylicKind").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for DesktopAcrylicKind {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::from_slice(
            b"enum(Microsoft.UI.Composition.SystemBackdrops.DesktopAcrylicKind;i4)",
        );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MicaKind(pub i32);
impl MicaKind {
    pub const Base: Self = Self(0i32);
    pub const BaseAlt: Self = Self(1i32);
}
impl ::core::marker::Copy for MicaKind {}
impl ::core::clone::Clone for MicaKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MicaKind {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for MicaKind {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for MicaKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MicaKind").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for MicaKind {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::from_slice(
            b"enum(Microsoft.UI.Composition.SystemBackdrops.MicaKind;i4)",
        );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SystemBackdropState(pub i32);
impl SystemBackdropState {
    pub const Active: Self = Self(0i32);
    pub const Fallback: Self = Self(1i32);
    pub const HighContrast: Self = Self(2i32);
}
impl ::core::marker::Copy for SystemBackdropState {}
impl ::core::clone::Clone for SystemBackdropState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SystemBackdropState {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for SystemBackdropState {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for SystemBackdropState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SystemBackdropState").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for SystemBackdropState {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::from_slice(
            b"enum(Microsoft.UI.Composition.SystemBackdrops.SystemBackdropState;i4)",
        );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SystemBackdropTheme(pub i32);
impl SystemBackdropTheme {
    pub const Default: Self = Self(0i32);
    pub const Light: Self = Self(1i32);
    pub const Dark: Self = Self(2i32);
}
impl ::core::marker::Copy for SystemBackdropTheme {}
impl ::core::clone::Clone for SystemBackdropTheme {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SystemBackdropTheme {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for SystemBackdropTheme {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for SystemBackdropTheme {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SystemBackdropTheme").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for SystemBackdropTheme {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::from_slice(
            b"enum(Microsoft.UI.Composition.SystemBackdrops.SystemBackdropTheme;i4)",
        );
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
