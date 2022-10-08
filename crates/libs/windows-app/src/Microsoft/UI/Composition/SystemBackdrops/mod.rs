#[doc(hidden)]
#[repr(transparent)]
pub struct IDesktopAcrylicController(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IDesktopAcrylicController {
    type Vtable = IDesktopAcrylicController_Vtbl;
}
unsafe impl ::windows::core::Interface for IDesktopAcrylicController {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x7c20a6af_8eb3_5f08_bdfc_6d35e35dfe45);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDesktopAcrylicController_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub FallbackColor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub SetFallbackColor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub LuminosityOpacity: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f32,
    ) -> ::windows::core::HRESULT,
    pub SetLuminosityOpacity: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f32,
    ) -> ::windows::core::HRESULT,
    pub TintColor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub SetTintColor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub TintOpacity: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f32,
    ) -> ::windows::core::HRESULT,
    pub SetTintOpacity: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f32,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDesktopAcrylicController2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IDesktopAcrylicController2 {
    type Vtable = IDesktopAcrylicController2_Vtbl;
}
unsafe impl ::windows::core::Interface for IDesktopAcrylicController2 {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x88e0a368_dfc7_5971_a50b_40df5aa5f5c2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDesktopAcrylicController2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ResetProperties:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDesktopAcrylicControllerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IDesktopAcrylicControllerStatics {
    type Vtable = IDesktopAcrylicControllerStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IDesktopAcrylicControllerStatics {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xa9e8f790_79ef_5416_9b67_6bcfe867c8b7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDesktopAcrylicControllerStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub IsSupported: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMicaController(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IMicaController {
    type Vtable = IMicaController_Vtbl;
}
unsafe impl ::windows::core::Interface for IMicaController {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x2de996a9_0a2a_5889_a89c_1f84060a8cab);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMicaController_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub FallbackColor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub SetFallbackColor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub LuminosityOpacity: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f32,
    ) -> ::windows::core::HRESULT,
    pub SetLuminosityOpacity: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f32,
    ) -> ::windows::core::HRESULT,
    pub TintColor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub SetTintColor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub TintOpacity: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f32,
    ) -> ::windows::core::HRESULT,
    pub SetTintOpacity: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f32,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMicaController2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IMicaController2 {
    type Vtable = IMicaController2_Vtbl;
}
unsafe impl ::windows::core::Interface for IMicaController2 {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xf1ed4a52_d9ca_506e_9586_caaefd3aa971);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMicaController2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Kind: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut MicaKind,
    ) -> ::windows::core::HRESULT,
    pub SetKind: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: MicaKind,
    ) -> ::windows::core::HRESULT,
    pub ResetProperties:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMicaControllerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IMicaControllerStatics {
    type Vtable = IMicaControllerStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IMicaControllerStatics {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x7d85d834_d514_5250_b7c4_0b7850d1efdc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMicaControllerStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub IsSupported: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISystemBackdropConfiguration(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ISystemBackdropConfiguration {
    type Vtable = ISystemBackdropConfiguration_Vtbl;
}
unsafe impl ::windows::core::Interface for ISystemBackdropConfiguration {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xebcce1b9_0e0c_5431_ab0e_00f3f0669962);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemBackdropConfiguration_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub HighContrastBackgroundColor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub SetHighContrastBackgroundColor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub IsHighContrast: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub SetIsHighContrast: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows::core::HRESULT,
    pub IsInputActive: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub SetIsInputActive: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows::core::HRESULT,
    pub Theme: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut SystemBackdropTheme,
    ) -> ::windows::core::HRESULT,
    pub SetTheme: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: SystemBackdropTheme,
    ) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Composition_SystemBackdrops\"`*"]
