#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IXamlControlsXamlMetaDataProvider(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXamlControlsXamlMetaDataProvider {
    type Vtable = IXamlControlsXamlMetaDataProvider_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IXamlControlsXamlMetaDataProvider {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x17fa3f58_3472_5aa2_a0f8_1ab8a519573d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlControlsXamlMetaDataProvider_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IXamlControlsXamlMetaDataProviderStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXamlControlsXamlMetaDataProviderStatics {
    type Vtable = IXamlControlsXamlMetaDataProviderStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IXamlControlsXamlMetaDataProviderStatics {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x2d7eb3fd_ecdb_5084_b7e0_12f9598381ef);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlControlsXamlMetaDataProviderStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Initialize:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "Required features: `\"Microsoft_UI_Xaml_Markup\"`"]
#[cfg(feature = "Microsoft_UI_Xaml_Markup")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct XamlControlsXamlMetaDataProvider(::windows_core::IUnknown);
#[cfg(feature = "Microsoft_UI_Xaml_Markup")]
impl XamlControlsXamlMetaDataProvider {
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
            XamlControlsXamlMetaDataProvider,
            ::windows_core::imp::IGenericFactory,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Initialize() -> ::windows_core::Result<()> {
        Self::IXamlControlsXamlMetaDataProviderStatics(|this| unsafe {
            (::windows_core::Interface::vtable(this).Initialize)(::windows_core::Interface::as_raw(
                this,
            ))
            .ok()
        })
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Markup\"`, `\"Windows_UI_Xaml_Interop\"`"]
    #[cfg(all(
        feature = "Microsoft_UI_Xaml_Markup",
        feature = "Windows_UI_Xaml_Interop"
    ))]
    pub fn GetXamlType<P0>(&self, r#type: P0) -> ::windows_core::Result<super::Markup::IXamlType>
    where
        P0: ::windows_core::IntoParam<
            super::super::super::super::Windows::UI::Xaml::Interop::TypeName,
        >,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetXamlType)(
                ::windows_core::Interface::as_raw(this),
                r#type.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Markup\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Markup")]
    pub fn GetXamlTypeByFullName(
        &self,
        fullname: &::windows_core::HSTRING,
    ) -> ::windows_core::Result<super::Markup::IXamlType> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetXamlTypeByFullName)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(fullname),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Markup\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Markup")]
    pub fn GetXmlnsDefinitions(
        &self,
    ) -> ::windows_core::Result<::windows_core::Array<super::Markup::XmlnsDefinition>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows_core::Interface::vtable(this).GetXmlnsDefinitions)(
                ::windows_core::Interface::as_raw(this),
                ::windows_core::Array::<super::Markup::XmlnsDefinition>::set_abi_len(
                    ::std::mem::transmute(&mut result__),
                ),
                result__.as_mut_ptr() as *mut _ as _,
            )
            .and_then(|| result__.assume_init())
        }
    }
    #[doc(hidden)]
    pub fn IXamlControlsXamlMetaDataProviderStatics<
        R,
        F: FnOnce(&IXamlControlsXamlMetaDataProviderStatics) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<
            XamlControlsXamlMetaDataProvider,
            IXamlControlsXamlMetaDataProviderStatics,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
#[cfg(feature = "Microsoft_UI_Xaml_Markup")]
impl ::windows_core::RuntimeType for XamlControlsXamlMetaDataProvider {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
#[cfg(feature = "Microsoft_UI_Xaml_Markup")]
unsafe impl ::windows_core::Interface for XamlControlsXamlMetaDataProvider {
    type Vtable = super::Markup::IXamlMetadataProvider_Vtbl;
}
#[cfg(feature = "Microsoft_UI_Xaml_Markup")]
unsafe impl ::windows_core::ComInterface for XamlControlsXamlMetaDataProvider {
    const IID: ::windows_core::GUID =
        <super::Markup::IXamlMetadataProvider as ::windows_core::ComInterface>::IID;
}
#[cfg(feature = "Microsoft_UI_Xaml_Markup")]
impl ::windows_core::RuntimeName for XamlControlsXamlMetaDataProvider {
    const NAME: &'static str = "Microsoft.UI.Xaml.XamlTypeInfo.XamlControlsXamlMetaDataProvider";
}
#[cfg(feature = "Microsoft_UI_Xaml_Markup")]
::windows_core::imp::interface_hierarchy!(
    XamlControlsXamlMetaDataProvider,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
#[cfg(feature = "Microsoft_UI_Xaml_Markup")]
impl ::windows_core::CanTryInto<super::Markup::IXamlMetadataProvider>
    for XamlControlsXamlMetaDataProvider
{
}
#[cfg(feature = "Microsoft_UI_Xaml_Markup")]
unsafe impl ::core::marker::Send for XamlControlsXamlMetaDataProvider {}
#[cfg(feature = "Microsoft_UI_Xaml_Markup")]
unsafe impl ::core::marker::Sync for XamlControlsXamlMetaDataProvider {}
