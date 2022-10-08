#[cfg(feature = "Windows_AppNotifications_Builder")]
pub mod Builder;
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppNotification(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IAppNotification {
    type Vtable = IAppNotification_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppNotification {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x373a6917_4116_5657_936a_15f99afdd667);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppNotification_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Tag: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub SetTag: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub Group: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub SetGroup: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub Id: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut u32,
    ) -> ::windows::core::HRESULT,
    pub Payload: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub Progress: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub SetProgress: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub Expiration: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::DateTime,
    ) -> ::windows::core::HRESULT,
    pub SetExpiration: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::windows::Foundation::DateTime,
    ) -> ::windows::core::HRESULT,
    pub ExpiresOnReboot: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub SetExpiresOnReboot: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows::core::HRESULT,
    pub Priority: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut AppNotificationPriority,
    ) -> ::windows::core::HRESULT,
    pub SetPriority: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: AppNotificationPriority,
    ) -> ::windows::core::HRESULT,
    pub SuppressDisplay: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub SetSuppressDisplay: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppNotificationActivatedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IAppNotificationActivatedEventArgs {
    type Vtable = IAppNotificationActivatedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppNotificationActivatedEventArgs {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x7a8afaf9_31cb_51d5_82be_db6bd5878b77);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppNotificationActivatedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Argument: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub UserInput: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppNotificationActivatedEventArgs2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IAppNotificationActivatedEventArgs2 {
    type Vtable = IAppNotificationActivatedEventArgs2_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppNotificationActivatedEventArgs2 {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x52c06b9b_2c50_5037_9416_a3be47b9d5bd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppNotificationActivatedEventArgs2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Arguments: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppNotificationFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IAppNotificationFactory {
    type Vtable = IAppNotificationFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppNotificationFactory {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x9ffee485_184a_5c65_87a9_c1d94469dbe7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppNotificationFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        payload: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppNotificationManager(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IAppNotificationManager {
    type Vtable = IAppNotificationManager_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppNotificationManager {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x55129688_b4bd_550b_ae6b_c24061954d91);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppNotificationManager_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Register:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Unregister:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub UnregisterAll:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub NotificationInvoked: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub RemoveNotificationInvoked: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub Show: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        notification: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub UpdateAsync: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        data: *mut ::core::ffi::c_void,
        tag: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        group: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub UpdateAsync2: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        data: *mut ::core::ffi::c_void,
        tag: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub Setting: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut AppNotificationSetting,
    ) -> ::windows::core::HRESULT,
    pub RemoveByIdAsync: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        notificationid: u32,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub RemoveByTagAsync: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        tag: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub RemoveByTagAndGroupAsync: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        tag: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        group: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub RemoveByGroupAsync: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        group: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub RemoveAllAsync: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub GetAllAsync: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppNotificationManager2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IAppNotificationManager2 {
    type Vtable = IAppNotificationManager2_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppNotificationManager2 {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x38ba268d_e0c7_522e_a79d_8a29dcdd7135);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppNotificationManager2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Register: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        displayname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        iconuri: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppNotificationManagerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IAppNotificationManagerStatics {
    type Vtable = IAppNotificationManagerStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppNotificationManagerStatics {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x6cfc0d8d_84a3_5592_b4c6_e3e7e7c680e4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppNotificationManagerStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Default: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppNotificationManagerStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IAppNotificationManagerStatics2 {
    type Vtable = IAppNotificationManagerStatics2_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppNotificationManagerStatics2 {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x6eb42a35_e82f_5732_98f1_129705602f2e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppNotificationManagerStatics2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub IsSupported: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppNotificationProgressData(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IAppNotificationProgressData {
    type Vtable = IAppNotificationProgressData_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppNotificationProgressData {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x4debfac0_809d_55cb_b879_924821876b97);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppNotificationProgressData_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub SequenceNumber: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut u32,
    ) -> ::windows::core::HRESULT,
    pub SetSequenceNumber: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: u32,
    ) -> ::windows::core::HRESULT,
    pub Title: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub SetTitle: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub Value: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows::core::HRESULT,
    pub SetValue: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
    ) -> ::windows::core::HRESULT,
    pub ValueStringOverride: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub SetValueStringOverride: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub Status: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub SetStatus: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppNotificationProgressDataFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IAppNotificationProgressDataFactory {
    type Vtable = IAppNotificationProgressDataFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppNotificationProgressDataFactory {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xc08e4a0f_3a75_55d6_8c3e_14f03ae46046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppNotificationProgressDataFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        sequencenumber: u32,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Windows_AppNotifications\"`*"]
#[repr(transparent)]
pub struct AppNotification(::windows::core::IUnknown);
impl AppNotification {
    pub fn Tag(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Tag)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetTag(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetTag)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn Group(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Group)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetGroup(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetGroup)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn Id(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Id)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<u32>(result__)
        }
    }
    pub fn Payload(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Payload)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn Progress(&self) -> ::windows::core::Result<AppNotificationProgressData> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Progress)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<AppNotificationProgressData>(result__)
        }
    }
    pub fn SetProgress(&self, value: &AppNotificationProgressData) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetProgress)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn Expiration(&self) -> ::windows::core::Result<::windows::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Expiration)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::DateTime>(result__)
        }
    }
    pub fn SetExpiration(
        &self,
        value: ::windows::Foundation::DateTime,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetExpiration)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn ExpiresOnReboot(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ExpiresOnReboot)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetExpiresOnReboot(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetExpiresOnReboot)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn Priority(&self) -> ::windows::core::Result<AppNotificationPriority> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Priority)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<AppNotificationPriority>(result__)
        }
    }
    pub fn SetPriority(&self, value: AppNotificationPriority) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetPriority)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn SuppressDisplay(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SuppressDisplay)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetSuppressDisplay(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetSuppressDisplay)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn CreateInstance(
        payload: &::windows::core::HSTRING,
    ) -> ::windows::core::Result<AppNotification> {
        Self::IAppNotificationFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateInstance)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(payload),
                result__.as_mut_ptr(),
            )
            .from_abi::<AppNotification>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IAppNotificationFactory<
        R,
        F: FnOnce(&IAppNotificationFactory) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<AppNotification, IAppNotificationFactory> =
            ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for AppNotification {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppNotification {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppNotification {}
impl ::core::fmt::Debug for AppNotification {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppNotification").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppNotification {
    const SIGNATURE : ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice ( b"rc(Microsoft.Windows.AppNotifications.AppNotification;{373a6917-4116-5657-936a-15f99afdd667})" ) ;
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for AppNotification {
    type Vtable = IAppNotification_Vtbl;
}
unsafe impl ::windows::core::Interface for AppNotification {
    const IID: ::windows::core::GUID = <IAppNotification as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppNotification {
    const NAME: &'static str = "Microsoft.Windows.AppNotifications.AppNotification";
}
::windows::core::interface_hierarchy!(
    AppNotification,
    ::windows::core::IUnknown,
    ::windows::core::IInspectable
);
unsafe impl ::core::marker::Send for AppNotification {}
unsafe impl ::core::marker::Sync for AppNotification {}
#[doc = "*Required features: `\"Windows_AppNotifications\"`*"]
#[repr(transparent)]
pub struct AppNotificationActivatedEventArgs(::windows::core::IUnknown);
impl AppNotificationActivatedEventArgs {
    pub fn Argument(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Argument)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn UserInput(
        &self,
    ) -> ::windows::core::Result<
        ::windows::Foundation::Collections::IMap<
            ::windows::core::HSTRING,
            ::windows::core::HSTRING,
        >,
    > {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).UserInput)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::Collections::IMap<
                ::windows::core::HSTRING,
                ::windows::core::HSTRING,
            >>(result__)
        }
    }
    pub fn Arguments(
        &self,
    ) -> ::windows::core::Result<
        ::windows::Foundation::Collections::IMap<
            ::windows::core::HSTRING,
            ::windows::core::HSTRING,
        >,
    > {
        let this = &::windows::core::Interface::cast::<IAppNotificationActivatedEventArgs2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Arguments)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::Collections::IMap<
                ::windows::core::HSTRING,
                ::windows::core::HSTRING,
            >>(result__)
        }
    }
}
impl ::core::clone::Clone for AppNotificationActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppNotificationActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppNotificationActivatedEventArgs {}
impl ::core::fmt::Debug for AppNotificationActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppNotificationActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppNotificationActivatedEventArgs {
    const SIGNATURE : ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice ( b"rc(Microsoft.Windows.AppNotifications.AppNotificationActivatedEventArgs;{7a8afaf9-31cb-51d5-82be-db6bd5878b77})" ) ;
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for AppNotificationActivatedEventArgs {
    type Vtable = IAppNotificationActivatedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for AppNotificationActivatedEventArgs {
    const IID: ::windows::core::GUID =
        <IAppNotificationActivatedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppNotificationActivatedEventArgs {
    const NAME: &'static str =
        "Microsoft.Windows.AppNotifications.AppNotificationActivatedEventArgs";
}
::windows::core::interface_hierarchy!(
    AppNotificationActivatedEventArgs,
    ::windows::core::IUnknown,
    ::windows::core::IInspectable
);
unsafe impl ::core::marker::Send for AppNotificationActivatedEventArgs {}
unsafe impl ::core::marker::Sync for AppNotificationActivatedEventArgs {}
#[doc = "*Required features: `\"Windows_AppNotifications\"`*"]
#[repr(transparent)]
pub struct AppNotificationManager(::windows::core::IUnknown);
impl AppNotificationManager {
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
    pub fn NotificationInvoked(
        &self,
        handler: &::windows::Foundation::TypedEventHandler<
            AppNotificationManager,
            AppNotificationActivatedEventArgs,
        >,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NotificationInvoked)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(handler),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveNotificationInvoked(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).RemoveNotificationInvoked)(
                ::windows::core::Vtable::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn Show(&self, notification: &AppNotification) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).Show)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(notification),
            )
            .ok()
        }
    }
    pub fn UpdateAsync(
        &self,
        data: &AppNotificationProgressData,
        tag: &::windows::core::HSTRING,
        group: &::windows::core::HSTRING,
    ) -> ::windows::core::Result<
        ::windows::Foundation::IAsyncOperation<AppNotificationProgressResult>,
    > {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).UpdateAsync)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(data),
                ::core::mem::transmute_copy(tag),
                ::core::mem::transmute_copy(group),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::IAsyncOperation<AppNotificationProgressResult>>(
                result__,
            )
        }
    }
    pub fn UpdateAsync2(
        &self,
        data: &AppNotificationProgressData,
        tag: &::windows::core::HSTRING,
    ) -> ::windows::core::Result<
        ::windows::Foundation::IAsyncOperation<AppNotificationProgressResult>,
    > {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).UpdateAsync2)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(data),
                ::core::mem::transmute_copy(tag),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::IAsyncOperation<AppNotificationProgressResult>>(
                result__,
            )
        }
    }
    pub fn Setting(&self) -> ::windows::core::Result<AppNotificationSetting> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Setting)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<AppNotificationSetting>(result__)
        }
    }
    pub fn RemoveByIdAsync(
        &self,
        notificationid: u32,
    ) -> ::windows::core::Result<::windows::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RemoveByIdAsync)(
                ::windows::core::Vtable::as_raw(this),
                notificationid,
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::IAsyncAction>(result__)
        }
    }
    pub fn RemoveByTagAsync(
        &self,
        tag: &::windows::core::HSTRING,
    ) -> ::windows::core::Result<::windows::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RemoveByTagAsync)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(tag),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::IAsyncAction>(result__)
        }
    }
    pub fn RemoveByTagAndGroupAsync(
        &self,
        tag: &::windows::core::HSTRING,
        group: &::windows::core::HSTRING,
    ) -> ::windows::core::Result<::windows::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RemoveByTagAndGroupAsync)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(tag),
                ::core::mem::transmute_copy(group),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::IAsyncAction>(result__)
        }
    }
    pub fn RemoveByGroupAsync(
        &self,
        group: &::windows::core::HSTRING,
    ) -> ::windows::core::Result<::windows::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RemoveByGroupAsync)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(group),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::IAsyncAction>(result__)
        }
    }
    pub fn RemoveAllAsync(&self) -> ::windows::core::Result<::windows::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RemoveAllAsync)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::IAsyncAction>(result__)
        }
    }
    pub fn GetAllAsync(
        &self,
    ) -> ::windows::core::Result<
        ::windows::Foundation::IAsyncOperation<
            ::windows::Foundation::Collections::IVector<AppNotification>,
        >,
    > {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetAllAsync)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::IAsyncOperation<
                ::windows::Foundation::Collections::IVector<AppNotification>,
            >>(result__)
        }
    }
    pub fn Register2(
        &self,
        displayname: &::windows::core::HSTRING,
        iconuri: &::windows::Foundation::Uri,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppNotificationManager2>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).Register)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(displayname),
                ::core::mem::transmute_copy(iconuri),
            )
            .ok()
        }
    }
    pub fn Default() -> ::windows::core::Result<AppNotificationManager> {
        Self::IAppNotificationManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Default)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<AppNotificationManager>(result__)
        })
    }
    pub fn IsSupported() -> ::windows::core::Result<bool> {
        Self::IAppNotificationManagerStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsSupported)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IAppNotificationManagerStatics<
        R,
        F: FnOnce(&IAppNotificationManagerStatics) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<
            AppNotificationManager,
            IAppNotificationManagerStatics,
        > = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IAppNotificationManagerStatics2<
        R,
        F: FnOnce(&IAppNotificationManagerStatics2) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<
            AppNotificationManager,
            IAppNotificationManagerStatics2,
        > = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for AppNotificationManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppNotificationManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppNotificationManager {}
