#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IActivationRegistrationManagerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IActivationRegistrationManagerStatics {
    type Vtable = IActivationRegistrationManagerStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IActivationRegistrationManagerStatics {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x5ac4e92e_017b_5d68_8198_f68636ab99d3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IActivationRegistrationManagerStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub RegisterForFileTypeActivation: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        supportedFileTypes_array_size: u32,
        supportedfiletypes: *const ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        logo: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        displayname: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        supportedVerbs_array_size: u32,
        supportedverbs: *const ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        exepath: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub RegisterForProtocolActivation: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        scheme: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        logo: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        displayname: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        exepath: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub RegisterForStartupActivation: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        taskid: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        exepath: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub UnregisterForFileTypeActivation: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        fileTypes_array_size: u32,
        filetypes: *const ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        exepath: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub UnregisterForProtocolActivation: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        scheme: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        exepath: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub UnregisterForStartupActivation: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        taskid: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IAppActivationArguments(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppActivationArguments {
    type Vtable = IAppActivationArguments_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IAppActivationArguments {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x14f99eaf_1580_5062_bdc8_d5d1c31138fb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppActivationArguments_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Kind: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ExtendedActivationKind,
    ) -> ::windows_core::HRESULT,
    pub Data: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IAppInstance(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppInstance {
    type Vtable = IAppInstance_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IAppInstance {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x75766ae4_0239_5a26_b9da_d5bfc75a4866);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppInstance_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub UnregisterKey:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_Foundation")]
    pub RedirectActivationToAsync: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        args: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RedirectActivationToAsync: usize,
    pub GetActivatedEventArgs: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_Foundation")]
    pub Activated: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    Activated: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub RemoveActivated: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RemoveActivated: usize,
    pub Key: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub IsCurrent: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub ProcessId: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut u32,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IAppInstanceStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppInstanceStatics {
    type Vtable = IAppInstanceStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IAppInstanceStatics {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x4f414b25_8330_5a9b_bbc1_8229d479649d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppInstanceStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub GetCurrent: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub GetInstances: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation_Collections"))]
    GetInstances: usize,
    pub FindOrRegisterForKey: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        key: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IAppInstanceStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppInstanceStatics2 {
    type Vtable = IAppInstanceStatics2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IAppInstanceStatics2 {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xfe9f1885_7160_5397_ba9b_5890b24fdc04);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppInstanceStatics2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Windows_ApplicationModel_Core")]
    pub Restart: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        arguments: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        result__: *mut ::windows::ApplicationModel::Core::AppRestartFailureReason,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_ApplicationModel_Core"))]
    Restart: usize,
}
pub struct ActivationRegistrationManager;
impl ActivationRegistrationManager {
    pub fn RegisterForFileTypeActivation(
        supportedfiletypes: &[::windows_core::HSTRING],
        logo: &::windows_core::HSTRING,
        displayname: &::windows_core::HSTRING,
        supportedverbs: &[::windows_core::HSTRING],
        exepath: &::windows_core::HSTRING,
    ) -> ::windows_core::Result<()> {
        Self::IActivationRegistrationManagerStatics(|this| unsafe {
            (::windows_core::Interface::vtable(this).RegisterForFileTypeActivation)(
                ::windows_core::Interface::as_raw(this),
                supportedfiletypes.len().try_into().unwrap(),
                ::core::mem::transmute(supportedfiletypes.as_ptr()),
                ::core::mem::transmute_copy(logo),
                ::core::mem::transmute_copy(displayname),
                supportedverbs.len().try_into().unwrap(),
                ::core::mem::transmute(supportedverbs.as_ptr()),
                ::core::mem::transmute_copy(exepath),
            )
            .ok()
        })
    }
    pub fn RegisterForProtocolActivation(
        scheme: &::windows_core::HSTRING,
        logo: &::windows_core::HSTRING,
        displayname: &::windows_core::HSTRING,
        exepath: &::windows_core::HSTRING,
    ) -> ::windows_core::Result<()> {
        Self::IActivationRegistrationManagerStatics(|this| unsafe {
            (::windows_core::Interface::vtable(this).RegisterForProtocolActivation)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(scheme),
                ::core::mem::transmute_copy(logo),
                ::core::mem::transmute_copy(displayname),
                ::core::mem::transmute_copy(exepath),
            )
            .ok()
        })
    }
    pub fn RegisterForStartupActivation(
        taskid: &::windows_core::HSTRING,
        exepath: &::windows_core::HSTRING,
    ) -> ::windows_core::Result<()> {
        Self::IActivationRegistrationManagerStatics(|this| unsafe {
            (::windows_core::Interface::vtable(this).RegisterForStartupActivation)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(taskid),
                ::core::mem::transmute_copy(exepath),
            )
            .ok()
        })
    }
    pub fn UnregisterForFileTypeActivation(
        filetypes: &[::windows_core::HSTRING],
        exepath: &::windows_core::HSTRING,
    ) -> ::windows_core::Result<()> {
        Self::IActivationRegistrationManagerStatics(|this| unsafe {
            (::windows_core::Interface::vtable(this).UnregisterForFileTypeActivation)(
                ::windows_core::Interface::as_raw(this),
                filetypes.len().try_into().unwrap(),
                ::core::mem::transmute(filetypes.as_ptr()),
                ::core::mem::transmute_copy(exepath),
            )
            .ok()
        })
    }
    pub fn UnregisterForProtocolActivation(
        scheme: &::windows_core::HSTRING,
        exepath: &::windows_core::HSTRING,
    ) -> ::windows_core::Result<()> {
        Self::IActivationRegistrationManagerStatics(|this| unsafe {
            (::windows_core::Interface::vtable(this).UnregisterForProtocolActivation)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(scheme),
                ::core::mem::transmute_copy(exepath),
            )
            .ok()
        })
    }
    pub fn UnregisterForStartupActivation(
        taskid: &::windows_core::HSTRING,
    ) -> ::windows_core::Result<()> {
        Self::IActivationRegistrationManagerStatics(|this| unsafe {
            (::windows_core::Interface::vtable(this).UnregisterForStartupActivation)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(taskid),
            )
            .ok()
        })
    }
    #[doc(hidden)]
    pub fn IActivationRegistrationManagerStatics<
        R,
        F: FnOnce(&IActivationRegistrationManagerStatics) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<
            ActivationRegistrationManager,
            IActivationRegistrationManagerStatics,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeName for ActivationRegistrationManager {
    const NAME: &'static str = "Microsoft.Windows.AppLifecycle.ActivationRegistrationManager";
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct AppActivationArguments(::windows_core::IUnknown);
impl AppActivationArguments {
    pub fn Kind(&self) -> ::windows_core::Result<ExtendedActivationKind> {
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
    pub fn Data(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Data)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for AppActivationArguments {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for AppActivationArguments {
    type Vtable = IAppActivationArguments_Vtbl;
}
unsafe impl ::windows_core::ComInterface for AppActivationArguments {
    const IID: ::windows_core::GUID =
        <IAppActivationArguments as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for AppActivationArguments {
    const NAME: &'static str = "Microsoft.Windows.AppLifecycle.AppActivationArguments";
}
::windows_core::imp::interface_hierarchy!(
    AppActivationArguments,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for AppActivationArguments {}
unsafe impl ::core::marker::Sync for AppActivationArguments {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct AppInstance(::windows_core::IUnknown);
impl AppInstance {
    pub fn UnregisterKey(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).UnregisterKey)(
                ::windows_core::Interface::as_raw(this),
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RedirectActivationToAsync<P0>(
        &self,
        args: P0,
    ) -> ::windows_core::Result<::windows::Foundation::IAsyncAction>
    where
        P0: ::windows_core::IntoParam<AppActivationArguments>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RedirectActivationToAsync)(
                ::windows_core::Interface::as_raw(this),
                args.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn GetActivatedEventArgs(&self) -> ::windows_core::Result<AppActivationArguments> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetActivatedEventArgs)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn Activated<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<::windows::Foundation::EventHandler<AppActivationArguments>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Activated)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveActivated(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveActivated)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn Key(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Key)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn IsCurrent(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsCurrent)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn ProcessId(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ProcessId)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn GetCurrent() -> ::windows_core::Result<AppInstance> {
        Self::IAppInstanceStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetCurrent)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_Foundation_Collections\"`"]
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub fn GetInstances(
    ) -> ::windows_core::Result<::windows::Foundation::Collections::IVector<AppInstance>> {
        Self::IAppInstanceStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetInstances)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn FindOrRegisterForKey(
        key: &::windows_core::HSTRING,
    ) -> ::windows_core::Result<AppInstance> {
        Self::IAppInstanceStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FindOrRegisterForKey)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(key),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_ApplicationModel_Core\"`"]
    #[cfg(feature = "Windows_ApplicationModel_Core")]
    pub fn Restart(
        arguments: &::windows_core::HSTRING,
    ) -> ::windows_core::Result<::windows::ApplicationModel::Core::AppRestartFailureReason> {
        Self::IAppInstanceStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Restart)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(arguments),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IAppInstanceStatics<R, F: FnOnce(&IAppInstanceStatics) -> ::windows_core::Result<R>>(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<AppInstance, IAppInstanceStatics> =
            ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IAppInstanceStatics2<
        R,
        F: FnOnce(&IAppInstanceStatics2) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<AppInstance, IAppInstanceStatics2> =
            ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for AppInstance {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for AppInstance {
    type Vtable = IAppInstance_Vtbl;
}
unsafe impl ::windows_core::ComInterface for AppInstance {
    const IID: ::windows_core::GUID = <IAppInstance as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for AppInstance {
    const NAME: &'static str = "Microsoft.Windows.AppLifecycle.AppInstance";
}
::windows_core::imp::interface_hierarchy!(
    AppInstance,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for AppInstance {}
unsafe impl ::core::marker::Sync for AppInstance {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ExtendedActivationKind(pub i32);
impl ExtendedActivationKind {
    pub const Launch: Self = Self(0i32);
    pub const Search: Self = Self(1i32);
    pub const ShareTarget: Self = Self(2i32);
    pub const File: Self = Self(3i32);
    pub const Protocol: Self = Self(4i32);
    pub const FileOpenPicker: Self = Self(5i32);
    pub const FileSavePicker: Self = Self(6i32);
    pub const CachedFileUpdater: Self = Self(7i32);
    pub const ContactPicker: Self = Self(8i32);
    pub const Device: Self = Self(9i32);
    pub const PrintTaskSettings: Self = Self(10i32);
    pub const CameraSettings: Self = Self(11i32);
    pub const RestrictedLaunch: Self = Self(12i32);
    pub const AppointmentsProvider: Self = Self(13i32);
    pub const Contact: Self = Self(14i32);
    pub const LockScreenCall: Self = Self(15i32);
    pub const VoiceCommand: Self = Self(16i32);
    pub const LockScreen: Self = Self(17i32);
    pub const PickerReturned: Self = Self(1000i32);
    pub const WalletAction: Self = Self(1001i32);
    pub const PickFileContinuation: Self = Self(1002i32);
    pub const PickSaveFileContinuation: Self = Self(1003i32);
    pub const PickFolderContinuation: Self = Self(1004i32);
    pub const WebAuthenticationBrokerContinuation: Self = Self(1005i32);
    pub const WebAccountProvider: Self = Self(1006i32);
    pub const ComponentUI: Self = Self(1007i32);
    pub const ProtocolForResults: Self = Self(1009i32);
    pub const ToastNotification: Self = Self(1010i32);
    pub const Print3DWorkflow: Self = Self(1011i32);
    pub const DialReceiver: Self = Self(1012i32);
    pub const DevicePairing: Self = Self(1013i32);
    pub const UserDataAccountsProvider: Self = Self(1014i32);
    pub const FilePickerExperience: Self = Self(1015i32);
    pub const LockScreenComponent: Self = Self(1016i32);
    pub const ContactPanel: Self = Self(1017i32);
    pub const PrintWorkflowForegroundTask: Self = Self(1018i32);
    pub const GameUIProvider: Self = Self(1019i32);
    pub const StartupTask: Self = Self(1020i32);
    pub const CommandLineLaunch: Self = Self(1021i32);
    pub const BarcodeScannerProvider: Self = Self(1022i32);
    pub const PrintSupportJobUI: Self = Self(1023i32);
    pub const PrintSupportSettingsUI: Self = Self(1024i32);
    pub const PhoneCallActivation: Self = Self(1025i32);
    pub const VpnForeground: Self = Self(1026i32);
    pub const Push: Self = Self(5000i32);
    pub const AppNotification: Self = Self(5001i32);
}
impl ::core::marker::Copy for ExtendedActivationKind {}
impl ::core::clone::Clone for ExtendedActivationKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ExtendedActivationKind {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for ExtendedActivationKind {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for ExtendedActivationKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ExtendedActivationKind").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ExtendedActivationKind {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::from_slice(
            b"enum(Microsoft.Windows.AppLifecycle.ExtendedActivationKind;i4)",
        );
}
