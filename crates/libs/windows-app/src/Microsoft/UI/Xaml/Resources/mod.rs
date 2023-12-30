#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICustomXamlResourceLoader(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICustomXamlResourceLoader {
    type Vtable = ICustomXamlResourceLoader_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICustomXamlResourceLoader {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x2832c2e5_2ace_5993_a173_3c9c3b992b2e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICustomXamlResourceLoader_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICustomXamlResourceLoaderFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICustomXamlResourceLoaderFactory {
    type Vtable = ICustomXamlResourceLoaderFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICustomXamlResourceLoaderFactory {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x174d49a6_e1e2_5f7b_a618_a8a953d1b5a0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICustomXamlResourceLoaderFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        baseinterface: *mut ::core::ffi::c_void,
        innerinterface: *mut *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICustomXamlResourceLoaderOverrides(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICustomXamlResourceLoaderOverrides {
    type Vtable = ICustomXamlResourceLoaderOverrides_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICustomXamlResourceLoaderOverrides {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x84bb504c_6730_586a_bd04_9198264b2dc7);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICustomXamlResourceLoaderOverrides_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub GetResource: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        resourceid: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        objecttype: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        propertyname: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        propertytype: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICustomXamlResourceLoaderStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICustomXamlResourceLoaderStatics {
    type Vtable = ICustomXamlResourceLoaderStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICustomXamlResourceLoaderStatics {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xe08a5a92_b1a2_539a_9d4a_7994e4468cd7);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICustomXamlResourceLoaderStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Current: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetCurrent: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct CustomXamlResourceLoader(::windows_core::IUnknown);
impl CustomXamlResourceLoader {
    pub unsafe fn CreateInstance<P0>(
        baseinterface: P0,
        innerinterface: &mut ::core::option::Option<::windows_core::IInspectable>,
    ) -> ::windows_core::Result<CustomXamlResourceLoader>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        Self::ICustomXamlResourceLoaderFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstance)(
                ::windows_core::Interface::as_raw(this),
                baseinterface.into_param().abi(),
                innerinterface as *mut _ as _,
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn GetResource(
        &self,
        resourceid: &::windows_core::HSTRING,
        objecttype: &::windows_core::HSTRING,
        propertyname: &::windows_core::HSTRING,
        propertytype: &::windows_core::HSTRING,
    ) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = &::windows_core::ComInterface::cast::<ICustomXamlResourceLoaderOverrides>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetResource)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(resourceid),
                ::core::mem::transmute_copy(objecttype),
                ::core::mem::transmute_copy(propertyname),
                ::core::mem::transmute_copy(propertytype),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn Current() -> ::windows_core::Result<CustomXamlResourceLoader> {
        Self::ICustomXamlResourceLoaderStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Current)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn SetCurrent<P0>(value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<CustomXamlResourceLoader>,
    {
        Self::ICustomXamlResourceLoaderStatics(|this| unsafe {
            (::windows_core::Interface::vtable(this).SetCurrent)(
                ::windows_core::Interface::as_raw(this),
                value.try_into_param()?.abi(),
            )
            .ok()
        })
    }
    #[doc(hidden)]
    pub fn ICustomXamlResourceLoaderFactory<
        R,
        F: FnOnce(&ICustomXamlResourceLoaderFactory) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<
            CustomXamlResourceLoader,
            ICustomXamlResourceLoaderFactory,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ICustomXamlResourceLoaderStatics<
        R,
        F: FnOnce(&ICustomXamlResourceLoaderStatics) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<
            CustomXamlResourceLoader,
            ICustomXamlResourceLoaderStatics,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for CustomXamlResourceLoader {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for CustomXamlResourceLoader {
    type Vtable = ICustomXamlResourceLoader_Vtbl;
}
unsafe impl ::windows_core::ComInterface for CustomXamlResourceLoader {
    const IID: ::windows_core::GUID =
        <ICustomXamlResourceLoader as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for CustomXamlResourceLoader {
    const NAME: &'static str = "Microsoft.UI.Xaml.Resources.CustomXamlResourceLoader";
}
::windows_core::imp::interface_hierarchy!(
    CustomXamlResourceLoader,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for CustomXamlResourceLoader {}
unsafe impl ::core::marker::Sync for CustomXamlResourceLoader {}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
