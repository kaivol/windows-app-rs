#[doc(hidden)]
#[repr(transparent)]
pub struct IKnownResourceQualifierNameStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IKnownResourceQualifierNameStatics {
    type Vtable = IKnownResourceQualifierNameStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IKnownResourceQualifierNameStatics {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xdd6cdedc_559b_50c8_ac53_82fe21f915f3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKnownResourceQualifierNameStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Contrast: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub Custom: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub DeviceFamily: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub HomeRegion: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub Language: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub LayoutDirection: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub Scale: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub TargetSize: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub Theme: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IResourceCandidate(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IResourceCandidate {
    type Vtable = IResourceCandidate_Vtbl;
}
unsafe impl ::windows::core::Interface for IResourceCandidate {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x6c54bc0c_ef1e_57b8_b478_34fece737356);
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceCandidate_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ValueAsString: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub ValueAsBytes: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result_size__: *mut u32,
        result__: *mut *mut u8,
    ) -> ::windows::core::HRESULT,
    pub Kind: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ResourceCandidateKind,
    ) -> ::windows::core::HRESULT,
    pub QualifierValues: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IResourceCandidateFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IResourceCandidateFactory {
    type Vtable = IResourceCandidateFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for IResourceCandidateFactory {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xbb2b30f8_c19b_5f43_88d9_69ad728a32f4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceCandidateFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        kind: ResourceCandidateKind,
        data: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub CreateInstance2: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        data_array_size: u32,
        data: *const u8,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Windows_ApplicationModel_Resources\"`*"]
#[repr(transparent)]
pub struct IResourceContext(::windows::core::IUnknown);
impl IResourceContext {
    pub fn QualifierValues(
        &self,
    ) -> ::windows::core::Result<
        ::windows::Foundation::Collections::IMap<
            ::windows::core::HSTRING,
            ::windows::core::HSTRING,
        >,
    > {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).QualifierValues)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::Collections::IMap<
                ::windows::core::HSTRING,
                ::windows::core::HSTRING,
            >>(result__)
        }
    }
}
::windows::core::interface_hierarchy!(
    IResourceContext,
    ::windows::core::IUnknown,
    ::windows::core::IInspectable
);
impl ::core::clone::Clone for IResourceContext {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IResourceContext {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IResourceContext {}
impl ::core::fmt::Debug for IResourceContext {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IResourceContext").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IResourceContext {
    const SIGNATURE: ::windows::core::ConstBuffer =
        ::windows::core::ConstBuffer::from_slice(b"{96fb48dc-f77d-55ff-af12-34861e3d4939}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for IResourceContext {
    type Vtable = IResourceContext_Vtbl;
}
unsafe impl ::windows::core::Interface for IResourceContext {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x96fb48dc_f77d_55ff_af12_34861e3d4939);
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceContext_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub QualifierValues: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IResourceContext2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IResourceContext2 {
    type Vtable = IResourceContext2_Vtbl;
}
unsafe impl ::windows::core::Interface for IResourceContext2 {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x7a3b1158_798c_5949_969d_03510b9ce6ca);
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceContext2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IResourceLoader(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IResourceLoader {
    type Vtable = IResourceLoader_Vtbl;
}
unsafe impl ::windows::core::Interface for IResourceLoader {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xbc3f76bf_da46_54cd_8715_8b8aaf16eaac);
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceLoader_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetString: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        resourceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub GetStringForUri: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        resourceuri: *mut ::core::ffi::c_void,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IResourceLoaderFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IResourceLoaderFactory {
    type Vtable = IResourceLoaderFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for IResourceLoaderFactory {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x871f83aa_fb34_50d6_b9b9_2c35f3ffc004);
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceLoaderFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        filename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub CreateInstance2: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        filename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        resourcemap: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IResourceLoaderStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IResourceLoaderStatics {
    type Vtable = IResourceLoaderStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IResourceLoaderStatics {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xec9c894a_1466_5f2f_8eee_a70cbd2b51bb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceLoaderStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetDefaultResourceFilePath: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Windows_ApplicationModel_Resources\"`*"]
#[repr(transparent)]
pub struct IResourceManager(::windows::core::IUnknown);
impl IResourceManager {
    pub fn MainResourceMap(&self) -> ::windows::core::Result<ResourceMap> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MainResourceMap)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<ResourceMap>(result__)
        }
    }
    pub fn CreateResourceContext(&self) -> ::windows::core::Result<ResourceContext> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateResourceContext)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<ResourceContext>(result__)
        }
    }
    pub fn ResourceNotFound(
        &self,
        handler: &::windows::Foundation::TypedEventHandler<
            ResourceManager,
            ResourceNotFoundEventArgs,
        >,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ResourceNotFound)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(handler),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveResourceNotFound(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).RemoveResourceNotFound)(
                ::windows::core::Vtable::as_raw(this),
                token,
            )
            .ok()
        }
    }
}
::windows::core::interface_hierarchy!(
    IResourceManager,
    ::windows::core::IUnknown,
    ::windows::core::IInspectable
);
impl ::core::clone::Clone for IResourceManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IResourceManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IResourceManager {}
impl ::core::fmt::Debug for IResourceManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IResourceManager").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IResourceManager {
    const SIGNATURE: ::windows::core::ConstBuffer =
        ::windows::core::ConstBuffer::from_slice(b"{ac2291ef-81be-5c99-a0ae-bcee0180b8a8}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for IResourceManager {
    type Vtable = IResourceManager_Vtbl;
}
unsafe impl ::windows::core::Interface for IResourceManager {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xac2291ef_81be_5c99_a0ae_bcee0180b8a8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceManager_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub MainResourceMap: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub CreateResourceContext: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub ResourceNotFound: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub RemoveResourceNotFound: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IResourceManager2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IResourceManager2 {
    type Vtable = IResourceManager2_Vtbl;
}
unsafe impl ::windows::core::Interface for IResourceManager2 {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x7ec10160_a154_5c42_8268_30e306b1f585);
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceManager2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IResourceManagerFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IResourceManagerFactory {
    type Vtable = IResourceManagerFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for IResourceManagerFactory {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xd6acf18f_458a_535b_a5c4_ac2dc4e49099);
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceManagerFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        filename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IResourceMap(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IResourceMap {
    type Vtable = IResourceMap_Vtbl;
}
unsafe impl ::windows::core::Interface for IResourceMap {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x4abbd9bc_df4e_5c7b_812c_7e7bb0c22377);
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceMap_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ResourceCount: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut u32,
    ) -> ::windows::core::HRESULT,
    pub GetSubtree: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        reference: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub TryGetSubtree: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        reference: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub GetValue: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        resource: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub GetValueWithContext: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        resource: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        context: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub GetValueByIndex: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        index: u32,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub GetValueByIndexWithContext: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        index: u32,
        context: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub TryGetValue: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        resource: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub TryGetValueWithContext: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        resource: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        context: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IResourceNotFoundEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IResourceNotFoundEventArgs {
    type Vtable = IResourceNotFoundEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IResourceNotFoundEventArgs {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x64abb08b_e77d_5b26_830f_15941e0e8200);
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceNotFoundEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Context: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub Name: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub SetResolvedCandidate: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        candidate: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Windows_ApplicationModel_Resources\"`*"]
pub struct KnownResourceQualifierName;
impl KnownResourceQualifierName {
    pub fn Contrast() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownResourceQualifierNameStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Contrast)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn Custom() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownResourceQualifierNameStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Custom)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn DeviceFamily() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownResourceQualifierNameStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DeviceFamily)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn HomeRegion() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownResourceQualifierNameStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).HomeRegion)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn Language() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownResourceQualifierNameStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Language)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn LayoutDirection() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownResourceQualifierNameStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LayoutDirection)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn Scale() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownResourceQualifierNameStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Scale)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn TargetSize() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownResourceQualifierNameStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TargetSize)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn Theme() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownResourceQualifierNameStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Theme)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IKnownResourceQualifierNameStatics<
        R,
        F: FnOnce(&IKnownResourceQualifierNameStatics) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<
            KnownResourceQualifierName,
            IKnownResourceQualifierNameStatics,
        > = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for KnownResourceQualifierName {
    const NAME: &'static str =
        "Microsoft.Windows.ApplicationModel.Resources.KnownResourceQualifierName";
}
#[doc = "*Required features: `\"Windows_ApplicationModel_Resources\"`*"]
#[repr(transparent)]
pub struct ResourceCandidate(::windows::core::IUnknown);
impl ResourceCandidate {
    pub fn ValueAsString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ValueAsString)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn ValueAsBytes(&self) -> ::windows::core::Result<::windows::core::Array<u8>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ValueAsBytes)(
                ::windows::core::Vtable::as_raw(this),
                ::windows::core::Array::<u8>::set_abi_len(result__.assume_init_mut()),
                result__.as_mut_ptr() as *mut _ as _,
            )
            .and_then(|| result__.assume_init())
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ResourceCandidateKind> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Kind)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<ResourceCandidateKind>(result__)
        }
    }
    pub fn QualifierValues(
        &self,
    ) -> ::windows::core::Result<
        ::windows::Foundation::Collections::IMapView<
            ::windows::core::HSTRING,
            ::windows::core::HSTRING,
        >,
    > {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).QualifierValues)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::Collections::IMapView<
                ::windows::core::HSTRING,
                ::windows::core::HSTRING,
            >>(result__)
        }
    }
    pub fn CreateInstance(
        kind: ResourceCandidateKind,
        data: &::windows::core::HSTRING,
    ) -> ::windows::core::Result<ResourceCandidate> {
        Self::IResourceCandidateFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateInstance)(
                ::windows::core::Vtable::as_raw(this),
                kind,
                ::core::mem::transmute_copy(data),
                result__.as_mut_ptr(),
            )
            .from_abi::<ResourceCandidate>(result__)
        })
    }
    pub fn CreateInstance2(data: &[u8]) -> ::windows::core::Result<ResourceCandidate> {
        Self::IResourceCandidateFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateInstance2)(
                ::windows::core::Vtable::as_raw(this),
                data.len() as u32,
                data.as_ptr(),
                result__.as_mut_ptr(),
            )
            .from_abi::<ResourceCandidate>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IResourceCandidateFactory<
        R,
        F: FnOnce(&IResourceCandidateFactory) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<ResourceCandidate, IResourceCandidateFactory> =
            ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for ResourceCandidate {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ResourceCandidate {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ResourceCandidate {}
impl ::core::fmt::Debug for ResourceCandidate {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ResourceCandidate").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ResourceCandidate {
    const SIGNATURE : ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice ( b"rc(Microsoft.Windows.ApplicationModel.Resources.ResourceCandidate;{6c54bc0c-ef1e-57b8-b478-34fece737356})" ) ;
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ResourceCandidate {
    type Vtable = IResourceCandidate_Vtbl;
}
unsafe impl ::windows::core::Interface for ResourceCandidate {
    const IID: ::windows::core::GUID = <IResourceCandidate as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ResourceCandidate {
    const NAME: &'static str = "Microsoft.Windows.ApplicationModel.Resources.ResourceCandidate";
}
::windows::core::interface_hierarchy!(
    ResourceCandidate,
    ::windows::core::IUnknown,
    ::windows::core::IInspectable
);
unsafe impl ::core::marker::Send for ResourceCandidate {}
unsafe impl ::core::marker::Sync for ResourceCandidate {}
#[doc = "*Required features: `\"Windows_ApplicationModel_Resources\"`*"]
#[repr(transparent)]
pub struct ResourceContext(::windows::core::IUnknown);
impl ResourceContext {
    pub fn QualifierValues(
        &self,
    ) -> ::windows::core::Result<
        ::windows::Foundation::Collections::IMap<
            ::windows::core::HSTRING,
            ::windows::core::HSTRING,
        >,
    > {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).QualifierValues)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::Collections::IMap<
                ::windows::core::HSTRING,
                ::windows::core::HSTRING,
            >>(result__)
        }
    }
}
impl ::core::clone::Clone for ResourceContext {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ResourceContext {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ResourceContext {}
impl ::core::fmt::Debug for ResourceContext {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ResourceContext").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ResourceContext {
    const SIGNATURE : ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice ( b"rc(Microsoft.Windows.ApplicationModel.Resources.ResourceContext;{96fb48dc-f77d-55ff-af12-34861e3d4939})" ) ;
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ResourceContext {
    type Vtable = IResourceContext_Vtbl;
}
unsafe impl ::windows::core::Interface for ResourceContext {
    const IID: ::windows::core::GUID = <IResourceContext as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ResourceContext {
    const NAME: &'static str = "Microsoft.Windows.ApplicationModel.Resources.ResourceContext";
}
::windows::core::interface_hierarchy!(
    ResourceContext,
    ::windows::core::IUnknown,
    ::windows::core::IInspectable
);
impl ::core::convert::TryFrom<ResourceContext> for IResourceContext {
    type Error = ::windows::core::Error;
    fn try_from(value: ResourceContext) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ResourceContext> for IResourceContext {
    type Error = ::windows::core::Error;
    fn try_from(value: &ResourceContext) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&ResourceContext>
    for ::windows::core::InParam<'a, IResourceContext>
{
    type Error = ::windows::core::Error;
    fn try_from(value: &ResourceContext) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for ResourceContext {}
unsafe impl ::core::marker::Sync for ResourceContext {}
#[doc = "*Required features: `\"Windows_ApplicationModel_Resources\"`*"]
#[repr(transparent)]
pub struct ResourceLoader(::windows::core::IUnknown);
impl ResourceLoader {
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
            ResourceLoader,
            ::windows::core::IGenericFactory,
        > = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn GetString(
        &self,
        resourceid: &::windows::core::HSTRING,
    ) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetString)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(resourceid),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn GetStringForUri(
        &self,
        resourceuri: &::windows::Foundation::Uri,
    ) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetStringForUri)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(resourceuri),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn CreateInstance(
        filename: &::windows::core::HSTRING,
    ) -> ::windows::core::Result<ResourceLoader> {
        Self::IResourceLoaderFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateInstance)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(filename),
                result__.as_mut_ptr(),
            )
            .from_abi::<ResourceLoader>(result__)
        })
    }
    pub fn CreateInstance2(
        filename: &::windows::core::HSTRING,
        resourcemap: &::windows::core::HSTRING,
    ) -> ::windows::core::Result<ResourceLoader> {
        Self::IResourceLoaderFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateInstance2)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(filename),
                ::core::mem::transmute_copy(resourcemap),
                result__.as_mut_ptr(),
            )
            .from_abi::<ResourceLoader>(result__)
        })
    }
    pub fn GetDefaultResourceFilePath() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IResourceLoaderStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDefaultResourceFilePath)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IResourceLoaderFactory<
        R,
        F: FnOnce(&IResourceLoaderFactory) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<ResourceLoader, IResourceLoaderFactory> =
            ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IResourceLoaderStatics<
        R,
        F: FnOnce(&IResourceLoaderStatics) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<ResourceLoader, IResourceLoaderStatics> =
            ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for ResourceLoader {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ResourceLoader {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ResourceLoader {}
impl ::core::fmt::Debug for ResourceLoader {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ResourceLoader").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ResourceLoader {
    const SIGNATURE : ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice ( b"rc(Microsoft.Windows.ApplicationModel.Resources.ResourceLoader;{bc3f76bf-da46-54cd-8715-8b8aaf16eaac})" ) ;
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ResourceLoader {
    type Vtable = IResourceLoader_Vtbl;
}
unsafe impl ::windows::core::Interface for ResourceLoader {
    const IID: ::windows::core::GUID = <IResourceLoader as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ResourceLoader {
    const NAME: &'static str = "Microsoft.Windows.ApplicationModel.Resources.ResourceLoader";
}
::windows::core::interface_hierarchy!(
    ResourceLoader,
    ::windows::core::IUnknown,
    ::windows::core::IInspectable
);
unsafe impl ::core::marker::Send for ResourceLoader {}
unsafe impl ::core::marker::Sync for ResourceLoader {}
#[doc = "*Required features: `\"Windows_ApplicationModel_Resources\"`*"]
#[repr(transparent)]
pub struct ResourceManager(::windows::core::IUnknown);
impl ResourceManager {
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
            ResourceManager,
            ::windows::core::IGenericFactory,
        > = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn MainResourceMap(&self) -> ::windows::core::Result<ResourceMap> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MainResourceMap)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<ResourceMap>(result__)
        }
    }
    pub fn CreateResourceContext(&self) -> ::windows::core::Result<ResourceContext> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateResourceContext)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<ResourceContext>(result__)
        }
    }
    pub fn ResourceNotFound(
        &self,
        handler: &::windows::Foundation::TypedEventHandler<
            ResourceManager,
            ResourceNotFoundEventArgs,
        >,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ResourceNotFound)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(handler),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveResourceNotFound(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).RemoveResourceNotFound)(
                ::windows::core::Vtable::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn CreateInstance(
        filename: &::windows::core::HSTRING,
    ) -> ::windows::core::Result<ResourceManager> {
        Self::IResourceManagerFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateInstance)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(filename),
                result__.as_mut_ptr(),
            )
            .from_abi::<ResourceManager>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IResourceManagerFactory<
        R,
        F: FnOnce(&IResourceManagerFactory) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<ResourceManager, IResourceManagerFactory> =
            ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for ResourceManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ResourceManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ResourceManager {}
