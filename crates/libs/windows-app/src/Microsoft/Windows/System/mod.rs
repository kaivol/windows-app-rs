#[cfg(feature = "Microsoft_Windows_System_Power")]
#[doc = "Required features: `\"Microsoft_Windows_System_Power\"`"]
pub mod Power;
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IEnvironmentManager(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IEnvironmentManager {
    type Vtable = IEnvironmentManager_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IEnvironmentManager {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xd1b239bb_7013_5176_b02a_63477410d986);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnvironmentManager_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub GetEnvironmentVariables: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation_Collections"))]
    GetEnvironmentVariables: usize,
    pub GetEnvironmentVariable: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        name: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub SetEnvironmentVariable: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        name: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        value: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub AppendToPath: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        path: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub RemoveFromPath: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        path: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub AddExecutableFileExtension: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        pathext: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub RemoveExecutableFileExtension: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        pathext: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IEnvironmentManager2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IEnvironmentManager2 {
    type Vtable = IEnvironmentManager2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IEnvironmentManager2 {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xcfc0ad51_02b7_57ff_8ca7_e015251737cb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnvironmentManager2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub AreChangesTracked: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IEnvironmentManagerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IEnvironmentManagerStatics {
    type Vtable = IEnvironmentManagerStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IEnvironmentManagerStatics {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x407b1522_6156_5398_93fd_d6411c35e7b1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnvironmentManagerStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub GetForProcess: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub GetForUser: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub GetForMachine: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub IsSupported: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct EnvironmentManager(::windows_core::IUnknown);
impl EnvironmentManager {
    #[doc = "Required features: `\"Windows_Foundation_Collections\"`"]
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub fn GetEnvironmentVariables(
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
            (::windows_core::Interface::vtable(this).GetEnvironmentVariables)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn GetEnvironmentVariable(
        &self,
        name: &::windows_core::HSTRING,
    ) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetEnvironmentVariable)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(name),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetEnvironmentVariable(
        &self,
        name: &::windows_core::HSTRING,
        value: &::windows_core::HSTRING,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetEnvironmentVariable)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(name),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn AppendToPath(&self, path: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).AppendToPath)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(path),
            )
            .ok()
        }
    }
    pub fn RemoveFromPath(&self, path: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveFromPath)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(path),
            )
            .ok()
        }
    }
    pub fn AddExecutableFileExtension(
        &self,
        pathext: &::windows_core::HSTRING,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).AddExecutableFileExtension)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(pathext),
            )
            .ok()
        }
    }
    pub fn RemoveExecutableFileExtension(
        &self,
        pathext: &::windows_core::HSTRING,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveExecutableFileExtension)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(pathext),
            )
            .ok()
        }
    }
    pub fn AreChangesTracked(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IEnvironmentManager2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AreChangesTracked)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn GetForProcess() -> ::windows_core::Result<EnvironmentManager> {
        Self::IEnvironmentManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetForProcess)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn GetForUser() -> ::windows_core::Result<EnvironmentManager> {
        Self::IEnvironmentManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetForUser)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn GetForMachine() -> ::windows_core::Result<EnvironmentManager> {
        Self::IEnvironmentManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetForMachine)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn IsSupported() -> ::windows_core::Result<bool> {
        Self::IEnvironmentManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsSupported)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IEnvironmentManagerStatics<
        R,
        F: FnOnce(&IEnvironmentManagerStatics) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<
            EnvironmentManager,
            IEnvironmentManagerStatics,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for EnvironmentManager {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for EnvironmentManager {
    type Vtable = IEnvironmentManager_Vtbl;
}
unsafe impl ::windows_core::ComInterface for EnvironmentManager {
    const IID: ::windows_core::GUID = <IEnvironmentManager as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for EnvironmentManager {
    const NAME: &'static str = "Microsoft.Windows.System.EnvironmentManager";
}
::windows_core::imp::interface_hierarchy!(
    EnvironmentManager,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for EnvironmentManager {}
unsafe impl ::core::marker::Sync for EnvironmentManager {}
