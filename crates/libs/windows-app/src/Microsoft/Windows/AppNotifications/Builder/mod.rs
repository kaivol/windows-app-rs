#[doc(hidden)]
#[repr(transparent)]
pub struct IAppNotificationBuilder(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IAppNotificationBuilder {
    type Vtable = IAppNotificationBuilder_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppNotificationBuilder {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xe801d31f_ce03_505c_adec_8a02724ec9de);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppNotificationBuilder_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub AddArgument: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        key: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub SetTimeStamp: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::windows::Foundation::DateTime,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub SetDuration: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        duration: AppNotificationDuration,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub SetScenario: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: AppNotificationScenario,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub AddText: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        text: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub AddText2: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        text: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        properties: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub SetAttributionText: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        text: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub SetAttributionText2: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        text: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        language: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub SetInlineImage: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        imageuri: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub SetInlineImage2: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        imageuri: *mut ::core::ffi::c_void,
        imagecrop: AppNotificationImageCrop,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub SetInlineImage3: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        imageuri: *mut ::core::ffi::c_void,
        imagecrop: AppNotificationImageCrop,
        alternatetext: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub SetAppLogoOverride: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        imageuri: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub SetAppLogoOverride2: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        imageuri: *mut ::core::ffi::c_void,
        imagecrop: AppNotificationImageCrop,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub SetAppLogoOverride3: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        imageuri: *mut ::core::ffi::c_void,
        imagecrop: AppNotificationImageCrop,
        alternatetext: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub SetHeroImage: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        imageuri: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub SetHeroImage2: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        imageuri: *mut ::core::ffi::c_void,
        alternatetext: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub SetAudioUri: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        audiouri: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub SetAudioUri2: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        audiouri: *mut ::core::ffi::c_void,
        r#loop: AppNotificationAudioLooping,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub SetAudioEvent: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        appnotificationsoundevent: AppNotificationSoundEvent,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub SetAudioEvent2: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        appnotificationsoundevent: AppNotificationSoundEvent,
        r#loop: AppNotificationAudioLooping,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub MuteAudio: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub AddTextBox: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        id: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub AddTextBox2: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        id: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        placeholdertext: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        title: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub AddButton: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub AddComboBox: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub AddProgressBar: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub BuildNotification: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub SetTag: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub SetGroup: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        group: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppNotificationBuilderStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IAppNotificationBuilderStatics {
    type Vtable = IAppNotificationBuilderStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppNotificationBuilderStatics {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xc7042d2a_d319_520e_a314_50081c8888cc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppNotificationBuilderStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub IsUrgentScenarioSupported: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppNotificationButton(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IAppNotificationButton {
    type Vtable = IAppNotificationButton_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppNotificationButton {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xa7c03031_5634_5098_aec9_47ecb60c3499);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppNotificationButton_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Content: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub SetContent: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub Arguments: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub SetArguments: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub Icon: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub SetIcon: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub ToolTip: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub SetToolTip: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub ContextMenuPlacement: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub SetContextMenuPlacement: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows::core::HRESULT,
    pub ButtonStyle: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut AppNotificationButtonStyle,
    ) -> ::windows::core::HRESULT,
    pub SetButtonStyle: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: AppNotificationButtonStyle,
    ) -> ::windows::core::HRESULT,
    pub InputId: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub SetInputId: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub InvokeUri: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub SetInvokeUri: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub TargetAppId: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub SetTargetAppId: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub AddArgument: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        key: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub SetIcon2: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub SetToolTip2: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub SetContextMenuPlacement2: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub SetButtonStyle2: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: AppNotificationButtonStyle,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub SetInputId2: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub SetInvokeUri2: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        protocoluri: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub SetInvokeUri3: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        protocoluri: *mut ::core::ffi::c_void,
        targetappid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppNotificationButtonFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IAppNotificationButtonFactory {
    type Vtable = IAppNotificationButtonFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppNotificationButtonFactory {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x4f109286_0a6d_5a5e_9e8f_9fe31669fbb8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppNotificationButtonFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        content: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppNotificationButtonStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IAppNotificationButtonStatics {
    type Vtable = IAppNotificationButtonStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppNotificationButtonStatics {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xfead7c18_4144_59a4_9611_86b7e8191853);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppNotificationButtonStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub IsToolTipSupported: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub IsButtonStyleSupported: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppNotificationComboBox(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IAppNotificationComboBox {
    type Vtable = IAppNotificationComboBox_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppNotificationComboBox {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x4547c9e2_4815_538c_be26_040ce17f8b62);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppNotificationComboBox_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Items: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub SetItems: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub Title: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub SetTitle: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub SelectedItem: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub SetSelectedItem: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub AddItem: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        id: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        content: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub SetTitle2: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub SetSelectedItem2: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        id: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppNotificationComboBoxFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IAppNotificationComboBoxFactory {
    type Vtable = IAppNotificationComboBoxFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppNotificationComboBoxFactory {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x1b31b5b0_9d86_59ed_8629_a79498ab5d4b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppNotificationComboBoxFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        id: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppNotificationProgressBar(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IAppNotificationProgressBar {
    type Vtable = IAppNotificationProgressBar_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppNotificationProgressBar {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xfced62f2_2074_5641_8630_87a14315ac86);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppNotificationProgressBar_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Title: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub SetTitle: unsafe extern "system" fn(
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
    pub SetTitle2: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub BindTitle: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub SetStatus2: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub BindStatus: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub SetValue2: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub BindValue: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub SetValueStringOverride2: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub BindValueStringOverride: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppNotificationTextProperties(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IAppNotificationTextProperties {
    type Vtable = IAppNotificationTextProperties_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppNotificationTextProperties {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x23a30d0b_5258_5853_932e_9521a3642afb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppNotificationTextProperties_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Language: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub SetLanguage: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub IncomingCallAlignment: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub SetIncomingCallAlignment: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows::core::HRESULT,
    pub MaxLines: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut i32,
    ) -> ::windows::core::HRESULT,
    pub SetMaxLines: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: i32,
    ) -> ::windows::core::HRESULT,
    pub SetLanguage2: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub SetIncomingCallAlignment2: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub SetMaxLines2: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: i32,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Windows_AppNotifications_Builder\"`*"]
#[repr(transparent)]
pub struct AppNotificationBuilder(::windows::core::IUnknown);
impl AppNotificationBuilder {
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
            AppNotificationBuilder,
            ::windows::core::IGenericFactory,
        > = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn AddArgument(
        &self,
        key: &::windows::core::HSTRING,
        value: &::windows::core::HSTRING,
    ) -> ::windows::core::Result<AppNotificationBuilder> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AddArgument)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(key),
                ::core::mem::transmute_copy(value),
                result__.as_mut_ptr(),
            )
            .from_abi::<AppNotificationBuilder>(result__)
        }
    }
    pub fn SetTimeStamp(
        &self,
        value: ::windows::Foundation::DateTime,
    ) -> ::windows::core::Result<AppNotificationBuilder> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SetTimeStamp)(
                ::windows::core::Vtable::as_raw(this),
                value,
                result__.as_mut_ptr(),
            )
            .from_abi::<AppNotificationBuilder>(result__)
        }
    }
    pub fn SetDuration(
        &self,
        duration: AppNotificationDuration,
    ) -> ::windows::core::Result<AppNotificationBuilder> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SetDuration)(
                ::windows::core::Vtable::as_raw(this),
                duration,
                result__.as_mut_ptr(),
            )
            .from_abi::<AppNotificationBuilder>(result__)
        }
    }
    pub fn SetScenario(
        &self,
        value: AppNotificationScenario,
    ) -> ::windows::core::Result<AppNotificationBuilder> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SetScenario)(
                ::windows::core::Vtable::as_raw(this),
                value,
                result__.as_mut_ptr(),
            )
            .from_abi::<AppNotificationBuilder>(result__)
        }
    }
    pub fn AddText(
        &self,
        text: &::windows::core::HSTRING,
    ) -> ::windows::core::Result<AppNotificationBuilder> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AddText)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(text),
                result__.as_mut_ptr(),
            )
            .from_abi::<AppNotificationBuilder>(result__)
        }
    }
    pub fn AddText2(
        &self,
        text: &::windows::core::HSTRING,
        properties: &AppNotificationTextProperties,
    ) -> ::windows::core::Result<AppNotificationBuilder> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AddText2)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(text),
                ::core::mem::transmute_copy(properties),
                result__.as_mut_ptr(),
            )
            .from_abi::<AppNotificationBuilder>(result__)
        }
    }
    pub fn SetAttributionText(
        &self,
        text: &::windows::core::HSTRING,
    ) -> ::windows::core::Result<AppNotificationBuilder> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SetAttributionText)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(text),
                result__.as_mut_ptr(),
            )
            .from_abi::<AppNotificationBuilder>(result__)
        }
    }
    pub fn SetAttributionText2(
        &self,
        text: &::windows::core::HSTRING,
        language: &::windows::core::HSTRING,
    ) -> ::windows::core::Result<AppNotificationBuilder> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SetAttributionText2)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(text),
                ::core::mem::transmute_copy(language),
                result__.as_mut_ptr(),
            )
            .from_abi::<AppNotificationBuilder>(result__)
        }
    }
    pub fn SetInlineImage(
        &self,
        imageuri: &::windows::Foundation::Uri,
    ) -> ::windows::core::Result<AppNotificationBuilder> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SetInlineImage)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(imageuri),
                result__.as_mut_ptr(),
            )
            .from_abi::<AppNotificationBuilder>(result__)
        }
    }
    pub fn SetInlineImage2(
        &self,
        imageuri: &::windows::Foundation::Uri,
        imagecrop: AppNotificationImageCrop,
    ) -> ::windows::core::Result<AppNotificationBuilder> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SetInlineImage2)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(imageuri),
                imagecrop,
                result__.as_mut_ptr(),
            )
            .from_abi::<AppNotificationBuilder>(result__)
        }
    }
    pub fn SetInlineImage3(
        &self,
        imageuri: &::windows::Foundation::Uri,
        imagecrop: AppNotificationImageCrop,
        alternatetext: &::windows::core::HSTRING,
    ) -> ::windows::core::Result<AppNotificationBuilder> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SetInlineImage3)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(imageuri),
                imagecrop,
                ::core::mem::transmute_copy(alternatetext),
                result__.as_mut_ptr(),
            )
            .from_abi::<AppNotificationBuilder>(result__)
        }
    }
    pub fn SetAppLogoOverride(
        &self,
        imageuri: &::windows::Foundation::Uri,
    ) -> ::windows::core::Result<AppNotificationBuilder> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SetAppLogoOverride)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(imageuri),
                result__.as_mut_ptr(),
            )
            .from_abi::<AppNotificationBuilder>(result__)
        }
    }
    pub fn SetAppLogoOverride2(
        &self,
        imageuri: &::windows::Foundation::Uri,
        imagecrop: AppNotificationImageCrop,
    ) -> ::windows::core::Result<AppNotificationBuilder> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SetAppLogoOverride2)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(imageuri),
                imagecrop,
                result__.as_mut_ptr(),
            )
            .from_abi::<AppNotificationBuilder>(result__)
        }
    }
    pub fn SetAppLogoOverride3(
        &self,
        imageuri: &::windows::Foundation::Uri,
        imagecrop: AppNotificationImageCrop,
        alternatetext: &::windows::core::HSTRING,
    ) -> ::windows::core::Result<AppNotificationBuilder> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SetAppLogoOverride3)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(imageuri),
                imagecrop,
                ::core::mem::transmute_copy(alternatetext),
                result__.as_mut_ptr(),
            )
            .from_abi::<AppNotificationBuilder>(result__)
        }
    }
    pub fn SetHeroImage(
        &self,
        imageuri: &::windows::Foundation::Uri,
    ) -> ::windows::core::Result<AppNotificationBuilder> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SetHeroImage)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(imageuri),
                result__.as_mut_ptr(),
            )
            .from_abi::<AppNotificationBuilder>(result__)
        }
    }
    pub fn SetHeroImage2(
        &self,
        imageuri: &::windows::Foundation::Uri,
        alternatetext: &::windows::core::HSTRING,
    ) -> ::windows::core::Result<AppNotificationBuilder> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SetHeroImage2)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(imageuri),
                ::core::mem::transmute_copy(alternatetext),
                result__.as_mut_ptr(),
            )
            .from_abi::<AppNotificationBuilder>(result__)
        }
    }
    pub fn SetAudioUri(
        &self,
        audiouri: &::windows::Foundation::Uri,
    ) -> ::windows::core::Result<AppNotificationBuilder> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SetAudioUri)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(audiouri),
                result__.as_mut_ptr(),
            )
            .from_abi::<AppNotificationBuilder>(result__)
        }
    }
    pub fn SetAudioUri2(
        &self,
        audiouri: &::windows::Foundation::Uri,
        r#loop: AppNotificationAudioLooping,
    ) -> ::windows::core::Result<AppNotificationBuilder> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SetAudioUri2)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(audiouri),
                r#loop,
                result__.as_mut_ptr(),
            )
            .from_abi::<AppNotificationBuilder>(result__)
        }
    }
    pub fn SetAudioEvent(
        &self,
        appnotificationsoundevent: AppNotificationSoundEvent,
    ) -> ::windows::core::Result<AppNotificationBuilder> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SetAudioEvent)(
                ::windows::core::Vtable::as_raw(this),
                appnotificationsoundevent,
                result__.as_mut_ptr(),
            )
            .from_abi::<AppNotificationBuilder>(result__)
        }
    }
    pub fn SetAudioEvent2(
        &self,
        appnotificationsoundevent: AppNotificationSoundEvent,
        r#loop: AppNotificationAudioLooping,
    ) -> ::windows::core::Result<AppNotificationBuilder> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SetAudioEvent2)(
                ::windows::core::Vtable::as_raw(this),
                appnotificationsoundevent,
                r#loop,
                result__.as_mut_ptr(),
            )
            .from_abi::<AppNotificationBuilder>(result__)
        }
    }
    pub fn MuteAudio(&self) -> ::windows::core::Result<AppNotificationBuilder> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MuteAudio)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<AppNotificationBuilder>(result__)
        }
    }
    pub fn AddTextBox(
        &self,
        id: &::windows::core::HSTRING,
    ) -> ::windows::core::Result<AppNotificationBuilder> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AddTextBox)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(id),
                result__.as_mut_ptr(),
            )
            .from_abi::<AppNotificationBuilder>(result__)
        }
    }
    pub fn AddTextBox2(
        &self,
        id: &::windows::core::HSTRING,
        placeholdertext: &::windows::core::HSTRING,
        title: &::windows::core::HSTRING,
    ) -> ::windows::core::Result<AppNotificationBuilder> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AddTextBox2)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(id),
                ::core::mem::transmute_copy(placeholdertext),
                ::core::mem::transmute_copy(title),
                result__.as_mut_ptr(),
            )
            .from_abi::<AppNotificationBuilder>(result__)
        }
    }
    pub fn AddButton(
        &self,
        value: &AppNotificationButton,
    ) -> ::windows::core::Result<AppNotificationBuilder> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AddButton)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(value),
                result__.as_mut_ptr(),
            )
            .from_abi::<AppNotificationBuilder>(result__)
        }
    }
    pub fn AddComboBox(
        &self,
        value: &AppNotificationComboBox,
    ) -> ::windows::core::Result<AppNotificationBuilder> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AddComboBox)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(value),
                result__.as_mut_ptr(),
            )
            .from_abi::<AppNotificationBuilder>(result__)
        }
    }
    pub fn AddProgressBar(
        &self,
        value: &AppNotificationProgressBar,
    ) -> ::windows::core::Result<AppNotificationBuilder> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AddProgressBar)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(value),
                result__.as_mut_ptr(),
            )
            .from_abi::<AppNotificationBuilder>(result__)
        }
    }
    pub fn BuildNotification(&self) -> ::windows::core::Result<super::AppNotification> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).BuildNotification)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::AppNotification>(result__)
        }
    }
    pub fn SetTag(
        &self,
        value: &::windows::core::HSTRING,
    ) -> ::windows::core::Result<AppNotificationBuilder> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SetTag)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(value),
                result__.as_mut_ptr(),
            )
            .from_abi::<AppNotificationBuilder>(result__)
        }
    }
    pub fn SetGroup(
        &self,
        group: &::windows::core::HSTRING,
    ) -> ::windows::core::Result<AppNotificationBuilder> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SetGroup)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(group),
                result__.as_mut_ptr(),
            )
            .from_abi::<AppNotificationBuilder>(result__)
        }
    }
    pub fn IsUrgentScenarioSupported() -> ::windows::core::Result<bool> {
        Self::IAppNotificationBuilderStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsUrgentScenarioSupported)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IAppNotificationBuilderStatics<
        R,
        F: FnOnce(&IAppNotificationBuilderStatics) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<
            AppNotificationBuilder,
            IAppNotificationBuilderStatics,
        > = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for AppNotificationBuilder {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppNotificationBuilder {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppNotificationBuilder {}
impl ::core::fmt::Debug for AppNotificationBuilder {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppNotificationBuilder").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppNotificationBuilder {
    const SIGNATURE : ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice ( b"rc(Microsoft.Windows.AppNotifications.Builder.AppNotificationBuilder;{e801d31f-ce03-505c-adec-8a02724ec9de})" ) ;
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for AppNotificationBuilder {
    type Vtable = IAppNotificationBuilder_Vtbl;
}
unsafe impl ::windows::core::Interface for AppNotificationBuilder {
    const IID: ::windows::core::GUID = <IAppNotificationBuilder as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppNotificationBuilder {
    const NAME: &'static str = "Microsoft.Windows.AppNotifications.Builder.AppNotificationBuilder";
}
::windows::core::interface_hierarchy!(
    AppNotificationBuilder,
    ::windows::core::IUnknown,
    ::windows::core::IInspectable
);
unsafe impl ::core::marker::Send for AppNotificationBuilder {}
unsafe impl ::core::marker::Sync for AppNotificationBuilder {}
#[doc = "*Required features: `\"Windows_AppNotifications_Builder\"`*"]
#[repr(transparent)]
pub struct AppNotificationButton(::windows::core::IUnknown);
impl AppNotificationButton {
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
            AppNotificationButton,
            ::windows::core::IGenericFactory,
        > = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Content(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Content)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetContent(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetContent)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
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
        let this = self;
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
    pub fn SetArguments<'a, P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<
                'a,
                ::windows::Foundation::Collections::IMap<
                    ::windows::core::HSTRING,
                    ::windows::core::HSTRING,
                >,
            >,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetArguments)(
                ::windows::core::Vtable::as_raw(this),
                value.try_into().map_err(|e| e.into())?.abi(),
            )
            .ok()
        }
    }
    pub fn Icon(&self) -> ::windows::core::Result<::windows::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Icon)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::Uri>(result__)
        }
    }
    pub fn SetIcon(&self, value: &::windows::Foundation::Uri) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetIcon)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn ToolTip(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ToolTip)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetToolTip(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetToolTip)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn ContextMenuPlacement(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ContextMenuPlacement)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetContextMenuPlacement(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetContextMenuPlacement)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn ButtonStyle(&self) -> ::windows::core::Result<AppNotificationButtonStyle> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ButtonStyle)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<AppNotificationButtonStyle>(result__)
        }
    }
    pub fn SetButtonStyle(&self, value: AppNotificationButtonStyle) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetButtonStyle)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn InputId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).InputId)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetInputId(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetInputId)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn InvokeUri(&self) -> ::windows::core::Result<::windows::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).InvokeUri)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::Uri>(result__)
        }
    }
    pub fn SetInvokeUri(&self, value: &::windows::Foundation::Uri) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetInvokeUri)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn TargetAppId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TargetAppId)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetTargetAppId(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetTargetAppId)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn AddArgument(
        &self,
        key: &::windows::core::HSTRING,
        value: &::windows::core::HSTRING,
    ) -> ::windows::core::Result<AppNotificationButton> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AddArgument)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(key),
                ::core::mem::transmute_copy(value),
                result__.as_mut_ptr(),
            )
            .from_abi::<AppNotificationButton>(result__)
        }
    }
    pub fn SetIcon2(
        &self,
        value: &::windows::Foundation::Uri,
    ) -> ::windows::core::Result<AppNotificationButton> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SetIcon2)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(value),
                result__.as_mut_ptr(),
            )
            .from_abi::<AppNotificationButton>(result__)
        }
    }
    pub fn SetToolTip2(
        &self,
        value: &::windows::core::HSTRING,
    ) -> ::windows::core::Result<AppNotificationButton> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SetToolTip2)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(value),
                result__.as_mut_ptr(),
            )
            .from_abi::<AppNotificationButton>(result__)
        }
    }
    pub fn SetContextMenuPlacement2(&self) -> ::windows::core::Result<AppNotificationButton> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SetContextMenuPlacement2)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<AppNotificationButton>(result__)
        }
    }
    pub fn SetButtonStyle2(
        &self,
        value: AppNotificationButtonStyle,
    ) -> ::windows::core::Result<AppNotificationButton> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SetButtonStyle2)(
                ::windows::core::Vtable::as_raw(this),
                value,
                result__.as_mut_ptr(),
            )
            .from_abi::<AppNotificationButton>(result__)
        }
    }
    pub fn SetInputId2(
        &self,
        value: &::windows::core::HSTRING,
    ) -> ::windows::core::Result<AppNotificationButton> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SetInputId2)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(value),
                result__.as_mut_ptr(),
            )
            .from_abi::<AppNotificationButton>(result__)
        }
    }
    pub fn SetInvokeUri2(
        &self,
        protocoluri: &::windows::Foundation::Uri,
    ) -> ::windows::core::Result<AppNotificationButton> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SetInvokeUri2)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(protocoluri),
                result__.as_mut_ptr(),
            )
            .from_abi::<AppNotificationButton>(result__)
        }
    }
    pub fn SetInvokeUri3(
        &self,
        protocoluri: &::windows::Foundation::Uri,
        targetappid: &::windows::core::HSTRING,
    ) -> ::windows::core::Result<AppNotificationButton> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SetInvokeUri3)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(protocoluri),
                ::core::mem::transmute_copy(targetappid),
                result__.as_mut_ptr(),
            )
            .from_abi::<AppNotificationButton>(result__)
        }
    }
    pub fn CreateInstance(
        content: &::windows::core::HSTRING,
    ) -> ::windows::core::Result<AppNotificationButton> {
        Self::IAppNotificationButtonFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateInstance)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(content),
                result__.as_mut_ptr(),
            )
            .from_abi::<AppNotificationButton>(result__)
        })
    }
    pub fn IsToolTipSupported() -> ::windows::core::Result<bool> {
        Self::IAppNotificationButtonStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsToolTipSupported)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        })
    }
    pub fn IsButtonStyleSupported() -> ::windows::core::Result<bool> {
        Self::IAppNotificationButtonStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsButtonStyleSupported)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IAppNotificationButtonFactory<
        R,
        F: FnOnce(&IAppNotificationButtonFactory) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<
            AppNotificationButton,
            IAppNotificationButtonFactory,
        > = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IAppNotificationButtonStatics<
        R,
        F: FnOnce(&IAppNotificationButtonStatics) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<
            AppNotificationButton,
            IAppNotificationButtonStatics,
        > = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for AppNotificationButton {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppNotificationButton {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppNotificationButton {}
impl ::core::fmt::Debug for AppNotificationButton {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppNotificationButton").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppNotificationButton {
    const SIGNATURE : ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice ( b"rc(Microsoft.Windows.AppNotifications.Builder.AppNotificationButton;{a7c03031-5634-5098-aec9-47ecb60c3499})" ) ;
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for AppNotificationButton {
    type Vtable = IAppNotificationButton_Vtbl;
}
unsafe impl ::windows::core::Interface for AppNotificationButton {
    const IID: ::windows::core::GUID = <IAppNotificationButton as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppNotificationButton {
    const NAME: &'static str = "Microsoft.Windows.AppNotifications.Builder.AppNotificationButton";
}
::windows::core::interface_hierarchy!(
    AppNotificationButton,
    ::windows::core::IUnknown,
    ::windows::core::IInspectable
);
unsafe impl ::core::marker::Send for AppNotificationButton {}
unsafe impl ::core::marker::Sync for AppNotificationButton {}
#[doc = "*Required features: `\"Windows_AppNotifications_Builder\"`*"]
#[repr(transparent)]
pub struct AppNotificationComboBox(::windows::core::IUnknown);
impl AppNotificationComboBox {
    pub fn Items(
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
            (::windows::core::Vtable::vtable(this).Items)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::Collections::IMap<
                ::windows::core::HSTRING,
                ::windows::core::HSTRING,
            >>(result__)
        }
    }
    pub fn SetItems<'a, P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<
                'a,
                ::windows::Foundation::Collections::IMap<
                    ::windows::core::HSTRING,
                    ::windows::core::HSTRING,
                >,
            >,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetItems)(
                ::windows::core::Vtable::as_raw(this),
                value.try_into().map_err(|e| e.into())?.abi(),
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
    pub fn SelectedItem(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SelectedItem)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetSelectedItem(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetSelectedItem)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn AddItem(
        &self,
        id: &::windows::core::HSTRING,
        content: &::windows::core::HSTRING,
    ) -> ::windows::core::Result<AppNotificationComboBox> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AddItem)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(id),
                ::core::mem::transmute_copy(content),
                result__.as_mut_ptr(),
            )
            .from_abi::<AppNotificationComboBox>(result__)
        }
    }
    pub fn SetTitle2(
        &self,
        value: &::windows::core::HSTRING,
    ) -> ::windows::core::Result<AppNotificationComboBox> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SetTitle2)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(value),
                result__.as_mut_ptr(),
            )
            .from_abi::<AppNotificationComboBox>(result__)
        }
    }
    pub fn SetSelectedItem2(
        &self,
        id: &::windows::core::HSTRING,
    ) -> ::windows::core::Result<AppNotificationComboBox> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SetSelectedItem2)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(id),
                result__.as_mut_ptr(),
            )
            .from_abi::<AppNotificationComboBox>(result__)
        }
    }
    pub fn CreateInstance(
        id: &::windows::core::HSTRING,
    ) -> ::windows::core::Result<AppNotificationComboBox> {
        Self::IAppNotificationComboBoxFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateInstance)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(id),
                result__.as_mut_ptr(),
            )
            .from_abi::<AppNotificationComboBox>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IAppNotificationComboBoxFactory<
        R,
        F: FnOnce(&IAppNotificationComboBoxFactory) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<
            AppNotificationComboBox,
            IAppNotificationComboBoxFactory,
        > = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for AppNotificationComboBox {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppNotificationComboBox {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppNotificationComboBox {}
impl ::core::fmt::Debug for AppNotificationComboBox {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppNotificationComboBox").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppNotificationComboBox {
    const SIGNATURE : ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice ( b"rc(Microsoft.Windows.AppNotifications.Builder.AppNotificationComboBox;{4547c9e2-4815-538c-be26-040ce17f8b62})" ) ;
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for AppNotificationComboBox {
    type Vtable = IAppNotificationComboBox_Vtbl;
}
unsafe impl ::windows::core::Interface for AppNotificationComboBox {
    const IID: ::windows::core::GUID =
        <IAppNotificationComboBox as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppNotificationComboBox {
    const NAME: &'static str = "Microsoft.Windows.AppNotifications.Builder.AppNotificationComboBox";
}
::windows::core::interface_hierarchy!(
    AppNotificationComboBox,
    ::windows::core::IUnknown,
    ::windows::core::IInspectable
);
unsafe impl ::core::marker::Send for AppNotificationComboBox {}
unsafe impl ::core::marker::Sync for AppNotificationComboBox {}
#[doc = "*Required features: `\"Windows_AppNotifications_Builder\"`*"]
#[repr(transparent)]
pub struct AppNotificationProgressBar(::windows::core::IUnknown);
impl AppNotificationProgressBar {
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
            AppNotificationProgressBar,
            ::windows::core::IGenericFactory,
        > = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
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
    pub fn SetTitle2(
        &self,
        value: &::windows::core::HSTRING,
    ) -> ::windows::core::Result<AppNotificationProgressBar> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SetTitle2)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(value),
                result__.as_mut_ptr(),
            )
            .from_abi::<AppNotificationProgressBar>(result__)
        }
    }
    pub fn BindTitle(&self) -> ::windows::core::Result<AppNotificationProgressBar> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).BindTitle)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<AppNotificationProgressBar>(result__)
        }
    }
    pub fn SetStatus2(
        &self,
        value: &::windows::core::HSTRING,
    ) -> ::windows::core::Result<AppNotificationProgressBar> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SetStatus2)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(value),
                result__.as_mut_ptr(),
            )
            .from_abi::<AppNotificationProgressBar>(result__)
        }
    }
    pub fn BindStatus(&self) -> ::windows::core::Result<AppNotificationProgressBar> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).BindStatus)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<AppNotificationProgressBar>(result__)
        }
    }
    pub fn SetValue2(&self, value: f64) -> ::windows::core::Result<AppNotificationProgressBar> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SetValue2)(
                ::windows::core::Vtable::as_raw(this),
                value,
                result__.as_mut_ptr(),
            )
            .from_abi::<AppNotificationProgressBar>(result__)
        }
    }
    pub fn BindValue(&self) -> ::windows::core::Result<AppNotificationProgressBar> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).BindValue)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<AppNotificationProgressBar>(result__)
        }
    }
    pub fn SetValueStringOverride2(
        &self,
        value: &::windows::core::HSTRING,
    ) -> ::windows::core::Result<AppNotificationProgressBar> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SetValueStringOverride2)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(value),
                result__.as_mut_ptr(),
            )
            .from_abi::<AppNotificationProgressBar>(result__)
        }
    }
    pub fn BindValueStringOverride(&self) -> ::windows::core::Result<AppNotificationProgressBar> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).BindValueStringOverride)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<AppNotificationProgressBar>(result__)
        }
    }
}
impl ::core::clone::Clone for AppNotificationProgressBar {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppNotificationProgressBar {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppNotificationProgressBar {}
impl ::core::fmt::Debug for AppNotificationProgressBar {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppNotificationProgressBar").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppNotificationProgressBar {
    const SIGNATURE : ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice ( b"rc(Microsoft.Windows.AppNotifications.Builder.AppNotificationProgressBar;{fced62f2-2074-5641-8630-87a14315ac86})" ) ;
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for AppNotificationProgressBar {
    type Vtable = IAppNotificationProgressBar_Vtbl;
}
unsafe impl ::windows::core::Interface for AppNotificationProgressBar {
    const IID: ::windows::core::GUID =
        <IAppNotificationProgressBar as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppNotificationProgressBar {
    const NAME: &'static str =
        "Microsoft.Windows.AppNotifications.Builder.AppNotificationProgressBar";
}
::windows::core::interface_hierarchy!(
    AppNotificationProgressBar,
    ::windows::core::IUnknown,
    ::windows::core::IInspectable
);
unsafe impl ::core::marker::Send for AppNotificationProgressBar {}
unsafe impl ::core::marker::Sync for AppNotificationProgressBar {}
#[doc = "*Required features: `\"Windows_AppNotifications_Builder\"`*"]
#[repr(transparent)]
pub struct AppNotificationTextProperties(::windows::core::IUnknown);
impl AppNotificationTextProperties {
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
            AppNotificationTextProperties,
            ::windows::core::IGenericFactory,
        > = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Language(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Language)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetLanguage(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetLanguage)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn IncomingCallAlignment(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IncomingCallAlignment)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetIncomingCallAlignment(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetIncomingCallAlignment)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn MaxLines(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MaxLines)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<i32>(result__)
        }
    }
    pub fn SetMaxLines(&self, value: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetMaxLines)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn SetLanguage2(
        &self,
        value: &::windows::core::HSTRING,
    ) -> ::windows::core::Result<AppNotificationTextProperties> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SetLanguage2)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(value),
                result__.as_mut_ptr(),
            )
            .from_abi::<AppNotificationTextProperties>(result__)
        }
    }
    pub fn SetIncomingCallAlignment2(
        &self,
    ) -> ::windows::core::Result<AppNotificationTextProperties> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SetIncomingCallAlignment2)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<AppNotificationTextProperties>(result__)
        }
    }
    pub fn SetMaxLines2(
        &self,
        value: i32,
    ) -> ::windows::core::Result<AppNotificationTextProperties> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SetMaxLines2)(
                ::windows::core::Vtable::as_raw(this),
                value,
                result__.as_mut_ptr(),
            )
            .from_abi::<AppNotificationTextProperties>(result__)
        }
    }
}
impl ::core::clone::Clone for AppNotificationTextProperties {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppNotificationTextProperties {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppNotificationTextProperties {}
impl ::core::fmt::Debug for AppNotificationTextProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppNotificationTextProperties").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppNotificationTextProperties {
    const SIGNATURE : ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice ( b"rc(Microsoft.Windows.AppNotifications.Builder.AppNotificationTextProperties;{23a30d0b-5258-5853-932e-9521a3642afb})" ) ;
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for AppNotificationTextProperties {
    type Vtable = IAppNotificationTextProperties_Vtbl;
}
unsafe impl ::windows::core::Interface for AppNotificationTextProperties {
    const IID: ::windows::core::GUID =
        <IAppNotificationTextProperties as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppNotificationTextProperties {
    const NAME: &'static str =
        "Microsoft.Windows.AppNotifications.Builder.AppNotificationTextProperties";
}
::windows::core::interface_hierarchy!(
    AppNotificationTextProperties,
    ::windows::core::IUnknown,
    ::windows::core::IInspectable
);
unsafe impl ::core::marker::Send for AppNotificationTextProperties {}
unsafe impl ::core::marker::Sync for AppNotificationTextProperties {}
#[doc = "*Required features: `\"Windows_AppNotifications_Builder\"`*"]
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
unsafe impl ::windows::core::Abi for AppNotificationAudioLooping {
    type Abi = Self;
}
impl ::core::fmt::Debug for AppNotificationAudioLooping {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppNotificationAudioLooping").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppNotificationAudioLooping {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.Windows.AppNotifications.Builder.AppNotificationAudioLooping;i4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Windows_AppNotifications_Builder\"`*"]
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
unsafe impl ::windows::core::Abi for AppNotificationButtonStyle {
    type Abi = Self;
}
impl ::core::fmt::Debug for AppNotificationButtonStyle {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppNotificationButtonStyle").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppNotificationButtonStyle {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.Windows.AppNotifications.Builder.AppNotificationButtonStyle;i4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Windows_AppNotifications_Builder\"`*"]
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
unsafe impl ::windows::core::Abi for AppNotificationDuration {
    type Abi = Self;
}
impl ::core::fmt::Debug for AppNotificationDuration {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppNotificationDuration").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppNotificationDuration {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.Windows.AppNotifications.Builder.AppNotificationDuration;i4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Windows_AppNotifications_Builder\"`*"]
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
unsafe impl ::windows::core::Abi for AppNotificationImageCrop {
    type Abi = Self;
}
impl ::core::fmt::Debug for AppNotificationImageCrop {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppNotificationImageCrop").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppNotificationImageCrop {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.Windows.AppNotifications.Builder.AppNotificationImageCrop;i4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Windows_AppNotifications_Builder\"`*"]
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
unsafe impl ::windows::core::Abi for AppNotificationScenario {
    type Abi = Self;
}
impl ::core::fmt::Debug for AppNotificationScenario {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppNotificationScenario").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppNotificationScenario {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.Windows.AppNotifications.Builder.AppNotificationScenario;i4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Windows_AppNotifications_Builder\"`*"]
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
unsafe impl ::windows::core::Abi for AppNotificationSoundEvent {
    type Abi = Self;
}
impl ::core::fmt::Debug for AppNotificationSoundEvent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppNotificationSoundEvent").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppNotificationSoundEvent {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.Windows.AppNotifications.Builder.AppNotificationSoundEvent;i4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
