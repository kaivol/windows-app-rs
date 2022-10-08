#[cfg(feature = "Windows_System_Power")]
pub mod Power;
#[doc(hidden)]
#[repr(transparent)]
pub struct IEnvironmentManager(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IEnvironmentManager {
    type Vtable = IEnvironmentManager_Vtbl;
}
unsafe impl ::windows::core::Interface for IEnvironmentManager {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xd1b239bb_7013_5176_b02a_63477410d986);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnvironmentManager_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetEnvironmentVariables: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub GetEnvironmentVariable: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub SetEnvironmentVariable: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub AppendToPath: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        path: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub RemoveFromPath: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        path: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub AddExecutableFileExtension: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        pathext: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub RemoveExecutableFileExtension: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        pathext: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEnvironmentManagerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IEnvironmentManagerStatics {
    type Vtable = IEnvironmentManagerStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IEnvironmentManagerStatics {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x407b1522_6156_5398_93fd_d6411c35e7b1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnvironmentManagerStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetForProcess: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub GetForUser: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub GetForMachine: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub IsSupported: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Windows_System\"`*"]
#[repr(transparent)]
pub struct EnvironmentManager(::windows::core::IUnknown);
impl EnvironmentManager {
    pub fn GetEnvironmentVariables(
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
            (::windows::core::Vtable::vtable(this).GetEnvironmentVariables)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::Collections::IMapView<
                ::windows::core::HSTRING,
                ::windows::core::HSTRING,
            >>(result__)
        }
    }
    pub fn GetEnvironmentVariable(
        &self,
        name: &::windows::core::HSTRING,
    ) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetEnvironmentVariable)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(name),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetEnvironmentVariable(
        &self,
        name: &::windows::core::HSTRING,
        value: &::windows::core::HSTRING,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetEnvironmentVariable)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(name),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn AppendToPath(&self, path: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).AppendToPath)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(path),
            )
            .ok()
        }
    }
    pub fn RemoveFromPath(&self, path: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).RemoveFromPath)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(path),
            )
            .ok()
        }
    }
    pub fn AddExecutableFileExtension(
        &self,
        pathext: &::windows::core::HSTRING,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).AddExecutableFileExtension)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(pathext),
            )
            .ok()
        }
    }
    pub fn RemoveExecutableFileExtension(
        &self,
        pathext: &::windows::core::HSTRING,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).RemoveExecutableFileExtension)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(pathext),
            )
            .ok()
        }
    }
    pub fn GetForProcess() -> ::windows::core::Result<EnvironmentManager> {
        Self::IEnvironmentManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetForProcess)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<EnvironmentManager>(result__)
        })
    }
    pub fn GetForUser() -> ::windows::core::Result<EnvironmentManager> {
        Self::IEnvironmentManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetForUser)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<EnvironmentManager>(result__)
        })
    }
    pub fn GetForMachine() -> ::windows::core::Result<EnvironmentManager> {
        Self::IEnvironmentManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetForMachine)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<EnvironmentManager>(result__)
        })
    }
    pub fn IsSupported() -> ::windows::core::Result<bool> {
        Self::IEnvironmentManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsSupported)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IEnvironmentManagerStatics<
        R,
        F: FnOnce(&IEnvironmentManagerStatics) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<
            EnvironmentManager,
            IEnvironmentManagerStatics,
        > = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for EnvironmentManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for EnvironmentManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EnvironmentManager {}
impl ::core::fmt::Debug for EnvironmentManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EnvironmentManager").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for EnvironmentManager {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.Windows.System.EnvironmentManager;{d1b239bb-7013-5176-b02a-63477410d986})",
    );
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for EnvironmentManager {
    type Vtable = IEnvironmentManager_Vtbl;
}
unsafe impl ::windows::core::Interface for EnvironmentManager {
    const IID: ::windows::core::GUID = <IEnvironmentManager as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for EnvironmentManager {
    const NAME: &'static str = "Microsoft.Windows.System.EnvironmentManager";
}
::windows::core::interface_hierarchy!(
    EnvironmentManager,
    ::windows::core::IUnknown,
    ::windows::core::IInspectable
);
unsafe impl ::core::marker::Send for EnvironmentManager {}
unsafe impl ::core::marker::Sync for EnvironmentManager {}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
