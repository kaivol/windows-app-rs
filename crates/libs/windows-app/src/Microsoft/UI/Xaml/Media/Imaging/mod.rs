#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IBitmapImage(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBitmapImage {
    type Vtable = IBitmapImage_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IBitmapImage {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x5cc29916_a411_5bc2_a3c5_a00d99a59da8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBitmapImage_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateOptions: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut BitmapCreateOptions,
    ) -> ::windows_core::HRESULT,
    pub SetCreateOptions: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: BitmapCreateOptions,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_Foundation")]
    pub UriSource: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    UriSource: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub SetUriSource: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    SetUriSource: usize,
    pub DecodePixelWidth: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut i32,
    ) -> ::windows_core::HRESULT,
    pub SetDecodePixelWidth: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: i32,
    ) -> ::windows_core::HRESULT,
    pub DecodePixelHeight: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut i32,
    ) -> ::windows_core::HRESULT,
    pub SetDecodePixelHeight: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: i32,
    ) -> ::windows_core::HRESULT,
    pub DecodePixelType: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut DecodePixelType,
    ) -> ::windows_core::HRESULT,
    pub SetDecodePixelType: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: DecodePixelType,
    ) -> ::windows_core::HRESULT,
    pub IsAnimatedBitmap: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub IsPlaying: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub AutoPlay: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetAutoPlay: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_Foundation")]
    pub DownloadProgress: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    DownloadProgress: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub RemoveDownloadProgress: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RemoveDownloadProgress: usize,
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
    pub Play: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IBitmapImageFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBitmapImageFactory {
    type Vtable = IBitmapImageFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IBitmapImageFactory {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xf037e0e9_f229_522e_95c9_da2211a14b05);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBitmapImageFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Windows_Foundation")]
    pub CreateInstanceWithUriSource: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        urisource: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    CreateInstanceWithUriSource: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IBitmapImageStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBitmapImageStatics {
    type Vtable = IBitmapImageStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IBitmapImageStatics {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x4bcf71a9_1897_51dc_8e3f_2c5c796d1cd9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBitmapImageStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateOptionsProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub UriSourceProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub DecodePixelWidthProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub DecodePixelHeightProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub DecodePixelTypeProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub IsAnimatedBitmapProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub IsPlayingProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub AutoPlayProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IBitmapSource(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBitmapSource {
    type Vtable = IBitmapSource_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IBitmapSource {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x8424269d_9b82_534f_8fea_af5b5ef96bf2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBitmapSource_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub PixelWidth: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut i32,
    ) -> ::windows_core::HRESULT,
    pub PixelHeight: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut i32,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_Storage_Streams")]
    pub SetSource: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        streamsource: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Storage_Streams"))]
    SetSource: usize,
    #[cfg(all(feature = "Windows_Foundation", feature = "Windows_Storage_Streams"))]
    pub SetSourceAsync: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        streamsource: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Windows_Foundation", feature = "Windows_Storage_Streams")))]
    SetSourceAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IBitmapSourceFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBitmapSourceFactory {
    type Vtable = IBitmapSourceFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IBitmapSourceFactory {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x0392f025_1868_5876_ad67_12e94a8da5bf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBitmapSourceFactory_Vtbl {
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
pub struct IBitmapSourceStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBitmapSourceStatics {
    type Vtable = IBitmapSourceStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IBitmapSourceStatics {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xefa3745e_4400_5f0b_bdc7_3f2911a3d719);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBitmapSourceStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub PixelWidthProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub PixelHeightProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDownloadProgressEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDownloadProgressEventArgs {
    type Vtable = IDownloadProgressEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDownloadProgressEventArgs {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x9a0ea80b_1a17_50d5_83f3_377738212619);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDownloadProgressEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Progress: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut i32,
    ) -> ::windows_core::HRESULT,
    pub SetProgress: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: i32,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IRenderTargetBitmap(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRenderTargetBitmap {
    type Vtable = IRenderTargetBitmap_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IRenderTargetBitmap {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xcf10407d_fa8b_57a3_9574_710529ae0b04);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRenderTargetBitmap_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub PixelWidth: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut i32,
    ) -> ::windows_core::HRESULT,
    pub PixelHeight: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut i32,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_Foundation")]
    pub RenderAsync: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RenderAsync: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub RenderToSizeAsync: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        scaledwidth: i32,
        scaledheight: i32,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RenderToSizeAsync: usize,
    #[cfg(all(feature = "Windows_Foundation", feature = "Windows_Storage_Streams"))]
    pub GetPixelsAsync: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Windows_Foundation", feature = "Windows_Storage_Streams")))]
    GetPixelsAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IRenderTargetBitmapStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRenderTargetBitmapStatics {
    type Vtable = IRenderTargetBitmapStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IRenderTargetBitmapStatics {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x83e822e4_9f84_5986_93b0_e4f7019c367d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRenderTargetBitmapStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub PixelWidthProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub PixelHeightProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ISoftwareBitmapSource(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISoftwareBitmapSource {
    type Vtable = ISoftwareBitmapSource_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ISoftwareBitmapSource {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xa6aca802_1f24_5a1e_bf08_781a85ed5511);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISoftwareBitmapSource_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Windows_Foundation", feature = "Windows_Graphics_Imaging"))]
    pub SetBitmapAsync: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        softwarebitmap: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Windows_Foundation", feature = "Windows_Graphics_Imaging")))]
    SetBitmapAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ISurfaceImageSource(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISurfaceImageSource {
    type Vtable = ISurfaceImageSource_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ISurfaceImageSource {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xac078d9c_d0e0_5ff9_b73e_98e82e4c8d36);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISurfaceImageSource_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ISurfaceImageSourceFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISurfaceImageSourceFactory {
    type Vtable = ISurfaceImageSourceFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ISurfaceImageSourceFactory {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x09a26ed2_11b3_5ef1_ac56_20d064ccca34);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISurfaceImageSourceFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateInstanceWithDimensions: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        pixelwidth: i32,
        pixelheight: i32,
        baseinterface: *mut ::core::ffi::c_void,
        innerinterface: *mut *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub CreateInstanceWithDimensionsAndOpacity:
        unsafe extern "system" fn(
            this: *mut ::core::ffi::c_void,
            pixelwidth: i32,
            pixelheight: i32,
            isopaque: bool,
            baseinterface: *mut ::core::ffi::c_void,
            innerinterface: *mut *mut ::core::ffi::c_void,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ISvgImageSource(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISvgImageSource {
    type Vtable = ISvgImageSource_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ISvgImageSource {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xd5b61d3c_b68d_53a2_b07b_ba6adfdd5887);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISvgImageSource_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Windows_Foundation")]
    pub UriSource: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    UriSource: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub SetUriSource: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    SetUriSource: usize,
    pub RasterizePixelWidth: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub SetRasterizePixelWidth: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
    ) -> ::windows_core::HRESULT,
    pub RasterizePixelHeight: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub SetRasterizePixelHeight: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_Foundation")]
    pub Opened: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    Opened: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub RemoveOpened: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RemoveOpened: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub OpenFailed: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    OpenFailed: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub RemoveOpenFailed: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RemoveOpenFailed: usize,
    #[cfg(all(feature = "Windows_Foundation", feature = "Windows_Storage_Streams"))]
    pub SetSourceAsync: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        streamsource: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Windows_Foundation", feature = "Windows_Storage_Streams")))]
    SetSourceAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ISvgImageSourceFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISvgImageSourceFactory {
    type Vtable = ISvgImageSourceFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ISvgImageSourceFactory {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x2f85673f_ac64_570d_9bda_94fa082eead9);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISvgImageSourceFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        baseinterface: *mut ::core::ffi::c_void,
        innerinterface: *mut *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_Foundation")]
    pub CreateInstanceWithUriSource: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        urisource: *mut ::core::ffi::c_void,
        baseinterface: *mut ::core::ffi::c_void,
        innerinterface: *mut *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    CreateInstanceWithUriSource: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ISvgImageSourceFailedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISvgImageSourceFailedEventArgs {
    type Vtable = ISvgImageSourceFailedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ISvgImageSourceFailedEventArgs {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x76e66278_7804_5439_a50d_14c5ba896714);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISvgImageSourceFailedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut SvgImageSourceLoadStatus,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ISvgImageSourceOpenedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISvgImageSourceOpenedEventArgs {
    type Vtable = ISvgImageSourceOpenedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ISvgImageSourceOpenedEventArgs {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x1c9860d5_38d0_5b21_8d48_072f1e254e39);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISvgImageSourceOpenedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ISvgImageSourceStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISvgImageSourceStatics {
    type Vtable = ISvgImageSourceStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ISvgImageSourceStatics {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xe3ad1068_f4c6_5513_a777_2980f0ba41bd);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISvgImageSourceStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub UriSourceProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub RasterizePixelWidthProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub RasterizePixelHeightProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IVirtualSurfaceImageSource(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IVirtualSurfaceImageSource {
    type Vtable = IVirtualSurfaceImageSource_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IVirtualSurfaceImageSource {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xe4ff96a6_fede_589c_a007_4178b53b6739);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVirtualSurfaceImageSource_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IVirtualSurfaceImageSourceFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IVirtualSurfaceImageSourceFactory {
    type Vtable = IVirtualSurfaceImageSourceFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IVirtualSurfaceImageSourceFactory {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x08490f2c_04a8_5031_b9c7_707060d7cd48);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVirtualSurfaceImageSourceFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateInstanceWithDimensions: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        pixelwidth: i32,
        pixelheight: i32,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub CreateInstanceWithDimensionsAndOpacity:
        unsafe extern "system" fn(
            this: *mut ::core::ffi::c_void,
            pixelwidth: i32,
            pixelheight: i32,
            isopaque: bool,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWriteableBitmap(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWriteableBitmap {
    type Vtable = IWriteableBitmap_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWriteableBitmap {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x78c824a9_0e43_5f1e_93bc_d046cca82b7e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWriteableBitmap_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Windows_Storage_Streams")]
    pub PixelBuffer: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Storage_Streams"))]
    PixelBuffer: usize,
    pub Invalidate:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWriteableBitmapFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWriteableBitmapFactory {
    type Vtable = IWriteableBitmapFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWriteableBitmapFactory {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x26e861d9_b080_512b_96c4_80050e7e08d1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWriteableBitmapFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateInstanceWithDimensions: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        pixelwidth: i32,
        pixelheight: i32,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IXamlRenderingBackgroundTask(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXamlRenderingBackgroundTask {
    type Vtable = IXamlRenderingBackgroundTask_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IXamlRenderingBackgroundTask {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x7807000c_a050_5121_ac74_3322d5358e39);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlRenderingBackgroundTask_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IXamlRenderingBackgroundTaskFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXamlRenderingBackgroundTaskFactory {
    type Vtable = IXamlRenderingBackgroundTaskFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IXamlRenderingBackgroundTaskFactory {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x205247a3_9ffe_599a_a21a_7181442a9d75);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlRenderingBackgroundTaskFactory_Vtbl {
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
pub struct IXamlRenderingBackgroundTaskOverrides(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXamlRenderingBackgroundTaskOverrides {
    type Vtable = IXamlRenderingBackgroundTaskOverrides_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IXamlRenderingBackgroundTaskOverrides {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x18733237_324b_57c0_89b2_5875472acc80);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlRenderingBackgroundTaskOverrides_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Windows_ApplicationModel_Background")]
    pub OnRun: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        taskinstance: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_ApplicationModel_Background"))]
    OnRun: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct BitmapImage(::windows_core::IUnknown);
impl BitmapImage {
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
            BitmapImage,
            ::windows_core::imp::IGenericFactory,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn CreateOptions(&self) -> ::windows_core::Result<BitmapCreateOptions> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateOptions)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetCreateOptions(&self, value: BitmapCreateOptions) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetCreateOptions)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn UriSource(&self) -> ::windows_core::Result<::windows::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).UriSource)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn SetUriSource<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows::Foundation::Uri>,
    {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetUriSource)(
                ::windows_core::Interface::as_raw(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn DecodePixelWidth(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DecodePixelWidth)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetDecodePixelWidth(&self, value: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetDecodePixelWidth)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn DecodePixelHeight(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DecodePixelHeight)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetDecodePixelHeight(&self, value: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetDecodePixelHeight)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn DecodePixelType(&self) -> ::windows_core::Result<DecodePixelType> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DecodePixelType)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetDecodePixelType(&self, value: DecodePixelType) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetDecodePixelType)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsAnimatedBitmap(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsAnimatedBitmap)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn IsPlaying(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsPlaying)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn AutoPlay(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AutoPlay)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetAutoPlay(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetAutoPlay)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn DownloadProgress<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<DownloadProgressEventHandler>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DownloadProgress)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveDownloadProgress(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveDownloadProgress)(
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
        P0: ::windows_core::IntoParam<super::super::RoutedEventHandler>,
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
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn ImageFailed<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::ExceptionRoutedEventHandler>,
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
    pub fn Play(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).Play)(::windows_core::Interface::as_raw(this))
                .ok()
        }
    }
    pub fn Stop(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).Stop)(::windows_core::Interface::as_raw(this))
                .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn CreateInstanceWithUriSource<P0>(urisource: P0) -> ::windows_core::Result<BitmapImage>
    where
        P0: ::windows_core::IntoParam<::windows::Foundation::Uri>,
    {
        Self::IBitmapImageFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstanceWithUriSource)(
                ::windows_core::Interface::as_raw(this),
                urisource.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn CreateOptionsProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IBitmapImageStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateOptionsProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn UriSourceProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IBitmapImageStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).UriSourceProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn DecodePixelWidthProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IBitmapImageStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DecodePixelWidthProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn DecodePixelHeightProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IBitmapImageStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DecodePixelHeightProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn DecodePixelTypeProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IBitmapImageStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DecodePixelTypeProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn IsAnimatedBitmapProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IBitmapImageStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsAnimatedBitmapProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn IsPlayingProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IBitmapImageStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsPlayingProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn AutoPlayProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IBitmapImageStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AutoPlayProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn PixelWidth(&self) -> ::windows_core::Result<i32> {
        let this = &::windows_core::ComInterface::cast::<IBitmapSource>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PixelWidth)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn PixelHeight(&self) -> ::windows_core::Result<i32> {
        let this = &::windows_core::ComInterface::cast::<IBitmapSource>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PixelHeight)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Storage_Streams\"`"]
    #[cfg(feature = "Windows_Storage_Streams")]
    pub fn SetSource<P0>(&self, streamsource: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<::windows::Storage::Streams::IRandomAccessStream>,
    {
        let this = &::windows_core::ComInterface::cast::<IBitmapSource>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetSource)(
                ::windows_core::Interface::as_raw(this),
                streamsource.try_into_param()?.abi(),
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`, `\"Windows_Storage_Streams\"`"]
    #[cfg(all(feature = "Windows_Foundation", feature = "Windows_Storage_Streams"))]
    pub fn SetSourceAsync<P0>(
        &self,
        streamsource: P0,
    ) -> ::windows_core::Result<::windows::Foundation::IAsyncAction>
    where
        P0: ::windows_core::TryIntoParam<::windows::Storage::Streams::IRandomAccessStream>,
    {
        let this = &::windows_core::ComInterface::cast::<IBitmapSource>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SetSourceAsync)(
                ::windows_core::Interface::as_raw(this),
                streamsource.try_into_param()?.abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn GetValue<P0>(&self, dp: P0) -> ::windows_core::Result<::windows_core::IInspectable>
    where
        P0: ::windows_core::IntoParam<super::super::DependencyProperty>,
    {
        let this = &::windows_core::ComInterface::cast::<super::super::IDependencyObject>(self)?;
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
        P0: ::windows_core::IntoParam<super::super::DependencyProperty>,
        P1: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = &::windows_core::ComInterface::cast::<super::super::IDependencyObject>(self)?;
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
        P0: ::windows_core::IntoParam<super::super::DependencyProperty>,
    {
        let this = &::windows_core::ComInterface::cast::<super::super::IDependencyObject>(self)?;
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
        P0: ::windows_core::IntoParam<super::super::DependencyProperty>,
    {
        let this = &::windows_core::ComInterface::cast::<super::super::IDependencyObject>(self)?;
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
        P0: ::windows_core::IntoParam<super::super::DependencyProperty>,
    {
        let this = &::windows_core::ComInterface::cast::<super::super::IDependencyObject>(self)?;
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
        P0: ::windows_core::IntoParam<super::super::DependencyProperty>,
        P1: ::windows_core::IntoParam<super::super::DependencyPropertyChangedCallback>,
    {
        let this = &::windows_core::ComInterface::cast::<super::super::IDependencyObject>(self)?;
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
        P0: ::windows_core::IntoParam<super::super::DependencyProperty>,
    {
        let this = &::windows_core::ComInterface::cast::<super::super::IDependencyObject>(self)?;
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
        let this = &::windows_core::ComInterface::cast::<super::super::IDependencyObject>(self)?;
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
    ) -> ::windows_core::Result<super::super::super::Dispatching::DispatcherQueue> {
        let this = &::windows_core::ComInterface::cast::<super::super::IDependencyObject>(self)?;
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
    pub fn IBitmapImageFactory<R, F: FnOnce(&IBitmapImageFactory) -> ::windows_core::Result<R>>(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<BitmapImage, IBitmapImageFactory> =
            ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IBitmapImageStatics<R, F: FnOnce(&IBitmapImageStatics) -> ::windows_core::Result<R>>(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<BitmapImage, IBitmapImageStatics> =
            ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for BitmapImage {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for BitmapImage {
    type Vtable = IBitmapImage_Vtbl;
}
unsafe impl ::windows_core::ComInterface for BitmapImage {
    const IID: ::windows_core::GUID = <IBitmapImage as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for BitmapImage {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.Imaging.BitmapImage";
}
::windows_core::imp::interface_hierarchy!(
    BitmapImage,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<BitmapSource> for BitmapImage {}
impl ::windows_core::CanTryInto<super::ImageSource> for BitmapImage {}
impl ::windows_core::CanTryInto<super::super::DependencyObject> for BitmapImage {}
unsafe impl ::core::marker::Send for BitmapImage {}
unsafe impl ::core::marker::Sync for BitmapImage {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct BitmapSource(::windows_core::IUnknown);
impl BitmapSource {
    pub fn PixelWidth(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PixelWidth)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn PixelHeight(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PixelHeight)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Storage_Streams\"`"]
    #[cfg(feature = "Windows_Storage_Streams")]
    pub fn SetSource<P0>(&self, streamsource: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<::windows::Storage::Streams::IRandomAccessStream>,
    {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetSource)(
                ::windows_core::Interface::as_raw(this),
                streamsource.try_into_param()?.abi(),
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`, `\"Windows_Storage_Streams\"`"]
    #[cfg(all(feature = "Windows_Foundation", feature = "Windows_Storage_Streams"))]
    pub fn SetSourceAsync<P0>(
        &self,
        streamsource: P0,
    ) -> ::windows_core::Result<::windows::Foundation::IAsyncAction>
    where
        P0: ::windows_core::TryIntoParam<::windows::Storage::Streams::IRandomAccessStream>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SetSourceAsync)(
                ::windows_core::Interface::as_raw(this),
                streamsource.try_into_param()?.abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub unsafe fn CreateInstance<P0>(
        baseinterface: P0,
        innerinterface: &mut ::core::option::Option<::windows_core::IInspectable>,
    ) -> ::windows_core::Result<BitmapSource>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        Self::IBitmapSourceFactory(|this| unsafe {
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
    pub fn PixelWidthProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IBitmapSourceStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PixelWidthProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn PixelHeightProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IBitmapSourceStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PixelHeightProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn GetValue<P0>(&self, dp: P0) -> ::windows_core::Result<::windows_core::IInspectable>
    where
        P0: ::windows_core::IntoParam<super::super::DependencyProperty>,
    {
        let this = &::windows_core::ComInterface::cast::<super::super::IDependencyObject>(self)?;
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
        P0: ::windows_core::IntoParam<super::super::DependencyProperty>,
        P1: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = &::windows_core::ComInterface::cast::<super::super::IDependencyObject>(self)?;
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
        P0: ::windows_core::IntoParam<super::super::DependencyProperty>,
    {
        let this = &::windows_core::ComInterface::cast::<super::super::IDependencyObject>(self)?;
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
        P0: ::windows_core::IntoParam<super::super::DependencyProperty>,
    {
        let this = &::windows_core::ComInterface::cast::<super::super::IDependencyObject>(self)?;
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
        P0: ::windows_core::IntoParam<super::super::DependencyProperty>,
    {
        let this = &::windows_core::ComInterface::cast::<super::super::IDependencyObject>(self)?;
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
        P0: ::windows_core::IntoParam<super::super::DependencyProperty>,
        P1: ::windows_core::IntoParam<super::super::DependencyPropertyChangedCallback>,
    {
        let this = &::windows_core::ComInterface::cast::<super::super::IDependencyObject>(self)?;
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
        P0: ::windows_core::IntoParam<super::super::DependencyProperty>,
    {
        let this = &::windows_core::ComInterface::cast::<super::super::IDependencyObject>(self)?;
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
        let this = &::windows_core::ComInterface::cast::<super::super::IDependencyObject>(self)?;
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
    ) -> ::windows_core::Result<super::super::super::Dispatching::DispatcherQueue> {
        let this = &::windows_core::ComInterface::cast::<super::super::IDependencyObject>(self)?;
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
    pub fn IBitmapSourceFactory<
        R,
        F: FnOnce(&IBitmapSourceFactory) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<BitmapSource, IBitmapSourceFactory> =
            ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IBitmapSourceStatics<
        R,
        F: FnOnce(&IBitmapSourceStatics) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<BitmapSource, IBitmapSourceStatics> =
            ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for BitmapSource {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for BitmapSource {
    type Vtable = IBitmapSource_Vtbl;
}
unsafe impl ::windows_core::ComInterface for BitmapSource {
    const IID: ::windows_core::GUID = <IBitmapSource as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for BitmapSource {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.Imaging.BitmapSource";
}
::windows_core::imp::interface_hierarchy!(
    BitmapSource,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<super::ImageSource> for BitmapSource {}
impl ::windows_core::CanTryInto<super::super::DependencyObject> for BitmapSource {}
unsafe impl ::core::marker::Send for BitmapSource {}
unsafe impl ::core::marker::Sync for BitmapSource {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct DownloadProgressEventArgs(::windows_core::IUnknown);
impl DownloadProgressEventArgs {
    pub fn Progress(&self) -> ::windows_core::Result<i32> {
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
    pub fn SetProgress(&self, value: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetProgress)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
}
impl ::windows_core::RuntimeType for DownloadProgressEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for DownloadProgressEventArgs {
    type Vtable = IDownloadProgressEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for DownloadProgressEventArgs {
    const IID: ::windows_core::GUID =
        <IDownloadProgressEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for DownloadProgressEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.Imaging.DownloadProgressEventArgs";
}
::windows_core::imp::interface_hierarchy!(
    DownloadProgressEventArgs,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for DownloadProgressEventArgs {}
unsafe impl ::core::marker::Sync for DownloadProgressEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct RenderTargetBitmap(::windows_core::IUnknown);
impl RenderTargetBitmap {
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
            RenderTargetBitmap,
            ::windows_core::imp::IGenericFactory,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn GetValue<P0>(&self, dp: P0) -> ::windows_core::Result<::windows_core::IInspectable>
    where
        P0: ::windows_core::IntoParam<super::super::DependencyProperty>,
    {
        let this = &::windows_core::ComInterface::cast::<super::super::IDependencyObject>(self)?;
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
        P0: ::windows_core::IntoParam<super::super::DependencyProperty>,
        P1: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = &::windows_core::ComInterface::cast::<super::super::IDependencyObject>(self)?;
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
        P0: ::windows_core::IntoParam<super::super::DependencyProperty>,
    {
        let this = &::windows_core::ComInterface::cast::<super::super::IDependencyObject>(self)?;
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
        P0: ::windows_core::IntoParam<super::super::DependencyProperty>,
    {
        let this = &::windows_core::ComInterface::cast::<super::super::IDependencyObject>(self)?;
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
        P0: ::windows_core::IntoParam<super::super::DependencyProperty>,
    {
        let this = &::windows_core::ComInterface::cast::<super::super::IDependencyObject>(self)?;
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
        P0: ::windows_core::IntoParam<super::super::DependencyProperty>,
        P1: ::windows_core::IntoParam<super::super::DependencyPropertyChangedCallback>,
    {
        let this = &::windows_core::ComInterface::cast::<super::super::IDependencyObject>(self)?;
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
        P0: ::windows_core::IntoParam<super::super::DependencyProperty>,
    {
        let this = &::windows_core::ComInterface::cast::<super::super::IDependencyObject>(self)?;
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
        let this = &::windows_core::ComInterface::cast::<super::super::IDependencyObject>(self)?;
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
    ) -> ::windows_core::Result<super::super::super::Dispatching::DispatcherQueue> {
        let this = &::windows_core::ComInterface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DispatcherQueue)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn PixelWidth(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PixelWidth)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn PixelHeight(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PixelHeight)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RenderAsync<P0>(
        &self,
        element: P0,
    ) -> ::windows_core::Result<::windows::Foundation::IAsyncAction>
    where
        P0: ::windows_core::TryIntoParam<super::super::UIElement>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RenderAsync)(
                ::windows_core::Interface::as_raw(this),
                element.try_into_param()?.abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RenderToSizeAsync<P0>(
        &self,
        element: P0,
        scaledwidth: i32,
        scaledheight: i32,
    ) -> ::windows_core::Result<::windows::Foundation::IAsyncAction>
    where
        P0: ::windows_core::TryIntoParam<super::super::UIElement>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RenderToSizeAsync)(
                ::windows_core::Interface::as_raw(this),
                element.try_into_param()?.abi(),
                scaledwidth,
                scaledheight,
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`, `\"Windows_Storage_Streams\"`"]
    #[cfg(all(feature = "Windows_Foundation", feature = "Windows_Storage_Streams"))]
    pub fn GetPixelsAsync(
        &self,
    ) -> ::windows_core::Result<
        ::windows::Foundation::IAsyncOperation<::windows::Storage::Streams::IBuffer>,
    > {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetPixelsAsync)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn PixelWidthProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IRenderTargetBitmapStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PixelWidthProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn PixelHeightProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IRenderTargetBitmapStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PixelHeightProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IRenderTargetBitmapStatics<
        R,
        F: FnOnce(&IRenderTargetBitmapStatics) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<
            RenderTargetBitmap,
            IRenderTargetBitmapStatics,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for RenderTargetBitmap {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for RenderTargetBitmap {
    type Vtable = IRenderTargetBitmap_Vtbl;
}
unsafe impl ::windows_core::ComInterface for RenderTargetBitmap {
    const IID: ::windows_core::GUID = <IRenderTargetBitmap as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for RenderTargetBitmap {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.Imaging.RenderTargetBitmap";
}
::windows_core::imp::interface_hierarchy!(
    RenderTargetBitmap,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<super::ImageSource> for RenderTargetBitmap {}
impl ::windows_core::CanTryInto<super::super::DependencyObject> for RenderTargetBitmap {}
unsafe impl ::core::marker::Send for RenderTargetBitmap {}
unsafe impl ::core::marker::Sync for RenderTargetBitmap {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct SoftwareBitmapSource(::windows_core::IUnknown);
impl SoftwareBitmapSource {
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
            SoftwareBitmapSource,
            ::windows_core::imp::IGenericFactory,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
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
    pub fn GetValue<P0>(&self, dp: P0) -> ::windows_core::Result<::windows_core::IInspectable>
    where
        P0: ::windows_core::IntoParam<super::super::DependencyProperty>,
    {
        let this = &::windows_core::ComInterface::cast::<super::super::IDependencyObject>(self)?;
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
        P0: ::windows_core::IntoParam<super::super::DependencyProperty>,
        P1: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = &::windows_core::ComInterface::cast::<super::super::IDependencyObject>(self)?;
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
        P0: ::windows_core::IntoParam<super::super::DependencyProperty>,
    {
        let this = &::windows_core::ComInterface::cast::<super::super::IDependencyObject>(self)?;
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
        P0: ::windows_core::IntoParam<super::super::DependencyProperty>,
    {
        let this = &::windows_core::ComInterface::cast::<super::super::IDependencyObject>(self)?;
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
        P0: ::windows_core::IntoParam<super::super::DependencyProperty>,
    {
        let this = &::windows_core::ComInterface::cast::<super::super::IDependencyObject>(self)?;
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
        P0: ::windows_core::IntoParam<super::super::DependencyProperty>,
        P1: ::windows_core::IntoParam<super::super::DependencyPropertyChangedCallback>,
    {
        let this = &::windows_core::ComInterface::cast::<super::super::IDependencyObject>(self)?;
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
        P0: ::windows_core::IntoParam<super::super::DependencyProperty>,
    {
        let this = &::windows_core::ComInterface::cast::<super::super::IDependencyObject>(self)?;
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
        let this = &::windows_core::ComInterface::cast::<super::super::IDependencyObject>(self)?;
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
    ) -> ::windows_core::Result<super::super::super::Dispatching::DispatcherQueue> {
        let this = &::windows_core::ComInterface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DispatcherQueue)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`, `\"Windows_Graphics_Imaging\"`"]
    #[cfg(all(feature = "Windows_Foundation", feature = "Windows_Graphics_Imaging"))]
    pub fn SetBitmapAsync<P0>(
        &self,
        softwarebitmap: P0,
    ) -> ::windows_core::Result<::windows::Foundation::IAsyncAction>
    where
        P0: ::windows_core::IntoParam<::windows::Graphics::Imaging::SoftwareBitmap>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SetBitmapAsync)(
                ::windows_core::Interface::as_raw(this),
                softwarebitmap.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for SoftwareBitmapSource {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for SoftwareBitmapSource {
    type Vtable = ISoftwareBitmapSource_Vtbl;
}
unsafe impl ::windows_core::ComInterface for SoftwareBitmapSource {
    const IID: ::windows_core::GUID = <ISoftwareBitmapSource as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for SoftwareBitmapSource {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.Imaging.SoftwareBitmapSource";
}
::windows_core::imp::interface_hierarchy!(
    SoftwareBitmapSource,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
#[cfg(feature = "Windows_Foundation")]
impl ::windows_core::CanTryInto<::windows::Foundation::IClosable> for SoftwareBitmapSource {}
impl ::windows_core::CanTryInto<super::ImageSource> for SoftwareBitmapSource {}
impl ::windows_core::CanTryInto<super::super::DependencyObject> for SoftwareBitmapSource {}
unsafe impl ::core::marker::Send for SoftwareBitmapSource {}
unsafe impl ::core::marker::Sync for SoftwareBitmapSource {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct SurfaceImageSource(::windows_core::IUnknown);
impl SurfaceImageSource {
    pub fn GetValue<P0>(&self, dp: P0) -> ::windows_core::Result<::windows_core::IInspectable>
    where
        P0: ::windows_core::IntoParam<super::super::DependencyProperty>,
    {
        let this = &::windows_core::ComInterface::cast::<super::super::IDependencyObject>(self)?;
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
        P0: ::windows_core::IntoParam<super::super::DependencyProperty>,
        P1: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = &::windows_core::ComInterface::cast::<super::super::IDependencyObject>(self)?;
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
        P0: ::windows_core::IntoParam<super::super::DependencyProperty>,
    {
        let this = &::windows_core::ComInterface::cast::<super::super::IDependencyObject>(self)?;
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
        P0: ::windows_core::IntoParam<super::super::DependencyProperty>,
    {
        let this = &::windows_core::ComInterface::cast::<super::super::IDependencyObject>(self)?;
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
        P0: ::windows_core::IntoParam<super::super::DependencyProperty>,
    {
        let this = &::windows_core::ComInterface::cast::<super::super::IDependencyObject>(self)?;
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
        P0: ::windows_core::IntoParam<super::super::DependencyProperty>,
        P1: ::windows_core::IntoParam<super::super::DependencyPropertyChangedCallback>,
    {
        let this = &::windows_core::ComInterface::cast::<super::super::IDependencyObject>(self)?;
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
        P0: ::windows_core::IntoParam<super::super::DependencyProperty>,
    {
        let this = &::windows_core::ComInterface::cast::<super::super::IDependencyObject>(self)?;
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
        let this = &::windows_core::ComInterface::cast::<super::super::IDependencyObject>(self)?;
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
    ) -> ::windows_core::Result<super::super::super::Dispatching::DispatcherQueue> {
        let this = &::windows_core::ComInterface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DispatcherQueue)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub unsafe fn CreateInstanceWithDimensions<P0>(
        pixelwidth: i32,
        pixelheight: i32,
        baseinterface: P0,
        innerinterface: &mut ::core::option::Option<::windows_core::IInspectable>,
    ) -> ::windows_core::Result<SurfaceImageSource>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        Self::ISurfaceImageSourceFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstanceWithDimensions)(
                ::windows_core::Interface::as_raw(this),
                pixelwidth,
                pixelheight,
                baseinterface.into_param().abi(),
                innerinterface as *mut _ as _,
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub unsafe fn CreateInstanceWithDimensionsAndOpacity<P0>(
        pixelwidth: i32,
        pixelheight: i32,
        isopaque: bool,
        baseinterface: P0,
        innerinterface: &mut ::core::option::Option<::windows_core::IInspectable>,
    ) -> ::windows_core::Result<SurfaceImageSource>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        Self::ISurfaceImageSourceFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstanceWithDimensionsAndOpacity)(
                ::windows_core::Interface::as_raw(this),
                pixelwidth,
                pixelheight,
                isopaque,
                baseinterface.into_param().abi(),
                innerinterface as *mut _ as _,
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ISurfaceImageSourceFactory<
        R,
        F: FnOnce(&ISurfaceImageSourceFactory) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<
            SurfaceImageSource,
            ISurfaceImageSourceFactory,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for SurfaceImageSource {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for SurfaceImageSource {
    type Vtable = ISurfaceImageSource_Vtbl;
}
unsafe impl ::windows_core::ComInterface for SurfaceImageSource {
    const IID: ::windows_core::GUID = <ISurfaceImageSource as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for SurfaceImageSource {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.Imaging.SurfaceImageSource";
}
::windows_core::imp::interface_hierarchy!(
    SurfaceImageSource,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<super::ImageSource> for SurfaceImageSource {}
impl ::windows_core::CanTryInto<super::super::DependencyObject> for SurfaceImageSource {}
unsafe impl ::core::marker::Send for SurfaceImageSource {}
unsafe impl ::core::marker::Sync for SurfaceImageSource {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct SvgImageSource(::windows_core::IUnknown);
impl SvgImageSource {
    pub fn GetValue<P0>(&self, dp: P0) -> ::windows_core::Result<::windows_core::IInspectable>
    where
        P0: ::windows_core::IntoParam<super::super::DependencyProperty>,
    {
        let this = &::windows_core::ComInterface::cast::<super::super::IDependencyObject>(self)?;
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
        P0: ::windows_core::IntoParam<super::super::DependencyProperty>,
        P1: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = &::windows_core::ComInterface::cast::<super::super::IDependencyObject>(self)?;
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
        P0: ::windows_core::IntoParam<super::super::DependencyProperty>,
    {
        let this = &::windows_core::ComInterface::cast::<super::super::IDependencyObject>(self)?;
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
        P0: ::windows_core::IntoParam<super::super::DependencyProperty>,
    {
        let this = &::windows_core::ComInterface::cast::<super::super::IDependencyObject>(self)?;
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
        P0: ::windows_core::IntoParam<super::super::DependencyProperty>,
    {
        let this = &::windows_core::ComInterface::cast::<super::super::IDependencyObject>(self)?;
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
        P0: ::windows_core::IntoParam<super::super::DependencyProperty>,
        P1: ::windows_core::IntoParam<super::super::DependencyPropertyChangedCallback>,
    {
        let this = &::windows_core::ComInterface::cast::<super::super::IDependencyObject>(self)?;
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
        P0: ::windows_core::IntoParam<super::super::DependencyProperty>,
    {
        let this = &::windows_core::ComInterface::cast::<super::super::IDependencyObject>(self)?;
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
        let this = &::windows_core::ComInterface::cast::<super::super::IDependencyObject>(self)?;
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
    ) -> ::windows_core::Result<super::super::super::Dispatching::DispatcherQueue> {
        let this = &::windows_core::ComInterface::cast::<super::super::IDependencyObject>(self)?;
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
    pub fn UriSource(&self) -> ::windows_core::Result<::windows::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).UriSource)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn SetUriSource<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows::Foundation::Uri>,
    {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetUriSource)(
                ::windows_core::Interface::as_raw(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn RasterizePixelWidth(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RasterizePixelWidth)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetRasterizePixelWidth(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetRasterizePixelWidth)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn RasterizePixelHeight(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RasterizePixelHeight)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetRasterizePixelHeight(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetRasterizePixelHeight)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn Opened<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<SvgImageSource, SvgImageSourceOpenedEventArgs>,
        >,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Opened)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveOpened(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveOpened)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn OpenFailed<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<SvgImageSource, SvgImageSourceFailedEventArgs>,
        >,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OpenFailed)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveOpenFailed(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveOpenFailed)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`, `\"Windows_Storage_Streams\"`"]
    #[cfg(all(feature = "Windows_Foundation", feature = "Windows_Storage_Streams"))]
    pub fn SetSourceAsync<P0>(
        &self,
        streamsource: P0,
    ) -> ::windows_core::Result<::windows::Foundation::IAsyncOperation<SvgImageSourceLoadStatus>>
    where
        P0: ::windows_core::TryIntoParam<::windows::Storage::Streams::IRandomAccessStream>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SetSourceAsync)(
                ::windows_core::Interface::as_raw(this),
                streamsource.try_into_param()?.abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub unsafe fn CreateInstance<P0>(
        baseinterface: P0,
        innerinterface: &mut ::core::option::Option<::windows_core::IInspectable>,
    ) -> ::windows_core::Result<SvgImageSource>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        Self::ISvgImageSourceFactory(|this| unsafe {
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
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub unsafe fn CreateInstanceWithUriSource<P0, P1>(
        urisource: P0,
        baseinterface: P1,
        innerinterface: &mut ::core::option::Option<::windows_core::IInspectable>,
    ) -> ::windows_core::Result<SvgImageSource>
    where
        P0: ::windows_core::IntoParam<::windows::Foundation::Uri>,
        P1: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        Self::ISvgImageSourceFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstanceWithUriSource)(
                ::windows_core::Interface::as_raw(this),
                urisource.into_param().abi(),
                baseinterface.into_param().abi(),
                innerinterface as *mut _ as _,
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn UriSourceProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::ISvgImageSourceStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).UriSourceProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn RasterizePixelWidthProperty() -> ::windows_core::Result<super::super::DependencyProperty>
    {
        Self::ISvgImageSourceStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RasterizePixelWidthProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn RasterizePixelHeightProperty() -> ::windows_core::Result<super::super::DependencyProperty>
    {
        Self::ISvgImageSourceStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RasterizePixelHeightProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ISvgImageSourceFactory<
        R,
        F: FnOnce(&ISvgImageSourceFactory) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<SvgImageSource, ISvgImageSourceFactory> =
            ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ISvgImageSourceStatics<
        R,
        F: FnOnce(&ISvgImageSourceStatics) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<SvgImageSource, ISvgImageSourceStatics> =
            ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for SvgImageSource {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for SvgImageSource {
    type Vtable = ISvgImageSource_Vtbl;
}
unsafe impl ::windows_core::ComInterface for SvgImageSource {
    const IID: ::windows_core::GUID = <ISvgImageSource as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for SvgImageSource {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.Imaging.SvgImageSource";
}
::windows_core::imp::interface_hierarchy!(
    SvgImageSource,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<super::ImageSource> for SvgImageSource {}
impl ::windows_core::CanTryInto<super::super::DependencyObject> for SvgImageSource {}
unsafe impl ::core::marker::Send for SvgImageSource {}
unsafe impl ::core::marker::Sync for SvgImageSource {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct SvgImageSourceFailedEventArgs(::windows_core::IUnknown);
impl SvgImageSourceFailedEventArgs {
    pub fn Status(&self) -> ::windows_core::Result<SvgImageSourceLoadStatus> {
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
impl ::windows_core::RuntimeType for SvgImageSourceFailedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for SvgImageSourceFailedEventArgs {
    type Vtable = ISvgImageSourceFailedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for SvgImageSourceFailedEventArgs {
    const IID: ::windows_core::GUID =
        <ISvgImageSourceFailedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for SvgImageSourceFailedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.Imaging.SvgImageSourceFailedEventArgs";
}
::windows_core::imp::interface_hierarchy!(
    SvgImageSourceFailedEventArgs,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for SvgImageSourceFailedEventArgs {}
unsafe impl ::core::marker::Sync for SvgImageSourceFailedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct SvgImageSourceOpenedEventArgs(::windows_core::IUnknown);
impl SvgImageSourceOpenedEventArgs {}
impl ::windows_core::RuntimeType for SvgImageSourceOpenedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for SvgImageSourceOpenedEventArgs {
    type Vtable = ISvgImageSourceOpenedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for SvgImageSourceOpenedEventArgs {
    const IID: ::windows_core::GUID =
        <ISvgImageSourceOpenedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for SvgImageSourceOpenedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.Imaging.SvgImageSourceOpenedEventArgs";
}
::windows_core::imp::interface_hierarchy!(
    SvgImageSourceOpenedEventArgs,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for SvgImageSourceOpenedEventArgs {}
unsafe impl ::core::marker::Sync for SvgImageSourceOpenedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct VirtualSurfaceImageSource(::windows_core::IUnknown);
impl VirtualSurfaceImageSource {
    pub fn GetValue<P0>(&self, dp: P0) -> ::windows_core::Result<::windows_core::IInspectable>
    where
        P0: ::windows_core::IntoParam<super::super::DependencyProperty>,
    {
        let this = &::windows_core::ComInterface::cast::<super::super::IDependencyObject>(self)?;
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
        P0: ::windows_core::IntoParam<super::super::DependencyProperty>,
        P1: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = &::windows_core::ComInterface::cast::<super::super::IDependencyObject>(self)?;
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
        P0: ::windows_core::IntoParam<super::super::DependencyProperty>,
    {
        let this = &::windows_core::ComInterface::cast::<super::super::IDependencyObject>(self)?;
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
        P0: ::windows_core::IntoParam<super::super::DependencyProperty>,
    {
        let this = &::windows_core::ComInterface::cast::<super::super::IDependencyObject>(self)?;
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
        P0: ::windows_core::IntoParam<super::super::DependencyProperty>,
    {
        let this = &::windows_core::ComInterface::cast::<super::super::IDependencyObject>(self)?;
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
        P0: ::windows_core::IntoParam<super::super::DependencyProperty>,
        P1: ::windows_core::IntoParam<super::super::DependencyPropertyChangedCallback>,
    {
        let this = &::windows_core::ComInterface::cast::<super::super::IDependencyObject>(self)?;
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
        P0: ::windows_core::IntoParam<super::super::DependencyProperty>,
    {
        let this = &::windows_core::ComInterface::cast::<super::super::IDependencyObject>(self)?;
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
        let this = &::windows_core::ComInterface::cast::<super::super::IDependencyObject>(self)?;
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
    ) -> ::windows_core::Result<super::super::super::Dispatching::DispatcherQueue> {
        let this = &::windows_core::ComInterface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DispatcherQueue)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn CreateInstanceWithDimensions(
        pixelwidth: i32,
        pixelheight: i32,
    ) -> ::windows_core::Result<VirtualSurfaceImageSource> {
        Self::IVirtualSurfaceImageSourceFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstanceWithDimensions)(
                ::windows_core::Interface::as_raw(this),
                pixelwidth,
                pixelheight,
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn CreateInstanceWithDimensionsAndOpacity(
        pixelwidth: i32,
        pixelheight: i32,
        isopaque: bool,
    ) -> ::windows_core::Result<VirtualSurfaceImageSource> {
        Self::IVirtualSurfaceImageSourceFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstanceWithDimensionsAndOpacity)(
                ::windows_core::Interface::as_raw(this),
                pixelwidth,
                pixelheight,
                isopaque,
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IVirtualSurfaceImageSourceFactory<
        R,
        F: FnOnce(&IVirtualSurfaceImageSourceFactory) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<
            VirtualSurfaceImageSource,
            IVirtualSurfaceImageSourceFactory,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for VirtualSurfaceImageSource {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for VirtualSurfaceImageSource {
    type Vtable = IVirtualSurfaceImageSource_Vtbl;
}
unsafe impl ::windows_core::ComInterface for VirtualSurfaceImageSource {
    const IID: ::windows_core::GUID =
        <IVirtualSurfaceImageSource as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for VirtualSurfaceImageSource {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.Imaging.VirtualSurfaceImageSource";
}
::windows_core::imp::interface_hierarchy!(
    VirtualSurfaceImageSource,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<SurfaceImageSource> for VirtualSurfaceImageSource {}
impl ::windows_core::CanTryInto<super::ImageSource> for VirtualSurfaceImageSource {}
impl ::windows_core::CanTryInto<super::super::DependencyObject> for VirtualSurfaceImageSource {}
unsafe impl ::core::marker::Send for VirtualSurfaceImageSource {}
unsafe impl ::core::marker::Sync for VirtualSurfaceImageSource {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct WriteableBitmap(::windows_core::IUnknown);
impl WriteableBitmap {
    pub fn PixelWidth(&self) -> ::windows_core::Result<i32> {
        let this = &::windows_core::ComInterface::cast::<IBitmapSource>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PixelWidth)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn PixelHeight(&self) -> ::windows_core::Result<i32> {
        let this = &::windows_core::ComInterface::cast::<IBitmapSource>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PixelHeight)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Storage_Streams\"`"]
    #[cfg(feature = "Windows_Storage_Streams")]
    pub fn SetSource<P0>(&self, streamsource: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<::windows::Storage::Streams::IRandomAccessStream>,
    {
        let this = &::windows_core::ComInterface::cast::<IBitmapSource>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetSource)(
                ::windows_core::Interface::as_raw(this),
                streamsource.try_into_param()?.abi(),
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`, `\"Windows_Storage_Streams\"`"]
    #[cfg(all(feature = "Windows_Foundation", feature = "Windows_Storage_Streams"))]
    pub fn SetSourceAsync<P0>(
        &self,
        streamsource: P0,
    ) -> ::windows_core::Result<::windows::Foundation::IAsyncAction>
    where
        P0: ::windows_core::TryIntoParam<::windows::Storage::Streams::IRandomAccessStream>,
    {
        let this = &::windows_core::ComInterface::cast::<IBitmapSource>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SetSourceAsync)(
                ::windows_core::Interface::as_raw(this),
                streamsource.try_into_param()?.abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn GetValue<P0>(&self, dp: P0) -> ::windows_core::Result<::windows_core::IInspectable>
    where
        P0: ::windows_core::IntoParam<super::super::DependencyProperty>,
    {
        let this = &::windows_core::ComInterface::cast::<super::super::IDependencyObject>(self)?;
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
        P0: ::windows_core::IntoParam<super::super::DependencyProperty>,
        P1: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = &::windows_core::ComInterface::cast::<super::super::IDependencyObject>(self)?;
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
        P0: ::windows_core::IntoParam<super::super::DependencyProperty>,
    {
        let this = &::windows_core::ComInterface::cast::<super::super::IDependencyObject>(self)?;
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
        P0: ::windows_core::IntoParam<super::super::DependencyProperty>,
    {
        let this = &::windows_core::ComInterface::cast::<super::super::IDependencyObject>(self)?;
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
        P0: ::windows_core::IntoParam<super::super::DependencyProperty>,
    {
        let this = &::windows_core::ComInterface::cast::<super::super::IDependencyObject>(self)?;
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
        P0: ::windows_core::IntoParam<super::super::DependencyProperty>,
        P1: ::windows_core::IntoParam<super::super::DependencyPropertyChangedCallback>,
    {
        let this = &::windows_core::ComInterface::cast::<super::super::IDependencyObject>(self)?;
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
        P0: ::windows_core::IntoParam<super::super::DependencyProperty>,
    {
        let this = &::windows_core::ComInterface::cast::<super::super::IDependencyObject>(self)?;
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
        let this = &::windows_core::ComInterface::cast::<super::super::IDependencyObject>(self)?;
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
    ) -> ::windows_core::Result<super::super::super::Dispatching::DispatcherQueue> {
        let this = &::windows_core::ComInterface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DispatcherQueue)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Storage_Streams\"`"]
    #[cfg(feature = "Windows_Storage_Streams")]
    pub fn PixelBuffer(&self) -> ::windows_core::Result<::windows::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PixelBuffer)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn Invalidate(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).Invalidate)(::windows_core::Interface::as_raw(
                this,
            ))
            .ok()
        }
    }
    pub fn CreateInstanceWithDimensions(
        pixelwidth: i32,
        pixelheight: i32,
    ) -> ::windows_core::Result<WriteableBitmap> {
        Self::IWriteableBitmapFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstanceWithDimensions)(
                ::windows_core::Interface::as_raw(this),
                pixelwidth,
                pixelheight,
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IWriteableBitmapFactory<
        R,
        F: FnOnce(&IWriteableBitmapFactory) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<WriteableBitmap, IWriteableBitmapFactory> =
            ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for WriteableBitmap {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for WriteableBitmap {
    type Vtable = IWriteableBitmap_Vtbl;
}
unsafe impl ::windows_core::ComInterface for WriteableBitmap {
    const IID: ::windows_core::GUID = <IWriteableBitmap as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for WriteableBitmap {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.Imaging.WriteableBitmap";
}
::windows_core::imp::interface_hierarchy!(
    WriteableBitmap,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<BitmapSource> for WriteableBitmap {}
impl ::windows_core::CanTryInto<super::ImageSource> for WriteableBitmap {}
impl ::windows_core::CanTryInto<super::super::DependencyObject> for WriteableBitmap {}
unsafe impl ::core::marker::Send for WriteableBitmap {}
unsafe impl ::core::marker::Sync for WriteableBitmap {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct XamlRenderingBackgroundTask(::windows_core::IUnknown);
impl XamlRenderingBackgroundTask {
    pub unsafe fn CreateInstance<P0>(
        baseinterface: P0,
        innerinterface: &mut ::core::option::Option<::windows_core::IInspectable>,
    ) -> ::windows_core::Result<XamlRenderingBackgroundTask>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        Self::IXamlRenderingBackgroundTaskFactory(|this| unsafe {
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
    #[doc = "Required features: `\"Windows_ApplicationModel_Background\"`"]
    #[cfg(feature = "Windows_ApplicationModel_Background")]
    pub fn OnRun<P0>(&self, taskinstance: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<
            ::windows::ApplicationModel::Background::IBackgroundTaskInstance,
        >,
    {
        let this =
            &::windows_core::ComInterface::cast::<IXamlRenderingBackgroundTaskOverrides>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).OnRun)(
                ::windows_core::Interface::as_raw(this),
                taskinstance.try_into_param()?.abi(),
            )
            .ok()
        }
    }
    #[doc(hidden)]
    pub fn IXamlRenderingBackgroundTaskFactory<
        R,
        F: FnOnce(&IXamlRenderingBackgroundTaskFactory) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<
            XamlRenderingBackgroundTask,
            IXamlRenderingBackgroundTaskFactory,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for XamlRenderingBackgroundTask {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for XamlRenderingBackgroundTask {
    type Vtable = IXamlRenderingBackgroundTask_Vtbl;
}
unsafe impl ::windows_core::ComInterface for XamlRenderingBackgroundTask {
    const IID: ::windows_core::GUID =
        <IXamlRenderingBackgroundTask as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for XamlRenderingBackgroundTask {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.Imaging.XamlRenderingBackgroundTask";
}
::windows_core::imp::interface_hierarchy!(
    XamlRenderingBackgroundTask,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for XamlRenderingBackgroundTask {}
unsafe impl ::core::marker::Sync for XamlRenderingBackgroundTask {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct BitmapCreateOptions(pub u32);
impl BitmapCreateOptions {
    pub const None: Self = Self(0u32);
    pub const IgnoreImageCache: Self = Self(8u32);
}
impl ::core::marker::Copy for BitmapCreateOptions {}
impl ::core::clone::Clone for BitmapCreateOptions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for BitmapCreateOptions {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for BitmapCreateOptions {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for BitmapCreateOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BitmapCreateOptions").field(&self.0).finish()
    }
}
impl BitmapCreateOptions {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for BitmapCreateOptions {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for BitmapCreateOptions {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for BitmapCreateOptions {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for BitmapCreateOptions {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for BitmapCreateOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::windows_core::RuntimeType for BitmapCreateOptions {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::from_slice(
            b"enum(Microsoft.UI.Xaml.Media.Imaging.BitmapCreateOptions;u4)",
        );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DecodePixelType(pub i32);
impl DecodePixelType {
    pub const Physical: Self = Self(0i32);
    pub const Logical: Self = Self(1i32);
}
impl ::core::marker::Copy for DecodePixelType {}
impl ::core::clone::Clone for DecodePixelType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DecodePixelType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for DecodePixelType {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for DecodePixelType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DecodePixelType").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for DecodePixelType {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::from_slice(
            b"enum(Microsoft.UI.Xaml.Media.Imaging.DecodePixelType;i4)",
        );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SvgImageSourceLoadStatus(pub i32);
impl SvgImageSourceLoadStatus {
    pub const Success: Self = Self(0i32);
    pub const NetworkError: Self = Self(1i32);
    pub const InvalidFormat: Self = Self(2i32);
    pub const Other: Self = Self(3i32);
}
impl ::core::marker::Copy for SvgImageSourceLoadStatus {}
impl ::core::clone::Clone for SvgImageSourceLoadStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SvgImageSourceLoadStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for SvgImageSourceLoadStatus {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for SvgImageSourceLoadStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SvgImageSourceLoadStatus").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for SvgImageSourceLoadStatus {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::from_slice(
            b"enum(Microsoft.UI.Xaml.Media.Imaging.SvgImageSourceLoadStatus;i4)",
        );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct DownloadProgressEventHandler(pub ::windows_core::IUnknown);
impl DownloadProgressEventHandler {
    pub fn new<
        F: FnMut(
                ::core::option::Option<&::windows_core::IInspectable>,
                ::core::option::Option<&DownloadProgressEventArgs>,
            ) -> ::windows_core::Result<()>
            + ::core::marker::Send
            + 'static,
    >(
        invoke: F,
    ) -> Self {
        let com = DownloadProgressEventHandlerBox::<F> {
            vtable: &DownloadProgressEventHandlerBox::<F>::VTABLE,
            count: ::windows_core::imp::RefCount::new(1),
            invoke,
        };
        unsafe { ::core::mem::transmute(::std::boxed::Box::new(com)) }
    }
    pub fn Invoke<P0, P1>(&self, sender: P0, e: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
        P1: ::windows_core::IntoParam<DownloadProgressEventArgs>,
    {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).Invoke)(
                ::windows_core::Interface::as_raw(this),
                sender.into_param().abi(),
                e.into_param().abi(),
            )
            .ok()
        }
    }
}
#[repr(C)]
struct DownloadProgressEventHandlerBox<
    F: FnMut(
            ::core::option::Option<&::windows_core::IInspectable>,
            ::core::option::Option<&DownloadProgressEventArgs>,
        ) -> ::windows_core::Result<()>
        + ::core::marker::Send
        + 'static,
> {
    vtable: *const DownloadProgressEventHandler_Vtbl,
    invoke: F,
    count: ::windows_core::imp::RefCount,
}
impl<
        F: FnMut(
                ::core::option::Option<&::windows_core::IInspectable>,
                ::core::option::Option<&DownloadProgressEventArgs>,
            ) -> ::windows_core::Result<()>
            + ::core::marker::Send
            + 'static,
    > DownloadProgressEventHandlerBox<F>
{
    const VTABLE: DownloadProgressEventHandler_Vtbl = DownloadProgressEventHandler_Vtbl {
        base__: ::windows_core::IUnknown_Vtbl {
            QueryInterface: Self::QueryInterface,
            AddRef: Self::AddRef,
            Release: Self::Release,
        },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn QueryInterface(
        this: *mut ::core::ffi::c_void,
        iid: *const ::windows_core::GUID,
        interface: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        if iid.is_null() || interface.is_null() {
            return ::windows_core::HRESULT(-2147467261);
        }
        *interface = if *iid == <DownloadProgressEventHandler as ::windows_core::ComInterface>::IID
            || *iid == <::windows_core::IUnknown as ::windows_core::ComInterface>::IID
            || *iid == <::windows_core::imp::IAgileObject as ::windows_core::ComInterface>::IID
        {
            &mut (*this).vtable as *mut _ as _
        } else {
            ::core::ptr::null_mut()
        };
        if (*interface).is_null() {
            ::windows_core::HRESULT(-2147467262)
        } else {
            (*this).count.add_ref();
            ::windows_core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: *mut ::core::ffi::c_void) -> u32 {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: *mut ::core::ffi::c_void) -> u32 {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            let _ = ::std::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(
        this: *mut ::core::ffi::c_void,
        sender: *mut ::core::ffi::c_void,
        e: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        ((*this).invoke)(
            ::windows_core::from_raw_borrowed(&sender),
            ::windows_core::from_raw_borrowed(&e),
        )
        .into()
    }
}
unsafe impl ::windows_core::Interface for DownloadProgressEventHandler {
    type Vtable = DownloadProgressEventHandler_Vtbl;
}
unsafe impl ::windows_core::ComInterface for DownloadProgressEventHandler {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x9a8e4af5_b124_5205_8ae9_3496e063c569);
}
impl ::windows_core::RuntimeType for DownloadProgressEventHandler {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct DownloadProgressEventHandler_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        sender: *mut ::core::ffi::c_void,
        e: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