impl ::core::fmt::Debug for ResourceManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ResourceManager").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ResourceManager {
    const SIGNATURE : ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice ( b"rc(Microsoft.Windows.ApplicationModel.Resources.ResourceManager;{ac2291ef-81be-5c99-a0ae-bcee0180b8a8})" ) ;
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ResourceManager {
    type Vtable = IResourceManager_Vtbl;
}
unsafe impl ::windows::core::Interface for ResourceManager {
    const IID: ::windows::core::GUID = <IResourceManager as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ResourceManager {
    const NAME: &'static str = "Microsoft.Windows.ApplicationModel.Resources.ResourceManager";
}
::windows::core::interface_hierarchy!(
    ResourceManager,
    ::windows::core::IUnknown,
    ::windows::core::IInspectable
);
impl ::core::convert::TryFrom<ResourceManager> for IResourceManager {
    type Error = ::windows::core::Error;
    fn try_from(value: ResourceManager) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ResourceManager> for IResourceManager {
    type Error = ::windows::core::Error;
    fn try_from(value: &ResourceManager) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&ResourceManager>
    for ::windows::core::InParam<'a, IResourceManager>
{
    type Error = ::windows::core::Error;
    fn try_from(value: &ResourceManager) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for ResourceManager {}
unsafe impl ::core::marker::Sync for ResourceManager {}
#[doc = "*Required features: `\"Windows_ApplicationModel_Resources\"`*"]
#[repr(transparent)]
pub struct ResourceMap(::windows::core::IUnknown);
impl ResourceMap {
    pub fn ResourceCount(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ResourceCount)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<u32>(result__)
        }
    }
    pub fn GetSubtree(
        &self,
        reference: &::windows::core::HSTRING,
    ) -> ::windows::core::Result<ResourceMap> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetSubtree)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(reference),
                result__.as_mut_ptr(),
            )
            .from_abi::<ResourceMap>(result__)
        }
    }
    pub fn TryGetSubtree(
        &self,
        reference: &::windows::core::HSTRING,
    ) -> ::windows::core::Result<ResourceMap> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TryGetSubtree)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(reference),
                result__.as_mut_ptr(),
            )
            .from_abi::<ResourceMap>(result__)
        }
    }
    pub fn GetValue(
        &self,
        resource: &::windows::core::HSTRING,
    ) -> ::windows::core::Result<ResourceCandidate> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetValue)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(resource),
                result__.as_mut_ptr(),
            )
            .from_abi::<ResourceCandidate>(result__)
        }
    }
    pub fn GetValueWithContext(
        &self,
        resource: &::windows::core::HSTRING,
        context: &ResourceContext,
    ) -> ::windows::core::Result<ResourceCandidate> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetValueWithContext)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(resource),
                ::core::mem::transmute_copy(context),
                result__.as_mut_ptr(),
            )
            .from_abi::<ResourceCandidate>(result__)
        }
    }
    pub fn GetValueByIndex(
        &self,
        index: u32,
    ) -> ::windows::core::Result<
        ::windows::Foundation::Collections::IKeyValuePair<
            ::windows::core::HSTRING,
            ResourceCandidate,
        >,
    > {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetValueByIndex)(
                ::windows::core::Vtable::as_raw(this),
                index,
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::Collections::IKeyValuePair<
                ::windows::core::HSTRING,
                ResourceCandidate,
            >>(result__)
        }
    }
    pub fn GetValueByIndexWithContext(
        &self,
        index: u32,
        context: &ResourceContext,
    ) -> ::windows::core::Result<
        ::windows::Foundation::Collections::IKeyValuePair<
            ::windows::core::HSTRING,
            ResourceCandidate,
        >,
    > {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetValueByIndexWithContext)(
                ::windows::core::Vtable::as_raw(this),
                index,
                ::core::mem::transmute_copy(context),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::Collections::IKeyValuePair<
                ::windows::core::HSTRING,
                ResourceCandidate,
            >>(result__)
        }
    }
    pub fn TryGetValue(
        &self,
        resource: &::windows::core::HSTRING,
    ) -> ::windows::core::Result<ResourceCandidate> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TryGetValue)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(resource),
                result__.as_mut_ptr(),
            )
            .from_abi::<ResourceCandidate>(result__)
        }
    }
    pub fn TryGetValueWithContext(
        &self,
        resource: &::windows::core::HSTRING,
        context: &ResourceContext,
    ) -> ::windows::core::Result<ResourceCandidate> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TryGetValueWithContext)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(resource),
                ::core::mem::transmute_copy(context),
                result__.as_mut_ptr(),
            )
            .from_abi::<ResourceCandidate>(result__)
        }
    }
}
impl ::core::clone::Clone for ResourceMap {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ResourceMap {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ResourceMap {}
impl ::core::fmt::Debug for ResourceMap {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ResourceMap").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ResourceMap {
    const SIGNATURE : ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice ( b"rc(Microsoft.Windows.ApplicationModel.Resources.ResourceMap;{4abbd9bc-df4e-5c7b-812c-7e7bb0c22377})" ) ;
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ResourceMap {
    type Vtable = IResourceMap_Vtbl;
}
unsafe impl ::windows::core::Interface for ResourceMap {
    const IID: ::windows::core::GUID = <IResourceMap as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ResourceMap {
    const NAME: &'static str = "Microsoft.Windows.ApplicationModel.Resources.ResourceMap";
}
::windows::core::interface_hierarchy!(
    ResourceMap,
    ::windows::core::IUnknown,
    ::windows::core::IInspectable
);
unsafe impl ::core::marker::Send for ResourceMap {}
unsafe impl ::core::marker::Sync for ResourceMap {}
#[doc = "*Required features: `\"Windows_ApplicationModel_Resources\"`*"]
#[repr(transparent)]
pub struct ResourceNotFoundEventArgs(::windows::core::IUnknown);
impl ResourceNotFoundEventArgs {
    pub fn Context(&self) -> ::windows::core::Result<ResourceContext> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Context)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<ResourceContext>(result__)
        }
    }
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Name)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetResolvedCandidate(
        &self,
        candidate: &ResourceCandidate,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetResolvedCandidate)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(candidate),
            )
            .ok()
        }
    }
}
impl ::core::clone::Clone for ResourceNotFoundEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ResourceNotFoundEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ResourceNotFoundEventArgs {}
impl ::core::fmt::Debug for ResourceNotFoundEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ResourceNotFoundEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ResourceNotFoundEventArgs {
    const SIGNATURE : ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice ( b"rc(Microsoft.Windows.ApplicationModel.Resources.ResourceNotFoundEventArgs;{64abb08b-e77d-5b26-830f-15941e0e8200})" ) ;
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ResourceNotFoundEventArgs {
    type Vtable = IResourceNotFoundEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for ResourceNotFoundEventArgs {
    const IID: ::windows::core::GUID =
        <IResourceNotFoundEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ResourceNotFoundEventArgs {
    const NAME: &'static str =
        "Microsoft.Windows.ApplicationModel.Resources.ResourceNotFoundEventArgs";
}
::windows::core::interface_hierarchy!(
    ResourceNotFoundEventArgs,
    ::windows::core::IUnknown,
    ::windows::core::IInspectable
);
unsafe impl ::core::marker::Send for ResourceNotFoundEventArgs {}
unsafe impl ::core::marker::Sync for ResourceNotFoundEventArgs {}
#[doc = "*Required features: `\"Windows_ApplicationModel_Resources\"`*"]
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
unsafe impl ::windows::core::Abi for ResourceCandidateKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for ResourceCandidateKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ResourceCandidateKind").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ResourceCandidateKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.Windows.ApplicationModel.Resources.ResourceCandidateKind;i4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
