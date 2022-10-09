#[doc(hidden)]
#[repr(transparent)]
pub struct IActivationRegistrationManagerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IActivationRegistrationManagerStatics {
    type Vtable = IActivationRegistrationManagerStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IActivationRegistrationManagerStatics {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x5ac4e92e_017b_5d68_8198_f68636ab99d3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IActivationRegistrationManagerStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub RegisterForFileTypeActivation: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        supportedFileTypes_array_size: u32,
        supportedfiletypes: *const ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        logo: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        displayname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        supportedVerbs_array_size: u32,
        supportedverbs: *const ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        exepath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub RegisterForProtocolActivation: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        scheme: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        logo: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        displayname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        exepath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub RegisterForStartupActivation: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        taskid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        exepath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub UnregisterForFileTypeActivation: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        fileTypes_array_size: u32,
        filetypes: *const ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        exepath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub UnregisterForProtocolActivation: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        scheme: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        exepath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub UnregisterForStartupActivation: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        taskid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppActivationArguments(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IAppActivationArguments {
    type Vtable = IAppActivationArguments_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppActivationArguments {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x14f99eaf_1580_5062_bdc8_d5d1c31138fb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppActivationArguments_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Kind: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ExtendedActivationKind,
    ) -> ::windows::core::HRESULT,
    pub Data: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppInstance(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IAppInstance {
    type Vtable = IAppInstance_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppInstance {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x75766ae4_0239_5a26_b9da_d5bfc75a4866);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppInstance_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub UnregisterKey:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RedirectActivationToAsync: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        args: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub GetActivatedEventArgs: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub Activated: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub RemoveActivated: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub Key: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub IsCurrent: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub ProcessId: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut u32,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppInstanceStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IAppInstanceStatics {
    type Vtable = IAppInstanceStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppInstanceStatics {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x4f414b25_8330_5a9b_bbc1_8229d479649d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppInstanceStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetCurrent: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub GetInstances: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub FindOrRegisterForKey: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        key: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppInstanceStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IAppInstanceStatics2 {
    type Vtable = IAppInstanceStatics2_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppInstanceStatics2 {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xfe9f1885_7160_5397_ba9b_5890b24fdc04);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppInstanceStatics2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Restart: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        arguments: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        result__: *mut ::windows::ApplicationModel::Core::AppRestartFailureReason,
    ) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Windows_AppLifecycle\"`*"]
pub struct ActivationRegistrationManager;
impl ActivationRegistrationManager {
    pub fn RegisterForFileTypeActivation(
        supportedfiletypes: &[::windows::core::HSTRING],
        logo: &::windows::core::HSTRING,
        displayname: &::windows::core::HSTRING,
        supportedverbs: &[::windows::core::HSTRING],
        exepath: &::windows::core::HSTRING,
    ) -> ::windows::core::Result<()> {
        Self::IActivationRegistrationManagerStatics(|this| unsafe {
            (::windows::core::Vtable::vtable(this).RegisterForFileTypeActivation)(
                ::windows::core::Vtable::as_raw(this),
                supportedfiletypes.len() as u32,
                ::core::mem::transmute(supportedfiletypes.as_ptr()),
                ::core::mem::transmute_copy(logo),
                ::core::mem::transmute_copy(displayname),
                supportedverbs.len() as u32,
                ::core::mem::transmute(supportedverbs.as_ptr()),
                ::core::mem::transmute_copy(exepath),
            )
            .ok()
        })
    }
    pub fn RegisterForProtocolActivation(
        scheme: &::windows::core::HSTRING,
        logo: &::windows::core::HSTRING,
        displayname: &::windows::core::HSTRING,
        exepath: &::windows::core::HSTRING,
    ) -> ::windows::core::Result<()> {
        Self::IActivationRegistrationManagerStatics(|this| unsafe {
            (::windows::core::Vtable::vtable(this).RegisterForProtocolActivation)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(scheme),
                ::core::mem::transmute_copy(logo),
                ::core::mem::transmute_copy(displayname),
                ::core::mem::transmute_copy(exepath),
            )
            .ok()
        })
    }
    pub fn RegisterForStartupActivation(
        taskid: &::windows::core::HSTRING,
        exepath: &::windows::core::HSTRING,
    ) -> ::windows::core::Result<()> {
        Self::IActivationRegistrationManagerStatics(|this| unsafe {
            (::windows::core::Vtable::vtable(this).RegisterForStartupActivation)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(taskid),
                ::core::mem::transmute_copy(exepath),
            )
            .ok()
        })
    }
    pub fn UnregisterForFileTypeActivation(
        filetypes: &[::windows::core::HSTRING],
        exepath: &::windows::core::HSTRING,
    ) -> ::windows::core::Result<()> {
        Self::IActivationRegistrationManagerStatics(|this| unsafe {
            (::windows::core::Vtable::vtable(this).UnregisterForFileTypeActivation)(
                ::windows::core::Vtable::as_raw(this),
                filetypes.len() as u32,
                ::core::mem::transmute(filetypes.as_ptr()),
                ::core::mem::transmute_copy(exepath),
            )
            .ok()
        })
    }
    pub fn UnregisterForProtocolActivation(
        scheme: &::windows::core::HSTRING,
        exepath: &::windows::core::HSTRING,
    ) -> ::windows::core::Result<()> {
        Self::IActivationRegistrationManagerStatics(|this| unsafe {
            (::windows::core::Vtable::vtable(this).UnregisterForProtocolActivation)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(scheme),
                ::core::mem::transmute_copy(exepath),
            )
            .ok()
        })
    }
    pub fn UnregisterForStartupActivation(
        taskid: &::windows::core::HSTRING,
    ) -> ::windows::core::Result<()> {
        Self::IActivationRegistrationManagerStatics(|this| unsafe {
            (::windows::core::Vtable::vtable(this).UnregisterForStartupActivation)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(taskid),
            )
            .ok()
        })
    }
    #[doc(hidden)]
    pub fn IActivationRegistrationManagerStatics<
        R,
        F: FnOnce(&IActivationRegistrationManagerStatics) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<
            ActivationRegistrationManager,
            IActivationRegistrationManagerStatics,
        > = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for ActivationRegistrationManager {
    const NAME: &'static str = "Microsoft.Windows.AppLifecycle.ActivationRegistrationManager";
}
#[doc = "*Required features: `\"Windows_AppLifecycle\"`*"]
#[repr(transparent)]
pub struct AppActivationArguments(::windows::core::IUnknown);
impl AppActivationArguments {
    pub fn Kind(&self) -> ::windows::core::Result<ExtendedActivationKind> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Kind)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<ExtendedActivationKind>(result__)
        }
    }
    pub fn Data(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Data)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
}
impl ::core::clone::Clone for AppActivationArguments {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppActivationArguments {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppActivationArguments {}
impl ::core::fmt::Debug for AppActivationArguments {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppActivationArguments").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppActivationArguments {
    const SIGNATURE : ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice ( b"rc(Microsoft.Windows.AppLifecycle.AppActivationArguments;{14f99eaf-1580-5062-bdc8-d5d1c31138fb})" ) ;
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for AppActivationArguments {
    type Vtable = IAppActivationArguments_Vtbl;
}
unsafe impl ::windows::core::Interface for AppActivationArguments {
    const IID: ::windows::core::GUID = <IAppActivationArguments as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppActivationArguments {
    const NAME: &'static str = "Microsoft.Windows.AppLifecycle.AppActivationArguments";
}
::windows::core::interface_hierarchy!(
    AppActivationArguments,
    ::windows::core::IUnknown,
    ::windows::core::IInspectable
);
unsafe impl ::core::marker::Send for AppActivationArguments {}
unsafe impl ::core::marker::Sync for AppActivationArguments {}
#[doc = "*Required features: `\"Windows_AppLifecycle\"`*"]
#[repr(transparent)]
pub struct AppInstance(::windows::core::IUnknown);
impl AppInstance {
    pub fn UnregisterKey(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).UnregisterKey)(::windows::core::Vtable::as_raw(
                this,
            ))
            .ok()
        }
    }
    pub fn RedirectActivationToAsync(
        &self,
        args: &AppActivationArguments,
    ) -> ::windows::core::Result<::windows::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RedirectActivationToAsync)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(args),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::IAsyncAction>(result__)
        }
    }
    pub fn GetActivatedEventArgs(&self) -> ::windows::core::Result<AppActivationArguments> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetActivatedEventArgs)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<AppActivationArguments>(result__)
        }
    }
    pub fn Activated(
        &self,
        handler: &::windows::Foundation::EventHandler<AppActivationArguments>,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Activated)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(handler),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveActivated(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).RemoveActivated)(
                ::windows::core::Vtable::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn Key(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Key)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn IsCurrent(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsCurrent)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn ProcessId(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ProcessId)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<u32>(result__)
        }
    }
    pub fn GetCurrent() -> ::windows::core::Result<AppInstance> {
        Self::IAppInstanceStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetCurrent)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<AppInstance>(result__)
        })
    }
    pub fn GetInstances(
    ) -> ::windows::core::Result<::windows::Foundation::Collections::IVector<AppInstance>> {
        Self::IAppInstanceStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetInstances)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::Collections::IVector<AppInstance>>(result__)
        })
    }
    pub fn FindOrRegisterForKey(
        key: &::windows::core::HSTRING,
    ) -> ::windows::core::Result<AppInstance> {
        Self::IAppInstanceStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FindOrRegisterForKey)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(key),
                result__.as_mut_ptr(),
            )
            .from_abi::<AppInstance>(result__)
        })
    }
    pub fn Restart(
        arguments: &::windows::core::HSTRING,
    ) -> ::windows::core::Result<::windows::ApplicationModel::Core::AppRestartFailureReason> {
        Self::IAppInstanceStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Restart)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(arguments),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::ApplicationModel::Core::AppRestartFailureReason>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IAppInstanceStatics<R, F: FnOnce(&IAppInstanceStatics) -> ::windows::core::Result<R>>(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<AppInstance, IAppInstanceStatics> =
            ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IAppInstanceStatics2<
        R,
        F: FnOnce(&IAppInstanceStatics2) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<AppInstance, IAppInstanceStatics2> =
            ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for AppInstance {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppInstance {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppInstance {}
impl ::core::fmt::Debug for AppInstance {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppInstance").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppInstance {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.Windows.AppLifecycle.AppInstance;{75766ae4-0239-5a26-b9da-d5bfc75a4866})",
    );
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for AppInstance {
    type Vtable = IAppInstance_Vtbl;
}
unsafe impl ::windows::core::Interface for AppInstance {
    const IID: ::windows::core::GUID = <IAppInstance as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppInstance {
    const NAME: &'static str = "Microsoft.Windows.AppLifecycle.AppInstance";
}
::windows::core::interface_hierarchy!(
    AppInstance,
    ::windows::core::IUnknown,
    ::windows::core::IInspectable
);
unsafe impl ::core::marker::Send for AppInstance {}
unsafe impl ::core::marker::Sync for AppInstance {}
#[doc = "*Required features: `\"Windows_AppLifecycle\"`*"]
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
unsafe impl ::windows::core::Abi for ExtendedActivationKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for ExtendedActivationKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ExtendedActivationKind").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ExtendedActivationKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.Windows.AppLifecycle.ExtendedActivationKind;i4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