#[repr(transparent)]
pub struct ISystemBackdropController(::windows::core::IUnknown);
impl ISystemBackdropController {
    pub fn SetTargetWithWindowId<'a, P0>(
        &self,
        windowid: super::super::WindowId,
        desktopwindowtarget: P0,
    ) -> ::windows::core::Result<bool>
    where
        P0: ::std::convert::Into<
            ::windows::core::InParam<'a, ::windows::UI::Composition::CompositionTarget>,
        >,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SetTargetWithWindowId)(
                ::windows::core::Vtable::as_raw(this),
                windowid,
                desktopwindowtarget.into().abi(),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetTargetWithCoreWindow<'a, P0>(
        &self,
        corewindow: &::windows::UI::Core::CoreWindow,
        compositiontarget: P0,
    ) -> ::windows::core::Result<bool>
    where
        P0: ::std::convert::Into<
            ::windows::core::InParam<'a, ::windows::UI::Composition::CompositionTarget>,
        >,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SetTargetWithCoreWindow)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(corewindow),
                compositiontarget.into().abi(),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<::windows::Foundation::IClosable>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).Close)(::windows::core::Vtable::as_raw(this))
                .ok()
        }
    }
}
::windows::core::interface_hierarchy!(
    ISystemBackdropController,
    ::windows::core::IUnknown,
    ::windows::core::IInspectable
);
impl ::core::convert::TryFrom<ISystemBackdropController> for ::windows::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: ISystemBackdropController) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ISystemBackdropController> for ::windows::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &ISystemBackdropController) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&ISystemBackdropController>
    for ::windows::core::InParam<'a, ::windows::Foundation::IClosable>
{
    type Error = ::windows::core::Error;
    fn try_from(value: &ISystemBackdropController) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::clone::Clone for ISystemBackdropController {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISystemBackdropController {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISystemBackdropController {}
impl ::core::fmt::Debug for ISystemBackdropController {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISystemBackdropController").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ISystemBackdropController {
    const SIGNATURE: ::windows::core::ConstBuffer =
        ::windows::core::ConstBuffer::from_slice(b"{5632d76c-0b74-5b52-aa33-80262068aeb2}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ISystemBackdropController {
    type Vtable = ISystemBackdropController_Vtbl;
}
unsafe impl ::windows::core::Interface for ISystemBackdropController {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x5632d76c_0b74_5b52_aa33_80262068aeb2);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemBackdropController_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub SetTargetWithWindowId: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        windowid: super::super::WindowId,
        desktopwindowtarget: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub SetTargetWithCoreWindow: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        corewindow: *mut ::core::ffi::c_void,
        compositiontarget: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Composition_SystemBackdrops\"`*"]
#[repr(transparent)]
pub struct ISystemBackdropControllerWithTargets(::windows::core::IUnknown);
impl ISystemBackdropControllerWithTargets {
    pub fn State(&self) -> ::windows::core::Result<SystemBackdropState> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).State)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<SystemBackdropState>(result__)
        }
    }
    pub fn AddSystemBackdropTarget<'a, P0, E0>(
        &self,
        systembackdroptarget: P0,
    ) -> ::windows::core::Result<bool>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<'a, super::ICompositionSupportsSystemBackdrop>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AddSystemBackdropTarget)(
                ::windows::core::Vtable::as_raw(this),
                systembackdroptarget.try_into().map_err(|e| e.into())?.abi(),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn RemoveAllSystemBackdropTargets(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).RemoveAllSystemBackdropTargets)(
                ::windows::core::Vtable::as_raw(this),
            )
            .ok()
        }
    }
    pub fn RemoveSystemBackdropTarget<'a, P0, E0>(
        &self,
        systembackdroptarget: P0,
    ) -> ::windows::core::Result<bool>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<'a, super::ICompositionSupportsSystemBackdrop>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RemoveSystemBackdropTarget)(
                ::windows::core::Vtable::as_raw(this),
                systembackdroptarget.try_into().map_err(|e| e.into())?.abi(),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetSystemBackdropConfiguration(
        &self,
        configuration: &SystemBackdropConfiguration,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetSystemBackdropConfiguration)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(configuration),
            )
            .ok()
        }
    }
    pub fn StateChanged(
        &self,
        handler: &::windows::Foundation::TypedEventHandler<
            ISystemBackdropControllerWithTargets,
            ::windows::core::IInspectable,
        >,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).StateChanged)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(handler),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveStateChanged(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).RemoveStateChanged)(
                ::windows::core::Vtable::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<::windows::Foundation::IClosable>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).Close)(::windows::core::Vtable::as_raw(this))
                .ok()
        }
    }
    pub fn SetTargetWithWindowId<'a, P0>(
        &self,
        windowid: super::super::WindowId,
        desktopwindowtarget: P0,
    ) -> ::windows::core::Result<bool>
    where
        P0: ::std::convert::Into<
            ::windows::core::InParam<'a, ::windows::UI::Composition::CompositionTarget>,
        >,
    {
        let this = &::windows::core::Interface::cast::<ISystemBackdropController>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SetTargetWithWindowId)(
                ::windows::core::Vtable::as_raw(this),
                windowid,
                desktopwindowtarget.into().abi(),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetTargetWithCoreWindow<'a, P0>(
        &self,
        corewindow: &::windows::UI::Core::CoreWindow,
        compositiontarget: P0,
    ) -> ::windows::core::Result<bool>
    where
        P0: ::std::convert::Into<
            ::windows::core::InParam<'a, ::windows::UI::Composition::CompositionTarget>,
        >,
    {
        let this = &::windows::core::Interface::cast::<ISystemBackdropController>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SetTargetWithCoreWindow)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(corewindow),
                compositiontarget.into().abi(),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        }
    }
}
::windows::core::interface_hierarchy!(
    ISystemBackdropControllerWithTargets,
    ::windows::core::IUnknown,
    ::windows::core::IInspectable
);
impl ::core::convert::TryFrom<ISystemBackdropControllerWithTargets>
    for ::windows::Foundation::IClosable
{
    type Error = ::windows::core::Error;
    fn try_from(value: ISystemBackdropControllerWithTargets) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ISystemBackdropControllerWithTargets>
    for ::windows::Foundation::IClosable
{
    type Error = ::windows::core::Error;
    fn try_from(value: &ISystemBackdropControllerWithTargets) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&ISystemBackdropControllerWithTargets>
    for ::windows::core::InParam<'a, ::windows::Foundation::IClosable>
{
    type Error = ::windows::core::Error;
    fn try_from(value: &ISystemBackdropControllerWithTargets) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<ISystemBackdropControllerWithTargets> for ISystemBackdropController {
    type Error = ::windows::core::Error;
    fn try_from(value: ISystemBackdropControllerWithTargets) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ISystemBackdropControllerWithTargets> for ISystemBackdropController {
    type Error = ::windows::core::Error;
    fn try_from(value: &ISystemBackdropControllerWithTargets) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&ISystemBackdropControllerWithTargets>
    for ::windows::core::InParam<'a, ISystemBackdropController>
{
    type Error = ::windows::core::Error;
    fn try_from(value: &ISystemBackdropControllerWithTargets) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::clone::Clone for ISystemBackdropControllerWithTargets {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISystemBackdropControllerWithTargets {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISystemBackdropControllerWithTargets {}
impl ::core::fmt::Debug for ISystemBackdropControllerWithTargets {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISystemBackdropControllerWithTargets").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ISystemBackdropControllerWithTargets {
    const SIGNATURE: ::windows::core::ConstBuffer =
        ::windows::core::ConstBuffer::from_slice(b"{9c56fe7c-98eb-5f89-ad97-dad57fc30c8c}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ISystemBackdropControllerWithTargets {
    type Vtable = ISystemBackdropControllerWithTargets_Vtbl;
}
unsafe impl ::windows::core::Interface for ISystemBackdropControllerWithTargets {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x9c56fe7c_98eb_5f89_ad97_dad57fc30c8c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemBackdropControllerWithTargets_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub State: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut SystemBackdropState,
    ) -> ::windows::core::HRESULT,
    pub AddSystemBackdropTarget: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        systembackdroptarget: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub RemoveAllSystemBackdropTargets:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RemoveSystemBackdropTarget: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        systembackdroptarget: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub SetSystemBackdropConfiguration: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        configuration: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub StateChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub RemoveStateChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Composition_SystemBackdrops\"`*"]
#[repr(transparent)]
pub struct DesktopAcrylicController(::windows::core::IUnknown);
impl DesktopAcrylicController {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<
            DesktopAcrylicController,
            ::windows::core::IGenericFactory,
        > = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<::windows::Foundation::IClosable>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).Close)(::windows::core::Vtable::as_raw(this))
                .ok()
        }
    }
    pub fn FallbackColor(&self) -> ::windows::core::Result<::windows::UI::Color> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FallbackColor)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Color>(result__)
        }
    }
    pub fn SetFallbackColor(&self, value: ::windows::UI::Color) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetFallbackColor)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn LuminosityOpacity(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LuminosityOpacity)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<f32>(result__)
        }
    }
    pub fn SetLuminosityOpacity(&self, value: f32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetLuminosityOpacity)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn TintColor(&self) -> ::windows::core::Result<::windows::UI::Color> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TintColor)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Color>(result__)
        }
    }
    pub fn SetTintColor(&self, value: ::windows::UI::Color) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetTintColor)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn TintOpacity(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TintOpacity)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<f32>(result__)
        }
    }
    pub fn SetTintOpacity(&self, value: f32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetTintOpacity)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn ResetProperties(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IDesktopAcrylicController2>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).ResetProperties)(
                ::windows::core::Vtable::as_raw(this),
            )
            .ok()
        }
    }
    pub fn IsSupported() -> ::windows::core::Result<bool> {
        Self::IDesktopAcrylicControllerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsSupported)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        })
    }
    pub fn SetTargetWithWindowId<'a, P0>(
        &self,
        windowid: super::super::WindowId,
        desktopwindowtarget: P0,
    ) -> ::windows::core::Result<bool>
    where
        P0: ::std::convert::Into<
            ::windows::core::InParam<'a, ::windows::UI::Composition::CompositionTarget>,
        >,
    {
        let this = &::windows::core::Interface::cast::<ISystemBackdropController>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SetTargetWithWindowId)(
                ::windows::core::Vtable::as_raw(this),
                windowid,
                desktopwindowtarget.into().abi(),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetTargetWithCoreWindow<'a, P0>(
        &self,
        corewindow: &::windows::UI::Core::CoreWindow,
        compositiontarget: P0,
    ) -> ::windows::core::Result<bool>
    where
        P0: ::std::convert::Into<
            ::windows::core::InParam<'a, ::windows::UI::Composition::CompositionTarget>,
        >,
    {
        let this = &::windows::core::Interface::cast::<ISystemBackdropController>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SetTargetWithCoreWindow)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(corewindow),
                compositiontarget.into().abi(),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn State(&self) -> ::windows::core::Result<SystemBackdropState> {
        let this = &::windows::core::Interface::cast::<ISystemBackdropControllerWithTargets>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).State)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<SystemBackdropState>(result__)
        }
    }
    pub fn AddSystemBackdropTarget<'a, P0, E0>(
        &self,
        systembackdroptarget: P0,
    ) -> ::windows::core::Result<bool>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<'a, super::ICompositionSupportsSystemBackdrop>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<ISystemBackdropControllerWithTargets>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AddSystemBackdropTarget)(
                ::windows::core::Vtable::as_raw(this),
                systembackdroptarget.try_into().map_err(|e| e.into())?.abi(),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn RemoveAllSystemBackdropTargets(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ISystemBackdropControllerWithTargets>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).RemoveAllSystemBackdropTargets)(
                ::windows::core::Vtable::as_raw(this),
            )
            .ok()
        }
    }
    pub fn RemoveSystemBackdropTarget<'a, P0, E0>(
        &self,
        systembackdroptarget: P0,
    ) -> ::windows::core::Result<bool>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<'a, super::ICompositionSupportsSystemBackdrop>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<ISystemBackdropControllerWithTargets>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RemoveSystemBackdropTarget)(
                ::windows::core::Vtable::as_raw(this),
                systembackdroptarget.try_into().map_err(|e| e.into())?.abi(),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetSystemBackdropConfiguration(
        &self,
        configuration: &SystemBackdropConfiguration,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ISystemBackdropControllerWithTargets>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetSystemBackdropConfiguration)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(configuration),
            )
            .ok()
        }
    }
    pub fn StateChanged(
        &self,
        handler: &::windows::Foundation::TypedEventHandler<
            ISystemBackdropControllerWithTargets,
            ::windows::core::IInspectable,
        >,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<ISystemBackdropControllerWithTargets>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).StateChanged)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(handler),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveStateChanged(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ISystemBackdropControllerWithTargets>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).RemoveStateChanged)(
                ::windows::core::Vtable::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc(hidden)]
    pub fn IDesktopAcrylicControllerStatics<
        R,
        F: FnOnce(&IDesktopAcrylicControllerStatics) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<
            DesktopAcrylicController,
            IDesktopAcrylicControllerStatics,
        > = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for DesktopAcrylicController {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DesktopAcrylicController {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DesktopAcrylicController {}
impl ::core::fmt::Debug for DesktopAcrylicController {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DesktopAcrylicController").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DesktopAcrylicController {
    const SIGNATURE : ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice ( b"rc(Microsoft.UI.Composition.SystemBackdrops.DesktopAcrylicController;{7c20a6af-8eb3-5f08-bdfc-6d35e35dfe45})" ) ;
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for DesktopAcrylicController {
    type Vtable = IDesktopAcrylicController_Vtbl;
}
unsafe impl ::windows::core::Interface for DesktopAcrylicController {
    const IID: ::windows::core::GUID =
        <IDesktopAcrylicController as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for DesktopAcrylicController {
    const NAME: &'static str = "Microsoft.UI.Composition.SystemBackdrops.DesktopAcrylicController";
}
::windows::core::interface_hierarchy!(
    DesktopAcrylicController,
    ::windows::core::IUnknown,
    ::windows::core::IInspectable
);
impl ::core::convert::TryFrom<DesktopAcrylicController> for ::windows::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: DesktopAcrylicController) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&DesktopAcrylicController> for ::windows::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &DesktopAcrylicController) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&DesktopAcrylicController>
    for ::windows::core::InParam<'a, ::windows::Foundation::IClosable>
{
    type Error = ::windows::core::Error;
    fn try_from(value: &DesktopAcrylicController) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<DesktopAcrylicController> for ISystemBackdropController {
    type Error = ::windows::core::Error;
    fn try_from(value: DesktopAcrylicController) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&DesktopAcrylicController> for ISystemBackdropController {
    type Error = ::windows::core::Error;
    fn try_from(value: &DesktopAcrylicController) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&DesktopAcrylicController>
    for ::windows::core::InParam<'a, ISystemBackdropController>
{
    type Error = ::windows::core::Error;
    fn try_from(value: &DesktopAcrylicController) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<DesktopAcrylicController> for ISystemBackdropControllerWithTargets {
    type Error = ::windows::core::Error;
    fn try_from(value: DesktopAcrylicController) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&DesktopAcrylicController> for ISystemBackdropControllerWithTargets {
    type Error = ::windows::core::Error;
    fn try_from(value: &DesktopAcrylicController) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&DesktopAcrylicController>
    for ::windows::core::InParam<'a, ISystemBackdropControllerWithTargets>
{
    type Error = ::windows::core::Error;
    fn try_from(value: &DesktopAcrylicController) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for DesktopAcrylicController {}
unsafe impl ::core::marker::Sync for DesktopAcrylicController {}
#[doc = "*Required features: `\"UI_Composition_SystemBackdrops\"`*"]
#[repr(transparent)]
pub struct MicaController(::windows::core::IUnknown);
impl MicaController {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<
            MicaController,
            ::windows::core::IGenericFactory,
        > = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<::windows::Foundation::IClosable>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).Close)(::windows::core::Vtable::as_raw(this))
                .ok()
        }
    }
    pub fn FallbackColor(&self) -> ::windows::core::Result<::windows::UI::Color> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FallbackColor)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Color>(result__)
        }
    }
    pub fn SetFallbackColor(&self, value: ::windows::UI::Color) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetFallbackColor)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn LuminosityOpacity(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LuminosityOpacity)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<f32>(result__)
        }
    }
    pub fn SetLuminosityOpacity(&self, value: f32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetLuminosityOpacity)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn TintColor(&self) -> ::windows::core::Result<::windows::UI::Color> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TintColor)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Color>(result__)
        }
    }
    pub fn SetTintColor(&self, value: ::windows::UI::Color) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetTintColor)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn TintOpacity(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TintOpacity)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<f32>(result__)
        }
    }
    pub fn SetTintOpacity(&self, value: f32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetTintOpacity)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<MicaKind> {
        let this = &::windows::core::Interface::cast::<IMicaController2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Kind)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<MicaKind>(result__)
        }
    }
    pub fn SetKind(&self, value: MicaKind) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMicaController2>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetKind)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn ResetProperties(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMicaController2>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).ResetProperties)(
                ::windows::core::Vtable::as_raw(this),
            )
            .ok()
        }
    }
    pub fn IsSupported() -> ::windows::core::Result<bool> {
        Self::IMicaControllerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsSupported)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        })
    }
    pub fn SetTargetWithWindowId<'a, P0>(
        &self,
        windowid: super::super::WindowId,
        desktopwindowtarget: P0,
    ) -> ::windows::core::Result<bool>
    where
        P0: ::std::convert::Into<
            ::windows::core::InParam<'a, ::windows::UI::Composition::CompositionTarget>,
        >,
    {
        let this = &::windows::core::Interface::cast::<ISystemBackdropController>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SetTargetWithWindowId)(
                ::windows::core::Vtable::as_raw(this),
                windowid,
                desktopwindowtarget.into().abi(),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetTargetWithCoreWindow<'a, P0>(
        &self,
        corewindow: &::windows::UI::Core::CoreWindow,
        compositiontarget: P0,
    ) -> ::windows::core::Result<bool>
    where
        P0: ::std::convert::Into<
            ::windows::core::InParam<'a, ::windows::UI::Composition::CompositionTarget>,
        >,
    {
        let this = &::windows::core::Interface::cast::<ISystemBackdropController>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SetTargetWithCoreWindow)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(corewindow),
                compositiontarget.into().abi(),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn State(&self) -> ::windows::core::Result<SystemBackdropState> {
        let this = &::windows::core::Interface::cast::<ISystemBackdropControllerWithTargets>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).State)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<SystemBackdropState>(result__)
        }
    }
    pub fn AddSystemBackdropTarget<'a, P0, E0>(
        &self,
        systembackdroptarget: P0,
    ) -> ::windows::core::Result<bool>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<'a, super::ICompositionSupportsSystemBackdrop>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<ISystemBackdropControllerWithTargets>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AddSystemBackdropTarget)(
                ::windows::core::Vtable::as_raw(this),
                systembackdroptarget.try_into().map_err(|e| e.into())?.abi(),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn RemoveAllSystemBackdropTargets(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ISystemBackdropControllerWithTargets>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).RemoveAllSystemBackdropTargets)(
                ::windows::core::Vtable::as_raw(this),
            )
            .ok()
        }
    }
    pub fn RemoveSystemBackdropTarget<'a, P0, E0>(
        &self,
        systembackdroptarget: P0,
    ) -> ::windows::core::Result<bool>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<'a, super::ICompositionSupportsSystemBackdrop>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<ISystemBackdropControllerWithTargets>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RemoveSystemBackdropTarget)(
                ::windows::core::Vtable::as_raw(this),
                systembackdroptarget.try_into().map_err(|e| e.into())?.abi(),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetSystemBackdropConfiguration(
        &self,
        configuration: &SystemBackdropConfiguration,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ISystemBackdropControllerWithTargets>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetSystemBackdropConfiguration)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(configuration),
            )
            .ok()
        }
    }
    pub fn StateChanged(
        &self,
        handler: &::windows::Foundation::TypedEventHandler<
            ISystemBackdropControllerWithTargets,
            ::windows::core::IInspectable,
        >,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<ISystemBackdropControllerWithTargets>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).StateChanged)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(handler),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveStateChanged(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ISystemBackdropControllerWithTargets>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).RemoveStateChanged)(
                ::windows::core::Vtable::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc(hidden)]
    pub fn IMicaControllerStatics<
        R,
        F: FnOnce(&IMicaControllerStatics) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<MicaController, IMicaControllerStatics> =
            ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for MicaController {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MicaController {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MicaController {}
impl ::core::fmt::Debug for MicaController {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MicaController").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MicaController {
    const SIGNATURE : ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice ( b"rc(Microsoft.UI.Composition.SystemBackdrops.MicaController;{2de996a9-0a2a-5889-a89c-1f84060a8cab})" ) ;
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for MicaController {
    type Vtable = IMicaController_Vtbl;
}
unsafe impl ::windows::core::Interface for MicaController {
    const IID: ::windows::core::GUID = <IMicaController as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MicaController {
    const NAME: &'static str = "Microsoft.UI.Composition.SystemBackdrops.MicaController";
}
::windows::core::interface_hierarchy!(
    MicaController,
    ::windows::core::IUnknown,
    ::windows::core::IInspectable
);
impl ::core::convert::TryFrom<MicaController> for ::windows::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: MicaController) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&MicaController> for ::windows::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &MicaController) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&MicaController>
    for ::windows::core::InParam<'a, ::windows::Foundation::IClosable>
{
    type Error = ::windows::core::Error;
    fn try_from(value: &MicaController) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<MicaController> for ISystemBackdropController {
    type Error = ::windows::core::Error;
    fn try_from(value: MicaController) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&MicaController> for ISystemBackdropController {
    type Error = ::windows::core::Error;
    fn try_from(value: &MicaController) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&MicaController>
    for ::windows::core::InParam<'a, ISystemBackdropController>
{
    type Error = ::windows::core::Error;
    fn try_from(value: &MicaController) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<MicaController> for ISystemBackdropControllerWithTargets {
    type Error = ::windows::core::Error;
    fn try_from(value: MicaController) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&MicaController> for ISystemBackdropControllerWithTargets {
    type Error = ::windows::core::Error;
    fn try_from(value: &MicaController) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&MicaController>
    for ::windows::core::InParam<'a, ISystemBackdropControllerWithTargets>
{
    type Error = ::windows::core::Error;
    fn try_from(value: &MicaController) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for MicaController {}
unsafe impl ::core::marker::Sync for MicaController {}
#[doc = "*Required features: `\"UI_Composition_SystemBackdrops\"`*"]
#[repr(transparent)]
pub struct SystemBackdropConfiguration(::windows::core::IUnknown);
impl SystemBackdropConfiguration {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<
            SystemBackdropConfiguration,
            ::windows::core::IGenericFactory,
        > = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn HighContrastBackgroundColor(
        &self,
    ) -> ::windows::core::Result<::windows::Foundation::IReference<::windows::UI::Color>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).HighContrastBackgroundColor)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::IReference<::windows::UI::Color>>(result__)
        }
    }
    pub fn SetHighContrastBackgroundColor<'a, P0, E0>(
        &self,
        value: P0,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<'a, ::windows::Foundation::IReference<::windows::UI::Color>>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetHighContrastBackgroundColor)(
                ::windows::core::Vtable::as_raw(this),
                value.try_into().map_err(|e| e.into())?.abi(),
            )
            .ok()
        }
    }
    pub fn IsHighContrast(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsHighContrast)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetIsHighContrast(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetIsHighContrast)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsInputActive(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsInputActive)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetIsInputActive(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetIsInputActive)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn Theme(&self) -> ::windows::core::Result<SystemBackdropTheme> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Theme)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<SystemBackdropTheme>(result__)
        }
    }
    pub fn SetTheme(&self, value: SystemBackdropTheme) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetTheme)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
}
impl ::core::clone::Clone for SystemBackdropConfiguration {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SystemBackdropConfiguration {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SystemBackdropConfiguration {}
impl ::core::fmt::Debug for SystemBackdropConfiguration {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SystemBackdropConfiguration").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SystemBackdropConfiguration {
    const SIGNATURE : ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice ( b"rc(Microsoft.UI.Composition.SystemBackdrops.SystemBackdropConfiguration;{ebcce1b9-0e0c-5431-ab0e-00f3f0669962})" ) ;
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for SystemBackdropConfiguration {
    type Vtable = ISystemBackdropConfiguration_Vtbl;
}
unsafe impl ::windows::core::Interface for SystemBackdropConfiguration {
    const IID: ::windows::core::GUID =
        <ISystemBackdropConfiguration as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for SystemBackdropConfiguration {
    const NAME: &'static str =
        "Microsoft.UI.Composition.SystemBackdrops.SystemBackdropConfiguration";
}
::windows::core::interface_hierarchy!(
    SystemBackdropConfiguration,
    ::windows::core::IUnknown,
    ::windows::core::IInspectable
);
unsafe impl ::core::marker::Send for SystemBackdropConfiguration {}
unsafe impl ::core::marker::Sync for SystemBackdropConfiguration {}
#[doc = "*Required features: `\"UI_Composition_SystemBackdrops\"`*"]
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
unsafe impl ::windows::core::Abi for MicaKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for MicaKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MicaKind").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MicaKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Composition.SystemBackdrops.MicaKind;i4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_Composition_SystemBackdrops\"`*"]
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
unsafe impl ::windows::core::Abi for SystemBackdropState {
    type Abi = Self;
}
impl ::core::fmt::Debug for SystemBackdropState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SystemBackdropState").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SystemBackdropState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Composition.SystemBackdrops.SystemBackdropState;i4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_Composition_SystemBackdrops\"`*"]
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
unsafe impl ::windows::core::Abi for SystemBackdropTheme {
    type Abi = Self;
}
impl ::core::fmt::Debug for SystemBackdropTheme {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SystemBackdropTheme").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SystemBackdropTheme {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Composition.SystemBackdrops.SystemBackdropTheme;i4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
