#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPushNotificationChannel(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPushNotificationChannel {
    type Vtable = IPushNotificationChannel_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPushNotificationChannel {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xda28bbcb_7695_5d38_af82_f30b72fef1f6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPushNotificationChannel_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Windows_Foundation")]
    pub Uri: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    Uri: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub ExpirationTime: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::DateTime,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    ExpirationTime: usize,
    pub Close: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPushNotificationCreateChannelResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPushNotificationCreateChannelResult {
    type Vtable = IPushNotificationCreateChannelResult_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPushNotificationCreateChannelResult {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x4df3717f_5d33_56e9_b381_1b350c95722e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPushNotificationCreateChannelResult_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Channel: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows_core::HRESULT,
    ) -> ::windows_core::HRESULT,
    pub Status: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut PushNotificationChannelStatus,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPushNotificationManager(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPushNotificationManager {
    type Vtable = IPushNotificationManager_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPushNotificationManager {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x902f4aba_ff63_5dfe_a88f_15cc6bed55ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPushNotificationManager_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Register:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Unregister:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub UnregisterAll:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_Foundation")]
    pub CreateChannelAsync: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        remoteid: ::windows_core::GUID,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    CreateChannelAsync: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub PushReceived: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    PushReceived: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub RemovePushReceived: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RemovePushReceived: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPushNotificationManagerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPushNotificationManagerStatics {
    type Vtable = IPushNotificationManagerStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPushNotificationManagerStatics {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x71329470_1b55_58dc_a00c_68c26f2d8bd9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPushNotificationManagerStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub IsSupported: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub Default: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPushNotificationReceivedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPushNotificationReceivedEventArgs {
    type Vtable = IPushNotificationReceivedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPushNotificationReceivedEventArgs {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xfbd4ec53_bb83_5495_8777_d3cf13e4299c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPushNotificationReceivedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Payload: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result_size__: *mut u32,
        result__: *mut *mut u8,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_ApplicationModel_Background")]
    pub GetDeferral: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_ApplicationModel_Background"))]
    GetDeferral: usize,
    #[cfg(all(
        feature = "Windows_ApplicationModel_Background",
        feature = "Windows_Foundation"
    ))]
    pub Canceled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(all(
        feature = "Windows_ApplicationModel_Background",
        feature = "Windows_Foundation"
    )))]
    Canceled: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub RemoveCanceled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RemoveCanceled: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct PushNotificationChannel(::windows_core::IUnknown);
impl PushNotificationChannel {
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn Uri(&self) -> ::windows_core::Result<::windows::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Uri)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn ExpirationTime(&self) -> ::windows_core::Result<::windows::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ExpirationTime)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this))
                .ok()
        }
    }
}
impl ::windows_core::RuntimeType for PushNotificationChannel {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for PushNotificationChannel {
    type Vtable = IPushNotificationChannel_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PushNotificationChannel {
    const IID: ::windows_core::GUID =
        <IPushNotificationChannel as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PushNotificationChannel {
    const NAME: &'static str = "Microsoft.Windows.PushNotifications.PushNotificationChannel";
}
::windows_core::imp::interface_hierarchy!(
    PushNotificationChannel,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for PushNotificationChannel {}
unsafe impl ::core::marker::Sync for PushNotificationChannel {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct PushNotificationCreateChannelResult(::windows_core::IUnknown);
impl PushNotificationCreateChannelResult {
    pub fn Channel(&self) -> ::windows_core::Result<PushNotificationChannel> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Channel)(
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
    pub fn Status(&self) -> ::windows_core::Result<PushNotificationChannelStatus> {
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
}
impl ::windows_core::RuntimeType for PushNotificationCreateChannelResult {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for PushNotificationCreateChannelResult {
    type Vtable = IPushNotificationCreateChannelResult_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PushNotificationCreateChannelResult {
    const IID: ::windows_core::GUID =
        <IPushNotificationCreateChannelResult as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PushNotificationCreateChannelResult {
    const NAME: &'static str =
        "Microsoft.Windows.PushNotifications.PushNotificationCreateChannelResult";
}
::windows_core::imp::interface_hierarchy!(
    PushNotificationCreateChannelResult,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for PushNotificationCreateChannelResult {}
unsafe impl ::core::marker::Sync for PushNotificationCreateChannelResult {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct PushNotificationManager(::windows_core::IUnknown);
impl PushNotificationManager {
    pub fn Register(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).Register)(::windows_core::Interface::as_raw(
                this,
            ))
            .ok()
        }
    }
    pub fn Unregister(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).Unregister)(::windows_core::Interface::as_raw(
                this,
            ))
            .ok()
        }
    }
    pub fn UnregisterAll(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).UnregisterAll)(
                ::windows_core::Interface::as_raw(this),
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn CreateChannelAsync(
        &self,
        remoteid: ::windows_core::GUID,
    ) -> ::windows_core::Result<
        ::windows::Foundation::IAsyncOperationWithProgress<
            PushNotificationCreateChannelResult,
            PushNotificationCreateChannelStatus,
        >,
    > {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateChannelAsync)(
                ::windows_core::Interface::as_raw(this),
                remoteid,
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn PushReceived<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<
                PushNotificationManager,
                PushNotificationReceivedEventArgs,
            >,
        >,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PushReceived)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemovePushReceived(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).RemovePushReceived)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn IsSupported() -> ::windows_core::Result<bool> {
        Self::IPushNotificationManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsSupported)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn Default() -> ::windows_core::Result<PushNotificationManager> {
        Self::IPushNotificationManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Default)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPushNotificationManagerStatics<
        R,
        F: FnOnce(&IPushNotificationManagerStatics) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<
            PushNotificationManager,
            IPushNotificationManagerStatics,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for PushNotificationManager {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for PushNotificationManager {
    type Vtable = IPushNotificationManager_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PushNotificationManager {
    const IID: ::windows_core::GUID =
        <IPushNotificationManager as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PushNotificationManager {
    const NAME: &'static str = "Microsoft.Windows.PushNotifications.PushNotificationManager";
}
::windows_core::imp::interface_hierarchy!(
    PushNotificationManager,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for PushNotificationManager {}
unsafe impl ::core::marker::Sync for PushNotificationManager {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct PushNotificationReceivedEventArgs(::windows_core::IUnknown);
impl PushNotificationReceivedEventArgs {
    pub fn Payload(&self) -> ::windows_core::Result<::windows_core::Array<u8>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows_core::Interface::vtable(this).Payload)(
                ::windows_core::Interface::as_raw(this),
                ::windows_core::Array::<u8>::set_abi_len(::std::mem::transmute(&mut result__)),
                result__.as_mut_ptr() as *mut _ as _,
            )
            .and_then(|| result__.assume_init())
        }
    }
    #[doc = "Required features: `\"Windows_ApplicationModel_Background\"`"]
    #[cfg(feature = "Windows_ApplicationModel_Background")]
    pub fn GetDeferral(
        &self,
    ) -> ::windows_core::Result<::windows::ApplicationModel::Background::BackgroundTaskDeferral>
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetDeferral)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_ApplicationModel_Background\"`, `\"Windows_Foundation\"`"]
    #[cfg(all(
        feature = "Windows_ApplicationModel_Background",
        feature = "Windows_Foundation"
    ))]
    pub fn Canceled<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::ApplicationModel::Background::BackgroundTaskCanceledEventHandler,
        >,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Canceled)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveCanceled(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveCanceled)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
}
impl ::windows_core::RuntimeType for PushNotificationReceivedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for PushNotificationReceivedEventArgs {
    type Vtable = IPushNotificationReceivedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PushNotificationReceivedEventArgs {
    const IID: ::windows_core::GUID =
        <IPushNotificationReceivedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PushNotificationReceivedEventArgs {
    const NAME: &'static str =
        "Microsoft.Windows.PushNotifications.PushNotificationReceivedEventArgs";
}
::windows_core::imp::interface_hierarchy!(
    PushNotificationReceivedEventArgs,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for PushNotificationReceivedEventArgs {}
unsafe impl ::core::marker::Sync for PushNotificationReceivedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PushNotificationChannelStatus(pub i32);
impl PushNotificationChannelStatus {
    pub const InProgress: Self = Self(0i32);
    pub const InProgressRetry: Self = Self(1i32);
    pub const CompletedSuccess: Self = Self(2i32);
    pub const CompletedFailure: Self = Self(3i32);
}
impl ::core::marker::Copy for PushNotificationChannelStatus {}
impl ::core::clone::Clone for PushNotificationChannelStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PushNotificationChannelStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for PushNotificationChannelStatus {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for PushNotificationChannelStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PushNotificationChannelStatus").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for PushNotificationChannelStatus {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::from_slice(
            b"enum(Microsoft.Windows.PushNotifications.PushNotificationChannelStatus;i4)",
        );
}
#[repr(C)]
pub struct PushNotificationCreateChannelStatus {
    pub status: PushNotificationChannelStatus,
    pub extendedError: ::windows_core::HRESULT,
    pub retryCount: u32,
}
impl ::core::marker::Copy for PushNotificationCreateChannelStatus {}
impl ::core::clone::Clone for PushNotificationCreateChannelStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PushNotificationCreateChannelStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PushNotificationCreateChannelStatus")
            .field("status", &self.status)
            .field("extendedError", &self.extendedError)
            .field("retryCount", &self.retryCount)
            .finish()
    }
}
impl ::windows_core::TypeKind for PushNotificationCreateChannelStatus {
    type TypeKind = ::windows_core::CopyType;
}
impl ::windows_core::RuntimeType for PushNotificationCreateChannelStatus {
    const SIGNATURE : ::windows_core::imp::ConstBuffer =::windows_core::imp::ConstBuffer::from_slice ( b"struct(Microsoft.Windows.PushNotifications.PushNotificationCreateChannelStatus;enum(Microsoft.Windows.PushNotifications.PushNotificationChannelStatus;i4);struct(Windows.Foundation.HResult;i4);u4)" ) ;
}
impl ::core::cmp::PartialEq for PushNotificationCreateChannelStatus {
    fn eq(&self, other: &Self) -> bool {
        self.status == other.status
            && self.extendedError == other.extendedError
            && self.retryCount == other.retryCount
    }
}
impl ::core::cmp::Eq for PushNotificationCreateChannelStatus {}
impl ::core::default::Default for PushNotificationCreateChannelStatus {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
