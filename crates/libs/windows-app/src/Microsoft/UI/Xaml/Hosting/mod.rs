#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDesktopWindowXamlSource(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDesktopWindowXamlSource {
    type Vtable = IDesktopWindowXamlSource_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDesktopWindowXamlSource {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x553af92c_1381_51d6_bee0_f34beb042ea8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDesktopWindowXamlSource_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Content: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetContent: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub HasFocus: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Microsoft_UI_Xaml_Media")]
    pub SystemBackdrop: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Microsoft_UI_Xaml_Media"))]
    SystemBackdrop: usize,
    #[cfg(feature = "Microsoft_UI_Xaml_Media")]
    pub SetSystemBackdrop: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Microsoft_UI_Xaml_Media"))]
    SetSystemBackdrop: usize,
    #[cfg(feature = "Microsoft_UI_Content")]
    pub SiteBridge: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Microsoft_UI_Content"))]
    SiteBridge: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub TakeFocusRequested: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    TakeFocusRequested: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub RemoveTakeFocusRequested: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RemoveTakeFocusRequested: usize,
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
    pub NavigateFocus: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        request: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub Initialize: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        parentwindowid: super::super::WindowId,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDesktopWindowXamlSourceFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDesktopWindowXamlSourceFactory {
    type Vtable = IDesktopWindowXamlSourceFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDesktopWindowXamlSourceFactory {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x7d2db617_14e7_5d49_aeec_ae10887e595d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDesktopWindowXamlSourceFactory_Vtbl {
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
pub struct IDesktopWindowXamlSourceGotFocusEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDesktopWindowXamlSourceGotFocusEventArgs {
    type Vtable = IDesktopWindowXamlSourceGotFocusEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDesktopWindowXamlSourceGotFocusEventArgs {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xcc63d863_2071_5f6b_aef9_c0ba35f3b8df);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDesktopWindowXamlSourceGotFocusEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Request: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDesktopWindowXamlSourceNative_Deprecated(::windows_core::IUnknown);
impl IDesktopWindowXamlSourceNative_Deprecated {
    #[doc = "Required features: `\"Windows_Win32_Foundation\"`"]
    #[cfg(feature = "Windows_Win32_Foundation")]
    pub unsafe fn AttachToWindow<P0>(&self, parentwnd: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows::Win32::Foundation::HWND>,
    {
        (::windows_core::Interface::vtable(self).AttachToWindow)(
            ::windows_core::Interface::as_raw(self),
            parentwnd.into_param().abi(),
        )
        .ok()
    }
    #[doc = "Required features: `\"Windows_Win32_Foundation\"`"]
    #[cfg(feature = "Windows_Win32_Foundation")]
    pub unsafe fn WindowHandle(
        &self,
        hwnd: *mut ::windows::Win32::Foundation::HWND,
    ) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).WindowHandle)(
            ::windows_core::Interface::as_raw(self),
            hwnd,
        )
        .ok()
    }
    #[doc = "Required features: `\"Windows_Win32_Foundation\"`, `\"Windows_Win32_UI_WindowsAndMessaging\"`"]
    #[cfg(all(
        feature = "Windows_Win32_Foundation",
        feature = "Windows_Win32_UI_WindowsAndMessaging"
    ))]
    pub unsafe fn PreTranslateMessage(
        &self,
        message: *const ::windows::Win32::UI::WindowsAndMessaging::MSG,
        result: *mut ::windows::Win32::Foundation::BOOL,
    ) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).PreTranslateMessage)(
            ::windows_core::Interface::as_raw(self),
            message,
            result,
        )
        .ok()
    }
}
::windows_core::imp::interface_hierarchy!(
    IDesktopWindowXamlSourceNative_Deprecated,
    ::windows_core::IUnknown
);
unsafe impl ::windows_core::Interface for IDesktopWindowXamlSourceNative_Deprecated {
    type Vtable = IDesktopWindowXamlSourceNative_Deprecated_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDesktopWindowXamlSourceNative_Deprecated {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x0aea2f26_facf_4588_8cf4_34555124db32);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDesktopWindowXamlSourceNative_Deprecated_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Windows_Win32_Foundation")]
    pub AttachToWindow: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        parentwnd: ::windows::Win32::Foundation::HWND,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Win32_Foundation"))]
    AttachToWindow: usize,
    #[cfg(feature = "Windows_Win32_Foundation")]
    pub WindowHandle: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        hwnd: *mut ::windows::Win32::Foundation::HWND,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Win32_Foundation"))]
    WindowHandle: usize,
    #[cfg(all(
        feature = "Windows_Win32_Foundation",
        feature = "Windows_Win32_UI_WindowsAndMessaging"
    ))]
    pub PreTranslateMessage: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        message: *const ::windows::Win32::UI::WindowsAndMessaging::MSG,
        result: *mut ::windows::Win32::Foundation::BOOL,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(all(
        feature = "Windows_Win32_Foundation",
        feature = "Windows_Win32_UI_WindowsAndMessaging"
    )))]
    PreTranslateMessage: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDesktopWindowXamlSourceTakeFocusRequestedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDesktopWindowXamlSourceTakeFocusRequestedEventArgs {
    type Vtable = IDesktopWindowXamlSourceTakeFocusRequestedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDesktopWindowXamlSourceTakeFocusRequestedEventArgs {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x4f5a0e2c_4ddc_5c03_939f_6f3bda560363);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDesktopWindowXamlSourceTakeFocusRequestedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Request: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IElementCompositionPreview(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IElementCompositionPreview {
    type Vtable = IElementCompositionPreview_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IElementCompositionPreview {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xc8ad1ef4_a93f_5a25_85bd_7c498d9856d3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IElementCompositionPreview_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IElementCompositionPreviewStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IElementCompositionPreviewStatics {
    type Vtable = IElementCompositionPreviewStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IElementCompositionPreviewStatics {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x84da5a6c_0cfa_532b_9b15_ccd986374342);
}
#[repr(C)]
#[doc(hidden)]
pub struct IElementCompositionPreviewStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Microsoft_UI_Composition")]
    pub GetElementVisual: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Microsoft_UI_Composition"))]
    GetElementVisual: usize,
    #[cfg(feature = "Microsoft_UI_Composition")]
    pub GetElementChildVisual: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Microsoft_UI_Composition"))]
    GetElementChildVisual: usize,
    #[cfg(feature = "Microsoft_UI_Composition")]
    pub SetElementChildVisual: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        visual: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Microsoft_UI_Composition"))]
    SetElementChildVisual: usize,
    #[cfg(all(
        feature = "Microsoft_UI_Composition",
        feature = "Microsoft_UI_Xaml_Controls"
    ))]
    pub GetScrollViewerManipulationPropertySet:
        unsafe extern "system" fn(
            this: *mut ::core::ffi::c_void,
            scrollviewer: *mut ::core::ffi::c_void,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT,
    #[cfg(not(all(
        feature = "Microsoft_UI_Composition",
        feature = "Microsoft_UI_Xaml_Controls"
    )))]
    GetScrollViewerManipulationPropertySet: usize,
    #[cfg(feature = "Microsoft_UI_Composition")]
    pub SetImplicitShowAnimation: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        animation: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Microsoft_UI_Composition"))]
    SetImplicitShowAnimation: usize,
    #[cfg(feature = "Microsoft_UI_Composition")]
    pub SetImplicitHideAnimation: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        animation: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Microsoft_UI_Composition"))]
    SetImplicitHideAnimation: usize,
    pub SetIsTranslationEnabled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Microsoft_UI_Composition")]
    pub GetPointerPositionPropertySet: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        targetelement: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Microsoft_UI_Composition"))]
    GetPointerPositionPropertySet: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IFindReferenceTargetsCallback(::windows_core::IUnknown);
