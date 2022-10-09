#[doc(hidden)]
#[repr(transparent)]
pub struct IAddPackageDependencyOptions(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IAddPackageDependencyOptions {
    type Vtable = IAddPackageDependencyOptions_Vtbl;
}
unsafe impl ::windows::core::Interface for IAddPackageDependencyOptions {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x01b801fd_24e3_5e6b_9f1c_805ab410b604);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAddPackageDependencyOptions_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Rank: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut i32,
    ) -> ::windows::core::HRESULT,
    pub SetRank: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: i32,
    ) -> ::windows::core::HRESULT,
    pub PrependIfRankCollision: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub SetPrependIfRankCollision: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICreatePackageDependencyOptions(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICreatePackageDependencyOptions {
    type Vtable = ICreatePackageDependencyOptions_Vtbl;
}
unsafe impl ::windows::core::Interface for ICreatePackageDependencyOptions {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xcdbb820f_3c69_55dc_a017_b4132574c5d6);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICreatePackageDependencyOptions_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Architectures: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut PackageDependencyProcessorArchitectures,
    ) -> ::windows::core::HRESULT,
    pub SetArchitectures: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: PackageDependencyProcessorArchitectures,
    ) -> ::windows::core::HRESULT,
    pub VerifyDependencyResolution: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub SetVerifyDependencyResolution: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows::core::HRESULT,
    pub LifetimeArtifactKind: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut PackageDependencyLifetimeArtifactKind,
    ) -> ::windows::core::HRESULT,
    pub SetLifetimeArtifactKind: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: PackageDependencyLifetimeArtifactKind,
    ) -> ::windows::core::HRESULT,
    pub LifetimeArtifact: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub SetLifetimeArtifact: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPackageDependency(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IPackageDependency {
    type Vtable = IPackageDependency_Vtbl;
}
unsafe impl ::windows::core::Interface for IPackageDependency {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x32ae7b95_e358_5a48_9669_c97d85ad6556);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageDependency_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub Delete:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Add: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub Add2: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        options: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPackageDependencyContext(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IPackageDependencyContext {
    type Vtable = IPackageDependencyContext_Vtbl;
}
unsafe impl ::windows::core::Interface for IPackageDependencyContext {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x9902c35a_a3f5_5645_af0f_cdf9fca00d5e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageDependencyContext_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ContextId: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut PackageDependencyContextId,
    ) -> ::windows::core::HRESULT,
    pub PackageDependencyId: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub PackageFullName: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub Remove:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPackageDependencyContextFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IPackageDependencyContextFactory {
    type Vtable = IPackageDependencyContextFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for IPackageDependencyContextFactory {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x9914f24f_bebf_516b_adab_5c3e8bf323f8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageDependencyContextFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        contextid: PackageDependencyContextId,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPackageDependencyRankStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IPackageDependencyRankStatics {
    type Vtable = IPackageDependencyRankStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IPackageDependencyRankStatics {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x260583bd_a4ab_53fd_a190_c446bfdb5384);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageDependencyRankStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Default: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut i32,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPackageDependencyStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IPackageDependencyStatics {
    type Vtable = IPackageDependencyStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IPackageDependencyStatics {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x17b656e1_1a58_5f3c_84a8_4430f6e749c2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageDependencyStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetFromId: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        id: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub GetFromIdForSystem: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        id: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub Create: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        packagefamilyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        minversion: ::windows::ApplicationModel::PackageVersion,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub Create2: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        packagefamilyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        minversion: ::windows::ApplicationModel::PackageVersion,
        options: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub CreateForSystem: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        packagefamilyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        minversion: ::windows::ApplicationModel::PackageVersion,
        options: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub GenerationId: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut u32,
    ) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Windows_ApplicationModel_DynamicDependency\"`*"]
#[repr(transparent)]
pub struct AddPackageDependencyOptions(::windows::core::IUnknown);
impl AddPackageDependencyOptions {
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
            AddPackageDependencyOptions,
            ::windows::core::IGenericFactory,
        > = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Rank(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Rank)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<i32>(result__)
        }
    }
    pub fn SetRank(&self, value: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetRank)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn PrependIfRankCollision(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PrependIfRankCollision)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetPrependIfRankCollision(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetPrependIfRankCollision)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
}
impl ::core::clone::Clone for AddPackageDependencyOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AddPackageDependencyOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AddPackageDependencyOptions {}
impl ::core::fmt::Debug for AddPackageDependencyOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AddPackageDependencyOptions").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AddPackageDependencyOptions {
    const SIGNATURE : ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice ( b"rc(Microsoft.Windows.ApplicationModel.DynamicDependency.AddPackageDependencyOptions;{01b801fd-24e3-5e6b-9f1c-805ab410b604})" ) ;
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for AddPackageDependencyOptions {
    type Vtable = IAddPackageDependencyOptions_Vtbl;
}
unsafe impl ::windows::core::Interface for AddPackageDependencyOptions {
    const IID: ::windows::core::GUID =
        <IAddPackageDependencyOptions as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AddPackageDependencyOptions {
    const NAME: &'static str =
        "Microsoft.Windows.ApplicationModel.DynamicDependency.AddPackageDependencyOptions";
}
::windows::core::interface_hierarchy!(
    AddPackageDependencyOptions,
    ::windows::core::IUnknown,
    ::windows::core::IInspectable
);
unsafe impl ::core::marker::Send for AddPackageDependencyOptions {}
unsafe impl ::core::marker::Sync for AddPackageDependencyOptions {}
#[doc = "*Required features: `\"Windows_ApplicationModel_DynamicDependency\"`*"]
#[repr(transparent)]
pub struct CreatePackageDependencyOptions(::windows::core::IUnknown);
impl CreatePackageDependencyOptions {
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
            CreatePackageDependencyOptions,
            ::windows::core::IGenericFactory,
        > = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Architectures(
        &self,
    ) -> ::windows::core::Result<PackageDependencyProcessorArchitectures> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Architectures)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<PackageDependencyProcessorArchitectures>(result__)
        }
    }
    pub fn SetArchitectures(
        &self,
        value: PackageDependencyProcessorArchitectures,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetArchitectures)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn VerifyDependencyResolution(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).VerifyDependencyResolution)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetVerifyDependencyResolution(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetVerifyDependencyResolution)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn LifetimeArtifactKind(
        &self,
    ) -> ::windows::core::Result<PackageDependencyLifetimeArtifactKind> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LifetimeArtifactKind)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<PackageDependencyLifetimeArtifactKind>(result__)
        }
    }
    pub fn SetLifetimeArtifactKind(
        &self,
        value: PackageDependencyLifetimeArtifactKind,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetLifetimeArtifactKind)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn LifetimeArtifact(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LifetimeArtifact)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetLifetimeArtifact(
        &self,
        value: &::windows::core::HSTRING,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetLifetimeArtifact)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
}
impl ::core::clone::Clone for CreatePackageDependencyOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CreatePackageDependencyOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CreatePackageDependencyOptions {}
impl ::core::fmt::Debug for CreatePackageDependencyOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CreatePackageDependencyOptions").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CreatePackageDependencyOptions {
    const SIGNATURE : ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice ( b"rc(Microsoft.Windows.ApplicationModel.DynamicDependency.CreatePackageDependencyOptions;{cdbb820f-3c69-55dc-a017-b4132574c5d6})" ) ;
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for CreatePackageDependencyOptions {
    type Vtable = ICreatePackageDependencyOptions_Vtbl;
}
unsafe impl ::windows::core::Interface for CreatePackageDependencyOptions {
    const IID: ::windows::core::GUID =
        <ICreatePackageDependencyOptions as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CreatePackageDependencyOptions {
    const NAME: &'static str =
        "Microsoft.Windows.ApplicationModel.DynamicDependency.CreatePackageDependencyOptions";
}
::windows::core::interface_hierarchy!(
    CreatePackageDependencyOptions,
    ::windows::core::IUnknown,
    ::windows::core::IInspectable
);
unsafe impl ::core::marker::Send for CreatePackageDependencyOptions {}
unsafe impl ::core::marker::Sync for CreatePackageDependencyOptions {}
#[doc = "*Required features: `\"Windows_ApplicationModel_DynamicDependency\"`*"]
#[repr(transparent)]
pub struct PackageDependency(::windows::core::IUnknown);
impl PackageDependency {
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Id)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn Delete(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).Delete)(::windows::core::Vtable::as_raw(this))
                .ok()
        }
    }
    pub fn Add(&self) -> ::windows::core::Result<PackageDependencyContext> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Add)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<PackageDependencyContext>(result__)
        }
    }
    pub fn Add2(
        &self,
        options: &AddPackageDependencyOptions,
    ) -> ::windows::core::Result<PackageDependencyContext> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Add2)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(options),
                result__.as_mut_ptr(),
            )
            .from_abi::<PackageDependencyContext>(result__)
        }
    }
    pub fn GetFromId(id: &::windows::core::HSTRING) -> ::windows::core::Result<PackageDependency> {
        Self::IPackageDependencyStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetFromId)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(id),
                result__.as_mut_ptr(),
            )
            .from_abi::<PackageDependency>(result__)
        })
    }
    pub fn GetFromIdForSystem(
        id: &::windows::core::HSTRING,
    ) -> ::windows::core::Result<PackageDependency> {
        Self::IPackageDependencyStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetFromIdForSystem)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(id),
                result__.as_mut_ptr(),
            )
            .from_abi::<PackageDependency>(result__)
        })
    }
    pub fn Create(
        packagefamilyname: &::windows::core::HSTRING,
        minversion: ::windows::ApplicationModel::PackageVersion,
    ) -> ::windows::core::Result<PackageDependency> {
        Self::IPackageDependencyStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Create)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(packagefamilyname),
                minversion,
                result__.as_mut_ptr(),
            )
            .from_abi::<PackageDependency>(result__)
        })
    }
    pub fn Create2(
        packagefamilyname: &::windows::core::HSTRING,
        minversion: ::windows::ApplicationModel::PackageVersion,
        options: &CreatePackageDependencyOptions,
    ) -> ::windows::core::Result<PackageDependency> {
        Self::IPackageDependencyStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Create2)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(packagefamilyname),
                minversion,
                ::core::mem::transmute_copy(options),
                result__.as_mut_ptr(),
            )
            .from_abi::<PackageDependency>(result__)
        })
    }
    pub fn CreateForSystem(
        packagefamilyname: &::windows::core::HSTRING,
        minversion: ::windows::ApplicationModel::PackageVersion,
        options: &CreatePackageDependencyOptions,
    ) -> ::windows::core::Result<PackageDependency> {
        Self::IPackageDependencyStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateForSystem)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(packagefamilyname),
                minversion,
                ::core::mem::transmute_copy(options),
                result__.as_mut_ptr(),
            )
            .from_abi::<PackageDependency>(result__)
        })
    }
    pub fn GenerationId() -> ::windows::core::Result<u32> {
        Self::IPackageDependencyStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GenerationId)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<u32>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPackageDependencyStatics<
        R,
        F: FnOnce(&IPackageDependencyStatics) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<PackageDependency, IPackageDependencyStatics> =
            ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for PackageDependency {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PackageDependency {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PackageDependency {}
impl ::core::fmt::Debug for PackageDependency {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PackageDependency").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PackageDependency {
    const SIGNATURE : ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice ( b"rc(Microsoft.Windows.ApplicationModel.DynamicDependency.PackageDependency;{32ae7b95-e358-5a48-9669-c97d85ad6556})" ) ;
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for PackageDependency {
    type Vtable = IPackageDependency_Vtbl;
}
unsafe impl ::windows::core::Interface for PackageDependency {
    const IID: ::windows::core::GUID = <IPackageDependency as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PackageDependency {
    const NAME: &'static str =
        "Microsoft.Windows.ApplicationModel.DynamicDependency.PackageDependency";
}
::windows::core::interface_hierarchy!(
    PackageDependency,
    ::windows::core::IUnknown,
    ::windows::core::IInspectable
);
unsafe impl ::core::marker::Send for PackageDependency {}
unsafe impl ::core::marker::Sync for PackageDependency {}
#[doc = "*Required features: `\"Windows_ApplicationModel_DynamicDependency\"`*"]
#[repr(transparent)]
pub struct PackageDependencyContext(::windows::core::IUnknown);
impl PackageDependencyContext {
    pub fn ContextId(&self) -> ::windows::core::Result<PackageDependencyContextId> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ContextId)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<PackageDependencyContextId>(result__)
        }
    }
    pub fn PackageDependencyId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PackageDependencyId)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn PackageFullName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PackageFullName)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn Remove(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).Remove)(::windows::core::Vtable::as_raw(this))
                .ok()
        }
    }
    pub fn CreateInstance(
        contextid: PackageDependencyContextId,
    ) -> ::windows::core::Result<PackageDependencyContext> {
        Self::IPackageDependencyContextFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateInstance)(
                ::windows::core::Vtable::as_raw(this),
                contextid,
                result__.as_mut_ptr(),
            )
            .from_abi::<PackageDependencyContext>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPackageDependencyContextFactory<
        R,
        F: FnOnce(&IPackageDependencyContextFactory) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<
            PackageDependencyContext,
            IPackageDependencyContextFactory,
        > = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for PackageDependencyContext {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PackageDependencyContext {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PackageDependencyContext {}
impl ::core::fmt::Debug for PackageDependencyContext {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PackageDependencyContext").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PackageDependencyContext {
    const SIGNATURE : ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice ( b"rc(Microsoft.Windows.ApplicationModel.DynamicDependency.PackageDependencyContext;{9902c35a-a3f5-5645-af0f-cdf9fca00d5e})" ) ;
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for PackageDependencyContext {
    type Vtable = IPackageDependencyContext_Vtbl;
}
unsafe impl ::windows::core::Interface for PackageDependencyContext {
    const IID: ::windows::core::GUID =
        <IPackageDependencyContext as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PackageDependencyContext {
    const NAME: &'static str =
        "Microsoft.Windows.ApplicationModel.DynamicDependency.PackageDependencyContext";
}
::windows::core::interface_hierarchy!(
    PackageDependencyContext,
    ::windows::core::IUnknown,
    ::windows::core::IInspectable
);
unsafe impl ::core::marker::Send for PackageDependencyContext {}
unsafe impl ::core::marker::Sync for PackageDependencyContext {}
#[doc = "*Required features: `\"Windows_ApplicationModel_DynamicDependency\"`*"]
pub struct PackageDependencyRank;
impl PackageDependencyRank {
    pub fn Default() -> ::windows::core::Result<i32> {
        Self::IPackageDependencyRankStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Default)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<i32>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPackageDependencyRankStatics<
        R,
        F: FnOnce(&IPackageDependencyRankStatics) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<
            PackageDependencyRank,
            IPackageDependencyRankStatics,
        > = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for PackageDependencyRank {
    const NAME: &'static str =
        "Microsoft.Windows.ApplicationModel.DynamicDependency.PackageDependencyRank";
}
#[doc = "*Required features: `\"Windows_ApplicationModel_DynamicDependency\"`*"]
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
unsafe impl ::windows::core::Abi for PackageDependencyLifetimeArtifactKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for PackageDependencyLifetimeArtifactKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PackageDependencyLifetimeArtifactKind").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PackageDependencyLifetimeArtifactKind {
    const SIGNATURE : ::windows::core::ConstBuffer =::windows::core::ConstBuffer::from_slice ( b"enum(Microsoft.Windows.ApplicationModel.DynamicDependency.PackageDependencyLifetimeArtifactKind;i4)" ) ;
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Windows_ApplicationModel_DynamicDependency\"`*"]
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
unsafe impl ::windows::core::Abi for PackageDependencyProcessorArchitectures {
    type Abi = Self;
}
impl ::core::fmt::Debug for PackageDependencyProcessorArchitectures {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PackageDependencyProcessorArchitectures").field(&self.0).finish()
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
unsafe impl ::windows::core::RuntimeType for PackageDependencyProcessorArchitectures {
    const SIGNATURE : ::windows::core::ConstBuffer =::windows::core::ConstBuffer::from_slice ( b"enum(Microsoft.Windows.ApplicationModel.DynamicDependency.PackageDependencyProcessorArchitectures;u4)" ) ;
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Windows_ApplicationModel_DynamicDependency\"`*"]
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
unsafe impl ::windows::core::Abi for PackageDependencyContextId {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for PackageDependencyContextId {
    const SIGNATURE : ::windows::core::ConstBuffer =::windows::core::ConstBuffer::from_slice ( b"struct(Microsoft.Windows.ApplicationModel.DynamicDependency.PackageDependencyContextId;u8)" ) ;
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
impl ::core::cmp::PartialEq for PackageDependencyContextId {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            ::windows::core::memcmp(
                self as *const _ as _,
                other as *const _ as _,
                core::mem::size_of::<PackageDependencyContextId>(),
            ) == 0
        }
    }
}
impl ::core::cmp::Eq for PackageDependencyContextId {}
impl ::core::default::Default for PackageDependencyContextId {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
