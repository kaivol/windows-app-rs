#[doc(hidden)]
#[repr(transparent)]
pub struct IBlock(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IBlock {
    type Vtable = IBlock_Vtbl;
}
unsafe impl ::windows::core::Interface for IBlock {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x8149d507_672f_5fd5_a10a_351389ba9659);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBlock_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub TextAlignment: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::TextAlignment,
    ) -> ::windows::core::HRESULT,
    pub SetTextAlignment: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: super::TextAlignment,
    ) -> ::windows::core::HRESULT,
    pub HorizontalTextAlignment: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::TextAlignment,
    ) -> ::windows::core::HRESULT,
    pub SetHorizontalTextAlignment: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: super::TextAlignment,
    ) -> ::windows::core::HRESULT,
    pub LineHeight: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows::core::HRESULT,
    pub SetLineHeight: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
    ) -> ::windows::core::HRESULT,
    pub LineStackingStrategy: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::LineStackingStrategy,
    ) -> ::windows::core::HRESULT,
    pub SetLineStackingStrategy: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: super::LineStackingStrategy,
    ) -> ::windows::core::HRESULT,
    pub Margin: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::Thickness,
    ) -> ::windows::core::HRESULT,
    pub SetMargin: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: super::Thickness,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBlockFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IBlockFactory {
    type Vtable = IBlockFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for IBlockFactory {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x21bd671c_33e2_56ef_be37_a128e898452c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBlockFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        baseinterface: *mut ::core::ffi::c_void,
        innerinterface: *mut *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBlockStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IBlockStatics {
    type Vtable = IBlockStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IBlockStatics {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x830feedf_9aa6_56cd_983e_055500171b45);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBlockStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub TextAlignmentProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub HorizontalTextAlignmentProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub LineHeightProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub LineStackingStrategyProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub MarginProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBold(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IBold {
    type Vtable = IBold_Vtbl;
}
unsafe impl ::windows::core::Interface for IBold {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x241a5f5a_c164_597f_b0db_fac7431297f2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBold_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGlyphs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IGlyphs {
    type Vtable = IGlyphs_Vtbl;
}
unsafe impl ::windows::core::Interface for IGlyphs {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x0fbf8cfe_18e7_5e45_9fa3_d2d0927958f4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGlyphs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub UnicodeString: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub SetUnicodeString: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub Indices: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub SetIndices: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub FontUri: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub SetFontUri: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Media")]
    pub StyleSimulations: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::Media::StyleSimulations,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    StyleSimulations: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetStyleSimulations: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: super::Media::StyleSimulations,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetStyleSimulations: usize,
    pub FontRenderingEmSize: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows::core::HRESULT,
    pub SetFontRenderingEmSize: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
    ) -> ::windows::core::HRESULT,
    pub OriginX: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows::core::HRESULT,
    pub SetOriginX: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
    ) -> ::windows::core::HRESULT,
    pub OriginY: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows::core::HRESULT,
    pub SetOriginY: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
    ) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Media")]
    pub Fill: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    Fill: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetFill: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetFill: usize,
    pub IsColorFontEnabled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub SetIsColorFontEnabled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows::core::HRESULT,
    pub ColorFontPaletteIndex: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut i32,
    ) -> ::windows::core::HRESULT,
    pub SetColorFontPaletteIndex: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: i32,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGlyphsStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IGlyphsStatics {
    type Vtable = IGlyphsStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IGlyphsStatics {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x8d9e241a_3e0e_5100_8ede_e008034ff8ae);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGlyphsStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub UnicodeStringProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub IndicesProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub FontUriProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub StyleSimulationsProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub FontRenderingEmSizeProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub OriginXProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub OriginYProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub FillProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub IsColorFontEnabledProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub ColorFontPaletteIndexProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHyperlink(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IHyperlink {
    type Vtable = IHyperlink_Vtbl;
}
unsafe impl ::windows::core::Interface for IHyperlink {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xac09bd16_cdfa_54c2_8d03_a474181545b1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHyperlink_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub NavigateUri: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub SetNavigateUri: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub UnderlineStyle: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut UnderlineStyle,
    ) -> ::windows::core::HRESULT,
    pub SetUnderlineStyle: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: UnderlineStyle,
    ) -> ::windows::core::HRESULT,
    pub XYFocusLeft: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub SetXYFocusLeft: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub XYFocusRight: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub SetXYFocusRight: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub XYFocusUp: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub SetXYFocusUp: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub XYFocusDown: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub SetXYFocusDown: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub ElementSoundMode: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::ElementSoundMode,
    ) -> ::windows::core::HRESULT,
    pub SetElementSoundMode: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: super::ElementSoundMode,
    ) -> ::windows::core::HRESULT,
    pub FocusState: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::FocusState,
    ) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Input")]
    pub XYFocusUpNavigationStrategy: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::Input::XYFocusNavigationStrategy,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    XYFocusUpNavigationStrategy: usize,
    #[cfg(feature = "UI_Xaml_Input")]
    pub SetXYFocusUpNavigationStrategy: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: super::Input::XYFocusNavigationStrategy,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    SetXYFocusUpNavigationStrategy: usize,
    #[cfg(feature = "UI_Xaml_Input")]
    pub XYFocusDownNavigationStrategy: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::Input::XYFocusNavigationStrategy,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    XYFocusDownNavigationStrategy: usize,
    #[cfg(feature = "UI_Xaml_Input")]
    pub SetXYFocusDownNavigationStrategy: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: super::Input::XYFocusNavigationStrategy,
    )
        -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    SetXYFocusDownNavigationStrategy: usize,
    #[cfg(feature = "UI_Xaml_Input")]
    pub XYFocusLeftNavigationStrategy: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::Input::XYFocusNavigationStrategy,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    XYFocusLeftNavigationStrategy: usize,
    #[cfg(feature = "UI_Xaml_Input")]
    pub SetXYFocusLeftNavigationStrategy: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: super::Input::XYFocusNavigationStrategy,
    )
        -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    SetXYFocusLeftNavigationStrategy: usize,
    #[cfg(feature = "UI_Xaml_Input")]
    pub XYFocusRightNavigationStrategy: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::Input::XYFocusNavigationStrategy,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    XYFocusRightNavigationStrategy: usize,
    #[cfg(feature = "UI_Xaml_Input")]
    pub SetXYFocusRightNavigationStrategy: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: super::Input::XYFocusNavigationStrategy,
    )
        -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    SetXYFocusRightNavigationStrategy: usize,
    pub IsTabStop: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub SetIsTabStop: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows::core::HRESULT,
    pub TabIndex: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut i32,
    ) -> ::windows::core::HRESULT,
    pub SetTabIndex: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: i32,
    ) -> ::windows::core::HRESULT,
    pub Click: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub RemoveClick: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub GotFocus: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub RemoveGotFocus: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub LostFocus: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub RemoveLostFocus: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub Focus: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: super::FocusState,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHyperlinkClickEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IHyperlinkClickEventArgs {
    type Vtable = IHyperlinkClickEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IHyperlinkClickEventArgs {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xf8f89552_873d_5ef5_82bf_c79a9509b07c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHyperlinkClickEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHyperlinkStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IHyperlinkStatics {
    type Vtable = IHyperlinkStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IHyperlinkStatics {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xe13598f4_7bc7_5ab9_885b_70f32f8c9531);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHyperlinkStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub NavigateUriProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub UnderlineStyleProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub XYFocusLeftProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub XYFocusRightProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub XYFocusUpProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub XYFocusDownProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub ElementSoundModeProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub FocusStateProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub XYFocusUpNavigationStrategyProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    )
        -> ::windows::core::HRESULT,
    pub XYFocusDownNavigationStrategyProperty:
        unsafe extern "system" fn(
            this: *mut ::core::ffi::c_void,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT,
    pub XYFocusLeftNavigationStrategyProperty:
        unsafe extern "system" fn(
            this: *mut ::core::ffi::c_void,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT,
    pub XYFocusRightNavigationStrategyProperty:
        unsafe extern "system" fn(
            this: *mut ::core::ffi::c_void,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT,
    pub IsTabStopProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub TabIndexProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInline(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IInline {
    type Vtable = IInline_Vtbl;
}
unsafe impl ::windows::core::Interface for IInline {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x813d427a_8980_5a79_a8fa_f27919cfb24f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInline_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInlineFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IInlineFactory {
    type Vtable = IInlineFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for IInlineFactory {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xfd253a36_fa2b_5b30_89a8_9f577871ec07);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInlineFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        baseinterface: *mut ::core::ffi::c_void,
        innerinterface: *mut *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInlineUIContainer(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IInlineUIContainer {
    type Vtable = IInlineUIContainer_Vtbl;
}
unsafe impl ::windows::core::Interface for IInlineUIContainer {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xd529bef6_c05a_5bad_85e8_640127cf86f5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInlineUIContainer_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Child: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub SetChild: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IItalic(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IItalic {
    type Vtable = IItalic_Vtbl;
}
unsafe impl ::windows::core::Interface for IItalic {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xca3cbebd_7a8d_5d7a_8fdf_538e8a680f6c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IItalic_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILineBreak(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ILineBreak {
    type Vtable = ILineBreak_Vtbl;
}
unsafe impl ::windows::core::Interface for ILineBreak {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x09307599_7cc2_5f54_b106_728620c16f76);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILineBreak_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IParagraph(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IParagraph {
    type Vtable = IParagraph_Vtbl;
}
unsafe impl ::windows::core::Interface for IParagraph {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x9ed64c77_329d_502f_a257_f58398edab51);
}
#[repr(C)]
#[doc(hidden)]
pub struct IParagraph_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Inlines: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub TextIndent: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows::core::HRESULT,
    pub SetTextIndent: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IParagraphStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IParagraphStatics {
    type Vtable = IParagraphStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IParagraphStatics {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x4eb89ab1_66c8_5fc0_aa5f_48c8092ceb5f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IParagraphStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub TextIndentProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRun(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IRun {
    type Vtable = IRun_Vtbl;
}
unsafe impl ::windows::core::Interface for IRun {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x1f905239_37cb_590b_9132_3ffb7741906e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRun_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Text: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub SetText: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub FlowDirection: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::FlowDirection,
    ) -> ::windows::core::HRESULT,
    pub SetFlowDirection: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: super::FlowDirection,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRunStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IRunStatics {
    type Vtable = IRunStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IRunStatics {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x051b3c5b_7600_51a5_80c5_93eb50fd684f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRunStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub FlowDirectionProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpan(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ISpan {
    type Vtable = ISpan_Vtbl;
}
unsafe impl ::windows::core::Interface for ISpan {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x91b93d4d_4e28_57b9_bffb_3566c2a3c2a1);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpan_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Inlines: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub SetInlines: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpanFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ISpanFactory {
    type Vtable = ISpanFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for ISpanFactory {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xa6e87c16_c175_55c8_bbd3_ce40f9d0a680);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpanFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        baseinterface: *mut ::core::ffi::c_void,
        innerinterface: *mut *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITextElement(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ITextElement {
    type Vtable = ITextElement_Vtbl;
}
unsafe impl ::windows::core::Interface for ITextElement {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xa122ba22_833f_5220_a47e_6cd507531abe);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextElement_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Name: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub FontSize: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows::core::HRESULT,
    pub SetFontSize: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
    ) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Media")]
    pub FontFamily: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    FontFamily: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetFontFamily: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetFontFamily: usize,
    pub FontWeight: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Text::FontWeight,
    ) -> ::windows::core::HRESULT,
    pub SetFontWeight: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::windows::UI::Text::FontWeight,
    ) -> ::windows::core::HRESULT,
    pub FontStyle: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Text::FontStyle,
    ) -> ::windows::core::HRESULT,
    pub SetFontStyle: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::windows::UI::Text::FontStyle,
    ) -> ::windows::core::HRESULT,
    pub FontStretch: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Text::FontStretch,
    ) -> ::windows::core::HRESULT,
    pub SetFontStretch: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::windows::UI::Text::FontStretch,
    ) -> ::windows::core::HRESULT,
    pub CharacterSpacing: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut i32,
    ) -> ::windows::core::HRESULT,
    pub SetCharacterSpacing: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: i32,
    ) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Media")]
    pub Foreground: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    Foreground: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetForeground: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetForeground: usize,
    pub Language: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub SetLanguage: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub IsTextScaleFactorEnabled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub SetIsTextScaleFactorEnabled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows::core::HRESULT,
    pub TextDecorations: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Text::TextDecorations,
    ) -> ::windows::core::HRESULT,
    pub SetTextDecorations: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::windows::UI::Text::TextDecorations,
    ) -> ::windows::core::HRESULT,
    pub ContentStart: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub ContentEnd: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub ElementStart: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub ElementEnd: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub AllowFocusOnInteraction: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub SetAllowFocusOnInteraction: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows::core::HRESULT,
    pub AccessKey: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub SetAccessKey: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub ExitDisplayModeOnAccessKeyInvoked: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    )
        -> ::windows::core::HRESULT,
    pub SetExitDisplayModeOnAccessKeyInvoked: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    )
        -> ::windows::core::HRESULT,
    pub IsAccessKeyScope: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub SetIsAccessKeyScope: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows::core::HRESULT,
    pub AccessKeyScopeOwner: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub SetAccessKeyScopeOwner: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Input")]
    pub KeyTipPlacementMode: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::Input::KeyTipPlacementMode,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    KeyTipPlacementMode: usize,
    #[cfg(feature = "UI_Xaml_Input")]
    pub SetKeyTipPlacementMode: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: super::Input::KeyTipPlacementMode,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    SetKeyTipPlacementMode: usize,
    pub KeyTipHorizontalOffset: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows::core::HRESULT,
    pub SetKeyTipHorizontalOffset: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
    ) -> ::windows::core::HRESULT,
    pub KeyTipVerticalOffset: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows::core::HRESULT,
    pub SetKeyTipVerticalOffset: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
    ) -> ::windows::core::HRESULT,
    pub XamlRoot: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub SetXamlRoot: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Input")]
    pub AccessKeyDisplayRequested: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    AccessKeyDisplayRequested: usize,
    pub RemoveAccessKeyDisplayRequested: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Input")]
    pub AccessKeyDisplayDismissed: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    AccessKeyDisplayDismissed: usize,
    pub RemoveAccessKeyDisplayDismissed: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Input")]
    pub AccessKeyInvoked: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    AccessKeyInvoked: usize,
    pub RemoveAccessKeyInvoked: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub FindName: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITextElementFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ITextElementFactory {
    type Vtable = ITextElementFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for ITextElementFactory {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xdf51fb95_a5e6_5b16_8e88_9f7cbfa234b1);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextElementFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITextElementOverrides(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ITextElementOverrides {
    type Vtable = ITextElementOverrides_Vtbl;
}
unsafe impl ::windows::core::Interface for ITextElementOverrides {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x41b01380_e49f_5fda_8c72_acc1ac1e91df);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextElementOverrides_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub OnDisconnectVisualChildren:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITextElementStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ITextElementStatics {
    type Vtable = ITextElementStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for ITextElementStatics {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xc9b55919_e1fe_5acd_bac7_c9d7f413b35c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextElementStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub FontSizeProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub FontFamilyProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub FontWeightProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub FontStyleProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub FontStretchProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub CharacterSpacingProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub ForegroundProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub LanguageProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub IsTextScaleFactorEnabledProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    )
        -> ::windows::core::HRESULT,
    pub TextDecorationsProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub AllowFocusOnInteractionProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub AccessKeyProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub ExitDisplayModeOnAccessKeyInvokedProperty:
        unsafe extern "system" fn(
            this: *mut ::core::ffi::c_void,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT,
    pub IsAccessKeyScopeProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub AccessKeyScopeOwnerProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub KeyTipPlacementModeProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub KeyTipHorizontalOffsetProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub KeyTipVerticalOffsetProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITextHighlighter(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ITextHighlighter {
    type Vtable = ITextHighlighter_Vtbl;
}
unsafe impl ::windows::core::Interface for ITextHighlighter {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xb756e861_1d2b_5f6f_81fd_c51a5bc068ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextHighlighter_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Ranges: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Media")]
    pub Foreground: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    Foreground: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetForeground: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetForeground: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub Background: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    Background: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetBackground: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetBackground: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITextHighlighterBase(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ITextHighlighterBase {
    type Vtable = ITextHighlighterBase_Vtbl;
}
unsafe impl ::windows::core::Interface for ITextHighlighterBase {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x5c21aaf0_3a17_5468_8aac_be14db0ed8c1);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextHighlighterBase_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITextHighlighterBaseFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ITextHighlighterBaseFactory {
    type Vtable = ITextHighlighterBaseFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for ITextHighlighterBaseFactory {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xe091e461_53ab_599e_aaea_800adc72da4f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextHighlighterBaseFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITextHighlighterFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ITextHighlighterFactory {
    type Vtable = ITextHighlighterFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for ITextHighlighterFactory {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x69c7311f_c019_5b93_b511_81418543bab7);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextHighlighterFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        baseinterface: *mut ::core::ffi::c_void,
        innerinterface: *mut *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITextHighlighterStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ITextHighlighterStatics {
    type Vtable = ITextHighlighterStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for ITextHighlighterStatics {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x4975047a_87ad_51a2_977c_e771de4f4035);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextHighlighterStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ForegroundProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub BackgroundProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITextPointer(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ITextPointer {
    type Vtable = ITextPointer_Vtbl;
}
unsafe impl ::windows::core::Interface for ITextPointer {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x842eb385_ee41_5930_979b_438fa7525a51);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextPointer_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Parent: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub VisualParent: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub LogicalDirection: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut LogicalDirection,
    ) -> ::windows::core::HRESULT,
    pub Offset: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut i32,
    ) -> ::windows::core::HRESULT,
    pub GetCharacterRect: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        direction: LogicalDirection,
        result__: *mut ::windows::Foundation::Rect,
    ) -> ::windows::core::HRESULT,
    pub GetPositionAtOffset: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        offset: i32,
        direction: LogicalDirection,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITypography(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ITypography {
    type Vtable = ITypography_Vtbl;
}
unsafe impl ::windows::core::Interface for ITypography {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xfa27e2e3_be5e_5d21_9a5e_90cf102af828);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITypography_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITypographyStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ITypographyStatics {
    type Vtable = ITypographyStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for ITypographyStatics {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x55fe4535_2125_533a_ada8_27be2b9e1193);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITypographyStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub AnnotationAlternatesProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub GetAnnotationAlternates: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        result__: *mut i32,
    ) -> ::windows::core::HRESULT,
    pub SetAnnotationAlternates: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        value: i32,
    ) -> ::windows::core::HRESULT,
    pub EastAsianExpertFormsProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub GetEastAsianExpertForms: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub SetEastAsianExpertForms: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows::core::HRESULT,
    pub EastAsianLanguageProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub GetEastAsianLanguage: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        result__: *mut super::FontEastAsianLanguage,
    ) -> ::windows::core::HRESULT,
    pub SetEastAsianLanguage: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        value: super::FontEastAsianLanguage,
    ) -> ::windows::core::HRESULT,
    pub EastAsianWidthsProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub GetEastAsianWidths: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        result__: *mut super::FontEastAsianWidths,
    ) -> ::windows::core::HRESULT,
    pub SetEastAsianWidths: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        value: super::FontEastAsianWidths,
    ) -> ::windows::core::HRESULT,
    pub StandardLigaturesProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub GetStandardLigatures: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub SetStandardLigatures: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows::core::HRESULT,
    pub ContextualLigaturesProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub GetContextualLigatures: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub SetContextualLigatures: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows::core::HRESULT,
    pub DiscretionaryLigaturesProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub GetDiscretionaryLigatures: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub SetDiscretionaryLigatures: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows::core::HRESULT,
    pub HistoricalLigaturesProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub GetHistoricalLigatures: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub SetHistoricalLigatures: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows::core::HRESULT,
    pub StandardSwashesProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub GetStandardSwashes: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        result__: *mut i32,
    ) -> ::windows::core::HRESULT,
    pub SetStandardSwashes: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        value: i32,
    ) -> ::windows::core::HRESULT,
    pub ContextualSwashesProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub GetContextualSwashes: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        result__: *mut i32,
    ) -> ::windows::core::HRESULT,
    pub SetContextualSwashes: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        value: i32,
    ) -> ::windows::core::HRESULT,
    pub ContextualAlternatesProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub GetContextualAlternates: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub SetContextualAlternates: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows::core::HRESULT,
    pub StylisticAlternatesProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub GetStylisticAlternates: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        result__: *mut i32,
    ) -> ::windows::core::HRESULT,
    pub SetStylisticAlternates: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        value: i32,
    ) -> ::windows::core::HRESULT,
    pub StylisticSet1Property: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub GetStylisticSet1: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub SetStylisticSet1: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows::core::HRESULT,
    pub StylisticSet2Property: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub GetStylisticSet2: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub SetStylisticSet2: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows::core::HRESULT,
    pub StylisticSet3Property: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub GetStylisticSet3: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub SetStylisticSet3: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows::core::HRESULT,
    pub StylisticSet4Property: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub GetStylisticSet4: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub SetStylisticSet4: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows::core::HRESULT,
    pub StylisticSet5Property: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub GetStylisticSet5: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub SetStylisticSet5: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows::core::HRESULT,
    pub StylisticSet6Property: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub GetStylisticSet6: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub SetStylisticSet6: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows::core::HRESULT,
    pub StylisticSet7Property: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub GetStylisticSet7: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub SetStylisticSet7: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows::core::HRESULT,
    pub StylisticSet8Property: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub GetStylisticSet8: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub SetStylisticSet8: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows::core::HRESULT,
    pub StylisticSet9Property: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub GetStylisticSet9: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub SetStylisticSet9: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows::core::HRESULT,
    pub StylisticSet10Property: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub GetStylisticSet10: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub SetStylisticSet10: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows::core::HRESULT,
    pub StylisticSet11Property: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub GetStylisticSet11: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub SetStylisticSet11: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows::core::HRESULT,
    pub StylisticSet12Property: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub GetStylisticSet12: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub SetStylisticSet12: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows::core::HRESULT,
    pub StylisticSet13Property: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub GetStylisticSet13: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub SetStylisticSet13: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows::core::HRESULT,
    pub StylisticSet14Property: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub GetStylisticSet14: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub SetStylisticSet14: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows::core::HRESULT,
    pub StylisticSet15Property: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub GetStylisticSet15: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub SetStylisticSet15: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows::core::HRESULT,
    pub StylisticSet16Property: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub GetStylisticSet16: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub SetStylisticSet16: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows::core::HRESULT,
    pub StylisticSet17Property: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub GetStylisticSet17: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub SetStylisticSet17: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows::core::HRESULT,
    pub StylisticSet18Property: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub GetStylisticSet18: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub SetStylisticSet18: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows::core::HRESULT,
    pub StylisticSet19Property: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub GetStylisticSet19: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub SetStylisticSet19: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows::core::HRESULT,
    pub StylisticSet20Property: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub GetStylisticSet20: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub SetStylisticSet20: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows::core::HRESULT,
    pub CapitalsProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub GetCapitals: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        result__: *mut super::FontCapitals,
    ) -> ::windows::core::HRESULT,
    pub SetCapitals: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        value: super::FontCapitals,
    ) -> ::windows::core::HRESULT,
    pub CapitalSpacingProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub GetCapitalSpacing: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub SetCapitalSpacing: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows::core::HRESULT,
    pub KerningProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub GetKerning: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub SetKerning: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows::core::HRESULT,
    pub CaseSensitiveFormsProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub GetCaseSensitiveForms: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub SetCaseSensitiveForms: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows::core::HRESULT,
    pub HistoricalFormsProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub GetHistoricalForms: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub SetHistoricalForms: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows::core::HRESULT,
    pub FractionProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub GetFraction: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        result__: *mut super::FontFraction,
    ) -> ::windows::core::HRESULT,
    pub SetFraction: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        value: super::FontFraction,
    ) -> ::windows::core::HRESULT,
    pub NumeralStyleProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub GetNumeralStyle: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        result__: *mut super::FontNumeralStyle,
    ) -> ::windows::core::HRESULT,
    pub SetNumeralStyle: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        value: super::FontNumeralStyle,
    ) -> ::windows::core::HRESULT,
    pub NumeralAlignmentProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub GetNumeralAlignment: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        result__: *mut super::FontNumeralAlignment,
    ) -> ::windows::core::HRESULT,
    pub SetNumeralAlignment: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        value: super::FontNumeralAlignment,
    ) -> ::windows::core::HRESULT,
    pub SlashedZeroProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub GetSlashedZero: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub SetSlashedZero: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows::core::HRESULT,
    pub MathematicalGreekProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub GetMathematicalGreek: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub SetMathematicalGreek: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows::core::HRESULT,
    pub VariantsProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub GetVariants: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        result__: *mut super::FontVariants,
    ) -> ::windows::core::HRESULT,
    pub SetVariants: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        value: super::FontVariants,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUnderline(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IUnderline {
    type Vtable = IUnderline_Vtbl;
}
unsafe impl ::windows::core::Interface for IUnderline {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x68aaec6e_ea71_5ed2_b83e_91684b25efb9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUnderline_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
}
#[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
#[repr(transparent)]
pub struct Block(::windows::core::IUnknown);
impl Block {
    pub fn TextAlignment(&self) -> ::windows::core::Result<super::TextAlignment> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TextAlignment)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::TextAlignment>(result__)
        }
    }
    pub fn SetTextAlignment(&self, value: super::TextAlignment) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetTextAlignment)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn HorizontalTextAlignment(&self) -> ::windows::core::Result<super::TextAlignment> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).HorizontalTextAlignment)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::TextAlignment>(result__)
        }
    }
    pub fn SetHorizontalTextAlignment(
        &self,
        value: super::TextAlignment,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetHorizontalTextAlignment)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn LineHeight(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LineHeight)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetLineHeight(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetLineHeight)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn LineStackingStrategy(&self) -> ::windows::core::Result<super::LineStackingStrategy> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LineStackingStrategy)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::LineStackingStrategy>(result__)
        }
    }
    pub fn SetLineStackingStrategy(
        &self,
        value: super::LineStackingStrategy,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetLineStackingStrategy)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn Margin(&self) -> ::windows::core::Result<super::Thickness> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Margin)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::Thickness>(result__)
        }
    }
    pub fn SetMargin(&self, value: super::Thickness) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetMargin)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn TextAlignmentProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IBlockStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TextAlignmentProperty)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn HorizontalTextAlignmentProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IBlockStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).HorizontalTextAlignmentProperty)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn LineHeightProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IBlockStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LineHeightProperty)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn LineStackingStrategyProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IBlockStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LineStackingStrategyProperty)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn MarginProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IBlockStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MarginProperty)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn GetValue(
        &self,
        dp: &super::DependencyProperty,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetValue)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(dp),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn SetValue<'a, P0>(
        &self,
        dp: &super::DependencyProperty,
        value: P0,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IInspectable>>,
    {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetValue)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(dp),
                value.into().abi(),
            )
            .ok()
        }
    }
    pub fn ClearValue(&self, dp: &super::DependencyProperty) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).ClearValue)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(dp),
            )
            .ok()
        }
    }
    pub fn ReadLocalValue(
        &self,
        dp: &super::DependencyProperty,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ReadLocalValue)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(dp),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn GetAnimationBaseValue(
        &self,
        dp: &super::DependencyProperty,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetAnimationBaseValue)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(dp),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn RegisterPropertyChangedCallback(
        &self,
        dp: &super::DependencyProperty,
        callback: &super::DependencyPropertyChangedCallback,
    ) -> ::windows::core::Result<i64> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RegisterPropertyChangedCallback)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(dp),
                ::core::mem::transmute_copy(callback),
                result__.as_mut_ptr(),
            )
            .from_abi::<i64>(result__)
        }
    }
    pub fn UnregisterPropertyChangedCallback(
        &self,
        dp: &super::DependencyProperty,
        token: i64,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).UnregisterPropertyChangedCallback)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(dp),
                token,
            )
            .ok()
        }
    }
    pub fn Dispatcher(&self) -> ::windows::core::Result<::windows::UI::Core::CoreDispatcher> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Dispatcher)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Core::CoreDispatcher>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Dispatching\"`*"]
    #[cfg(feature = "UI_Dispatching")]
    pub fn DispatcherQueue(
        &self,
    ) -> ::windows::core::Result<super::super::Dispatching::DispatcherQueue> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DispatcherQueue)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::super::Dispatching::DispatcherQueue>(result__)
        }
    }
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Name)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn FontSize(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FontSize)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetFontSize(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetFontSize)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn FontFamily(&self) -> ::windows::core::Result<super::Media::FontFamily> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FontFamily)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::Media::FontFamily>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetFontFamily<'a, P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::Media::FontFamily>>,
    {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetFontFamily)(
                ::windows::core::Vtable::as_raw(this),
                value.into().abi(),
            )
            .ok()
        }
    }
    pub fn FontWeight(&self) -> ::windows::core::Result<::windows::UI::Text::FontWeight> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FontWeight)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Text::FontWeight>(result__)
        }
    }
    pub fn SetFontWeight(
        &self,
        value: ::windows::UI::Text::FontWeight,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetFontWeight)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn FontStyle(&self) -> ::windows::core::Result<::windows::UI::Text::FontStyle> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FontStyle)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Text::FontStyle>(result__)
        }
    }
    pub fn SetFontStyle(
        &self,
        value: ::windows::UI::Text::FontStyle,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetFontStyle)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn FontStretch(&self) -> ::windows::core::Result<::windows::UI::Text::FontStretch> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FontStretch)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Text::FontStretch>(result__)
        }
    }
    pub fn SetFontStretch(
        &self,
        value: ::windows::UI::Text::FontStretch,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetFontStretch)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn CharacterSpacing(&self) -> ::windows::core::Result<i32> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CharacterSpacing)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<i32>(result__)
        }
    }
    pub fn SetCharacterSpacing(&self, value: i32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetCharacterSpacing)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn Foreground(&self) -> ::windows::core::Result<super::Media::Brush> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Foreground)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::Media::Brush>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetForeground<'a, P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::Media::Brush>>,
    {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetForeground)(
                ::windows::core::Vtable::as_raw(this),
                value.into().abi(),
            )
            .ok()
        }
    }
    pub fn Language(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
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
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetLanguage)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn IsTextScaleFactorEnabled(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsTextScaleFactorEnabled)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetIsTextScaleFactorEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetIsTextScaleFactorEnabled)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn TextDecorations(&self) -> ::windows::core::Result<::windows::UI::Text::TextDecorations> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TextDecorations)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Text::TextDecorations>(result__)
        }
    }
    pub fn SetTextDecorations(
        &self,
        value: ::windows::UI::Text::TextDecorations,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetTextDecorations)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn ContentStart(&self) -> ::windows::core::Result<TextPointer> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ContentStart)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    pub fn ContentEnd(&self) -> ::windows::core::Result<TextPointer> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ContentEnd)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    pub fn ElementStart(&self) -> ::windows::core::Result<TextPointer> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ElementStart)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    pub fn ElementEnd(&self) -> ::windows::core::Result<TextPointer> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ElementEnd)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    pub fn AllowFocusOnInteraction(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AllowFocusOnInteraction)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetAllowFocusOnInteraction(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetAllowFocusOnInteraction)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn AccessKey(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AccessKey)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetAccessKey(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetAccessKey)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn ExitDisplayModeOnAccessKeyInvoked(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ExitDisplayModeOnAccessKeyInvoked)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetExitDisplayModeOnAccessKeyInvoked(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetExitDisplayModeOnAccessKeyInvoked)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsAccessKeyScope(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsAccessKeyScope)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetIsAccessKeyScope(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetIsAccessKeyScope)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn AccessKeyScopeOwner(&self) -> ::windows::core::Result<super::DependencyObject> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AccessKeyScopeOwner)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::DependencyObject>(result__)
        }
    }
    pub fn SetAccessKeyScopeOwner<'a, P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::DependencyObject>>,
    {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetAccessKeyScopeOwner)(
                ::windows::core::Vtable::as_raw(this),
                value.into().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn KeyTipPlacementMode(
        &self,
    ) -> ::windows::core::Result<super::Input::KeyTipPlacementMode> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).KeyTipPlacementMode)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::Input::KeyTipPlacementMode>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetKeyTipPlacementMode(
        &self,
        value: super::Input::KeyTipPlacementMode,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetKeyTipPlacementMode)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn KeyTipHorizontalOffset(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).KeyTipHorizontalOffset)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetKeyTipHorizontalOffset(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetKeyTipHorizontalOffset)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn KeyTipVerticalOffset(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).KeyTipVerticalOffset)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetKeyTipVerticalOffset(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetKeyTipVerticalOffset)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn XamlRoot(&self) -> ::windows::core::Result<super::XamlRoot> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).XamlRoot)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::XamlRoot>(result__)
        }
    }
    pub fn SetXamlRoot(&self, value: &super::XamlRoot) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetXamlRoot)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn AccessKeyDisplayRequested(
        &self,
        handler: &::windows::Foundation::TypedEventHandler<
            TextElement,
            super::Input::AccessKeyDisplayRequestedEventArgs,
        >,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AccessKeyDisplayRequested)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(handler),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveAccessKeyDisplayRequested(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).RemoveAccessKeyDisplayRequested)(
                ::windows::core::Vtable::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn AccessKeyDisplayDismissed(
        &self,
        handler: &::windows::Foundation::TypedEventHandler<
            TextElement,
            super::Input::AccessKeyDisplayDismissedEventArgs,
        >,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AccessKeyDisplayDismissed)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(handler),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveAccessKeyDisplayDismissed(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).RemoveAccessKeyDisplayDismissed)(
                ::windows::core::Vtable::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn AccessKeyInvoked(
        &self,
        handler: &::windows::Foundation::TypedEventHandler<
            TextElement,
            super::Input::AccessKeyInvokedEventArgs,
        >,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AccessKeyInvoked)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(handler),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveAccessKeyInvoked(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).RemoveAccessKeyInvoked)(
                ::windows::core::Vtable::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn FindName(
        &self,
        name: &::windows::core::HSTRING,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FindName)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(name),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc(hidden)]
    pub fn IBlockStatics<R, F: FnOnce(&IBlockStatics) -> ::windows::core::Result<R>>(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<Block, IBlockStatics> =
            ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for Block {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for Block {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Block {}
impl ::core::fmt::Debug for Block {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Block").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for Block {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Xaml.Documents.Block;{8149d507-672f-5fd5-a10a-351389ba9659})",
    );
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for Block {
    type Vtable = IBlock_Vtbl;
}
unsafe impl ::windows::core::Interface for Block {
    const IID: ::windows::core::GUID = <IBlock as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for Block {
    const NAME: &'static str = "Microsoft.UI.Xaml.Documents.Block";
}
::windows::core::interface_hierarchy!(
    Block,
    ::windows::core::IUnknown,
    ::windows::core::IInspectable
);
impl ::core::convert::From<Block> for TextElement {
    fn from(value: Block) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&Block> for TextElement {
    fn from(value: &Block) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::core::convert::From<&Block> for ::windows::core::InParam<'a, TextElement> {
    fn from(value: &Block) -> Self {
        ::windows::core::InParam::owned(value.into())
    }
}
impl ::core::convert::From<Block> for super::DependencyObject {
    fn from(value: Block) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&Block> for super::DependencyObject {
    fn from(value: &Block) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::core::convert::From<&Block> for ::windows::core::InParam<'a, super::DependencyObject> {
    fn from(value: &Block) -> Self {
        ::windows::core::InParam::owned(value.into())
    }
}
unsafe impl ::core::marker::Send for Block {}
unsafe impl ::core::marker::Sync for Block {}
#[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
#[repr(transparent)]
pub struct BlockCollection(::windows::core::IUnknown);
impl BlockCollection {
    pub fn First(
        &self,
    ) -> ::windows::core::Result<::windows::Foundation::Collections::IIterator<Block>> {
        let this = &::windows::core::Interface::cast::<
            ::windows::Foundation::Collections::IIterable<Block>,
        >(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).First)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::Collections::IIterator<Block>>(result__)
        }
    }
    pub fn GetAt(&self, index: u32) -> ::windows::core::Result<Block> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetAt)(
                ::windows::core::Vtable::as_raw(this),
                index,
                result__.as_mut_ptr(),
            )
            .from_abi::<Block>(result__)
        }
    }
    pub fn Size(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Size)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<u32>(result__)
        }
    }
    pub fn GetView(
        &self,
    ) -> ::windows::core::Result<::windows::Foundation::Collections::IVectorView<Block>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetView)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::Collections::IVectorView<Block>>(result__)
        }
    }
    pub fn IndexOf<'a, P0>(&self, value: P0, index: &mut u32) -> ::windows::core::Result<bool>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, Block>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IndexOf)(
                ::windows::core::Vtable::as_raw(this),
                value.into().abi(),
                index,
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetAt<'a, P0>(&self, index: u32, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, Block>>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetAt)(
                ::windows::core::Vtable::as_raw(this),
                index,
                value.into().abi(),
            )
            .ok()
        }
    }
    pub fn InsertAt<'a, P0>(&self, index: u32, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, Block>>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).InsertAt)(
                ::windows::core::Vtable::as_raw(this),
                index,
                value.into().abi(),
            )
            .ok()
        }
    }
    pub fn RemoveAt(&self, index: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).RemoveAt)(
                ::windows::core::Vtable::as_raw(this),
                index,
            )
            .ok()
        }
    }
    pub fn Append<'a, P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, Block>>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).Append)(
                ::windows::core::Vtable::as_raw(this),
                value.into().abi(),
            )
            .ok()
        }
    }
    pub fn RemoveAtEnd(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).RemoveAtEnd)(::windows::core::Vtable::as_raw(
                this,
            ))
            .ok()
        }
    }
    pub fn Clear(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).Clear)(::windows::core::Vtable::as_raw(this))
                .ok()
        }
    }
    pub fn GetMany(
        &self,
        startindex: u32,
        items: &mut [::core::option::Option<Block>],
    ) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetMany)(
                ::windows::core::Vtable::as_raw(this),
                startindex,
                items.len() as u32,
                ::core::mem::transmute_copy(&items),
                result__.as_mut_ptr(),
            )
            .from_abi::<u32>(result__)
        }
    }
    pub fn ReplaceAll(
        &self,
        items: &[::core::option::Option<Block>],
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).ReplaceAll)(
                ::windows::core::Vtable::as_raw(this),
                items.len() as u32,
                ::core::mem::transmute(items.as_ptr()),
            )
            .ok()
        }
    }
}
impl ::core::clone::Clone for BlockCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BlockCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BlockCollection {}
impl ::core::fmt::Debug for BlockCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BlockCollection").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for BlockCollection {
    const SIGNATURE : ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice ( b"rc(Microsoft.UI.Xaml.Documents.BlockCollection;pinterface({913337e9-11a1-4345-a3a2-4e7f956e222d};rc(Microsoft.UI.Xaml.Documents.Block;{8149d507-672f-5fd5-a10a-351389ba9659})))" ) ;
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for BlockCollection {
    type Vtable = ::windows::Foundation::Collections::IVector_Vtbl<Block>;
}
unsafe impl ::windows::core::Interface for BlockCollection {
    const IID: ::windows::core::GUID =
        <::windows::Foundation::Collections::IVector<Block> as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for BlockCollection {
    const NAME: &'static str = "Microsoft.UI.Xaml.Documents.BlockCollection";
}
impl ::core::iter::IntoIterator for BlockCollection {
    type Item = Block;
    type IntoIter = ::windows::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
impl ::core::iter::IntoIterator for &BlockCollection {
    type Item = Block;
    type IntoIter = ::windows::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::windows::Foundation::Collections::VectorIterator::new(
            ::core::convert::TryInto::try_into(self).ok(),
        )
    }
}
::windows::core::interface_hierarchy!(
    BlockCollection,
    ::windows::core::IUnknown,
    ::windows::core::IInspectable
);
impl ::core::convert::TryFrom<BlockCollection>
    for ::windows::Foundation::Collections::IIterable<Block>
{
    type Error = ::windows::core::Error;
    fn try_from(value: BlockCollection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&BlockCollection>
    for ::windows::Foundation::Collections::IIterable<Block>
{
    type Error = ::windows::core::Error;
    fn try_from(value: &BlockCollection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&BlockCollection>
    for ::windows::core::InParam<'a, ::windows::Foundation::Collections::IIterable<Block>>
{
    type Error = ::windows::core::Error;
    fn try_from(value: &BlockCollection) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<BlockCollection>
    for ::windows::Foundation::Collections::IVector<Block>
{
    type Error = ::windows::core::Error;
    fn try_from(value: BlockCollection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&BlockCollection>
    for ::windows::Foundation::Collections::IVector<Block>
{
    type Error = ::windows::core::Error;
    fn try_from(value: &BlockCollection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&BlockCollection>
    for ::windows::core::InParam<'a, ::windows::Foundation::Collections::IVector<Block>>
{
    type Error = ::windows::core::Error;
    fn try_from(value: &BlockCollection) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for BlockCollection {}
unsafe impl ::core::marker::Sync for BlockCollection {}
#[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
#[repr(transparent)]
pub struct Bold(::windows::core::IUnknown);
impl Bold {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<Bold, ::windows::core::IGenericFactory> =
            ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn GetValue(
        &self,
        dp: &super::DependencyProperty,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetValue)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(dp),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn SetValue<'a, P0>(
        &self,
        dp: &super::DependencyProperty,
        value: P0,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IInspectable>>,
    {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetValue)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(dp),
                value.into().abi(),
            )
            .ok()
        }
    }
    pub fn ClearValue(&self, dp: &super::DependencyProperty) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).ClearValue)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(dp),
            )
            .ok()
        }
    }
    pub fn ReadLocalValue(
        &self,
        dp: &super::DependencyProperty,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ReadLocalValue)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(dp),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn GetAnimationBaseValue(
        &self,
        dp: &super::DependencyProperty,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetAnimationBaseValue)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(dp),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn RegisterPropertyChangedCallback(
        &self,
        dp: &super::DependencyProperty,
        callback: &super::DependencyPropertyChangedCallback,
    ) -> ::windows::core::Result<i64> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RegisterPropertyChangedCallback)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(dp),
                ::core::mem::transmute_copy(callback),
                result__.as_mut_ptr(),
            )
            .from_abi::<i64>(result__)
        }
    }
    pub fn UnregisterPropertyChangedCallback(
        &self,
        dp: &super::DependencyProperty,
        token: i64,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).UnregisterPropertyChangedCallback)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(dp),
                token,
            )
            .ok()
        }
    }
    pub fn Dispatcher(&self) -> ::windows::core::Result<::windows::UI::Core::CoreDispatcher> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Dispatcher)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Core::CoreDispatcher>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Dispatching\"`*"]
    #[cfg(feature = "UI_Dispatching")]
    pub fn DispatcherQueue(
        &self,
    ) -> ::windows::core::Result<super::super::Dispatching::DispatcherQueue> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DispatcherQueue)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::super::Dispatching::DispatcherQueue>(result__)
        }
    }
    pub fn Inlines(&self) -> ::windows::core::Result<InlineCollection> {
        let this = &::windows::core::Interface::cast::<ISpan>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Inlines)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<InlineCollection>(result__)
        }
    }
    pub fn SetInlines(&self, value: &InlineCollection) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ISpan>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetInlines)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Name)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn FontSize(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FontSize)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetFontSize(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetFontSize)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn FontFamily(&self) -> ::windows::core::Result<super::Media::FontFamily> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FontFamily)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::Media::FontFamily>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetFontFamily<'a, P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::Media::FontFamily>>,
    {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetFontFamily)(
                ::windows::core::Vtable::as_raw(this),
                value.into().abi(),
            )
            .ok()
        }
    }
    pub fn FontWeight(&self) -> ::windows::core::Result<::windows::UI::Text::FontWeight> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FontWeight)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Text::FontWeight>(result__)
        }
    }
    pub fn SetFontWeight(
        &self,
        value: ::windows::UI::Text::FontWeight,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetFontWeight)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn FontStyle(&self) -> ::windows::core::Result<::windows::UI::Text::FontStyle> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FontStyle)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Text::FontStyle>(result__)
        }
    }
    pub fn SetFontStyle(
        &self,
        value: ::windows::UI::Text::FontStyle,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetFontStyle)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn FontStretch(&self) -> ::windows::core::Result<::windows::UI::Text::FontStretch> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FontStretch)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Text::FontStretch>(result__)
        }
    }
    pub fn SetFontStretch(
        &self,
        value: ::windows::UI::Text::FontStretch,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetFontStretch)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn CharacterSpacing(&self) -> ::windows::core::Result<i32> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CharacterSpacing)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<i32>(result__)
        }
    }
    pub fn SetCharacterSpacing(&self, value: i32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetCharacterSpacing)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn Foreground(&self) -> ::windows::core::Result<super::Media::Brush> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Foreground)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::Media::Brush>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetForeground<'a, P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::Media::Brush>>,
    {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetForeground)(
                ::windows::core::Vtable::as_raw(this),
                value.into().abi(),
            )
            .ok()
        }
    }
    pub fn Language(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
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
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetLanguage)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn IsTextScaleFactorEnabled(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsTextScaleFactorEnabled)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetIsTextScaleFactorEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetIsTextScaleFactorEnabled)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn TextDecorations(&self) -> ::windows::core::Result<::windows::UI::Text::TextDecorations> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TextDecorations)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Text::TextDecorations>(result__)
        }
    }
    pub fn SetTextDecorations(
        &self,
        value: ::windows::UI::Text::TextDecorations,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetTextDecorations)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn ContentStart(&self) -> ::windows::core::Result<TextPointer> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ContentStart)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    pub fn ContentEnd(&self) -> ::windows::core::Result<TextPointer> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ContentEnd)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    pub fn ElementStart(&self) -> ::windows::core::Result<TextPointer> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ElementStart)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    pub fn ElementEnd(&self) -> ::windows::core::Result<TextPointer> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ElementEnd)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    pub fn AllowFocusOnInteraction(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AllowFocusOnInteraction)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetAllowFocusOnInteraction(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetAllowFocusOnInteraction)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn AccessKey(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AccessKey)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetAccessKey(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetAccessKey)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn ExitDisplayModeOnAccessKeyInvoked(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ExitDisplayModeOnAccessKeyInvoked)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetExitDisplayModeOnAccessKeyInvoked(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetExitDisplayModeOnAccessKeyInvoked)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsAccessKeyScope(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsAccessKeyScope)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetIsAccessKeyScope(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetIsAccessKeyScope)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn AccessKeyScopeOwner(&self) -> ::windows::core::Result<super::DependencyObject> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AccessKeyScopeOwner)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::DependencyObject>(result__)
        }
    }
    pub fn SetAccessKeyScopeOwner<'a, P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::DependencyObject>>,
    {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetAccessKeyScopeOwner)(
                ::windows::core::Vtable::as_raw(this),
                value.into().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn KeyTipPlacementMode(
        &self,
    ) -> ::windows::core::Result<super::Input::KeyTipPlacementMode> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).KeyTipPlacementMode)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::Input::KeyTipPlacementMode>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetKeyTipPlacementMode(
        &self,
        value: super::Input::KeyTipPlacementMode,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetKeyTipPlacementMode)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn KeyTipHorizontalOffset(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).KeyTipHorizontalOffset)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetKeyTipHorizontalOffset(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetKeyTipHorizontalOffset)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn KeyTipVerticalOffset(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).KeyTipVerticalOffset)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetKeyTipVerticalOffset(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetKeyTipVerticalOffset)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn XamlRoot(&self) -> ::windows::core::Result<super::XamlRoot> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).XamlRoot)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::XamlRoot>(result__)
        }
    }
    pub fn SetXamlRoot(&self, value: &super::XamlRoot) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetXamlRoot)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn AccessKeyDisplayRequested(
        &self,
        handler: &::windows::Foundation::TypedEventHandler<
            TextElement,
            super::Input::AccessKeyDisplayRequestedEventArgs,
        >,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AccessKeyDisplayRequested)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(handler),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveAccessKeyDisplayRequested(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).RemoveAccessKeyDisplayRequested)(
                ::windows::core::Vtable::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn AccessKeyDisplayDismissed(
        &self,
        handler: &::windows::Foundation::TypedEventHandler<
            TextElement,
            super::Input::AccessKeyDisplayDismissedEventArgs,
        >,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AccessKeyDisplayDismissed)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(handler),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveAccessKeyDisplayDismissed(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).RemoveAccessKeyDisplayDismissed)(
                ::windows::core::Vtable::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn AccessKeyInvoked(
        &self,
        handler: &::windows::Foundation::TypedEventHandler<
            TextElement,
            super::Input::AccessKeyInvokedEventArgs,
        >,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AccessKeyInvoked)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(handler),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveAccessKeyInvoked(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).RemoveAccessKeyInvoked)(
                ::windows::core::Vtable::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn FindName(
        &self,
        name: &::windows::core::HSTRING,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FindName)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(name),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
}
impl ::core::clone::Clone for Bold {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for Bold {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Bold {}
impl ::core::fmt::Debug for Bold {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Bold").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for Bold {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Xaml.Documents.Bold;{241a5f5a-c164-597f-b0db-fac7431297f2})",
    );
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for Bold {
    type Vtable = IBold_Vtbl;
}
unsafe impl ::windows::core::Interface for Bold {
    const IID: ::windows::core::GUID = <IBold as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for Bold {
    const NAME: &'static str = "Microsoft.UI.Xaml.Documents.Bold";
}
::windows::core::interface_hierarchy!(
    Bold,
    ::windows::core::IUnknown,
    ::windows::core::IInspectable
);
impl ::core::convert::From<Bold> for Span {
    fn from(value: Bold) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&Bold> for Span {
    fn from(value: &Bold) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::core::convert::From<&Bold> for ::windows::core::InParam<'a, Span> {
    fn from(value: &Bold) -> Self {
        ::windows::core::InParam::owned(value.into())
    }
}
impl ::core::convert::From<Bold> for Inline {
    fn from(value: Bold) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&Bold> for Inline {
    fn from(value: &Bold) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::core::convert::From<&Bold> for ::windows::core::InParam<'a, Inline> {
    fn from(value: &Bold) -> Self {
        ::windows::core::InParam::owned(value.into())
    }
}
impl ::core::convert::From<Bold> for TextElement {
    fn from(value: Bold) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&Bold> for TextElement {
    fn from(value: &Bold) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::core::convert::From<&Bold> for ::windows::core::InParam<'a, TextElement> {
    fn from(value: &Bold) -> Self {
        ::windows::core::InParam::owned(value.into())
    }
}
impl ::core::convert::From<Bold> for super::DependencyObject {
    fn from(value: Bold) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&Bold> for super::DependencyObject {
    fn from(value: &Bold) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::core::convert::From<&Bold> for ::windows::core::InParam<'a, super::DependencyObject> {
    fn from(value: &Bold) -> Self {
        ::windows::core::InParam::owned(value.into())
    }
}
unsafe impl ::core::marker::Send for Bold {}
unsafe impl ::core::marker::Sync for Bold {}
#[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
#[repr(transparent)]
pub struct Glyphs(::windows::core::IUnknown);
impl Glyphs {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<Glyphs, ::windows::core::IGenericFactory> =
            ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"UI_Composition\"`*"]
    #[cfg(feature = "UI_Composition")]
    pub fn PopulatePropertyInfo(
        &self,
        propertyname: &::windows::core::HSTRING,
        propertyinfo: &super::super::Composition::AnimationPropertyInfo,
    ) -> ::windows::core::Result<()> {
        let this =
            &::windows::core::Interface::cast::<super::super::Composition::IAnimationObject>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).PopulatePropertyInfo)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(propertyname),
                ::core::mem::transmute_copy(propertyinfo),
            )
            .ok()
        }
    }
    pub fn GetValue(
        &self,
        dp: &super::DependencyProperty,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetValue)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(dp),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn SetValue<'a, P0>(
        &self,
        dp: &super::DependencyProperty,
        value: P0,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IInspectable>>,
    {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetValue)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(dp),
                value.into().abi(),
            )
            .ok()
        }
    }
    pub fn ClearValue(&self, dp: &super::DependencyProperty) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).ClearValue)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(dp),
            )
            .ok()
        }
    }
    pub fn ReadLocalValue(
        &self,
        dp: &super::DependencyProperty,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ReadLocalValue)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(dp),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn GetAnimationBaseValue(
        &self,
        dp: &super::DependencyProperty,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetAnimationBaseValue)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(dp),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn RegisterPropertyChangedCallback(
        &self,
        dp: &super::DependencyProperty,
        callback: &super::DependencyPropertyChangedCallback,
    ) -> ::windows::core::Result<i64> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RegisterPropertyChangedCallback)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(dp),
                ::core::mem::transmute_copy(callback),
                result__.as_mut_ptr(),
            )
            .from_abi::<i64>(result__)
        }
    }
    pub fn UnregisterPropertyChangedCallback(
        &self,
        dp: &super::DependencyProperty,
        token: i64,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).UnregisterPropertyChangedCallback)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(dp),
                token,
            )
            .ok()
        }
    }
    pub fn Dispatcher(&self) -> ::windows::core::Result<::windows::UI::Core::CoreDispatcher> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Dispatcher)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Core::CoreDispatcher>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Dispatching\"`*"]
    #[cfg(feature = "UI_Dispatching")]
    pub fn DispatcherQueue(
        &self,
    ) -> ::windows::core::Result<super::super::Dispatching::DispatcherQueue> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DispatcherQueue)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::super::Dispatching::DispatcherQueue>(result__)
        }
    }
    pub fn Triggers(&self) -> ::windows::core::Result<super::TriggerCollection> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Triggers)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::TriggerCollection>(result__)
        }
    }
    pub fn Resources(&self) -> ::windows::core::Result<super::ResourceDictionary> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Resources)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::ResourceDictionary>(result__)
        }
    }
    pub fn SetResources<'a, P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::ResourceDictionary>>,
    {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetResources)(
                ::windows::core::Vtable::as_raw(this),
                value.into().abi(),
            )
            .ok()
        }
    }
    pub fn Tag(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Tag)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn SetTag<'a, P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IInspectable>>,
    {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetTag)(
                ::windows::core::Vtable::as_raw(this),
                value.into().abi(),
            )
            .ok()
        }
    }
    pub fn Language(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
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
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetLanguage)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn ActualWidth(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ActualWidth)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn ActualHeight(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ActualHeight)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn Width(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Width)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetWidth(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetWidth)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn Height(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Height)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetHeight(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetHeight)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn MinWidth(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MinWidth)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetMinWidth(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetMinWidth)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn MaxWidth(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MaxWidth)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetMaxWidth(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetMaxWidth)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn MinHeight(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MinHeight)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetMinHeight(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetMinHeight)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn MaxHeight(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MaxHeight)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetMaxHeight(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetMaxHeight)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn HorizontalAlignment(&self) -> ::windows::core::Result<super::HorizontalAlignment> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).HorizontalAlignment)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::HorizontalAlignment>(result__)
        }
    }
    pub fn SetHorizontalAlignment(
        &self,
        value: super::HorizontalAlignment,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetHorizontalAlignment)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn VerticalAlignment(&self) -> ::windows::core::Result<super::VerticalAlignment> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).VerticalAlignment)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::VerticalAlignment>(result__)
        }
    }
    pub fn SetVerticalAlignment(
        &self,
        value: super::VerticalAlignment,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetVerticalAlignment)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn Margin(&self) -> ::windows::core::Result<super::Thickness> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Margin)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::Thickness>(result__)
        }
    }
    pub fn SetMargin(&self, value: super::Thickness) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetMargin)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Name)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetName)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn BaseUri(&self) -> ::windows::core::Result<::windows::Foundation::Uri> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).BaseUri)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::Uri>(result__)
        }
    }
    pub fn DataContext(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DataContext)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn SetDataContext<'a, P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IInspectable>>,
    {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetDataContext)(
                ::windows::core::Vtable::as_raw(this),
                value.into().abi(),
            )
            .ok()
        }
    }
    pub fn AllowFocusOnInteraction(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AllowFocusOnInteraction)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetAllowFocusOnInteraction(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetAllowFocusOnInteraction)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn FocusVisualMargin(&self) -> ::windows::core::Result<super::Thickness> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FocusVisualMargin)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::Thickness>(result__)
        }
    }
    pub fn SetFocusVisualMargin(&self, value: super::Thickness) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetFocusVisualMargin)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn FocusVisualSecondaryThickness(&self) -> ::windows::core::Result<super::Thickness> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FocusVisualSecondaryThickness)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::Thickness>(result__)
        }
    }
    pub fn SetFocusVisualSecondaryThickness(
        &self,
        value: super::Thickness,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetFocusVisualSecondaryThickness)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn FocusVisualPrimaryThickness(&self) -> ::windows::core::Result<super::Thickness> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FocusVisualPrimaryThickness)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::Thickness>(result__)
        }
    }
    pub fn SetFocusVisualPrimaryThickness(
        &self,
        value: super::Thickness,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetFocusVisualPrimaryThickness)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn FocusVisualSecondaryBrush(&self) -> ::windows::core::Result<super::Media::Brush> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FocusVisualSecondaryBrush)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::Media::Brush>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetFocusVisualSecondaryBrush<'a, P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::Media::Brush>>,
    {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetFocusVisualSecondaryBrush)(
                ::windows::core::Vtable::as_raw(this),
                value.into().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn FocusVisualPrimaryBrush(&self) -> ::windows::core::Result<super::Media::Brush> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FocusVisualPrimaryBrush)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::Media::Brush>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetFocusVisualPrimaryBrush<'a, P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::Media::Brush>>,
    {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetFocusVisualPrimaryBrush)(
                ::windows::core::Vtable::as_raw(this),
                value.into().abi(),
            )
            .ok()
        }
    }
    pub fn AllowFocusWhenDisabled(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AllowFocusWhenDisabled)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetAllowFocusWhenDisabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetAllowFocusWhenDisabled)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn Style(&self) -> ::windows::core::Result<super::Style> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Style)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::Style>(result__)
        }
    }
    pub fn SetStyle(&self, value: &super::Style) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetStyle)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn Parent(&self) -> ::windows::core::Result<super::DependencyObject> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Parent)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::DependencyObject>(result__)
        }
    }
    pub fn FlowDirection(&self) -> ::windows::core::Result<super::FlowDirection> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FlowDirection)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::FlowDirection>(result__)
        }
    }
    pub fn SetFlowDirection(&self, value: super::FlowDirection) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetFlowDirection)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn RequestedTheme(&self) -> ::windows::core::Result<super::ElementTheme> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RequestedTheme)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::ElementTheme>(result__)
        }
    }
    pub fn SetRequestedTheme(&self, value: super::ElementTheme) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetRequestedTheme)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsLoaded(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsLoaded)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn ActualTheme(&self) -> ::windows::core::Result<super::ElementTheme> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ActualTheme)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::ElementTheme>(result__)
        }
    }
    pub fn Loaded(
        &self,
        handler: &super::RoutedEventHandler,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Loaded)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(handler),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveLoaded(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).RemoveLoaded)(
                ::windows::core::Vtable::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn Unloaded(
        &self,
        handler: &super::RoutedEventHandler,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Unloaded)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(handler),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveUnloaded(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).RemoveUnloaded)(
                ::windows::core::Vtable::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn DataContextChanged(
        &self,
        handler: &::windows::Foundation::TypedEventHandler<
            super::FrameworkElement,
            super::DataContextChangedEventArgs,
        >,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DataContextChanged)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(handler),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveDataContextChanged(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).RemoveDataContextChanged)(
                ::windows::core::Vtable::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn SizeChanged(
        &self,
        handler: &super::SizeChangedEventHandler,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SizeChanged)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(handler),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveSizeChanged(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).RemoveSizeChanged)(
                ::windows::core::Vtable::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn LayoutUpdated(
        &self,
        handler: &::windows::Foundation::EventHandler<::windows::core::IInspectable>,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LayoutUpdated)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(handler),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveLayoutUpdated(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).RemoveLayoutUpdated)(
                ::windows::core::Vtable::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn Loading(
        &self,
        handler: &::windows::Foundation::TypedEventHandler<
            super::FrameworkElement,
            ::windows::core::IInspectable,
        >,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Loading)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(handler),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveLoading(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).RemoveLoading)(
                ::windows::core::Vtable::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn ActualThemeChanged(
        &self,
        handler: &::windows::Foundation::TypedEventHandler<
            super::FrameworkElement,
            ::windows::core::IInspectable,
        >,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ActualThemeChanged)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(handler),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveActualThemeChanged(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).RemoveActualThemeChanged)(
                ::windows::core::Vtable::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn EffectiveViewportChanged(
        &self,
        handler: &::windows::Foundation::TypedEventHandler<
            super::FrameworkElement,
            super::EffectiveViewportChangedEventArgs,
        >,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).EffectiveViewportChanged)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(handler),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveEffectiveViewportChanged(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).RemoveEffectiveViewportChanged)(
                ::windows::core::Vtable::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn FindName(
        &self,
        name: &::windows::core::HSTRING,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FindName)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(name),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    #[cfg(feature = "UI_Xaml_Data")]
    pub fn SetBinding<'a, P0>(
        &self,
        dp: &super::DependencyProperty,
        binding: P0,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::Data::BindingBase>>,
    {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetBinding)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(dp),
                binding.into().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    #[cfg(feature = "UI_Xaml_Data")]
    pub fn GetBindingExpression(
        &self,
        dp: &super::DependencyProperty,
    ) -> ::windows::core::Result<super::Data::BindingExpression> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetBindingExpression)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(dp),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::Data::BindingExpression>(result__)
        }
    }
    pub fn InvalidateViewport(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IFrameworkElementProtected>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).InvalidateViewport)(
                ::windows::core::Vtable::as_raw(this),
            )
            .ok()
        }
    }
    pub fn UnicodeString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).UnicodeString)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetUnicodeString(
        &self,
        value: &::windows::core::HSTRING,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetUnicodeString)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn Indices(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Indices)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetIndices(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetIndices)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn FontUri(&self) -> ::windows::core::Result<::windows::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FontUri)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::Uri>(result__)
        }
    }
    pub fn SetFontUri(&self, value: &::windows::Foundation::Uri) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetFontUri)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn StyleSimulations(&self) -> ::windows::core::Result<super::Media::StyleSimulations> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).StyleSimulations)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::Media::StyleSimulations>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetStyleSimulations(
        &self,
        value: super::Media::StyleSimulations,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetStyleSimulations)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn FontRenderingEmSize(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FontRenderingEmSize)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetFontRenderingEmSize(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetFontRenderingEmSize)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn OriginX(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).OriginX)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetOriginX(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetOriginX)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn OriginY(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).OriginY)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetOriginY(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetOriginY)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn Fill(&self) -> ::windows::core::Result<super::Media::Brush> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Fill)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::Media::Brush>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetFill<'a, P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::Media::Brush>>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetFill)(
                ::windows::core::Vtable::as_raw(this),
                value.into().abi(),
            )
            .ok()
        }
    }
    pub fn IsColorFontEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsColorFontEnabled)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetIsColorFontEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetIsColorFontEnabled)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn ColorFontPaletteIndex(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ColorFontPaletteIndex)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<i32>(result__)
        }
    }
    pub fn SetColorFontPaletteIndex(&self, value: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetColorFontPaletteIndex)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn UnicodeStringProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IGlyphsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).UnicodeStringProperty)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn IndicesProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IGlyphsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IndicesProperty)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn FontUriProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IGlyphsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FontUriProperty)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn StyleSimulationsProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IGlyphsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).StyleSimulationsProperty)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn FontRenderingEmSizeProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IGlyphsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FontRenderingEmSizeProperty)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn OriginXProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IGlyphsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).OriginXProperty)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn OriginYProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IGlyphsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).OriginYProperty)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn FillProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IGlyphsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FillProperty)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn IsColorFontEnabledProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IGlyphsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsColorFontEnabledProperty)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn ColorFontPaletteIndexProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IGlyphsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ColorFontPaletteIndexProperty)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn DesiredSize(&self) -> ::windows::core::Result<::windows::Foundation::Size> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DesiredSize)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::Size>(result__)
        }
    }
    pub fn AllowDrop(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AllowDrop)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetAllowDrop(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetAllowDrop)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn Opacity(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Opacity)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetOpacity(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetOpacity)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn Clip(&self) -> ::windows::core::Result<super::Media::RectangleGeometry> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Clip)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::Media::RectangleGeometry>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetClip(&self, value: &super::Media::RectangleGeometry) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetClip)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn RenderTransform(&self) -> ::windows::core::Result<super::Media::Transform> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RenderTransform)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::Media::Transform>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetRenderTransform<'a, P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::Media::Transform>>,
    {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetRenderTransform)(
                ::windows::core::Vtable::as_raw(this),
                value.into().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn Projection(&self) -> ::windows::core::Result<super::Media::Projection> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Projection)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::Media::Projection>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetProjection<'a, P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::Media::Projection>>,
    {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetProjection)(
                ::windows::core::Vtable::as_raw(this),
                value.into().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Media_Media3D\"`*"]
    #[cfg(feature = "UI_Xaml_Media_Media3D")]
    pub fn Transform3D(&self) -> ::windows::core::Result<super::Media::Media3D::Transform3D> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Transform3D)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::Media::Media3D::Transform3D>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Media_Media3D\"`*"]
    #[cfg(feature = "UI_Xaml_Media_Media3D")]
    pub fn SetTransform3D<'a, P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::Media::Media3D::Transform3D>>,
    {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetTransform3D)(
                ::windows::core::Vtable::as_raw(this),
                value.into().abi(),
            )
            .ok()
        }
    }
    pub fn RenderTransformOrigin(&self) -> ::windows::core::Result<::windows::Foundation::Point> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RenderTransformOrigin)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::Point>(result__)
        }
    }
    pub fn SetRenderTransformOrigin(
        &self,
        value: ::windows::Foundation::Point,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetRenderTransformOrigin)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsHitTestVisible(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsHitTestVisible)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetIsHitTestVisible(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetIsHitTestVisible)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn Visibility(&self) -> ::windows::core::Result<super::Visibility> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Visibility)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::Visibility>(result__)
        }
    }
    pub fn SetVisibility(&self, value: super::Visibility) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetVisibility)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn RenderSize(&self) -> ::windows::core::Result<::windows::Foundation::Size> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RenderSize)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::Size>(result__)
        }
    }
    pub fn UseLayoutRounding(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).UseLayoutRounding)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetUseLayoutRounding(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetUseLayoutRounding)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Media_Animation\"`*"]
    #[cfg(feature = "UI_Xaml_Media_Animation")]
    pub fn Transitions(
        &self,
    ) -> ::windows::core::Result<super::Media::Animation::TransitionCollection> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Transitions)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::Media::Animation::TransitionCollection>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Media_Animation\"`*"]
    #[cfg(feature = "UI_Xaml_Media_Animation")]
    pub fn SetTransitions(
        &self,
        value: &super::Media::Animation::TransitionCollection,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetTransitions)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn CacheMode(&self) -> ::windows::core::Result<super::Media::CacheMode> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CacheMode)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::Media::CacheMode>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetCacheMode<'a, P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::Media::CacheMode>>,
    {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetCacheMode)(
                ::windows::core::Vtable::as_raw(this),
                value.into().abi(),
            )
            .ok()
        }
    }
    pub fn IsTapEnabled(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsTapEnabled)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetIsTapEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetIsTapEnabled)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsDoubleTapEnabled(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsDoubleTapEnabled)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetIsDoubleTapEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetIsDoubleTapEnabled)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn CanDrag(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CanDrag)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetCanDrag(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetCanDrag)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsRightTapEnabled(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsRightTapEnabled)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetIsRightTapEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetIsRightTapEnabled)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsHoldingEnabled(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsHoldingEnabled)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetIsHoldingEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetIsHoldingEnabled)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn ManipulationMode(&self) -> ::windows::core::Result<super::Input::ManipulationModes> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ManipulationMode)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::Input::ManipulationModes>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetManipulationMode(
        &self,
        value: super::Input::ManipulationModes,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetManipulationMode)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn PointerCaptures(
        &self,
    ) -> ::windows::core::Result<
        ::windows::Foundation::Collections::IVectorView<super::Input::Pointer>,
    > {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PointerCaptures)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::Collections::IVectorView<super::Input::Pointer>>(
                result__,
            )
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Primitives\"`*"]
    #[cfg(feature = "UI_Xaml_Controls_Primitives")]
    pub fn ContextFlyout(
        &self,
    ) -> ::windows::core::Result<super::Controls::Primitives::FlyoutBase> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ContextFlyout)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::Controls::Primitives::FlyoutBase>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_Primitives\"`*"]
    #[cfg(feature = "UI_Xaml_Controls_Primitives")]
    pub fn SetContextFlyout<'a, P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<
            ::windows::core::InParam<'a, super::Controls::Primitives::FlyoutBase>,
        >,
    {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetContextFlyout)(
                ::windows::core::Vtable::as_raw(this),
                value.into().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn CompositeMode(&self) -> ::windows::core::Result<super::Media::ElementCompositeMode> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CompositeMode)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::Media::ElementCompositeMode>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetCompositeMode(
        &self,
        value: super::Media::ElementCompositeMode,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetCompositeMode)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn Lights(
        &self,
    ) -> ::windows::core::Result<::windows::Foundation::Collections::IVector<super::Media::XamlLight>>
    {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Lights)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::Collections::IVector<super::Media::XamlLight>>(
                result__,
            )
        }
    }
    pub fn CanBeScrollAnchor(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CanBeScrollAnchor)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetCanBeScrollAnchor(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetCanBeScrollAnchor)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn ExitDisplayModeOnAccessKeyInvoked(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ExitDisplayModeOnAccessKeyInvoked)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetExitDisplayModeOnAccessKeyInvoked(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetExitDisplayModeOnAccessKeyInvoked)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsAccessKeyScope(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsAccessKeyScope)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetIsAccessKeyScope(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetIsAccessKeyScope)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn AccessKeyScopeOwner(&self) -> ::windows::core::Result<super::DependencyObject> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AccessKeyScopeOwner)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::DependencyObject>(result__)
        }
    }
    pub fn SetAccessKeyScopeOwner<'a, P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::DependencyObject>>,
    {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetAccessKeyScopeOwner)(
                ::windows::core::Vtable::as_raw(this),
                value.into().abi(),
            )
            .ok()
        }
    }
    pub fn AccessKey(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AccessKey)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetAccessKey(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetAccessKey)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn KeyTipPlacementMode(
        &self,
    ) -> ::windows::core::Result<super::Input::KeyTipPlacementMode> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).KeyTipPlacementMode)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::Input::KeyTipPlacementMode>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetKeyTipPlacementMode(
        &self,
        value: super::Input::KeyTipPlacementMode,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetKeyTipPlacementMode)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn KeyTipHorizontalOffset(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).KeyTipHorizontalOffset)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetKeyTipHorizontalOffset(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetKeyTipHorizontalOffset)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn KeyTipVerticalOffset(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).KeyTipVerticalOffset)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetKeyTipVerticalOffset(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetKeyTipVerticalOffset)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn KeyTipTarget(&self) -> ::windows::core::Result<super::DependencyObject> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).KeyTipTarget)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::DependencyObject>(result__)
        }
    }
    pub fn SetKeyTipTarget<'a, P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::DependencyObject>>,
    {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetKeyTipTarget)(
                ::windows::core::Vtable::as_raw(this),
                value.into().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn XYFocusKeyboardNavigation(
        &self,
    ) -> ::windows::core::Result<super::Input::XYFocusKeyboardNavigationMode> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).XYFocusKeyboardNavigation)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::Input::XYFocusKeyboardNavigationMode>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetXYFocusKeyboardNavigation(
        &self,
        value: super::Input::XYFocusKeyboardNavigationMode,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetXYFocusKeyboardNavigation)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn XYFocusUpNavigationStrategy(
        &self,
    ) -> ::windows::core::Result<super::Input::XYFocusNavigationStrategy> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).XYFocusUpNavigationStrategy)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::Input::XYFocusNavigationStrategy>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetXYFocusUpNavigationStrategy(
        &self,
        value: super::Input::XYFocusNavigationStrategy,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetXYFocusUpNavigationStrategy)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn XYFocusDownNavigationStrategy(
        &self,
    ) -> ::windows::core::Result<super::Input::XYFocusNavigationStrategy> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).XYFocusDownNavigationStrategy)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::Input::XYFocusNavigationStrategy>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetXYFocusDownNavigationStrategy(
        &self,
        value: super::Input::XYFocusNavigationStrategy,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetXYFocusDownNavigationStrategy)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn XYFocusLeftNavigationStrategy(
        &self,
    ) -> ::windows::core::Result<super::Input::XYFocusNavigationStrategy> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).XYFocusLeftNavigationStrategy)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::Input::XYFocusNavigationStrategy>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetXYFocusLeftNavigationStrategy(
        &self,
        value: super::Input::XYFocusNavigationStrategy,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetXYFocusLeftNavigationStrategy)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn XYFocusRightNavigationStrategy(
        &self,
    ) -> ::windows::core::Result<super::Input::XYFocusNavigationStrategy> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).XYFocusRightNavigationStrategy)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::Input::XYFocusNavigationStrategy>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetXYFocusRightNavigationStrategy(
        &self,
        value: super::Input::XYFocusNavigationStrategy,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetXYFocusRightNavigationStrategy)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn KeyboardAccelerators(
        &self,
    ) -> ::windows::core::Result<
        ::windows::Foundation::Collections::IVector<super::Input::KeyboardAccelerator>,
    > {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).KeyboardAccelerators)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::Collections::IVector<
                super::Input::KeyboardAccelerator,
            >>(result__)
        }
    }
    pub fn KeyboardAcceleratorPlacementTarget(
        &self,
    ) -> ::windows::core::Result<super::DependencyObject> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).KeyboardAcceleratorPlacementTarget)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::DependencyObject>(result__)
        }
    }
    pub fn SetKeyboardAcceleratorPlacementTarget<'a, P0>(
        &self,
        value: P0,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::DependencyObject>>,
    {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetKeyboardAcceleratorPlacementTarget)(
                ::windows::core::Vtable::as_raw(this),
                value.into().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn KeyboardAcceleratorPlacementMode(
        &self,
    ) -> ::windows::core::Result<super::Input::KeyboardAcceleratorPlacementMode> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).KeyboardAcceleratorPlacementMode)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::Input::KeyboardAcceleratorPlacementMode>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetKeyboardAcceleratorPlacementMode(
        &self,
        value: super::Input::KeyboardAcceleratorPlacementMode,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetKeyboardAcceleratorPlacementMode)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn HighContrastAdjustment(
        &self,
    ) -> ::windows::core::Result<super::ElementHighContrastAdjustment> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).HighContrastAdjustment)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::ElementHighContrastAdjustment>(result__)
        }
    }
    pub fn SetHighContrastAdjustment(
        &self,
        value: super::ElementHighContrastAdjustment,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetHighContrastAdjustment)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn TabFocusNavigation(
        &self,
    ) -> ::windows::core::Result<super::Input::KeyboardNavigationMode> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TabFocusNavigation)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::Input::KeyboardNavigationMode>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetTabFocusNavigation(
        &self,
        value: super::Input::KeyboardNavigationMode,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetTabFocusNavigation)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn OpacityTransition(&self) -> ::windows::core::Result<super::ScalarTransition> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).OpacityTransition)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::ScalarTransition>(result__)
        }
    }
    pub fn SetOpacityTransition<'a, P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::ScalarTransition>>,
    {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetOpacityTransition)(
                ::windows::core::Vtable::as_raw(this),
                value.into().abi(),
            )
            .ok()
        }
    }
    pub fn Translation(&self) -> ::windows::core::Result<::windows::Foundation::Numerics::Vector3> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Translation)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::Numerics::Vector3>(result__)
        }
    }
    pub fn SetTranslation(
        &self,
        value: ::windows::Foundation::Numerics::Vector3,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetTranslation)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn TranslationTransition(&self) -> ::windows::core::Result<super::Vector3Transition> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TranslationTransition)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::Vector3Transition>(result__)
        }
    }
    pub fn SetTranslationTransition<'a, P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::Vector3Transition>>,
    {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetTranslationTransition)(
                ::windows::core::Vtable::as_raw(this),
                value.into().abi(),
            )
            .ok()
        }
    }
    pub fn Rotation(&self) -> ::windows::core::Result<f32> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Rotation)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<f32>(result__)
        }
    }
    pub fn SetRotation(&self, value: f32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetRotation)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn RotationTransition(&self) -> ::windows::core::Result<super::ScalarTransition> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RotationTransition)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::ScalarTransition>(result__)
        }
    }
    pub fn SetRotationTransition<'a, P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::ScalarTransition>>,
    {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetRotationTransition)(
                ::windows::core::Vtable::as_raw(this),
                value.into().abi(),
            )
            .ok()
        }
    }
    pub fn Scale(&self) -> ::windows::core::Result<::windows::Foundation::Numerics::Vector3> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Scale)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::Numerics::Vector3>(result__)
        }
    }
    pub fn SetScale(
        &self,
        value: ::windows::Foundation::Numerics::Vector3,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetScale)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn ScaleTransition(&self) -> ::windows::core::Result<super::Vector3Transition> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ScaleTransition)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::Vector3Transition>(result__)
        }
    }
    pub fn SetScaleTransition<'a, P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::Vector3Transition>>,
    {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetScaleTransition)(
                ::windows::core::Vtable::as_raw(this),
                value.into().abi(),
            )
            .ok()
        }
    }
    pub fn TransformMatrix(
        &self,
    ) -> ::windows::core::Result<::windows::Foundation::Numerics::Matrix4x4> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TransformMatrix)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::Numerics::Matrix4x4>(result__)
        }
    }
    pub fn SetTransformMatrix(
        &self,
        value: ::windows::Foundation::Numerics::Matrix4x4,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetTransformMatrix)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn CenterPoint(&self) -> ::windows::core::Result<::windows::Foundation::Numerics::Vector3> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CenterPoint)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::Numerics::Vector3>(result__)
        }
    }
    pub fn SetCenterPoint(
        &self,
        value: ::windows::Foundation::Numerics::Vector3,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetCenterPoint)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn RotationAxis(
        &self,
    ) -> ::windows::core::Result<::windows::Foundation::Numerics::Vector3> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RotationAxis)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::Numerics::Vector3>(result__)
        }
    }
    pub fn SetRotationAxis(
        &self,
        value: ::windows::Foundation::Numerics::Vector3,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetRotationAxis)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn ActualOffset(
        &self,
    ) -> ::windows::core::Result<::windows::Foundation::Numerics::Vector3> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ActualOffset)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::Numerics::Vector3>(result__)
        }
    }
    pub fn ActualSize(&self) -> ::windows::core::Result<::windows::Foundation::Numerics::Vector2> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ActualSize)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::Numerics::Vector2>(result__)
        }
    }
    pub fn XamlRoot(&self) -> ::windows::core::Result<super::XamlRoot> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).XamlRoot)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::XamlRoot>(result__)
        }
    }
    pub fn SetXamlRoot(&self, value: &super::XamlRoot) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetXamlRoot)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn Shadow(&self) -> ::windows::core::Result<super::Media::Shadow> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Shadow)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::Media::Shadow>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetShadow<'a, P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::Media::Shadow>>,
    {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetShadow)(
                ::windows::core::Vtable::as_raw(this),
                value.into().abi(),
            )
            .ok()
        }
    }
    pub fn RasterizationScale(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RasterizationScale)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetRasterizationScale(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetRasterizationScale)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn FocusState(&self) -> ::windows::core::Result<super::FocusState> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FocusState)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::FocusState>(result__)
        }
    }
    pub fn UseSystemFocusVisuals(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).UseSystemFocusVisuals)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetUseSystemFocusVisuals(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetUseSystemFocusVisuals)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn XYFocusLeft(&self) -> ::windows::core::Result<super::DependencyObject> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).XYFocusLeft)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::DependencyObject>(result__)
        }
    }
    pub fn SetXYFocusLeft<'a, P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::DependencyObject>>,
    {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetXYFocusLeft)(
                ::windows::core::Vtable::as_raw(this),
                value.into().abi(),
            )
            .ok()
        }
    }
    pub fn XYFocusRight(&self) -> ::windows::core::Result<super::DependencyObject> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).XYFocusRight)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::DependencyObject>(result__)
        }
    }
    pub fn SetXYFocusRight<'a, P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::DependencyObject>>,
    {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetXYFocusRight)(
                ::windows::core::Vtable::as_raw(this),
                value.into().abi(),
            )
            .ok()
        }
    }
    pub fn XYFocusUp(&self) -> ::windows::core::Result<super::DependencyObject> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).XYFocusUp)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::DependencyObject>(result__)
        }
    }
    pub fn SetXYFocusUp<'a, P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::DependencyObject>>,
    {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetXYFocusUp)(
                ::windows::core::Vtable::as_raw(this),
                value.into().abi(),
            )
            .ok()
        }
    }
    pub fn XYFocusDown(&self) -> ::windows::core::Result<super::DependencyObject> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).XYFocusDown)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::DependencyObject>(result__)
        }
    }
    pub fn SetXYFocusDown<'a, P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::DependencyObject>>,
    {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetXYFocusDown)(
                ::windows::core::Vtable::as_raw(this),
                value.into().abi(),
            )
            .ok()
        }
    }
    pub fn IsTabStop(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsTabStop)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetIsTabStop(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetIsTabStop)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn TabIndex(&self) -> ::windows::core::Result<i32> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TabIndex)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<i32>(result__)
        }
    }
    pub fn SetTabIndex(&self, value: i32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetTabIndex)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn KeyUp(
        &self,
        handler: &super::Input::KeyEventHandler,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).KeyUp)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(handler),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveKeyUp(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).RemoveKeyUp)(
                ::windows::core::Vtable::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn KeyDown(
        &self,
        handler: &super::Input::KeyEventHandler,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).KeyDown)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(handler),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveKeyDown(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).RemoveKeyDown)(
                ::windows::core::Vtable::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn GotFocus(
        &self,
        handler: &super::RoutedEventHandler,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GotFocus)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(handler),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveGotFocus(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).RemoveGotFocus)(
                ::windows::core::Vtable::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn LostFocus(
        &self,
        handler: &super::RoutedEventHandler,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LostFocus)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(handler),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveLostFocus(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).RemoveLostFocus)(
                ::windows::core::Vtable::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn DragStarting(
        &self,
        handler: &::windows::Foundation::TypedEventHandler<
            super::UIElement,
            super::DragStartingEventArgs,
        >,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DragStarting)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(handler),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveDragStarting(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).RemoveDragStarting)(
                ::windows::core::Vtable::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn DropCompleted(
        &self,
        handler: &::windows::Foundation::TypedEventHandler<
            super::UIElement,
            super::DropCompletedEventArgs,
        >,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DropCompleted)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(handler),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveDropCompleted(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).RemoveDropCompleted)(
                ::windows::core::Vtable::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn CharacterReceived(
        &self,
        handler: &::windows::Foundation::TypedEventHandler<
            super::UIElement,
            super::Input::CharacterReceivedRoutedEventArgs,
        >,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CharacterReceived)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(handler),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveCharacterReceived(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).RemoveCharacterReceived)(
                ::windows::core::Vtable::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn DragEnter(
        &self,
        handler: &super::DragEventHandler,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DragEnter)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(handler),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveDragEnter(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).RemoveDragEnter)(
                ::windows::core::Vtable::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn DragLeave(
        &self,
        handler: &super::DragEventHandler,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DragLeave)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(handler),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveDragLeave(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).RemoveDragLeave)(
                ::windows::core::Vtable::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn DragOver(
        &self,
        handler: &super::DragEventHandler,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DragOver)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(handler),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveDragOver(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).RemoveDragOver)(
                ::windows::core::Vtable::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn Drop(
        &self,
        handler: &super::DragEventHandler,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Drop)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(handler),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveDrop(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).RemoveDrop)(
                ::windows::core::Vtable::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn PointerPressed(
        &self,
        handler: &super::Input::PointerEventHandler,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PointerPressed)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(handler),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemovePointerPressed(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).RemovePointerPressed)(
                ::windows::core::Vtable::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn PointerMoved(
        &self,
        handler: &super::Input::PointerEventHandler,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PointerMoved)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(handler),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemovePointerMoved(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).RemovePointerMoved)(
                ::windows::core::Vtable::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn PointerReleased(
        &self,
        handler: &super::Input::PointerEventHandler,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PointerReleased)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(handler),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemovePointerReleased(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).RemovePointerReleased)(
                ::windows::core::Vtable::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn PointerEntered(
        &self,
        handler: &super::Input::PointerEventHandler,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PointerEntered)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(handler),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemovePointerEntered(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).RemovePointerEntered)(
                ::windows::core::Vtable::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn PointerExited(
        &self,
        handler: &super::Input::PointerEventHandler,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PointerExited)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(handler),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemovePointerExited(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).RemovePointerExited)(
                ::windows::core::Vtable::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn PointerCaptureLost(
        &self,
        handler: &super::Input::PointerEventHandler,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PointerCaptureLost)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(handler),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemovePointerCaptureLost(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).RemovePointerCaptureLost)(
                ::windows::core::Vtable::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn PointerCanceled(
        &self,
        handler: &super::Input::PointerEventHandler,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PointerCanceled)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(handler),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemovePointerCanceled(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).RemovePointerCanceled)(
                ::windows::core::Vtable::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn PointerWheelChanged(
        &self,
        handler: &super::Input::PointerEventHandler,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PointerWheelChanged)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(handler),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemovePointerWheelChanged(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).RemovePointerWheelChanged)(
                ::windows::core::Vtable::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn Tapped(
        &self,
        handler: &super::Input::TappedEventHandler,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Tapped)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(handler),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveTapped(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).RemoveTapped)(
                ::windows::core::Vtable::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn DoubleTapped(
        &self,
        handler: &super::Input::DoubleTappedEventHandler,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DoubleTapped)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(handler),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveDoubleTapped(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).RemoveDoubleTapped)(
                ::windows::core::Vtable::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn Holding(
        &self,
        handler: &super::Input::HoldingEventHandler,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Holding)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(handler),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveHolding(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).RemoveHolding)(
                ::windows::core::Vtable::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn ContextRequested(
        &self,
        handler: &::windows::Foundation::TypedEventHandler<
            super::UIElement,
            super::Input::ContextRequestedEventArgs,
        >,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ContextRequested)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(handler),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveContextRequested(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).RemoveContextRequested)(
                ::windows::core::Vtable::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn ContextCanceled(
        &self,
        handler: &::windows::Foundation::TypedEventHandler<
            super::UIElement,
            super::RoutedEventArgs,
        >,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ContextCanceled)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(handler),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveContextCanceled(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).RemoveContextCanceled)(
                ::windows::core::Vtable::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn RightTapped(
        &self,
        handler: &super::Input::RightTappedEventHandler,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RightTapped)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(handler),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveRightTapped(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).RemoveRightTapped)(
                ::windows::core::Vtable::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn ManipulationStarting(
        &self,
        handler: &super::Input::ManipulationStartingEventHandler,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ManipulationStarting)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(handler),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveManipulationStarting(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).RemoveManipulationStarting)(
                ::windows::core::Vtable::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn ManipulationInertiaStarting(
        &self,
        handler: &super::Input::ManipulationInertiaStartingEventHandler,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ManipulationInertiaStarting)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(handler),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveManipulationInertiaStarting(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).RemoveManipulationInertiaStarting)(
                ::windows::core::Vtable::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn ManipulationStarted(
        &self,
        handler: &super::Input::ManipulationStartedEventHandler,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ManipulationStarted)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(handler),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveManipulationStarted(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).RemoveManipulationStarted)(
                ::windows::core::Vtable::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn ManipulationDelta(
        &self,
        handler: &super::Input::ManipulationDeltaEventHandler,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ManipulationDelta)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(handler),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveManipulationDelta(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).RemoveManipulationDelta)(
                ::windows::core::Vtable::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn ManipulationCompleted(
        &self,
        handler: &super::Input::ManipulationCompletedEventHandler,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ManipulationCompleted)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(handler),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveManipulationCompleted(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).RemoveManipulationCompleted)(
                ::windows::core::Vtable::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn AccessKeyDisplayRequested(
        &self,
        handler: &::windows::Foundation::TypedEventHandler<
            super::UIElement,
            super::Input::AccessKeyDisplayRequestedEventArgs,
        >,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AccessKeyDisplayRequested)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(handler),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveAccessKeyDisplayRequested(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).RemoveAccessKeyDisplayRequested)(
                ::windows::core::Vtable::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn AccessKeyDisplayDismissed(
        &self,
        handler: &::windows::Foundation::TypedEventHandler<
            super::UIElement,
            super::Input::AccessKeyDisplayDismissedEventArgs,
        >,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AccessKeyDisplayDismissed)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(handler),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveAccessKeyDisplayDismissed(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).RemoveAccessKeyDisplayDismissed)(
                ::windows::core::Vtable::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn AccessKeyInvoked(
        &self,
        handler: &::windows::Foundation::TypedEventHandler<
            super::UIElement,
            super::Input::AccessKeyInvokedEventArgs,
        >,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AccessKeyInvoked)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(handler),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveAccessKeyInvoked(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).RemoveAccessKeyInvoked)(
                ::windows::core::Vtable::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn ProcessKeyboardAccelerators(
        &self,
        handler: &::windows::Foundation::TypedEventHandler<
            super::UIElement,
            super::Input::ProcessKeyboardAcceleratorEventArgs,
        >,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ProcessKeyboardAccelerators)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(handler),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveProcessKeyboardAccelerators(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).RemoveProcessKeyboardAccelerators)(
                ::windows::core::Vtable::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn GettingFocus(
        &self,
        handler: &::windows::Foundation::TypedEventHandler<
            super::UIElement,
            super::Input::GettingFocusEventArgs,
        >,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GettingFocus)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(handler),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveGettingFocus(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).RemoveGettingFocus)(
                ::windows::core::Vtable::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn LosingFocus(
        &self,
        handler: &::windows::Foundation::TypedEventHandler<
            super::UIElement,
            super::Input::LosingFocusEventArgs,
        >,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LosingFocus)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(handler),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveLosingFocus(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).RemoveLosingFocus)(
                ::windows::core::Vtable::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn NoFocusCandidateFound(
        &self,
        handler: &::windows::Foundation::TypedEventHandler<
            super::UIElement,
            super::Input::NoFocusCandidateFoundEventArgs,
        >,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NoFocusCandidateFound)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(handler),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveNoFocusCandidateFound(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).RemoveNoFocusCandidateFound)(
                ::windows::core::Vtable::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn PreviewKeyDown(
        &self,
        handler: &super::Input::KeyEventHandler,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PreviewKeyDown)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(handler),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemovePreviewKeyDown(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).RemovePreviewKeyDown)(
                ::windows::core::Vtable::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn PreviewKeyUp(
        &self,
        handler: &super::Input::KeyEventHandler,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PreviewKeyUp)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(handler),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemovePreviewKeyUp(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).RemovePreviewKeyUp)(
                ::windows::core::Vtable::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn BringIntoViewRequested(
        &self,
        handler: &::windows::Foundation::TypedEventHandler<
            super::UIElement,
            super::BringIntoViewRequestedEventArgs,
        >,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).BringIntoViewRequested)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(handler),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveBringIntoViewRequested(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).RemoveBringIntoViewRequested)(
                ::windows::core::Vtable::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn Measure(
        &self,
        availablesize: ::windows::Foundation::Size,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).Measure)(
                ::windows::core::Vtable::as_raw(this),
                availablesize,
            )
            .ok()
        }
    }
    pub fn Arrange(&self, finalrect: ::windows::Foundation::Rect) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).Arrange)(
                ::windows::core::Vtable::as_raw(this),
                finalrect,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn CapturePointer(&self, value: &super::Input::Pointer) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CapturePointer)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(value),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn ReleasePointerCapture(
        &self,
        value: &super::Input::Pointer,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).ReleasePointerCapture)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn ReleasePointerCaptures(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).ReleasePointerCaptures)(
                ::windows::core::Vtable::as_raw(this),
            )
            .ok()
        }
    }
    pub fn AddHandler<'a, P0>(
        &self,
        routedevent: &super::RoutedEvent,
        handler: P0,
        handledeventstoo: bool,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IInspectable>>,
    {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).AddHandler)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(routedevent),
                handler.into().abi(),
                handledeventstoo,
            )
            .ok()
        }
    }
    pub fn RemoveHandler<'a, P0>(
        &self,
        routedevent: &super::RoutedEvent,
        handler: P0,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IInspectable>>,
    {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).RemoveHandler)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(routedevent),
                handler.into().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn TransformToVisual<'a, P0>(
        &self,
        visual: P0,
    ) -> ::windows::core::Result<super::Media::GeneralTransform>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::UIElement>>,
    {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TransformToVisual)(
                ::windows::core::Vtable::as_raw(this),
                visual.into().abi(),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::Media::GeneralTransform>(result__)
        }
    }
    pub fn InvalidateMeasure(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).InvalidateMeasure)(
                ::windows::core::Vtable::as_raw(this),
            )
            .ok()
        }
    }
    pub fn InvalidateArrange(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).InvalidateArrange)(
                ::windows::core::Vtable::as_raw(this),
            )
            .ok()
        }
    }
    pub fn UpdateLayout(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).UpdateLayout)(::windows::core::Vtable::as_raw(
                this,
            ))
            .ok()
        }
    }
    pub fn CancelDirectManipulations(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CancelDirectManipulations)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    #[cfg(feature = "UI_Input")]
    pub fn StartDragAsync(
        &self,
        pointerpoint: &super::super::Input::PointerPoint,
    ) -> ::windows::core::Result<
        ::windows::Foundation::IAsyncOperation<
            ::windows::ApplicationModel::DataTransfer::DataPackageOperation,
        >,
    > {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).StartDragAsync)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(pointerpoint),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::IAsyncOperation<
                ::windows::ApplicationModel::DataTransfer::DataPackageOperation,
            >>(result__)
        }
    }
    pub fn StartBringIntoView(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).StartBringIntoView)(
                ::windows::core::Vtable::as_raw(this),
            )
            .ok()
        }
    }
    pub fn StartBringIntoViewWithOptions(
        &self,
        options: &super::BringIntoViewOptions,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).StartBringIntoViewWithOptions)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(options),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn TryInvokeKeyboardAccelerator(
        &self,
        args: &super::Input::ProcessKeyboardAcceleratorEventArgs,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).TryInvokeKeyboardAccelerator)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(args),
            )
            .ok()
        }
    }
    pub fn Focus(&self, value: super::FocusState) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Focus)(
                ::windows::core::Vtable::as_raw(this),
                value,
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition\"`*"]
    #[cfg(feature = "UI_Composition")]
    pub fn StartAnimation<'a, P0, E0>(&self, animation: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<'a, super::super::Composition::ICompositionAnimationBase>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).StartAnimation)(
                ::windows::core::Vtable::as_raw(this),
                animation.try_into().map_err(|e| e.into())?.abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Composition\"`*"]
    #[cfg(feature = "UI_Composition")]
    pub fn StopAnimation<'a, P0, E0>(&self, animation: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<'a, super::super::Composition::ICompositionAnimationBase>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).StopAnimation)(
                ::windows::core::Vtable::as_raw(this),
                animation.try_into().map_err(|e| e.into())?.abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    #[cfg(feature = "UI_Input")]
    pub fn ProtectedCursor(&self) -> ::windows::core::Result<super::super::Input::InputCursor> {
        let this = &::windows::core::Interface::cast::<super::IUIElementProtected>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ProtectedCursor)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::super::Input::InputCursor>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    #[cfg(feature = "UI_Input")]
    pub fn SetProtectedCursor<'a, P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Input::InputCursor>>,
    {
        let this = &::windows::core::Interface::cast::<super::IUIElementProtected>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetProtectedCursor)(
                ::windows::core::Vtable::as_raw(this),
                value.into().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Composition\"`*"]
    #[cfg(feature = "UI_Composition")]
    pub fn GetVisualInternal(&self) -> ::windows::core::Result<super::super::Composition::Visual> {
        let this =
            &::windows::core::Interface::cast::<super::super::Composition::IVisualElement2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetVisualInternal)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::super::Composition::Visual>(result__)
        }
    }
    #[doc(hidden)]
    pub fn IGlyphsStatics<R, F: FnOnce(&IGlyphsStatics) -> ::windows::core::Result<R>>(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<Glyphs, IGlyphsStatics> =
            ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for Glyphs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for Glyphs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Glyphs {}
impl ::core::fmt::Debug for Glyphs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Glyphs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for Glyphs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Xaml.Documents.Glyphs;{0fbf8cfe-18e7-5e45-9fa3-d2d0927958f4})",
    );
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for Glyphs {
    type Vtable = IGlyphs_Vtbl;
}
unsafe impl ::windows::core::Interface for Glyphs {
    const IID: ::windows::core::GUID = <IGlyphs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for Glyphs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Documents.Glyphs";
}
::windows::core::interface_hierarchy!(
    Glyphs,
    ::windows::core::IUnknown,
    ::windows::core::IInspectable
);
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<Glyphs> for super::super::Composition::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: Glyphs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<&Glyphs> for super::super::Composition::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: &Glyphs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::core::convert::TryFrom<&Glyphs>
    for ::windows::core::InParam<'a, super::super::Composition::IAnimationObject>
{
    type Error = ::windows::core::Error;
    fn try_from(value: &Glyphs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<Glyphs> for super::super::Composition::IVisualElement {
    type Error = ::windows::core::Error;
    fn try_from(value: Glyphs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<&Glyphs> for super::super::Composition::IVisualElement {
    type Error = ::windows::core::Error;
    fn try_from(value: &Glyphs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::core::convert::TryFrom<&Glyphs>
    for ::windows::core::InParam<'a, super::super::Composition::IVisualElement>
{
    type Error = ::windows::core::Error;
    fn try_from(value: &Glyphs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<Glyphs> for super::super::Composition::IVisualElement2 {
    type Error = ::windows::core::Error;
    fn try_from(value: Glyphs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<&Glyphs> for super::super::Composition::IVisualElement2 {
    type Error = ::windows::core::Error;
    fn try_from(value: &Glyphs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::core::convert::TryFrom<&Glyphs>
    for ::windows::core::InParam<'a, super::super::Composition::IVisualElement2>
{
    type Error = ::windows::core::Error;
    fn try_from(value: &Glyphs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::From<Glyphs> for super::FrameworkElement {
    fn from(value: Glyphs) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&Glyphs> for super::FrameworkElement {
    fn from(value: &Glyphs) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::core::convert::From<&Glyphs> for ::windows::core::InParam<'a, super::FrameworkElement> {
    fn from(value: &Glyphs) -> Self {
        ::windows::core::InParam::owned(value.into())
    }
}
impl ::core::convert::From<Glyphs> for super::UIElement {
    fn from(value: Glyphs) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&Glyphs> for super::UIElement {
    fn from(value: &Glyphs) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::core::convert::From<&Glyphs> for ::windows::core::InParam<'a, super::UIElement> {
    fn from(value: &Glyphs) -> Self {
        ::windows::core::InParam::owned(value.into())
    }
}
impl ::core::convert::From<Glyphs> for super::DependencyObject {
    fn from(value: Glyphs) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&Glyphs> for super::DependencyObject {
    fn from(value: &Glyphs) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::core::convert::From<&Glyphs> for ::windows::core::InParam<'a, super::DependencyObject> {
    fn from(value: &Glyphs) -> Self {
        ::windows::core::InParam::owned(value.into())
    }
}
unsafe impl ::core::marker::Send for Glyphs {}
unsafe impl ::core::marker::Sync for Glyphs {}
#[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
#[repr(transparent)]
pub struct Hyperlink(::windows::core::IUnknown);
impl Hyperlink {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<Hyperlink, ::windows::core::IGenericFactory> =
            ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn GetValue(
        &self,
        dp: &super::DependencyProperty,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetValue)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(dp),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn SetValue<'a, P0>(
        &self,
        dp: &super::DependencyProperty,
        value: P0,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IInspectable>>,
    {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetValue)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(dp),
                value.into().abi(),
            )
            .ok()
        }
    }
    pub fn ClearValue(&self, dp: &super::DependencyProperty) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).ClearValue)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(dp),
            )
            .ok()
        }
    }
    pub fn ReadLocalValue(
        &self,
        dp: &super::DependencyProperty,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ReadLocalValue)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(dp),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn GetAnimationBaseValue(
        &self,
        dp: &super::DependencyProperty,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetAnimationBaseValue)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(dp),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn RegisterPropertyChangedCallback(
        &self,
        dp: &super::DependencyProperty,
        callback: &super::DependencyPropertyChangedCallback,
    ) -> ::windows::core::Result<i64> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RegisterPropertyChangedCallback)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(dp),
                ::core::mem::transmute_copy(callback),
                result__.as_mut_ptr(),
            )
            .from_abi::<i64>(result__)
        }
    }
    pub fn UnregisterPropertyChangedCallback(
        &self,
        dp: &super::DependencyProperty,
        token: i64,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).UnregisterPropertyChangedCallback)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(dp),
                token,
            )
            .ok()
        }
    }
    pub fn Dispatcher(&self) -> ::windows::core::Result<::windows::UI::Core::CoreDispatcher> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Dispatcher)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Core::CoreDispatcher>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Dispatching\"`*"]
    #[cfg(feature = "UI_Dispatching")]
    pub fn DispatcherQueue(
        &self,
    ) -> ::windows::core::Result<super::super::Dispatching::DispatcherQueue> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DispatcherQueue)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::super::Dispatching::DispatcherQueue>(result__)
        }
    }
    pub fn NavigateUri(&self) -> ::windows::core::Result<::windows::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NavigateUri)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::Uri>(result__)
        }
    }
    pub fn SetNavigateUri(
        &self,
        value: &::windows::Foundation::Uri,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetNavigateUri)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn UnderlineStyle(&self) -> ::windows::core::Result<UnderlineStyle> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).UnderlineStyle)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<UnderlineStyle>(result__)
        }
    }
    pub fn SetUnderlineStyle(&self, value: UnderlineStyle) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetUnderlineStyle)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn XYFocusLeft(&self) -> ::windows::core::Result<super::DependencyObject> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).XYFocusLeft)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::DependencyObject>(result__)
        }
    }
    pub fn SetXYFocusLeft<'a, P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::DependencyObject>>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetXYFocusLeft)(
                ::windows::core::Vtable::as_raw(this),
                value.into().abi(),
            )
            .ok()
        }
    }
    pub fn XYFocusRight(&self) -> ::windows::core::Result<super::DependencyObject> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).XYFocusRight)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::DependencyObject>(result__)
        }
    }
    pub fn SetXYFocusRight<'a, P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::DependencyObject>>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetXYFocusRight)(
                ::windows::core::Vtable::as_raw(this),
                value.into().abi(),
            )
            .ok()
        }
    }
    pub fn XYFocusUp(&self) -> ::windows::core::Result<super::DependencyObject> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).XYFocusUp)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::DependencyObject>(result__)
        }
    }
    pub fn SetXYFocusUp<'a, P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::DependencyObject>>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetXYFocusUp)(
                ::windows::core::Vtable::as_raw(this),
                value.into().abi(),
            )
            .ok()
        }
    }
    pub fn XYFocusDown(&self) -> ::windows::core::Result<super::DependencyObject> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).XYFocusDown)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::DependencyObject>(result__)
        }
    }
    pub fn SetXYFocusDown<'a, P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::DependencyObject>>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetXYFocusDown)(
                ::windows::core::Vtable::as_raw(this),
                value.into().abi(),
            )
            .ok()
        }
    }
    pub fn ElementSoundMode(&self) -> ::windows::core::Result<super::ElementSoundMode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ElementSoundMode)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::ElementSoundMode>(result__)
        }
    }
    pub fn SetElementSoundMode(
        &self,
        value: super::ElementSoundMode,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetElementSoundMode)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn FocusState(&self) -> ::windows::core::Result<super::FocusState> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FocusState)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::FocusState>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn XYFocusUpNavigationStrategy(
        &self,
    ) -> ::windows::core::Result<super::Input::XYFocusNavigationStrategy> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).XYFocusUpNavigationStrategy)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::Input::XYFocusNavigationStrategy>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetXYFocusUpNavigationStrategy(
        &self,
        value: super::Input::XYFocusNavigationStrategy,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetXYFocusUpNavigationStrategy)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn XYFocusDownNavigationStrategy(
        &self,
    ) -> ::windows::core::Result<super::Input::XYFocusNavigationStrategy> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).XYFocusDownNavigationStrategy)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::Input::XYFocusNavigationStrategy>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetXYFocusDownNavigationStrategy(
        &self,
        value: super::Input::XYFocusNavigationStrategy,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetXYFocusDownNavigationStrategy)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn XYFocusLeftNavigationStrategy(
        &self,
    ) -> ::windows::core::Result<super::Input::XYFocusNavigationStrategy> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).XYFocusLeftNavigationStrategy)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::Input::XYFocusNavigationStrategy>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetXYFocusLeftNavigationStrategy(
        &self,
        value: super::Input::XYFocusNavigationStrategy,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetXYFocusLeftNavigationStrategy)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn XYFocusRightNavigationStrategy(
        &self,
    ) -> ::windows::core::Result<super::Input::XYFocusNavigationStrategy> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).XYFocusRightNavigationStrategy)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::Input::XYFocusNavigationStrategy>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetXYFocusRightNavigationStrategy(
        &self,
        value: super::Input::XYFocusNavigationStrategy,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetXYFocusRightNavigationStrategy)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsTabStop(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsTabStop)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetIsTabStop(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetIsTabStop)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn TabIndex(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TabIndex)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<i32>(result__)
        }
    }
    pub fn SetTabIndex(&self, value: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetTabIndex)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn Click(
        &self,
        handler: &::windows::Foundation::TypedEventHandler<Hyperlink, HyperlinkClickEventArgs>,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Click)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(handler),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveClick(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).RemoveClick)(
                ::windows::core::Vtable::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn GotFocus(
        &self,
        handler: &super::RoutedEventHandler,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GotFocus)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(handler),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveGotFocus(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).RemoveGotFocus)(
                ::windows::core::Vtable::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn LostFocus(
        &self,
        handler: &super::RoutedEventHandler,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LostFocus)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(handler),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveLostFocus(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).RemoveLostFocus)(
                ::windows::core::Vtable::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn Focus(&self, value: super::FocusState) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Focus)(
                ::windows::core::Vtable::as_raw(this),
                value,
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn NavigateUriProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IHyperlinkStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NavigateUriProperty)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn UnderlineStyleProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IHyperlinkStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).UnderlineStyleProperty)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn XYFocusLeftProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IHyperlinkStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).XYFocusLeftProperty)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn XYFocusRightProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IHyperlinkStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).XYFocusRightProperty)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn XYFocusUpProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IHyperlinkStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).XYFocusUpProperty)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn XYFocusDownProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IHyperlinkStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).XYFocusDownProperty)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn ElementSoundModeProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IHyperlinkStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ElementSoundModeProperty)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn FocusStateProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IHyperlinkStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FocusStateProperty)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn XYFocusUpNavigationStrategyProperty(
    ) -> ::windows::core::Result<super::DependencyProperty> {
        Self::IHyperlinkStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).XYFocusUpNavigationStrategyProperty)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn XYFocusDownNavigationStrategyProperty(
    ) -> ::windows::core::Result<super::DependencyProperty> {
        Self::IHyperlinkStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).XYFocusDownNavigationStrategyProperty)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn XYFocusLeftNavigationStrategyProperty(
    ) -> ::windows::core::Result<super::DependencyProperty> {
        Self::IHyperlinkStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).XYFocusLeftNavigationStrategyProperty)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn XYFocusRightNavigationStrategyProperty(
    ) -> ::windows::core::Result<super::DependencyProperty> {
        Self::IHyperlinkStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).XYFocusRightNavigationStrategyProperty)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn IsTabStopProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IHyperlinkStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsTabStopProperty)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn TabIndexProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IHyperlinkStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TabIndexProperty)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn Inlines(&self) -> ::windows::core::Result<InlineCollection> {
        let this = &::windows::core::Interface::cast::<ISpan>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Inlines)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<InlineCollection>(result__)
        }
    }
    pub fn SetInlines(&self, value: &InlineCollection) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ISpan>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetInlines)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Name)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn FontSize(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FontSize)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetFontSize(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetFontSize)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn FontFamily(&self) -> ::windows::core::Result<super::Media::FontFamily> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FontFamily)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::Media::FontFamily>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetFontFamily<'a, P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::Media::FontFamily>>,
    {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetFontFamily)(
                ::windows::core::Vtable::as_raw(this),
                value.into().abi(),
            )
            .ok()
        }
    }
    pub fn FontWeight(&self) -> ::windows::core::Result<::windows::UI::Text::FontWeight> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FontWeight)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Text::FontWeight>(result__)
        }
    }
    pub fn SetFontWeight(
        &self,
        value: ::windows::UI::Text::FontWeight,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetFontWeight)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn FontStyle(&self) -> ::windows::core::Result<::windows::UI::Text::FontStyle> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FontStyle)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Text::FontStyle>(result__)
        }
    }
    pub fn SetFontStyle(
        &self,
        value: ::windows::UI::Text::FontStyle,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetFontStyle)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn FontStretch(&self) -> ::windows::core::Result<::windows::UI::Text::FontStretch> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FontStretch)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Text::FontStretch>(result__)
        }
    }
    pub fn SetFontStretch(
        &self,
        value: ::windows::UI::Text::FontStretch,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetFontStretch)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn CharacterSpacing(&self) -> ::windows::core::Result<i32> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CharacterSpacing)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<i32>(result__)
        }
    }
    pub fn SetCharacterSpacing(&self, value: i32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetCharacterSpacing)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn Foreground(&self) -> ::windows::core::Result<super::Media::Brush> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Foreground)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::Media::Brush>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetForeground<'a, P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::Media::Brush>>,
    {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetForeground)(
                ::windows::core::Vtable::as_raw(this),
                value.into().abi(),
            )
            .ok()
        }
    }
    pub fn Language(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
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
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetLanguage)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn IsTextScaleFactorEnabled(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsTextScaleFactorEnabled)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetIsTextScaleFactorEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetIsTextScaleFactorEnabled)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn TextDecorations(&self) -> ::windows::core::Result<::windows::UI::Text::TextDecorations> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TextDecorations)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Text::TextDecorations>(result__)
        }
    }
    pub fn SetTextDecorations(
        &self,
        value: ::windows::UI::Text::TextDecorations,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetTextDecorations)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn ContentStart(&self) -> ::windows::core::Result<TextPointer> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ContentStart)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    pub fn ContentEnd(&self) -> ::windows::core::Result<TextPointer> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ContentEnd)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    pub fn ElementStart(&self) -> ::windows::core::Result<TextPointer> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ElementStart)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    pub fn ElementEnd(&self) -> ::windows::core::Result<TextPointer> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ElementEnd)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    pub fn AllowFocusOnInteraction(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AllowFocusOnInteraction)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetAllowFocusOnInteraction(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetAllowFocusOnInteraction)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn AccessKey(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AccessKey)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetAccessKey(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetAccessKey)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn ExitDisplayModeOnAccessKeyInvoked(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ExitDisplayModeOnAccessKeyInvoked)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetExitDisplayModeOnAccessKeyInvoked(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetExitDisplayModeOnAccessKeyInvoked)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsAccessKeyScope(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsAccessKeyScope)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetIsAccessKeyScope(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetIsAccessKeyScope)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn AccessKeyScopeOwner(&self) -> ::windows::core::Result<super::DependencyObject> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AccessKeyScopeOwner)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::DependencyObject>(result__)
        }
    }
    pub fn SetAccessKeyScopeOwner<'a, P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::DependencyObject>>,
    {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetAccessKeyScopeOwner)(
                ::windows::core::Vtable::as_raw(this),
                value.into().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn KeyTipPlacementMode(
        &self,
    ) -> ::windows::core::Result<super::Input::KeyTipPlacementMode> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).KeyTipPlacementMode)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::Input::KeyTipPlacementMode>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetKeyTipPlacementMode(
        &self,
        value: super::Input::KeyTipPlacementMode,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetKeyTipPlacementMode)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn KeyTipHorizontalOffset(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).KeyTipHorizontalOffset)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetKeyTipHorizontalOffset(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetKeyTipHorizontalOffset)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn KeyTipVerticalOffset(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).KeyTipVerticalOffset)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetKeyTipVerticalOffset(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetKeyTipVerticalOffset)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn XamlRoot(&self) -> ::windows::core::Result<super::XamlRoot> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).XamlRoot)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::XamlRoot>(result__)
        }
    }
    pub fn SetXamlRoot(&self, value: &super::XamlRoot) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetXamlRoot)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn AccessKeyDisplayRequested(
        &self,
        handler: &::windows::Foundation::TypedEventHandler<
            TextElement,
            super::Input::AccessKeyDisplayRequestedEventArgs,
        >,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AccessKeyDisplayRequested)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(handler),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveAccessKeyDisplayRequested(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).RemoveAccessKeyDisplayRequested)(
                ::windows::core::Vtable::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn AccessKeyDisplayDismissed(
        &self,
        handler: &::windows::Foundation::TypedEventHandler<
            TextElement,
            super::Input::AccessKeyDisplayDismissedEventArgs,
        >,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AccessKeyDisplayDismissed)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(handler),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveAccessKeyDisplayDismissed(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).RemoveAccessKeyDisplayDismissed)(
                ::windows::core::Vtable::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn AccessKeyInvoked(
        &self,
        handler: &::windows::Foundation::TypedEventHandler<
            TextElement,
            super::Input::AccessKeyInvokedEventArgs,
        >,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AccessKeyInvoked)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(handler),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveAccessKeyInvoked(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).RemoveAccessKeyInvoked)(
                ::windows::core::Vtable::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn FindName(
        &self,
        name: &::windows::core::HSTRING,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FindName)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(name),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc(hidden)]
    pub fn IHyperlinkStatics<R, F: FnOnce(&IHyperlinkStatics) -> ::windows::core::Result<R>>(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<Hyperlink, IHyperlinkStatics> =
            ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for Hyperlink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for Hyperlink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Hyperlink {}
impl ::core::fmt::Debug for Hyperlink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Hyperlink").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for Hyperlink {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Xaml.Documents.Hyperlink;{ac09bd16-cdfa-54c2-8d03-a474181545b1})",
    );
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for Hyperlink {
    type Vtable = IHyperlink_Vtbl;
}
unsafe impl ::windows::core::Interface for Hyperlink {
    const IID: ::windows::core::GUID = <IHyperlink as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for Hyperlink {
    const NAME: &'static str = "Microsoft.UI.Xaml.Documents.Hyperlink";
}
::windows::core::interface_hierarchy!(
    Hyperlink,
    ::windows::core::IUnknown,
    ::windows::core::IInspectable
);
impl ::core::convert::From<Hyperlink> for Span {
    fn from(value: Hyperlink) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&Hyperlink> for Span {
    fn from(value: &Hyperlink) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::core::convert::From<&Hyperlink> for ::windows::core::InParam<'a, Span> {
    fn from(value: &Hyperlink) -> Self {
        ::windows::core::InParam::owned(value.into())
    }
}
impl ::core::convert::From<Hyperlink> for Inline {
    fn from(value: Hyperlink) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&Hyperlink> for Inline {
    fn from(value: &Hyperlink) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::core::convert::From<&Hyperlink> for ::windows::core::InParam<'a, Inline> {
    fn from(value: &Hyperlink) -> Self {
        ::windows::core::InParam::owned(value.into())
    }
}
impl ::core::convert::From<Hyperlink> for TextElement {
    fn from(value: Hyperlink) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&Hyperlink> for TextElement {
    fn from(value: &Hyperlink) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::core::convert::From<&Hyperlink> for ::windows::core::InParam<'a, TextElement> {
    fn from(value: &Hyperlink) -> Self {
        ::windows::core::InParam::owned(value.into())
    }
}
impl ::core::convert::From<Hyperlink> for super::DependencyObject {
    fn from(value: Hyperlink) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&Hyperlink> for super::DependencyObject {
    fn from(value: &Hyperlink) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::core::convert::From<&Hyperlink>
    for ::windows::core::InParam<'a, super::DependencyObject>
{
    fn from(value: &Hyperlink) -> Self {
        ::windows::core::InParam::owned(value.into())
    }
}
unsafe impl ::core::marker::Send for Hyperlink {}
unsafe impl ::core::marker::Sync for Hyperlink {}
#[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
#[repr(transparent)]
pub struct HyperlinkClickEventArgs(::windows::core::IUnknown);
impl HyperlinkClickEventArgs {
    pub fn OriginalSource(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IRoutedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).OriginalSource)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
}
impl ::core::clone::Clone for HyperlinkClickEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HyperlinkClickEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HyperlinkClickEventArgs {}
impl ::core::fmt::Debug for HyperlinkClickEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HyperlinkClickEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for HyperlinkClickEventArgs {
    const SIGNATURE : ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice ( b"rc(Microsoft.UI.Xaml.Documents.HyperlinkClickEventArgs;{f8f89552-873d-5ef5-82bf-c79a9509b07c})" ) ;
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for HyperlinkClickEventArgs {
    type Vtable = IHyperlinkClickEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for HyperlinkClickEventArgs {
    const IID: ::windows::core::GUID =
        <IHyperlinkClickEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for HyperlinkClickEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Documents.HyperlinkClickEventArgs";
}
::windows::core::interface_hierarchy!(
    HyperlinkClickEventArgs,
    ::windows::core::IUnknown,
    ::windows::core::IInspectable
);
impl ::core::convert::From<HyperlinkClickEventArgs> for super::RoutedEventArgs {
    fn from(value: HyperlinkClickEventArgs) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&HyperlinkClickEventArgs> for super::RoutedEventArgs {
    fn from(value: &HyperlinkClickEventArgs) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::core::convert::From<&HyperlinkClickEventArgs>
    for ::windows::core::InParam<'a, super::RoutedEventArgs>
{
    fn from(value: &HyperlinkClickEventArgs) -> Self {
        ::windows::core::InParam::owned(value.into())
    }
}
unsafe impl ::core::marker::Send for HyperlinkClickEventArgs {}
unsafe impl ::core::marker::Sync for HyperlinkClickEventArgs {}
#[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
#[repr(transparent)]
pub struct Inline(::windows::core::IUnknown);
impl Inline {
    pub fn GetValue(
        &self,
        dp: &super::DependencyProperty,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetValue)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(dp),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn SetValue<'a, P0>(
        &self,
        dp: &super::DependencyProperty,
        value: P0,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IInspectable>>,
    {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetValue)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(dp),
                value.into().abi(),
            )
            .ok()
        }
    }
    pub fn ClearValue(&self, dp: &super::DependencyProperty) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).ClearValue)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(dp),
            )
            .ok()
        }
    }
    pub fn ReadLocalValue(
        &self,
        dp: &super::DependencyProperty,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ReadLocalValue)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(dp),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn GetAnimationBaseValue(
        &self,
        dp: &super::DependencyProperty,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetAnimationBaseValue)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(dp),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn RegisterPropertyChangedCallback(
        &self,
        dp: &super::DependencyProperty,
        callback: &super::DependencyPropertyChangedCallback,
    ) -> ::windows::core::Result<i64> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RegisterPropertyChangedCallback)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(dp),
                ::core::mem::transmute_copy(callback),
                result__.as_mut_ptr(),
            )
            .from_abi::<i64>(result__)
        }
    }
    pub fn UnregisterPropertyChangedCallback(
        &self,
        dp: &super::DependencyProperty,
        token: i64,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).UnregisterPropertyChangedCallback)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(dp),
                token,
            )
            .ok()
        }
    }
    pub fn Dispatcher(&self) -> ::windows::core::Result<::windows::UI::Core::CoreDispatcher> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Dispatcher)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Core::CoreDispatcher>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Dispatching\"`*"]
    #[cfg(feature = "UI_Dispatching")]
    pub fn DispatcherQueue(
        &self,
    ) -> ::windows::core::Result<super::super::Dispatching::DispatcherQueue> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DispatcherQueue)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::super::Dispatching::DispatcherQueue>(result__)
        }
    }
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Name)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn FontSize(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FontSize)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetFontSize(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetFontSize)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn FontFamily(&self) -> ::windows::core::Result<super::Media::FontFamily> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FontFamily)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::Media::FontFamily>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetFontFamily<'a, P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::Media::FontFamily>>,
    {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetFontFamily)(
                ::windows::core::Vtable::as_raw(this),
                value.into().abi(),
            )
            .ok()
        }
    }
    pub fn FontWeight(&self) -> ::windows::core::Result<::windows::UI::Text::FontWeight> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FontWeight)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Text::FontWeight>(result__)
        }
    }
    pub fn SetFontWeight(
        &self,
        value: ::windows::UI::Text::FontWeight,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetFontWeight)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn FontStyle(&self) -> ::windows::core::Result<::windows::UI::Text::FontStyle> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FontStyle)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Text::FontStyle>(result__)
        }
    }
    pub fn SetFontStyle(
        &self,
        value: ::windows::UI::Text::FontStyle,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetFontStyle)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn FontStretch(&self) -> ::windows::core::Result<::windows::UI::Text::FontStretch> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FontStretch)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Text::FontStretch>(result__)
        }
    }
    pub fn SetFontStretch(
        &self,
        value: ::windows::UI::Text::FontStretch,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetFontStretch)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn CharacterSpacing(&self) -> ::windows::core::Result<i32> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CharacterSpacing)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<i32>(result__)
        }
    }
    pub fn SetCharacterSpacing(&self, value: i32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetCharacterSpacing)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn Foreground(&self) -> ::windows::core::Result<super::Media::Brush> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Foreground)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::Media::Brush>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetForeground<'a, P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::Media::Brush>>,
    {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetForeground)(
                ::windows::core::Vtable::as_raw(this),
                value.into().abi(),
            )
            .ok()
        }
    }
    pub fn Language(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
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
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetLanguage)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn IsTextScaleFactorEnabled(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsTextScaleFactorEnabled)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetIsTextScaleFactorEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetIsTextScaleFactorEnabled)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn TextDecorations(&self) -> ::windows::core::Result<::windows::UI::Text::TextDecorations> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TextDecorations)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Text::TextDecorations>(result__)
        }
    }
    pub fn SetTextDecorations(
        &self,
        value: ::windows::UI::Text::TextDecorations,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetTextDecorations)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn ContentStart(&self) -> ::windows::core::Result<TextPointer> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ContentStart)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    pub fn ContentEnd(&self) -> ::windows::core::Result<TextPointer> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ContentEnd)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    pub fn ElementStart(&self) -> ::windows::core::Result<TextPointer> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ElementStart)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    pub fn ElementEnd(&self) -> ::windows::core::Result<TextPointer> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ElementEnd)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    pub fn AllowFocusOnInteraction(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AllowFocusOnInteraction)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetAllowFocusOnInteraction(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetAllowFocusOnInteraction)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn AccessKey(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AccessKey)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetAccessKey(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetAccessKey)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn ExitDisplayModeOnAccessKeyInvoked(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ExitDisplayModeOnAccessKeyInvoked)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetExitDisplayModeOnAccessKeyInvoked(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetExitDisplayModeOnAccessKeyInvoked)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsAccessKeyScope(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsAccessKeyScope)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetIsAccessKeyScope(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetIsAccessKeyScope)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn AccessKeyScopeOwner(&self) -> ::windows::core::Result<super::DependencyObject> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AccessKeyScopeOwner)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::DependencyObject>(result__)
        }
    }
    pub fn SetAccessKeyScopeOwner<'a, P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::DependencyObject>>,
    {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetAccessKeyScopeOwner)(
                ::windows::core::Vtable::as_raw(this),
                value.into().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn KeyTipPlacementMode(
        &self,
    ) -> ::windows::core::Result<super::Input::KeyTipPlacementMode> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).KeyTipPlacementMode)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::Input::KeyTipPlacementMode>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetKeyTipPlacementMode(
        &self,
        value: super::Input::KeyTipPlacementMode,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetKeyTipPlacementMode)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn KeyTipHorizontalOffset(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).KeyTipHorizontalOffset)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetKeyTipHorizontalOffset(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetKeyTipHorizontalOffset)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn KeyTipVerticalOffset(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).KeyTipVerticalOffset)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetKeyTipVerticalOffset(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetKeyTipVerticalOffset)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn XamlRoot(&self) -> ::windows::core::Result<super::XamlRoot> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).XamlRoot)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::XamlRoot>(result__)
        }
    }
    pub fn SetXamlRoot(&self, value: &super::XamlRoot) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetXamlRoot)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn AccessKeyDisplayRequested(
        &self,
        handler: &::windows::Foundation::TypedEventHandler<
            TextElement,
            super::Input::AccessKeyDisplayRequestedEventArgs,
        >,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AccessKeyDisplayRequested)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(handler),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveAccessKeyDisplayRequested(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).RemoveAccessKeyDisplayRequested)(
                ::windows::core::Vtable::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn AccessKeyDisplayDismissed(
        &self,
        handler: &::windows::Foundation::TypedEventHandler<
            TextElement,
            super::Input::AccessKeyDisplayDismissedEventArgs,
        >,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AccessKeyDisplayDismissed)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(handler),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveAccessKeyDisplayDismissed(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).RemoveAccessKeyDisplayDismissed)(
                ::windows::core::Vtable::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn AccessKeyInvoked(
        &self,
        handler: &::windows::Foundation::TypedEventHandler<
            TextElement,
            super::Input::AccessKeyInvokedEventArgs,
        >,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AccessKeyInvoked)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(handler),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveAccessKeyInvoked(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).RemoveAccessKeyInvoked)(
                ::windows::core::Vtable::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn FindName(
        &self,
        name: &::windows::core::HSTRING,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FindName)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(name),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
}
impl ::core::clone::Clone for Inline {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for Inline {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Inline {}
impl ::core::fmt::Debug for Inline {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Inline").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for Inline {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Xaml.Documents.Inline;{813d427a-8980-5a79-a8fa-f27919cfb24f})",
    );
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for Inline {
    type Vtable = IInline_Vtbl;
}
unsafe impl ::windows::core::Interface for Inline {
    const IID: ::windows::core::GUID = <IInline as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for Inline {
    const NAME: &'static str = "Microsoft.UI.Xaml.Documents.Inline";
}
::windows::core::interface_hierarchy!(
    Inline,
    ::windows::core::IUnknown,
    ::windows::core::IInspectable
);
impl ::core::convert::From<Inline> for TextElement {
    fn from(value: Inline) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&Inline> for TextElement {
    fn from(value: &Inline) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::core::convert::From<&Inline> for ::windows::core::InParam<'a, TextElement> {
    fn from(value: &Inline) -> Self {
        ::windows::core::InParam::owned(value.into())
    }
}
impl ::core::convert::From<Inline> for super::DependencyObject {
    fn from(value: Inline) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&Inline> for super::DependencyObject {
    fn from(value: &Inline) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::core::convert::From<&Inline> for ::windows::core::InParam<'a, super::DependencyObject> {
    fn from(value: &Inline) -> Self {
        ::windows::core::InParam::owned(value.into())
    }
}
unsafe impl ::core::marker::Send for Inline {}
unsafe impl ::core::marker::Sync for Inline {}
#[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
#[repr(transparent)]
pub struct InlineCollection(::windows::core::IUnknown);
impl InlineCollection {
    pub fn First(
        &self,
    ) -> ::windows::core::Result<::windows::Foundation::Collections::IIterator<Inline>> {
        let this = &::windows::core::Interface::cast::<
            ::windows::Foundation::Collections::IIterable<Inline>,
        >(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).First)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::Collections::IIterator<Inline>>(result__)
        }
    }
    pub fn GetAt(&self, index: u32) -> ::windows::core::Result<Inline> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetAt)(
                ::windows::core::Vtable::as_raw(this),
                index,
                result__.as_mut_ptr(),
            )
            .from_abi::<Inline>(result__)
        }
    }
    pub fn Size(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Size)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<u32>(result__)
        }
    }
    pub fn GetView(
        &self,
    ) -> ::windows::core::Result<::windows::Foundation::Collections::IVectorView<Inline>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetView)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::Collections::IVectorView<Inline>>(result__)
        }
    }
    pub fn IndexOf<'a, P0>(&self, value: P0, index: &mut u32) -> ::windows::core::Result<bool>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, Inline>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IndexOf)(
                ::windows::core::Vtable::as_raw(this),
                value.into().abi(),
                index,
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetAt<'a, P0>(&self, index: u32, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, Inline>>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetAt)(
                ::windows::core::Vtable::as_raw(this),
                index,
                value.into().abi(),
            )
            .ok()
        }
    }
    pub fn InsertAt<'a, P0>(&self, index: u32, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, Inline>>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).InsertAt)(
                ::windows::core::Vtable::as_raw(this),
                index,
                value.into().abi(),
            )
            .ok()
        }
    }
    pub fn RemoveAt(&self, index: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).RemoveAt)(
                ::windows::core::Vtable::as_raw(this),
                index,
            )
            .ok()
        }
    }
    pub fn Append<'a, P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, Inline>>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).Append)(
                ::windows::core::Vtable::as_raw(this),
                value.into().abi(),
            )
            .ok()
        }
    }
    pub fn RemoveAtEnd(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).RemoveAtEnd)(::windows::core::Vtable::as_raw(
                this,
            ))
            .ok()
        }
    }
    pub fn Clear(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).Clear)(::windows::core::Vtable::as_raw(this))
                .ok()
        }
    }
    pub fn GetMany(
        &self,
        startindex: u32,
        items: &mut [::core::option::Option<Inline>],
    ) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetMany)(
                ::windows::core::Vtable::as_raw(this),
                startindex,
                items.len() as u32,
                ::core::mem::transmute_copy(&items),
                result__.as_mut_ptr(),
            )
            .from_abi::<u32>(result__)
        }
    }
    pub fn ReplaceAll(
        &self,
        items: &[::core::option::Option<Inline>],
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).ReplaceAll)(
                ::windows::core::Vtable::as_raw(this),
                items.len() as u32,
                ::core::mem::transmute(items.as_ptr()),
            )
            .ok()
        }
    }
}
impl ::core::clone::Clone for InlineCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for InlineCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InlineCollection {}
impl ::core::fmt::Debug for InlineCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InlineCollection").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for InlineCollection {
    const SIGNATURE : ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice ( b"rc(Microsoft.UI.Xaml.Documents.InlineCollection;pinterface({913337e9-11a1-4345-a3a2-4e7f956e222d};rc(Microsoft.UI.Xaml.Documents.Inline;{813d427a-8980-5a79-a8fa-f27919cfb24f})))" ) ;
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for InlineCollection {
    type Vtable = ::windows::Foundation::Collections::IVector_Vtbl<Inline>;
}
unsafe impl ::windows::core::Interface for InlineCollection {
    const IID: ::windows::core::GUID =
        <::windows::Foundation::Collections::IVector<Inline> as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for InlineCollection {
    const NAME: &'static str = "Microsoft.UI.Xaml.Documents.InlineCollection";
}
impl ::core::iter::IntoIterator for InlineCollection {
    type Item = Inline;
    type IntoIter = ::windows::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
impl ::core::iter::IntoIterator for &InlineCollection {
    type Item = Inline;
    type IntoIter = ::windows::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::windows::Foundation::Collections::VectorIterator::new(
            ::core::convert::TryInto::try_into(self).ok(),
        )
    }
}
::windows::core::interface_hierarchy!(
    InlineCollection,
    ::windows::core::IUnknown,
    ::windows::core::IInspectable
);
impl ::core::convert::TryFrom<InlineCollection>
    for ::windows::Foundation::Collections::IIterable<Inline>
{
    type Error = ::windows::core::Error;
    fn try_from(value: InlineCollection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&InlineCollection>
    for ::windows::Foundation::Collections::IIterable<Inline>
{
    type Error = ::windows::core::Error;
    fn try_from(value: &InlineCollection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&InlineCollection>
    for ::windows::core::InParam<'a, ::windows::Foundation::Collections::IIterable<Inline>>
{
    type Error = ::windows::core::Error;
    fn try_from(value: &InlineCollection) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<InlineCollection>
    for ::windows::Foundation::Collections::IVector<Inline>
{
    type Error = ::windows::core::Error;
    fn try_from(value: InlineCollection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&InlineCollection>
    for ::windows::Foundation::Collections::IVector<Inline>
{
    type Error = ::windows::core::Error;
    fn try_from(value: &InlineCollection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&InlineCollection>
    for ::windows::core::InParam<'a, ::windows::Foundation::Collections::IVector<Inline>>
{
    type Error = ::windows::core::Error;
    fn try_from(value: &InlineCollection) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for InlineCollection {}
unsafe impl ::core::marker::Sync for InlineCollection {}
#[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
#[repr(transparent)]
pub struct InlineUIContainer(::windows::core::IUnknown);
impl InlineUIContainer {
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
            InlineUIContainer,
            ::windows::core::IGenericFactory,
        > = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn GetValue(
        &self,
        dp: &super::DependencyProperty,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetValue)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(dp),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn SetValue<'a, P0>(
        &self,
        dp: &super::DependencyProperty,
        value: P0,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IInspectable>>,
    {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetValue)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(dp),
                value.into().abi(),
            )
            .ok()
        }
    }
    pub fn ClearValue(&self, dp: &super::DependencyProperty) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).ClearValue)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(dp),
            )
            .ok()
        }
    }
    pub fn ReadLocalValue(
        &self,
        dp: &super::DependencyProperty,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ReadLocalValue)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(dp),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn GetAnimationBaseValue(
        &self,
        dp: &super::DependencyProperty,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetAnimationBaseValue)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(dp),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn RegisterPropertyChangedCallback(
        &self,
        dp: &super::DependencyProperty,
        callback: &super::DependencyPropertyChangedCallback,
    ) -> ::windows::core::Result<i64> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RegisterPropertyChangedCallback)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(dp),
                ::core::mem::transmute_copy(callback),
                result__.as_mut_ptr(),
            )
            .from_abi::<i64>(result__)
        }
    }
    pub fn UnregisterPropertyChangedCallback(
        &self,
        dp: &super::DependencyProperty,
        token: i64,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).UnregisterPropertyChangedCallback)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(dp),
                token,
            )
            .ok()
        }
    }
    pub fn Dispatcher(&self) -> ::windows::core::Result<::windows::UI::Core::CoreDispatcher> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Dispatcher)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Core::CoreDispatcher>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Dispatching\"`*"]
    #[cfg(feature = "UI_Dispatching")]
    pub fn DispatcherQueue(
        &self,
    ) -> ::windows::core::Result<super::super::Dispatching::DispatcherQueue> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DispatcherQueue)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::super::Dispatching::DispatcherQueue>(result__)
        }
    }
    pub fn Child(&self) -> ::windows::core::Result<super::UIElement> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Child)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::UIElement>(result__)
        }
    }
    pub fn SetChild<'a, P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::UIElement>>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetChild)(
                ::windows::core::Vtable::as_raw(this),
                value.into().abi(),
            )
            .ok()
        }
    }
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Name)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn FontSize(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FontSize)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetFontSize(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetFontSize)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn FontFamily(&self) -> ::windows::core::Result<super::Media::FontFamily> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FontFamily)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::Media::FontFamily>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetFontFamily<'a, P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::Media::FontFamily>>,
    {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetFontFamily)(
                ::windows::core::Vtable::as_raw(this),
                value.into().abi(),
            )
            .ok()
        }
    }
    pub fn FontWeight(&self) -> ::windows::core::Result<::windows::UI::Text::FontWeight> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FontWeight)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Text::FontWeight>(result__)
        }
    }
    pub fn SetFontWeight(
        &self,
        value: ::windows::UI::Text::FontWeight,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetFontWeight)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn FontStyle(&self) -> ::windows::core::Result<::windows::UI::Text::FontStyle> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FontStyle)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Text::FontStyle>(result__)
        }
    }
    pub fn SetFontStyle(
        &self,
        value: ::windows::UI::Text::FontStyle,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetFontStyle)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn FontStretch(&self) -> ::windows::core::Result<::windows::UI::Text::FontStretch> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FontStretch)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Text::FontStretch>(result__)
        }
    }
    pub fn SetFontStretch(
        &self,
        value: ::windows::UI::Text::FontStretch,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetFontStretch)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn CharacterSpacing(&self) -> ::windows::core::Result<i32> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CharacterSpacing)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<i32>(result__)
        }
    }
    pub fn SetCharacterSpacing(&self, value: i32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetCharacterSpacing)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn Foreground(&self) -> ::windows::core::Result<super::Media::Brush> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Foreground)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::Media::Brush>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetForeground<'a, P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::Media::Brush>>,
    {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetForeground)(
                ::windows::core::Vtable::as_raw(this),
                value.into().abi(),
            )
            .ok()
        }
    }
    pub fn Language(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
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
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetLanguage)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn IsTextScaleFactorEnabled(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsTextScaleFactorEnabled)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetIsTextScaleFactorEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetIsTextScaleFactorEnabled)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn TextDecorations(&self) -> ::windows::core::Result<::windows::UI::Text::TextDecorations> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TextDecorations)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Text::TextDecorations>(result__)
        }
    }
    pub fn SetTextDecorations(
        &self,
        value: ::windows::UI::Text::TextDecorations,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetTextDecorations)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn ContentStart(&self) -> ::windows::core::Result<TextPointer> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ContentStart)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    pub fn ContentEnd(&self) -> ::windows::core::Result<TextPointer> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ContentEnd)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    pub fn ElementStart(&self) -> ::windows::core::Result<TextPointer> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ElementStart)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    pub fn ElementEnd(&self) -> ::windows::core::Result<TextPointer> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ElementEnd)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    pub fn AllowFocusOnInteraction(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AllowFocusOnInteraction)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetAllowFocusOnInteraction(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetAllowFocusOnInteraction)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn AccessKey(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AccessKey)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetAccessKey(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetAccessKey)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn ExitDisplayModeOnAccessKeyInvoked(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ExitDisplayModeOnAccessKeyInvoked)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetExitDisplayModeOnAccessKeyInvoked(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetExitDisplayModeOnAccessKeyInvoked)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsAccessKeyScope(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsAccessKeyScope)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetIsAccessKeyScope(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetIsAccessKeyScope)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn AccessKeyScopeOwner(&self) -> ::windows::core::Result<super::DependencyObject> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AccessKeyScopeOwner)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::DependencyObject>(result__)
        }
    }
    pub fn SetAccessKeyScopeOwner<'a, P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::DependencyObject>>,
    {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetAccessKeyScopeOwner)(
                ::windows::core::Vtable::as_raw(this),
                value.into().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn KeyTipPlacementMode(
        &self,
    ) -> ::windows::core::Result<super::Input::KeyTipPlacementMode> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).KeyTipPlacementMode)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::Input::KeyTipPlacementMode>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetKeyTipPlacementMode(
        &self,
        value: super::Input::KeyTipPlacementMode,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetKeyTipPlacementMode)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn KeyTipHorizontalOffset(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).KeyTipHorizontalOffset)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetKeyTipHorizontalOffset(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetKeyTipHorizontalOffset)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn KeyTipVerticalOffset(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).KeyTipVerticalOffset)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetKeyTipVerticalOffset(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetKeyTipVerticalOffset)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn XamlRoot(&self) -> ::windows::core::Result<super::XamlRoot> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).XamlRoot)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::XamlRoot>(result__)
        }
    }
    pub fn SetXamlRoot(&self, value: &super::XamlRoot) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetXamlRoot)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn AccessKeyDisplayRequested(
        &self,
        handler: &::windows::Foundation::TypedEventHandler<
            TextElement,
            super::Input::AccessKeyDisplayRequestedEventArgs,
        >,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AccessKeyDisplayRequested)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(handler),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveAccessKeyDisplayRequested(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).RemoveAccessKeyDisplayRequested)(
                ::windows::core::Vtable::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn AccessKeyDisplayDismissed(
        &self,
        handler: &::windows::Foundation::TypedEventHandler<
            TextElement,
            super::Input::AccessKeyDisplayDismissedEventArgs,
        >,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AccessKeyDisplayDismissed)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(handler),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveAccessKeyDisplayDismissed(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).RemoveAccessKeyDisplayDismissed)(
                ::windows::core::Vtable::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn AccessKeyInvoked(
        &self,
        handler: &::windows::Foundation::TypedEventHandler<
            TextElement,
            super::Input::AccessKeyInvokedEventArgs,
        >,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AccessKeyInvoked)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(handler),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveAccessKeyInvoked(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).RemoveAccessKeyInvoked)(
                ::windows::core::Vtable::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn FindName(
        &self,
        name: &::windows::core::HSTRING,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FindName)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(name),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
}
impl ::core::clone::Clone for InlineUIContainer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for InlineUIContainer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InlineUIContainer {}
impl ::core::fmt::Debug for InlineUIContainer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InlineUIContainer").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for InlineUIContainer {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Xaml.Documents.InlineUIContainer;{d529bef6-c05a-5bad-85e8-640127cf86f5})",
    );
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for InlineUIContainer {
    type Vtable = IInlineUIContainer_Vtbl;
}
unsafe impl ::windows::core::Interface for InlineUIContainer {
    const IID: ::windows::core::GUID = <IInlineUIContainer as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for InlineUIContainer {
    const NAME: &'static str = "Microsoft.UI.Xaml.Documents.InlineUIContainer";
}
::windows::core::interface_hierarchy!(
    InlineUIContainer,
    ::windows::core::IUnknown,
    ::windows::core::IInspectable
);
impl ::core::convert::From<InlineUIContainer> for Inline {
    fn from(value: InlineUIContainer) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&InlineUIContainer> for Inline {
    fn from(value: &InlineUIContainer) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::core::convert::From<&InlineUIContainer> for ::windows::core::InParam<'a, Inline> {
    fn from(value: &InlineUIContainer) -> Self {
        ::windows::core::InParam::owned(value.into())
    }
}
impl ::core::convert::From<InlineUIContainer> for TextElement {
    fn from(value: InlineUIContainer) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&InlineUIContainer> for TextElement {
    fn from(value: &InlineUIContainer) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::core::convert::From<&InlineUIContainer> for ::windows::core::InParam<'a, TextElement> {
    fn from(value: &InlineUIContainer) -> Self {
        ::windows::core::InParam::owned(value.into())
    }
}
impl ::core::convert::From<InlineUIContainer> for super::DependencyObject {
    fn from(value: InlineUIContainer) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&InlineUIContainer> for super::DependencyObject {
    fn from(value: &InlineUIContainer) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::core::convert::From<&InlineUIContainer>
    for ::windows::core::InParam<'a, super::DependencyObject>
{
    fn from(value: &InlineUIContainer) -> Self {
        ::windows::core::InParam::owned(value.into())
    }
}
unsafe impl ::core::marker::Send for InlineUIContainer {}
unsafe impl ::core::marker::Sync for InlineUIContainer {}
#[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
#[repr(transparent)]
pub struct Italic(::windows::core::IUnknown);
impl Italic {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<Italic, ::windows::core::IGenericFactory> =
            ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn GetValue(
        &self,
        dp: &super::DependencyProperty,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetValue)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(dp),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn SetValue<'a, P0>(
        &self,
        dp: &super::DependencyProperty,
        value: P0,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IInspectable>>,
    {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetValue)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(dp),
                value.into().abi(),
            )
            .ok()
        }
    }
    pub fn ClearValue(&self, dp: &super::DependencyProperty) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).ClearValue)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(dp),
            )
            .ok()
        }
    }
    pub fn ReadLocalValue(
        &self,
        dp: &super::DependencyProperty,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ReadLocalValue)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(dp),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn GetAnimationBaseValue(
        &self,
        dp: &super::DependencyProperty,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetAnimationBaseValue)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(dp),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn RegisterPropertyChangedCallback(
        &self,
        dp: &super::DependencyProperty,
        callback: &super::DependencyPropertyChangedCallback,
    ) -> ::windows::core::Result<i64> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RegisterPropertyChangedCallback)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(dp),
                ::core::mem::transmute_copy(callback),
                result__.as_mut_ptr(),
            )
            .from_abi::<i64>(result__)
        }
    }
    pub fn UnregisterPropertyChangedCallback(
        &self,
        dp: &super::DependencyProperty,
        token: i64,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).UnregisterPropertyChangedCallback)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(dp),
                token,
            )
            .ok()
        }
    }
    pub fn Dispatcher(&self) -> ::windows::core::Result<::windows::UI::Core::CoreDispatcher> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Dispatcher)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Core::CoreDispatcher>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Dispatching\"`*"]
    #[cfg(feature = "UI_Dispatching")]
    pub fn DispatcherQueue(
        &self,
    ) -> ::windows::core::Result<super::super::Dispatching::DispatcherQueue> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DispatcherQueue)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::super::Dispatching::DispatcherQueue>(result__)
        }
    }
    pub fn Inlines(&self) -> ::windows::core::Result<InlineCollection> {
        let this = &::windows::core::Interface::cast::<ISpan>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Inlines)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<InlineCollection>(result__)
        }
    }
    pub fn SetInlines(&self, value: &InlineCollection) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ISpan>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetInlines)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Name)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn FontSize(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FontSize)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetFontSize(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetFontSize)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn FontFamily(&self) -> ::windows::core::Result<super::Media::FontFamily> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FontFamily)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::Media::FontFamily>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetFontFamily<'a, P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::Media::FontFamily>>,
    {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetFontFamily)(
                ::windows::core::Vtable::as_raw(this),
                value.into().abi(),
            )
            .ok()
        }
    }
    pub fn FontWeight(&self) -> ::windows::core::Result<::windows::UI::Text::FontWeight> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FontWeight)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Text::FontWeight>(result__)
        }
    }
    pub fn SetFontWeight(
        &self,
        value: ::windows::UI::Text::FontWeight,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetFontWeight)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn FontStyle(&self) -> ::windows::core::Result<::windows::UI::Text::FontStyle> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FontStyle)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Text::FontStyle>(result__)
        }
    }
    pub fn SetFontStyle(
        &self,
        value: ::windows::UI::Text::FontStyle,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetFontStyle)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn FontStretch(&self) -> ::windows::core::Result<::windows::UI::Text::FontStretch> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FontStretch)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Text::FontStretch>(result__)
        }
    }
    pub fn SetFontStretch(
        &self,
        value: ::windows::UI::Text::FontStretch,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetFontStretch)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn CharacterSpacing(&self) -> ::windows::core::Result<i32> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CharacterSpacing)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<i32>(result__)
        }
    }
    pub fn SetCharacterSpacing(&self, value: i32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetCharacterSpacing)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn Foreground(&self) -> ::windows::core::Result<super::Media::Brush> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Foreground)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::Media::Brush>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetForeground<'a, P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::Media::Brush>>,
    {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetForeground)(
                ::windows::core::Vtable::as_raw(this),
                value.into().abi(),
            )
            .ok()
        }
    }
    pub fn Language(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
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
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetLanguage)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn IsTextScaleFactorEnabled(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsTextScaleFactorEnabled)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetIsTextScaleFactorEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetIsTextScaleFactorEnabled)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn TextDecorations(&self) -> ::windows::core::Result<::windows::UI::Text::TextDecorations> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TextDecorations)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Text::TextDecorations>(result__)
        }
    }
    pub fn SetTextDecorations(
        &self,
        value: ::windows::UI::Text::TextDecorations,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetTextDecorations)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn ContentStart(&self) -> ::windows::core::Result<TextPointer> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ContentStart)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    pub fn ContentEnd(&self) -> ::windows::core::Result<TextPointer> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ContentEnd)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    pub fn ElementStart(&self) -> ::windows::core::Result<TextPointer> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ElementStart)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    pub fn ElementEnd(&self) -> ::windows::core::Result<TextPointer> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ElementEnd)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    pub fn AllowFocusOnInteraction(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AllowFocusOnInteraction)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetAllowFocusOnInteraction(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetAllowFocusOnInteraction)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn AccessKey(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AccessKey)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetAccessKey(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetAccessKey)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn ExitDisplayModeOnAccessKeyInvoked(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ExitDisplayModeOnAccessKeyInvoked)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetExitDisplayModeOnAccessKeyInvoked(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetExitDisplayModeOnAccessKeyInvoked)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsAccessKeyScope(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsAccessKeyScope)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetIsAccessKeyScope(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetIsAccessKeyScope)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn AccessKeyScopeOwner(&self) -> ::windows::core::Result<super::DependencyObject> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AccessKeyScopeOwner)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::DependencyObject>(result__)
        }
    }
    pub fn SetAccessKeyScopeOwner<'a, P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::DependencyObject>>,
    {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetAccessKeyScopeOwner)(
                ::windows::core::Vtable::as_raw(this),
                value.into().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn KeyTipPlacementMode(
        &self,
    ) -> ::windows::core::Result<super::Input::KeyTipPlacementMode> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).KeyTipPlacementMode)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::Input::KeyTipPlacementMode>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetKeyTipPlacementMode(
        &self,
        value: super::Input::KeyTipPlacementMode,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetKeyTipPlacementMode)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn KeyTipHorizontalOffset(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).KeyTipHorizontalOffset)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetKeyTipHorizontalOffset(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetKeyTipHorizontalOffset)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn KeyTipVerticalOffset(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).KeyTipVerticalOffset)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetKeyTipVerticalOffset(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetKeyTipVerticalOffset)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn XamlRoot(&self) -> ::windows::core::Result<super::XamlRoot> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).XamlRoot)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::XamlRoot>(result__)
        }
    }
    pub fn SetXamlRoot(&self, value: &super::XamlRoot) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetXamlRoot)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn AccessKeyDisplayRequested(
        &self,
        handler: &::windows::Foundation::TypedEventHandler<
            TextElement,
            super::Input::AccessKeyDisplayRequestedEventArgs,
        >,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AccessKeyDisplayRequested)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(handler),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveAccessKeyDisplayRequested(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).RemoveAccessKeyDisplayRequested)(
                ::windows::core::Vtable::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn AccessKeyDisplayDismissed(
        &self,
        handler: &::windows::Foundation::TypedEventHandler<
            TextElement,
            super::Input::AccessKeyDisplayDismissedEventArgs,
        >,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AccessKeyDisplayDismissed)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(handler),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveAccessKeyDisplayDismissed(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).RemoveAccessKeyDisplayDismissed)(
                ::windows::core::Vtable::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn AccessKeyInvoked(
        &self,
        handler: &::windows::Foundation::TypedEventHandler<
            TextElement,
            super::Input::AccessKeyInvokedEventArgs,
        >,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AccessKeyInvoked)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(handler),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveAccessKeyInvoked(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).RemoveAccessKeyInvoked)(
                ::windows::core::Vtable::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn FindName(
        &self,
        name: &::windows::core::HSTRING,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FindName)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(name),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
}
impl ::core::clone::Clone for Italic {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for Italic {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Italic {}
impl ::core::fmt::Debug for Italic {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Italic").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for Italic {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Xaml.Documents.Italic;{ca3cbebd-7a8d-5d7a-8fdf-538e8a680f6c})",
    );
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for Italic {
    type Vtable = IItalic_Vtbl;
}
unsafe impl ::windows::core::Interface for Italic {
    const IID: ::windows::core::GUID = <IItalic as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for Italic {
    const NAME: &'static str = "Microsoft.UI.Xaml.Documents.Italic";
}
::windows::core::interface_hierarchy!(
    Italic,
    ::windows::core::IUnknown,
    ::windows::core::IInspectable
);
impl ::core::convert::From<Italic> for Span {
    fn from(value: Italic) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&Italic> for Span {
    fn from(value: &Italic) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::core::convert::From<&Italic> for ::windows::core::InParam<'a, Span> {
    fn from(value: &Italic) -> Self {
        ::windows::core::InParam::owned(value.into())
    }
}
impl ::core::convert::From<Italic> for Inline {
    fn from(value: Italic) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&Italic> for Inline {
    fn from(value: &Italic) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::core::convert::From<&Italic> for ::windows::core::InParam<'a, Inline> {
    fn from(value: &Italic) -> Self {
        ::windows::core::InParam::owned(value.into())
    }
}
impl ::core::convert::From<Italic> for TextElement {
    fn from(value: Italic) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&Italic> for TextElement {
    fn from(value: &Italic) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::core::convert::From<&Italic> for ::windows::core::InParam<'a, TextElement> {
    fn from(value: &Italic) -> Self {
        ::windows::core::InParam::owned(value.into())
    }
}
impl ::core::convert::From<Italic> for super::DependencyObject {
    fn from(value: Italic) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&Italic> for super::DependencyObject {
    fn from(value: &Italic) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::core::convert::From<&Italic> for ::windows::core::InParam<'a, super::DependencyObject> {
    fn from(value: &Italic) -> Self {
        ::windows::core::InParam::owned(value.into())
    }
}
unsafe impl ::core::marker::Send for Italic {}
unsafe impl ::core::marker::Sync for Italic {}
#[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
#[repr(transparent)]
pub struct LineBreak(::windows::core::IUnknown);
impl LineBreak {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<LineBreak, ::windows::core::IGenericFactory> =
            ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn GetValue(
        &self,
        dp: &super::DependencyProperty,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetValue)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(dp),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn SetValue<'a, P0>(
        &self,
        dp: &super::DependencyProperty,
        value: P0,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IInspectable>>,
    {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetValue)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(dp),
                value.into().abi(),
            )
            .ok()
        }
    }
    pub fn ClearValue(&self, dp: &super::DependencyProperty) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).ClearValue)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(dp),
            )
            .ok()
        }
    }
    pub fn ReadLocalValue(
        &self,
        dp: &super::DependencyProperty,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ReadLocalValue)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(dp),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn GetAnimationBaseValue(
        &self,
        dp: &super::DependencyProperty,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetAnimationBaseValue)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(dp),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn RegisterPropertyChangedCallback(
        &self,
        dp: &super::DependencyProperty,
        callback: &super::DependencyPropertyChangedCallback,
    ) -> ::windows::core::Result<i64> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RegisterPropertyChangedCallback)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(dp),
                ::core::mem::transmute_copy(callback),
                result__.as_mut_ptr(),
            )
            .from_abi::<i64>(result__)
        }
    }
    pub fn UnregisterPropertyChangedCallback(
        &self,
        dp: &super::DependencyProperty,
        token: i64,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).UnregisterPropertyChangedCallback)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(dp),
                token,
            )
            .ok()
        }
    }
    pub fn Dispatcher(&self) -> ::windows::core::Result<::windows::UI::Core::CoreDispatcher> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Dispatcher)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Core::CoreDispatcher>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Dispatching\"`*"]
    #[cfg(feature = "UI_Dispatching")]
    pub fn DispatcherQueue(
        &self,
    ) -> ::windows::core::Result<super::super::Dispatching::DispatcherQueue> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DispatcherQueue)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::super::Dispatching::DispatcherQueue>(result__)
        }
    }
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Name)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn FontSize(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FontSize)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetFontSize(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetFontSize)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn FontFamily(&self) -> ::windows::core::Result<super::Media::FontFamily> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FontFamily)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::Media::FontFamily>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetFontFamily<'a, P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::Media::FontFamily>>,
    {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetFontFamily)(
                ::windows::core::Vtable::as_raw(this),
                value.into().abi(),
            )
            .ok()
        }
    }
    pub fn FontWeight(&self) -> ::windows::core::Result<::windows::UI::Text::FontWeight> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FontWeight)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Text::FontWeight>(result__)
        }
    }
    pub fn SetFontWeight(
        &self,
        value: ::windows::UI::Text::FontWeight,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetFontWeight)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn FontStyle(&self) -> ::windows::core::Result<::windows::UI::Text::FontStyle> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FontStyle)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Text::FontStyle>(result__)
        }
    }
    pub fn SetFontStyle(
        &self,
        value: ::windows::UI::Text::FontStyle,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetFontStyle)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn FontStretch(&self) -> ::windows::core::Result<::windows::UI::Text::FontStretch> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FontStretch)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Text::FontStretch>(result__)
        }
    }
    pub fn SetFontStretch(
        &self,
        value: ::windows::UI::Text::FontStretch,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetFontStretch)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn CharacterSpacing(&self) -> ::windows::core::Result<i32> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CharacterSpacing)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<i32>(result__)
        }
    }
    pub fn SetCharacterSpacing(&self, value: i32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetCharacterSpacing)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn Foreground(&self) -> ::windows::core::Result<super::Media::Brush> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Foreground)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::Media::Brush>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetForeground<'a, P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::Media::Brush>>,
    {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetForeground)(
                ::windows::core::Vtable::as_raw(this),
                value.into().abi(),
            )
            .ok()
        }
    }
    pub fn Language(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
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
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetLanguage)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn IsTextScaleFactorEnabled(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsTextScaleFactorEnabled)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetIsTextScaleFactorEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetIsTextScaleFactorEnabled)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn TextDecorations(&self) -> ::windows::core::Result<::windows::UI::Text::TextDecorations> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TextDecorations)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Text::TextDecorations>(result__)
        }
    }
    pub fn SetTextDecorations(
        &self,
        value: ::windows::UI::Text::TextDecorations,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetTextDecorations)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn ContentStart(&self) -> ::windows::core::Result<TextPointer> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ContentStart)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    pub fn ContentEnd(&self) -> ::windows::core::Result<TextPointer> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ContentEnd)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    pub fn ElementStart(&self) -> ::windows::core::Result<TextPointer> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ElementStart)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    pub fn ElementEnd(&self) -> ::windows::core::Result<TextPointer> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ElementEnd)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    pub fn AllowFocusOnInteraction(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AllowFocusOnInteraction)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetAllowFocusOnInteraction(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetAllowFocusOnInteraction)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn AccessKey(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AccessKey)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetAccessKey(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetAccessKey)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn ExitDisplayModeOnAccessKeyInvoked(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ExitDisplayModeOnAccessKeyInvoked)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetExitDisplayModeOnAccessKeyInvoked(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetExitDisplayModeOnAccessKeyInvoked)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsAccessKeyScope(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsAccessKeyScope)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetIsAccessKeyScope(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetIsAccessKeyScope)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn AccessKeyScopeOwner(&self) -> ::windows::core::Result<super::DependencyObject> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AccessKeyScopeOwner)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::DependencyObject>(result__)
        }
    }
    pub fn SetAccessKeyScopeOwner<'a, P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::DependencyObject>>,
    {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetAccessKeyScopeOwner)(
                ::windows::core::Vtable::as_raw(this),
                value.into().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn KeyTipPlacementMode(
        &self,
    ) -> ::windows::core::Result<super::Input::KeyTipPlacementMode> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).KeyTipPlacementMode)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::Input::KeyTipPlacementMode>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetKeyTipPlacementMode(
        &self,
        value: super::Input::KeyTipPlacementMode,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetKeyTipPlacementMode)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn KeyTipHorizontalOffset(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).KeyTipHorizontalOffset)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetKeyTipHorizontalOffset(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetKeyTipHorizontalOffset)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn KeyTipVerticalOffset(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).KeyTipVerticalOffset)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetKeyTipVerticalOffset(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetKeyTipVerticalOffset)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn XamlRoot(&self) -> ::windows::core::Result<super::XamlRoot> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).XamlRoot)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::XamlRoot>(result__)
        }
    }
    pub fn SetXamlRoot(&self, value: &super::XamlRoot) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetXamlRoot)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn AccessKeyDisplayRequested(
        &self,
        handler: &::windows::Foundation::TypedEventHandler<
            TextElement,
            super::Input::AccessKeyDisplayRequestedEventArgs,
        >,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AccessKeyDisplayRequested)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(handler),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveAccessKeyDisplayRequested(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).RemoveAccessKeyDisplayRequested)(
                ::windows::core::Vtable::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn AccessKeyDisplayDismissed(
        &self,
        handler: &::windows::Foundation::TypedEventHandler<
            TextElement,
            super::Input::AccessKeyDisplayDismissedEventArgs,
        >,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AccessKeyDisplayDismissed)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(handler),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveAccessKeyDisplayDismissed(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).RemoveAccessKeyDisplayDismissed)(
                ::windows::core::Vtable::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn AccessKeyInvoked(
        &self,
        handler: &::windows::Foundation::TypedEventHandler<
            TextElement,
            super::Input::AccessKeyInvokedEventArgs,
        >,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AccessKeyInvoked)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(handler),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveAccessKeyInvoked(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).RemoveAccessKeyInvoked)(
                ::windows::core::Vtable::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn FindName(
        &self,
        name: &::windows::core::HSTRING,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FindName)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(name),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
}
impl ::core::clone::Clone for LineBreak {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for LineBreak {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LineBreak {}
impl ::core::fmt::Debug for LineBreak {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LineBreak").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for LineBreak {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Xaml.Documents.LineBreak;{09307599-7cc2-5f54-b106-728620c16f76})",
    );
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for LineBreak {
    type Vtable = ILineBreak_Vtbl;
}
unsafe impl ::windows::core::Interface for LineBreak {
    const IID: ::windows::core::GUID = <ILineBreak as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for LineBreak {
    const NAME: &'static str = "Microsoft.UI.Xaml.Documents.LineBreak";
}
::windows::core::interface_hierarchy!(
    LineBreak,
    ::windows::core::IUnknown,
    ::windows::core::IInspectable
);
impl ::core::convert::From<LineBreak> for Inline {
    fn from(value: LineBreak) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&LineBreak> for Inline {
    fn from(value: &LineBreak) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::core::convert::From<&LineBreak> for ::windows::core::InParam<'a, Inline> {
    fn from(value: &LineBreak) -> Self {
        ::windows::core::InParam::owned(value.into())
    }
}
impl ::core::convert::From<LineBreak> for TextElement {
    fn from(value: LineBreak) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&LineBreak> for TextElement {
    fn from(value: &LineBreak) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::core::convert::From<&LineBreak> for ::windows::core::InParam<'a, TextElement> {
    fn from(value: &LineBreak) -> Self {
        ::windows::core::InParam::owned(value.into())
    }
}
impl ::core::convert::From<LineBreak> for super::DependencyObject {
    fn from(value: LineBreak) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&LineBreak> for super::DependencyObject {
    fn from(value: &LineBreak) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::core::convert::From<&LineBreak>
    for ::windows::core::InParam<'a, super::DependencyObject>
{
    fn from(value: &LineBreak) -> Self {
        ::windows::core::InParam::owned(value.into())
    }
}
unsafe impl ::core::marker::Send for LineBreak {}
unsafe impl ::core::marker::Sync for LineBreak {}
#[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
#[repr(transparent)]
pub struct Paragraph(::windows::core::IUnknown);
impl Paragraph {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<Paragraph, ::windows::core::IGenericFactory> =
            ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn TextAlignment(&self) -> ::windows::core::Result<super::TextAlignment> {
        let this = &::windows::core::Interface::cast::<IBlock>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TextAlignment)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::TextAlignment>(result__)
        }
    }
    pub fn SetTextAlignment(&self, value: super::TextAlignment) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IBlock>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetTextAlignment)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn HorizontalTextAlignment(&self) -> ::windows::core::Result<super::TextAlignment> {
        let this = &::windows::core::Interface::cast::<IBlock>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).HorizontalTextAlignment)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::TextAlignment>(result__)
        }
    }
    pub fn SetHorizontalTextAlignment(
        &self,
        value: super::TextAlignment,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IBlock>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetHorizontalTextAlignment)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn LineHeight(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<IBlock>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LineHeight)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetLineHeight(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IBlock>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetLineHeight)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn LineStackingStrategy(&self) -> ::windows::core::Result<super::LineStackingStrategy> {
        let this = &::windows::core::Interface::cast::<IBlock>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LineStackingStrategy)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::LineStackingStrategy>(result__)
        }
    }
    pub fn SetLineStackingStrategy(
        &self,
        value: super::LineStackingStrategy,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IBlock>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetLineStackingStrategy)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn Margin(&self) -> ::windows::core::Result<super::Thickness> {
        let this = &::windows::core::Interface::cast::<IBlock>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Margin)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::Thickness>(result__)
        }
    }
    pub fn SetMargin(&self, value: super::Thickness) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IBlock>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetMargin)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn GetValue(
        &self,
        dp: &super::DependencyProperty,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetValue)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(dp),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn SetValue<'a, P0>(
        &self,
        dp: &super::DependencyProperty,
        value: P0,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IInspectable>>,
    {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetValue)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(dp),
                value.into().abi(),
            )
            .ok()
        }
    }
    pub fn ClearValue(&self, dp: &super::DependencyProperty) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).ClearValue)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(dp),
            )
            .ok()
        }
    }
    pub fn ReadLocalValue(
        &self,
        dp: &super::DependencyProperty,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ReadLocalValue)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(dp),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn GetAnimationBaseValue(
        &self,
        dp: &super::DependencyProperty,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetAnimationBaseValue)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(dp),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn RegisterPropertyChangedCallback(
        &self,
        dp: &super::DependencyProperty,
        callback: &super::DependencyPropertyChangedCallback,
    ) -> ::windows::core::Result<i64> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RegisterPropertyChangedCallback)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(dp),
                ::core::mem::transmute_copy(callback),
                result__.as_mut_ptr(),
            )
            .from_abi::<i64>(result__)
        }
    }
    pub fn UnregisterPropertyChangedCallback(
        &self,
        dp: &super::DependencyProperty,
        token: i64,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).UnregisterPropertyChangedCallback)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(dp),
                token,
            )
            .ok()
        }
    }
    pub fn Dispatcher(&self) -> ::windows::core::Result<::windows::UI::Core::CoreDispatcher> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Dispatcher)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Core::CoreDispatcher>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Dispatching\"`*"]
    #[cfg(feature = "UI_Dispatching")]
    pub fn DispatcherQueue(
        &self,
    ) -> ::windows::core::Result<super::super::Dispatching::DispatcherQueue> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DispatcherQueue)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::super::Dispatching::DispatcherQueue>(result__)
        }
    }
    pub fn Inlines(&self) -> ::windows::core::Result<InlineCollection> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Inlines)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<InlineCollection>(result__)
        }
    }
    pub fn TextIndent(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TextIndent)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetTextIndent(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetTextIndent)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn TextIndentProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IParagraphStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TextIndentProperty)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Name)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn FontSize(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FontSize)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetFontSize(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetFontSize)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn FontFamily(&self) -> ::windows::core::Result<super::Media::FontFamily> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FontFamily)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::Media::FontFamily>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetFontFamily<'a, P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::Media::FontFamily>>,
    {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetFontFamily)(
                ::windows::core::Vtable::as_raw(this),
                value.into().abi(),
            )
            .ok()
        }
    }
    pub fn FontWeight(&self) -> ::windows::core::Result<::windows::UI::Text::FontWeight> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FontWeight)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Text::FontWeight>(result__)
        }
    }
    pub fn SetFontWeight(
        &self,
        value: ::windows::UI::Text::FontWeight,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetFontWeight)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn FontStyle(&self) -> ::windows::core::Result<::windows::UI::Text::FontStyle> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FontStyle)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Text::FontStyle>(result__)
        }
    }
    pub fn SetFontStyle(
        &self,
        value: ::windows::UI::Text::FontStyle,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetFontStyle)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn FontStretch(&self) -> ::windows::core::Result<::windows::UI::Text::FontStretch> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FontStretch)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Text::FontStretch>(result__)
        }
    }
    pub fn SetFontStretch(
        &self,
        value: ::windows::UI::Text::FontStretch,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetFontStretch)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn CharacterSpacing(&self) -> ::windows::core::Result<i32> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CharacterSpacing)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<i32>(result__)
        }
    }
    pub fn SetCharacterSpacing(&self, value: i32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetCharacterSpacing)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn Foreground(&self) -> ::windows::core::Result<super::Media::Brush> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Foreground)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::Media::Brush>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetForeground<'a, P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::Media::Brush>>,
    {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetForeground)(
                ::windows::core::Vtable::as_raw(this),
                value.into().abi(),
            )
            .ok()
        }
    }
    pub fn Language(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
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
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetLanguage)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn IsTextScaleFactorEnabled(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsTextScaleFactorEnabled)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetIsTextScaleFactorEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetIsTextScaleFactorEnabled)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn TextDecorations(&self) -> ::windows::core::Result<::windows::UI::Text::TextDecorations> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TextDecorations)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Text::TextDecorations>(result__)
        }
    }
    pub fn SetTextDecorations(
        &self,
        value: ::windows::UI::Text::TextDecorations,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetTextDecorations)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn ContentStart(&self) -> ::windows::core::Result<TextPointer> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ContentStart)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    pub fn ContentEnd(&self) -> ::windows::core::Result<TextPointer> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ContentEnd)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    pub fn ElementStart(&self) -> ::windows::core::Result<TextPointer> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ElementStart)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    pub fn ElementEnd(&self) -> ::windows::core::Result<TextPointer> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ElementEnd)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    pub fn AllowFocusOnInteraction(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AllowFocusOnInteraction)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetAllowFocusOnInteraction(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetAllowFocusOnInteraction)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn AccessKey(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AccessKey)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetAccessKey(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetAccessKey)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn ExitDisplayModeOnAccessKeyInvoked(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ExitDisplayModeOnAccessKeyInvoked)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetExitDisplayModeOnAccessKeyInvoked(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetExitDisplayModeOnAccessKeyInvoked)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsAccessKeyScope(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsAccessKeyScope)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetIsAccessKeyScope(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetIsAccessKeyScope)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn AccessKeyScopeOwner(&self) -> ::windows::core::Result<super::DependencyObject> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AccessKeyScopeOwner)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::DependencyObject>(result__)
        }
    }
    pub fn SetAccessKeyScopeOwner<'a, P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::DependencyObject>>,
    {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetAccessKeyScopeOwner)(
                ::windows::core::Vtable::as_raw(this),
                value.into().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn KeyTipPlacementMode(
        &self,
    ) -> ::windows::core::Result<super::Input::KeyTipPlacementMode> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).KeyTipPlacementMode)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::Input::KeyTipPlacementMode>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetKeyTipPlacementMode(
        &self,
        value: super::Input::KeyTipPlacementMode,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetKeyTipPlacementMode)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn KeyTipHorizontalOffset(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).KeyTipHorizontalOffset)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetKeyTipHorizontalOffset(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetKeyTipHorizontalOffset)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn KeyTipVerticalOffset(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).KeyTipVerticalOffset)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetKeyTipVerticalOffset(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetKeyTipVerticalOffset)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn XamlRoot(&self) -> ::windows::core::Result<super::XamlRoot> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).XamlRoot)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::XamlRoot>(result__)
        }
    }
    pub fn SetXamlRoot(&self, value: &super::XamlRoot) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetXamlRoot)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn AccessKeyDisplayRequested(
        &self,
        handler: &::windows::Foundation::TypedEventHandler<
            TextElement,
            super::Input::AccessKeyDisplayRequestedEventArgs,
        >,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AccessKeyDisplayRequested)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(handler),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveAccessKeyDisplayRequested(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).RemoveAccessKeyDisplayRequested)(
                ::windows::core::Vtable::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn AccessKeyDisplayDismissed(
        &self,
        handler: &::windows::Foundation::TypedEventHandler<
            TextElement,
            super::Input::AccessKeyDisplayDismissedEventArgs,
        >,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AccessKeyDisplayDismissed)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(handler),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveAccessKeyDisplayDismissed(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).RemoveAccessKeyDisplayDismissed)(
                ::windows::core::Vtable::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn AccessKeyInvoked(
        &self,
        handler: &::windows::Foundation::TypedEventHandler<
            TextElement,
            super::Input::AccessKeyInvokedEventArgs,
        >,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AccessKeyInvoked)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(handler),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveAccessKeyInvoked(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).RemoveAccessKeyInvoked)(
                ::windows::core::Vtable::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn FindName(
        &self,
        name: &::windows::core::HSTRING,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FindName)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(name),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc(hidden)]
    pub fn IParagraphStatics<R, F: FnOnce(&IParagraphStatics) -> ::windows::core::Result<R>>(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<Paragraph, IParagraphStatics> =
            ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for Paragraph {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for Paragraph {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Paragraph {}
impl ::core::fmt::Debug for Paragraph {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Paragraph").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for Paragraph {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Xaml.Documents.Paragraph;{9ed64c77-329d-502f-a257-f58398edab51})",
    );
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for Paragraph {
    type Vtable = IParagraph_Vtbl;
}
unsafe impl ::windows::core::Interface for Paragraph {
    const IID: ::windows::core::GUID = <IParagraph as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for Paragraph {
    const NAME: &'static str = "Microsoft.UI.Xaml.Documents.Paragraph";
}
::windows::core::interface_hierarchy!(
    Paragraph,
    ::windows::core::IUnknown,
    ::windows::core::IInspectable
);
impl ::core::convert::From<Paragraph> for Block {
    fn from(value: Paragraph) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&Paragraph> for Block {
    fn from(value: &Paragraph) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::core::convert::From<&Paragraph> for ::windows::core::InParam<'a, Block> {
    fn from(value: &Paragraph) -> Self {
        ::windows::core::InParam::owned(value.into())
    }
}
impl ::core::convert::From<Paragraph> for TextElement {
    fn from(value: Paragraph) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&Paragraph> for TextElement {
    fn from(value: &Paragraph) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::core::convert::From<&Paragraph> for ::windows::core::InParam<'a, TextElement> {
    fn from(value: &Paragraph) -> Self {
        ::windows::core::InParam::owned(value.into())
    }
}
impl ::core::convert::From<Paragraph> for super::DependencyObject {
    fn from(value: Paragraph) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&Paragraph> for super::DependencyObject {
    fn from(value: &Paragraph) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::core::convert::From<&Paragraph>
    for ::windows::core::InParam<'a, super::DependencyObject>
{
    fn from(value: &Paragraph) -> Self {
        ::windows::core::InParam::owned(value.into())
    }
}
unsafe impl ::core::marker::Send for Paragraph {}
unsafe impl ::core::marker::Sync for Paragraph {}
#[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
#[repr(transparent)]
pub struct Run(::windows::core::IUnknown);
impl Run {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<Run, ::windows::core::IGenericFactory> =
            ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn GetValue(
        &self,
        dp: &super::DependencyProperty,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetValue)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(dp),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn SetValue<'a, P0>(
        &self,
        dp: &super::DependencyProperty,
        value: P0,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IInspectable>>,
    {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetValue)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(dp),
                value.into().abi(),
            )
            .ok()
        }
    }
    pub fn ClearValue(&self, dp: &super::DependencyProperty) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).ClearValue)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(dp),
            )
            .ok()
        }
    }
    pub fn ReadLocalValue(
        &self,
        dp: &super::DependencyProperty,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ReadLocalValue)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(dp),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn GetAnimationBaseValue(
        &self,
        dp: &super::DependencyProperty,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetAnimationBaseValue)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(dp),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn RegisterPropertyChangedCallback(
        &self,
        dp: &super::DependencyProperty,
        callback: &super::DependencyPropertyChangedCallback,
    ) -> ::windows::core::Result<i64> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RegisterPropertyChangedCallback)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(dp),
                ::core::mem::transmute_copy(callback),
                result__.as_mut_ptr(),
            )
            .from_abi::<i64>(result__)
        }
    }
    pub fn UnregisterPropertyChangedCallback(
        &self,
        dp: &super::DependencyProperty,
        token: i64,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).UnregisterPropertyChangedCallback)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(dp),
                token,
            )
            .ok()
        }
    }
    pub fn Dispatcher(&self) -> ::windows::core::Result<::windows::UI::Core::CoreDispatcher> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Dispatcher)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Core::CoreDispatcher>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Dispatching\"`*"]
    #[cfg(feature = "UI_Dispatching")]
    pub fn DispatcherQueue(
        &self,
    ) -> ::windows::core::Result<super::super::Dispatching::DispatcherQueue> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DispatcherQueue)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::super::Dispatching::DispatcherQueue>(result__)
        }
    }
    pub fn Text(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Text)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetText)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn FlowDirection(&self) -> ::windows::core::Result<super::FlowDirection> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FlowDirection)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::FlowDirection>(result__)
        }
    }
    pub fn SetFlowDirection(&self, value: super::FlowDirection) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetFlowDirection)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn FlowDirectionProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IRunStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FlowDirectionProperty)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Name)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn FontSize(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FontSize)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetFontSize(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetFontSize)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn FontFamily(&self) -> ::windows::core::Result<super::Media::FontFamily> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FontFamily)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::Media::FontFamily>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetFontFamily<'a, P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::Media::FontFamily>>,
    {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetFontFamily)(
                ::windows::core::Vtable::as_raw(this),
                value.into().abi(),
            )
            .ok()
        }
    }
    pub fn FontWeight(&self) -> ::windows::core::Result<::windows::UI::Text::FontWeight> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FontWeight)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Text::FontWeight>(result__)
        }
    }
    pub fn SetFontWeight(
        &self,
        value: ::windows::UI::Text::FontWeight,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetFontWeight)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn FontStyle(&self) -> ::windows::core::Result<::windows::UI::Text::FontStyle> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FontStyle)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Text::FontStyle>(result__)
        }
    }
    pub fn SetFontStyle(
        &self,
        value: ::windows::UI::Text::FontStyle,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetFontStyle)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn FontStretch(&self) -> ::windows::core::Result<::windows::UI::Text::FontStretch> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FontStretch)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Text::FontStretch>(result__)
        }
    }
    pub fn SetFontStretch(
        &self,
        value: ::windows::UI::Text::FontStretch,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetFontStretch)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn CharacterSpacing(&self) -> ::windows::core::Result<i32> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CharacterSpacing)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<i32>(result__)
        }
    }
    pub fn SetCharacterSpacing(&self, value: i32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetCharacterSpacing)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn Foreground(&self) -> ::windows::core::Result<super::Media::Brush> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Foreground)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::Media::Brush>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetForeground<'a, P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::Media::Brush>>,
    {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetForeground)(
                ::windows::core::Vtable::as_raw(this),
                value.into().abi(),
            )
            .ok()
        }
    }
    pub fn Language(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
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
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetLanguage)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn IsTextScaleFactorEnabled(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsTextScaleFactorEnabled)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetIsTextScaleFactorEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetIsTextScaleFactorEnabled)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn TextDecorations(&self) -> ::windows::core::Result<::windows::UI::Text::TextDecorations> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TextDecorations)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Text::TextDecorations>(result__)
        }
    }
    pub fn SetTextDecorations(
        &self,
        value: ::windows::UI::Text::TextDecorations,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetTextDecorations)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn ContentStart(&self) -> ::windows::core::Result<TextPointer> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ContentStart)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    pub fn ContentEnd(&self) -> ::windows::core::Result<TextPointer> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ContentEnd)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    pub fn ElementStart(&self) -> ::windows::core::Result<TextPointer> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ElementStart)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    pub fn ElementEnd(&self) -> ::windows::core::Result<TextPointer> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ElementEnd)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    pub fn AllowFocusOnInteraction(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AllowFocusOnInteraction)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetAllowFocusOnInteraction(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetAllowFocusOnInteraction)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn AccessKey(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AccessKey)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetAccessKey(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetAccessKey)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn ExitDisplayModeOnAccessKeyInvoked(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ExitDisplayModeOnAccessKeyInvoked)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetExitDisplayModeOnAccessKeyInvoked(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetExitDisplayModeOnAccessKeyInvoked)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsAccessKeyScope(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsAccessKeyScope)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetIsAccessKeyScope(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetIsAccessKeyScope)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn AccessKeyScopeOwner(&self) -> ::windows::core::Result<super::DependencyObject> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AccessKeyScopeOwner)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::DependencyObject>(result__)
        }
    }
    pub fn SetAccessKeyScopeOwner<'a, P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::DependencyObject>>,
    {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetAccessKeyScopeOwner)(
                ::windows::core::Vtable::as_raw(this),
                value.into().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn KeyTipPlacementMode(
        &self,
    ) -> ::windows::core::Result<super::Input::KeyTipPlacementMode> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).KeyTipPlacementMode)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::Input::KeyTipPlacementMode>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetKeyTipPlacementMode(
        &self,
        value: super::Input::KeyTipPlacementMode,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetKeyTipPlacementMode)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn KeyTipHorizontalOffset(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).KeyTipHorizontalOffset)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetKeyTipHorizontalOffset(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetKeyTipHorizontalOffset)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn KeyTipVerticalOffset(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).KeyTipVerticalOffset)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetKeyTipVerticalOffset(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetKeyTipVerticalOffset)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn XamlRoot(&self) -> ::windows::core::Result<super::XamlRoot> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).XamlRoot)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::XamlRoot>(result__)
        }
    }
    pub fn SetXamlRoot(&self, value: &super::XamlRoot) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetXamlRoot)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn AccessKeyDisplayRequested(
        &self,
        handler: &::windows::Foundation::TypedEventHandler<
            TextElement,
            super::Input::AccessKeyDisplayRequestedEventArgs,
        >,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AccessKeyDisplayRequested)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(handler),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveAccessKeyDisplayRequested(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).RemoveAccessKeyDisplayRequested)(
                ::windows::core::Vtable::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn AccessKeyDisplayDismissed(
        &self,
        handler: &::windows::Foundation::TypedEventHandler<
            TextElement,
            super::Input::AccessKeyDisplayDismissedEventArgs,
        >,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AccessKeyDisplayDismissed)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(handler),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveAccessKeyDisplayDismissed(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).RemoveAccessKeyDisplayDismissed)(
                ::windows::core::Vtable::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn AccessKeyInvoked(
        &self,
        handler: &::windows::Foundation::TypedEventHandler<
            TextElement,
            super::Input::AccessKeyInvokedEventArgs,
        >,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AccessKeyInvoked)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(handler),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveAccessKeyInvoked(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).RemoveAccessKeyInvoked)(
                ::windows::core::Vtable::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn FindName(
        &self,
        name: &::windows::core::HSTRING,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FindName)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(name),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc(hidden)]
    pub fn IRunStatics<R, F: FnOnce(&IRunStatics) -> ::windows::core::Result<R>>(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<Run, IRunStatics> =
            ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for Run {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for Run {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Run {}
impl ::core::fmt::Debug for Run {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Run").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for Run {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Xaml.Documents.Run;{1f905239-37cb-590b-9132-3ffb7741906e})",
    );
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for Run {
    type Vtable = IRun_Vtbl;
}
unsafe impl ::windows::core::Interface for Run {
    const IID: ::windows::core::GUID = <IRun as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for Run {
    const NAME: &'static str = "Microsoft.UI.Xaml.Documents.Run";
}
::windows::core::interface_hierarchy!(
    Run,
    ::windows::core::IUnknown,
    ::windows::core::IInspectable
);
impl ::core::convert::From<Run> for Inline {
    fn from(value: Run) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&Run> for Inline {
    fn from(value: &Run) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::core::convert::From<&Run> for ::windows::core::InParam<'a, Inline> {
    fn from(value: &Run) -> Self {
        ::windows::core::InParam::owned(value.into())
    }
}
impl ::core::convert::From<Run> for TextElement {
    fn from(value: Run) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&Run> for TextElement {
    fn from(value: &Run) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::core::convert::From<&Run> for ::windows::core::InParam<'a, TextElement> {
    fn from(value: &Run) -> Self {
        ::windows::core::InParam::owned(value.into())
    }
}
impl ::core::convert::From<Run> for super::DependencyObject {
    fn from(value: Run) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&Run> for super::DependencyObject {
    fn from(value: &Run) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::core::convert::From<&Run> for ::windows::core::InParam<'a, super::DependencyObject> {
    fn from(value: &Run) -> Self {
        ::windows::core::InParam::owned(value.into())
    }
}
unsafe impl ::core::marker::Send for Run {}
unsafe impl ::core::marker::Sync for Run {}
#[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
#[repr(transparent)]
pub struct Span(::windows::core::IUnknown);
impl Span {
    pub fn GetValue(
        &self,
        dp: &super::DependencyProperty,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetValue)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(dp),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn SetValue<'a, P0>(
        &self,
        dp: &super::DependencyProperty,
        value: P0,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IInspectable>>,
    {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetValue)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(dp),
                value.into().abi(),
            )
            .ok()
        }
    }
    pub fn ClearValue(&self, dp: &super::DependencyProperty) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).ClearValue)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(dp),
            )
            .ok()
        }
    }
    pub fn ReadLocalValue(
        &self,
        dp: &super::DependencyProperty,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ReadLocalValue)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(dp),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn GetAnimationBaseValue(
        &self,
        dp: &super::DependencyProperty,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetAnimationBaseValue)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(dp),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn RegisterPropertyChangedCallback(
        &self,
        dp: &super::DependencyProperty,
        callback: &super::DependencyPropertyChangedCallback,
    ) -> ::windows::core::Result<i64> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RegisterPropertyChangedCallback)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(dp),
                ::core::mem::transmute_copy(callback),
                result__.as_mut_ptr(),
            )
            .from_abi::<i64>(result__)
        }
    }
    pub fn UnregisterPropertyChangedCallback(
        &self,
        dp: &super::DependencyProperty,
        token: i64,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).UnregisterPropertyChangedCallback)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(dp),
                token,
            )
            .ok()
        }
    }
    pub fn Dispatcher(&self) -> ::windows::core::Result<::windows::UI::Core::CoreDispatcher> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Dispatcher)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Core::CoreDispatcher>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Dispatching\"`*"]
    #[cfg(feature = "UI_Dispatching")]
    pub fn DispatcherQueue(
        &self,
    ) -> ::windows::core::Result<super::super::Dispatching::DispatcherQueue> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DispatcherQueue)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::super::Dispatching::DispatcherQueue>(result__)
        }
    }
    pub fn Inlines(&self) -> ::windows::core::Result<InlineCollection> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Inlines)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<InlineCollection>(result__)
        }
    }
    pub fn SetInlines(&self, value: &InlineCollection) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetInlines)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn new() -> ::windows::core::Result<Span> {
        Self::ISpanFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateInstance)(
                ::windows::core::Vtable::as_raw(this),
                ::core::ptr::null_mut(),
                &mut ::core::option::Option::<::windows::core::IInspectable>::None as *mut _ as _,
                result__.as_mut_ptr(),
            )
            .from_abi::<Span>(result__)
        })
    }
    pub fn compose<T>(compose: T) -> ::windows::core::Result<Span>
    where
        T: ::windows::core::Compose,
    {
        Self::ISpanFactory(|this| unsafe {
            let (derived__, base__) = ::windows::core::Compose::compose(compose);
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateInstance)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(&derived__),
                base__ as *mut _ as _,
                result__.as_mut_ptr(),
            )
            .from_abi::<Span>(result__)
        })
    }
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Name)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn FontSize(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FontSize)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetFontSize(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetFontSize)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn FontFamily(&self) -> ::windows::core::Result<super::Media::FontFamily> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FontFamily)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::Media::FontFamily>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetFontFamily<'a, P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::Media::FontFamily>>,
    {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetFontFamily)(
                ::windows::core::Vtable::as_raw(this),
                value.into().abi(),
            )
            .ok()
        }
    }
    pub fn FontWeight(&self) -> ::windows::core::Result<::windows::UI::Text::FontWeight> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FontWeight)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Text::FontWeight>(result__)
        }
    }
    pub fn SetFontWeight(
        &self,
        value: ::windows::UI::Text::FontWeight,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetFontWeight)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn FontStyle(&self) -> ::windows::core::Result<::windows::UI::Text::FontStyle> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FontStyle)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Text::FontStyle>(result__)
        }
    }
    pub fn SetFontStyle(
        &self,
        value: ::windows::UI::Text::FontStyle,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetFontStyle)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn FontStretch(&self) -> ::windows::core::Result<::windows::UI::Text::FontStretch> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FontStretch)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Text::FontStretch>(result__)
        }
    }
    pub fn SetFontStretch(
        &self,
        value: ::windows::UI::Text::FontStretch,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetFontStretch)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn CharacterSpacing(&self) -> ::windows::core::Result<i32> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CharacterSpacing)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<i32>(result__)
        }
    }
    pub fn SetCharacterSpacing(&self, value: i32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetCharacterSpacing)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn Foreground(&self) -> ::windows::core::Result<super::Media::Brush> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Foreground)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::Media::Brush>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetForeground<'a, P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::Media::Brush>>,
    {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetForeground)(
                ::windows::core::Vtable::as_raw(this),
                value.into().abi(),
            )
            .ok()
        }
    }
    pub fn Language(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
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
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetLanguage)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn IsTextScaleFactorEnabled(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsTextScaleFactorEnabled)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetIsTextScaleFactorEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetIsTextScaleFactorEnabled)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn TextDecorations(&self) -> ::windows::core::Result<::windows::UI::Text::TextDecorations> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TextDecorations)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Text::TextDecorations>(result__)
        }
    }
    pub fn SetTextDecorations(
        &self,
        value: ::windows::UI::Text::TextDecorations,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetTextDecorations)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn ContentStart(&self) -> ::windows::core::Result<TextPointer> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ContentStart)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    pub fn ContentEnd(&self) -> ::windows::core::Result<TextPointer> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ContentEnd)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    pub fn ElementStart(&self) -> ::windows::core::Result<TextPointer> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ElementStart)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    pub fn ElementEnd(&self) -> ::windows::core::Result<TextPointer> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ElementEnd)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    pub fn AllowFocusOnInteraction(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AllowFocusOnInteraction)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetAllowFocusOnInteraction(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetAllowFocusOnInteraction)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn AccessKey(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AccessKey)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetAccessKey(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetAccessKey)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn ExitDisplayModeOnAccessKeyInvoked(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ExitDisplayModeOnAccessKeyInvoked)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetExitDisplayModeOnAccessKeyInvoked(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetExitDisplayModeOnAccessKeyInvoked)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsAccessKeyScope(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsAccessKeyScope)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetIsAccessKeyScope(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetIsAccessKeyScope)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn AccessKeyScopeOwner(&self) -> ::windows::core::Result<super::DependencyObject> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AccessKeyScopeOwner)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::DependencyObject>(result__)
        }
    }
    pub fn SetAccessKeyScopeOwner<'a, P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::DependencyObject>>,
    {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetAccessKeyScopeOwner)(
                ::windows::core::Vtable::as_raw(this),
                value.into().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn KeyTipPlacementMode(
        &self,
    ) -> ::windows::core::Result<super::Input::KeyTipPlacementMode> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).KeyTipPlacementMode)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::Input::KeyTipPlacementMode>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetKeyTipPlacementMode(
        &self,
        value: super::Input::KeyTipPlacementMode,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetKeyTipPlacementMode)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn KeyTipHorizontalOffset(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).KeyTipHorizontalOffset)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetKeyTipHorizontalOffset(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetKeyTipHorizontalOffset)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn KeyTipVerticalOffset(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).KeyTipVerticalOffset)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetKeyTipVerticalOffset(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetKeyTipVerticalOffset)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn XamlRoot(&self) -> ::windows::core::Result<super::XamlRoot> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).XamlRoot)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::XamlRoot>(result__)
        }
    }
    pub fn SetXamlRoot(&self, value: &super::XamlRoot) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetXamlRoot)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn AccessKeyDisplayRequested(
        &self,
        handler: &::windows::Foundation::TypedEventHandler<
            TextElement,
            super::Input::AccessKeyDisplayRequestedEventArgs,
        >,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AccessKeyDisplayRequested)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(handler),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveAccessKeyDisplayRequested(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).RemoveAccessKeyDisplayRequested)(
                ::windows::core::Vtable::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn AccessKeyDisplayDismissed(
        &self,
        handler: &::windows::Foundation::TypedEventHandler<
            TextElement,
            super::Input::AccessKeyDisplayDismissedEventArgs,
        >,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AccessKeyDisplayDismissed)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(handler),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveAccessKeyDisplayDismissed(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).RemoveAccessKeyDisplayDismissed)(
                ::windows::core::Vtable::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn AccessKeyInvoked(
        &self,
        handler: &::windows::Foundation::TypedEventHandler<
            TextElement,
            super::Input::AccessKeyInvokedEventArgs,
        >,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AccessKeyInvoked)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(handler),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveAccessKeyInvoked(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).RemoveAccessKeyInvoked)(
                ::windows::core::Vtable::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn FindName(
        &self,
        name: &::windows::core::HSTRING,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FindName)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(name),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc(hidden)]
    pub fn ISpanFactory<R, F: FnOnce(&ISpanFactory) -> ::windows::core::Result<R>>(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<Span, ISpanFactory> =
            ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for Span {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for Span {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Span {}
impl ::core::fmt::Debug for Span {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Span").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for Span {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Xaml.Documents.Span;{91b93d4d-4e28-57b9-bffb-3566c2a3c2a1})",
    );
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for Span {
    type Vtable = ISpan_Vtbl;
}
unsafe impl ::windows::core::Interface for Span {
    const IID: ::windows::core::GUID = <ISpan as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for Span {
    const NAME: &'static str = "Microsoft.UI.Xaml.Documents.Span";
}
::windows::core::interface_hierarchy!(
    Span,
    ::windows::core::IUnknown,
    ::windows::core::IInspectable
);
impl ::core::convert::From<Span> for Inline {
    fn from(value: Span) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&Span> for Inline {
    fn from(value: &Span) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::core::convert::From<&Span> for ::windows::core::InParam<'a, Inline> {
    fn from(value: &Span) -> Self {
        ::windows::core::InParam::owned(value.into())
    }
}
impl ::core::convert::From<Span> for TextElement {
    fn from(value: Span) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&Span> for TextElement {
    fn from(value: &Span) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::core::convert::From<&Span> for ::windows::core::InParam<'a, TextElement> {
    fn from(value: &Span) -> Self {
        ::windows::core::InParam::owned(value.into())
    }
}
impl ::core::convert::From<Span> for super::DependencyObject {
    fn from(value: Span) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&Span> for super::DependencyObject {
    fn from(value: &Span) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::core::convert::From<&Span> for ::windows::core::InParam<'a, super::DependencyObject> {
    fn from(value: &Span) -> Self {
        ::windows::core::InParam::owned(value.into())
    }
}
unsafe impl ::core::marker::Send for Span {}
unsafe impl ::core::marker::Sync for Span {}
#[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
#[repr(transparent)]
pub struct TextElement(::windows::core::IUnknown);
impl TextElement {
    pub fn GetValue(
        &self,
        dp: &super::DependencyProperty,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetValue)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(dp),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn SetValue<'a, P0>(
        &self,
        dp: &super::DependencyProperty,
        value: P0,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IInspectable>>,
    {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetValue)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(dp),
                value.into().abi(),
            )
            .ok()
        }
    }
    pub fn ClearValue(&self, dp: &super::DependencyProperty) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).ClearValue)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(dp),
            )
            .ok()
        }
    }
    pub fn ReadLocalValue(
        &self,
        dp: &super::DependencyProperty,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ReadLocalValue)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(dp),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn GetAnimationBaseValue(
        &self,
        dp: &super::DependencyProperty,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetAnimationBaseValue)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(dp),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn RegisterPropertyChangedCallback(
        &self,
        dp: &super::DependencyProperty,
        callback: &super::DependencyPropertyChangedCallback,
    ) -> ::windows::core::Result<i64> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RegisterPropertyChangedCallback)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(dp),
                ::core::mem::transmute_copy(callback),
                result__.as_mut_ptr(),
            )
            .from_abi::<i64>(result__)
        }
    }
    pub fn UnregisterPropertyChangedCallback(
        &self,
        dp: &super::DependencyProperty,
        token: i64,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).UnregisterPropertyChangedCallback)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(dp),
                token,
            )
            .ok()
        }
    }
    pub fn Dispatcher(&self) -> ::windows::core::Result<::windows::UI::Core::CoreDispatcher> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Dispatcher)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Core::CoreDispatcher>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Dispatching\"`*"]
    #[cfg(feature = "UI_Dispatching")]
    pub fn DispatcherQueue(
        &self,
    ) -> ::windows::core::Result<super::super::Dispatching::DispatcherQueue> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DispatcherQueue)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::super::Dispatching::DispatcherQueue>(result__)
        }
    }
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Name)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn FontSize(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FontSize)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetFontSize(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetFontSize)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn FontFamily(&self) -> ::windows::core::Result<super::Media::FontFamily> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FontFamily)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::Media::FontFamily>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetFontFamily<'a, P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::Media::FontFamily>>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetFontFamily)(
                ::windows::core::Vtable::as_raw(this),
                value.into().abi(),
            )
            .ok()
        }
    }
    pub fn FontWeight(&self) -> ::windows::core::Result<::windows::UI::Text::FontWeight> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FontWeight)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Text::FontWeight>(result__)
        }
    }
    pub fn SetFontWeight(
        &self,
        value: ::windows::UI::Text::FontWeight,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetFontWeight)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn FontStyle(&self) -> ::windows::core::Result<::windows::UI::Text::FontStyle> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FontStyle)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Text::FontStyle>(result__)
        }
    }
    pub fn SetFontStyle(
        &self,
        value: ::windows::UI::Text::FontStyle,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetFontStyle)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn FontStretch(&self) -> ::windows::core::Result<::windows::UI::Text::FontStretch> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FontStretch)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Text::FontStretch>(result__)
        }
    }
    pub fn SetFontStretch(
        &self,
        value: ::windows::UI::Text::FontStretch,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetFontStretch)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn CharacterSpacing(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CharacterSpacing)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<i32>(result__)
        }
    }
    pub fn SetCharacterSpacing(&self, value: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetCharacterSpacing)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn Foreground(&self) -> ::windows::core::Result<super::Media::Brush> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Foreground)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::Media::Brush>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetForeground<'a, P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::Media::Brush>>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetForeground)(
                ::windows::core::Vtable::as_raw(this),
                value.into().abi(),
            )
            .ok()
        }
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
    pub fn IsTextScaleFactorEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsTextScaleFactorEnabled)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetIsTextScaleFactorEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetIsTextScaleFactorEnabled)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn TextDecorations(&self) -> ::windows::core::Result<::windows::UI::Text::TextDecorations> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TextDecorations)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Text::TextDecorations>(result__)
        }
    }
    pub fn SetTextDecorations(
        &self,
        value: ::windows::UI::Text::TextDecorations,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetTextDecorations)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn ContentStart(&self) -> ::windows::core::Result<TextPointer> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ContentStart)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    pub fn ContentEnd(&self) -> ::windows::core::Result<TextPointer> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ContentEnd)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    pub fn ElementStart(&self) -> ::windows::core::Result<TextPointer> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ElementStart)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    pub fn ElementEnd(&self) -> ::windows::core::Result<TextPointer> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ElementEnd)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    pub fn AllowFocusOnInteraction(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AllowFocusOnInteraction)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetAllowFocusOnInteraction(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetAllowFocusOnInteraction)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn AccessKey(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AccessKey)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetAccessKey(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetAccessKey)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn ExitDisplayModeOnAccessKeyInvoked(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ExitDisplayModeOnAccessKeyInvoked)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetExitDisplayModeOnAccessKeyInvoked(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetExitDisplayModeOnAccessKeyInvoked)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsAccessKeyScope(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsAccessKeyScope)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetIsAccessKeyScope(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetIsAccessKeyScope)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn AccessKeyScopeOwner(&self) -> ::windows::core::Result<super::DependencyObject> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AccessKeyScopeOwner)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::DependencyObject>(result__)
        }
    }
    pub fn SetAccessKeyScopeOwner<'a, P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::DependencyObject>>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetAccessKeyScopeOwner)(
                ::windows::core::Vtable::as_raw(this),
                value.into().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn KeyTipPlacementMode(
        &self,
    ) -> ::windows::core::Result<super::Input::KeyTipPlacementMode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).KeyTipPlacementMode)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::Input::KeyTipPlacementMode>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetKeyTipPlacementMode(
        &self,
        value: super::Input::KeyTipPlacementMode,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetKeyTipPlacementMode)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn KeyTipHorizontalOffset(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).KeyTipHorizontalOffset)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetKeyTipHorizontalOffset(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetKeyTipHorizontalOffset)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn KeyTipVerticalOffset(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).KeyTipVerticalOffset)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetKeyTipVerticalOffset(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetKeyTipVerticalOffset)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn XamlRoot(&self) -> ::windows::core::Result<super::XamlRoot> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).XamlRoot)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::XamlRoot>(result__)
        }
    }
    pub fn SetXamlRoot(&self, value: &super::XamlRoot) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetXamlRoot)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn AccessKeyDisplayRequested(
        &self,
        handler: &::windows::Foundation::TypedEventHandler<
            TextElement,
            super::Input::AccessKeyDisplayRequestedEventArgs,
        >,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AccessKeyDisplayRequested)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(handler),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveAccessKeyDisplayRequested(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).RemoveAccessKeyDisplayRequested)(
                ::windows::core::Vtable::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn AccessKeyDisplayDismissed(
        &self,
        handler: &::windows::Foundation::TypedEventHandler<
            TextElement,
            super::Input::AccessKeyDisplayDismissedEventArgs,
        >,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AccessKeyDisplayDismissed)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(handler),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveAccessKeyDisplayDismissed(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).RemoveAccessKeyDisplayDismissed)(
                ::windows::core::Vtable::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn AccessKeyInvoked(
        &self,
        handler: &::windows::Foundation::TypedEventHandler<
            TextElement,
            super::Input::AccessKeyInvokedEventArgs,
        >,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AccessKeyInvoked)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(handler),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveAccessKeyInvoked(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).RemoveAccessKeyInvoked)(
                ::windows::core::Vtable::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn FindName(
        &self,
        name: &::windows::core::HSTRING,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FindName)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(name),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn FontSizeProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITextElementStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FontSizeProperty)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn FontFamilyProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITextElementStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FontFamilyProperty)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn FontWeightProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITextElementStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FontWeightProperty)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn FontStyleProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITextElementStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FontStyleProperty)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn FontStretchProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITextElementStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FontStretchProperty)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn CharacterSpacingProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITextElementStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CharacterSpacingProperty)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn ForegroundProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITextElementStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ForegroundProperty)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn LanguageProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITextElementStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LanguageProperty)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn IsTextScaleFactorEnabledProperty() -> ::windows::core::Result<super::DependencyProperty>
    {
        Self::ITextElementStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsTextScaleFactorEnabledProperty)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn TextDecorationsProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITextElementStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TextDecorationsProperty)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn AllowFocusOnInteractionProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITextElementStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AllowFocusOnInteractionProperty)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn AccessKeyProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITextElementStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AccessKeyProperty)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn ExitDisplayModeOnAccessKeyInvokedProperty(
    ) -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITextElementStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ExitDisplayModeOnAccessKeyInvokedProperty)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn IsAccessKeyScopeProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITextElementStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsAccessKeyScopeProperty)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn AccessKeyScopeOwnerProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITextElementStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AccessKeyScopeOwnerProperty)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn KeyTipPlacementModeProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITextElementStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).KeyTipPlacementModeProperty)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn KeyTipHorizontalOffsetProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITextElementStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).KeyTipHorizontalOffsetProperty)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn KeyTipVerticalOffsetProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITextElementStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).KeyTipVerticalOffsetProperty)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ITextElementStatics<R, F: FnOnce(&ITextElementStatics) -> ::windows::core::Result<R>>(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<TextElement, ITextElementStatics> =
            ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for TextElement {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for TextElement {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TextElement {}
impl ::core::fmt::Debug for TextElement {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TextElement").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for TextElement {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Xaml.Documents.TextElement;{a122ba22-833f-5220-a47e-6cd507531abe})",
    );
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for TextElement {
    type Vtable = ITextElement_Vtbl;
}
unsafe impl ::windows::core::Interface for TextElement {
    const IID: ::windows::core::GUID = <ITextElement as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for TextElement {
    const NAME: &'static str = "Microsoft.UI.Xaml.Documents.TextElement";
}
::windows::core::interface_hierarchy!(
    TextElement,
    ::windows::core::IUnknown,
    ::windows::core::IInspectable
);
impl ::core::convert::From<TextElement> for super::DependencyObject {
    fn from(value: TextElement) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&TextElement> for super::DependencyObject {
    fn from(value: &TextElement) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::core::convert::From<&TextElement>
    for ::windows::core::InParam<'a, super::DependencyObject>
{
    fn from(value: &TextElement) -> Self {
        ::windows::core::InParam::owned(value.into())
    }
}
unsafe impl ::core::marker::Send for TextElement {}
unsafe impl ::core::marker::Sync for TextElement {}
#[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
#[repr(transparent)]
pub struct TextHighlighter(::windows::core::IUnknown);
impl TextHighlighter {
    pub fn Ranges(
        &self,
    ) -> ::windows::core::Result<::windows::Foundation::Collections::IVector<TextRange>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Ranges)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::Collections::IVector<TextRange>>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn Foreground(&self) -> ::windows::core::Result<super::Media::Brush> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Foreground)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::Media::Brush>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetForeground<'a, P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::Media::Brush>>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetForeground)(
                ::windows::core::Vtable::as_raw(this),
                value.into().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn Background(&self) -> ::windows::core::Result<super::Media::Brush> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Background)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::Media::Brush>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetBackground<'a, P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::Media::Brush>>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetBackground)(
                ::windows::core::Vtable::as_raw(this),
                value.into().abi(),
            )
            .ok()
        }
    }
    pub fn new() -> ::windows::core::Result<TextHighlighter> {
        Self::ITextHighlighterFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateInstance)(
                ::windows::core::Vtable::as_raw(this),
                ::core::ptr::null_mut(),
                &mut ::core::option::Option::<::windows::core::IInspectable>::None as *mut _ as _,
                result__.as_mut_ptr(),
            )
            .from_abi::<TextHighlighter>(result__)
        })
    }
    pub fn compose<T>(compose: T) -> ::windows::core::Result<TextHighlighter>
    where
        T: ::windows::core::Compose,
    {
        Self::ITextHighlighterFactory(|this| unsafe {
            let (derived__, base__) = ::windows::core::Compose::compose(compose);
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateInstance)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(&derived__),
                base__ as *mut _ as _,
                result__.as_mut_ptr(),
            )
            .from_abi::<TextHighlighter>(result__)
        })
    }
    pub fn ForegroundProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITextHighlighterStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ForegroundProperty)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn BackgroundProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITextHighlighterStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).BackgroundProperty)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ITextHighlighterFactory<
        R,
        F: FnOnce(&ITextHighlighterFactory) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<TextHighlighter, ITextHighlighterFactory> =
            ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ITextHighlighterStatics<
        R,
        F: FnOnce(&ITextHighlighterStatics) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<TextHighlighter, ITextHighlighterStatics> =
            ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for TextHighlighter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for TextHighlighter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TextHighlighter {}
