#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IAppNotificationBuilder(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppNotificationBuilder {
    type Vtable = IAppNotificationBuilder_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IAppNotificationBuilder {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xe801d31f_ce03_505c_adec_8a02724ec9de);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppNotificationBuilder_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub AddArgument: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        key: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        value: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_Foundation")]
    pub SetTimeStamp: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::windows::Foundation::DateTime,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    SetTimeStamp: usize,
    pub SetDuration: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        duration: AppNotificationDuration,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetScenario: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: AppNotificationScenario,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub AddText: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        text: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub AddText2: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        text: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        properties: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetAttributionText: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        text: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetAttributionText2: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        text: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        language: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_Foundation")]
    pub SetInlineImage: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        imageuri: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    SetInlineImage: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub SetInlineImage2: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        imageuri: *mut ::core::ffi::c_void,
        imagecrop: AppNotificationImageCrop,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    SetInlineImage2: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub SetInlineImage3: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        imageuri: *mut ::core::ffi::c_void,
        imagecrop: AppNotificationImageCrop,
        alternatetext: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    SetInlineImage3: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub SetAppLogoOverride: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        imageuri: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    SetAppLogoOverride: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub SetAppLogoOverride2: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        imageuri: *mut ::core::ffi::c_void,
        imagecrop: AppNotificationImageCrop,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    SetAppLogoOverride2: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub SetAppLogoOverride3: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        imageuri: *mut ::core::ffi::c_void,
        imagecrop: AppNotificationImageCrop,
        alternatetext: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    SetAppLogoOverride3: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub SetHeroImage: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        imageuri: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    SetHeroImage: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub SetHeroImage2: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        imageuri: *mut ::core::ffi::c_void,
        alternatetext: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    SetHeroImage2: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub SetAudioUri: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        audiouri: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    SetAudioUri: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub SetAudioUri2: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        audiouri: *mut ::core::ffi::c_void,
        r#loop: AppNotificationAudioLooping,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    SetAudioUri2: usize,
    pub SetAudioEvent: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        appnotificationsoundevent: AppNotificationSoundEvent,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetAudioEvent2: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        appnotificationsoundevent: AppNotificationSoundEvent,
        r#loop: AppNotificationAudioLooping,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub MuteAudio: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub AddTextBox: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        id: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub AddTextBox2: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        id: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        placeholdertext: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        title: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub AddButton: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub AddComboBox: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub AddProgressBar: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub BuildNotification: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetTag: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetGroup: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        group: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IAppNotificationBuilderStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppNotificationBuilderStatics {
    type Vtable = IAppNotificationBuilderStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IAppNotificationBuilderStatics {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xc7042d2a_d319_520e_a314_50081c8888cc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppNotificationBuilderStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub IsUrgentScenarioSupported: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IAppNotificationButton(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppNotificationButton {
    type Vtable = IAppNotificationButton_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IAppNotificationButton {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xa7c03031_5634_5098_aec9_47ecb60c3499);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppNotificationButton_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Content: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub SetContent: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub Arguments: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation_Collections"))]
    Arguments: usize,
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub SetArguments: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation_Collections"))]
    SetArguments: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub Icon: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    Icon: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub SetIcon: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    SetIcon: usize,
    pub ToolTip: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub SetToolTip: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub ContextMenuPlacement: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetContextMenuPlacement: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
    pub ButtonStyle: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut AppNotificationButtonStyle,
    ) -> ::windows_core::HRESULT,
    pub SetButtonStyle: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: AppNotificationButtonStyle,
    ) -> ::windows_core::HRESULT,
    pub InputId: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub SetInputId: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_Foundation")]
    pub InvokeUri: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    InvokeUri: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub SetInvokeUri: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    SetInvokeUri: usize,
    pub TargetAppId: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub SetTargetAppId: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub AddArgument: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        key: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        value: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_Foundation")]
    pub SetIcon2: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    SetIcon2: usize,
    pub SetToolTip2: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetContextMenuPlacement2: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetButtonStyle2: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: AppNotificationButtonStyle,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetInputId2: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_Foundation")]
    pub SetInvokeUri2: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        protocoluri: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    SetInvokeUri2: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub SetInvokeUri2: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        protocoluri: *mut ::core::ffi::c_void,
        targetappid: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    SetInvokeUri2: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IAppNotificationButtonFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppNotificationButtonFactory {
    type Vtable = IAppNotificationButtonFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IAppNotificationButtonFactory {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x4f109286_0a6d_5a5e_9e8f_9fe31669fbb8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppNotificationButtonFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        content: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IAppNotificationButtonStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppNotificationButtonStatics {
    type Vtable = IAppNotificationButtonStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IAppNotificationButtonStatics {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xfead7c18_4144_59a4_9611_86b7e8191853);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppNotificationButtonStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub IsToolTipSupported: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub IsButtonStyleSupported: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IAppNotificationComboBox(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppNotificationComboBox {
    type Vtable = IAppNotificationComboBox_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IAppNotificationComboBox {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x4547c9e2_4815_538c_be26_040ce17f8b62);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppNotificationComboBox_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub Items: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation_Collections"))]
    Items: usize,
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub SetItems: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation_Collections"))]
    SetItems: usize,
    pub Title: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub SetTitle: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub SelectedItem: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub SetSelectedItem: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub AddItem: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        id: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        content: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetTitle2: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetSelectedItem2: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        id: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IAppNotificationComboBoxFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppNotificationComboBoxFactory {
    type Vtable = IAppNotificationComboBoxFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IAppNotificationComboBoxFactory {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x1b31b5b0_9d86_59ed_8629_a79498ab5d4b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppNotificationComboBoxFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        id: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IAppNotificationProgressBar(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppNotificationProgressBar {
    type Vtable = IAppNotificationProgressBar_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IAppNotificationProgressBar {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xfced62f2_2074_5641_8630_87a14315ac86);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppNotificationProgressBar_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Title: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub SetTitle: unsafe extern "system" fn(
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
    pub SetTitle2: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub BindTitle: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetStatus2: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub BindStatus: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetValue2: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub BindValue: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetValueStringOverride2: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub BindValueStringOverride: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IAppNotificationTextProperties(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppNotificationTextProperties {
    type Vtable = IAppNotificationTextProperties_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IAppNotificationTextProperties {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x23a30d0b_5258_5853_932e_9521a3642afb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppNotificationTextProperties_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Language: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub SetLanguage: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub IncomingCallAlignment: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetIncomingCallAlignment: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
    pub MaxLines: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut i32,
    ) -> ::windows_core::HRESULT,
    pub SetMaxLines: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: i32,
    ) -> ::windows_core::HRESULT,
    pub SetLanguage2: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetIncomingCallAlignment2: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetMaxLines2: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: i32,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct AppNotificationBuilder(::windows_core::IUnknown);
impl AppNotificationBuilder {
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
            AppNotificationBuilder,
            ::windows_core::imp::IGenericFactory,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn AddArgument(
        &self,
        key: &::windows_core::HSTRING,
        value: &::windows_core::HSTRING,
    ) -> ::windows_core::Result<AppNotificationBuilder> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AddArgument)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(key),
                ::core::mem::transmute_copy(value),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn SetTimeStamp(
        &self,
        value: ::windows::Foundation::DateTime,
    ) -> ::windows_core::Result<AppNotificationBuilder> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SetTimeStamp)(
                ::windows_core::Interface::as_raw(this),
                value,
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetDuration(
        &self,
        duration: AppNotificationDuration,
    ) -> ::windows_core::Result<AppNotificationBuilder> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SetDuration)(
                ::windows_core::Interface::as_raw(this),
                duration,
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetScenario(
        &self,
        value: AppNotificationScenario,
    ) -> ::windows_core::Result<AppNotificationBuilder> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SetScenario)(
                ::windows_core::Interface::as_raw(this),
                value,
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn AddText(
        &self,
        text: &::windows_core::HSTRING,
    ) -> ::windows_core::Result<AppNotificationBuilder> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AddText)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(text),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn AddText2<P0>(
        &self,
        text: &::windows_core::HSTRING,
        properties: P0,
    ) -> ::windows_core::Result<AppNotificationBuilder>
    where
        P0: ::windows_core::IntoParam<AppNotificationTextProperties>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AddText2)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(text),
                properties.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetAttributionText(
        &self,
        text: &::windows_core::HSTRING,
    ) -> ::windows_core::Result<AppNotificationBuilder> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SetAttributionText)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(text),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetAttributionText2(
        &self,
        text: &::windows_core::HSTRING,
        language: &::windows_core::HSTRING,
    ) -> ::windows_core::Result<AppNotificationBuilder> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SetAttributionText2)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(text),
                ::core::mem::transmute_copy(language),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn SetInlineImage<P0>(&self, imageuri: P0) -> ::windows_core::Result<AppNotificationBuilder>
    where
        P0: ::windows_core::IntoParam<::windows::Foundation::Uri>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SetInlineImage)(
                ::windows_core::Interface::as_raw(this),
                imageuri.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn SetInlineImage2<P0>(
        &self,
        imageuri: P0,
        imagecrop: AppNotificationImageCrop,
    ) -> ::windows_core::Result<AppNotificationBuilder>
    where
        P0: ::windows_core::IntoParam<::windows::Foundation::Uri>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SetInlineImage2)(
                ::windows_core::Interface::as_raw(this),
                imageuri.into_param().abi(),
                imagecrop,
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn SetInlineImage3<P0>(
        &self,
        imageuri: P0,
        imagecrop: AppNotificationImageCrop,
        alternatetext: &::windows_core::HSTRING,
    ) -> ::windows_core::Result<AppNotificationBuilder>
    where
        P0: ::windows_core::IntoParam<::windows::Foundation::Uri>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SetInlineImage3)(
                ::windows_core::Interface::as_raw(this),
                imageuri.into_param().abi(),
                imagecrop,
                ::core::mem::transmute_copy(alternatetext),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn SetAppLogoOverride<P0>(
        &self,
        imageuri: P0,
    ) -> ::windows_core::Result<AppNotificationBuilder>
    where
        P0: ::windows_core::IntoParam<::windows::Foundation::Uri>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SetAppLogoOverride)(
                ::windows_core::Interface::as_raw(this),
                imageuri.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn SetAppLogoOverride2<P0>(
        &self,
        imageuri: P0,
        imagecrop: AppNotificationImageCrop,
    ) -> ::windows_core::Result<AppNotificationBuilder>
    where
        P0: ::windows_core::IntoParam<::windows::Foundation::Uri>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SetAppLogoOverride2)(
                ::windows_core::Interface::as_raw(this),
                imageuri.into_param().abi(),
                imagecrop,
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn SetAppLogoOverride3<P0>(
        &self,
        imageuri: P0,
        imagecrop: AppNotificationImageCrop,
        alternatetext: &::windows_core::HSTRING,
    ) -> ::windows_core::Result<AppNotificationBuilder>
    where
        P0: ::windows_core::IntoParam<::windows::Foundation::Uri>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SetAppLogoOverride3)(
                ::windows_core::Interface::as_raw(this),
                imageuri.into_param().abi(),
                imagecrop,
                ::core::mem::transmute_copy(alternatetext),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn SetHeroImage<P0>(&self, imageuri: P0) -> ::windows_core::Result<AppNotificationBuilder>
    where
        P0: ::windows_core::IntoParam<::windows::Foundation::Uri>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SetHeroImage)(
                ::windows_core::Interface::as_raw(this),
                imageuri.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn SetHeroImage2<P0>(
        &self,
        imageuri: P0,
        alternatetext: &::windows_core::HSTRING,
    ) -> ::windows_core::Result<AppNotificationBuilder>
    where
        P0: ::windows_core::IntoParam<::windows::Foundation::Uri>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SetHeroImage2)(
                ::windows_core::Interface::as_raw(this),
                imageuri.into_param().abi(),
                ::core::mem::transmute_copy(alternatetext),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn SetAudioUri<P0>(&self, audiouri: P0) -> ::windows_core::Result<AppNotificationBuilder>
    where
        P0: ::windows_core::IntoParam<::windows::Foundation::Uri>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SetAudioUri)(
                ::windows_core::Interface::as_raw(this),
                audiouri.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn SetAudioUri2<P0>(
        &self,
        audiouri: P0,
        r#loop: AppNotificationAudioLooping,
    ) -> ::windows_core::Result<AppNotificationBuilder>
    where
        P0: ::windows_core::IntoParam<::windows::Foundation::Uri>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SetAudioUri2)(
                ::windows_core::Interface::as_raw(this),
                audiouri.into_param().abi(),
                r#loop,
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetAudioEvent(
        &self,
        appnotificationsoundevent: AppNotificationSoundEvent,
    ) -> ::windows_core::Result<AppNotificationBuilder> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SetAudioEvent)(
                ::windows_core::Interface::as_raw(this),
                appnotificationsoundevent,
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetAudioEvent2(
        &self,
        appnotificationsoundevent: AppNotificationSoundEvent,
        r#loop: AppNotificationAudioLooping,
    ) -> ::windows_core::Result<AppNotificationBuilder> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SetAudioEvent2)(
                ::windows_core::Interface::as_raw(this),
                appnotificationsoundevent,
                r#loop,
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn MuteAudio(&self) -> ::windows_core::Result<AppNotificationBuilder> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MuteAudio)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn AddTextBox(
        &self,
        id: &::windows_core::HSTRING,
    ) -> ::windows_core::Result<AppNotificationBuilder> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AddTextBox)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(id),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn AddTextBox2(
        &self,
        id: &::windows_core::HSTRING,
        placeholdertext: &::windows_core::HSTRING,
        title: &::windows_core::HSTRING,
    ) -> ::windows_core::Result<AppNotificationBuilder> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AddTextBox2)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(id),
                ::core::mem::transmute_copy(placeholdertext),
                ::core::mem::transmute_copy(title),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn AddButton<P0>(&self, value: P0) -> ::windows_core::Result<AppNotificationBuilder>
    where
        P0: ::windows_core::IntoParam<AppNotificationButton>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AddButton)(
                ::windows_core::Interface::as_raw(this),
                value.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn AddComboBox<P0>(&self, value: P0) -> ::windows_core::Result<AppNotificationBuilder>
    where
        P0: ::windows_core::IntoParam<AppNotificationComboBox>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AddComboBox)(
                ::windows_core::Interface::as_raw(this),
                value.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn AddProgressBar<P0>(&self, value: P0) -> ::windows_core::Result<AppNotificationBuilder>
    where
        P0: ::windows_core::IntoParam<AppNotificationProgressBar>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AddProgressBar)(
                ::windows_core::Interface::as_raw(this),
                value.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn BuildNotification(&self) -> ::windows_core::Result<super::AppNotification> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).BuildNotification)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetTag(
        &self,
        value: &::windows_core::HSTRING,
    ) -> ::windows_core::Result<AppNotificationBuilder> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SetTag)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(value),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetGroup(
        &self,
        group: &::windows_core::HSTRING,
    ) -> ::windows_core::Result<AppNotificationBuilder> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SetGroup)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(group),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn IsUrgentScenarioSupported() -> ::windows_core::Result<bool> {
        Self::IAppNotificationBuilderStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsUrgentScenarioSupported)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IAppNotificationBuilderStatics<
        R,
        F: FnOnce(&IAppNotificationBuilderStatics) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<
            AppNotificationBuilder,
            IAppNotificationBuilderStatics,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for AppNotificationBuilder {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for AppNotificationBuilder {
    type Vtable = IAppNotificationBuilder_Vtbl;
}
unsafe impl ::windows_core::ComInterface for AppNotificationBuilder {
    const IID: ::windows_core::GUID =
        <IAppNotificationBuilder as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for AppNotificationBuilder {
    const NAME: &'static str = "Microsoft.Windows.AppNotifications.Builder.AppNotificationBuilder";
}
::windows_core::imp::interface_hierarchy!(
    AppNotificationBuilder,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for AppNotificationBuilder {}
unsafe impl ::core::marker::Sync for AppNotificationBuilder {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct AppNotificationButton(::windows_core::IUnknown);
impl AppNotificationButton {
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
            AppNotificationButton,
            ::windows_core::imp::IGenericFactory,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Content(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Content)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetContent(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetContent)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation_Collections\"`"]
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub fn Arguments(
        &self,
    ) -> ::windows_core::Result<
        ::windows::Foundation::Collections::IMap<::windows_core::HSTRING, ::windows_core::HSTRING>,
    > {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Arguments)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation_Collections\"`"]
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub fn SetArguments<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<
            ::windows::Foundation::Collections::IMap<
                ::windows_core::HSTRING,
                ::windows_core::HSTRING,
            >,
        >,
    {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetArguments)(
                ::windows_core::Interface::as_raw(this),
                value.try_into_param()?.abi(),
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn Icon(&self) -> ::windows_core::Result<::windows::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Icon)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn SetIcon<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows::Foundation::Uri>,
    {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetIcon)(
                ::windows_core::Interface::as_raw(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn ToolTip(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ToolTip)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetToolTip(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetToolTip)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn ContextMenuPlacement(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ContextMenuPlacement)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetContextMenuPlacement(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetContextMenuPlacement)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn ButtonStyle(&self) -> ::windows_core::Result<AppNotificationButtonStyle> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ButtonStyle)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetButtonStyle(&self, value: AppNotificationButtonStyle) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetButtonStyle)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn InputId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).InputId)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetInputId(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetInputId)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn InvokeUri(&self) -> ::windows_core::Result<::windows::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).InvokeUri)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn SetInvokeUri<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows::Foundation::Uri>,
    {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetInvokeUri)(
                ::windows_core::Interface::as_raw(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn TargetAppId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TargetAppId)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetTargetAppId(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetTargetAppId)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn AddArgument(
        &self,
        key: &::windows_core::HSTRING,
        value: &::windows_core::HSTRING,
    ) -> ::windows_core::Result<AppNotificationButton> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AddArgument)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(key),
                ::core::mem::transmute_copy(value),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn SetIcon2<P0>(&self, value: P0) -> ::windows_core::Result<AppNotificationButton>
    where
        P0: ::windows_core::IntoParam<::windows::Foundation::Uri>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SetIcon2)(
                ::windows_core::Interface::as_raw(this),
                value.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetToolTip2(
        &self,
        value: &::windows_core::HSTRING,
    ) -> ::windows_core::Result<AppNotificationButton> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SetToolTip2)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(value),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetContextMenuPlacement2(&self) -> ::windows_core::Result<AppNotificationButton> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SetContextMenuPlacement2)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetButtonStyle2(
        &self,
        value: AppNotificationButtonStyle,
    ) -> ::windows_core::Result<AppNotificationButton> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SetButtonStyle2)(
                ::windows_core::Interface::as_raw(this),
                value,
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetInputId2(
        &self,
        value: &::windows_core::HSTRING,
    ) -> ::windows_core::Result<AppNotificationButton> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SetInputId2)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(value),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn SetInvokeUri2<P0>(
        &self,
        protocoluri: P0,
    ) -> ::windows_core::Result<AppNotificationButton>
    where
        P0: ::windows_core::IntoParam<::windows::Foundation::Uri>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SetInvokeUri2)(
                ::windows_core::Interface::as_raw(this),
                protocoluri.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn SetInvokeUri2<P0>(
        &self,
        protocoluri: P0,
        targetappid: &::windows_core::HSTRING,
    ) -> ::windows_core::Result<AppNotificationButton>
    where
        P0: ::windows_core::IntoParam<::windows::Foundation::Uri>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SetInvokeUri2)(
                ::windows_core::Interface::as_raw(this),
                protocoluri.into_param().abi(),
                ::core::mem::transmute_copy(targetappid),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn CreateInstance(
        content: &::windows_core::HSTRING,
    ) -> ::windows_core::Result<AppNotificationButton> {
        Self::IAppNotificationButtonFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstance)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(content),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn IsToolTipSupported() -> ::windows_core::Result<bool> {
        Self::IAppNotificationButtonStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsToolTipSupported)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn IsButtonStyleSupported() -> ::windows_core::Result<bool> {
        Self::IAppNotificationButtonStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsButtonStyleSupported)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IAppNotificationButtonFactory<
        R,
        F: FnOnce(&IAppNotificationButtonFactory) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<
            AppNotificationButton,
            IAppNotificationButtonFactory,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IAppNotificationButtonStatics<
        R,
        F: FnOnce(&IAppNotificationButtonStatics) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<
            AppNotificationButton,
            IAppNotificationButtonStatics,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for AppNotificationButton {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for AppNotificationButton {
    type Vtable = IAppNotificationButton_Vtbl;
}
unsafe impl ::windows_core::ComInterface for AppNotificationButton {
    const IID: ::windows_core::GUID = <IAppNotificationButton as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for AppNotificationButton {
    const NAME: &'static str = "Microsoft.Windows.AppNotifications.Builder.AppNotificationButton";
}
::windows_core::imp::interface_hierarchy!(
    AppNotificationButton,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for AppNotificationButton {}
unsafe impl ::core::marker::Sync for AppNotificationButton {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct AppNotificationComboBox(::windows_core::IUnknown);
impl AppNotificationComboBox {
    #[doc = "Required features: `\"Windows_Foundation_Collections\"`"]
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub fn Items(
        &self,
    ) -> ::windows_core::Result<
        ::windows::Foundation::Collections::IMap<::windows_core::HSTRING, ::windows_core::HSTRING>,
    > {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Items)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation_Collections\"`"]
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub fn SetItems<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<
            ::windows::Foundation::Collections::IMap<
                ::windows_core::HSTRING,
                ::windows_core::HSTRING,
            >,
        >,
    {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetItems)(
                ::windows_core::Interface::as_raw(this),
                value.try_into_param()?.abi(),
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
    pub fn SelectedItem(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SelectedItem)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetSelectedItem(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetSelectedItem)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn AddItem(
        &self,
        id: &::windows_core::HSTRING,
        content: &::windows_core::HSTRING,
    ) -> ::windows_core::Result<AppNotificationComboBox> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AddItem)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(id),
                ::core::mem::transmute_copy(content),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetTitle2(
        &self,
        value: &::windows_core::HSTRING,
    ) -> ::windows_core::Result<AppNotificationComboBox> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SetTitle2)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(value),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetSelectedItem2(
        &self,
        id: &::windows_core::HSTRING,
    ) -> ::windows_core::Result<AppNotificationComboBox> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SetSelectedItem2)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(id),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn CreateInstance(
        id: &::windows_core::HSTRING,
    ) -> ::windows_core::Result<AppNotificationComboBox> {
        Self::IAppNotificationComboBoxFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstance)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(id),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IAppNotificationComboBoxFactory<
        R,
        F: FnOnce(&IAppNotificationComboBoxFactory) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<
            AppNotificationComboBox,
            IAppNotificationComboBoxFactory,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for AppNotificationComboBox {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for AppNotificationComboBox {
    type Vtable = IAppNotificationComboBox_Vtbl;
}
unsafe impl ::windows_core::ComInterface for AppNotificationComboBox {
    const IID: ::windows_core::GUID =
        <IAppNotificationComboBox as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for AppNotificationComboBox {
    const NAME: &'static str = "Microsoft.Windows.AppNotifications.Builder.AppNotificationComboBox";
}
::windows_core::imp::interface_hierarchy!(
    AppNotificationComboBox,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for AppNotificationComboBox {}
unsafe impl ::core::marker::Sync for AppNotificationComboBox {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct AppNotificationProgressBar(::windows_core::IUnknown);
impl AppNotificationProgressBar {
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
            AppNotificationProgressBar,
            ::windows_core::imp::IGenericFactory,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
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
    pub fn SetTitle2(
        &self,
        value: &::windows_core::HSTRING,
    ) -> ::windows_core::Result<AppNotificationProgressBar> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SetTitle2)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(value),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn BindTitle(&self) -> ::windows_core::Result<AppNotificationProgressBar> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).BindTitle)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetStatus2(
        &self,
        value: &::windows_core::HSTRING,
    ) -> ::windows_core::Result<AppNotificationProgressBar> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SetStatus2)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(value),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn BindStatus(&self) -> ::windows_core::Result<AppNotificationProgressBar> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).BindStatus)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetValue2(&self, value: f64) -> ::windows_core::Result<AppNotificationProgressBar> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SetValue2)(
                ::windows_core::Interface::as_raw(this),
                value,
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn BindValue(&self) -> ::windows_core::Result<AppNotificationProgressBar> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).BindValue)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetValueStringOverride2(
        &self,
        value: &::windows_core::HSTRING,
    ) -> ::windows_core::Result<AppNotificationProgressBar> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SetValueStringOverride2)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(value),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn BindValueStringOverride(&self) -> ::windows_core::Result<AppNotificationProgressBar> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).BindValueStringOverride)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for AppNotificationProgressBar {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for AppNotificationProgressBar {
    type Vtable = IAppNotificationProgressBar_Vtbl;
}
unsafe impl ::windows_core::ComInterface for AppNotificationProgressBar {
    const IID: ::windows_core::GUID =
        <IAppNotificationProgressBar as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for AppNotificationProgressBar {
    const NAME: &'static str =
        "Microsoft.Windows.AppNotifications.Builder.AppNotificationProgressBar";
}
::windows_core::imp::interface_hierarchy!(
    AppNotificationProgressBar,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for AppNotificationProgressBar {}
unsafe impl ::core::marker::Sync for AppNotificationProgressBar {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct AppNotificationTextProperties(::windows_core::IUnknown);
impl AppNotificationTextProperties {
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
            AppNotificationTextProperties,
            ::windows_core::imp::IGenericFactory,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Language(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Language)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetLanguage(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetLanguage)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn IncomingCallAlignment(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IncomingCallAlignment)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetIncomingCallAlignment(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetIncomingCallAlignment)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn MaxLines(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MaxLines)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetMaxLines(&self, value: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetMaxLines)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn SetLanguage2(
        &self,
        value: &::windows_core::HSTRING,
    ) -> ::windows_core::Result<AppNotificationTextProperties> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SetLanguage2)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(value),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetIncomingCallAlignment2(
        &self,
    ) -> ::windows_core::Result<AppNotificationTextProperties> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SetIncomingCallAlignment2)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetMaxLines2(
        &self,
        value: i32,
    ) -> ::windows_core::Result<AppNotificationTextProperties> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SetMaxLines2)(
                ::windows_core::Interface::as_raw(this),
                value,
                &mut result__,
            )
            .from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for AppNotificationTextProperties {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for AppNotificationTextProperties {
    type Vtable = IAppNotificationTextProperties_Vtbl;
}
unsafe impl ::windows_core::ComInterface for AppNotificationTextProperties {
    const IID: ::windows_core::GUID =
        <IAppNotificationTextProperties as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for AppNotificationTextProperties {
    const NAME: &'static str =
        "Microsoft.Windows.AppNotifications.Builder.AppNotificationTextProperties";
}
::windows_core::imp::interface_hierarchy!(
    AppNotificationTextProperties,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for AppNotificationTextProperties {}
unsafe impl ::core::marker::Sync for AppNotificationTextProperties {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AppNotificationAudioLooping(pub i32);
impl AppNotificationAudioLooping {
    pub const None: Self = Self(0i32);
    pub const Loop: Self = Self(1i32);
}
impl ::core::marker::Copy for AppNotificationAudioLooping {}
impl ::core::clone::Clone for AppNotificationAudioLooping {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AppNotificationAudioLooping {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for AppNotificationAudioLooping {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for AppNotificationAudioLooping {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppNotificationAudioLooping").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for AppNotificationAudioLooping {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::from_slice(
            b"enum(Microsoft.Windows.AppNotifications.Builder.AppNotificationAudioLooping;i4)",
        );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AppNotificationButtonStyle(pub i32);
impl AppNotificationButtonStyle {
    pub const Default: Self = Self(0i32);
    pub const Success: Self = Self(1i32);
    pub const Critical: Self = Self(2i32);
}
impl ::core::marker::Copy for AppNotificationButtonStyle {}
impl ::core::clone::Clone for AppNotificationButtonStyle {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AppNotificationButtonStyle {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for AppNotificationButtonStyle {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for AppNotificationButtonStyle {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppNotificationButtonStyle").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for AppNotificationButtonStyle {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::from_slice(
            b"enum(Microsoft.Windows.AppNotifications.Builder.AppNotificationButtonStyle;i4)",
        );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AppNotificationDuration(pub i32);
impl AppNotificationDuration {
    pub const Default: Self = Self(0i32);
    pub const Long: Self = Self(1i32);
}
impl ::core::marker::Copy for AppNotificationDuration {}
impl ::core::clone::Clone for AppNotificationDuration {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AppNotificationDuration {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for AppNotificationDuration {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for AppNotificationDuration {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppNotificationDuration").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for AppNotificationDuration {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::from_slice(
            b"enum(Microsoft.Windows.AppNotifications.Builder.AppNotificationDuration;i4)",
        );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AppNotificationImageCrop(pub i32);
impl AppNotificationImageCrop {
    pub const Default: Self = Self(0i32);
    pub const Circle: Self = Self(1i32);
}
impl ::core::marker::Copy for AppNotificationImageCrop {}
impl ::core::clone::Clone for AppNotificationImageCrop {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AppNotificationImageCrop {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for AppNotificationImageCrop {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for AppNotificationImageCrop {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppNotificationImageCrop").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for AppNotificationImageCrop {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::from_slice(
            b"enum(Microsoft.Windows.AppNotifications.Builder.AppNotificationImageCrop;i4)",
        );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AppNotificationScenario(pub i32);
impl AppNotificationScenario {
    pub const Default: Self = Self(0i32);
    pub const Reminder: Self = Self(1i32);
    pub const Alarm: Self = Self(2i32);
    pub const IncomingCall: Self = Self(3i32);
    pub const Urgent: Self = Self(4i32);
}
impl ::core::marker::Copy for AppNotificationScenario {}
impl ::core::clone::Clone for AppNotificationScenario {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AppNotificationScenario {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for AppNotificationScenario {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for AppNotificationScenario {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppNotificationScenario").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for AppNotificationScenario {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::from_slice(
            b"enum(Microsoft.Windows.AppNotifications.Builder.AppNotificationScenario;i4)",
        );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AppNotificationSoundEvent(pub i32);
impl AppNotificationSoundEvent {
    pub const Default: Self = Self(0i32);
    pub const IM: Self = Self(1i32);
    pub const Mail: Self = Self(2i32);
    pub const Reminder: Self = Self(3i32);
    pub const SMS: Self = Self(4i32);
    pub const Alarm: Self = Self(5i32);
    pub const Alarm2: Self = Self(6i32);
    pub const Alarm3: Self = Self(7i32);
    pub const Alarm4: Self = Self(8i32);
    pub const Alarm5: Self = Self(9i32);
    pub const Alarm6: Self = Self(10i32);
    pub const Alarm7: Self = Self(11i32);
    pub const Alarm8: Self = Self(12i32);
    pub const Alarm9: Self = Self(13i32);
    pub const Alarm10: Self = Self(14i32);
    pub const Call: Self = Self(15i32);
    pub const Call2: Self = Self(16i32);
    pub const Call3: Self = Self(17i32);
    pub const Call4: Self = Self(18i32);
    pub const Call5: Self = Self(19i32);
    pub const Call6: Self = Self(20i32);
    pub const Call7: Self = Self(21i32);
    pub const Call8: Self = Self(22i32);
    pub const Call9: Self = Self(23i32);
    pub const Call10: Self = Self(24i32);
}
impl ::core::marker::Copy for AppNotificationSoundEvent {}
impl ::core::clone::Clone for AppNotificationSoundEvent {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AppNotificationSoundEvent {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for AppNotificationSoundEvent {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for AppNotificationSoundEvent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppNotificationSoundEvent").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for AppNotificationSoundEvent {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::from_slice(
            b"enum(Microsoft.Windows.AppNotifications.Builder.AppNotificationSoundEvent;i4)",
        );
}