impl ::core::fmt::Debug for AppNotificationManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppNotificationManager").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppNotificationManager {
    const SIGNATURE : ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice ( b"rc(Microsoft.Windows.AppNotifications.AppNotificationManager;{55129688-b4bd-550b-ae6b-c24061954d91})" ) ;
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for AppNotificationManager {
    type Vtable = IAppNotificationManager_Vtbl;
}
unsafe impl ::windows::core::Interface for AppNotificationManager {
    const IID: ::windows::core::GUID = <IAppNotificationManager as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppNotificationManager {
    const NAME: &'static str = "Microsoft.Windows.AppNotifications.AppNotificationManager";
}
::windows::core::interface_hierarchy!(
    AppNotificationManager,
    ::windows::core::IUnknown,
    ::windows::core::IInspectable
);
unsafe impl ::core::marker::Send for AppNotificationManager {}
unsafe impl ::core::marker::Sync for AppNotificationManager {}
#[doc = "*Required features: `\"Windows_AppNotifications\"`*"]
#[repr(transparent)]
pub struct AppNotificationProgressData(::windows::core::IUnknown);
impl AppNotificationProgressData {
    pub fn SequenceNumber(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SequenceNumber)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<u32>(result__)
        }
    }
    pub fn SetSequenceNumber(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetSequenceNumber)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn Title(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Title)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetTitle(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetTitle)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn Value(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Value)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetValue(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetValue)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn ValueStringOverride(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ValueStringOverride)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetValueStringOverride(
        &self,
        value: &::windows::core::HSTRING,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetValueStringOverride)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn Status(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Status)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetStatus(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetStatus)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn CreateInstance(
        sequencenumber: u32,
    ) -> ::windows::core::Result<AppNotificationProgressData> {
        Self::IAppNotificationProgressDataFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateInstance)(
                ::windows::core::Vtable::as_raw(this),
                sequencenumber,
                result__.as_mut_ptr(),
            )
            .from_abi::<AppNotificationProgressData>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IAppNotificationProgressDataFactory<
        R,
        F: FnOnce(&IAppNotificationProgressDataFactory) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<
            AppNotificationProgressData,
            IAppNotificationProgressDataFactory,
        > = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for AppNotificationProgressData {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppNotificationProgressData {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppNotificationProgressData {}
impl ::core::fmt::Debug for AppNotificationProgressData {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppNotificationProgressData").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppNotificationProgressData {
    const SIGNATURE : ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice ( b"rc(Microsoft.Windows.AppNotifications.AppNotificationProgressData;{4debfac0-809d-55cb-b879-924821876b97})" ) ;
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for AppNotificationProgressData {
    type Vtable = IAppNotificationProgressData_Vtbl;
}
unsafe impl ::windows::core::Interface for AppNotificationProgressData {
    const IID: ::windows::core::GUID =
        <IAppNotificationProgressData as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppNotificationProgressData {
    const NAME: &'static str = "Microsoft.Windows.AppNotifications.AppNotificationProgressData";
}
::windows::core::interface_hierarchy!(
    AppNotificationProgressData,
    ::windows::core::IUnknown,
    ::windows::core::IInspectable
);
unsafe impl ::core::marker::Send for AppNotificationProgressData {}
unsafe impl ::core::marker::Sync for AppNotificationProgressData {}
#[doc = "*Required features: `\"Windows_AppNotifications\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AppNotificationPriority(pub i32);
impl AppNotificationPriority {
    pub const Default: Self = Self(0i32);
    pub const High: Self = Self(1i32);
}
impl ::core::marker::Copy for AppNotificationPriority {}
impl ::core::clone::Clone for AppNotificationPriority {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AppNotificationPriority {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for AppNotificationPriority {
    type Abi = Self;
}
impl ::core::fmt::Debug for AppNotificationPriority {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppNotificationPriority").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppNotificationPriority {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.Windows.AppNotifications.AppNotificationPriority;i4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Windows_AppNotifications\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AppNotificationProgressResult(pub i32);
impl AppNotificationProgressResult {
    pub const Succeeded: Self = Self(0i32);
    pub const AppNotificationNotFound: Self = Self(1i32);
    pub const Unsupported: Self = Self(2i32);
}
impl ::core::marker::Copy for AppNotificationProgressResult {}
impl ::core::clone::Clone for AppNotificationProgressResult {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AppNotificationProgressResult {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for AppNotificationProgressResult {
    type Abi = Self;
}
impl ::core::fmt::Debug for AppNotificationProgressResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppNotificationProgressResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppNotificationProgressResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.Windows.AppNotifications.AppNotificationProgressResult;i4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Windows_AppNotifications\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AppNotificationSetting(pub i32);
impl AppNotificationSetting {
    pub const Enabled: Self = Self(0i32);
    pub const DisabledForApplication: Self = Self(1i32);
    pub const DisabledForUser: Self = Self(2i32);
    pub const DisabledByGroupPolicy: Self = Self(3i32);
    pub const DisabledByManifest: Self = Self(4i32);
    pub const Unsupported: Self = Self(5i32);
}
impl ::core::marker::Copy for AppNotificationSetting {}
impl ::core::clone::Clone for AppNotificationSetting {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AppNotificationSetting {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for AppNotificationSetting {
    type Abi = Self;
}
impl ::core::fmt::Debug for AppNotificationSetting {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppNotificationSetting").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppNotificationSetting {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.Windows.AppNotifications.AppNotificationSetting;i4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
