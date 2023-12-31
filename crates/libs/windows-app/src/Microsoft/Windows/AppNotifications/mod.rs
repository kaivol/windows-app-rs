#[cfg(feature = "Microsoft_Windows_AppNotifications_Builder")]
#[doc = "Required features: `\"Microsoft_Windows_AppNotifications_Builder\"`"]
pub mod Builder;
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IAppNotification(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppNotification {
    type Vtable = IAppNotification_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IAppNotification {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x373a6917_4116_5657_936a_15f99afdd667);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppNotification_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Tag: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub SetTag: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub Group: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub SetGroup: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub Id: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut u32,
    ) -> ::windows_core::HRESULT,
    pub Payload: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub Progress: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetProgress: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_Foundation")]
    pub Expiration: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::DateTime,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    Expiration: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub SetExpiration: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::windows::Foundation::DateTime,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    SetExpiration: usize,
    pub ExpiresOnReboot: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetExpiresOnReboot: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
    pub Priority: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut AppNotificationPriority,
    ) -> ::windows_core::HRESULT,
    pub SetPriority: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: AppNotificationPriority,
    ) -> ::windows_core::HRESULT,
    pub SuppressDisplay: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetSuppressDisplay: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IAppNotificationActivatedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppNotificationActivatedEventArgs {
    type Vtable = IAppNotificationActivatedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IAppNotificationActivatedEventArgs {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x7a8afaf9_31cb_51d5_82be_db6bd5878b77);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppNotificationActivatedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Argument: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub UserInput: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation_Collections"))]
    UserInput: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IAppNotificationActivatedEventArgs2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppNotificationActivatedEventArgs2 {
    type Vtable = IAppNotificationActivatedEventArgs2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IAppNotificationActivatedEventArgs2 {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x52c06b9b_2c50_5037_9416_a3be47b9d5bd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppNotificationActivatedEventArgs2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub Arguments: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation_Collections"))]
    Arguments: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IAppNotificationFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppNotificationFactory {
    type Vtable = IAppNotificationFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IAppNotificationFactory {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x9ffee485_184a_5c65_87a9_c1d94469dbe7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppNotificationFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        payload: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IAppNotificationManager(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppNotificationManager {
    type Vtable = IAppNotificationManager_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IAppNotificationManager {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x55129688_b4bd_550b_ae6b_c24061954d91);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppNotificationManager_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Register:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Unregister:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub UnregisterAll:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_Foundation")]
    pub NotificationInvoked: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    NotificationInvoked: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub RemoveNotificationInvoked: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RemoveNotificationInvoked: usize,
    pub Show: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        notification: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_Foundation")]
    pub UpdateAsync: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        data: *mut ::core::ffi::c_void,
        tag: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        group: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    UpdateAsync: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub UpdateAsync2: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        data: *mut ::core::ffi::c_void,
        tag: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    UpdateAsync2: usize,
    pub Setting: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut AppNotificationSetting,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_Foundation")]
    pub RemoveByIdAsync: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        notificationid: u32,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RemoveByIdAsync: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub RemoveByTagAsync: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        tag: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RemoveByTagAsync: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub RemoveByTagAndGroupAsync: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        tag: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        group: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RemoveByTagAndGroupAsync: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub RemoveByGroupAsync: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        group: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RemoveByGroupAsync: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub RemoveAllAsync: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RemoveAllAsync: usize,
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub GetAllAsync: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation_Collections"))]
    GetAllAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IAppNotificationManager2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppNotificationManager2 {
    type Vtable = IAppNotificationManager2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IAppNotificationManager2 {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x38ba268d_e0c7_522e_a79d_8a29dcdd7135);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppNotificationManager2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Windows_Foundation")]
    pub Register: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        displayname: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        iconuri: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    Register: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IAppNotificationManagerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppNotificationManagerStatics {
    type Vtable = IAppNotificationManagerStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IAppNotificationManagerStatics {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x6cfc0d8d_84a3_5592_b4c6_e3e7e7c680e4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppNotificationManagerStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Default: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IAppNotificationManagerStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppNotificationManagerStatics2 {
    type Vtable = IAppNotificationManagerStatics2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IAppNotificationManagerStatics2 {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x6eb42a35_e82f_5732_98f1_129705602f2e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppNotificationManagerStatics2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub IsSupported: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IAppNotificationProgressData(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppNotificationProgressData {
    type Vtable = IAppNotificationProgressData_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IAppNotificationProgressData {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x4debfac0_809d_55cb_b879_924821876b97);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppNotificationProgressData_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SequenceNumber: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut u32,
    ) -> ::windows_core::HRESULT,
    pub SetSequenceNumber: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: u32,
    ) -> ::windows_core::HRESULT,
    pub Title: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub SetTitle: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub Value: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub SetValue: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
    ) -> ::windows_core::HRESULT,
    pub ValueStringOverride: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub SetValueStringOverride: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub Status: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub SetStatus: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IAppNotificationProgressDataFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppNotificationProgressDataFactory {
    type Vtable = IAppNotificationProgressDataFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IAppNotificationProgressDataFactory {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xc08e4a0f_3a75_55d6_8c3e_14f03ae46046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppNotificationProgressDataFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        sequencenumber: u32,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct AppNotification(::windows_core::IUnknown);
impl AppNotification {
    pub fn Tag(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Tag)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetTag(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetTag)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn Group(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Group)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetGroup(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetGroup)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn Id(&self) -> ::windows_core::Result<u32> {
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
    pub fn Payload(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Payload)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn Progress(&self) -> ::windows_core::Result<AppNotificationProgressData> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Progress)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetProgress<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<AppNotificationProgressData>,
    {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetProgress)(
                ::windows_core::Interface::as_raw(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn Expiration(&self) -> ::windows_core::Result<::windows::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Expiration)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn SetExpiration(
        &self,
        value: ::windows::Foundation::DateTime,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetExpiration)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn ExpiresOnReboot(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ExpiresOnReboot)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetExpiresOnReboot(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetExpiresOnReboot)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn Priority(&self) -> ::windows_core::Result<AppNotificationPriority> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Priority)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetPriority(&self, value: AppNotificationPriority) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetPriority)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn SuppressDisplay(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SuppressDisplay)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetSuppressDisplay(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetSuppressDisplay)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn CreateInstance(
        payload: &::windows_core::HSTRING,
    ) -> ::windows_core::Result<AppNotification> {
        Self::IAppNotificationFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstance)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(payload),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IAppNotificationFactory<
        R,
        F: FnOnce(&IAppNotificationFactory) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<AppNotification, IAppNotificationFactory> =
            ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for AppNotification {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for AppNotification {
    type Vtable = IAppNotification_Vtbl;
}
unsafe impl ::windows_core::ComInterface for AppNotification {
    const IID: ::windows_core::GUID = <IAppNotification as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for AppNotification {
    const NAME: &'static str = "Microsoft.Windows.AppNotifications.AppNotification";
}
::windows_core::imp::interface_hierarchy!(
    AppNotification,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for AppNotification {}
unsafe impl ::core::marker::Sync for AppNotification {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct AppNotificationActivatedEventArgs(::windows_core::IUnknown);
impl AppNotificationActivatedEventArgs {
    pub fn Argument(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Argument)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation_Collections\"`"]
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub fn UserInput(
        &self,
    ) -> ::windows_core::Result<
        ::windows::Foundation::Collections::IMap<::windows_core::HSTRING, ::windows_core::HSTRING>,
    > {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).UserInput)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation_Collections\"`"]
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub fn Arguments(
        &self,
    ) -> ::windows_core::Result<
        ::windows::Foundation::Collections::IMap<::windows_core::HSTRING, ::windows_core::HSTRING>,
    > {
        let this =
            &::windows_core::ComInterface::cast::<IAppNotificationActivatedEventArgs2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Arguments)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for AppNotificationActivatedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for AppNotificationActivatedEventArgs {
    type Vtable = IAppNotificationActivatedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for AppNotificationActivatedEventArgs {
    const IID: ::windows_core::GUID =
        <IAppNotificationActivatedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for AppNotificationActivatedEventArgs {
    const NAME: &'static str =
        "Microsoft.Windows.AppNotifications.AppNotificationActivatedEventArgs";
}
::windows_core::imp::interface_hierarchy!(
    AppNotificationActivatedEventArgs,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for AppNotificationActivatedEventArgs {}
unsafe impl ::core::marker::Sync for AppNotificationActivatedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct AppNotificationManager(::windows_core::IUnknown);
impl AppNotificationManager {
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
    pub fn NotificationInvoked<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<
                AppNotificationManager,
                AppNotificationActivatedEventArgs,
            >,
        >,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NotificationInvoked)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveNotificationInvoked(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveNotificationInvoked)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn Show<P0>(&self, notification: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<AppNotification>,
    {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).Show)(
                ::windows_core::Interface::as_raw(this),
                notification.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn UpdateAsync<P0>(
        &self,
        data: P0,
        tag: &::windows_core::HSTRING,
        group: &::windows_core::HSTRING,
    ) -> ::windows_core::Result<::windows::Foundation::IAsyncOperation<AppNotificationProgressResult>>
    where
        P0: ::windows_core::IntoParam<AppNotificationProgressData>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).UpdateAsync)(
                ::windows_core::Interface::as_raw(this),
                data.into_param().abi(),
                ::core::mem::transmute_copy(tag),
                ::core::mem::transmute_copy(group),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn UpdateAsync2<P0>(
        &self,
        data: P0,
        tag: &::windows_core::HSTRING,
    ) -> ::windows_core::Result<::windows::Foundation::IAsyncOperation<AppNotificationProgressResult>>
    where
        P0: ::windows_core::IntoParam<AppNotificationProgressData>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).UpdateAsync2)(
                ::windows_core::Interface::as_raw(this),
                data.into_param().abi(),
                ::core::mem::transmute_copy(tag),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn Setting(&self) -> ::windows_core::Result<AppNotificationSetting> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Setting)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveByIdAsync(
        &self,
        notificationid: u32,
    ) -> ::windows_core::Result<::windows::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RemoveByIdAsync)(
                ::windows_core::Interface::as_raw(this),
                notificationid,
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveByTagAsync(
        &self,
        tag: &::windows_core::HSTRING,
    ) -> ::windows_core::Result<::windows::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RemoveByTagAsync)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(tag),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveByTagAndGroupAsync(
        &self,
        tag: &::windows_core::HSTRING,
        group: &::windows_core::HSTRING,
    ) -> ::windows_core::Result<::windows::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RemoveByTagAndGroupAsync)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(tag),
                ::core::mem::transmute_copy(group),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveByGroupAsync(
        &self,
        group: &::windows_core::HSTRING,
    ) -> ::windows_core::Result<::windows::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RemoveByGroupAsync)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(group),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveAllAsync(&self) -> ::windows_core::Result<::windows::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RemoveAllAsync)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation_Collections\"`"]
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub fn GetAllAsync(
        &self,
    ) -> ::windows_core::Result<
        ::windows::Foundation::IAsyncOperation<
            ::windows::Foundation::Collections::IVector<AppNotification>,
        >,
    > {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetAllAsync)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn Register2<P0>(
        &self,
        displayname: &::windows_core::HSTRING,
        iconuri: P0,
    ) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows::Foundation::Uri>,
    {
        let this = &::windows_core::ComInterface::cast::<IAppNotificationManager2>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).Register)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(displayname),
                iconuri.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn Default() -> ::windows_core::Result<AppNotificationManager> {
        Self::IAppNotificationManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Default)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn IsSupported() -> ::windows_core::Result<bool> {
        Self::IAppNotificationManagerStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsSupported)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IAppNotificationManagerStatics<
        R,
        F: FnOnce(&IAppNotificationManagerStatics) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<
            AppNotificationManager,
            IAppNotificationManagerStatics,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IAppNotificationManagerStatics2<
        R,
        F: FnOnce(&IAppNotificationManagerStatics2) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<
            AppNotificationManager,
            IAppNotificationManagerStatics2,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for AppNotificationManager {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for AppNotificationManager {
    type Vtable = IAppNotificationManager_Vtbl;
}
unsafe impl ::windows_core::ComInterface for AppNotificationManager {
    const IID: ::windows_core::GUID =
        <IAppNotificationManager as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for AppNotificationManager {
    const NAME: &'static str = "Microsoft.Windows.AppNotifications.AppNotificationManager";
}
::windows_core::imp::interface_hierarchy!(
    AppNotificationManager,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for AppNotificationManager {}
unsafe impl ::core::marker::Sync for AppNotificationManager {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct AppNotificationProgressData(::windows_core::IUnknown);
impl AppNotificationProgressData {
    pub fn SequenceNumber(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SequenceNumber)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetSequenceNumber(&self, value: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetSequenceNumber)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn Title(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Title)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetTitle(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetTitle)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn Value(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Value)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetValue(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetValue)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn ValueStringOverride(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ValueStringOverride)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetValueStringOverride(
        &self,
        value: &::windows_core::HSTRING,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetValueStringOverride)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn Status(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
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
    pub fn SetStatus(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetStatus)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn CreateInstance(
        sequencenumber: u32,
    ) -> ::windows_core::Result<AppNotificationProgressData> {
        Self::IAppNotificationProgressDataFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstance)(
                ::windows_core::Interface::as_raw(this),
                sequencenumber,
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IAppNotificationProgressDataFactory<
        R,
        F: FnOnce(&IAppNotificationProgressDataFactory) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<
            AppNotificationProgressData,
            IAppNotificationProgressDataFactory,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for AppNotificationProgressData {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for AppNotificationProgressData {
    type Vtable = IAppNotificationProgressData_Vtbl;
}
unsafe impl ::windows_core::ComInterface for AppNotificationProgressData {
    const IID: ::windows_core::GUID =
        <IAppNotificationProgressData as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for AppNotificationProgressData {
    const NAME: &'static str = "Microsoft.Windows.AppNotifications.AppNotificationProgressData";
}
::windows_core::imp::interface_hierarchy!(
    AppNotificationProgressData,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for AppNotificationProgressData {}
unsafe impl ::core::marker::Sync for AppNotificationProgressData {}
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
impl ::windows_core::TypeKind for AppNotificationPriority {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for AppNotificationPriority {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppNotificationPriority").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for AppNotificationPriority {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::from_slice(
            b"enum(Microsoft.Windows.AppNotifications.AppNotificationPriority;i4)",
        );
}
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
impl ::windows_core::TypeKind for AppNotificationProgressResult {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for AppNotificationProgressResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppNotificationProgressResult").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for AppNotificationProgressResult {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::from_slice(
            b"enum(Microsoft.Windows.AppNotifications.AppNotificationProgressResult;i4)",
        );
}
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
impl ::windows_core::TypeKind for AppNotificationSetting {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for AppNotificationSetting {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppNotificationSetting").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for AppNotificationSetting {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::from_slice(
            b"enum(Microsoft.Windows.AppNotifications.AppNotificationSetting;i4)",
        );
}
