#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IContentCoordinateConverter(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IContentCoordinateConverter {
    type Vtable = IContentCoordinateConverter_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IContentCoordinateConverter {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x10a11230_5eb4_5840_a346_570f4a49040f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContentCoordinateConverter_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Windows_Foundation", feature = "Windows_Graphics"))]
    pub ConvertLocalToScreenWithPoint: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        localpoint: ::windows::Foundation::Point,
        result__: *mut ::windows::Graphics::PointInt32,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Windows_Foundation", feature = "Windows_Graphics")))]
    ConvertLocalToScreenWithPoint: usize,
    #[cfg(all(feature = "Windows_Foundation", feature = "Windows_Graphics"))]
    pub ConvertLocalToScreenWithPoints: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        localPoints_array_size: u32,
        localpoints: *const ::windows::Foundation::Point,
        result_size__: *mut u32,
        result__: *mut *mut ::windows::Graphics::PointInt32,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Windows_Foundation", feature = "Windows_Graphics")))]
    ConvertLocalToScreenWithPoints: usize,
    #[cfg(all(feature = "Windows_Foundation", feature = "Windows_Graphics"))]
    pub ConvertLocalToScreenWithPointsAndRoundingMode:
        unsafe extern "system" fn(
            this: *mut ::core::ffi::c_void,
            localPoints_array_size: u32,
            localpoints: *const ::windows::Foundation::Point,
            roundingmode: ContentCoordinateRoundingMode,
            result_size__: *mut u32,
            result__: *mut *mut ::windows::Graphics::PointInt32,
        ) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Windows_Foundation", feature = "Windows_Graphics")))]
    ConvertLocalToScreenWithPointsAndRoundingMode: usize,
    #[cfg(all(feature = "Windows_Foundation", feature = "Windows_Graphics"))]
    pub ConvertLocalToScreenWithRect: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        localrect: ::windows::Foundation::Rect,
        result__: *mut ::windows::Graphics::RectInt32,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Windows_Foundation", feature = "Windows_Graphics")))]
    ConvertLocalToScreenWithRect: usize,
    #[cfg(all(feature = "Windows_Foundation", feature = "Windows_Graphics"))]
    pub ConvertScreenToLocalWithPoint: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        screenpoint: ::windows::Graphics::PointInt32,
        result__: *mut ::windows::Foundation::Point,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Windows_Foundation", feature = "Windows_Graphics")))]
    ConvertScreenToLocalWithPoint: usize,
    #[cfg(all(feature = "Windows_Foundation", feature = "Windows_Graphics"))]
    pub ConvertScreenToLocalWithPoints: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        screenPoints_array_size: u32,
        screenpoints: *const ::windows::Graphics::PointInt32,
        result_size__: *mut u32,
        result__: *mut *mut ::windows::Foundation::Point,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Windows_Foundation", feature = "Windows_Graphics")))]
    ConvertScreenToLocalWithPoints: usize,
    #[cfg(all(feature = "Windows_Foundation", feature = "Windows_Graphics"))]
    pub ConvertScreenToLocalWithRect: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        screenrect: ::windows::Graphics::RectInt32,
        result__: *mut ::windows::Foundation::Rect,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Windows_Foundation", feature = "Windows_Graphics")))]
    ConvertScreenToLocalWithRect: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IContentCoordinateConverterFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IContentCoordinateConverterFactory {
    type Vtable = IContentCoordinateConverterFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IContentCoordinateConverterFactory {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xa4b216ee_0e26_56bd_9209_a75cf9d80f27);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContentCoordinateConverterFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IContentCoordinateConverterStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IContentCoordinateConverterStatics {
    type Vtable = IContentCoordinateConverterStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IContentCoordinateConverterStatics {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xf56374ce_b6df_5b42_a58f_4e3bb039e3a9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContentCoordinateConverterStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateForWindowId: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        windowid: super::WindowId,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IContentDeferral(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IContentDeferral {
    type Vtable = IContentDeferral_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IContentDeferral {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x470529f5_cd93_599b_968e_f8a689bc3a07);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContentDeferral_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Complete:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IContentEnvironmentSettingChangedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IContentEnvironmentSettingChangedEventArgs {
    type Vtable = IContentEnvironmentSettingChangedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IContentEnvironmentSettingChangedEventArgs {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x76478051_fc80_5eec_a3f3_62606abe06b7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContentEnvironmentSettingChangedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SettingName: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IContentEnvironmentStateChangedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IContentEnvironmentStateChangedEventArgs {
    type Vtable = IContentEnvironmentStateChangedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IContentEnvironmentStateChangedEventArgs {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x8970fa4f_10ba_5f67_970b_8c72bc009b67);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContentEnvironmentStateChangedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub DidAppWindowIdChange: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub DidDisplayIdChange: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IContentIsland(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IContentIsland {
    type Vtable = IContentIsland_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IContentIsland {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x5b2504ba_361c_50aa_bd6e_4122c6d93889);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContentIsland_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Windows_Foundation_Numerics")]
    pub ActualSize: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::Numerics::Vector2,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation_Numerics"))]
    ActualSize: usize,
    pub AppData: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetAppData: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub CoordinateConverter: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub CustomProperties: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation_Collections"))]
    CustomProperties: usize,
    #[cfg(feature = "Microsoft_UI_Dispatching")]
    pub DispatcherQueue: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Microsoft_UI_Dispatching"))]
    DispatcherQueue: usize,
    pub Environment: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub Id: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut u64,
    ) -> ::windows_core::HRESULT,
    pub IsConnected: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub IsHitTestVisibleWhenTransparent: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetIsHitTestVisibleWhenTransparent: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    )
        -> ::windows_core::HRESULT,
    pub IsIslandEnabled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetIsIslandEnabled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
    pub IsIslandVisible: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetIsIslandVisible: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
    pub IsSiteEnabled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub IsSiteVisible: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub LayoutDirection: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ContentLayoutDirection,
    ) -> ::windows_core::HRESULT,
    pub RasterizationScale: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f32,
    ) -> ::windows_core::HRESULT,
    pub GetAutomationHostProvider: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub GetStateChangeDeferral: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_Foundation_Numerics")]
    pub RequestSize: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        size: ::windows::Foundation::Numerics::Vector2,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation_Numerics"))]
    RequestSize: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub AutomationProviderRequested: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    AutomationProviderRequested: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub RemoveAutomationProviderRequested: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    )
        -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RemoveAutomationProviderRequested: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub StateChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    StateChanged: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub RemoveStateChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RemoveStateChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IContentIslandAutomationProviderRequestedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IContentIslandAutomationProviderRequestedEventArgs {
    type Vtable = IContentIslandAutomationProviderRequestedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IContentIslandAutomationProviderRequestedEventArgs {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x9fe24bed_2b9c_5137_887f_403c94841824);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContentIslandAutomationProviderRequestedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub AutomationProvider: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetAutomationProvider: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub Handled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IContentIslandEnvironment(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IContentIslandEnvironment {
    type Vtable = IContentIslandEnvironment_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IContentIslandEnvironment {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xc334236d_da88_566d_811d_74aef2eba978);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContentIslandEnvironment_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub AppWindowId: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::WindowId,
    ) -> ::windows_core::HRESULT,
    pub DisplayId: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::DisplayId,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_Foundation")]
    pub SettingChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    SettingChanged: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub RemoveSettingChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RemoveSettingChanged: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub StateChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    StateChanged: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub RemoveStateChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RemoveStateChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IContentIslandEnvironmentFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IContentIslandEnvironmentFactory {
    type Vtable = IContentIslandEnvironmentFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IContentIslandEnvironmentFactory {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x47a782d6_b177_5c1e_bf87_90437dd809d0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContentIslandEnvironmentFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IContentIslandFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IContentIslandFactory {
    type Vtable = IContentIslandFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IContentIslandFactory {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x82383f52_e81a_5ec9_a954_bac8a931ba7d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContentIslandFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IContentIslandStateChangedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IContentIslandStateChangedEventArgs {
    type Vtable = IContentIslandStateChangedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IContentIslandStateChangedEventArgs {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xc828eeb2_0c62_5b40_9d48_77c06083c278);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContentIslandStateChangedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub DidActualSizeChange: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub DidSiteEnabledChange: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub DidSiteVisibleChange: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub DidLayoutDirectionChange: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub DidRasterizationScaleChange: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IContentIslandStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IContentIslandStatics {
    type Vtable = IContentIslandStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IContentIslandStatics {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x7b9eb7cc_8c43_5e0a_ab23_ab48628fd223);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContentIslandStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Microsoft_UI_Composition")]
    pub Create: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        root: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Microsoft_UI_Composition"))]
    Create: usize,
    #[cfg(feature = "Microsoft_UI_Composition")]
    pub FindAllForCompositor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        compositor: *mut ::core::ffi::c_void,
        result_size__: *mut u32,
        result__: *mut *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Microsoft_UI_Composition"))]
    FindAllForCompositor: usize,
    pub FindAllForCurrentThread: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result_size__: *mut u32,
        result__: *mut *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Microsoft_UI_Composition")]
    pub GetByVisual: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        child: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Microsoft_UI_Composition"))]
    GetByVisual: usize,
    pub GetFromId: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        id: u64,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IContentSite(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IContentSite {
    type Vtable = IContentSite_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IContentSite {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x996c60c4_02b2_5eef_93b0_dd6b1ec2fd7b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContentSite_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Windows_Foundation_Numerics")]
    pub ActualSize: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::Numerics::Vector2,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation_Numerics"))]
    ActualSize: usize,
    #[cfg(feature = "Windows_Foundation_Numerics")]
    pub SetActualSize: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::windows::Foundation::Numerics::Vector2,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation_Numerics"))]
    SetActualSize: usize,
    #[cfg(feature = "Windows_Graphics")]
    pub ClientSize: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Graphics::SizeInt32,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Graphics"))]
    ClientSize: usize,
    #[cfg(feature = "Windows_Graphics")]
    pub SetClientSize: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::windows::Graphics::SizeInt32,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Graphics"))]
    SetClientSize: usize,
    pub CoordinateConverter: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Microsoft_UI_Dispatching")]
    pub DispatcherQueue: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Microsoft_UI_Dispatching"))]
    DispatcherQueue: usize,
    pub Environment: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub IsConnected: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub IsSiteEnabled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetIsSiteEnabled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
    pub IsSiteVisible: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetIsSiteVisible: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
    pub LayoutDirection: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ContentLayoutDirection,
    ) -> ::windows_core::HRESULT,
    pub SetLayoutDirection: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ContentLayoutDirection,
    ) -> ::windows_core::HRESULT,
    pub OverrideScale: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f32,
    ) -> ::windows_core::HRESULT,
    pub SetOverrideScale: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f32,
    ) -> ::windows_core::HRESULT,
    pub ParentScale: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f32,
    ) -> ::windows_core::HRESULT,
    pub SetParentScale: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f32,
    ) -> ::windows_core::HRESULT,
    pub RasterizationScale: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f32,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_Foundation_Numerics")]
    pub RequestedSize: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::Numerics::Vector2,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation_Numerics"))]
    RequestedSize: usize,
    pub ShouldApplyRasterizationScale: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetShouldApplyRasterizationScale: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
    pub View: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub GetIslandStateChangeDeferral: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_Foundation")]
    pub RequestedStateChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RequestedStateChanged: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub RemoveRequestedStateChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RemoveRequestedStateChanged: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IContentSiteBridge(::windows_core::IUnknown);
impl IContentSiteBridge {
    #[doc = "Required features: `\"Microsoft_UI_Dispatching\"`"]
    #[cfg(feature = "Microsoft_UI_Dispatching")]
    pub fn DispatcherQueue(&self) -> ::windows_core::Result<super::Dispatching::DispatcherQueue> {
        let this = self;
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
    pub fn LayoutDirectionOverride(
        &self,
    ) -> ::windows_core::Result<::windows::Foundation::IReference<ContentLayoutDirection>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LayoutDirectionOverride)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn SetLayoutDirectionOverride<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<::windows::Foundation::IReference<ContentLayoutDirection>>,
    {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetLayoutDirectionOverride)(
                ::windows_core::Interface::as_raw(this),
                value.try_into_param()?.abi(),
            )
            .ok()
        }
    }
    pub fn OverrideScale(&self) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OverrideScale)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetOverrideScale(&self, value: f32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetOverrideScale)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<::windows::Foundation::IClosable>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this))
                .ok()
        }
    }
}
::windows_core::imp::interface_hierarchy!(
    IContentSiteBridge,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
#[cfg(feature = "Windows_Foundation")]
impl ::windows_core::CanTryInto<::windows::Foundation::IClosable> for IContentSiteBridge {}
impl ::windows_core::RuntimeType for IContentSiteBridge {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_interface::<Self>();
}
unsafe impl ::windows_core::Interface for IContentSiteBridge {
    type Vtable = IContentSiteBridge_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IContentSiteBridge {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xfaaab99e_a42b_549c_92df_3b6d6e1e368b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContentSiteBridge_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Microsoft_UI_Dispatching")]
    pub DispatcherQueue: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Microsoft_UI_Dispatching"))]
    DispatcherQueue: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub LayoutDirectionOverride: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    LayoutDirectionOverride: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub SetLayoutDirectionOverride: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    SetLayoutDirectionOverride: usize,
    pub OverrideScale: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f32,
    ) -> ::windows_core::HRESULT,
    pub SetOverrideScale: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f32,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IContentSiteEnvironment(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IContentSiteEnvironment {
    type Vtable = IContentSiteEnvironment_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IContentSiteEnvironment {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x685d085d_be53_55d1_aec4_ba2273d5468b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContentSiteEnvironment_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub AppWindowId: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::WindowId,
    ) -> ::windows_core::HRESULT,
    pub SetAppWindowId: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: super::WindowId,
    ) -> ::windows_core::HRESULT,
    pub DisplayId: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::DisplayId,
    ) -> ::windows_core::HRESULT,
    pub SetDisplayId: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: super::DisplayId,
    ) -> ::windows_core::HRESULT,
    pub View: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub NotifySettingChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        setting: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IContentSiteEnvironmentFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IContentSiteEnvironmentFactory {
    type Vtable = IContentSiteEnvironmentFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IContentSiteEnvironmentFactory {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x0befa998_cb15_5f16_a4a5_c0ed1674e186);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContentSiteEnvironmentFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IContentSiteEnvironmentView(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IContentSiteEnvironmentView {
    type Vtable = IContentSiteEnvironmentView_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IContentSiteEnvironmentView {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x5b6fe420_0bb3_54dd_8589_786cf02e38f1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContentSiteEnvironmentView_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub AppWindowId: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::WindowId,
    ) -> ::windows_core::HRESULT,
    pub DisplayId: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::DisplayId,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IContentSiteEnvironmentViewFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IContentSiteEnvironmentViewFactory {
    type Vtable = IContentSiteEnvironmentViewFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IContentSiteEnvironmentViewFactory {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xc901edf2_f184_5a64_8d58_8cf8efa8b678);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContentSiteEnvironmentViewFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IContentSiteFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IContentSiteFactory {
    type Vtable = IContentSiteFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IContentSiteFactory {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x72fb98d5_b28a_57f1_91fa_24c014a342c1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContentSiteFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IContentSiteRequestedStateChangedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IContentSiteRequestedStateChangedEventArgs {
    type Vtable = IContentSiteRequestedStateChangedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IContentSiteRequestedStateChangedEventArgs {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x1b55fd1d_7292_562b_b4a1_d4de7972e684);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContentSiteRequestedStateChangedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub DidRequestedSizeChange: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IContentSiteView(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IContentSiteView {
    type Vtable = IContentSiteView_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IContentSiteView {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x2d5d8dd5_358e_5b05_993b_b2666d1786b3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContentSiteView_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Windows_Foundation_Numerics")]
    pub ActualSize: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::Numerics::Vector2,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation_Numerics"))]
    ActualSize: usize,
    #[cfg(feature = "Windows_Graphics")]
    pub ClientSize: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Graphics::SizeInt32,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Graphics"))]
    ClientSize: usize,
    pub CoordinateConverter: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Microsoft_UI_Dispatching")]
    pub DispatcherQueue: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Microsoft_UI_Dispatching"))]
    DispatcherQueue: usize,
    pub EnvironmentView: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub IsConnected: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub IsSiteEnabled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub IsSiteVisible: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub LayoutDirection: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ContentLayoutDirection,
    ) -> ::windows_core::HRESULT,
    pub OverrideScale: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f32,
    ) -> ::windows_core::HRESULT,
    pub ParentScale: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f32,
    ) -> ::windows_core::HRESULT,
    pub RasterizationScale: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f32,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_Foundation_Numerics")]
    pub RequestedSize: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::Numerics::Vector2,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation_Numerics"))]
    RequestedSize: usize,
    pub ShouldApplyRasterizationScale: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IContentSiteViewFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IContentSiteViewFactory {
    type Vtable = IContentSiteViewFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IContentSiteViewFactory {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x9efd72f0_63ef_5b6a_a50c_5685bd8100f1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContentSiteViewFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDesktopChildSiteBridge(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDesktopChildSiteBridge {
    type Vtable = IDesktopChildSiteBridge_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDesktopChildSiteBridge {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xb2f2ff7b_1825_51b0_b80b_7599889c569f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDesktopChildSiteBridge_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ResizePolicy: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ContentSizePolicy,
    ) -> ::windows_core::HRESULT,
    pub SetResizePolicy: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ContentSizePolicy,
    ) -> ::windows_core::HRESULT,
    pub SiteView: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDesktopChildSiteBridgeStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDesktopChildSiteBridgeStatics {
    type Vtable = IDesktopChildSiteBridgeStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDesktopChildSiteBridgeStatics {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xab6b82de_6a47_5de3_a860_613c8db679ab);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDesktopChildSiteBridgeStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Microsoft_UI_Composition")]
    pub Create: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        compositor: *mut ::core::ffi::c_void,
        parentwindowid: super::WindowId,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Microsoft_UI_Composition"))]
    Create: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDesktopSiteBridge(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDesktopSiteBridge {
    type Vtable = IDesktopSiteBridge_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDesktopSiteBridge {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xf0ae8750_905c_50a2_8a12_4545c6245bb4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDesktopSiteBridge_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub IsEnabled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub IsVisible: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub WindowId: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::WindowId,
    ) -> ::windows_core::HRESULT,
    pub Connect: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        content: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub Disable:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Enable:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Hide: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_Graphics")]
    pub MoveAndResize: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        rect: ::windows::Graphics::RectInt32,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Graphics"))]
    MoveAndResize: usize,
    pub MoveInZOrderAtBottom:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub MoveInZOrderAtTop:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub MoveInZOrderBelow: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        windowid: super::WindowId,
    ) -> ::windows_core::HRESULT,
    pub Show: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDesktopSiteBridgeFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDesktopSiteBridgeFactory {
    type Vtable = IDesktopSiteBridgeFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDesktopSiteBridgeFactory {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xd94ee1ff_3af1_54d0_9311_652b29c57c5b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDesktopSiteBridgeFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDesktopSiteBridgeStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDesktopSiteBridgeStatics {
    type Vtable = IDesktopSiteBridgeStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDesktopSiteBridgeStatics {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xe0b38daf_9cd4_50c5_83ee_c76e3cf34eba);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDesktopSiteBridgeStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub IsSupported: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ContentCoordinateConverter(::windows_core::IUnknown);
impl ContentCoordinateConverter {
    #[doc = "Required features: `\"Windows_Foundation\"`, `\"Windows_Graphics\"`"]
    #[cfg(all(feature = "Windows_Foundation", feature = "Windows_Graphics"))]
    pub fn ConvertLocalToScreenWithPoint(
        &self,
        localpoint: ::windows::Foundation::Point,
    ) -> ::windows_core::Result<::windows::Graphics::PointInt32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ConvertLocalToScreenWithPoint)(
                ::windows_core::Interface::as_raw(this),
                localpoint,
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`, `\"Windows_Graphics\"`"]
    #[cfg(all(feature = "Windows_Foundation", feature = "Windows_Graphics"))]
    pub fn ConvertLocalToScreenWithPoints(
        &self,
        localpoints: &[::windows::Foundation::Point],
    ) -> ::windows_core::Result<::windows_core::Array<::windows::Graphics::PointInt32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows_core::Interface::vtable(this).ConvertLocalToScreenWithPoints)(
                ::windows_core::Interface::as_raw(this),
                localpoints.len().try_into().unwrap(),
                localpoints.as_ptr(),
                ::windows_core::Array::<::windows::Graphics::PointInt32>::set_abi_len(
                    ::std::mem::transmute(&mut result__),
                ),
                result__.as_mut_ptr() as *mut _ as _,
            )
            .and_then(|| result__.assume_init())
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`, `\"Windows_Graphics\"`"]
    #[cfg(all(feature = "Windows_Foundation", feature = "Windows_Graphics"))]
    pub fn ConvertLocalToScreenWithPointsAndRoundingMode(
        &self,
        localpoints: &[::windows::Foundation::Point],
        roundingmode: ContentCoordinateRoundingMode,
    ) -> ::windows_core::Result<::windows_core::Array<::windows::Graphics::PointInt32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows_core::Interface::vtable(this).ConvertLocalToScreenWithPointsAndRoundingMode)(
                ::windows_core::Interface::as_raw(this),
                localpoints.len().try_into().unwrap(),
                localpoints.as_ptr(),
                roundingmode,
                ::windows_core::Array::<::windows::Graphics::PointInt32>::set_abi_len(
                    ::std::mem::transmute(&mut result__),
                ),
                result__.as_mut_ptr() as *mut _ as _,
            )
            .and_then(|| result__.assume_init())
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`, `\"Windows_Graphics\"`"]
    #[cfg(all(feature = "Windows_Foundation", feature = "Windows_Graphics"))]
    pub fn ConvertLocalToScreenWithRect(
        &self,
        localrect: ::windows::Foundation::Rect,
    ) -> ::windows_core::Result<::windows::Graphics::RectInt32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ConvertLocalToScreenWithRect)(
                ::windows_core::Interface::as_raw(this),
                localrect,
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`, `\"Windows_Graphics\"`"]
    #[cfg(all(feature = "Windows_Foundation", feature = "Windows_Graphics"))]
    pub fn ConvertScreenToLocalWithPoint(
        &self,
        screenpoint: ::windows::Graphics::PointInt32,
    ) -> ::windows_core::Result<::windows::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ConvertScreenToLocalWithPoint)(
                ::windows_core::Interface::as_raw(this),
                screenpoint,
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`, `\"Windows_Graphics\"`"]
    #[cfg(all(feature = "Windows_Foundation", feature = "Windows_Graphics"))]
    pub fn ConvertScreenToLocalWithPoints(
        &self,
        screenpoints: &[::windows::Graphics::PointInt32],
    ) -> ::windows_core::Result<::windows_core::Array<::windows::Foundation::Point>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows_core::Interface::vtable(this).ConvertScreenToLocalWithPoints)(
                ::windows_core::Interface::as_raw(this),
                screenpoints.len().try_into().unwrap(),
                screenpoints.as_ptr(),
                ::windows_core::Array::<::windows::Foundation::Point>::set_abi_len(
                    ::std::mem::transmute(&mut result__),
                ),
                result__.as_mut_ptr() as *mut _ as _,
            )
            .and_then(|| result__.assume_init())
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`, `\"Windows_Graphics\"`"]
    #[cfg(all(feature = "Windows_Foundation", feature = "Windows_Graphics"))]
    pub fn ConvertScreenToLocalWithRect(
        &self,
        screenrect: ::windows::Graphics::RectInt32,
    ) -> ::windows_core::Result<::windows::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ConvertScreenToLocalWithRect)(
                ::windows_core::Interface::as_raw(this),
                screenrect,
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn CreateForWindowId(
        windowid: super::WindowId,
    ) -> ::windows_core::Result<ContentCoordinateConverter> {
        Self::IContentCoordinateConverterStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateForWindowId)(
                ::windows_core::Interface::as_raw(this),
                windowid,
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IContentCoordinateConverterStatics<
        R,
        F: FnOnce(&IContentCoordinateConverterStatics) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<
            ContentCoordinateConverter,
            IContentCoordinateConverterStatics,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for ContentCoordinateConverter {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for ContentCoordinateConverter {
    type Vtable = IContentCoordinateConverter_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ContentCoordinateConverter {
    const IID: ::windows_core::GUID =
        <IContentCoordinateConverter as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ContentCoordinateConverter {
    const NAME: &'static str = "Microsoft.UI.Content.ContentCoordinateConverter";
}
::windows_core::imp::interface_hierarchy!(
    ContentCoordinateConverter,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for ContentCoordinateConverter {}
unsafe impl ::core::marker::Sync for ContentCoordinateConverter {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ContentDeferral(::windows_core::IUnknown);
impl ContentDeferral {
    pub fn Complete(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).Complete)(::windows_core::Interface::as_raw(
                this,
            ))
            .ok()
        }
    }
}
impl ::windows_core::RuntimeType for ContentDeferral {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for ContentDeferral {
    type Vtable = IContentDeferral_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ContentDeferral {
    const IID: ::windows_core::GUID = <IContentDeferral as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ContentDeferral {
    const NAME: &'static str = "Microsoft.UI.Content.ContentDeferral";
}
::windows_core::imp::interface_hierarchy!(
    ContentDeferral,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for ContentDeferral {}
unsafe impl ::core::marker::Sync for ContentDeferral {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ContentEnvironmentSettingChangedEventArgs(::windows_core::IUnknown);
impl ContentEnvironmentSettingChangedEventArgs {
    pub fn SettingName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SettingName)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for ContentEnvironmentSettingChangedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for ContentEnvironmentSettingChangedEventArgs {
    type Vtable = IContentEnvironmentSettingChangedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ContentEnvironmentSettingChangedEventArgs {
    const IID: ::windows_core::GUID =
        <IContentEnvironmentSettingChangedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ContentEnvironmentSettingChangedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Content.ContentEnvironmentSettingChangedEventArgs";
}
::windows_core::imp::interface_hierarchy!(
    ContentEnvironmentSettingChangedEventArgs,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for ContentEnvironmentSettingChangedEventArgs {}
unsafe impl ::core::marker::Sync for ContentEnvironmentSettingChangedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ContentEnvironmentStateChangedEventArgs(::windows_core::IUnknown);
impl ContentEnvironmentStateChangedEventArgs {
    pub fn DidAppWindowIdChange(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DidAppWindowIdChange)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn DidDisplayIdChange(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DidDisplayIdChange)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for ContentEnvironmentStateChangedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for ContentEnvironmentStateChangedEventArgs {
    type Vtable = IContentEnvironmentStateChangedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ContentEnvironmentStateChangedEventArgs {
    const IID: ::windows_core::GUID =
        <IContentEnvironmentStateChangedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ContentEnvironmentStateChangedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Content.ContentEnvironmentStateChangedEventArgs";
}
::windows_core::imp::interface_hierarchy!(
    ContentEnvironmentStateChangedEventArgs,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for ContentEnvironmentStateChangedEventArgs {}
unsafe impl ::core::marker::Sync for ContentEnvironmentStateChangedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ContentIsland(::windows_core::IUnknown);
impl ContentIsland {
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<::windows::Foundation::IClosable>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this))
                .ok()
        }
    }
    pub fn IsClosed(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<super::IClosableNotifier>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsClosed)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn Closed<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::ClosableNotifierHandler>,
    {
        let this = &::windows_core::ComInterface::cast::<super::IClosableNotifier>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Closed)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveClosed(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::IClosableNotifier>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveClosed)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn FrameworkClosed<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::ClosableNotifierHandler>,
    {
        let this = &::windows_core::ComInterface::cast::<super::IClosableNotifier>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FrameworkClosed)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveFrameworkClosed(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::IClosableNotifier>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveFrameworkClosed)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Composition\"`, `\"Windows_UI_Composition\"`"]
    #[cfg(all(
        feature = "Microsoft_UI_Composition",
        feature = "Windows_UI_Composition"
    ))]
    pub fn SystemBackdrop(
        &self,
    ) -> ::windows_core::Result<::windows::UI::Composition::CompositionBrush> {
        let this = &::windows_core::ComInterface::cast::<
            super::Composition::ICompositionSupportsSystemBackdrop,
        >(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SystemBackdrop)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Composition\"`, `\"Windows_UI_Composition\"`"]
    #[cfg(all(
        feature = "Microsoft_UI_Composition",
        feature = "Windows_UI_Composition"
    ))]
    pub fn SetSystemBackdrop<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<::windows::UI::Composition::CompositionBrush>,
    {
        let this = &::windows_core::ComInterface::cast::<
            super::Composition::ICompositionSupportsSystemBackdrop,
        >(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetSystemBackdrop)(
                ::windows_core::Interface::as_raw(this),
                value.try_into_param()?.abi(),
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation_Numerics\"`"]
    #[cfg(feature = "Windows_Foundation_Numerics")]
    pub fn ActualSize(&self) -> ::windows_core::Result<::windows::Foundation::Numerics::Vector2> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ActualSize)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn AppData(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AppData)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetAppData<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetAppData)(
                ::windows_core::Interface::as_raw(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn CoordinateConverter(&self) -> ::windows_core::Result<ContentCoordinateConverter> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CoordinateConverter)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation_Collections\"`"]
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub fn CustomProperties(
        &self,
    ) -> ::windows_core::Result<::windows::Foundation::Collections::IPropertySet> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CustomProperties)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Dispatching\"`"]
    #[cfg(feature = "Microsoft_UI_Dispatching")]
    pub fn DispatcherQueue(&self) -> ::windows_core::Result<super::Dispatching::DispatcherQueue> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DispatcherQueue)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn Environment(&self) -> ::windows_core::Result<ContentIslandEnvironment> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Environment)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn Id(&self) -> ::windows_core::Result<u64> {
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
    pub fn IsConnected(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsConnected)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn IsHitTestVisibleWhenTransparent(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsHitTestVisibleWhenTransparent)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetIsHitTestVisibleWhenTransparent(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetIsHitTestVisibleWhenTransparent)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsIslandEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsIslandEnabled)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetIsIslandEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetIsIslandEnabled)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsIslandVisible(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsIslandVisible)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetIsIslandVisible(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetIsIslandVisible)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsSiteEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsSiteEnabled)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn IsSiteVisible(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsSiteVisible)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn LayoutDirection(&self) -> ::windows_core::Result<ContentLayoutDirection> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LayoutDirection)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn RasterizationScale(&self) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RasterizationScale)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn GetAutomationHostProvider(
        &self,
    ) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetAutomationHostProvider)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn GetStateChangeDeferral(&self) -> ::windows_core::Result<ContentDeferral> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetStateChangeDeferral)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation_Numerics\"`"]
    #[cfg(feature = "Windows_Foundation_Numerics")]
    pub fn RequestSize(
        &self,
        size: ::windows::Foundation::Numerics::Vector2,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).RequestSize)(
                ::windows_core::Interface::as_raw(this),
                size,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn AutomationProviderRequested<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<
                ContentIsland,
                ContentIslandAutomationProviderRequestedEventArgs,
            >,
        >,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AutomationProviderRequested)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveAutomationProviderRequested(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveAutomationProviderRequested)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn StateChanged<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<
                ContentIsland,
                ContentIslandStateChangedEventArgs,
            >,
        >,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StateChanged)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveStateChanged(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveStateChanged)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Composition\"`"]
    #[cfg(feature = "Microsoft_UI_Composition")]
    pub fn Create<P0>(root: P0) -> ::windows_core::Result<ContentIsland>
    where
        P0: ::windows_core::TryIntoParam<super::Composition::Visual>,
    {
        Self::IContentIslandStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Create)(
                ::windows_core::Interface::as_raw(this),
                root.try_into_param()?.abi(),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Microsoft_UI_Composition\"`"]
    #[cfg(feature = "Microsoft_UI_Composition")]
    pub fn FindAllForCompositor<P0>(
        compositor: P0,
    ) -> ::windows_core::Result<::windows_core::Array<ContentIsland>>
    where
        P0: ::windows_core::IntoParam<super::Composition::Compositor>,
    {
        Self::IContentIslandStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows_core::Interface::vtable(this).FindAllForCompositor)(
                ::windows_core::Interface::as_raw(this),
                compositor.into_param().abi(),
                ::windows_core::Array::<ContentIsland>::set_abi_len(::std::mem::transmute(
                    &mut result__,
                )),
                result__.as_mut_ptr() as *mut _ as _,
            )
            .and_then(|| result__.assume_init())
        })
    }
    pub fn FindAllForCurrentThread() -> ::windows_core::Result<::windows_core::Array<ContentIsland>>
    {
        Self::IContentIslandStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows_core::Interface::vtable(this).FindAllForCurrentThread)(
                ::windows_core::Interface::as_raw(this),
                ::windows_core::Array::<ContentIsland>::set_abi_len(::std::mem::transmute(
                    &mut result__,
                )),
                result__.as_mut_ptr() as *mut _ as _,
            )
            .and_then(|| result__.assume_init())
        })
    }
    #[doc = "Required features: `\"Microsoft_UI_Composition\"`"]
    #[cfg(feature = "Microsoft_UI_Composition")]
    pub fn GetByVisual<P0>(child: P0) -> ::windows_core::Result<ContentIsland>
    where
        P0: ::windows_core::TryIntoParam<super::Composition::Visual>,
    {
        Self::IContentIslandStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetByVisual)(
                ::windows_core::Interface::as_raw(this),
                child.try_into_param()?.abi(),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn GetFromId(id: u64) -> ::windows_core::Result<ContentIsland> {
        Self::IContentIslandStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetFromId)(
                ::windows_core::Interface::as_raw(this),
                id,
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IContentIslandStatics<
        R,
        F: FnOnce(&IContentIslandStatics) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<ContentIsland, IContentIslandStatics> =
            ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for ContentIsland {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for ContentIsland {
    type Vtable = IContentIsland_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ContentIsland {
    const IID: ::windows_core::GUID = <IContentIsland as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ContentIsland {
    const NAME: &'static str = "Microsoft.UI.Content.ContentIsland";
}
::windows_core::imp::interface_hierarchy!(
    ContentIsland,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
#[cfg(feature = "Windows_Foundation")]
impl ::windows_core::CanTryInto<::windows::Foundation::IClosable> for ContentIsland {}
impl ::windows_core::CanTryInto<super::IClosableNotifier> for ContentIsland {}
#[cfg(feature = "Microsoft_UI_Composition")]
impl ::windows_core::CanTryInto<super::Composition::ICompositionSupportsSystemBackdrop>
    for ContentIsland
{
}
unsafe impl ::core::marker::Send for ContentIsland {}
unsafe impl ::core::marker::Sync for ContentIsland {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ContentIslandAutomationProviderRequestedEventArgs(::windows_core::IUnknown);
impl ContentIslandAutomationProviderRequestedEventArgs {
    pub fn AutomationProvider(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AutomationProvider)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetAutomationProvider<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetAutomationProvider)(
                ::windows_core::Interface::as_raw(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn Handled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Handled)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetHandled)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
}
impl ::windows_core::RuntimeType for ContentIslandAutomationProviderRequestedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for ContentIslandAutomationProviderRequestedEventArgs {
    type Vtable = IContentIslandAutomationProviderRequestedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ContentIslandAutomationProviderRequestedEventArgs {
    const IID: ::windows_core::GUID =
        <IContentIslandAutomationProviderRequestedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ContentIslandAutomationProviderRequestedEventArgs {
    const NAME: &'static str =
        "Microsoft.UI.Content.ContentIslandAutomationProviderRequestedEventArgs";
}
::windows_core::imp::interface_hierarchy!(
    ContentIslandAutomationProviderRequestedEventArgs,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for ContentIslandAutomationProviderRequestedEventArgs {}
unsafe impl ::core::marker::Sync for ContentIslandAutomationProviderRequestedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ContentIslandEnvironment(::windows_core::IUnknown);
impl ContentIslandEnvironment {
    pub fn AppWindowId(&self) -> ::windows_core::Result<super::WindowId> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AppWindowId)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn DisplayId(&self) -> ::windows_core::Result<super::DisplayId> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DisplayId)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn SettingChanged<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<
                ContentIslandEnvironment,
                ContentEnvironmentSettingChangedEventArgs,
            >,
        >,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SettingChanged)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveSettingChanged(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveSettingChanged)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn StateChanged<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<
                ContentIslandEnvironment,
                ContentEnvironmentStateChangedEventArgs,
            >,
        >,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StateChanged)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveStateChanged(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveStateChanged)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
}
impl ::windows_core::RuntimeType for ContentIslandEnvironment {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for ContentIslandEnvironment {
    type Vtable = IContentIslandEnvironment_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ContentIslandEnvironment {
    const IID: ::windows_core::GUID =
        <IContentIslandEnvironment as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ContentIslandEnvironment {
    const NAME: &'static str = "Microsoft.UI.Content.ContentIslandEnvironment";
}
::windows_core::imp::interface_hierarchy!(
    ContentIslandEnvironment,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for ContentIslandEnvironment {}
unsafe impl ::core::marker::Sync for ContentIslandEnvironment {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ContentIslandStateChangedEventArgs(::windows_core::IUnknown);
impl ContentIslandStateChangedEventArgs {
    pub fn DidActualSizeChange(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DidActualSizeChange)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn DidSiteEnabledChange(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DidSiteEnabledChange)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn DidSiteVisibleChange(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DidSiteVisibleChange)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn DidLayoutDirectionChange(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DidLayoutDirectionChange)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn DidRasterizationScaleChange(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DidRasterizationScaleChange)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for ContentIslandStateChangedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for ContentIslandStateChangedEventArgs {
    type Vtable = IContentIslandStateChangedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ContentIslandStateChangedEventArgs {
    const IID: ::windows_core::GUID =
        <IContentIslandStateChangedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ContentIslandStateChangedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Content.ContentIslandStateChangedEventArgs";
}
::windows_core::imp::interface_hierarchy!(
    ContentIslandStateChangedEventArgs,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for ContentIslandStateChangedEventArgs {}
unsafe impl ::core::marker::Sync for ContentIslandStateChangedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ContentSite(::windows_core::IUnknown);
impl ContentSite {
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<::windows::Foundation::IClosable>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this))
                .ok()
        }
    }
    pub fn IsClosed(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<super::IClosableNotifier>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsClosed)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn Closed<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::ClosableNotifierHandler>,
    {
        let this = &::windows_core::ComInterface::cast::<super::IClosableNotifier>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Closed)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveClosed(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::IClosableNotifier>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveClosed)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn FrameworkClosed<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::ClosableNotifierHandler>,
    {
        let this = &::windows_core::ComInterface::cast::<super::IClosableNotifier>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FrameworkClosed)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveFrameworkClosed(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::IClosableNotifier>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveFrameworkClosed)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation_Numerics\"`"]
    #[cfg(feature = "Windows_Foundation_Numerics")]
    pub fn ActualSize(&self) -> ::windows_core::Result<::windows::Foundation::Numerics::Vector2> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ActualSize)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation_Numerics\"`"]
    #[cfg(feature = "Windows_Foundation_Numerics")]
    pub fn SetActualSize(
        &self,
        value: ::windows::Foundation::Numerics::Vector2,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetActualSize)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Graphics\"`"]
    #[cfg(feature = "Windows_Graphics")]
    pub fn ClientSize(&self) -> ::windows_core::Result<::windows::Graphics::SizeInt32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ClientSize)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Graphics\"`"]
    #[cfg(feature = "Windows_Graphics")]
    pub fn SetClientSize(
        &self,
        value: ::windows::Graphics::SizeInt32,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetClientSize)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn CoordinateConverter(&self) -> ::windows_core::Result<ContentCoordinateConverter> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CoordinateConverter)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Dispatching\"`"]
    #[cfg(feature = "Microsoft_UI_Dispatching")]
    pub fn DispatcherQueue(&self) -> ::windows_core::Result<super::Dispatching::DispatcherQueue> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DispatcherQueue)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn Environment(&self) -> ::windows_core::Result<ContentSiteEnvironment> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Environment)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn IsConnected(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsConnected)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn IsSiteEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsSiteEnabled)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetIsSiteEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetIsSiteEnabled)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsSiteVisible(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsSiteVisible)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetIsSiteVisible(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetIsSiteVisible)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn LayoutDirection(&self) -> ::windows_core::Result<ContentLayoutDirection> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LayoutDirection)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetLayoutDirection(&self, value: ContentLayoutDirection) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetLayoutDirection)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn OverrideScale(&self) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OverrideScale)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetOverrideScale(&self, value: f32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetOverrideScale)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn ParentScale(&self) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ParentScale)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetParentScale(&self, value: f32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetParentScale)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn RasterizationScale(&self) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RasterizationScale)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation_Numerics\"`"]
    #[cfg(feature = "Windows_Foundation_Numerics")]
    pub fn RequestedSize(
        &self,
    ) -> ::windows_core::Result<::windows::Foundation::Numerics::Vector2> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RequestedSize)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn ShouldApplyRasterizationScale(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ShouldApplyRasterizationScale)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetShouldApplyRasterizationScale(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetShouldApplyRasterizationScale)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn View(&self) -> ::windows_core::Result<ContentSiteView> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).View)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn GetIslandStateChangeDeferral(&self) -> ::windows_core::Result<ContentDeferral> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetIslandStateChangeDeferral)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RequestedStateChanged<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<
                ContentSite,
                ContentSiteRequestedStateChangedEventArgs,
            >,
        >,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RequestedStateChanged)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveRequestedStateChanged(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveRequestedStateChanged)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
}
impl ::windows_core::RuntimeType for ContentSite {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for ContentSite {
    type Vtable = IContentSite_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ContentSite {
    const IID: ::windows_core::GUID = <IContentSite as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ContentSite {
    const NAME: &'static str = "Microsoft.UI.Content.ContentSite";
}
::windows_core::imp::interface_hierarchy!(
    ContentSite,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
#[cfg(feature = "Windows_Foundation")]
impl ::windows_core::CanTryInto<::windows::Foundation::IClosable> for ContentSite {}
impl ::windows_core::CanTryInto<super::IClosableNotifier> for ContentSite {}
unsafe impl ::core::marker::Send for ContentSite {}
unsafe impl ::core::marker::Sync for ContentSite {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ContentSiteEnvironment(::windows_core::IUnknown);
impl ContentSiteEnvironment {
    pub fn AppWindowId(&self) -> ::windows_core::Result<super::WindowId> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AppWindowId)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetAppWindowId(&self, value: super::WindowId) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetAppWindowId)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn DisplayId(&self) -> ::windows_core::Result<super::DisplayId> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DisplayId)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetDisplayId(&self, value: super::DisplayId) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetDisplayId)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn View(&self) -> ::windows_core::Result<ContentSiteEnvironmentView> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).View)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn NotifySettingChanged(
        &self,
        setting: &::windows_core::HSTRING,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).NotifySettingChanged)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(setting),
            )
            .ok()
        }
    }
}
impl ::windows_core::RuntimeType for ContentSiteEnvironment {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for ContentSiteEnvironment {
    type Vtable = IContentSiteEnvironment_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ContentSiteEnvironment {
    const IID: ::windows_core::GUID =
        <IContentSiteEnvironment as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ContentSiteEnvironment {
    const NAME: &'static str = "Microsoft.UI.Content.ContentSiteEnvironment";
}
::windows_core::imp::interface_hierarchy!(
    ContentSiteEnvironment,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for ContentSiteEnvironment {}
unsafe impl ::core::marker::Sync for ContentSiteEnvironment {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ContentSiteEnvironmentView(::windows_core::IUnknown);
impl ContentSiteEnvironmentView {
    pub fn AppWindowId(&self) -> ::windows_core::Result<super::WindowId> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AppWindowId)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn DisplayId(&self) -> ::windows_core::Result<super::DisplayId> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DisplayId)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for ContentSiteEnvironmentView {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for ContentSiteEnvironmentView {
    type Vtable = IContentSiteEnvironmentView_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ContentSiteEnvironmentView {
    const IID: ::windows_core::GUID =
        <IContentSiteEnvironmentView as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ContentSiteEnvironmentView {
    const NAME: &'static str = "Microsoft.UI.Content.ContentSiteEnvironmentView";
}
::windows_core::imp::interface_hierarchy!(
    ContentSiteEnvironmentView,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for ContentSiteEnvironmentView {}
unsafe impl ::core::marker::Sync for ContentSiteEnvironmentView {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ContentSiteRequestedStateChangedEventArgs(::windows_core::IUnknown);
impl ContentSiteRequestedStateChangedEventArgs {
    pub fn DidRequestedSizeChange(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DidRequestedSizeChange)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for ContentSiteRequestedStateChangedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for ContentSiteRequestedStateChangedEventArgs {
    type Vtable = IContentSiteRequestedStateChangedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ContentSiteRequestedStateChangedEventArgs {
    const IID: ::windows_core::GUID =
        <IContentSiteRequestedStateChangedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ContentSiteRequestedStateChangedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Content.ContentSiteRequestedStateChangedEventArgs";
}
::windows_core::imp::interface_hierarchy!(
    ContentSiteRequestedStateChangedEventArgs,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for ContentSiteRequestedStateChangedEventArgs {}
unsafe impl ::core::marker::Sync for ContentSiteRequestedStateChangedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ContentSiteView(::windows_core::IUnknown);
impl ContentSiteView {
    #[doc = "Required features: `\"Windows_Foundation_Numerics\"`"]
    #[cfg(feature = "Windows_Foundation_Numerics")]
    pub fn ActualSize(&self) -> ::windows_core::Result<::windows::Foundation::Numerics::Vector2> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ActualSize)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Graphics\"`"]
    #[cfg(feature = "Windows_Graphics")]
    pub fn ClientSize(&self) -> ::windows_core::Result<::windows::Graphics::SizeInt32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ClientSize)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn CoordinateConverter(&self) -> ::windows_core::Result<ContentCoordinateConverter> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CoordinateConverter)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Dispatching\"`"]
    #[cfg(feature = "Microsoft_UI_Dispatching")]
    pub fn DispatcherQueue(&self) -> ::windows_core::Result<super::Dispatching::DispatcherQueue> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DispatcherQueue)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn EnvironmentView(&self) -> ::windows_core::Result<ContentSiteEnvironmentView> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).EnvironmentView)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn IsConnected(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsConnected)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn IsSiteEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsSiteEnabled)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn IsSiteVisible(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsSiteVisible)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn LayoutDirection(&self) -> ::windows_core::Result<ContentLayoutDirection> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LayoutDirection)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn OverrideScale(&self) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OverrideScale)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn ParentScale(&self) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ParentScale)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn RasterizationScale(&self) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RasterizationScale)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation_Numerics\"`"]
    #[cfg(feature = "Windows_Foundation_Numerics")]
    pub fn RequestedSize(
        &self,
    ) -> ::windows_core::Result<::windows::Foundation::Numerics::Vector2> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RequestedSize)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn ShouldApplyRasterizationScale(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ShouldApplyRasterizationScale)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for ContentSiteView {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for ContentSiteView {
    type Vtable = IContentSiteView_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ContentSiteView {
    const IID: ::windows_core::GUID = <IContentSiteView as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ContentSiteView {
    const NAME: &'static str = "Microsoft.UI.Content.ContentSiteView";
}
::windows_core::imp::interface_hierarchy!(
    ContentSiteView,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for ContentSiteView {}
unsafe impl ::core::marker::Sync for ContentSiteView {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct DesktopChildSiteBridge(::windows_core::IUnknown);
impl DesktopChildSiteBridge {
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<::windows::Foundation::IClosable>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this))
                .ok()
        }
    }
    pub fn IsClosed(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<super::IClosableNotifier>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsClosed)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn Closed<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::ClosableNotifierHandler>,
    {
        let this = &::windows_core::ComInterface::cast::<super::IClosableNotifier>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Closed)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveClosed(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::IClosableNotifier>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveClosed)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn FrameworkClosed<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::ClosableNotifierHandler>,
    {
        let this = &::windows_core::ComInterface::cast::<super::IClosableNotifier>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FrameworkClosed)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveFrameworkClosed(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::IClosableNotifier>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveFrameworkClosed)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Dispatching\"`"]
    #[cfg(feature = "Microsoft_UI_Dispatching")]
    pub fn DispatcherQueue(&self) -> ::windows_core::Result<super::Dispatching::DispatcherQueue> {
        let this = &::windows_core::ComInterface::cast::<IContentSiteBridge>(self)?;
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
    pub fn LayoutDirectionOverride(
        &self,
    ) -> ::windows_core::Result<::windows::Foundation::IReference<ContentLayoutDirection>> {
        let this = &::windows_core::ComInterface::cast::<IContentSiteBridge>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LayoutDirectionOverride)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn SetLayoutDirectionOverride<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<::windows::Foundation::IReference<ContentLayoutDirection>>,
    {
        let this = &::windows_core::ComInterface::cast::<IContentSiteBridge>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetLayoutDirectionOverride)(
                ::windows_core::Interface::as_raw(this),
                value.try_into_param()?.abi(),
            )
            .ok()
        }
    }
    pub fn OverrideScale(&self) -> ::windows_core::Result<f32> {
        let this = &::windows_core::ComInterface::cast::<IContentSiteBridge>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OverrideScale)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetOverrideScale(&self, value: f32) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IContentSiteBridge>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetOverrideScale)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn ResizePolicy(&self) -> ::windows_core::Result<ContentSizePolicy> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ResizePolicy)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetResizePolicy(&self, value: ContentSizePolicy) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetResizePolicy)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn SiteView(&self) -> ::windows_core::Result<ContentSiteView> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SiteView)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Composition\"`"]
    #[cfg(feature = "Microsoft_UI_Composition")]
    pub fn Create<P0>(
        compositor: P0,
        parentwindowid: super::WindowId,
    ) -> ::windows_core::Result<DesktopChildSiteBridge>
    where
        P0: ::windows_core::IntoParam<super::Composition::Compositor>,
    {
        Self::IDesktopChildSiteBridgeStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Create)(
                ::windows_core::Interface::as_raw(this),
                compositor.into_param().abi(),
                parentwindowid,
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn IsEnabled(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IDesktopSiteBridge>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsEnabled)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn IsVisible(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IDesktopSiteBridge>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsVisible)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn WindowId(&self) -> ::windows_core::Result<super::WindowId> {
        let this = &::windows_core::ComInterface::cast::<IDesktopSiteBridge>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).WindowId)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn Connect<P0>(&self, content: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<ContentIsland>,
    {
        let this = &::windows_core::ComInterface::cast::<IDesktopSiteBridge>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).Connect)(
                ::windows_core::Interface::as_raw(this),
                content.try_into_param()?.abi(),
            )
            .ok()
        }
    }
    pub fn Disable(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IDesktopSiteBridge>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).Disable)(::windows_core::Interface::as_raw(
                this,
            ))
            .ok()
        }
    }
    pub fn Enable(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IDesktopSiteBridge>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).Enable)(::windows_core::Interface::as_raw(
                this,
            ))
            .ok()
        }
    }
    pub fn Hide(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IDesktopSiteBridge>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).Hide)(::windows_core::Interface::as_raw(this))
                .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Graphics\"`"]
    #[cfg(feature = "Windows_Graphics")]
    pub fn MoveAndResize(
        &self,
        rect: ::windows::Graphics::RectInt32,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IDesktopSiteBridge>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).MoveAndResize)(
                ::windows_core::Interface::as_raw(this),
                rect,
            )
            .ok()
        }
    }
    pub fn MoveInZOrderAtBottom(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IDesktopSiteBridge>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).MoveInZOrderAtBottom)(
                ::windows_core::Interface::as_raw(this),
            )
            .ok()
        }
    }
    pub fn MoveInZOrderAtTop(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IDesktopSiteBridge>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).MoveInZOrderAtTop)(
                ::windows_core::Interface::as_raw(this),
            )
            .ok()
        }
    }
    pub fn MoveInZOrderBelow(&self, windowid: super::WindowId) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IDesktopSiteBridge>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).MoveInZOrderBelow)(
                ::windows_core::Interface::as_raw(this),
                windowid,
            )
            .ok()
        }
    }
    pub fn Show(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IDesktopSiteBridge>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).Show)(::windows_core::Interface::as_raw(this))
                .ok()
        }
    }
    #[doc(hidden)]
    pub fn IDesktopChildSiteBridgeStatics<
        R,
        F: FnOnce(&IDesktopChildSiteBridgeStatics) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<
            DesktopChildSiteBridge,
            IDesktopChildSiteBridgeStatics,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for DesktopChildSiteBridge {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for DesktopChildSiteBridge {
    type Vtable = IDesktopChildSiteBridge_Vtbl;
}
unsafe impl ::windows_core::ComInterface for DesktopChildSiteBridge {
    const IID: ::windows_core::GUID =
        <IDesktopChildSiteBridge as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for DesktopChildSiteBridge {
    const NAME: &'static str = "Microsoft.UI.Content.DesktopChildSiteBridge";
}
::windows_core::imp::interface_hierarchy!(
    DesktopChildSiteBridge,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
#[cfg(feature = "Windows_Foundation")]
impl ::windows_core::CanTryInto<::windows::Foundation::IClosable> for DesktopChildSiteBridge {}
impl ::windows_core::CanTryInto<super::IClosableNotifier> for DesktopChildSiteBridge {}
impl ::windows_core::CanTryInto<IContentSiteBridge> for DesktopChildSiteBridge {}
impl ::windows_core::CanTryInto<DesktopSiteBridge> for DesktopChildSiteBridge {}
unsafe impl ::core::marker::Send for DesktopChildSiteBridge {}
unsafe impl ::core::marker::Sync for DesktopChildSiteBridge {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct DesktopSiteBridge(::windows_core::IUnknown);
impl DesktopSiteBridge {
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<::windows::Foundation::IClosable>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this))
                .ok()
        }
    }
    pub fn IsClosed(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<super::IClosableNotifier>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsClosed)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn Closed<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::ClosableNotifierHandler>,
    {
        let this = &::windows_core::ComInterface::cast::<super::IClosableNotifier>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Closed)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveClosed(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::IClosableNotifier>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveClosed)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn FrameworkClosed<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::ClosableNotifierHandler>,
    {
        let this = &::windows_core::ComInterface::cast::<super::IClosableNotifier>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FrameworkClosed)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveFrameworkClosed(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::IClosableNotifier>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveFrameworkClosed)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Dispatching\"`"]
    #[cfg(feature = "Microsoft_UI_Dispatching")]
    pub fn DispatcherQueue(&self) -> ::windows_core::Result<super::Dispatching::DispatcherQueue> {
        let this = &::windows_core::ComInterface::cast::<IContentSiteBridge>(self)?;
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
    pub fn LayoutDirectionOverride(
        &self,
    ) -> ::windows_core::Result<::windows::Foundation::IReference<ContentLayoutDirection>> {
        let this = &::windows_core::ComInterface::cast::<IContentSiteBridge>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LayoutDirectionOverride)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn SetLayoutDirectionOverride<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<::windows::Foundation::IReference<ContentLayoutDirection>>,
    {
        let this = &::windows_core::ComInterface::cast::<IContentSiteBridge>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetLayoutDirectionOverride)(
                ::windows_core::Interface::as_raw(this),
                value.try_into_param()?.abi(),
            )
            .ok()
        }
    }
    pub fn OverrideScale(&self) -> ::windows_core::Result<f32> {
        let this = &::windows_core::ComInterface::cast::<IContentSiteBridge>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OverrideScale)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetOverrideScale(&self, value: f32) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IContentSiteBridge>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetOverrideScale)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsEnabled)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn IsVisible(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsVisible)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn WindowId(&self) -> ::windows_core::Result<super::WindowId> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).WindowId)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn Connect<P0>(&self, content: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<ContentIsland>,
    {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).Connect)(
                ::windows_core::Interface::as_raw(this),
                content.try_into_param()?.abi(),
            )
            .ok()
        }
    }
    pub fn Disable(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).Disable)(::windows_core::Interface::as_raw(
                this,
            ))
            .ok()
        }
    }
    pub fn Enable(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).Enable)(::windows_core::Interface::as_raw(
                this,
            ))
            .ok()
        }
    }
    pub fn Hide(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).Hide)(::windows_core::Interface::as_raw(this))
                .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Graphics\"`"]
    #[cfg(feature = "Windows_Graphics")]
    pub fn MoveAndResize(
        &self,
        rect: ::windows::Graphics::RectInt32,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).MoveAndResize)(
                ::windows_core::Interface::as_raw(this),
                rect,
            )
            .ok()
        }
    }
    pub fn MoveInZOrderAtBottom(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).MoveInZOrderAtBottom)(
                ::windows_core::Interface::as_raw(this),
            )
            .ok()
        }
    }
    pub fn MoveInZOrderAtTop(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).MoveInZOrderAtTop)(
                ::windows_core::Interface::as_raw(this),
            )
            .ok()
        }
    }
    pub fn MoveInZOrderBelow(&self, windowid: super::WindowId) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).MoveInZOrderBelow)(
                ::windows_core::Interface::as_raw(this),
                windowid,
            )
            .ok()
        }
    }
    pub fn Show(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).Show)(::windows_core::Interface::as_raw(this))
                .ok()
        }
    }
    pub fn IsSupported() -> ::windows_core::Result<bool> {
        Self::IDesktopSiteBridgeStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsSupported)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IDesktopSiteBridgeStatics<
        R,
        F: FnOnce(&IDesktopSiteBridgeStatics) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<
            DesktopSiteBridge,
            IDesktopSiteBridgeStatics,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for DesktopSiteBridge {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for DesktopSiteBridge {
    type Vtable = IDesktopSiteBridge_Vtbl;
}
unsafe impl ::windows_core::ComInterface for DesktopSiteBridge {
    const IID: ::windows_core::GUID = <IDesktopSiteBridge as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for DesktopSiteBridge {
    const NAME: &'static str = "Microsoft.UI.Content.DesktopSiteBridge";
}
::windows_core::imp::interface_hierarchy!(
    DesktopSiteBridge,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
#[cfg(feature = "Windows_Foundation")]
impl ::windows_core::CanTryInto<::windows::Foundation::IClosable> for DesktopSiteBridge {}
impl ::windows_core::CanTryInto<super::IClosableNotifier> for DesktopSiteBridge {}
impl ::windows_core::CanTryInto<IContentSiteBridge> for DesktopSiteBridge {}
unsafe impl ::core::marker::Send for DesktopSiteBridge {}
unsafe impl ::core::marker::Sync for DesktopSiteBridge {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ContentCoordinateRoundingMode(pub i32);
impl ContentCoordinateRoundingMode {
    pub const Auto: Self = Self(0i32);
    pub const Floor: Self = Self(1i32);
    pub const Round: Self = Self(2i32);
    pub const Ceiling: Self = Self(3i32);
}
impl ::core::marker::Copy for ContentCoordinateRoundingMode {}
impl ::core::clone::Clone for ContentCoordinateRoundingMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ContentCoordinateRoundingMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for ContentCoordinateRoundingMode {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for ContentCoordinateRoundingMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContentCoordinateRoundingMode").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ContentCoordinateRoundingMode {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::from_slice(
            b"enum(Microsoft.UI.Content.ContentCoordinateRoundingMode;i4)",
        );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ContentLayoutDirection(pub i32);
impl ContentLayoutDirection {
    pub const LeftToRight: Self = Self(0i32);
    pub const RightToLeft: Self = Self(1i32);
}
impl ::core::marker::Copy for ContentLayoutDirection {}
impl ::core::clone::Clone for ContentLayoutDirection {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ContentLayoutDirection {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for ContentLayoutDirection {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for ContentLayoutDirection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContentLayoutDirection").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ContentLayoutDirection {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::from_slice(
            b"enum(Microsoft.UI.Content.ContentLayoutDirection;i4)",
        );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ContentSizePolicy(pub i32);
impl ContentSizePolicy {
    pub const None: Self = Self(0i32);
    pub const ResizeContentToParentWindow: Self = Self(1i32);
    pub const ResizeParentWindowToContent: Self = Self(2i32);
}
impl ::core::marker::Copy for ContentSizePolicy {}
impl ::core::clone::Clone for ContentSizePolicy {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ContentSizePolicy {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for ContentSizePolicy {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for ContentSizePolicy {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContentSizePolicy").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ContentSizePolicy {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::from_slice(
            b"enum(Microsoft.UI.Content.ContentSizePolicy;i4)",
        );
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
