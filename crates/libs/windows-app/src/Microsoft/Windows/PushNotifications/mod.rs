#[doc(hidden)]
#[repr(transparent)]
pub struct IPushNotificationChannel(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IPushNotificationChannel {
    type Vtable = IPushNotificationChannel_Vtbl;
}
unsafe impl ::windows::core::Interface for IPushNotificationChannel {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xda28bbcb_7695_5d38_af82_f30b72fef1f6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPushNotificationChannel_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Uri: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub ExpirationTime: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::DateTime,
    ) -> ::windows::core::HRESULT,
    pub Close:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPushNotificationCreateChannelResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IPushNotificationCreateChannelResult {
    type Vtable = IPushNotificationCreateChannelResult_Vtbl;
}
unsafe impl ::windows::core::Interface for IPushNotificationCreateChannelResult {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x4df3717f_5d33_56e9_b381_1b350c95722e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPushNotificationCreateChannelResult_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Channel: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::HRESULT,
    ) -> ::windows::core::HRESULT,
    pub Status: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut PushNotificationChannelStatus,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPushNotificationManager(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IPushNotificationManager {
    type Vtable = IPushNotificationManager_Vtbl;
}
unsafe impl ::windows::core::Interface for IPushNotificationManager {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x902f4aba_ff63_5dfe_a88f_15cc6bed55ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPushNotificationManager_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Register:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Unregister:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub UnregisterAll:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateChannelAsync: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        remoteid: ::windows::core::GUID,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub PushReceived: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub RemovePushReceived: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPushNotificationManagerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IPushNotificationManagerStatics {
    type Vtable = IPushNotificationManagerStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IPushNotificationManagerStatics {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x71329470_1b55_58dc_a00c_68c26f2d8bd9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPushNotificationManagerStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub IsSupported: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub Default: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPushNotificationReceivedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IPushNotificationReceivedEventArgs {
    type Vtable = IPushNotificationReceivedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IPushNotificationReceivedEventArgs {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xfbd4ec53_bb83_5495_8777_d3cf13e4299c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPushNotificationReceivedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Payload: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result_size__: *mut u32,
        result__: *mut *mut u8,
    ) -> ::windows::core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub Canceled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub RemoveCanceled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Windows_PushNotifications\"`*"]
#[repr(transparent)]
pub struct PushNotificationChannel(::windows::core::IUnknown);
impl PushNotificationChannel {
    pub fn Uri(&self) -> ::windows::core::Result<::windows::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Uri)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::Uri>(result__)
        }
    }
    pub fn ExpirationTime(&self) -> ::windows::core::Result<::windows::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ExpirationTime)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::DateTime>(result__)
        }
    }
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).Close)(::windows::core::Vtable::as_raw(this))
                .ok()
        }
    }
}
impl ::core::clone::Clone for PushNotificationChannel {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PushNotificationChannel {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PushNotificationChannel {}
impl ::core::fmt::Debug for PushNotificationChannel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PushNotificationChannel").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PushNotificationChannel {
    const SIGNATURE : ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice ( b"rc(Microsoft.Windows.PushNotifications.PushNotificationChannel;{da28bbcb-7695-5d38-af82-f30b72fef1f6})" ) ;
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for PushNotificationChannel {
    type Vtable = IPushNotificationChannel_Vtbl;
}
unsafe impl ::windows::core::Interface for PushNotificationChannel {
    const IID: ::windows::core::GUID =
        <IPushNotificationChannel as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PushNotificationChannel {
    const NAME: &'static str = "Microsoft.Windows.PushNotifications.PushNotificationChannel";
}
::windows::core::interface_hierarchy!(
    PushNotificationChannel,
    ::windows::core::IUnknown,
    ::windows::core::IInspectable
);
unsafe impl ::core::marker::Send for PushNotificationChannel {}
unsafe impl ::core::marker::Sync for PushNotificationChannel {}
#[doc = "*Required features: `\"Windows_PushNotifications\"`*"]
#[repr(transparent)]
pub struct PushNotificationCreateChannelResult(::windows::core::IUnknown);
impl PushNotificationCreateChannelResult {
    pub fn Channel(&self) -> ::windows::core::Result<PushNotificationChannel> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Channel)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<PushNotificationChannel>(result__)
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
    pub fn Status(&self) -> ::windows::core::Result<PushNotificationChannelStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Status)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<PushNotificationChannelStatus>(result__)
        }
    }
}
impl ::core::clone::Clone for PushNotificationCreateChannelResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PushNotificationCreateChannelResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PushNotificationCreateChannelResult {}
impl ::core::fmt::Debug for PushNotificationCreateChannelResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PushNotificationCreateChannelResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PushNotificationCreateChannelResult {
    const SIGNATURE : ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice ( b"rc(Microsoft.Windows.PushNotifications.PushNotificationCreateChannelResult;{4df3717f-5d33-56e9-b381-1b350c95722e})" ) ;
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for PushNotificationCreateChannelResult {
    type Vtable = IPushNotificationCreateChannelResult_Vtbl;
}
unsafe impl ::windows::core::Interface for PushNotificationCreateChannelResult {
    const IID: ::windows::core::GUID =
        <IPushNotificationCreateChannelResult as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PushNotificationCreateChannelResult {
    const NAME: &'static str =
        "Microsoft.Windows.PushNotifications.PushNotificationCreateChannelResult";
}
::windows::core::interface_hierarchy!(
    PushNotificationCreateChannelResult,
    ::windows::core::IUnknown,
    ::windows::core::IInspectable
);
unsafe impl ::core::marker::Send for PushNotificationCreateChannelResult {}
unsafe impl ::core::marker::Sync for PushNotificationCreateChannelResult {}
#[doc = "*Required features: `\"Windows_PushNotifications\"`*"]
#[repr(transparent)]
pub struct PushNotificationManager(::windows::core::IUnknown);
impl PushNotificationManager {
    pub fn Register(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).Register)(::windows::core::Vtable::as_raw(this))
                .ok()
        }
    }
    pub fn Unregister(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).Unregister)(::windows::core::Vtable::as_raw(
                this,
            ))
            .ok()
        }
    }
    pub fn UnregisterAll(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).UnregisterAll)(::windows::core::Vtable::as_raw(
                this,
            ))
            .ok()
        }
    }
    pub fn CreateChannelAsync(
        &self,
        remoteid: ::windows::core::GUID,
    ) -> ::windows::core::Result<
        ::windows::Foundation::IAsyncOperationWithProgress<
            PushNotificationCreateChannelResult,
            PushNotificationCreateChannelStatus,
        >,
    > {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateChannelAsync)(
                ::windows::core::Vtable::as_raw(this),
                remoteid,
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::IAsyncOperationWithProgress<
                PushNotificationCreateChannelResult,
                PushNotificationCreateChannelStatus,
            >>(result__)
        }
    }
    pub fn PushReceived(
        &self,
        handler: &::windows::Foundation::TypedEventHandler<
            PushNotificationManager,
            PushNotificationReceivedEventArgs,
        >,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PushReceived)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(handler),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemovePushReceived(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).RemovePushReceived)(
                ::windows::core::Vtable::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn IsSupported() -> ::windows::core::Result<bool> {
        Self::IPushNotificationManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsSupported)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        })
    }
    pub fn Default() -> ::windows::core::Result<PushNotificationManager> {
        Self::IPushNotificationManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Default)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<PushNotificationManager>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPushNotificationManagerStatics<
        R,
        F: FnOnce(&IPushNotificationManagerStatics) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<
            PushNotificationManager,
            IPushNotificationManagerStatics,
        > = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for PushNotificationManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PushNotificationManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PushNotificationManager {}