impl IFindReferenceTargetsCallback {
    pub unsafe fn FoundTrackerTarget<P0>(&self, target: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IReferenceTrackerTarget>,
    {
        (::windows_core::Interface::vtable(self).FoundTrackerTarget)(
            ::windows_core::Interface::as_raw(self),
            target.into_param().abi(),
        )
        .ok()
    }
}
::windows_core::imp::interface_hierarchy!(IFindReferenceTargetsCallback, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFindReferenceTargetsCallback {
    type Vtable = IFindReferenceTargetsCallback_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IFindReferenceTargetsCallback {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x04b3486c_4687_4229_8d14_505ab584dd88);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFindReferenceTargetsCallback_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub FoundTrackerTarget: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        target: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IReferenceTracker(::windows_core::IUnknown);
impl IReferenceTracker {
    pub unsafe fn ConnectFromTrackerSource(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ConnectFromTrackerSource)(
            ::windows_core::Interface::as_raw(self),
        )
        .ok()
    }
    pub unsafe fn DisconnectFromTrackerSource(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DisconnectFromTrackerSource)(
            ::windows_core::Interface::as_raw(self),
        )
        .ok()
    }
    pub unsafe fn FindTrackerTargets<P0>(&self, callback: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IFindReferenceTargetsCallback>,
    {
        (::windows_core::Interface::vtable(self).FindTrackerTargets)(
            ::windows_core::Interface::as_raw(self),
            callback.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn GetReferenceTrackerManager(
        &self,
    ) -> ::windows_core::Result<IReferenceTrackerManager> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetReferenceTrackerManager)(
            ::windows_core::Interface::as_raw(self),
            &mut result__,
        )
        .from_abi(result__)
    }
    pub unsafe fn AddRefFromTrackerSource(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AddRefFromTrackerSource)(
            ::windows_core::Interface::as_raw(self),
        )
        .ok()
    }
    pub unsafe fn ReleaseFromTrackerSource(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ReleaseFromTrackerSource)(
            ::windows_core::Interface::as_raw(self),
        )
        .ok()
    }
    pub unsafe fn PegFromTrackerSource(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).PegFromTrackerSource)(
            ::windows_core::Interface::as_raw(self),
        )
        .ok()
    }
}
::windows_core::imp::interface_hierarchy!(IReferenceTracker, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IReferenceTracker {
    type Vtable = IReferenceTracker_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IReferenceTracker {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x11d3b13a_180e_4789_a8be_7712882893e6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IReferenceTracker_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub ConnectFromTrackerSource:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub DisconnectFromTrackerSource:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub FindTrackerTargets: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        callback: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub GetReferenceTrackerManager: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub AddRefFromTrackerSource:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub ReleaseFromTrackerSource:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub PegFromTrackerSource:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IReferenceTrackerExtension(::windows_core::IUnknown);
impl IReferenceTrackerExtension {}
::windows_core::imp::interface_hierarchy!(IReferenceTrackerExtension, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IReferenceTrackerExtension {
    type Vtable = IReferenceTrackerExtension_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IReferenceTrackerExtension {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x4e897caa_59d5_4613_8f8c_f7ebd1f399b0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IReferenceTrackerExtension_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IReferenceTrackerHost(::windows_core::IUnknown);
impl IReferenceTrackerHost {
    pub unsafe fn DisconnectUnusedReferenceSources(
        &self,
        options: __MIDL___MIDL_itf_microsoft2Eui2Examl2Ehosting2Ereferencetracker_0000_0004_0001,
    ) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DisconnectUnusedReferenceSources)(
            ::windows_core::Interface::as_raw(self),
            options,
        )
        .ok()
    }
    pub unsafe fn ReleaseDisconnectedReferenceSources(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ReleaseDisconnectedReferenceSources)(
            ::windows_core::Interface::as_raw(self),
        )
        .ok()
    }
    pub unsafe fn NotifyEndOfReferenceTrackingOnThread(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).NotifyEndOfReferenceTrackingOnThread)(
            ::windows_core::Interface::as_raw(self),
        )
        .ok()
    }
    pub unsafe fn GetTrackerTarget<P0>(
        &self,
        unknown: P0,
    ) -> ::windows_core::Result<IReferenceTrackerTarget>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetTrackerTarget)(
            ::windows_core::Interface::as_raw(self),
            unknown.into_param().abi(),
            &mut result__,
        )
        .from_abi(result__)
    }
    pub unsafe fn AddMemoryPressure(&self, bytesallocated: u64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AddMemoryPressure)(
            ::windows_core::Interface::as_raw(self),
            bytesallocated,
        )
        .ok()
    }
    pub unsafe fn RemoveMemoryPressure(&self, bytesallocated: u64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RemoveMemoryPressure)(
            ::windows_core::Interface::as_raw(self),
            bytesallocated,
        )
        .ok()
    }
}
::windows_core::imp::interface_hierarchy!(IReferenceTrackerHost, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IReferenceTrackerHost {
    type Vtable = IReferenceTrackerHost_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IReferenceTrackerHost {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x29a71c6a_3c42_4416_a39d_e2825a07a773);
}
#[repr(C)]
#[doc(hidden)]
pub struct IReferenceTrackerHost_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub DisconnectUnusedReferenceSources: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        options: __MIDL___MIDL_itf_microsoft2Eui2Examl2Ehosting2Ereferencetracker_0000_0004_0001,
    ) -> ::windows_core::HRESULT,
    pub ReleaseDisconnectedReferenceSources:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub NotifyEndOfReferenceTrackingOnThread:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetTrackerTarget: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        unknown: *mut ::core::ffi::c_void,
        newreference: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub AddMemoryPressure: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        bytesallocated: u64,
    ) -> ::windows_core::HRESULT,
    pub RemoveMemoryPressure: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        bytesallocated: u64,
    ) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IReferenceTrackerManager(::windows_core::IUnknown);
