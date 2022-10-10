#[doc(hidden)]
#[repr(transparent)]
pub struct IDeploymentInitializeOptions(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IDeploymentInitializeOptions {
    type Vtable = IDeploymentInitializeOptions_Vtbl;
}
unsafe impl ::windows::core::Interface for IDeploymentInitializeOptions {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x578a5fd4_9d7f_5e01_97b8_d8ea61db4027);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeploymentInitializeOptions_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ForceDeployment: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub SetForceDeployment: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDeploymentInitializeOptions2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IDeploymentInitializeOptions2 {
    type Vtable = IDeploymentInitializeOptions2_Vtbl;
}
unsafe impl ::windows::core::Interface for IDeploymentInitializeOptions2 {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xad902820_149f_5e16_a566_9b2363997de2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeploymentInitializeOptions2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub OnErrorShowUI: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub SetOnErrorShowUI: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDeploymentManagerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IDeploymentManagerStatics {
    type Vtable = IDeploymentManagerStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IDeploymentManagerStatics {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x6782a9d0_bfd0_50ea_81b0_32e9ed37cdf0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeploymentManagerStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetStatus: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub Initialize: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDeploymentManagerStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IDeploymentManagerStatics2 {
    type Vtable = IDeploymentManagerStatics2_Vtbl;
}
unsafe impl ::windows::core::Interface for IDeploymentManagerStatics2 {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xf49c16ee_6ebc_5f15_bebb_2ba49f8c0b30);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeploymentManagerStatics2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Initialize: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        deploymentinitializeoptions: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDeploymentResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IDeploymentResult {
    type Vtable = IDeploymentResult_Vtbl;
}
unsafe impl ::windows::core::Interface for IDeploymentResult {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x27203f62_463d_587a_8eb7_870098901078);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeploymentResult_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut DeploymentStatus,
    ) -> ::windows::core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::HRESULT,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDeploymentResultFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IDeploymentResultFactory {
    type Vtable = IDeploymentResultFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for IDeploymentResultFactory {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xacd7bdae_4ae6_5cac_8205_1e8c305f953b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeploymentResultFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        status: DeploymentStatus,
        extendederror: ::windows::core::HRESULT,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IReleaseInfoStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IReleaseInfoStatics {
    type Vtable = IReleaseInfoStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IReleaseInfoStatics {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xed9be8ff_073c_5c66_bf97_ef0ce67405c3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IReleaseInfoStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Major: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut u16,
    ) -> ::windows::core::HRESULT,
    pub Minor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut u16,
    ) -> ::windows::core::HRESULT,
    pub Patch: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut u16,
    ) -> ::windows::core::HRESULT,
    pub VersionTag: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub AsString: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRuntimeInfoStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IRuntimeInfoStatics {
    type Vtable = IRuntimeInfoStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IRuntimeInfoStatics {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xe5cb9549_8951_590e_a753_8f281cd77ab5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRuntimeInfoStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Version: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::ApplicationModel::PackageVersion,
    ) -> ::windows::core::HRESULT,
    pub AsString: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Windows_ApplicationModel_WindowsAppRuntime\"`*"]
#[repr(transparent)]
pub struct DeploymentInitializeOptions(::windows::core::IUnknown);
impl DeploymentInitializeOptions {
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
            DeploymentInitializeOptions,
            ::windows::core::IGenericFactory,
        > = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn ForceDeployment(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ForceDeployment)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetForceDeployment(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetForceDeployment)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn OnErrorShowUI(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IDeploymentInitializeOptions2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).OnErrorShowUI)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetOnErrorShowUI(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IDeploymentInitializeOptions2>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetOnErrorShowUI)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
}
impl ::core::clone::Clone for DeploymentInitializeOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DeploymentInitializeOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DeploymentInitializeOptions {}
impl ::core::fmt::Debug for DeploymentInitializeOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DeploymentInitializeOptions").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DeploymentInitializeOptions {
    const SIGNATURE : ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice ( b"rc(Microsoft.Windows.ApplicationModel.WindowsAppRuntime.DeploymentInitializeOptions;{578a5fd4-9d7f-5e01-97b8-d8ea61db4027})" ) ;
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for DeploymentInitializeOptions {
    type Vtable = IDeploymentInitializeOptions_Vtbl;
}
unsafe impl ::windows::core::Interface for DeploymentInitializeOptions {
    const IID: ::windows::core::GUID =
        <IDeploymentInitializeOptions as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for DeploymentInitializeOptions {
    const NAME: &'static str =
        "Microsoft.Windows.ApplicationModel.WindowsAppRuntime.DeploymentInitializeOptions";
}
::windows::core::interface_hierarchy!(
    DeploymentInitializeOptions,
    ::windows::core::IUnknown,
    ::windows::core::IInspectable
);
unsafe impl ::core::marker::Send for DeploymentInitializeOptions {}
unsafe impl ::core::marker::Sync for DeploymentInitializeOptions {}
#[doc = "*Required features: `\"Windows_ApplicationModel_WindowsAppRuntime\"`*"]
pub struct DeploymentManager;
impl DeploymentManager {
    pub fn GetStatus() -> ::windows::core::Result<DeploymentResult> {
        Self::IDeploymentManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetStatus)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<DeploymentResult>(result__)
        })
    }
    pub fn Initialize() -> ::windows::core::Result<DeploymentResult> {
        Self::IDeploymentManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Initialize)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<DeploymentResult>(result__)
        })
    }
    pub fn Initialize2(
        deploymentinitializeoptions: &DeploymentInitializeOptions,
    ) -> ::windows::core::Result<DeploymentResult> {
        Self::IDeploymentManagerStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Initialize)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(deploymentinitializeoptions),
                result__.as_mut_ptr(),
            )
            .from_abi::<DeploymentResult>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IDeploymentManagerStatics<
        R,
        F: FnOnce(&IDeploymentManagerStatics) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<DeploymentManager, IDeploymentManagerStatics> =
            ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IDeploymentManagerStatics2<
        R,
        F: FnOnce(&IDeploymentManagerStatics2) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<
            DeploymentManager,
            IDeploymentManagerStatics2,
        > = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for DeploymentManager {
    const NAME: &'static str =
        "Microsoft.Windows.ApplicationModel.WindowsAppRuntime.DeploymentManager";
}
#[doc = "*Required features: `\"Windows_ApplicationModel_WindowsAppRuntime\"`*"]
#[repr(transparent)]
pub struct DeploymentResult(::windows::core::IUnknown);
impl DeploymentResult {
    pub fn Status(&self) -> ::windows::core::Result<DeploymentStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Status)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<DeploymentStatus>(result__)
        }
    }
    pub fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ExtendedError)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::HRESULT>(result__)
        }
    }
    pub fn CreateInstance(
        status: DeploymentStatus,
        extendederror: ::windows::core::HRESULT,
    ) -> ::windows::core::Result<DeploymentResult> {
        Self::IDeploymentResultFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateInstance)(
                ::windows::core::Vtable::as_raw(this),
                status,
                extendederror,
                result__.as_mut_ptr(),
            )
            .from_abi::<DeploymentResult>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IDeploymentResultFactory<
        R,
        F: FnOnce(&IDeploymentResultFactory) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<DeploymentResult, IDeploymentResultFactory> =
            ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for DeploymentResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DeploymentResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DeploymentResult {}
