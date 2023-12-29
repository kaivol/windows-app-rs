#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IAddPackageDependencyOptions(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAddPackageDependencyOptions {
    type Vtable = IAddPackageDependencyOptions_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IAddPackageDependencyOptions {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x01b801fd_24e3_5e6b_9f1c_805ab410b604);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAddPackageDependencyOptions_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Rank: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut i32,
    ) -> ::windows_core::HRESULT,
    pub SetRank: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: i32,
    ) -> ::windows_core::HRESULT,
    pub PrependIfRankCollision: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetPrependIfRankCollision: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICreatePackageDependencyOptions(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICreatePackageDependencyOptions {
    type Vtable = ICreatePackageDependencyOptions_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICreatePackageDependencyOptions {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xcdbb820f_3c69_55dc_a017_b4132574c5d6);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICreatePackageDependencyOptions_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Architectures: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut PackageDependencyProcessorArchitectures,
    ) -> ::windows_core::HRESULT,
    pub SetArchitectures: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: PackageDependencyProcessorArchitectures,
    ) -> ::windows_core::HRESULT,
    pub VerifyDependencyResolution: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetVerifyDependencyResolution: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
    pub LifetimeArtifactKind: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut PackageDependencyLifetimeArtifactKind,
    ) -> ::windows_core::HRESULT,
    pub SetLifetimeArtifactKind: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: PackageDependencyLifetimeArtifactKind,
    ) -> ::windows_core::HRESULT,
    pub LifetimeArtifact: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub SetLifetimeArtifact: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPackageDependency(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPackageDependency {
    type Vtable = IPackageDependency_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPackageDependency {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x32ae7b95_e358_5a48_9669_c97d85ad6556);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageDependency_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub Delete:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Add: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub Add2: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        options: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPackageDependencyContext(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPackageDependencyContext {
    type Vtable = IPackageDependencyContext_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPackageDependencyContext {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x9902c35a_a3f5_5645_af0f_cdf9fca00d5e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageDependencyContext_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ContextId: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut PackageDependencyContextId,
    ) -> ::windows_core::HRESULT,
    pub PackageDependencyId: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub PackageFullName: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub Remove:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPackageDependencyContextFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPackageDependencyContextFactory {
    type Vtable = IPackageDependencyContextFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPackageDependencyContextFactory {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x9914f24f_bebf_516b_adab_5c3e8bf323f8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageDependencyContextFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        contextid: PackageDependencyContextId,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPackageDependencyRankStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPackageDependencyRankStatics {
    type Vtable = IPackageDependencyRankStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPackageDependencyRankStatics {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x260583bd_a4ab_53fd_a190_c446bfdb5384);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageDependencyRankStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Default: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut i32,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPackageDependencyStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPackageDependencyStatics {
    type Vtable = IPackageDependencyStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPackageDependencyStatics {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x17b656e1_1a58_5f3c_84a8_4430f6e749c2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageDependencyStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub GetFromId: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        id: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub GetFromIdForSystem: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        id: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_ApplicationModel")]
    pub Create: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        packagefamilyname: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        minversion: ::windows::ApplicationModel::PackageVersion,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_ApplicationModel"))]
    Create: usize,
    #[cfg(feature = "Windows_ApplicationModel")]
    pub Create2: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        packagefamilyname: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        minversion: ::windows::ApplicationModel::PackageVersion,
        options: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_ApplicationModel"))]
    Create2: usize,
    #[cfg(feature = "Windows_ApplicationModel")]
    pub CreateForSystem: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        packagefamilyname: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        minversion: ::windows::ApplicationModel::PackageVersion,
        options: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_ApplicationModel"))]
    CreateForSystem: usize,
    pub GenerationId: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut u32,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPackageDependencyStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPackageDependencyStatics2 {
    type Vtable = IPackageDependencyStatics2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPackageDependencyStatics2 {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xc7c6e4f3_c0ca_5fdb_bef2_57e6020ffe4e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageDependencyStatics2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub PackageGraphRevisionId: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut u32,
    ) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct AddPackageDependencyOptions(::windows_core::IUnknown);
impl AddPackageDependencyOptions {
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
            AddPackageDependencyOptions,
            ::windows_core::imp::IGenericFactory,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Rank(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Rank)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetRank(&self, value: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetRank)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn PrependIfRankCollision(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PrependIfRankCollision)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetPrependIfRankCollision(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetPrependIfRankCollision)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
}
impl ::windows_core::RuntimeType for AddPackageDependencyOptions {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for AddPackageDependencyOptions {
    type Vtable = IAddPackageDependencyOptions_Vtbl;
}
unsafe impl ::windows_core::ComInterface for AddPackageDependencyOptions {
    const IID: ::windows_core::GUID =
        <IAddPackageDependencyOptions as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for AddPackageDependencyOptions {
    const NAME: &'static str =
        "Microsoft.Windows.ApplicationModel.DynamicDependency.AddPackageDependencyOptions";
}
::windows_core::imp::interface_hierarchy!(
    AddPackageDependencyOptions,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for AddPackageDependencyOptions {}
unsafe impl ::core::marker::Sync for AddPackageDependencyOptions {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct CreatePackageDependencyOptions(::windows_core::IUnknown);
impl CreatePackageDependencyOptions {
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
            CreatePackageDependencyOptions,
            ::windows_core::imp::IGenericFactory,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Architectures(&self) -> ::windows_core::Result<PackageDependencyProcessorArchitectures> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Architectures)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetArchitectures(
        &self,
        value: PackageDependencyProcessorArchitectures,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetArchitectures)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn VerifyDependencyResolution(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).VerifyDependencyResolution)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetVerifyDependencyResolution(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetVerifyDependencyResolution)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn LifetimeArtifactKind(
        &self,
    ) -> ::windows_core::Result<PackageDependencyLifetimeArtifactKind> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LifetimeArtifactKind)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetLifetimeArtifactKind(
        &self,
        value: PackageDependencyLifetimeArtifactKind,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetLifetimeArtifactKind)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn LifetimeArtifact(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LifetimeArtifact)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetLifetimeArtifact(
        &self,
        value: &::windows_core::HSTRING,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetLifetimeArtifact)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
}
impl ::windows_core::RuntimeType for CreatePackageDependencyOptions {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for CreatePackageDependencyOptions {
    type Vtable = ICreatePackageDependencyOptions_Vtbl;
}
unsafe impl ::windows_core::ComInterface for CreatePackageDependencyOptions {
    const IID: ::windows_core::GUID =
        <ICreatePackageDependencyOptions as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for CreatePackageDependencyOptions {
    const NAME: &'static str =
        "Microsoft.Windows.ApplicationModel.DynamicDependency.CreatePackageDependencyOptions";
}
::windows_core::imp::interface_hierarchy!(
    CreatePackageDependencyOptions,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for CreatePackageDependencyOptions {}
unsafe impl ::core::marker::Sync for CreatePackageDependencyOptions {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct PackageDependency(::windows_core::IUnknown);
impl PackageDependency {
    pub fn Id(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Id)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn Delete(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).Delete)(::windows_core::Interface::as_raw(
                this,
            ))
            .ok()
        }
    }
    pub fn Add(&self) -> ::windows_core::Result<PackageDependencyContext> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Add)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn Add2<P0>(&self, options: P0) -> ::windows_core::Result<PackageDependencyContext>
    where
        P0: ::windows_core::IntoParam<AddPackageDependencyOptions>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Add2)(
                ::windows_core::Interface::as_raw(this),
                options.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn GetFromId(id: &::windows_core::HSTRING) -> ::windows_core::Result<PackageDependency> {
        Self::IPackageDependencyStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetFromId)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(id),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn GetFromIdForSystem(
        id: &::windows_core::HSTRING,
    ) -> ::windows_core::Result<PackageDependency> {
        Self::IPackageDependencyStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetFromIdForSystem)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(id),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_ApplicationModel\"`"]
    #[cfg(feature = "Windows_ApplicationModel")]
    pub fn Create(
        packagefamilyname: &::windows_core::HSTRING,
        minversion: ::windows::ApplicationModel::PackageVersion,
    ) -> ::windows_core::Result<PackageDependency> {
        Self::IPackageDependencyStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Create)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(packagefamilyname),
                minversion,
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_ApplicationModel\"`"]
    #[cfg(feature = "Windows_ApplicationModel")]
    pub fn Create2<P0>(
        packagefamilyname: &::windows_core::HSTRING,
        minversion: ::windows::ApplicationModel::PackageVersion,
        options: P0,
    ) -> ::windows_core::Result<PackageDependency>
    where
        P0: ::windows_core::IntoParam<CreatePackageDependencyOptions>,
    {
        Self::IPackageDependencyStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Create2)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(packagefamilyname),
                minversion,
                options.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_ApplicationModel\"`"]
    #[cfg(feature = "Windows_ApplicationModel")]
    pub fn CreateForSystem<P0>(
        packagefamilyname: &::windows_core::HSTRING,
        minversion: ::windows::ApplicationModel::PackageVersion,
        options: P0,
    ) -> ::windows_core::Result<PackageDependency>
    where
        P0: ::windows_core::IntoParam<CreatePackageDependencyOptions>,
    {
        Self::IPackageDependencyStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateForSystem)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(packagefamilyname),
                minversion,
                options.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn GenerationId() -> ::windows_core::Result<u32> {
        Self::IPackageDependencyStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GenerationId)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn PackageGraphRevisionId() -> ::windows_core::Result<u32> {
        Self::IPackageDependencyStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PackageGraphRevisionId)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPackageDependencyStatics<
        R,
        F: FnOnce(&IPackageDependencyStatics) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<
            PackageDependency,
            IPackageDependencyStatics,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IPackageDependencyStatics2<
        R,
        F: FnOnce(&IPackageDependencyStatics2) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<
            PackageDependency,
            IPackageDependencyStatics2,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for PackageDependency {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for PackageDependency {
    type Vtable = IPackageDependency_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PackageDependency {
    const IID: ::windows_core::GUID = <IPackageDependency as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PackageDependency {
    const NAME: &'static str =
        "Microsoft.Windows.ApplicationModel.DynamicDependency.PackageDependency";
}
::windows_core::imp::interface_hierarchy!(
    PackageDependency,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for PackageDependency {}
unsafe impl ::core::marker::Sync for PackageDependency {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct PackageDependencyContext(::windows_core::IUnknown);
impl PackageDependencyContext {
    pub fn ContextId(&self) -> ::windows_core::Result<PackageDependencyContextId> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ContextId)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn PackageDependencyId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PackageDependencyId)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn PackageFullName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PackageFullName)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn Remove(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).Remove)(::windows_core::Interface::as_raw(
                this,
            ))
            .ok()
        }
    }
    pub fn CreateInstance(
        contextid: PackageDependencyContextId,
    ) -> ::windows_core::Result<PackageDependencyContext> {
        Self::IPackageDependencyContextFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstance)(
                ::windows_core::Interface::as_raw(this),
                contextid,
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPackageDependencyContextFactory<
        R,
        F: FnOnce(&IPackageDependencyContextFactory) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<
            PackageDependencyContext,
            IPackageDependencyContextFactory,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for PackageDependencyContext {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for PackageDependencyContext {
    type Vtable = IPackageDependencyContext_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PackageDependencyContext {
    const IID: ::windows_core::GUID =
        <IPackageDependencyContext as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PackageDependencyContext {
    const NAME: &'static str =
        "Microsoft.Windows.ApplicationModel.DynamicDependency.PackageDependencyContext";
}
::windows_core::imp::interface_hierarchy!(
    PackageDependencyContext,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for PackageDependencyContext {}
unsafe impl ::core::marker::Sync for PackageDependencyContext {}
pub struct PackageDependencyRank;
impl PackageDependencyRank {
    pub fn Default() -> ::windows_core::Result<i32> {
        Self::IPackageDependencyRankStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Default)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPackageDependencyRankStatics<
        R,
        F: FnOnce(&IPackageDependencyRankStatics) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<
            PackageDependencyRank,
            IPackageDependencyRankStatics,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeName for PackageDependencyRank {
    const NAME: &'static str =
        "Microsoft.Windows.ApplicationModel.DynamicDependency.PackageDependencyRank";
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PackageDependencyLifetimeArtifactKind(pub i32);
impl PackageDependencyLifetimeArtifactKind {
    pub const Process: Self = Self(0i32);
    pub const FilePath: Self = Self(1i32);
    pub const RegistryKey: Self = Self(2i32);
}
impl ::core::marker::Copy for PackageDependencyLifetimeArtifactKind {}
impl ::core::clone::Clone for PackageDependencyLifetimeArtifactKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PackageDependencyLifetimeArtifactKind {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for PackageDependencyLifetimeArtifactKind {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for PackageDependencyLifetimeArtifactKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PackageDependencyLifetimeArtifactKind").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for PackageDependencyLifetimeArtifactKind {
    const SIGNATURE : ::windows_core::imp::ConstBuffer =::windows_core::imp::ConstBuffer::from_slice ( b"enum(Microsoft.Windows.ApplicationModel.DynamicDependency.PackageDependencyLifetimeArtifactKind;i4)" ) ;
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PackageDependencyProcessorArchitectures(pub u32);
impl PackageDependencyProcessorArchitectures {
    pub const None: Self = Self(0u32);
    pub const Neutral: Self = Self(1u32);
    pub const X86: Self = Self(2u32);
    pub const X64: Self = Self(4u32);
    pub const Arm: Self = Self(8u32);
    pub const Arm64: Self = Self(16u32);
    pub const X86OnArm64: Self = Self(32u32);
}
impl ::core::marker::Copy for PackageDependencyProcessorArchitectures {}
impl ::core::clone::Clone for PackageDependencyProcessorArchitectures {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PackageDependencyProcessorArchitectures {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for PackageDependencyProcessorArchitectures {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for PackageDependencyProcessorArchitectures {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PackageDependencyProcessorArchitectures").field(&self.0).finish()
    }
}
impl PackageDependencyProcessorArchitectures {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for PackageDependencyProcessorArchitectures {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for PackageDependencyProcessorArchitectures {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for PackageDependencyProcessorArchitectures {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for PackageDependencyProcessorArchitectures {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for PackageDependencyProcessorArchitectures {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::windows_core::RuntimeType for PackageDependencyProcessorArchitectures {
    const SIGNATURE : ::windows_core::imp::ConstBuffer =::windows_core::imp::ConstBuffer::from_slice ( b"enum(Microsoft.Windows.ApplicationModel.DynamicDependency.PackageDependencyProcessorArchitectures;u4)" ) ;
}
#[repr(C)]
pub struct PackageDependencyContextId {
    pub Id: u64,
}
impl ::core::marker::Copy for PackageDependencyContextId {}
impl ::core::clone::Clone for PackageDependencyContextId {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PackageDependencyContextId {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PackageDependencyContextId").field("Id", &self.Id).finish()
    }
}
impl ::windows_core::TypeKind for PackageDependencyContextId {
    type TypeKind = ::windows_core::CopyType;
}
impl ::windows_core::RuntimeType for PackageDependencyContextId {
    const SIGNATURE : ::windows_core::imp::ConstBuffer =::windows_core::imp::ConstBuffer::from_slice ( b"struct(Microsoft.Windows.ApplicationModel.DynamicDependency.PackageDependencyContextId;u8)" ) ;
}
impl ::core::cmp::PartialEq for PackageDependencyContextId {
    fn eq(&self, other: &Self) -> bool {
        self.Id == other.Id
    }
}
impl ::core::cmp::Eq for PackageDependencyContextId {}
impl ::core::default::Default for PackageDependencyContextId {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