impl IReferenceTrackerManager {
    pub unsafe fn ReferenceTrackingStarted(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ReferenceTrackingStarted)(
            ::windows_core::Interface::as_raw(self),
        )
        .ok()
    }
    pub unsafe fn FindTrackerTargetsCompleted(&self, findfailed: u8) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).FindTrackerTargetsCompleted)(
            ::windows_core::Interface::as_raw(self),
            findfailed,
        )
        .ok()
    }
    pub unsafe fn ReferenceTrackingCompleted(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ReferenceTrackingCompleted)(
            ::windows_core::Interface::as_raw(self),
        )
        .ok()
    }
    pub unsafe fn SetReferenceTrackerHost<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IReferenceTrackerHost>,
    {
        (::windows_core::Interface::vtable(self).SetReferenceTrackerHost)(
            ::windows_core::Interface::as_raw(self),
            value.into_param().abi(),
        )
        .ok()
    }
}
::windows_core::imp::interface_hierarchy!(IReferenceTrackerManager, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IReferenceTrackerManager {
    type Vtable = IReferenceTrackerManager_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IReferenceTrackerManager {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x3cf184b4_7ccb_4dda_8455_7e6ce99a3298);
}
#[repr(C)]
#[doc(hidden)]
pub struct IReferenceTrackerManager_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub ReferenceTrackingStarted:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub FindTrackerTargetsCompleted: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        findfailed: u8,
    ) -> ::windows_core::HRESULT,
    pub ReferenceTrackingCompleted:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetReferenceTrackerHost: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IReferenceTrackerTarget(::windows_core::IUnknown);
