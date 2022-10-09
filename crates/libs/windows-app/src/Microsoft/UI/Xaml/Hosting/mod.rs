#[doc = "*Required features: `\"UI_Xaml_Hosting\"`*"]
#[repr(transparent)]
pub struct IDesktopWindowXamlSourceNative(::windows::core::IUnknown);
impl IDesktopWindowXamlSourceNative {
    pub unsafe fn AttachToWindow<'a, P0>(&self, parentwnd: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::Win32::Foundation::HWND>,
    {
        (::windows::core::Vtable::vtable(self).AttachToWindow)(
            ::windows::core::Vtable::as_raw(self),
            parentwnd.into(),
        )
        .ok()
    }
    pub unsafe fn WindowHandle(
        &self,
        hwnd: *mut ::windows::Win32::Foundation::HWND,
    ) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).WindowHandle)(
            ::windows::core::Vtable::as_raw(self),
            ::core::mem::transmute(hwnd),
        )
        .ok()
    }
    pub unsafe fn PreTranslateMessage(
        &self,
        message: *const ::windows::Win32::UI::WindowsAndMessaging::MSG,
        result: *mut ::windows::Win32::Foundation::BOOL,
    ) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).PreTranslateMessage)(
            ::windows::core::Vtable::as_raw(self),
            ::core::mem::transmute(message),
            ::core::mem::transmute(result),
        )
        .ok()
    }
}
::windows::core::interface_hierarchy!(IDesktopWindowXamlSourceNative, ::windows::core::IUnknown);
impl ::core::clone::Clone for IDesktopWindowXamlSourceNative {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDesktopWindowXamlSourceNative {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDesktopWindowXamlSourceNative {}
impl ::core::fmt::Debug for IDesktopWindowXamlSourceNative {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDesktopWindowXamlSourceNative").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IDesktopWindowXamlSourceNative {
    type Vtable = IDesktopWindowXamlSourceNative_Vtbl;
}
unsafe impl ::windows::core::Interface for IDesktopWindowXamlSourceNative {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x0aea2f26_facf_4588_8cf4_34555124db32);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDesktopWindowXamlSourceNative_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub AttachToWindow: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        parentwnd: ::windows::Win32::Foundation::HWND,
    ) -> ::windows::core::HRESULT,
    pub WindowHandle: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        hwnd: *mut ::windows::Win32::Foundation::HWND,
    ) -> ::windows::core::HRESULT,
    pub PreTranslateMessage: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        message: *const ::windows::Win32::UI::WindowsAndMessaging::MSG,
        result: *mut ::windows::Win32::Foundation::BOOL,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IElementCompositionPreview(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IElementCompositionPreview {
    type Vtable = IElementCompositionPreview_Vtbl;
}
unsafe impl ::windows::core::Interface for IElementCompositionPreview {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xc8ad1ef4_a93f_5a25_85bd_7c498d9856d3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IElementCompositionPreview_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IElementCompositionPreviewStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IElementCompositionPreviewStatics {
    type Vtable = IElementCompositionPreviewStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IElementCompositionPreviewStatics {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x84da5a6c_0cfa_532b_9b15_ccd986374342);
}
#[repr(C)]
#[doc(hidden)]
pub struct IElementCompositionPreviewStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "UI_Composition")]
    pub GetElementVisual: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))]
    GetElementVisual: usize,
    #[cfg(feature = "UI_Composition")]
    pub GetElementChildVisual: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))]
    GetElementChildVisual: usize,
    #[cfg(feature = "UI_Composition")]
    pub SetElementChildVisual: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        visual: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))]
    SetElementChildVisual: usize,
    #[cfg(all(feature = "UI_Composition", feature = "UI_Xaml_Controls"))]
    pub GetScrollViewerManipulationPropertySet:
        unsafe extern "system" fn(
            this: *mut ::core::ffi::c_void,
            scrollviewer: *mut ::core::ffi::c_void,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "UI_Composition", feature = "UI_Xaml_Controls")))]
    GetScrollViewerManipulationPropertySet: usize,
    #[cfg(feature = "UI_Composition")]
    pub SetImplicitShowAnimation: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        animation: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))]
    SetImplicitShowAnimation: usize,
    #[cfg(feature = "UI_Composition")]
    pub SetImplicitHideAnimation: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        animation: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))]
    SetImplicitHideAnimation: usize,
    pub SetIsTranslationEnabled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Composition")]
    pub GetPointerPositionPropertySet: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        targetelement: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))]
    GetPointerPositionPropertySet: usize,
}
#[doc = "*Required features: `\"UI_Xaml_Hosting\"`*"]
#[repr(transparent)]
pub struct IFindReferenceTargetsCallback(::windows::core::IUnknown);
impl IFindReferenceTargetsCallback {
    pub unsafe fn FoundTrackerTarget<'a, P0>(&self, target: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IReferenceTrackerTarget>>,
    {
        (::windows::core::Vtable::vtable(self).FoundTrackerTarget)(
            ::windows::core::Vtable::as_raw(self),
            target.into().abi(),
        )
        .ok()
    }
}
::windows::core::interface_hierarchy!(IFindReferenceTargetsCallback, ::windows::core::IUnknown);
impl ::core::clone::Clone for IFindReferenceTargetsCallback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFindReferenceTargetsCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFindReferenceTargetsCallback {}
impl ::core::fmt::Debug for IFindReferenceTargetsCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFindReferenceTargetsCallback").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IFindReferenceTargetsCallback {
    type Vtable = IFindReferenceTargetsCallback_Vtbl;
}
unsafe impl ::windows::core::Interface for IFindReferenceTargetsCallback {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x04b3486c_4687_4229_8d14_505ab584dd88);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFindReferenceTargetsCallback_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub FoundTrackerTarget: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        target: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Xaml_Hosting\"`*"]
#[repr(transparent)]
pub struct IReferenceTracker(::windows::core::IUnknown);
impl IReferenceTracker {
    pub unsafe fn ConnectFromTrackerSource(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).ConnectFromTrackerSource)(
            ::windows::core::Vtable::as_raw(self),
        )
        .ok()
    }
    pub unsafe fn DisconnectFromTrackerSource(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).DisconnectFromTrackerSource)(
            ::windows::core::Vtable::as_raw(self),
        )
        .ok()
    }
    pub unsafe fn FindTrackerTargets<'a, P0>(&self, callback: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IFindReferenceTargetsCallback>>,
    {
        (::windows::core::Vtable::vtable(self).FindTrackerTargets)(
            ::windows::core::Vtable::as_raw(self),
            callback.into().abi(),
        )
        .ok()
    }
    pub unsafe fn GetReferenceTrackerManager(
        &self,
    ) -> ::windows::core::Result<IReferenceTrackerManager> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetReferenceTrackerManager)(
            ::windows::core::Vtable::as_raw(self),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<IReferenceTrackerManager>(result__)
    }
    pub unsafe fn AddRefFromTrackerSource(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).AddRefFromTrackerSource)(
            ::windows::core::Vtable::as_raw(self),
        )
        .ok()
    }
    pub unsafe fn ReleaseFromTrackerSource(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).ReleaseFromTrackerSource)(
            ::windows::core::Vtable::as_raw(self),
        )
        .ok()
    }
    pub unsafe fn PegFromTrackerSource(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).PegFromTrackerSource)(
            ::windows::core::Vtable::as_raw(self),
        )
        .ok()
    }
}
::windows::core::interface_hierarchy!(IReferenceTracker, ::windows::core::IUnknown);
impl ::core::clone::Clone for IReferenceTracker {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IReferenceTracker {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IReferenceTracker {}
impl ::core::fmt::Debug for IReferenceTracker {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IReferenceTracker").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IReferenceTracker {
    type Vtable = IReferenceTracker_Vtbl;
}
unsafe impl ::windows::core::Interface for IReferenceTracker {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x11d3b13a_180e_4789_a8be_7712882893e6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IReferenceTracker_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub ConnectFromTrackerSource:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub DisconnectFromTrackerSource:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub FindTrackerTargets: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        callback: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub GetReferenceTrackerManager: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub AddRefFromTrackerSource:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ReleaseFromTrackerSource:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub PegFromTrackerSource:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Xaml_Hosting\"`*"]
#[repr(transparent)]
pub struct IReferenceTrackerExtension(::windows::core::IUnknown);
impl IReferenceTrackerExtension {}
::windows::core::interface_hierarchy!(IReferenceTrackerExtension, ::windows::core::IUnknown);
impl ::core::clone::Clone for IReferenceTrackerExtension {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IReferenceTrackerExtension {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IReferenceTrackerExtension {}
impl ::core::fmt::Debug for IReferenceTrackerExtension {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IReferenceTrackerExtension").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IReferenceTrackerExtension {
    type Vtable = IReferenceTrackerExtension_Vtbl;
}
unsafe impl ::windows::core::Interface for IReferenceTrackerExtension {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x4e897caa_59d5_4613_8f8c_f7ebd1f399b0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IReferenceTrackerExtension_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
}
#[doc = "*Required features: `\"UI_Xaml_Hosting\"`*"]
#[repr(transparent)]
pub struct IReferenceTrackerHost(::windows::core::IUnknown);
impl IReferenceTrackerHost {
    pub unsafe fn DisconnectUnusedReferenceSources(
        &self,
        options: __MIDL___MIDL_itf_microsoft2Eui2Examl2Ehosting2Ereferencetracker_0000_0004_0001,
    ) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).DisconnectUnusedReferenceSources)(
            ::windows::core::Vtable::as_raw(self),
            options,
        )
        .ok()
    }
    pub unsafe fn ReleaseDisconnectedReferenceSources(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).ReleaseDisconnectedReferenceSources)(
            ::windows::core::Vtable::as_raw(self),
        )
        .ok()
    }
    pub unsafe fn NotifyEndOfReferenceTrackingOnThread(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).NotifyEndOfReferenceTrackingOnThread)(
            ::windows::core::Vtable::as_raw(self),
        )
        .ok()
    }
    pub unsafe fn GetTrackerTarget<'a, P0>(
        &self,
        unknown: P0,
    ) -> ::windows::core::Result<IReferenceTrackerTarget>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IUnknown>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetTrackerTarget)(
            ::windows::core::Vtable::as_raw(self),
            unknown.into().abi(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<IReferenceTrackerTarget>(result__)
    }
    pub unsafe fn AddMemoryPressure(&self, bytesallocated: u64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).AddMemoryPressure)(
            ::windows::core::Vtable::as_raw(self),
            bytesallocated,
        )
        .ok()
    }
    pub unsafe fn RemoveMemoryPressure(&self, bytesallocated: u64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).RemoveMemoryPressure)(
            ::windows::core::Vtable::as_raw(self),
            bytesallocated,
        )
        .ok()
    }
}
::windows::core::interface_hierarchy!(IReferenceTrackerHost, ::windows::core::IUnknown);
impl ::core::clone::Clone for IReferenceTrackerHost {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IReferenceTrackerHost {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IReferenceTrackerHost {}
impl ::core::fmt::Debug for IReferenceTrackerHost {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IReferenceTrackerHost").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IReferenceTrackerHost {
    type Vtable = IReferenceTrackerHost_Vtbl;
}
unsafe impl ::windows::core::Interface for IReferenceTrackerHost {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x29a71c6a_3c42_4416_a39d_e2825a07a773);
}
#[repr(C)]
#[doc(hidden)]
pub struct IReferenceTrackerHost_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub DisconnectUnusedReferenceSources: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        options: __MIDL___MIDL_itf_microsoft2Eui2Examl2Ehosting2Ereferencetracker_0000_0004_0001,
    )
        -> ::windows::core::HRESULT,
    pub ReleaseDisconnectedReferenceSources:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub NotifyEndOfReferenceTrackingOnThread:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetTrackerTarget: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        unknown: *mut ::core::ffi::c_void,
        newreference: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub AddMemoryPressure: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        bytesallocated: u64,
    ) -> ::windows::core::HRESULT,
    pub RemoveMemoryPressure: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        bytesallocated: u64,
    ) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Xaml_Hosting\"`*"]
