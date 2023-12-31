#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IAppWindow(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppWindow {
    type Vtable = IAppWindow_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IAppWindow {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xcfa788b3_643b_5c5e_ad4e_321d48a82acd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppWindow_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::WindowId,
    ) -> ::windows_core::HRESULT,
    pub IsShownInSwitchers: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetIsShownInSwitchers: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
    pub IsVisible: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub OwnerWindowId: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::WindowId,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_Graphics")]
    pub Position: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Graphics::PointInt32,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Graphics"))]
    Position: usize,
    pub Presenter: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_Graphics")]
    pub Size: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Graphics::SizeInt32,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Graphics"))]
    Size: usize,
    pub Title: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub SetTitle: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub TitleBar: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub Destroy:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Hide: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_Graphics")]
    pub Move: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        position: ::windows::Graphics::PointInt32,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Graphics"))]
    Move: usize,
    #[cfg(feature = "Windows_Graphics")]
    pub MoveAndResize: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        rect: ::windows::Graphics::RectInt32,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Graphics"))]
    MoveAndResize: usize,
    #[cfg(feature = "Windows_Graphics")]
    pub MoveAndResizeRelativeToDisplayArea: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        rect: ::windows::Graphics::RectInt32,
        displayarea: *mut ::core::ffi::c_void,
    )
        -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Graphics"))]
    MoveAndResizeRelativeToDisplayArea: usize,
    #[cfg(feature = "Windows_Graphics")]
    pub Resize: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        size: ::windows::Graphics::SizeInt32,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Graphics"))]
    Resize: usize,
    pub SetIcon: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        iconpath: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub SetIconWithIconId: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        iconid: super::IconId,
    ) -> ::windows_core::HRESULT,
    pub SetPresenter: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        appwindowpresenter: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetPresenterByKind: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        appwindowpresenterkind: AppWindowPresenterKind,
    ) -> ::windows_core::HRESULT,
    pub Show: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub ShowWithActivation: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        activatewindow: bool,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_Foundation")]
    pub Changed: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    Changed: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub RemoveChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RemoveChanged: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub Closing: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    Closing: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub RemoveClosing: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RemoveClosing: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub Destroying: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    Destroying: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub RemoveDestroying: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RemoveDestroying: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IAppWindow2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppWindow2 {
    type Vtable = IAppWindow2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IAppWindow2 {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x6cd41292_794c_5cac_8961_210d012c6ebc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppWindow2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Windows_Graphics")]
    pub ClientSize: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Graphics::SizeInt32,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Graphics"))]
    ClientSize: usize,
    pub MoveInZOrderAtBottom:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub MoveInZOrderAtTop:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub MoveInZOrderBelow: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        windowid: super::WindowId,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_Graphics")]
    pub ResizeClient: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        size: ::windows::Graphics::SizeInt32,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Graphics"))]
    ResizeClient: usize,
    pub ShowOnceWithRequestedStartupState:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IAppWindow3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppWindow3 {
    type Vtable = IAppWindow3_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IAppWindow3 {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x2f260cea_193d_5dd6_a904_d7649a608d2f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppWindow3_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Microsoft_UI_Dispatching")]
    pub AssociateWithDispatcherQueue: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        dispatcherqueue: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Microsoft_UI_Dispatching"))]
    AssociateWithDispatcherQueue: usize,
    #[cfg(feature = "Microsoft_UI_Dispatching")]
    pub DispatcherQueue: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Microsoft_UI_Dispatching"))]
    DispatcherQueue: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IAppWindowChangedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppWindowChangedEventArgs {
    type Vtable = IAppWindowChangedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IAppWindowChangedEventArgs {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x2182bc5d_fdac_5c3e_bf37_7d8d684e9d1d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppWindowChangedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub DidPositionChange: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub DidPresenterChange: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub DidSizeChange: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub DidVisibilityChange: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IAppWindowChangedEventArgs2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppWindowChangedEventArgs2 {
    type Vtable = IAppWindowChangedEventArgs2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IAppWindowChangedEventArgs2 {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xa773ab4c_a5ec_50e8_98ac_247fe6cd4227);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppWindowChangedEventArgs2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub DidZOrderChange: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub IsZOrderAtBottom: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub IsZOrderAtTop: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub ZOrderBelowWindowId: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::WindowId,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IAppWindowClosingEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppWindowClosingEventArgs {
    type Vtable = IAppWindowClosingEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IAppWindowClosingEventArgs {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x0e09d90b_2261_590b_9ad1_8504991d8754);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppWindowClosingEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Cancel: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetCancel: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IAppWindowPresenter(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppWindowPresenter {
    type Vtable = IAppWindowPresenter_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IAppWindowPresenter {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xbc3042c2_c6c6_5632_8989_ff0ec6d3b40d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppWindowPresenter_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Kind: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut AppWindowPresenterKind,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IAppWindowPresenterFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppWindowPresenterFactory {
    type Vtable = IAppWindowPresenterFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IAppWindowPresenterFactory {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x62082e3c_1368_5238_90d1_e932dc718a82);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppWindowPresenterFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IAppWindowStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppWindowStatics {
    type Vtable = IAppWindowStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IAppWindowStatics {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x3c315c24_d540_5d72_b518_b226b83627cb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppWindowStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub CreateWithPresenter: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        appwindowpresenter: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub CreateWithPresenterAndOwner: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        appwindowpresenter: *mut ::core::ffi::c_void,
        ownerwindowid: super::WindowId,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub GetFromWindowId: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        windowid: super::WindowId,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IAppWindowStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppWindowStatics2 {
    type Vtable = IAppWindowStatics2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IAppWindowStatics2 {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xcabc23db_4606_5d6e_89a5_06de1d8bd3e2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppWindowStatics2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Microsoft_UI_Dispatching")]
    pub CreateWithDispatcherQueue: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        appwindowpresenter: *mut ::core::ffi::c_void,
        ownerwindowid: super::WindowId,
        dispatcherqueue: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Microsoft_UI_Dispatching"))]
    CreateWithDispatcherQueue: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IAppWindowTitleBar(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppWindowTitleBar {
    type Vtable = IAppWindowTitleBar_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IAppWindowTitleBar {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x5574efa2_c91c_5700_a363_539c71a7aaf4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppWindowTitleBar_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Windows_Foundation", feature = "Windows_UI"))]
    pub BackgroundColor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Windows_Foundation", feature = "Windows_UI")))]
    BackgroundColor: usize,
    #[cfg(all(feature = "Windows_Foundation", feature = "Windows_UI"))]
    pub SetBackgroundColor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Windows_Foundation", feature = "Windows_UI")))]
    SetBackgroundColor: usize,
    #[cfg(all(feature = "Windows_Foundation", feature = "Windows_UI"))]
    pub ButtonBackgroundColor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Windows_Foundation", feature = "Windows_UI")))]
    ButtonBackgroundColor: usize,
    #[cfg(all(feature = "Windows_Foundation", feature = "Windows_UI"))]
    pub SetButtonBackgroundColor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Windows_Foundation", feature = "Windows_UI")))]
    SetButtonBackgroundColor: usize,
    #[cfg(all(feature = "Windows_Foundation", feature = "Windows_UI"))]
    pub ButtonForegroundColor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Windows_Foundation", feature = "Windows_UI")))]
    ButtonForegroundColor: usize,
    #[cfg(all(feature = "Windows_Foundation", feature = "Windows_UI"))]
    pub SetButtonForegroundColor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Windows_Foundation", feature = "Windows_UI")))]
    SetButtonForegroundColor: usize,
    #[cfg(all(feature = "Windows_Foundation", feature = "Windows_UI"))]
    pub ButtonHoverBackgroundColor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Windows_Foundation", feature = "Windows_UI")))]
    ButtonHoverBackgroundColor: usize,
    #[cfg(all(feature = "Windows_Foundation", feature = "Windows_UI"))]
    pub SetButtonHoverBackgroundColor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Windows_Foundation", feature = "Windows_UI")))]
    SetButtonHoverBackgroundColor: usize,
    #[cfg(all(feature = "Windows_Foundation", feature = "Windows_UI"))]
    pub ButtonHoverForegroundColor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Windows_Foundation", feature = "Windows_UI")))]
    ButtonHoverForegroundColor: usize,
    #[cfg(all(feature = "Windows_Foundation", feature = "Windows_UI"))]
    pub SetButtonHoverForegroundColor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Windows_Foundation", feature = "Windows_UI")))]
    SetButtonHoverForegroundColor: usize,
    #[cfg(all(feature = "Windows_Foundation", feature = "Windows_UI"))]
    pub ButtonInactiveBackgroundColor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Windows_Foundation", feature = "Windows_UI")))]
    ButtonInactiveBackgroundColor: usize,
    #[cfg(all(feature = "Windows_Foundation", feature = "Windows_UI"))]
    pub SetButtonInactiveBackgroundColor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Windows_Foundation", feature = "Windows_UI")))]
    SetButtonInactiveBackgroundColor: usize,
    #[cfg(all(feature = "Windows_Foundation", feature = "Windows_UI"))]
    pub ButtonInactiveForegroundColor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Windows_Foundation", feature = "Windows_UI")))]
    ButtonInactiveForegroundColor: usize,
    #[cfg(all(feature = "Windows_Foundation", feature = "Windows_UI"))]
    pub SetButtonInactiveForegroundColor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Windows_Foundation", feature = "Windows_UI")))]
    SetButtonInactiveForegroundColor: usize,
    #[cfg(all(feature = "Windows_Foundation", feature = "Windows_UI"))]
    pub ButtonPressedBackgroundColor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Windows_Foundation", feature = "Windows_UI")))]
    ButtonPressedBackgroundColor: usize,
    #[cfg(all(feature = "Windows_Foundation", feature = "Windows_UI"))]
    pub SetButtonPressedBackgroundColor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Windows_Foundation", feature = "Windows_UI")))]
    SetButtonPressedBackgroundColor: usize,
    #[cfg(all(feature = "Windows_Foundation", feature = "Windows_UI"))]
    pub ButtonPressedForegroundColor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Windows_Foundation", feature = "Windows_UI")))]
    ButtonPressedForegroundColor: usize,
    #[cfg(all(feature = "Windows_Foundation", feature = "Windows_UI"))]
    pub SetButtonPressedForegroundColor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Windows_Foundation", feature = "Windows_UI")))]
    SetButtonPressedForegroundColor: usize,
    pub ExtendsContentIntoTitleBar: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetExtendsContentIntoTitleBar: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Windows_Foundation", feature = "Windows_UI"))]
    pub ForegroundColor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Windows_Foundation", feature = "Windows_UI")))]
    ForegroundColor: usize,
    #[cfg(all(feature = "Windows_Foundation", feature = "Windows_UI"))]
    pub SetForegroundColor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Windows_Foundation", feature = "Windows_UI")))]
    SetForegroundColor: usize,
    pub Height: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut i32,
    ) -> ::windows_core::HRESULT,
    pub IconShowOptions: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut IconShowOptions,
    ) -> ::windows_core::HRESULT,
    pub SetIconShowOptions: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: IconShowOptions,
    ) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Windows_Foundation", feature = "Windows_UI"))]
    pub InactiveBackgroundColor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Windows_Foundation", feature = "Windows_UI")))]
    InactiveBackgroundColor: usize,
    #[cfg(all(feature = "Windows_Foundation", feature = "Windows_UI"))]
    pub SetInactiveBackgroundColor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Windows_Foundation", feature = "Windows_UI")))]
    SetInactiveBackgroundColor: usize,
    #[cfg(all(feature = "Windows_Foundation", feature = "Windows_UI"))]
    pub InactiveForegroundColor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Windows_Foundation", feature = "Windows_UI")))]
    InactiveForegroundColor: usize,
    #[cfg(all(feature = "Windows_Foundation", feature = "Windows_UI"))]
    pub SetInactiveForegroundColor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Windows_Foundation", feature = "Windows_UI")))]
    SetInactiveForegroundColor: usize,
    pub LeftInset: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut i32,
    ) -> ::windows_core::HRESULT,
    pub RightInset: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut i32,
    ) -> ::windows_core::HRESULT,
    pub ResetToDefault:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Windows_Graphics", feature = "deprecated"))]
    pub SetDragRectangles: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value_array_size: u32,
        value: *const ::windows::Graphics::RectInt32,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Windows_Graphics", feature = "deprecated")))]
    SetDragRectangles: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IAppWindowTitleBar2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppWindowTitleBar2 {
    type Vtable = IAppWindowTitleBar2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IAppWindowTitleBar2 {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x86faed38_748a_5b4b_9ccf_3ba0496c9041);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppWindowTitleBar2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub PreferredHeightOption: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut TitleBarHeightOption,
    ) -> ::windows_core::HRESULT,
    pub SetPreferredHeightOption: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: TitleBarHeightOption,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IAppWindowTitleBarStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppWindowTitleBarStatics {
    type Vtable = IAppWindowTitleBarStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IAppWindowTitleBarStatics {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x9e1da52e_8b15_54d6_a886_f7b9f9d930b2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppWindowTitleBarStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub IsCustomizationSupported: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICompactOverlayPresenter(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICompactOverlayPresenter {
    type Vtable = ICompactOverlayPresenter_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICompactOverlayPresenter {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xefeb0812_6fc7_5b7d_bd92_cc8f9a6454c9);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompactOverlayPresenter_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub InitialSize: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut CompactOverlaySize,
    ) -> ::windows_core::HRESULT,
    pub SetInitialSize: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: CompactOverlaySize,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICompactOverlayPresenterStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICompactOverlayPresenterStatics {
    type Vtable = ICompactOverlayPresenterStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICompactOverlayPresenterStatics {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xeab93186_4f6a_52f9_8c03_da57a1522f6e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompactOverlayPresenterStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDisplayArea(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDisplayArea {
    type Vtable = IDisplayArea_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDisplayArea {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x5c7e0537_b621_5579_bcae_a84aa8746167);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDisplayArea_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub DisplayId: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::DisplayId,
    ) -> ::windows_core::HRESULT,
    pub IsPrimary: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_Graphics")]
    pub OuterBounds: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Graphics::RectInt32,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Graphics"))]
    OuterBounds: usize,
    #[cfg(feature = "Windows_Graphics")]
    pub WorkArea: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Graphics::RectInt32,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Graphics"))]
    WorkArea: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDisplayAreaStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDisplayAreaStatics {
    type Vtable = IDisplayAreaStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDisplayAreaStatics {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x02ab4926_211e_5d49_8e4b_2af193daed09);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDisplayAreaStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Primary: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub CreateWatcher: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub FindAll: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation_Collections"))]
    FindAll: usize,
    pub GetFromWindowId: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        windowid: super::WindowId,
        displayareafallback: DisplayAreaFallback,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_Graphics")]
    pub GetFromPoint: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        point: ::windows::Graphics::PointInt32,
        displayareafallback: DisplayAreaFallback,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Graphics"))]
    GetFromPoint: usize,
    #[cfg(feature = "Windows_Graphics")]
    pub GetFromRect: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        rect: ::windows::Graphics::RectInt32,
        displayareafallback: DisplayAreaFallback,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Graphics"))]
    GetFromRect: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDisplayAreaStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDisplayAreaStatics2 {
    type Vtable = IDisplayAreaStatics2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDisplayAreaStatics2 {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x7207ad4b_890d_5dd7_bc18_78ffd9544d8f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDisplayAreaStatics2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub GetFromDisplayId: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        displayid: super::DisplayId,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDisplayAreaWatcher(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDisplayAreaWatcher {
    type Vtable = IDisplayAreaWatcher_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDisplayAreaWatcher {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x83f6562f_d3a0_548b_8e4f_a99be3d95c9c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDisplayAreaWatcher_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut DisplayAreaWatcherStatus,
    ) -> ::windows_core::HRESULT,
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_Foundation")]
    pub Added: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    Added: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub RemoveAdded: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RemoveAdded: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub EnumerationCompleted: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    EnumerationCompleted: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub RemoveEnumerationCompleted: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RemoveEnumerationCompleted: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub Removed: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    Removed: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub RemoveRemoved: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RemoveRemoved: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub Stopped: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    Stopped: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub RemoveStopped: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RemoveStopped: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub Updated: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    Updated: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub RemoveUpdated: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RemoveUpdated: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IFullScreenPresenter(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFullScreenPresenter {
    type Vtable = IFullScreenPresenter_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IFullScreenPresenter {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xfa9141fd_b8dd_5da1_8b2b_7cdadb76f593);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFullScreenPresenter_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IFullScreenPresenterStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFullScreenPresenterStatics {
    type Vtable = IFullScreenPresenterStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IFullScreenPresenterStatics {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x2ec0d2c1_e086_55bb_a3b2_44942e231c67);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFullScreenPresenterStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IOverlappedPresenter(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IOverlappedPresenter {
    type Vtable = IOverlappedPresenter_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IOverlappedPresenter {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x21693970_4f4c_5172_9e9d_682a2d174884);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOverlappedPresenter_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub HasBorder: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub HasTitleBar: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub IsAlwaysOnTop: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetIsAlwaysOnTop: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
    pub IsMaximizable: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetIsMaximizable: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
    pub IsMinimizable: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetIsMinimizable: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
    pub IsModal: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetIsModal: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
    pub IsResizable: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetIsResizable: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
    pub State: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut OverlappedPresenterState,
    ) -> ::windows_core::HRESULT,
    pub Maximize:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Minimize:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Restore:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetBorderAndTitleBar: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        hasborder: bool,
        hastitlebar: bool,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IOverlappedPresenter2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IOverlappedPresenter2 {
    type Vtable = IOverlappedPresenter2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IOverlappedPresenter2 {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x5c6ccd93_4244_5cd2_b355_ed5ea34df730);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOverlappedPresenter2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub MinimizeWithActivation: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        activatewindow: bool,
    ) -> ::windows_core::HRESULT,
    pub RestoreWithActivation: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        activatewindow: bool,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IOverlappedPresenterStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IOverlappedPresenterStatics {
    type Vtable = IOverlappedPresenterStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IOverlappedPresenterStatics {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x997225e4_7b00_5aee_a4be_d4068d1999e2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOverlappedPresenterStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub CreateForContextMenu: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub CreateForDialog: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub CreateForToolWindow: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IOverlappedPresenterStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IOverlappedPresenterStatics2 {
    type Vtable = IOverlappedPresenterStatics2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IOverlappedPresenterStatics2 {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xed5c4f92_32f4_5d15_80d0_b2a5efa04d39);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOverlappedPresenterStatics2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub RequestedStartupState: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut OverlappedPresenterState,
    ) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct AppWindow(::windows_core::IUnknown);
impl AppWindow {
    pub fn Id(&self) -> ::windows_core::Result<super::WindowId> {
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
    pub fn IsShownInSwitchers(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsShownInSwitchers)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetIsShownInSwitchers(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetIsShownInSwitchers)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
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
    pub fn OwnerWindowId(&self) -> ::windows_core::Result<super::WindowId> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OwnerWindowId)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Graphics\"`"]
    #[cfg(feature = "Windows_Graphics")]
    pub fn Position(&self) -> ::windows_core::Result<::windows::Graphics::PointInt32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Position)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn Presenter(&self) -> ::windows_core::Result<AppWindowPresenter> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Presenter)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Graphics\"`"]
    #[cfg(feature = "Windows_Graphics")]
    pub fn Size(&self) -> ::windows_core::Result<::windows::Graphics::SizeInt32> {
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
    pub fn TitleBar(&self) -> ::windows_core::Result<AppWindowTitleBar> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TitleBar)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn Destroy(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).Destroy)(::windows_core::Interface::as_raw(
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
    pub fn Move(&self, position: ::windows::Graphics::PointInt32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).Move)(
                ::windows_core::Interface::as_raw(this),
                position,
            )
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
    #[doc = "Required features: `\"Windows_Graphics\"`"]
    #[cfg(feature = "Windows_Graphics")]
    pub fn MoveAndResizeRelativeToDisplayArea<P0>(
        &self,
        rect: ::windows::Graphics::RectInt32,
        displayarea: P0,
    ) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<DisplayArea>,
    {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).MoveAndResizeRelativeToDisplayArea)(
                ::windows_core::Interface::as_raw(this),
                rect,
                displayarea.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Graphics\"`"]
    #[cfg(feature = "Windows_Graphics")]
    pub fn Resize(&self, size: ::windows::Graphics::SizeInt32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).Resize)(
                ::windows_core::Interface::as_raw(this),
                size,
            )
            .ok()
        }
    }
    pub fn SetIcon(&self, iconpath: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetIcon)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(iconpath),
            )
            .ok()
        }
    }
    pub fn SetIconWithIconId(&self, iconid: super::IconId) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetIconWithIconId)(
                ::windows_core::Interface::as_raw(this),
                iconid,
            )
            .ok()
        }
    }
    pub fn SetPresenter<P0>(&self, appwindowpresenter: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<AppWindowPresenter>,
    {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetPresenter)(
                ::windows_core::Interface::as_raw(this),
                appwindowpresenter.try_into_param()?.abi(),
            )
            .ok()
        }
    }
    pub fn SetPresenterByKind(
        &self,
        appwindowpresenterkind: AppWindowPresenterKind,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetPresenterByKind)(
                ::windows_core::Interface::as_raw(this),
                appwindowpresenterkind,
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
    pub fn ShowWithActivation(&self, activatewindow: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).ShowWithActivation)(
                ::windows_core::Interface::as_raw(this),
                activatewindow,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn Changed<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<AppWindow, AppWindowChangedEventArgs>,
        >,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Changed)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveChanged(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveChanged)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn Closing<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<AppWindow, AppWindowClosingEventArgs>,
        >,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Closing)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveClosing(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveClosing)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn Destroying<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<AppWindow, ::windows_core::IInspectable>,
        >,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Destroying)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveDestroying(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveDestroying)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Graphics\"`"]
    #[cfg(feature = "Windows_Graphics")]
    pub fn ClientSize(&self) -> ::windows_core::Result<::windows::Graphics::SizeInt32> {
        let this = &::windows_core::ComInterface::cast::<IAppWindow2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ClientSize)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn MoveInZOrderAtBottom(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IAppWindow2>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).MoveInZOrderAtBottom)(
                ::windows_core::Interface::as_raw(this),
            )
            .ok()
        }
    }
    pub fn MoveInZOrderAtTop(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IAppWindow2>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).MoveInZOrderAtTop)(
                ::windows_core::Interface::as_raw(this),
            )
            .ok()
        }
    }
    pub fn MoveInZOrderBelow(&self, windowid: super::WindowId) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IAppWindow2>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).MoveInZOrderBelow)(
                ::windows_core::Interface::as_raw(this),
                windowid,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Graphics\"`"]
    #[cfg(feature = "Windows_Graphics")]
    pub fn ResizeClient(&self, size: ::windows::Graphics::SizeInt32) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IAppWindow2>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).ResizeClient)(
                ::windows_core::Interface::as_raw(this),
                size,
            )
            .ok()
        }
    }
    pub fn ShowOnceWithRequestedStartupState(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IAppWindow2>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).ShowOnceWithRequestedStartupState)(
                ::windows_core::Interface::as_raw(this),
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Dispatching\"`"]
    #[cfg(feature = "Microsoft_UI_Dispatching")]
    pub fn AssociateWithDispatcherQueue<P0>(
        &self,
        dispatcherqueue: P0,
    ) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::Dispatching::DispatcherQueue>,
    {
        let this = &::windows_core::ComInterface::cast::<IAppWindow3>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).AssociateWithDispatcherQueue)(
                ::windows_core::Interface::as_raw(this),
                dispatcherqueue.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Dispatching\"`"]
    #[cfg(feature = "Microsoft_UI_Dispatching")]
    pub fn DispatcherQueue(&self) -> ::windows_core::Result<super::Dispatching::DispatcherQueue> {
        let this = &::windows_core::ComInterface::cast::<IAppWindow3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DispatcherQueue)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn Create() -> ::windows_core::Result<AppWindow> {
        Self::IAppWindowStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Create)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn CreateWithPresenter<P0>(appwindowpresenter: P0) -> ::windows_core::Result<AppWindow>
    where
        P0: ::windows_core::TryIntoParam<AppWindowPresenter>,
    {
        Self::IAppWindowStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateWithPresenter)(
                ::windows_core::Interface::as_raw(this),
                appwindowpresenter.try_into_param()?.abi(),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn CreateWithPresenterAndOwner<P0>(
        appwindowpresenter: P0,
        ownerwindowid: super::WindowId,
    ) -> ::windows_core::Result<AppWindow>
    where
        P0: ::windows_core::TryIntoParam<AppWindowPresenter>,
    {
        Self::IAppWindowStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateWithPresenterAndOwner)(
                ::windows_core::Interface::as_raw(this),
                appwindowpresenter.try_into_param()?.abi(),
                ownerwindowid,
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn GetFromWindowId(windowid: super::WindowId) -> ::windows_core::Result<AppWindow> {
        Self::IAppWindowStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetFromWindowId)(
                ::windows_core::Interface::as_raw(this),
                windowid,
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Microsoft_UI_Dispatching\"`"]
    #[cfg(feature = "Microsoft_UI_Dispatching")]
    pub fn CreateWithDispatcherQueue<P0, P1>(
        appwindowpresenter: P0,
        ownerwindowid: super::WindowId,
        dispatcherqueue: P1,
    ) -> ::windows_core::Result<AppWindow>
    where
        P0: ::windows_core::TryIntoParam<AppWindowPresenter>,
        P1: ::windows_core::IntoParam<super::Dispatching::DispatcherQueue>,
    {
        Self::IAppWindowStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateWithDispatcherQueue)(
                ::windows_core::Interface::as_raw(this),
                appwindowpresenter.try_into_param()?.abi(),
                ownerwindowid,
                dispatcherqueue.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IAppWindowStatics<R, F: FnOnce(&IAppWindowStatics) -> ::windows_core::Result<R>>(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<AppWindow, IAppWindowStatics> =
            ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IAppWindowStatics2<R, F: FnOnce(&IAppWindowStatics2) -> ::windows_core::Result<R>>(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<AppWindow, IAppWindowStatics2> =
            ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for AppWindow {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for AppWindow {
    type Vtable = IAppWindow_Vtbl;
}
unsafe impl ::windows_core::ComInterface for AppWindow {
    const IID: ::windows_core::GUID = <IAppWindow as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for AppWindow {
    const NAME: &'static str = "Microsoft.UI.Windowing.AppWindow";
}
::windows_core::imp::interface_hierarchy!(
    AppWindow,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for AppWindow {}
unsafe impl ::core::marker::Sync for AppWindow {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct AppWindowChangedEventArgs(::windows_core::IUnknown);
impl AppWindowChangedEventArgs {
    pub fn DidPositionChange(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DidPositionChange)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn DidPresenterChange(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DidPresenterChange)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn DidSizeChange(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DidSizeChange)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn DidVisibilityChange(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DidVisibilityChange)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn DidZOrderChange(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IAppWindowChangedEventArgs2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DidZOrderChange)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn IsZOrderAtBottom(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IAppWindowChangedEventArgs2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsZOrderAtBottom)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn IsZOrderAtTop(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IAppWindowChangedEventArgs2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsZOrderAtTop)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn ZOrderBelowWindowId(&self) -> ::windows_core::Result<super::WindowId> {
        let this = &::windows_core::ComInterface::cast::<IAppWindowChangedEventArgs2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ZOrderBelowWindowId)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for AppWindowChangedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for AppWindowChangedEventArgs {
    type Vtable = IAppWindowChangedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for AppWindowChangedEventArgs {
    const IID: ::windows_core::GUID =
        <IAppWindowChangedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for AppWindowChangedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Windowing.AppWindowChangedEventArgs";
}
::windows_core::imp::interface_hierarchy!(
    AppWindowChangedEventArgs,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for AppWindowChangedEventArgs {}
unsafe impl ::core::marker::Sync for AppWindowChangedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct AppWindowClosingEventArgs(::windows_core::IUnknown);
impl AppWindowClosingEventArgs {
    pub fn Cancel(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Cancel)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetCancel(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetCancel)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
}
impl ::windows_core::RuntimeType for AppWindowClosingEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for AppWindowClosingEventArgs {
    type Vtable = IAppWindowClosingEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for AppWindowClosingEventArgs {
    const IID: ::windows_core::GUID =
        <IAppWindowClosingEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for AppWindowClosingEventArgs {
    const NAME: &'static str = "Microsoft.UI.Windowing.AppWindowClosingEventArgs";
}
::windows_core::imp::interface_hierarchy!(
    AppWindowClosingEventArgs,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for AppWindowClosingEventArgs {}
unsafe impl ::core::marker::Sync for AppWindowClosingEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct AppWindowPresenter(::windows_core::IUnknown);
impl AppWindowPresenter {
    pub fn Kind(&self) -> ::windows_core::Result<AppWindowPresenterKind> {
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
}
impl ::windows_core::RuntimeType for AppWindowPresenter {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for AppWindowPresenter {
    type Vtable = IAppWindowPresenter_Vtbl;
}
unsafe impl ::windows_core::ComInterface for AppWindowPresenter {
    const IID: ::windows_core::GUID = <IAppWindowPresenter as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for AppWindowPresenter {
    const NAME: &'static str = "Microsoft.UI.Windowing.AppWindowPresenter";
}
::windows_core::imp::interface_hierarchy!(
    AppWindowPresenter,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for AppWindowPresenter {}
unsafe impl ::core::marker::Sync for AppWindowPresenter {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct AppWindowTitleBar(::windows_core::IUnknown);
impl AppWindowTitleBar {
    #[doc = "Required features: `\"Windows_Foundation\"`, `\"Windows_UI\"`"]
    #[cfg(all(feature = "Windows_Foundation", feature = "Windows_UI"))]
    pub fn BackgroundColor(
        &self,
    ) -> ::windows_core::Result<::windows::Foundation::IReference<::windows::UI::Color>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).BackgroundColor)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`, `\"Windows_UI\"`"]
    #[cfg(all(feature = "Windows_Foundation", feature = "Windows_UI"))]
    pub fn SetBackgroundColor<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<::windows::Foundation::IReference<::windows::UI::Color>>,
    {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetBackgroundColor)(
                ::windows_core::Interface::as_raw(this),
                value.try_into_param()?.abi(),
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`, `\"Windows_UI\"`"]
    #[cfg(all(feature = "Windows_Foundation", feature = "Windows_UI"))]
    pub fn ButtonBackgroundColor(
        &self,
    ) -> ::windows_core::Result<::windows::Foundation::IReference<::windows::UI::Color>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ButtonBackgroundColor)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`, `\"Windows_UI\"`"]
    #[cfg(all(feature = "Windows_Foundation", feature = "Windows_UI"))]
    pub fn SetButtonBackgroundColor<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<::windows::Foundation::IReference<::windows::UI::Color>>,
    {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetButtonBackgroundColor)(
                ::windows_core::Interface::as_raw(this),
                value.try_into_param()?.abi(),
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`, `\"Windows_UI\"`"]
    #[cfg(all(feature = "Windows_Foundation", feature = "Windows_UI"))]
    pub fn ButtonForegroundColor(
        &self,
    ) -> ::windows_core::Result<::windows::Foundation::IReference<::windows::UI::Color>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ButtonForegroundColor)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`, `\"Windows_UI\"`"]
    #[cfg(all(feature = "Windows_Foundation", feature = "Windows_UI"))]
    pub fn SetButtonForegroundColor<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<::windows::Foundation::IReference<::windows::UI::Color>>,
    {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetButtonForegroundColor)(
                ::windows_core::Interface::as_raw(this),
                value.try_into_param()?.abi(),
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`, `\"Windows_UI\"`"]
    #[cfg(all(feature = "Windows_Foundation", feature = "Windows_UI"))]
    pub fn ButtonHoverBackgroundColor(
        &self,
    ) -> ::windows_core::Result<::windows::Foundation::IReference<::windows::UI::Color>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ButtonHoverBackgroundColor)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`, `\"Windows_UI\"`"]
    #[cfg(all(feature = "Windows_Foundation", feature = "Windows_UI"))]
    pub fn SetButtonHoverBackgroundColor<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<::windows::Foundation::IReference<::windows::UI::Color>>,
    {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetButtonHoverBackgroundColor)(
                ::windows_core::Interface::as_raw(this),
                value.try_into_param()?.abi(),
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`, `\"Windows_UI\"`"]
    #[cfg(all(feature = "Windows_Foundation", feature = "Windows_UI"))]
    pub fn ButtonHoverForegroundColor(
        &self,
    ) -> ::windows_core::Result<::windows::Foundation::IReference<::windows::UI::Color>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ButtonHoverForegroundColor)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`, `\"Windows_UI\"`"]
    #[cfg(all(feature = "Windows_Foundation", feature = "Windows_UI"))]
    pub fn SetButtonHoverForegroundColor<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<::windows::Foundation::IReference<::windows::UI::Color>>,
    {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetButtonHoverForegroundColor)(
                ::windows_core::Interface::as_raw(this),
                value.try_into_param()?.abi(),
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`, `\"Windows_UI\"`"]
    #[cfg(all(feature = "Windows_Foundation", feature = "Windows_UI"))]
    pub fn ButtonInactiveBackgroundColor(
        &self,
    ) -> ::windows_core::Result<::windows::Foundation::IReference<::windows::UI::Color>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ButtonInactiveBackgroundColor)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`, `\"Windows_UI\"`"]
    #[cfg(all(feature = "Windows_Foundation", feature = "Windows_UI"))]
    pub fn SetButtonInactiveBackgroundColor<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<::windows::Foundation::IReference<::windows::UI::Color>>,
    {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetButtonInactiveBackgroundColor)(
                ::windows_core::Interface::as_raw(this),
                value.try_into_param()?.abi(),
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`, `\"Windows_UI\"`"]
    #[cfg(all(feature = "Windows_Foundation", feature = "Windows_UI"))]
    pub fn ButtonInactiveForegroundColor(
        &self,
    ) -> ::windows_core::Result<::windows::Foundation::IReference<::windows::UI::Color>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ButtonInactiveForegroundColor)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`, `\"Windows_UI\"`"]
    #[cfg(all(feature = "Windows_Foundation", feature = "Windows_UI"))]
    pub fn SetButtonInactiveForegroundColor<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<::windows::Foundation::IReference<::windows::UI::Color>>,
    {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetButtonInactiveForegroundColor)(
                ::windows_core::Interface::as_raw(this),
                value.try_into_param()?.abi(),
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`, `\"Windows_UI\"`"]
    #[cfg(all(feature = "Windows_Foundation", feature = "Windows_UI"))]
    pub fn ButtonPressedBackgroundColor(
        &self,
    ) -> ::windows_core::Result<::windows::Foundation::IReference<::windows::UI::Color>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ButtonPressedBackgroundColor)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`, `\"Windows_UI\"`"]
    #[cfg(all(feature = "Windows_Foundation", feature = "Windows_UI"))]
    pub fn SetButtonPressedBackgroundColor<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<::windows::Foundation::IReference<::windows::UI::Color>>,
    {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetButtonPressedBackgroundColor)(
                ::windows_core::Interface::as_raw(this),
                value.try_into_param()?.abi(),
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`, `\"Windows_UI\"`"]
    #[cfg(all(feature = "Windows_Foundation", feature = "Windows_UI"))]
    pub fn ButtonPressedForegroundColor(
        &self,
    ) -> ::windows_core::Result<::windows::Foundation::IReference<::windows::UI::Color>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ButtonPressedForegroundColor)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`, `\"Windows_UI\"`"]
    #[cfg(all(feature = "Windows_Foundation", feature = "Windows_UI"))]
    pub fn SetButtonPressedForegroundColor<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<::windows::Foundation::IReference<::windows::UI::Color>>,
    {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetButtonPressedForegroundColor)(
                ::windows_core::Interface::as_raw(this),
                value.try_into_param()?.abi(),
            )
            .ok()
        }
    }
    pub fn ExtendsContentIntoTitleBar(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ExtendsContentIntoTitleBar)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetExtendsContentIntoTitleBar(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetExtendsContentIntoTitleBar)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`, `\"Windows_UI\"`"]
    #[cfg(all(feature = "Windows_Foundation", feature = "Windows_UI"))]
    pub fn ForegroundColor(
        &self,
    ) -> ::windows_core::Result<::windows::Foundation::IReference<::windows::UI::Color>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ForegroundColor)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`, `\"Windows_UI\"`"]
    #[cfg(all(feature = "Windows_Foundation", feature = "Windows_UI"))]
    pub fn SetForegroundColor<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<::windows::Foundation::IReference<::windows::UI::Color>>,
    {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetForegroundColor)(
                ::windows_core::Interface::as_raw(this),
                value.try_into_param()?.abi(),
            )
            .ok()
        }
    }
    pub fn Height(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Height)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn IconShowOptions(&self) -> ::windows_core::Result<IconShowOptions> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IconShowOptions)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetIconShowOptions(&self, value: IconShowOptions) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetIconShowOptions)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`, `\"Windows_UI\"`"]
    #[cfg(all(feature = "Windows_Foundation", feature = "Windows_UI"))]
    pub fn InactiveBackgroundColor(
        &self,
    ) -> ::windows_core::Result<::windows::Foundation::IReference<::windows::UI::Color>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).InactiveBackgroundColor)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`, `\"Windows_UI\"`"]
    #[cfg(all(feature = "Windows_Foundation", feature = "Windows_UI"))]
    pub fn SetInactiveBackgroundColor<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<::windows::Foundation::IReference<::windows::UI::Color>>,
    {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetInactiveBackgroundColor)(
                ::windows_core::Interface::as_raw(this),
                value.try_into_param()?.abi(),
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`, `\"Windows_UI\"`"]
    #[cfg(all(feature = "Windows_Foundation", feature = "Windows_UI"))]
    pub fn InactiveForegroundColor(
        &self,
    ) -> ::windows_core::Result<::windows::Foundation::IReference<::windows::UI::Color>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).InactiveForegroundColor)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`, `\"Windows_UI\"`"]
    #[cfg(all(feature = "Windows_Foundation", feature = "Windows_UI"))]
    pub fn SetInactiveForegroundColor<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<::windows::Foundation::IReference<::windows::UI::Color>>,
    {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetInactiveForegroundColor)(
                ::windows_core::Interface::as_raw(this),
                value.try_into_param()?.abi(),
            )
            .ok()
        }
    }
    pub fn LeftInset(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LeftInset)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn RightInset(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RightInset)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn ResetToDefault(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).ResetToDefault)(
                ::windows_core::Interface::as_raw(this),
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Graphics\"`, `\"deprecated\"`"]
    #[cfg(all(feature = "Windows_Graphics", feature = "deprecated"))]
    pub fn SetDragRectangles(
        &self,
        value: &[::windows::Graphics::RectInt32],
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetDragRectangles)(
                ::windows_core::Interface::as_raw(this),
                value.len().try_into().unwrap(),
                value.as_ptr(),
            )
            .ok()
        }
    }
    pub fn PreferredHeightOption(&self) -> ::windows_core::Result<TitleBarHeightOption> {
        let this = &::windows_core::ComInterface::cast::<IAppWindowTitleBar2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreferredHeightOption)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetPreferredHeightOption(
        &self,
        value: TitleBarHeightOption,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IAppWindowTitleBar2>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetPreferredHeightOption)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsCustomizationSupported() -> ::windows_core::Result<bool> {
        Self::IAppWindowTitleBarStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsCustomizationSupported)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IAppWindowTitleBarStatics<
        R,
        F: FnOnce(&IAppWindowTitleBarStatics) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<
            AppWindowTitleBar,
            IAppWindowTitleBarStatics,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for AppWindowTitleBar {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for AppWindowTitleBar {
    type Vtable = IAppWindowTitleBar_Vtbl;
}
unsafe impl ::windows_core::ComInterface for AppWindowTitleBar {
    const IID: ::windows_core::GUID = <IAppWindowTitleBar as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for AppWindowTitleBar {
    const NAME: &'static str = "Microsoft.UI.Windowing.AppWindowTitleBar";
}
::windows_core::imp::interface_hierarchy!(
    AppWindowTitleBar,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for AppWindowTitleBar {}
unsafe impl ::core::marker::Sync for AppWindowTitleBar {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct CompactOverlayPresenter(::windows_core::IUnknown);
impl CompactOverlayPresenter {
    pub fn Kind(&self) -> ::windows_core::Result<AppWindowPresenterKind> {
        let this = &::windows_core::ComInterface::cast::<IAppWindowPresenter>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn InitialSize(&self) -> ::windows_core::Result<CompactOverlaySize> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).InitialSize)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetInitialSize(&self, value: CompactOverlaySize) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetInitialSize)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn Create() -> ::windows_core::Result<CompactOverlayPresenter> {
        Self::ICompactOverlayPresenterStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Create)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ICompactOverlayPresenterStatics<
        R,
        F: FnOnce(&ICompactOverlayPresenterStatics) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<
            CompactOverlayPresenter,
            ICompactOverlayPresenterStatics,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for CompactOverlayPresenter {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for CompactOverlayPresenter {
    type Vtable = ICompactOverlayPresenter_Vtbl;
}
unsafe impl ::windows_core::ComInterface for CompactOverlayPresenter {
    const IID: ::windows_core::GUID =
        <ICompactOverlayPresenter as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for CompactOverlayPresenter {
    const NAME: &'static str = "Microsoft.UI.Windowing.CompactOverlayPresenter";
}
::windows_core::imp::interface_hierarchy!(
    CompactOverlayPresenter,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<AppWindowPresenter> for CompactOverlayPresenter {}
unsafe impl ::core::marker::Send for CompactOverlayPresenter {}
unsafe impl ::core::marker::Sync for CompactOverlayPresenter {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct DisplayArea(::windows_core::IUnknown);
impl DisplayArea {
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
    pub fn IsPrimary(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsPrimary)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Graphics\"`"]
    #[cfg(feature = "Windows_Graphics")]
    pub fn OuterBounds(&self) -> ::windows_core::Result<::windows::Graphics::RectInt32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OuterBounds)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Graphics\"`"]
    #[cfg(feature = "Windows_Graphics")]
    pub fn WorkArea(&self) -> ::windows_core::Result<::windows::Graphics::RectInt32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).WorkArea)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn Primary() -> ::windows_core::Result<DisplayArea> {
        Self::IDisplayAreaStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Primary)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn CreateWatcher() -> ::windows_core::Result<DisplayAreaWatcher> {
        Self::IDisplayAreaStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateWatcher)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_Foundation_Collections\"`"]
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub fn FindAll(
    ) -> ::windows_core::Result<::windows::Foundation::Collections::IVectorView<DisplayArea>> {
        Self::IDisplayAreaStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FindAll)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn GetFromWindowId(
        windowid: super::WindowId,
        displayareafallback: DisplayAreaFallback,
    ) -> ::windows_core::Result<DisplayArea> {
        Self::IDisplayAreaStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetFromWindowId)(
                ::windows_core::Interface::as_raw(this),
                windowid,
                displayareafallback,
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_Graphics\"`"]
    #[cfg(feature = "Windows_Graphics")]
    pub fn GetFromPoint(
        point: ::windows::Graphics::PointInt32,
        displayareafallback: DisplayAreaFallback,
    ) -> ::windows_core::Result<DisplayArea> {
        Self::IDisplayAreaStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetFromPoint)(
                ::windows_core::Interface::as_raw(this),
                point,
                displayareafallback,
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_Graphics\"`"]
    #[cfg(feature = "Windows_Graphics")]
    pub fn GetFromRect(
        rect: ::windows::Graphics::RectInt32,
        displayareafallback: DisplayAreaFallback,
    ) -> ::windows_core::Result<DisplayArea> {
        Self::IDisplayAreaStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetFromRect)(
                ::windows_core::Interface::as_raw(this),
                rect,
                displayareafallback,
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn GetFromDisplayId(displayid: super::DisplayId) -> ::windows_core::Result<DisplayArea> {
        Self::IDisplayAreaStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetFromDisplayId)(
                ::windows_core::Interface::as_raw(this),
                displayid,
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IDisplayAreaStatics<R, F: FnOnce(&IDisplayAreaStatics) -> ::windows_core::Result<R>>(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<DisplayArea, IDisplayAreaStatics> =
            ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IDisplayAreaStatics2<
        R,
        F: FnOnce(&IDisplayAreaStatics2) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<DisplayArea, IDisplayAreaStatics2> =
            ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for DisplayArea {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for DisplayArea {
    type Vtable = IDisplayArea_Vtbl;
}
unsafe impl ::windows_core::ComInterface for DisplayArea {
    const IID: ::windows_core::GUID = <IDisplayArea as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for DisplayArea {
    const NAME: &'static str = "Microsoft.UI.Windowing.DisplayArea";
}
::windows_core::imp::interface_hierarchy!(
    DisplayArea,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for DisplayArea {}
unsafe impl ::core::marker::Sync for DisplayArea {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct DisplayAreaWatcher(::windows_core::IUnknown);
impl DisplayAreaWatcher {
    pub fn Status(&self) -> ::windows_core::Result<DisplayAreaWatcherStatus> {
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
    pub fn Start(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).Start)(::windows_core::Interface::as_raw(this))
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
    pub fn Added<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<DisplayAreaWatcher, DisplayArea>,
        >,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Added)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveAdded(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveAdded)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn EnumerationCompleted<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<
                DisplayAreaWatcher,
                ::windows_core::IInspectable,
            >,
        >,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).EnumerationCompleted)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveEnumerationCompleted(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveEnumerationCompleted)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn Removed<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<DisplayAreaWatcher, DisplayArea>,
        >,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Removed)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveRemoved(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveRemoved)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn Stopped<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<
                DisplayAreaWatcher,
                ::windows_core::IInspectable,
            >,
        >,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Stopped)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveStopped(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveStopped)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn Updated<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<DisplayAreaWatcher, DisplayArea>,
        >,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Updated)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveUpdated(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveUpdated)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
}
impl ::windows_core::RuntimeType for DisplayAreaWatcher {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for DisplayAreaWatcher {
    type Vtable = IDisplayAreaWatcher_Vtbl;
}
unsafe impl ::windows_core::ComInterface for DisplayAreaWatcher {
    const IID: ::windows_core::GUID = <IDisplayAreaWatcher as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for DisplayAreaWatcher {
    const NAME: &'static str = "Microsoft.UI.Windowing.DisplayAreaWatcher";
}
::windows_core::imp::interface_hierarchy!(
    DisplayAreaWatcher,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for DisplayAreaWatcher {}
unsafe impl ::core::marker::Sync for DisplayAreaWatcher {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct FullScreenPresenter(::windows_core::IUnknown);
impl FullScreenPresenter {
    pub fn Kind(&self) -> ::windows_core::Result<AppWindowPresenterKind> {
        let this = &::windows_core::ComInterface::cast::<IAppWindowPresenter>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn Create() -> ::windows_core::Result<FullScreenPresenter> {
        Self::IFullScreenPresenterStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Create)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IFullScreenPresenterStatics<
        R,
        F: FnOnce(&IFullScreenPresenterStatics) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<
            FullScreenPresenter,
            IFullScreenPresenterStatics,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for FullScreenPresenter {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for FullScreenPresenter {
    type Vtable = IFullScreenPresenter_Vtbl;
}
unsafe impl ::windows_core::ComInterface for FullScreenPresenter {
    const IID: ::windows_core::GUID = <IFullScreenPresenter as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for FullScreenPresenter {
    const NAME: &'static str = "Microsoft.UI.Windowing.FullScreenPresenter";
}
::windows_core::imp::interface_hierarchy!(
    FullScreenPresenter,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<AppWindowPresenter> for FullScreenPresenter {}
unsafe impl ::core::marker::Send for FullScreenPresenter {}
unsafe impl ::core::marker::Sync for FullScreenPresenter {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct OverlappedPresenter(::windows_core::IUnknown);
impl OverlappedPresenter {
    pub fn Kind(&self) -> ::windows_core::Result<AppWindowPresenterKind> {
        let this = &::windows_core::ComInterface::cast::<IAppWindowPresenter>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn HasBorder(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HasBorder)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn HasTitleBar(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HasTitleBar)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn IsAlwaysOnTop(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsAlwaysOnTop)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetIsAlwaysOnTop(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetIsAlwaysOnTop)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsMaximizable(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsMaximizable)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetIsMaximizable(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetIsMaximizable)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsMinimizable(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsMinimizable)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetIsMinimizable(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetIsMinimizable)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsModal(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsModal)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetIsModal(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetIsModal)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsResizable(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsResizable)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetIsResizable(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetIsResizable)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn State(&self) -> ::windows_core::Result<OverlappedPresenterState> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).State)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn Maximize(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).Maximize)(::windows_core::Interface::as_raw(
                this,
            ))
            .ok()
        }
    }
    pub fn Minimize(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).Minimize)(::windows_core::Interface::as_raw(
                this,
            ))
            .ok()
        }
    }
    pub fn Restore(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).Restore)(::windows_core::Interface::as_raw(
                this,
            ))
            .ok()
        }
    }
    pub fn SetBorderAndTitleBar(
        &self,
        hasborder: bool,
        hastitlebar: bool,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetBorderAndTitleBar)(
                ::windows_core::Interface::as_raw(this),
                hasborder,
                hastitlebar,
            )
            .ok()
        }
    }
    pub fn MinimizeWithActivation(&self, activatewindow: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IOverlappedPresenter2>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).MinimizeWithActivation)(
                ::windows_core::Interface::as_raw(this),
                activatewindow,
            )
            .ok()
        }
    }
    pub fn RestoreWithActivation(&self, activatewindow: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IOverlappedPresenter2>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).RestoreWithActivation)(
                ::windows_core::Interface::as_raw(this),
                activatewindow,
            )
            .ok()
        }
    }
    pub fn Create() -> ::windows_core::Result<OverlappedPresenter> {
        Self::IOverlappedPresenterStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Create)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn CreateForContextMenu() -> ::windows_core::Result<OverlappedPresenter> {
        Self::IOverlappedPresenterStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateForContextMenu)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn CreateForDialog() -> ::windows_core::Result<OverlappedPresenter> {
        Self::IOverlappedPresenterStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateForDialog)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn CreateForToolWindow() -> ::windows_core::Result<OverlappedPresenter> {
        Self::IOverlappedPresenterStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateForToolWindow)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn RequestedStartupState() -> ::windows_core::Result<OverlappedPresenterState> {
        Self::IOverlappedPresenterStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RequestedStartupState)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IOverlappedPresenterStatics<
        R,
        F: FnOnce(&IOverlappedPresenterStatics) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<
            OverlappedPresenter,
            IOverlappedPresenterStatics,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IOverlappedPresenterStatics2<
        R,
        F: FnOnce(&IOverlappedPresenterStatics2) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<
            OverlappedPresenter,
            IOverlappedPresenterStatics2,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for OverlappedPresenter {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for OverlappedPresenter {
    type Vtable = IOverlappedPresenter_Vtbl;
}
unsafe impl ::windows_core::ComInterface for OverlappedPresenter {
    const IID: ::windows_core::GUID = <IOverlappedPresenter as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for OverlappedPresenter {
    const NAME: &'static str = "Microsoft.UI.Windowing.OverlappedPresenter";
}
::windows_core::imp::interface_hierarchy!(
    OverlappedPresenter,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<AppWindowPresenter> for OverlappedPresenter {}
unsafe impl ::core::marker::Send for OverlappedPresenter {}
unsafe impl ::core::marker::Sync for OverlappedPresenter {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AppWindowPresenterKind(pub i32);
impl AppWindowPresenterKind {
    pub const Default: Self = Self(0i32);
    pub const CompactOverlay: Self = Self(1i32);
    pub const FullScreen: Self = Self(2i32);
    pub const Overlapped: Self = Self(3i32);
}
impl ::core::marker::Copy for AppWindowPresenterKind {}
impl ::core::clone::Clone for AppWindowPresenterKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AppWindowPresenterKind {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for AppWindowPresenterKind {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for AppWindowPresenterKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppWindowPresenterKind").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for AppWindowPresenterKind {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::from_slice(
            b"enum(Microsoft.UI.Windowing.AppWindowPresenterKind;i4)",
        );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CompactOverlaySize(pub i32);
impl CompactOverlaySize {
    pub const Small: Self = Self(0i32);
    pub const Medium: Self = Self(1i32);
    pub const Large: Self = Self(2i32);
}
impl ::core::marker::Copy for CompactOverlaySize {}
impl ::core::clone::Clone for CompactOverlaySize {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CompactOverlaySize {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for CompactOverlaySize {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for CompactOverlaySize {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CompactOverlaySize").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for CompactOverlaySize {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::from_slice(
            b"enum(Microsoft.UI.Windowing.CompactOverlaySize;i4)",
        );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DisplayAreaFallback(pub i32);
impl DisplayAreaFallback {
    pub const None: Self = Self(0i32);
    pub const Primary: Self = Self(1i32);
    pub const Nearest: Self = Self(2i32);
}
impl ::core::marker::Copy for DisplayAreaFallback {}
impl ::core::clone::Clone for DisplayAreaFallback {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DisplayAreaFallback {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for DisplayAreaFallback {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for DisplayAreaFallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DisplayAreaFallback").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for DisplayAreaFallback {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::from_slice(
            b"enum(Microsoft.UI.Windowing.DisplayAreaFallback;i4)",
        );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DisplayAreaWatcherStatus(pub i32);
impl DisplayAreaWatcherStatus {
    pub const Created: Self = Self(0i32);
    pub const Started: Self = Self(1i32);
    pub const EnumerationCompleted: Self = Self(2i32);
    pub const Stopping: Self = Self(3i32);
    pub const Stopped: Self = Self(4i32);
    pub const Aborted: Self = Self(5i32);
}
impl ::core::marker::Copy for DisplayAreaWatcherStatus {}
impl ::core::clone::Clone for DisplayAreaWatcherStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DisplayAreaWatcherStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for DisplayAreaWatcherStatus {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for DisplayAreaWatcherStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DisplayAreaWatcherStatus").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for DisplayAreaWatcherStatus {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::from_slice(
            b"enum(Microsoft.UI.Windowing.DisplayAreaWatcherStatus;i4)",
        );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct IconShowOptions(pub i32);
impl IconShowOptions {
    pub const ShowIconAndSystemMenu: Self = Self(0i32);
    pub const HideIconAndSystemMenu: Self = Self(1i32);
}
impl ::core::marker::Copy for IconShowOptions {}
impl ::core::clone::Clone for IconShowOptions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IconShowOptions {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for IconShowOptions {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for IconShowOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IconShowOptions").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for IconShowOptions {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::from_slice(
            b"enum(Microsoft.UI.Windowing.IconShowOptions;i4)",
        );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct OverlappedPresenterState(pub i32);
impl OverlappedPresenterState {
    pub const Maximized: Self = Self(0i32);
    pub const Minimized: Self = Self(1i32);
    pub const Restored: Self = Self(2i32);
}
impl ::core::marker::Copy for OverlappedPresenterState {}
impl ::core::clone::Clone for OverlappedPresenterState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for OverlappedPresenterState {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for OverlappedPresenterState {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for OverlappedPresenterState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OverlappedPresenterState").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for OverlappedPresenterState {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::from_slice(
            b"enum(Microsoft.UI.Windowing.OverlappedPresenterState;i4)",
        );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TitleBarHeightOption(pub i32);
impl TitleBarHeightOption {
    pub const Standard: Self = Self(0i32);
    pub const Tall: Self = Self(1i32);
    pub const Collapsed: Self = Self(2i32);
}
impl ::core::marker::Copy for TitleBarHeightOption {}
impl ::core::clone::Clone for TitleBarHeightOption {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TitleBarHeightOption {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for TitleBarHeightOption {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for TitleBarHeightOption {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TitleBarHeightOption").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for TitleBarHeightOption {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::from_slice(
            b"enum(Microsoft.UI.Windowing.TitleBarHeightOption;i4)",
        );
}
