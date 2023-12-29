#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IBlock(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBlock {
    type Vtable = IBlock_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IBlock {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x8149d507_672f_5fd5_a10a_351389ba9659);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBlock_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub TextAlignment: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::TextAlignment,
    ) -> ::windows_core::HRESULT,
    pub SetTextAlignment: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: super::TextAlignment,
    ) -> ::windows_core::HRESULT,
    pub HorizontalTextAlignment: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::TextAlignment,
    ) -> ::windows_core::HRESULT,
    pub SetHorizontalTextAlignment: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: super::TextAlignment,
    ) -> ::windows_core::HRESULT,
    pub LineHeight: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub SetLineHeight: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
    ) -> ::windows_core::HRESULT,
    pub LineStackingStrategy: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::LineStackingStrategy,
    ) -> ::windows_core::HRESULT,
    pub SetLineStackingStrategy: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: super::LineStackingStrategy,
    ) -> ::windows_core::HRESULT,
    pub Margin: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::Thickness,
    ) -> ::windows_core::HRESULT,
    pub SetMargin: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: super::Thickness,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IBlockFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBlockFactory {
    type Vtable = IBlockFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IBlockFactory {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x21bd671c_33e2_56ef_be37_a128e898452c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBlockFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        baseinterface: *mut ::core::ffi::c_void,
        innerinterface: *mut *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IBlockStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBlockStatics {
    type Vtable = IBlockStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IBlockStatics {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x830feedf_9aa6_56cd_983e_055500171b45);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBlockStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub TextAlignmentProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub HorizontalTextAlignmentProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub LineHeightProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub LineStackingStrategyProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub MarginProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IBold(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBold {
    type Vtable = IBold_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IBold {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x241a5f5a_c164_597f_b0db_fac7431297f2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBold_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IGlyphs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGlyphs {
    type Vtable = IGlyphs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IGlyphs {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x0fbf8cfe_18e7_5e45_9fa3_d2d0927958f4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGlyphs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub UnicodeString: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub SetUnicodeString: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub Indices: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub SetIndices: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_Foundation")]
    pub FontUri: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    FontUri: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub SetFontUri: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    SetFontUri: usize,
    #[cfg(feature = "Microsoft_UI_Xaml_Media")]
    pub StyleSimulations: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::Media::StyleSimulations,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Microsoft_UI_Xaml_Media"))]
    StyleSimulations: usize,
    #[cfg(feature = "Microsoft_UI_Xaml_Media")]
    pub SetStyleSimulations: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: super::Media::StyleSimulations,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Microsoft_UI_Xaml_Media"))]
    SetStyleSimulations: usize,
    pub FontRenderingEmSize: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub SetFontRenderingEmSize: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
    ) -> ::windows_core::HRESULT,
    pub OriginX: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub SetOriginX: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
    ) -> ::windows_core::HRESULT,
    pub OriginY: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub SetOriginY: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Microsoft_UI_Xaml_Media")]
    pub Fill: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Microsoft_UI_Xaml_Media"))]
    Fill: usize,
    #[cfg(feature = "Microsoft_UI_Xaml_Media")]
    pub SetFill: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Microsoft_UI_Xaml_Media"))]
    SetFill: usize,
    pub IsColorFontEnabled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetIsColorFontEnabled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
    pub ColorFontPaletteIndex: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut i32,
    ) -> ::windows_core::HRESULT,
    pub SetColorFontPaletteIndex: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: i32,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IGlyphsStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGlyphsStatics {
    type Vtable = IGlyphsStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IGlyphsStatics {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x8d9e241a_3e0e_5100_8ede_e008034ff8ae);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGlyphsStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub UnicodeStringProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub IndicesProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub FontUriProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub StyleSimulationsProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub FontRenderingEmSizeProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub OriginXProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub OriginYProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub FillProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub IsColorFontEnabledProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub ColorFontPaletteIndexProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IHyperlink(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHyperlink {
    type Vtable = IHyperlink_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IHyperlink {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xac09bd16_cdfa_54c2_8d03_a474181545b1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHyperlink_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Windows_Foundation")]
    pub NavigateUri: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    NavigateUri: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub SetNavigateUri: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    SetNavigateUri: usize,
    pub UnderlineStyle: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut UnderlineStyle,
    ) -> ::windows_core::HRESULT,
    pub SetUnderlineStyle: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: UnderlineStyle,
    ) -> ::windows_core::HRESULT,
    pub XYFocusLeft: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetXYFocusLeft: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub XYFocusRight: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetXYFocusRight: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub XYFocusUp: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetXYFocusUp: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub XYFocusDown: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetXYFocusDown: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub ElementSoundMode: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::ElementSoundMode,
    ) -> ::windows_core::HRESULT,
    pub SetElementSoundMode: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: super::ElementSoundMode,
    ) -> ::windows_core::HRESULT,
    pub FocusState: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::FocusState,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Microsoft_UI_Xaml_Input")]
    pub XYFocusUpNavigationStrategy: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::Input::XYFocusNavigationStrategy,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Microsoft_UI_Xaml_Input"))]
    XYFocusUpNavigationStrategy: usize,
    #[cfg(feature = "Microsoft_UI_Xaml_Input")]
    pub SetXYFocusUpNavigationStrategy: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: super::Input::XYFocusNavigationStrategy,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Microsoft_UI_Xaml_Input"))]
    SetXYFocusUpNavigationStrategy: usize,
    #[cfg(feature = "Microsoft_UI_Xaml_Input")]
    pub XYFocusDownNavigationStrategy: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::Input::XYFocusNavigationStrategy,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Microsoft_UI_Xaml_Input"))]
    XYFocusDownNavigationStrategy: usize,
    #[cfg(feature = "Microsoft_UI_Xaml_Input")]
    pub SetXYFocusDownNavigationStrategy: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: super::Input::XYFocusNavigationStrategy,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Microsoft_UI_Xaml_Input"))]
    SetXYFocusDownNavigationStrategy: usize,
    #[cfg(feature = "Microsoft_UI_Xaml_Input")]
    pub XYFocusLeftNavigationStrategy: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::Input::XYFocusNavigationStrategy,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Microsoft_UI_Xaml_Input"))]
    XYFocusLeftNavigationStrategy: usize,
    #[cfg(feature = "Microsoft_UI_Xaml_Input")]
    pub SetXYFocusLeftNavigationStrategy: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: super::Input::XYFocusNavigationStrategy,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Microsoft_UI_Xaml_Input"))]
    SetXYFocusLeftNavigationStrategy: usize,
    #[cfg(feature = "Microsoft_UI_Xaml_Input")]
    pub XYFocusRightNavigationStrategy: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::Input::XYFocusNavigationStrategy,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Microsoft_UI_Xaml_Input"))]
    XYFocusRightNavigationStrategy: usize,
    #[cfg(feature = "Microsoft_UI_Xaml_Input")]
    pub SetXYFocusRightNavigationStrategy: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: super::Input::XYFocusNavigationStrategy,
    )
        -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Microsoft_UI_Xaml_Input"))]
    SetXYFocusRightNavigationStrategy: usize,
    pub IsTabStop: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetIsTabStop: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
    pub TabIndex: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut i32,
    ) -> ::windows_core::HRESULT,
    pub SetTabIndex: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: i32,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_Foundation")]
    pub Click: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    Click: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub RemoveClick: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RemoveClick: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub GotFocus: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    GotFocus: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub RemoveGotFocus: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RemoveGotFocus: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub LostFocus: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    LostFocus: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub RemoveLostFocus: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RemoveLostFocus: usize,
    pub Focus: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: super::FocusState,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IHyperlinkClickEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHyperlinkClickEventArgs {
    type Vtable = IHyperlinkClickEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IHyperlinkClickEventArgs {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xf8f89552_873d_5ef5_82bf_c79a9509b07c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHyperlinkClickEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IHyperlinkStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHyperlinkStatics {
    type Vtable = IHyperlinkStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IHyperlinkStatics {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xe13598f4_7bc7_5ab9_885b_70f32f8c9531);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHyperlinkStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub NavigateUriProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub UnderlineStyleProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub XYFocusLeftProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub XYFocusRightProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub XYFocusUpProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub XYFocusDownProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub ElementSoundModeProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub FocusStateProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub XYFocusUpNavigationStrategyProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    )
        -> ::windows_core::HRESULT,
    pub XYFocusDownNavigationStrategyProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    )
        -> ::windows_core::HRESULT,
    pub XYFocusLeftNavigationStrategyProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    )
        -> ::windows_core::HRESULT,
    pub XYFocusRightNavigationStrategyProperty:
        unsafe extern "system" fn(
            this: *mut ::core::ffi::c_void,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT,
    pub IsTabStopProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub TabIndexProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IInline(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IInline {
    type Vtable = IInline_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IInline {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x813d427a_8980_5a79_a8fa_f27919cfb24f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInline_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IInlineFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IInlineFactory {
    type Vtable = IInlineFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IInlineFactory {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xfd253a36_fa2b_5b30_89a8_9f577871ec07);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInlineFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        baseinterface: *mut ::core::ffi::c_void,
        innerinterface: *mut *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IInlineUIContainer(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IInlineUIContainer {
    type Vtable = IInlineUIContainer_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IInlineUIContainer {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xd529bef6_c05a_5bad_85e8_640127cf86f5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInlineUIContainer_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Child: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetChild: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IItalic(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IItalic {
    type Vtable = IItalic_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IItalic {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xca3cbebd_7a8d_5d7a_8fdf_538e8a680f6c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IItalic_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ILineBreak(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILineBreak {
    type Vtable = ILineBreak_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ILineBreak {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x09307599_7cc2_5f54_b106_728620c16f76);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILineBreak_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IParagraph(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IParagraph {
    type Vtable = IParagraph_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IParagraph {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x9ed64c77_329d_502f_a257_f58398edab51);
}
#[repr(C)]
#[doc(hidden)]
pub struct IParagraph_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub Inlines: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation_Collections"))]
    Inlines: usize,
    pub TextIndent: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub SetTextIndent: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IParagraphStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IParagraphStatics {
    type Vtable = IParagraphStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IParagraphStatics {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x4eb89ab1_66c8_5fc0_aa5f_48c8092ceb5f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IParagraphStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub TextIndentProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IRun(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRun {
    type Vtable = IRun_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IRun {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x1f905239_37cb_590b_9132_3ffb7741906e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRun_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Text: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub SetText: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub FlowDirection: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::FlowDirection,
    ) -> ::windows_core::HRESULT,
    pub SetFlowDirection: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: super::FlowDirection,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IRunStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRunStatics {
    type Vtable = IRunStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IRunStatics {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x051b3c5b_7600_51a5_80c5_93eb50fd684f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRunStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub FlowDirectionProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ISpan(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpan {
    type Vtable = ISpan_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ISpan {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x91b93d4d_4e28_57b9_bffb_3566c2a3c2a1);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpan_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub Inlines: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation_Collections"))]
    Inlines: usize,
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub SetInlines: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation_Collections"))]
    SetInlines: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ISpanFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpanFactory {
    type Vtable = ISpanFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ISpanFactory {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xa6e87c16_c175_55c8_bbd3_ce40f9d0a680);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpanFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        baseinterface: *mut ::core::ffi::c_void,
        innerinterface: *mut *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ITextElement(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITextElement {
    type Vtable = ITextElement_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ITextElement {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xa122ba22_833f_5220_a47e_6cd507531abe);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextElement_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Name: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub FontSize: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub SetFontSize: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Microsoft_UI_Xaml_Media")]
    pub FontFamily: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Microsoft_UI_Xaml_Media"))]
    FontFamily: usize,
    #[cfg(feature = "Microsoft_UI_Xaml_Media")]
    pub SetFontFamily: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Microsoft_UI_Xaml_Media"))]
    SetFontFamily: usize,
    #[cfg(feature = "Windows_UI_Text")]
    pub FontWeight: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Text::FontWeight,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI_Text"))]
    FontWeight: usize,
    #[cfg(feature = "Windows_UI_Text")]
    pub SetFontWeight: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::windows::UI::Text::FontWeight,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI_Text"))]
    SetFontWeight: usize,
    #[cfg(feature = "Windows_UI_Text")]
    pub FontStyle: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Text::FontStyle,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI_Text"))]
    FontStyle: usize,
    #[cfg(feature = "Windows_UI_Text")]
    pub SetFontStyle: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::windows::UI::Text::FontStyle,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI_Text"))]
    SetFontStyle: usize,
    #[cfg(feature = "Windows_UI_Text")]
    pub FontStretch: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Text::FontStretch,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI_Text"))]
    FontStretch: usize,
    #[cfg(feature = "Windows_UI_Text")]
    pub SetFontStretch: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::windows::UI::Text::FontStretch,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI_Text"))]
    SetFontStretch: usize,
    pub CharacterSpacing: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut i32,
    ) -> ::windows_core::HRESULT,
    pub SetCharacterSpacing: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: i32,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Microsoft_UI_Xaml_Media")]
    pub Foreground: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Microsoft_UI_Xaml_Media"))]
    Foreground: usize,
    #[cfg(feature = "Microsoft_UI_Xaml_Media")]
    pub SetForeground: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Microsoft_UI_Xaml_Media"))]
    SetForeground: usize,
    pub Language: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub SetLanguage: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub IsTextScaleFactorEnabled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetIsTextScaleFactorEnabled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_UI_Text")]
    pub TextDecorations: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Text::TextDecorations,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI_Text"))]
    TextDecorations: usize,
    #[cfg(feature = "Windows_UI_Text")]
    pub SetTextDecorations: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::windows::UI::Text::TextDecorations,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI_Text"))]
    SetTextDecorations: usize,
    pub ContentStart: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub ContentEnd: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub ElementStart: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub ElementEnd: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub AllowFocusOnInteraction: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetAllowFocusOnInteraction: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
    pub AccessKey: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub SetAccessKey: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub ExitDisplayModeOnAccessKeyInvoked: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    )
        -> ::windows_core::HRESULT,
    pub SetExitDisplayModeOnAccessKeyInvoked: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    )
        -> ::windows_core::HRESULT,
    pub IsAccessKeyScope: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetIsAccessKeyScope: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
    pub AccessKeyScopeOwner: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetAccessKeyScopeOwner: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Microsoft_UI_Xaml_Input")]
    pub KeyTipPlacementMode: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::Input::KeyTipPlacementMode,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Microsoft_UI_Xaml_Input"))]
    KeyTipPlacementMode: usize,
    #[cfg(feature = "Microsoft_UI_Xaml_Input")]
    pub SetKeyTipPlacementMode: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: super::Input::KeyTipPlacementMode,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Microsoft_UI_Xaml_Input"))]
    SetKeyTipPlacementMode: usize,
    pub KeyTipHorizontalOffset: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub SetKeyTipHorizontalOffset: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
    ) -> ::windows_core::HRESULT,
    pub KeyTipVerticalOffset: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub SetKeyTipVerticalOffset: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
    ) -> ::windows_core::HRESULT,
    pub XamlRoot: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetXamlRoot: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Microsoft_UI_Xaml_Input", feature = "Windows_Foundation"))]
    pub AccessKeyDisplayRequested: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Microsoft_UI_Xaml_Input", feature = "Windows_Foundation")))]
    AccessKeyDisplayRequested: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub RemoveAccessKeyDisplayRequested: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RemoveAccessKeyDisplayRequested: usize,
    #[cfg(all(feature = "Microsoft_UI_Xaml_Input", feature = "Windows_Foundation"))]
    pub AccessKeyDisplayDismissed: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Microsoft_UI_Xaml_Input", feature = "Windows_Foundation")))]
    AccessKeyDisplayDismissed: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub RemoveAccessKeyDisplayDismissed: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RemoveAccessKeyDisplayDismissed: usize,
    #[cfg(all(feature = "Microsoft_UI_Xaml_Input", feature = "Windows_Foundation"))]
    pub AccessKeyInvoked: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Microsoft_UI_Xaml_Input", feature = "Windows_Foundation")))]
    AccessKeyInvoked: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub RemoveAccessKeyInvoked: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RemoveAccessKeyInvoked: usize,
    pub FindName: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        name: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ITextElementFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITextElementFactory {
    type Vtable = ITextElementFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ITextElementFactory {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xdf51fb95_a5e6_5b16_8e88_9f7cbfa234b1);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextElementFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ITextElementOverrides(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITextElementOverrides {
    type Vtable = ITextElementOverrides_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ITextElementOverrides {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x41b01380_e49f_5fda_8c72_acc1ac1e91df);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextElementOverrides_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub OnDisconnectVisualChildren:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ITextElementStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITextElementStatics {
    type Vtable = ITextElementStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ITextElementStatics {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xc9b55919_e1fe_5acd_bac7_c9d7f413b35c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextElementStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub FontSizeProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub FontFamilyProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub FontWeightProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub FontStyleProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub FontStretchProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub CharacterSpacingProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub ForegroundProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub LanguageProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub IsTextScaleFactorEnabledProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub TextDecorationsProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub AllowFocusOnInteractionProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub AccessKeyProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub ExitDisplayModeOnAccessKeyInvokedProperty:
        unsafe extern "system" fn(
            this: *mut ::core::ffi::c_void,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT,
    pub IsAccessKeyScopeProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub AccessKeyScopeOwnerProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub KeyTipPlacementModeProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub KeyTipHorizontalOffsetProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub KeyTipVerticalOffsetProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ITextHighlighter(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITextHighlighter {
    type Vtable = ITextHighlighter_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ITextHighlighter {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xb756e861_1d2b_5f6f_81fd_c51a5bc068ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextHighlighter_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub Ranges: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation_Collections"))]
    Ranges: usize,
    #[cfg(feature = "Microsoft_UI_Xaml_Media")]
    pub Foreground: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Microsoft_UI_Xaml_Media"))]
    Foreground: usize,
    #[cfg(feature = "Microsoft_UI_Xaml_Media")]
    pub SetForeground: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Microsoft_UI_Xaml_Media"))]
    SetForeground: usize,
    #[cfg(feature = "Microsoft_UI_Xaml_Media")]
    pub Background: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Microsoft_UI_Xaml_Media"))]
    Background: usize,
    #[cfg(feature = "Microsoft_UI_Xaml_Media")]
    pub SetBackground: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Microsoft_UI_Xaml_Media"))]
    SetBackground: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ITextHighlighterBase(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITextHighlighterBase {
    type Vtable = ITextHighlighterBase_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ITextHighlighterBase {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x5c21aaf0_3a17_5468_8aac_be14db0ed8c1);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextHighlighterBase_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ITextHighlighterBaseFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITextHighlighterBaseFactory {
    type Vtable = ITextHighlighterBaseFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ITextHighlighterBaseFactory {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xe091e461_53ab_599e_aaea_800adc72da4f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextHighlighterBaseFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ITextHighlighterFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITextHighlighterFactory {
    type Vtable = ITextHighlighterFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ITextHighlighterFactory {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x69c7311f_c019_5b93_b511_81418543bab7);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextHighlighterFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        baseinterface: *mut ::core::ffi::c_void,
        innerinterface: *mut *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ITextHighlighterStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITextHighlighterStatics {
    type Vtable = ITextHighlighterStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ITextHighlighterStatics {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x4975047a_87ad_51a2_977c_e771de4f4035);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextHighlighterStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ForegroundProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub BackgroundProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ITextPointer(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITextPointer {
    type Vtable = ITextPointer_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ITextPointer {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x842eb385_ee41_5930_979b_438fa7525a51);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextPointer_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Parent: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub VisualParent: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub LogicalDirection: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut LogicalDirection,
    ) -> ::windows_core::HRESULT,
    pub Offset: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut i32,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_Foundation")]
    pub GetCharacterRect: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        direction: LogicalDirection,
        result__: *mut ::windows::Foundation::Rect,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    GetCharacterRect: usize,
    pub GetPositionAtOffset: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        offset: i32,
        direction: LogicalDirection,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ITypography(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITypography {
    type Vtable = ITypography_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ITypography {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xfa27e2e3_be5e_5d21_9a5e_90cf102af828);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITypography_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ITypographyStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITypographyStatics {
    type Vtable = ITypographyStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ITypographyStatics {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x55fe4535_2125_533a_ada8_27be2b9e1193);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITypographyStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub AnnotationAlternatesProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub GetAnnotationAlternates: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        result__: *mut i32,
    ) -> ::windows_core::HRESULT,
    pub SetAnnotationAlternates: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        value: i32,
    ) -> ::windows_core::HRESULT,
    pub EastAsianExpertFormsProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub GetEastAsianExpertForms: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetEastAsianExpertForms: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
    pub EastAsianLanguageProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub GetEastAsianLanguage: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        result__: *mut super::FontEastAsianLanguage,
    ) -> ::windows_core::HRESULT,
    pub SetEastAsianLanguage: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        value: super::FontEastAsianLanguage,
    ) -> ::windows_core::HRESULT,
    pub EastAsianWidthsProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub GetEastAsianWidths: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        result__: *mut super::FontEastAsianWidths,
    ) -> ::windows_core::HRESULT,
    pub SetEastAsianWidths: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        value: super::FontEastAsianWidths,
    ) -> ::windows_core::HRESULT,
    pub StandardLigaturesProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub GetStandardLigatures: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetStandardLigatures: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
    pub ContextualLigaturesProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub GetContextualLigatures: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetContextualLigatures: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
    pub DiscretionaryLigaturesProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub GetDiscretionaryLigatures: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetDiscretionaryLigatures: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
    pub HistoricalLigaturesProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub GetHistoricalLigatures: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetHistoricalLigatures: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
    pub StandardSwashesProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub GetStandardSwashes: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        result__: *mut i32,
    ) -> ::windows_core::HRESULT,
    pub SetStandardSwashes: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        value: i32,
    ) -> ::windows_core::HRESULT,
    pub ContextualSwashesProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub GetContextualSwashes: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        result__: *mut i32,
    ) -> ::windows_core::HRESULT,
    pub SetContextualSwashes: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        value: i32,
    ) -> ::windows_core::HRESULT,
    pub ContextualAlternatesProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub GetContextualAlternates: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetContextualAlternates: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
    pub StylisticAlternatesProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub GetStylisticAlternates: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        result__: *mut i32,
    ) -> ::windows_core::HRESULT,
    pub SetStylisticAlternates: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        value: i32,
    ) -> ::windows_core::HRESULT,
    pub StylisticSet1Property: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub GetStylisticSet1: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetStylisticSet1: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
    pub StylisticSet2Property: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub GetStylisticSet2: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetStylisticSet2: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
    pub StylisticSet3Property: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub GetStylisticSet3: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetStylisticSet3: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
    pub StylisticSet4Property: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub GetStylisticSet4: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetStylisticSet4: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
    pub StylisticSet5Property: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub GetStylisticSet5: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetStylisticSet5: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
    pub StylisticSet6Property: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub GetStylisticSet6: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetStylisticSet6: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
    pub StylisticSet7Property: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub GetStylisticSet7: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetStylisticSet7: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
    pub StylisticSet8Property: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub GetStylisticSet8: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetStylisticSet8: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
    pub StylisticSet9Property: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub GetStylisticSet9: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetStylisticSet9: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
    pub StylisticSet10Property: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub GetStylisticSet10: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetStylisticSet10: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
    pub StylisticSet11Property: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub GetStylisticSet11: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetStylisticSet11: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
    pub StylisticSet12Property: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub GetStylisticSet12: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetStylisticSet12: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
    pub StylisticSet13Property: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub GetStylisticSet13: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetStylisticSet13: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
    pub StylisticSet14Property: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub GetStylisticSet14: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetStylisticSet14: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
    pub StylisticSet15Property: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub GetStylisticSet15: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetStylisticSet15: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
    pub StylisticSet16Property: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub GetStylisticSet16: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetStylisticSet16: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
    pub StylisticSet17Property: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub GetStylisticSet17: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetStylisticSet17: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
    pub StylisticSet18Property: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub GetStylisticSet18: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetStylisticSet18: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
    pub StylisticSet19Property: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub GetStylisticSet19: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetStylisticSet19: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
    pub StylisticSet20Property: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub GetStylisticSet20: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetStylisticSet20: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
    pub CapitalsProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub GetCapitals: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        result__: *mut super::FontCapitals,
    ) -> ::windows_core::HRESULT,
    pub SetCapitals: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        value: super::FontCapitals,
    ) -> ::windows_core::HRESULT,
    pub CapitalSpacingProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub GetCapitalSpacing: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetCapitalSpacing: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
    pub KerningProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub GetKerning: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetKerning: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
    pub CaseSensitiveFormsProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub GetCaseSensitiveForms: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetCaseSensitiveForms: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
    pub HistoricalFormsProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub GetHistoricalForms: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetHistoricalForms: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
    pub FractionProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub GetFraction: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        result__: *mut super::FontFraction,
    ) -> ::windows_core::HRESULT,
    pub SetFraction: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        value: super::FontFraction,
    ) -> ::windows_core::HRESULT,
    pub NumeralStyleProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub GetNumeralStyle: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        result__: *mut super::FontNumeralStyle,
    ) -> ::windows_core::HRESULT,
    pub SetNumeralStyle: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        value: super::FontNumeralStyle,
    ) -> ::windows_core::HRESULT,
    pub NumeralAlignmentProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub GetNumeralAlignment: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        result__: *mut super::FontNumeralAlignment,
    ) -> ::windows_core::HRESULT,
    pub SetNumeralAlignment: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        value: super::FontNumeralAlignment,
    ) -> ::windows_core::HRESULT,
    pub SlashedZeroProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub GetSlashedZero: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetSlashedZero: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
    pub MathematicalGreekProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub GetMathematicalGreek: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetMathematicalGreek: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
    pub VariantsProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub GetVariants: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        result__: *mut super::FontVariants,
    ) -> ::windows_core::HRESULT,
    pub SetVariants: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        value: super::FontVariants,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IUnderline(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUnderline {
    type Vtable = IUnderline_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IUnderline {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x68aaec6e_ea71_5ed2_b83e_91684b25efb9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUnderline_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct Block(::windows_core::IUnknown);
impl Block {
    pub fn TextAlignment(&self) -> ::windows_core::Result<super::TextAlignment> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TextAlignment)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetTextAlignment(&self, value: super::TextAlignment) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetTextAlignment)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn HorizontalTextAlignment(&self) -> ::windows_core::Result<super::TextAlignment> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HorizontalTextAlignment)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetHorizontalTextAlignment(
        &self,
        value: super::TextAlignment,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetHorizontalTextAlignment)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn LineHeight(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LineHeight)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetLineHeight(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetLineHeight)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn LineStackingStrategy(&self) -> ::windows_core::Result<super::LineStackingStrategy> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LineStackingStrategy)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetLineStackingStrategy(
        &self,
        value: super::LineStackingStrategy,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetLineStackingStrategy)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn Margin(&self) -> ::windows_core::Result<super::Thickness> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Margin)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetMargin(&self, value: super::Thickness) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetMargin)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn TextAlignmentProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IBlockStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TextAlignmentProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn HorizontalTextAlignmentProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IBlockStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HorizontalTextAlignmentProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn LineHeightProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IBlockStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LineHeightProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn LineStackingStrategyProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IBlockStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LineStackingStrategyProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn MarginProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IBlockStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MarginProperty)(
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
    pub fn Name(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Name)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn FontSize(&self) -> ::windows_core::Result<f64> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FontSize)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetFontSize(&self, value: f64) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetFontSize)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Media\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Media")]
    pub fn FontFamily(&self) -> ::windows_core::Result<super::Media::FontFamily> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FontFamily)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Media\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Media")]
    pub fn SetFontFamily<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::Media::FontFamily>,
    {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetFontFamily)(
                ::windows_core::Interface::as_raw(this),
                value.try_into_param()?.abi(),
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_UI_Text\"`"]
    #[cfg(feature = "Windows_UI_Text")]
    pub fn FontWeight(&self) -> ::windows_core::Result<::windows::UI::Text::FontWeight> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FontWeight)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_UI_Text\"`"]
    #[cfg(feature = "Windows_UI_Text")]
    pub fn SetFontWeight(
        &self,
        value: ::windows::UI::Text::FontWeight,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetFontWeight)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_UI_Text\"`"]
    #[cfg(feature = "Windows_UI_Text")]
    pub fn FontStyle(&self) -> ::windows_core::Result<::windows::UI::Text::FontStyle> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FontStyle)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_UI_Text\"`"]
    #[cfg(feature = "Windows_UI_Text")]
    pub fn SetFontStyle(
        &self,
        value: ::windows::UI::Text::FontStyle,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetFontStyle)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_UI_Text\"`"]
    #[cfg(feature = "Windows_UI_Text")]
    pub fn FontStretch(&self) -> ::windows_core::Result<::windows::UI::Text::FontStretch> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FontStretch)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_UI_Text\"`"]
    #[cfg(feature = "Windows_UI_Text")]
    pub fn SetFontStretch(
        &self,
        value: ::windows::UI::Text::FontStretch,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetFontStretch)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn CharacterSpacing(&self) -> ::windows_core::Result<i32> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CharacterSpacing)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetCharacterSpacing(&self, value: i32) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetCharacterSpacing)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Media\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Media")]
    pub fn Foreground(&self) -> ::windows_core::Result<super::Media::Brush> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Foreground)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Media\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Media")]
    pub fn SetForeground<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::Media::Brush>,
    {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetForeground)(
                ::windows_core::Interface::as_raw(this),
                value.try_into_param()?.abi(),
            )
            .ok()
        }
    }
    pub fn Language(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
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
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetLanguage)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn IsTextScaleFactorEnabled(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsTextScaleFactorEnabled)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetIsTextScaleFactorEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetIsTextScaleFactorEnabled)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_UI_Text\"`"]
    #[cfg(feature = "Windows_UI_Text")]
    pub fn TextDecorations(&self) -> ::windows_core::Result<::windows::UI::Text::TextDecorations> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TextDecorations)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_UI_Text\"`"]
    #[cfg(feature = "Windows_UI_Text")]
    pub fn SetTextDecorations(
        &self,
        value: ::windows::UI::Text::TextDecorations,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetTextDecorations)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn ContentStart(&self) -> ::windows_core::Result<TextPointer> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ContentStart)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn ContentEnd(&self) -> ::windows_core::Result<TextPointer> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ContentEnd)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn ElementStart(&self) -> ::windows_core::Result<TextPointer> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ElementStart)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn ElementEnd(&self) -> ::windows_core::Result<TextPointer> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ElementEnd)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn AllowFocusOnInteraction(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AllowFocusOnInteraction)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetAllowFocusOnInteraction(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetAllowFocusOnInteraction)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn AccessKey(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AccessKey)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetAccessKey(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetAccessKey)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn ExitDisplayModeOnAccessKeyInvoked(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ExitDisplayModeOnAccessKeyInvoked)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetExitDisplayModeOnAccessKeyInvoked(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetExitDisplayModeOnAccessKeyInvoked)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsAccessKeyScope(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsAccessKeyScope)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetIsAccessKeyScope(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetIsAccessKeyScope)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn AccessKeyScopeOwner(&self) -> ::windows_core::Result<super::DependencyObject> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AccessKeyScopeOwner)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetAccessKeyScopeOwner<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetAccessKeyScopeOwner)(
                ::windows_core::Interface::as_raw(this),
                value.try_into_param()?.abi(),
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Input\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Input")]
    pub fn KeyTipPlacementMode(&self) -> ::windows_core::Result<super::Input::KeyTipPlacementMode> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).KeyTipPlacementMode)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Input\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Input")]
    pub fn SetKeyTipPlacementMode(
        &self,
        value: super::Input::KeyTipPlacementMode,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetKeyTipPlacementMode)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn KeyTipHorizontalOffset(&self) -> ::windows_core::Result<f64> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).KeyTipHorizontalOffset)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetKeyTipHorizontalOffset(&self, value: f64) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetKeyTipHorizontalOffset)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn KeyTipVerticalOffset(&self) -> ::windows_core::Result<f64> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).KeyTipVerticalOffset)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetKeyTipVerticalOffset(&self, value: f64) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetKeyTipVerticalOffset)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn XamlRoot(&self) -> ::windows_core::Result<super::XamlRoot> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).XamlRoot)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetXamlRoot<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::XamlRoot>,
    {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetXamlRoot)(
                ::windows_core::Interface::as_raw(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Input\"`, `\"Windows_Foundation\"`"]
    #[cfg(all(feature = "Microsoft_UI_Xaml_Input", feature = "Windows_Foundation"))]
    pub fn AccessKeyDisplayRequested<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<
                TextElement,
                super::Input::AccessKeyDisplayRequestedEventArgs,
            >,
        >,
    {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AccessKeyDisplayRequested)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveAccessKeyDisplayRequested(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveAccessKeyDisplayRequested)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Input\"`, `\"Windows_Foundation\"`"]
    #[cfg(all(feature = "Microsoft_UI_Xaml_Input", feature = "Windows_Foundation"))]
    pub fn AccessKeyDisplayDismissed<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<
                TextElement,
                super::Input::AccessKeyDisplayDismissedEventArgs,
            >,
        >,
    {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AccessKeyDisplayDismissed)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveAccessKeyDisplayDismissed(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveAccessKeyDisplayDismissed)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Input\"`, `\"Windows_Foundation\"`"]
    #[cfg(all(feature = "Microsoft_UI_Xaml_Input", feature = "Windows_Foundation"))]
    pub fn AccessKeyInvoked<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<
                TextElement,
                super::Input::AccessKeyInvokedEventArgs,
            >,
        >,
    {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AccessKeyInvoked)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveAccessKeyInvoked(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveAccessKeyInvoked)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn FindName(
        &self,
        name: &::windows_core::HSTRING,
    ) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FindName)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(name),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn OnDisconnectVisualChildren(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElementOverrides>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).OnDisconnectVisualChildren)(
                ::windows_core::Interface::as_raw(this),
            )
            .ok()
        }
    }
    #[doc(hidden)]
    pub fn IBlockStatics<R, F: FnOnce(&IBlockStatics) -> ::windows_core::Result<R>>(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<Block, IBlockStatics> =
            ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for Block {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for Block {
    type Vtable = IBlock_Vtbl;
}
unsafe impl ::windows_core::ComInterface for Block {
    const IID: ::windows_core::GUID = <IBlock as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for Block {
    const NAME: &'static str = "Microsoft.UI.Xaml.Documents.Block";
}
::windows_core::imp::interface_hierarchy!(
    Block,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<TextElement> for Block {}
impl ::windows_core::CanTryInto<super::DependencyObject> for Block {}
unsafe impl ::core::marker::Send for Block {}
unsafe impl ::core::marker::Sync for Block {}
#[doc = "Required features: `\"Windows_Foundation_Collections\"`"]
#[cfg(feature = "Windows_Foundation_Collections")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct BlockCollection(::windows_core::IUnknown);
#[cfg(feature = "Windows_Foundation_Collections")]
impl BlockCollection {
    #[doc = "Required features: `\"Windows_Foundation_Collections\"`"]
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub fn First(
        &self,
    ) -> ::windows_core::Result<::windows::Foundation::Collections::IIterator<Block>> {
        let this = &::windows_core::ComInterface::cast::<
            ::windows::Foundation::Collections::IIterable<Block>,
        >(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).First)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation_Collections\"`"]
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub fn GetAt(&self, index: u32) -> ::windows_core::Result<Block> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetAt)(
                ::windows_core::Interface::as_raw(this),
                index,
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation_Collections\"`"]
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub fn Size(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Size)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation_Collections\"`"]
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub fn GetView(
        &self,
    ) -> ::windows_core::Result<::windows::Foundation::Collections::IVectorView<Block>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetView)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation_Collections\"`"]
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub fn IndexOf<P0>(&self, value: P0, index: &mut u32) -> ::windows_core::Result<bool>
    where
        P0: ::windows_core::TryIntoParam<Block>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IndexOf)(
                ::windows_core::Interface::as_raw(this),
                value.try_into_param()?.abi(),
                index,
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation_Collections\"`"]
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub fn SetAt<P0>(&self, index: u32, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<Block>,
    {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetAt)(
                ::windows_core::Interface::as_raw(this),
                index,
                value.try_into_param()?.abi(),
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation_Collections\"`"]
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub fn InsertAt<P0>(&self, index: u32, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<Block>,
    {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).InsertAt)(
                ::windows_core::Interface::as_raw(this),
                index,
                value.try_into_param()?.abi(),
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation_Collections\"`"]
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub fn RemoveAt(&self, index: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveAt)(
                ::windows_core::Interface::as_raw(this),
                index,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation_Collections\"`"]
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub fn Append<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<Block>,
    {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).Append)(
                ::windows_core::Interface::as_raw(this),
                value.try_into_param()?.abi(),
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation_Collections\"`"]
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub fn RemoveAtEnd(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveAtEnd)(
                ::windows_core::Interface::as_raw(this),
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation_Collections\"`"]
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub fn Clear(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).Clear)(::windows_core::Interface::as_raw(this))
                .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation_Collections\"`"]
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub fn GetMany(
        &self,
        startindex: u32,
        items: &mut [::core::option::Option<Block>],
    ) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetMany)(
                ::windows_core::Interface::as_raw(this),
                startindex,
                items.len().try_into().unwrap(),
                ::core::mem::transmute_copy(&items),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation_Collections\"`"]
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub fn ReplaceAll(
        &self,
        items: &[::core::option::Option<Block>],
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).ReplaceAll)(
                ::windows_core::Interface::as_raw(this),
                items.len().try_into().unwrap(),
                ::core::mem::transmute(items.as_ptr()),
            )
            .ok()
        }
    }
}
#[cfg(feature = "Windows_Foundation_Collections")]
impl ::windows_core::RuntimeType for BlockCollection {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
#[cfg(feature = "Windows_Foundation_Collections")]
unsafe impl ::windows_core::Interface for BlockCollection {
    type Vtable = ::windows::Foundation::Collections::IVector_Vtbl<Block>;
}
#[cfg(feature = "Windows_Foundation_Collections")]
unsafe impl ::windows_core::ComInterface for BlockCollection {
    const IID: ::windows_core::GUID =
        <::windows::Foundation::Collections::IVector<Block> as ::windows_core::ComInterface>::IID;
}
#[cfg(feature = "Windows_Foundation_Collections")]
impl ::windows_core::RuntimeName for BlockCollection {
    const NAME: &'static str = "Microsoft.UI.Xaml.Documents.BlockCollection";
}
#[cfg(feature = "Windows_Foundation_Collections")]
impl ::core::iter::IntoIterator for BlockCollection {
    type Item = Block;
    type IntoIter = ::windows::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Windows_Foundation_Collections")]
impl ::core::iter::IntoIterator for &BlockCollection {
    type Item = Block;
    type IntoIter = ::windows::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::windows::Foundation::Collections::VectorIterator::new(
            ::windows_core::ComInterface::cast(self).ok(),
        )
    }
}
#[cfg(feature = "Windows_Foundation_Collections")]
::windows_core::imp::interface_hierarchy!(
    BlockCollection,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
#[cfg(feature = "Windows_Foundation_Collections")]
impl ::windows_core::CanTryInto<::windows::Foundation::Collections::IIterable<Block>>
    for BlockCollection
{
}
#[cfg(feature = "Windows_Foundation_Collections")]
impl ::windows_core::CanTryInto<::windows::Foundation::Collections::IVector<Block>>
    for BlockCollection
{
}
#[cfg(feature = "Windows_Foundation_Collections")]
unsafe impl ::core::marker::Send for BlockCollection {}
#[cfg(feature = "Windows_Foundation_Collections")]
unsafe impl ::core::marker::Sync for BlockCollection {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct Bold(::windows_core::IUnknown);
impl Bold {
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
            Bold,
            ::windows_core::imp::IGenericFactory,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
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
    #[doc = "Required features: `\"Windows_Foundation_Collections\"`"]
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub fn Inlines(&self) -> ::windows_core::Result<InlineCollection> {
        let this = &::windows_core::ComInterface::cast::<ISpan>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Inlines)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation_Collections\"`"]
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub fn SetInlines<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<InlineCollection>,
    {
        let this = &::windows_core::ComInterface::cast::<ISpan>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetInlines)(
                ::windows_core::Interface::as_raw(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn Name(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Name)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn FontSize(&self) -> ::windows_core::Result<f64> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FontSize)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetFontSize(&self, value: f64) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetFontSize)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Media\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Media")]
    pub fn FontFamily(&self) -> ::windows_core::Result<super::Media::FontFamily> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FontFamily)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Media\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Media")]
    pub fn SetFontFamily<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::Media::FontFamily>,
    {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetFontFamily)(
                ::windows_core::Interface::as_raw(this),
                value.try_into_param()?.abi(),
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_UI_Text\"`"]
    #[cfg(feature = "Windows_UI_Text")]
    pub fn FontWeight(&self) -> ::windows_core::Result<::windows::UI::Text::FontWeight> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FontWeight)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_UI_Text\"`"]
    #[cfg(feature = "Windows_UI_Text")]
    pub fn SetFontWeight(
        &self,
        value: ::windows::UI::Text::FontWeight,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetFontWeight)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_UI_Text\"`"]
    #[cfg(feature = "Windows_UI_Text")]
    pub fn FontStyle(&self) -> ::windows_core::Result<::windows::UI::Text::FontStyle> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FontStyle)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_UI_Text\"`"]
    #[cfg(feature = "Windows_UI_Text")]
    pub fn SetFontStyle(
        &self,
        value: ::windows::UI::Text::FontStyle,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetFontStyle)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_UI_Text\"`"]
    #[cfg(feature = "Windows_UI_Text")]
    pub fn FontStretch(&self) -> ::windows_core::Result<::windows::UI::Text::FontStretch> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FontStretch)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_UI_Text\"`"]
    #[cfg(feature = "Windows_UI_Text")]
    pub fn SetFontStretch(
        &self,
        value: ::windows::UI::Text::FontStretch,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetFontStretch)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn CharacterSpacing(&self) -> ::windows_core::Result<i32> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CharacterSpacing)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetCharacterSpacing(&self, value: i32) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetCharacterSpacing)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Media\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Media")]
    pub fn Foreground(&self) -> ::windows_core::Result<super::Media::Brush> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Foreground)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Media\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Media")]
    pub fn SetForeground<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::Media::Brush>,
    {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetForeground)(
                ::windows_core::Interface::as_raw(this),
                value.try_into_param()?.abi(),
            )
            .ok()
        }
    }
    pub fn Language(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
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
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetLanguage)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn IsTextScaleFactorEnabled(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsTextScaleFactorEnabled)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetIsTextScaleFactorEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetIsTextScaleFactorEnabled)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_UI_Text\"`"]
    #[cfg(feature = "Windows_UI_Text")]
    pub fn TextDecorations(&self) -> ::windows_core::Result<::windows::UI::Text::TextDecorations> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TextDecorations)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_UI_Text\"`"]
    #[cfg(feature = "Windows_UI_Text")]
    pub fn SetTextDecorations(
        &self,
        value: ::windows::UI::Text::TextDecorations,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetTextDecorations)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn ContentStart(&self) -> ::windows_core::Result<TextPointer> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ContentStart)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn ContentEnd(&self) -> ::windows_core::Result<TextPointer> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ContentEnd)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn ElementStart(&self) -> ::windows_core::Result<TextPointer> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ElementStart)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn ElementEnd(&self) -> ::windows_core::Result<TextPointer> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ElementEnd)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn AllowFocusOnInteraction(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AllowFocusOnInteraction)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetAllowFocusOnInteraction(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetAllowFocusOnInteraction)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn AccessKey(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AccessKey)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetAccessKey(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetAccessKey)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn ExitDisplayModeOnAccessKeyInvoked(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ExitDisplayModeOnAccessKeyInvoked)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetExitDisplayModeOnAccessKeyInvoked(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetExitDisplayModeOnAccessKeyInvoked)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsAccessKeyScope(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsAccessKeyScope)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetIsAccessKeyScope(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetIsAccessKeyScope)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn AccessKeyScopeOwner(&self) -> ::windows_core::Result<super::DependencyObject> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AccessKeyScopeOwner)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetAccessKeyScopeOwner<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetAccessKeyScopeOwner)(
                ::windows_core::Interface::as_raw(this),
                value.try_into_param()?.abi(),
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Input\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Input")]
    pub fn KeyTipPlacementMode(&self) -> ::windows_core::Result<super::Input::KeyTipPlacementMode> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).KeyTipPlacementMode)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Input\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Input")]
    pub fn SetKeyTipPlacementMode(
        &self,
        value: super::Input::KeyTipPlacementMode,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetKeyTipPlacementMode)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn KeyTipHorizontalOffset(&self) -> ::windows_core::Result<f64> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).KeyTipHorizontalOffset)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetKeyTipHorizontalOffset(&self, value: f64) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetKeyTipHorizontalOffset)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn KeyTipVerticalOffset(&self) -> ::windows_core::Result<f64> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).KeyTipVerticalOffset)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetKeyTipVerticalOffset(&self, value: f64) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetKeyTipVerticalOffset)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn XamlRoot(&self) -> ::windows_core::Result<super::XamlRoot> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).XamlRoot)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetXamlRoot<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::XamlRoot>,
    {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetXamlRoot)(
                ::windows_core::Interface::as_raw(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Input\"`, `\"Windows_Foundation\"`"]
    #[cfg(all(feature = "Microsoft_UI_Xaml_Input", feature = "Windows_Foundation"))]
    pub fn AccessKeyDisplayRequested<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<
                TextElement,
                super::Input::AccessKeyDisplayRequestedEventArgs,
            >,
        >,
    {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AccessKeyDisplayRequested)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveAccessKeyDisplayRequested(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveAccessKeyDisplayRequested)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Input\"`, `\"Windows_Foundation\"`"]
    #[cfg(all(feature = "Microsoft_UI_Xaml_Input", feature = "Windows_Foundation"))]
    pub fn AccessKeyDisplayDismissed<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<
                TextElement,
                super::Input::AccessKeyDisplayDismissedEventArgs,
            >,
        >,
    {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AccessKeyDisplayDismissed)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveAccessKeyDisplayDismissed(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveAccessKeyDisplayDismissed)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Input\"`, `\"Windows_Foundation\"`"]
    #[cfg(all(feature = "Microsoft_UI_Xaml_Input", feature = "Windows_Foundation"))]
    pub fn AccessKeyInvoked<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<
                TextElement,
                super::Input::AccessKeyInvokedEventArgs,
            >,
        >,
    {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AccessKeyInvoked)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveAccessKeyInvoked(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveAccessKeyInvoked)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn FindName(
        &self,
        name: &::windows_core::HSTRING,
    ) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FindName)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(name),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn OnDisconnectVisualChildren(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElementOverrides>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).OnDisconnectVisualChildren)(
                ::windows_core::Interface::as_raw(this),
            )
            .ok()
        }
    }
}
impl ::windows_core::RuntimeType for Bold {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for Bold {
    type Vtable = IBold_Vtbl;
}
unsafe impl ::windows_core::ComInterface for Bold {
    const IID: ::windows_core::GUID = <IBold as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for Bold {
    const NAME: &'static str = "Microsoft.UI.Xaml.Documents.Bold";
}
::windows_core::imp::interface_hierarchy!(
    Bold,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<Span> for Bold {}
impl ::windows_core::CanTryInto<Inline> for Bold {}
impl ::windows_core::CanTryInto<TextElement> for Bold {}
impl ::windows_core::CanTryInto<super::DependencyObject> for Bold {}
unsafe impl ::core::marker::Send for Bold {}
unsafe impl ::core::marker::Sync for Bold {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct Glyphs(::windows_core::IUnknown);
impl Glyphs {
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
            Glyphs,
            ::windows_core::imp::IGenericFactory,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "Required features: `\"Microsoft_UI_Composition\"`"]
    #[cfg(feature = "Microsoft_UI_Composition")]
    pub fn PopulatePropertyInfo<P0>(
        &self,
        propertyname: &::windows_core::HSTRING,
        propertyinfo: P0,
    ) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Composition::AnimationPropertyInfo>,
    {
        let this = &::windows_core::ComInterface::cast::<
            super::super::Composition::IAnimationObject,
        >(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).PopulatePropertyInfo)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(propertyname),
                propertyinfo.into_param().abi(),
            )
            .ok()
        }
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
    #[doc = "Required features: `\"Windows_Foundation_Collections\"`"]
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub fn Triggers(&self) -> ::windows_core::Result<super::TriggerCollection> {
        let this = &::windows_core::ComInterface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Triggers)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn Resources(&self) -> ::windows_core::Result<super::ResourceDictionary> {
        let this = &::windows_core::ComInterface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Resources)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetResources<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::ResourceDictionary>,
    {
        let this = &::windows_core::ComInterface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetResources)(
                ::windows_core::Interface::as_raw(this),
                value.try_into_param()?.abi(),
            )
            .ok()
        }
    }
    pub fn Tag(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = &::windows_core::ComInterface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Tag)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetTag<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = &::windows_core::ComInterface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetTag)(
                ::windows_core::Interface::as_raw(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn Language(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<super::IFrameworkElement>(self)?;
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
        let this = &::windows_core::ComInterface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetLanguage)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn ActualWidth(&self) -> ::windows_core::Result<f64> {
        let this = &::windows_core::ComInterface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ActualWidth)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn ActualHeight(&self) -> ::windows_core::Result<f64> {
        let this = &::windows_core::ComInterface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ActualHeight)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn Width(&self) -> ::windows_core::Result<f64> {
        let this = &::windows_core::ComInterface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Width)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetWidth(&self, value: f64) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetWidth)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn Height(&self) -> ::windows_core::Result<f64> {
        let this = &::windows_core::ComInterface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Height)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetHeight(&self, value: f64) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetHeight)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn MinWidth(&self) -> ::windows_core::Result<f64> {
        let this = &::windows_core::ComInterface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MinWidth)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetMinWidth(&self, value: f64) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetMinWidth)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn MaxWidth(&self) -> ::windows_core::Result<f64> {
        let this = &::windows_core::ComInterface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MaxWidth)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetMaxWidth(&self, value: f64) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetMaxWidth)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn MinHeight(&self) -> ::windows_core::Result<f64> {
        let this = &::windows_core::ComInterface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MinHeight)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetMinHeight(&self, value: f64) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetMinHeight)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn MaxHeight(&self) -> ::windows_core::Result<f64> {
        let this = &::windows_core::ComInterface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MaxHeight)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetMaxHeight(&self, value: f64) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetMaxHeight)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn HorizontalAlignment(&self) -> ::windows_core::Result<super::HorizontalAlignment> {
        let this = &::windows_core::ComInterface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HorizontalAlignment)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetHorizontalAlignment(
        &self,
        value: super::HorizontalAlignment,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetHorizontalAlignment)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn VerticalAlignment(&self) -> ::windows_core::Result<super::VerticalAlignment> {
        let this = &::windows_core::ComInterface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).VerticalAlignment)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetVerticalAlignment(
        &self,
        value: super::VerticalAlignment,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetVerticalAlignment)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn Margin(&self) -> ::windows_core::Result<super::Thickness> {
        let this = &::windows_core::ComInterface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Margin)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetMargin(&self, value: super::Thickness) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetMargin)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn Name(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Name)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetName(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetName)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn BaseUri(&self) -> ::windows_core::Result<::windows::Foundation::Uri> {
        let this = &::windows_core::ComInterface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).BaseUri)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn DataContext(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = &::windows_core::ComInterface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DataContext)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetDataContext<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = &::windows_core::ComInterface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetDataContext)(
                ::windows_core::Interface::as_raw(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn AllowFocusOnInteraction(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AllowFocusOnInteraction)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetAllowFocusOnInteraction(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetAllowFocusOnInteraction)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn FocusVisualMargin(&self) -> ::windows_core::Result<super::Thickness> {
        let this = &::windows_core::ComInterface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FocusVisualMargin)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetFocusVisualMargin(&self, value: super::Thickness) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetFocusVisualMargin)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn FocusVisualSecondaryThickness(&self) -> ::windows_core::Result<super::Thickness> {
        let this = &::windows_core::ComInterface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FocusVisualSecondaryThickness)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetFocusVisualSecondaryThickness(
        &self,
        value: super::Thickness,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetFocusVisualSecondaryThickness)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn FocusVisualPrimaryThickness(&self) -> ::windows_core::Result<super::Thickness> {
        let this = &::windows_core::ComInterface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FocusVisualPrimaryThickness)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetFocusVisualPrimaryThickness(
        &self,
        value: super::Thickness,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetFocusVisualPrimaryThickness)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Media\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Media")]
    pub fn FocusVisualSecondaryBrush(&self) -> ::windows_core::Result<super::Media::Brush> {
        let this = &::windows_core::ComInterface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FocusVisualSecondaryBrush)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Media\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Media")]
    pub fn SetFocusVisualSecondaryBrush<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::Media::Brush>,
    {
        let this = &::windows_core::ComInterface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetFocusVisualSecondaryBrush)(
                ::windows_core::Interface::as_raw(this),
                value.try_into_param()?.abi(),
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Media\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Media")]
    pub fn FocusVisualPrimaryBrush(&self) -> ::windows_core::Result<super::Media::Brush> {
        let this = &::windows_core::ComInterface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FocusVisualPrimaryBrush)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Media\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Media")]
    pub fn SetFocusVisualPrimaryBrush<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::Media::Brush>,
    {
        let this = &::windows_core::ComInterface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetFocusVisualPrimaryBrush)(
                ::windows_core::Interface::as_raw(this),
                value.try_into_param()?.abi(),
            )
            .ok()
        }
    }
    pub fn AllowFocusWhenDisabled(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AllowFocusWhenDisabled)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetAllowFocusWhenDisabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetAllowFocusWhenDisabled)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn Style(&self) -> ::windows_core::Result<super::Style> {
        let this = &::windows_core::ComInterface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Style)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetStyle<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::Style>,
    {
        let this = &::windows_core::ComInterface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetStyle)(
                ::windows_core::Interface::as_raw(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn Parent(&self) -> ::windows_core::Result<super::DependencyObject> {
        let this = &::windows_core::ComInterface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Parent)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn FlowDirection(&self) -> ::windows_core::Result<super::FlowDirection> {
        let this = &::windows_core::ComInterface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FlowDirection)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetFlowDirection(&self, value: super::FlowDirection) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetFlowDirection)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn RequestedTheme(&self) -> ::windows_core::Result<super::ElementTheme> {
        let this = &::windows_core::ComInterface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RequestedTheme)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetRequestedTheme(&self, value: super::ElementTheme) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetRequestedTheme)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsLoaded(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsLoaded)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn ActualTheme(&self) -> ::windows_core::Result<super::ElementTheme> {
        let this = &::windows_core::ComInterface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ActualTheme)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn Loaded<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::RoutedEventHandler>,
    {
        let this = &::windows_core::ComInterface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Loaded)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveLoaded(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveLoaded)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn Unloaded<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::RoutedEventHandler>,
    {
        let this = &::windows_core::ComInterface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Unloaded)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveUnloaded(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveUnloaded)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn DataContextChanged<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<
                super::FrameworkElement,
                super::DataContextChangedEventArgs,
            >,
        >,
    {
        let this = &::windows_core::ComInterface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DataContextChanged)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveDataContextChanged(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveDataContextChanged)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn SizeChanged<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::SizeChangedEventHandler>,
    {
        let this = &::windows_core::ComInterface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SizeChanged)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveSizeChanged(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveSizeChanged)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn LayoutUpdated<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::EventHandler<::windows_core::IInspectable>,
        >,
    {
        let this = &::windows_core::ComInterface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LayoutUpdated)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveLayoutUpdated(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveLayoutUpdated)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn Loading<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<
                super::FrameworkElement,
                ::windows_core::IInspectable,
            >,
        >,
    {
        let this = &::windows_core::ComInterface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Loading)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveLoading(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveLoading)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn ActualThemeChanged<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<
                super::FrameworkElement,
                ::windows_core::IInspectable,
            >,
        >,
    {
        let this = &::windows_core::ComInterface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ActualThemeChanged)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveActualThemeChanged(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveActualThemeChanged)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn EffectiveViewportChanged<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<
                super::FrameworkElement,
                super::EffectiveViewportChangedEventArgs,
            >,
        >,
    {
        let this = &::windows_core::ComInterface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).EffectiveViewportChanged)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveEffectiveViewportChanged(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveEffectiveViewportChanged)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn FindName(
        &self,
        name: &::windows_core::HSTRING,
    ) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = &::windows_core::ComInterface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FindName)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(name),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Data\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Data")]
    pub fn SetBinding<P0, P1>(&self, dp: P0, binding: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::DependencyProperty>,
        P1: ::windows_core::TryIntoParam<super::Data::BindingBase>,
    {
        let this = &::windows_core::ComInterface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetBinding)(
                ::windows_core::Interface::as_raw(this),
                dp.into_param().abi(),
                binding.try_into_param()?.abi(),
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Data\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Data")]
    pub fn GetBindingExpression<P0>(
        &self,
        dp: P0,
    ) -> ::windows_core::Result<super::Data::BindingExpression>
    where
        P0: ::windows_core::IntoParam<super::DependencyProperty>,
    {
        let this = &::windows_core::ComInterface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetBindingExpression)(
                ::windows_core::Interface::as_raw(this),
                dp.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn MeasureOverride(
        &self,
        availablesize: ::windows::Foundation::Size,
    ) -> ::windows_core::Result<::windows::Foundation::Size> {
        let this = &::windows_core::ComInterface::cast::<super::IFrameworkElementOverrides>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MeasureOverride)(
                ::windows_core::Interface::as_raw(this),
                availablesize,
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn ArrangeOverride(
        &self,
        finalsize: ::windows::Foundation::Size,
    ) -> ::windows_core::Result<::windows::Foundation::Size> {
        let this = &::windows_core::ComInterface::cast::<super::IFrameworkElementOverrides>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ArrangeOverride)(
                ::windows_core::Interface::as_raw(this),
                finalsize,
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn OnApplyTemplate(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::IFrameworkElementOverrides>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).OnApplyTemplate)(
                ::windows_core::Interface::as_raw(this),
            )
            .ok()
        }
    }
    pub fn GoToElementStateCore(
        &self,
        statename: &::windows_core::HSTRING,
        usetransitions: bool,
    ) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<super::IFrameworkElementOverrides>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GoToElementStateCore)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(statename),
                usetransitions,
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn InvalidateViewport(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::IFrameworkElementProtected>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).InvalidateViewport)(
                ::windows_core::Interface::as_raw(this),
            )
            .ok()
        }
    }
    pub fn UnicodeString(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).UnicodeString)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetUnicodeString(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetUnicodeString)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn Indices(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Indices)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetIndices(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetIndices)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn FontUri(&self) -> ::windows_core::Result<::windows::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FontUri)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn SetFontUri<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows::Foundation::Uri>,
    {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetFontUri)(
                ::windows_core::Interface::as_raw(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Media\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Media")]
    pub fn StyleSimulations(&self) -> ::windows_core::Result<super::Media::StyleSimulations> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StyleSimulations)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Media\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Media")]
    pub fn SetStyleSimulations(
        &self,
        value: super::Media::StyleSimulations,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetStyleSimulations)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn FontRenderingEmSize(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FontRenderingEmSize)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetFontRenderingEmSize(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetFontRenderingEmSize)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn OriginX(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OriginX)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetOriginX(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetOriginX)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn OriginY(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OriginY)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetOriginY(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetOriginY)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Media\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Media")]
    pub fn Fill(&self) -> ::windows_core::Result<super::Media::Brush> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Fill)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Media\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Media")]
    pub fn SetFill<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::Media::Brush>,
    {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetFill)(
                ::windows_core::Interface::as_raw(this),
                value.try_into_param()?.abi(),
            )
            .ok()
        }
    }
    pub fn IsColorFontEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsColorFontEnabled)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetIsColorFontEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetIsColorFontEnabled)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn ColorFontPaletteIndex(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ColorFontPaletteIndex)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetColorFontPaletteIndex(&self, value: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetColorFontPaletteIndex)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn UnicodeStringProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IGlyphsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).UnicodeStringProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn IndicesProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IGlyphsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IndicesProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn FontUriProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IGlyphsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FontUriProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn StyleSimulationsProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IGlyphsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StyleSimulationsProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn FontRenderingEmSizeProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IGlyphsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FontRenderingEmSizeProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn OriginXProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IGlyphsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OriginXProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn OriginYProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IGlyphsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OriginYProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn FillProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IGlyphsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FillProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn IsColorFontEnabledProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IGlyphsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsColorFontEnabledProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn ColorFontPaletteIndexProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IGlyphsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ColorFontPaletteIndexProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn DesiredSize(&self) -> ::windows_core::Result<::windows::Foundation::Size> {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DesiredSize)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn AllowDrop(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AllowDrop)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetAllowDrop(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetAllowDrop)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn Opacity(&self) -> ::windows_core::Result<f64> {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Opacity)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetOpacity(&self, value: f64) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetOpacity)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Media\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Media")]
    pub fn Clip(&self) -> ::windows_core::Result<super::Media::RectangleGeometry> {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Clip)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Media\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Media")]
    pub fn SetClip<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::Media::RectangleGeometry>,
    {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetClip)(
                ::windows_core::Interface::as_raw(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Media\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Media")]
    pub fn RenderTransform(&self) -> ::windows_core::Result<super::Media::Transform> {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RenderTransform)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Media\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Media")]
    pub fn SetRenderTransform<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::Media::Transform>,
    {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetRenderTransform)(
                ::windows_core::Interface::as_raw(this),
                value.try_into_param()?.abi(),
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Media\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Media")]
    pub fn Projection(&self) -> ::windows_core::Result<super::Media::Projection> {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Projection)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Media\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Media")]
    pub fn SetProjection<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::Media::Projection>,
    {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetProjection)(
                ::windows_core::Interface::as_raw(this),
                value.try_into_param()?.abi(),
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Media_Media3D\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Media_Media3D")]
    pub fn Transform3D(&self) -> ::windows_core::Result<super::Media::Media3D::Transform3D> {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Transform3D)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Media_Media3D\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Media_Media3D")]
    pub fn SetTransform3D<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::Media::Media3D::Transform3D>,
    {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetTransform3D)(
                ::windows_core::Interface::as_raw(this),
                value.try_into_param()?.abi(),
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RenderTransformOrigin(&self) -> ::windows_core::Result<::windows::Foundation::Point> {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RenderTransformOrigin)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn SetRenderTransformOrigin(
        &self,
        value: ::windows::Foundation::Point,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetRenderTransformOrigin)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsHitTestVisible(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsHitTestVisible)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetIsHitTestVisible(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetIsHitTestVisible)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn Visibility(&self) -> ::windows_core::Result<super::Visibility> {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Visibility)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetVisibility(&self, value: super::Visibility) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetVisibility)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RenderSize(&self) -> ::windows_core::Result<::windows::Foundation::Size> {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RenderSize)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn UseLayoutRounding(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).UseLayoutRounding)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetUseLayoutRounding(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetUseLayoutRounding)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Media_Animation\"`, `\"Windows_Foundation_Collections\"`"]
    #[cfg(all(
        feature = "Microsoft_UI_Xaml_Media_Animation",
        feature = "Windows_Foundation_Collections"
    ))]
    pub fn Transitions(
        &self,
    ) -> ::windows_core::Result<super::Media::Animation::TransitionCollection> {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Transitions)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Media_Animation\"`, `\"Windows_Foundation_Collections\"`"]
    #[cfg(all(
        feature = "Microsoft_UI_Xaml_Media_Animation",
        feature = "Windows_Foundation_Collections"
    ))]
    pub fn SetTransitions<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::Media::Animation::TransitionCollection>,
    {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetTransitions)(
                ::windows_core::Interface::as_raw(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Media\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Media")]
    pub fn CacheMode(&self) -> ::windows_core::Result<super::Media::CacheMode> {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CacheMode)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Media\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Media")]
    pub fn SetCacheMode<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::Media::CacheMode>,
    {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetCacheMode)(
                ::windows_core::Interface::as_raw(this),
                value.try_into_param()?.abi(),
            )
            .ok()
        }
    }
    pub fn IsTapEnabled(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsTapEnabled)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetIsTapEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetIsTapEnabled)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsDoubleTapEnabled(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsDoubleTapEnabled)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetIsDoubleTapEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetIsDoubleTapEnabled)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn CanDrag(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CanDrag)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetCanDrag(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetCanDrag)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsRightTapEnabled(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsRightTapEnabled)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetIsRightTapEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetIsRightTapEnabled)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsHoldingEnabled(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsHoldingEnabled)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetIsHoldingEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetIsHoldingEnabled)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Input\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Input")]
    pub fn ManipulationMode(&self) -> ::windows_core::Result<super::Input::ManipulationModes> {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ManipulationMode)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Input\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Input")]
    pub fn SetManipulationMode(
        &self,
        value: super::Input::ManipulationModes,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetManipulationMode)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Input\"`, `\"Windows_Foundation_Collections\"`"]
    #[cfg(all(
        feature = "Microsoft_UI_Xaml_Input",
        feature = "Windows_Foundation_Collections"
    ))]
    pub fn PointerCaptures(
        &self,
    ) -> ::windows_core::Result<
        ::windows::Foundation::Collections::IVectorView<super::Input::Pointer>,
    > {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PointerCaptures)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Controls_Primitives\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Controls_Primitives")]
    pub fn ContextFlyout(&self) -> ::windows_core::Result<super::Controls::Primitives::FlyoutBase> {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ContextFlyout)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Controls_Primitives\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Controls_Primitives")]
    pub fn SetContextFlyout<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::Controls::Primitives::FlyoutBase>,
    {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetContextFlyout)(
                ::windows_core::Interface::as_raw(this),
                value.try_into_param()?.abi(),
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Media\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Media")]
    pub fn CompositeMode(&self) -> ::windows_core::Result<super::Media::ElementCompositeMode> {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CompositeMode)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Media\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Media")]
    pub fn SetCompositeMode(
        &self,
        value: super::Media::ElementCompositeMode,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetCompositeMode)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Media\"`, `\"Windows_Foundation_Collections\"`"]
    #[cfg(all(
        feature = "Microsoft_UI_Xaml_Media",
        feature = "Windows_Foundation_Collections"
    ))]
    pub fn Lights(
        &self,
    ) -> ::windows_core::Result<::windows::Foundation::Collections::IVector<super::Media::XamlLight>>
    {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Lights)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn CanBeScrollAnchor(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CanBeScrollAnchor)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetCanBeScrollAnchor(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetCanBeScrollAnchor)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn ExitDisplayModeOnAccessKeyInvoked(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ExitDisplayModeOnAccessKeyInvoked)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetExitDisplayModeOnAccessKeyInvoked(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetExitDisplayModeOnAccessKeyInvoked)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsAccessKeyScope(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsAccessKeyScope)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetIsAccessKeyScope(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetIsAccessKeyScope)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn AccessKeyScopeOwner(&self) -> ::windows_core::Result<super::DependencyObject> {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AccessKeyScopeOwner)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetAccessKeyScopeOwner<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetAccessKeyScopeOwner)(
                ::windows_core::Interface::as_raw(this),
                value.try_into_param()?.abi(),
            )
            .ok()
        }
    }
    pub fn AccessKey(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AccessKey)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetAccessKey(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetAccessKey)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Input\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Input")]
    pub fn KeyTipPlacementMode(&self) -> ::windows_core::Result<super::Input::KeyTipPlacementMode> {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).KeyTipPlacementMode)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Input\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Input")]
    pub fn SetKeyTipPlacementMode(
        &self,
        value: super::Input::KeyTipPlacementMode,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetKeyTipPlacementMode)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn KeyTipHorizontalOffset(&self) -> ::windows_core::Result<f64> {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).KeyTipHorizontalOffset)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetKeyTipHorizontalOffset(&self, value: f64) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetKeyTipHorizontalOffset)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn KeyTipVerticalOffset(&self) -> ::windows_core::Result<f64> {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).KeyTipVerticalOffset)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetKeyTipVerticalOffset(&self, value: f64) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetKeyTipVerticalOffset)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn KeyTipTarget(&self) -> ::windows_core::Result<super::DependencyObject> {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).KeyTipTarget)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetKeyTipTarget<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetKeyTipTarget)(
                ::windows_core::Interface::as_raw(this),
                value.try_into_param()?.abi(),
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Input\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Input")]
    pub fn XYFocusKeyboardNavigation(
        &self,
    ) -> ::windows_core::Result<super::Input::XYFocusKeyboardNavigationMode> {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).XYFocusKeyboardNavigation)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Input\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Input")]
    pub fn SetXYFocusKeyboardNavigation(
        &self,
        value: super::Input::XYFocusKeyboardNavigationMode,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetXYFocusKeyboardNavigation)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Input\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Input")]
    pub fn XYFocusUpNavigationStrategy(
        &self,
    ) -> ::windows_core::Result<super::Input::XYFocusNavigationStrategy> {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).XYFocusUpNavigationStrategy)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Input\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Input")]
    pub fn SetXYFocusUpNavigationStrategy(
        &self,
        value: super::Input::XYFocusNavigationStrategy,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetXYFocusUpNavigationStrategy)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Input\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Input")]
    pub fn XYFocusDownNavigationStrategy(
        &self,
    ) -> ::windows_core::Result<super::Input::XYFocusNavigationStrategy> {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).XYFocusDownNavigationStrategy)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Input\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Input")]
    pub fn SetXYFocusDownNavigationStrategy(
        &self,
        value: super::Input::XYFocusNavigationStrategy,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetXYFocusDownNavigationStrategy)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Input\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Input")]
    pub fn XYFocusLeftNavigationStrategy(
        &self,
    ) -> ::windows_core::Result<super::Input::XYFocusNavigationStrategy> {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).XYFocusLeftNavigationStrategy)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Input\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Input")]
    pub fn SetXYFocusLeftNavigationStrategy(
        &self,
        value: super::Input::XYFocusNavigationStrategy,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetXYFocusLeftNavigationStrategy)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Input\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Input")]
    pub fn XYFocusRightNavigationStrategy(
        &self,
    ) -> ::windows_core::Result<super::Input::XYFocusNavigationStrategy> {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).XYFocusRightNavigationStrategy)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Input\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Input")]
    pub fn SetXYFocusRightNavigationStrategy(
        &self,
        value: super::Input::XYFocusNavigationStrategy,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetXYFocusRightNavigationStrategy)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Input\"`, `\"Windows_Foundation_Collections\"`"]
    #[cfg(all(
        feature = "Microsoft_UI_Xaml_Input",
        feature = "Windows_Foundation_Collections"
    ))]
    pub fn KeyboardAccelerators(
        &self,
    ) -> ::windows_core::Result<
        ::windows::Foundation::Collections::IVector<super::Input::KeyboardAccelerator>,
    > {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).KeyboardAccelerators)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn KeyboardAcceleratorPlacementTarget(
        &self,
    ) -> ::windows_core::Result<super::DependencyObject> {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).KeyboardAcceleratorPlacementTarget)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetKeyboardAcceleratorPlacementTarget<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetKeyboardAcceleratorPlacementTarget)(
                ::windows_core::Interface::as_raw(this),
                value.try_into_param()?.abi(),
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Input\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Input")]
    pub fn KeyboardAcceleratorPlacementMode(
        &self,
    ) -> ::windows_core::Result<super::Input::KeyboardAcceleratorPlacementMode> {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).KeyboardAcceleratorPlacementMode)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Input\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Input")]
    pub fn SetKeyboardAcceleratorPlacementMode(
        &self,
        value: super::Input::KeyboardAcceleratorPlacementMode,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetKeyboardAcceleratorPlacementMode)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn HighContrastAdjustment(
        &self,
    ) -> ::windows_core::Result<super::ElementHighContrastAdjustment> {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HighContrastAdjustment)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetHighContrastAdjustment(
        &self,
        value: super::ElementHighContrastAdjustment,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetHighContrastAdjustment)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Input\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Input")]
    pub fn TabFocusNavigation(
        &self,
    ) -> ::windows_core::Result<super::Input::KeyboardNavigationMode> {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TabFocusNavigation)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Input\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Input")]
    pub fn SetTabFocusNavigation(
        &self,
        value: super::Input::KeyboardNavigationMode,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetTabFocusNavigation)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn OpacityTransition(&self) -> ::windows_core::Result<super::ScalarTransition> {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OpacityTransition)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetOpacityTransition<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::ScalarTransition>,
    {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetOpacityTransition)(
                ::windows_core::Interface::as_raw(this),
                value.try_into_param()?.abi(),
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation_Numerics\"`"]
    #[cfg(feature = "Windows_Foundation_Numerics")]
    pub fn Translation(&self) -> ::windows_core::Result<::windows::Foundation::Numerics::Vector3> {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Translation)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation_Numerics\"`"]
    #[cfg(feature = "Windows_Foundation_Numerics")]
    pub fn SetTranslation(
        &self,
        value: ::windows::Foundation::Numerics::Vector3,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetTranslation)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn TranslationTransition(&self) -> ::windows_core::Result<super::Vector3Transition> {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TranslationTransition)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetTranslationTransition<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::Vector3Transition>,
    {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetTranslationTransition)(
                ::windows_core::Interface::as_raw(this),
                value.try_into_param()?.abi(),
            )
            .ok()
        }
    }
    pub fn Rotation(&self) -> ::windows_core::Result<f32> {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Rotation)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetRotation(&self, value: f32) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetRotation)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn RotationTransition(&self) -> ::windows_core::Result<super::ScalarTransition> {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RotationTransition)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetRotationTransition<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::ScalarTransition>,
    {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetRotationTransition)(
                ::windows_core::Interface::as_raw(this),
                value.try_into_param()?.abi(),
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation_Numerics\"`"]
    #[cfg(feature = "Windows_Foundation_Numerics")]
    pub fn Scale(&self) -> ::windows_core::Result<::windows::Foundation::Numerics::Vector3> {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Scale)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation_Numerics\"`"]
    #[cfg(feature = "Windows_Foundation_Numerics")]
    pub fn SetScale(
        &self,
        value: ::windows::Foundation::Numerics::Vector3,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetScale)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn ScaleTransition(&self) -> ::windows_core::Result<super::Vector3Transition> {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ScaleTransition)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetScaleTransition<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::Vector3Transition>,
    {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetScaleTransition)(
                ::windows_core::Interface::as_raw(this),
                value.try_into_param()?.abi(),
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation_Numerics\"`"]
    #[cfg(feature = "Windows_Foundation_Numerics")]
    pub fn TransformMatrix(
        &self,
    ) -> ::windows_core::Result<::windows::Foundation::Numerics::Matrix4x4> {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TransformMatrix)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation_Numerics\"`"]
    #[cfg(feature = "Windows_Foundation_Numerics")]
    pub fn SetTransformMatrix(
        &self,
        value: ::windows::Foundation::Numerics::Matrix4x4,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetTransformMatrix)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation_Numerics\"`"]
    #[cfg(feature = "Windows_Foundation_Numerics")]
    pub fn CenterPoint(&self) -> ::windows_core::Result<::windows::Foundation::Numerics::Vector3> {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CenterPoint)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation_Numerics\"`"]
    #[cfg(feature = "Windows_Foundation_Numerics")]
    pub fn SetCenterPoint(
        &self,
        value: ::windows::Foundation::Numerics::Vector3,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetCenterPoint)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation_Numerics\"`"]
    #[cfg(feature = "Windows_Foundation_Numerics")]
    pub fn RotationAxis(&self) -> ::windows_core::Result<::windows::Foundation::Numerics::Vector3> {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RotationAxis)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation_Numerics\"`"]
    #[cfg(feature = "Windows_Foundation_Numerics")]
    pub fn SetRotationAxis(
        &self,
        value: ::windows::Foundation::Numerics::Vector3,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetRotationAxis)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation_Numerics\"`"]
    #[cfg(feature = "Windows_Foundation_Numerics")]
    pub fn ActualOffset(&self) -> ::windows_core::Result<::windows::Foundation::Numerics::Vector3> {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ActualOffset)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation_Numerics\"`"]
    #[cfg(feature = "Windows_Foundation_Numerics")]
    pub fn ActualSize(&self) -> ::windows_core::Result<::windows::Foundation::Numerics::Vector2> {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ActualSize)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn XamlRoot(&self) -> ::windows_core::Result<super::XamlRoot> {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).XamlRoot)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetXamlRoot<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::XamlRoot>,
    {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetXamlRoot)(
                ::windows_core::Interface::as_raw(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Media\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Media")]
    pub fn Shadow(&self) -> ::windows_core::Result<super::Media::Shadow> {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Shadow)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Media\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Media")]
    pub fn SetShadow<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::Media::Shadow>,
    {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetShadow)(
                ::windows_core::Interface::as_raw(this),
                value.try_into_param()?.abi(),
            )
            .ok()
        }
    }
    pub fn RasterizationScale(&self) -> ::windows_core::Result<f64> {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RasterizationScale)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetRasterizationScale(&self, value: f64) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetRasterizationScale)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn FocusState(&self) -> ::windows_core::Result<super::FocusState> {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FocusState)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn UseSystemFocusVisuals(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).UseSystemFocusVisuals)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetUseSystemFocusVisuals(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetUseSystemFocusVisuals)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn XYFocusLeft(&self) -> ::windows_core::Result<super::DependencyObject> {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).XYFocusLeft)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetXYFocusLeft<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetXYFocusLeft)(
                ::windows_core::Interface::as_raw(this),
                value.try_into_param()?.abi(),
            )
            .ok()
        }
    }
    pub fn XYFocusRight(&self) -> ::windows_core::Result<super::DependencyObject> {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).XYFocusRight)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetXYFocusRight<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetXYFocusRight)(
                ::windows_core::Interface::as_raw(this),
                value.try_into_param()?.abi(),
            )
            .ok()
        }
    }
    pub fn XYFocusUp(&self) -> ::windows_core::Result<super::DependencyObject> {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).XYFocusUp)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetXYFocusUp<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetXYFocusUp)(
                ::windows_core::Interface::as_raw(this),
                value.try_into_param()?.abi(),
            )
            .ok()
        }
    }
    pub fn XYFocusDown(&self) -> ::windows_core::Result<super::DependencyObject> {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).XYFocusDown)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetXYFocusDown<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetXYFocusDown)(
                ::windows_core::Interface::as_raw(this),
                value.try_into_param()?.abi(),
            )
            .ok()
        }
    }
    pub fn IsTabStop(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsTabStop)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetIsTabStop(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetIsTabStop)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn TabIndex(&self) -> ::windows_core::Result<i32> {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TabIndex)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetTabIndex(&self, value: i32) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetTabIndex)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Input\"`, `\"Windows_Foundation\"`"]
    #[cfg(all(feature = "Microsoft_UI_Xaml_Input", feature = "Windows_Foundation"))]
    pub fn KeyUp<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::Input::KeyEventHandler>,
    {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).KeyUp)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveKeyUp(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveKeyUp)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Input\"`, `\"Windows_Foundation\"`"]
    #[cfg(all(feature = "Microsoft_UI_Xaml_Input", feature = "Windows_Foundation"))]
    pub fn KeyDown<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::Input::KeyEventHandler>,
    {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).KeyDown)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveKeyDown(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveKeyDown)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn GotFocus<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::RoutedEventHandler>,
    {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GotFocus)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveGotFocus(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveGotFocus)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn LostFocus<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::RoutedEventHandler>,
    {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LostFocus)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveLostFocus(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveLostFocus)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn DragStarting<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<
                super::UIElement,
                super::DragStartingEventArgs,
            >,
        >,
    {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DragStarting)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveDragStarting(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveDragStarting)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn DropCompleted<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<
                super::UIElement,
                super::DropCompletedEventArgs,
            >,
        >,
    {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DropCompleted)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveDropCompleted(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveDropCompleted)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Input\"`, `\"Windows_Foundation\"`"]
    #[cfg(all(feature = "Microsoft_UI_Xaml_Input", feature = "Windows_Foundation"))]
    pub fn CharacterReceived<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<
                super::UIElement,
                super::Input::CharacterReceivedRoutedEventArgs,
            >,
        >,
    {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CharacterReceived)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveCharacterReceived(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveCharacterReceived)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn DragEnter<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::DragEventHandler>,
    {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DragEnter)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveDragEnter(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveDragEnter)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn DragLeave<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::DragEventHandler>,
    {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DragLeave)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveDragLeave(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveDragLeave)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn DragOver<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::DragEventHandler>,
    {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DragOver)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveDragOver(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveDragOver)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn Drop<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::DragEventHandler>,
    {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Drop)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveDrop(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveDrop)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Input\"`, `\"Windows_Foundation\"`"]
    #[cfg(all(feature = "Microsoft_UI_Xaml_Input", feature = "Windows_Foundation"))]
    pub fn PointerPressed<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::Input::PointerEventHandler>,
    {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PointerPressed)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemovePointerPressed(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).RemovePointerPressed)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Input\"`, `\"Windows_Foundation\"`"]
    #[cfg(all(feature = "Microsoft_UI_Xaml_Input", feature = "Windows_Foundation"))]
    pub fn PointerMoved<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::Input::PointerEventHandler>,
    {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PointerMoved)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemovePointerMoved(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).RemovePointerMoved)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Input\"`, `\"Windows_Foundation\"`"]
    #[cfg(all(feature = "Microsoft_UI_Xaml_Input", feature = "Windows_Foundation"))]
    pub fn PointerReleased<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::Input::PointerEventHandler>,
    {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PointerReleased)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemovePointerReleased(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).RemovePointerReleased)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Input\"`, `\"Windows_Foundation\"`"]
    #[cfg(all(feature = "Microsoft_UI_Xaml_Input", feature = "Windows_Foundation"))]
    pub fn PointerEntered<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::Input::PointerEventHandler>,
    {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PointerEntered)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemovePointerEntered(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).RemovePointerEntered)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Input\"`, `\"Windows_Foundation\"`"]
    #[cfg(all(feature = "Microsoft_UI_Xaml_Input", feature = "Windows_Foundation"))]
    pub fn PointerExited<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::Input::PointerEventHandler>,
    {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PointerExited)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemovePointerExited(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).RemovePointerExited)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Input\"`, `\"Windows_Foundation\"`"]
    #[cfg(all(feature = "Microsoft_UI_Xaml_Input", feature = "Windows_Foundation"))]
    pub fn PointerCaptureLost<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::Input::PointerEventHandler>,
    {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PointerCaptureLost)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemovePointerCaptureLost(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).RemovePointerCaptureLost)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Input\"`, `\"Windows_Foundation\"`"]
    #[cfg(all(feature = "Microsoft_UI_Xaml_Input", feature = "Windows_Foundation"))]
    pub fn PointerCanceled<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::Input::PointerEventHandler>,
    {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PointerCanceled)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemovePointerCanceled(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).RemovePointerCanceled)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Input\"`, `\"Windows_Foundation\"`"]
    #[cfg(all(feature = "Microsoft_UI_Xaml_Input", feature = "Windows_Foundation"))]
    pub fn PointerWheelChanged<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::Input::PointerEventHandler>,
    {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PointerWheelChanged)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemovePointerWheelChanged(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).RemovePointerWheelChanged)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Input\"`, `\"Windows_Foundation\"`"]
    #[cfg(all(feature = "Microsoft_UI_Xaml_Input", feature = "Windows_Foundation"))]
    pub fn Tapped<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::Input::TappedEventHandler>,
    {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Tapped)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveTapped(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveTapped)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Input\"`, `\"Windows_Foundation\"`"]
    #[cfg(all(feature = "Microsoft_UI_Xaml_Input", feature = "Windows_Foundation"))]
    pub fn DoubleTapped<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::Input::DoubleTappedEventHandler>,
    {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DoubleTapped)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveDoubleTapped(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveDoubleTapped)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Input\"`, `\"Windows_Foundation\"`"]
    #[cfg(all(feature = "Microsoft_UI_Xaml_Input", feature = "Windows_Foundation"))]
    pub fn Holding<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::Input::HoldingEventHandler>,
    {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Holding)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveHolding(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveHolding)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Input\"`, `\"Windows_Foundation\"`"]
    #[cfg(all(feature = "Microsoft_UI_Xaml_Input", feature = "Windows_Foundation"))]
    pub fn ContextRequested<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<
                super::UIElement,
                super::Input::ContextRequestedEventArgs,
            >,
        >,
    {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ContextRequested)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveContextRequested(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveContextRequested)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn ContextCanceled<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<super::UIElement, super::RoutedEventArgs>,
        >,
    {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ContextCanceled)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveContextCanceled(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveContextCanceled)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Input\"`, `\"Windows_Foundation\"`"]
    #[cfg(all(feature = "Microsoft_UI_Xaml_Input", feature = "Windows_Foundation"))]
    pub fn RightTapped<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::Input::RightTappedEventHandler>,
    {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RightTapped)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveRightTapped(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveRightTapped)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Input\"`, `\"Windows_Foundation\"`"]
    #[cfg(all(feature = "Microsoft_UI_Xaml_Input", feature = "Windows_Foundation"))]
    pub fn ManipulationStarting<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::Input::ManipulationStartingEventHandler>,
    {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ManipulationStarting)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveManipulationStarting(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveManipulationStarting)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Input\"`, `\"Windows_Foundation\"`"]
    #[cfg(all(feature = "Microsoft_UI_Xaml_Input", feature = "Windows_Foundation"))]
    pub fn ManipulationInertiaStarting<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::Input::ManipulationInertiaStartingEventHandler>,
    {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ManipulationInertiaStarting)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveManipulationInertiaStarting(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveManipulationInertiaStarting)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Input\"`, `\"Windows_Foundation\"`"]
    #[cfg(all(feature = "Microsoft_UI_Xaml_Input", feature = "Windows_Foundation"))]
    pub fn ManipulationStarted<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::Input::ManipulationStartedEventHandler>,
    {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ManipulationStarted)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveManipulationStarted(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveManipulationStarted)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Input\"`, `\"Windows_Foundation\"`"]
    #[cfg(all(feature = "Microsoft_UI_Xaml_Input", feature = "Windows_Foundation"))]
    pub fn ManipulationDelta<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::Input::ManipulationDeltaEventHandler>,
    {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ManipulationDelta)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveManipulationDelta(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveManipulationDelta)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Input\"`, `\"Windows_Foundation\"`"]
    #[cfg(all(feature = "Microsoft_UI_Xaml_Input", feature = "Windows_Foundation"))]
    pub fn ManipulationCompleted<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::Input::ManipulationCompletedEventHandler>,
    {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ManipulationCompleted)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveManipulationCompleted(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveManipulationCompleted)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Input\"`, `\"Windows_Foundation\"`"]
    #[cfg(all(feature = "Microsoft_UI_Xaml_Input", feature = "Windows_Foundation"))]
    pub fn AccessKeyDisplayRequested<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<
                super::UIElement,
                super::Input::AccessKeyDisplayRequestedEventArgs,
            >,
        >,
    {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AccessKeyDisplayRequested)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveAccessKeyDisplayRequested(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveAccessKeyDisplayRequested)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Input\"`, `\"Windows_Foundation\"`"]
    #[cfg(all(feature = "Microsoft_UI_Xaml_Input", feature = "Windows_Foundation"))]
    pub fn AccessKeyDisplayDismissed<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<
                super::UIElement,
                super::Input::AccessKeyDisplayDismissedEventArgs,
            >,
        >,
    {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AccessKeyDisplayDismissed)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveAccessKeyDisplayDismissed(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveAccessKeyDisplayDismissed)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Input\"`, `\"Windows_Foundation\"`"]
    #[cfg(all(feature = "Microsoft_UI_Xaml_Input", feature = "Windows_Foundation"))]
    pub fn AccessKeyInvoked<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<
                super::UIElement,
                super::Input::AccessKeyInvokedEventArgs,
            >,
        >,
    {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AccessKeyInvoked)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveAccessKeyInvoked(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveAccessKeyInvoked)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Input\"`, `\"Windows_Foundation\"`"]
    #[cfg(all(feature = "Microsoft_UI_Xaml_Input", feature = "Windows_Foundation"))]
    pub fn ProcessKeyboardAccelerators<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<
                super::UIElement,
                super::Input::ProcessKeyboardAcceleratorEventArgs,
            >,
        >,
    {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ProcessKeyboardAccelerators)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveProcessKeyboardAccelerators(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveProcessKeyboardAccelerators)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Input\"`, `\"Windows_Foundation\"`"]
    #[cfg(all(feature = "Microsoft_UI_Xaml_Input", feature = "Windows_Foundation"))]
    pub fn GettingFocus<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<
                super::UIElement,
                super::Input::GettingFocusEventArgs,
            >,
        >,
    {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GettingFocus)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveGettingFocus(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveGettingFocus)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Input\"`, `\"Windows_Foundation\"`"]
    #[cfg(all(feature = "Microsoft_UI_Xaml_Input", feature = "Windows_Foundation"))]
    pub fn LosingFocus<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<
                super::UIElement,
                super::Input::LosingFocusEventArgs,
            >,
        >,
    {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LosingFocus)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveLosingFocus(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveLosingFocus)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Input\"`, `\"Windows_Foundation\"`"]
    #[cfg(all(feature = "Microsoft_UI_Xaml_Input", feature = "Windows_Foundation"))]
    pub fn NoFocusCandidateFound<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<
                super::UIElement,
                super::Input::NoFocusCandidateFoundEventArgs,
            >,
        >,
    {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NoFocusCandidateFound)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveNoFocusCandidateFound(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveNoFocusCandidateFound)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Input\"`, `\"Windows_Foundation\"`"]
    #[cfg(all(feature = "Microsoft_UI_Xaml_Input", feature = "Windows_Foundation"))]
    pub fn PreviewKeyDown<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::Input::KeyEventHandler>,
    {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviewKeyDown)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemovePreviewKeyDown(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).RemovePreviewKeyDown)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Input\"`, `\"Windows_Foundation\"`"]
    #[cfg(all(feature = "Microsoft_UI_Xaml_Input", feature = "Windows_Foundation"))]
    pub fn PreviewKeyUp<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::Input::KeyEventHandler>,
    {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviewKeyUp)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemovePreviewKeyUp(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).RemovePreviewKeyUp)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn BringIntoViewRequested<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<
                super::UIElement,
                super::BringIntoViewRequestedEventArgs,
            >,
        >,
    {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).BringIntoViewRequested)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveBringIntoViewRequested(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveBringIntoViewRequested)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn Measure(
        &self,
        availablesize: ::windows::Foundation::Size,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).Measure)(
                ::windows_core::Interface::as_raw(this),
                availablesize,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn Arrange(&self, finalrect: ::windows::Foundation::Rect) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).Arrange)(
                ::windows_core::Interface::as_raw(this),
                finalrect,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Input\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Input")]
    pub fn CapturePointer<P0>(&self, value: P0) -> ::windows_core::Result<bool>
    where
        P0: ::windows_core::IntoParam<super::Input::Pointer>,
    {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CapturePointer)(
                ::windows_core::Interface::as_raw(this),
                value.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Input\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Input")]
    pub fn ReleasePointerCapture<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::Input::Pointer>,
    {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).ReleasePointerCapture)(
                ::windows_core::Interface::as_raw(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn ReleasePointerCaptures(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).ReleasePointerCaptures)(
                ::windows_core::Interface::as_raw(this),
            )
            .ok()
        }
    }
    pub fn AddHandler<P0, P1>(
        &self,
        routedevent: P0,
        handler: P1,
        handledeventstoo: bool,
    ) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::RoutedEvent>,
        P1: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).AddHandler)(
                ::windows_core::Interface::as_raw(this),
                routedevent.into_param().abi(),
                handler.into_param().abi(),
                handledeventstoo,
            )
            .ok()
        }
    }
    pub fn RemoveHandler<P0, P1>(&self, routedevent: P0, handler: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::RoutedEvent>,
        P1: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveHandler)(
                ::windows_core::Interface::as_raw(this),
                routedevent.into_param().abi(),
                handler.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Media\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Media")]
    pub fn TransformToVisual<P0>(
        &self,
        visual: P0,
    ) -> ::windows_core::Result<super::Media::GeneralTransform>
    where
        P0: ::windows_core::TryIntoParam<super::UIElement>,
    {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TransformToVisual)(
                ::windows_core::Interface::as_raw(this),
                visual.try_into_param()?.abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn InvalidateMeasure(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).InvalidateMeasure)(
                ::windows_core::Interface::as_raw(this),
            )
            .ok()
        }
    }
    pub fn InvalidateArrange(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).InvalidateArrange)(
                ::windows_core::Interface::as_raw(this),
            )
            .ok()
        }
    }
    pub fn UpdateLayout(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).UpdateLayout)(
                ::windows_core::Interface::as_raw(this),
            )
            .ok()
        }
    }
    pub fn CancelDirectManipulations(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CancelDirectManipulations)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Input\"`, `\"Windows_ApplicationModel_DataTransfer\"`, `\"Windows_Foundation\"`"]
    #[cfg(all(
        feature = "Microsoft_UI_Input",
        feature = "Windows_ApplicationModel_DataTransfer",
        feature = "Windows_Foundation"
    ))]
    pub fn StartDragAsync<P0>(
        &self,
        pointerpoint: P0,
    ) -> ::windows_core::Result<
        ::windows::Foundation::IAsyncOperation<
            ::windows::ApplicationModel::DataTransfer::DataPackageOperation,
        >,
    >
    where
        P0: ::windows_core::IntoParam<super::super::Input::PointerPoint>,
    {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StartDragAsync)(
                ::windows_core::Interface::as_raw(this),
                pointerpoint.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn StartBringIntoView(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).StartBringIntoView)(
                ::windows_core::Interface::as_raw(this),
            )
            .ok()
        }
    }
    pub fn StartBringIntoViewWithOptions<P0>(&self, options: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::BringIntoViewOptions>,
    {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).StartBringIntoViewWithOptions)(
                ::windows_core::Interface::as_raw(this),
                options.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Input\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Input")]
    pub fn TryInvokeKeyboardAccelerator<P0>(&self, args: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::Input::ProcessKeyboardAcceleratorEventArgs>,
    {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).TryInvokeKeyboardAccelerator)(
                ::windows_core::Interface::as_raw(this),
                args.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn Focus(&self, value: super::FocusState) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Focus)(
                ::windows_core::Interface::as_raw(this),
                value,
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Composition\"`"]
    #[cfg(feature = "Microsoft_UI_Composition")]
    pub fn StartAnimation<P0>(&self, animation: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::Composition::ICompositionAnimationBase>,
    {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).StartAnimation)(
                ::windows_core::Interface::as_raw(this),
                animation.try_into_param()?.abi(),
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Composition\"`"]
    #[cfg(feature = "Microsoft_UI_Composition")]
    pub fn StopAnimation<P0>(&self, animation: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::Composition::ICompositionAnimationBase>,
    {
        let this = &::windows_core::ComInterface::cast::<super::IUIElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).StopAnimation)(
                ::windows_core::Interface::as_raw(this),
                animation.try_into_param()?.abi(),
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Automation_Peers\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Automation_Peers")]
    pub fn OnCreateAutomationPeer(
        &self,
    ) -> ::windows_core::Result<super::Automation::Peers::AutomationPeer> {
        let this = &::windows_core::ComInterface::cast::<super::IUIElementOverrides>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OnCreateAutomationPeer)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn OnDisconnectVisualChildren(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::IUIElementOverrides>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).OnDisconnectVisualChildren)(
                ::windows_core::Interface::as_raw(this),
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation_Collections\"`"]
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub fn FindSubElementsForTouchTargeting(
        &self,
        point: ::windows::Foundation::Point,
        boundingrect: ::windows::Foundation::Rect,
    ) -> ::windows_core::Result<
        ::windows::Foundation::Collections::IIterable<
            ::windows::Foundation::Collections::IIterable<::windows::Foundation::Point>,
        >,
    > {
        let this = &::windows_core::ComInterface::cast::<super::IUIElementOverrides>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FindSubElementsForTouchTargeting)(
                ::windows_core::Interface::as_raw(this),
                point,
                boundingrect,
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation_Collections\"`"]
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub fn GetChildrenInTabFocusOrder(
        &self,
    ) -> ::windows_core::Result<
        ::windows::Foundation::Collections::IIterable<super::DependencyObject>,
    > {
        let this = &::windows_core::ComInterface::cast::<super::IUIElementOverrides>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetChildrenInTabFocusOrder)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Input\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Input")]
    pub fn OnKeyboardAcceleratorInvoked<P0>(&self, args: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::Input::KeyboardAcceleratorInvokedEventArgs>,
    {
        let this = &::windows_core::ComInterface::cast::<super::IUIElementOverrides>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).OnKeyboardAcceleratorInvoked)(
                ::windows_core::Interface::as_raw(this),
                args.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Input\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Input")]
    pub fn OnProcessKeyboardAccelerators<P0>(&self, args: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::Input::ProcessKeyboardAcceleratorEventArgs>,
    {
        let this = &::windows_core::ComInterface::cast::<super::IUIElementOverrides>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).OnProcessKeyboardAccelerators)(
                ::windows_core::Interface::as_raw(this),
                args.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn OnBringIntoViewRequested<P0>(&self, e: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::BringIntoViewRequestedEventArgs>,
    {
        let this = &::windows_core::ComInterface::cast::<super::IUIElementOverrides>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).OnBringIntoViewRequested)(
                ::windows_core::Interface::as_raw(this),
                e.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Composition\"`"]
    #[cfg(feature = "Microsoft_UI_Composition")]
    pub fn PopulatePropertyInfoOverride<P0>(
        &self,
        propertyname: &::windows_core::HSTRING,
        animationpropertyinfo: P0,
    ) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Composition::AnimationPropertyInfo>,
    {
        let this = &::windows_core::ComInterface::cast::<super::IUIElementOverrides>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).PopulatePropertyInfoOverride)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(propertyname),
                animationpropertyinfo.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Input\"`"]
    #[cfg(feature = "Microsoft_UI_Input")]
    pub fn ProtectedCursor(&self) -> ::windows_core::Result<super::super::Input::InputCursor> {
        let this = &::windows_core::ComInterface::cast::<super::IUIElementProtected>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ProtectedCursor)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Input\"`"]
    #[cfg(feature = "Microsoft_UI_Input")]
    pub fn SetProtectedCursor<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::Input::InputCursor>,
    {
        let this = &::windows_core::ComInterface::cast::<super::IUIElementProtected>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetProtectedCursor)(
                ::windows_core::Interface::as_raw(this),
                value.try_into_param()?.abi(),
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Composition\"`"]
    #[cfg(feature = "Microsoft_UI_Composition")]
    pub fn GetVisualInternal(&self) -> ::windows_core::Result<super::super::Composition::Visual> {
        let this = &::windows_core::ComInterface::cast::<super::super::Composition::IVisualElement2>(
            self,
        )?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetVisualInternal)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc(hidden)]
    pub fn IGlyphsStatics<R, F: FnOnce(&IGlyphsStatics) -> ::windows_core::Result<R>>(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<Glyphs, IGlyphsStatics> =
            ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for Glyphs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for Glyphs {
    type Vtable = IGlyphs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for Glyphs {
    const IID: ::windows_core::GUID = <IGlyphs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for Glyphs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Documents.Glyphs";
}
::windows_core::imp::interface_hierarchy!(
    Glyphs,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
#[cfg(feature = "Microsoft_UI_Composition")]
impl ::windows_core::CanTryInto<super::super::Composition::IAnimationObject> for Glyphs {}
#[cfg(feature = "Microsoft_UI_Composition")]
impl ::windows_core::CanTryInto<super::super::Composition::IVisualElement> for Glyphs {}
#[cfg(feature = "Microsoft_UI_Composition")]
impl ::windows_core::CanTryInto<super::super::Composition::IVisualElement2> for Glyphs {}
impl ::windows_core::CanTryInto<super::FrameworkElement> for Glyphs {}
impl ::windows_core::CanTryInto<super::UIElement> for Glyphs {}
impl ::windows_core::CanTryInto<super::DependencyObject> for Glyphs {}
unsafe impl ::core::marker::Send for Glyphs {}
unsafe impl ::core::marker::Sync for Glyphs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct Hyperlink(::windows_core::IUnknown);
impl Hyperlink {
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
            Hyperlink,
            ::windows_core::imp::IGenericFactory,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
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
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn NavigateUri(&self) -> ::windows_core::Result<::windows::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NavigateUri)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn SetNavigateUri<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows::Foundation::Uri>,
    {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetNavigateUri)(
                ::windows_core::Interface::as_raw(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn UnderlineStyle(&self) -> ::windows_core::Result<UnderlineStyle> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).UnderlineStyle)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetUnderlineStyle(&self, value: UnderlineStyle) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetUnderlineStyle)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn XYFocusLeft(&self) -> ::windows_core::Result<super::DependencyObject> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).XYFocusLeft)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetXYFocusLeft<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetXYFocusLeft)(
                ::windows_core::Interface::as_raw(this),
                value.try_into_param()?.abi(),
            )
            .ok()
        }
    }
    pub fn XYFocusRight(&self) -> ::windows_core::Result<super::DependencyObject> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).XYFocusRight)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetXYFocusRight<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetXYFocusRight)(
                ::windows_core::Interface::as_raw(this),
                value.try_into_param()?.abi(),
            )
            .ok()
        }
    }
    pub fn XYFocusUp(&self) -> ::windows_core::Result<super::DependencyObject> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).XYFocusUp)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetXYFocusUp<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetXYFocusUp)(
                ::windows_core::Interface::as_raw(this),
                value.try_into_param()?.abi(),
            )
            .ok()
        }
    }
    pub fn XYFocusDown(&self) -> ::windows_core::Result<super::DependencyObject> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).XYFocusDown)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetXYFocusDown<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetXYFocusDown)(
                ::windows_core::Interface::as_raw(this),
                value.try_into_param()?.abi(),
            )
            .ok()
        }
    }
    pub fn ElementSoundMode(&self) -> ::windows_core::Result<super::ElementSoundMode> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ElementSoundMode)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetElementSoundMode(
        &self,
        value: super::ElementSoundMode,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetElementSoundMode)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn FocusState(&self) -> ::windows_core::Result<super::FocusState> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FocusState)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Input\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Input")]
    pub fn XYFocusUpNavigationStrategy(
        &self,
    ) -> ::windows_core::Result<super::Input::XYFocusNavigationStrategy> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).XYFocusUpNavigationStrategy)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Input\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Input")]
    pub fn SetXYFocusUpNavigationStrategy(
        &self,
        value: super::Input::XYFocusNavigationStrategy,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetXYFocusUpNavigationStrategy)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Input\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Input")]
    pub fn XYFocusDownNavigationStrategy(
        &self,
    ) -> ::windows_core::Result<super::Input::XYFocusNavigationStrategy> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).XYFocusDownNavigationStrategy)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Input\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Input")]
    pub fn SetXYFocusDownNavigationStrategy(
        &self,
        value: super::Input::XYFocusNavigationStrategy,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetXYFocusDownNavigationStrategy)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Input\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Input")]
    pub fn XYFocusLeftNavigationStrategy(
        &self,
    ) -> ::windows_core::Result<super::Input::XYFocusNavigationStrategy> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).XYFocusLeftNavigationStrategy)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Input\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Input")]
    pub fn SetXYFocusLeftNavigationStrategy(
        &self,
        value: super::Input::XYFocusNavigationStrategy,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetXYFocusLeftNavigationStrategy)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Input\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Input")]
    pub fn XYFocusRightNavigationStrategy(
        &self,
    ) -> ::windows_core::Result<super::Input::XYFocusNavigationStrategy> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).XYFocusRightNavigationStrategy)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Input\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Input")]
    pub fn SetXYFocusRightNavigationStrategy(
        &self,
        value: super::Input::XYFocusNavigationStrategy,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetXYFocusRightNavigationStrategy)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsTabStop(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsTabStop)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetIsTabStop(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetIsTabStop)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn TabIndex(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TabIndex)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetTabIndex(&self, value: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetTabIndex)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn Click<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<Hyperlink, HyperlinkClickEventArgs>,
        >,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Click)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveClick(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveClick)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn GotFocus<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::RoutedEventHandler>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GotFocus)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveGotFocus(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveGotFocus)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn LostFocus<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::RoutedEventHandler>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LostFocus)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveLostFocus(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveLostFocus)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn Focus(&self, value: super::FocusState) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Focus)(
                ::windows_core::Interface::as_raw(this),
                value,
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn NavigateUriProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IHyperlinkStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NavigateUriProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn UnderlineStyleProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IHyperlinkStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).UnderlineStyleProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn XYFocusLeftProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IHyperlinkStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).XYFocusLeftProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn XYFocusRightProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IHyperlinkStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).XYFocusRightProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn XYFocusUpProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IHyperlinkStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).XYFocusUpProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn XYFocusDownProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IHyperlinkStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).XYFocusDownProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn ElementSoundModeProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IHyperlinkStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ElementSoundModeProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn FocusStateProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IHyperlinkStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FocusStateProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn XYFocusUpNavigationStrategyProperty() -> ::windows_core::Result<super::DependencyProperty>
    {
        Self::IHyperlinkStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).XYFocusUpNavigationStrategyProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn XYFocusDownNavigationStrategyProperty(
    ) -> ::windows_core::Result<super::DependencyProperty> {
        Self::IHyperlinkStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).XYFocusDownNavigationStrategyProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn XYFocusLeftNavigationStrategyProperty(
    ) -> ::windows_core::Result<super::DependencyProperty> {
        Self::IHyperlinkStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).XYFocusLeftNavigationStrategyProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn XYFocusRightNavigationStrategyProperty(
    ) -> ::windows_core::Result<super::DependencyProperty> {
        Self::IHyperlinkStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).XYFocusRightNavigationStrategyProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn IsTabStopProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IHyperlinkStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsTabStopProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn TabIndexProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IHyperlinkStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TabIndexProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_Foundation_Collections\"`"]
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub fn Inlines(&self) -> ::windows_core::Result<InlineCollection> {
        let this = &::windows_core::ComInterface::cast::<ISpan>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Inlines)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation_Collections\"`"]
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub fn SetInlines<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<InlineCollection>,
    {
        let this = &::windows_core::ComInterface::cast::<ISpan>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetInlines)(
                ::windows_core::Interface::as_raw(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn Name(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Name)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn FontSize(&self) -> ::windows_core::Result<f64> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FontSize)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetFontSize(&self, value: f64) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetFontSize)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Media\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Media")]
    pub fn FontFamily(&self) -> ::windows_core::Result<super::Media::FontFamily> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FontFamily)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Media\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Media")]
    pub fn SetFontFamily<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::Media::FontFamily>,
    {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetFontFamily)(
                ::windows_core::Interface::as_raw(this),
                value.try_into_param()?.abi(),
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_UI_Text\"`"]
    #[cfg(feature = "Windows_UI_Text")]
    pub fn FontWeight(&self) -> ::windows_core::Result<::windows::UI::Text::FontWeight> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FontWeight)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_UI_Text\"`"]
    #[cfg(feature = "Windows_UI_Text")]
    pub fn SetFontWeight(
        &self,
        value: ::windows::UI::Text::FontWeight,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetFontWeight)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_UI_Text\"`"]
    #[cfg(feature = "Windows_UI_Text")]
    pub fn FontStyle(&self) -> ::windows_core::Result<::windows::UI::Text::FontStyle> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FontStyle)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_UI_Text\"`"]
    #[cfg(feature = "Windows_UI_Text")]
    pub fn SetFontStyle(
        &self,
        value: ::windows::UI::Text::FontStyle,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetFontStyle)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_UI_Text\"`"]
    #[cfg(feature = "Windows_UI_Text")]
    pub fn FontStretch(&self) -> ::windows_core::Result<::windows::UI::Text::FontStretch> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FontStretch)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_UI_Text\"`"]
    #[cfg(feature = "Windows_UI_Text")]
    pub fn SetFontStretch(
        &self,
        value: ::windows::UI::Text::FontStretch,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetFontStretch)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn CharacterSpacing(&self) -> ::windows_core::Result<i32> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CharacterSpacing)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetCharacterSpacing(&self, value: i32) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetCharacterSpacing)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Media\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Media")]
    pub fn Foreground(&self) -> ::windows_core::Result<super::Media::Brush> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Foreground)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Media\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Media")]
    pub fn SetForeground<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::Media::Brush>,
    {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetForeground)(
                ::windows_core::Interface::as_raw(this),
                value.try_into_param()?.abi(),
            )
            .ok()
        }
    }
    pub fn Language(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
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
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetLanguage)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn IsTextScaleFactorEnabled(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsTextScaleFactorEnabled)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetIsTextScaleFactorEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetIsTextScaleFactorEnabled)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_UI_Text\"`"]
    #[cfg(feature = "Windows_UI_Text")]
    pub fn TextDecorations(&self) -> ::windows_core::Result<::windows::UI::Text::TextDecorations> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TextDecorations)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_UI_Text\"`"]
    #[cfg(feature = "Windows_UI_Text")]
    pub fn SetTextDecorations(
        &self,
        value: ::windows::UI::Text::TextDecorations,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetTextDecorations)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn ContentStart(&self) -> ::windows_core::Result<TextPointer> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ContentStart)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn ContentEnd(&self) -> ::windows_core::Result<TextPointer> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ContentEnd)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn ElementStart(&self) -> ::windows_core::Result<TextPointer> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ElementStart)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn ElementEnd(&self) -> ::windows_core::Result<TextPointer> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ElementEnd)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn AllowFocusOnInteraction(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AllowFocusOnInteraction)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetAllowFocusOnInteraction(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetAllowFocusOnInteraction)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn AccessKey(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AccessKey)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetAccessKey(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetAccessKey)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn ExitDisplayModeOnAccessKeyInvoked(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ExitDisplayModeOnAccessKeyInvoked)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetExitDisplayModeOnAccessKeyInvoked(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetExitDisplayModeOnAccessKeyInvoked)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsAccessKeyScope(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsAccessKeyScope)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetIsAccessKeyScope(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetIsAccessKeyScope)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn AccessKeyScopeOwner(&self) -> ::windows_core::Result<super::DependencyObject> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AccessKeyScopeOwner)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetAccessKeyScopeOwner<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetAccessKeyScopeOwner)(
                ::windows_core::Interface::as_raw(this),
                value.try_into_param()?.abi(),
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Input\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Input")]
    pub fn KeyTipPlacementMode(&self) -> ::windows_core::Result<super::Input::KeyTipPlacementMode> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).KeyTipPlacementMode)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Input\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Input")]
    pub fn SetKeyTipPlacementMode(
        &self,
        value: super::Input::KeyTipPlacementMode,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetKeyTipPlacementMode)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn KeyTipHorizontalOffset(&self) -> ::windows_core::Result<f64> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).KeyTipHorizontalOffset)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetKeyTipHorizontalOffset(&self, value: f64) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetKeyTipHorizontalOffset)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn KeyTipVerticalOffset(&self) -> ::windows_core::Result<f64> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).KeyTipVerticalOffset)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetKeyTipVerticalOffset(&self, value: f64) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetKeyTipVerticalOffset)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn XamlRoot(&self) -> ::windows_core::Result<super::XamlRoot> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).XamlRoot)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetXamlRoot<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::XamlRoot>,
    {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetXamlRoot)(
                ::windows_core::Interface::as_raw(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Input\"`, `\"Windows_Foundation\"`"]
    #[cfg(all(feature = "Microsoft_UI_Xaml_Input", feature = "Windows_Foundation"))]
    pub fn AccessKeyDisplayRequested<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<
                TextElement,
                super::Input::AccessKeyDisplayRequestedEventArgs,
            >,
        >,
    {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AccessKeyDisplayRequested)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveAccessKeyDisplayRequested(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveAccessKeyDisplayRequested)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Input\"`, `\"Windows_Foundation\"`"]
    #[cfg(all(feature = "Microsoft_UI_Xaml_Input", feature = "Windows_Foundation"))]
    pub fn AccessKeyDisplayDismissed<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<
                TextElement,
                super::Input::AccessKeyDisplayDismissedEventArgs,
            >,
        >,
    {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AccessKeyDisplayDismissed)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveAccessKeyDisplayDismissed(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveAccessKeyDisplayDismissed)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Input\"`, `\"Windows_Foundation\"`"]
    #[cfg(all(feature = "Microsoft_UI_Xaml_Input", feature = "Windows_Foundation"))]
    pub fn AccessKeyInvoked<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<
                TextElement,
                super::Input::AccessKeyInvokedEventArgs,
            >,
        >,
    {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AccessKeyInvoked)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveAccessKeyInvoked(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveAccessKeyInvoked)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn FindName(
        &self,
        name: &::windows_core::HSTRING,
    ) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FindName)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(name),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn OnDisconnectVisualChildren(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElementOverrides>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).OnDisconnectVisualChildren)(
                ::windows_core::Interface::as_raw(this),
            )
            .ok()
        }
    }
    #[doc(hidden)]
    pub fn IHyperlinkStatics<R, F: FnOnce(&IHyperlinkStatics) -> ::windows_core::Result<R>>(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<Hyperlink, IHyperlinkStatics> =
            ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for Hyperlink {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for Hyperlink {
    type Vtable = IHyperlink_Vtbl;
}
unsafe impl ::windows_core::ComInterface for Hyperlink {
    const IID: ::windows_core::GUID = <IHyperlink as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for Hyperlink {
    const NAME: &'static str = "Microsoft.UI.Xaml.Documents.Hyperlink";
}
::windows_core::imp::interface_hierarchy!(
    Hyperlink,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<Span> for Hyperlink {}
impl ::windows_core::CanTryInto<Inline> for Hyperlink {}
impl ::windows_core::CanTryInto<TextElement> for Hyperlink {}
impl ::windows_core::CanTryInto<super::DependencyObject> for Hyperlink {}
unsafe impl ::core::marker::Send for Hyperlink {}
unsafe impl ::core::marker::Sync for Hyperlink {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct HyperlinkClickEventArgs(::windows_core::IUnknown);
impl HyperlinkClickEventArgs {
    pub fn OriginalSource(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = &::windows_core::ComInterface::cast::<super::IRoutedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OriginalSource)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for HyperlinkClickEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for HyperlinkClickEventArgs {
    type Vtable = IHyperlinkClickEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for HyperlinkClickEventArgs {
    const IID: ::windows_core::GUID =
        <IHyperlinkClickEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for HyperlinkClickEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Documents.HyperlinkClickEventArgs";
}
::windows_core::imp::interface_hierarchy!(
    HyperlinkClickEventArgs,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<super::RoutedEventArgs> for HyperlinkClickEventArgs {}
unsafe impl ::core::marker::Send for HyperlinkClickEventArgs {}
unsafe impl ::core::marker::Sync for HyperlinkClickEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct Inline(::windows_core::IUnknown);
impl Inline {
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
    pub fn Name(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Name)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn FontSize(&self) -> ::windows_core::Result<f64> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FontSize)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetFontSize(&self, value: f64) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetFontSize)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Media\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Media")]
    pub fn FontFamily(&self) -> ::windows_core::Result<super::Media::FontFamily> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FontFamily)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Media\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Media")]
    pub fn SetFontFamily<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::Media::FontFamily>,
    {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetFontFamily)(
                ::windows_core::Interface::as_raw(this),
                value.try_into_param()?.abi(),
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_UI_Text\"`"]
    #[cfg(feature = "Windows_UI_Text")]
    pub fn FontWeight(&self) -> ::windows_core::Result<::windows::UI::Text::FontWeight> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FontWeight)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_UI_Text\"`"]
    #[cfg(feature = "Windows_UI_Text")]
    pub fn SetFontWeight(
        &self,
        value: ::windows::UI::Text::FontWeight,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetFontWeight)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_UI_Text\"`"]
    #[cfg(feature = "Windows_UI_Text")]
    pub fn FontStyle(&self) -> ::windows_core::Result<::windows::UI::Text::FontStyle> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FontStyle)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_UI_Text\"`"]
    #[cfg(feature = "Windows_UI_Text")]
    pub fn SetFontStyle(
        &self,
        value: ::windows::UI::Text::FontStyle,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetFontStyle)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_UI_Text\"`"]
    #[cfg(feature = "Windows_UI_Text")]
    pub fn FontStretch(&self) -> ::windows_core::Result<::windows::UI::Text::FontStretch> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FontStretch)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_UI_Text\"`"]
    #[cfg(feature = "Windows_UI_Text")]
    pub fn SetFontStretch(
        &self,
        value: ::windows::UI::Text::FontStretch,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetFontStretch)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn CharacterSpacing(&self) -> ::windows_core::Result<i32> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CharacterSpacing)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetCharacterSpacing(&self, value: i32) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetCharacterSpacing)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Media\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Media")]
    pub fn Foreground(&self) -> ::windows_core::Result<super::Media::Brush> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Foreground)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Media\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Media")]
    pub fn SetForeground<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::Media::Brush>,
    {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetForeground)(
                ::windows_core::Interface::as_raw(this),
                value.try_into_param()?.abi(),
            )
            .ok()
        }
    }
    pub fn Language(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
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
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetLanguage)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn IsTextScaleFactorEnabled(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsTextScaleFactorEnabled)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetIsTextScaleFactorEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetIsTextScaleFactorEnabled)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_UI_Text\"`"]
    #[cfg(feature = "Windows_UI_Text")]
    pub fn TextDecorations(&self) -> ::windows_core::Result<::windows::UI::Text::TextDecorations> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TextDecorations)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_UI_Text\"`"]
    #[cfg(feature = "Windows_UI_Text")]
    pub fn SetTextDecorations(
        &self,
        value: ::windows::UI::Text::TextDecorations,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetTextDecorations)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn ContentStart(&self) -> ::windows_core::Result<TextPointer> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ContentStart)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn ContentEnd(&self) -> ::windows_core::Result<TextPointer> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ContentEnd)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn ElementStart(&self) -> ::windows_core::Result<TextPointer> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ElementStart)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn ElementEnd(&self) -> ::windows_core::Result<TextPointer> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ElementEnd)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn AllowFocusOnInteraction(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AllowFocusOnInteraction)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetAllowFocusOnInteraction(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetAllowFocusOnInteraction)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn AccessKey(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AccessKey)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetAccessKey(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetAccessKey)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn ExitDisplayModeOnAccessKeyInvoked(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ExitDisplayModeOnAccessKeyInvoked)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetExitDisplayModeOnAccessKeyInvoked(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetExitDisplayModeOnAccessKeyInvoked)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsAccessKeyScope(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsAccessKeyScope)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetIsAccessKeyScope(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetIsAccessKeyScope)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn AccessKeyScopeOwner(&self) -> ::windows_core::Result<super::DependencyObject> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AccessKeyScopeOwner)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetAccessKeyScopeOwner<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetAccessKeyScopeOwner)(
                ::windows_core::Interface::as_raw(this),
                value.try_into_param()?.abi(),
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Input\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Input")]
    pub fn KeyTipPlacementMode(&self) -> ::windows_core::Result<super::Input::KeyTipPlacementMode> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).KeyTipPlacementMode)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Input\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Input")]
    pub fn SetKeyTipPlacementMode(
        &self,
        value: super::Input::KeyTipPlacementMode,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetKeyTipPlacementMode)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn KeyTipHorizontalOffset(&self) -> ::windows_core::Result<f64> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).KeyTipHorizontalOffset)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetKeyTipHorizontalOffset(&self, value: f64) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetKeyTipHorizontalOffset)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn KeyTipVerticalOffset(&self) -> ::windows_core::Result<f64> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).KeyTipVerticalOffset)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetKeyTipVerticalOffset(&self, value: f64) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetKeyTipVerticalOffset)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn XamlRoot(&self) -> ::windows_core::Result<super::XamlRoot> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).XamlRoot)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetXamlRoot<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::XamlRoot>,
    {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetXamlRoot)(
                ::windows_core::Interface::as_raw(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Input\"`, `\"Windows_Foundation\"`"]
    #[cfg(all(feature = "Microsoft_UI_Xaml_Input", feature = "Windows_Foundation"))]
    pub fn AccessKeyDisplayRequested<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<
                TextElement,
                super::Input::AccessKeyDisplayRequestedEventArgs,
            >,
        >,
    {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AccessKeyDisplayRequested)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveAccessKeyDisplayRequested(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveAccessKeyDisplayRequested)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Input\"`, `\"Windows_Foundation\"`"]
    #[cfg(all(feature = "Microsoft_UI_Xaml_Input", feature = "Windows_Foundation"))]
    pub fn AccessKeyDisplayDismissed<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<
                TextElement,
                super::Input::AccessKeyDisplayDismissedEventArgs,
            >,
        >,
    {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AccessKeyDisplayDismissed)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveAccessKeyDisplayDismissed(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveAccessKeyDisplayDismissed)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Input\"`, `\"Windows_Foundation\"`"]
    #[cfg(all(feature = "Microsoft_UI_Xaml_Input", feature = "Windows_Foundation"))]
    pub fn AccessKeyInvoked<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<
                TextElement,
                super::Input::AccessKeyInvokedEventArgs,
            >,
        >,
    {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AccessKeyInvoked)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveAccessKeyInvoked(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveAccessKeyInvoked)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn FindName(
        &self,
        name: &::windows_core::HSTRING,
    ) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FindName)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(name),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn OnDisconnectVisualChildren(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElementOverrides>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).OnDisconnectVisualChildren)(
                ::windows_core::Interface::as_raw(this),
            )
            .ok()
        }
    }
}
impl ::windows_core::RuntimeType for Inline {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for Inline {
    type Vtable = IInline_Vtbl;
}
unsafe impl ::windows_core::ComInterface for Inline {
    const IID: ::windows_core::GUID = <IInline as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for Inline {
    const NAME: &'static str = "Microsoft.UI.Xaml.Documents.Inline";
}
::windows_core::imp::interface_hierarchy!(
    Inline,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<TextElement> for Inline {}
impl ::windows_core::CanTryInto<super::DependencyObject> for Inline {}
unsafe impl ::core::marker::Send for Inline {}
unsafe impl ::core::marker::Sync for Inline {}
#[doc = "Required features: `\"Windows_Foundation_Collections\"`"]
#[cfg(feature = "Windows_Foundation_Collections")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct InlineCollection(::windows_core::IUnknown);
#[cfg(feature = "Windows_Foundation_Collections")]
impl InlineCollection {
    #[doc = "Required features: `\"Windows_Foundation_Collections\"`"]
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub fn First(
        &self,
    ) -> ::windows_core::Result<::windows::Foundation::Collections::IIterator<Inline>> {
        let this = &::windows_core::ComInterface::cast::<
            ::windows::Foundation::Collections::IIterable<Inline>,
        >(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).First)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation_Collections\"`"]
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub fn GetAt(&self, index: u32) -> ::windows_core::Result<Inline> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetAt)(
                ::windows_core::Interface::as_raw(this),
                index,
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation_Collections\"`"]
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub fn Size(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Size)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation_Collections\"`"]
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub fn GetView(
        &self,
    ) -> ::windows_core::Result<::windows::Foundation::Collections::IVectorView<Inline>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetView)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation_Collections\"`"]
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub fn IndexOf<P0>(&self, value: P0, index: &mut u32) -> ::windows_core::Result<bool>
    where
        P0: ::windows_core::TryIntoParam<Inline>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IndexOf)(
                ::windows_core::Interface::as_raw(this),
                value.try_into_param()?.abi(),
                index,
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation_Collections\"`"]
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub fn SetAt<P0>(&self, index: u32, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<Inline>,
    {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetAt)(
                ::windows_core::Interface::as_raw(this),
                index,
                value.try_into_param()?.abi(),
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation_Collections\"`"]
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub fn InsertAt<P0>(&self, index: u32, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<Inline>,
    {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).InsertAt)(
                ::windows_core::Interface::as_raw(this),
                index,
                value.try_into_param()?.abi(),
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation_Collections\"`"]
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub fn RemoveAt(&self, index: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveAt)(
                ::windows_core::Interface::as_raw(this),
                index,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation_Collections\"`"]
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub fn Append<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<Inline>,
    {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).Append)(
                ::windows_core::Interface::as_raw(this),
                value.try_into_param()?.abi(),
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation_Collections\"`"]
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub fn RemoveAtEnd(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveAtEnd)(
                ::windows_core::Interface::as_raw(this),
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation_Collections\"`"]
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub fn Clear(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).Clear)(::windows_core::Interface::as_raw(this))
                .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation_Collections\"`"]
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub fn GetMany(
        &self,
        startindex: u32,
        items: &mut [::core::option::Option<Inline>],
    ) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetMany)(
                ::windows_core::Interface::as_raw(this),
                startindex,
                items.len().try_into().unwrap(),
                ::core::mem::transmute_copy(&items),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation_Collections\"`"]
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub fn ReplaceAll(
        &self,
        items: &[::core::option::Option<Inline>],
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).ReplaceAll)(
                ::windows_core::Interface::as_raw(this),
                items.len().try_into().unwrap(),
                ::core::mem::transmute(items.as_ptr()),
            )
            .ok()
        }
    }
}
#[cfg(feature = "Windows_Foundation_Collections")]
impl ::windows_core::RuntimeType for InlineCollection {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
#[cfg(feature = "Windows_Foundation_Collections")]
unsafe impl ::windows_core::Interface for InlineCollection {
    type Vtable = ::windows::Foundation::Collections::IVector_Vtbl<Inline>;
}
#[cfg(feature = "Windows_Foundation_Collections")]
unsafe impl ::windows_core::ComInterface for InlineCollection {
    const IID: ::windows_core::GUID =
        <::windows::Foundation::Collections::IVector<Inline> as ::windows_core::ComInterface>::IID;
}
#[cfg(feature = "Windows_Foundation_Collections")]
impl ::windows_core::RuntimeName for InlineCollection {
    const NAME: &'static str = "Microsoft.UI.Xaml.Documents.InlineCollection";
}
#[cfg(feature = "Windows_Foundation_Collections")]
impl ::core::iter::IntoIterator for InlineCollection {
    type Item = Inline;
    type IntoIter = ::windows::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Windows_Foundation_Collections")]
impl ::core::iter::IntoIterator for &InlineCollection {
    type Item = Inline;
    type IntoIter = ::windows::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::windows::Foundation::Collections::VectorIterator::new(
            ::windows_core::ComInterface::cast(self).ok(),
        )
    }
}
#[cfg(feature = "Windows_Foundation_Collections")]
::windows_core::imp::interface_hierarchy!(
    InlineCollection,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
#[cfg(feature = "Windows_Foundation_Collections")]
impl ::windows_core::CanTryInto<::windows::Foundation::Collections::IIterable<Inline>>
    for InlineCollection
{
}
#[cfg(feature = "Windows_Foundation_Collections")]
impl ::windows_core::CanTryInto<::windows::Foundation::Collections::IVector<Inline>>
    for InlineCollection
{
}
#[cfg(feature = "Windows_Foundation_Collections")]
unsafe impl ::core::marker::Send for InlineCollection {}
#[cfg(feature = "Windows_Foundation_Collections")]
unsafe impl ::core::marker::Sync for InlineCollection {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct InlineUIContainer(::windows_core::IUnknown);
impl InlineUIContainer {
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
            InlineUIContainer,
            ::windows_core::imp::IGenericFactory,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
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
    pub fn Child(&self) -> ::windows_core::Result<super::UIElement> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Child)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetChild<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::UIElement>,
    {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetChild)(
                ::windows_core::Interface::as_raw(this),
                value.try_into_param()?.abi(),
            )
            .ok()
        }
    }
    pub fn Name(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Name)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn FontSize(&self) -> ::windows_core::Result<f64> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FontSize)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetFontSize(&self, value: f64) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetFontSize)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Media\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Media")]
    pub fn FontFamily(&self) -> ::windows_core::Result<super::Media::FontFamily> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FontFamily)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Media\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Media")]
    pub fn SetFontFamily<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::Media::FontFamily>,
    {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetFontFamily)(
                ::windows_core::Interface::as_raw(this),
                value.try_into_param()?.abi(),
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_UI_Text\"`"]
    #[cfg(feature = "Windows_UI_Text")]
    pub fn FontWeight(&self) -> ::windows_core::Result<::windows::UI::Text::FontWeight> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FontWeight)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_UI_Text\"`"]
    #[cfg(feature = "Windows_UI_Text")]
    pub fn SetFontWeight(
        &self,
        value: ::windows::UI::Text::FontWeight,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetFontWeight)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_UI_Text\"`"]
    #[cfg(feature = "Windows_UI_Text")]
    pub fn FontStyle(&self) -> ::windows_core::Result<::windows::UI::Text::FontStyle> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FontStyle)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_UI_Text\"`"]
    #[cfg(feature = "Windows_UI_Text")]
    pub fn SetFontStyle(
        &self,
        value: ::windows::UI::Text::FontStyle,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetFontStyle)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_UI_Text\"`"]
    #[cfg(feature = "Windows_UI_Text")]
    pub fn FontStretch(&self) -> ::windows_core::Result<::windows::UI::Text::FontStretch> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FontStretch)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_UI_Text\"`"]
    #[cfg(feature = "Windows_UI_Text")]
    pub fn SetFontStretch(
        &self,
        value: ::windows::UI::Text::FontStretch,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetFontStretch)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn CharacterSpacing(&self) -> ::windows_core::Result<i32> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CharacterSpacing)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetCharacterSpacing(&self, value: i32) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetCharacterSpacing)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Media\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Media")]
    pub fn Foreground(&self) -> ::windows_core::Result<super::Media::Brush> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Foreground)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Media\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Media")]
    pub fn SetForeground<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::Media::Brush>,
    {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetForeground)(
                ::windows_core::Interface::as_raw(this),
                value.try_into_param()?.abi(),
            )
            .ok()
        }
    }
    pub fn Language(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
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
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetLanguage)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn IsTextScaleFactorEnabled(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsTextScaleFactorEnabled)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetIsTextScaleFactorEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetIsTextScaleFactorEnabled)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_UI_Text\"`"]
    #[cfg(feature = "Windows_UI_Text")]
    pub fn TextDecorations(&self) -> ::windows_core::Result<::windows::UI::Text::TextDecorations> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TextDecorations)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_UI_Text\"`"]
    #[cfg(feature = "Windows_UI_Text")]
    pub fn SetTextDecorations(
        &self,
        value: ::windows::UI::Text::TextDecorations,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetTextDecorations)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn ContentStart(&self) -> ::windows_core::Result<TextPointer> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ContentStart)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn ContentEnd(&self) -> ::windows_core::Result<TextPointer> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ContentEnd)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn ElementStart(&self) -> ::windows_core::Result<TextPointer> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ElementStart)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn ElementEnd(&self) -> ::windows_core::Result<TextPointer> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ElementEnd)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn AllowFocusOnInteraction(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AllowFocusOnInteraction)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetAllowFocusOnInteraction(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetAllowFocusOnInteraction)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn AccessKey(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AccessKey)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetAccessKey(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetAccessKey)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn ExitDisplayModeOnAccessKeyInvoked(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ExitDisplayModeOnAccessKeyInvoked)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetExitDisplayModeOnAccessKeyInvoked(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetExitDisplayModeOnAccessKeyInvoked)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsAccessKeyScope(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsAccessKeyScope)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetIsAccessKeyScope(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetIsAccessKeyScope)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn AccessKeyScopeOwner(&self) -> ::windows_core::Result<super::DependencyObject> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AccessKeyScopeOwner)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetAccessKeyScopeOwner<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetAccessKeyScopeOwner)(
                ::windows_core::Interface::as_raw(this),
                value.try_into_param()?.abi(),
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Input\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Input")]
    pub fn KeyTipPlacementMode(&self) -> ::windows_core::Result<super::Input::KeyTipPlacementMode> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).KeyTipPlacementMode)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Input\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Input")]
    pub fn SetKeyTipPlacementMode(
        &self,
        value: super::Input::KeyTipPlacementMode,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetKeyTipPlacementMode)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn KeyTipHorizontalOffset(&self) -> ::windows_core::Result<f64> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).KeyTipHorizontalOffset)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetKeyTipHorizontalOffset(&self, value: f64) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetKeyTipHorizontalOffset)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn KeyTipVerticalOffset(&self) -> ::windows_core::Result<f64> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).KeyTipVerticalOffset)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetKeyTipVerticalOffset(&self, value: f64) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetKeyTipVerticalOffset)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn XamlRoot(&self) -> ::windows_core::Result<super::XamlRoot> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).XamlRoot)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetXamlRoot<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::XamlRoot>,
    {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetXamlRoot)(
                ::windows_core::Interface::as_raw(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Input\"`, `\"Windows_Foundation\"`"]
    #[cfg(all(feature = "Microsoft_UI_Xaml_Input", feature = "Windows_Foundation"))]
    pub fn AccessKeyDisplayRequested<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<
                TextElement,
                super::Input::AccessKeyDisplayRequestedEventArgs,
            >,
        >,
    {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AccessKeyDisplayRequested)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveAccessKeyDisplayRequested(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveAccessKeyDisplayRequested)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Input\"`, `\"Windows_Foundation\"`"]
    #[cfg(all(feature = "Microsoft_UI_Xaml_Input", feature = "Windows_Foundation"))]
    pub fn AccessKeyDisplayDismissed<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<
                TextElement,
                super::Input::AccessKeyDisplayDismissedEventArgs,
            >,
        >,
    {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AccessKeyDisplayDismissed)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveAccessKeyDisplayDismissed(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveAccessKeyDisplayDismissed)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Input\"`, `\"Windows_Foundation\"`"]
    #[cfg(all(feature = "Microsoft_UI_Xaml_Input", feature = "Windows_Foundation"))]
    pub fn AccessKeyInvoked<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<
                TextElement,
                super::Input::AccessKeyInvokedEventArgs,
            >,
        >,
    {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AccessKeyInvoked)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveAccessKeyInvoked(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveAccessKeyInvoked)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn FindName(
        &self,
        name: &::windows_core::HSTRING,
    ) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FindName)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(name),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn OnDisconnectVisualChildren(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElementOverrides>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).OnDisconnectVisualChildren)(
                ::windows_core::Interface::as_raw(this),
            )
            .ok()
        }
    }
}
impl ::windows_core::RuntimeType for InlineUIContainer {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for InlineUIContainer {
    type Vtable = IInlineUIContainer_Vtbl;
}
unsafe impl ::windows_core::ComInterface for InlineUIContainer {
    const IID: ::windows_core::GUID = <IInlineUIContainer as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for InlineUIContainer {
    const NAME: &'static str = "Microsoft.UI.Xaml.Documents.InlineUIContainer";
}
::windows_core::imp::interface_hierarchy!(
    InlineUIContainer,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<Inline> for InlineUIContainer {}
impl ::windows_core::CanTryInto<TextElement> for InlineUIContainer {}
impl ::windows_core::CanTryInto<super::DependencyObject> for InlineUIContainer {}
unsafe impl ::core::marker::Send for InlineUIContainer {}
unsafe impl ::core::marker::Sync for InlineUIContainer {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct Italic(::windows_core::IUnknown);
impl Italic {
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
            Italic,
            ::windows_core::imp::IGenericFactory,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
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
    #[doc = "Required features: `\"Windows_Foundation_Collections\"`"]
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub fn Inlines(&self) -> ::windows_core::Result<InlineCollection> {
        let this = &::windows_core::ComInterface::cast::<ISpan>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Inlines)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation_Collections\"`"]
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub fn SetInlines<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<InlineCollection>,
    {
        let this = &::windows_core::ComInterface::cast::<ISpan>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetInlines)(
                ::windows_core::Interface::as_raw(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn Name(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Name)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn FontSize(&self) -> ::windows_core::Result<f64> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FontSize)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetFontSize(&self, value: f64) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetFontSize)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Media\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Media")]
    pub fn FontFamily(&self) -> ::windows_core::Result<super::Media::FontFamily> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FontFamily)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Media\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Media")]
    pub fn SetFontFamily<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::Media::FontFamily>,
    {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetFontFamily)(
                ::windows_core::Interface::as_raw(this),
                value.try_into_param()?.abi(),
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_UI_Text\"`"]
    #[cfg(feature = "Windows_UI_Text")]
    pub fn FontWeight(&self) -> ::windows_core::Result<::windows::UI::Text::FontWeight> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FontWeight)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_UI_Text\"`"]
    #[cfg(feature = "Windows_UI_Text")]
    pub fn SetFontWeight(
        &self,
        value: ::windows::UI::Text::FontWeight,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetFontWeight)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_UI_Text\"`"]
    #[cfg(feature = "Windows_UI_Text")]
    pub fn FontStyle(&self) -> ::windows_core::Result<::windows::UI::Text::FontStyle> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FontStyle)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_UI_Text\"`"]
    #[cfg(feature = "Windows_UI_Text")]
    pub fn SetFontStyle(
        &self,
        value: ::windows::UI::Text::FontStyle,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetFontStyle)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_UI_Text\"`"]
    #[cfg(feature = "Windows_UI_Text")]
    pub fn FontStretch(&self) -> ::windows_core::Result<::windows::UI::Text::FontStretch> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FontStretch)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_UI_Text\"`"]
    #[cfg(feature = "Windows_UI_Text")]
    pub fn SetFontStretch(
        &self,
        value: ::windows::UI::Text::FontStretch,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetFontStretch)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn CharacterSpacing(&self) -> ::windows_core::Result<i32> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CharacterSpacing)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetCharacterSpacing(&self, value: i32) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetCharacterSpacing)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Media\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Media")]
    pub fn Foreground(&self) -> ::windows_core::Result<super::Media::Brush> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Foreground)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Media\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Media")]
    pub fn SetForeground<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::Media::Brush>,
    {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetForeground)(
                ::windows_core::Interface::as_raw(this),
                value.try_into_param()?.abi(),
            )
            .ok()
        }
    }
    pub fn Language(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
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
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetLanguage)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn IsTextScaleFactorEnabled(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsTextScaleFactorEnabled)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetIsTextScaleFactorEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetIsTextScaleFactorEnabled)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_UI_Text\"`"]
    #[cfg(feature = "Windows_UI_Text")]
    pub fn TextDecorations(&self) -> ::windows_core::Result<::windows::UI::Text::TextDecorations> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TextDecorations)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_UI_Text\"`"]
    #[cfg(feature = "Windows_UI_Text")]
    pub fn SetTextDecorations(
        &self,
        value: ::windows::UI::Text::TextDecorations,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetTextDecorations)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn ContentStart(&self) -> ::windows_core::Result<TextPointer> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ContentStart)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn ContentEnd(&self) -> ::windows_core::Result<TextPointer> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ContentEnd)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn ElementStart(&self) -> ::windows_core::Result<TextPointer> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ElementStart)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn ElementEnd(&self) -> ::windows_core::Result<TextPointer> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ElementEnd)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn AllowFocusOnInteraction(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AllowFocusOnInteraction)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetAllowFocusOnInteraction(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetAllowFocusOnInteraction)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn AccessKey(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AccessKey)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetAccessKey(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetAccessKey)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn ExitDisplayModeOnAccessKeyInvoked(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ExitDisplayModeOnAccessKeyInvoked)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetExitDisplayModeOnAccessKeyInvoked(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetExitDisplayModeOnAccessKeyInvoked)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsAccessKeyScope(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsAccessKeyScope)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetIsAccessKeyScope(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetIsAccessKeyScope)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn AccessKeyScopeOwner(&self) -> ::windows_core::Result<super::DependencyObject> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AccessKeyScopeOwner)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetAccessKeyScopeOwner<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetAccessKeyScopeOwner)(
                ::windows_core::Interface::as_raw(this),
                value.try_into_param()?.abi(),
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Input\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Input")]
    pub fn KeyTipPlacementMode(&self) -> ::windows_core::Result<super::Input::KeyTipPlacementMode> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).KeyTipPlacementMode)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Input\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Input")]
    pub fn SetKeyTipPlacementMode(
        &self,
        value: super::Input::KeyTipPlacementMode,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetKeyTipPlacementMode)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn KeyTipHorizontalOffset(&self) -> ::windows_core::Result<f64> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).KeyTipHorizontalOffset)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetKeyTipHorizontalOffset(&self, value: f64) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetKeyTipHorizontalOffset)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn KeyTipVerticalOffset(&self) -> ::windows_core::Result<f64> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).KeyTipVerticalOffset)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetKeyTipVerticalOffset(&self, value: f64) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetKeyTipVerticalOffset)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn XamlRoot(&self) -> ::windows_core::Result<super::XamlRoot> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).XamlRoot)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetXamlRoot<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::XamlRoot>,
    {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetXamlRoot)(
                ::windows_core::Interface::as_raw(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Input\"`, `\"Windows_Foundation\"`"]
    #[cfg(all(feature = "Microsoft_UI_Xaml_Input", feature = "Windows_Foundation"))]
    pub fn AccessKeyDisplayRequested<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<
                TextElement,
                super::Input::AccessKeyDisplayRequestedEventArgs,
            >,
        >,
    {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AccessKeyDisplayRequested)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveAccessKeyDisplayRequested(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveAccessKeyDisplayRequested)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Input\"`, `\"Windows_Foundation\"`"]
    #[cfg(all(feature = "Microsoft_UI_Xaml_Input", feature = "Windows_Foundation"))]
    pub fn AccessKeyDisplayDismissed<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<
                TextElement,
                super::Input::AccessKeyDisplayDismissedEventArgs,
            >,
        >,
    {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AccessKeyDisplayDismissed)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveAccessKeyDisplayDismissed(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveAccessKeyDisplayDismissed)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Input\"`, `\"Windows_Foundation\"`"]
    #[cfg(all(feature = "Microsoft_UI_Xaml_Input", feature = "Windows_Foundation"))]
    pub fn AccessKeyInvoked<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<
                TextElement,
                super::Input::AccessKeyInvokedEventArgs,
            >,
        >,
    {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AccessKeyInvoked)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveAccessKeyInvoked(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveAccessKeyInvoked)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn FindName(
        &self,
        name: &::windows_core::HSTRING,
    ) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FindName)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(name),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn OnDisconnectVisualChildren(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElementOverrides>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).OnDisconnectVisualChildren)(
                ::windows_core::Interface::as_raw(this),
            )
            .ok()
        }
    }
}
impl ::windows_core::RuntimeType for Italic {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for Italic {
    type Vtable = IItalic_Vtbl;
}
unsafe impl ::windows_core::ComInterface for Italic {
    const IID: ::windows_core::GUID = <IItalic as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for Italic {
    const NAME: &'static str = "Microsoft.UI.Xaml.Documents.Italic";
}
::windows_core::imp::interface_hierarchy!(
    Italic,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<Span> for Italic {}
impl ::windows_core::CanTryInto<Inline> for Italic {}
impl ::windows_core::CanTryInto<TextElement> for Italic {}
impl ::windows_core::CanTryInto<super::DependencyObject> for Italic {}
unsafe impl ::core::marker::Send for Italic {}
unsafe impl ::core::marker::Sync for Italic {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct LineBreak(::windows_core::IUnknown);
impl LineBreak {
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
            LineBreak,
            ::windows_core::imp::IGenericFactory,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
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
    pub fn Name(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Name)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn FontSize(&self) -> ::windows_core::Result<f64> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FontSize)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetFontSize(&self, value: f64) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetFontSize)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Media\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Media")]
    pub fn FontFamily(&self) -> ::windows_core::Result<super::Media::FontFamily> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FontFamily)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Media\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Media")]
    pub fn SetFontFamily<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::Media::FontFamily>,
    {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetFontFamily)(
                ::windows_core::Interface::as_raw(this),
                value.try_into_param()?.abi(),
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_UI_Text\"`"]
    #[cfg(feature = "Windows_UI_Text")]
    pub fn FontWeight(&self) -> ::windows_core::Result<::windows::UI::Text::FontWeight> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FontWeight)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_UI_Text\"`"]
    #[cfg(feature = "Windows_UI_Text")]
    pub fn SetFontWeight(
        &self,
        value: ::windows::UI::Text::FontWeight,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetFontWeight)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_UI_Text\"`"]
    #[cfg(feature = "Windows_UI_Text")]
    pub fn FontStyle(&self) -> ::windows_core::Result<::windows::UI::Text::FontStyle> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FontStyle)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_UI_Text\"`"]
    #[cfg(feature = "Windows_UI_Text")]
    pub fn SetFontStyle(
        &self,
        value: ::windows::UI::Text::FontStyle,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetFontStyle)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_UI_Text\"`"]
    #[cfg(feature = "Windows_UI_Text")]
    pub fn FontStretch(&self) -> ::windows_core::Result<::windows::UI::Text::FontStretch> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FontStretch)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_UI_Text\"`"]
    #[cfg(feature = "Windows_UI_Text")]
    pub fn SetFontStretch(
        &self,
        value: ::windows::UI::Text::FontStretch,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetFontStretch)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn CharacterSpacing(&self) -> ::windows_core::Result<i32> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CharacterSpacing)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetCharacterSpacing(&self, value: i32) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetCharacterSpacing)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Media\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Media")]
    pub fn Foreground(&self) -> ::windows_core::Result<super::Media::Brush> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Foreground)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Media\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Media")]
    pub fn SetForeground<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::Media::Brush>,
    {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetForeground)(
                ::windows_core::Interface::as_raw(this),
                value.try_into_param()?.abi(),
            )
            .ok()
        }
    }
    pub fn Language(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
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
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetLanguage)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn IsTextScaleFactorEnabled(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsTextScaleFactorEnabled)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetIsTextScaleFactorEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetIsTextScaleFactorEnabled)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_UI_Text\"`"]
    #[cfg(feature = "Windows_UI_Text")]
    pub fn TextDecorations(&self) -> ::windows_core::Result<::windows::UI::Text::TextDecorations> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TextDecorations)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_UI_Text\"`"]
    #[cfg(feature = "Windows_UI_Text")]
    pub fn SetTextDecorations(
        &self,
        value: ::windows::UI::Text::TextDecorations,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetTextDecorations)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn ContentStart(&self) -> ::windows_core::Result<TextPointer> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ContentStart)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn ContentEnd(&self) -> ::windows_core::Result<TextPointer> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ContentEnd)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn ElementStart(&self) -> ::windows_core::Result<TextPointer> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ElementStart)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn ElementEnd(&self) -> ::windows_core::Result<TextPointer> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ElementEnd)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn AllowFocusOnInteraction(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AllowFocusOnInteraction)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetAllowFocusOnInteraction(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetAllowFocusOnInteraction)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn AccessKey(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AccessKey)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetAccessKey(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetAccessKey)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn ExitDisplayModeOnAccessKeyInvoked(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ExitDisplayModeOnAccessKeyInvoked)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetExitDisplayModeOnAccessKeyInvoked(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetExitDisplayModeOnAccessKeyInvoked)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsAccessKeyScope(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsAccessKeyScope)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetIsAccessKeyScope(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetIsAccessKeyScope)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn AccessKeyScopeOwner(&self) -> ::windows_core::Result<super::DependencyObject> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AccessKeyScopeOwner)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetAccessKeyScopeOwner<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetAccessKeyScopeOwner)(
                ::windows_core::Interface::as_raw(this),
                value.try_into_param()?.abi(),
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Input\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Input")]
    pub fn KeyTipPlacementMode(&self) -> ::windows_core::Result<super::Input::KeyTipPlacementMode> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).KeyTipPlacementMode)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Input\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Input")]
    pub fn SetKeyTipPlacementMode(
        &self,
        value: super::Input::KeyTipPlacementMode,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetKeyTipPlacementMode)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn KeyTipHorizontalOffset(&self) -> ::windows_core::Result<f64> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).KeyTipHorizontalOffset)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetKeyTipHorizontalOffset(&self, value: f64) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetKeyTipHorizontalOffset)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn KeyTipVerticalOffset(&self) -> ::windows_core::Result<f64> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).KeyTipVerticalOffset)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetKeyTipVerticalOffset(&self, value: f64) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetKeyTipVerticalOffset)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn XamlRoot(&self) -> ::windows_core::Result<super::XamlRoot> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).XamlRoot)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetXamlRoot<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::XamlRoot>,
    {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetXamlRoot)(
                ::windows_core::Interface::as_raw(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Input\"`, `\"Windows_Foundation\"`"]
    #[cfg(all(feature = "Microsoft_UI_Xaml_Input", feature = "Windows_Foundation"))]
    pub fn AccessKeyDisplayRequested<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<
                TextElement,
                super::Input::AccessKeyDisplayRequestedEventArgs,
            >,
        >,
    {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AccessKeyDisplayRequested)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveAccessKeyDisplayRequested(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveAccessKeyDisplayRequested)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Input\"`, `\"Windows_Foundation\"`"]
    #[cfg(all(feature = "Microsoft_UI_Xaml_Input", feature = "Windows_Foundation"))]
    pub fn AccessKeyDisplayDismissed<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<
                TextElement,
                super::Input::AccessKeyDisplayDismissedEventArgs,
            >,
        >,
    {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AccessKeyDisplayDismissed)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveAccessKeyDisplayDismissed(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveAccessKeyDisplayDismissed)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Input\"`, `\"Windows_Foundation\"`"]
    #[cfg(all(feature = "Microsoft_UI_Xaml_Input", feature = "Windows_Foundation"))]
    pub fn AccessKeyInvoked<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<
                TextElement,
                super::Input::AccessKeyInvokedEventArgs,
            >,
        >,
    {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AccessKeyInvoked)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveAccessKeyInvoked(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveAccessKeyInvoked)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn FindName(
        &self,
        name: &::windows_core::HSTRING,
    ) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FindName)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(name),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn OnDisconnectVisualChildren(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElementOverrides>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).OnDisconnectVisualChildren)(
                ::windows_core::Interface::as_raw(this),
            )
            .ok()
        }
    }
}
impl ::windows_core::RuntimeType for LineBreak {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for LineBreak {
    type Vtable = ILineBreak_Vtbl;
}
unsafe impl ::windows_core::ComInterface for LineBreak {
    const IID: ::windows_core::GUID = <ILineBreak as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for LineBreak {
    const NAME: &'static str = "Microsoft.UI.Xaml.Documents.LineBreak";
}
::windows_core::imp::interface_hierarchy!(
    LineBreak,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<Inline> for LineBreak {}
impl ::windows_core::CanTryInto<TextElement> for LineBreak {}
impl ::windows_core::CanTryInto<super::DependencyObject> for LineBreak {}
unsafe impl ::core::marker::Send for LineBreak {}
unsafe impl ::core::marker::Sync for LineBreak {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct Paragraph(::windows_core::IUnknown);
impl Paragraph {
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
            Paragraph,
            ::windows_core::imp::IGenericFactory,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn TextAlignment(&self) -> ::windows_core::Result<super::TextAlignment> {
        let this = &::windows_core::ComInterface::cast::<IBlock>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TextAlignment)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetTextAlignment(&self, value: super::TextAlignment) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IBlock>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetTextAlignment)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn HorizontalTextAlignment(&self) -> ::windows_core::Result<super::TextAlignment> {
        let this = &::windows_core::ComInterface::cast::<IBlock>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HorizontalTextAlignment)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetHorizontalTextAlignment(
        &self,
        value: super::TextAlignment,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IBlock>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetHorizontalTextAlignment)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn LineHeight(&self) -> ::windows_core::Result<f64> {
        let this = &::windows_core::ComInterface::cast::<IBlock>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LineHeight)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetLineHeight(&self, value: f64) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IBlock>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetLineHeight)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn LineStackingStrategy(&self) -> ::windows_core::Result<super::LineStackingStrategy> {
        let this = &::windows_core::ComInterface::cast::<IBlock>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LineStackingStrategy)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetLineStackingStrategy(
        &self,
        value: super::LineStackingStrategy,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IBlock>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetLineStackingStrategy)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn Margin(&self) -> ::windows_core::Result<super::Thickness> {
        let this = &::windows_core::ComInterface::cast::<IBlock>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Margin)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetMargin(&self, value: super::Thickness) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IBlock>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetMargin)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
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
    #[doc = "Required features: `\"Windows_Foundation_Collections\"`"]
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub fn Inlines(&self) -> ::windows_core::Result<InlineCollection> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Inlines)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn TextIndent(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TextIndent)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetTextIndent(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetTextIndent)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn TextIndentProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IParagraphStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TextIndentProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn Name(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Name)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn FontSize(&self) -> ::windows_core::Result<f64> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FontSize)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetFontSize(&self, value: f64) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetFontSize)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Media\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Media")]
    pub fn FontFamily(&self) -> ::windows_core::Result<super::Media::FontFamily> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FontFamily)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Media\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Media")]
    pub fn SetFontFamily<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::Media::FontFamily>,
    {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetFontFamily)(
                ::windows_core::Interface::as_raw(this),
                value.try_into_param()?.abi(),
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_UI_Text\"`"]
    #[cfg(feature = "Windows_UI_Text")]
    pub fn FontWeight(&self) -> ::windows_core::Result<::windows::UI::Text::FontWeight> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FontWeight)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_UI_Text\"`"]
    #[cfg(feature = "Windows_UI_Text")]
    pub fn SetFontWeight(
        &self,
        value: ::windows::UI::Text::FontWeight,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetFontWeight)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_UI_Text\"`"]
    #[cfg(feature = "Windows_UI_Text")]
    pub fn FontStyle(&self) -> ::windows_core::Result<::windows::UI::Text::FontStyle> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FontStyle)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_UI_Text\"`"]
    #[cfg(feature = "Windows_UI_Text")]
    pub fn SetFontStyle(
        &self,
        value: ::windows::UI::Text::FontStyle,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetFontStyle)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_UI_Text\"`"]
    #[cfg(feature = "Windows_UI_Text")]
    pub fn FontStretch(&self) -> ::windows_core::Result<::windows::UI::Text::FontStretch> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FontStretch)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_UI_Text\"`"]
    #[cfg(feature = "Windows_UI_Text")]
    pub fn SetFontStretch(
        &self,
        value: ::windows::UI::Text::FontStretch,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetFontStretch)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn CharacterSpacing(&self) -> ::windows_core::Result<i32> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CharacterSpacing)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetCharacterSpacing(&self, value: i32) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetCharacterSpacing)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Media\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Media")]
    pub fn Foreground(&self) -> ::windows_core::Result<super::Media::Brush> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Foreground)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Media\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Media")]
    pub fn SetForeground<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::Media::Brush>,
    {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetForeground)(
                ::windows_core::Interface::as_raw(this),
                value.try_into_param()?.abi(),
            )
            .ok()
        }
    }
    pub fn Language(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
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
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetLanguage)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn IsTextScaleFactorEnabled(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsTextScaleFactorEnabled)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetIsTextScaleFactorEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetIsTextScaleFactorEnabled)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_UI_Text\"`"]
    #[cfg(feature = "Windows_UI_Text")]
    pub fn TextDecorations(&self) -> ::windows_core::Result<::windows::UI::Text::TextDecorations> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TextDecorations)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_UI_Text\"`"]
    #[cfg(feature = "Windows_UI_Text")]
    pub fn SetTextDecorations(
        &self,
        value: ::windows::UI::Text::TextDecorations,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetTextDecorations)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn ContentStart(&self) -> ::windows_core::Result<TextPointer> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ContentStart)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn ContentEnd(&self) -> ::windows_core::Result<TextPointer> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ContentEnd)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn ElementStart(&self) -> ::windows_core::Result<TextPointer> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ElementStart)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn ElementEnd(&self) -> ::windows_core::Result<TextPointer> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ElementEnd)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn AllowFocusOnInteraction(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AllowFocusOnInteraction)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetAllowFocusOnInteraction(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetAllowFocusOnInteraction)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn AccessKey(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AccessKey)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetAccessKey(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetAccessKey)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn ExitDisplayModeOnAccessKeyInvoked(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ExitDisplayModeOnAccessKeyInvoked)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetExitDisplayModeOnAccessKeyInvoked(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetExitDisplayModeOnAccessKeyInvoked)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsAccessKeyScope(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsAccessKeyScope)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetIsAccessKeyScope(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetIsAccessKeyScope)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn AccessKeyScopeOwner(&self) -> ::windows_core::Result<super::DependencyObject> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AccessKeyScopeOwner)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetAccessKeyScopeOwner<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetAccessKeyScopeOwner)(
                ::windows_core::Interface::as_raw(this),
                value.try_into_param()?.abi(),
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Input\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Input")]
    pub fn KeyTipPlacementMode(&self) -> ::windows_core::Result<super::Input::KeyTipPlacementMode> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).KeyTipPlacementMode)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Input\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Input")]
    pub fn SetKeyTipPlacementMode(
        &self,
        value: super::Input::KeyTipPlacementMode,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetKeyTipPlacementMode)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn KeyTipHorizontalOffset(&self) -> ::windows_core::Result<f64> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).KeyTipHorizontalOffset)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetKeyTipHorizontalOffset(&self, value: f64) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetKeyTipHorizontalOffset)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn KeyTipVerticalOffset(&self) -> ::windows_core::Result<f64> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).KeyTipVerticalOffset)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetKeyTipVerticalOffset(&self, value: f64) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetKeyTipVerticalOffset)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn XamlRoot(&self) -> ::windows_core::Result<super::XamlRoot> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).XamlRoot)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetXamlRoot<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::XamlRoot>,
    {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetXamlRoot)(
                ::windows_core::Interface::as_raw(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Input\"`, `\"Windows_Foundation\"`"]
    #[cfg(all(feature = "Microsoft_UI_Xaml_Input", feature = "Windows_Foundation"))]
    pub fn AccessKeyDisplayRequested<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<
                TextElement,
                super::Input::AccessKeyDisplayRequestedEventArgs,
            >,
        >,
    {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AccessKeyDisplayRequested)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveAccessKeyDisplayRequested(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveAccessKeyDisplayRequested)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Input\"`, `\"Windows_Foundation\"`"]
    #[cfg(all(feature = "Microsoft_UI_Xaml_Input", feature = "Windows_Foundation"))]
    pub fn AccessKeyDisplayDismissed<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<
                TextElement,
                super::Input::AccessKeyDisplayDismissedEventArgs,
            >,
        >,
    {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AccessKeyDisplayDismissed)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveAccessKeyDisplayDismissed(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveAccessKeyDisplayDismissed)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Input\"`, `\"Windows_Foundation\"`"]
    #[cfg(all(feature = "Microsoft_UI_Xaml_Input", feature = "Windows_Foundation"))]
    pub fn AccessKeyInvoked<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<
                TextElement,
                super::Input::AccessKeyInvokedEventArgs,
            >,
        >,
    {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AccessKeyInvoked)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveAccessKeyInvoked(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveAccessKeyInvoked)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn FindName(
        &self,
        name: &::windows_core::HSTRING,
    ) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FindName)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(name),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn OnDisconnectVisualChildren(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElementOverrides>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).OnDisconnectVisualChildren)(
                ::windows_core::Interface::as_raw(this),
            )
            .ok()
        }
    }
    #[doc(hidden)]
    pub fn IParagraphStatics<R, F: FnOnce(&IParagraphStatics) -> ::windows_core::Result<R>>(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<Paragraph, IParagraphStatics> =
            ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for Paragraph {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for Paragraph {
    type Vtable = IParagraph_Vtbl;
}
unsafe impl ::windows_core::ComInterface for Paragraph {
    const IID: ::windows_core::GUID = <IParagraph as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for Paragraph {
    const NAME: &'static str = "Microsoft.UI.Xaml.Documents.Paragraph";
}
::windows_core::imp::interface_hierarchy!(
    Paragraph,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<Block> for Paragraph {}
impl ::windows_core::CanTryInto<TextElement> for Paragraph {}
impl ::windows_core::CanTryInto<super::DependencyObject> for Paragraph {}
unsafe impl ::core::marker::Send for Paragraph {}
unsafe impl ::core::marker::Sync for Paragraph {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct Run(::windows_core::IUnknown);
impl Run {
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
            Run,
            ::windows_core::imp::IGenericFactory,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
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
    pub fn Text(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Text)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetText(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetText)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn FlowDirection(&self) -> ::windows_core::Result<super::FlowDirection> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FlowDirection)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetFlowDirection(&self, value: super::FlowDirection) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetFlowDirection)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn FlowDirectionProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IRunStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FlowDirectionProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn Name(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Name)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn FontSize(&self) -> ::windows_core::Result<f64> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FontSize)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetFontSize(&self, value: f64) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetFontSize)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Media\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Media")]
    pub fn FontFamily(&self) -> ::windows_core::Result<super::Media::FontFamily> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FontFamily)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Media\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Media")]
    pub fn SetFontFamily<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::Media::FontFamily>,
    {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetFontFamily)(
                ::windows_core::Interface::as_raw(this),
                value.try_into_param()?.abi(),
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_UI_Text\"`"]
    #[cfg(feature = "Windows_UI_Text")]
    pub fn FontWeight(&self) -> ::windows_core::Result<::windows::UI::Text::FontWeight> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FontWeight)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_UI_Text\"`"]
    #[cfg(feature = "Windows_UI_Text")]
    pub fn SetFontWeight(
        &self,
        value: ::windows::UI::Text::FontWeight,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetFontWeight)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_UI_Text\"`"]
    #[cfg(feature = "Windows_UI_Text")]
    pub fn FontStyle(&self) -> ::windows_core::Result<::windows::UI::Text::FontStyle> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FontStyle)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_UI_Text\"`"]
    #[cfg(feature = "Windows_UI_Text")]
    pub fn SetFontStyle(
        &self,
        value: ::windows::UI::Text::FontStyle,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetFontStyle)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_UI_Text\"`"]
    #[cfg(feature = "Windows_UI_Text")]
    pub fn FontStretch(&self) -> ::windows_core::Result<::windows::UI::Text::FontStretch> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FontStretch)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_UI_Text\"`"]
    #[cfg(feature = "Windows_UI_Text")]
    pub fn SetFontStretch(
        &self,
        value: ::windows::UI::Text::FontStretch,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetFontStretch)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn CharacterSpacing(&self) -> ::windows_core::Result<i32> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CharacterSpacing)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetCharacterSpacing(&self, value: i32) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetCharacterSpacing)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Media\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Media")]
    pub fn Foreground(&self) -> ::windows_core::Result<super::Media::Brush> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Foreground)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Media\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Media")]
    pub fn SetForeground<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::Media::Brush>,
    {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetForeground)(
                ::windows_core::Interface::as_raw(this),
                value.try_into_param()?.abi(),
            )
            .ok()
        }
    }
    pub fn Language(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
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
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetLanguage)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn IsTextScaleFactorEnabled(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsTextScaleFactorEnabled)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetIsTextScaleFactorEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetIsTextScaleFactorEnabled)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_UI_Text\"`"]
    #[cfg(feature = "Windows_UI_Text")]
    pub fn TextDecorations(&self) -> ::windows_core::Result<::windows::UI::Text::TextDecorations> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TextDecorations)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_UI_Text\"`"]
    #[cfg(feature = "Windows_UI_Text")]
    pub fn SetTextDecorations(
        &self,
        value: ::windows::UI::Text::TextDecorations,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetTextDecorations)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn ContentStart(&self) -> ::windows_core::Result<TextPointer> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ContentStart)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn ContentEnd(&self) -> ::windows_core::Result<TextPointer> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ContentEnd)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn ElementStart(&self) -> ::windows_core::Result<TextPointer> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ElementStart)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn ElementEnd(&self) -> ::windows_core::Result<TextPointer> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ElementEnd)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn AllowFocusOnInteraction(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AllowFocusOnInteraction)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetAllowFocusOnInteraction(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetAllowFocusOnInteraction)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn AccessKey(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AccessKey)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetAccessKey(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetAccessKey)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn ExitDisplayModeOnAccessKeyInvoked(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ExitDisplayModeOnAccessKeyInvoked)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetExitDisplayModeOnAccessKeyInvoked(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetExitDisplayModeOnAccessKeyInvoked)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsAccessKeyScope(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsAccessKeyScope)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetIsAccessKeyScope(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetIsAccessKeyScope)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn AccessKeyScopeOwner(&self) -> ::windows_core::Result<super::DependencyObject> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AccessKeyScopeOwner)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetAccessKeyScopeOwner<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetAccessKeyScopeOwner)(
                ::windows_core::Interface::as_raw(this),
                value.try_into_param()?.abi(),
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Input\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Input")]
    pub fn KeyTipPlacementMode(&self) -> ::windows_core::Result<super::Input::KeyTipPlacementMode> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).KeyTipPlacementMode)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Input\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Input")]
    pub fn SetKeyTipPlacementMode(
        &self,
        value: super::Input::KeyTipPlacementMode,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetKeyTipPlacementMode)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn KeyTipHorizontalOffset(&self) -> ::windows_core::Result<f64> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).KeyTipHorizontalOffset)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetKeyTipHorizontalOffset(&self, value: f64) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetKeyTipHorizontalOffset)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn KeyTipVerticalOffset(&self) -> ::windows_core::Result<f64> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).KeyTipVerticalOffset)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetKeyTipVerticalOffset(&self, value: f64) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetKeyTipVerticalOffset)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn XamlRoot(&self) -> ::windows_core::Result<super::XamlRoot> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).XamlRoot)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetXamlRoot<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::XamlRoot>,
    {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetXamlRoot)(
                ::windows_core::Interface::as_raw(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Input\"`, `\"Windows_Foundation\"`"]
    #[cfg(all(feature = "Microsoft_UI_Xaml_Input", feature = "Windows_Foundation"))]
    pub fn AccessKeyDisplayRequested<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<
                TextElement,
                super::Input::AccessKeyDisplayRequestedEventArgs,
            >,
        >,
    {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AccessKeyDisplayRequested)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveAccessKeyDisplayRequested(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveAccessKeyDisplayRequested)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Input\"`, `\"Windows_Foundation\"`"]
    #[cfg(all(feature = "Microsoft_UI_Xaml_Input", feature = "Windows_Foundation"))]
    pub fn AccessKeyDisplayDismissed<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<
                TextElement,
                super::Input::AccessKeyDisplayDismissedEventArgs,
            >,
        >,
    {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AccessKeyDisplayDismissed)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveAccessKeyDisplayDismissed(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveAccessKeyDisplayDismissed)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Input\"`, `\"Windows_Foundation\"`"]
    #[cfg(all(feature = "Microsoft_UI_Xaml_Input", feature = "Windows_Foundation"))]
    pub fn AccessKeyInvoked<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<
                TextElement,
                super::Input::AccessKeyInvokedEventArgs,
            >,
        >,
    {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AccessKeyInvoked)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveAccessKeyInvoked(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveAccessKeyInvoked)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn FindName(
        &self,
        name: &::windows_core::HSTRING,
    ) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FindName)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(name),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn OnDisconnectVisualChildren(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElementOverrides>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).OnDisconnectVisualChildren)(
                ::windows_core::Interface::as_raw(this),
            )
            .ok()
        }
    }
    #[doc(hidden)]
    pub fn IRunStatics<R, F: FnOnce(&IRunStatics) -> ::windows_core::Result<R>>(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<Run, IRunStatics> =
            ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for Run {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for Run {
    type Vtable = IRun_Vtbl;
}
unsafe impl ::windows_core::ComInterface for Run {
    const IID: ::windows_core::GUID = <IRun as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for Run {
    const NAME: &'static str = "Microsoft.UI.Xaml.Documents.Run";
}
::windows_core::imp::interface_hierarchy!(
    Run,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<Inline> for Run {}
impl ::windows_core::CanTryInto<TextElement> for Run {}
impl ::windows_core::CanTryInto<super::DependencyObject> for Run {}
unsafe impl ::core::marker::Send for Run {}
unsafe impl ::core::marker::Sync for Run {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct Span(::windows_core::IUnknown);
impl Span {
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
    #[doc = "Required features: `\"Windows_Foundation_Collections\"`"]
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub fn Inlines(&self) -> ::windows_core::Result<InlineCollection> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Inlines)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation_Collections\"`"]
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub fn SetInlines<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<InlineCollection>,
    {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetInlines)(
                ::windows_core::Interface::as_raw(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn Name(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Name)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn FontSize(&self) -> ::windows_core::Result<f64> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FontSize)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetFontSize(&self, value: f64) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetFontSize)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Media\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Media")]
    pub fn FontFamily(&self) -> ::windows_core::Result<super::Media::FontFamily> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FontFamily)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Media\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Media")]
    pub fn SetFontFamily<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::Media::FontFamily>,
    {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetFontFamily)(
                ::windows_core::Interface::as_raw(this),
                value.try_into_param()?.abi(),
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_UI_Text\"`"]
    #[cfg(feature = "Windows_UI_Text")]
    pub fn FontWeight(&self) -> ::windows_core::Result<::windows::UI::Text::FontWeight> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FontWeight)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_UI_Text\"`"]
    #[cfg(feature = "Windows_UI_Text")]
    pub fn SetFontWeight(
        &self,
        value: ::windows::UI::Text::FontWeight,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetFontWeight)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_UI_Text\"`"]
    #[cfg(feature = "Windows_UI_Text")]
    pub fn FontStyle(&self) -> ::windows_core::Result<::windows::UI::Text::FontStyle> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FontStyle)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_UI_Text\"`"]
    #[cfg(feature = "Windows_UI_Text")]
    pub fn SetFontStyle(
        &self,
        value: ::windows::UI::Text::FontStyle,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetFontStyle)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_UI_Text\"`"]
    #[cfg(feature = "Windows_UI_Text")]
    pub fn FontStretch(&self) -> ::windows_core::Result<::windows::UI::Text::FontStretch> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FontStretch)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_UI_Text\"`"]
    #[cfg(feature = "Windows_UI_Text")]
    pub fn SetFontStretch(
        &self,
        value: ::windows::UI::Text::FontStretch,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetFontStretch)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn CharacterSpacing(&self) -> ::windows_core::Result<i32> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CharacterSpacing)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetCharacterSpacing(&self, value: i32) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetCharacterSpacing)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Media\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Media")]
    pub fn Foreground(&self) -> ::windows_core::Result<super::Media::Brush> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Foreground)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Media\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Media")]
    pub fn SetForeground<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::Media::Brush>,
    {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetForeground)(
                ::windows_core::Interface::as_raw(this),
                value.try_into_param()?.abi(),
            )
            .ok()
        }
    }
    pub fn Language(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
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
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetLanguage)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn IsTextScaleFactorEnabled(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsTextScaleFactorEnabled)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetIsTextScaleFactorEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetIsTextScaleFactorEnabled)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_UI_Text\"`"]
    #[cfg(feature = "Windows_UI_Text")]
    pub fn TextDecorations(&self) -> ::windows_core::Result<::windows::UI::Text::TextDecorations> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TextDecorations)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_UI_Text\"`"]
    #[cfg(feature = "Windows_UI_Text")]
    pub fn SetTextDecorations(
        &self,
        value: ::windows::UI::Text::TextDecorations,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetTextDecorations)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn ContentStart(&self) -> ::windows_core::Result<TextPointer> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ContentStart)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn ContentEnd(&self) -> ::windows_core::Result<TextPointer> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ContentEnd)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn ElementStart(&self) -> ::windows_core::Result<TextPointer> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ElementStart)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn ElementEnd(&self) -> ::windows_core::Result<TextPointer> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ElementEnd)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn AllowFocusOnInteraction(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AllowFocusOnInteraction)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetAllowFocusOnInteraction(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetAllowFocusOnInteraction)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn AccessKey(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AccessKey)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetAccessKey(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetAccessKey)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn ExitDisplayModeOnAccessKeyInvoked(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ExitDisplayModeOnAccessKeyInvoked)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetExitDisplayModeOnAccessKeyInvoked(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetExitDisplayModeOnAccessKeyInvoked)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsAccessKeyScope(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsAccessKeyScope)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetIsAccessKeyScope(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetIsAccessKeyScope)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn AccessKeyScopeOwner(&self) -> ::windows_core::Result<super::DependencyObject> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AccessKeyScopeOwner)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetAccessKeyScopeOwner<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetAccessKeyScopeOwner)(
                ::windows_core::Interface::as_raw(this),
                value.try_into_param()?.abi(),
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Input\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Input")]
    pub fn KeyTipPlacementMode(&self) -> ::windows_core::Result<super::Input::KeyTipPlacementMode> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).KeyTipPlacementMode)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Input\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Input")]
    pub fn SetKeyTipPlacementMode(
        &self,
        value: super::Input::KeyTipPlacementMode,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetKeyTipPlacementMode)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn KeyTipHorizontalOffset(&self) -> ::windows_core::Result<f64> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).KeyTipHorizontalOffset)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetKeyTipHorizontalOffset(&self, value: f64) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetKeyTipHorizontalOffset)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn KeyTipVerticalOffset(&self) -> ::windows_core::Result<f64> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).KeyTipVerticalOffset)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetKeyTipVerticalOffset(&self, value: f64) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetKeyTipVerticalOffset)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn XamlRoot(&self) -> ::windows_core::Result<super::XamlRoot> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).XamlRoot)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetXamlRoot<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::XamlRoot>,
    {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetXamlRoot)(
                ::windows_core::Interface::as_raw(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Input\"`, `\"Windows_Foundation\"`"]
    #[cfg(all(feature = "Microsoft_UI_Xaml_Input", feature = "Windows_Foundation"))]
    pub fn AccessKeyDisplayRequested<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<
                TextElement,
                super::Input::AccessKeyDisplayRequestedEventArgs,
            >,
        >,
    {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AccessKeyDisplayRequested)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveAccessKeyDisplayRequested(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveAccessKeyDisplayRequested)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Input\"`, `\"Windows_Foundation\"`"]
    #[cfg(all(feature = "Microsoft_UI_Xaml_Input", feature = "Windows_Foundation"))]
    pub fn AccessKeyDisplayDismissed<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<
                TextElement,
                super::Input::AccessKeyDisplayDismissedEventArgs,
            >,
        >,
    {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AccessKeyDisplayDismissed)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveAccessKeyDisplayDismissed(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveAccessKeyDisplayDismissed)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Input\"`, `\"Windows_Foundation\"`"]
    #[cfg(all(feature = "Microsoft_UI_Xaml_Input", feature = "Windows_Foundation"))]
    pub fn AccessKeyInvoked<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<
                TextElement,
                super::Input::AccessKeyInvokedEventArgs,
            >,
        >,
    {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AccessKeyInvoked)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveAccessKeyInvoked(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveAccessKeyInvoked)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn FindName(
        &self,
        name: &::windows_core::HSTRING,
    ) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FindName)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(name),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn OnDisconnectVisualChildren(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElementOverrides>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).OnDisconnectVisualChildren)(
                ::windows_core::Interface::as_raw(this),
            )
            .ok()
        }
    }
}
impl ::windows_core::RuntimeType for Span {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for Span {
    type Vtable = ISpan_Vtbl;
}
unsafe impl ::windows_core::ComInterface for Span {
    const IID: ::windows_core::GUID = <ISpan as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for Span {
    const NAME: &'static str = "Microsoft.UI.Xaml.Documents.Span";
}
::windows_core::imp::interface_hierarchy!(
    Span,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<Inline> for Span {}
impl ::windows_core::CanTryInto<TextElement> for Span {}
impl ::windows_core::CanTryInto<super::DependencyObject> for Span {}
unsafe impl ::core::marker::Send for Span {}
unsafe impl ::core::marker::Sync for Span {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct TextElement(::windows_core::IUnknown);
impl TextElement {
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
    pub fn Name(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Name)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn FontSize(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FontSize)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetFontSize(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetFontSize)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Media\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Media")]
    pub fn FontFamily(&self) -> ::windows_core::Result<super::Media::FontFamily> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FontFamily)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Media\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Media")]
    pub fn SetFontFamily<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::Media::FontFamily>,
    {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetFontFamily)(
                ::windows_core::Interface::as_raw(this),
                value.try_into_param()?.abi(),
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_UI_Text\"`"]
    #[cfg(feature = "Windows_UI_Text")]
    pub fn FontWeight(&self) -> ::windows_core::Result<::windows::UI::Text::FontWeight> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FontWeight)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_UI_Text\"`"]
    #[cfg(feature = "Windows_UI_Text")]
    pub fn SetFontWeight(
        &self,
        value: ::windows::UI::Text::FontWeight,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetFontWeight)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_UI_Text\"`"]
    #[cfg(feature = "Windows_UI_Text")]
    pub fn FontStyle(&self) -> ::windows_core::Result<::windows::UI::Text::FontStyle> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FontStyle)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_UI_Text\"`"]
    #[cfg(feature = "Windows_UI_Text")]
    pub fn SetFontStyle(
        &self,
        value: ::windows::UI::Text::FontStyle,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetFontStyle)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_UI_Text\"`"]
    #[cfg(feature = "Windows_UI_Text")]
    pub fn FontStretch(&self) -> ::windows_core::Result<::windows::UI::Text::FontStretch> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FontStretch)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_UI_Text\"`"]
    #[cfg(feature = "Windows_UI_Text")]
    pub fn SetFontStretch(
        &self,
        value: ::windows::UI::Text::FontStretch,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetFontStretch)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn CharacterSpacing(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CharacterSpacing)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetCharacterSpacing(&self, value: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetCharacterSpacing)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Media\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Media")]
    pub fn Foreground(&self) -> ::windows_core::Result<super::Media::Brush> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Foreground)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Media\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Media")]
    pub fn SetForeground<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::Media::Brush>,
    {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetForeground)(
                ::windows_core::Interface::as_raw(this),
                value.try_into_param()?.abi(),
            )
            .ok()
        }
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
    pub fn IsTextScaleFactorEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsTextScaleFactorEnabled)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetIsTextScaleFactorEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetIsTextScaleFactorEnabled)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_UI_Text\"`"]
    #[cfg(feature = "Windows_UI_Text")]
    pub fn TextDecorations(&self) -> ::windows_core::Result<::windows::UI::Text::TextDecorations> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TextDecorations)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_UI_Text\"`"]
    #[cfg(feature = "Windows_UI_Text")]
    pub fn SetTextDecorations(
        &self,
        value: ::windows::UI::Text::TextDecorations,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetTextDecorations)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn ContentStart(&self) -> ::windows_core::Result<TextPointer> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ContentStart)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn ContentEnd(&self) -> ::windows_core::Result<TextPointer> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ContentEnd)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn ElementStart(&self) -> ::windows_core::Result<TextPointer> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ElementStart)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn ElementEnd(&self) -> ::windows_core::Result<TextPointer> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ElementEnd)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn AllowFocusOnInteraction(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AllowFocusOnInteraction)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetAllowFocusOnInteraction(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetAllowFocusOnInteraction)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn AccessKey(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AccessKey)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetAccessKey(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetAccessKey)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn ExitDisplayModeOnAccessKeyInvoked(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ExitDisplayModeOnAccessKeyInvoked)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetExitDisplayModeOnAccessKeyInvoked(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetExitDisplayModeOnAccessKeyInvoked)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsAccessKeyScope(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsAccessKeyScope)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetIsAccessKeyScope(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetIsAccessKeyScope)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn AccessKeyScopeOwner(&self) -> ::windows_core::Result<super::DependencyObject> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AccessKeyScopeOwner)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetAccessKeyScopeOwner<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetAccessKeyScopeOwner)(
                ::windows_core::Interface::as_raw(this),
                value.try_into_param()?.abi(),
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Input\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Input")]
    pub fn KeyTipPlacementMode(&self) -> ::windows_core::Result<super::Input::KeyTipPlacementMode> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).KeyTipPlacementMode)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Input\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Input")]
    pub fn SetKeyTipPlacementMode(
        &self,
        value: super::Input::KeyTipPlacementMode,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetKeyTipPlacementMode)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn KeyTipHorizontalOffset(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).KeyTipHorizontalOffset)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetKeyTipHorizontalOffset(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetKeyTipHorizontalOffset)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn KeyTipVerticalOffset(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).KeyTipVerticalOffset)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetKeyTipVerticalOffset(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetKeyTipVerticalOffset)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn XamlRoot(&self) -> ::windows_core::Result<super::XamlRoot> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).XamlRoot)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetXamlRoot<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::XamlRoot>,
    {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetXamlRoot)(
                ::windows_core::Interface::as_raw(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Input\"`, `\"Windows_Foundation\"`"]
    #[cfg(all(feature = "Microsoft_UI_Xaml_Input", feature = "Windows_Foundation"))]
    pub fn AccessKeyDisplayRequested<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<
                TextElement,
                super::Input::AccessKeyDisplayRequestedEventArgs,
            >,
        >,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AccessKeyDisplayRequested)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveAccessKeyDisplayRequested(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveAccessKeyDisplayRequested)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Input\"`, `\"Windows_Foundation\"`"]
    #[cfg(all(feature = "Microsoft_UI_Xaml_Input", feature = "Windows_Foundation"))]
    pub fn AccessKeyDisplayDismissed<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<
                TextElement,
                super::Input::AccessKeyDisplayDismissedEventArgs,
            >,
        >,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AccessKeyDisplayDismissed)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveAccessKeyDisplayDismissed(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveAccessKeyDisplayDismissed)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Input\"`, `\"Windows_Foundation\"`"]
    #[cfg(all(feature = "Microsoft_UI_Xaml_Input", feature = "Windows_Foundation"))]
    pub fn AccessKeyInvoked<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<
                TextElement,
                super::Input::AccessKeyInvokedEventArgs,
            >,
        >,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AccessKeyInvoked)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveAccessKeyInvoked(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveAccessKeyInvoked)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn FindName(
        &self,
        name: &::windows_core::HSTRING,
    ) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FindName)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(name),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn OnDisconnectVisualChildren(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElementOverrides>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).OnDisconnectVisualChildren)(
                ::windows_core::Interface::as_raw(this),
            )
            .ok()
        }
    }
    pub fn FontSizeProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::ITextElementStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FontSizeProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn FontFamilyProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::ITextElementStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FontFamilyProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn FontWeightProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::ITextElementStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FontWeightProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn FontStyleProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::ITextElementStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FontStyleProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn FontStretchProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::ITextElementStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FontStretchProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn CharacterSpacingProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::ITextElementStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CharacterSpacingProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn ForegroundProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::ITextElementStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ForegroundProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn LanguageProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::ITextElementStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LanguageProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn IsTextScaleFactorEnabledProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::ITextElementStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsTextScaleFactorEnabledProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn TextDecorationsProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::ITextElementStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TextDecorationsProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn AllowFocusOnInteractionProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::ITextElementStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AllowFocusOnInteractionProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn AccessKeyProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::ITextElementStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AccessKeyProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn ExitDisplayModeOnAccessKeyInvokedProperty(
    ) -> ::windows_core::Result<super::DependencyProperty> {
        Self::ITextElementStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ExitDisplayModeOnAccessKeyInvokedProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn IsAccessKeyScopeProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::ITextElementStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsAccessKeyScopeProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn AccessKeyScopeOwnerProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::ITextElementStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AccessKeyScopeOwnerProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn KeyTipPlacementModeProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::ITextElementStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).KeyTipPlacementModeProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn KeyTipHorizontalOffsetProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::ITextElementStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).KeyTipHorizontalOffsetProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn KeyTipVerticalOffsetProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::ITextElementStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).KeyTipVerticalOffsetProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ITextElementStatics<R, F: FnOnce(&ITextElementStatics) -> ::windows_core::Result<R>>(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<TextElement, ITextElementStatics> =
            ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for TextElement {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for TextElement {
    type Vtable = ITextElement_Vtbl;
}
unsafe impl ::windows_core::ComInterface for TextElement {
    const IID: ::windows_core::GUID = <ITextElement as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for TextElement {
    const NAME: &'static str = "Microsoft.UI.Xaml.Documents.TextElement";
}
::windows_core::imp::interface_hierarchy!(
    TextElement,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<super::DependencyObject> for TextElement {}
unsafe impl ::core::marker::Send for TextElement {}
unsafe impl ::core::marker::Sync for TextElement {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct TextHighlighter(::windows_core::IUnknown);
impl TextHighlighter {
    #[doc = "Required features: `\"Windows_Foundation_Collections\"`"]
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub fn Ranges(
        &self,
    ) -> ::windows_core::Result<::windows::Foundation::Collections::IVector<TextRange>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Ranges)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Media\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Media")]
    pub fn Foreground(&self) -> ::windows_core::Result<super::Media::Brush> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Foreground)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Media\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Media")]
    pub fn SetForeground<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::Media::Brush>,
    {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetForeground)(
                ::windows_core::Interface::as_raw(this),
                value.try_into_param()?.abi(),
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Media\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Media")]
    pub fn Background(&self) -> ::windows_core::Result<super::Media::Brush> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Background)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Media\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Media")]
    pub fn SetBackground<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::Media::Brush>,
    {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetBackground)(
                ::windows_core::Interface::as_raw(this),
                value.try_into_param()?.abi(),
            )
            .ok()
        }
    }
    pub fn ForegroundProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::ITextHighlighterStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ForegroundProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn BackgroundProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::ITextHighlighterStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).BackgroundProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ITextHighlighterStatics<
        R,
        F: FnOnce(&ITextHighlighterStatics) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<TextHighlighter, ITextHighlighterStatics> =
            ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for TextHighlighter {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for TextHighlighter {
    type Vtable = ITextHighlighter_Vtbl;
}
unsafe impl ::windows_core::ComInterface for TextHighlighter {
    const IID: ::windows_core::GUID = <ITextHighlighter as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for TextHighlighter {
    const NAME: &'static str = "Microsoft.UI.Xaml.Documents.TextHighlighter";
}
::windows_core::imp::interface_hierarchy!(
    TextHighlighter,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for TextHighlighter {}
unsafe impl ::core::marker::Sync for TextHighlighter {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct TextHighlighterBase(::windows_core::IUnknown);
impl TextHighlighterBase {
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
}
impl ::windows_core::RuntimeType for TextHighlighterBase {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for TextHighlighterBase {
    type Vtable = ITextHighlighterBase_Vtbl;
}
unsafe impl ::windows_core::ComInterface for TextHighlighterBase {
    const IID: ::windows_core::GUID = <ITextHighlighterBase as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for TextHighlighterBase {
    const NAME: &'static str = "Microsoft.UI.Xaml.Documents.TextHighlighterBase";
}
::windows_core::imp::interface_hierarchy!(
    TextHighlighterBase,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<super::DependencyObject> for TextHighlighterBase {}
unsafe impl ::core::marker::Send for TextHighlighterBase {}
unsafe impl ::core::marker::Sync for TextHighlighterBase {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct TextPointer(::windows_core::IUnknown);
impl TextPointer {
    pub fn Parent(&self) -> ::windows_core::Result<super::DependencyObject> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Parent)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn VisualParent(&self) -> ::windows_core::Result<super::FrameworkElement> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).VisualParent)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn LogicalDirection(&self) -> ::windows_core::Result<LogicalDirection> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LogicalDirection)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn Offset(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Offset)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn GetCharacterRect(
        &self,
        direction: LogicalDirection,
    ) -> ::windows_core::Result<::windows::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetCharacterRect)(
                ::windows_core::Interface::as_raw(this),
                direction,
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn GetPositionAtOffset(
        &self,
        offset: i32,
        direction: LogicalDirection,
    ) -> ::windows_core::Result<TextPointer> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetPositionAtOffset)(
                ::windows_core::Interface::as_raw(this),
                offset,
                direction,
                &mut result__,
            )
            .from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for TextPointer {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for TextPointer {
    type Vtable = ITextPointer_Vtbl;
}
unsafe impl ::windows_core::ComInterface for TextPointer {
    const IID: ::windows_core::GUID = <ITextPointer as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for TextPointer {
    const NAME: &'static str = "Microsoft.UI.Xaml.Documents.TextPointer";
}
::windows_core::imp::interface_hierarchy!(
    TextPointer,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for TextPointer {}
unsafe impl ::core::marker::Sync for TextPointer {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct Typography(::windows_core::IUnknown);
impl Typography {
    pub fn AnnotationAlternatesProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AnnotationAlternatesProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn GetAnnotationAlternates<P0>(element: P0) -> ::windows_core::Result<i32>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetAnnotationAlternates)(
                ::windows_core::Interface::as_raw(this),
                element.try_into_param()?.abi(),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn SetAnnotationAlternates<P0>(element: P0, value: i32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        Self::ITypographyStatics(|this| unsafe {
            (::windows_core::Interface::vtable(this).SetAnnotationAlternates)(
                ::windows_core::Interface::as_raw(this),
                element.try_into_param()?.abi(),
                value,
            )
            .ok()
        })
    }
    pub fn EastAsianExpertFormsProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).EastAsianExpertFormsProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn GetEastAsianExpertForms<P0>(element: P0) -> ::windows_core::Result<bool>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetEastAsianExpertForms)(
                ::windows_core::Interface::as_raw(this),
                element.try_into_param()?.abi(),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn SetEastAsianExpertForms<P0>(element: P0, value: bool) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        Self::ITypographyStatics(|this| unsafe {
            (::windows_core::Interface::vtable(this).SetEastAsianExpertForms)(
                ::windows_core::Interface::as_raw(this),
                element.try_into_param()?.abi(),
                value,
            )
            .ok()
        })
    }
    pub fn EastAsianLanguageProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).EastAsianLanguageProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn GetEastAsianLanguage<P0>(
        element: P0,
    ) -> ::windows_core::Result<super::FontEastAsianLanguage>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetEastAsianLanguage)(
                ::windows_core::Interface::as_raw(this),
                element.try_into_param()?.abi(),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn SetEastAsianLanguage<P0>(
        element: P0,
        value: super::FontEastAsianLanguage,
    ) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        Self::ITypographyStatics(|this| unsafe {
            (::windows_core::Interface::vtable(this).SetEastAsianLanguage)(
                ::windows_core::Interface::as_raw(this),
                element.try_into_param()?.abi(),
                value,
            )
            .ok()
        })
    }
    pub fn EastAsianWidthsProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).EastAsianWidthsProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn GetEastAsianWidths<P0>(element: P0) -> ::windows_core::Result<super::FontEastAsianWidths>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetEastAsianWidths)(
                ::windows_core::Interface::as_raw(this),
                element.try_into_param()?.abi(),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn SetEastAsianWidths<P0>(
        element: P0,
        value: super::FontEastAsianWidths,
    ) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        Self::ITypographyStatics(|this| unsafe {
            (::windows_core::Interface::vtable(this).SetEastAsianWidths)(
                ::windows_core::Interface::as_raw(this),
                element.try_into_param()?.abi(),
                value,
            )
            .ok()
        })
    }
    pub fn StandardLigaturesProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StandardLigaturesProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn GetStandardLigatures<P0>(element: P0) -> ::windows_core::Result<bool>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetStandardLigatures)(
                ::windows_core::Interface::as_raw(this),
                element.try_into_param()?.abi(),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn SetStandardLigatures<P0>(element: P0, value: bool) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        Self::ITypographyStatics(|this| unsafe {
            (::windows_core::Interface::vtable(this).SetStandardLigatures)(
                ::windows_core::Interface::as_raw(this),
                element.try_into_param()?.abi(),
                value,
            )
            .ok()
        })
    }
    pub fn ContextualLigaturesProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ContextualLigaturesProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn GetContextualLigatures<P0>(element: P0) -> ::windows_core::Result<bool>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetContextualLigatures)(
                ::windows_core::Interface::as_raw(this),
                element.try_into_param()?.abi(),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn SetContextualLigatures<P0>(element: P0, value: bool) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        Self::ITypographyStatics(|this| unsafe {
            (::windows_core::Interface::vtable(this).SetContextualLigatures)(
                ::windows_core::Interface::as_raw(this),
                element.try_into_param()?.abi(),
                value,
            )
            .ok()
        })
    }
    pub fn DiscretionaryLigaturesProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DiscretionaryLigaturesProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn GetDiscretionaryLigatures<P0>(element: P0) -> ::windows_core::Result<bool>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetDiscretionaryLigatures)(
                ::windows_core::Interface::as_raw(this),
                element.try_into_param()?.abi(),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn SetDiscretionaryLigatures<P0>(element: P0, value: bool) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        Self::ITypographyStatics(|this| unsafe {
            (::windows_core::Interface::vtable(this).SetDiscretionaryLigatures)(
                ::windows_core::Interface::as_raw(this),
                element.try_into_param()?.abi(),
                value,
            )
            .ok()
        })
    }
    pub fn HistoricalLigaturesProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HistoricalLigaturesProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn GetHistoricalLigatures<P0>(element: P0) -> ::windows_core::Result<bool>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetHistoricalLigatures)(
                ::windows_core::Interface::as_raw(this),
                element.try_into_param()?.abi(),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn SetHistoricalLigatures<P0>(element: P0, value: bool) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        Self::ITypographyStatics(|this| unsafe {
            (::windows_core::Interface::vtable(this).SetHistoricalLigatures)(
                ::windows_core::Interface::as_raw(this),
                element.try_into_param()?.abi(),
                value,
            )
            .ok()
        })
    }
    pub fn StandardSwashesProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StandardSwashesProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn GetStandardSwashes<P0>(element: P0) -> ::windows_core::Result<i32>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetStandardSwashes)(
                ::windows_core::Interface::as_raw(this),
                element.try_into_param()?.abi(),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn SetStandardSwashes<P0>(element: P0, value: i32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        Self::ITypographyStatics(|this| unsafe {
            (::windows_core::Interface::vtable(this).SetStandardSwashes)(
                ::windows_core::Interface::as_raw(this),
                element.try_into_param()?.abi(),
                value,
            )
            .ok()
        })
    }
    pub fn ContextualSwashesProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ContextualSwashesProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn GetContextualSwashes<P0>(element: P0) -> ::windows_core::Result<i32>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetContextualSwashes)(
                ::windows_core::Interface::as_raw(this),
                element.try_into_param()?.abi(),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn SetContextualSwashes<P0>(element: P0, value: i32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        Self::ITypographyStatics(|this| unsafe {
            (::windows_core::Interface::vtable(this).SetContextualSwashes)(
                ::windows_core::Interface::as_raw(this),
                element.try_into_param()?.abi(),
                value,
            )
            .ok()
        })
    }
    pub fn ContextualAlternatesProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ContextualAlternatesProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn GetContextualAlternates<P0>(element: P0) -> ::windows_core::Result<bool>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetContextualAlternates)(
                ::windows_core::Interface::as_raw(this),
                element.try_into_param()?.abi(),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn SetContextualAlternates<P0>(element: P0, value: bool) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        Self::ITypographyStatics(|this| unsafe {
            (::windows_core::Interface::vtable(this).SetContextualAlternates)(
                ::windows_core::Interface::as_raw(this),
                element.try_into_param()?.abi(),
                value,
            )
            .ok()
        })
    }
    pub fn StylisticAlternatesProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StylisticAlternatesProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn GetStylisticAlternates<P0>(element: P0) -> ::windows_core::Result<i32>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetStylisticAlternates)(
                ::windows_core::Interface::as_raw(this),
                element.try_into_param()?.abi(),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn SetStylisticAlternates<P0>(element: P0, value: i32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        Self::ITypographyStatics(|this| unsafe {
            (::windows_core::Interface::vtable(this).SetStylisticAlternates)(
                ::windows_core::Interface::as_raw(this),
                element.try_into_param()?.abi(),
                value,
            )
            .ok()
        })
    }
    pub fn StylisticSet1Property() -> ::windows_core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StylisticSet1Property)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn GetStylisticSet1<P0>(element: P0) -> ::windows_core::Result<bool>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetStylisticSet1)(
                ::windows_core::Interface::as_raw(this),
                element.try_into_param()?.abi(),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn SetStylisticSet1<P0>(element: P0, value: bool) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        Self::ITypographyStatics(|this| unsafe {
            (::windows_core::Interface::vtable(this).SetStylisticSet1)(
                ::windows_core::Interface::as_raw(this),
                element.try_into_param()?.abi(),
                value,
            )
            .ok()
        })
    }
    pub fn StylisticSet2Property() -> ::windows_core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StylisticSet2Property)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn GetStylisticSet2<P0>(element: P0) -> ::windows_core::Result<bool>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetStylisticSet2)(
                ::windows_core::Interface::as_raw(this),
                element.try_into_param()?.abi(),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn SetStylisticSet2<P0>(element: P0, value: bool) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        Self::ITypographyStatics(|this| unsafe {
            (::windows_core::Interface::vtable(this).SetStylisticSet2)(
                ::windows_core::Interface::as_raw(this),
                element.try_into_param()?.abi(),
                value,
            )
            .ok()
        })
    }
    pub fn StylisticSet3Property() -> ::windows_core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StylisticSet3Property)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn GetStylisticSet3<P0>(element: P0) -> ::windows_core::Result<bool>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetStylisticSet3)(
                ::windows_core::Interface::as_raw(this),
                element.try_into_param()?.abi(),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn SetStylisticSet3<P0>(element: P0, value: bool) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        Self::ITypographyStatics(|this| unsafe {
            (::windows_core::Interface::vtable(this).SetStylisticSet3)(
                ::windows_core::Interface::as_raw(this),
                element.try_into_param()?.abi(),
                value,
            )
            .ok()
        })
    }
    pub fn StylisticSet4Property() -> ::windows_core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StylisticSet4Property)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn GetStylisticSet4<P0>(element: P0) -> ::windows_core::Result<bool>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetStylisticSet4)(
                ::windows_core::Interface::as_raw(this),
                element.try_into_param()?.abi(),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn SetStylisticSet4<P0>(element: P0, value: bool) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        Self::ITypographyStatics(|this| unsafe {
            (::windows_core::Interface::vtable(this).SetStylisticSet4)(
                ::windows_core::Interface::as_raw(this),
                element.try_into_param()?.abi(),
                value,
            )
            .ok()
        })
    }
    pub fn StylisticSet5Property() -> ::windows_core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StylisticSet5Property)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn GetStylisticSet5<P0>(element: P0) -> ::windows_core::Result<bool>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetStylisticSet5)(
                ::windows_core::Interface::as_raw(this),
                element.try_into_param()?.abi(),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn SetStylisticSet5<P0>(element: P0, value: bool) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        Self::ITypographyStatics(|this| unsafe {
            (::windows_core::Interface::vtable(this).SetStylisticSet5)(
                ::windows_core::Interface::as_raw(this),
                element.try_into_param()?.abi(),
                value,
            )
            .ok()
        })
    }
    pub fn StylisticSet6Property() -> ::windows_core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StylisticSet6Property)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn GetStylisticSet6<P0>(element: P0) -> ::windows_core::Result<bool>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetStylisticSet6)(
                ::windows_core::Interface::as_raw(this),
                element.try_into_param()?.abi(),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn SetStylisticSet6<P0>(element: P0, value: bool) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        Self::ITypographyStatics(|this| unsafe {
            (::windows_core::Interface::vtable(this).SetStylisticSet6)(
                ::windows_core::Interface::as_raw(this),
                element.try_into_param()?.abi(),
                value,
            )
            .ok()
        })
    }
    pub fn StylisticSet7Property() -> ::windows_core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StylisticSet7Property)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn GetStylisticSet7<P0>(element: P0) -> ::windows_core::Result<bool>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetStylisticSet7)(
                ::windows_core::Interface::as_raw(this),
                element.try_into_param()?.abi(),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn SetStylisticSet7<P0>(element: P0, value: bool) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        Self::ITypographyStatics(|this| unsafe {
            (::windows_core::Interface::vtable(this).SetStylisticSet7)(
                ::windows_core::Interface::as_raw(this),
                element.try_into_param()?.abi(),
                value,
            )
            .ok()
        })
    }
    pub fn StylisticSet8Property() -> ::windows_core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StylisticSet8Property)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn GetStylisticSet8<P0>(element: P0) -> ::windows_core::Result<bool>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetStylisticSet8)(
                ::windows_core::Interface::as_raw(this),
                element.try_into_param()?.abi(),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn SetStylisticSet8<P0>(element: P0, value: bool) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        Self::ITypographyStatics(|this| unsafe {
            (::windows_core::Interface::vtable(this).SetStylisticSet8)(
                ::windows_core::Interface::as_raw(this),
                element.try_into_param()?.abi(),
                value,
            )
            .ok()
        })
    }
    pub fn StylisticSet9Property() -> ::windows_core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StylisticSet9Property)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn GetStylisticSet9<P0>(element: P0) -> ::windows_core::Result<bool>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetStylisticSet9)(
                ::windows_core::Interface::as_raw(this),
                element.try_into_param()?.abi(),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn SetStylisticSet9<P0>(element: P0, value: bool) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        Self::ITypographyStatics(|this| unsafe {
            (::windows_core::Interface::vtable(this).SetStylisticSet9)(
                ::windows_core::Interface::as_raw(this),
                element.try_into_param()?.abi(),
                value,
            )
            .ok()
        })
    }
    pub fn StylisticSet10Property() -> ::windows_core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StylisticSet10Property)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn GetStylisticSet10<P0>(element: P0) -> ::windows_core::Result<bool>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetStylisticSet10)(
                ::windows_core::Interface::as_raw(this),
                element.try_into_param()?.abi(),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn SetStylisticSet10<P0>(element: P0, value: bool) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        Self::ITypographyStatics(|this| unsafe {
            (::windows_core::Interface::vtable(this).SetStylisticSet10)(
                ::windows_core::Interface::as_raw(this),
                element.try_into_param()?.abi(),
                value,
            )
            .ok()
        })
    }
    pub fn StylisticSet11Property() -> ::windows_core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StylisticSet11Property)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn GetStylisticSet11<P0>(element: P0) -> ::windows_core::Result<bool>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetStylisticSet11)(
                ::windows_core::Interface::as_raw(this),
                element.try_into_param()?.abi(),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn SetStylisticSet11<P0>(element: P0, value: bool) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        Self::ITypographyStatics(|this| unsafe {
            (::windows_core::Interface::vtable(this).SetStylisticSet11)(
                ::windows_core::Interface::as_raw(this),
                element.try_into_param()?.abi(),
                value,
            )
            .ok()
        })
    }
    pub fn StylisticSet12Property() -> ::windows_core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StylisticSet12Property)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn GetStylisticSet12<P0>(element: P0) -> ::windows_core::Result<bool>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetStylisticSet12)(
                ::windows_core::Interface::as_raw(this),
                element.try_into_param()?.abi(),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn SetStylisticSet12<P0>(element: P0, value: bool) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        Self::ITypographyStatics(|this| unsafe {
            (::windows_core::Interface::vtable(this).SetStylisticSet12)(
                ::windows_core::Interface::as_raw(this),
                element.try_into_param()?.abi(),
                value,
            )
            .ok()
        })
    }
    pub fn StylisticSet13Property() -> ::windows_core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StylisticSet13Property)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn GetStylisticSet13<P0>(element: P0) -> ::windows_core::Result<bool>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetStylisticSet13)(
                ::windows_core::Interface::as_raw(this),
                element.try_into_param()?.abi(),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn SetStylisticSet13<P0>(element: P0, value: bool) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        Self::ITypographyStatics(|this| unsafe {
            (::windows_core::Interface::vtable(this).SetStylisticSet13)(
                ::windows_core::Interface::as_raw(this),
                element.try_into_param()?.abi(),
                value,
            )
            .ok()
        })
    }
    pub fn StylisticSet14Property() -> ::windows_core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StylisticSet14Property)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn GetStylisticSet14<P0>(element: P0) -> ::windows_core::Result<bool>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetStylisticSet14)(
                ::windows_core::Interface::as_raw(this),
                element.try_into_param()?.abi(),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn SetStylisticSet14<P0>(element: P0, value: bool) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        Self::ITypographyStatics(|this| unsafe {
            (::windows_core::Interface::vtable(this).SetStylisticSet14)(
                ::windows_core::Interface::as_raw(this),
                element.try_into_param()?.abi(),
                value,
            )
            .ok()
        })
    }
    pub fn StylisticSet15Property() -> ::windows_core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StylisticSet15Property)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn GetStylisticSet15<P0>(element: P0) -> ::windows_core::Result<bool>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetStylisticSet15)(
                ::windows_core::Interface::as_raw(this),
                element.try_into_param()?.abi(),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn SetStylisticSet15<P0>(element: P0, value: bool) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        Self::ITypographyStatics(|this| unsafe {
            (::windows_core::Interface::vtable(this).SetStylisticSet15)(
                ::windows_core::Interface::as_raw(this),
                element.try_into_param()?.abi(),
                value,
            )
            .ok()
        })
    }
    pub fn StylisticSet16Property() -> ::windows_core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StylisticSet16Property)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn GetStylisticSet16<P0>(element: P0) -> ::windows_core::Result<bool>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetStylisticSet16)(
                ::windows_core::Interface::as_raw(this),
                element.try_into_param()?.abi(),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn SetStylisticSet16<P0>(element: P0, value: bool) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        Self::ITypographyStatics(|this| unsafe {
            (::windows_core::Interface::vtable(this).SetStylisticSet16)(
                ::windows_core::Interface::as_raw(this),
                element.try_into_param()?.abi(),
                value,
            )
            .ok()
        })
    }
    pub fn StylisticSet17Property() -> ::windows_core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StylisticSet17Property)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn GetStylisticSet17<P0>(element: P0) -> ::windows_core::Result<bool>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetStylisticSet17)(
                ::windows_core::Interface::as_raw(this),
                element.try_into_param()?.abi(),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn SetStylisticSet17<P0>(element: P0, value: bool) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        Self::ITypographyStatics(|this| unsafe {
            (::windows_core::Interface::vtable(this).SetStylisticSet17)(
                ::windows_core::Interface::as_raw(this),
                element.try_into_param()?.abi(),
                value,
            )
            .ok()
        })
    }
    pub fn StylisticSet18Property() -> ::windows_core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StylisticSet18Property)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn GetStylisticSet18<P0>(element: P0) -> ::windows_core::Result<bool>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetStylisticSet18)(
                ::windows_core::Interface::as_raw(this),
                element.try_into_param()?.abi(),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn SetStylisticSet18<P0>(element: P0, value: bool) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        Self::ITypographyStatics(|this| unsafe {
            (::windows_core::Interface::vtable(this).SetStylisticSet18)(
                ::windows_core::Interface::as_raw(this),
                element.try_into_param()?.abi(),
                value,
            )
            .ok()
        })
    }
    pub fn StylisticSet19Property() -> ::windows_core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StylisticSet19Property)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn GetStylisticSet19<P0>(element: P0) -> ::windows_core::Result<bool>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetStylisticSet19)(
                ::windows_core::Interface::as_raw(this),
                element.try_into_param()?.abi(),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn SetStylisticSet19<P0>(element: P0, value: bool) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        Self::ITypographyStatics(|this| unsafe {
            (::windows_core::Interface::vtable(this).SetStylisticSet19)(
                ::windows_core::Interface::as_raw(this),
                element.try_into_param()?.abi(),
                value,
            )
            .ok()
        })
    }
    pub fn StylisticSet20Property() -> ::windows_core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StylisticSet20Property)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn GetStylisticSet20<P0>(element: P0) -> ::windows_core::Result<bool>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetStylisticSet20)(
                ::windows_core::Interface::as_raw(this),
                element.try_into_param()?.abi(),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn SetStylisticSet20<P0>(element: P0, value: bool) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        Self::ITypographyStatics(|this| unsafe {
            (::windows_core::Interface::vtable(this).SetStylisticSet20)(
                ::windows_core::Interface::as_raw(this),
                element.try_into_param()?.abi(),
                value,
            )
            .ok()
        })
    }
    pub fn CapitalsProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CapitalsProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn GetCapitals<P0>(element: P0) -> ::windows_core::Result<super::FontCapitals>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetCapitals)(
                ::windows_core::Interface::as_raw(this),
                element.try_into_param()?.abi(),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn SetCapitals<P0>(element: P0, value: super::FontCapitals) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        Self::ITypographyStatics(|this| unsafe {
            (::windows_core::Interface::vtable(this).SetCapitals)(
                ::windows_core::Interface::as_raw(this),
                element.try_into_param()?.abi(),
                value,
            )
            .ok()
        })
    }
    pub fn CapitalSpacingProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CapitalSpacingProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn GetCapitalSpacing<P0>(element: P0) -> ::windows_core::Result<bool>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetCapitalSpacing)(
                ::windows_core::Interface::as_raw(this),
                element.try_into_param()?.abi(),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn SetCapitalSpacing<P0>(element: P0, value: bool) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        Self::ITypographyStatics(|this| unsafe {
            (::windows_core::Interface::vtable(this).SetCapitalSpacing)(
                ::windows_core::Interface::as_raw(this),
                element.try_into_param()?.abi(),
                value,
            )
            .ok()
        })
    }
    pub fn KerningProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).KerningProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn GetKerning<P0>(element: P0) -> ::windows_core::Result<bool>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetKerning)(
                ::windows_core::Interface::as_raw(this),
                element.try_into_param()?.abi(),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn SetKerning<P0>(element: P0, value: bool) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        Self::ITypographyStatics(|this| unsafe {
            (::windows_core::Interface::vtable(this).SetKerning)(
                ::windows_core::Interface::as_raw(this),
                element.try_into_param()?.abi(),
                value,
            )
            .ok()
        })
    }
    pub fn CaseSensitiveFormsProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CaseSensitiveFormsProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn GetCaseSensitiveForms<P0>(element: P0) -> ::windows_core::Result<bool>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetCaseSensitiveForms)(
                ::windows_core::Interface::as_raw(this),
                element.try_into_param()?.abi(),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn SetCaseSensitiveForms<P0>(element: P0, value: bool) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        Self::ITypographyStatics(|this| unsafe {
            (::windows_core::Interface::vtable(this).SetCaseSensitiveForms)(
                ::windows_core::Interface::as_raw(this),
                element.try_into_param()?.abi(),
                value,
            )
            .ok()
        })
    }
    pub fn HistoricalFormsProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HistoricalFormsProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn GetHistoricalForms<P0>(element: P0) -> ::windows_core::Result<bool>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetHistoricalForms)(
                ::windows_core::Interface::as_raw(this),
                element.try_into_param()?.abi(),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn SetHistoricalForms<P0>(element: P0, value: bool) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        Self::ITypographyStatics(|this| unsafe {
            (::windows_core::Interface::vtable(this).SetHistoricalForms)(
                ::windows_core::Interface::as_raw(this),
                element.try_into_param()?.abi(),
                value,
            )
            .ok()
        })
    }
    pub fn FractionProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FractionProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn GetFraction<P0>(element: P0) -> ::windows_core::Result<super::FontFraction>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetFraction)(
                ::windows_core::Interface::as_raw(this),
                element.try_into_param()?.abi(),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn SetFraction<P0>(element: P0, value: super::FontFraction) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        Self::ITypographyStatics(|this| unsafe {
            (::windows_core::Interface::vtable(this).SetFraction)(
                ::windows_core::Interface::as_raw(this),
                element.try_into_param()?.abi(),
                value,
            )
            .ok()
        })
    }
    pub fn NumeralStyleProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NumeralStyleProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn GetNumeralStyle<P0>(element: P0) -> ::windows_core::Result<super::FontNumeralStyle>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetNumeralStyle)(
                ::windows_core::Interface::as_raw(this),
                element.try_into_param()?.abi(),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn SetNumeralStyle<P0>(
        element: P0,
        value: super::FontNumeralStyle,
    ) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        Self::ITypographyStatics(|this| unsafe {
            (::windows_core::Interface::vtable(this).SetNumeralStyle)(
                ::windows_core::Interface::as_raw(this),
                element.try_into_param()?.abi(),
                value,
            )
            .ok()
        })
    }
    pub fn NumeralAlignmentProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NumeralAlignmentProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn GetNumeralAlignment<P0>(
        element: P0,
    ) -> ::windows_core::Result<super::FontNumeralAlignment>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetNumeralAlignment)(
                ::windows_core::Interface::as_raw(this),
                element.try_into_param()?.abi(),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn SetNumeralAlignment<P0>(
        element: P0,
        value: super::FontNumeralAlignment,
    ) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        Self::ITypographyStatics(|this| unsafe {
            (::windows_core::Interface::vtable(this).SetNumeralAlignment)(
                ::windows_core::Interface::as_raw(this),
                element.try_into_param()?.abi(),
                value,
            )
            .ok()
        })
    }
    pub fn SlashedZeroProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SlashedZeroProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn GetSlashedZero<P0>(element: P0) -> ::windows_core::Result<bool>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetSlashedZero)(
                ::windows_core::Interface::as_raw(this),
                element.try_into_param()?.abi(),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn SetSlashedZero<P0>(element: P0, value: bool) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        Self::ITypographyStatics(|this| unsafe {
            (::windows_core::Interface::vtable(this).SetSlashedZero)(
                ::windows_core::Interface::as_raw(this),
                element.try_into_param()?.abi(),
                value,
            )
            .ok()
        })
    }
    pub fn MathematicalGreekProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MathematicalGreekProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn GetMathematicalGreek<P0>(element: P0) -> ::windows_core::Result<bool>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetMathematicalGreek)(
                ::windows_core::Interface::as_raw(this),
                element.try_into_param()?.abi(),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn SetMathematicalGreek<P0>(element: P0, value: bool) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        Self::ITypographyStatics(|this| unsafe {
            (::windows_core::Interface::vtable(this).SetMathematicalGreek)(
                ::windows_core::Interface::as_raw(this),
                element.try_into_param()?.abi(),
                value,
            )
            .ok()
        })
    }
    pub fn VariantsProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).VariantsProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn GetVariants<P0>(element: P0) -> ::windows_core::Result<super::FontVariants>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        Self::ITypographyStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetVariants)(
                ::windows_core::Interface::as_raw(this),
                element.try_into_param()?.abi(),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn SetVariants<P0>(element: P0, value: super::FontVariants) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        Self::ITypographyStatics(|this| unsafe {
            (::windows_core::Interface::vtable(this).SetVariants)(
                ::windows_core::Interface::as_raw(this),
                element.try_into_param()?.abi(),
                value,
            )
            .ok()
        })
    }
    #[doc(hidden)]
    pub fn ITypographyStatics<R, F: FnOnce(&ITypographyStatics) -> ::windows_core::Result<R>>(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<Typography, ITypographyStatics> =
            ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for Typography {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for Typography {
    type Vtable = ITypography_Vtbl;
}
unsafe impl ::windows_core::ComInterface for Typography {
    const IID: ::windows_core::GUID = <ITypography as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for Typography {
    const NAME: &'static str = "Microsoft.UI.Xaml.Documents.Typography";
}
::windows_core::imp::interface_hierarchy!(
    Typography,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for Typography {}
unsafe impl ::core::marker::Sync for Typography {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct Underline(::windows_core::IUnknown);
impl Underline {
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
            Underline,
            ::windows_core::imp::IGenericFactory,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
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
    #[doc = "Required features: `\"Windows_Foundation_Collections\"`"]
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub fn Inlines(&self) -> ::windows_core::Result<InlineCollection> {
        let this = &::windows_core::ComInterface::cast::<ISpan>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Inlines)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation_Collections\"`"]
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub fn SetInlines<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<InlineCollection>,
    {
        let this = &::windows_core::ComInterface::cast::<ISpan>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetInlines)(
                ::windows_core::Interface::as_raw(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn Name(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Name)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn FontSize(&self) -> ::windows_core::Result<f64> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FontSize)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetFontSize(&self, value: f64) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetFontSize)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Media\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Media")]
    pub fn FontFamily(&self) -> ::windows_core::Result<super::Media::FontFamily> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FontFamily)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Media\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Media")]
    pub fn SetFontFamily<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::Media::FontFamily>,
    {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetFontFamily)(
                ::windows_core::Interface::as_raw(this),
                value.try_into_param()?.abi(),
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_UI_Text\"`"]
    #[cfg(feature = "Windows_UI_Text")]
    pub fn FontWeight(&self) -> ::windows_core::Result<::windows::UI::Text::FontWeight> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FontWeight)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_UI_Text\"`"]
    #[cfg(feature = "Windows_UI_Text")]
    pub fn SetFontWeight(
        &self,
        value: ::windows::UI::Text::FontWeight,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetFontWeight)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_UI_Text\"`"]
    #[cfg(feature = "Windows_UI_Text")]
    pub fn FontStyle(&self) -> ::windows_core::Result<::windows::UI::Text::FontStyle> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FontStyle)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_UI_Text\"`"]
    #[cfg(feature = "Windows_UI_Text")]
    pub fn SetFontStyle(
        &self,
        value: ::windows::UI::Text::FontStyle,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetFontStyle)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_UI_Text\"`"]
    #[cfg(feature = "Windows_UI_Text")]
    pub fn FontStretch(&self) -> ::windows_core::Result<::windows::UI::Text::FontStretch> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FontStretch)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_UI_Text\"`"]
    #[cfg(feature = "Windows_UI_Text")]
    pub fn SetFontStretch(
        &self,
        value: ::windows::UI::Text::FontStretch,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetFontStretch)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn CharacterSpacing(&self) -> ::windows_core::Result<i32> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CharacterSpacing)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetCharacterSpacing(&self, value: i32) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetCharacterSpacing)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Media\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Media")]
    pub fn Foreground(&self) -> ::windows_core::Result<super::Media::Brush> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Foreground)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Media\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Media")]
    pub fn SetForeground<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::Media::Brush>,
    {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetForeground)(
                ::windows_core::Interface::as_raw(this),
                value.try_into_param()?.abi(),
            )
            .ok()
        }
    }
    pub fn Language(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
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
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetLanguage)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn IsTextScaleFactorEnabled(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsTextScaleFactorEnabled)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetIsTextScaleFactorEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetIsTextScaleFactorEnabled)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_UI_Text\"`"]
    #[cfg(feature = "Windows_UI_Text")]
    pub fn TextDecorations(&self) -> ::windows_core::Result<::windows::UI::Text::TextDecorations> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TextDecorations)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_UI_Text\"`"]
    #[cfg(feature = "Windows_UI_Text")]
    pub fn SetTextDecorations(
        &self,
        value: ::windows::UI::Text::TextDecorations,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetTextDecorations)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn ContentStart(&self) -> ::windows_core::Result<TextPointer> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ContentStart)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn ContentEnd(&self) -> ::windows_core::Result<TextPointer> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ContentEnd)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn ElementStart(&self) -> ::windows_core::Result<TextPointer> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ElementStart)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn ElementEnd(&self) -> ::windows_core::Result<TextPointer> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ElementEnd)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn AllowFocusOnInteraction(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AllowFocusOnInteraction)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetAllowFocusOnInteraction(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetAllowFocusOnInteraction)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn AccessKey(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AccessKey)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetAccessKey(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetAccessKey)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn ExitDisplayModeOnAccessKeyInvoked(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ExitDisplayModeOnAccessKeyInvoked)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetExitDisplayModeOnAccessKeyInvoked(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetExitDisplayModeOnAccessKeyInvoked)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsAccessKeyScope(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsAccessKeyScope)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetIsAccessKeyScope(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetIsAccessKeyScope)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn AccessKeyScopeOwner(&self) -> ::windows_core::Result<super::DependencyObject> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AccessKeyScopeOwner)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetAccessKeyScopeOwner<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetAccessKeyScopeOwner)(
                ::windows_core::Interface::as_raw(this),
                value.try_into_param()?.abi(),
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Input\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Input")]
    pub fn KeyTipPlacementMode(&self) -> ::windows_core::Result<super::Input::KeyTipPlacementMode> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).KeyTipPlacementMode)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Input\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Input")]
    pub fn SetKeyTipPlacementMode(
        &self,
        value: super::Input::KeyTipPlacementMode,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetKeyTipPlacementMode)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn KeyTipHorizontalOffset(&self) -> ::windows_core::Result<f64> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).KeyTipHorizontalOffset)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetKeyTipHorizontalOffset(&self, value: f64) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetKeyTipHorizontalOffset)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn KeyTipVerticalOffset(&self) -> ::windows_core::Result<f64> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).KeyTipVerticalOffset)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetKeyTipVerticalOffset(&self, value: f64) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetKeyTipVerticalOffset)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn XamlRoot(&self) -> ::windows_core::Result<super::XamlRoot> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).XamlRoot)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetXamlRoot<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::XamlRoot>,
    {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetXamlRoot)(
                ::windows_core::Interface::as_raw(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Input\"`, `\"Windows_Foundation\"`"]
    #[cfg(all(feature = "Microsoft_UI_Xaml_Input", feature = "Windows_Foundation"))]
    pub fn AccessKeyDisplayRequested<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<
                TextElement,
                super::Input::AccessKeyDisplayRequestedEventArgs,
            >,
        >,
    {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AccessKeyDisplayRequested)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveAccessKeyDisplayRequested(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveAccessKeyDisplayRequested)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Input\"`, `\"Windows_Foundation\"`"]
    #[cfg(all(feature = "Microsoft_UI_Xaml_Input", feature = "Windows_Foundation"))]
    pub fn AccessKeyDisplayDismissed<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<
                TextElement,
                super::Input::AccessKeyDisplayDismissedEventArgs,
            >,
        >,
    {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AccessKeyDisplayDismissed)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveAccessKeyDisplayDismissed(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveAccessKeyDisplayDismissed)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Input\"`, `\"Windows_Foundation\"`"]
    #[cfg(all(feature = "Microsoft_UI_Xaml_Input", feature = "Windows_Foundation"))]
    pub fn AccessKeyInvoked<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<
                TextElement,
                super::Input::AccessKeyInvokedEventArgs,
            >,
        >,
    {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AccessKeyInvoked)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveAccessKeyInvoked(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveAccessKeyInvoked)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn FindName(
        &self,
        name: &::windows_core::HSTRING,
    ) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = &::windows_core::ComInterface::cast::<ITextElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FindName)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(name),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn OnDisconnectVisualChildren(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextElementOverrides>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).OnDisconnectVisualChildren)(
                ::windows_core::Interface::as_raw(this),
            )
            .ok()
        }
    }
}
impl ::windows_core::RuntimeType for Underline {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for Underline {
    type Vtable = IUnderline_Vtbl;
}
unsafe impl ::windows_core::ComInterface for Underline {
    const IID: ::windows_core::GUID = <IUnderline as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for Underline {
    const NAME: &'static str = "Microsoft.UI.Xaml.Documents.Underline";
}
::windows_core::imp::interface_hierarchy!(
    Underline,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<Span> for Underline {}
impl ::windows_core::CanTryInto<Inline> for Underline {}
impl ::windows_core::CanTryInto<TextElement> for Underline {}
impl ::windows_core::CanTryInto<super::DependencyObject> for Underline {}
unsafe impl ::core::marker::Send for Underline {}
unsafe impl ::core::marker::Sync for Underline {}
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
impl ::windows_core::TypeKind for LogicalDirection {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for LogicalDirection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LogicalDirection").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for LogicalDirection {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::from_slice(
            b"enum(Microsoft.UI.Xaml.Documents.LogicalDirection;i4)",
        );
}
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
impl ::windows_core::TypeKind for UnderlineStyle {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for UnderlineStyle {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UnderlineStyle").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for UnderlineStyle {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::from_slice(
            b"enum(Microsoft.UI.Xaml.Documents.UnderlineStyle;i4)",
        );
}
#[repr(C)]
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
impl ::windows_core::TypeKind for TextRange {
    type TypeKind = ::windows_core::CopyType;
}
impl ::windows_core::RuntimeType for TextRange {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::from_slice(
            b"struct(Microsoft.UI.Xaml.Documents.TextRange;i4;i4)",
        );
}
impl ::core::cmp::PartialEq for TextRange {
    fn eq(&self, other: &Self) -> bool {
        self.StartIndex == other.StartIndex && self.Length == other.Length
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