#[repr(transparent)]
pub struct IReferenceTrackerManager(::windows::core::IUnknown);
impl IReferenceTrackerManager {
    pub unsafe fn ReferenceTrackingStarted(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).ReferenceTrackingStarted)(
            ::windows::core::Vtable::as_raw(self),
        )
        .ok()
    }
    pub unsafe fn FindTrackerTargetsCompleted(
        &self,
        findfailed: u8,
    ) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).FindTrackerTargetsCompleted)(
            ::windows::core::Vtable::as_raw(self),
            findfailed,
        )
        .ok()
    }
    pub unsafe fn ReferenceTrackingCompleted(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).ReferenceTrackingCompleted)(
            ::windows::core::Vtable::as_raw(self),
        )
        .ok()
    }
    pub unsafe fn SetReferenceTrackerHost<'a, P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IReferenceTrackerHost>>,
    {
        (::windows::core::Vtable::vtable(self).SetReferenceTrackerHost)(
            ::windows::core::Vtable::as_raw(self),
            value.into().abi(),
        )
        .ok()
    }
}
::windows::core::interface_hierarchy!(IReferenceTrackerManager, ::windows::core::IUnknown);
impl ::core::clone::Clone for IReferenceTrackerManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IReferenceTrackerManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IReferenceTrackerManager {}
impl ::core::fmt::Debug for IReferenceTrackerManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IReferenceTrackerManager").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IReferenceTrackerManager {
    type Vtable = IReferenceTrackerManager_Vtbl;
}
unsafe impl ::windows::core::Interface for IReferenceTrackerManager {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x3cf184b4_7ccb_4dda_8455_7e6ce99a3298);
}
#[repr(C)]
#[doc(hidden)]
pub struct IReferenceTrackerManager_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub ReferenceTrackingStarted:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub FindTrackerTargetsCompleted: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        findfailed: u8,
    ) -> ::windows::core::HRESULT,
    pub ReferenceTrackingCompleted:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetReferenceTrackerHost: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Xaml_Hosting\"`*"]
