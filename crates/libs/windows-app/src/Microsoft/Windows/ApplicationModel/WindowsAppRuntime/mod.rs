#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDeploymentInitializeOptions(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDeploymentInitializeOptions {
    type Vtable = IDeploymentInitializeOptions_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDeploymentInitializeOptions {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x578a5fd4_9d7f_5e01_97b8_d8ea61db4027);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeploymentInitializeOptions_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ForceDeployment: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetForceDeployment: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDeploymentInitializeOptions2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDeploymentInitializeOptions2 {
    type Vtable = IDeploymentInitializeOptions2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDeploymentInitializeOptions2 {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xad902820_149f_5e16_a566_9b2363997de2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeploymentInitializeOptions2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub OnErrorShowUI: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetOnErrorShowUI: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDeploymentManagerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDeploymentManagerStatics {
    type Vtable = IDeploymentManagerStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDeploymentManagerStatics {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x6782a9d0_bfd0_50ea_81b0_32e9ed37cdf0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeploymentManagerStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub GetStatus: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub Initialize: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDeploymentManagerStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDeploymentManagerStatics2 {
    type Vtable = IDeploymentManagerStatics2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDeploymentManagerStatics2 {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xf49c16ee_6ebc_5f15_bebb_2ba49f8c0b30);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeploymentManagerStatics2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Initialize: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        deploymentinitializeoptions: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDeploymentResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDeploymentResult {
    type Vtable = IDeploymentResult_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDeploymentResult {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x27203f62_463d_587a_8eb7_870098901078);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeploymentResult_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut DeploymentStatus,
    ) -> ::windows_core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows_core::HRESULT,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDeploymentResultFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDeploymentResultFactory {
    type Vtable = IDeploymentResultFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDeploymentResultFactory {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xacd7bdae_4ae6_5cac_8205_1e8c305f953b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeploymentResultFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        status: DeploymentStatus,
        extendederror: ::windows_core::HRESULT,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct DeploymentInitializeOptions(::windows_core::IUnknown);
impl DeploymentInitializeOptions {
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
            DeploymentInitializeOptions,
            ::windows_core::imp::IGenericFactory,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn ForceDeployment(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ForceDeployment)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetForceDeployment(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetForceDeployment)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn OnErrorShowUI(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IDeploymentInitializeOptions2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OnErrorShowUI)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetOnErrorShowUI(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IDeploymentInitializeOptions2>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetOnErrorShowUI)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
}
impl ::windows_core::RuntimeType for DeploymentInitializeOptions {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for DeploymentInitializeOptions {
    type Vtable = IDeploymentInitializeOptions_Vtbl;
}
unsafe impl ::windows_core::ComInterface for DeploymentInitializeOptions {
    const IID: ::windows_core::GUID =
        <IDeploymentInitializeOptions as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for DeploymentInitializeOptions {
    const NAME: &'static str =
        "Microsoft.Windows.ApplicationModel.WindowsAppRuntime.DeploymentInitializeOptions";
}
::windows_core::imp::interface_hierarchy!(
    DeploymentInitializeOptions,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for DeploymentInitializeOptions {}
unsafe impl ::core::marker::Sync for DeploymentInitializeOptions {}
pub struct DeploymentManager;
impl DeploymentManager {
    pub fn GetStatus() -> ::windows_core::Result<DeploymentResult> {
        Self::IDeploymentManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetStatus)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn Initialize() -> ::windows_core::Result<DeploymentResult> {
        Self::IDeploymentManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Initialize)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn Initialize2<P0>(
        deploymentinitializeoptions: P0,
    ) -> ::windows_core::Result<DeploymentResult>
    where
        P0: ::windows_core::IntoParam<DeploymentInitializeOptions>,
    {
        Self::IDeploymentManagerStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Initialize)(
                ::windows_core::Interface::as_raw(this),
                deploymentinitializeoptions.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IDeploymentManagerStatics<
        R,
        F: FnOnce(&IDeploymentManagerStatics) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<
            DeploymentManager,
            IDeploymentManagerStatics,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IDeploymentManagerStatics2<
        R,
        F: FnOnce(&IDeploymentManagerStatics2) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<
            DeploymentManager,
            IDeploymentManagerStatics2,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeName for DeploymentManager {
    const NAME: &'static str =
        "Microsoft.Windows.ApplicationModel.WindowsAppRuntime.DeploymentManager";
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct DeploymentResult(::windows_core::IUnknown);
impl DeploymentResult {
    pub fn Status(&self) -> ::windows_core::Result<DeploymentStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Status)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn ExtendedError(&self) -> ::windows_core::Result<::windows_core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ExtendedError)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn CreateInstance(
        status: DeploymentStatus,
        extendederror: ::windows_core::HRESULT,
    ) -> ::windows_core::Result<DeploymentResult> {
        Self::IDeploymentResultFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstance)(
                ::windows_core::Interface::as_raw(this),
                status,
                extendederror,
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IDeploymentResultFactory<
        R,
        F: FnOnce(&IDeploymentResultFactory) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<
            DeploymentResult,
            IDeploymentResultFactory,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for DeploymentResult {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for DeploymentResult {
    type Vtable = IDeploymentResult_Vtbl;
}
unsafe impl ::windows_core::ComInterface for DeploymentResult {
    const IID: ::windows_core::GUID = <IDeploymentResult as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for DeploymentResult {
    const NAME: &'static str =
        "Microsoft.Windows.ApplicationModel.WindowsAppRuntime.DeploymentResult";
}
::windows_core::imp::interface_hierarchy!(
    DeploymentResult,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for DeploymentResult {}
unsafe impl ::core::marker::Sync for DeploymentResult {}
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
impl ::windows_core::TypeKind for DeploymentStatus {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for DeploymentStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DeploymentStatus").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for DeploymentStatus {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::from_slice(
            b"enum(Microsoft.Windows.ApplicationModel.WindowsAppRuntime.DeploymentStatus;i4)",
        );
}