impl ::core::fmt::Debug for TextHighlighter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TextHighlighter").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for TextHighlighter {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Xaml.Documents.TextHighlighter;{b756e861-1d2b-5f6f-81fd-c51a5bc068ff})",
    );
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for TextHighlighter {
    type Vtable = ITextHighlighter_Vtbl;
}
unsafe impl ::windows::core::Interface for TextHighlighter {
    const IID: ::windows::core::GUID = <ITextHighlighter as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for TextHighlighter {
    const NAME: &'static str = "Microsoft.UI.Xaml.Documents.TextHighlighter";
}
::windows::core::interface_hierarchy!(
    TextHighlighter,
    ::windows::core::IUnknown,
    ::windows::core::IInspectable
);
unsafe impl ::core::marker::Send for TextHighlighter {}
unsafe impl ::core::marker::Sync for TextHighlighter {}
#[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
#[repr(transparent)]
pub struct TextHighlighterBase(::windows::core::IUnknown);
impl TextHighlighterBase {
    pub fn GetValue(
        &self,
        dp: &super::DependencyProperty,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetValue)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(dp),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn SetValue<'a, P0>(
        &self,
        dp: &super::DependencyProperty,
        value: P0,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IInspectable>>,
    {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetValue)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(dp),
                value.into().abi(),
            )
            .ok()
        }
    }
    pub fn ClearValue(&self, dp: &super::DependencyProperty) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).ClearValue)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(dp),
            )
            .ok()
        }
    }
    pub fn ReadLocalValue(
        &self,
        dp: &super::DependencyProperty,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ReadLocalValue)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(dp),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn GetAnimationBaseValue(
        &self,
        dp: &super::DependencyProperty,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetAnimationBaseValue)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(dp),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn RegisterPropertyChangedCallback(
        &self,
        dp: &super::DependencyProperty,
        callback: &super::DependencyPropertyChangedCallback,
    ) -> ::windows::core::Result<i64> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RegisterPropertyChangedCallback)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(dp),
                ::core::mem::transmute_copy(callback),
                result__.as_mut_ptr(),
            )
            .from_abi::<i64>(result__)
        }
    }
    pub fn UnregisterPropertyChangedCallback(
        &self,
        dp: &super::DependencyProperty,
        token: i64,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).UnregisterPropertyChangedCallback)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(dp),
                token,
            )
            .ok()
        }
    }
    pub fn Dispatcher(&self) -> ::windows::core::Result<::windows::UI::Core::CoreDispatcher> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Dispatcher)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Core::CoreDispatcher>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Dispatching\"`*"]
    #[cfg(feature = "UI_Dispatching")]
    pub fn DispatcherQueue(
        &self,
    ) -> ::windows::core::Result<super::super::Dispatching::DispatcherQueue> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DispatcherQueue)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::super::Dispatching::DispatcherQueue>(result__)
        }
    }
}
impl ::core::clone::Clone for TextHighlighterBase {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for TextHighlighterBase {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TextHighlighterBase {}
impl ::core::fmt::Debug for TextHighlighterBase {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TextHighlighterBase").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for TextHighlighterBase {
    const SIGNATURE : ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice ( b"rc(Microsoft.UI.Xaml.Documents.TextHighlighterBase;{5c21aaf0-3a17-5468-8aac-be14db0ed8c1})" ) ;
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for TextHighlighterBase {
    type Vtable = ITextHighlighterBase_Vtbl;
}
unsafe impl ::windows::core::Interface for TextHighlighterBase {
    const IID: ::windows::core::GUID = <ITextHighlighterBase as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for TextHighlighterBase {
    const NAME: &'static str = "Microsoft.UI.Xaml.Documents.TextHighlighterBase";
}
::windows::core::interface_hierarchy!(
    TextHighlighterBase,
    ::windows::core::IUnknown,
    ::windows::core::IInspectable
);
impl ::core::convert::From<TextHighlighterBase> for super::DependencyObject {
    fn from(value: TextHighlighterBase) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&TextHighlighterBase> for super::DependencyObject {
    fn from(value: &TextHighlighterBase) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::core::convert::From<&TextHighlighterBase>
    for ::windows::core::InParam<'a, super::DependencyObject>
{
    fn from(value: &TextHighlighterBase) -> Self {
        ::windows::core::InParam::owned(value.into())
    }
}
unsafe impl ::core::marker::Send for TextHighlighterBase {}
unsafe impl ::core::marker::Sync for TextHighlighterBase {}
#[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
#[repr(transparent)]
pub struct TextPointer(::windows::core::IUnknown);
impl TextPointer {
    pub fn Parent(&self) -> ::windows::core::Result<super::DependencyObject> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Parent)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::DependencyObject>(result__)
        }
    }
    pub fn VisualParent(&self) -> ::windows::core::Result<super::FrameworkElement> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).VisualParent)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::FrameworkElement>(result__)
        }
    }
    pub fn LogicalDirection(&self) -> ::windows::core::Result<LogicalDirection> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LogicalDirection)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<LogicalDirection>(result__)
        }
    }
    pub fn Offset(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Offset)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<i32>(result__)
        }
    }
    pub fn GetCharacterRect(
        &self,
        direction: LogicalDirection,
    ) -> ::windows::core::Result<::windows::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetCharacterRect)(
                ::windows::core::Vtable::as_raw(this),
                direction,
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::Rect>(result__)
        }
    }
    pub fn GetPositionAtOffset(
        &self,
        offset: i32,
        direction: LogicalDirection,
    ) -> ::windows::core::Result<TextPointer> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetPositionAtOffset)(
                ::windows::core::Vtable::as_raw(this),
                offset,
                direction,
                result__.as_mut_ptr(),
            )
            .from_abi::<TextPointer>(result__)
        }
    }
}
impl ::core::clone::Clone for TextPointer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for TextPointer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TextPointer {}
impl ::core::fmt::Debug for TextPointer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TextPointer").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for TextPointer {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Xaml.Documents.TextPointer;{842eb385-ee41-5930-979b-438fa7525a51})",
    );
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for TextPointer {
    type Vtable = ITextPointer_Vtbl;
}
unsafe impl ::windows::core::Interface for TextPointer {
    const IID: ::windows::core::GUID = <ITextPointer as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for TextPointer {
    const NAME: &'static str = "Microsoft.UI.Xaml.Documents.TextPointer";
}
::windows::core::interface_hierarchy!(
    TextPointer,
    ::windows::core::IUnknown,
    ::windows::core::IInspectable
);
unsafe impl ::core::marker::Send for TextPointer {}
unsafe impl ::core::marker::Sync for TextPointer {}
#[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
#[repr(transparent)]
pub struct Typography(::windows::core::IUnknown);
impl Typography {
    pub fn AnnotationAlternatesProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AnnotationAlternatesProperty)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn GetAnnotationAlternates<'a, P0>(element: P0) -> ::windows::core::Result<i32>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::DependencyObject>>,
    {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetAnnotationAlternates)(
                ::windows::core::Vtable::as_raw(this),
                element.into().abi(),
                result__.as_mut_ptr(),
            )
            .from_abi::<i32>(result__)
        })
    }
    pub fn SetAnnotationAlternates<'a, P0>(element: P0, value: i32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::DependencyObject>>,
    {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::core::Vtable::vtable(this).SetAnnotationAlternates)(
                ::windows::core::Vtable::as_raw(this),
                element.into().abi(),
                value,
            )
            .ok()
        })
    }
    pub fn EastAsianExpertFormsProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).EastAsianExpertFormsProperty)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn GetEastAsianExpertForms<'a, P0>(element: P0) -> ::windows::core::Result<bool>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::DependencyObject>>,
    {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetEastAsianExpertForms)(
                ::windows::core::Vtable::as_raw(this),
                element.into().abi(),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        })
    }
    pub fn SetEastAsianExpertForms<'a, P0>(element: P0, value: bool) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::DependencyObject>>,
    {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::core::Vtable::vtable(this).SetEastAsianExpertForms)(
                ::windows::core::Vtable::as_raw(this),
                element.into().abi(),
                value,
            )
            .ok()
        })
    }
    pub fn EastAsianLanguageProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).EastAsianLanguageProperty)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn GetEastAsianLanguage<'a, P0>(
        element: P0,
    ) -> ::windows::core::Result<super::FontEastAsianLanguage>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::DependencyObject>>,
    {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetEastAsianLanguage)(
                ::windows::core::Vtable::as_raw(this),
                element.into().abi(),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::FontEastAsianLanguage>(result__)
        })
    }
    pub fn SetEastAsianLanguage<'a, P0>(
        element: P0,
        value: super::FontEastAsianLanguage,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::DependencyObject>>,
    {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::core::Vtable::vtable(this).SetEastAsianLanguage)(
                ::windows::core::Vtable::as_raw(this),
                element.into().abi(),
                value,
            )
            .ok()
        })
    }
    pub fn EastAsianWidthsProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).EastAsianWidthsProperty)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn GetEastAsianWidths<'a, P0>(
        element: P0,
    ) -> ::windows::core::Result<super::FontEastAsianWidths>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::DependencyObject>>,
    {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetEastAsianWidths)(
                ::windows::core::Vtable::as_raw(this),
                element.into().abi(),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::FontEastAsianWidths>(result__)
        })
    }
    pub fn SetEastAsianWidths<'a, P0>(
        element: P0,
        value: super::FontEastAsianWidths,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::DependencyObject>>,
    {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::core::Vtable::vtable(this).SetEastAsianWidths)(
                ::windows::core::Vtable::as_raw(this),
                element.into().abi(),
                value,
            )
            .ok()
        })
    }
    pub fn StandardLigaturesProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).StandardLigaturesProperty)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn GetStandardLigatures<'a, P0>(element: P0) -> ::windows::core::Result<bool>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::DependencyObject>>,
    {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetStandardLigatures)(
                ::windows::core::Vtable::as_raw(this),
                element.into().abi(),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        })
    }
    pub fn SetStandardLigatures<'a, P0>(element: P0, value: bool) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::DependencyObject>>,
    {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::core::Vtable::vtable(this).SetStandardLigatures)(
                ::windows::core::Vtable::as_raw(this),
                element.into().abi(),
                value,
            )
            .ok()
        })
    }
    pub fn ContextualLigaturesProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ContextualLigaturesProperty)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn GetContextualLigatures<'a, P0>(element: P0) -> ::windows::core::Result<bool>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::DependencyObject>>,
    {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetContextualLigatures)(
                ::windows::core::Vtable::as_raw(this),
                element.into().abi(),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        })
    }
    pub fn SetContextualLigatures<'a, P0>(element: P0, value: bool) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::DependencyObject>>,
    {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::core::Vtable::vtable(this).SetContextualLigatures)(
                ::windows::core::Vtable::as_raw(this),
                element.into().abi(),
                value,
            )
            .ok()
        })
    }
    pub fn DiscretionaryLigaturesProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DiscretionaryLigaturesProperty)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn GetDiscretionaryLigatures<'a, P0>(element: P0) -> ::windows::core::Result<bool>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::DependencyObject>>,
    {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDiscretionaryLigatures)(
                ::windows::core::Vtable::as_raw(this),
                element.into().abi(),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        })
    }
    pub fn SetDiscretionaryLigatures<'a, P0>(
        element: P0,
        value: bool,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::DependencyObject>>,
    {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::core::Vtable::vtable(this).SetDiscretionaryLigatures)(
                ::windows::core::Vtable::as_raw(this),
                element.into().abi(),
                value,
            )
            .ok()
        })
    }
    pub fn HistoricalLigaturesProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).HistoricalLigaturesProperty)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn GetHistoricalLigatures<'a, P0>(element: P0) -> ::windows::core::Result<bool>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::DependencyObject>>,
    {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetHistoricalLigatures)(
                ::windows::core::Vtable::as_raw(this),
                element.into().abi(),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        })
    }
    pub fn SetHistoricalLigatures<'a, P0>(element: P0, value: bool) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::DependencyObject>>,
    {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::core::Vtable::vtable(this).SetHistoricalLigatures)(
                ::windows::core::Vtable::as_raw(this),
                element.into().abi(),
                value,
            )
            .ok()
        })
    }
    pub fn StandardSwashesProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).StandardSwashesProperty)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn GetStandardSwashes<'a, P0>(element: P0) -> ::windows::core::Result<i32>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::DependencyObject>>,
    {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetStandardSwashes)(
                ::windows::core::Vtable::as_raw(this),
                element.into().abi(),
                result__.as_mut_ptr(),
            )
            .from_abi::<i32>(result__)
        })
    }
    pub fn SetStandardSwashes<'a, P0>(element: P0, value: i32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::DependencyObject>>,
    {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::core::Vtable::vtable(this).SetStandardSwashes)(
                ::windows::core::Vtable::as_raw(this),
                element.into().abi(),
                value,
            )
            .ok()
        })
    }
    pub fn ContextualSwashesProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ContextualSwashesProperty)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn GetContextualSwashes<'a, P0>(element: P0) -> ::windows::core::Result<i32>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::DependencyObject>>,
    {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetContextualSwashes)(
                ::windows::core::Vtable::as_raw(this),
                element.into().abi(),
                result__.as_mut_ptr(),
            )
            .from_abi::<i32>(result__)
        })
    }
    pub fn SetContextualSwashes<'a, P0>(element: P0, value: i32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::DependencyObject>>,
    {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::core::Vtable::vtable(this).SetContextualSwashes)(
                ::windows::core::Vtable::as_raw(this),
                element.into().abi(),
                value,
            )
            .ok()
        })
    }
    pub fn ContextualAlternatesProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ContextualAlternatesProperty)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn GetContextualAlternates<'a, P0>(element: P0) -> ::windows::core::Result<bool>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::DependencyObject>>,
    {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetContextualAlternates)(
                ::windows::core::Vtable::as_raw(this),
                element.into().abi(),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        })
    }
    pub fn SetContextualAlternates<'a, P0>(element: P0, value: bool) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::DependencyObject>>,
    {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::core::Vtable::vtable(this).SetContextualAlternates)(
                ::windows::core::Vtable::as_raw(this),
                element.into().abi(),
                value,
            )
            .ok()
        })
    }
    pub fn StylisticAlternatesProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).StylisticAlternatesProperty)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn GetStylisticAlternates<'a, P0>(element: P0) -> ::windows::core::Result<i32>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::DependencyObject>>,
    {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetStylisticAlternates)(
                ::windows::core::Vtable::as_raw(this),
                element.into().abi(),
                result__.as_mut_ptr(),
            )
            .from_abi::<i32>(result__)
        })
    }
    pub fn SetStylisticAlternates<'a, P0>(element: P0, value: i32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::DependencyObject>>,
    {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::core::Vtable::vtable(this).SetStylisticAlternates)(
                ::windows::core::Vtable::as_raw(this),
                element.into().abi(),
                value,
            )
            .ok()
        })
    }
    pub fn StylisticSet1Property() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).StylisticSet1Property)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn GetStylisticSet1<'a, P0>(element: P0) -> ::windows::core::Result<bool>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::DependencyObject>>,
    {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetStylisticSet1)(
                ::windows::core::Vtable::as_raw(this),
                element.into().abi(),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        })
    }
    pub fn SetStylisticSet1<'a, P0>(element: P0, value: bool) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::DependencyObject>>,
    {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::core::Vtable::vtable(this).SetStylisticSet1)(
                ::windows::core::Vtable::as_raw(this),
                element.into().abi(),
                value,
            )
            .ok()
        })
    }
    pub fn StylisticSet2Property() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).StylisticSet2Property)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn GetStylisticSet2<'a, P0>(element: P0) -> ::windows::core::Result<bool>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::DependencyObject>>,
    {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetStylisticSet2)(
                ::windows::core::Vtable::as_raw(this),
                element.into().abi(),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        })
    }
    pub fn SetStylisticSet2<'a, P0>(element: P0, value: bool) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::DependencyObject>>,
    {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::core::Vtable::vtable(this).SetStylisticSet2)(
                ::windows::core::Vtable::as_raw(this),
                element.into().abi(),
                value,
            )
            .ok()
        })
    }
    pub fn StylisticSet3Property() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).StylisticSet3Property)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn GetStylisticSet3<'a, P0>(element: P0) -> ::windows::core::Result<bool>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::DependencyObject>>,
    {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetStylisticSet3)(
                ::windows::core::Vtable::as_raw(this),
                element.into().abi(),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        })
    }
    pub fn SetStylisticSet3<'a, P0>(element: P0, value: bool) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::DependencyObject>>,
    {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::core::Vtable::vtable(this).SetStylisticSet3)(
                ::windows::core::Vtable::as_raw(this),
                element.into().abi(),
                value,
            )
            .ok()
        })
    }
    pub fn StylisticSet4Property() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).StylisticSet4Property)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn GetStylisticSet4<'a, P0>(element: P0) -> ::windows::core::Result<bool>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::DependencyObject>>,
    {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetStylisticSet4)(
                ::windows::core::Vtable::as_raw(this),
                element.into().abi(),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        })
    }
    pub fn SetStylisticSet4<'a, P0>(element: P0, value: bool) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::DependencyObject>>,
    {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::core::Vtable::vtable(this).SetStylisticSet4)(
                ::windows::core::Vtable::as_raw(this),
                element.into().abi(),
                value,
            )
            .ok()
        })
    }
    pub fn StylisticSet5Property() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).StylisticSet5Property)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn GetStylisticSet5<'a, P0>(element: P0) -> ::windows::core::Result<bool>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::DependencyObject>>,
    {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetStylisticSet5)(
                ::windows::core::Vtable::as_raw(this),
                element.into().abi(),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        })
    }
    pub fn SetStylisticSet5<'a, P0>(element: P0, value: bool) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::DependencyObject>>,
    {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::core::Vtable::vtable(this).SetStylisticSet5)(
                ::windows::core::Vtable::as_raw(this),
                element.into().abi(),
                value,
            )
            .ok()
        })
    }
    pub fn StylisticSet6Property() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).StylisticSet6Property)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn GetStylisticSet6<'a, P0>(element: P0) -> ::windows::core::Result<bool>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::DependencyObject>>,
    {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetStylisticSet6)(
                ::windows::core::Vtable::as_raw(this),
                element.into().abi(),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        })
    }
    pub fn SetStylisticSet6<'a, P0>(element: P0, value: bool) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::DependencyObject>>,
    {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::core::Vtable::vtable(this).SetStylisticSet6)(
                ::windows::core::Vtable::as_raw(this),
                element.into().abi(),
                value,
            )
            .ok()
        })
    }
    pub fn StylisticSet7Property() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).StylisticSet7Property)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn GetStylisticSet7<'a, P0>(element: P0) -> ::windows::core::Result<bool>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::DependencyObject>>,
    {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetStylisticSet7)(
                ::windows::core::Vtable::as_raw(this),
                element.into().abi(),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        })
    }
    pub fn SetStylisticSet7<'a, P0>(element: P0, value: bool) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::DependencyObject>>,
    {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::core::Vtable::vtable(this).SetStylisticSet7)(
                ::windows::core::Vtable::as_raw(this),
                element.into().abi(),
                value,
            )
            .ok()
        })
    }
    pub fn StylisticSet8Property() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).StylisticSet8Property)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn GetStylisticSet8<'a, P0>(element: P0) -> ::windows::core::Result<bool>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::DependencyObject>>,
    {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetStylisticSet8)(
                ::windows::core::Vtable::as_raw(this),
                element.into().abi(),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        })
    }
    pub fn SetStylisticSet8<'a, P0>(element: P0, value: bool) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::DependencyObject>>,
    {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::core::Vtable::vtable(this).SetStylisticSet8)(
                ::windows::core::Vtable::as_raw(this),
                element.into().abi(),
                value,
            )
            .ok()
        })
    }
    pub fn StylisticSet9Property() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).StylisticSet9Property)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn GetStylisticSet9<'a, P0>(element: P0) -> ::windows::core::Result<bool>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::DependencyObject>>,
    {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetStylisticSet9)(
                ::windows::core::Vtable::as_raw(this),
                element.into().abi(),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        })
    }
    pub fn SetStylisticSet9<'a, P0>(element: P0, value: bool) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::DependencyObject>>,
    {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::core::Vtable::vtable(this).SetStylisticSet9)(
                ::windows::core::Vtable::as_raw(this),
                element.into().abi(),
                value,
            )
            .ok()
        })
    }
    pub fn StylisticSet10Property() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).StylisticSet10Property)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn GetStylisticSet10<'a, P0>(element: P0) -> ::windows::core::Result<bool>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::DependencyObject>>,
    {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetStylisticSet10)(
                ::windows::core::Vtable::as_raw(this),
                element.into().abi(),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        })
    }
    pub fn SetStylisticSet10<'a, P0>(element: P0, value: bool) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::DependencyObject>>,
    {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::core::Vtable::vtable(this).SetStylisticSet10)(
                ::windows::core::Vtable::as_raw(this),
                element.into().abi(),
                value,
            )
            .ok()
        })
    }
    pub fn StylisticSet11Property() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).StylisticSet11Property)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn GetStylisticSet11<'a, P0>(element: P0) -> ::windows::core::Result<bool>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::DependencyObject>>,
    {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetStylisticSet11)(
                ::windows::core::Vtable::as_raw(this),
                element.into().abi(),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        })
    }
    pub fn SetStylisticSet11<'a, P0>(element: P0, value: bool) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::DependencyObject>>,
    {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::core::Vtable::vtable(this).SetStylisticSet11)(
                ::windows::core::Vtable::as_raw(this),
                element.into().abi(),
                value,
            )
            .ok()
        })
    }
    pub fn StylisticSet12Property() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).StylisticSet12Property)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn GetStylisticSet12<'a, P0>(element: P0) -> ::windows::core::Result<bool>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::DependencyObject>>,
    {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetStylisticSet12)(
                ::windows::core::Vtable::as_raw(this),
                element.into().abi(),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        })
    }
    pub fn SetStylisticSet12<'a, P0>(element: P0, value: bool) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::DependencyObject>>,
    {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::core::Vtable::vtable(this).SetStylisticSet12)(
                ::windows::core::Vtable::as_raw(this),
                element.into().abi(),
                value,
            )
            .ok()
        })
    }
    pub fn StylisticSet13Property() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).StylisticSet13Property)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn GetStylisticSet13<'a, P0>(element: P0) -> ::windows::core::Result<bool>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::DependencyObject>>,
    {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetStylisticSet13)(
                ::windows::core::Vtable::as_raw(this),
                element.into().abi(),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        })
    }
    pub fn SetStylisticSet13<'a, P0>(element: P0, value: bool) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::DependencyObject>>,
    {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::core::Vtable::vtable(this).SetStylisticSet13)(
                ::windows::core::Vtable::as_raw(this),
                element.into().abi(),
                value,
            )
            .ok()
        })
    }
    pub fn StylisticSet14Property() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).StylisticSet14Property)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn GetStylisticSet14<'a, P0>(element: P0) -> ::windows::core::Result<bool>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::DependencyObject>>,
    {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetStylisticSet14)(
                ::windows::core::Vtable::as_raw(this),
                element.into().abi(),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        })
    }
    pub fn SetStylisticSet14<'a, P0>(element: P0, value: bool) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::DependencyObject>>,
    {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::core::Vtable::vtable(this).SetStylisticSet14)(
                ::windows::core::Vtable::as_raw(this),
                element.into().abi(),
                value,
            )
            .ok()
        })
    }
    pub fn StylisticSet15Property() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).StylisticSet15Property)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn GetStylisticSet15<'a, P0>(element: P0) -> ::windows::core::Result<bool>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::DependencyObject>>,
    {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetStylisticSet15)(
                ::windows::core::Vtable::as_raw(this),
                element.into().abi(),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        })
    }
    pub fn SetStylisticSet15<'a, P0>(element: P0, value: bool) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::DependencyObject>>,
    {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::core::Vtable::vtable(this).SetStylisticSet15)(
                ::windows::core::Vtable::as_raw(this),
                element.into().abi(),
                value,
            )
            .ok()
        })
    }
    pub fn StylisticSet16Property() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).StylisticSet16Property)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn GetStylisticSet16<'a, P0>(element: P0) -> ::windows::core::Result<bool>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::DependencyObject>>,
    {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetStylisticSet16)(
                ::windows::core::Vtable::as_raw(this),
                element.into().abi(),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        })
    }
    pub fn SetStylisticSet16<'a, P0>(element: P0, value: bool) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::DependencyObject>>,
    {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::core::Vtable::vtable(this).SetStylisticSet16)(
                ::windows::core::Vtable::as_raw(this),
                element.into().abi(),
                value,
            )
            .ok()
        })
    }
    pub fn StylisticSet17Property() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).StylisticSet17Property)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn GetStylisticSet17<'a, P0>(element: P0) -> ::windows::core::Result<bool>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::DependencyObject>>,
    {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetStylisticSet17)(
                ::windows::core::Vtable::as_raw(this),
                element.into().abi(),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        })
    }
    pub fn SetStylisticSet17<'a, P0>(element: P0, value: bool) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::DependencyObject>>,
    {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::core::Vtable::vtable(this).SetStylisticSet17)(
                ::windows::core::Vtable::as_raw(this),
                element.into().abi(),
                value,
            )
            .ok()
        })
    }
    pub fn StylisticSet18Property() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).StylisticSet18Property)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn GetStylisticSet18<'a, P0>(element: P0) -> ::windows::core::Result<bool>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::DependencyObject>>,
    {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetStylisticSet18)(
                ::windows::core::Vtable::as_raw(this),
                element.into().abi(),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        })
    }
    pub fn SetStylisticSet18<'a, P0>(element: P0, value: bool) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::DependencyObject>>,
    {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::core::Vtable::vtable(this).SetStylisticSet18)(
                ::windows::core::Vtable::as_raw(this),
                element.into().abi(),
                value,
            )
            .ok()
        })
    }
    pub fn StylisticSet19Property() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).StylisticSet19Property)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn GetStylisticSet19<'a, P0>(element: P0) -> ::windows::core::Result<bool>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::DependencyObject>>,
    {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetStylisticSet19)(
                ::windows::core::Vtable::as_raw(this),
                element.into().abi(),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        })
    }
    pub fn SetStylisticSet19<'a, P0>(element: P0, value: bool) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::DependencyObject>>,
    {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::core::Vtable::vtable(this).SetStylisticSet19)(
                ::windows::core::Vtable::as_raw(this),
                element.into().abi(),
                value,
            )
            .ok()
        })
    }
    pub fn StylisticSet20Property() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).StylisticSet20Property)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn GetStylisticSet20<'a, P0>(element: P0) -> ::windows::core::Result<bool>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::DependencyObject>>,
    {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetStylisticSet20)(
                ::windows::core::Vtable::as_raw(this),
                element.into().abi(),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        })
    }
    pub fn SetStylisticSet20<'a, P0>(element: P0, value: bool) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::DependencyObject>>,
    {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::core::Vtable::vtable(this).SetStylisticSet20)(
                ::windows::core::Vtable::as_raw(this),
                element.into().abi(),
                value,
            )
            .ok()
        })
    }
    pub fn CapitalsProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CapitalsProperty)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn GetCapitals<'a, P0>(element: P0) -> ::windows::core::Result<super::FontCapitals>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::DependencyObject>>,
    {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetCapitals)(
                ::windows::core::Vtable::as_raw(this),
                element.into().abi(),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::FontCapitals>(result__)
        })
    }
    pub fn SetCapitals<'a, P0>(
        element: P0,
        value: super::FontCapitals,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::DependencyObject>>,
    {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::core::Vtable::vtable(this).SetCapitals)(
                ::windows::core::Vtable::as_raw(this),
                element.into().abi(),
                value,
            )
            .ok()
        })
    }
    pub fn CapitalSpacingProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CapitalSpacingProperty)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn GetCapitalSpacing<'a, P0>(element: P0) -> ::windows::core::Result<bool>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::DependencyObject>>,
    {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetCapitalSpacing)(
                ::windows::core::Vtable::as_raw(this),
                element.into().abi(),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        })
    }
    pub fn SetCapitalSpacing<'a, P0>(element: P0, value: bool) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::DependencyObject>>,
    {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::core::Vtable::vtable(this).SetCapitalSpacing)(
                ::windows::core::Vtable::as_raw(this),
                element.into().abi(),
                value,
            )
            .ok()
        })
    }
    pub fn KerningProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).KerningProperty)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn GetKerning<'a, P0>(element: P0) -> ::windows::core::Result<bool>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::DependencyObject>>,
    {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetKerning)(
                ::windows::core::Vtable::as_raw(this),
                element.into().abi(),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        })
    }
    pub fn SetKerning<'a, P0>(element: P0, value: bool) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::DependencyObject>>,
    {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::core::Vtable::vtable(this).SetKerning)(
                ::windows::core::Vtable::as_raw(this),
                element.into().abi(),
                value,
            )
            .ok()
        })
    }
    pub fn CaseSensitiveFormsProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CaseSensitiveFormsProperty)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn GetCaseSensitiveForms<'a, P0>(element: P0) -> ::windows::core::Result<bool>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::DependencyObject>>,
    {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetCaseSensitiveForms)(
                ::windows::core::Vtable::as_raw(this),
                element.into().abi(),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        })
    }
    pub fn SetCaseSensitiveForms<'a, P0>(element: P0, value: bool) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::DependencyObject>>,
    {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::core::Vtable::vtable(this).SetCaseSensitiveForms)(
                ::windows::core::Vtable::as_raw(this),
                element.into().abi(),
                value,
            )
            .ok()
        })
    }
    pub fn HistoricalFormsProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).HistoricalFormsProperty)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn GetHistoricalForms<'a, P0>(element: P0) -> ::windows::core::Result<bool>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::DependencyObject>>,
    {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetHistoricalForms)(
                ::windows::core::Vtable::as_raw(this),
                element.into().abi(),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        })
    }
    pub fn SetHistoricalForms<'a, P0>(element: P0, value: bool) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::DependencyObject>>,
    {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::core::Vtable::vtable(this).SetHistoricalForms)(
                ::windows::core::Vtable::as_raw(this),
                element.into().abi(),
                value,
            )
            .ok()
        })
    }
    pub fn FractionProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FractionProperty)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn GetFraction<'a, P0>(element: P0) -> ::windows::core::Result<super::FontFraction>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::DependencyObject>>,
    {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetFraction)(
                ::windows::core::Vtable::as_raw(this),
                element.into().abi(),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::FontFraction>(result__)
        })
    }
    pub fn SetFraction<'a, P0>(
        element: P0,
        value: super::FontFraction,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::DependencyObject>>,
    {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::core::Vtable::vtable(this).SetFraction)(
                ::windows::core::Vtable::as_raw(this),
                element.into().abi(),
                value,
            )
            .ok()
        })
    }
    pub fn NumeralStyleProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NumeralStyleProperty)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn GetNumeralStyle<'a, P0>(element: P0) -> ::windows::core::Result<super::FontNumeralStyle>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::DependencyObject>>,
    {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetNumeralStyle)(
                ::windows::core::Vtable::as_raw(this),
                element.into().abi(),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::FontNumeralStyle>(result__)
        })
    }
    pub fn SetNumeralStyle<'a, P0>(
        element: P0,
        value: super::FontNumeralStyle,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::DependencyObject>>,
    {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::core::Vtable::vtable(this).SetNumeralStyle)(
                ::windows::core::Vtable::as_raw(this),
                element.into().abi(),
                value,
            )
            .ok()
        })
    }
    pub fn NumeralAlignmentProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NumeralAlignmentProperty)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn GetNumeralAlignment<'a, P0>(
        element: P0,
    ) -> ::windows::core::Result<super::FontNumeralAlignment>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::DependencyObject>>,
    {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetNumeralAlignment)(
                ::windows::core::Vtable::as_raw(this),
                element.into().abi(),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::FontNumeralAlignment>(result__)
        })
    }
    pub fn SetNumeralAlignment<'a, P0>(
        element: P0,
        value: super::FontNumeralAlignment,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::DependencyObject>>,
    {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::core::Vtable::vtable(this).SetNumeralAlignment)(
                ::windows::core::Vtable::as_raw(this),
                element.into().abi(),
                value,
            )
            .ok()
        })
    }
    pub fn SlashedZeroProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SlashedZeroProperty)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn GetSlashedZero<'a, P0>(element: P0) -> ::windows::core::Result<bool>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::DependencyObject>>,
    {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetSlashedZero)(
                ::windows::core::Vtable::as_raw(this),
                element.into().abi(),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        })
    }
    pub fn SetSlashedZero<'a, P0>(element: P0, value: bool) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::DependencyObject>>,
    {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::core::Vtable::vtable(this).SetSlashedZero)(
                ::windows::core::Vtable::as_raw(this),
                element.into().abi(),
                value,
            )
            .ok()
        })
    }
    pub fn MathematicalGreekProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MathematicalGreekProperty)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn GetMathematicalGreek<'a, P0>(element: P0) -> ::windows::core::Result<bool>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::DependencyObject>>,
    {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetMathematicalGreek)(
                ::windows::core::Vtable::as_raw(this),
                element.into().abi(),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        })
    }
    pub fn SetMathematicalGreek<'a, P0>(element: P0, value: bool) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::DependencyObject>>,
    {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::core::Vtable::vtable(this).SetMathematicalGreek)(
                ::windows::core::Vtable::as_raw(this),
                element.into().abi(),
                value,
            )
            .ok()
        })
    }
    pub fn VariantsProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).VariantsProperty)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn GetVariants<'a, P0>(element: P0) -> ::windows::core::Result<super::FontVariants>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::DependencyObject>>,
    {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetVariants)(
                ::windows::core::Vtable::as_raw(this),
                element.into().abi(),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::FontVariants>(result__)
        })
    }
    pub fn SetVariants<'a, P0>(
        element: P0,
        value: super::FontVariants,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::DependencyObject>>,
    {
        Self::ITypographyStatics(|this| unsafe {
            (::windows::core::Vtable::vtable(this).SetVariants)(
                ::windows::core::Vtable::as_raw(this),
                element.into().abi(),
                value,
            )
            .ok()
        })
    }
    #[doc(hidden)]
    pub fn ITypographyStatics<R, F: FnOnce(&ITypographyStatics) -> ::windows::core::Result<R>>(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<Typography, ITypographyStatics> =
            ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for Typography {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for Typography {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Typography {}
impl ::core::fmt::Debug for Typography {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Typography").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for Typography {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Xaml.Documents.Typography;{fa27e2e3-be5e-5d21-9a5e-90cf102af828})",
    );
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for Typography {
    type Vtable = ITypography_Vtbl;
}
unsafe impl ::windows::core::Interface for Typography {
    const IID: ::windows::core::GUID = <ITypography as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for Typography {
    const NAME: &'static str = "Microsoft.UI.Xaml.Documents.Typography";
}
::windows::core::interface_hierarchy!(
    Typography,
    ::windows::core::IUnknown,
    ::windows::core::IInspectable
);
unsafe impl ::core::marker::Send for Typography {}
unsafe impl ::core::marker::Sync for Typography {}
#[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
#[repr(transparent)]
pub struct Underline(::windows::core::IUnknown);
impl Underline {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<Underline, ::windows::core::IGenericFactory> =
            ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn GetValue(
        &self,
        dp: &super::DependencyProperty,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetValue)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(dp),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn SetValue<'a, P0>(
        &self,
        dp: &super::DependencyProperty,
        value: P0,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IInspectable>>,
    {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetValue)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(dp),
                value.into().abi(),
            )
            .ok()
        }
    }
    pub fn ClearValue(&self, dp: &super::DependencyProperty) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).ClearValue)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(dp),
            )
            .ok()
        }
    }
    pub fn ReadLocalValue(
        &self,
        dp: &super::DependencyProperty,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ReadLocalValue)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(dp),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn GetAnimationBaseValue(
        &self,
        dp: &super::DependencyProperty,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetAnimationBaseValue)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(dp),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn RegisterPropertyChangedCallback(
        &self,
        dp: &super::DependencyProperty,
        callback: &super::DependencyPropertyChangedCallback,
    ) -> ::windows::core::Result<i64> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RegisterPropertyChangedCallback)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(dp),
                ::core::mem::transmute_copy(callback),
                result__.as_mut_ptr(),
            )
            .from_abi::<i64>(result__)
        }
    }
    pub fn UnregisterPropertyChangedCallback(
        &self,
        dp: &super::DependencyProperty,
        token: i64,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).UnregisterPropertyChangedCallback)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(dp),
                token,
            )
            .ok()
        }
    }
    pub fn Dispatcher(&self) -> ::windows::core::Result<::windows::UI::Core::CoreDispatcher> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Dispatcher)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Core::CoreDispatcher>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Dispatching\"`*"]
    #[cfg(feature = "UI_Dispatching")]
    pub fn DispatcherQueue(
        &self,
    ) -> ::windows::core::Result<super::super::Dispatching::DispatcherQueue> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DispatcherQueue)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::super::Dispatching::DispatcherQueue>(result__)
        }
    }
    pub fn Inlines(&self) -> ::windows::core::Result<InlineCollection> {
        let this = &::windows::core::Interface::cast::<ISpan>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Inlines)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<InlineCollection>(result__)
        }
    }
    pub fn SetInlines(&self, value: &InlineCollection) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ISpan>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetInlines)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Name)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn FontSize(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FontSize)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetFontSize(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetFontSize)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn FontFamily(&self) -> ::windows::core::Result<super::Media::FontFamily> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FontFamily)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::Media::FontFamily>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetFontFamily<'a, P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::Media::FontFamily>>,
    {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetFontFamily)(
                ::windows::core::Vtable::as_raw(this),
                value.into().abi(),
            )
            .ok()
        }
    }
    pub fn FontWeight(&self) -> ::windows::core::Result<::windows::UI::Text::FontWeight> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FontWeight)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Text::FontWeight>(result__)
        }
    }
    pub fn SetFontWeight(
        &self,
        value: ::windows::UI::Text::FontWeight,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetFontWeight)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn FontStyle(&self) -> ::windows::core::Result<::windows::UI::Text::FontStyle> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FontStyle)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Text::FontStyle>(result__)
        }
    }
    pub fn SetFontStyle(
        &self,
        value: ::windows::UI::Text::FontStyle,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetFontStyle)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn FontStretch(&self) -> ::windows::core::Result<::windows::UI::Text::FontStretch> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FontStretch)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Text::FontStretch>(result__)
        }
    }
    pub fn SetFontStretch(
        &self,
        value: ::windows::UI::Text::FontStretch,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetFontStretch)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn CharacterSpacing(&self) -> ::windows::core::Result<i32> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CharacterSpacing)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<i32>(result__)
        }
    }
    pub fn SetCharacterSpacing(&self, value: i32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetCharacterSpacing)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn Foreground(&self) -> ::windows::core::Result<super::Media::Brush> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Foreground)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::Media::Brush>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Media\"`*"]
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetForeground<'a, P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::Media::Brush>>,
    {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetForeground)(
                ::windows::core::Vtable::as_raw(this),
                value.into().abi(),
            )
            .ok()
        }
    }
    pub fn Language(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
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
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetLanguage)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn IsTextScaleFactorEnabled(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsTextScaleFactorEnabled)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetIsTextScaleFactorEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetIsTextScaleFactorEnabled)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn TextDecorations(&self) -> ::windows::core::Result<::windows::UI::Text::TextDecorations> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TextDecorations)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Text::TextDecorations>(result__)
        }
    }
    pub fn SetTextDecorations(
        &self,
        value: ::windows::UI::Text::TextDecorations,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetTextDecorations)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn ContentStart(&self) -> ::windows::core::Result<TextPointer> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ContentStart)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    pub fn ContentEnd(&self) -> ::windows::core::Result<TextPointer> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ContentEnd)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    pub fn ElementStart(&self) -> ::windows::core::Result<TextPointer> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ElementStart)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    pub fn ElementEnd(&self) -> ::windows::core::Result<TextPointer> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ElementEnd)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<TextPointer>(result__)
        }
    }
    pub fn AllowFocusOnInteraction(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AllowFocusOnInteraction)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetAllowFocusOnInteraction(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetAllowFocusOnInteraction)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn AccessKey(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AccessKey)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetAccessKey(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetAccessKey)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn ExitDisplayModeOnAccessKeyInvoked(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ExitDisplayModeOnAccessKeyInvoked)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetExitDisplayModeOnAccessKeyInvoked(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetExitDisplayModeOnAccessKeyInvoked)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsAccessKeyScope(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsAccessKeyScope)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetIsAccessKeyScope(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetIsAccessKeyScope)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn AccessKeyScopeOwner(&self) -> ::windows::core::Result<super::DependencyObject> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AccessKeyScopeOwner)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::DependencyObject>(result__)
        }
    }
    pub fn SetAccessKeyScopeOwner<'a, P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::DependencyObject>>,
    {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetAccessKeyScopeOwner)(
                ::windows::core::Vtable::as_raw(this),
                value.into().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn KeyTipPlacementMode(
        &self,
    ) -> ::windows::core::Result<super::Input::KeyTipPlacementMode> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).KeyTipPlacementMode)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::Input::KeyTipPlacementMode>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetKeyTipPlacementMode(
        &self,
        value: super::Input::KeyTipPlacementMode,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetKeyTipPlacementMode)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn KeyTipHorizontalOffset(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).KeyTipHorizontalOffset)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetKeyTipHorizontalOffset(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetKeyTipHorizontalOffset)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn KeyTipVerticalOffset(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).KeyTipVerticalOffset)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetKeyTipVerticalOffset(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetKeyTipVerticalOffset)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn XamlRoot(&self) -> ::windows::core::Result<super::XamlRoot> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).XamlRoot)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::XamlRoot>(result__)
        }
    }
    pub fn SetXamlRoot(&self, value: &super::XamlRoot) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetXamlRoot)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn AccessKeyDisplayRequested(
        &self,
        handler: &::windows::Foundation::TypedEventHandler<
            TextElement,
            super::Input::AccessKeyDisplayRequestedEventArgs,
        >,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AccessKeyDisplayRequested)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(handler),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveAccessKeyDisplayRequested(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).RemoveAccessKeyDisplayRequested)(
                ::windows::core::Vtable::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn AccessKeyDisplayDismissed(
        &self,
        handler: &::windows::Foundation::TypedEventHandler<
            TextElement,
            super::Input::AccessKeyDisplayDismissedEventArgs,
        >,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AccessKeyDisplayDismissed)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(handler),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveAccessKeyDisplayDismissed(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).RemoveAccessKeyDisplayDismissed)(
                ::windows::core::Vtable::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Input\"`*"]
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn AccessKeyInvoked(
        &self,
        handler: &::windows::Foundation::TypedEventHandler<
            TextElement,
            super::Input::AccessKeyInvokedEventArgs,
        >,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AccessKeyInvoked)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(handler),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveAccessKeyInvoked(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).RemoveAccessKeyInvoked)(
                ::windows::core::Vtable::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn FindName(
        &self,
        name: &::windows::core::HSTRING,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FindName)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(name),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
}
impl ::core::clone::Clone for Underline {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for Underline {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Underline {}
impl ::core::fmt::Debug for Underline {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Underline").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for Underline {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Xaml.Documents.Underline;{68aaec6e-ea71-5ed2-b83e-91684b25efb9})",
    );
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for Underline {
    type Vtable = IUnderline_Vtbl;
}
unsafe impl ::windows::core::Interface for Underline {
    const IID: ::windows::core::GUID = <IUnderline as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for Underline {
    const NAME: &'static str = "Microsoft.UI.Xaml.Documents.Underline";
}
::windows::core::interface_hierarchy!(
    Underline,
    ::windows::core::IUnknown,
    ::windows::core::IInspectable
);
impl ::core::convert::From<Underline> for Span {
    fn from(value: Underline) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&Underline> for Span {
    fn from(value: &Underline) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::core::convert::From<&Underline> for ::windows::core::InParam<'a, Span> {
    fn from(value: &Underline) -> Self {
        ::windows::core::InParam::owned(value.into())
    }
}
impl ::core::convert::From<Underline> for Inline {
    fn from(value: Underline) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&Underline> for Inline {
    fn from(value: &Underline) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::core::convert::From<&Underline> for ::windows::core::InParam<'a, Inline> {
    fn from(value: &Underline) -> Self {
        ::windows::core::InParam::owned(value.into())
    }
}
impl ::core::convert::From<Underline> for TextElement {
    fn from(value: Underline) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&Underline> for TextElement {
    fn from(value: &Underline) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::core::convert::From<&Underline> for ::windows::core::InParam<'a, TextElement> {
    fn from(value: &Underline) -> Self {
        ::windows::core::InParam::owned(value.into())
    }
}
impl ::core::convert::From<Underline> for super::DependencyObject {
    fn from(value: Underline) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&Underline> for super::DependencyObject {
    fn from(value: &Underline) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::core::convert::From<&Underline>
    for ::windows::core::InParam<'a, super::DependencyObject>
{
    fn from(value: &Underline) -> Self {
        ::windows::core::InParam::owned(value.into())
    }
}
unsafe impl ::core::marker::Send for Underline {}
unsafe impl ::core::marker::Sync for Underline {}
#[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct LogicalDirection(pub i32);
impl LogicalDirection {
    pub const Backward: Self = Self(0i32);
    pub const Forward: Self = Self(1i32);
}
impl ::core::marker::Copy for LogicalDirection {}
impl ::core::clone::Clone for LogicalDirection {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for LogicalDirection {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for LogicalDirection {
    type Abi = Self;
}
impl ::core::fmt::Debug for LogicalDirection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LogicalDirection").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for LogicalDirection {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Xaml.Documents.LogicalDirection;i4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct UnderlineStyle(pub i32);
impl UnderlineStyle {
    pub const None: Self = Self(0i32);
    pub const Single: Self = Self(1i32);
}
impl ::core::marker::Copy for UnderlineStyle {}
impl ::core::clone::Clone for UnderlineStyle {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UnderlineStyle {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for UnderlineStyle {
    type Abi = Self;
}
impl ::core::fmt::Debug for UnderlineStyle {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UnderlineStyle").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UnderlineStyle {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Xaml.Documents.UnderlineStyle;i4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[repr(C)]
#[doc = "*Required features: `\"UI_Xaml_Documents\"`*"]
pub struct TextRange {
    pub StartIndex: i32,
    pub Length: i32,
}
impl ::core::marker::Copy for TextRange {}
impl ::core::clone::Clone for TextRange {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TextRange {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TextRange")
            .field("StartIndex", &self.StartIndex)
            .field("Length", &self.Length)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for TextRange {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for TextRange {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"struct(Microsoft.UI.Xaml.Documents.TextRange;i4;i4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
impl ::core::cmp::PartialEq for TextRange {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            ::windows::core::memcmp(
                self as *const _ as _,
                other as *const _ as _,
                core::mem::size_of::<TextRange>(),
            ) == 0
        }
    }
}
impl ::core::cmp::Eq for TextRange {}
impl ::core::default::Default for TextRange {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
