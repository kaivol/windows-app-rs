#[cfg(feature = "Microsoft_UI_Xaml_Automation_Peers")]
#[doc = "Required features: `\"Microsoft_UI_Xaml_Automation_Peers\"`"]
pub mod Peers;
#[cfg(feature = "Microsoft_UI_Xaml_Automation_Provider")]
#[doc = "Required features: `\"Microsoft_UI_Xaml_Automation_Provider\"`"]
pub mod Provider;
#[cfg(feature = "Microsoft_UI_Xaml_Automation_Text")]
#[doc = "Required features: `\"Microsoft_UI_Xaml_Automation_Text\"`"]
pub mod Text;
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IAnnotationPatternIdentifiers(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAnnotationPatternIdentifiers {
    type Vtable = IAnnotationPatternIdentifiers_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IAnnotationPatternIdentifiers {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x92d76915_0cd3_59cd_8ae0_c9004628ba1e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAnnotationPatternIdentifiers_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IAnnotationPatternIdentifiersStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAnnotationPatternIdentifiersStatics {
    type Vtable = IAnnotationPatternIdentifiersStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IAnnotationPatternIdentifiersStatics {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x20a136e2_4a47_5de5_9e1e_ecfc6d92f52a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAnnotationPatternIdentifiersStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub AnnotationTypeIdProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub AnnotationTypeNameProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub AuthorProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub DateTimeProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub TargetProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IAutomationAnnotation(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAutomationAnnotation {
    type Vtable = IAutomationAnnotation_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IAutomationAnnotation {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xc2cc46ad_1414_5f1b_808a_89e5d53d82fe);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAutomationAnnotation_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Type: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut AnnotationType,
    ) -> ::windows_core::HRESULT,
    pub SetType: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: AnnotationType,
    ) -> ::windows_core::HRESULT,
    pub Element: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetElement: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IAutomationAnnotationFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAutomationAnnotationFactory {
    type Vtable = IAutomationAnnotationFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IAutomationAnnotationFactory {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x95f82773_eac5_572e_87de_24d9514b9a89);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAutomationAnnotationFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        r#type: AnnotationType,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub CreateWithElementParameter: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        r#type: AnnotationType,
        element: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IAutomationAnnotationStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAutomationAnnotationStatics {
    type Vtable = IAutomationAnnotationStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IAutomationAnnotationStatics {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xc5abdc1e_fc26_5444_a8b3_59b2c0a95578);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAutomationAnnotationStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub TypeProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub ElementProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IAutomationElementIdentifiers(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAutomationElementIdentifiers {
    type Vtable = IAutomationElementIdentifiers_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IAutomationElementIdentifiers {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x2fb51a33_b0cf_5a4c_9ed3_267eca7aeefc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAutomationElementIdentifiers_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IAutomationElementIdentifiersStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAutomationElementIdentifiersStatics {
    type Vtable = IAutomationElementIdentifiersStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IAutomationElementIdentifiersStatics {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x72af6b8c_3e12_5e7a_a2ec_26dc193f9df9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAutomationElementIdentifiersStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub AcceleratorKeyProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub AccessKeyProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub AutomationIdProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub BoundingRectangleProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub ClassNameProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub ClickablePointProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub ControlTypeProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub HasKeyboardFocusProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub HelpTextProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub IsContentElementProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub IsControlElementProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub IsEnabledProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub IsKeyboardFocusableProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub IsOffscreenProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub IsPasswordProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub IsRequiredForFormProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub ItemStatusProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub ItemTypeProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub LabeledByProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub LocalizedControlTypeProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub NameProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub OrientationProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub LiveSettingProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub ControlledPeersProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub PositionInSetProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SizeOfSetProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub LevelProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub AnnotationsProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub LandmarkTypeProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub LocalizedLandmarkTypeProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub IsPeripheralProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub IsDataValidForFormProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub FullDescriptionProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub DescribedByProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub FlowsToProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub FlowsFromProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub CultureProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub HeadingLevelProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub IsDialogProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IAutomationProperties(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAutomationProperties {
    type Vtable = IAutomationProperties_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IAutomationProperties {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x525c6a71_dd8a_52a0_977b_db1b02f8e896);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAutomationProperties_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IAutomationPropertiesStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAutomationPropertiesStatics {
    type Vtable = IAutomationPropertiesStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IAutomationPropertiesStatics {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xb1e3e0f3_112f_5966_87dc_7862d4ad50e5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAutomationPropertiesStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub AcceleratorKeyProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub GetAcceleratorKey: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub SetAcceleratorKey: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        value: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub AccessKeyProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub GetAccessKey: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub SetAccessKey: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        value: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub AutomationIdProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub GetAutomationId: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub SetAutomationId: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        value: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub HelpTextProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub GetHelpText: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub SetHelpText: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        value: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub IsRequiredForFormProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub GetIsRequiredForForm: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetIsRequiredForForm: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
    pub ItemStatusProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub GetItemStatus: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub SetItemStatus: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        value: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub ItemTypeProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub GetItemType: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub SetItemType: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        value: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub LabeledByProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub GetLabeledBy: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetLabeledBy: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub NameProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub GetName: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        value: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub LiveSettingProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Microsoft_UI_Xaml_Automation_Peers")]
    pub GetLiveSetting: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        result__: *mut Peers::AutomationLiveSetting,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Microsoft_UI_Xaml_Automation_Peers"))]
    GetLiveSetting: usize,
    #[cfg(feature = "Microsoft_UI_Xaml_Automation_Peers")]
    pub SetLiveSetting: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        value: Peers::AutomationLiveSetting,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Microsoft_UI_Xaml_Automation_Peers"))]
    SetLiveSetting: usize,
    pub AccessibilityViewProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Microsoft_UI_Xaml_Automation_Peers")]
    pub GetAccessibilityView: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        result__: *mut Peers::AccessibilityView,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Microsoft_UI_Xaml_Automation_Peers"))]
    GetAccessibilityView: usize,
    #[cfg(feature = "Microsoft_UI_Xaml_Automation_Peers")]
    pub SetAccessibilityView: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        value: Peers::AccessibilityView,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Microsoft_UI_Xaml_Automation_Peers"))]
    SetAccessibilityView: usize,
    pub ControlledPeersProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub GetControlledPeers: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation_Collections"))]
    GetControlledPeers: usize,
    pub PositionInSetProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub GetPositionInSet: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        result__: *mut i32,
    ) -> ::windows_core::HRESULT,
    pub SetPositionInSet: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        value: i32,
    ) -> ::windows_core::HRESULT,
    pub SizeOfSetProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub GetSizeOfSet: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        result__: *mut i32,
    ) -> ::windows_core::HRESULT,
    pub SetSizeOfSet: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        value: i32,
    ) -> ::windows_core::HRESULT,
    pub LevelProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub GetLevel: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        result__: *mut i32,
    ) -> ::windows_core::HRESULT,
    pub SetLevel: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        value: i32,
    ) -> ::windows_core::HRESULT,
    pub AnnotationsProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub GetAnnotations: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation_Collections"))]
    GetAnnotations: usize,
    pub LandmarkTypeProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Microsoft_UI_Xaml_Automation_Peers")]
    pub GetLandmarkType: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        result__: *mut Peers::AutomationLandmarkType,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Microsoft_UI_Xaml_Automation_Peers"))]
    GetLandmarkType: usize,
    #[cfg(feature = "Microsoft_UI_Xaml_Automation_Peers")]
    pub SetLandmarkType: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        value: Peers::AutomationLandmarkType,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Microsoft_UI_Xaml_Automation_Peers"))]
    SetLandmarkType: usize,
    pub LocalizedLandmarkTypeProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub GetLocalizedLandmarkType: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub SetLocalizedLandmarkType: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        value: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub IsPeripheralProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub GetIsPeripheral: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetIsPeripheral: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
    pub IsDataValidForFormProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub GetIsDataValidForForm: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetIsDataValidForForm: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
    pub FullDescriptionProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub GetFullDescription: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub SetFullDescription: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        value: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub LocalizedControlTypeProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub GetLocalizedControlType: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub SetLocalizedControlType: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        value: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub DescribedByProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub GetDescribedBy: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation_Collections"))]
    GetDescribedBy: usize,
    pub FlowsToProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub GetFlowsTo: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation_Collections"))]
    GetFlowsTo: usize,
    pub FlowsFromProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub GetFlowsFrom: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation_Collections"))]
    GetFlowsFrom: usize,
    pub CultureProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub GetCulture: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        result__: *mut i32,
    ) -> ::windows_core::HRESULT,
    pub SetCulture: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        value: i32,
    ) -> ::windows_core::HRESULT,
    pub HeadingLevelProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Microsoft_UI_Xaml_Automation_Peers")]
    pub GetHeadingLevel: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        result__: *mut Peers::AutomationHeadingLevel,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Microsoft_UI_Xaml_Automation_Peers"))]
    GetHeadingLevel: usize,
    #[cfg(feature = "Microsoft_UI_Xaml_Automation_Peers")]
    pub SetHeadingLevel: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        value: Peers::AutomationHeadingLevel,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Microsoft_UI_Xaml_Automation_Peers"))]
    SetHeadingLevel: usize,
    pub IsDialogProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub GetIsDialog: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetIsDialog: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IAutomationPropertiesStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAutomationPropertiesStatics2 {
    type Vtable = IAutomationPropertiesStatics2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IAutomationPropertiesStatics2 {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xd933a3ed_e90a_5df0_853d_cad17a0b9ec8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAutomationPropertiesStatics2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub AutomationControlTypeProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Microsoft_UI_Xaml_Automation_Peers")]
    pub GetAutomationControlType: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        result__: *mut Peers::AutomationControlType,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Microsoft_UI_Xaml_Automation_Peers"))]
    GetAutomationControlType: usize,
    #[cfg(feature = "Microsoft_UI_Xaml_Automation_Peers")]
    pub SetAutomationControlType: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        value: Peers::AutomationControlType,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Microsoft_UI_Xaml_Automation_Peers"))]
    SetAutomationControlType: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IAutomationProperty(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAutomationProperty {
    type Vtable = IAutomationProperty_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IAutomationProperty {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x5ca6b2c8_ff86_5a41_aa18_6948fae592cf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAutomationProperty_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDockPatternIdentifiers(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDockPatternIdentifiers {
    type Vtable = IDockPatternIdentifiers_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDockPatternIdentifiers {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x75574f99_d145_547e_972b_7d879f93c03e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDockPatternIdentifiers_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDockPatternIdentifiersStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDockPatternIdentifiersStatics {
    type Vtable = IDockPatternIdentifiersStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDockPatternIdentifiersStatics {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x02d5a72c_f49d_53a9_b9fb_af2719d16ccf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDockPatternIdentifiersStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub DockPositionProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDragPatternIdentifiers(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDragPatternIdentifiers {
    type Vtable = IDragPatternIdentifiers_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDragPatternIdentifiers {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xaa2fdfd5_fb45_5d2b_8d92_a8e7b07061c2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDragPatternIdentifiers_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDragPatternIdentifiersStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDragPatternIdentifiersStatics {
    type Vtable = IDragPatternIdentifiersStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDragPatternIdentifiersStatics {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x482eee70_0bfc_5552_9e7d_8dffc526b2f7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDragPatternIdentifiersStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub DropEffectProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub DropEffectsProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub GrabbedItemsProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub IsGrabbedProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDropTargetPatternIdentifiers(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDropTargetPatternIdentifiers {
    type Vtable = IDropTargetPatternIdentifiers_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDropTargetPatternIdentifiers {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x133e8ff3_1ddd_5cbb_b908_1484d7c04da7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDropTargetPatternIdentifiers_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDropTargetPatternIdentifiersStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDropTargetPatternIdentifiersStatics {
    type Vtable = IDropTargetPatternIdentifiersStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDropTargetPatternIdentifiersStatics {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x6da6f0bd_b942_5283_be35_501ae87f88c7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDropTargetPatternIdentifiersStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub DropTargetEffectProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub DropTargetEffectsProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IExpandCollapsePatternIdentifiers(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IExpandCollapsePatternIdentifiers {
    type Vtable = IExpandCollapsePatternIdentifiers_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IExpandCollapsePatternIdentifiers {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xcec15d9f_8630_569a_86a0_524bbea618ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IExpandCollapsePatternIdentifiers_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IExpandCollapsePatternIdentifiersStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IExpandCollapsePatternIdentifiersStatics {
    type Vtable = IExpandCollapsePatternIdentifiersStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IExpandCollapsePatternIdentifiersStatics {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x283101f4_c40c_55bf_a23b_d62b73b6aa35);
}
#[repr(C)]
#[doc(hidden)]
pub struct IExpandCollapsePatternIdentifiersStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ExpandCollapseStateProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IGridItemPatternIdentifiers(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGridItemPatternIdentifiers {
    type Vtable = IGridItemPatternIdentifiers_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IGridItemPatternIdentifiers {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x93609087_1114_557d_b17b_f801e41cebbb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGridItemPatternIdentifiers_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IGridItemPatternIdentifiersStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGridItemPatternIdentifiersStatics {
    type Vtable = IGridItemPatternIdentifiersStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IGridItemPatternIdentifiersStatics {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x8072bc18_87d0_5a02_a0a1_f9aec968c0e7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGridItemPatternIdentifiersStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ColumnProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub ColumnSpanProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub ContainingGridProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub RowProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub RowSpanProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IGridPatternIdentifiers(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGridPatternIdentifiers {
    type Vtable = IGridPatternIdentifiers_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IGridPatternIdentifiers {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xe5e1e250_c37c_54a2_8c61_1d9ccd3bb39c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGridPatternIdentifiers_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IGridPatternIdentifiersStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGridPatternIdentifiersStatics {
    type Vtable = IGridPatternIdentifiersStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IGridPatternIdentifiersStatics {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xe861604c_101f_5a6d_a308_3714f510f744);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGridPatternIdentifiersStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ColumnCountProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub RowCountProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMultipleViewPatternIdentifiers(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMultipleViewPatternIdentifiers {
    type Vtable = IMultipleViewPatternIdentifiers_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMultipleViewPatternIdentifiers {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x70e4c847_2b82_5ecf_b808_e9d453c1fe53);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMultipleViewPatternIdentifiers_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMultipleViewPatternIdentifiersStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMultipleViewPatternIdentifiersStatics {
    type Vtable = IMultipleViewPatternIdentifiersStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMultipleViewPatternIdentifiersStatics {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xac71daef_d094_5c90_94af_1fa474ab45fe);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMultipleViewPatternIdentifiersStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CurrentViewProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SupportedViewsProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IRangeValuePatternIdentifiers(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRangeValuePatternIdentifiers {
    type Vtable = IRangeValuePatternIdentifiers_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IRangeValuePatternIdentifiers {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xc114db37_6a75_5ef1_a542_d3b13f92cbfe);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRangeValuePatternIdentifiers_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IRangeValuePatternIdentifiersStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRangeValuePatternIdentifiersStatics {
    type Vtable = IRangeValuePatternIdentifiersStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IRangeValuePatternIdentifiersStatics {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x0aaa9ad7_f9b8_52a1_bc96_2a97fe389ed0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRangeValuePatternIdentifiersStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub IsReadOnlyProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub LargeChangeProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub MaximumProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub MinimumProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SmallChangeProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub ValueProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IScrollPatternIdentifiers(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IScrollPatternIdentifiers {
    type Vtable = IScrollPatternIdentifiers_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IScrollPatternIdentifiers {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x04f1a4b8_edc7_55f2_96df_a9c7e809372e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IScrollPatternIdentifiers_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IScrollPatternIdentifiersStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IScrollPatternIdentifiersStatics {
    type Vtable = IScrollPatternIdentifiersStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IScrollPatternIdentifiersStatics {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x0f94f2f0_e0d2_5a24_b415_8d1506ce47aa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IScrollPatternIdentifiersStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub HorizontallyScrollableProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub HorizontalScrollPercentProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub HorizontalViewSizeProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub NoScroll: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub VerticallyScrollableProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub VerticalScrollPercentProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub VerticalViewSizeProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ISelectionItemPatternIdentifiers(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISelectionItemPatternIdentifiers {
    type Vtable = ISelectionItemPatternIdentifiers_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ISelectionItemPatternIdentifiers {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xce3a549d_a2cb_594d_a2a4_44778c09cca5);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISelectionItemPatternIdentifiers_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ISelectionItemPatternIdentifiersStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISelectionItemPatternIdentifiersStatics {
    type Vtable = ISelectionItemPatternIdentifiersStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ISelectionItemPatternIdentifiersStatics {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x2b8ead7c_4e03_5b84_9e34_8b7384cbd862);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISelectionItemPatternIdentifiersStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub IsSelectedProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SelectionContainerProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ISelectionPatternIdentifiers(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISelectionPatternIdentifiers {
    type Vtable = ISelectionPatternIdentifiers_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ISelectionPatternIdentifiers {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x401743d2_1fba_5d05_b89f_631676453237);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISelectionPatternIdentifiers_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ISelectionPatternIdentifiersStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISelectionPatternIdentifiersStatics {
    type Vtable = ISelectionPatternIdentifiersStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ISelectionPatternIdentifiersStatics {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xf3ed111b_b20a_5e5e_a232_07f607fd5c07);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISelectionPatternIdentifiersStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CanSelectMultipleProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub IsSelectionRequiredProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SelectionProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ISpreadsheetItemPatternIdentifiers(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpreadsheetItemPatternIdentifiers {
    type Vtable = ISpreadsheetItemPatternIdentifiers_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ISpreadsheetItemPatternIdentifiers {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xdca2ec46_8564_5c9c_ba90_2c08455f697b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpreadsheetItemPatternIdentifiers_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ISpreadsheetItemPatternIdentifiersStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpreadsheetItemPatternIdentifiersStatics {
    type Vtable = ISpreadsheetItemPatternIdentifiersStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ISpreadsheetItemPatternIdentifiersStatics {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x7eb10f80_8d3a_59ad_a2b9_05d8cecf18db);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpreadsheetItemPatternIdentifiersStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub FormulaProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IStylesPatternIdentifiers(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IStylesPatternIdentifiers {
    type Vtable = IStylesPatternIdentifiers_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IStylesPatternIdentifiers {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x13aeca5e_b496_5df5_aea5_330e1f0490eb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStylesPatternIdentifiers_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IStylesPatternIdentifiersStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IStylesPatternIdentifiersStatics {
    type Vtable = IStylesPatternIdentifiersStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IStylesPatternIdentifiersStatics {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xb232287a_bc4c_581e_a33c_3d6aee10d04b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStylesPatternIdentifiersStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ExtendedPropertiesProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub FillColorProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub FillPatternColorProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub FillPatternStyleProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub ShapeProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub StyleIdProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub StyleNameProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ITableItemPatternIdentifiers(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITableItemPatternIdentifiers {
    type Vtable = ITableItemPatternIdentifiers_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ITableItemPatternIdentifiers {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xb4de5d03_a5b4_5ca1_8715_16c8c6a10fcc);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITableItemPatternIdentifiers_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ITableItemPatternIdentifiersStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITableItemPatternIdentifiersStatics {
    type Vtable = ITableItemPatternIdentifiersStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ITableItemPatternIdentifiersStatics {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x81d24bd7_66fb_53ef_9b32_d00f9c240a14);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITableItemPatternIdentifiersStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ColumnHeaderItemsProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub RowHeaderItemsProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ITablePatternIdentifiers(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITablePatternIdentifiers {
    type Vtable = ITablePatternIdentifiers_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ITablePatternIdentifiers {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x3d7f9c0b_ff8f_50fa_bc01_2cc3c2e06e2c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITablePatternIdentifiers_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ITablePatternIdentifiersStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITablePatternIdentifiersStatics {
    type Vtable = ITablePatternIdentifiersStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ITablePatternIdentifiersStatics {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x3660935e_bcbb_5848_8e9a_264854f7a19a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITablePatternIdentifiersStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ColumnHeadersProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub RowHeadersProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub RowOrColumnMajorProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ITogglePatternIdentifiers(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITogglePatternIdentifiers {
    type Vtable = ITogglePatternIdentifiers_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ITogglePatternIdentifiers {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xa0d2df4c_ba59_51d9_9c01_034d7941c280);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITogglePatternIdentifiers_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ITogglePatternIdentifiersStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITogglePatternIdentifiersStatics {
    type Vtable = ITogglePatternIdentifiersStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ITogglePatternIdentifiersStatics {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x862920b5_dcb3_5691_a456_c2f15c476dfb);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITogglePatternIdentifiersStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ToggleStateProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ITransformPattern2Identifiers(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITransformPattern2Identifiers {
    type Vtable = ITransformPattern2Identifiers_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ITransformPattern2Identifiers {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x6ef7595c_db8c_51b0_878b_34b7ef12f4da);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransformPattern2Identifiers_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ITransformPattern2IdentifiersStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITransformPattern2IdentifiersStatics {
    type Vtable = ITransformPattern2IdentifiersStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ITransformPattern2IdentifiersStatics {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xd9876ff5_89ed_5333_8111_ad25a28bee8b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransformPattern2IdentifiersStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CanZoomProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub ZoomLevelProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub MaxZoomProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub MinZoomProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ITransformPatternIdentifiers(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITransformPatternIdentifiers {
    type Vtable = ITransformPatternIdentifiers_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ITransformPatternIdentifiers {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x2348187b_c50f_5a0e_bc05_305ac71b3b6b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransformPatternIdentifiers_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ITransformPatternIdentifiersStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITransformPatternIdentifiersStatics {
    type Vtable = ITransformPatternIdentifiersStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ITransformPatternIdentifiersStatics {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xcb7d84e4_5429_5188_8aa0_5f96558a8790);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransformPatternIdentifiersStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CanMoveProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub CanResizeProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub CanRotateProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IValuePatternIdentifiers(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IValuePatternIdentifiers {
    type Vtable = IValuePatternIdentifiers_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IValuePatternIdentifiers {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xfb493395_fb97_59d5_9323_4651ce964b55);
}
#[repr(C)]
#[doc(hidden)]
pub struct IValuePatternIdentifiers_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IValuePatternIdentifiersStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IValuePatternIdentifiersStatics {
    type Vtable = IValuePatternIdentifiersStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IValuePatternIdentifiersStatics {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x2019faf5_ce64_59a7_bc13_0677c3146724);
}
#[repr(C)]
#[doc(hidden)]
pub struct IValuePatternIdentifiersStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub IsReadOnlyProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub ValueProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWindowPatternIdentifiers(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWindowPatternIdentifiers {
    type Vtable = IWindowPatternIdentifiers_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWindowPatternIdentifiers {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xbec579e1_91be_5d8f_aaca_6ad8839872d2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowPatternIdentifiers_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWindowPatternIdentifiersStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWindowPatternIdentifiersStatics {
    type Vtable = IWindowPatternIdentifiersStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWindowPatternIdentifiersStatics {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x06762744_d3d7_5441_b879_373681d47f64);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowPatternIdentifiersStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CanMaximizeProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub CanMinimizeProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub IsModalProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub IsTopmostProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub WindowInteractionStateProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub WindowVisualStateProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct AnnotationPatternIdentifiers(::windows_core::IUnknown);
impl AnnotationPatternIdentifiers {
    pub fn AnnotationTypeIdProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IAnnotationPatternIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AnnotationTypeIdProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn AnnotationTypeNameProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IAnnotationPatternIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AnnotationTypeNameProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn AuthorProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IAnnotationPatternIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AuthorProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn DateTimeProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IAnnotationPatternIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DateTimeProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn TargetProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IAnnotationPatternIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TargetProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IAnnotationPatternIdentifiersStatics<
        R,
        F: FnOnce(&IAnnotationPatternIdentifiersStatics) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<
            AnnotationPatternIdentifiers,
            IAnnotationPatternIdentifiersStatics,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for AnnotationPatternIdentifiers {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for AnnotationPatternIdentifiers {
    type Vtable = IAnnotationPatternIdentifiers_Vtbl;
}
unsafe impl ::windows_core::ComInterface for AnnotationPatternIdentifiers {
    const IID: ::windows_core::GUID =
        <IAnnotationPatternIdentifiers as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for AnnotationPatternIdentifiers {
    const NAME: &'static str = "Microsoft.UI.Xaml.Automation.AnnotationPatternIdentifiers";
}
::windows_core::imp::interface_hierarchy!(
    AnnotationPatternIdentifiers,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for AnnotationPatternIdentifiers {}
unsafe impl ::core::marker::Sync for AnnotationPatternIdentifiers {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct AutomationAnnotation(::windows_core::IUnknown);
impl AutomationAnnotation {
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
            AutomationAnnotation,
            ::windows_core::imp::IGenericFactory,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Type(&self) -> ::windows_core::Result<AnnotationType> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Type)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetType(&self, value: AnnotationType) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetType)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn Element(&self) -> ::windows_core::Result<super::UIElement> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Element)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetElement<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::UIElement>,
    {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetElement)(
                ::windows_core::Interface::as_raw(this),
                value.try_into_param()?.abi(),
            )
            .ok()
        }
    }
    pub fn CreateInstance(r#type: AnnotationType) -> ::windows_core::Result<AutomationAnnotation> {
        Self::IAutomationAnnotationFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstance)(
                ::windows_core::Interface::as_raw(this),
                r#type,
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn CreateWithElementParameter<P0>(
        r#type: AnnotationType,
        element: P0,
    ) -> ::windows_core::Result<AutomationAnnotation>
    where
        P0: ::windows_core::TryIntoParam<super::UIElement>,
    {
        Self::IAutomationAnnotationFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateWithElementParameter)(
                ::windows_core::Interface::as_raw(this),
                r#type,
                element.try_into_param()?.abi(),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn TypeProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IAutomationAnnotationStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TypeProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn ElementProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IAutomationAnnotationStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ElementProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn GetValue<P0>(&self, dp: P0) -> ::windows_core::Result<::windows_core::IInspectable>
    where
        P0: ::windows_core::IntoParam<super::DependencyProperty>,
    {
        let this = &::windows_core::ComInterface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetValue)(
                ::windows_core::Interface::as_raw(this),
                dp.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetValue<P0, P1>(&self, dp: P0, value: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::DependencyProperty>,
        P1: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = &::windows_core::ComInterface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetValue)(
                ::windows_core::Interface::as_raw(this),
                dp.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn ClearValue<P0>(&self, dp: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::DependencyProperty>,
    {
        let this = &::windows_core::ComInterface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).ClearValue)(
                ::windows_core::Interface::as_raw(this),
                dp.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn ReadLocalValue<P0>(&self, dp: P0) -> ::windows_core::Result<::windows_core::IInspectable>
    where
        P0: ::windows_core::IntoParam<super::DependencyProperty>,
    {
        let this = &::windows_core::ComInterface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ReadLocalValue)(
                ::windows_core::Interface::as_raw(this),
                dp.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn GetAnimationBaseValue<P0>(
        &self,
        dp: P0,
    ) -> ::windows_core::Result<::windows_core::IInspectable>
    where
        P0: ::windows_core::IntoParam<super::DependencyProperty>,
    {
        let this = &::windows_core::ComInterface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetAnimationBaseValue)(
                ::windows_core::Interface::as_raw(this),
                dp.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn RegisterPropertyChangedCallback<P0, P1>(
        &self,
        dp: P0,
        callback: P1,
    ) -> ::windows_core::Result<i64>
    where
        P0: ::windows_core::IntoParam<super::DependencyProperty>,
        P1: ::windows_core::IntoParam<super::DependencyPropertyChangedCallback>,
    {
        let this = &::windows_core::ComInterface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RegisterPropertyChangedCallback)(
                ::windows_core::Interface::as_raw(this),
                dp.into_param().abi(),
                callback.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn UnregisterPropertyChangedCallback<P0>(
        &self,
        dp: P0,
        token: i64,
    ) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::DependencyProperty>,
    {
        let this = &::windows_core::ComInterface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).UnregisterPropertyChangedCallback)(
                ::windows_core::Interface::as_raw(this),
                dp.into_param().abi(),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_UI_Core\"`"]
    #[cfg(feature = "Windows_UI_Core")]
    pub fn Dispatcher(&self) -> ::windows_core::Result<::windows::UI::Core::CoreDispatcher> {
        let this = &::windows_core::ComInterface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Dispatcher)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Dispatching\"`"]
    #[cfg(feature = "Microsoft_UI_Dispatching")]
    pub fn DispatcherQueue(
        &self,
    ) -> ::windows_core::Result<super::super::Dispatching::DispatcherQueue> {
        let this = &::windows_core::ComInterface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DispatcherQueue)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc(hidden)]
    pub fn IAutomationAnnotationFactory<
        R,
        F: FnOnce(&IAutomationAnnotationFactory) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<
            AutomationAnnotation,
            IAutomationAnnotationFactory,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IAutomationAnnotationStatics<
        R,
        F: FnOnce(&IAutomationAnnotationStatics) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<
            AutomationAnnotation,
            IAutomationAnnotationStatics,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for AutomationAnnotation {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for AutomationAnnotation {
    type Vtable = IAutomationAnnotation_Vtbl;
}
unsafe impl ::windows_core::ComInterface for AutomationAnnotation {
    const IID: ::windows_core::GUID = <IAutomationAnnotation as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for AutomationAnnotation {
    const NAME: &'static str = "Microsoft.UI.Xaml.Automation.AutomationAnnotation";
}
::windows_core::imp::interface_hierarchy!(
    AutomationAnnotation,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<super::DependencyObject> for AutomationAnnotation {}
unsafe impl ::core::marker::Send for AutomationAnnotation {}
unsafe impl ::core::marker::Sync for AutomationAnnotation {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct AutomationElementIdentifiers(::windows_core::IUnknown);
impl AutomationElementIdentifiers {
    pub fn AcceleratorKeyProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AcceleratorKeyProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn AccessKeyProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AccessKeyProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn AutomationIdProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AutomationIdProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn BoundingRectangleProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).BoundingRectangleProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn ClassNameProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ClassNameProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn ClickablePointProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ClickablePointProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn ControlTypeProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ControlTypeProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn HasKeyboardFocusProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HasKeyboardFocusProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn HelpTextProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HelpTextProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn IsContentElementProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsContentElementProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn IsControlElementProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsControlElementProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn IsEnabledProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsEnabledProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn IsKeyboardFocusableProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsKeyboardFocusableProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn IsOffscreenProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsOffscreenProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn IsPasswordProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsPasswordProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn IsRequiredForFormProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsRequiredForFormProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn ItemStatusProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ItemStatusProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn ItemTypeProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ItemTypeProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn LabeledByProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LabeledByProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn LocalizedControlTypeProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LocalizedControlTypeProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn NameProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NameProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn OrientationProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OrientationProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn LiveSettingProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LiveSettingProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn ControlledPeersProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ControlledPeersProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn PositionInSetProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PositionInSetProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn SizeOfSetProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SizeOfSetProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn LevelProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LevelProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn AnnotationsProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AnnotationsProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn LandmarkTypeProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LandmarkTypeProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn LocalizedLandmarkTypeProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LocalizedLandmarkTypeProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn IsPeripheralProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsPeripheralProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn IsDataValidForFormProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsDataValidForFormProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn FullDescriptionProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FullDescriptionProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn DescribedByProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DescribedByProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn FlowsToProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FlowsToProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn FlowsFromProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FlowsFromProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn CultureProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CultureProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn HeadingLevelProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HeadingLevelProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn IsDialogProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsDialogProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IAutomationElementIdentifiersStatics<
        R,
        F: FnOnce(&IAutomationElementIdentifiersStatics) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<
            AutomationElementIdentifiers,
            IAutomationElementIdentifiersStatics,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for AutomationElementIdentifiers {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for AutomationElementIdentifiers {
    type Vtable = IAutomationElementIdentifiers_Vtbl;
}
unsafe impl ::windows_core::ComInterface for AutomationElementIdentifiers {
    const IID: ::windows_core::GUID =
        <IAutomationElementIdentifiers as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for AutomationElementIdentifiers {
    const NAME: &'static str = "Microsoft.UI.Xaml.Automation.AutomationElementIdentifiers";
}
::windows_core::imp::interface_hierarchy!(
    AutomationElementIdentifiers,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for AutomationElementIdentifiers {}
unsafe impl ::core::marker::Sync for AutomationElementIdentifiers {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct AutomationProperties(::windows_core::IUnknown);
impl AutomationProperties {
    pub fn AcceleratorKeyProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AcceleratorKeyProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn GetAcceleratorKey<P0>(element: P0) -> ::windows_core::Result<::windows_core::HSTRING>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetAcceleratorKey)(
                ::windows_core::Interface::as_raw(this),
                element.try_into_param()?.abi(),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn SetAcceleratorKey<P0>(
        element: P0,
        value: &::windows_core::HSTRING,
    ) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            (::windows_core::Interface::vtable(this).SetAcceleratorKey)(
                ::windows_core::Interface::as_raw(this),
                element.try_into_param()?.abi(),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        })
    }
    pub fn AccessKeyProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AccessKeyProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn GetAccessKey<P0>(element: P0) -> ::windows_core::Result<::windows_core::HSTRING>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetAccessKey)(
                ::windows_core::Interface::as_raw(this),
                element.try_into_param()?.abi(),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn SetAccessKey<P0>(
        element: P0,
        value: &::windows_core::HSTRING,
    ) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            (::windows_core::Interface::vtable(this).SetAccessKey)(
                ::windows_core::Interface::as_raw(this),
                element.try_into_param()?.abi(),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        })
    }
    pub fn AutomationIdProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AutomationIdProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn GetAutomationId<P0>(element: P0) -> ::windows_core::Result<::windows_core::HSTRING>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetAutomationId)(
                ::windows_core::Interface::as_raw(this),
                element.try_into_param()?.abi(),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn SetAutomationId<P0>(
        element: P0,
        value: &::windows_core::HSTRING,
    ) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            (::windows_core::Interface::vtable(this).SetAutomationId)(
                ::windows_core::Interface::as_raw(this),
                element.try_into_param()?.abi(),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        })
    }
    pub fn HelpTextProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HelpTextProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn GetHelpText<P0>(element: P0) -> ::windows_core::Result<::windows_core::HSTRING>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetHelpText)(
                ::windows_core::Interface::as_raw(this),
                element.try_into_param()?.abi(),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn SetHelpText<P0>(
        element: P0,
        value: &::windows_core::HSTRING,
    ) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            (::windows_core::Interface::vtable(this).SetHelpText)(
                ::windows_core::Interface::as_raw(this),
                element.try_into_param()?.abi(),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        })
    }
    pub fn IsRequiredForFormProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsRequiredForFormProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn GetIsRequiredForForm<P0>(element: P0) -> ::windows_core::Result<bool>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetIsRequiredForForm)(
                ::windows_core::Interface::as_raw(this),
                element.try_into_param()?.abi(),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn SetIsRequiredForForm<P0>(element: P0, value: bool) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            (::windows_core::Interface::vtable(this).SetIsRequiredForForm)(
                ::windows_core::Interface::as_raw(this),
                element.try_into_param()?.abi(),
                value,
            )
            .ok()
        })
    }
    pub fn ItemStatusProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ItemStatusProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn GetItemStatus<P0>(element: P0) -> ::windows_core::Result<::windows_core::HSTRING>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetItemStatus)(
                ::windows_core::Interface::as_raw(this),
                element.try_into_param()?.abi(),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn SetItemStatus<P0>(
        element: P0,
        value: &::windows_core::HSTRING,
    ) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            (::windows_core::Interface::vtable(this).SetItemStatus)(
                ::windows_core::Interface::as_raw(this),
                element.try_into_param()?.abi(),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        })
    }
    pub fn ItemTypeProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ItemTypeProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn GetItemType<P0>(element: P0) -> ::windows_core::Result<::windows_core::HSTRING>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetItemType)(
                ::windows_core::Interface::as_raw(this),
                element.try_into_param()?.abi(),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn SetItemType<P0>(
        element: P0,
        value: &::windows_core::HSTRING,
    ) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            (::windows_core::Interface::vtable(this).SetItemType)(
                ::windows_core::Interface::as_raw(this),
                element.try_into_param()?.abi(),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        })
    }
    pub fn LabeledByProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LabeledByProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn GetLabeledBy<P0>(element: P0) -> ::windows_core::Result<super::UIElement>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetLabeledBy)(
                ::windows_core::Interface::as_raw(this),
                element.try_into_param()?.abi(),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn SetLabeledBy<P0, P1>(element: P0, value: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
        P1: ::windows_core::TryIntoParam<super::UIElement>,
    {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            (::windows_core::Interface::vtable(this).SetLabeledBy)(
                ::windows_core::Interface::as_raw(this),
                element.try_into_param()?.abi(),
                value.try_into_param()?.abi(),
            )
            .ok()
        })
    }
    pub fn NameProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NameProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn GetName<P0>(element: P0) -> ::windows_core::Result<::windows_core::HSTRING>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetName)(
                ::windows_core::Interface::as_raw(this),
                element.try_into_param()?.abi(),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn SetName<P0>(element: P0, value: &::windows_core::HSTRING) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            (::windows_core::Interface::vtable(this).SetName)(
                ::windows_core::Interface::as_raw(this),
                element.try_into_param()?.abi(),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        })
    }
    pub fn LiveSettingProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LiveSettingProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Automation_Peers\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Automation_Peers")]
    pub fn GetLiveSetting<P0>(element: P0) -> ::windows_core::Result<Peers::AutomationLiveSetting>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetLiveSetting)(
                ::windows_core::Interface::as_raw(this),
                element.try_into_param()?.abi(),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Automation_Peers\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Automation_Peers")]
    pub fn SetLiveSetting<P0>(
        element: P0,
        value: Peers::AutomationLiveSetting,
    ) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            (::windows_core::Interface::vtable(this).SetLiveSetting)(
                ::windows_core::Interface::as_raw(this),
                element.try_into_param()?.abi(),
                value,
            )
            .ok()
        })
    }
    pub fn AccessibilityViewProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AccessibilityViewProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Automation_Peers\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Automation_Peers")]
    pub fn GetAccessibilityView<P0>(element: P0) -> ::windows_core::Result<Peers::AccessibilityView>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetAccessibilityView)(
                ::windows_core::Interface::as_raw(this),
                element.try_into_param()?.abi(),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Automation_Peers\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Automation_Peers")]
    pub fn SetAccessibilityView<P0>(
        element: P0,
        value: Peers::AccessibilityView,
    ) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            (::windows_core::Interface::vtable(this).SetAccessibilityView)(
                ::windows_core::Interface::as_raw(this),
                element.try_into_param()?.abi(),
                value,
            )
            .ok()
        })
    }
    pub fn ControlledPeersProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ControlledPeersProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_Foundation_Collections\"`"]
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub fn GetControlledPeers<P0>(
        element: P0,
    ) -> ::windows_core::Result<::windows::Foundation::Collections::IVector<super::UIElement>>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetControlledPeers)(
                ::windows_core::Interface::as_raw(this),
                element.try_into_param()?.abi(),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn PositionInSetProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PositionInSetProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn GetPositionInSet<P0>(element: P0) -> ::windows_core::Result<i32>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetPositionInSet)(
                ::windows_core::Interface::as_raw(this),
                element.try_into_param()?.abi(),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn SetPositionInSet<P0>(element: P0, value: i32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            (::windows_core::Interface::vtable(this).SetPositionInSet)(
                ::windows_core::Interface::as_raw(this),
                element.try_into_param()?.abi(),
                value,
            )
            .ok()
        })
    }
    pub fn SizeOfSetProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SizeOfSetProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn GetSizeOfSet<P0>(element: P0) -> ::windows_core::Result<i32>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetSizeOfSet)(
                ::windows_core::Interface::as_raw(this),
                element.try_into_param()?.abi(),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn SetSizeOfSet<P0>(element: P0, value: i32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            (::windows_core::Interface::vtable(this).SetSizeOfSet)(
                ::windows_core::Interface::as_raw(this),
                element.try_into_param()?.abi(),
                value,
            )
            .ok()
        })
    }
    pub fn LevelProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LevelProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn GetLevel<P0>(element: P0) -> ::windows_core::Result<i32>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetLevel)(
                ::windows_core::Interface::as_raw(this),
                element.try_into_param()?.abi(),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn SetLevel<P0>(element: P0, value: i32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            (::windows_core::Interface::vtable(this).SetLevel)(
                ::windows_core::Interface::as_raw(this),
                element.try_into_param()?.abi(),
                value,
            )
            .ok()
        })
    }
    pub fn AnnotationsProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AnnotationsProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_Foundation_Collections\"`"]
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub fn GetAnnotations<P0>(
        element: P0,
    ) -> ::windows_core::Result<::windows::Foundation::Collections::IVector<AutomationAnnotation>>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetAnnotations)(
                ::windows_core::Interface::as_raw(this),
                element.try_into_param()?.abi(),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn LandmarkTypeProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LandmarkTypeProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Automation_Peers\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Automation_Peers")]
    pub fn GetLandmarkType<P0>(element: P0) -> ::windows_core::Result<Peers::AutomationLandmarkType>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetLandmarkType)(
                ::windows_core::Interface::as_raw(this),
                element.try_into_param()?.abi(),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Automation_Peers\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Automation_Peers")]
    pub fn SetLandmarkType<P0>(
        element: P0,
        value: Peers::AutomationLandmarkType,
    ) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            (::windows_core::Interface::vtable(this).SetLandmarkType)(
                ::windows_core::Interface::as_raw(this),
                element.try_into_param()?.abi(),
                value,
            )
            .ok()
        })
    }
    pub fn LocalizedLandmarkTypeProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LocalizedLandmarkTypeProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn GetLocalizedLandmarkType<P0>(
        element: P0,
    ) -> ::windows_core::Result<::windows_core::HSTRING>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetLocalizedLandmarkType)(
                ::windows_core::Interface::as_raw(this),
                element.try_into_param()?.abi(),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn SetLocalizedLandmarkType<P0>(
        element: P0,
        value: &::windows_core::HSTRING,
    ) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            (::windows_core::Interface::vtable(this).SetLocalizedLandmarkType)(
                ::windows_core::Interface::as_raw(this),
                element.try_into_param()?.abi(),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        })
    }
    pub fn IsPeripheralProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsPeripheralProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn GetIsPeripheral<P0>(element: P0) -> ::windows_core::Result<bool>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetIsPeripheral)(
                ::windows_core::Interface::as_raw(this),
                element.try_into_param()?.abi(),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn SetIsPeripheral<P0>(element: P0, value: bool) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            (::windows_core::Interface::vtable(this).SetIsPeripheral)(
                ::windows_core::Interface::as_raw(this),
                element.try_into_param()?.abi(),
                value,
            )
            .ok()
        })
    }
    pub fn IsDataValidForFormProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsDataValidForFormProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn GetIsDataValidForForm<P0>(element: P0) -> ::windows_core::Result<bool>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetIsDataValidForForm)(
                ::windows_core::Interface::as_raw(this),
                element.try_into_param()?.abi(),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn SetIsDataValidForForm<P0>(element: P0, value: bool) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            (::windows_core::Interface::vtable(this).SetIsDataValidForForm)(
                ::windows_core::Interface::as_raw(this),
                element.try_into_param()?.abi(),
                value,
            )
            .ok()
        })
    }
    pub fn FullDescriptionProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FullDescriptionProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn GetFullDescription<P0>(element: P0) -> ::windows_core::Result<::windows_core::HSTRING>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetFullDescription)(
                ::windows_core::Interface::as_raw(this),
                element.try_into_param()?.abi(),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn SetFullDescription<P0>(
        element: P0,
        value: &::windows_core::HSTRING,
    ) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            (::windows_core::Interface::vtable(this).SetFullDescription)(
                ::windows_core::Interface::as_raw(this),
                element.try_into_param()?.abi(),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        })
    }
    pub fn LocalizedControlTypeProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LocalizedControlTypeProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn GetLocalizedControlType<P0>(
        element: P0,
    ) -> ::windows_core::Result<::windows_core::HSTRING>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetLocalizedControlType)(
                ::windows_core::Interface::as_raw(this),
                element.try_into_param()?.abi(),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn SetLocalizedControlType<P0>(
        element: P0,
        value: &::windows_core::HSTRING,
    ) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            (::windows_core::Interface::vtable(this).SetLocalizedControlType)(
                ::windows_core::Interface::as_raw(this),
                element.try_into_param()?.abi(),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        })
    }
    pub fn DescribedByProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DescribedByProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_Foundation_Collections\"`"]
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub fn GetDescribedBy<P0>(
        element: P0,
    ) -> ::windows_core::Result<::windows::Foundation::Collections::IVector<super::DependencyObject>>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetDescribedBy)(
                ::windows_core::Interface::as_raw(this),
                element.try_into_param()?.abi(),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn FlowsToProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FlowsToProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_Foundation_Collections\"`"]
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub fn GetFlowsTo<P0>(
        element: P0,
    ) -> ::windows_core::Result<::windows::Foundation::Collections::IVector<super::DependencyObject>>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetFlowsTo)(
                ::windows_core::Interface::as_raw(this),
                element.try_into_param()?.abi(),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn FlowsFromProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FlowsFromProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_Foundation_Collections\"`"]
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub fn GetFlowsFrom<P0>(
        element: P0,
    ) -> ::windows_core::Result<::windows::Foundation::Collections::IVector<super::DependencyObject>>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetFlowsFrom)(
                ::windows_core::Interface::as_raw(this),
                element.try_into_param()?.abi(),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn CultureProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CultureProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn GetCulture<P0>(element: P0) -> ::windows_core::Result<i32>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetCulture)(
                ::windows_core::Interface::as_raw(this),
                element.try_into_param()?.abi(),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn SetCulture<P0>(element: P0, value: i32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            (::windows_core::Interface::vtable(this).SetCulture)(
                ::windows_core::Interface::as_raw(this),
                element.try_into_param()?.abi(),
                value,
            )
            .ok()
        })
    }
    pub fn HeadingLevelProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HeadingLevelProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Automation_Peers\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Automation_Peers")]
    pub fn GetHeadingLevel<P0>(element: P0) -> ::windows_core::Result<Peers::AutomationHeadingLevel>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetHeadingLevel)(
                ::windows_core::Interface::as_raw(this),
                element.try_into_param()?.abi(),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Automation_Peers\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Automation_Peers")]
    pub fn SetHeadingLevel<P0>(
        element: P0,
        value: Peers::AutomationHeadingLevel,
    ) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            (::windows_core::Interface::vtable(this).SetHeadingLevel)(
                ::windows_core::Interface::as_raw(this),
                element.try_into_param()?.abi(),
                value,
            )
            .ok()
        })
    }
    pub fn IsDialogProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsDialogProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn GetIsDialog<P0>(element: P0) -> ::windows_core::Result<bool>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetIsDialog)(
                ::windows_core::Interface::as_raw(this),
                element.try_into_param()?.abi(),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn SetIsDialog<P0>(element: P0, value: bool) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            (::windows_core::Interface::vtable(this).SetIsDialog)(
                ::windows_core::Interface::as_raw(this),
                element.try_into_param()?.abi(),
                value,
            )
            .ok()
        })
    }
    pub fn AutomationControlTypeProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AutomationControlTypeProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Automation_Peers\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Automation_Peers")]
    pub fn GetAutomationControlType<P0>(
        element: P0,
    ) -> ::windows_core::Result<Peers::AutomationControlType>
    where
        P0: ::windows_core::TryIntoParam<super::UIElement>,
    {
        Self::IAutomationPropertiesStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetAutomationControlType)(
                ::windows_core::Interface::as_raw(this),
                element.try_into_param()?.abi(),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Automation_Peers\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Automation_Peers")]
    pub fn SetAutomationControlType<P0>(
        element: P0,
        value: Peers::AutomationControlType,
    ) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::UIElement>,
    {
        Self::IAutomationPropertiesStatics2(|this| unsafe {
            (::windows_core::Interface::vtable(this).SetAutomationControlType)(
                ::windows_core::Interface::as_raw(this),
                element.try_into_param()?.abi(),
                value,
            )
            .ok()
        })
    }
    #[doc(hidden)]
    pub fn IAutomationPropertiesStatics<
        R,
        F: FnOnce(&IAutomationPropertiesStatics) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<
            AutomationProperties,
            IAutomationPropertiesStatics,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IAutomationPropertiesStatics2<
        R,
        F: FnOnce(&IAutomationPropertiesStatics2) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<
            AutomationProperties,
            IAutomationPropertiesStatics2,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for AutomationProperties {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for AutomationProperties {
    type Vtable = IAutomationProperties_Vtbl;
}
unsafe impl ::windows_core::ComInterface for AutomationProperties {
    const IID: ::windows_core::GUID = <IAutomationProperties as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for AutomationProperties {
    const NAME: &'static str = "Microsoft.UI.Xaml.Automation.AutomationProperties";
}
::windows_core::imp::interface_hierarchy!(
    AutomationProperties,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for AutomationProperties {}
unsafe impl ::core::marker::Sync for AutomationProperties {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct AutomationProperty(::windows_core::IUnknown);
impl AutomationProperty {}
impl ::windows_core::RuntimeType for AutomationProperty {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for AutomationProperty {
    type Vtable = IAutomationProperty_Vtbl;
}
unsafe impl ::windows_core::ComInterface for AutomationProperty {
    const IID: ::windows_core::GUID = <IAutomationProperty as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for AutomationProperty {
    const NAME: &'static str = "Microsoft.UI.Xaml.Automation.AutomationProperty";
}
::windows_core::imp::interface_hierarchy!(
    AutomationProperty,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for AutomationProperty {}
unsafe impl ::core::marker::Sync for AutomationProperty {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct DockPatternIdentifiers(::windows_core::IUnknown);
impl DockPatternIdentifiers {
    pub fn DockPositionProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IDockPatternIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DockPositionProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IDockPatternIdentifiersStatics<
        R,
        F: FnOnce(&IDockPatternIdentifiersStatics) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<
            DockPatternIdentifiers,
            IDockPatternIdentifiersStatics,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for DockPatternIdentifiers {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for DockPatternIdentifiers {
    type Vtable = IDockPatternIdentifiers_Vtbl;
}
unsafe impl ::windows_core::ComInterface for DockPatternIdentifiers {
    const IID: ::windows_core::GUID =
        <IDockPatternIdentifiers as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for DockPatternIdentifiers {
    const NAME: &'static str = "Microsoft.UI.Xaml.Automation.DockPatternIdentifiers";
}
::windows_core::imp::interface_hierarchy!(
    DockPatternIdentifiers,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for DockPatternIdentifiers {}
unsafe impl ::core::marker::Sync for DockPatternIdentifiers {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct DragPatternIdentifiers(::windows_core::IUnknown);
impl DragPatternIdentifiers {
    pub fn DropEffectProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IDragPatternIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DropEffectProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn DropEffectsProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IDragPatternIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DropEffectsProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn GrabbedItemsProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IDragPatternIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GrabbedItemsProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn IsGrabbedProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IDragPatternIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsGrabbedProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IDragPatternIdentifiersStatics<
        R,
        F: FnOnce(&IDragPatternIdentifiersStatics) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<
            DragPatternIdentifiers,
            IDragPatternIdentifiersStatics,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for DragPatternIdentifiers {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for DragPatternIdentifiers {
    type Vtable = IDragPatternIdentifiers_Vtbl;
}
unsafe impl ::windows_core::ComInterface for DragPatternIdentifiers {
    const IID: ::windows_core::GUID =
        <IDragPatternIdentifiers as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for DragPatternIdentifiers {
    const NAME: &'static str = "Microsoft.UI.Xaml.Automation.DragPatternIdentifiers";
}
::windows_core::imp::interface_hierarchy!(
    DragPatternIdentifiers,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for DragPatternIdentifiers {}
unsafe impl ::core::marker::Sync for DragPatternIdentifiers {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct DropTargetPatternIdentifiers(::windows_core::IUnknown);
impl DropTargetPatternIdentifiers {
    pub fn DropTargetEffectProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IDropTargetPatternIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DropTargetEffectProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn DropTargetEffectsProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IDropTargetPatternIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DropTargetEffectsProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IDropTargetPatternIdentifiersStatics<
        R,
        F: FnOnce(&IDropTargetPatternIdentifiersStatics) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<
            DropTargetPatternIdentifiers,
            IDropTargetPatternIdentifiersStatics,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for DropTargetPatternIdentifiers {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for DropTargetPatternIdentifiers {
    type Vtable = IDropTargetPatternIdentifiers_Vtbl;
}
unsafe impl ::windows_core::ComInterface for DropTargetPatternIdentifiers {
    const IID: ::windows_core::GUID =
        <IDropTargetPatternIdentifiers as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for DropTargetPatternIdentifiers {
    const NAME: &'static str = "Microsoft.UI.Xaml.Automation.DropTargetPatternIdentifiers";
}
::windows_core::imp::interface_hierarchy!(
    DropTargetPatternIdentifiers,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for DropTargetPatternIdentifiers {}
unsafe impl ::core::marker::Sync for DropTargetPatternIdentifiers {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ExpandCollapsePatternIdentifiers(::windows_core::IUnknown);
impl ExpandCollapsePatternIdentifiers {
    pub fn ExpandCollapseStateProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IExpandCollapsePatternIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ExpandCollapseStateProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IExpandCollapsePatternIdentifiersStatics<
        R,
        F: FnOnce(&IExpandCollapsePatternIdentifiersStatics) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<
            ExpandCollapsePatternIdentifiers,
            IExpandCollapsePatternIdentifiersStatics,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for ExpandCollapsePatternIdentifiers {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for ExpandCollapsePatternIdentifiers {
    type Vtable = IExpandCollapsePatternIdentifiers_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ExpandCollapsePatternIdentifiers {
    const IID: ::windows_core::GUID =
        <IExpandCollapsePatternIdentifiers as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ExpandCollapsePatternIdentifiers {
    const NAME: &'static str = "Microsoft.UI.Xaml.Automation.ExpandCollapsePatternIdentifiers";
}
::windows_core::imp::interface_hierarchy!(
    ExpandCollapsePatternIdentifiers,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for ExpandCollapsePatternIdentifiers {}
unsafe impl ::core::marker::Sync for ExpandCollapsePatternIdentifiers {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct GridItemPatternIdentifiers(::windows_core::IUnknown);
impl GridItemPatternIdentifiers {
    pub fn ColumnProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IGridItemPatternIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ColumnProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn ColumnSpanProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IGridItemPatternIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ColumnSpanProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn ContainingGridProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IGridItemPatternIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ContainingGridProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn RowProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IGridItemPatternIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RowProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn RowSpanProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IGridItemPatternIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RowSpanProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IGridItemPatternIdentifiersStatics<
        R,
        F: FnOnce(&IGridItemPatternIdentifiersStatics) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<
            GridItemPatternIdentifiers,
            IGridItemPatternIdentifiersStatics,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for GridItemPatternIdentifiers {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for GridItemPatternIdentifiers {
    type Vtable = IGridItemPatternIdentifiers_Vtbl;
}
unsafe impl ::windows_core::ComInterface for GridItemPatternIdentifiers {
    const IID: ::windows_core::GUID =
        <IGridItemPatternIdentifiers as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for GridItemPatternIdentifiers {
    const NAME: &'static str = "Microsoft.UI.Xaml.Automation.GridItemPatternIdentifiers";
}
::windows_core::imp::interface_hierarchy!(
    GridItemPatternIdentifiers,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for GridItemPatternIdentifiers {}
unsafe impl ::core::marker::Sync for GridItemPatternIdentifiers {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct GridPatternIdentifiers(::windows_core::IUnknown);
impl GridPatternIdentifiers {
    pub fn ColumnCountProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IGridPatternIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ColumnCountProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn RowCountProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IGridPatternIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RowCountProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IGridPatternIdentifiersStatics<
        R,
        F: FnOnce(&IGridPatternIdentifiersStatics) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<
            GridPatternIdentifiers,
            IGridPatternIdentifiersStatics,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for GridPatternIdentifiers {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for GridPatternIdentifiers {
    type Vtable = IGridPatternIdentifiers_Vtbl;
}
unsafe impl ::windows_core::ComInterface for GridPatternIdentifiers {
    const IID: ::windows_core::GUID =
        <IGridPatternIdentifiers as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for GridPatternIdentifiers {
    const NAME: &'static str = "Microsoft.UI.Xaml.Automation.GridPatternIdentifiers";
}
::windows_core::imp::interface_hierarchy!(
    GridPatternIdentifiers,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for GridPatternIdentifiers {}
unsafe impl ::core::marker::Sync for GridPatternIdentifiers {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct MultipleViewPatternIdentifiers(::windows_core::IUnknown);
impl MultipleViewPatternIdentifiers {
    pub fn CurrentViewProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IMultipleViewPatternIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CurrentViewProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn SupportedViewsProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IMultipleViewPatternIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SupportedViewsProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IMultipleViewPatternIdentifiersStatics<
        R,
        F: FnOnce(&IMultipleViewPatternIdentifiersStatics) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<
            MultipleViewPatternIdentifiers,
            IMultipleViewPatternIdentifiersStatics,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for MultipleViewPatternIdentifiers {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for MultipleViewPatternIdentifiers {
    type Vtable = IMultipleViewPatternIdentifiers_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MultipleViewPatternIdentifiers {
    const IID: ::windows_core::GUID =
        <IMultipleViewPatternIdentifiers as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MultipleViewPatternIdentifiers {
    const NAME: &'static str = "Microsoft.UI.Xaml.Automation.MultipleViewPatternIdentifiers";
}
::windows_core::imp::interface_hierarchy!(
    MultipleViewPatternIdentifiers,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for MultipleViewPatternIdentifiers {}
unsafe impl ::core::marker::Sync for MultipleViewPatternIdentifiers {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct RangeValuePatternIdentifiers(::windows_core::IUnknown);
impl RangeValuePatternIdentifiers {
    pub fn IsReadOnlyProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IRangeValuePatternIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsReadOnlyProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn LargeChangeProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IRangeValuePatternIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LargeChangeProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn MaximumProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IRangeValuePatternIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MaximumProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn MinimumProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IRangeValuePatternIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MinimumProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn SmallChangeProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IRangeValuePatternIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SmallChangeProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn ValueProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IRangeValuePatternIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ValueProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IRangeValuePatternIdentifiersStatics<
        R,
        F: FnOnce(&IRangeValuePatternIdentifiersStatics) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<
            RangeValuePatternIdentifiers,
            IRangeValuePatternIdentifiersStatics,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for RangeValuePatternIdentifiers {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for RangeValuePatternIdentifiers {
    type Vtable = IRangeValuePatternIdentifiers_Vtbl;
}
unsafe impl ::windows_core::ComInterface for RangeValuePatternIdentifiers {
    const IID: ::windows_core::GUID =
        <IRangeValuePatternIdentifiers as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for RangeValuePatternIdentifiers {
    const NAME: &'static str = "Microsoft.UI.Xaml.Automation.RangeValuePatternIdentifiers";
}
::windows_core::imp::interface_hierarchy!(
    RangeValuePatternIdentifiers,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for RangeValuePatternIdentifiers {}
unsafe impl ::core::marker::Sync for RangeValuePatternIdentifiers {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ScrollPatternIdentifiers(::windows_core::IUnknown);
impl ScrollPatternIdentifiers {
    pub fn HorizontallyScrollableProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IScrollPatternIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HorizontallyScrollableProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn HorizontalScrollPercentProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IScrollPatternIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HorizontalScrollPercentProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn HorizontalViewSizeProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IScrollPatternIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HorizontalViewSizeProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn NoScroll() -> ::windows_core::Result<f64> {
        Self::IScrollPatternIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NoScroll)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn VerticallyScrollableProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IScrollPatternIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).VerticallyScrollableProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn VerticalScrollPercentProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IScrollPatternIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).VerticalScrollPercentProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn VerticalViewSizeProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IScrollPatternIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).VerticalViewSizeProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IScrollPatternIdentifiersStatics<
        R,
        F: FnOnce(&IScrollPatternIdentifiersStatics) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<
            ScrollPatternIdentifiers,
            IScrollPatternIdentifiersStatics,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for ScrollPatternIdentifiers {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for ScrollPatternIdentifiers {
    type Vtable = IScrollPatternIdentifiers_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ScrollPatternIdentifiers {
    const IID: ::windows_core::GUID =
        <IScrollPatternIdentifiers as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ScrollPatternIdentifiers {
    const NAME: &'static str = "Microsoft.UI.Xaml.Automation.ScrollPatternIdentifiers";
}
::windows_core::imp::interface_hierarchy!(
    ScrollPatternIdentifiers,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for ScrollPatternIdentifiers {}
unsafe impl ::core::marker::Sync for ScrollPatternIdentifiers {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct SelectionItemPatternIdentifiers(::windows_core::IUnknown);
impl SelectionItemPatternIdentifiers {
    pub fn IsSelectedProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::ISelectionItemPatternIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsSelectedProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn SelectionContainerProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::ISelectionItemPatternIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SelectionContainerProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ISelectionItemPatternIdentifiersStatics<
        R,
        F: FnOnce(&ISelectionItemPatternIdentifiersStatics) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<
            SelectionItemPatternIdentifiers,
            ISelectionItemPatternIdentifiersStatics,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for SelectionItemPatternIdentifiers {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for SelectionItemPatternIdentifiers {
    type Vtable = ISelectionItemPatternIdentifiers_Vtbl;
}
unsafe impl ::windows_core::ComInterface for SelectionItemPatternIdentifiers {
    const IID: ::windows_core::GUID =
        <ISelectionItemPatternIdentifiers as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for SelectionItemPatternIdentifiers {
    const NAME: &'static str = "Microsoft.UI.Xaml.Automation.SelectionItemPatternIdentifiers";
}
::windows_core::imp::interface_hierarchy!(
    SelectionItemPatternIdentifiers,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for SelectionItemPatternIdentifiers {}
unsafe impl ::core::marker::Sync for SelectionItemPatternIdentifiers {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct SelectionPatternIdentifiers(::windows_core::IUnknown);
impl SelectionPatternIdentifiers {
    pub fn CanSelectMultipleProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::ISelectionPatternIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CanSelectMultipleProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn IsSelectionRequiredProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::ISelectionPatternIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsSelectionRequiredProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn SelectionProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::ISelectionPatternIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SelectionProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ISelectionPatternIdentifiersStatics<
        R,
        F: FnOnce(&ISelectionPatternIdentifiersStatics) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<
            SelectionPatternIdentifiers,
            ISelectionPatternIdentifiersStatics,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for SelectionPatternIdentifiers {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for SelectionPatternIdentifiers {
    type Vtable = ISelectionPatternIdentifiers_Vtbl;
}
unsafe impl ::windows_core::ComInterface for SelectionPatternIdentifiers {
    const IID: ::windows_core::GUID =
        <ISelectionPatternIdentifiers as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for SelectionPatternIdentifiers {
    const NAME: &'static str = "Microsoft.UI.Xaml.Automation.SelectionPatternIdentifiers";
}
::windows_core::imp::interface_hierarchy!(
    SelectionPatternIdentifiers,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for SelectionPatternIdentifiers {}
unsafe impl ::core::marker::Sync for SelectionPatternIdentifiers {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct SpreadsheetItemPatternIdentifiers(::windows_core::IUnknown);
impl SpreadsheetItemPatternIdentifiers {
    pub fn FormulaProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::ISpreadsheetItemPatternIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FormulaProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ISpreadsheetItemPatternIdentifiersStatics<
        R,
        F: FnOnce(&ISpreadsheetItemPatternIdentifiersStatics) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<
            SpreadsheetItemPatternIdentifiers,
            ISpreadsheetItemPatternIdentifiersStatics,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for SpreadsheetItemPatternIdentifiers {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for SpreadsheetItemPatternIdentifiers {
    type Vtable = ISpreadsheetItemPatternIdentifiers_Vtbl;
}
unsafe impl ::windows_core::ComInterface for SpreadsheetItemPatternIdentifiers {
    const IID: ::windows_core::GUID =
        <ISpreadsheetItemPatternIdentifiers as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for SpreadsheetItemPatternIdentifiers {
    const NAME: &'static str = "Microsoft.UI.Xaml.Automation.SpreadsheetItemPatternIdentifiers";
}
::windows_core::imp::interface_hierarchy!(
    SpreadsheetItemPatternIdentifiers,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for SpreadsheetItemPatternIdentifiers {}
unsafe impl ::core::marker::Sync for SpreadsheetItemPatternIdentifiers {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct StylesPatternIdentifiers(::windows_core::IUnknown);
impl StylesPatternIdentifiers {
    pub fn ExtendedPropertiesProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IStylesPatternIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ExtendedPropertiesProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn FillColorProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IStylesPatternIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FillColorProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn FillPatternColorProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IStylesPatternIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FillPatternColorProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn FillPatternStyleProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IStylesPatternIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FillPatternStyleProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn ShapeProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IStylesPatternIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ShapeProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn StyleIdProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IStylesPatternIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StyleIdProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn StyleNameProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IStylesPatternIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StyleNameProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IStylesPatternIdentifiersStatics<
        R,
        F: FnOnce(&IStylesPatternIdentifiersStatics) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<
            StylesPatternIdentifiers,
            IStylesPatternIdentifiersStatics,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for StylesPatternIdentifiers {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for StylesPatternIdentifiers {
    type Vtable = IStylesPatternIdentifiers_Vtbl;
}
unsafe impl ::windows_core::ComInterface for StylesPatternIdentifiers {
    const IID: ::windows_core::GUID =
        <IStylesPatternIdentifiers as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for StylesPatternIdentifiers {
    const NAME: &'static str = "Microsoft.UI.Xaml.Automation.StylesPatternIdentifiers";
}
::windows_core::imp::interface_hierarchy!(
    StylesPatternIdentifiers,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for StylesPatternIdentifiers {}
unsafe impl ::core::marker::Sync for StylesPatternIdentifiers {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct TableItemPatternIdentifiers(::windows_core::IUnknown);
impl TableItemPatternIdentifiers {
    pub fn ColumnHeaderItemsProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::ITableItemPatternIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ColumnHeaderItemsProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn RowHeaderItemsProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::ITableItemPatternIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RowHeaderItemsProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ITableItemPatternIdentifiersStatics<
        R,
        F: FnOnce(&ITableItemPatternIdentifiersStatics) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<
            TableItemPatternIdentifiers,
            ITableItemPatternIdentifiersStatics,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for TableItemPatternIdentifiers {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for TableItemPatternIdentifiers {
    type Vtable = ITableItemPatternIdentifiers_Vtbl;
}
unsafe impl ::windows_core::ComInterface for TableItemPatternIdentifiers {
    const IID: ::windows_core::GUID =
        <ITableItemPatternIdentifiers as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for TableItemPatternIdentifiers {
    const NAME: &'static str = "Microsoft.UI.Xaml.Automation.TableItemPatternIdentifiers";
}
::windows_core::imp::interface_hierarchy!(
    TableItemPatternIdentifiers,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for TableItemPatternIdentifiers {}
unsafe impl ::core::marker::Sync for TableItemPatternIdentifiers {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct TablePatternIdentifiers(::windows_core::IUnknown);
impl TablePatternIdentifiers {
    pub fn ColumnHeadersProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::ITablePatternIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ColumnHeadersProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn RowHeadersProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::ITablePatternIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RowHeadersProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn RowOrColumnMajorProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::ITablePatternIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RowOrColumnMajorProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ITablePatternIdentifiersStatics<
        R,
        F: FnOnce(&ITablePatternIdentifiersStatics) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<
            TablePatternIdentifiers,
            ITablePatternIdentifiersStatics,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for TablePatternIdentifiers {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for TablePatternIdentifiers {
    type Vtable = ITablePatternIdentifiers_Vtbl;
}
unsafe impl ::windows_core::ComInterface for TablePatternIdentifiers {
    const IID: ::windows_core::GUID =
        <ITablePatternIdentifiers as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for TablePatternIdentifiers {
    const NAME: &'static str = "Microsoft.UI.Xaml.Automation.TablePatternIdentifiers";
}
::windows_core::imp::interface_hierarchy!(
    TablePatternIdentifiers,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for TablePatternIdentifiers {}
unsafe impl ::core::marker::Sync for TablePatternIdentifiers {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct TogglePatternIdentifiers(::windows_core::IUnknown);
impl TogglePatternIdentifiers {
    pub fn ToggleStateProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::ITogglePatternIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ToggleStateProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ITogglePatternIdentifiersStatics<
        R,
        F: FnOnce(&ITogglePatternIdentifiersStatics) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<
            TogglePatternIdentifiers,
            ITogglePatternIdentifiersStatics,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for TogglePatternIdentifiers {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for TogglePatternIdentifiers {
    type Vtable = ITogglePatternIdentifiers_Vtbl;
}
unsafe impl ::windows_core::ComInterface for TogglePatternIdentifiers {
    const IID: ::windows_core::GUID =
        <ITogglePatternIdentifiers as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for TogglePatternIdentifiers {
    const NAME: &'static str = "Microsoft.UI.Xaml.Automation.TogglePatternIdentifiers";
}
::windows_core::imp::interface_hierarchy!(
    TogglePatternIdentifiers,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for TogglePatternIdentifiers {}
unsafe impl ::core::marker::Sync for TogglePatternIdentifiers {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct TransformPattern2Identifiers(::windows_core::IUnknown);
impl TransformPattern2Identifiers {
    pub fn CanZoomProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::ITransformPattern2IdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CanZoomProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn ZoomLevelProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::ITransformPattern2IdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ZoomLevelProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn MaxZoomProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::ITransformPattern2IdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MaxZoomProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn MinZoomProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::ITransformPattern2IdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MinZoomProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ITransformPattern2IdentifiersStatics<
        R,
        F: FnOnce(&ITransformPattern2IdentifiersStatics) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<
            TransformPattern2Identifiers,
            ITransformPattern2IdentifiersStatics,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for TransformPattern2Identifiers {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for TransformPattern2Identifiers {
    type Vtable = ITransformPattern2Identifiers_Vtbl;
}
unsafe impl ::windows_core::ComInterface for TransformPattern2Identifiers {
    const IID: ::windows_core::GUID =
        <ITransformPattern2Identifiers as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for TransformPattern2Identifiers {
    const NAME: &'static str = "Microsoft.UI.Xaml.Automation.TransformPattern2Identifiers";
}
::windows_core::imp::interface_hierarchy!(
    TransformPattern2Identifiers,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for TransformPattern2Identifiers {}
unsafe impl ::core::marker::Sync for TransformPattern2Identifiers {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct TransformPatternIdentifiers(::windows_core::IUnknown);
impl TransformPatternIdentifiers {
    pub fn CanMoveProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::ITransformPatternIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CanMoveProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn CanResizeProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::ITransformPatternIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CanResizeProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn CanRotateProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::ITransformPatternIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CanRotateProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ITransformPatternIdentifiersStatics<
        R,
        F: FnOnce(&ITransformPatternIdentifiersStatics) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<
            TransformPatternIdentifiers,
            ITransformPatternIdentifiersStatics,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for TransformPatternIdentifiers {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for TransformPatternIdentifiers {
    type Vtable = ITransformPatternIdentifiers_Vtbl;
}
unsafe impl ::windows_core::ComInterface for TransformPatternIdentifiers {
    const IID: ::windows_core::GUID =
        <ITransformPatternIdentifiers as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for TransformPatternIdentifiers {
    const NAME: &'static str = "Microsoft.UI.Xaml.Automation.TransformPatternIdentifiers";
}
::windows_core::imp::interface_hierarchy!(
    TransformPatternIdentifiers,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for TransformPatternIdentifiers {}
unsafe impl ::core::marker::Sync for TransformPatternIdentifiers {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ValuePatternIdentifiers(::windows_core::IUnknown);
impl ValuePatternIdentifiers {
    pub fn IsReadOnlyProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IValuePatternIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsReadOnlyProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn ValueProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IValuePatternIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ValueProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IValuePatternIdentifiersStatics<
        R,
        F: FnOnce(&IValuePatternIdentifiersStatics) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<
            ValuePatternIdentifiers,
            IValuePatternIdentifiersStatics,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for ValuePatternIdentifiers {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for ValuePatternIdentifiers {
    type Vtable = IValuePatternIdentifiers_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ValuePatternIdentifiers {
    const IID: ::windows_core::GUID =
        <IValuePatternIdentifiers as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ValuePatternIdentifiers {
    const NAME: &'static str = "Microsoft.UI.Xaml.Automation.ValuePatternIdentifiers";
}
::windows_core::imp::interface_hierarchy!(
    ValuePatternIdentifiers,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for ValuePatternIdentifiers {}
unsafe impl ::core::marker::Sync for ValuePatternIdentifiers {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct WindowPatternIdentifiers(::windows_core::IUnknown);
impl WindowPatternIdentifiers {
    pub fn CanMaximizeProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IWindowPatternIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CanMaximizeProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn CanMinimizeProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IWindowPatternIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CanMinimizeProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn IsModalProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IWindowPatternIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsModalProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn IsTopmostProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IWindowPatternIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsTopmostProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn WindowInteractionStateProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IWindowPatternIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).WindowInteractionStateProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn WindowVisualStateProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IWindowPatternIdentifiersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).WindowVisualStateProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IWindowPatternIdentifiersStatics<
        R,
        F: FnOnce(&IWindowPatternIdentifiersStatics) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<
            WindowPatternIdentifiers,
            IWindowPatternIdentifiersStatics,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for WindowPatternIdentifiers {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for WindowPatternIdentifiers {
    type Vtable = IWindowPatternIdentifiers_Vtbl;
}
unsafe impl ::windows_core::ComInterface for WindowPatternIdentifiers {
    const IID: ::windows_core::GUID =
        <IWindowPatternIdentifiers as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for WindowPatternIdentifiers {
    const NAME: &'static str = "Microsoft.UI.Xaml.Automation.WindowPatternIdentifiers";
}
::windows_core::imp::interface_hierarchy!(
    WindowPatternIdentifiers,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for WindowPatternIdentifiers {}
unsafe impl ::core::marker::Sync for WindowPatternIdentifiers {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AnnotationType(pub i32);
impl AnnotationType {
    pub const Unknown: Self = Self(60000i32);
    pub const SpellingError: Self = Self(60001i32);
    pub const GrammarError: Self = Self(60002i32);
    pub const Comment: Self = Self(60003i32);
    pub const FormulaError: Self = Self(60004i32);
    pub const TrackChanges: Self = Self(60005i32);
    pub const Header: Self = Self(60006i32);
    pub const Footer: Self = Self(60007i32);
    pub const Highlighted: Self = Self(60008i32);
    pub const Endnote: Self = Self(60009i32);
    pub const Footnote: Self = Self(60010i32);
    pub const InsertionChange: Self = Self(60011i32);
    pub const DeletionChange: Self = Self(60012i32);
    pub const MoveChange: Self = Self(60013i32);
    pub const FormatChange: Self = Self(60014i32);
    pub const UnsyncedChange: Self = Self(60015i32);
    pub const EditingLockedChange: Self = Self(60016i32);
    pub const ExternalChange: Self = Self(60017i32);
    pub const ConflictingChange: Self = Self(60018i32);
    pub const Author: Self = Self(60019i32);
    pub const AdvancedProofingIssue: Self = Self(60020i32);
    pub const DataValidationError: Self = Self(60021i32);
    pub const CircularReferenceError: Self = Self(60022i32);
}
impl ::core::marker::Copy for AnnotationType {}
impl ::core::clone::Clone for AnnotationType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AnnotationType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for AnnotationType {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for AnnotationType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AnnotationType").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for AnnotationType {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::from_slice(
            b"enum(Microsoft.UI.Xaml.Automation.AnnotationType;i4)",
        );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AutomationActiveEnd(pub i32);
impl AutomationActiveEnd {
    pub const None: Self = Self(0i32);
    pub const Start: Self = Self(1i32);
    pub const End: Self = Self(2i32);
}
impl ::core::marker::Copy for AutomationActiveEnd {}
impl ::core::clone::Clone for AutomationActiveEnd {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AutomationActiveEnd {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for AutomationActiveEnd {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for AutomationActiveEnd {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AutomationActiveEnd").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for AutomationActiveEnd {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::from_slice(
            b"enum(Microsoft.UI.Xaml.Automation.AutomationActiveEnd;i4)",
        );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AutomationAnimationStyle(pub i32);
impl AutomationAnimationStyle {
    pub const None: Self = Self(0i32);
    pub const LasVegasLights: Self = Self(1i32);
    pub const BlinkingBackground: Self = Self(2i32);
    pub const SparkleText: Self = Self(3i32);
    pub const MarchingBlackAnts: Self = Self(4i32);
    pub const MarchingRedAnts: Self = Self(5i32);
    pub const Shimmer: Self = Self(6i32);
    pub const Other: Self = Self(7i32);
}
impl ::core::marker::Copy for AutomationAnimationStyle {}
impl ::core::clone::Clone for AutomationAnimationStyle {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AutomationAnimationStyle {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for AutomationAnimationStyle {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for AutomationAnimationStyle {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AutomationAnimationStyle").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for AutomationAnimationStyle {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::from_slice(
            b"enum(Microsoft.UI.Xaml.Automation.AutomationAnimationStyle;i4)",
        );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AutomationBulletStyle(pub i32);
impl AutomationBulletStyle {
    pub const None: Self = Self(0i32);
    pub const HollowRoundBullet: Self = Self(1i32);
    pub const FilledRoundBullet: Self = Self(2i32);
    pub const HollowSquareBullet: Self = Self(3i32);
    pub const FilledSquareBullet: Self = Self(4i32);
    pub const DashBullet: Self = Self(5i32);
    pub const Other: Self = Self(6i32);
}
impl ::core::marker::Copy for AutomationBulletStyle {}
impl ::core::clone::Clone for AutomationBulletStyle {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AutomationBulletStyle {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for AutomationBulletStyle {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for AutomationBulletStyle {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AutomationBulletStyle").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for AutomationBulletStyle {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::from_slice(
            b"enum(Microsoft.UI.Xaml.Automation.AutomationBulletStyle;i4)",
        );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AutomationCaretBidiMode(pub i32);
impl AutomationCaretBidiMode {
    pub const LTR: Self = Self(0i32);
    pub const RTL: Self = Self(1i32);
}
impl ::core::marker::Copy for AutomationCaretBidiMode {}
impl ::core::clone::Clone for AutomationCaretBidiMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AutomationCaretBidiMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for AutomationCaretBidiMode {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for AutomationCaretBidiMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AutomationCaretBidiMode").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for AutomationCaretBidiMode {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::from_slice(
            b"enum(Microsoft.UI.Xaml.Automation.AutomationCaretBidiMode;i4)",
        );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AutomationCaretPosition(pub i32);
impl AutomationCaretPosition {
    pub const Unknown: Self = Self(0i32);
    pub const EndOfLine: Self = Self(1i32);
    pub const BeginningOfLine: Self = Self(2i32);
}
impl ::core::marker::Copy for AutomationCaretPosition {}
impl ::core::clone::Clone for AutomationCaretPosition {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AutomationCaretPosition {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for AutomationCaretPosition {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for AutomationCaretPosition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AutomationCaretPosition").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for AutomationCaretPosition {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::from_slice(
            b"enum(Microsoft.UI.Xaml.Automation.AutomationCaretPosition;i4)",
        );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AutomationFlowDirections(pub i32);
impl AutomationFlowDirections {
    pub const Default: Self = Self(0i32);
    pub const RightToLeft: Self = Self(1i32);
    pub const BottomToTop: Self = Self(2i32);
    pub const Vertical: Self = Self(3i32);
}
impl ::core::marker::Copy for AutomationFlowDirections {}
impl ::core::clone::Clone for AutomationFlowDirections {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AutomationFlowDirections {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for AutomationFlowDirections {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for AutomationFlowDirections {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AutomationFlowDirections").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for AutomationFlowDirections {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::from_slice(
            b"enum(Microsoft.UI.Xaml.Automation.AutomationFlowDirections;i4)",
        );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AutomationOutlineStyles(pub i32);
impl AutomationOutlineStyles {
    pub const None: Self = Self(0i32);
    pub const Outline: Self = Self(1i32);
    pub const Shadow: Self = Self(2i32);
    pub const Engraved: Self = Self(3i32);
    pub const Embossed: Self = Self(4i32);
}
impl ::core::marker::Copy for AutomationOutlineStyles {}
impl ::core::clone::Clone for AutomationOutlineStyles {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AutomationOutlineStyles {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for AutomationOutlineStyles {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for AutomationOutlineStyles {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AutomationOutlineStyles").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for AutomationOutlineStyles {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::from_slice(
            b"enum(Microsoft.UI.Xaml.Automation.AutomationOutlineStyles;i4)",
        );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AutomationStyleId(pub i32);
impl AutomationStyleId {
    pub const Heading1: Self = Self(70001i32);
    pub const Heading2: Self = Self(70002i32);
    pub const Heading3: Self = Self(70003i32);
    pub const Heading4: Self = Self(70004i32);
    pub const Heading5: Self = Self(70005i32);
    pub const Heading6: Self = Self(70006i32);
    pub const Heading7: Self = Self(70007i32);
    pub const Heading8: Self = Self(70008i32);
    pub const Heading9: Self = Self(70009i32);
    pub const Title: Self = Self(70010i32);
    pub const Subtitle: Self = Self(70011i32);
    pub const Normal: Self = Self(70012i32);
    pub const Emphasis: Self = Self(70013i32);
    pub const Quote: Self = Self(70014i32);
    pub const BulletedList: Self = Self(70015i32);
}
impl ::core::marker::Copy for AutomationStyleId {}
impl ::core::clone::Clone for AutomationStyleId {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AutomationStyleId {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for AutomationStyleId {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for AutomationStyleId {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AutomationStyleId").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for AutomationStyleId {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::from_slice(
            b"enum(Microsoft.UI.Xaml.Automation.AutomationStyleId;i4)",
        );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AutomationTextDecorationLineStyle(pub i32);
impl AutomationTextDecorationLineStyle {
    pub const None: Self = Self(0i32);
    pub const Single: Self = Self(1i32);
    pub const WordsOnly: Self = Self(2i32);
    pub const Double: Self = Self(3i32);
    pub const Dot: Self = Self(4i32);
    pub const Dash: Self = Self(5i32);
    pub const DashDot: Self = Self(6i32);
    pub const DashDotDot: Self = Self(7i32);
    pub const Wavy: Self = Self(8i32);
    pub const ThickSingle: Self = Self(9i32);
    pub const DoubleWavy: Self = Self(10i32);
    pub const ThickWavy: Self = Self(11i32);
    pub const LongDash: Self = Self(12i32);
    pub const ThickDash: Self = Self(13i32);
    pub const ThickDashDot: Self = Self(14i32);
    pub const ThickDashDotDot: Self = Self(15i32);
    pub const ThickDot: Self = Self(16i32);
    pub const ThickLongDash: Self = Self(17i32);
    pub const Other: Self = Self(18i32);
}
impl ::core::marker::Copy for AutomationTextDecorationLineStyle {}
impl ::core::clone::Clone for AutomationTextDecorationLineStyle {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AutomationTextDecorationLineStyle {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for AutomationTextDecorationLineStyle {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for AutomationTextDecorationLineStyle {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AutomationTextDecorationLineStyle").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for AutomationTextDecorationLineStyle {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::from_slice(
            b"enum(Microsoft.UI.Xaml.Automation.AutomationTextDecorationLineStyle;i4)",
        );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AutomationTextEditChangeType(pub i32);
impl AutomationTextEditChangeType {
    pub const None: Self = Self(0i32);
    pub const AutoCorrect: Self = Self(1i32);
    pub const Composition: Self = Self(2i32);
    pub const CompositionFinalized: Self = Self(3i32);
}
impl ::core::marker::Copy for AutomationTextEditChangeType {}
impl ::core::clone::Clone for AutomationTextEditChangeType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AutomationTextEditChangeType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for AutomationTextEditChangeType {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for AutomationTextEditChangeType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AutomationTextEditChangeType").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for AutomationTextEditChangeType {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::from_slice(
            b"enum(Microsoft.UI.Xaml.Automation.AutomationTextEditChangeType;i4)",
        );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DockPosition(pub i32);
impl DockPosition {
    pub const Top: Self = Self(0i32);
    pub const Left: Self = Self(1i32);
    pub const Bottom: Self = Self(2i32);
    pub const Right: Self = Self(3i32);
    pub const Fill: Self = Self(4i32);
    pub const None: Self = Self(5i32);
}
impl ::core::marker::Copy for DockPosition {}
impl ::core::clone::Clone for DockPosition {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DockPosition {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for DockPosition {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for DockPosition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DockPosition").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for DockPosition {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::from_slice(
            b"enum(Microsoft.UI.Xaml.Automation.DockPosition;i4)",
        );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ExpandCollapseState(pub i32);
impl ExpandCollapseState {
    pub const Collapsed: Self = Self(0i32);
    pub const Expanded: Self = Self(1i32);
    pub const PartiallyExpanded: Self = Self(2i32);
    pub const LeafNode: Self = Self(3i32);
}
impl ::core::marker::Copy for ExpandCollapseState {}
impl ::core::clone::Clone for ExpandCollapseState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ExpandCollapseState {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for ExpandCollapseState {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for ExpandCollapseState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ExpandCollapseState").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ExpandCollapseState {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::from_slice(
            b"enum(Microsoft.UI.Xaml.Automation.ExpandCollapseState;i4)",
        );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RowOrColumnMajor(pub i32);
impl RowOrColumnMajor {
    pub const RowMajor: Self = Self(0i32);
    pub const ColumnMajor: Self = Self(1i32);
    pub const Indeterminate: Self = Self(2i32);
}
impl ::core::marker::Copy for RowOrColumnMajor {}
impl ::core::clone::Clone for RowOrColumnMajor {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RowOrColumnMajor {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for RowOrColumnMajor {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for RowOrColumnMajor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RowOrColumnMajor").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for RowOrColumnMajor {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::from_slice(
            b"enum(Microsoft.UI.Xaml.Automation.RowOrColumnMajor;i4)",
        );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ScrollAmount(pub i32);
impl ScrollAmount {
    pub const LargeDecrement: Self = Self(0i32);
    pub const SmallDecrement: Self = Self(1i32);
    pub const NoAmount: Self = Self(2i32);
    pub const LargeIncrement: Self = Self(3i32);
    pub const SmallIncrement: Self = Self(4i32);
}
impl ::core::marker::Copy for ScrollAmount {}
impl ::core::clone::Clone for ScrollAmount {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ScrollAmount {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for ScrollAmount {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for ScrollAmount {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ScrollAmount").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ScrollAmount {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::from_slice(
            b"enum(Microsoft.UI.Xaml.Automation.ScrollAmount;i4)",
        );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SupportedTextSelection(pub i32);
impl SupportedTextSelection {
    pub const None: Self = Self(0i32);
    pub const Single: Self = Self(1i32);
    pub const Multiple: Self = Self(2i32);
}
impl ::core::marker::Copy for SupportedTextSelection {}
impl ::core::clone::Clone for SupportedTextSelection {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SupportedTextSelection {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for SupportedTextSelection {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for SupportedTextSelection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SupportedTextSelection").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for SupportedTextSelection {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::from_slice(
            b"enum(Microsoft.UI.Xaml.Automation.SupportedTextSelection;i4)",
        );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SynchronizedInputType(pub i32);
impl SynchronizedInputType {
    pub const KeyUp: Self = Self(1i32);
    pub const KeyDown: Self = Self(2i32);
    pub const LeftMouseUp: Self = Self(4i32);
    pub const LeftMouseDown: Self = Self(8i32);
    pub const RightMouseUp: Self = Self(16i32);
    pub const RightMouseDown: Self = Self(32i32);
}
impl ::core::marker::Copy for SynchronizedInputType {}
impl ::core::clone::Clone for SynchronizedInputType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SynchronizedInputType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for SynchronizedInputType {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for SynchronizedInputType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SynchronizedInputType").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for SynchronizedInputType {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::from_slice(
            b"enum(Microsoft.UI.Xaml.Automation.SynchronizedInputType;i4)",
        );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ToggleState(pub i32);
impl ToggleState {
    pub const Off: Self = Self(0i32);
    pub const On: Self = Self(1i32);
    pub const Indeterminate: Self = Self(2i32);
}
impl ::core::marker::Copy for ToggleState {}
impl ::core::clone::Clone for ToggleState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ToggleState {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for ToggleState {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for ToggleState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ToggleState").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ToggleState {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::from_slice(
            b"enum(Microsoft.UI.Xaml.Automation.ToggleState;i4)",
        );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WindowInteractionState(pub i32);
impl WindowInteractionState {
    pub const Running: Self = Self(0i32);
    pub const Closing: Self = Self(1i32);
    pub const ReadyForUserInteraction: Self = Self(2i32);
    pub const BlockedByModalWindow: Self = Self(3i32);
    pub const NotResponding: Self = Self(4i32);
}
impl ::core::marker::Copy for WindowInteractionState {}
impl ::core::clone::Clone for WindowInteractionState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WindowInteractionState {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for WindowInteractionState {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for WindowInteractionState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WindowInteractionState").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for WindowInteractionState {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::from_slice(
            b"enum(Microsoft.UI.Xaml.Automation.WindowInteractionState;i4)",
        );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WindowVisualState(pub i32);
impl WindowVisualState {
    pub const Normal: Self = Self(0i32);
    pub const Maximized: Self = Self(1i32);
    pub const Minimized: Self = Self(2i32);
}
impl ::core::marker::Copy for WindowVisualState {}
impl ::core::clone::Clone for WindowVisualState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WindowVisualState {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for WindowVisualState {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for WindowVisualState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WindowVisualState").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for WindowVisualState {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::from_slice(
            b"enum(Microsoft.UI.Xaml.Automation.WindowVisualState;i4)",
        );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ZoomUnit(pub i32);
impl ZoomUnit {
    pub const NoAmount: Self = Self(0i32);
    pub const LargeDecrement: Self = Self(1i32);
    pub const SmallDecrement: Self = Self(2i32);
    pub const LargeIncrement: Self = Self(3i32);
    pub const SmallIncrement: Self = Self(4i32);
}
impl ::core::marker::Copy for ZoomUnit {}
impl ::core::clone::Clone for ZoomUnit {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ZoomUnit {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for ZoomUnit {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for ZoomUnit {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ZoomUnit").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ZoomUnit {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::from_slice(
            b"enum(Microsoft.UI.Xaml.Automation.ZoomUnit;i4)",
        );
}