impl ::core::fmt::Debug for PushNotificationManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PushNotificationManager").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PushNotificationManager {
    const SIGNATURE : ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice ( b"rc(Microsoft.Windows.PushNotifications.PushNotificationManager;{902f4aba-ff63-5dfe-a88f-15cc6bed55ff})" ) ;
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for PushNotificationManager {
    type Vtable = IPushNotificationManager_Vtbl;
}
unsafe impl ::windows::core::Interface for PushNotificationManager {
    const IID: ::windows::core::GUID =
        <IPushNotificationManager as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PushNotificationManager {
    const NAME: &'static str = "Microsoft.Windows.PushNotifications.PushNotificationManager";
}
::windows::core::interface_hierarchy!(
    PushNotificationManager,
    ::windows::core::IUnknown,
    ::windows::core::IInspectable
);
unsafe impl ::core::marker::Send for PushNotificationManager {}
unsafe impl ::core::marker::Sync for PushNotificationManager {}
#[doc = "*Required features: `\"Windows_PushNotifications\"`*"]
#[repr(transparent)]
pub struct PushNotificationReceivedEventArgs(::windows::core::IUnknown);
impl PushNotificationReceivedEventArgs {
    pub fn Payload(&self) -> ::windows::core::Result<::windows::core::Array<u8>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Payload)(
                ::windows::core::Vtable::as_raw(this),
                ::windows::core::Array::<u8>::set_abi_len(result__.assume_init_mut()),
                result__.as_mut_ptr() as *mut _ as _,
            )
            .and_then(|| result__.assume_init())
        }
    }
    pub fn GetDeferral(
        &self,
    ) -> ::windows::core::Result<::windows::ApplicationModel::Background::BackgroundTaskDeferral>
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDeferral)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::ApplicationModel::Background::BackgroundTaskDeferral>(result__)
        }
    }
    pub fn Canceled(
        &self,
        handler: &::windows::ApplicationModel::Background::BackgroundTaskCanceledEventHandler,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Canceled)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(handler),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveCanceled(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).RemoveCanceled)(
                ::windows::core::Vtable::as_raw(this),
                token,
            )
            .ok()
        }
    }
}
impl ::core::clone::Clone for PushNotificationReceivedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PushNotificationReceivedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PushNotificationReceivedEventArgs {}
impl ::core::fmt::Debug for PushNotificationReceivedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PushNotificationReceivedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PushNotificationReceivedEventArgs {
    const SIGNATURE : ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice ( b"rc(Microsoft.Windows.PushNotifications.PushNotificationReceivedEventArgs;{fbd4ec53-bb83-5495-8777-d3cf13e4299c})" ) ;
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for PushNotificationReceivedEventArgs {
    type Vtable = IPushNotificationReceivedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for PushNotificationReceivedEventArgs {
    const IID: ::windows::core::GUID =
        <IPushNotificationReceivedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PushNotificationReceivedEventArgs {
    const NAME: &'static str =
        "Microsoft.Windows.PushNotifications.PushNotificationReceivedEventArgs";
}
::windows::core::interface_hierarchy!(
    PushNotificationReceivedEventArgs,
    ::windows::core::IUnknown,
    ::windows::core::IInspectable
);
unsafe impl ::core::marker::Send for PushNotificationReceivedEventArgs {}
unsafe impl ::core::marker::Sync for PushNotificationReceivedEventArgs {}
#[doc = "*Required features: `\"Windows_PushNotifications\"`*"]
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
unsafe impl ::windows::core::Abi for PushNotificationChannelStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for PushNotificationChannelStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PushNotificationChannelStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PushNotificationChannelStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.Windows.PushNotifications.PushNotificationChannelStatus;i4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Windows_PushNotifications\"`*"]
pub struct PushNotificationCreateChannelStatus {
    pub status: PushNotificationChannelStatus,
    pub extendedError: ::windows::core::HRESULT,
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
unsafe impl ::windows::core::Abi for PushNotificationCreateChannelStatus {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for PushNotificationCreateChannelStatus {
    const SIGNATURE : ::windows::core::ConstBuffer =::windows::core::ConstBuffer::from_slice ( b"struct(Microsoft.Windows.PushNotifications.PushNotificationCreateChannelStatus;enum(Microsoft.Windows.PushNotifications.PushNotificationChannelStatus;i4);struct(Windows.Foundation.HResult;i4);u4)" ) ;
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
impl ::core::cmp::PartialEq for PushNotificationCreateChannelStatus {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            ::windows::core::memcmp(
                self as *const _ as _,
                other as *const _ as _,
                core::mem::size_of::<PushNotificationCreateChannelStatus>(),
            ) == 0
        }
    }
}
impl ::core::cmp::Eq for PushNotificationCreateChannelStatus {}
impl ::core::default::Default for PushNotificationCreateChannelStatus {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