#[repr(transparent)]
pub struct IReferenceTrackerTarget(::windows::core::IUnknown);
impl IReferenceTrackerTarget {
    pub unsafe fn AddRefFromReferenceTracker(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).AddRefFromReferenceTracker)(
            ::windows::core::Vtable::as_raw(self),
        )
    }
    pub unsafe fn ReleaseFromReferenceTracker(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).ReleaseFromReferenceTracker)(
            ::windows::core::Vtable::as_raw(self),
        )
    }
    pub unsafe fn Peg(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Peg)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Unpeg(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Unpeg)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
::windows::core::interface_hierarchy!(IReferenceTrackerTarget, ::windows::core::IUnknown);
impl ::core::clone::Clone for IReferenceTrackerTarget {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IReferenceTrackerTarget {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IReferenceTrackerTarget {}
impl ::core::fmt::Debug for IReferenceTrackerTarget {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IReferenceTrackerTarget").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IReferenceTrackerTarget {
    type Vtable = IReferenceTrackerTarget_Vtbl;
}
unsafe impl ::windows::core::Interface for IReferenceTrackerTarget {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x64bd43f8_bfee_4ec4_b7eb_2935158dae21);
}
#[repr(C)]
#[doc(hidden)]
pub struct IReferenceTrackerTarget_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub AddRefFromReferenceTracker:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub ReleaseFromReferenceTracker:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub Peg: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Unpeg:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Xaml_Hosting\"`*"]
#[repr(transparent)]
pub struct ITrackerOwner(::windows::core::IUnknown);
impl ITrackerOwner {
    pub unsafe fn CreateTrackerHandle(
        &self,
        returnvalue: *mut *mut TrackerHandle__,
    ) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).CreateTrackerHandle)(
            ::windows::core::Vtable::as_raw(self),
            ::core::mem::transmute(returnvalue),
        )
        .ok()
    }
    pub unsafe fn DeleteTrackerHandle(
        &self,
        handle: *mut TrackerHandle__,
    ) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).DeleteTrackerHandle)(
            ::windows::core::Vtable::as_raw(self),
            ::core::mem::transmute(handle),
        )
        .ok()
    }
    pub unsafe fn SetTrackerValue<'a, P0>(
        &self,
        handle: *mut TrackerHandle__,
        value: P0,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).SetTrackerValue)(
            ::windows::core::Vtable::as_raw(self),
            ::core::mem::transmute(handle),
            value.into().abi(),
        )
        .ok()
    }
    pub unsafe fn TryGetSafeTrackerValue(
        &self,
        handle: *mut TrackerHandle__,
        returnvalue: *mut ::core::option::Option<::windows::core::IUnknown>,
    ) -> u8 {
        (::windows::core::Vtable::vtable(self).TryGetSafeTrackerValue)(
            ::windows::core::Vtable::as_raw(self),
            ::core::mem::transmute(handle),
            ::core::mem::transmute(returnvalue),
        )
    }
}
::windows::core::interface_hierarchy!(ITrackerOwner, ::windows::core::IUnknown);
impl ::core::clone::Clone for ITrackerOwner {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITrackerOwner {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITrackerOwner {}
impl ::core::fmt::Debug for ITrackerOwner {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITrackerOwner").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for ITrackerOwner {
    type Vtable = ITrackerOwner_Vtbl;
}
unsafe impl ::windows::core::Interface for ITrackerOwner {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xeb24c20b_9816_4ac7_8cff_36f67a118f4e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITrackerOwner_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub CreateTrackerHandle: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        returnvalue: *mut *mut TrackerHandle__,
    ) -> ::windows::core::HRESULT,
    pub DeleteTrackerHandle: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handle: *mut TrackerHandle__,
    ) -> ::windows::core::HRESULT,
    pub SetTrackerValue: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handle: *mut TrackerHandle__,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub TryGetSafeTrackerValue: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handle: *mut TrackerHandle__,
        returnvalue: *mut *mut ::core::ffi::c_void,
    ) -> u8,
}
#[doc = "*Required features: `\"UI_Xaml_Hosting\"`*"]
#[repr(transparent)]
pub struct ElementCompositionPreview(::windows::core::IUnknown);
impl ElementCompositionPreview {
    #[doc = "*Required features: `\"UI_Composition\"`*"]
    #[cfg(feature = "UI_Composition")]
    pub fn GetElementVisual<'a, P0>(
        element: P0,
    ) -> ::windows::core::Result<super::super::Composition::Visual>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::UIElement>>,
    {
        Self::IElementCompositionPreviewStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetElementVisual)(
                ::windows::core::Vtable::as_raw(this),
                element.into().abi(),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::super::Composition::Visual>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Composition\"`*"]
    #[cfg(feature = "UI_Composition")]
    pub fn GetElementChildVisual<'a, P0>(
        element: P0,
    ) -> ::windows::core::Result<super::super::Composition::Visual>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::UIElement>>,
    {
        Self::IElementCompositionPreviewStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetElementChildVisual)(
                ::windows::core::Vtable::as_raw(this),
                element.into().abi(),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::super::Composition::Visual>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Composition\"`*"]
    #[cfg(feature = "UI_Composition")]
    pub fn SetElementChildVisual<'a, P0, P1>(element: P0, visual: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::UIElement>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, super::super::Composition::Visual>>,
    {
        Self::IElementCompositionPreviewStatics(|this| unsafe {
            (::windows::core::Vtable::vtable(this).SetElementChildVisual)(
                ::windows::core::Vtable::as_raw(this),
                element.into().abi(),
                visual.into().abi(),
            )
            .ok()
        })
    }
    #[doc = "*Required features: `\"UI_Composition\"`, `\"UI_Xaml_Controls\"`*"]
    #[cfg(all(feature = "UI_Composition", feature = "UI_Xaml_Controls"))]
    pub fn GetScrollViewerManipulationPropertySet(
        scrollviewer: &super::Controls::ScrollViewer,
    ) -> ::windows::core::Result<super::super::Composition::CompositionPropertySet> {
        Self::IElementCompositionPreviewStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetScrollViewerManipulationPropertySet)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(scrollviewer),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::super::Composition::CompositionPropertySet>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Composition\"`*"]
    #[cfg(feature = "UI_Composition")]
    pub fn SetImplicitShowAnimation<'a, P0, P1, E1>(
        element: P0,
        animation: P1,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::UIElement>>,
        P1: ::std::convert::TryInto<
            ::windows::core::InParam<'a, super::super::Composition::ICompositionAnimationBase>,
            Error = E1,
        >,
        E1: ::std::convert::Into<::windows::core::Error>,
    {
        Self::IElementCompositionPreviewStatics(|this| unsafe {
            (::windows::core::Vtable::vtable(this).SetImplicitShowAnimation)(
                ::windows::core::Vtable::as_raw(this),
                element.into().abi(),
                animation.try_into().map_err(|e| e.into())?.abi(),
            )
            .ok()
        })
    }
    #[doc = "*Required features: `\"UI_Composition\"`*"]
    #[cfg(feature = "UI_Composition")]
    pub fn SetImplicitHideAnimation<'a, P0, P1, E1>(
        element: P0,
        animation: P1,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::UIElement>>,
        P1: ::std::convert::TryInto<
            ::windows::core::InParam<'a, super::super::Composition::ICompositionAnimationBase>,
            Error = E1,
        >,
        E1: ::std::convert::Into<::windows::core::Error>,
    {
        Self::IElementCompositionPreviewStatics(|this| unsafe {
            (::windows::core::Vtable::vtable(this).SetImplicitHideAnimation)(
                ::windows::core::Vtable::as_raw(this),
                element.into().abi(),
                animation.try_into().map_err(|e| e.into())?.abi(),
            )
            .ok()
        })
    }
    pub fn SetIsTranslationEnabled<'a, P0>(element: P0, value: bool) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::UIElement>>,
    {
        Self::IElementCompositionPreviewStatics(|this| unsafe {
            (::windows::core::Vtable::vtable(this).SetIsTranslationEnabled)(
                ::windows::core::Vtable::as_raw(this),
                element.into().abi(),
                value,
            )
            .ok()
        })
    }
    #[doc = "*Required features: `\"UI_Composition\"`*"]
    #[cfg(feature = "UI_Composition")]
    pub fn GetPointerPositionPropertySet<'a, P0>(
        targetelement: P0,
    ) -> ::windows::core::Result<super::super::Composition::CompositionPropertySet>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::UIElement>>,
    {
        Self::IElementCompositionPreviewStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetPointerPositionPropertySet)(
                ::windows::core::Vtable::as_raw(this),
                targetelement.into().abi(),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::super::Composition::CompositionPropertySet>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IElementCompositionPreviewStatics<
        R,
        F: FnOnce(&IElementCompositionPreviewStatics) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<
            ElementCompositionPreview,
            IElementCompositionPreviewStatics,
        > = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for ElementCompositionPreview {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ElementCompositionPreview {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ElementCompositionPreview {}
impl ::core::fmt::Debug for ElementCompositionPreview {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ElementCompositionPreview").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ElementCompositionPreview {
    const SIGNATURE : ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice ( b"rc(Microsoft.UI.Xaml.Hosting.ElementCompositionPreview;{c8ad1ef4-a93f-5a25-85bd-7c498d9856d3})" ) ;
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ElementCompositionPreview {
    type Vtable = IElementCompositionPreview_Vtbl;
}
unsafe impl ::windows::core::Interface for ElementCompositionPreview {
    const IID: ::windows::core::GUID =
        <IElementCompositionPreview as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ElementCompositionPreview {
    const NAME: &'static str = "Microsoft.UI.Xaml.Hosting.ElementCompositionPreview";
}
::windows::core::interface_hierarchy!(
    ElementCompositionPreview,
    ::windows::core::IUnknown,
    ::windows::core::IInspectable
);
unsafe impl ::core::marker::Send for ElementCompositionPreview {}
unsafe impl ::core::marker::Sync for ElementCompositionPreview {}
#[doc = "*Required features: `\"UI_Xaml_Hosting\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct __MIDL___MIDL_itf_microsoft2Eui2Examl2Ehosting2Ereferencetracker_0000_0004_0001(pub i32);
#[doc = "*Required features: `\"UI_Xaml_Hosting\"`*"]
pub const XAML_REFERENCETRACKER_DISCONNECT_DEFAULT:
    __MIDL___MIDL_itf_microsoft2Eui2Examl2Ehosting2Ereferencetracker_0000_0004_0001 =
    __MIDL___MIDL_itf_microsoft2Eui2Examl2Ehosting2Ereferencetracker_0000_0004_0001(0i32);
#[doc = "*Required features: `\"UI_Xaml_Hosting\"`*"]
pub const XAML_REFERENCETRACKER_DISCONNECT_SUSPEND:
    __MIDL___MIDL_itf_microsoft2Eui2Examl2Ehosting2Ereferencetracker_0000_0004_0001 =
    __MIDL___MIDL_itf_microsoft2Eui2Examl2Ehosting2Ereferencetracker_0000_0004_0001(1i32);
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
unsafe impl ::windows::core::Abi
    for __MIDL___MIDL_itf_microsoft2Eui2Examl2Ehosting2Ereferencetracker_0000_0004_0001
{
    type Abi = Self;
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
#[doc = "*Required features: `\"UI_Xaml_Hosting\"`*"]
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
unsafe impl ::windows::core::Abi for TrackerHandle__ {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for TrackerHandle__ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            ::windows::core::memcmp(
                self as *const _ as _,
                other as *const _ as _,
                core::mem::size_of::<TrackerHandle__>(),
            ) == 0
        }
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