impl IReferenceTrackerTarget {
    pub unsafe fn AddRefFromReferenceTracker(&self) -> u32 {
        (::windows_core::Interface::vtable(self).AddRefFromReferenceTracker)(
            ::windows_core::Interface::as_raw(self),
        )
    }
    pub unsafe fn ReleaseFromReferenceTracker(&self) -> u32 {
        (::windows_core::Interface::vtable(self).ReleaseFromReferenceTracker)(
            ::windows_core::Interface::as_raw(self),
        )
    }
    pub unsafe fn Peg(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Peg)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Unpeg(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Unpeg)(::windows_core::Interface::as_raw(self))
            .ok()
    }
}
::windows_core::imp::interface_hierarchy!(IReferenceTrackerTarget, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IReferenceTrackerTarget {
    type Vtable = IReferenceTrackerTarget_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IReferenceTrackerTarget {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x64bd43f8_bfee_4ec4_b7eb_2935158dae21);
}
#[repr(C)]
#[doc(hidden)]
pub struct IReferenceTrackerTarget_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub AddRefFromReferenceTracker:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub ReleaseFromReferenceTracker:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub Peg: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Unpeg: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ITrackerOwner(::windows_core::IUnknown);
impl ITrackerOwner {
    pub unsafe fn CreateTrackerHandle(
        &self,
        returnvalue: *mut *mut TrackerHandle__,
    ) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).CreateTrackerHandle)(
            ::windows_core::Interface::as_raw(self),
            returnvalue,
        )
        .ok()
    }
    pub unsafe fn DeleteTrackerHandle(
        &self,
        handle: *mut TrackerHandle__,
    ) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DeleteTrackerHandle)(
            ::windows_core::Interface::as_raw(self),
            handle,
        )
        .ok()
    }
    pub unsafe fn SetTrackerValue<P0>(
        &self,
        handle: *mut TrackerHandle__,
        value: P0,
    ) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        (::windows_core::Interface::vtable(self).SetTrackerValue)(
            ::windows_core::Interface::as_raw(self),
            handle,
            value.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn TryGetSafeTrackerValue(
        &self,
        handle: *mut TrackerHandle__,
        returnvalue: *mut ::core::option::Option<::windows_core::IUnknown>,
    ) -> u8 {
        (::windows_core::Interface::vtable(self).TryGetSafeTrackerValue)(
            ::windows_core::Interface::as_raw(self),
            handle,
            ::core::mem::transmute(returnvalue),
        )
    }
}
::windows_core::imp::interface_hierarchy!(ITrackerOwner, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITrackerOwner {
    type Vtable = ITrackerOwner_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ITrackerOwner {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xeb24c20b_9816_4ac7_8cff_36f67a118f4e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITrackerOwner_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub CreateTrackerHandle: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        returnvalue: *mut *mut TrackerHandle__,
    ) -> ::windows_core::HRESULT,
    pub DeleteTrackerHandle: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handle: *mut TrackerHandle__,
    ) -> ::windows_core::HRESULT,
    pub SetTrackerValue: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handle: *mut TrackerHandle__,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub TryGetSafeTrackerValue: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handle: *mut TrackerHandle__,
        returnvalue: *mut *mut ::core::ffi::c_void,
    ) -> u8,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWindowsXamlManager(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWindowsXamlManager {
    type Vtable = IWindowsXamlManager_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWindowsXamlManager {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x85a2e562_7e8f_5333_a104_a3e672a2ffee);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsXamlManager_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWindowsXamlManagerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWindowsXamlManagerStatics {
    type Vtable = IWindowsXamlManagerStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWindowsXamlManagerStatics {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x56cb591d_de97_539f_881d_8ccdc44fa6c4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsXamlManagerStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub InitializeForCurrentThread: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IXamlSourceFocusNavigationRequest(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXamlSourceFocusNavigationRequest {
    type Vtable = IXamlSourceFocusNavigationRequest_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IXamlSourceFocusNavigationRequest {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xc883ea8b_4ce2_58be_b547_66dedf620312);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlSourceFocusNavigationRequest_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Reason: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut XamlSourceFocusNavigationReason,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_Foundation")]
    pub HintRect: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::Rect,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    HintRect: usize,
    pub CorrelationId: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows_core::GUID,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IXamlSourceFocusNavigationRequestFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXamlSourceFocusNavigationRequestFactory {
    type Vtable = IXamlSourceFocusNavigationRequestFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IXamlSourceFocusNavigationRequestFactory {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x7a5124dd_2876_5ed8_b564_5867731d7f1e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlSourceFocusNavigationRequestFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        reason: XamlSourceFocusNavigationReason,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_Foundation")]
    pub CreateInstanceWithHintRect: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        reason: XamlSourceFocusNavigationReason,
        hintrect: ::windows::Foundation::Rect,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    CreateInstanceWithHintRect: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub CreateInstanceWithHintRectAndCorrelationId:
        unsafe extern "system" fn(
            this: *mut ::core::ffi::c_void,
            reason: XamlSourceFocusNavigationReason,
            hintrect: ::windows::Foundation::Rect,
            correlationid: ::windows_core::GUID,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    CreateInstanceWithHintRectAndCorrelationId: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IXamlSourceFocusNavigationResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXamlSourceFocusNavigationResult {
    type Vtable = IXamlSourceFocusNavigationResult_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IXamlSourceFocusNavigationResult {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xd6bf378e_2aac_5e5b_ac8a_6c5d9a4c1cb8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlSourceFocusNavigationResult_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub WasFocusMoved: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IXamlSourceFocusNavigationResultFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXamlSourceFocusNavigationResultFactory {
    type Vtable = IXamlSourceFocusNavigationResultFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IXamlSourceFocusNavigationResultFactory {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xf533f53b_5c00_5c88_9a41_3888cb86e495);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlSourceFocusNavigationResultFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        focusmoved: bool,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct DesktopWindowXamlSource(::windows_core::IUnknown);
impl DesktopWindowXamlSource {
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<::windows::Foundation::IClosable>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this))
                .ok()
        }
    }
    pub fn Content(&self) -> ::windows_core::Result<super::UIElement> {
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
    pub fn SetContent<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::UIElement>,
    {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetContent)(
                ::windows_core::Interface::as_raw(this),
                value.try_into_param()?.abi(),
            )
            .ok()
        }
    }
    pub fn HasFocus(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HasFocus)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Media\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Media")]
    pub fn SystemBackdrop(&self) -> ::windows_core::Result<super::Media::SystemBackdrop> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SystemBackdrop)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Media\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Media")]
    pub fn SetSystemBackdrop<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::Media::SystemBackdrop>,
    {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetSystemBackdrop)(
                ::windows_core::Interface::as_raw(this),
                value.try_into_param()?.abi(),
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Content\"`"]
    #[cfg(feature = "Microsoft_UI_Content")]
    pub fn SiteBridge(
        &self,
    ) -> ::windows_core::Result<super::super::Content::DesktopChildSiteBridge> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SiteBridge)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn TakeFocusRequested<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<
                DesktopWindowXamlSource,
                DesktopWindowXamlSourceTakeFocusRequestedEventArgs,
            >,
        >,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TakeFocusRequested)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveTakeFocusRequested(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveTakeFocusRequested)(
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
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<
                DesktopWindowXamlSource,
                DesktopWindowXamlSourceGotFocusEventArgs,
            >,
        >,
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
    pub fn NavigateFocus<P0>(
        &self,
        request: P0,
    ) -> ::windows_core::Result<XamlSourceFocusNavigationResult>
    where
        P0: ::windows_core::IntoParam<XamlSourceFocusNavigationRequest>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NavigateFocus)(
                ::windows_core::Interface::as_raw(this),
                request.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn Initialize(&self, parentwindowid: super::super::WindowId) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).Initialize)(
                ::windows_core::Interface::as_raw(this),
                parentwindowid,
            )
            .ok()
        }
    }
}
impl ::windows_core::RuntimeType for DesktopWindowXamlSource {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for DesktopWindowXamlSource {
    type Vtable = IDesktopWindowXamlSource_Vtbl;
}
unsafe impl ::windows_core::ComInterface for DesktopWindowXamlSource {
    const IID: ::windows_core::GUID =
        <IDesktopWindowXamlSource as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for DesktopWindowXamlSource {
    const NAME: &'static str = "Microsoft.UI.Xaml.Hosting.DesktopWindowXamlSource";
}
::windows_core::imp::interface_hierarchy!(
    DesktopWindowXamlSource,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
#[cfg(feature = "Windows_Foundation")]
impl ::windows_core::CanTryInto<::windows::Foundation::IClosable> for DesktopWindowXamlSource {}
unsafe impl ::core::marker::Send for DesktopWindowXamlSource {}
unsafe impl ::core::marker::Sync for DesktopWindowXamlSource {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct DesktopWindowXamlSourceGotFocusEventArgs(::windows_core::IUnknown);
impl DesktopWindowXamlSourceGotFocusEventArgs {
    pub fn Request(&self) -> ::windows_core::Result<XamlSourceFocusNavigationRequest> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Request)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for DesktopWindowXamlSourceGotFocusEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for DesktopWindowXamlSourceGotFocusEventArgs {
    type Vtable = IDesktopWindowXamlSourceGotFocusEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for DesktopWindowXamlSourceGotFocusEventArgs {
    const IID: ::windows_core::GUID =
        <IDesktopWindowXamlSourceGotFocusEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for DesktopWindowXamlSourceGotFocusEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Hosting.DesktopWindowXamlSourceGotFocusEventArgs";
}
::windows_core::imp::interface_hierarchy!(
    DesktopWindowXamlSourceGotFocusEventArgs,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for DesktopWindowXamlSourceGotFocusEventArgs {}
unsafe impl ::core::marker::Sync for DesktopWindowXamlSourceGotFocusEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct DesktopWindowXamlSourceTakeFocusRequestedEventArgs(::windows_core::IUnknown);
impl DesktopWindowXamlSourceTakeFocusRequestedEventArgs {
    pub fn Request(&self) -> ::windows_core::Result<XamlSourceFocusNavigationRequest> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Request)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for DesktopWindowXamlSourceTakeFocusRequestedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for DesktopWindowXamlSourceTakeFocusRequestedEventArgs {
    type Vtable = IDesktopWindowXamlSourceTakeFocusRequestedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for DesktopWindowXamlSourceTakeFocusRequestedEventArgs {
    const IID: ::windows_core::GUID =
        <IDesktopWindowXamlSourceTakeFocusRequestedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for DesktopWindowXamlSourceTakeFocusRequestedEventArgs {
    const NAME: &'static str =
        "Microsoft.UI.Xaml.Hosting.DesktopWindowXamlSourceTakeFocusRequestedEventArgs";
}
::windows_core::imp::interface_hierarchy!(
    DesktopWindowXamlSourceTakeFocusRequestedEventArgs,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for DesktopWindowXamlSourceTakeFocusRequestedEventArgs {}
unsafe impl ::core::marker::Sync for DesktopWindowXamlSourceTakeFocusRequestedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ElementCompositionPreview(::windows_core::IUnknown);
impl ElementCompositionPreview {
    #[doc = "Required features: `\"Microsoft_UI_Composition\"`"]
    #[cfg(feature = "Microsoft_UI_Composition")]
    pub fn GetElementVisual<P0>(
        element: P0,
    ) -> ::windows_core::Result<super::super::Composition::Visual>
    where
        P0: ::windows_core::TryIntoParam<super::UIElement>,
    {
        Self::IElementCompositionPreviewStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetElementVisual)(
                ::windows_core::Interface::as_raw(this),
                element.try_into_param()?.abi(),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Microsoft_UI_Composition\"`"]
    #[cfg(feature = "Microsoft_UI_Composition")]
    pub fn GetElementChildVisual<P0>(
        element: P0,
    ) -> ::windows_core::Result<super::super::Composition::Visual>
    where
        P0: ::windows_core::TryIntoParam<super::UIElement>,
    {
        Self::IElementCompositionPreviewStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetElementChildVisual)(
                ::windows_core::Interface::as_raw(this),
                element.try_into_param()?.abi(),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Microsoft_UI_Composition\"`"]
    #[cfg(feature = "Microsoft_UI_Composition")]
    pub fn SetElementChildVisual<P0, P1>(element: P0, visual: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::UIElement>,
        P1: ::windows_core::TryIntoParam<super::super::Composition::Visual>,
    {
        Self::IElementCompositionPreviewStatics(|this| unsafe {
            (::windows_core::Interface::vtable(this).SetElementChildVisual)(
                ::windows_core::Interface::as_raw(this),
                element.try_into_param()?.abi(),
                visual.try_into_param()?.abi(),
            )
            .ok()
        })
    }
    #[doc = "Required features: `\"Microsoft_UI_Composition\"`, `\"Microsoft_UI_Xaml_Controls\"`"]
    #[cfg(all(
        feature = "Microsoft_UI_Composition",
        feature = "Microsoft_UI_Xaml_Controls"
    ))]
    pub fn GetScrollViewerManipulationPropertySet<P0>(
        scrollviewer: P0,
    ) -> ::windows_core::Result<super::super::Composition::CompositionPropertySet>
    where
        P0: ::windows_core::IntoParam<super::Controls::ScrollViewer>,
    {
        Self::IElementCompositionPreviewStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetScrollViewerManipulationPropertySet)(
                ::windows_core::Interface::as_raw(this),
                scrollviewer.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Microsoft_UI_Composition\"`"]
    #[cfg(feature = "Microsoft_UI_Composition")]
    pub fn SetImplicitShowAnimation<P0, P1>(
        element: P0,
        animation: P1,
    ) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::UIElement>,
        P1: ::windows_core::TryIntoParam<super::super::Composition::ICompositionAnimationBase>,
    {
        Self::IElementCompositionPreviewStatics(|this| unsafe {
            (::windows_core::Interface::vtable(this).SetImplicitShowAnimation)(
                ::windows_core::Interface::as_raw(this),
                element.try_into_param()?.abi(),
                animation.try_into_param()?.abi(),
            )
            .ok()
        })
    }
    #[doc = "Required features: `\"Microsoft_UI_Composition\"`"]
    #[cfg(feature = "Microsoft_UI_Composition")]
    pub fn SetImplicitHideAnimation<P0, P1>(
        element: P0,
        animation: P1,
    ) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::UIElement>,
        P1: ::windows_core::TryIntoParam<super::super::Composition::ICompositionAnimationBase>,
    {
        Self::IElementCompositionPreviewStatics(|this| unsafe {
            (::windows_core::Interface::vtable(this).SetImplicitHideAnimation)(
                ::windows_core::Interface::as_raw(this),
                element.try_into_param()?.abi(),
                animation.try_into_param()?.abi(),
            )
            .ok()
        })
    }
    pub fn SetIsTranslationEnabled<P0>(element: P0, value: bool) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::UIElement>,
    {
        Self::IElementCompositionPreviewStatics(|this| unsafe {
            (::windows_core::Interface::vtable(this).SetIsTranslationEnabled)(
                ::windows_core::Interface::as_raw(this),
                element.try_into_param()?.abi(),
                value,
            )
            .ok()
        })
    }
    #[doc = "Required features: `\"Microsoft_UI_Composition\"`"]
    #[cfg(feature = "Microsoft_UI_Composition")]
    pub fn GetPointerPositionPropertySet<P0>(
        targetelement: P0,
    ) -> ::windows_core::Result<super::super::Composition::CompositionPropertySet>
    where
        P0: ::windows_core::TryIntoParam<super::UIElement>,
    {
        Self::IElementCompositionPreviewStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetPointerPositionPropertySet)(
                ::windows_core::Interface::as_raw(this),
                targetelement.try_into_param()?.abi(),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IElementCompositionPreviewStatics<
        R,
        F: FnOnce(&IElementCompositionPreviewStatics) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<
            ElementCompositionPreview,
            IElementCompositionPreviewStatics,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for ElementCompositionPreview {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for ElementCompositionPreview {
    type Vtable = IElementCompositionPreview_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ElementCompositionPreview {
    const IID: ::windows_core::GUID =
        <IElementCompositionPreview as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ElementCompositionPreview {
    const NAME: &'static str = "Microsoft.UI.Xaml.Hosting.ElementCompositionPreview";
}
::windows_core::imp::interface_hierarchy!(
    ElementCompositionPreview,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for ElementCompositionPreview {}
unsafe impl ::core::marker::Sync for ElementCompositionPreview {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct WindowsXamlManager(::windows_core::IUnknown);
impl WindowsXamlManager {
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<::windows::Foundation::IClosable>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this))
                .ok()
        }
    }
    pub fn InitializeForCurrentThread() -> ::windows_core::Result<WindowsXamlManager> {
        Self::IWindowsXamlManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).InitializeForCurrentThread)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IWindowsXamlManagerStatics<
        R,
        F: FnOnce(&IWindowsXamlManagerStatics) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<
            WindowsXamlManager,
            IWindowsXamlManagerStatics,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for WindowsXamlManager {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for WindowsXamlManager {
    type Vtable = IWindowsXamlManager_Vtbl;
}
unsafe impl ::windows_core::ComInterface for WindowsXamlManager {
    const IID: ::windows_core::GUID = <IWindowsXamlManager as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for WindowsXamlManager {
    const NAME: &'static str = "Microsoft.UI.Xaml.Hosting.WindowsXamlManager";
}
::windows_core::imp::interface_hierarchy!(
    WindowsXamlManager,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
#[cfg(feature = "Windows_Foundation")]
impl ::windows_core::CanTryInto<::windows::Foundation::IClosable> for WindowsXamlManager {}
unsafe impl ::core::marker::Send for WindowsXamlManager {}
unsafe impl ::core::marker::Sync for WindowsXamlManager {}
pub const XAML_REFERENCETRACKER_DISCONNECT_DEFAULT:
    __MIDL___MIDL_itf_microsoft2Eui2Examl2Ehosting2Ereferencetracker_0000_0004_0001 =
    __MIDL___MIDL_itf_microsoft2Eui2Examl2Ehosting2Ereferencetracker_0000_0004_0001(0i32);
pub const XAML_REFERENCETRACKER_DISCONNECT_SUSPEND:
    __MIDL___MIDL_itf_microsoft2Eui2Examl2Ehosting2Ereferencetracker_0000_0004_0001 =
    __MIDL___MIDL_itf_microsoft2Eui2Examl2Ehosting2Ereferencetracker_0000_0004_0001(1i32);
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct XamlSourceFocusNavigationRequest(::windows_core::IUnknown);
impl XamlSourceFocusNavigationRequest {
    pub fn Reason(&self) -> ::windows_core::Result<XamlSourceFocusNavigationReason> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Reason)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn HintRect(&self) -> ::windows_core::Result<::windows::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HintRect)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn CorrelationId(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CorrelationId)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn CreateInstance(
        reason: XamlSourceFocusNavigationReason,
    ) -> ::windows_core::Result<XamlSourceFocusNavigationRequest> {
        Self::IXamlSourceFocusNavigationRequestFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstance)(
                ::windows_core::Interface::as_raw(this),
                reason,
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn CreateInstanceWithHintRect(
        reason: XamlSourceFocusNavigationReason,
        hintrect: ::windows::Foundation::Rect,
    ) -> ::windows_core::Result<XamlSourceFocusNavigationRequest> {
        Self::IXamlSourceFocusNavigationRequestFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstanceWithHintRect)(
                ::windows_core::Interface::as_raw(this),
                reason,
                hintrect,
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn CreateInstanceWithHintRectAndCorrelationId(
        reason: XamlSourceFocusNavigationReason,
        hintrect: ::windows::Foundation::Rect,
        correlationid: ::windows_core::GUID,
    ) -> ::windows_core::Result<XamlSourceFocusNavigationRequest> {
        Self::IXamlSourceFocusNavigationRequestFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstanceWithHintRectAndCorrelationId)(
                ::windows_core::Interface::as_raw(this),
                reason,
                hintrect,
                correlationid,
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IXamlSourceFocusNavigationRequestFactory<
        R,
        F: FnOnce(&IXamlSourceFocusNavigationRequestFactory) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<
            XamlSourceFocusNavigationRequest,
            IXamlSourceFocusNavigationRequestFactory,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for XamlSourceFocusNavigationRequest {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for XamlSourceFocusNavigationRequest {
    type Vtable = IXamlSourceFocusNavigationRequest_Vtbl;
}
unsafe impl ::windows_core::ComInterface for XamlSourceFocusNavigationRequest {
    const IID: ::windows_core::GUID =
        <IXamlSourceFocusNavigationRequest as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for XamlSourceFocusNavigationRequest {
    const NAME: &'static str = "Microsoft.UI.Xaml.Hosting.XamlSourceFocusNavigationRequest";
}
::windows_core::imp::interface_hierarchy!(
    XamlSourceFocusNavigationRequest,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for XamlSourceFocusNavigationRequest {}
unsafe impl ::core::marker::Sync for XamlSourceFocusNavigationRequest {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct XamlSourceFocusNavigationResult(::windows_core::IUnknown);
impl XamlSourceFocusNavigationResult {
    pub fn WasFocusMoved(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).WasFocusMoved)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn CreateInstance(
        focusmoved: bool,
    ) -> ::windows_core::Result<XamlSourceFocusNavigationResult> {
        Self::IXamlSourceFocusNavigationResultFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstance)(
                ::windows_core::Interface::as_raw(this),
                focusmoved,
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IXamlSourceFocusNavigationResultFactory<
        R,
        F: FnOnce(&IXamlSourceFocusNavigationResultFactory) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<
            XamlSourceFocusNavigationResult,
            IXamlSourceFocusNavigationResultFactory,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for XamlSourceFocusNavigationResult {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for XamlSourceFocusNavigationResult {
    type Vtable = IXamlSourceFocusNavigationResult_Vtbl;
}
unsafe impl ::windows_core::ComInterface for XamlSourceFocusNavigationResult {
    const IID: ::windows_core::GUID =
        <IXamlSourceFocusNavigationResult as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for XamlSourceFocusNavigationResult {
    const NAME: &'static str = "Microsoft.UI.Xaml.Hosting.XamlSourceFocusNavigationResult";
}
::windows_core::imp::interface_hierarchy!(
    XamlSourceFocusNavigationResult,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for XamlSourceFocusNavigationResult {}
unsafe impl ::core::marker::Sync for XamlSourceFocusNavigationResult {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct XamlSourceFocusNavigationReason(pub i32);
impl XamlSourceFocusNavigationReason {
    pub const Programmatic: Self = Self(0i32);
    pub const Restore: Self = Self(1i32);
    pub const First: Self = Self(3i32);
    pub const Last: Self = Self(4i32);
    pub const Left: Self = Self(7i32);
    pub const Up: Self = Self(8i32);
    pub const Right: Self = Self(9i32);
    pub const Down: Self = Self(10i32);
}
impl ::core::marker::Copy for XamlSourceFocusNavigationReason {}
impl ::core::clone::Clone for XamlSourceFocusNavigationReason {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for XamlSourceFocusNavigationReason {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for XamlSourceFocusNavigationReason {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for XamlSourceFocusNavigationReason {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XamlSourceFocusNavigationReason").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for XamlSourceFocusNavigationReason {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::from_slice(
            b"enum(Microsoft.UI.Xaml.Hosting.XamlSourceFocusNavigationReason;i4)",
        );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct __MIDL___MIDL_itf_microsoft2Eui2Examl2Ehosting2Ereferencetracker_0000_0004_0001(pub i32);
impl ::core::marker::Copy
    for __MIDL___MIDL_itf_microsoft2Eui2Examl2Ehosting2Ereferencetracker_0000_0004_0001
{
}
impl ::core::clone::Clone
    for __MIDL___MIDL_itf_microsoft2Eui2Examl2Ehosting2Ereferencetracker_0000_0004_0001
{
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default
    for __MIDL___MIDL_itf_microsoft2Eui2Examl2Ehosting2Ereferencetracker_0000_0004_0001
{
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind
    for __MIDL___MIDL_itf_microsoft2Eui2Examl2Ehosting2Ereferencetracker_0000_0004_0001
{
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug
    for __MIDL___MIDL_itf_microsoft2Eui2Examl2Ehosting2Ereferencetracker_0000_0004_0001
{
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple(
            "__MIDL___MIDL_itf_microsoft2Eui2Examl2Ehosting2Ereferencetracker_0000_0004_0001",
        )
        .field(&self.0)
        .finish()
    }
}
#[repr(C)]
pub struct TrackerHandle__ {
    pub unused: i32,
}
impl ::core::marker::Copy for TrackerHandle__ {}
impl ::core::clone::Clone for TrackerHandle__ {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TrackerHandle__ {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TrackerHandle__").field("unused", &self.unused).finish()
    }
}
impl ::windows_core::TypeKind for TrackerHandle__ {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for TrackerHandle__ {
    fn eq(&self, other: &Self) -> bool {
        self.unused == other.unused
    }
}
impl ::core::cmp::Eq for TrackerHandle__ {}
impl ::core::default::Default for TrackerHandle__ {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