impl ::core::fmt::Debug for DeploymentResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DeploymentResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DeploymentResult {
    const SIGNATURE : ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice ( b"rc(Microsoft.Windows.ApplicationModel.WindowsAppRuntime.DeploymentResult;{27203f62-463d-587a-8eb7-870098901078})" ) ;
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for DeploymentResult {
    type Vtable = IDeploymentResult_Vtbl;
}
unsafe impl ::windows::core::Interface for DeploymentResult {
    const IID: ::windows::core::GUID = <IDeploymentResult as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for DeploymentResult {
    const NAME: &'static str =
        "Microsoft.Windows.ApplicationModel.WindowsAppRuntime.DeploymentResult";
}
::windows::core::interface_hierarchy!(
    DeploymentResult,
    ::windows::core::IUnknown,
    ::windows::core::IInspectable
);
unsafe impl ::core::marker::Send for DeploymentResult {}
unsafe impl ::core::marker::Sync for DeploymentResult {}
#[doc = "*Required features: `\"Windows_ApplicationModel_WindowsAppRuntime\"`*"]
pub struct ReleaseInfo;
impl ReleaseInfo {
    pub fn Major() -> ::windows::core::Result<u16> {
        Self::IReleaseInfoStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Major)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<u16>(result__)
        })
    }
    pub fn Minor() -> ::windows::core::Result<u16> {
        Self::IReleaseInfoStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Minor)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<u16>(result__)
        })
    }
    pub fn Patch() -> ::windows::core::Result<u16> {
        Self::IReleaseInfoStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Patch)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<u16>(result__)
        })
    }
    pub fn VersionTag() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IReleaseInfoStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).VersionTag)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn AsString() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IReleaseInfoStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AsString)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IReleaseInfoStatics<R, F: FnOnce(&IReleaseInfoStatics) -> ::windows::core::Result<R>>(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<ReleaseInfo, IReleaseInfoStatics> =
            ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for ReleaseInfo {
    const NAME: &'static str = "Microsoft.Windows.ApplicationModel.WindowsAppRuntime.ReleaseInfo";
}
#[doc = "*Required features: `\"Windows_ApplicationModel_WindowsAppRuntime\"`*"]
pub struct RuntimeInfo;
impl RuntimeInfo {
    pub fn Version() -> ::windows::core::Result<::windows::ApplicationModel::PackageVersion> {
        Self::IRuntimeInfoStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Version)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::ApplicationModel::PackageVersion>(result__)
        })
    }
    pub fn AsString() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IRuntimeInfoStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AsString)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IRuntimeInfoStatics<R, F: FnOnce(&IRuntimeInfoStatics) -> ::windows::core::Result<R>>(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<RuntimeInfo, IRuntimeInfoStatics> =
            ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for RuntimeInfo {
    const NAME: &'static str = "Microsoft.Windows.ApplicationModel.WindowsAppRuntime.RuntimeInfo";
}
#[doc = "*Required features: `\"Windows_ApplicationModel_WindowsAppRuntime\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DeploymentStatus(pub i32);
impl DeploymentStatus {
    pub const Unknown: Self = Self(0i32);
    pub const Ok: Self = Self(1i32);
    pub const PackageInstallRequired: Self = Self(2i32);
    pub const PackageInstallFailed: Self = Self(3i32);
}
impl ::core::marker::Copy for DeploymentStatus {}
impl ::core::clone::Clone for DeploymentStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DeploymentStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DeploymentStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for DeploymentStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DeploymentStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DeploymentStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.Windows.ApplicationModel.WindowsAppRuntime.DeploymentStatus;i4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
