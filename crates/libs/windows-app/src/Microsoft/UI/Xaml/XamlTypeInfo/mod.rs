#[doc(hidden)]
#[repr(transparent)]
pub struct IXamlControlsXamlMetaDataProvider(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IXamlControlsXamlMetaDataProvider {
    type Vtable = IXamlControlsXamlMetaDataProvider_Vtbl;
}
unsafe impl ::windows::core::Interface for IXamlControlsXamlMetaDataProvider {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x17fa3f58_3472_5aa2_a0f8_1ab8a519573d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlControlsXamlMetaDataProvider_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXamlControlsXamlMetaDataProviderStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IXamlControlsXamlMetaDataProviderStatics {
    type Vtable = IXamlControlsXamlMetaDataProviderStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IXamlControlsXamlMetaDataProviderStatics {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x2d7eb3fd_ecdb_5084_b7e0_12f9598381ef);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlControlsXamlMetaDataProviderStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Initialize:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Xaml_XamlTypeInfo\"`, `\"UI_Xaml_Markup\"`*"]
#[cfg(feature = "UI_Xaml_Markup")]
#[repr(transparent)]
pub struct XamlControlsXamlMetaDataProvider(::windows::core::IUnknown);
#[cfg(feature = "UI_Xaml_Markup")]
impl XamlControlsXamlMetaDataProvider {
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
            XamlControlsXamlMetaDataProvider,
            ::windows::core::IGenericFactory,
        > = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Initialize() -> ::windows::core::Result<()> {
        Self::IXamlControlsXamlMetaDataProviderStatics(|this| unsafe {
            (::windows::core::Vtable::vtable(this).Initialize)(::windows::core::Vtable::as_raw(
                this,
            ))
            .ok()
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
    #[cfg(feature = "UI_Xaml_Markup")]
    pub fn GetXamlType<'a, P0>(
        &self,
        r#type: P0,
    ) -> ::windows::core::Result<super::Markup::IXamlType>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, crate::core::TypeName>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetXamlType)(
                ::windows::core::Vtable::as_raw(this),
                r#type.into().abi(),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::Markup::IXamlType>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
    #[cfg(feature = "UI_Xaml_Markup")]
    pub fn GetXamlTypeByFullName(
        &self,
        fullname: &::windows::core::HSTRING,
    ) -> ::windows::core::Result<super::Markup::IXamlType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetXamlTypeByFullName)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(fullname),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::Markup::IXamlType>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
    #[cfg(feature = "UI_Xaml_Markup")]
    pub fn GetXmlnsDefinitions(
        &self,
    ) -> ::windows::core::Result<::windows::core::Array<super::Markup::XmlnsDefinition>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetXmlnsDefinitions)(
                ::windows::core::Vtable::as_raw(this),
                ::windows::core::Array::<super::Markup::XmlnsDefinition>::set_abi_len(
                    result__.assume_init_mut(),
                ),
                result__.as_mut_ptr() as *mut _ as _,
            )
            .and_then(|| result__.assume_init())
        }
    }
    #[doc(hidden)]
    pub fn IXamlControlsXamlMetaDataProviderStatics<
        R,
        F: FnOnce(&IXamlControlsXamlMetaDataProviderStatics) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<
            XamlControlsXamlMetaDataProvider,
            IXamlControlsXamlMetaDataProviderStatics,
        > = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
#[cfg(feature = "UI_Xaml_Markup")]
impl ::core::clone::Clone for XamlControlsXamlMetaDataProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "UI_Xaml_Markup")]
impl ::core::cmp::PartialEq for XamlControlsXamlMetaDataProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "UI_Xaml_Markup")]
impl ::core::cmp::Eq for XamlControlsXamlMetaDataProvider {}
#[cfg(feature = "UI_Xaml_Markup")]
impl ::core::fmt::Debug for XamlControlsXamlMetaDataProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XamlControlsXamlMetaDataProvider").field(&self.0).finish()
    }
}
#[cfg(feature = "UI_Xaml_Markup")]
unsafe impl ::windows::core::RuntimeType for XamlControlsXamlMetaDataProvider {
    const SIGNATURE : ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice ( b"rc(Microsoft.UI.Xaml.XamlTypeInfo.XamlControlsXamlMetaDataProvider;{a96251f0-2214-5d53-8746-ce99a2593cd7})" ) ;
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "UI_Xaml_Markup")]
unsafe impl ::windows::core::Vtable for XamlControlsXamlMetaDataProvider {
    type Vtable = super::Markup::IXamlMetadataProvider_Vtbl;
}
#[cfg(feature = "UI_Xaml_Markup")]
unsafe impl ::windows::core::Interface for XamlControlsXamlMetaDataProvider {
    const IID: ::windows::core::GUID =
        <super::Markup::IXamlMetadataProvider as ::windows::core::Interface>::IID;
}
#[cfg(feature = "UI_Xaml_Markup")]
impl ::windows::core::RuntimeName for XamlControlsXamlMetaDataProvider {
    const NAME: &'static str = "Microsoft.UI.Xaml.XamlTypeInfo.XamlControlsXamlMetaDataProvider";
}
#[cfg(feature = "UI_Xaml_Markup")]
::windows::core::interface_hierarchy!(
    XamlControlsXamlMetaDataProvider,
    ::windows::core::IUnknown,
    ::windows::core::IInspectable
);
#[cfg(feature = "UI_Xaml_Markup")]
impl ::core::convert::TryFrom<XamlControlsXamlMetaDataProvider>
    for super::Markup::IXamlMetadataProvider
{
    type Error = ::windows::core::Error;
    fn try_from(value: XamlControlsXamlMetaDataProvider) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "UI_Xaml_Markup")]
impl ::core::convert::TryFrom<&XamlControlsXamlMetaDataProvider>
    for super::Markup::IXamlMetadataProvider
{
    type Error = ::windows::core::Error;
    fn try_from(value: &XamlControlsXamlMetaDataProvider) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "UI_Xaml_Markup")]
impl<'a> ::core::convert::TryFrom<&XamlControlsXamlMetaDataProvider>
    for ::windows::core::InParam<'a, super::Markup::IXamlMetadataProvider>
{
    type Error = ::windows::core::Error;
    fn try_from(value: &XamlControlsXamlMetaDataProvider) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "UI_Xaml_Markup")]
unsafe impl ::core::marker::Send for XamlControlsXamlMetaDataProvider {}
#[cfg(feature = "UI_Xaml_Markup")]
unsafe impl ::core::marker::Sync for XamlControlsXamlMetaDataProvider {}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
