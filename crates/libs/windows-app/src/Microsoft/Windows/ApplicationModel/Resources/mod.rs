#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IKnownResourceQualifierNameStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IKnownResourceQualifierNameStatics {
    type Vtable = IKnownResourceQualifierNameStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IKnownResourceQualifierNameStatics {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xdd6cdedc_559b_50c8_ac53_82fe21f915f3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKnownResourceQualifierNameStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Contrast: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub Custom: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub DeviceFamily: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub HomeRegion: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub Language: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub LayoutDirection: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub Scale: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub TargetSize: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub Theme: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IResourceCandidate(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IResourceCandidate {
    type Vtable = IResourceCandidate_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IResourceCandidate {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x6c54bc0c_ef1e_57b8_b478_34fece737356);
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceCandidate_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ValueAsString: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub ValueAsBytes: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result_size__: *mut u32,
        result__: *mut *mut u8,
    ) -> ::windows_core::HRESULT,
    pub Kind: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ResourceCandidateKind,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub QualifierValues: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation_Collections"))]
    QualifierValues: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IResourceCandidateFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IResourceCandidateFactory {
    type Vtable = IResourceCandidateFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IResourceCandidateFactory {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xbb2b30f8_c19b_5f43_88d9_69ad728a32f4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceCandidateFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        kind: ResourceCandidateKind,
        data: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub CreateInstance2: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        data_array_size: u32,
        data: *const u8,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IResourceContext(::windows_core::IUnknown);
impl IResourceContext {
    #[doc = "Required features: `\"Windows_Foundation_Collections\"`"]
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub fn QualifierValues(
        &self,
    ) -> ::windows_core::Result<
        ::windows::Foundation::Collections::IMap<::windows_core::HSTRING, ::windows_core::HSTRING>,
    > {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).QualifierValues)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(
    IResourceContext,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::RuntimeType for IResourceContext {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_interface::<Self>();
}
unsafe impl ::windows_core::Interface for IResourceContext {
    type Vtable = IResourceContext_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IResourceContext {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x96fb48dc_f77d_55ff_af12_34861e3d4939);
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceContext_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub QualifierValues: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation_Collections"))]
    QualifierValues: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IResourceContext2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IResourceContext2 {
    type Vtable = IResourceContext2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IResourceContext2 {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x7a3b1158_798c_5949_969d_03510b9ce6ca);
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceContext2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IResourceLoader(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IResourceLoader {
    type Vtable = IResourceLoader_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IResourceLoader {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xbc3f76bf_da46_54cd_8715_8b8aaf16eaac);
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceLoader_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub GetString: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        resourceid: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_Foundation")]
    pub GetStringForUri: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        resourceuri: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    GetStringForUri: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IResourceLoaderFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IResourceLoaderFactory {
    type Vtable = IResourceLoaderFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IResourceLoaderFactory {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x871f83aa_fb34_50d6_b9b9_2c35f3ffc004);
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceLoaderFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        filename: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub CreateInstance2: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        filename: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        resourcemap: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IResourceLoaderStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IResourceLoaderStatics {
    type Vtable = IResourceLoaderStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IResourceLoaderStatics {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xec9c894a_1466_5f2f_8eee_a70cbd2b51bb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceLoaderStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub GetDefaultResourceFilePath: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IResourceManager(::windows_core::IUnknown);
impl IResourceManager {
    pub fn MainResourceMap(&self) -> ::windows_core::Result<ResourceMap> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MainResourceMap)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn CreateResourceContext(&self) -> ::windows_core::Result<ResourceContext> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateResourceContext)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn ResourceNotFound<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<ResourceManager, ResourceNotFoundEventArgs>,
        >,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ResourceNotFound)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveResourceNotFound(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveResourceNotFound)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
}
::windows_core::imp::interface_hierarchy!(
    IResourceManager,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::RuntimeType for IResourceManager {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_interface::<Self>();
}
unsafe impl ::windows_core::Interface for IResourceManager {
    type Vtable = IResourceManager_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IResourceManager {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xac2291ef_81be_5c99_a0ae_bcee0180b8a8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceManager_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub MainResourceMap: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub CreateResourceContext: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_Foundation")]
    pub ResourceNotFound: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    ResourceNotFound: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub RemoveResourceNotFound: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RemoveResourceNotFound: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IResourceManager2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IResourceManager2 {
    type Vtable = IResourceManager2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IResourceManager2 {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x7ec10160_a154_5c42_8268_30e306b1f585);
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceManager2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IResourceManagerFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IResourceManagerFactory {
    type Vtable = IResourceManagerFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IResourceManagerFactory {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xd6acf18f_458a_535b_a5c4_ac2dc4e49099);
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceManagerFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        filename: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IResourceMap(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IResourceMap {
    type Vtable = IResourceMap_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IResourceMap {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x4abbd9bc_df4e_5c7b_812c_7e7bb0c22377);
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceMap_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ResourceCount: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut u32,
    ) -> ::windows_core::HRESULT,
    pub GetSubtree: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        reference: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub TryGetSubtree: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        reference: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub GetValue: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        resource: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub GetValueWithContext: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        resource: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        context: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub GetValueByIndex: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        index: u32,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation_Collections"))]
    GetValueByIndex: usize,
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub GetValueByIndexWithContext: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        index: u32,
        context: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation_Collections"))]
    GetValueByIndexWithContext: usize,
    pub TryGetValue: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        resource: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub TryGetValueWithContext: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        resource: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        context: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IResourceNotFoundEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IResourceNotFoundEventArgs {
    type Vtable = IResourceNotFoundEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IResourceNotFoundEventArgs {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x64abb08b_e77d_5b26_830f_15941e0e8200);
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceNotFoundEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Context: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub Name: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub SetResolvedCandidate: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        candidate: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
pub struct KnownResourceQualifierName;
impl KnownResourceQualifierName {
    pub fn Contrast() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IKnownResourceQualifierNameStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Contrast)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn Custom() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IKnownResourceQualifierNameStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Custom)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn DeviceFamily() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IKnownResourceQualifierNameStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DeviceFamily)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn HomeRegion() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IKnownResourceQualifierNameStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HomeRegion)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn Language() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IKnownResourceQualifierNameStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Language)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn LayoutDirection() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IKnownResourceQualifierNameStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LayoutDirection)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn Scale() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IKnownResourceQualifierNameStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Scale)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn TargetSize() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IKnownResourceQualifierNameStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TargetSize)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn Theme() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IKnownResourceQualifierNameStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Theme)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IKnownResourceQualifierNameStatics<
        R,
        F: FnOnce(&IKnownResourceQualifierNameStatics) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<
            KnownResourceQualifierName,
            IKnownResourceQualifierNameStatics,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeName for KnownResourceQualifierName {
    const NAME: &'static str =
        "Microsoft.Windows.ApplicationModel.Resources.KnownResourceQualifierName";
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ResourceCandidate(::windows_core::IUnknown);
impl ResourceCandidate {
    pub fn ValueAsString(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ValueAsString)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn ValueAsBytes(&self) -> ::windows_core::Result<::windows_core::Array<u8>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows_core::Interface::vtable(this).ValueAsBytes)(
                ::windows_core::Interface::as_raw(this),
                ::windows_core::Array::<u8>::set_abi_len(::std::mem::transmute(&mut result__)),
                result__.as_mut_ptr() as *mut _ as _,
            )
            .and_then(|| result__.assume_init())
        }
    }
    pub fn Kind(&self) -> ::windows_core::Result<ResourceCandidateKind> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation_Collections\"`"]
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub fn QualifierValues(
        &self,
    ) -> ::windows_core::Result<
        ::windows::Foundation::Collections::IMapView<
            ::windows_core::HSTRING,
            ::windows_core::HSTRING,
        >,
    > {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).QualifierValues)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn CreateInstance(
        kind: ResourceCandidateKind,
        data: &::windows_core::HSTRING,
    ) -> ::windows_core::Result<ResourceCandidate> {
        Self::IResourceCandidateFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstance)(
                ::windows_core::Interface::as_raw(this),
                kind,
                ::core::mem::transmute_copy(data),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn CreateInstance2(data: &[u8]) -> ::windows_core::Result<ResourceCandidate> {
        Self::IResourceCandidateFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstance2)(
                ::windows_core::Interface::as_raw(this),
                data.len().try_into().unwrap(),
                data.as_ptr(),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IResourceCandidateFactory<
        R,
        F: FnOnce(&IResourceCandidateFactory) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<
            ResourceCandidate,
            IResourceCandidateFactory,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for ResourceCandidate {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for ResourceCandidate {
    type Vtable = IResourceCandidate_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ResourceCandidate {
    const IID: ::windows_core::GUID = <IResourceCandidate as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ResourceCandidate {
    const NAME: &'static str = "Microsoft.Windows.ApplicationModel.Resources.ResourceCandidate";
}
::windows_core::imp::interface_hierarchy!(
    ResourceCandidate,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for ResourceCandidate {}
unsafe impl ::core::marker::Sync for ResourceCandidate {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ResourceContext(::windows_core::IUnknown);
impl ResourceContext {
    #[doc = "Required features: `\"Windows_Foundation_Collections\"`"]
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub fn QualifierValues(
        &self,
    ) -> ::windows_core::Result<
        ::windows::Foundation::Collections::IMap<::windows_core::HSTRING, ::windows_core::HSTRING>,
    > {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).QualifierValues)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for ResourceContext {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for ResourceContext {
    type Vtable = IResourceContext_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ResourceContext {
    const IID: ::windows_core::GUID = <IResourceContext as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ResourceContext {
    const NAME: &'static str = "Microsoft.Windows.ApplicationModel.Resources.ResourceContext";
}
::windows_core::imp::interface_hierarchy!(
    ResourceContext,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<IResourceContext> for ResourceContext {}
unsafe impl ::core::marker::Send for ResourceContext {}
unsafe impl ::core::marker::Sync for ResourceContext {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ResourceLoader(::windows_core::IUnknown);
impl ResourceLoader {
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
            ResourceLoader,
            ::windows_core::imp::IGenericFactory,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn GetString(
        &self,
        resourceid: &::windows_core::HSTRING,
    ) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetString)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(resourceid),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn GetStringForUri<P0>(
        &self,
        resourceuri: P0,
    ) -> ::windows_core::Result<::windows_core::HSTRING>
    where
        P0: ::windows_core::IntoParam<::windows::Foundation::Uri>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetStringForUri)(
                ::windows_core::Interface::as_raw(this),
                resourceuri.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn CreateInstance(
        filename: &::windows_core::HSTRING,
    ) -> ::windows_core::Result<ResourceLoader> {
        Self::IResourceLoaderFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstance)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(filename),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn CreateInstance2(
        filename: &::windows_core::HSTRING,
        resourcemap: &::windows_core::HSTRING,
    ) -> ::windows_core::Result<ResourceLoader> {
        Self::IResourceLoaderFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstance2)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(filename),
                ::core::mem::transmute_copy(resourcemap),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn GetDefaultResourceFilePath() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IResourceLoaderStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetDefaultResourceFilePath)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IResourceLoaderFactory<
        R,
        F: FnOnce(&IResourceLoaderFactory) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<ResourceLoader, IResourceLoaderFactory> =
            ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IResourceLoaderStatics<
        R,
        F: FnOnce(&IResourceLoaderStatics) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<ResourceLoader, IResourceLoaderStatics> =
            ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for ResourceLoader {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for ResourceLoader {
    type Vtable = IResourceLoader_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ResourceLoader {
    const IID: ::windows_core::GUID = <IResourceLoader as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ResourceLoader {
    const NAME: &'static str = "Microsoft.Windows.ApplicationModel.Resources.ResourceLoader";
}
::windows_core::imp::interface_hierarchy!(
    ResourceLoader,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for ResourceLoader {}
unsafe impl ::core::marker::Sync for ResourceLoader {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ResourceManager(::windows_core::IUnknown);
impl ResourceManager {
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
            ResourceManager,
            ::windows_core::imp::IGenericFactory,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn MainResourceMap(&self) -> ::windows_core::Result<ResourceMap> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MainResourceMap)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn CreateResourceContext(&self) -> ::windows_core::Result<ResourceContext> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateResourceContext)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn ResourceNotFound<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<ResourceManager, ResourceNotFoundEventArgs>,
        >,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ResourceNotFound)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveResourceNotFound(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveResourceNotFound)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn CreateInstance(
        filename: &::windows_core::HSTRING,
    ) -> ::windows_core::Result<ResourceManager> {
        Self::IResourceManagerFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstance)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(filename),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IResourceManagerFactory<
        R,
        F: FnOnce(&IResourceManagerFactory) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<ResourceManager, IResourceManagerFactory> =
            ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for ResourceManager {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for ResourceManager {
    type Vtable = IResourceManager_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ResourceManager {
    const IID: ::windows_core::GUID = <IResourceManager as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ResourceManager {
    const NAME: &'static str = "Microsoft.Windows.ApplicationModel.Resources.ResourceManager";
}
::windows_core::imp::interface_hierarchy!(
    ResourceManager,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<IResourceManager> for ResourceManager {}
unsafe impl ::core::marker::Send for ResourceManager {}
unsafe impl ::core::marker::Sync for ResourceManager {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ResourceMap(::windows_core::IUnknown);
impl ResourceMap {
    pub fn ResourceCount(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ResourceCount)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn GetSubtree(
        &self,
        reference: &::windows_core::HSTRING,
    ) -> ::windows_core::Result<ResourceMap> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetSubtree)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(reference),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn TryGetSubtree(
        &self,
        reference: &::windows_core::HSTRING,
    ) -> ::windows_core::Result<ResourceMap> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TryGetSubtree)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(reference),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn GetValue(
        &self,
        resource: &::windows_core::HSTRING,
    ) -> ::windows_core::Result<ResourceCandidate> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetValue)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(resource),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn GetValueWithContext<P0>(
        &self,
        resource: &::windows_core::HSTRING,
        context: P0,
    ) -> ::windows_core::Result<ResourceCandidate>
    where
        P0: ::windows_core::IntoParam<ResourceContext>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetValueWithContext)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(resource),
                context.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation_Collections\"`"]
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub fn GetValueByIndex(
        &self,
        index: u32,
    ) -> ::windows_core::Result<
        ::windows::Foundation::Collections::IKeyValuePair<
            ::windows_core::HSTRING,
            ResourceCandidate,
        >,
    > {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetValueByIndex)(
                ::windows_core::Interface::as_raw(this),
                index,
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation_Collections\"`"]
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub fn GetValueByIndexWithContext<P0>(
        &self,
        index: u32,
        context: P0,
    ) -> ::windows_core::Result<
        ::windows::Foundation::Collections::IKeyValuePair<
            ::windows_core::HSTRING,
            ResourceCandidate,
        >,
    >
    where
        P0: ::windows_core::IntoParam<ResourceContext>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetValueByIndexWithContext)(
                ::windows_core::Interface::as_raw(this),
                index,
                context.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn TryGetValue(
        &self,
        resource: &::windows_core::HSTRING,
    ) -> ::windows_core::Result<ResourceCandidate> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TryGetValue)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(resource),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn TryGetValueWithContext<P0>(
        &self,
        resource: &::windows_core::HSTRING,
        context: P0,
    ) -> ::windows_core::Result<ResourceCandidate>
    where
        P0: ::windows_core::IntoParam<ResourceContext>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TryGetValueWithContext)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(resource),
                context.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for ResourceMap {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for ResourceMap {
    type Vtable = IResourceMap_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ResourceMap {
    const IID: ::windows_core::GUID = <IResourceMap as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ResourceMap {
    const NAME: &'static str = "Microsoft.Windows.ApplicationModel.Resources.ResourceMap";
}
::windows_core::imp::interface_hierarchy!(
    ResourceMap,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for ResourceMap {}
unsafe impl ::core::marker::Sync for ResourceMap {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ResourceNotFoundEventArgs(::windows_core::IUnknown);
impl ResourceNotFoundEventArgs {
    pub fn Context(&self) -> ::windows_core::Result<ResourceContext> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Context)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn Name(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Name)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetResolvedCandidate<P0>(&self, candidate: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<ResourceCandidate>,
    {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetResolvedCandidate)(
                ::windows_core::Interface::as_raw(this),
                candidate.into_param().abi(),
            )
            .ok()
        }
    }
}
impl ::windows_core::RuntimeType for ResourceNotFoundEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for ResourceNotFoundEventArgs {
    type Vtable = IResourceNotFoundEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ResourceNotFoundEventArgs {
    const IID: ::windows_core::GUID =
        <IResourceNotFoundEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ResourceNotFoundEventArgs {
    const NAME: &'static str =
        "Microsoft.Windows.ApplicationModel.Resources.ResourceNotFoundEventArgs";
}
::windows_core::imp::interface_hierarchy!(
    ResourceNotFoundEventArgs,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for ResourceNotFoundEventArgs {}
unsafe impl ::core::marker::Sync for ResourceNotFoundEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ResourceCandidateKind(pub i32);
impl ResourceCandidateKind {
    pub const Unknown: Self = Self(0i32);
    pub const String: Self = Self(1i32);
    pub const FilePath: Self = Self(2i32);
    pub const EmbeddedData: Self = Self(3i32);
}
impl ::core::marker::Copy for ResourceCandidateKind {}
impl ::core::clone::Clone for ResourceCandidateKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ResourceCandidateKind {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for ResourceCandidateKind {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for ResourceCandidateKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ResourceCandidateKind").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ResourceCandidateKind {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::from_slice(
            b"enum(Microsoft.Windows.ApplicationModel.Resources.ResourceCandidateKind;i4)",
        );
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
