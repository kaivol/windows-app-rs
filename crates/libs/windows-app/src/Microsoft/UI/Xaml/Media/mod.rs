#[cfg(feature = "Microsoft_UI_Xaml_Media_Animation")]
#[doc = "Required features: `\"Microsoft_UI_Xaml_Media_Animation\"`"]
pub mod Animation;
#[cfg(feature = "Microsoft_UI_Xaml_Media_Imaging")]
#[doc = "Required features: `\"Microsoft_UI_Xaml_Media_Imaging\"`"]
pub mod Imaging;
#[cfg(feature = "Microsoft_UI_Xaml_Media_Media3D")]
#[doc = "Required features: `\"Microsoft_UI_Xaml_Media_Media3D\"`"]
pub mod Media3D;
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IAcrylicBrush(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAcrylicBrush {
    type Vtable = IAcrylicBrush_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IAcrylicBrush {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x3a8c760a_941f_58bc_a6d4_aa7a0dd1d036);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAcrylicBrush_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Windows_UI")]
    pub TintColor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI"))]
    TintColor: usize,
    #[cfg(feature = "Windows_UI")]
    pub SetTintColor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::windows::UI::Color,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI"))]
    SetTintColor: usize,
    pub TintOpacity: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub SetTintOpacity: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_Foundation")]
    pub TintTransitionDuration: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::TimeSpan,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    TintTransitionDuration: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub SetTintTransitionDuration: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::windows::Foundation::TimeSpan,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    SetTintTransitionDuration: usize,
    pub AlwaysUseFallback: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetAlwaysUseFallback: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IAcrylicBrush2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAcrylicBrush2 {
    type Vtable = IAcrylicBrush2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IAcrylicBrush2 {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x23fad570_43ed_5a73_9de7_a303553d5414);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAcrylicBrush2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Windows_Foundation")]
    pub TintLuminosityOpacity: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    TintLuminosityOpacity: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub SetTintLuminosityOpacity: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    SetTintLuminosityOpacity: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IAcrylicBrushFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAcrylicBrushFactory {
    type Vtable = IAcrylicBrushFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IAcrylicBrushFactory {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x80173353_611d_5a02_8864_1aaa279dff1c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAcrylicBrushFactory_Vtbl {
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
pub struct IAcrylicBrushStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAcrylicBrushStatics {
    type Vtable = IAcrylicBrushStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IAcrylicBrushStatics {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x9d9d366b_00a3_5f3e_98b8_1df7fec1828c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAcrylicBrushStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub TintColorProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub TintOpacityProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub TintTransitionDurationProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub AlwaysUseFallbackProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IAcrylicBrushStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAcrylicBrushStatics2 {
    type Vtable = IAcrylicBrushStatics2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IAcrylicBrushStatics2 {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x6e3eb0bd_20a1_52ea_aede_478061012279);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAcrylicBrushStatics2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub TintLuminosityOpacityProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IArcSegment(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IArcSegment {
    type Vtable = IArcSegment_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IArcSegment {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x6b7ce02b_87be_5acb_9d3b_c9964c6962d0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IArcSegment_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Windows_Foundation")]
    pub Point: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::Point,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    Point: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub SetPoint: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::windows::Foundation::Point,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    SetPoint: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub Size: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::Size,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    Size: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub SetSize: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::windows::Foundation::Size,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    SetSize: usize,
    pub RotationAngle: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub SetRotationAngle: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
    ) -> ::windows_core::HRESULT,
    pub IsLargeArc: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetIsLargeArc: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
    pub SweepDirection: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut SweepDirection,
    ) -> ::windows_core::HRESULT,
    pub SetSweepDirection: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: SweepDirection,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IArcSegmentStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IArcSegmentStatics {
    type Vtable = IArcSegmentStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IArcSegmentStatics {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x5ba7ccb3_5bc7_5038_99c5_93dc730230cf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IArcSegmentStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub PointProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SizeProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub RotationAngleProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub IsLargeArcProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SweepDirectionProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IBezierSegment(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBezierSegment {
    type Vtable = IBezierSegment_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IBezierSegment {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x0f36bade_892e_51fe_b94a_3875e86feaae);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBezierSegment_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Windows_Foundation")]
    pub Point1: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::Point,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    Point1: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub SetPoint1: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::windows::Foundation::Point,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    SetPoint1: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub Point2: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::Point,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    Point2: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub SetPoint2: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::windows::Foundation::Point,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    SetPoint2: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub Point3: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::Point,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    Point3: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub SetPoint3: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::windows::Foundation::Point,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    SetPoint3: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IBezierSegmentStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBezierSegmentStatics {
    type Vtable = IBezierSegmentStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IBezierSegmentStatics {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x98e74d5c_c97a_50b0_ae0e_d436dc9df16d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBezierSegmentStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Point1Property: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub Point2Property: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub Point3Property: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IBitmapCache(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBitmapCache {
    type Vtable = IBitmapCache_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IBitmapCache {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x4b3a8983_27a2_592a_bda4_270431eae038);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBitmapCache_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IBrush(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBrush {
    type Vtable = IBrush_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IBrush {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x2de3cb83_1329_5679_88f8_c822bc5442cb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBrush_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Opacity: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub SetOpacity: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
    ) -> ::windows_core::HRESULT,
    pub Transform: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetTransform: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub RelativeTransform: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetRelativeTransform: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IBrushFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBrushFactory {
    type Vtable = IBrushFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IBrushFactory {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xb5258717_6c49_5ba5_87fd_35df382647a5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBrushFactory_Vtbl {
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
pub struct IBrushOverrides(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBrushOverrides {
    type Vtable = IBrushOverrides_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IBrushOverrides {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xb6b08394_bacf_53db_9ac7_be1c693e3513);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBrushOverrides_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Microsoft_UI_Composition")]
    pub PopulatePropertyInfoOverride: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        propertyname: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        animationpropertyinfo: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Microsoft_UI_Composition"))]
    PopulatePropertyInfoOverride: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IBrushStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBrushStatics {
    type Vtable = IBrushStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IBrushStatics {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x5b854f50_f818_5f01_91b0_28132d3f5957);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBrushStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub OpacityProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub TransformProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub RelativeTransformProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICacheMode(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICacheMode {
    type Vtable = ICacheMode_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICacheMode {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x2ff1a1cb_0f48_53fd_b1de_e2223dfb2ff6);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICacheMode_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICacheModeFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICacheModeFactory {
    type Vtable = ICacheModeFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICacheModeFactory {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xe257811e_dcc5_51d8_829a_3e9400198a41);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICacheModeFactory_Vtbl {
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
pub struct ICompositeTransform(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICompositeTransform {
    type Vtable = ICompositeTransform_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICompositeTransform {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x55c5f8f3_20e4_5b80_a046_ce4d0f62f2fe);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompositeTransform_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CenterX: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub SetCenterX: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
    ) -> ::windows_core::HRESULT,
    pub CenterY: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub SetCenterY: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
    ) -> ::windows_core::HRESULT,
    pub ScaleX: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub SetScaleX: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
    ) -> ::windows_core::HRESULT,
    pub ScaleY: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub SetScaleY: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
    ) -> ::windows_core::HRESULT,
    pub SkewX: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub SetSkewX: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
    ) -> ::windows_core::HRESULT,
    pub SkewY: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub SetSkewY: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
    ) -> ::windows_core::HRESULT,
    pub Rotation: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub SetRotation: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
    ) -> ::windows_core::HRESULT,
    pub TranslateX: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub SetTranslateX: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
    ) -> ::windows_core::HRESULT,
    pub TranslateY: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub SetTranslateY: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICompositeTransformStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICompositeTransformStatics {
    type Vtable = ICompositeTransformStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICompositeTransformStatics {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x7701385b_8eab_5071_bfa5_b453e1e52b43);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompositeTransformStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CenterXProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub CenterYProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub ScaleXProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub ScaleYProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SkewXProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SkewYProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub RotationProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub TranslateXProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub TranslateYProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICompositionTarget(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICompositionTarget {
    type Vtable = ICompositionTarget_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICompositionTarget {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x7d938324_e3ad_597c_93f6_520725410e68);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompositionTarget_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICompositionTargetStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICompositionTargetStatics {
    type Vtable = ICompositionTargetStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICompositionTargetStatics {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x12a4be6f_6db1_5165_b622_d57ab782745b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompositionTargetStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Windows_Foundation")]
    pub Rendering: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    Rendering: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub RemoveRendering: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RemoveRendering: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub Rendered: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    Rendered: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub RemoveRendered: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RemoveRendered: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub SurfaceContentsLost: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    SurfaceContentsLost: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub RemoveSurfaceContentsLost: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RemoveSurfaceContentsLost: usize,
    #[cfg(feature = "Microsoft_UI_Composition")]
    pub GetCompositorForCurrentThread: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Microsoft_UI_Composition"))]
    GetCompositorForCurrentThread: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDesktopAcrylicBackdrop(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDesktopAcrylicBackdrop {
    type Vtable = IDesktopAcrylicBackdrop_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDesktopAcrylicBackdrop {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xbfd9915b_82a6_5df6_aff0_a4824ddc1143);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDesktopAcrylicBackdrop_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDesktopAcrylicBackdropFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDesktopAcrylicBackdropFactory {
    type Vtable = IDesktopAcrylicBackdropFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDesktopAcrylicBackdropFactory {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x00922e6d_ae51_564a_bce2_1973d5e463dd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDesktopAcrylicBackdropFactory_Vtbl {
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
pub struct IEllipseGeometry(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IEllipseGeometry {
    type Vtable = IEllipseGeometry_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IEllipseGeometry {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xababd262_d8e4_5b49_bce9_0108a5209d45);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEllipseGeometry_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Windows_Foundation")]
    pub Center: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::Point,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    Center: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub SetCenter: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::windows::Foundation::Point,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    SetCenter: usize,
    pub RadiusX: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub SetRadiusX: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
    ) -> ::windows_core::HRESULT,
    pub RadiusY: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub SetRadiusY: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IEllipseGeometryStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IEllipseGeometryStatics {
    type Vtable = IEllipseGeometryStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IEllipseGeometryStatics {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xe8a33c80_d72f_5248_a71f_4b70a0757f89);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEllipseGeometryStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CenterProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub RadiusXProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub RadiusYProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IFontFamily(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFontFamily {
    type Vtable = IFontFamily_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IFontFamily {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x18fa5bc1_7294_527c_bb02_b213e0b3a2a3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFontFamily_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Source: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IFontFamilyFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFontFamilyFactory {
    type Vtable = IFontFamilyFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IFontFamilyFactory {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x61b88a77_d0f9_5e9e_8c28_eda01fede22e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFontFamilyFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateInstanceWithName: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        familyname: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        baseinterface: *mut ::core::ffi::c_void,
        innerinterface: *mut *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IFontFamilyStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFontFamilyStatics {
    type Vtable = IFontFamilyStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IFontFamilyStatics {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xb3eadceb_c471_58fe_93d0_d71b04a7fd54);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFontFamilyStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub XamlAutoFontFamily: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IGeneralTransform(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGeneralTransform {
    type Vtable = IGeneralTransform_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IGeneralTransform {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x04eedeeb_31e5_54c0_ae3f_8bd06645d339);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeneralTransform_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Inverse: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_Foundation")]
    pub TransformPoint: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        point: ::windows::Foundation::Point,
        result__: *mut ::windows::Foundation::Point,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    TransformPoint: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub TryTransform: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        inpoint: ::windows::Foundation::Point,
        outpoint: *mut ::windows::Foundation::Point,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    TryTransform: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub TransformBounds: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        rect: ::windows::Foundation::Rect,
        result__: *mut ::windows::Foundation::Rect,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    TransformBounds: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IGeneralTransformFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGeneralTransformFactory {
    type Vtable = IGeneralTransformFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IGeneralTransformFactory {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x2f1025a3_5391_5d1b_8382_3caaa1d26a96);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeneralTransformFactory_Vtbl {
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
pub struct IGeneralTransformOverrides(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGeneralTransformOverrides {
    type Vtable = IGeneralTransformOverrides_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IGeneralTransformOverrides {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xce8970f1_83f8_543f_9cf5_439c461601f1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeneralTransformOverrides_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub InverseCore: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_Foundation")]
    pub TryTransformCore: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        inpoint: ::windows::Foundation::Point,
        outpoint: *mut ::windows::Foundation::Point,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    TryTransformCore: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub TransformBoundsCore: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        rect: ::windows::Foundation::Rect,
        result__: *mut ::windows::Foundation::Rect,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    TransformBoundsCore: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IGeometry(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGeometry {
    type Vtable = IGeometry_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IGeometry {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xdc102dcc_3be2_5414_8599_94b6e76ef39b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeometry_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Transform: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetTransform: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_Foundation")]
    pub Bounds: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::Rect,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    Bounds: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IGeometryFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGeometryFactory {
    type Vtable = IGeometryFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IGeometryFactory {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x4edcd536_7949_548a_a9b1_6ff03b951cf3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeometryFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IGeometryGroup(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGeometryGroup {
    type Vtable = IGeometryGroup_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IGeometryGroup {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xb4dde569_ea96_5883_914c_ebb7d818dd3a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeometryGroup_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub FillRule: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut FillRule,
    ) -> ::windows_core::HRESULT,
    pub SetFillRule: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: FillRule,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub Children: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation_Collections"))]
    Children: usize,
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub SetChildren: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation_Collections"))]
    SetChildren: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IGeometryGroupStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGeometryGroupStatics {
    type Vtable = IGeometryGroupStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IGeometryGroupStatics {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x56a23da5_d015_568a_9f8b_11b125cfd9b4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeometryGroupStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub FillRuleProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub ChildrenProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IGeometryStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGeometryStatics {
    type Vtable = IGeometryStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IGeometryStatics {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x349f78d0_4978_5742_b7d2_b34ea2c95600);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeometryStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Empty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub StandardFlatteningTolerance: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub TransformProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IGradientBrush(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGradientBrush {
    type Vtable = IGradientBrush_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IGradientBrush {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x77c347fa_c4c4_5174_a945_65cab3aa1c75);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGradientBrush_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SpreadMethod: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut GradientSpreadMethod,
    ) -> ::windows_core::HRESULT,
    pub SetSpreadMethod: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: GradientSpreadMethod,
    ) -> ::windows_core::HRESULT,
    pub MappingMode: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut BrushMappingMode,
    ) -> ::windows_core::HRESULT,
    pub SetMappingMode: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: BrushMappingMode,
    ) -> ::windows_core::HRESULT,
    pub ColorInterpolationMode: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ColorInterpolationMode,
    ) -> ::windows_core::HRESULT,
    pub SetColorInterpolationMode: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ColorInterpolationMode,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub GradientStops: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation_Collections"))]
    GradientStops: usize,
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub SetGradientStops: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation_Collections"))]
    SetGradientStops: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IGradientBrushFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGradientBrushFactory {
    type Vtable = IGradientBrushFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IGradientBrushFactory {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x64ff6177_1eda_565b_b7aa_ac50152e3136);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGradientBrushFactory_Vtbl {
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
pub struct IGradientBrushStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGradientBrushStatics {
    type Vtable = IGradientBrushStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IGradientBrushStatics {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x4d3697d7_c6db_501c_8fa2_da30b8c8ca3b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGradientBrushStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SpreadMethodProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub MappingModeProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub ColorInterpolationModeProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub GradientStopsProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IGradientStop(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGradientStop {
    type Vtable = IGradientStop_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IGradientStop {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x48bcb039_e8e1_5743_94c3_f766011d3b5d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGradientStop_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Windows_UI")]
    pub Color: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI"))]
    Color: usize,
    #[cfg(feature = "Windows_UI")]
    pub SetColor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::windows::UI::Color,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI"))]
    SetColor: usize,
    pub Offset: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub SetOffset: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IGradientStopStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGradientStopStatics {
    type Vtable = IGradientStopStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IGradientStopStatics {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x0b566c1b_37de_5bfd_b419_0f7c4c0a0523);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGradientStopStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ColorProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub OffsetProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IImageBrush(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IImageBrush {
    type Vtable = IImageBrush_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IImageBrush {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xedcd91a3_a868_5ba6_9489_5b12b4c29d85);
}
#[repr(C)]
#[doc(hidden)]
pub struct IImageBrush_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ImageSource: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetImageSource: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_Foundation")]
    pub ImageFailed: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    ImageFailed: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub RemoveImageFailed: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RemoveImageFailed: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub ImageOpened: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    ImageOpened: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub RemoveImageOpened: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RemoveImageOpened: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IImageBrushStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IImageBrushStatics {
    type Vtable = IImageBrushStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IImageBrushStatics {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xce8082dc_a505_5b4f_8861_79630f52c189);
}
#[repr(C)]
#[doc(hidden)]
pub struct IImageBrushStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ImageSourceProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IImageSource(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IImageSource {
    type Vtable = IImageSource_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IImageSource {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x6c2038f6_d6d5_55e9_9b9e_082f12dbff60);
}
#[repr(C)]
#[doc(hidden)]
pub struct IImageSource_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IImageSourceFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IImageSourceFactory {
    type Vtable = IImageSourceFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IImageSourceFactory {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x0b1e64a3_e353_5901_b84b_ae9842aea5cd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IImageSourceFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ILineGeometry(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILineGeometry {
    type Vtable = ILineGeometry_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ILineGeometry {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x467ef3c5_bc43_50ed_bb23_16be2c63356e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILineGeometry_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Windows_Foundation")]
    pub StartPoint: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::Point,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    StartPoint: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub SetStartPoint: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::windows::Foundation::Point,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    SetStartPoint: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub EndPoint: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::Point,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    EndPoint: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub SetEndPoint: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::windows::Foundation::Point,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    SetEndPoint: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ILineGeometryStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILineGeometryStatics {
    type Vtable = ILineGeometryStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ILineGeometryStatics {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xce0ecbf3_9389_5304_b7c8_5e610902f258);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILineGeometryStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub StartPointProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub EndPointProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ILineSegment(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILineSegment {
    type Vtable = ILineSegment_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ILineSegment {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x0c618e54_d883_588c_8875_bd8dfd6a6a3e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILineSegment_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Windows_Foundation")]
    pub Point: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::Point,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    Point: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub SetPoint: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::windows::Foundation::Point,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    SetPoint: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ILineSegmentStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILineSegmentStatics {
    type Vtable = ILineSegmentStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ILineSegmentStatics {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xc3ec48a9_b9c0_561f_9925_d1d1b2a6bae6);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILineSegmentStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub PointProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ILinearGradientBrush(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILinearGradientBrush {
    type Vtable = ILinearGradientBrush_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ILinearGradientBrush {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xc0ab9638_1bd9_5fa4_9649_48cfa12f0d1e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILinearGradientBrush_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Windows_Foundation")]
    pub StartPoint: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::Point,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    StartPoint: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub SetStartPoint: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::windows::Foundation::Point,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    SetStartPoint: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub EndPoint: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::Point,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    EndPoint: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub SetEndPoint: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::windows::Foundation::Point,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    SetEndPoint: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ILinearGradientBrushFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILinearGradientBrushFactory {
    type Vtable = ILinearGradientBrushFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ILinearGradientBrushFactory {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xc0ba7de3_ccfd_534c_882f_3ab39ae723f3);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILinearGradientBrushFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub CreateInstanceWithGradientStopCollectionAndAngle:
        unsafe extern "system" fn(
            this: *mut ::core::ffi::c_void,
            gradientstopcollection: *mut ::core::ffi::c_void,
            angle: f64,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation_Collections"))]
    CreateInstanceWithGradientStopCollectionAndAngle: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ILinearGradientBrushStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILinearGradientBrushStatics {
    type Vtable = ILinearGradientBrushStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ILinearGradientBrushStatics {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xdf029e84_f6be_5b7e_ba22_3b4e7a6bceee);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILinearGradientBrushStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub StartPointProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub EndPointProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ILoadedImageSourceLoadCompletedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILoadedImageSourceLoadCompletedEventArgs {
    type Vtable = ILoadedImageSourceLoadCompletedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ILoadedImageSourceLoadCompletedEventArgs {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x4121bb7c_48e8_542d_b950_3ea7e709c0d6);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILoadedImageSourceLoadCompletedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut LoadedImageSourceLoadStatus,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ILoadedImageSurface(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILoadedImageSurface {
    type Vtable = ILoadedImageSurface_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ILoadedImageSurface {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xb5275540_1706_5851_95cc_498ee81fb183);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILoadedImageSurface_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Windows_Foundation")]
    pub DecodedPhysicalSize: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::Size,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    DecodedPhysicalSize: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub DecodedSize: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::Size,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    DecodedSize: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub NaturalSize: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::Size,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    NaturalSize: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub LoadCompleted: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    LoadCompleted: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub RemoveLoadCompleted: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RemoveLoadCompleted: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ILoadedImageSurfaceStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILoadedImageSurfaceStatics {
    type Vtable = ILoadedImageSurfaceStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ILoadedImageSurfaceStatics {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x25d390c4_4e32_52c2_868f_f2ede74ee442);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILoadedImageSurfaceStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Windows_Foundation")]
    pub StartLoadFromUriWithSize: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        uri: *mut ::core::ffi::c_void,
        desiredmaxsize: ::windows::Foundation::Size,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    StartLoadFromUriWithSize: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub StartLoadFromUri: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        uri: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    StartLoadFromUri: usize,
    #[cfg(all(feature = "Windows_Foundation", feature = "Windows_Storage_Streams"))]
    pub StartLoadFromStreamWithSize: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        stream: *mut ::core::ffi::c_void,
        desiredmaxsize: ::windows::Foundation::Size,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Windows_Foundation", feature = "Windows_Storage_Streams")))]
    StartLoadFromStreamWithSize: usize,
    #[cfg(feature = "Windows_Storage_Streams")]
    pub StartLoadFromStream: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        stream: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Storage_Streams"))]
    StartLoadFromStream: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMatrix3DProjection(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMatrix3DProjection {
    type Vtable = IMatrix3DProjection_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMatrix3DProjection {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xfc3338ef_f390_5bb1_932e_3b7c08788187);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMatrix3DProjection_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Microsoft_UI_Xaml_Media_Media3D")]
    pub ProjectionMatrix: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut Media3D::Matrix3D,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Microsoft_UI_Xaml_Media_Media3D"))]
    ProjectionMatrix: usize,
    #[cfg(feature = "Microsoft_UI_Xaml_Media_Media3D")]
    pub SetProjectionMatrix: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: Media3D::Matrix3D,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Microsoft_UI_Xaml_Media_Media3D"))]
    SetProjectionMatrix: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMatrix3DProjectionStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMatrix3DProjectionStatics {
    type Vtable = IMatrix3DProjectionStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMatrix3DProjectionStatics {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xa5a7e267_7a5d_58ef_a8cd_b88ebdf82207);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMatrix3DProjectionStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ProjectionMatrixProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMatrixHelper(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMatrixHelper {
    type Vtable = IMatrixHelper_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMatrixHelper {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x9571fd76_cc5c_534d_ac85_cb4ac870c912);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMatrixHelper_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMatrixHelperStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMatrixHelperStatics {
    type Vtable = IMatrixHelperStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMatrixHelperStatics {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x5762cf6b_4fb0_532f_8368_de960042701f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMatrixHelperStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Identity: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut Matrix,
    ) -> ::windows_core::HRESULT,
    pub FromElements: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        m11: f64,
        m12: f64,
        m21: f64,
        m22: f64,
        offsetx: f64,
        offsety: f64,
        result__: *mut Matrix,
    ) -> ::windows_core::HRESULT,
    pub GetIsIdentity: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        target: Matrix,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_Foundation")]
    pub Transform: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        target: Matrix,
        point: ::windows::Foundation::Point,
        result__: *mut ::windows::Foundation::Point,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    Transform: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMatrixTransform(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMatrixTransform {
    type Vtable = IMatrixTransform_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMatrixTransform {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xf03138e1_addd_59fa_b993_3ea69b888ace);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMatrixTransform_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Matrix: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut Matrix,
    ) -> ::windows_core::HRESULT,
    pub SetMatrix: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: Matrix,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMatrixTransformStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMatrixTransformStatics {
    type Vtable = IMatrixTransformStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMatrixTransformStatics {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xd7db9de3_5071_5115_98fb_ccad2fd46e44);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMatrixTransformStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub MatrixProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMediaTransportControlsThumbnailRequestedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaTransportControlsThumbnailRequestedEventArgs {
    type Vtable = IMediaTransportControlsThumbnailRequestedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMediaTransportControlsThumbnailRequestedEventArgs {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xfe0ffb86_74b0_5031_accc_b34d0382a637);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaTransportControlsThumbnailRequestedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Windows_Storage_Streams")]
    pub SetThumbnailImage: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        source: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Storage_Streams"))]
    SetThumbnailImage: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub GetDeferral: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    GetDeferral: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMicaBackdrop(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMicaBackdrop {
    type Vtable = IMicaBackdrop_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMicaBackdrop {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xc156a404_3dac_593a_b1f3_7a33c289dc83);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMicaBackdrop_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Microsoft_UI_Composition_SystemBackdrops")]
    pub Kind: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::super::Composition::SystemBackdrops::MicaKind,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Microsoft_UI_Composition_SystemBackdrops"))]
    Kind: usize,
    #[cfg(feature = "Microsoft_UI_Composition_SystemBackdrops")]
    pub SetKind: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: super::super::Composition::SystemBackdrops::MicaKind,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Microsoft_UI_Composition_SystemBackdrops"))]
    SetKind: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMicaBackdropFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMicaBackdropFactory {
    type Vtable = IMicaBackdropFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMicaBackdropFactory {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x774379ce_74bd_59d4_849d_d99c4184d838);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMicaBackdropFactory_Vtbl {
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
pub struct IMicaBackdropStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMicaBackdropStatics {
    type Vtable = IMicaBackdropStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMicaBackdropStatics {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xa63abdce_c796_5509_9f4d_072bc1e599f1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMicaBackdropStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub KindProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPathFigure(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPathFigure {
    type Vtable = IPathFigure_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPathFigure {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x0ee00712_bf65_5f27_9c06_14abdf6656d7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPathFigure_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub Segments: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation_Collections"))]
    Segments: usize,
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub SetSegments: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation_Collections"))]
    SetSegments: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub StartPoint: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::Point,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    StartPoint: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub SetStartPoint: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::windows::Foundation::Point,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    SetStartPoint: usize,
    pub IsClosed: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetIsClosed: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
    pub IsFilled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetIsFilled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPathFigureStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPathFigureStatics {
    type Vtable = IPathFigureStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPathFigureStatics {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x93bc33c4_879a_5edb_b8d7_7ecb861a7314);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPathFigureStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SegmentsProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub StartPointProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub IsClosedProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub IsFilledProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPathGeometry(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPathGeometry {
    type Vtable = IPathGeometry_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPathGeometry {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x11b9d95d_d3d9_5337_a05c_73a27a2b5124);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPathGeometry_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub FillRule: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut FillRule,
    ) -> ::windows_core::HRESULT,
    pub SetFillRule: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: FillRule,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub Figures: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation_Collections"))]
    Figures: usize,
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub SetFigures: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation_Collections"))]
    SetFigures: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPathGeometryStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPathGeometryStatics {
    type Vtable = IPathGeometryStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPathGeometryStatics {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xd7f408fe_6c3a_5cce_91cc_c5a95d4b345a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPathGeometryStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub FillRuleProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub FiguresProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPathSegment(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPathSegment {
    type Vtable = IPathSegment_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPathSegment {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xb922ebbe_08f0_57e9_8785_7e57097f3bd4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPathSegment_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPathSegmentFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPathSegmentFactory {
    type Vtable = IPathSegmentFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPathSegmentFactory {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x0559a4ff_ac4b_5bb7_b541_d373960e083b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPathSegmentFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPlaneProjection(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPlaneProjection {
    type Vtable = IPlaneProjection_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPlaneProjection {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xd3e22836_0322_5d75_941c_a5ffb05192b2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlaneProjection_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub LocalOffsetX: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub SetLocalOffsetX: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
    ) -> ::windows_core::HRESULT,
    pub LocalOffsetY: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub SetLocalOffsetY: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
    ) -> ::windows_core::HRESULT,
    pub LocalOffsetZ: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub SetLocalOffsetZ: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
    ) -> ::windows_core::HRESULT,
    pub RotationX: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub SetRotationX: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
    ) -> ::windows_core::HRESULT,
    pub RotationY: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub SetRotationY: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
    ) -> ::windows_core::HRESULT,
    pub RotationZ: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub SetRotationZ: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
    ) -> ::windows_core::HRESULT,
    pub CenterOfRotationX: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub SetCenterOfRotationX: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
    ) -> ::windows_core::HRESULT,
    pub CenterOfRotationY: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub SetCenterOfRotationY: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
    ) -> ::windows_core::HRESULT,
    pub CenterOfRotationZ: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub SetCenterOfRotationZ: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
    ) -> ::windows_core::HRESULT,
    pub GlobalOffsetX: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub SetGlobalOffsetX: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
    ) -> ::windows_core::HRESULT,
    pub GlobalOffsetY: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub SetGlobalOffsetY: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
    ) -> ::windows_core::HRESULT,
    pub GlobalOffsetZ: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub SetGlobalOffsetZ: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Microsoft_UI_Xaml_Media_Media3D")]
    pub ProjectionMatrix: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut Media3D::Matrix3D,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Microsoft_UI_Xaml_Media_Media3D"))]
    ProjectionMatrix: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPlaneProjectionStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPlaneProjectionStatics {
    type Vtable = IPlaneProjectionStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPlaneProjectionStatics {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x96d86c18_90dd_564a_828a_8735e4219b1d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlaneProjectionStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub LocalOffsetXProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub LocalOffsetYProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub LocalOffsetZProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub RotationXProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub RotationYProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub RotationZProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub CenterOfRotationXProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub CenterOfRotationYProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub CenterOfRotationZProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub GlobalOffsetXProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub GlobalOffsetYProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub GlobalOffsetZProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub ProjectionMatrixProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPolyBezierSegment(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPolyBezierSegment {
    type Vtable = IPolyBezierSegment_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPolyBezierSegment {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xd7f760a0_b93a_562a_8118_6330ed22c307);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPolyBezierSegment_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub Points: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation_Collections"))]
    Points: usize,
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub SetPoints: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation_Collections"))]
    SetPoints: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPolyBezierSegmentStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPolyBezierSegmentStatics {
    type Vtable = IPolyBezierSegmentStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPolyBezierSegmentStatics {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x738ef089_a80f_53e0_816f_f787a4461907);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPolyBezierSegmentStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub PointsProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPolyLineSegment(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPolyLineSegment {
    type Vtable = IPolyLineSegment_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPolyLineSegment {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x426ef287_0287_536f_ad9e_6a05ecbb323a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPolyLineSegment_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub Points: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation_Collections"))]
    Points: usize,
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub SetPoints: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation_Collections"))]
    SetPoints: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPolyLineSegmentStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPolyLineSegmentStatics {
    type Vtable = IPolyLineSegmentStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPolyLineSegmentStatics {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xcf54e568_101a_5349_9189_6f9a1e7f5280);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPolyLineSegmentStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub PointsProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPolyQuadraticBezierSegment(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPolyQuadraticBezierSegment {
    type Vtable = IPolyQuadraticBezierSegment_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPolyQuadraticBezierSegment {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x56372f4c_c531_5c3e_b0e0_1645f5a8d872);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPolyQuadraticBezierSegment_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub Points: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation_Collections"))]
    Points: usize,
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub SetPoints: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation_Collections"))]
    SetPoints: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPolyQuadraticBezierSegmentStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPolyQuadraticBezierSegmentStatics {
    type Vtable = IPolyQuadraticBezierSegmentStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPolyQuadraticBezierSegmentStatics {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x7eb6374d_cd30_5507_b2ab_c4e3a7dc60bf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPolyQuadraticBezierSegmentStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub PointsProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IProjection(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IProjection {
    type Vtable = IProjection_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IProjection {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xc95364b3_6058_5ee5_9e28_d38b7679fcd4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProjection_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IProjectionFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IProjectionFactory {
    type Vtable = IProjectionFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IProjectionFactory {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x870ea34f_db61_5b75_89ad_e0480c802937);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProjectionFactory_Vtbl {
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
pub struct IQuadraticBezierSegment(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IQuadraticBezierSegment {
    type Vtable = IQuadraticBezierSegment_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IQuadraticBezierSegment {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x6048abe4_7a12_5195_bd61_71dfd0361c38);
}
#[repr(C)]
#[doc(hidden)]
pub struct IQuadraticBezierSegment_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Windows_Foundation")]
    pub Point1: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::Point,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    Point1: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub SetPoint1: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::windows::Foundation::Point,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    SetPoint1: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub Point2: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::Point,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    Point2: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub SetPoint2: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::windows::Foundation::Point,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    SetPoint2: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IQuadraticBezierSegmentStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IQuadraticBezierSegmentStatics {
    type Vtable = IQuadraticBezierSegmentStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IQuadraticBezierSegmentStatics {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x4d56ea65_0a1a_528a_a5b6_41da03ac71f4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IQuadraticBezierSegmentStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Point1Property: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub Point2Property: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IRadialGradientBrush(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRadialGradientBrush {
    type Vtable = IRadialGradientBrush_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IRadialGradientBrush {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x5d493ce1_b844_546a_b772_b3bcba7e98ee);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRadialGradientBrush_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Windows_Foundation")]
    pub Center: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::Point,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    Center: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub SetCenter: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::windows::Foundation::Point,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    SetCenter: usize,
    pub RadiusX: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub SetRadiusX: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
    ) -> ::windows_core::HRESULT,
    pub RadiusY: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub SetRadiusY: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_Foundation")]
    pub GradientOrigin: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::Point,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    GradientOrigin: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub SetGradientOrigin: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::windows::Foundation::Point,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    SetGradientOrigin: usize,
    pub MappingMode: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut BrushMappingMode,
    ) -> ::windows_core::HRESULT,
    pub SetMappingMode: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: BrushMappingMode,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Microsoft_UI_Composition")]
    pub InterpolationSpace: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::super::Composition::CompositionColorSpace,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Microsoft_UI_Composition"))]
    InterpolationSpace: usize,
    #[cfg(feature = "Microsoft_UI_Composition")]
    pub SetInterpolationSpace: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: super::super::Composition::CompositionColorSpace,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Microsoft_UI_Composition"))]
    SetInterpolationSpace: usize,
    pub SpreadMethod: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut GradientSpreadMethod,
    ) -> ::windows_core::HRESULT,
    pub SetSpreadMethod: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: GradientSpreadMethod,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub GradientStops: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation_Collections"))]
    GradientStops: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IRadialGradientBrushFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRadialGradientBrushFactory {
    type Vtable = IRadialGradientBrushFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IRadialGradientBrushFactory {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xd90ba26e_9e67_54bd_a2d9_61c8f9f1d433);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRadialGradientBrushFactory_Vtbl {
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
pub struct IRadialGradientBrushStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRadialGradientBrushStatics {
    type Vtable = IRadialGradientBrushStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IRadialGradientBrushStatics {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xf275a0b8_66f9_5b7d_a415_7eda65fe6dd3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRadialGradientBrushStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CenterProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub RadiusXProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub RadiusYProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub GradientOriginProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub InterpolationSpaceProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub MappingModeProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SpreadMethodProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IRectangleGeometry(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRectangleGeometry {
    type Vtable = IRectangleGeometry_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IRectangleGeometry {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xb6143890_a5f5_54e0_ab42_d88bab451f04);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRectangleGeometry_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Windows_Foundation")]
    pub Rect: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::Rect,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    Rect: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub SetRect: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::windows::Foundation::Rect,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    SetRect: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IRectangleGeometryStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRectangleGeometryStatics {
    type Vtable = IRectangleGeometryStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IRectangleGeometryStatics {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x1ae7ac26_8a8b_55a5_b035_586e2b642919);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRectangleGeometryStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub RectProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IRenderedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRenderedEventArgs {
    type Vtable = IRenderedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IRenderedEventArgs {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xb268b885_118d_5b66_8099_3b6bb8644726);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRenderedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Windows_Foundation")]
    pub FrameDuration: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::TimeSpan,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    FrameDuration: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IRenderingEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRenderingEventArgs {
    type Vtable = IRenderingEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IRenderingEventArgs {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xa67c8f8d_1885_5fc9_975c_901224f79b1e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRenderingEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Windows_Foundation")]
    pub RenderingTime: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::TimeSpan,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RenderingTime: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IRotateTransform(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRotateTransform {
    type Vtable = IRotateTransform_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IRotateTransform {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xd4686e7c_a374_5cac_8927_0ef07c5b254d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRotateTransform_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CenterX: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub SetCenterX: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
    ) -> ::windows_core::HRESULT,
    pub CenterY: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub SetCenterY: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
    ) -> ::windows_core::HRESULT,
    pub Angle: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub SetAngle: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IRotateTransformStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRotateTransformStatics {
    type Vtable = IRotateTransformStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IRotateTransformStatics {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x8ec4c662_04f8_51d7_bcb2_17f10c2faa38);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRotateTransformStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CenterXProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub CenterYProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub AngleProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IScaleTransform(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IScaleTransform {
    type Vtable = IScaleTransform_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IScaleTransform {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x94b064a4_34f0_5ef9_8b67_444f5699f52a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IScaleTransform_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CenterX: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub SetCenterX: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
    ) -> ::windows_core::HRESULT,
    pub CenterY: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub SetCenterY: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
    ) -> ::windows_core::HRESULT,
    pub ScaleX: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub SetScaleX: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
    ) -> ::windows_core::HRESULT,
    pub ScaleY: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub SetScaleY: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IScaleTransformStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IScaleTransformStatics {
    type Vtable = IScaleTransformStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IScaleTransformStatics {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x76485bd5_a5bf_5790_a837_8193c84df353);
}
#[repr(C)]
#[doc(hidden)]
pub struct IScaleTransformStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CenterXProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub CenterYProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub ScaleXProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub ScaleYProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IShadow(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IShadow {
    type Vtable = IShadow_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IShadow {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xcc12fd6a_50aa_5eb3_9a0e_b938b454c439);
}
#[repr(C)]
#[doc(hidden)]
pub struct IShadow_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IShadowFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IShadowFactory {
    type Vtable = IShadowFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IShadowFactory {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xc9115fbb_fcc3_52bf_b8ee_c348102a46e0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IShadowFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ISkewTransform(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISkewTransform {
    type Vtable = ISkewTransform_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ISkewTransform {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x230abaa6_a9b6_5210_873f_36bea29d7c06);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISkewTransform_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CenterX: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub SetCenterX: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
    ) -> ::windows_core::HRESULT,
    pub CenterY: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub SetCenterY: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
    ) -> ::windows_core::HRESULT,
    pub AngleX: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub SetAngleX: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
    ) -> ::windows_core::HRESULT,
    pub AngleY: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub SetAngleY: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ISkewTransformStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISkewTransformStatics {
    type Vtable = ISkewTransformStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ISkewTransformStatics {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x93265150_53d0_52e3_88a3_3d93e2ed861a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISkewTransformStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CenterXProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub CenterYProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub AngleXProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub AngleYProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ISolidColorBrush(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISolidColorBrush {
    type Vtable = ISolidColorBrush_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ISolidColorBrush {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xb3865c31_37c8_55c1_8a72_d41c67642e2a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISolidColorBrush_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Windows_UI")]
    pub Color: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI"))]
    Color: usize,
    #[cfg(feature = "Windows_UI")]
    pub SetColor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::windows::UI::Color,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI"))]
    SetColor: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ISolidColorBrushFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISolidColorBrushFactory {
    type Vtable = ISolidColorBrushFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ISolidColorBrushFactory {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x7b559384_4daa_54f4_91ef_33a23fd816ca);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISolidColorBrushFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Windows_UI")]
    pub CreateInstanceWithColor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        color: ::windows::UI::Color,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI"))]
    CreateInstanceWithColor: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ISolidColorBrushStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISolidColorBrushStatics {
    type Vtable = ISolidColorBrushStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ISolidColorBrushStatics {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x6bc16da0_c4e6_59b8_995b_b31e48424c07);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISolidColorBrushStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ColorProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ISurfaceImageSourceManagerNative(::windows_core::IUnknown);
impl ISurfaceImageSourceManagerNative {
    pub unsafe fn FlushAllSurfacesWithDevice<P0>(&self, device: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        (::windows_core::Interface::vtable(self).FlushAllSurfacesWithDevice)(
            ::windows_core::Interface::as_raw(self),
            device.into_param().abi(),
        )
        .ok()
    }
}
::windows_core::imp::interface_hierarchy!(
    ISurfaceImageSourceManagerNative,
    ::windows_core::IUnknown
);
unsafe impl ::windows_core::Interface for ISurfaceImageSourceManagerNative {
    type Vtable = ISurfaceImageSourceManagerNative_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ISurfaceImageSourceManagerNative {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x81521d7e_ff74_4a6b_8289_44bfd11cf0cc);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISurfaceImageSourceManagerNative_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub FlushAllSurfacesWithDevice: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        device: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ISurfaceImageSourceNative(::windows_core::IUnknown);
impl ISurfaceImageSourceNative {
    #[doc = "Required features: `\"Windows_Win32_Graphics_Dxgi\"`"]
    #[cfg(feature = "Windows_Win32_Graphics_Dxgi")]
    pub unsafe fn SetDevice<P0>(&self, device: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows::Win32::Graphics::Dxgi::IDXGIDevice>,
    {
        (::windows_core::Interface::vtable(self).SetDevice)(
            ::windows_core::Interface::as_raw(self),
            device.into_param().abi(),
        )
        .ok()
    }
    #[doc = "Required features: `\"Windows_Win32_Foundation\"`, `\"Windows_Win32_Graphics_Dxgi\"`"]
    #[cfg(all(
        feature = "Windows_Win32_Foundation",
        feature = "Windows_Win32_Graphics_Dxgi"
    ))]
    pub unsafe fn BeginDraw(
        &self,
        updaterect: ::windows::Win32::Foundation::RECT,
        surface: *mut ::core::option::Option<::windows::Win32::Graphics::Dxgi::IDXGISurface>,
        offset: *mut ::windows::Win32::Foundation::POINT,
    ) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).BeginDraw)(
            ::windows_core::Interface::as_raw(self),
            ::core::mem::transmute(updaterect),
            ::core::mem::transmute(surface),
            offset,
        )
        .ok()
    }
    pub unsafe fn EndDraw(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).EndDraw)(::windows_core::Interface::as_raw(self))
            .ok()
    }
}
::windows_core::imp::interface_hierarchy!(ISurfaceImageSourceNative, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISurfaceImageSourceNative {
    type Vtable = ISurfaceImageSourceNative_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ISurfaceImageSourceNative {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xe4cecd6c_f14b_4f46_83c3_8bbda27c6504);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISurfaceImageSourceNative_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Windows_Win32_Graphics_Dxgi")]
    pub SetDevice: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        device: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Win32_Graphics_Dxgi"))]
    SetDevice: usize,
    #[cfg(all(
        feature = "Windows_Win32_Foundation",
        feature = "Windows_Win32_Graphics_Dxgi"
    ))]
    pub BeginDraw: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        updaterect: ::windows::Win32::Foundation::RECT,
        surface: *mut *mut ::core::ffi::c_void,
        offset: *mut ::windows::Win32::Foundation::POINT,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(all(
        feature = "Windows_Win32_Foundation",
        feature = "Windows_Win32_Graphics_Dxgi"
    )))]
    BeginDraw: usize,
    pub EndDraw:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ISurfaceImageSourceNativeWithD2D(::windows_core::IUnknown);
impl ISurfaceImageSourceNativeWithD2D {
    pub unsafe fn SetDevice<P0>(&self, device: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        (::windows_core::Interface::vtable(self).SetDevice)(
            ::windows_core::Interface::as_raw(self),
            device.into_param().abi(),
        )
        .ok()
    }
    #[doc = "Required features: `\"Windows_Win32_Foundation\"`"]
    #[cfg(feature = "Windows_Win32_Foundation")]
    pub unsafe fn BeginDraw<T>(
        &self,
        updaterect: *const ::windows::Win32::Foundation::RECT,
        offset: *mut ::windows::Win32::Foundation::POINT,
    ) -> ::windows_core::Result<T>
    where
        T: ::windows_core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows_core::Interface::vtable(self).BeginDraw)(
            ::windows_core::Interface::as_raw(self),
            updaterect,
            &T::IID,
            &mut result__,
            offset,
        )
        .from_abi(result__)
    }
    pub unsafe fn EndDraw(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).EndDraw)(::windows_core::Interface::as_raw(self))
            .ok()
    }
    pub unsafe fn SuspendDraw(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SuspendDraw)(::windows_core::Interface::as_raw(
            self,
        ))
        .ok()
    }
    pub unsafe fn ResumeDraw(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ResumeDraw)(::windows_core::Interface::as_raw(
            self,
        ))
        .ok()
    }
}
::windows_core::imp::interface_hierarchy!(
    ISurfaceImageSourceNativeWithD2D,
    ::windows_core::IUnknown
);
unsafe impl ::windows_core::Interface for ISurfaceImageSourceNativeWithD2D {
    type Vtable = ISurfaceImageSourceNativeWithD2D_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ISurfaceImageSourceNativeWithD2D {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xcb833102_d5d1_448b_a31a_52a9509f24e6);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISurfaceImageSourceNativeWithD2D_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub SetDevice: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        device: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_Win32_Foundation")]
    pub BeginDraw: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        updaterect: *const ::windows::Win32::Foundation::RECT,
        iid: *const ::windows_core::GUID,
        updateobject: *mut *mut ::core::ffi::c_void,
        offset: *mut ::windows::Win32::Foundation::POINT,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Win32_Foundation"))]
    BeginDraw: usize,
    pub EndDraw:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SuspendDraw:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub ResumeDraw:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ISwapChainBackgroundPanelNative(::windows_core::IUnknown);
impl ISwapChainBackgroundPanelNative {
    #[doc = "Required features: `\"Windows_Win32_Graphics_Dxgi\"`"]
    #[cfg(feature = "Windows_Win32_Graphics_Dxgi")]
    pub unsafe fn SetSwapChain<P0>(&self, swapchain: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows::Win32::Graphics::Dxgi::IDXGISwapChain>,
    {
        (::windows_core::Interface::vtable(self).SetSwapChain)(
            ::windows_core::Interface::as_raw(self),
            swapchain.into_param().abi(),
        )
        .ok()
    }
}
::windows_core::imp::interface_hierarchy!(
    ISwapChainBackgroundPanelNative,
    ::windows_core::IUnknown
);
unsafe impl ::windows_core::Interface for ISwapChainBackgroundPanelNative {
    type Vtable = ISwapChainBackgroundPanelNative_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ISwapChainBackgroundPanelNative {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x24d43d84_4246_4aa7_9774_8604cb73d90d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISwapChainBackgroundPanelNative_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Windows_Win32_Graphics_Dxgi")]
    pub SetSwapChain: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        swapchain: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Win32_Graphics_Dxgi"))]
    SetSwapChain: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ISwapChainPanelNative(::windows_core::IUnknown);
impl ISwapChainPanelNative {
    #[doc = "Required features: `\"Windows_Win32_Graphics_Dxgi\"`"]
    #[cfg(feature = "Windows_Win32_Graphics_Dxgi")]
    pub unsafe fn SetSwapChain<P0>(&self, swapchain: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows::Win32::Graphics::Dxgi::IDXGISwapChain>,
    {
        (::windows_core::Interface::vtable(self).SetSwapChain)(
            ::windows_core::Interface::as_raw(self),
            swapchain.into_param().abi(),
        )
        .ok()
    }
}
::windows_core::imp::interface_hierarchy!(ISwapChainPanelNative, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISwapChainPanelNative {
    type Vtable = ISwapChainPanelNative_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ISwapChainPanelNative {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x63aad0b8_7c24_40ff_85a8_640d944cc325);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISwapChainPanelNative_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Windows_Win32_Graphics_Dxgi")]
    pub SetSwapChain: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        swapchain: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Win32_Graphics_Dxgi"))]
    SetSwapChain: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ISwapChainPanelNative2(::windows_core::IUnknown);
impl ISwapChainPanelNative2 {
    #[doc = "Required features: `\"Windows_Win32_Graphics_Dxgi\"`"]
    #[cfg(feature = "Windows_Win32_Graphics_Dxgi")]
    pub unsafe fn SetSwapChain<P0>(&self, swapchain: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows::Win32::Graphics::Dxgi::IDXGISwapChain>,
    {
        (::windows_core::Interface::vtable(self).base__.SetSwapChain)(
            ::windows_core::Interface::as_raw(self),
            swapchain.into_param().abi(),
        )
        .ok()
    }
    #[doc = "Required features: `\"Windows_Win32_Foundation\"`"]
    #[cfg(feature = "Windows_Win32_Foundation")]
    pub unsafe fn SetSwapChainHandle<P0>(&self, swapchainhandle: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows::Win32::Foundation::HANDLE>,
    {
        (::windows_core::Interface::vtable(self).SetSwapChainHandle)(
            ::windows_core::Interface::as_raw(self),
            swapchainhandle.into_param().abi(),
        )
        .ok()
    }
}
::windows_core::imp::interface_hierarchy!(
    ISwapChainPanelNative2,
    ::windows_core::IUnknown,
    ISwapChainPanelNative
);
unsafe impl ::windows_core::Interface for ISwapChainPanelNative2 {
    type Vtable = ISwapChainPanelNative2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ISwapChainPanelNative2 {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x88fd8248_10da_4810_bb4c_010dd27faea9);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISwapChainPanelNative2_Vtbl {
    pub base__: ISwapChainPanelNative_Vtbl,
    #[cfg(feature = "Windows_Win32_Foundation")]
    pub SetSwapChainHandle: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        swapchainhandle: ::windows::Win32::Foundation::HANDLE,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Win32_Foundation"))]
    SetSwapChainHandle: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ISystemBackdrop(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISystemBackdrop {
    type Vtable = ISystemBackdrop_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ISystemBackdrop {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x5aeed5c4_37ac_5852_b73f_1b76ebc3205f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemBackdrop_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Microsoft_UI_Composition_SystemBackdrops")]
    pub GetDefaultSystemBackdropConfiguration: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        target: *mut ::core::ffi::c_void,
        xamlroot: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    )
        -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Microsoft_UI_Composition_SystemBackdrops"))]
    GetDefaultSystemBackdropConfiguration: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ISystemBackdropFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISystemBackdropFactory {
    type Vtable = ISystemBackdropFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ISystemBackdropFactory {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x1e07656b_fad2_5b29_913f_b6748bc45942);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemBackdropFactory_Vtbl {
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
pub struct ISystemBackdropOverrides(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISystemBackdropOverrides {
    type Vtable = ISystemBackdropOverrides_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ISystemBackdropOverrides {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xeb1f5399_cad7_5611_b637_09d76a07e708);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemBackdropOverrides_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Microsoft_UI_Composition")]
    pub OnTargetConnected: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        connectedtarget: *mut ::core::ffi::c_void,
        xamlroot: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Microsoft_UI_Composition"))]
    OnTargetConnected: usize,
    #[cfg(feature = "Microsoft_UI_Composition")]
    pub OnTargetDisconnected: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        disconnectedtarget: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Microsoft_UI_Composition"))]
    OnTargetDisconnected: usize,
    #[cfg(feature = "Microsoft_UI_Composition")]
    pub OnDefaultSystemBackdropConfigurationChanged:
        unsafe extern "system" fn(
            this: *mut ::core::ffi::c_void,
            target: *mut ::core::ffi::c_void,
            xamlroot: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Microsoft_UI_Composition"))]
    OnDefaultSystemBackdropConfigurationChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IThemeShadow(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IThemeShadow {
    type Vtable = IThemeShadow_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IThemeShadow {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xc264208a_d1f4_58ae_8a88_fc59148bee69);
}
#[repr(C)]
#[doc(hidden)]
pub struct IThemeShadow_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub Receivers: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation_Collections"))]
    Receivers: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IThemeShadowFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IThemeShadowFactory {
    type Vtable = IThemeShadowFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IThemeShadowFactory {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x704a9c96_76a0_569e_8ceb_34e92a23fe11);
}
#[repr(C)]
#[doc(hidden)]
pub struct IThemeShadowFactory_Vtbl {
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
pub struct ITileBrush(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITileBrush {
    type Vtable = ITileBrush_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ITileBrush {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xee46060d_cabc_505d_883c_75d2e0e45875);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITileBrush_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub AlignmentX: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut AlignmentX,
    ) -> ::windows_core::HRESULT,
    pub SetAlignmentX: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: AlignmentX,
    ) -> ::windows_core::HRESULT,
    pub AlignmentY: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut AlignmentY,
    ) -> ::windows_core::HRESULT,
    pub SetAlignmentY: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: AlignmentY,
    ) -> ::windows_core::HRESULT,
    pub Stretch: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut Stretch,
    ) -> ::windows_core::HRESULT,
    pub SetStretch: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: Stretch,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ITileBrushFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITileBrushFactory {
    type Vtable = ITileBrushFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ITileBrushFactory {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x8542e5e6_5177_506f_8a3b_aa7da651f099);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITileBrushFactory_Vtbl {
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
pub struct ITileBrushStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITileBrushStatics {
    type Vtable = ITileBrushStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ITileBrushStatics {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xf402197b_9047_5f8a_90bc_6f5d8c748a5f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITileBrushStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub AlignmentXProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub AlignmentYProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub StretchProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ITransform(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITransform {
    type Vtable = ITransform_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ITransform {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x92a8dee5_1413_56b9_8cca_3c46918fde1b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransform_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ITransformFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITransformFactory {
    type Vtable = ITransformFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ITransformFactory {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x7da293f9_b82e_5d15_b623_c08210cbb640);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransformFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ITransformGroup(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITransformGroup {
    type Vtable = ITransformGroup_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ITransformGroup {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x17c55f3b_899c_588f_8bd4_40fa3a5fcb04);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransformGroup_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub Children: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation_Collections"))]
    Children: usize,
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub SetChildren: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation_Collections"))]
    SetChildren: usize,
    pub Value: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut Matrix,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ITransformGroupStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITransformGroupStatics {
    type Vtable = ITransformGroupStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ITransformGroupStatics {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x8f1508f6_7dcf_53d5_bbc0_d8fcd96d7399);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransformGroupStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ChildrenProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ITranslateTransform(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITranslateTransform {
    type Vtable = ITranslateTransform_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ITranslateTransform {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xcfa21ca9_b79f_5450_b459_a96c48cb2c8f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITranslateTransform_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub X: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub SetX: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
    ) -> ::windows_core::HRESULT,
    pub Y: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub SetY: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ITranslateTransformStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITranslateTransformStatics {
    type Vtable = ITranslateTransformStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ITranslateTransformStatics {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x1342eb11_5a6e_5263_ab3e_9b672a86fc0c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITranslateTransformStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub XProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub YProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IVirtualSurfaceImageSourceNative(::windows_core::IUnknown);
impl IVirtualSurfaceImageSourceNative {
    #[doc = "Required features: `\"Windows_Win32_Graphics_Dxgi\"`"]
    #[cfg(feature = "Windows_Win32_Graphics_Dxgi")]
    pub unsafe fn SetDevice<P0>(&self, device: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows::Win32::Graphics::Dxgi::IDXGIDevice>,
    {
        (::windows_core::Interface::vtable(self).base__.SetDevice)(
            ::windows_core::Interface::as_raw(self),
            device.into_param().abi(),
        )
        .ok()
    }
    #[doc = "Required features: `\"Windows_Win32_Foundation\"`, `\"Windows_Win32_Graphics_Dxgi\"`"]
    #[cfg(all(
        feature = "Windows_Win32_Foundation",
        feature = "Windows_Win32_Graphics_Dxgi"
    ))]
    pub unsafe fn BeginDraw(
        &self,
        updaterect: ::windows::Win32::Foundation::RECT,
        surface: *mut ::core::option::Option<::windows::Win32::Graphics::Dxgi::IDXGISurface>,
        offset: *mut ::windows::Win32::Foundation::POINT,
    ) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.BeginDraw)(
            ::windows_core::Interface::as_raw(self),
            ::core::mem::transmute(updaterect),
            ::core::mem::transmute(surface),
            offset,
        )
        .ok()
    }
    pub unsafe fn EndDraw(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.EndDraw)(::windows_core::Interface::as_raw(
            self,
        ))
        .ok()
    }
    #[doc = "Required features: `\"Windows_Win32_Foundation\"`"]
    #[cfg(feature = "Windows_Win32_Foundation")]
    pub unsafe fn Invalidate(
        &self,
        updaterect: ::windows::Win32::Foundation::RECT,
    ) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Invalidate)(
            ::windows_core::Interface::as_raw(self),
            ::core::mem::transmute(updaterect),
        )
        .ok()
    }
    pub unsafe fn GetUpdateRectCount(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetUpdateRectCount)(
            ::windows_core::Interface::as_raw(self),
            &mut result__,
        )
        .from_abi(result__)
    }
    #[doc = "Required features: `\"Windows_Win32_Foundation\"`"]
    #[cfg(feature = "Windows_Win32_Foundation")]
    pub unsafe fn GetUpdateRects(
        &self,
        updates: &mut [::windows::Win32::Foundation::RECT],
    ) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetUpdateRects)(
            ::windows_core::Interface::as_raw(self),
            ::core::mem::transmute(updates.as_ptr()),
            updates.len().try_into().unwrap(),
        )
        .ok()
    }
    #[doc = "Required features: `\"Windows_Win32_Foundation\"`"]
    #[cfg(feature = "Windows_Win32_Foundation")]
    pub unsafe fn GetVisibleBounds(
        &self,
    ) -> ::windows_core::Result<::windows::Win32::Foundation::RECT> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetVisibleBounds)(
            ::windows_core::Interface::as_raw(self),
            &mut result__,
        )
        .from_abi(result__)
    }
    pub unsafe fn RegisterForUpdatesNeeded<P0>(&self, callback: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IVirtualSurfaceUpdatesCallbackNative>,
    {
        (::windows_core::Interface::vtable(self).RegisterForUpdatesNeeded)(
            ::windows_core::Interface::as_raw(self),
            callback.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn Resize(&self, newwidth: i32, newheight: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Resize)(
            ::windows_core::Interface::as_raw(self),
            newwidth,
            newheight,
        )
        .ok()
    }
}
::windows_core::imp::interface_hierarchy!(
    IVirtualSurfaceImageSourceNative,
    ::windows_core::IUnknown,
    ISurfaceImageSourceNative
);
unsafe impl ::windows_core::Interface for IVirtualSurfaceImageSourceNative {
    type Vtable = IVirtualSurfaceImageSourceNative_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IVirtualSurfaceImageSourceNative {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x9e43c18e_7816_474c_840f_5c9c8b0e2207);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVirtualSurfaceImageSourceNative_Vtbl {
    pub base__: ISurfaceImageSourceNative_Vtbl,
    #[cfg(feature = "Windows_Win32_Foundation")]
    pub Invalidate: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        updaterect: ::windows::Win32::Foundation::RECT,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Win32_Foundation"))]
    Invalidate: usize,
    pub GetUpdateRectCount: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        count: *mut u32,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_Win32_Foundation")]
    pub GetUpdateRects: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        updates: *mut ::windows::Win32::Foundation::RECT,
        count: u32,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Win32_Foundation"))]
    GetUpdateRects: usize,
    #[cfg(feature = "Windows_Win32_Foundation")]
    pub GetVisibleBounds: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        bounds: *mut ::windows::Win32::Foundation::RECT,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Win32_Foundation"))]
    GetVisibleBounds: usize,
    pub RegisterForUpdatesNeeded: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        callback: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub Resize: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        newwidth: i32,
        newheight: i32,
    ) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IVirtualSurfaceUpdatesCallbackNative(::windows_core::IUnknown);
impl IVirtualSurfaceUpdatesCallbackNative {
    pub unsafe fn UpdatesNeeded(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).UpdatesNeeded)(::windows_core::Interface::as_raw(
            self,
        ))
        .ok()
    }
}
::windows_core::imp::interface_hierarchy!(
    IVirtualSurfaceUpdatesCallbackNative,
    ::windows_core::IUnknown
);
unsafe impl ::windows_core::Interface for IVirtualSurfaceUpdatesCallbackNative {
    type Vtable = IVirtualSurfaceUpdatesCallbackNative_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IVirtualSurfaceUpdatesCallbackNative {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xe8e84ac7_b7b8_40f4_b033_f877a756c52b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVirtualSurfaceUpdatesCallbackNative_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub UpdatesNeeded:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IVisualTreeHelper(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IVisualTreeHelper {
    type Vtable = IVisualTreeHelper_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IVisualTreeHelper {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x5f69ac1e_6504_5e3f_a11c_87684c1db814);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVisualTreeHelper_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IVisualTreeHelperStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IVisualTreeHelperStatics {
    type Vtable = IVisualTreeHelperStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IVisualTreeHelperStatics {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x5aece43c_7651_5bb5_855c_2198496e455e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVisualTreeHelperStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub FindElementsInHostCoordinatesPoint: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        intersectingpoint: ::windows::Foundation::Point,
        subtree: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    )
        -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation_Collections"))]
    FindElementsInHostCoordinatesPoint: usize,
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub FindElementsInHostCoordinatesRect: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        intersectingrect: ::windows::Foundation::Rect,
        subtree: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    )
        -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation_Collections"))]
    FindElementsInHostCoordinatesRect: usize,
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub FindAllElementsInHostCoordinatesPoint: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        intersectingpoint: ::windows::Foundation::Point,
        subtree: *mut ::core::ffi::c_void,
        includeallelements: bool,
        result__: *mut *mut ::core::ffi::c_void,
    )
        -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation_Collections"))]
    FindAllElementsInHostCoordinatesPoint: usize,
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub FindAllElementsInHostCoordinatesRect: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        intersectingrect: ::windows::Foundation::Rect,
        subtree: *mut ::core::ffi::c_void,
        includeallelements: bool,
        result__: *mut *mut ::core::ffi::c_void,
    )
        -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation_Collections"))]
    FindAllElementsInHostCoordinatesRect: usize,
    pub GetChild: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        reference: *mut ::core::ffi::c_void,
        childindex: i32,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub GetChildrenCount: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        reference: *mut ::core::ffi::c_void,
        result__: *mut i32,
    ) -> ::windows_core::HRESULT,
    pub GetParent: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        reference: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub DisconnectChildrenRecursive: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(all(
        feature = "Microsoft_UI_Xaml_Controls_Primitives",
        feature = "Windows_Foundation_Collections"
    ))]
    pub GetOpenPopups: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        window: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(all(
        feature = "Microsoft_UI_Xaml_Controls_Primitives",
        feature = "Windows_Foundation_Collections"
    )))]
    GetOpenPopups: usize,
    #[cfg(all(
        feature = "Microsoft_UI_Xaml_Controls_Primitives",
        feature = "Windows_Foundation_Collections"
    ))]
    pub GetOpenPopupsForXamlRoot: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        xamlroot: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(all(
        feature = "Microsoft_UI_Xaml_Controls_Primitives",
        feature = "Windows_Foundation_Collections"
    )))]
    GetOpenPopupsForXamlRoot: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IXamlCompositionBrushBase(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXamlCompositionBrushBase {
    type Vtable = IXamlCompositionBrushBase_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IXamlCompositionBrushBase {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xfeaead28_5cd0_5e58_bcea_8670f832aca9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlCompositionBrushBase_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Windows_UI")]
    pub FallbackColor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI"))]
    FallbackColor: usize,
    #[cfg(feature = "Windows_UI")]
    pub SetFallbackColor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::windows::UI::Color,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI"))]
    SetFallbackColor: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IXamlCompositionBrushBaseFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXamlCompositionBrushBaseFactory {
    type Vtable = IXamlCompositionBrushBaseFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IXamlCompositionBrushBaseFactory {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xb1626d56_0f6f_5416_ada4_5c8105d3f082);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlCompositionBrushBaseFactory_Vtbl {
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
pub struct IXamlCompositionBrushBaseOverrides(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXamlCompositionBrushBaseOverrides {
    type Vtable = IXamlCompositionBrushBaseOverrides_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IXamlCompositionBrushBaseOverrides {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x8881b559_54a0_56c4_a79a_135152fb1dfa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlCompositionBrushBaseOverrides_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub OnConnected:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub OnDisconnected:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IXamlCompositionBrushBaseProtected(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXamlCompositionBrushBaseProtected {
    type Vtable = IXamlCompositionBrushBaseProtected_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IXamlCompositionBrushBaseProtected {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x6617e1a5_e27a_5b95_b03e_6758b58f92a0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlCompositionBrushBaseProtected_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Microsoft_UI_Composition")]
    pub CompositionBrush: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Microsoft_UI_Composition"))]
    CompositionBrush: usize,
    #[cfg(feature = "Microsoft_UI_Composition")]
    pub SetCompositionBrush: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Microsoft_UI_Composition"))]
    SetCompositionBrush: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IXamlCompositionBrushBaseStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXamlCompositionBrushBaseStatics {
    type Vtable = IXamlCompositionBrushBaseStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IXamlCompositionBrushBaseStatics {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x3eed6e16_c386_5a1c_b70d_ef1c0015e691);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlCompositionBrushBaseStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub FallbackColorProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IXamlLight(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXamlLight {
    type Vtable = IXamlLight_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IXamlLight {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xdcd20139_8cd5_5da5_a25c_2b7b813d8d58);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlLight_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IXamlLightFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXamlLightFactory {
    type Vtable = IXamlLightFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IXamlLightFactory {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x76da6306_96fc_553e_bb39_9a4801d06f48);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlLightFactory_Vtbl {
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
pub struct IXamlLightOverrides(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXamlLightOverrides {
    type Vtable = IXamlLightOverrides_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IXamlLightOverrides {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x696d4f30_92ee_540d_ad70_33d4489514d0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlLightOverrides_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub GetId: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub OnConnected: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        newelement: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub OnDisconnected: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        oldelement: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IXamlLightProtected(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXamlLightProtected {
    type Vtable = IXamlLightProtected_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IXamlLightProtected {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xc307bf12_fdaf_54ca_a631_ad0e86263c6e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlLightProtected_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Microsoft_UI_Composition")]
    pub CompositionLight: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Microsoft_UI_Composition"))]
    CompositionLight: usize,
    #[cfg(feature = "Microsoft_UI_Composition")]
    pub SetCompositionLight: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Microsoft_UI_Composition"))]
    SetCompositionLight: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IXamlLightStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXamlLightStatics {
    type Vtable = IXamlLightStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IXamlLightStatics {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xa2d8ea26_26ff_5374_b1dd_f232d5604f6a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlLightStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub AddTargetElement: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        lightid: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        element: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub RemoveTargetElement: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        lightid: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        element: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub AddTargetBrush: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        lightid: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        brush: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub RemoveTargetBrush: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        lightid: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        brush: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct AcrylicBrush(::windows_core::IUnknown);
impl AcrylicBrush {
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn TintColor(&self) -> ::windows_core::Result<::windows::UI::Color> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TintColor)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn SetTintColor(&self, value: ::windows::UI::Color) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetTintColor)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn TintOpacity(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TintOpacity)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetTintOpacity(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetTintOpacity)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn TintTransitionDuration(
        &self,
    ) -> ::windows_core::Result<::windows::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TintTransitionDuration)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn SetTintTransitionDuration(
        &self,
        value: ::windows::Foundation::TimeSpan,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetTintTransitionDuration)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn AlwaysUseFallback(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AlwaysUseFallback)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetAlwaysUseFallback(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetAlwaysUseFallback)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn TintLuminosityOpacity(
        &self,
    ) -> ::windows_core::Result<::windows::Foundation::IReference<f64>> {
        let this = &::windows_core::ComInterface::cast::<IAcrylicBrush2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TintLuminosityOpacity)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn SetTintLuminosityOpacity<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<::windows::Foundation::IReference<f64>>,
    {
        let this = &::windows_core::ComInterface::cast::<IAcrylicBrush2>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetTintLuminosityOpacity)(
                ::windows_core::Interface::as_raw(this),
                value.try_into_param()?.abi(),
            )
            .ok()
        }
    }
    pub unsafe fn CreateInstance<P0>(
        baseinterface: P0,
        innerinterface: &mut ::core::option::Option<::windows_core::IInspectable>,
    ) -> ::windows_core::Result<AcrylicBrush>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        Self::IAcrylicBrushFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstance)(
                ::windows_core::Interface::as_raw(this),
                baseinterface.into_param().abi(),
                innerinterface as *mut _ as _,
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn TintColorProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IAcrylicBrushStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TintColorProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn TintOpacityProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IAcrylicBrushStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TintOpacityProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn TintTransitionDurationProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IAcrylicBrushStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TintTransitionDurationProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn AlwaysUseFallbackProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IAcrylicBrushStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AlwaysUseFallbackProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn TintLuminosityOpacityProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IAcrylicBrushStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TintLuminosityOpacityProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
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
    pub fn Opacity(&self) -> ::windows_core::Result<f64> {
        let this = &::windows_core::ComInterface::cast::<IBrush>(self)?;
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
        let this = &::windows_core::ComInterface::cast::<IBrush>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetOpacity)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn Transform(&self) -> ::windows_core::Result<Transform> {
        let this = &::windows_core::ComInterface::cast::<IBrush>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Transform)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetTransform<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<Transform>,
    {
        let this = &::windows_core::ComInterface::cast::<IBrush>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetTransform)(
                ::windows_core::Interface::as_raw(this),
                value.try_into_param()?.abi(),
            )
            .ok()
        }
    }
    pub fn RelativeTransform(&self) -> ::windows_core::Result<Transform> {
        let this = &::windows_core::ComInterface::cast::<IBrush>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RelativeTransform)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetRelativeTransform<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<Transform>,
    {
        let this = &::windows_core::ComInterface::cast::<IBrush>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetRelativeTransform)(
                ::windows_core::Interface::as_raw(this),
                value.try_into_param()?.abi(),
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
        let this = &::windows_core::ComInterface::cast::<IBrushOverrides>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).PopulatePropertyInfoOverride)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(propertyname),
                animationpropertyinfo.into_param().abi(),
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
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn FallbackColor(&self) -> ::windows_core::Result<::windows::UI::Color> {
        let this = &::windows_core::ComInterface::cast::<IXamlCompositionBrushBase>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FallbackColor)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn SetFallbackColor(&self, value: ::windows::UI::Color) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IXamlCompositionBrushBase>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetFallbackColor)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn OnConnected(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IXamlCompositionBrushBaseOverrides>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).OnConnected)(
                ::windows_core::Interface::as_raw(this),
            )
            .ok()
        }
    }
    pub fn OnDisconnected(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IXamlCompositionBrushBaseOverrides>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).OnDisconnected)(
                ::windows_core::Interface::as_raw(this),
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Composition\"`"]
    #[cfg(feature = "Microsoft_UI_Composition")]
    pub fn CompositionBrush(
        &self,
    ) -> ::windows_core::Result<super::super::Composition::CompositionBrush> {
        let this = &::windows_core::ComInterface::cast::<IXamlCompositionBrushBaseProtected>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CompositionBrush)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Composition\"`"]
    #[cfg(feature = "Microsoft_UI_Composition")]
    pub fn SetCompositionBrush<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::Composition::CompositionBrush>,
    {
        let this = &::windows_core::ComInterface::cast::<IXamlCompositionBrushBaseProtected>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetCompositionBrush)(
                ::windows_core::Interface::as_raw(this),
                value.try_into_param()?.abi(),
            )
            .ok()
        }
    }
    #[doc(hidden)]
    pub fn IAcrylicBrushFactory<
        R,
        F: FnOnce(&IAcrylicBrushFactory) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<AcrylicBrush, IAcrylicBrushFactory> =
            ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IAcrylicBrushStatics<
        R,
        F: FnOnce(&IAcrylicBrushStatics) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<AcrylicBrush, IAcrylicBrushStatics> =
            ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IAcrylicBrushStatics2<
        R,
        F: FnOnce(&IAcrylicBrushStatics2) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<AcrylicBrush, IAcrylicBrushStatics2> =
            ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for AcrylicBrush {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for AcrylicBrush {
    type Vtable = IAcrylicBrush_Vtbl;
}
unsafe impl ::windows_core::ComInterface for AcrylicBrush {
    const IID: ::windows_core::GUID = <IAcrylicBrush as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for AcrylicBrush {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.AcrylicBrush";
}
::windows_core::imp::interface_hierarchy!(
    AcrylicBrush,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
#[cfg(feature = "Microsoft_UI_Composition")]
impl ::windows_core::CanTryInto<super::super::Composition::IAnimationObject> for AcrylicBrush {}
impl ::windows_core::CanTryInto<XamlCompositionBrushBase> for AcrylicBrush {}
impl ::windows_core::CanTryInto<Brush> for AcrylicBrush {}
impl ::windows_core::CanTryInto<super::DependencyObject> for AcrylicBrush {}
unsafe impl ::core::marker::Send for AcrylicBrush {}
unsafe impl ::core::marker::Sync for AcrylicBrush {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ArcSegment(::windows_core::IUnknown);
impl ArcSegment {
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
            ArcSegment,
            ::windows_core::imp::IGenericFactory,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn Point(&self) -> ::windows_core::Result<::windows::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Point)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn SetPoint(&self, value: ::windows::Foundation::Point) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetPoint)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn Size(&self) -> ::windows_core::Result<::windows::Foundation::Size> {
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
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn SetSize(&self, value: ::windows::Foundation::Size) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetSize)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn RotationAngle(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RotationAngle)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetRotationAngle(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetRotationAngle)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsLargeArc(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsLargeArc)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetIsLargeArc(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetIsLargeArc)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn SweepDirection(&self) -> ::windows_core::Result<SweepDirection> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SweepDirection)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetSweepDirection(&self, value: SweepDirection) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetSweepDirection)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn PointProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IArcSegmentStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PointProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn SizeProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IArcSegmentStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SizeProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn RotationAngleProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IArcSegmentStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RotationAngleProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn IsLargeArcProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IArcSegmentStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsLargeArcProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn SweepDirectionProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IArcSegmentStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SweepDirectionProperty)(
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
    pub fn IArcSegmentStatics<R, F: FnOnce(&IArcSegmentStatics) -> ::windows_core::Result<R>>(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<ArcSegment, IArcSegmentStatics> =
            ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for ArcSegment {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for ArcSegment {
    type Vtable = IArcSegment_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ArcSegment {
    const IID: ::windows_core::GUID = <IArcSegment as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ArcSegment {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.ArcSegment";
}
::windows_core::imp::interface_hierarchy!(
    ArcSegment,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<PathSegment> for ArcSegment {}
impl ::windows_core::CanTryInto<super::DependencyObject> for ArcSegment {}
unsafe impl ::core::marker::Send for ArcSegment {}
unsafe impl ::core::marker::Sync for ArcSegment {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct BezierSegment(::windows_core::IUnknown);
impl BezierSegment {
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
            BezierSegment,
            ::windows_core::imp::IGenericFactory,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn Point1(&self) -> ::windows_core::Result<::windows::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Point1)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn SetPoint1(&self, value: ::windows::Foundation::Point) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetPoint1)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn Point2(&self) -> ::windows_core::Result<::windows::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Point2)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn SetPoint2(&self, value: ::windows::Foundation::Point) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetPoint2)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn Point3(&self) -> ::windows_core::Result<::windows::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Point3)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn SetPoint3(&self, value: ::windows::Foundation::Point) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetPoint3)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn Point1Property() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IBezierSegmentStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Point1Property)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn Point2Property() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IBezierSegmentStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Point2Property)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn Point3Property() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IBezierSegmentStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Point3Property)(
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
    pub fn IBezierSegmentStatics<
        R,
        F: FnOnce(&IBezierSegmentStatics) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<BezierSegment, IBezierSegmentStatics> =
            ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for BezierSegment {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for BezierSegment {
    type Vtable = IBezierSegment_Vtbl;
}
unsafe impl ::windows_core::ComInterface for BezierSegment {
    const IID: ::windows_core::GUID = <IBezierSegment as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for BezierSegment {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.BezierSegment";
}
::windows_core::imp::interface_hierarchy!(
    BezierSegment,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<PathSegment> for BezierSegment {}
impl ::windows_core::CanTryInto<super::DependencyObject> for BezierSegment {}
unsafe impl ::core::marker::Send for BezierSegment {}
unsafe impl ::core::marker::Sync for BezierSegment {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct BitmapCache(::windows_core::IUnknown);
impl BitmapCache {
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
            BitmapCache,
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
}
impl ::windows_core::RuntimeType for BitmapCache {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for BitmapCache {
    type Vtable = IBitmapCache_Vtbl;
}
unsafe impl ::windows_core::ComInterface for BitmapCache {
    const IID: ::windows_core::GUID = <IBitmapCache as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for BitmapCache {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.BitmapCache";
}
::windows_core::imp::interface_hierarchy!(
    BitmapCache,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<CacheMode> for BitmapCache {}
impl ::windows_core::CanTryInto<super::DependencyObject> for BitmapCache {}
unsafe impl ::core::marker::Send for BitmapCache {}
unsafe impl ::core::marker::Sync for BitmapCache {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct Brush(::windows_core::IUnknown);
impl Brush {
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
    pub fn Opacity(&self) -> ::windows_core::Result<f64> {
        let this = self;
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
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetOpacity)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn Transform(&self) -> ::windows_core::Result<Transform> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Transform)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetTransform<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<Transform>,
    {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetTransform)(
                ::windows_core::Interface::as_raw(this),
                value.try_into_param()?.abi(),
            )
            .ok()
        }
    }
    pub fn RelativeTransform(&self) -> ::windows_core::Result<Transform> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RelativeTransform)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetRelativeTransform<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<Transform>,
    {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetRelativeTransform)(
                ::windows_core::Interface::as_raw(this),
                value.try_into_param()?.abi(),
            )
            .ok()
        }
    }
    pub unsafe fn CreateInstance<P0>(
        baseinterface: P0,
        innerinterface: &mut ::core::option::Option<::windows_core::IInspectable>,
    ) -> ::windows_core::Result<Brush>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        Self::IBrushFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstance)(
                ::windows_core::Interface::as_raw(this),
                baseinterface.into_param().abi(),
                innerinterface as *mut _ as _,
                &mut result__,
            )
            .from_abi(result__)
        })
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
        let this = &::windows_core::ComInterface::cast::<IBrushOverrides>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).PopulatePropertyInfoOverride)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(propertyname),
                animationpropertyinfo.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn OpacityProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IBrushStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OpacityProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn TransformProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IBrushStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TransformProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn RelativeTransformProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IBrushStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RelativeTransformProperty)(
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
    pub fn IBrushFactory<R, F: FnOnce(&IBrushFactory) -> ::windows_core::Result<R>>(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<Brush, IBrushFactory> =
            ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IBrushStatics<R, F: FnOnce(&IBrushStatics) -> ::windows_core::Result<R>>(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<Brush, IBrushStatics> =
            ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for Brush {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for Brush {
    type Vtable = IBrush_Vtbl;
}
unsafe impl ::windows_core::ComInterface for Brush {
    const IID: ::windows_core::GUID = <IBrush as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for Brush {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.Brush";
}
::windows_core::imp::interface_hierarchy!(
    Brush,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
#[cfg(feature = "Microsoft_UI_Composition")]
impl ::windows_core::CanTryInto<super::super::Composition::IAnimationObject> for Brush {}
impl ::windows_core::CanTryInto<super::DependencyObject> for Brush {}
unsafe impl ::core::marker::Send for Brush {}
unsafe impl ::core::marker::Sync for Brush {}
#[doc = "Required features: `\"Windows_Foundation_Collections\"`"]
#[cfg(feature = "Windows_Foundation_Collections")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct BrushCollection(::windows_core::IUnknown);
#[cfg(feature = "Windows_Foundation_Collections")]
impl BrushCollection {
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
            BrushCollection,
            ::windows_core::imp::IGenericFactory,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "Required features: `\"Windows_Foundation_Collections\"`"]
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub fn First(
        &self,
    ) -> ::windows_core::Result<::windows::Foundation::Collections::IIterator<Brush>> {
        let this = &::windows_core::ComInterface::cast::<
            ::windows::Foundation::Collections::IIterable<Brush>,
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
    pub fn GetAt(&self, index: u32) -> ::windows_core::Result<Brush> {
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
    ) -> ::windows_core::Result<::windows::Foundation::Collections::IVectorView<Brush>> {
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
        P0: ::windows_core::TryIntoParam<Brush>,
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
        P0: ::windows_core::TryIntoParam<Brush>,
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
        P0: ::windows_core::TryIntoParam<Brush>,
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
        P0: ::windows_core::TryIntoParam<Brush>,
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
        items: &mut [::core::option::Option<Brush>],
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
        items: &[::core::option::Option<Brush>],
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
impl ::windows_core::RuntimeType for BrushCollection {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
#[cfg(feature = "Windows_Foundation_Collections")]
unsafe impl ::windows_core::Interface for BrushCollection {
    type Vtable = ::windows::Foundation::Collections::IVector_Vtbl<Brush>;
}
#[cfg(feature = "Windows_Foundation_Collections")]
unsafe impl ::windows_core::ComInterface for BrushCollection {
    const IID: ::windows_core::GUID =
        <::windows::Foundation::Collections::IVector<Brush> as ::windows_core::ComInterface>::IID;
}
#[cfg(feature = "Windows_Foundation_Collections")]
impl ::windows_core::RuntimeName for BrushCollection {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.BrushCollection";
}
#[cfg(feature = "Windows_Foundation_Collections")]
impl ::core::iter::IntoIterator for BrushCollection {
    type Item = Brush;
    type IntoIter = ::windows::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Windows_Foundation_Collections")]
impl ::core::iter::IntoIterator for &BrushCollection {
    type Item = Brush;
    type IntoIter = ::windows::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::windows::Foundation::Collections::VectorIterator::new(
            ::windows_core::ComInterface::cast(self).ok(),
        )
    }
}
#[cfg(feature = "Windows_Foundation_Collections")]
::windows_core::imp::interface_hierarchy!(
    BrushCollection,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
#[cfg(feature = "Windows_Foundation_Collections")]
impl ::windows_core::CanTryInto<::windows::Foundation::Collections::IIterable<Brush>>
    for BrushCollection
{
}
#[cfg(feature = "Windows_Foundation_Collections")]
impl ::windows_core::CanTryInto<::windows::Foundation::Collections::IVector<Brush>>
    for BrushCollection
{
}
#[cfg(feature = "Windows_Foundation_Collections")]
unsafe impl ::core::marker::Send for BrushCollection {}
#[cfg(feature = "Windows_Foundation_Collections")]
unsafe impl ::core::marker::Sync for BrushCollection {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct CacheMode(::windows_core::IUnknown);
impl CacheMode {
    pub unsafe fn CreateInstance<P0>(
        baseinterface: P0,
        innerinterface: &mut ::core::option::Option<::windows_core::IInspectable>,
    ) -> ::windows_core::Result<CacheMode>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        Self::ICacheModeFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstance)(
                ::windows_core::Interface::as_raw(this),
                baseinterface.into_param().abi(),
                innerinterface as *mut _ as _,
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
    pub fn ICacheModeFactory<R, F: FnOnce(&ICacheModeFactory) -> ::windows_core::Result<R>>(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<CacheMode, ICacheModeFactory> =
            ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for CacheMode {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for CacheMode {
    type Vtable = ICacheMode_Vtbl;
}
unsafe impl ::windows_core::ComInterface for CacheMode {
    const IID: ::windows_core::GUID = <ICacheMode as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for CacheMode {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.CacheMode";
}
::windows_core::imp::interface_hierarchy!(
    CacheMode,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<super::DependencyObject> for CacheMode {}
unsafe impl ::core::marker::Send for CacheMode {}
unsafe impl ::core::marker::Sync for CacheMode {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct CompositeTransform(::windows_core::IUnknown);
impl CompositeTransform {
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
            CompositeTransform,
            ::windows_core::imp::IGenericFactory,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn CenterX(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CenterX)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetCenterX(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetCenterX)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn CenterY(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CenterY)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetCenterY(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetCenterY)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn ScaleX(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ScaleX)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetScaleX(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetScaleX)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn ScaleY(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ScaleY)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetScaleY(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetScaleY)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn SkewX(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SkewX)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetSkewX(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetSkewX)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn SkewY(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SkewY)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetSkewY(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetSkewY)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn Rotation(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Rotation)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetRotation(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetRotation)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn TranslateX(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TranslateX)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetTranslateX(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetTranslateX)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn TranslateY(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TranslateY)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetTranslateY(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetTranslateY)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn CenterXProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::ICompositeTransformStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CenterXProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn CenterYProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::ICompositeTransformStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CenterYProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn ScaleXProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::ICompositeTransformStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ScaleXProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn ScaleYProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::ICompositeTransformStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ScaleYProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn SkewXProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::ICompositeTransformStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SkewXProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn SkewYProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::ICompositeTransformStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SkewYProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn RotationProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::ICompositeTransformStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RotationProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn TranslateXProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::ICompositeTransformStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TranslateXProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn TranslateYProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::ICompositeTransformStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TranslateYProperty)(
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
    pub fn Inverse(&self) -> ::windows_core::Result<GeneralTransform> {
        let this = &::windows_core::ComInterface::cast::<IGeneralTransform>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Inverse)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn TransformPoint(
        &self,
        point: ::windows::Foundation::Point,
    ) -> ::windows_core::Result<::windows::Foundation::Point> {
        let this = &::windows_core::ComInterface::cast::<IGeneralTransform>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TransformPoint)(
                ::windows_core::Interface::as_raw(this),
                point,
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn TryTransform(
        &self,
        inpoint: ::windows::Foundation::Point,
        outpoint: &mut ::windows::Foundation::Point,
    ) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IGeneralTransform>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TryTransform)(
                ::windows_core::Interface::as_raw(this),
                inpoint,
                outpoint,
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn TransformBounds(
        &self,
        rect: ::windows::Foundation::Rect,
    ) -> ::windows_core::Result<::windows::Foundation::Rect> {
        let this = &::windows_core::ComInterface::cast::<IGeneralTransform>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TransformBounds)(
                ::windows_core::Interface::as_raw(this),
                rect,
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn InverseCore(&self) -> ::windows_core::Result<GeneralTransform> {
        let this = &::windows_core::ComInterface::cast::<IGeneralTransformOverrides>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).InverseCore)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn TryTransformCore(
        &self,
        inpoint: ::windows::Foundation::Point,
        outpoint: &mut ::windows::Foundation::Point,
    ) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IGeneralTransformOverrides>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TryTransformCore)(
                ::windows_core::Interface::as_raw(this),
                inpoint,
                outpoint,
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn TransformBoundsCore(
        &self,
        rect: ::windows::Foundation::Rect,
    ) -> ::windows_core::Result<::windows::Foundation::Rect> {
        let this = &::windows_core::ComInterface::cast::<IGeneralTransformOverrides>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TransformBoundsCore)(
                ::windows_core::Interface::as_raw(this),
                rect,
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc(hidden)]
    pub fn ICompositeTransformStatics<
        R,
        F: FnOnce(&ICompositeTransformStatics) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<
            CompositeTransform,
            ICompositeTransformStatics,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for CompositeTransform {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for CompositeTransform {
    type Vtable = ICompositeTransform_Vtbl;
}
unsafe impl ::windows_core::ComInterface for CompositeTransform {
    const IID: ::windows_core::GUID = <ICompositeTransform as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for CompositeTransform {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.CompositeTransform";
}
::windows_core::imp::interface_hierarchy!(
    CompositeTransform,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<Transform> for CompositeTransform {}
impl ::windows_core::CanTryInto<GeneralTransform> for CompositeTransform {}
impl ::windows_core::CanTryInto<super::DependencyObject> for CompositeTransform {}
unsafe impl ::core::marker::Send for CompositeTransform {}
unsafe impl ::core::marker::Sync for CompositeTransform {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct CompositionTarget(::windows_core::IUnknown);
impl CompositionTarget {
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn Rendering<P0>(
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::EventHandler<::windows_core::IInspectable>,
        >,
    {
        Self::ICompositionTargetStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Rendering)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveRendering(
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        Self::ICompositionTargetStatics(|this| unsafe {
            (::windows_core::Interface::vtable(this).RemoveRendering)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        })
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn Rendered<P0>(
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<::windows::Foundation::EventHandler<RenderedEventArgs>>,
    {
        Self::ICompositionTargetStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Rendered)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveRendered(
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        Self::ICompositionTargetStatics(|this| unsafe {
            (::windows_core::Interface::vtable(this).RemoveRendered)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        })
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn SurfaceContentsLost<P0>(
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::EventHandler<::windows_core::IInspectable>,
        >,
    {
        Self::ICompositionTargetStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SurfaceContentsLost)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveSurfaceContentsLost(
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        Self::ICompositionTargetStatics(|this| unsafe {
            (::windows_core::Interface::vtable(this).RemoveSurfaceContentsLost)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        })
    }
    #[doc = "Required features: `\"Microsoft_UI_Composition\"`"]
    #[cfg(feature = "Microsoft_UI_Composition")]
    pub fn GetCompositorForCurrentThread(
    ) -> ::windows_core::Result<super::super::Composition::Compositor> {
        Self::ICompositionTargetStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetCompositorForCurrentThread)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ICompositionTargetStatics<
        R,
        F: FnOnce(&ICompositionTargetStatics) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<
            CompositionTarget,
            ICompositionTargetStatics,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for CompositionTarget {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for CompositionTarget {
    type Vtable = ICompositionTarget_Vtbl;
}
unsafe impl ::windows_core::ComInterface for CompositionTarget {
    const IID: ::windows_core::GUID = <ICompositionTarget as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for CompositionTarget {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.CompositionTarget";
}
::windows_core::imp::interface_hierarchy!(
    CompositionTarget,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for CompositionTarget {}
unsafe impl ::core::marker::Sync for CompositionTarget {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct DesktopAcrylicBackdrop(::windows_core::IUnknown);
impl DesktopAcrylicBackdrop {
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
    pub unsafe fn CreateInstance<P0>(
        baseinterface: P0,
        innerinterface: &mut ::core::option::Option<::windows_core::IInspectable>,
    ) -> ::windows_core::Result<DesktopAcrylicBackdrop>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        Self::IDesktopAcrylicBackdropFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstance)(
                ::windows_core::Interface::as_raw(this),
                baseinterface.into_param().abi(),
                innerinterface as *mut _ as _,
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Microsoft_UI_Composition_SystemBackdrops\"`"]
    #[cfg(feature = "Microsoft_UI_Composition_SystemBackdrops")]
    pub fn GetDefaultSystemBackdropConfiguration<P0, P1>(
        &self,
        target: P0,
        xamlroot: P1,
    ) -> ::windows_core::Result<
        super::super::Composition::SystemBackdrops::SystemBackdropConfiguration,
    >
    where
        P0: ::windows_core::TryIntoParam<
            super::super::Composition::ICompositionSupportsSystemBackdrop,
        >,
        P1: ::windows_core::IntoParam<super::XamlRoot>,
    {
        let this = &::windows_core::ComInterface::cast::<ISystemBackdrop>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetDefaultSystemBackdropConfiguration)(
                ::windows_core::Interface::as_raw(this),
                target.try_into_param()?.abi(),
                xamlroot.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Composition\"`"]
    #[cfg(feature = "Microsoft_UI_Composition")]
    pub fn OnTargetConnected<P0, P1>(
        &self,
        connectedtarget: P0,
        xamlroot: P1,
    ) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<
            super::super::Composition::ICompositionSupportsSystemBackdrop,
        >,
        P1: ::windows_core::IntoParam<super::XamlRoot>,
    {
        let this = &::windows_core::ComInterface::cast::<ISystemBackdropOverrides>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).OnTargetConnected)(
                ::windows_core::Interface::as_raw(this),
                connectedtarget.try_into_param()?.abi(),
                xamlroot.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Composition\"`"]
    #[cfg(feature = "Microsoft_UI_Composition")]
    pub fn OnTargetDisconnected<P0>(&self, disconnectedtarget: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<
            super::super::Composition::ICompositionSupportsSystemBackdrop,
        >,
    {
        let this = &::windows_core::ComInterface::cast::<ISystemBackdropOverrides>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).OnTargetDisconnected)(
                ::windows_core::Interface::as_raw(this),
                disconnectedtarget.try_into_param()?.abi(),
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Composition\"`"]
    #[cfg(feature = "Microsoft_UI_Composition")]
    pub fn OnDefaultSystemBackdropConfigurationChanged<P0, P1>(
        &self,
        target: P0,
        xamlroot: P1,
    ) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<
            super::super::Composition::ICompositionSupportsSystemBackdrop,
        >,
        P1: ::windows_core::IntoParam<super::XamlRoot>,
    {
        let this = &::windows_core::ComInterface::cast::<ISystemBackdropOverrides>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).OnDefaultSystemBackdropConfigurationChanged)(
                ::windows_core::Interface::as_raw(this),
                target.try_into_param()?.abi(),
                xamlroot.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc(hidden)]
    pub fn IDesktopAcrylicBackdropFactory<
        R,
        F: FnOnce(&IDesktopAcrylicBackdropFactory) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<
            DesktopAcrylicBackdrop,
            IDesktopAcrylicBackdropFactory,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for DesktopAcrylicBackdrop {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for DesktopAcrylicBackdrop {
    type Vtable = IDesktopAcrylicBackdrop_Vtbl;
}
unsafe impl ::windows_core::ComInterface for DesktopAcrylicBackdrop {
    const IID: ::windows_core::GUID =
        <IDesktopAcrylicBackdrop as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for DesktopAcrylicBackdrop {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.DesktopAcrylicBackdrop";
}
::windows_core::imp::interface_hierarchy!(
    DesktopAcrylicBackdrop,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<SystemBackdrop> for DesktopAcrylicBackdrop {}
impl ::windows_core::CanTryInto<super::DependencyObject> for DesktopAcrylicBackdrop {}
unsafe impl ::core::marker::Send for DesktopAcrylicBackdrop {}
unsafe impl ::core::marker::Sync for DesktopAcrylicBackdrop {}
#[doc = "Required features: `\"Windows_Foundation_Collections\"`"]
#[cfg(feature = "Windows_Foundation_Collections")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct DoubleCollection(::windows_core::IUnknown);
#[cfg(feature = "Windows_Foundation_Collections")]
impl DoubleCollection {
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
            DoubleCollection,
            ::windows_core::imp::IGenericFactory,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "Required features: `\"Windows_Foundation_Collections\"`"]
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub fn First(
        &self,
    ) -> ::windows_core::Result<::windows::Foundation::Collections::IIterator<f64>> {
        let this = &::windows_core::ComInterface::cast::<
            ::windows::Foundation::Collections::IIterable<f64>,
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
    pub fn GetAt(&self, index: u32) -> ::windows_core::Result<f64> {
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
    ) -> ::windows_core::Result<::windows::Foundation::Collections::IVectorView<f64>> {
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
    pub fn IndexOf(&self, value: f64, index: &mut u32) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IndexOf)(
                ::windows_core::Interface::as_raw(this),
                value,
                index,
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation_Collections\"`"]
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub fn SetAt(&self, index: u32, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetAt)(
                ::windows_core::Interface::as_raw(this),
                index,
                value,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation_Collections\"`"]
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub fn InsertAt(&self, index: u32, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).InsertAt)(
                ::windows_core::Interface::as_raw(this),
                index,
                value,
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
    pub fn Append(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).Append)(
                ::windows_core::Interface::as_raw(this),
                value,
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
    pub fn GetMany(&self, startindex: u32, items: &mut [f64]) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetMany)(
                ::windows_core::Interface::as_raw(this),
                startindex,
                items.len().try_into().unwrap(),
                items.as_mut_ptr(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation_Collections\"`"]
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub fn ReplaceAll(&self, items: &[f64]) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).ReplaceAll)(
                ::windows_core::Interface::as_raw(this),
                items.len().try_into().unwrap(),
                items.as_ptr(),
            )
            .ok()
        }
    }
}
#[cfg(feature = "Windows_Foundation_Collections")]
impl ::windows_core::RuntimeType for DoubleCollection {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
#[cfg(feature = "Windows_Foundation_Collections")]
unsafe impl ::windows_core::Interface for DoubleCollection {
    type Vtable = ::windows::Foundation::Collections::IVector_Vtbl<f64>;
}
#[cfg(feature = "Windows_Foundation_Collections")]
unsafe impl ::windows_core::ComInterface for DoubleCollection {
    const IID: ::windows_core::GUID =
        <::windows::Foundation::Collections::IVector<f64> as ::windows_core::ComInterface>::IID;
}
#[cfg(feature = "Windows_Foundation_Collections")]
impl ::windows_core::RuntimeName for DoubleCollection {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.DoubleCollection";
}
#[cfg(feature = "Windows_Foundation_Collections")]
impl ::core::iter::IntoIterator for DoubleCollection {
    type Item = f64;
    type IntoIter = ::windows::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Windows_Foundation_Collections")]
impl ::core::iter::IntoIterator for &DoubleCollection {
    type Item = f64;
    type IntoIter = ::windows::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::windows::Foundation::Collections::VectorIterator::new(
            ::windows_core::ComInterface::cast(self).ok(),
        )
    }
}
#[cfg(feature = "Windows_Foundation_Collections")]
::windows_core::imp::interface_hierarchy!(
    DoubleCollection,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
#[cfg(feature = "Windows_Foundation_Collections")]
impl ::windows_core::CanTryInto<::windows::Foundation::Collections::IIterable<f64>>
    for DoubleCollection
{
}
#[cfg(feature = "Windows_Foundation_Collections")]
impl ::windows_core::CanTryInto<::windows::Foundation::Collections::IVector<f64>>
    for DoubleCollection
{
}
#[cfg(feature = "Windows_Foundation_Collections")]
unsafe impl ::core::marker::Send for DoubleCollection {}
#[cfg(feature = "Windows_Foundation_Collections")]
unsafe impl ::core::marker::Sync for DoubleCollection {}
pub const E_SURFACE_CONTENTS_LOST: u32 = 2150301728u32;
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct EllipseGeometry(::windows_core::IUnknown);
impl EllipseGeometry {
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
            EllipseGeometry,
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
    pub fn Center(&self) -> ::windows_core::Result<::windows::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Center)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn SetCenter(&self, value: ::windows::Foundation::Point) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetCenter)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn RadiusX(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RadiusX)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetRadiusX(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetRadiusX)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn RadiusY(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RadiusY)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetRadiusY(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetRadiusY)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn CenterProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IEllipseGeometryStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CenterProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn RadiusXProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IEllipseGeometryStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RadiusXProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn RadiusYProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IEllipseGeometryStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RadiusYProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn Transform(&self) -> ::windows_core::Result<Transform> {
        let this = &::windows_core::ComInterface::cast::<IGeometry>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Transform)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetTransform<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<Transform>,
    {
        let this = &::windows_core::ComInterface::cast::<IGeometry>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetTransform)(
                ::windows_core::Interface::as_raw(this),
                value.try_into_param()?.abi(),
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn Bounds(&self) -> ::windows_core::Result<::windows::Foundation::Rect> {
        let this = &::windows_core::ComInterface::cast::<IGeometry>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Bounds)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc(hidden)]
    pub fn IEllipseGeometryStatics<
        R,
        F: FnOnce(&IEllipseGeometryStatics) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<EllipseGeometry, IEllipseGeometryStatics> =
            ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for EllipseGeometry {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for EllipseGeometry {
    type Vtable = IEllipseGeometry_Vtbl;
}
unsafe impl ::windows_core::ComInterface for EllipseGeometry {
    const IID: ::windows_core::GUID = <IEllipseGeometry as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for EllipseGeometry {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.EllipseGeometry";
}
::windows_core::imp::interface_hierarchy!(
    EllipseGeometry,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<Geometry> for EllipseGeometry {}
impl ::windows_core::CanTryInto<super::DependencyObject> for EllipseGeometry {}
unsafe impl ::core::marker::Send for EllipseGeometry {}
unsafe impl ::core::marker::Sync for EllipseGeometry {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct FontFamily(::windows_core::IUnknown);
impl FontFamily {
    pub fn Source(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Source)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub unsafe fn CreateInstanceWithName<P0>(
        familyname: &::windows_core::HSTRING,
        baseinterface: P0,
        innerinterface: &mut ::core::option::Option<::windows_core::IInspectable>,
    ) -> ::windows_core::Result<FontFamily>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        Self::IFontFamilyFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstanceWithName)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(familyname),
                baseinterface.into_param().abi(),
                innerinterface as *mut _ as _,
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn XamlAutoFontFamily() -> ::windows_core::Result<FontFamily> {
        Self::IFontFamilyStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).XamlAutoFontFamily)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IFontFamilyFactory<R, F: FnOnce(&IFontFamilyFactory) -> ::windows_core::Result<R>>(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<FontFamily, IFontFamilyFactory> =
            ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IFontFamilyStatics<R, F: FnOnce(&IFontFamilyStatics) -> ::windows_core::Result<R>>(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<FontFamily, IFontFamilyStatics> =
            ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for FontFamily {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for FontFamily {
    type Vtable = IFontFamily_Vtbl;
}
unsafe impl ::windows_core::ComInterface for FontFamily {
    const IID: ::windows_core::GUID = <IFontFamily as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for FontFamily {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.FontFamily";
}
::windows_core::imp::interface_hierarchy!(
    FontFamily,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for FontFamily {}
unsafe impl ::core::marker::Sync for FontFamily {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct GeneralTransform(::windows_core::IUnknown);
impl GeneralTransform {
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
    pub fn Inverse(&self) -> ::windows_core::Result<GeneralTransform> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Inverse)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn TransformPoint(
        &self,
        point: ::windows::Foundation::Point,
    ) -> ::windows_core::Result<::windows::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TransformPoint)(
                ::windows_core::Interface::as_raw(this),
                point,
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn TryTransform(
        &self,
        inpoint: ::windows::Foundation::Point,
        outpoint: &mut ::windows::Foundation::Point,
    ) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TryTransform)(
                ::windows_core::Interface::as_raw(this),
                inpoint,
                outpoint,
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn TransformBounds(
        &self,
        rect: ::windows::Foundation::Rect,
    ) -> ::windows_core::Result<::windows::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TransformBounds)(
                ::windows_core::Interface::as_raw(this),
                rect,
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub unsafe fn CreateInstance<P0>(
        baseinterface: P0,
        innerinterface: &mut ::core::option::Option<::windows_core::IInspectable>,
    ) -> ::windows_core::Result<GeneralTransform>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        Self::IGeneralTransformFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstance)(
                ::windows_core::Interface::as_raw(this),
                baseinterface.into_param().abi(),
                innerinterface as *mut _ as _,
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn InverseCore(&self) -> ::windows_core::Result<GeneralTransform> {
        let this = &::windows_core::ComInterface::cast::<IGeneralTransformOverrides>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).InverseCore)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn TryTransformCore(
        &self,
        inpoint: ::windows::Foundation::Point,
        outpoint: &mut ::windows::Foundation::Point,
    ) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IGeneralTransformOverrides>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TryTransformCore)(
                ::windows_core::Interface::as_raw(this),
                inpoint,
                outpoint,
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn TransformBoundsCore(
        &self,
        rect: ::windows::Foundation::Rect,
    ) -> ::windows_core::Result<::windows::Foundation::Rect> {
        let this = &::windows_core::ComInterface::cast::<IGeneralTransformOverrides>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TransformBoundsCore)(
                ::windows_core::Interface::as_raw(this),
                rect,
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc(hidden)]
    pub fn IGeneralTransformFactory<
        R,
        F: FnOnce(&IGeneralTransformFactory) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<
            GeneralTransform,
            IGeneralTransformFactory,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for GeneralTransform {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for GeneralTransform {
    type Vtable = IGeneralTransform_Vtbl;
}
unsafe impl ::windows_core::ComInterface for GeneralTransform {
    const IID: ::windows_core::GUID = <IGeneralTransform as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for GeneralTransform {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.GeneralTransform";
}
::windows_core::imp::interface_hierarchy!(
    GeneralTransform,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<super::DependencyObject> for GeneralTransform {}
unsafe impl ::core::marker::Send for GeneralTransform {}
unsafe impl ::core::marker::Sync for GeneralTransform {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct Geometry(::windows_core::IUnknown);
impl Geometry {
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
    pub fn Transform(&self) -> ::windows_core::Result<Transform> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Transform)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetTransform<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<Transform>,
    {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetTransform)(
                ::windows_core::Interface::as_raw(this),
                value.try_into_param()?.abi(),
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn Bounds(&self) -> ::windows_core::Result<::windows::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Bounds)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn Empty() -> ::windows_core::Result<Geometry> {
        Self::IGeometryStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Empty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn StandardFlatteningTolerance() -> ::windows_core::Result<f64> {
        Self::IGeometryStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StandardFlatteningTolerance)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn TransformProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IGeometryStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TransformProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IGeometryStatics<R, F: FnOnce(&IGeometryStatics) -> ::windows_core::Result<R>>(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<Geometry, IGeometryStatics> =
            ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for Geometry {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for Geometry {
    type Vtable = IGeometry_Vtbl;
}
unsafe impl ::windows_core::ComInterface for Geometry {
    const IID: ::windows_core::GUID = <IGeometry as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for Geometry {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.Geometry";
}
::windows_core::imp::interface_hierarchy!(
    Geometry,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<super::DependencyObject> for Geometry {}
unsafe impl ::core::marker::Send for Geometry {}
unsafe impl ::core::marker::Sync for Geometry {}
#[doc = "Required features: `\"Windows_Foundation_Collections\"`"]
#[cfg(feature = "Windows_Foundation_Collections")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct GeometryCollection(::windows_core::IUnknown);
#[cfg(feature = "Windows_Foundation_Collections")]
impl GeometryCollection {
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
            GeometryCollection,
            ::windows_core::imp::IGenericFactory,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "Required features: `\"Windows_Foundation_Collections\"`"]
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub fn First(
        &self,
    ) -> ::windows_core::Result<::windows::Foundation::Collections::IIterator<Geometry>> {
        let this = &::windows_core::ComInterface::cast::<
            ::windows::Foundation::Collections::IIterable<Geometry>,
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
    pub fn GetAt(&self, index: u32) -> ::windows_core::Result<Geometry> {
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
    ) -> ::windows_core::Result<::windows::Foundation::Collections::IVectorView<Geometry>> {
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
        P0: ::windows_core::TryIntoParam<Geometry>,
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
        P0: ::windows_core::TryIntoParam<Geometry>,
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
        P0: ::windows_core::TryIntoParam<Geometry>,
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
        P0: ::windows_core::TryIntoParam<Geometry>,
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
        items: &mut [::core::option::Option<Geometry>],
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
        items: &[::core::option::Option<Geometry>],
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
impl ::windows_core::RuntimeType for GeometryCollection {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
#[cfg(feature = "Windows_Foundation_Collections")]
unsafe impl ::windows_core::Interface for GeometryCollection {
    type Vtable = ::windows::Foundation::Collections::IVector_Vtbl<Geometry>;
}
#[cfg(feature = "Windows_Foundation_Collections")]
unsafe impl ::windows_core::ComInterface for GeometryCollection {
    const IID : ::windows_core::GUID = < ::windows::Foundation::Collections:: IVector :: < Geometry > as::windows_core::ComInterface >::IID ;
}
#[cfg(feature = "Windows_Foundation_Collections")]
impl ::windows_core::RuntimeName for GeometryCollection {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.GeometryCollection";
}
#[cfg(feature = "Windows_Foundation_Collections")]
impl ::core::iter::IntoIterator for GeometryCollection {
    type Item = Geometry;
    type IntoIter = ::windows::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Windows_Foundation_Collections")]
impl ::core::iter::IntoIterator for &GeometryCollection {
    type Item = Geometry;
    type IntoIter = ::windows::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::windows::Foundation::Collections::VectorIterator::new(
            ::windows_core::ComInterface::cast(self).ok(),
        )
    }
}
#[cfg(feature = "Windows_Foundation_Collections")]
::windows_core::imp::interface_hierarchy!(
    GeometryCollection,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
#[cfg(feature = "Windows_Foundation_Collections")]
impl ::windows_core::CanTryInto<::windows::Foundation::Collections::IIterable<Geometry>>
    for GeometryCollection
{
}
#[cfg(feature = "Windows_Foundation_Collections")]
impl ::windows_core::CanTryInto<::windows::Foundation::Collections::IVector<Geometry>>
    for GeometryCollection
{
}
#[cfg(feature = "Windows_Foundation_Collections")]
unsafe impl ::core::marker::Send for GeometryCollection {}
#[cfg(feature = "Windows_Foundation_Collections")]
unsafe impl ::core::marker::Sync for GeometryCollection {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct GeometryGroup(::windows_core::IUnknown);
impl GeometryGroup {
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
            GeometryGroup,
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
    pub fn Transform(&self) -> ::windows_core::Result<Transform> {
        let this = &::windows_core::ComInterface::cast::<IGeometry>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Transform)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetTransform<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<Transform>,
    {
        let this = &::windows_core::ComInterface::cast::<IGeometry>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetTransform)(
                ::windows_core::Interface::as_raw(this),
                value.try_into_param()?.abi(),
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn Bounds(&self) -> ::windows_core::Result<::windows::Foundation::Rect> {
        let this = &::windows_core::ComInterface::cast::<IGeometry>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Bounds)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn FillRule(&self) -> ::windows_core::Result<FillRule> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FillRule)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetFillRule(&self, value: FillRule) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetFillRule)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation_Collections\"`"]
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub fn Children(&self) -> ::windows_core::Result<GeometryCollection> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Children)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation_Collections\"`"]
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub fn SetChildren<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<GeometryCollection>,
    {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetChildren)(
                ::windows_core::Interface::as_raw(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn FillRuleProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IGeometryGroupStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FillRuleProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn ChildrenProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IGeometryGroupStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ChildrenProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IGeometryGroupStatics<
        R,
        F: FnOnce(&IGeometryGroupStatics) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<GeometryGroup, IGeometryGroupStatics> =
            ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for GeometryGroup {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for GeometryGroup {
    type Vtable = IGeometryGroup_Vtbl;
}
unsafe impl ::windows_core::ComInterface for GeometryGroup {
    const IID: ::windows_core::GUID = <IGeometryGroup as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for GeometryGroup {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.GeometryGroup";
}
::windows_core::imp::interface_hierarchy!(
    GeometryGroup,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<Geometry> for GeometryGroup {}
impl ::windows_core::CanTryInto<super::DependencyObject> for GeometryGroup {}
unsafe impl ::core::marker::Send for GeometryGroup {}
unsafe impl ::core::marker::Sync for GeometryGroup {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct GradientBrush(::windows_core::IUnknown);
impl GradientBrush {
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
    pub fn Opacity(&self) -> ::windows_core::Result<f64> {
        let this = &::windows_core::ComInterface::cast::<IBrush>(self)?;
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
        let this = &::windows_core::ComInterface::cast::<IBrush>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetOpacity)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn Transform(&self) -> ::windows_core::Result<Transform> {
        let this = &::windows_core::ComInterface::cast::<IBrush>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Transform)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetTransform<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<Transform>,
    {
        let this = &::windows_core::ComInterface::cast::<IBrush>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetTransform)(
                ::windows_core::Interface::as_raw(this),
                value.try_into_param()?.abi(),
            )
            .ok()
        }
    }
    pub fn RelativeTransform(&self) -> ::windows_core::Result<Transform> {
        let this = &::windows_core::ComInterface::cast::<IBrush>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RelativeTransform)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetRelativeTransform<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<Transform>,
    {
        let this = &::windows_core::ComInterface::cast::<IBrush>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetRelativeTransform)(
                ::windows_core::Interface::as_raw(this),
                value.try_into_param()?.abi(),
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
        let this = &::windows_core::ComInterface::cast::<IBrushOverrides>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).PopulatePropertyInfoOverride)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(propertyname),
                animationpropertyinfo.into_param().abi(),
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
    pub fn SpreadMethod(&self) -> ::windows_core::Result<GradientSpreadMethod> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SpreadMethod)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetSpreadMethod(&self, value: GradientSpreadMethod) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetSpreadMethod)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn MappingMode(&self) -> ::windows_core::Result<BrushMappingMode> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MappingMode)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetMappingMode(&self, value: BrushMappingMode) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetMappingMode)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn ColorInterpolationMode(&self) -> ::windows_core::Result<ColorInterpolationMode> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ColorInterpolationMode)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetColorInterpolationMode(
        &self,
        value: ColorInterpolationMode,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetColorInterpolationMode)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation_Collections\"`"]
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub fn GradientStops(&self) -> ::windows_core::Result<GradientStopCollection> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GradientStops)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation_Collections\"`"]
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub fn SetGradientStops<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<GradientStopCollection>,
    {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetGradientStops)(
                ::windows_core::Interface::as_raw(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub unsafe fn CreateInstance<P0>(
        baseinterface: P0,
        innerinterface: &mut ::core::option::Option<::windows_core::IInspectable>,
    ) -> ::windows_core::Result<GradientBrush>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        Self::IGradientBrushFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstance)(
                ::windows_core::Interface::as_raw(this),
                baseinterface.into_param().abi(),
                innerinterface as *mut _ as _,
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn SpreadMethodProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IGradientBrushStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SpreadMethodProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn MappingModeProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IGradientBrushStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MappingModeProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn ColorInterpolationModeProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IGradientBrushStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ColorInterpolationModeProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn GradientStopsProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IGradientBrushStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GradientStopsProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IGradientBrushFactory<
        R,
        F: FnOnce(&IGradientBrushFactory) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<GradientBrush, IGradientBrushFactory> =
            ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IGradientBrushStatics<
        R,
        F: FnOnce(&IGradientBrushStatics) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<GradientBrush, IGradientBrushStatics> =
            ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for GradientBrush {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for GradientBrush {
    type Vtable = IGradientBrush_Vtbl;
}
unsafe impl ::windows_core::ComInterface for GradientBrush {
    const IID: ::windows_core::GUID = <IGradientBrush as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for GradientBrush {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.GradientBrush";
}
::windows_core::imp::interface_hierarchy!(
    GradientBrush,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
#[cfg(feature = "Microsoft_UI_Composition")]
impl ::windows_core::CanTryInto<super::super::Composition::IAnimationObject> for GradientBrush {}
impl ::windows_core::CanTryInto<Brush> for GradientBrush {}
impl ::windows_core::CanTryInto<super::DependencyObject> for GradientBrush {}
unsafe impl ::core::marker::Send for GradientBrush {}
unsafe impl ::core::marker::Sync for GradientBrush {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct GradientStop(::windows_core::IUnknown);
impl GradientStop {
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
            GradientStop,
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
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn Color(&self) -> ::windows_core::Result<::windows::UI::Color> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Color)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn SetColor(&self, value: ::windows::UI::Color) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetColor)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn Offset(&self) -> ::windows_core::Result<f64> {
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
    pub fn SetOffset(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetOffset)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn ColorProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IGradientStopStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ColorProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn OffsetProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IGradientStopStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OffsetProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IGradientStopStatics<
        R,
        F: FnOnce(&IGradientStopStatics) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<GradientStop, IGradientStopStatics> =
            ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for GradientStop {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for GradientStop {
    type Vtable = IGradientStop_Vtbl;
}
unsafe impl ::windows_core::ComInterface for GradientStop {
    const IID: ::windows_core::GUID = <IGradientStop as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for GradientStop {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.GradientStop";
}
::windows_core::imp::interface_hierarchy!(
    GradientStop,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<super::DependencyObject> for GradientStop {}
unsafe impl ::core::marker::Send for GradientStop {}
unsafe impl ::core::marker::Sync for GradientStop {}
#[doc = "Required features: `\"Windows_Foundation_Collections\"`"]
#[cfg(feature = "Windows_Foundation_Collections")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct GradientStopCollection(::windows_core::IUnknown);
#[cfg(feature = "Windows_Foundation_Collections")]
impl GradientStopCollection {
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
            GradientStopCollection,
            ::windows_core::imp::IGenericFactory,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "Required features: `\"Windows_Foundation_Collections\"`"]
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub fn First(
        &self,
    ) -> ::windows_core::Result<::windows::Foundation::Collections::IIterator<GradientStop>> {
        let this = &::windows_core::ComInterface::cast::<
            ::windows::Foundation::Collections::IIterable<GradientStop>,
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
    pub fn GetAt(&self, index: u32) -> ::windows_core::Result<GradientStop> {
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
    ) -> ::windows_core::Result<::windows::Foundation::Collections::IVectorView<GradientStop>> {
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
        P0: ::windows_core::IntoParam<GradientStop>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IndexOf)(
                ::windows_core::Interface::as_raw(this),
                value.into_param().abi(),
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
        P0: ::windows_core::IntoParam<GradientStop>,
    {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetAt)(
                ::windows_core::Interface::as_raw(this),
                index,
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation_Collections\"`"]
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub fn InsertAt<P0>(&self, index: u32, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<GradientStop>,
    {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).InsertAt)(
                ::windows_core::Interface::as_raw(this),
                index,
                value.into_param().abi(),
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
        P0: ::windows_core::IntoParam<GradientStop>,
    {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).Append)(
                ::windows_core::Interface::as_raw(this),
                value.into_param().abi(),
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
        items: &mut [::core::option::Option<GradientStop>],
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
        items: &[::core::option::Option<GradientStop>],
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
impl ::windows_core::RuntimeType for GradientStopCollection {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
#[cfg(feature = "Windows_Foundation_Collections")]
unsafe impl ::windows_core::Interface for GradientStopCollection {
    type Vtable = ::windows::Foundation::Collections::IVector_Vtbl<GradientStop>;
}
#[cfg(feature = "Windows_Foundation_Collections")]
unsafe impl ::windows_core::ComInterface for GradientStopCollection {
    const IID : ::windows_core::GUID = < ::windows::Foundation::Collections:: IVector :: < GradientStop > as::windows_core::ComInterface >::IID ;
}
#[cfg(feature = "Windows_Foundation_Collections")]
impl ::windows_core::RuntimeName for GradientStopCollection {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.GradientStopCollection";
}
#[cfg(feature = "Windows_Foundation_Collections")]
impl ::core::iter::IntoIterator for GradientStopCollection {
    type Item = GradientStop;
    type IntoIter = ::windows::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Windows_Foundation_Collections")]
impl ::core::iter::IntoIterator for &GradientStopCollection {
    type Item = GradientStop;
    type IntoIter = ::windows::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::windows::Foundation::Collections::VectorIterator::new(
            ::windows_core::ComInterface::cast(self).ok(),
        )
    }
}
#[cfg(feature = "Windows_Foundation_Collections")]
::windows_core::imp::interface_hierarchy!(
    GradientStopCollection,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
#[cfg(feature = "Windows_Foundation_Collections")]
impl ::windows_core::CanTryInto<::windows::Foundation::Collections::IIterable<GradientStop>>
    for GradientStopCollection
{
}
#[cfg(feature = "Windows_Foundation_Collections")]
impl ::windows_core::CanTryInto<::windows::Foundation::Collections::IVector<GradientStop>>
    for GradientStopCollection
{
}
#[cfg(feature = "Windows_Foundation_Collections")]
unsafe impl ::core::marker::Send for GradientStopCollection {}
#[cfg(feature = "Windows_Foundation_Collections")]
unsafe impl ::core::marker::Sync for GradientStopCollection {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ImageBrush(::windows_core::IUnknown);
impl ImageBrush {
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
            ImageBrush,
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
    pub fn Opacity(&self) -> ::windows_core::Result<f64> {
        let this = &::windows_core::ComInterface::cast::<IBrush>(self)?;
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
        let this = &::windows_core::ComInterface::cast::<IBrush>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetOpacity)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn Transform(&self) -> ::windows_core::Result<Transform> {
        let this = &::windows_core::ComInterface::cast::<IBrush>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Transform)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetTransform<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<Transform>,
    {
        let this = &::windows_core::ComInterface::cast::<IBrush>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetTransform)(
                ::windows_core::Interface::as_raw(this),
                value.try_into_param()?.abi(),
            )
            .ok()
        }
    }
    pub fn RelativeTransform(&self) -> ::windows_core::Result<Transform> {
        let this = &::windows_core::ComInterface::cast::<IBrush>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RelativeTransform)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetRelativeTransform<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<Transform>,
    {
        let this = &::windows_core::ComInterface::cast::<IBrush>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetRelativeTransform)(
                ::windows_core::Interface::as_raw(this),
                value.try_into_param()?.abi(),
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
        let this = &::windows_core::ComInterface::cast::<IBrushOverrides>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).PopulatePropertyInfoOverride)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(propertyname),
                animationpropertyinfo.into_param().abi(),
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
    pub fn ImageSource(&self) -> ::windows_core::Result<ImageSource> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ImageSource)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetImageSource<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<ImageSource>,
    {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetImageSource)(
                ::windows_core::Interface::as_raw(this),
                value.try_into_param()?.abi(),
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn ImageFailed<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::ExceptionRoutedEventHandler>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ImageFailed)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveImageFailed(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveImageFailed)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn ImageOpened<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::RoutedEventHandler>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ImageOpened)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveImageOpened(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveImageOpened)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn ImageSourceProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IImageBrushStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ImageSourceProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn AlignmentX(&self) -> ::windows_core::Result<AlignmentX> {
        let this = &::windows_core::ComInterface::cast::<ITileBrush>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AlignmentX)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetAlignmentX(&self, value: AlignmentX) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITileBrush>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetAlignmentX)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn AlignmentY(&self) -> ::windows_core::Result<AlignmentY> {
        let this = &::windows_core::ComInterface::cast::<ITileBrush>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AlignmentY)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetAlignmentY(&self, value: AlignmentY) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITileBrush>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetAlignmentY)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn Stretch(&self) -> ::windows_core::Result<Stretch> {
        let this = &::windows_core::ComInterface::cast::<ITileBrush>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Stretch)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetStretch(&self, value: Stretch) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITileBrush>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetStretch)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc(hidden)]
    pub fn IImageBrushStatics<R, F: FnOnce(&IImageBrushStatics) -> ::windows_core::Result<R>>(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<ImageBrush, IImageBrushStatics> =
            ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for ImageBrush {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for ImageBrush {
    type Vtable = IImageBrush_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ImageBrush {
    const IID: ::windows_core::GUID = <IImageBrush as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ImageBrush {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.ImageBrush";
}
::windows_core::imp::interface_hierarchy!(
    ImageBrush,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
#[cfg(feature = "Microsoft_UI_Composition")]
impl ::windows_core::CanTryInto<super::super::Composition::IAnimationObject> for ImageBrush {}
impl ::windows_core::CanTryInto<TileBrush> for ImageBrush {}
impl ::windows_core::CanTryInto<Brush> for ImageBrush {}
impl ::windows_core::CanTryInto<super::DependencyObject> for ImageBrush {}
unsafe impl ::core::marker::Send for ImageBrush {}
unsafe impl ::core::marker::Sync for ImageBrush {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ImageSource(::windows_core::IUnknown);
impl ImageSource {
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
impl ::windows_core::RuntimeType for ImageSource {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for ImageSource {
    type Vtable = IImageSource_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ImageSource {
    const IID: ::windows_core::GUID = <IImageSource as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ImageSource {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.ImageSource";
}
::windows_core::imp::interface_hierarchy!(
    ImageSource,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<super::DependencyObject> for ImageSource {}
unsafe impl ::core::marker::Send for ImageSource {}
unsafe impl ::core::marker::Sync for ImageSource {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct LineGeometry(::windows_core::IUnknown);
impl LineGeometry {
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
            LineGeometry,
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
    pub fn Transform(&self) -> ::windows_core::Result<Transform> {
        let this = &::windows_core::ComInterface::cast::<IGeometry>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Transform)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetTransform<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<Transform>,
    {
        let this = &::windows_core::ComInterface::cast::<IGeometry>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetTransform)(
                ::windows_core::Interface::as_raw(this),
                value.try_into_param()?.abi(),
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn Bounds(&self) -> ::windows_core::Result<::windows::Foundation::Rect> {
        let this = &::windows_core::ComInterface::cast::<IGeometry>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Bounds)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn StartPoint(&self) -> ::windows_core::Result<::windows::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StartPoint)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn SetStartPoint(&self, value: ::windows::Foundation::Point) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetStartPoint)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn EndPoint(&self) -> ::windows_core::Result<::windows::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).EndPoint)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn SetEndPoint(&self, value: ::windows::Foundation::Point) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetEndPoint)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn StartPointProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::ILineGeometryStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StartPointProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn EndPointProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::ILineGeometryStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).EndPointProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ILineGeometryStatics<
        R,
        F: FnOnce(&ILineGeometryStatics) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<LineGeometry, ILineGeometryStatics> =
            ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for LineGeometry {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for LineGeometry {
    type Vtable = ILineGeometry_Vtbl;
}
unsafe impl ::windows_core::ComInterface for LineGeometry {
    const IID: ::windows_core::GUID = <ILineGeometry as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for LineGeometry {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.LineGeometry";
}
::windows_core::imp::interface_hierarchy!(
    LineGeometry,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<Geometry> for LineGeometry {}
impl ::windows_core::CanTryInto<super::DependencyObject> for LineGeometry {}
unsafe impl ::core::marker::Send for LineGeometry {}
unsafe impl ::core::marker::Sync for LineGeometry {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct LineSegment(::windows_core::IUnknown);
impl LineSegment {
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
            LineSegment,
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
    pub fn Point(&self) -> ::windows_core::Result<::windows::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Point)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn SetPoint(&self, value: ::windows::Foundation::Point) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetPoint)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn PointProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::ILineSegmentStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PointProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ILineSegmentStatics<R, F: FnOnce(&ILineSegmentStatics) -> ::windows_core::Result<R>>(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<LineSegment, ILineSegmentStatics> =
            ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for LineSegment {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for LineSegment {
    type Vtable = ILineSegment_Vtbl;
}
unsafe impl ::windows_core::ComInterface for LineSegment {
    const IID: ::windows_core::GUID = <ILineSegment as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for LineSegment {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.LineSegment";
}
::windows_core::imp::interface_hierarchy!(
    LineSegment,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<PathSegment> for LineSegment {}
impl ::windows_core::CanTryInto<super::DependencyObject> for LineSegment {}
unsafe impl ::core::marker::Send for LineSegment {}
unsafe impl ::core::marker::Sync for LineSegment {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct LinearGradientBrush(::windows_core::IUnknown);
impl LinearGradientBrush {
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
            LinearGradientBrush,
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
    pub fn Opacity(&self) -> ::windows_core::Result<f64> {
        let this = &::windows_core::ComInterface::cast::<IBrush>(self)?;
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
        let this = &::windows_core::ComInterface::cast::<IBrush>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetOpacity)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn Transform(&self) -> ::windows_core::Result<Transform> {
        let this = &::windows_core::ComInterface::cast::<IBrush>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Transform)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetTransform<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<Transform>,
    {
        let this = &::windows_core::ComInterface::cast::<IBrush>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetTransform)(
                ::windows_core::Interface::as_raw(this),
                value.try_into_param()?.abi(),
            )
            .ok()
        }
    }
    pub fn RelativeTransform(&self) -> ::windows_core::Result<Transform> {
        let this = &::windows_core::ComInterface::cast::<IBrush>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RelativeTransform)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetRelativeTransform<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<Transform>,
    {
        let this = &::windows_core::ComInterface::cast::<IBrush>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetRelativeTransform)(
                ::windows_core::Interface::as_raw(this),
                value.try_into_param()?.abi(),
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
        let this = &::windows_core::ComInterface::cast::<IBrushOverrides>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).PopulatePropertyInfoOverride)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(propertyname),
                animationpropertyinfo.into_param().abi(),
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
    pub fn SpreadMethod(&self) -> ::windows_core::Result<GradientSpreadMethod> {
        let this = &::windows_core::ComInterface::cast::<IGradientBrush>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SpreadMethod)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetSpreadMethod(&self, value: GradientSpreadMethod) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IGradientBrush>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetSpreadMethod)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn MappingMode(&self) -> ::windows_core::Result<BrushMappingMode> {
        let this = &::windows_core::ComInterface::cast::<IGradientBrush>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MappingMode)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetMappingMode(&self, value: BrushMappingMode) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IGradientBrush>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetMappingMode)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn ColorInterpolationMode(&self) -> ::windows_core::Result<ColorInterpolationMode> {
        let this = &::windows_core::ComInterface::cast::<IGradientBrush>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ColorInterpolationMode)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetColorInterpolationMode(
        &self,
        value: ColorInterpolationMode,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IGradientBrush>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetColorInterpolationMode)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation_Collections\"`"]
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub fn GradientStops(&self) -> ::windows_core::Result<GradientStopCollection> {
        let this = &::windows_core::ComInterface::cast::<IGradientBrush>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GradientStops)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation_Collections\"`"]
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub fn SetGradientStops<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<GradientStopCollection>,
    {
        let this = &::windows_core::ComInterface::cast::<IGradientBrush>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetGradientStops)(
                ::windows_core::Interface::as_raw(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn StartPoint(&self) -> ::windows_core::Result<::windows::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StartPoint)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn SetStartPoint(&self, value: ::windows::Foundation::Point) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetStartPoint)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn EndPoint(&self) -> ::windows_core::Result<::windows::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).EndPoint)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn SetEndPoint(&self, value: ::windows::Foundation::Point) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetEndPoint)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation_Collections\"`"]
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub fn CreateInstanceWithGradientStopCollectionAndAngle<P0>(
        gradientstopcollection: P0,
        angle: f64,
    ) -> ::windows_core::Result<LinearGradientBrush>
    where
        P0: ::windows_core::IntoParam<GradientStopCollection>,
    {
        Self::ILinearGradientBrushFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this)
                .CreateInstanceWithGradientStopCollectionAndAngle)(
                ::windows_core::Interface::as_raw(this),
                gradientstopcollection.into_param().abi(),
                angle,
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn StartPointProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::ILinearGradientBrushStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StartPointProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn EndPointProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::ILinearGradientBrushStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).EndPointProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ILinearGradientBrushFactory<
        R,
        F: FnOnce(&ILinearGradientBrushFactory) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<
            LinearGradientBrush,
            ILinearGradientBrushFactory,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ILinearGradientBrushStatics<
        R,
        F: FnOnce(&ILinearGradientBrushStatics) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<
            LinearGradientBrush,
            ILinearGradientBrushStatics,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for LinearGradientBrush {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for LinearGradientBrush {
    type Vtable = ILinearGradientBrush_Vtbl;
}
unsafe impl ::windows_core::ComInterface for LinearGradientBrush {
    const IID: ::windows_core::GUID = <ILinearGradientBrush as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for LinearGradientBrush {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.LinearGradientBrush";
}
::windows_core::imp::interface_hierarchy!(
    LinearGradientBrush,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
#[cfg(feature = "Microsoft_UI_Composition")]
impl ::windows_core::CanTryInto<super::super::Composition::IAnimationObject>
    for LinearGradientBrush
{
}
impl ::windows_core::CanTryInto<GradientBrush> for LinearGradientBrush {}
impl ::windows_core::CanTryInto<Brush> for LinearGradientBrush {}
impl ::windows_core::CanTryInto<super::DependencyObject> for LinearGradientBrush {}
unsafe impl ::core::marker::Send for LinearGradientBrush {}
unsafe impl ::core::marker::Sync for LinearGradientBrush {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct LoadedImageSourceLoadCompletedEventArgs(::windows_core::IUnknown);
impl LoadedImageSourceLoadCompletedEventArgs {
    pub fn Status(&self) -> ::windows_core::Result<LoadedImageSourceLoadStatus> {
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
impl ::windows_core::RuntimeType for LoadedImageSourceLoadCompletedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for LoadedImageSourceLoadCompletedEventArgs {
    type Vtable = ILoadedImageSourceLoadCompletedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for LoadedImageSourceLoadCompletedEventArgs {
    const IID: ::windows_core::GUID =
        <ILoadedImageSourceLoadCompletedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for LoadedImageSourceLoadCompletedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.LoadedImageSourceLoadCompletedEventArgs";
}
::windows_core::imp::interface_hierarchy!(
    LoadedImageSourceLoadCompletedEventArgs,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for LoadedImageSourceLoadCompletedEventArgs {}
unsafe impl ::core::marker::Sync for LoadedImageSourceLoadCompletedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct LoadedImageSurface(::windows_core::IUnknown);
impl LoadedImageSurface {
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<::windows::Foundation::IClosable>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this))
                .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn DecodedPhysicalSize(&self) -> ::windows_core::Result<::windows::Foundation::Size> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DecodedPhysicalSize)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn DecodedSize(&self) -> ::windows_core::Result<::windows::Foundation::Size> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DecodedSize)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn NaturalSize(&self) -> ::windows_core::Result<::windows::Foundation::Size> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NaturalSize)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn LoadCompleted<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<
                LoadedImageSurface,
                LoadedImageSourceLoadCompletedEventArgs,
            >,
        >,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LoadCompleted)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveLoadCompleted(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveLoadCompleted)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn StartLoadFromUriWithSize<P0>(
        uri: P0,
        desiredmaxsize: ::windows::Foundation::Size,
    ) -> ::windows_core::Result<LoadedImageSurface>
    where
        P0: ::windows_core::IntoParam<::windows::Foundation::Uri>,
    {
        Self::ILoadedImageSurfaceStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StartLoadFromUriWithSize)(
                ::windows_core::Interface::as_raw(this),
                uri.into_param().abi(),
                desiredmaxsize,
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn StartLoadFromUri<P0>(uri: P0) -> ::windows_core::Result<LoadedImageSurface>
    where
        P0: ::windows_core::IntoParam<::windows::Foundation::Uri>,
    {
        Self::ILoadedImageSurfaceStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StartLoadFromUri)(
                ::windows_core::Interface::as_raw(this),
                uri.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_Foundation\"`, `\"Windows_Storage_Streams\"`"]
    #[cfg(all(feature = "Windows_Foundation", feature = "Windows_Storage_Streams"))]
    pub fn StartLoadFromStreamWithSize<P0>(
        stream: P0,
        desiredmaxsize: ::windows::Foundation::Size,
    ) -> ::windows_core::Result<LoadedImageSurface>
    where
        P0: ::windows_core::TryIntoParam<::windows::Storage::Streams::IRandomAccessStream>,
    {
        Self::ILoadedImageSurfaceStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StartLoadFromStreamWithSize)(
                ::windows_core::Interface::as_raw(this),
                stream.try_into_param()?.abi(),
                desiredmaxsize,
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_Storage_Streams\"`"]
    #[cfg(feature = "Windows_Storage_Streams")]
    pub fn StartLoadFromStream<P0>(stream: P0) -> ::windows_core::Result<LoadedImageSurface>
    where
        P0: ::windows_core::TryIntoParam<::windows::Storage::Streams::IRandomAccessStream>,
    {
        Self::ILoadedImageSurfaceStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StartLoadFromStream)(
                ::windows_core::Interface::as_raw(this),
                stream.try_into_param()?.abi(),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ILoadedImageSurfaceStatics<
        R,
        F: FnOnce(&ILoadedImageSurfaceStatics) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<
            LoadedImageSurface,
            ILoadedImageSurfaceStatics,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for LoadedImageSurface {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for LoadedImageSurface {
    type Vtable = ILoadedImageSurface_Vtbl;
}
unsafe impl ::windows_core::ComInterface for LoadedImageSurface {
    const IID: ::windows_core::GUID = <ILoadedImageSurface as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for LoadedImageSurface {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.LoadedImageSurface";
}
::windows_core::imp::interface_hierarchy!(
    LoadedImageSurface,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
#[cfg(feature = "Windows_Foundation")]
impl ::windows_core::CanTryInto<::windows::Foundation::IClosable> for LoadedImageSurface {}
#[cfg(feature = "Microsoft_UI_Composition")]
impl ::windows_core::CanTryInto<super::super::Composition::ICompositionSurface>
    for LoadedImageSurface
{
}
unsafe impl ::core::marker::Send for LoadedImageSurface {}
unsafe impl ::core::marker::Sync for LoadedImageSurface {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct Matrix3DProjection(::windows_core::IUnknown);
impl Matrix3DProjection {
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
            Matrix3DProjection,
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
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Media_Media3D\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Media_Media3D")]
    pub fn ProjectionMatrix(&self) -> ::windows_core::Result<Media3D::Matrix3D> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ProjectionMatrix)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Media_Media3D\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Media_Media3D")]
    pub fn SetProjectionMatrix(&self, value: Media3D::Matrix3D) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetProjectionMatrix)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn ProjectionMatrixProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IMatrix3DProjectionStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ProjectionMatrixProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IMatrix3DProjectionStatics<
        R,
        F: FnOnce(&IMatrix3DProjectionStatics) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<
            Matrix3DProjection,
            IMatrix3DProjectionStatics,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for Matrix3DProjection {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for Matrix3DProjection {
    type Vtable = IMatrix3DProjection_Vtbl;
}
unsafe impl ::windows_core::ComInterface for Matrix3DProjection {
    const IID: ::windows_core::GUID = <IMatrix3DProjection as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for Matrix3DProjection {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.Matrix3DProjection";
}
::windows_core::imp::interface_hierarchy!(
    Matrix3DProjection,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<Projection> for Matrix3DProjection {}
impl ::windows_core::CanTryInto<super::DependencyObject> for Matrix3DProjection {}
unsafe impl ::core::marker::Send for Matrix3DProjection {}
unsafe impl ::core::marker::Sync for Matrix3DProjection {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct MatrixHelper(::windows_core::IUnknown);
impl MatrixHelper {
    pub fn Identity() -> ::windows_core::Result<Matrix> {
        Self::IMatrixHelperStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Identity)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn FromElements(
        m11: f64,
        m12: f64,
        m21: f64,
        m22: f64,
        offsetx: f64,
        offsety: f64,
    ) -> ::windows_core::Result<Matrix> {
        Self::IMatrixHelperStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FromElements)(
                ::windows_core::Interface::as_raw(this),
                m11,
                m12,
                m21,
                m22,
                offsetx,
                offsety,
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn GetIsIdentity(target: Matrix) -> ::windows_core::Result<bool> {
        Self::IMatrixHelperStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetIsIdentity)(
                ::windows_core::Interface::as_raw(this),
                target,
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn Transform(
        target: Matrix,
        point: ::windows::Foundation::Point,
    ) -> ::windows_core::Result<::windows::Foundation::Point> {
        Self::IMatrixHelperStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Transform)(
                ::windows_core::Interface::as_raw(this),
                target,
                point,
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IMatrixHelperStatics<
        R,
        F: FnOnce(&IMatrixHelperStatics) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<MatrixHelper, IMatrixHelperStatics> =
            ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for MatrixHelper {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for MatrixHelper {
    type Vtable = IMatrixHelper_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MatrixHelper {
    const IID: ::windows_core::GUID = <IMatrixHelper as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MatrixHelper {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.MatrixHelper";
}
::windows_core::imp::interface_hierarchy!(
    MatrixHelper,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for MatrixHelper {}
unsafe impl ::core::marker::Sync for MatrixHelper {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct MatrixTransform(::windows_core::IUnknown);
impl MatrixTransform {
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
            MatrixTransform,
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
    pub fn Inverse(&self) -> ::windows_core::Result<GeneralTransform> {
        let this = &::windows_core::ComInterface::cast::<IGeneralTransform>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Inverse)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn TransformPoint(
        &self,
        point: ::windows::Foundation::Point,
    ) -> ::windows_core::Result<::windows::Foundation::Point> {
        let this = &::windows_core::ComInterface::cast::<IGeneralTransform>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TransformPoint)(
                ::windows_core::Interface::as_raw(this),
                point,
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn TryTransform(
        &self,
        inpoint: ::windows::Foundation::Point,
        outpoint: &mut ::windows::Foundation::Point,
    ) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IGeneralTransform>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TryTransform)(
                ::windows_core::Interface::as_raw(this),
                inpoint,
                outpoint,
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn TransformBounds(
        &self,
        rect: ::windows::Foundation::Rect,
    ) -> ::windows_core::Result<::windows::Foundation::Rect> {
        let this = &::windows_core::ComInterface::cast::<IGeneralTransform>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TransformBounds)(
                ::windows_core::Interface::as_raw(this),
                rect,
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn InverseCore(&self) -> ::windows_core::Result<GeneralTransform> {
        let this = &::windows_core::ComInterface::cast::<IGeneralTransformOverrides>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).InverseCore)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn TryTransformCore(
        &self,
        inpoint: ::windows::Foundation::Point,
        outpoint: &mut ::windows::Foundation::Point,
    ) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IGeneralTransformOverrides>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TryTransformCore)(
                ::windows_core::Interface::as_raw(this),
                inpoint,
                outpoint,
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn TransformBoundsCore(
        &self,
        rect: ::windows::Foundation::Rect,
    ) -> ::windows_core::Result<::windows::Foundation::Rect> {
        let this = &::windows_core::ComInterface::cast::<IGeneralTransformOverrides>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TransformBoundsCore)(
                ::windows_core::Interface::as_raw(this),
                rect,
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn Matrix(&self) -> ::windows_core::Result<Matrix> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Matrix)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetMatrix(&self, value: Matrix) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetMatrix)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn MatrixProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IMatrixTransformStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MatrixProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IMatrixTransformStatics<
        R,
        F: FnOnce(&IMatrixTransformStatics) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<MatrixTransform, IMatrixTransformStatics> =
            ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for MatrixTransform {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for MatrixTransform {
    type Vtable = IMatrixTransform_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MatrixTransform {
    const IID: ::windows_core::GUID = <IMatrixTransform as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MatrixTransform {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.MatrixTransform";
}
::windows_core::imp::interface_hierarchy!(
    MatrixTransform,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<Transform> for MatrixTransform {}
impl ::windows_core::CanTryInto<GeneralTransform> for MatrixTransform {}
impl ::windows_core::CanTryInto<super::DependencyObject> for MatrixTransform {}
unsafe impl ::core::marker::Send for MatrixTransform {}
unsafe impl ::core::marker::Sync for MatrixTransform {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct MediaTransportControlsThumbnailRequestedEventArgs(::windows_core::IUnknown);
impl MediaTransportControlsThumbnailRequestedEventArgs {
    #[doc = "Required features: `\"Windows_Storage_Streams\"`"]
    #[cfg(feature = "Windows_Storage_Streams")]
    pub fn SetThumbnailImage<P0>(&self, source: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<::windows::Storage::Streams::IInputStream>,
    {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetThumbnailImage)(
                ::windows_core::Interface::as_raw(this),
                source.try_into_param()?.abi(),
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn GetDeferral(&self) -> ::windows_core::Result<::windows::Foundation::Deferral> {
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
}
impl ::windows_core::RuntimeType for MediaTransportControlsThumbnailRequestedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for MediaTransportControlsThumbnailRequestedEventArgs {
    type Vtable = IMediaTransportControlsThumbnailRequestedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MediaTransportControlsThumbnailRequestedEventArgs {
    const IID: ::windows_core::GUID =
        <IMediaTransportControlsThumbnailRequestedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MediaTransportControlsThumbnailRequestedEventArgs {
    const NAME: &'static str =
        "Microsoft.UI.Xaml.Media.MediaTransportControlsThumbnailRequestedEventArgs";
}
::windows_core::imp::interface_hierarchy!(
    MediaTransportControlsThumbnailRequestedEventArgs,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for MediaTransportControlsThumbnailRequestedEventArgs {}
unsafe impl ::core::marker::Sync for MediaTransportControlsThumbnailRequestedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct MicaBackdrop(::windows_core::IUnknown);
impl MicaBackdrop {
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
    #[doc = "Required features: `\"Microsoft_UI_Composition_SystemBackdrops\"`"]
    #[cfg(feature = "Microsoft_UI_Composition_SystemBackdrops")]
    pub fn Kind(
        &self,
    ) -> ::windows_core::Result<super::super::Composition::SystemBackdrops::MicaKind> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Composition_SystemBackdrops\"`"]
    #[cfg(feature = "Microsoft_UI_Composition_SystemBackdrops")]
    pub fn SetKind(
        &self,
        value: super::super::Composition::SystemBackdrops::MicaKind,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetKind)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub unsafe fn CreateInstance<P0>(
        baseinterface: P0,
        innerinterface: &mut ::core::option::Option<::windows_core::IInspectable>,
    ) -> ::windows_core::Result<MicaBackdrop>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        Self::IMicaBackdropFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstance)(
                ::windows_core::Interface::as_raw(this),
                baseinterface.into_param().abi(),
                innerinterface as *mut _ as _,
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn KindProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IMicaBackdropStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).KindProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Microsoft_UI_Composition_SystemBackdrops\"`"]
    #[cfg(feature = "Microsoft_UI_Composition_SystemBackdrops")]
    pub fn GetDefaultSystemBackdropConfiguration<P0, P1>(
        &self,
        target: P0,
        xamlroot: P1,
    ) -> ::windows_core::Result<
        super::super::Composition::SystemBackdrops::SystemBackdropConfiguration,
    >
    where
        P0: ::windows_core::TryIntoParam<
            super::super::Composition::ICompositionSupportsSystemBackdrop,
        >,
        P1: ::windows_core::IntoParam<super::XamlRoot>,
    {
        let this = &::windows_core::ComInterface::cast::<ISystemBackdrop>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetDefaultSystemBackdropConfiguration)(
                ::windows_core::Interface::as_raw(this),
                target.try_into_param()?.abi(),
                xamlroot.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Composition\"`"]
    #[cfg(feature = "Microsoft_UI_Composition")]
    pub fn OnTargetConnected<P0, P1>(
        &self,
        connectedtarget: P0,
        xamlroot: P1,
    ) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<
            super::super::Composition::ICompositionSupportsSystemBackdrop,
        >,
        P1: ::windows_core::IntoParam<super::XamlRoot>,
    {
        let this = &::windows_core::ComInterface::cast::<ISystemBackdropOverrides>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).OnTargetConnected)(
                ::windows_core::Interface::as_raw(this),
                connectedtarget.try_into_param()?.abi(),
                xamlroot.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Composition\"`"]
    #[cfg(feature = "Microsoft_UI_Composition")]
    pub fn OnTargetDisconnected<P0>(&self, disconnectedtarget: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<
            super::super::Composition::ICompositionSupportsSystemBackdrop,
        >,
    {
        let this = &::windows_core::ComInterface::cast::<ISystemBackdropOverrides>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).OnTargetDisconnected)(
                ::windows_core::Interface::as_raw(this),
                disconnectedtarget.try_into_param()?.abi(),
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Composition\"`"]
    #[cfg(feature = "Microsoft_UI_Composition")]
    pub fn OnDefaultSystemBackdropConfigurationChanged<P0, P1>(
        &self,
        target: P0,
        xamlroot: P1,
    ) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<
            super::super::Composition::ICompositionSupportsSystemBackdrop,
        >,
        P1: ::windows_core::IntoParam<super::XamlRoot>,
    {
        let this = &::windows_core::ComInterface::cast::<ISystemBackdropOverrides>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).OnDefaultSystemBackdropConfigurationChanged)(
                ::windows_core::Interface::as_raw(this),
                target.try_into_param()?.abi(),
                xamlroot.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc(hidden)]
    pub fn IMicaBackdropFactory<
        R,
        F: FnOnce(&IMicaBackdropFactory) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<MicaBackdrop, IMicaBackdropFactory> =
            ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IMicaBackdropStatics<
        R,
        F: FnOnce(&IMicaBackdropStatics) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<MicaBackdrop, IMicaBackdropStatics> =
            ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for MicaBackdrop {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for MicaBackdrop {
    type Vtable = IMicaBackdrop_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MicaBackdrop {
    const IID: ::windows_core::GUID = <IMicaBackdrop as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MicaBackdrop {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.MicaBackdrop";
}
::windows_core::imp::interface_hierarchy!(
    MicaBackdrop,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<SystemBackdrop> for MicaBackdrop {}
impl ::windows_core::CanTryInto<super::DependencyObject> for MicaBackdrop {}
unsafe impl ::core::marker::Send for MicaBackdrop {}
unsafe impl ::core::marker::Sync for MicaBackdrop {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct PathFigure(::windows_core::IUnknown);
impl PathFigure {
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
            PathFigure,
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
    pub fn Segments(&self) -> ::windows_core::Result<PathSegmentCollection> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Segments)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation_Collections\"`"]
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub fn SetSegments<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<PathSegmentCollection>,
    {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetSegments)(
                ::windows_core::Interface::as_raw(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn StartPoint(&self) -> ::windows_core::Result<::windows::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StartPoint)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn SetStartPoint(&self, value: ::windows::Foundation::Point) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetStartPoint)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsClosed(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsClosed)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetIsClosed(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetIsClosed)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsFilled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsFilled)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetIsFilled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetIsFilled)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn SegmentsProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IPathFigureStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SegmentsProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn StartPointProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IPathFigureStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StartPointProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn IsClosedProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IPathFigureStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsClosedProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn IsFilledProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IPathFigureStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsFilledProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPathFigureStatics<R, F: FnOnce(&IPathFigureStatics) -> ::windows_core::Result<R>>(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<PathFigure, IPathFigureStatics> =
            ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for PathFigure {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for PathFigure {
    type Vtable = IPathFigure_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PathFigure {
    const IID: ::windows_core::GUID = <IPathFigure as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PathFigure {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.PathFigure";
}
::windows_core::imp::interface_hierarchy!(
    PathFigure,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<super::DependencyObject> for PathFigure {}
unsafe impl ::core::marker::Send for PathFigure {}
unsafe impl ::core::marker::Sync for PathFigure {}
#[doc = "Required features: `\"Windows_Foundation_Collections\"`"]
#[cfg(feature = "Windows_Foundation_Collections")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct PathFigureCollection(::windows_core::IUnknown);
#[cfg(feature = "Windows_Foundation_Collections")]
impl PathFigureCollection {
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
            PathFigureCollection,
            ::windows_core::imp::IGenericFactory,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "Required features: `\"Windows_Foundation_Collections\"`"]
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub fn First(
        &self,
    ) -> ::windows_core::Result<::windows::Foundation::Collections::IIterator<PathFigure>> {
        let this = &::windows_core::ComInterface::cast::<
            ::windows::Foundation::Collections::IIterable<PathFigure>,
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
    pub fn GetAt(&self, index: u32) -> ::windows_core::Result<PathFigure> {
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
    ) -> ::windows_core::Result<::windows::Foundation::Collections::IVectorView<PathFigure>> {
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
        P0: ::windows_core::IntoParam<PathFigure>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IndexOf)(
                ::windows_core::Interface::as_raw(this),
                value.into_param().abi(),
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
        P0: ::windows_core::IntoParam<PathFigure>,
    {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetAt)(
                ::windows_core::Interface::as_raw(this),
                index,
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation_Collections\"`"]
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub fn InsertAt<P0>(&self, index: u32, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<PathFigure>,
    {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).InsertAt)(
                ::windows_core::Interface::as_raw(this),
                index,
                value.into_param().abi(),
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
        P0: ::windows_core::IntoParam<PathFigure>,
    {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).Append)(
                ::windows_core::Interface::as_raw(this),
                value.into_param().abi(),
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
        items: &mut [::core::option::Option<PathFigure>],
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
        items: &[::core::option::Option<PathFigure>],
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
impl ::windows_core::RuntimeType for PathFigureCollection {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
#[cfg(feature = "Windows_Foundation_Collections")]
unsafe impl ::windows_core::Interface for PathFigureCollection {
    type Vtable = ::windows::Foundation::Collections::IVector_Vtbl<PathFigure>;
}
#[cfg(feature = "Windows_Foundation_Collections")]
unsafe impl ::windows_core::ComInterface for PathFigureCollection {
    const IID : ::windows_core::GUID = < ::windows::Foundation::Collections:: IVector :: < PathFigure > as::windows_core::ComInterface >::IID ;
}
#[cfg(feature = "Windows_Foundation_Collections")]
impl ::windows_core::RuntimeName for PathFigureCollection {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.PathFigureCollection";
}
#[cfg(feature = "Windows_Foundation_Collections")]
impl ::core::iter::IntoIterator for PathFigureCollection {
    type Item = PathFigure;
    type IntoIter = ::windows::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Windows_Foundation_Collections")]
impl ::core::iter::IntoIterator for &PathFigureCollection {
    type Item = PathFigure;
    type IntoIter = ::windows::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::windows::Foundation::Collections::VectorIterator::new(
            ::windows_core::ComInterface::cast(self).ok(),
        )
    }
}
#[cfg(feature = "Windows_Foundation_Collections")]
::windows_core::imp::interface_hierarchy!(
    PathFigureCollection,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
#[cfg(feature = "Windows_Foundation_Collections")]
impl ::windows_core::CanTryInto<::windows::Foundation::Collections::IIterable<PathFigure>>
    for PathFigureCollection
{
}
#[cfg(feature = "Windows_Foundation_Collections")]
impl ::windows_core::CanTryInto<::windows::Foundation::Collections::IVector<PathFigure>>
    for PathFigureCollection
{
}
#[cfg(feature = "Windows_Foundation_Collections")]
unsafe impl ::core::marker::Send for PathFigureCollection {}
#[cfg(feature = "Windows_Foundation_Collections")]
unsafe impl ::core::marker::Sync for PathFigureCollection {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct PathGeometry(::windows_core::IUnknown);
impl PathGeometry {
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
            PathGeometry,
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
    pub fn Transform(&self) -> ::windows_core::Result<Transform> {
        let this = &::windows_core::ComInterface::cast::<IGeometry>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Transform)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetTransform<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<Transform>,
    {
        let this = &::windows_core::ComInterface::cast::<IGeometry>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetTransform)(
                ::windows_core::Interface::as_raw(this),
                value.try_into_param()?.abi(),
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn Bounds(&self) -> ::windows_core::Result<::windows::Foundation::Rect> {
        let this = &::windows_core::ComInterface::cast::<IGeometry>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Bounds)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn FillRule(&self) -> ::windows_core::Result<FillRule> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FillRule)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetFillRule(&self, value: FillRule) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetFillRule)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation_Collections\"`"]
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub fn Figures(&self) -> ::windows_core::Result<PathFigureCollection> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Figures)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation_Collections\"`"]
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub fn SetFigures<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<PathFigureCollection>,
    {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetFigures)(
                ::windows_core::Interface::as_raw(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn FillRuleProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IPathGeometryStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FillRuleProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn FiguresProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IPathGeometryStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FiguresProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPathGeometryStatics<
        R,
        F: FnOnce(&IPathGeometryStatics) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<PathGeometry, IPathGeometryStatics> =
            ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for PathGeometry {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for PathGeometry {
    type Vtable = IPathGeometry_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PathGeometry {
    const IID: ::windows_core::GUID = <IPathGeometry as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PathGeometry {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.PathGeometry";
}
::windows_core::imp::interface_hierarchy!(
    PathGeometry,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<Geometry> for PathGeometry {}
impl ::windows_core::CanTryInto<super::DependencyObject> for PathGeometry {}
unsafe impl ::core::marker::Send for PathGeometry {}
unsafe impl ::core::marker::Sync for PathGeometry {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct PathSegment(::windows_core::IUnknown);
impl PathSegment {
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
impl ::windows_core::RuntimeType for PathSegment {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for PathSegment {
    type Vtable = IPathSegment_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PathSegment {
    const IID: ::windows_core::GUID = <IPathSegment as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PathSegment {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.PathSegment";
}
::windows_core::imp::interface_hierarchy!(
    PathSegment,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<super::DependencyObject> for PathSegment {}
unsafe impl ::core::marker::Send for PathSegment {}
unsafe impl ::core::marker::Sync for PathSegment {}
#[doc = "Required features: `\"Windows_Foundation_Collections\"`"]
#[cfg(feature = "Windows_Foundation_Collections")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct PathSegmentCollection(::windows_core::IUnknown);
#[cfg(feature = "Windows_Foundation_Collections")]
impl PathSegmentCollection {
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
            PathSegmentCollection,
            ::windows_core::imp::IGenericFactory,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "Required features: `\"Windows_Foundation_Collections\"`"]
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub fn First(
        &self,
    ) -> ::windows_core::Result<::windows::Foundation::Collections::IIterator<PathSegment>> {
        let this = &::windows_core::ComInterface::cast::<
            ::windows::Foundation::Collections::IIterable<PathSegment>,
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
    pub fn GetAt(&self, index: u32) -> ::windows_core::Result<PathSegment> {
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
    ) -> ::windows_core::Result<::windows::Foundation::Collections::IVectorView<PathSegment>> {
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
        P0: ::windows_core::TryIntoParam<PathSegment>,
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
        P0: ::windows_core::TryIntoParam<PathSegment>,
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
        P0: ::windows_core::TryIntoParam<PathSegment>,
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
        P0: ::windows_core::TryIntoParam<PathSegment>,
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
        items: &mut [::core::option::Option<PathSegment>],
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
        items: &[::core::option::Option<PathSegment>],
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
impl ::windows_core::RuntimeType for PathSegmentCollection {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
#[cfg(feature = "Windows_Foundation_Collections")]
unsafe impl ::windows_core::Interface for PathSegmentCollection {
    type Vtable = ::windows::Foundation::Collections::IVector_Vtbl<PathSegment>;
}
#[cfg(feature = "Windows_Foundation_Collections")]
unsafe impl ::windows_core::ComInterface for PathSegmentCollection {
    const IID : ::windows_core::GUID = < ::windows::Foundation::Collections:: IVector :: < PathSegment > as::windows_core::ComInterface >::IID ;
}
#[cfg(feature = "Windows_Foundation_Collections")]
impl ::windows_core::RuntimeName for PathSegmentCollection {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.PathSegmentCollection";
}
#[cfg(feature = "Windows_Foundation_Collections")]
impl ::core::iter::IntoIterator for PathSegmentCollection {
    type Item = PathSegment;
    type IntoIter = ::windows::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Windows_Foundation_Collections")]
impl ::core::iter::IntoIterator for &PathSegmentCollection {
    type Item = PathSegment;
    type IntoIter = ::windows::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::windows::Foundation::Collections::VectorIterator::new(
            ::windows_core::ComInterface::cast(self).ok(),
        )
    }
}
#[cfg(feature = "Windows_Foundation_Collections")]
::windows_core::imp::interface_hierarchy!(
    PathSegmentCollection,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
#[cfg(feature = "Windows_Foundation_Collections")]
impl ::windows_core::CanTryInto<::windows::Foundation::Collections::IIterable<PathSegment>>
    for PathSegmentCollection
{
}
#[cfg(feature = "Windows_Foundation_Collections")]
impl ::windows_core::CanTryInto<::windows::Foundation::Collections::IVector<PathSegment>>
    for PathSegmentCollection
{
}
#[cfg(feature = "Windows_Foundation_Collections")]
unsafe impl ::core::marker::Send for PathSegmentCollection {}
#[cfg(feature = "Windows_Foundation_Collections")]
unsafe impl ::core::marker::Sync for PathSegmentCollection {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct PlaneProjection(::windows_core::IUnknown);
impl PlaneProjection {
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
            PlaneProjection,
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
    pub fn LocalOffsetX(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LocalOffsetX)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetLocalOffsetX(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetLocalOffsetX)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn LocalOffsetY(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LocalOffsetY)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetLocalOffsetY(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetLocalOffsetY)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn LocalOffsetZ(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LocalOffsetZ)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetLocalOffsetZ(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetLocalOffsetZ)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn RotationX(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RotationX)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetRotationX(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetRotationX)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn RotationY(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RotationY)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetRotationY(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetRotationY)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn RotationZ(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RotationZ)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetRotationZ(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetRotationZ)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn CenterOfRotationX(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CenterOfRotationX)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetCenterOfRotationX(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetCenterOfRotationX)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn CenterOfRotationY(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CenterOfRotationY)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetCenterOfRotationY(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetCenterOfRotationY)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn CenterOfRotationZ(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CenterOfRotationZ)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetCenterOfRotationZ(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetCenterOfRotationZ)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn GlobalOffsetX(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GlobalOffsetX)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetGlobalOffsetX(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetGlobalOffsetX)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn GlobalOffsetY(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GlobalOffsetY)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetGlobalOffsetY(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetGlobalOffsetY)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn GlobalOffsetZ(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GlobalOffsetZ)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetGlobalOffsetZ(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetGlobalOffsetZ)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Media_Media3D\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Media_Media3D")]
    pub fn ProjectionMatrix(&self) -> ::windows_core::Result<Media3D::Matrix3D> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ProjectionMatrix)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn LocalOffsetXProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IPlaneProjectionStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LocalOffsetXProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn LocalOffsetYProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IPlaneProjectionStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LocalOffsetYProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn LocalOffsetZProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IPlaneProjectionStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LocalOffsetZProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn RotationXProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IPlaneProjectionStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RotationXProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn RotationYProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IPlaneProjectionStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RotationYProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn RotationZProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IPlaneProjectionStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RotationZProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn CenterOfRotationXProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IPlaneProjectionStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CenterOfRotationXProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn CenterOfRotationYProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IPlaneProjectionStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CenterOfRotationYProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn CenterOfRotationZProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IPlaneProjectionStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CenterOfRotationZProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn GlobalOffsetXProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IPlaneProjectionStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GlobalOffsetXProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn GlobalOffsetYProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IPlaneProjectionStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GlobalOffsetYProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn GlobalOffsetZProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IPlaneProjectionStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GlobalOffsetZProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn ProjectionMatrixProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IPlaneProjectionStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ProjectionMatrixProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPlaneProjectionStatics<
        R,
        F: FnOnce(&IPlaneProjectionStatics) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<PlaneProjection, IPlaneProjectionStatics> =
            ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for PlaneProjection {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for PlaneProjection {
    type Vtable = IPlaneProjection_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PlaneProjection {
    const IID: ::windows_core::GUID = <IPlaneProjection as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PlaneProjection {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.PlaneProjection";
}
::windows_core::imp::interface_hierarchy!(
    PlaneProjection,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<Projection> for PlaneProjection {}
impl ::windows_core::CanTryInto<super::DependencyObject> for PlaneProjection {}
unsafe impl ::core::marker::Send for PlaneProjection {}
unsafe impl ::core::marker::Sync for PlaneProjection {}
#[doc = "Required features: `\"Windows_Foundation_Collections\"`"]
#[cfg(feature = "Windows_Foundation_Collections")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct PointCollection(::windows_core::IUnknown);
#[cfg(feature = "Windows_Foundation_Collections")]
impl PointCollection {
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
            PointCollection,
            ::windows_core::imp::IGenericFactory,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "Required features: `\"Windows_Foundation_Collections\"`"]
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub fn First(
        &self,
    ) -> ::windows_core::Result<
        ::windows::Foundation::Collections::IIterator<::windows::Foundation::Point>,
    > {
        let this = &::windows_core::ComInterface::cast::<
            ::windows::Foundation::Collections::IIterable<::windows::Foundation::Point>,
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
    pub fn GetAt(&self, index: u32) -> ::windows_core::Result<::windows::Foundation::Point> {
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
    ) -> ::windows_core::Result<
        ::windows::Foundation::Collections::IVectorView<::windows::Foundation::Point>,
    > {
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
    pub fn IndexOf(
        &self,
        value: ::windows::Foundation::Point,
        index: &mut u32,
    ) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IndexOf)(
                ::windows_core::Interface::as_raw(this),
                value,
                index,
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation_Collections\"`"]
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub fn SetAt(
        &self,
        index: u32,
        value: ::windows::Foundation::Point,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetAt)(
                ::windows_core::Interface::as_raw(this),
                index,
                value,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation_Collections\"`"]
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub fn InsertAt(
        &self,
        index: u32,
        value: ::windows::Foundation::Point,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).InsertAt)(
                ::windows_core::Interface::as_raw(this),
                index,
                value,
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
    pub fn Append(&self, value: ::windows::Foundation::Point) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).Append)(
                ::windows_core::Interface::as_raw(this),
                value,
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
        items: &mut [::windows::Foundation::Point],
    ) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetMany)(
                ::windows_core::Interface::as_raw(this),
                startindex,
                items.len().try_into().unwrap(),
                items.as_mut_ptr(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation_Collections\"`"]
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub fn ReplaceAll(&self, items: &[::windows::Foundation::Point]) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).ReplaceAll)(
                ::windows_core::Interface::as_raw(this),
                items.len().try_into().unwrap(),
                items.as_ptr(),
            )
            .ok()
        }
    }
}
#[cfg(feature = "Windows_Foundation_Collections")]
impl ::windows_core::RuntimeType for PointCollection {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
#[cfg(feature = "Windows_Foundation_Collections")]
unsafe impl ::windows_core::Interface for PointCollection {
    type Vtable = ::windows::Foundation::Collections::IVector_Vtbl<::windows::Foundation::Point>;
}
#[cfg(feature = "Windows_Foundation_Collections")]
unsafe impl ::windows_core::ComInterface for PointCollection {
    const IID: ::windows_core::GUID = <::windows::Foundation::Collections::IVector<
        ::windows::Foundation::Point,
    > as ::windows_core::ComInterface>::IID;
}
#[cfg(feature = "Windows_Foundation_Collections")]
impl ::windows_core::RuntimeName for PointCollection {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.PointCollection";
}
#[cfg(feature = "Windows_Foundation_Collections")]
impl ::core::iter::IntoIterator for PointCollection {
    type Item = ::windows::Foundation::Point;
    type IntoIter = ::windows::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Windows_Foundation_Collections")]
impl ::core::iter::IntoIterator for &PointCollection {
    type Item = ::windows::Foundation::Point;
    type IntoIter = ::windows::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::windows::Foundation::Collections::VectorIterator::new(
            ::windows_core::ComInterface::cast(self).ok(),
        )
    }
}
#[cfg(feature = "Windows_Foundation_Collections")]
::windows_core::imp::interface_hierarchy!(
    PointCollection,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
#[cfg(feature = "Windows_Foundation_Collections")]
impl
    ::windows_core::CanTryInto<
        ::windows::Foundation::Collections::IIterable<::windows::Foundation::Point>,
    > for PointCollection
{
}
#[cfg(feature = "Windows_Foundation_Collections")]
impl
    ::windows_core::CanTryInto<
        ::windows::Foundation::Collections::IVector<::windows::Foundation::Point>,
    > for PointCollection
{
}
#[cfg(feature = "Windows_Foundation_Collections")]
unsafe impl ::core::marker::Send for PointCollection {}
#[cfg(feature = "Windows_Foundation_Collections")]
unsafe impl ::core::marker::Sync for PointCollection {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct PolyBezierSegment(::windows_core::IUnknown);
impl PolyBezierSegment {
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
            PolyBezierSegment,
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
    pub fn Points(&self) -> ::windows_core::Result<PointCollection> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Points)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation_Collections\"`"]
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub fn SetPoints<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<PointCollection>,
    {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetPoints)(
                ::windows_core::Interface::as_raw(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn PointsProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IPolyBezierSegmentStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PointsProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPolyBezierSegmentStatics<
        R,
        F: FnOnce(&IPolyBezierSegmentStatics) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<
            PolyBezierSegment,
            IPolyBezierSegmentStatics,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for PolyBezierSegment {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for PolyBezierSegment {
    type Vtable = IPolyBezierSegment_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PolyBezierSegment {
    const IID: ::windows_core::GUID = <IPolyBezierSegment as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PolyBezierSegment {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.PolyBezierSegment";
}
::windows_core::imp::interface_hierarchy!(
    PolyBezierSegment,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<PathSegment> for PolyBezierSegment {}
impl ::windows_core::CanTryInto<super::DependencyObject> for PolyBezierSegment {}
unsafe impl ::core::marker::Send for PolyBezierSegment {}
unsafe impl ::core::marker::Sync for PolyBezierSegment {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct PolyLineSegment(::windows_core::IUnknown);
impl PolyLineSegment {
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
            PolyLineSegment,
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
    pub fn Points(&self) -> ::windows_core::Result<PointCollection> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Points)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation_Collections\"`"]
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub fn SetPoints<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<PointCollection>,
    {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetPoints)(
                ::windows_core::Interface::as_raw(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn PointsProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IPolyLineSegmentStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PointsProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPolyLineSegmentStatics<
        R,
        F: FnOnce(&IPolyLineSegmentStatics) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<PolyLineSegment, IPolyLineSegmentStatics> =
            ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for PolyLineSegment {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for PolyLineSegment {
    type Vtable = IPolyLineSegment_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PolyLineSegment {
    const IID: ::windows_core::GUID = <IPolyLineSegment as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PolyLineSegment {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.PolyLineSegment";
}
::windows_core::imp::interface_hierarchy!(
    PolyLineSegment,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<PathSegment> for PolyLineSegment {}
impl ::windows_core::CanTryInto<super::DependencyObject> for PolyLineSegment {}
unsafe impl ::core::marker::Send for PolyLineSegment {}
unsafe impl ::core::marker::Sync for PolyLineSegment {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct PolyQuadraticBezierSegment(::windows_core::IUnknown);
impl PolyQuadraticBezierSegment {
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
            PolyQuadraticBezierSegment,
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
    pub fn Points(&self) -> ::windows_core::Result<PointCollection> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Points)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation_Collections\"`"]
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub fn SetPoints<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<PointCollection>,
    {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetPoints)(
                ::windows_core::Interface::as_raw(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn PointsProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IPolyQuadraticBezierSegmentStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PointsProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPolyQuadraticBezierSegmentStatics<
        R,
        F: FnOnce(&IPolyQuadraticBezierSegmentStatics) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<
            PolyQuadraticBezierSegment,
            IPolyQuadraticBezierSegmentStatics,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for PolyQuadraticBezierSegment {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for PolyQuadraticBezierSegment {
    type Vtable = IPolyQuadraticBezierSegment_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PolyQuadraticBezierSegment {
    const IID: ::windows_core::GUID =
        <IPolyQuadraticBezierSegment as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PolyQuadraticBezierSegment {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.PolyQuadraticBezierSegment";
}
::windows_core::imp::interface_hierarchy!(
    PolyQuadraticBezierSegment,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<PathSegment> for PolyQuadraticBezierSegment {}
impl ::windows_core::CanTryInto<super::DependencyObject> for PolyQuadraticBezierSegment {}
unsafe impl ::core::marker::Send for PolyQuadraticBezierSegment {}
unsafe impl ::core::marker::Sync for PolyQuadraticBezierSegment {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct Projection(::windows_core::IUnknown);
impl Projection {
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
    pub unsafe fn CreateInstance<P0>(
        baseinterface: P0,
        innerinterface: &mut ::core::option::Option<::windows_core::IInspectable>,
    ) -> ::windows_core::Result<Projection>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        Self::IProjectionFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstance)(
                ::windows_core::Interface::as_raw(this),
                baseinterface.into_param().abi(),
                innerinterface as *mut _ as _,
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IProjectionFactory<R, F: FnOnce(&IProjectionFactory) -> ::windows_core::Result<R>>(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<Projection, IProjectionFactory> =
            ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for Projection {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for Projection {
    type Vtable = IProjection_Vtbl;
}
unsafe impl ::windows_core::ComInterface for Projection {
    const IID: ::windows_core::GUID = <IProjection as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for Projection {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.Projection";
}
::windows_core::imp::interface_hierarchy!(
    Projection,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<super::DependencyObject> for Projection {}
unsafe impl ::core::marker::Send for Projection {}
unsafe impl ::core::marker::Sync for Projection {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct QuadraticBezierSegment(::windows_core::IUnknown);
impl QuadraticBezierSegment {
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
            QuadraticBezierSegment,
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
    pub fn Point1(&self) -> ::windows_core::Result<::windows::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Point1)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn SetPoint1(&self, value: ::windows::Foundation::Point) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetPoint1)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn Point2(&self) -> ::windows_core::Result<::windows::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Point2)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn SetPoint2(&self, value: ::windows::Foundation::Point) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetPoint2)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn Point1Property() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IQuadraticBezierSegmentStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Point1Property)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn Point2Property() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IQuadraticBezierSegmentStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Point2Property)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IQuadraticBezierSegmentStatics<
        R,
        F: FnOnce(&IQuadraticBezierSegmentStatics) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<
            QuadraticBezierSegment,
            IQuadraticBezierSegmentStatics,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for QuadraticBezierSegment {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for QuadraticBezierSegment {
    type Vtable = IQuadraticBezierSegment_Vtbl;
}
unsafe impl ::windows_core::ComInterface for QuadraticBezierSegment {
    const IID: ::windows_core::GUID =
        <IQuadraticBezierSegment as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for QuadraticBezierSegment {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.QuadraticBezierSegment";
}
::windows_core::imp::interface_hierarchy!(
    QuadraticBezierSegment,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<PathSegment> for QuadraticBezierSegment {}
impl ::windows_core::CanTryInto<super::DependencyObject> for QuadraticBezierSegment {}
unsafe impl ::core::marker::Send for QuadraticBezierSegment {}
unsafe impl ::core::marker::Sync for QuadraticBezierSegment {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct RadialGradientBrush(::windows_core::IUnknown);
impl RadialGradientBrush {
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
    pub fn Opacity(&self) -> ::windows_core::Result<f64> {
        let this = &::windows_core::ComInterface::cast::<IBrush>(self)?;
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
        let this = &::windows_core::ComInterface::cast::<IBrush>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetOpacity)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn Transform(&self) -> ::windows_core::Result<Transform> {
        let this = &::windows_core::ComInterface::cast::<IBrush>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Transform)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetTransform<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<Transform>,
    {
        let this = &::windows_core::ComInterface::cast::<IBrush>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetTransform)(
                ::windows_core::Interface::as_raw(this),
                value.try_into_param()?.abi(),
            )
            .ok()
        }
    }
    pub fn RelativeTransform(&self) -> ::windows_core::Result<Transform> {
        let this = &::windows_core::ComInterface::cast::<IBrush>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RelativeTransform)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetRelativeTransform<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<Transform>,
    {
        let this = &::windows_core::ComInterface::cast::<IBrush>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetRelativeTransform)(
                ::windows_core::Interface::as_raw(this),
                value.try_into_param()?.abi(),
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
        let this = &::windows_core::ComInterface::cast::<IBrushOverrides>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).PopulatePropertyInfoOverride)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(propertyname),
                animationpropertyinfo.into_param().abi(),
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
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn Center(&self) -> ::windows_core::Result<::windows::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Center)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn SetCenter(&self, value: ::windows::Foundation::Point) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetCenter)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn RadiusX(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RadiusX)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetRadiusX(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetRadiusX)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn RadiusY(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RadiusY)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetRadiusY(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetRadiusY)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn GradientOrigin(&self) -> ::windows_core::Result<::windows::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GradientOrigin)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn SetGradientOrigin(
        &self,
        value: ::windows::Foundation::Point,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetGradientOrigin)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn MappingMode(&self) -> ::windows_core::Result<BrushMappingMode> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MappingMode)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetMappingMode(&self, value: BrushMappingMode) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetMappingMode)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Composition\"`"]
    #[cfg(feature = "Microsoft_UI_Composition")]
    pub fn InterpolationSpace(
        &self,
    ) -> ::windows_core::Result<super::super::Composition::CompositionColorSpace> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).InterpolationSpace)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Composition\"`"]
    #[cfg(feature = "Microsoft_UI_Composition")]
    pub fn SetInterpolationSpace(
        &self,
        value: super::super::Composition::CompositionColorSpace,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetInterpolationSpace)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn SpreadMethod(&self) -> ::windows_core::Result<GradientSpreadMethod> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SpreadMethod)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetSpreadMethod(&self, value: GradientSpreadMethod) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetSpreadMethod)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation_Collections\"`"]
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub fn GradientStops(
        &self,
    ) -> ::windows_core::Result<::windows::Foundation::Collections::IObservableVector<GradientStop>>
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GradientStops)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub unsafe fn CreateInstance<P0>(
        baseinterface: P0,
        innerinterface: &mut ::core::option::Option<::windows_core::IInspectable>,
    ) -> ::windows_core::Result<RadialGradientBrush>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        Self::IRadialGradientBrushFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstance)(
                ::windows_core::Interface::as_raw(this),
                baseinterface.into_param().abi(),
                innerinterface as *mut _ as _,
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn CenterProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IRadialGradientBrushStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CenterProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn RadiusXProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IRadialGradientBrushStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RadiusXProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn RadiusYProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IRadialGradientBrushStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RadiusYProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn GradientOriginProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IRadialGradientBrushStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GradientOriginProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn InterpolationSpaceProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IRadialGradientBrushStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).InterpolationSpaceProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn MappingModeProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IRadialGradientBrushStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MappingModeProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn SpreadMethodProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IRadialGradientBrushStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SpreadMethodProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn FallbackColor(&self) -> ::windows_core::Result<::windows::UI::Color> {
        let this = &::windows_core::ComInterface::cast::<IXamlCompositionBrushBase>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FallbackColor)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn SetFallbackColor(&self, value: ::windows::UI::Color) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IXamlCompositionBrushBase>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetFallbackColor)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn OnConnected(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IXamlCompositionBrushBaseOverrides>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).OnConnected)(
                ::windows_core::Interface::as_raw(this),
            )
            .ok()
        }
    }
    pub fn OnDisconnected(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IXamlCompositionBrushBaseOverrides>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).OnDisconnected)(
                ::windows_core::Interface::as_raw(this),
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Composition\"`"]
    #[cfg(feature = "Microsoft_UI_Composition")]
    pub fn CompositionBrush(
        &self,
    ) -> ::windows_core::Result<super::super::Composition::CompositionBrush> {
        let this = &::windows_core::ComInterface::cast::<IXamlCompositionBrushBaseProtected>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CompositionBrush)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Composition\"`"]
    #[cfg(feature = "Microsoft_UI_Composition")]
    pub fn SetCompositionBrush<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::Composition::CompositionBrush>,
    {
        let this = &::windows_core::ComInterface::cast::<IXamlCompositionBrushBaseProtected>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetCompositionBrush)(
                ::windows_core::Interface::as_raw(this),
                value.try_into_param()?.abi(),
            )
            .ok()
        }
    }
    #[doc(hidden)]
    pub fn IRadialGradientBrushFactory<
        R,
        F: FnOnce(&IRadialGradientBrushFactory) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<
            RadialGradientBrush,
            IRadialGradientBrushFactory,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IRadialGradientBrushStatics<
        R,
        F: FnOnce(&IRadialGradientBrushStatics) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<
            RadialGradientBrush,
            IRadialGradientBrushStatics,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for RadialGradientBrush {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for RadialGradientBrush {
    type Vtable = IRadialGradientBrush_Vtbl;
}
unsafe impl ::windows_core::ComInterface for RadialGradientBrush {
    const IID: ::windows_core::GUID = <IRadialGradientBrush as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for RadialGradientBrush {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.RadialGradientBrush";
}
::windows_core::imp::interface_hierarchy!(
    RadialGradientBrush,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
#[cfg(feature = "Microsoft_UI_Composition")]
impl ::windows_core::CanTryInto<super::super::Composition::IAnimationObject>
    for RadialGradientBrush
{
}
impl ::windows_core::CanTryInto<XamlCompositionBrushBase> for RadialGradientBrush {}
impl ::windows_core::CanTryInto<Brush> for RadialGradientBrush {}
impl ::windows_core::CanTryInto<super::DependencyObject> for RadialGradientBrush {}
unsafe impl ::core::marker::Send for RadialGradientBrush {}
unsafe impl ::core::marker::Sync for RadialGradientBrush {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct RectangleGeometry(::windows_core::IUnknown);
impl RectangleGeometry {
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
            RectangleGeometry,
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
    pub fn Transform(&self) -> ::windows_core::Result<Transform> {
        let this = &::windows_core::ComInterface::cast::<IGeometry>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Transform)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetTransform<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<Transform>,
    {
        let this = &::windows_core::ComInterface::cast::<IGeometry>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetTransform)(
                ::windows_core::Interface::as_raw(this),
                value.try_into_param()?.abi(),
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn Bounds(&self) -> ::windows_core::Result<::windows::Foundation::Rect> {
        let this = &::windows_core::ComInterface::cast::<IGeometry>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Bounds)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn Rect(&self) -> ::windows_core::Result<::windows::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Rect)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn SetRect(&self, value: ::windows::Foundation::Rect) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetRect)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn RectProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IRectangleGeometryStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RectProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IRectangleGeometryStatics<
        R,
        F: FnOnce(&IRectangleGeometryStatics) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<
            RectangleGeometry,
            IRectangleGeometryStatics,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for RectangleGeometry {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for RectangleGeometry {
    type Vtable = IRectangleGeometry_Vtbl;
}
unsafe impl ::windows_core::ComInterface for RectangleGeometry {
    const IID: ::windows_core::GUID = <IRectangleGeometry as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for RectangleGeometry {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.RectangleGeometry";
}
::windows_core::imp::interface_hierarchy!(
    RectangleGeometry,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<Geometry> for RectangleGeometry {}
impl ::windows_core::CanTryInto<super::DependencyObject> for RectangleGeometry {}
unsafe impl ::core::marker::Send for RectangleGeometry {}
unsafe impl ::core::marker::Sync for RectangleGeometry {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct RenderedEventArgs(::windows_core::IUnknown);
impl RenderedEventArgs {
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn FrameDuration(&self) -> ::windows_core::Result<::windows::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FrameDuration)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for RenderedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for RenderedEventArgs {
    type Vtable = IRenderedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for RenderedEventArgs {
    const IID: ::windows_core::GUID = <IRenderedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for RenderedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.RenderedEventArgs";
}
::windows_core::imp::interface_hierarchy!(
    RenderedEventArgs,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for RenderedEventArgs {}
unsafe impl ::core::marker::Sync for RenderedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct RenderingEventArgs(::windows_core::IUnknown);
impl RenderingEventArgs {
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RenderingTime(&self) -> ::windows_core::Result<::windows::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RenderingTime)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for RenderingEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for RenderingEventArgs {
    type Vtable = IRenderingEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for RenderingEventArgs {
    const IID: ::windows_core::GUID = <IRenderingEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for RenderingEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.RenderingEventArgs";
}
::windows_core::imp::interface_hierarchy!(
    RenderingEventArgs,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for RenderingEventArgs {}
unsafe impl ::core::marker::Sync for RenderingEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct RotateTransform(::windows_core::IUnknown);
impl RotateTransform {
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
            RotateTransform,
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
    pub fn Inverse(&self) -> ::windows_core::Result<GeneralTransform> {
        let this = &::windows_core::ComInterface::cast::<IGeneralTransform>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Inverse)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn TransformPoint(
        &self,
        point: ::windows::Foundation::Point,
    ) -> ::windows_core::Result<::windows::Foundation::Point> {
        let this = &::windows_core::ComInterface::cast::<IGeneralTransform>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TransformPoint)(
                ::windows_core::Interface::as_raw(this),
                point,
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn TryTransform(
        &self,
        inpoint: ::windows::Foundation::Point,
        outpoint: &mut ::windows::Foundation::Point,
    ) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IGeneralTransform>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TryTransform)(
                ::windows_core::Interface::as_raw(this),
                inpoint,
                outpoint,
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn TransformBounds(
        &self,
        rect: ::windows::Foundation::Rect,
    ) -> ::windows_core::Result<::windows::Foundation::Rect> {
        let this = &::windows_core::ComInterface::cast::<IGeneralTransform>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TransformBounds)(
                ::windows_core::Interface::as_raw(this),
                rect,
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn InverseCore(&self) -> ::windows_core::Result<GeneralTransform> {
        let this = &::windows_core::ComInterface::cast::<IGeneralTransformOverrides>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).InverseCore)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn TryTransformCore(
        &self,
        inpoint: ::windows::Foundation::Point,
        outpoint: &mut ::windows::Foundation::Point,
    ) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IGeneralTransformOverrides>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TryTransformCore)(
                ::windows_core::Interface::as_raw(this),
                inpoint,
                outpoint,
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn TransformBoundsCore(
        &self,
        rect: ::windows::Foundation::Rect,
    ) -> ::windows_core::Result<::windows::Foundation::Rect> {
        let this = &::windows_core::ComInterface::cast::<IGeneralTransformOverrides>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TransformBoundsCore)(
                ::windows_core::Interface::as_raw(this),
                rect,
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn CenterX(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CenterX)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetCenterX(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetCenterX)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn CenterY(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CenterY)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetCenterY(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetCenterY)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn Angle(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Angle)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetAngle(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetAngle)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn CenterXProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IRotateTransformStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CenterXProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn CenterYProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IRotateTransformStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CenterYProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn AngleProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IRotateTransformStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AngleProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IRotateTransformStatics<
        R,
        F: FnOnce(&IRotateTransformStatics) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<RotateTransform, IRotateTransformStatics> =
            ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for RotateTransform {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for RotateTransform {
    type Vtable = IRotateTransform_Vtbl;
}
unsafe impl ::windows_core::ComInterface for RotateTransform {
    const IID: ::windows_core::GUID = <IRotateTransform as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for RotateTransform {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.RotateTransform";
}
::windows_core::imp::interface_hierarchy!(
    RotateTransform,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<Transform> for RotateTransform {}
impl ::windows_core::CanTryInto<GeneralTransform> for RotateTransform {}
impl ::windows_core::CanTryInto<super::DependencyObject> for RotateTransform {}
unsafe impl ::core::marker::Send for RotateTransform {}
unsafe impl ::core::marker::Sync for RotateTransform {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ScaleTransform(::windows_core::IUnknown);
impl ScaleTransform {
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
            ScaleTransform,
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
    pub fn Inverse(&self) -> ::windows_core::Result<GeneralTransform> {
        let this = &::windows_core::ComInterface::cast::<IGeneralTransform>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Inverse)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn TransformPoint(
        &self,
        point: ::windows::Foundation::Point,
    ) -> ::windows_core::Result<::windows::Foundation::Point> {
        let this = &::windows_core::ComInterface::cast::<IGeneralTransform>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TransformPoint)(
                ::windows_core::Interface::as_raw(this),
                point,
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn TryTransform(
        &self,
        inpoint: ::windows::Foundation::Point,
        outpoint: &mut ::windows::Foundation::Point,
    ) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IGeneralTransform>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TryTransform)(
                ::windows_core::Interface::as_raw(this),
                inpoint,
                outpoint,
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn TransformBounds(
        &self,
        rect: ::windows::Foundation::Rect,
    ) -> ::windows_core::Result<::windows::Foundation::Rect> {
        let this = &::windows_core::ComInterface::cast::<IGeneralTransform>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TransformBounds)(
                ::windows_core::Interface::as_raw(this),
                rect,
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn InverseCore(&self) -> ::windows_core::Result<GeneralTransform> {
        let this = &::windows_core::ComInterface::cast::<IGeneralTransformOverrides>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).InverseCore)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn TryTransformCore(
        &self,
        inpoint: ::windows::Foundation::Point,
        outpoint: &mut ::windows::Foundation::Point,
    ) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IGeneralTransformOverrides>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TryTransformCore)(
                ::windows_core::Interface::as_raw(this),
                inpoint,
                outpoint,
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn TransformBoundsCore(
        &self,
        rect: ::windows::Foundation::Rect,
    ) -> ::windows_core::Result<::windows::Foundation::Rect> {
        let this = &::windows_core::ComInterface::cast::<IGeneralTransformOverrides>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TransformBoundsCore)(
                ::windows_core::Interface::as_raw(this),
                rect,
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn CenterX(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CenterX)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetCenterX(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetCenterX)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn CenterY(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CenterY)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetCenterY(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetCenterY)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn ScaleX(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ScaleX)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetScaleX(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetScaleX)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn ScaleY(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ScaleY)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetScaleY(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetScaleY)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn CenterXProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IScaleTransformStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CenterXProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn CenterYProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IScaleTransformStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CenterYProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn ScaleXProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IScaleTransformStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ScaleXProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn ScaleYProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IScaleTransformStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ScaleYProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IScaleTransformStatics<
        R,
        F: FnOnce(&IScaleTransformStatics) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<ScaleTransform, IScaleTransformStatics> =
            ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for ScaleTransform {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for ScaleTransform {
    type Vtable = IScaleTransform_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ScaleTransform {
    const IID: ::windows_core::GUID = <IScaleTransform as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ScaleTransform {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.ScaleTransform";
}
::windows_core::imp::interface_hierarchy!(
    ScaleTransform,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<Transform> for ScaleTransform {}
impl ::windows_core::CanTryInto<GeneralTransform> for ScaleTransform {}
impl ::windows_core::CanTryInto<super::DependencyObject> for ScaleTransform {}
unsafe impl ::core::marker::Send for ScaleTransform {}
unsafe impl ::core::marker::Sync for ScaleTransform {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct Shadow(::windows_core::IUnknown);
impl Shadow {
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
impl ::windows_core::RuntimeType for Shadow {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for Shadow {
    type Vtable = IShadow_Vtbl;
}
unsafe impl ::windows_core::ComInterface for Shadow {
    const IID: ::windows_core::GUID = <IShadow as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for Shadow {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.Shadow";
}
::windows_core::imp::interface_hierarchy!(
    Shadow,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<super::DependencyObject> for Shadow {}
unsafe impl ::core::marker::Send for Shadow {}
unsafe impl ::core::marker::Sync for Shadow {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct SkewTransform(::windows_core::IUnknown);
impl SkewTransform {
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
            SkewTransform,
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
    pub fn Inverse(&self) -> ::windows_core::Result<GeneralTransform> {
        let this = &::windows_core::ComInterface::cast::<IGeneralTransform>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Inverse)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn TransformPoint(
        &self,
        point: ::windows::Foundation::Point,
    ) -> ::windows_core::Result<::windows::Foundation::Point> {
        let this = &::windows_core::ComInterface::cast::<IGeneralTransform>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TransformPoint)(
                ::windows_core::Interface::as_raw(this),
                point,
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn TryTransform(
        &self,
        inpoint: ::windows::Foundation::Point,
        outpoint: &mut ::windows::Foundation::Point,
    ) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IGeneralTransform>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TryTransform)(
                ::windows_core::Interface::as_raw(this),
                inpoint,
                outpoint,
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn TransformBounds(
        &self,
        rect: ::windows::Foundation::Rect,
    ) -> ::windows_core::Result<::windows::Foundation::Rect> {
        let this = &::windows_core::ComInterface::cast::<IGeneralTransform>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TransformBounds)(
                ::windows_core::Interface::as_raw(this),
                rect,
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn InverseCore(&self) -> ::windows_core::Result<GeneralTransform> {
        let this = &::windows_core::ComInterface::cast::<IGeneralTransformOverrides>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).InverseCore)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn TryTransformCore(
        &self,
        inpoint: ::windows::Foundation::Point,
        outpoint: &mut ::windows::Foundation::Point,
    ) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IGeneralTransformOverrides>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TryTransformCore)(
                ::windows_core::Interface::as_raw(this),
                inpoint,
                outpoint,
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn TransformBoundsCore(
        &self,
        rect: ::windows::Foundation::Rect,
    ) -> ::windows_core::Result<::windows::Foundation::Rect> {
        let this = &::windows_core::ComInterface::cast::<IGeneralTransformOverrides>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TransformBoundsCore)(
                ::windows_core::Interface::as_raw(this),
                rect,
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn CenterX(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CenterX)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetCenterX(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetCenterX)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn CenterY(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CenterY)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetCenterY(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetCenterY)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn AngleX(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AngleX)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetAngleX(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetAngleX)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn AngleY(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AngleY)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetAngleY(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetAngleY)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn CenterXProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::ISkewTransformStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CenterXProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn CenterYProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::ISkewTransformStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CenterYProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn AngleXProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::ISkewTransformStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AngleXProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn AngleYProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::ISkewTransformStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AngleYProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ISkewTransformStatics<
        R,
        F: FnOnce(&ISkewTransformStatics) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<SkewTransform, ISkewTransformStatics> =
            ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for SkewTransform {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for SkewTransform {
    type Vtable = ISkewTransform_Vtbl;
}
unsafe impl ::windows_core::ComInterface for SkewTransform {
    const IID: ::windows_core::GUID = <ISkewTransform as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for SkewTransform {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.SkewTransform";
}
::windows_core::imp::interface_hierarchy!(
    SkewTransform,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<Transform> for SkewTransform {}
impl ::windows_core::CanTryInto<GeneralTransform> for SkewTransform {}
impl ::windows_core::CanTryInto<super::DependencyObject> for SkewTransform {}
unsafe impl ::core::marker::Send for SkewTransform {}
unsafe impl ::core::marker::Sync for SkewTransform {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct SolidColorBrush(::windows_core::IUnknown);
impl SolidColorBrush {
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
            SolidColorBrush,
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
    pub fn Opacity(&self) -> ::windows_core::Result<f64> {
        let this = &::windows_core::ComInterface::cast::<IBrush>(self)?;
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
        let this = &::windows_core::ComInterface::cast::<IBrush>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetOpacity)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn Transform(&self) -> ::windows_core::Result<Transform> {
        let this = &::windows_core::ComInterface::cast::<IBrush>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Transform)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetTransform<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<Transform>,
    {
        let this = &::windows_core::ComInterface::cast::<IBrush>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetTransform)(
                ::windows_core::Interface::as_raw(this),
                value.try_into_param()?.abi(),
            )
            .ok()
        }
    }
    pub fn RelativeTransform(&self) -> ::windows_core::Result<Transform> {
        let this = &::windows_core::ComInterface::cast::<IBrush>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RelativeTransform)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetRelativeTransform<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<Transform>,
    {
        let this = &::windows_core::ComInterface::cast::<IBrush>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetRelativeTransform)(
                ::windows_core::Interface::as_raw(this),
                value.try_into_param()?.abi(),
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
        let this = &::windows_core::ComInterface::cast::<IBrushOverrides>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).PopulatePropertyInfoOverride)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(propertyname),
                animationpropertyinfo.into_param().abi(),
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
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn Color(&self) -> ::windows_core::Result<::windows::UI::Color> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Color)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn SetColor(&self, value: ::windows::UI::Color) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetColor)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn CreateInstanceWithColor(
        color: ::windows::UI::Color,
    ) -> ::windows_core::Result<SolidColorBrush> {
        Self::ISolidColorBrushFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstanceWithColor)(
                ::windows_core::Interface::as_raw(this),
                color,
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn ColorProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::ISolidColorBrushStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ColorProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ISolidColorBrushFactory<
        R,
        F: FnOnce(&ISolidColorBrushFactory) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<SolidColorBrush, ISolidColorBrushFactory> =
            ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ISolidColorBrushStatics<
        R,
        F: FnOnce(&ISolidColorBrushStatics) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<SolidColorBrush, ISolidColorBrushStatics> =
            ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for SolidColorBrush {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for SolidColorBrush {
    type Vtable = ISolidColorBrush_Vtbl;
}
unsafe impl ::windows_core::ComInterface for SolidColorBrush {
    const IID: ::windows_core::GUID = <ISolidColorBrush as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for SolidColorBrush {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.SolidColorBrush";
}
::windows_core::imp::interface_hierarchy!(
    SolidColorBrush,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
#[cfg(feature = "Microsoft_UI_Composition")]
impl ::windows_core::CanTryInto<super::super::Composition::IAnimationObject> for SolidColorBrush {}
impl ::windows_core::CanTryInto<Brush> for SolidColorBrush {}
impl ::windows_core::CanTryInto<super::DependencyObject> for SolidColorBrush {}
unsafe impl ::core::marker::Send for SolidColorBrush {}
unsafe impl ::core::marker::Sync for SolidColorBrush {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct SystemBackdrop(::windows_core::IUnknown);
impl SystemBackdrop {
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
    #[doc = "Required features: `\"Microsoft_UI_Composition_SystemBackdrops\"`"]
    #[cfg(feature = "Microsoft_UI_Composition_SystemBackdrops")]
    pub fn GetDefaultSystemBackdropConfiguration<P0, P1>(
        &self,
        target: P0,
        xamlroot: P1,
    ) -> ::windows_core::Result<
        super::super::Composition::SystemBackdrops::SystemBackdropConfiguration,
    >
    where
        P0: ::windows_core::TryIntoParam<
            super::super::Composition::ICompositionSupportsSystemBackdrop,
        >,
        P1: ::windows_core::IntoParam<super::XamlRoot>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetDefaultSystemBackdropConfiguration)(
                ::windows_core::Interface::as_raw(this),
                target.try_into_param()?.abi(),
                xamlroot.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub unsafe fn CreateInstance<P0>(
        baseinterface: P0,
        innerinterface: &mut ::core::option::Option<::windows_core::IInspectable>,
    ) -> ::windows_core::Result<SystemBackdrop>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        Self::ISystemBackdropFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstance)(
                ::windows_core::Interface::as_raw(this),
                baseinterface.into_param().abi(),
                innerinterface as *mut _ as _,
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Microsoft_UI_Composition\"`"]
    #[cfg(feature = "Microsoft_UI_Composition")]
    pub fn OnTargetConnected<P0, P1>(
        &self,
        connectedtarget: P0,
        xamlroot: P1,
    ) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<
            super::super::Composition::ICompositionSupportsSystemBackdrop,
        >,
        P1: ::windows_core::IntoParam<super::XamlRoot>,
    {
        let this = &::windows_core::ComInterface::cast::<ISystemBackdropOverrides>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).OnTargetConnected)(
                ::windows_core::Interface::as_raw(this),
                connectedtarget.try_into_param()?.abi(),
                xamlroot.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Composition\"`"]
    #[cfg(feature = "Microsoft_UI_Composition")]
    pub fn OnTargetDisconnected<P0>(&self, disconnectedtarget: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<
            super::super::Composition::ICompositionSupportsSystemBackdrop,
        >,
    {
        let this = &::windows_core::ComInterface::cast::<ISystemBackdropOverrides>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).OnTargetDisconnected)(
                ::windows_core::Interface::as_raw(this),
                disconnectedtarget.try_into_param()?.abi(),
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Composition\"`"]
    #[cfg(feature = "Microsoft_UI_Composition")]
    pub fn OnDefaultSystemBackdropConfigurationChanged<P0, P1>(
        &self,
        target: P0,
        xamlroot: P1,
    ) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<
            super::super::Composition::ICompositionSupportsSystemBackdrop,
        >,
        P1: ::windows_core::IntoParam<super::XamlRoot>,
    {
        let this = &::windows_core::ComInterface::cast::<ISystemBackdropOverrides>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).OnDefaultSystemBackdropConfigurationChanged)(
                ::windows_core::Interface::as_raw(this),
                target.try_into_param()?.abi(),
                xamlroot.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc(hidden)]
    pub fn ISystemBackdropFactory<
        R,
        F: FnOnce(&ISystemBackdropFactory) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<SystemBackdrop, ISystemBackdropFactory> =
            ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for SystemBackdrop {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for SystemBackdrop {
    type Vtable = ISystemBackdrop_Vtbl;
}
unsafe impl ::windows_core::ComInterface for SystemBackdrop {
    const IID: ::windows_core::GUID = <ISystemBackdrop as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for SystemBackdrop {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.SystemBackdrop";
}
::windows_core::imp::interface_hierarchy!(
    SystemBackdrop,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<super::DependencyObject> for SystemBackdrop {}
unsafe impl ::core::marker::Send for SystemBackdrop {}
unsafe impl ::core::marker::Sync for SystemBackdrop {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ThemeShadow(::windows_core::IUnknown);
impl ThemeShadow {
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
    pub fn Receivers(&self) -> ::windows_core::Result<super::UIElementWeakCollection> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Receivers)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub unsafe fn CreateInstance<P0>(
        baseinterface: P0,
        innerinterface: &mut ::core::option::Option<::windows_core::IInspectable>,
    ) -> ::windows_core::Result<ThemeShadow>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        Self::IThemeShadowFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstance)(
                ::windows_core::Interface::as_raw(this),
                baseinterface.into_param().abi(),
                innerinterface as *mut _ as _,
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IThemeShadowFactory<R, F: FnOnce(&IThemeShadowFactory) -> ::windows_core::Result<R>>(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<ThemeShadow, IThemeShadowFactory> =
            ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for ThemeShadow {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for ThemeShadow {
    type Vtable = IThemeShadow_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ThemeShadow {
    const IID: ::windows_core::GUID = <IThemeShadow as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ThemeShadow {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.ThemeShadow";
}
::windows_core::imp::interface_hierarchy!(
    ThemeShadow,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<Shadow> for ThemeShadow {}
impl ::windows_core::CanTryInto<super::DependencyObject> for ThemeShadow {}
unsafe impl ::core::marker::Send for ThemeShadow {}
unsafe impl ::core::marker::Sync for ThemeShadow {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct TileBrush(::windows_core::IUnknown);
impl TileBrush {
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
    pub fn Opacity(&self) -> ::windows_core::Result<f64> {
        let this = &::windows_core::ComInterface::cast::<IBrush>(self)?;
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
        let this = &::windows_core::ComInterface::cast::<IBrush>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetOpacity)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn Transform(&self) -> ::windows_core::Result<Transform> {
        let this = &::windows_core::ComInterface::cast::<IBrush>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Transform)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetTransform<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<Transform>,
    {
        let this = &::windows_core::ComInterface::cast::<IBrush>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetTransform)(
                ::windows_core::Interface::as_raw(this),
                value.try_into_param()?.abi(),
            )
            .ok()
        }
    }
    pub fn RelativeTransform(&self) -> ::windows_core::Result<Transform> {
        let this = &::windows_core::ComInterface::cast::<IBrush>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RelativeTransform)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetRelativeTransform<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<Transform>,
    {
        let this = &::windows_core::ComInterface::cast::<IBrush>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetRelativeTransform)(
                ::windows_core::Interface::as_raw(this),
                value.try_into_param()?.abi(),
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
        let this = &::windows_core::ComInterface::cast::<IBrushOverrides>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).PopulatePropertyInfoOverride)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(propertyname),
                animationpropertyinfo.into_param().abi(),
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
    pub fn AlignmentX(&self) -> ::windows_core::Result<AlignmentX> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AlignmentX)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetAlignmentX(&self, value: AlignmentX) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetAlignmentX)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn AlignmentY(&self) -> ::windows_core::Result<AlignmentY> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AlignmentY)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetAlignmentY(&self, value: AlignmentY) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetAlignmentY)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn Stretch(&self) -> ::windows_core::Result<Stretch> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Stretch)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetStretch(&self, value: Stretch) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetStretch)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub unsafe fn CreateInstance<P0>(
        baseinterface: P0,
        innerinterface: &mut ::core::option::Option<::windows_core::IInspectable>,
    ) -> ::windows_core::Result<TileBrush>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        Self::ITileBrushFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstance)(
                ::windows_core::Interface::as_raw(this),
                baseinterface.into_param().abi(),
                innerinterface as *mut _ as _,
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn AlignmentXProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::ITileBrushStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AlignmentXProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn AlignmentYProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::ITileBrushStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AlignmentYProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn StretchProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::ITileBrushStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StretchProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ITileBrushFactory<R, F: FnOnce(&ITileBrushFactory) -> ::windows_core::Result<R>>(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<TileBrush, ITileBrushFactory> =
            ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ITileBrushStatics<R, F: FnOnce(&ITileBrushStatics) -> ::windows_core::Result<R>>(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<TileBrush, ITileBrushStatics> =
            ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for TileBrush {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for TileBrush {
    type Vtable = ITileBrush_Vtbl;
}
unsafe impl ::windows_core::ComInterface for TileBrush {
    const IID: ::windows_core::GUID = <ITileBrush as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for TileBrush {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.TileBrush";
}
::windows_core::imp::interface_hierarchy!(
    TileBrush,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
#[cfg(feature = "Microsoft_UI_Composition")]
impl ::windows_core::CanTryInto<super::super::Composition::IAnimationObject> for TileBrush {}
impl ::windows_core::CanTryInto<Brush> for TileBrush {}
impl ::windows_core::CanTryInto<super::DependencyObject> for TileBrush {}
unsafe impl ::core::marker::Send for TileBrush {}
unsafe impl ::core::marker::Sync for TileBrush {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct Transform(::windows_core::IUnknown);
impl Transform {
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
    pub fn Inverse(&self) -> ::windows_core::Result<GeneralTransform> {
        let this = &::windows_core::ComInterface::cast::<IGeneralTransform>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Inverse)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn TransformPoint(
        &self,
        point: ::windows::Foundation::Point,
    ) -> ::windows_core::Result<::windows::Foundation::Point> {
        let this = &::windows_core::ComInterface::cast::<IGeneralTransform>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TransformPoint)(
                ::windows_core::Interface::as_raw(this),
                point,
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn TryTransform(
        &self,
        inpoint: ::windows::Foundation::Point,
        outpoint: &mut ::windows::Foundation::Point,
    ) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IGeneralTransform>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TryTransform)(
                ::windows_core::Interface::as_raw(this),
                inpoint,
                outpoint,
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn TransformBounds(
        &self,
        rect: ::windows::Foundation::Rect,
    ) -> ::windows_core::Result<::windows::Foundation::Rect> {
        let this = &::windows_core::ComInterface::cast::<IGeneralTransform>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TransformBounds)(
                ::windows_core::Interface::as_raw(this),
                rect,
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn InverseCore(&self) -> ::windows_core::Result<GeneralTransform> {
        let this = &::windows_core::ComInterface::cast::<IGeneralTransformOverrides>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).InverseCore)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn TryTransformCore(
        &self,
        inpoint: ::windows::Foundation::Point,
        outpoint: &mut ::windows::Foundation::Point,
    ) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IGeneralTransformOverrides>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TryTransformCore)(
                ::windows_core::Interface::as_raw(this),
                inpoint,
                outpoint,
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn TransformBoundsCore(
        &self,
        rect: ::windows::Foundation::Rect,
    ) -> ::windows_core::Result<::windows::Foundation::Rect> {
        let this = &::windows_core::ComInterface::cast::<IGeneralTransformOverrides>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TransformBoundsCore)(
                ::windows_core::Interface::as_raw(this),
                rect,
                &mut result__,
            )
            .from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for Transform {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for Transform {
    type Vtable = ITransform_Vtbl;
}
unsafe impl ::windows_core::ComInterface for Transform {
    const IID: ::windows_core::GUID = <ITransform as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for Transform {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.Transform";
}
::windows_core::imp::interface_hierarchy!(
    Transform,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<GeneralTransform> for Transform {}
impl ::windows_core::CanTryInto<super::DependencyObject> for Transform {}
unsafe impl ::core::marker::Send for Transform {}
unsafe impl ::core::marker::Sync for Transform {}
#[doc = "Required features: `\"Windows_Foundation_Collections\"`"]
#[cfg(feature = "Windows_Foundation_Collections")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct TransformCollection(::windows_core::IUnknown);
#[cfg(feature = "Windows_Foundation_Collections")]
impl TransformCollection {
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
            TransformCollection,
            ::windows_core::imp::IGenericFactory,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "Required features: `\"Windows_Foundation_Collections\"`"]
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub fn First(
        &self,
    ) -> ::windows_core::Result<::windows::Foundation::Collections::IIterator<Transform>> {
        let this = &::windows_core::ComInterface::cast::<
            ::windows::Foundation::Collections::IIterable<Transform>,
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
    pub fn GetAt(&self, index: u32) -> ::windows_core::Result<Transform> {
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
    ) -> ::windows_core::Result<::windows::Foundation::Collections::IVectorView<Transform>> {
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
        P0: ::windows_core::TryIntoParam<Transform>,
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
        P0: ::windows_core::TryIntoParam<Transform>,
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
        P0: ::windows_core::TryIntoParam<Transform>,
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
        P0: ::windows_core::TryIntoParam<Transform>,
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
        items: &mut [::core::option::Option<Transform>],
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
        items: &[::core::option::Option<Transform>],
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
impl ::windows_core::RuntimeType for TransformCollection {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
#[cfg(feature = "Windows_Foundation_Collections")]
unsafe impl ::windows_core::Interface for TransformCollection {
    type Vtable = ::windows::Foundation::Collections::IVector_Vtbl<Transform>;
}
#[cfg(feature = "Windows_Foundation_Collections")]
unsafe impl ::windows_core::ComInterface for TransformCollection {
    const IID : ::windows_core::GUID = < ::windows::Foundation::Collections:: IVector :: < Transform > as::windows_core::ComInterface >::IID ;
}
#[cfg(feature = "Windows_Foundation_Collections")]
impl ::windows_core::RuntimeName for TransformCollection {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.TransformCollection";
}
#[cfg(feature = "Windows_Foundation_Collections")]
impl ::core::iter::IntoIterator for TransformCollection {
    type Item = Transform;
    type IntoIter = ::windows::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Windows_Foundation_Collections")]
impl ::core::iter::IntoIterator for &TransformCollection {
    type Item = Transform;
    type IntoIter = ::windows::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::windows::Foundation::Collections::VectorIterator::new(
            ::windows_core::ComInterface::cast(self).ok(),
        )
    }
}
#[cfg(feature = "Windows_Foundation_Collections")]
::windows_core::imp::interface_hierarchy!(
    TransformCollection,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
#[cfg(feature = "Windows_Foundation_Collections")]
impl ::windows_core::CanTryInto<::windows::Foundation::Collections::IIterable<Transform>>
    for TransformCollection
{
}
#[cfg(feature = "Windows_Foundation_Collections")]
impl ::windows_core::CanTryInto<::windows::Foundation::Collections::IVector<Transform>>
    for TransformCollection
{
}
#[cfg(feature = "Windows_Foundation_Collections")]
unsafe impl ::core::marker::Send for TransformCollection {}
#[cfg(feature = "Windows_Foundation_Collections")]
unsafe impl ::core::marker::Sync for TransformCollection {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct TransformGroup(::windows_core::IUnknown);
impl TransformGroup {
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
            TransformGroup,
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
    pub fn Inverse(&self) -> ::windows_core::Result<GeneralTransform> {
        let this = &::windows_core::ComInterface::cast::<IGeneralTransform>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Inverse)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn TransformPoint(
        &self,
        point: ::windows::Foundation::Point,
    ) -> ::windows_core::Result<::windows::Foundation::Point> {
        let this = &::windows_core::ComInterface::cast::<IGeneralTransform>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TransformPoint)(
                ::windows_core::Interface::as_raw(this),
                point,
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn TryTransform(
        &self,
        inpoint: ::windows::Foundation::Point,
        outpoint: &mut ::windows::Foundation::Point,
    ) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IGeneralTransform>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TryTransform)(
                ::windows_core::Interface::as_raw(this),
                inpoint,
                outpoint,
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn TransformBounds(
        &self,
        rect: ::windows::Foundation::Rect,
    ) -> ::windows_core::Result<::windows::Foundation::Rect> {
        let this = &::windows_core::ComInterface::cast::<IGeneralTransform>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TransformBounds)(
                ::windows_core::Interface::as_raw(this),
                rect,
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn InverseCore(&self) -> ::windows_core::Result<GeneralTransform> {
        let this = &::windows_core::ComInterface::cast::<IGeneralTransformOverrides>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).InverseCore)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn TryTransformCore(
        &self,
        inpoint: ::windows::Foundation::Point,
        outpoint: &mut ::windows::Foundation::Point,
    ) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IGeneralTransformOverrides>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TryTransformCore)(
                ::windows_core::Interface::as_raw(this),
                inpoint,
                outpoint,
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn TransformBoundsCore(
        &self,
        rect: ::windows::Foundation::Rect,
    ) -> ::windows_core::Result<::windows::Foundation::Rect> {
        let this = &::windows_core::ComInterface::cast::<IGeneralTransformOverrides>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TransformBoundsCore)(
                ::windows_core::Interface::as_raw(this),
                rect,
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation_Collections\"`"]
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub fn Children(&self) -> ::windows_core::Result<TransformCollection> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Children)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation_Collections\"`"]
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub fn SetChildren<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<TransformCollection>,
    {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetChildren)(
                ::windows_core::Interface::as_raw(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn Value(&self) -> ::windows_core::Result<Matrix> {
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
    pub fn ChildrenProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::ITransformGroupStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ChildrenProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ITransformGroupStatics<
        R,
        F: FnOnce(&ITransformGroupStatics) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<TransformGroup, ITransformGroupStatics> =
            ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for TransformGroup {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for TransformGroup {
    type Vtable = ITransformGroup_Vtbl;
}
unsafe impl ::windows_core::ComInterface for TransformGroup {
    const IID: ::windows_core::GUID = <ITransformGroup as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for TransformGroup {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.TransformGroup";
}
::windows_core::imp::interface_hierarchy!(
    TransformGroup,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<Transform> for TransformGroup {}
impl ::windows_core::CanTryInto<GeneralTransform> for TransformGroup {}
impl ::windows_core::CanTryInto<super::DependencyObject> for TransformGroup {}
unsafe impl ::core::marker::Send for TransformGroup {}
unsafe impl ::core::marker::Sync for TransformGroup {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct TranslateTransform(::windows_core::IUnknown);
impl TranslateTransform {
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
            TranslateTransform,
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
    pub fn Inverse(&self) -> ::windows_core::Result<GeneralTransform> {
        let this = &::windows_core::ComInterface::cast::<IGeneralTransform>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Inverse)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn TransformPoint(
        &self,
        point: ::windows::Foundation::Point,
    ) -> ::windows_core::Result<::windows::Foundation::Point> {
        let this = &::windows_core::ComInterface::cast::<IGeneralTransform>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TransformPoint)(
                ::windows_core::Interface::as_raw(this),
                point,
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn TryTransform(
        &self,
        inpoint: ::windows::Foundation::Point,
        outpoint: &mut ::windows::Foundation::Point,
    ) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IGeneralTransform>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TryTransform)(
                ::windows_core::Interface::as_raw(this),
                inpoint,
                outpoint,
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn TransformBounds(
        &self,
        rect: ::windows::Foundation::Rect,
    ) -> ::windows_core::Result<::windows::Foundation::Rect> {
        let this = &::windows_core::ComInterface::cast::<IGeneralTransform>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TransformBounds)(
                ::windows_core::Interface::as_raw(this),
                rect,
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn InverseCore(&self) -> ::windows_core::Result<GeneralTransform> {
        let this = &::windows_core::ComInterface::cast::<IGeneralTransformOverrides>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).InverseCore)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn TryTransformCore(
        &self,
        inpoint: ::windows::Foundation::Point,
        outpoint: &mut ::windows::Foundation::Point,
    ) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IGeneralTransformOverrides>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TryTransformCore)(
                ::windows_core::Interface::as_raw(this),
                inpoint,
                outpoint,
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn TransformBoundsCore(
        &self,
        rect: ::windows::Foundation::Rect,
    ) -> ::windows_core::Result<::windows::Foundation::Rect> {
        let this = &::windows_core::ComInterface::cast::<IGeneralTransformOverrides>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TransformBoundsCore)(
                ::windows_core::Interface::as_raw(this),
                rect,
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn X(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).X)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetX(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetX)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn Y(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Y)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetY(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetY)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn XProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::ITranslateTransformStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).XProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn YProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::ITranslateTransformStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).YProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ITranslateTransformStatics<
        R,
        F: FnOnce(&ITranslateTransformStatics) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<
            TranslateTransform,
            ITranslateTransformStatics,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for TranslateTransform {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for TranslateTransform {
    type Vtable = ITranslateTransform_Vtbl;
}
unsafe impl ::windows_core::ComInterface for TranslateTransform {
    const IID: ::windows_core::GUID = <ITranslateTransform as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for TranslateTransform {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.TranslateTransform";
}
::windows_core::imp::interface_hierarchy!(
    TranslateTransform,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<Transform> for TranslateTransform {}
impl ::windows_core::CanTryInto<GeneralTransform> for TranslateTransform {}
impl ::windows_core::CanTryInto<super::DependencyObject> for TranslateTransform {}
unsafe impl ::core::marker::Send for TranslateTransform {}
unsafe impl ::core::marker::Sync for TranslateTransform {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct VisualTreeHelper(::windows_core::IUnknown);
impl VisualTreeHelper {
    #[doc = "Required features: `\"Windows_Foundation_Collections\"`"]
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub fn FindElementsInHostCoordinatesPoint<P0>(
        intersectingpoint: ::windows::Foundation::Point,
        subtree: P0,
    ) -> ::windows_core::Result<::windows::Foundation::Collections::IIterable<super::UIElement>>
    where
        P0: ::windows_core::TryIntoParam<super::UIElement>,
    {
        Self::IVisualTreeHelperStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FindElementsInHostCoordinatesPoint)(
                ::windows_core::Interface::as_raw(this),
                intersectingpoint,
                subtree.try_into_param()?.abi(),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_Foundation_Collections\"`"]
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub fn FindElementsInHostCoordinatesRect<P0>(
        intersectingrect: ::windows::Foundation::Rect,
        subtree: P0,
    ) -> ::windows_core::Result<::windows::Foundation::Collections::IIterable<super::UIElement>>
    where
        P0: ::windows_core::TryIntoParam<super::UIElement>,
    {
        Self::IVisualTreeHelperStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FindElementsInHostCoordinatesRect)(
                ::windows_core::Interface::as_raw(this),
                intersectingrect,
                subtree.try_into_param()?.abi(),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_Foundation_Collections\"`"]
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub fn FindAllElementsInHostCoordinatesPoint<P0>(
        intersectingpoint: ::windows::Foundation::Point,
        subtree: P0,
        includeallelements: bool,
    ) -> ::windows_core::Result<::windows::Foundation::Collections::IIterable<super::UIElement>>
    where
        P0: ::windows_core::TryIntoParam<super::UIElement>,
    {
        Self::IVisualTreeHelperStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FindAllElementsInHostCoordinatesPoint)(
                ::windows_core::Interface::as_raw(this),
                intersectingpoint,
                subtree.try_into_param()?.abi(),
                includeallelements,
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_Foundation_Collections\"`"]
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub fn FindAllElementsInHostCoordinatesRect<P0>(
        intersectingrect: ::windows::Foundation::Rect,
        subtree: P0,
        includeallelements: bool,
    ) -> ::windows_core::Result<::windows::Foundation::Collections::IIterable<super::UIElement>>
    where
        P0: ::windows_core::TryIntoParam<super::UIElement>,
    {
        Self::IVisualTreeHelperStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FindAllElementsInHostCoordinatesRect)(
                ::windows_core::Interface::as_raw(this),
                intersectingrect,
                subtree.try_into_param()?.abi(),
                includeallelements,
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn GetChild<P0>(
        reference: P0,
        childindex: i32,
    ) -> ::windows_core::Result<super::DependencyObject>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        Self::IVisualTreeHelperStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetChild)(
                ::windows_core::Interface::as_raw(this),
                reference.try_into_param()?.abi(),
                childindex,
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn GetChildrenCount<P0>(reference: P0) -> ::windows_core::Result<i32>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        Self::IVisualTreeHelperStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetChildrenCount)(
                ::windows_core::Interface::as_raw(this),
                reference.try_into_param()?.abi(),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn GetParent<P0>(reference: P0) -> ::windows_core::Result<super::DependencyObject>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        Self::IVisualTreeHelperStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetParent)(
                ::windows_core::Interface::as_raw(this),
                reference.try_into_param()?.abi(),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn DisconnectChildrenRecursive<P0>(element: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::UIElement>,
    {
        Self::IVisualTreeHelperStatics(|this| unsafe {
            (::windows_core::Interface::vtable(this).DisconnectChildrenRecursive)(
                ::windows_core::Interface::as_raw(this),
                element.try_into_param()?.abi(),
            )
            .ok()
        })
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Controls_Primitives\"`, `\"Windows_Foundation_Collections\"`"]
    #[cfg(all(
        feature = "Microsoft_UI_Xaml_Controls_Primitives",
        feature = "Windows_Foundation_Collections"
    ))]
    pub fn GetOpenPopups<P0>(
        window: P0,
    ) -> ::windows_core::Result<
        ::windows::Foundation::Collections::IVectorView<super::Controls::Primitives::Popup>,
    >
    where
        P0: ::windows_core::TryIntoParam<super::Window>,
    {
        Self::IVisualTreeHelperStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetOpenPopups)(
                ::windows_core::Interface::as_raw(this),
                window.try_into_param()?.abi(),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Controls_Primitives\"`, `\"Windows_Foundation_Collections\"`"]
    #[cfg(all(
        feature = "Microsoft_UI_Xaml_Controls_Primitives",
        feature = "Windows_Foundation_Collections"
    ))]
    pub fn GetOpenPopupsForXamlRoot<P0>(
        xamlroot: P0,
    ) -> ::windows_core::Result<
        ::windows::Foundation::Collections::IVectorView<super::Controls::Primitives::Popup>,
    >
    where
        P0: ::windows_core::IntoParam<super::XamlRoot>,
    {
        Self::IVisualTreeHelperStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetOpenPopupsForXamlRoot)(
                ::windows_core::Interface::as_raw(this),
                xamlroot.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IVisualTreeHelperStatics<
        R,
        F: FnOnce(&IVisualTreeHelperStatics) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<
            VisualTreeHelper,
            IVisualTreeHelperStatics,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for VisualTreeHelper {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for VisualTreeHelper {
    type Vtable = IVisualTreeHelper_Vtbl;
}
unsafe impl ::windows_core::ComInterface for VisualTreeHelper {
    const IID: ::windows_core::GUID = <IVisualTreeHelper as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for VisualTreeHelper {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.VisualTreeHelper";
}
::windows_core::imp::interface_hierarchy!(
    VisualTreeHelper,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for VisualTreeHelper {}
unsafe impl ::core::marker::Sync for VisualTreeHelper {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct XamlCompositionBrushBase(::windows_core::IUnknown);
impl XamlCompositionBrushBase {
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
    pub fn Opacity(&self) -> ::windows_core::Result<f64> {
        let this = &::windows_core::ComInterface::cast::<IBrush>(self)?;
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
        let this = &::windows_core::ComInterface::cast::<IBrush>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetOpacity)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn Transform(&self) -> ::windows_core::Result<Transform> {
        let this = &::windows_core::ComInterface::cast::<IBrush>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Transform)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetTransform<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<Transform>,
    {
        let this = &::windows_core::ComInterface::cast::<IBrush>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetTransform)(
                ::windows_core::Interface::as_raw(this),
                value.try_into_param()?.abi(),
            )
            .ok()
        }
    }
    pub fn RelativeTransform(&self) -> ::windows_core::Result<Transform> {
        let this = &::windows_core::ComInterface::cast::<IBrush>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RelativeTransform)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetRelativeTransform<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<Transform>,
    {
        let this = &::windows_core::ComInterface::cast::<IBrush>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetRelativeTransform)(
                ::windows_core::Interface::as_raw(this),
                value.try_into_param()?.abi(),
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
        let this = &::windows_core::ComInterface::cast::<IBrushOverrides>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).PopulatePropertyInfoOverride)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(propertyname),
                animationpropertyinfo.into_param().abi(),
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
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn FallbackColor(&self) -> ::windows_core::Result<::windows::UI::Color> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FallbackColor)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn SetFallbackColor(&self, value: ::windows::UI::Color) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetFallbackColor)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub unsafe fn CreateInstance<P0>(
        baseinterface: P0,
        innerinterface: &mut ::core::option::Option<::windows_core::IInspectable>,
    ) -> ::windows_core::Result<XamlCompositionBrushBase>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        Self::IXamlCompositionBrushBaseFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstance)(
                ::windows_core::Interface::as_raw(this),
                baseinterface.into_param().abi(),
                innerinterface as *mut _ as _,
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn OnConnected(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IXamlCompositionBrushBaseOverrides>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).OnConnected)(
                ::windows_core::Interface::as_raw(this),
            )
            .ok()
        }
    }
    pub fn OnDisconnected(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IXamlCompositionBrushBaseOverrides>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).OnDisconnected)(
                ::windows_core::Interface::as_raw(this),
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Composition\"`"]
    #[cfg(feature = "Microsoft_UI_Composition")]
    pub fn CompositionBrush(
        &self,
    ) -> ::windows_core::Result<super::super::Composition::CompositionBrush> {
        let this = &::windows_core::ComInterface::cast::<IXamlCompositionBrushBaseProtected>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CompositionBrush)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Composition\"`"]
    #[cfg(feature = "Microsoft_UI_Composition")]
    pub fn SetCompositionBrush<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::Composition::CompositionBrush>,
    {
        let this = &::windows_core::ComInterface::cast::<IXamlCompositionBrushBaseProtected>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetCompositionBrush)(
                ::windows_core::Interface::as_raw(this),
                value.try_into_param()?.abi(),
            )
            .ok()
        }
    }
    pub fn FallbackColorProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IXamlCompositionBrushBaseStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FallbackColorProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IXamlCompositionBrushBaseFactory<
        R,
        F: FnOnce(&IXamlCompositionBrushBaseFactory) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<
            XamlCompositionBrushBase,
            IXamlCompositionBrushBaseFactory,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IXamlCompositionBrushBaseStatics<
        R,
        F: FnOnce(&IXamlCompositionBrushBaseStatics) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<
            XamlCompositionBrushBase,
            IXamlCompositionBrushBaseStatics,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for XamlCompositionBrushBase {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for XamlCompositionBrushBase {
    type Vtable = IXamlCompositionBrushBase_Vtbl;
}
unsafe impl ::windows_core::ComInterface for XamlCompositionBrushBase {
    const IID: ::windows_core::GUID =
        <IXamlCompositionBrushBase as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for XamlCompositionBrushBase {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.XamlCompositionBrushBase";
}
::windows_core::imp::interface_hierarchy!(
    XamlCompositionBrushBase,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
#[cfg(feature = "Microsoft_UI_Composition")]
impl ::windows_core::CanTryInto<super::super::Composition::IAnimationObject>
    for XamlCompositionBrushBase
{
}
impl ::windows_core::CanTryInto<Brush> for XamlCompositionBrushBase {}
impl ::windows_core::CanTryInto<super::DependencyObject> for XamlCompositionBrushBase {}
unsafe impl ::core::marker::Send for XamlCompositionBrushBase {}
unsafe impl ::core::marker::Sync for XamlCompositionBrushBase {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct XamlLight(::windows_core::IUnknown);
impl XamlLight {
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
    pub unsafe fn CreateInstance<P0>(
        baseinterface: P0,
        innerinterface: &mut ::core::option::Option<::windows_core::IInspectable>,
    ) -> ::windows_core::Result<XamlLight>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        Self::IXamlLightFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstance)(
                ::windows_core::Interface::as_raw(this),
                baseinterface.into_param().abi(),
                innerinterface as *mut _ as _,
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn GetId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IXamlLightOverrides>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetId)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn OnConnected<P0>(&self, newelement: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::UIElement>,
    {
        let this = &::windows_core::ComInterface::cast::<IXamlLightOverrides>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).OnConnected)(
                ::windows_core::Interface::as_raw(this),
                newelement.try_into_param()?.abi(),
            )
            .ok()
        }
    }
    pub fn OnDisconnected<P0>(&self, oldelement: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::UIElement>,
    {
        let this = &::windows_core::ComInterface::cast::<IXamlLightOverrides>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).OnDisconnected)(
                ::windows_core::Interface::as_raw(this),
                oldelement.try_into_param()?.abi(),
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Composition\"`"]
    #[cfg(feature = "Microsoft_UI_Composition")]
    pub fn CompositionLight(
        &self,
    ) -> ::windows_core::Result<super::super::Composition::CompositionLight> {
        let this = &::windows_core::ComInterface::cast::<IXamlLightProtected>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CompositionLight)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Composition\"`"]
    #[cfg(feature = "Microsoft_UI_Composition")]
    pub fn SetCompositionLight<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::Composition::CompositionLight>,
    {
        let this = &::windows_core::ComInterface::cast::<IXamlLightProtected>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetCompositionLight)(
                ::windows_core::Interface::as_raw(this),
                value.try_into_param()?.abi(),
            )
            .ok()
        }
    }
    pub fn AddTargetElement<P0>(
        lightid: &::windows_core::HSTRING,
        element: P0,
    ) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::UIElement>,
    {
        Self::IXamlLightStatics(|this| unsafe {
            (::windows_core::Interface::vtable(this).AddTargetElement)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(lightid),
                element.try_into_param()?.abi(),
            )
            .ok()
        })
    }
    pub fn RemoveTargetElement<P0>(
        lightid: &::windows_core::HSTRING,
        element: P0,
    ) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::UIElement>,
    {
        Self::IXamlLightStatics(|this| unsafe {
            (::windows_core::Interface::vtable(this).RemoveTargetElement)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(lightid),
                element.try_into_param()?.abi(),
            )
            .ok()
        })
    }
    pub fn AddTargetBrush<P0>(
        lightid: &::windows_core::HSTRING,
        brush: P0,
    ) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<Brush>,
    {
        Self::IXamlLightStatics(|this| unsafe {
            (::windows_core::Interface::vtable(this).AddTargetBrush)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(lightid),
                brush.try_into_param()?.abi(),
            )
            .ok()
        })
    }
    pub fn RemoveTargetBrush<P0>(
        lightid: &::windows_core::HSTRING,
        brush: P0,
    ) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<Brush>,
    {
        Self::IXamlLightStatics(|this| unsafe {
            (::windows_core::Interface::vtable(this).RemoveTargetBrush)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(lightid),
                brush.try_into_param()?.abi(),
            )
            .ok()
        })
    }
    #[doc(hidden)]
    pub fn IXamlLightFactory<R, F: FnOnce(&IXamlLightFactory) -> ::windows_core::Result<R>>(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<XamlLight, IXamlLightFactory> =
            ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IXamlLightStatics<R, F: FnOnce(&IXamlLightStatics) -> ::windows_core::Result<R>>(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<XamlLight, IXamlLightStatics> =
            ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for XamlLight {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for XamlLight {
    type Vtable = IXamlLight_Vtbl;
}
unsafe impl ::windows_core::ComInterface for XamlLight {
    const IID: ::windows_core::GUID = <IXamlLight as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for XamlLight {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.XamlLight";
}
::windows_core::imp::interface_hierarchy!(
    XamlLight,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<super::DependencyObject> for XamlLight {}
unsafe impl ::core::marker::Send for XamlLight {}
unsafe impl ::core::marker::Sync for XamlLight {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AlignmentX(pub i32);
impl AlignmentX {
    pub const Left: Self = Self(0i32);
    pub const Center: Self = Self(1i32);
    pub const Right: Self = Self(2i32);
}
impl ::core::marker::Copy for AlignmentX {}
impl ::core::clone::Clone for AlignmentX {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AlignmentX {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for AlignmentX {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for AlignmentX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AlignmentX").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for AlignmentX {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::from_slice(
            b"enum(Microsoft.UI.Xaml.Media.AlignmentX;i4)",
        );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AlignmentY(pub i32);
impl AlignmentY {
    pub const Top: Self = Self(0i32);
    pub const Center: Self = Self(1i32);
    pub const Bottom: Self = Self(2i32);
}
impl ::core::marker::Copy for AlignmentY {}
impl ::core::clone::Clone for AlignmentY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AlignmentY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for AlignmentY {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for AlignmentY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AlignmentY").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for AlignmentY {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::from_slice(
            b"enum(Microsoft.UI.Xaml.Media.AlignmentY;i4)",
        );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct BrushMappingMode(pub i32);
impl BrushMappingMode {
    pub const Absolute: Self = Self(0i32);
    pub const RelativeToBoundingBox: Self = Self(1i32);
}
impl ::core::marker::Copy for BrushMappingMode {}
impl ::core::clone::Clone for BrushMappingMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for BrushMappingMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for BrushMappingMode {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for BrushMappingMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BrushMappingMode").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for BrushMappingMode {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::from_slice(
            b"enum(Microsoft.UI.Xaml.Media.BrushMappingMode;i4)",
        );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ColorInterpolationMode(pub i32);
impl ColorInterpolationMode {
    pub const ScRgbLinearInterpolation: Self = Self(0i32);
    pub const SRgbLinearInterpolation: Self = Self(1i32);
}
impl ::core::marker::Copy for ColorInterpolationMode {}
impl ::core::clone::Clone for ColorInterpolationMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ColorInterpolationMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for ColorInterpolationMode {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for ColorInterpolationMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ColorInterpolationMode").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ColorInterpolationMode {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::from_slice(
            b"enum(Microsoft.UI.Xaml.Media.ColorInterpolationMode;i4)",
        );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ElementCompositeMode(pub i32);
impl ElementCompositeMode {
    pub const Inherit: Self = Self(0i32);
    pub const SourceOver: Self = Self(1i32);
    pub const MinBlend: Self = Self(2i32);
}
impl ::core::marker::Copy for ElementCompositeMode {}
impl ::core::clone::Clone for ElementCompositeMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ElementCompositeMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for ElementCompositeMode {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for ElementCompositeMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ElementCompositeMode").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ElementCompositeMode {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::from_slice(
            b"enum(Microsoft.UI.Xaml.Media.ElementCompositeMode;i4)",
        );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FastPlayFallbackBehaviour(pub i32);
impl FastPlayFallbackBehaviour {
    pub const Skip: Self = Self(0i32);
    pub const Hide: Self = Self(1i32);
    pub const Disable: Self = Self(2i32);
}
impl ::core::marker::Copy for FastPlayFallbackBehaviour {}
impl ::core::clone::Clone for FastPlayFallbackBehaviour {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FastPlayFallbackBehaviour {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for FastPlayFallbackBehaviour {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for FastPlayFallbackBehaviour {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FastPlayFallbackBehaviour").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for FastPlayFallbackBehaviour {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::from_slice(
            b"enum(Microsoft.UI.Xaml.Media.FastPlayFallbackBehaviour;i4)",
        );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FillRule(pub i32);
impl FillRule {
    pub const EvenOdd: Self = Self(0i32);
    pub const Nonzero: Self = Self(1i32);
}
impl ::core::marker::Copy for FillRule {}
impl ::core::clone::Clone for FillRule {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FillRule {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for FillRule {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for FillRule {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FillRule").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for FillRule {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::from_slice(b"enum(Microsoft.UI.Xaml.Media.FillRule;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct GradientSpreadMethod(pub i32);
impl GradientSpreadMethod {
    pub const Pad: Self = Self(0i32);
    pub const Reflect: Self = Self(1i32);
    pub const Repeat: Self = Self(2i32);
}
impl ::core::marker::Copy for GradientSpreadMethod {}
impl ::core::clone::Clone for GradientSpreadMethod {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GradientSpreadMethod {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for GradientSpreadMethod {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for GradientSpreadMethod {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GradientSpreadMethod").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for GradientSpreadMethod {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::from_slice(
            b"enum(Microsoft.UI.Xaml.Media.GradientSpreadMethod;i4)",
        );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct LoadedImageSourceLoadStatus(pub i32);
impl LoadedImageSourceLoadStatus {
    pub const Success: Self = Self(0i32);
    pub const NetworkError: Self = Self(1i32);
    pub const InvalidFormat: Self = Self(2i32);
    pub const Other: Self = Self(3i32);
}
impl ::core::marker::Copy for LoadedImageSourceLoadStatus {}
impl ::core::clone::Clone for LoadedImageSourceLoadStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for LoadedImageSourceLoadStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for LoadedImageSourceLoadStatus {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for LoadedImageSourceLoadStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LoadedImageSourceLoadStatus").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for LoadedImageSourceLoadStatus {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::from_slice(
            b"enum(Microsoft.UI.Xaml.Media.LoadedImageSourceLoadStatus;i4)",
        );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PenLineCap(pub i32);
impl PenLineCap {
    pub const Flat: Self = Self(0i32);
    pub const Square: Self = Self(1i32);
    pub const Round: Self = Self(2i32);
    pub const Triangle: Self = Self(3i32);
}
impl ::core::marker::Copy for PenLineCap {}
impl ::core::clone::Clone for PenLineCap {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PenLineCap {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for PenLineCap {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for PenLineCap {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PenLineCap").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for PenLineCap {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::from_slice(
            b"enum(Microsoft.UI.Xaml.Media.PenLineCap;i4)",
        );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PenLineJoin(pub i32);
impl PenLineJoin {
    pub const Miter: Self = Self(0i32);
    pub const Bevel: Self = Self(1i32);
    pub const Round: Self = Self(2i32);
}
impl ::core::marker::Copy for PenLineJoin {}
impl ::core::clone::Clone for PenLineJoin {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PenLineJoin {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for PenLineJoin {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for PenLineJoin {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PenLineJoin").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for PenLineJoin {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::from_slice(
            b"enum(Microsoft.UI.Xaml.Media.PenLineJoin;i4)",
        );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct Stretch(pub i32);
impl Stretch {
    pub const None: Self = Self(0i32);
    pub const Fill: Self = Self(1i32);
    pub const Uniform: Self = Self(2i32);
    pub const UniformToFill: Self = Self(3i32);
}
impl ::core::marker::Copy for Stretch {}
impl ::core::clone::Clone for Stretch {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for Stretch {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for Stretch {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for Stretch {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Stretch").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for Stretch {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::from_slice(b"enum(Microsoft.UI.Xaml.Media.Stretch;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct StyleSimulations(pub i32);
impl StyleSimulations {
    pub const None: Self = Self(0i32);
    pub const BoldSimulation: Self = Self(1i32);
    pub const ItalicSimulation: Self = Self(2i32);
    pub const BoldItalicSimulation: Self = Self(3i32);
}
impl ::core::marker::Copy for StyleSimulations {}
impl ::core::clone::Clone for StyleSimulations {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for StyleSimulations {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for StyleSimulations {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for StyleSimulations {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StyleSimulations").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for StyleSimulations {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::from_slice(
            b"enum(Microsoft.UI.Xaml.Media.StyleSimulations;i4)",
        );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SweepDirection(pub i32);
impl SweepDirection {
    pub const Counterclockwise: Self = Self(0i32);
    pub const Clockwise: Self = Self(1i32);
}
impl ::core::marker::Copy for SweepDirection {}
impl ::core::clone::Clone for SweepDirection {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SweepDirection {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for SweepDirection {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for SweepDirection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SweepDirection").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for SweepDirection {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::from_slice(
            b"enum(Microsoft.UI.Xaml.Media.SweepDirection;i4)",
        );
}
#[repr(C)]
pub struct Matrix {
    pub M11: f64,
    pub M12: f64,
    pub M21: f64,
    pub M22: f64,
    pub OffsetX: f64,
    pub OffsetY: f64,
}
impl ::core::marker::Copy for Matrix {}
impl ::core::clone::Clone for Matrix {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for Matrix {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("Matrix")
            .field("M11", &self.M11)
            .field("M12", &self.M12)
            .field("M21", &self.M21)
            .field("M22", &self.M22)
            .field("OffsetX", &self.OffsetX)
            .field("OffsetY", &self.OffsetY)
            .finish()
    }
}
impl ::windows_core::TypeKind for Matrix {
    type TypeKind = ::windows_core::CopyType;
}
impl ::windows_core::RuntimeType for Matrix {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::from_slice(
            b"struct(Microsoft.UI.Xaml.Media.Matrix;f8;f8;f8;f8;f8;f8)",
        );
}
impl ::core::cmp::PartialEq for Matrix {
    fn eq(&self, other: &Self) -> bool {
        self.M11 == other.M11
            && self.M12 == other.M12
            && self.M21 == other.M21
            && self.M22 == other.M22
            && self.OffsetX == other.OffsetX
            && self.OffsetY == other.OffsetY
    }
}
impl ::core::cmp::Eq for Matrix {}
impl ::core::default::Default for Matrix {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
