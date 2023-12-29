#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IThemeSettings(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IThemeSettings {
    type Vtable = IThemeSettings_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IThemeSettings {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x2228ee7e_6d15_563c_8f3c_e8783ba13846);
}
#[repr(C)]
#[doc(hidden)]
pub struct IThemeSettings_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Windows_Foundation")]
    pub Changed: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    Changed: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub RemoveChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RemoveChanged: usize,
    pub HighContrast: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub HighContrastScheme: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IThemeSettingsStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IThemeSettingsStatics {
    type Vtable = IThemeSettingsStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IThemeSettingsStatics {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x1586907d_30db_5f97_8fa1_8940c75dccc0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IThemeSettingsStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateForWindowId: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        windowid: super::WindowId,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ThemeSettings(::windows_core::IUnknown);
impl ThemeSettings {
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn Changed<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<ThemeSettings, ::windows_core::IInspectable>,
        >,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Changed)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveChanged(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveChanged)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn HighContrast(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HighContrast)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn HighContrastScheme(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HighContrastScheme)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn CreateForWindowId(windowid: super::WindowId) -> ::windows_core::Result<ThemeSettings> {
        Self::IThemeSettingsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateForWindowId)(
                ::windows_core::Interface::as_raw(this),
                windowid,
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IThemeSettingsStatics<
        R,
        F: FnOnce(&IThemeSettingsStatics) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<ThemeSettings, IThemeSettingsStatics> =
            ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for ThemeSettings {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for ThemeSettings {
    type Vtable = IThemeSettings_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ThemeSettings {
    const IID: ::windows_core::GUID = <IThemeSettings as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ThemeSettings {
    const NAME: &'static str = "Microsoft.UI.System.ThemeSettings";
}
::windows_core::imp::interface_hierarchy!(
    ThemeSettings,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for ThemeSettings {}
unsafe impl ::core::marker::Sync for ThemeSettings {}
