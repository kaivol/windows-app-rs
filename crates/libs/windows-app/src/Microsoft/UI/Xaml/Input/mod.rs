#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IAccessKeyDisplayDismissedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAccessKeyDisplayDismissedEventArgs {
    type Vtable = IAccessKeyDisplayDismissedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IAccessKeyDisplayDismissedEventArgs {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x125a83d8_7f86_5ea9_9063_b9407e644587);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccessKeyDisplayDismissedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IAccessKeyDisplayRequestedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAccessKeyDisplayRequestedEventArgs {
    type Vtable = IAccessKeyDisplayRequestedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IAccessKeyDisplayRequestedEventArgs {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xc4ed84d8_2b27_59b1_9cf0_7f9164de58cb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccessKeyDisplayRequestedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub PressedKeys: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IAccessKeyInvokedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAccessKeyInvokedEventArgs {
    type Vtable = IAccessKeyInvokedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IAccessKeyInvokedEventArgs {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xd00c11a4_f9fb_5707_9692_98b80bb8546d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccessKeyInvokedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
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
pub struct IAccessKeyManager(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAccessKeyManager {
    type Vtable = IAccessKeyManager_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IAccessKeyManager {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x8f2a4402_a635_53dc_bc17_da911eabaade);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccessKeyManager_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IAccessKeyManagerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAccessKeyManagerStatics {
    type Vtable = IAccessKeyManagerStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IAccessKeyManagerStatics {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x3375aef7_742f_5e84_b76f_c187e08253bf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccessKeyManagerStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub IsDisplayModeEnabled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub AreKeyTipsEnabled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetAreKeyTipsEnabled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_Foundation")]
    pub IsDisplayModeEnabledChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    IsDisplayModeEnabledChanged: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub RemoveIsDisplayModeEnabledChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    )
        -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RemoveIsDisplayModeEnabledChanged: usize,
    pub ExitDisplayMode:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IAccessKeyManagerStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAccessKeyManagerStatics2 {
    type Vtable = IAccessKeyManagerStatics2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IAccessKeyManagerStatics2 {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x512c9f63_24ad_5df2_b8ed_472406db31c0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccessKeyManagerStatics2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub EnterDisplayModeForXamlRoot: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        xamlroot: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICanExecuteRequestedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICanExecuteRequestedEventArgs {
    type Vtable = ICanExecuteRequestedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICanExecuteRequestedEventArgs {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xe4bf6d7d_f6eb_53ca_a2d4_c741ec871e38);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICanExecuteRequestedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Parameter: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub CanExecute: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetCanExecute: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICharacterReceivedRoutedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICharacterReceivedRoutedEventArgs {
    type Vtable = ICharacterReceivedRoutedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICharacterReceivedRoutedEventArgs {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xe26ca5bb_34c3_5c1e_9a16_00b80b07a899);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICharacterReceivedRoutedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Character: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut u16,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_UI_Core")]
    pub KeyStatus: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Core::CorePhysicalKeyStatus,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI_Core"))]
    KeyStatus: usize,
    pub Handled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICommand(::windows_core::IUnknown);
impl ICommand {
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn CanExecuteChanged<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::EventHandler<::windows_core::IInspectable>,
        >,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CanExecuteChanged)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveCanExecuteChanged(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveCanExecuteChanged)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn CanExecute<P0>(&self, parameter: P0) -> ::windows_core::Result<bool>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CanExecute)(
                ::windows_core::Interface::as_raw(this),
                parameter.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn Execute<P0>(&self, parameter: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).Execute)(
                ::windows_core::Interface::as_raw(this),
                parameter.into_param().abi(),
            )
            .ok()
        }
    }
}
::windows_core::imp::interface_hierarchy!(
    ICommand,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::RuntimeType for ICommand {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_interface::<Self>();
}
unsafe impl ::windows_core::Interface for ICommand {
    type Vtable = ICommand_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICommand {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xe5af3542_ca67_4081_995b_709dd13792df);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICommand_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Windows_Foundation")]
    pub CanExecuteChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    CanExecuteChanged: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub RemoveCanExecuteChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RemoveCanExecuteChanged: usize,
    pub CanExecute: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        parameter: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub Execute: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        parameter: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IContextRequestedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IContextRequestedEventArgs {
    type Vtable = IContextRequestedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IContextRequestedEventArgs {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xbcedcb98_77b5_53c0_802e_fd52f3806e51);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContextRequestedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Handled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_Foundation")]
    pub TryGetPosition: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        relativeto: *mut ::core::ffi::c_void,
        point: *mut ::windows::Foundation::Point,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    TryGetPosition: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDoubleTappedRoutedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDoubleTappedRoutedEventArgs {
    type Vtable = IDoubleTappedRoutedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDoubleTappedRoutedEventArgs {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x32b9549d_11d8_53a5_a953_02409537a11f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDoubleTappedRoutedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Microsoft_UI_Input")]
    pub PointerDeviceType: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::super::Input::PointerDeviceType,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Microsoft_UI_Input"))]
    PointerDeviceType: usize,
    pub Handled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_Foundation")]
    pub GetPosition: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        relativeto: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::Point,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    GetPosition: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IExecuteRequestedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IExecuteRequestedEventArgs {
    type Vtable = IExecuteRequestedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IExecuteRequestedEventArgs {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xe1a9fd0c_34d0_5ae2_8f5d_377e7a8a2708);
}
#[repr(C)]
#[doc(hidden)]
pub struct IExecuteRequestedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Parameter: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IFindNextElementOptions(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFindNextElementOptions {
    type Vtable = IFindNextElementOptions_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IFindNextElementOptions {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x7f88e76b_7417_5447_aed4_2fabd291bdc6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFindNextElementOptions_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SearchRoot: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetSearchRoot: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_Foundation")]
    pub ExclusionRect: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::Rect,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    ExclusionRect: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub SetExclusionRect: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::windows::Foundation::Rect,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    SetExclusionRect: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub HintRect: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::Rect,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    HintRect: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub SetHintRect: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::windows::Foundation::Rect,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    SetHintRect: usize,
    pub XYFocusNavigationStrategyOverride: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut XYFocusNavigationStrategyOverride,
    )
        -> ::windows_core::HRESULT,
    pub SetXYFocusNavigationStrategyOverride: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: XYFocusNavigationStrategyOverride,
    )
        -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IFocusManager(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFocusManager {
    type Vtable = IFocusManager_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IFocusManager {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x9fd07bc5_d2d4_53fe_a31a_846de8b7a257);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFocusManager_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IFocusManagerGotFocusEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFocusManagerGotFocusEventArgs {
    type Vtable = IFocusManagerGotFocusEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IFocusManagerGotFocusEventArgs {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x50aca341_4519_59cf_83b1_c9c45cfdb816);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFocusManagerGotFocusEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub NewFocusedElement: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub CorrelationId: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows_core::GUID,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IFocusManagerLostFocusEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFocusManagerLostFocusEventArgs {
    type Vtable = IFocusManagerLostFocusEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IFocusManagerLostFocusEventArgs {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xfdaf2c3f_a22e_5902_abce_b60758fbed1e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFocusManagerLostFocusEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub OldFocusedElement: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub CorrelationId: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows_core::GUID,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IFocusManagerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFocusManagerStatics {
    type Vtable = IFocusManagerStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IFocusManagerStatics {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xe73dce04_e23a_5fb3_96ab_7df04c51dff2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFocusManagerStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
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
    #[cfg(feature = "Windows_Foundation")]
    pub GettingFocus: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    GettingFocus: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub RemoveGettingFocus: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RemoveGettingFocus: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub LosingFocus: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    LosingFocus: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub RemoveLosingFocus: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RemoveLosingFocus: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub TryFocusAsync: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        value: super::FocusState,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    TryFocusAsync: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub TryMoveFocusAsync: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        focusnavigationdirection: FocusNavigationDirection,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    TryMoveFocusAsync: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub TryMoveFocusWithOptionsAsync: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        focusnavigationdirection: FocusNavigationDirection,
        focusnavigationoptions: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    TryMoveFocusWithOptionsAsync: usize,
    pub TryMoveFocusWithOptions: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        focusnavigationdirection: FocusNavigationDirection,
        focusnavigationoptions: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub FindNextElement: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        focusnavigationdirection: FocusNavigationDirection,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub FindFirstFocusableElement: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        searchscope: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub FindLastFocusableElement: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        searchscope: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub FindNextElementWithOptions: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        focusnavigationdirection: FocusNavigationDirection,
        focusnavigationoptions: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub FindNextFocusableElement: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        focusnavigationdirection: FocusNavigationDirection,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_Foundation")]
    pub FindNextFocusableElementWithHint: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        focusnavigationdirection: FocusNavigationDirection,
        hintrect: ::windows::Foundation::Rect,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    FindNextFocusableElementWithHint: usize,
    pub TryMoveFocus: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        focusnavigationdirection: FocusNavigationDirection,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub GetFocusedElement: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub GetFocusedElementWithRoot: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        xamlroot: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IFocusMovementResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFocusMovementResult {
    type Vtable = IFocusMovementResult_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IFocusMovementResult {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xa46259fd_3edd_554b_a188_0a47b71e4e1a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFocusMovementResult_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Succeeded: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IGettingFocusEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGettingFocusEventArgs {
    type Vtable = IGettingFocusEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IGettingFocusEventArgs {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x37fd3af0_bd3c_5bf5_a9cd_71a1e87af950);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGettingFocusEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub OldFocusedElement: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub NewFocusedElement: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetNewFocusedElement: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub FocusState: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::FocusState,
    ) -> ::windows_core::HRESULT,
    pub Direction: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut FocusNavigationDirection,
    ) -> ::windows_core::HRESULT,
    pub Handled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
    pub InputDevice: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut FocusInputDeviceKind,
    ) -> ::windows_core::HRESULT,
    pub Cancel: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetCancel: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
    pub CorrelationId: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows_core::GUID,
    ) -> ::windows_core::HRESULT,
    pub TryCancel: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub TrySetNewFocusedElement: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IHoldingRoutedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHoldingRoutedEventArgs {
    type Vtable = IHoldingRoutedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IHoldingRoutedEventArgs {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x8272a4b2_2221_551e_b0bb_16e29138ab20);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHoldingRoutedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Microsoft_UI_Input")]
    pub PointerDeviceType: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::super::Input::PointerDeviceType,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Microsoft_UI_Input"))]
    PointerDeviceType: usize,
    #[cfg(feature = "Microsoft_UI_Input")]
    pub HoldingState: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::super::Input::HoldingState,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Microsoft_UI_Input"))]
    HoldingState: usize,
    pub Handled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_Foundation")]
    pub GetPosition: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        relativeto: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::Point,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    GetPosition: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IInertiaExpansionBehavior(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IInertiaExpansionBehavior {
    type Vtable = IInertiaExpansionBehavior_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IInertiaExpansionBehavior {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xd60029b7_f0cd_5aea_abe5_7410d09118c6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInertiaExpansionBehavior_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub DesiredDeceleration: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub SetDesiredDeceleration: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
    ) -> ::windows_core::HRESULT,
    pub DesiredExpansion: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub SetDesiredExpansion: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IInertiaRotationBehavior(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IInertiaRotationBehavior {
    type Vtable = IInertiaRotationBehavior_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IInertiaRotationBehavior {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x27b4bd03_9149_5691_bce5_fa33b32c4a81);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInertiaRotationBehavior_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub DesiredDeceleration: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub SetDesiredDeceleration: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
    ) -> ::windows_core::HRESULT,
    pub DesiredRotation: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub SetDesiredRotation: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IInertiaTranslationBehavior(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IInertiaTranslationBehavior {
    type Vtable = IInertiaTranslationBehavior_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IInertiaTranslationBehavior {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xd4f91cf5_3317_5914_b25a_ea6ee55b96d0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInertiaTranslationBehavior_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub DesiredDeceleration: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub SetDesiredDeceleration: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
    ) -> ::windows_core::HRESULT,
    pub DesiredDisplacement: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub SetDesiredDisplacement: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IInputScope(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IInputScope {
    type Vtable = IInputScope_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IInputScope {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x76ea58b1_e910_5176_9147_695cc95e7da2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputScope_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub Names: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation_Collections"))]
    Names: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IInputScopeName(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IInputScopeName {
    type Vtable = IInputScopeName_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IInputScopeName {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xee99a66d_28d0_53cb_82ee_1b6ee58bcc35);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputScopeName_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub NameValue: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut InputScopeNameValue,
    ) -> ::windows_core::HRESULT,
    pub SetNameValue: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: InputScopeNameValue,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IInputScopeNameFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IInputScopeNameFactory {
    type Vtable = IInputScopeNameFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IInputScopeNameFactory {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xfeec2efd_bc09_5cd6_9b47_6d35d1d87c61);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputScopeNameFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        namevalue: InputScopeNameValue,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IKeyRoutedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IKeyRoutedEventArgs {
    type Vtable = IKeyRoutedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IKeyRoutedEventArgs {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xee357007_a2d6_5c75_9431_05fd66ec7915);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKeyRoutedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Windows_System")]
    pub Key: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::System::VirtualKey,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_System"))]
    Key: usize,
    #[cfg(feature = "Windows_UI_Core")]
    pub KeyStatus: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Core::CorePhysicalKeyStatus,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI_Core"))]
    KeyStatus: usize,
    pub Handled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_System")]
    pub OriginalKey: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::System::VirtualKey,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_System"))]
    OriginalKey: usize,
    pub DeviceId: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IKeyboardAccelerator(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IKeyboardAccelerator {
    type Vtable = IKeyboardAccelerator_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IKeyboardAccelerator {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x6f8bf1e2_4e91_5cf9_a6be_4770caf3d770);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKeyboardAccelerator_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Windows_System")]
    pub Key: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::System::VirtualKey,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_System"))]
    Key: usize,
    #[cfg(feature = "Windows_System")]
    pub SetKey: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::windows::System::VirtualKey,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_System"))]
    SetKey: usize,
    #[cfg(feature = "Windows_System")]
    pub Modifiers: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::System::VirtualKeyModifiers,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_System"))]
    Modifiers: usize,
    #[cfg(feature = "Windows_System")]
    pub SetModifiers: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::windows::System::VirtualKeyModifiers,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_System"))]
    SetModifiers: usize,
    pub IsEnabled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetIsEnabled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
    pub ScopeOwner: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetScopeOwner: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_Foundation")]
    pub Invoked: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    Invoked: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub RemoveInvoked: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RemoveInvoked: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IKeyboardAcceleratorFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IKeyboardAcceleratorFactory {
    type Vtable = IKeyboardAcceleratorFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IKeyboardAcceleratorFactory {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xca1d410a_af2a_51b9_a1de_6c0af9f3b598);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKeyboardAcceleratorFactory_Vtbl {
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
pub struct IKeyboardAcceleratorInvokedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IKeyboardAcceleratorInvokedEventArgs {
    type Vtable = IKeyboardAcceleratorInvokedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IKeyboardAcceleratorInvokedEventArgs {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x62c9fdb0_b574_527d_97eb_5c7f674441e0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKeyboardAcceleratorInvokedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Handled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
    pub Element: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub KeyboardAccelerator: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IKeyboardAcceleratorStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IKeyboardAcceleratorStatics {
    type Vtable = IKeyboardAcceleratorStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IKeyboardAcceleratorStatics {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x73e674ca_73f4_5e77_b8d6_ff7852a63b0b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKeyboardAcceleratorStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub KeyProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub ModifiersProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub IsEnabledProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub ScopeOwnerProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ILosingFocusEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILosingFocusEventArgs {
    type Vtable = ILosingFocusEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ILosingFocusEventArgs {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xfa0e5ffa_2b1b_52f8_bb66_e35f51e73cf3);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILosingFocusEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub OldFocusedElement: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub NewFocusedElement: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetNewFocusedElement: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub FocusState: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::FocusState,
    ) -> ::windows_core::HRESULT,
    pub Direction: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut FocusNavigationDirection,
    ) -> ::windows_core::HRESULT,
    pub Handled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
    pub InputDevice: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut FocusInputDeviceKind,
    ) -> ::windows_core::HRESULT,
    pub Cancel: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetCancel: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
    pub CorrelationId: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows_core::GUID,
    ) -> ::windows_core::HRESULT,
    pub TryCancel: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub TrySetNewFocusedElement: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IManipulationCompletedRoutedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IManipulationCompletedRoutedEventArgs {
    type Vtable = IManipulationCompletedRoutedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IManipulationCompletedRoutedEventArgs {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xe3be9e4e_c5fb_5859_a81d_ce12fc3a2f4d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IManipulationCompletedRoutedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Container: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_Foundation")]
    pub Position: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::Point,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    Position: usize,
    pub IsInertial: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Microsoft_UI_Input", feature = "Windows_Foundation"))]
    pub Cumulative: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::super::Input::ManipulationDelta,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Microsoft_UI_Input", feature = "Windows_Foundation")))]
    Cumulative: usize,
    #[cfg(all(feature = "Microsoft_UI_Input", feature = "Windows_Foundation"))]
    pub Velocities: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::super::Input::ManipulationVelocities,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Microsoft_UI_Input", feature = "Windows_Foundation")))]
    Velocities: usize,
    pub Handled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Microsoft_UI_Input")]
    pub PointerDeviceType: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::super::Input::PointerDeviceType,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Microsoft_UI_Input"))]
    PointerDeviceType: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IManipulationDeltaRoutedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IManipulationDeltaRoutedEventArgs {
    type Vtable = IManipulationDeltaRoutedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IManipulationDeltaRoutedEventArgs {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x51369745_960f_54ac_93fa_763d22910dea);
}
#[repr(C)]
#[doc(hidden)]
pub struct IManipulationDeltaRoutedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Container: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_Foundation")]
    pub Position: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::Point,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    Position: usize,
    pub IsInertial: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Microsoft_UI_Input", feature = "Windows_Foundation"))]
    pub Delta: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::super::Input::ManipulationDelta,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Microsoft_UI_Input", feature = "Windows_Foundation")))]
    Delta: usize,
    #[cfg(all(feature = "Microsoft_UI_Input", feature = "Windows_Foundation"))]
    pub Cumulative: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::super::Input::ManipulationDelta,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Microsoft_UI_Input", feature = "Windows_Foundation")))]
    Cumulative: usize,
    #[cfg(all(feature = "Microsoft_UI_Input", feature = "Windows_Foundation"))]
    pub Velocities: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::super::Input::ManipulationVelocities,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Microsoft_UI_Input", feature = "Windows_Foundation")))]
    Velocities: usize,
    pub Handled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Microsoft_UI_Input")]
    pub PointerDeviceType: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::super::Input::PointerDeviceType,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Microsoft_UI_Input"))]
    PointerDeviceType: usize,
    pub Complete:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IManipulationInertiaStartingRoutedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IManipulationInertiaStartingRoutedEventArgs {
    type Vtable = IManipulationInertiaStartingRoutedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IManipulationInertiaStartingRoutedEventArgs {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x17d510be_5514_5952_9afd_959b60ab9394);
}
#[repr(C)]
#[doc(hidden)]
pub struct IManipulationInertiaStartingRoutedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Container: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub ExpansionBehavior: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetExpansionBehavior: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub RotationBehavior: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetRotationBehavior: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub TranslationBehavior: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetTranslationBehavior: unsafe extern "system" fn(
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
    #[cfg(feature = "Microsoft_UI_Input")]
    pub PointerDeviceType: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::super::Input::PointerDeviceType,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Microsoft_UI_Input"))]
    PointerDeviceType: usize,
    #[cfg(all(feature = "Microsoft_UI_Input", feature = "Windows_Foundation"))]
    pub Delta: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::super::Input::ManipulationDelta,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Microsoft_UI_Input", feature = "Windows_Foundation")))]
    Delta: usize,
    #[cfg(all(feature = "Microsoft_UI_Input", feature = "Windows_Foundation"))]
    pub Cumulative: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::super::Input::ManipulationDelta,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Microsoft_UI_Input", feature = "Windows_Foundation")))]
    Cumulative: usize,
    #[cfg(all(feature = "Microsoft_UI_Input", feature = "Windows_Foundation"))]
    pub Velocities: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::super::Input::ManipulationVelocities,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Microsoft_UI_Input", feature = "Windows_Foundation")))]
    Velocities: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IManipulationPivot(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IManipulationPivot {
    type Vtable = IManipulationPivot_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IManipulationPivot {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x286baba4_313d_507c_adc5_f739732cea27);
}
#[repr(C)]
#[doc(hidden)]
pub struct IManipulationPivot_Vtbl {
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
    pub Radius: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub SetRadius: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IManipulationPivotFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IManipulationPivotFactory {
    type Vtable = IManipulationPivotFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IManipulationPivotFactory {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x67143ccd_ea6c_5fe2_bef2_adcbd7af52fd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IManipulationPivotFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Windows_Foundation")]
    pub CreateInstanceWithCenterAndRadius: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        center: ::windows::Foundation::Point,
        radius: f64,
        result__: *mut *mut ::core::ffi::c_void,
    )
        -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    CreateInstanceWithCenterAndRadius: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IManipulationStartedRoutedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IManipulationStartedRoutedEventArgs {
    type Vtable = IManipulationStartedRoutedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IManipulationStartedRoutedEventArgs {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x61857950_5821_5652_9fdf_c6277c5886f5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IManipulationStartedRoutedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Container: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_Foundation")]
    pub Position: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::Point,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    Position: usize,
    pub Handled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Microsoft_UI_Input")]
    pub PointerDeviceType: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::super::Input::PointerDeviceType,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Microsoft_UI_Input"))]
    PointerDeviceType: usize,
    #[cfg(all(feature = "Microsoft_UI_Input", feature = "Windows_Foundation"))]
    pub Cumulative: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::super::Input::ManipulationDelta,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Microsoft_UI_Input", feature = "Windows_Foundation")))]
    Cumulative: usize,
    pub Complete:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IManipulationStartedRoutedEventArgsFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IManipulationStartedRoutedEventArgsFactory {
    type Vtable = IManipulationStartedRoutedEventArgsFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IManipulationStartedRoutedEventArgsFactory {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x5681b0de_3fa7_503e_9c46_a80339760292);
}
#[repr(C)]
#[doc(hidden)]
pub struct IManipulationStartedRoutedEventArgsFactory_Vtbl {
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
pub struct IManipulationStartingRoutedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IManipulationStartingRoutedEventArgs {
    type Vtable = IManipulationStartingRoutedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IManipulationStartingRoutedEventArgs {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x93a99f86_f5a0_5326_91b0_851c897af79f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IManipulationStartingRoutedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Mode: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ManipulationModes,
    ) -> ::windows_core::HRESULT,
    pub SetMode: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ManipulationModes,
    ) -> ::windows_core::HRESULT,
    pub Container: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetContainer: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub Pivot: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetPivot: unsafe extern "system" fn(
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
pub struct INoFocusCandidateFoundEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for INoFocusCandidateFoundEventArgs {
    type Vtable = INoFocusCandidateFoundEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for INoFocusCandidateFoundEventArgs {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xa2d7153a_cd2a_59cb_a574_ac82e30b9201);
}
#[repr(C)]
#[doc(hidden)]
pub struct INoFocusCandidateFoundEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Direction: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut FocusNavigationDirection,
    ) -> ::windows_core::HRESULT,
    pub Handled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
    pub InputDevice: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut FocusInputDeviceKind,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPointer(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPointer {
    type Vtable = IPointer_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPointer {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x1f9afbf5_11a3_5e68_aa1b_72febfa0ab23);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPointer_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub PointerId: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut u32,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Microsoft_UI_Input")]
    pub PointerDeviceType: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::super::Input::PointerDeviceType,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Microsoft_UI_Input"))]
    PointerDeviceType: usize,
    pub IsInContact: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub IsInRange: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPointerRoutedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPointerRoutedEventArgs {
    type Vtable = IPointerRoutedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPointerRoutedEventArgs {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x66e78a9a_1bec_5f92_b1a1_ea6334ee511c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPointerRoutedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Pointer: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_System")]
    pub KeyModifiers: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::System::VirtualKeyModifiers,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_System"))]
    KeyModifiers: usize,
    pub Handled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
    pub IsGenerated: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Microsoft_UI_Input")]
    pub GetCurrentPoint: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        relativeto: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Microsoft_UI_Input"))]
    GetCurrentPoint: usize,
    #[cfg(all(
        feature = "Microsoft_UI_Input",
        feature = "Windows_Foundation_Collections"
    ))]
    pub GetIntermediatePoints: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        relativeto: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(all(
        feature = "Microsoft_UI_Input",
        feature = "Windows_Foundation_Collections"
    )))]
    GetIntermediatePoints: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IProcessKeyboardAcceleratorEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IProcessKeyboardAcceleratorEventArgs {
    type Vtable = IProcessKeyboardAcceleratorEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IProcessKeyboardAcceleratorEventArgs {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x9be0d058_3d26_5811_b50a_3bb80ca766c9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProcessKeyboardAcceleratorEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Windows_System")]
    pub Key: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::System::VirtualKey,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_System"))]
    Key: usize,
    #[cfg(feature = "Windows_System")]
    pub Modifiers: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::System::VirtualKeyModifiers,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_System"))]
    Modifiers: usize,
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
pub struct IRightTappedRoutedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRightTappedRoutedEventArgs {
    type Vtable = IRightTappedRoutedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IRightTappedRoutedEventArgs {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x3972fafb_2915_5c62_bb6b_54ad84ff400d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRightTappedRoutedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Microsoft_UI_Input")]
    pub PointerDeviceType: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::super::Input::PointerDeviceType,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Microsoft_UI_Input"))]
    PointerDeviceType: usize,
    pub Handled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_Foundation")]
    pub GetPosition: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        relativeto: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::Point,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    GetPosition: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IStandardUICommand(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IStandardUICommand {
    type Vtable = IStandardUICommand_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IStandardUICommand {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x5f395d50_5449_59ab_9cb2_4e3700033f03);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStandardUICommand_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Kind: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut StandardUICommandKind,
    ) -> ::windows_core::HRESULT,
    pub SetKind: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: StandardUICommandKind,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IStandardUICommandFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IStandardUICommandFactory {
    type Vtable = IStandardUICommandFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IStandardUICommandFactory {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x5800f099_3746_5bcf_b1ce_af3d6bf8e83f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStandardUICommandFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        baseinterface: *mut ::core::ffi::c_void,
        innerinterface: *mut *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub CreateInstanceWithKind: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        kind: StandardUICommandKind,
        baseinterface: *mut ::core::ffi::c_void,
        innerinterface: *mut *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IStandardUICommandStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IStandardUICommandStatics {
    type Vtable = IStandardUICommandStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IStandardUICommandStatics {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xab80c197_85cc_5d36_81aa_156cd63be31a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStandardUICommandStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub KindProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ITappedRoutedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITappedRoutedEventArgs {
    type Vtable = ITappedRoutedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ITappedRoutedEventArgs {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x73f74b8c_3709_547e_8e0c_51c03c89126a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITappedRoutedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Microsoft_UI_Input")]
    pub PointerDeviceType: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::super::Input::PointerDeviceType,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Microsoft_UI_Input"))]
    PointerDeviceType: usize,
    pub Handled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_Foundation")]
    pub GetPosition: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        relativeto: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::Point,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    GetPosition: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IXamlUICommand(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXamlUICommand {
    type Vtable = IXamlUICommand_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IXamlUICommand {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xa457f2cb_51e0_541c_9c42_dd1dcbdf58fb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlUICommand_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Label: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub SetLabel: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Microsoft_UI_Xaml_Controls")]
    pub IconSource: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Microsoft_UI_Xaml_Controls"))]
    IconSource: usize,
    #[cfg(feature = "Microsoft_UI_Xaml_Controls")]
    pub SetIconSource: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Microsoft_UI_Xaml_Controls"))]
    SetIconSource: usize,
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub KeyboardAccelerators: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation_Collections"))]
    KeyboardAccelerators: usize,
    pub AccessKey: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub SetAccessKey: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub Command: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetCommand: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_Foundation")]
    pub ExecuteRequested: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    ExecuteRequested: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub RemoveExecuteRequested: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RemoveExecuteRequested: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub CanExecuteRequested: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    CanExecuteRequested: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub RemoveCanExecuteRequested: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RemoveCanExecuteRequested: usize,
    pub NotifyCanExecuteChanged:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IXamlUICommandFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXamlUICommandFactory {
    type Vtable = IXamlUICommandFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IXamlUICommandFactory {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xf1f80a20_0e31_5505_8bc3_cdd1f0947f1d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlUICommandFactory_Vtbl {
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
pub struct IXamlUICommandStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXamlUICommandStatics {
    type Vtable = IXamlUICommandStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IXamlUICommandStatics {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x981dbda6_cdcb_5e35_b24b_c4f60ba148d9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlUICommandStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub LabelProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub IconSourceProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub KeyboardAcceleratorsProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub AccessKeyProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub DescriptionProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub CommandProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct AccessKeyDisplayDismissedEventArgs(::windows_core::IUnknown);
impl AccessKeyDisplayDismissedEventArgs {
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
            AccessKeyDisplayDismissedEventArgs,
            ::windows_core::imp::IGenericFactory,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for AccessKeyDisplayDismissedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for AccessKeyDisplayDismissedEventArgs {
    type Vtable = IAccessKeyDisplayDismissedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for AccessKeyDisplayDismissedEventArgs {
    const IID: ::windows_core::GUID =
        <IAccessKeyDisplayDismissedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for AccessKeyDisplayDismissedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.AccessKeyDisplayDismissedEventArgs";
}
::windows_core::imp::interface_hierarchy!(
    AccessKeyDisplayDismissedEventArgs,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for AccessKeyDisplayDismissedEventArgs {}
unsafe impl ::core::marker::Sync for AccessKeyDisplayDismissedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct AccessKeyDisplayRequestedEventArgs(::windows_core::IUnknown);
impl AccessKeyDisplayRequestedEventArgs {
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
            AccessKeyDisplayRequestedEventArgs,
            ::windows_core::imp::IGenericFactory,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn PressedKeys(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PressedKeys)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for AccessKeyDisplayRequestedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for AccessKeyDisplayRequestedEventArgs {
    type Vtable = IAccessKeyDisplayRequestedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for AccessKeyDisplayRequestedEventArgs {
    const IID: ::windows_core::GUID =
        <IAccessKeyDisplayRequestedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for AccessKeyDisplayRequestedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.AccessKeyDisplayRequestedEventArgs";
}
::windows_core::imp::interface_hierarchy!(
    AccessKeyDisplayRequestedEventArgs,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for AccessKeyDisplayRequestedEventArgs {}
unsafe impl ::core::marker::Sync for AccessKeyDisplayRequestedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct AccessKeyInvokedEventArgs(::windows_core::IUnknown);
impl AccessKeyInvokedEventArgs {
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
            AccessKeyInvokedEventArgs,
            ::windows_core::imp::IGenericFactory,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
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
impl ::windows_core::RuntimeType for AccessKeyInvokedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for AccessKeyInvokedEventArgs {
    type Vtable = IAccessKeyInvokedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for AccessKeyInvokedEventArgs {
    const IID: ::windows_core::GUID =
        <IAccessKeyInvokedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for AccessKeyInvokedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.AccessKeyInvokedEventArgs";
}
::windows_core::imp::interface_hierarchy!(
    AccessKeyInvokedEventArgs,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for AccessKeyInvokedEventArgs {}
unsafe impl ::core::marker::Sync for AccessKeyInvokedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct AccessKeyManager(::windows_core::IUnknown);
impl AccessKeyManager {
    pub fn IsDisplayModeEnabled() -> ::windows_core::Result<bool> {
        Self::IAccessKeyManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsDisplayModeEnabled)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn AreKeyTipsEnabled() -> ::windows_core::Result<bool> {
        Self::IAccessKeyManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AreKeyTipsEnabled)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn SetAreKeyTipsEnabled(value: bool) -> ::windows_core::Result<()> {
        Self::IAccessKeyManagerStatics(|this| unsafe {
            (::windows_core::Interface::vtable(this).SetAreKeyTipsEnabled)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        })
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn IsDisplayModeEnabledChanged<P0>(
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<
                ::windows_core::IInspectable,
                ::windows_core::IInspectable,
            >,
        >,
    {
        Self::IAccessKeyManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsDisplayModeEnabledChanged)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveIsDisplayModeEnabledChanged(
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        Self::IAccessKeyManagerStatics(|this| unsafe {
            (::windows_core::Interface::vtable(this).RemoveIsDisplayModeEnabledChanged)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        })
    }
    pub fn ExitDisplayMode() -> ::windows_core::Result<()> {
        Self::IAccessKeyManagerStatics(|this| unsafe {
            (::windows_core::Interface::vtable(this).ExitDisplayMode)(
                ::windows_core::Interface::as_raw(this),
            )
            .ok()
        })
    }
    pub fn EnterDisplayModeForXamlRoot<P0>(xamlroot: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::XamlRoot>,
    {
        Self::IAccessKeyManagerStatics2(|this| unsafe {
            (::windows_core::Interface::vtable(this).EnterDisplayModeForXamlRoot)(
                ::windows_core::Interface::as_raw(this),
                xamlroot.into_param().abi(),
            )
            .ok()
        })
    }
    #[doc(hidden)]
    pub fn IAccessKeyManagerStatics<
        R,
        F: FnOnce(&IAccessKeyManagerStatics) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<
            AccessKeyManager,
            IAccessKeyManagerStatics,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IAccessKeyManagerStatics2<
        R,
        F: FnOnce(&IAccessKeyManagerStatics2) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<
            AccessKeyManager,
            IAccessKeyManagerStatics2,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for AccessKeyManager {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for AccessKeyManager {
    type Vtable = IAccessKeyManager_Vtbl;
}
unsafe impl ::windows_core::ComInterface for AccessKeyManager {
    const IID: ::windows_core::GUID = <IAccessKeyManager as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for AccessKeyManager {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.AccessKeyManager";
}
::windows_core::imp::interface_hierarchy!(
    AccessKeyManager,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for AccessKeyManager {}
unsafe impl ::core::marker::Sync for AccessKeyManager {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct CanExecuteRequestedEventArgs(::windows_core::IUnknown);
impl CanExecuteRequestedEventArgs {
    pub fn Parameter(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Parameter)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn CanExecute(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CanExecute)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetCanExecute(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetCanExecute)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
}
impl ::windows_core::RuntimeType for CanExecuteRequestedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for CanExecuteRequestedEventArgs {
    type Vtable = ICanExecuteRequestedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for CanExecuteRequestedEventArgs {
    const IID: ::windows_core::GUID =
        <ICanExecuteRequestedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for CanExecuteRequestedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.CanExecuteRequestedEventArgs";
}
::windows_core::imp::interface_hierarchy!(
    CanExecuteRequestedEventArgs,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for CanExecuteRequestedEventArgs {}
unsafe impl ::core::marker::Sync for CanExecuteRequestedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct CharacterReceivedRoutedEventArgs(::windows_core::IUnknown);
impl CharacterReceivedRoutedEventArgs {
    pub fn Character(&self) -> ::windows_core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Character)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_UI_Core\"`"]
    #[cfg(feature = "Windows_UI_Core")]
    pub fn KeyStatus(&self) -> ::windows_core::Result<::windows::UI::Core::CorePhysicalKeyStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).KeyStatus)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
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
impl ::windows_core::RuntimeType for CharacterReceivedRoutedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for CharacterReceivedRoutedEventArgs {
    type Vtable = ICharacterReceivedRoutedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for CharacterReceivedRoutedEventArgs {
    const IID: ::windows_core::GUID =
        <ICharacterReceivedRoutedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for CharacterReceivedRoutedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.CharacterReceivedRoutedEventArgs";
}
::windows_core::imp::interface_hierarchy!(
    CharacterReceivedRoutedEventArgs,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<super::RoutedEventArgs> for CharacterReceivedRoutedEventArgs {}
unsafe impl ::core::marker::Send for CharacterReceivedRoutedEventArgs {}
unsafe impl ::core::marker::Sync for CharacterReceivedRoutedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ContextRequestedEventArgs(::windows_core::IUnknown);
impl ContextRequestedEventArgs {
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
            ContextRequestedEventArgs,
            ::windows_core::imp::IGenericFactory,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
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
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn TryGetPosition<P0>(
        &self,
        relativeto: P0,
        point: &mut ::windows::Foundation::Point,
    ) -> ::windows_core::Result<bool>
    where
        P0: ::windows_core::TryIntoParam<super::UIElement>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TryGetPosition)(
                ::windows_core::Interface::as_raw(this),
                relativeto.try_into_param()?.abi(),
                point,
                &mut result__,
            )
            .from_abi(result__)
        }
    }
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
impl ::windows_core::RuntimeType for ContextRequestedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for ContextRequestedEventArgs {
    type Vtable = IContextRequestedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ContextRequestedEventArgs {
    const IID: ::windows_core::GUID =
        <IContextRequestedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ContextRequestedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.ContextRequestedEventArgs";
}
::windows_core::imp::interface_hierarchy!(
    ContextRequestedEventArgs,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<super::RoutedEventArgs> for ContextRequestedEventArgs {}
unsafe impl ::core::marker::Send for ContextRequestedEventArgs {}
unsafe impl ::core::marker::Sync for ContextRequestedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct DoubleTappedRoutedEventArgs(::windows_core::IUnknown);
impl DoubleTappedRoutedEventArgs {
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
            DoubleTappedRoutedEventArgs,
            ::windows_core::imp::IGenericFactory,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "Required features: `\"Microsoft_UI_Input\"`"]
    #[cfg(feature = "Microsoft_UI_Input")]
    pub fn PointerDeviceType(
        &self,
    ) -> ::windows_core::Result<super::super::Input::PointerDeviceType> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PointerDeviceType)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
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
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn GetPosition<P0>(
        &self,
        relativeto: P0,
    ) -> ::windows_core::Result<::windows::Foundation::Point>
    where
        P0: ::windows_core::TryIntoParam<super::UIElement>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetPosition)(
                ::windows_core::Interface::as_raw(this),
                relativeto.try_into_param()?.abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
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
impl ::windows_core::RuntimeType for DoubleTappedRoutedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for DoubleTappedRoutedEventArgs {
    type Vtable = IDoubleTappedRoutedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for DoubleTappedRoutedEventArgs {
    const IID: ::windows_core::GUID =
        <IDoubleTappedRoutedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for DoubleTappedRoutedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.DoubleTappedRoutedEventArgs";
}
::windows_core::imp::interface_hierarchy!(
    DoubleTappedRoutedEventArgs,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<super::RoutedEventArgs> for DoubleTappedRoutedEventArgs {}
unsafe impl ::core::marker::Send for DoubleTappedRoutedEventArgs {}
unsafe impl ::core::marker::Sync for DoubleTappedRoutedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ExecuteRequestedEventArgs(::windows_core::IUnknown);
impl ExecuteRequestedEventArgs {
    pub fn Parameter(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Parameter)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for ExecuteRequestedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for ExecuteRequestedEventArgs {
    type Vtable = IExecuteRequestedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ExecuteRequestedEventArgs {
    const IID: ::windows_core::GUID =
        <IExecuteRequestedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ExecuteRequestedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.ExecuteRequestedEventArgs";
}
::windows_core::imp::interface_hierarchy!(
    ExecuteRequestedEventArgs,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for ExecuteRequestedEventArgs {}
unsafe impl ::core::marker::Sync for ExecuteRequestedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct FindNextElementOptions(::windows_core::IUnknown);
impl FindNextElementOptions {
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
            FindNextElementOptions,
            ::windows_core::imp::IGenericFactory,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn SearchRoot(&self) -> ::windows_core::Result<super::DependencyObject> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SearchRoot)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetSearchRoot<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetSearchRoot)(
                ::windows_core::Interface::as_raw(this),
                value.try_into_param()?.abi(),
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn ExclusionRect(&self) -> ::windows_core::Result<::windows::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ExclusionRect)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn SetExclusionRect(
        &self,
        value: ::windows::Foundation::Rect,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetExclusionRect)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
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
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn SetHintRect(&self, value: ::windows::Foundation::Rect) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetHintRect)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn XYFocusNavigationStrategyOverride(
        &self,
    ) -> ::windows_core::Result<XYFocusNavigationStrategyOverride> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).XYFocusNavigationStrategyOverride)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetXYFocusNavigationStrategyOverride(
        &self,
        value: XYFocusNavigationStrategyOverride,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetXYFocusNavigationStrategyOverride)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
}
impl ::windows_core::RuntimeType for FindNextElementOptions {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for FindNextElementOptions {
    type Vtable = IFindNextElementOptions_Vtbl;
}
unsafe impl ::windows_core::ComInterface for FindNextElementOptions {
    const IID: ::windows_core::GUID =
        <IFindNextElementOptions as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for FindNextElementOptions {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.FindNextElementOptions";
}
::windows_core::imp::interface_hierarchy!(
    FindNextElementOptions,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for FindNextElementOptions {}
unsafe impl ::core::marker::Sync for FindNextElementOptions {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct FocusManager(::windows_core::IUnknown);
impl FocusManager {
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn GotFocus<P0>(
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::EventHandler<FocusManagerGotFocusEventArgs>,
        >,
    {
        Self::IFocusManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GotFocus)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveGotFocus(
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        Self::IFocusManagerStatics(|this| unsafe {
            (::windows_core::Interface::vtable(this).RemoveGotFocus)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        })
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn LostFocus<P0>(
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::EventHandler<FocusManagerLostFocusEventArgs>,
        >,
    {
        Self::IFocusManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LostFocus)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveLostFocus(
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        Self::IFocusManagerStatics(|this| unsafe {
            (::windows_core::Interface::vtable(this).RemoveLostFocus)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        })
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn GettingFocus<P0>(
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<::windows::Foundation::EventHandler<GettingFocusEventArgs>>,
    {
        Self::IFocusManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GettingFocus)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveGettingFocus(
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        Self::IFocusManagerStatics(|this| unsafe {
            (::windows_core::Interface::vtable(this).RemoveGettingFocus)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        })
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn LosingFocus<P0>(
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<::windows::Foundation::EventHandler<LosingFocusEventArgs>>,
    {
        Self::IFocusManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LosingFocus)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveLosingFocus(
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        Self::IFocusManagerStatics(|this| unsafe {
            (::windows_core::Interface::vtable(this).RemoveLosingFocus)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        })
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn TryFocusAsync<P0>(
        element: P0,
        value: super::FocusState,
    ) -> ::windows_core::Result<::windows::Foundation::IAsyncOperation<FocusMovementResult>>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        Self::IFocusManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TryFocusAsync)(
                ::windows_core::Interface::as_raw(this),
                element.try_into_param()?.abi(),
                value,
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn TryMoveFocusAsync(
        focusnavigationdirection: FocusNavigationDirection,
    ) -> ::windows_core::Result<::windows::Foundation::IAsyncOperation<FocusMovementResult>> {
        Self::IFocusManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TryMoveFocusAsync)(
                ::windows_core::Interface::as_raw(this),
                focusnavigationdirection,
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn TryMoveFocusWithOptionsAsync<P0>(
        focusnavigationdirection: FocusNavigationDirection,
        focusnavigationoptions: P0,
    ) -> ::windows_core::Result<::windows::Foundation::IAsyncOperation<FocusMovementResult>>
    where
        P0: ::windows_core::IntoParam<FindNextElementOptions>,
    {
        Self::IFocusManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TryMoveFocusWithOptionsAsync)(
                ::windows_core::Interface::as_raw(this),
                focusnavigationdirection,
                focusnavigationoptions.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn TryMoveFocusWithOptions<P0>(
        focusnavigationdirection: FocusNavigationDirection,
        focusnavigationoptions: P0,
    ) -> ::windows_core::Result<bool>
    where
        P0: ::windows_core::IntoParam<FindNextElementOptions>,
    {
        Self::IFocusManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TryMoveFocusWithOptions)(
                ::windows_core::Interface::as_raw(this),
                focusnavigationdirection,
                focusnavigationoptions.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn FindNextElement(
        focusnavigationdirection: FocusNavigationDirection,
    ) -> ::windows_core::Result<super::DependencyObject> {
        Self::IFocusManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FindNextElement)(
                ::windows_core::Interface::as_raw(this),
                focusnavigationdirection,
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn FindFirstFocusableElement<P0>(
        searchscope: P0,
    ) -> ::windows_core::Result<super::DependencyObject>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        Self::IFocusManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FindFirstFocusableElement)(
                ::windows_core::Interface::as_raw(this),
                searchscope.try_into_param()?.abi(),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn FindLastFocusableElement<P0>(
        searchscope: P0,
    ) -> ::windows_core::Result<super::DependencyObject>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        Self::IFocusManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FindLastFocusableElement)(
                ::windows_core::Interface::as_raw(this),
                searchscope.try_into_param()?.abi(),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn FindNextElementWithOptions<P0>(
        focusnavigationdirection: FocusNavigationDirection,
        focusnavigationoptions: P0,
    ) -> ::windows_core::Result<super::DependencyObject>
    where
        P0: ::windows_core::IntoParam<FindNextElementOptions>,
    {
        Self::IFocusManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FindNextElementWithOptions)(
                ::windows_core::Interface::as_raw(this),
                focusnavigationdirection,
                focusnavigationoptions.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn FindNextFocusableElement(
        focusnavigationdirection: FocusNavigationDirection,
    ) -> ::windows_core::Result<super::UIElement> {
        Self::IFocusManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FindNextFocusableElement)(
                ::windows_core::Interface::as_raw(this),
                focusnavigationdirection,
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn FindNextFocusableElementWithHint(
        focusnavigationdirection: FocusNavigationDirection,
        hintrect: ::windows::Foundation::Rect,
    ) -> ::windows_core::Result<super::UIElement> {
        Self::IFocusManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FindNextFocusableElementWithHint)(
                ::windows_core::Interface::as_raw(this),
                focusnavigationdirection,
                hintrect,
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn TryMoveFocus(
        focusnavigationdirection: FocusNavigationDirection,
    ) -> ::windows_core::Result<bool> {
        Self::IFocusManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TryMoveFocus)(
                ::windows_core::Interface::as_raw(this),
                focusnavigationdirection,
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn GetFocusedElement() -> ::windows_core::Result<::windows_core::IInspectable> {
        Self::IFocusManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetFocusedElement)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn GetFocusedElementWithRoot<P0>(
        xamlroot: P0,
    ) -> ::windows_core::Result<::windows_core::IInspectable>
    where
        P0: ::windows_core::IntoParam<super::XamlRoot>,
    {
        Self::IFocusManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetFocusedElementWithRoot)(
                ::windows_core::Interface::as_raw(this),
                xamlroot.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IFocusManagerStatics<
        R,
        F: FnOnce(&IFocusManagerStatics) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<FocusManager, IFocusManagerStatics> =
            ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for FocusManager {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for FocusManager {
    type Vtable = IFocusManager_Vtbl;
}
unsafe impl ::windows_core::ComInterface for FocusManager {
    const IID: ::windows_core::GUID = <IFocusManager as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for FocusManager {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.FocusManager";
}
::windows_core::imp::interface_hierarchy!(
    FocusManager,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for FocusManager {}
unsafe impl ::core::marker::Sync for FocusManager {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct FocusManagerGotFocusEventArgs(::windows_core::IUnknown);
impl FocusManagerGotFocusEventArgs {
    pub fn NewFocusedElement(&self) -> ::windows_core::Result<super::DependencyObject> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NewFocusedElement)(
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
}
impl ::windows_core::RuntimeType for FocusManagerGotFocusEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for FocusManagerGotFocusEventArgs {
    type Vtable = IFocusManagerGotFocusEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for FocusManagerGotFocusEventArgs {
    const IID: ::windows_core::GUID =
        <IFocusManagerGotFocusEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for FocusManagerGotFocusEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.FocusManagerGotFocusEventArgs";
}
::windows_core::imp::interface_hierarchy!(
    FocusManagerGotFocusEventArgs,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for FocusManagerGotFocusEventArgs {}
unsafe impl ::core::marker::Sync for FocusManagerGotFocusEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct FocusManagerLostFocusEventArgs(::windows_core::IUnknown);
impl FocusManagerLostFocusEventArgs {
    pub fn OldFocusedElement(&self) -> ::windows_core::Result<super::DependencyObject> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OldFocusedElement)(
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
}
impl ::windows_core::RuntimeType for FocusManagerLostFocusEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for FocusManagerLostFocusEventArgs {
    type Vtable = IFocusManagerLostFocusEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for FocusManagerLostFocusEventArgs {
    const IID: ::windows_core::GUID =
        <IFocusManagerLostFocusEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for FocusManagerLostFocusEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.FocusManagerLostFocusEventArgs";
}
::windows_core::imp::interface_hierarchy!(
    FocusManagerLostFocusEventArgs,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for FocusManagerLostFocusEventArgs {}
unsafe impl ::core::marker::Sync for FocusManagerLostFocusEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct FocusMovementResult(::windows_core::IUnknown);
impl FocusMovementResult {
    pub fn Succeeded(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Succeeded)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for FocusMovementResult {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for FocusMovementResult {
    type Vtable = IFocusMovementResult_Vtbl;
}
unsafe impl ::windows_core::ComInterface for FocusMovementResult {
    const IID: ::windows_core::GUID = <IFocusMovementResult as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for FocusMovementResult {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.FocusMovementResult";
}
::windows_core::imp::interface_hierarchy!(
    FocusMovementResult,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for FocusMovementResult {}
unsafe impl ::core::marker::Sync for FocusMovementResult {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct GettingFocusEventArgs(::windows_core::IUnknown);
impl GettingFocusEventArgs {
    pub fn OldFocusedElement(&self) -> ::windows_core::Result<super::DependencyObject> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OldFocusedElement)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn NewFocusedElement(&self) -> ::windows_core::Result<super::DependencyObject> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NewFocusedElement)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetNewFocusedElement<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetNewFocusedElement)(
                ::windows_core::Interface::as_raw(this),
                value.try_into_param()?.abi(),
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
    pub fn Direction(&self) -> ::windows_core::Result<FocusNavigationDirection> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Direction)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
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
    pub fn InputDevice(&self) -> ::windows_core::Result<FocusInputDeviceKind> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).InputDevice)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
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
    pub fn TryCancel(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TryCancel)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn TrySetNewFocusedElement<P0>(&self, element: P0) -> ::windows_core::Result<bool>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TrySetNewFocusedElement)(
                ::windows_core::Interface::as_raw(this),
                element.try_into_param()?.abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
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
impl ::windows_core::RuntimeType for GettingFocusEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for GettingFocusEventArgs {
    type Vtable = IGettingFocusEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for GettingFocusEventArgs {
    const IID: ::windows_core::GUID = <IGettingFocusEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for GettingFocusEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.GettingFocusEventArgs";
}
::windows_core::imp::interface_hierarchy!(
    GettingFocusEventArgs,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<super::RoutedEventArgs> for GettingFocusEventArgs {}
unsafe impl ::core::marker::Send for GettingFocusEventArgs {}
unsafe impl ::core::marker::Sync for GettingFocusEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct HoldingRoutedEventArgs(::windows_core::IUnknown);
impl HoldingRoutedEventArgs {
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
            HoldingRoutedEventArgs,
            ::windows_core::imp::IGenericFactory,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "Required features: `\"Microsoft_UI_Input\"`"]
    #[cfg(feature = "Microsoft_UI_Input")]
    pub fn PointerDeviceType(
        &self,
    ) -> ::windows_core::Result<super::super::Input::PointerDeviceType> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PointerDeviceType)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Input\"`"]
    #[cfg(feature = "Microsoft_UI_Input")]
    pub fn HoldingState(&self) -> ::windows_core::Result<super::super::Input::HoldingState> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HoldingState)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
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
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn GetPosition<P0>(
        &self,
        relativeto: P0,
    ) -> ::windows_core::Result<::windows::Foundation::Point>
    where
        P0: ::windows_core::TryIntoParam<super::UIElement>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetPosition)(
                ::windows_core::Interface::as_raw(this),
                relativeto.try_into_param()?.abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
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
impl ::windows_core::RuntimeType for HoldingRoutedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for HoldingRoutedEventArgs {
    type Vtable = IHoldingRoutedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for HoldingRoutedEventArgs {
    const IID: ::windows_core::GUID =
        <IHoldingRoutedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for HoldingRoutedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.HoldingRoutedEventArgs";
}
::windows_core::imp::interface_hierarchy!(
    HoldingRoutedEventArgs,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<super::RoutedEventArgs> for HoldingRoutedEventArgs {}
unsafe impl ::core::marker::Send for HoldingRoutedEventArgs {}
unsafe impl ::core::marker::Sync for HoldingRoutedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct InertiaExpansionBehavior(::windows_core::IUnknown);
impl InertiaExpansionBehavior {
    pub fn DesiredDeceleration(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DesiredDeceleration)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetDesiredDeceleration(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetDesiredDeceleration)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn DesiredExpansion(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DesiredExpansion)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetDesiredExpansion(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetDesiredExpansion)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
}
impl ::windows_core::RuntimeType for InertiaExpansionBehavior {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for InertiaExpansionBehavior {
    type Vtable = IInertiaExpansionBehavior_Vtbl;
}
unsafe impl ::windows_core::ComInterface for InertiaExpansionBehavior {
    const IID: ::windows_core::GUID =
        <IInertiaExpansionBehavior as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for InertiaExpansionBehavior {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.InertiaExpansionBehavior";
}
::windows_core::imp::interface_hierarchy!(
    InertiaExpansionBehavior,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for InertiaExpansionBehavior {}
unsafe impl ::core::marker::Sync for InertiaExpansionBehavior {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct InertiaRotationBehavior(::windows_core::IUnknown);
impl InertiaRotationBehavior {
    pub fn DesiredDeceleration(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DesiredDeceleration)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetDesiredDeceleration(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetDesiredDeceleration)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn DesiredRotation(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DesiredRotation)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetDesiredRotation(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetDesiredRotation)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
}
impl ::windows_core::RuntimeType for InertiaRotationBehavior {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for InertiaRotationBehavior {
    type Vtable = IInertiaRotationBehavior_Vtbl;
}
unsafe impl ::windows_core::ComInterface for InertiaRotationBehavior {
    const IID: ::windows_core::GUID =
        <IInertiaRotationBehavior as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for InertiaRotationBehavior {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.InertiaRotationBehavior";
}
::windows_core::imp::interface_hierarchy!(
    InertiaRotationBehavior,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for InertiaRotationBehavior {}
unsafe impl ::core::marker::Sync for InertiaRotationBehavior {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct InertiaTranslationBehavior(::windows_core::IUnknown);
impl InertiaTranslationBehavior {
    pub fn DesiredDeceleration(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DesiredDeceleration)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetDesiredDeceleration(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetDesiredDeceleration)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn DesiredDisplacement(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DesiredDisplacement)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetDesiredDisplacement(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetDesiredDisplacement)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
}
impl ::windows_core::RuntimeType for InertiaTranslationBehavior {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for InertiaTranslationBehavior {
    type Vtable = IInertiaTranslationBehavior_Vtbl;
}
unsafe impl ::windows_core::ComInterface for InertiaTranslationBehavior {
    const IID: ::windows_core::GUID =
        <IInertiaTranslationBehavior as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for InertiaTranslationBehavior {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.InertiaTranslationBehavior";
}
::windows_core::imp::interface_hierarchy!(
    InertiaTranslationBehavior,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for InertiaTranslationBehavior {}
unsafe impl ::core::marker::Sync for InertiaTranslationBehavior {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct InputScope(::windows_core::IUnknown);
impl InputScope {
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
            InputScope,
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
    pub fn Names(
        &self,
    ) -> ::windows_core::Result<::windows::Foundation::Collections::IVector<InputScopeName>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Names)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for InputScope {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for InputScope {
    type Vtable = IInputScope_Vtbl;
}
unsafe impl ::windows_core::ComInterface for InputScope {
    const IID: ::windows_core::GUID = <IInputScope as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for InputScope {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.InputScope";
}
::windows_core::imp::interface_hierarchy!(
    InputScope,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<super::DependencyObject> for InputScope {}
unsafe impl ::core::marker::Send for InputScope {}
unsafe impl ::core::marker::Sync for InputScope {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct InputScopeName(::windows_core::IUnknown);
impl InputScopeName {
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
            InputScopeName,
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
    pub fn NameValue(&self) -> ::windows_core::Result<InputScopeNameValue> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NameValue)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetNameValue(&self, value: InputScopeNameValue) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetNameValue)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn CreateInstance(
        namevalue: InputScopeNameValue,
    ) -> ::windows_core::Result<InputScopeName> {
        Self::IInputScopeNameFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstance)(
                ::windows_core::Interface::as_raw(this),
                namevalue,
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IInputScopeNameFactory<
        R,
        F: FnOnce(&IInputScopeNameFactory) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<InputScopeName, IInputScopeNameFactory> =
            ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for InputScopeName {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for InputScopeName {
    type Vtable = IInputScopeName_Vtbl;
}
unsafe impl ::windows_core::ComInterface for InputScopeName {
    const IID: ::windows_core::GUID = <IInputScopeName as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for InputScopeName {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.InputScopeName";
}
::windows_core::imp::interface_hierarchy!(
    InputScopeName,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<super::DependencyObject> for InputScopeName {}
unsafe impl ::core::marker::Send for InputScopeName {}
unsafe impl ::core::marker::Sync for InputScopeName {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct KeyRoutedEventArgs(::windows_core::IUnknown);
impl KeyRoutedEventArgs {
    #[doc = "Required features: `\"Windows_System\"`"]
    #[cfg(feature = "Windows_System")]
    pub fn Key(&self) -> ::windows_core::Result<::windows::System::VirtualKey> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Key)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_UI_Core\"`"]
    #[cfg(feature = "Windows_UI_Core")]
    pub fn KeyStatus(&self) -> ::windows_core::Result<::windows::UI::Core::CorePhysicalKeyStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).KeyStatus)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
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
    #[doc = "Required features: `\"Windows_System\"`"]
    #[cfg(feature = "Windows_System")]
    pub fn OriginalKey(&self) -> ::windows_core::Result<::windows::System::VirtualKey> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OriginalKey)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn DeviceId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DeviceId)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
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
impl ::windows_core::RuntimeType for KeyRoutedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for KeyRoutedEventArgs {
    type Vtable = IKeyRoutedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for KeyRoutedEventArgs {
    const IID: ::windows_core::GUID = <IKeyRoutedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for KeyRoutedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.KeyRoutedEventArgs";
}
::windows_core::imp::interface_hierarchy!(
    KeyRoutedEventArgs,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<super::RoutedEventArgs> for KeyRoutedEventArgs {}
unsafe impl ::core::marker::Send for KeyRoutedEventArgs {}
unsafe impl ::core::marker::Sync for KeyRoutedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct KeyboardAccelerator(::windows_core::IUnknown);
impl KeyboardAccelerator {
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
    #[doc = "Required features: `\"Windows_System\"`"]
    #[cfg(feature = "Windows_System")]
    pub fn Key(&self) -> ::windows_core::Result<::windows::System::VirtualKey> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Key)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_System\"`"]
    #[cfg(feature = "Windows_System")]
    pub fn SetKey(&self, value: ::windows::System::VirtualKey) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetKey)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_System\"`"]
    #[cfg(feature = "Windows_System")]
    pub fn Modifiers(&self) -> ::windows_core::Result<::windows::System::VirtualKeyModifiers> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Modifiers)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_System\"`"]
    #[cfg(feature = "Windows_System")]
    pub fn SetModifiers(
        &self,
        value: ::windows::System::VirtualKeyModifiers,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetModifiers)(
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
    pub fn SetIsEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetIsEnabled)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn ScopeOwner(&self) -> ::windows_core::Result<super::DependencyObject> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ScopeOwner)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetScopeOwner<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetScopeOwner)(
                ::windows_core::Interface::as_raw(this),
                value.try_into_param()?.abi(),
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn Invoked<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<
                KeyboardAccelerator,
                KeyboardAcceleratorInvokedEventArgs,
            >,
        >,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Invoked)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveInvoked(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveInvoked)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub unsafe fn CreateInstance<P0>(
        baseinterface: P0,
        innerinterface: &mut ::core::option::Option<::windows_core::IInspectable>,
    ) -> ::windows_core::Result<KeyboardAccelerator>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        Self::IKeyboardAcceleratorFactory(|this| unsafe {
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
    pub fn KeyProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IKeyboardAcceleratorStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).KeyProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn ModifiersProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IKeyboardAcceleratorStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ModifiersProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn IsEnabledProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IKeyboardAcceleratorStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsEnabledProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn ScopeOwnerProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IKeyboardAcceleratorStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ScopeOwnerProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IKeyboardAcceleratorFactory<
        R,
        F: FnOnce(&IKeyboardAcceleratorFactory) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<
            KeyboardAccelerator,
            IKeyboardAcceleratorFactory,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IKeyboardAcceleratorStatics<
        R,
        F: FnOnce(&IKeyboardAcceleratorStatics) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<
            KeyboardAccelerator,
            IKeyboardAcceleratorStatics,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for KeyboardAccelerator {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for KeyboardAccelerator {
    type Vtable = IKeyboardAccelerator_Vtbl;
}
unsafe impl ::windows_core::ComInterface for KeyboardAccelerator {
    const IID: ::windows_core::GUID = <IKeyboardAccelerator as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for KeyboardAccelerator {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.KeyboardAccelerator";
}
::windows_core::imp::interface_hierarchy!(
    KeyboardAccelerator,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<super::DependencyObject> for KeyboardAccelerator {}
unsafe impl ::core::marker::Send for KeyboardAccelerator {}
unsafe impl ::core::marker::Sync for KeyboardAccelerator {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct KeyboardAcceleratorInvokedEventArgs(::windows_core::IUnknown);
impl KeyboardAcceleratorInvokedEventArgs {
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
    pub fn Element(&self) -> ::windows_core::Result<super::DependencyObject> {
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
    pub fn KeyboardAccelerator(&self) -> ::windows_core::Result<KeyboardAccelerator> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).KeyboardAccelerator)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for KeyboardAcceleratorInvokedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for KeyboardAcceleratorInvokedEventArgs {
    type Vtable = IKeyboardAcceleratorInvokedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for KeyboardAcceleratorInvokedEventArgs {
    const IID: ::windows_core::GUID =
        <IKeyboardAcceleratorInvokedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for KeyboardAcceleratorInvokedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.KeyboardAcceleratorInvokedEventArgs";
}
::windows_core::imp::interface_hierarchy!(
    KeyboardAcceleratorInvokedEventArgs,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for KeyboardAcceleratorInvokedEventArgs {}
unsafe impl ::core::marker::Sync for KeyboardAcceleratorInvokedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct LosingFocusEventArgs(::windows_core::IUnknown);
impl LosingFocusEventArgs {
    pub fn OldFocusedElement(&self) -> ::windows_core::Result<super::DependencyObject> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OldFocusedElement)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn NewFocusedElement(&self) -> ::windows_core::Result<super::DependencyObject> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NewFocusedElement)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetNewFocusedElement<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetNewFocusedElement)(
                ::windows_core::Interface::as_raw(this),
                value.try_into_param()?.abi(),
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
    pub fn Direction(&self) -> ::windows_core::Result<FocusNavigationDirection> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Direction)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
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
    pub fn InputDevice(&self) -> ::windows_core::Result<FocusInputDeviceKind> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).InputDevice)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
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
    pub fn TryCancel(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TryCancel)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn TrySetNewFocusedElement<P0>(&self, element: P0) -> ::windows_core::Result<bool>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TrySetNewFocusedElement)(
                ::windows_core::Interface::as_raw(this),
                element.try_into_param()?.abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
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
impl ::windows_core::RuntimeType for LosingFocusEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for LosingFocusEventArgs {
    type Vtable = ILosingFocusEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for LosingFocusEventArgs {
    const IID: ::windows_core::GUID = <ILosingFocusEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for LosingFocusEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.LosingFocusEventArgs";
}
::windows_core::imp::interface_hierarchy!(
    LosingFocusEventArgs,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<super::RoutedEventArgs> for LosingFocusEventArgs {}
unsafe impl ::core::marker::Send for LosingFocusEventArgs {}
unsafe impl ::core::marker::Sync for LosingFocusEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ManipulationCompletedRoutedEventArgs(::windows_core::IUnknown);
impl ManipulationCompletedRoutedEventArgs {
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
            ManipulationCompletedRoutedEventArgs,
            ::windows_core::imp::IGenericFactory,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Container(&self) -> ::windows_core::Result<super::UIElement> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Container)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn Position(&self) -> ::windows_core::Result<::windows::Foundation::Point> {
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
    pub fn IsInertial(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsInertial)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Input\"`, `\"Windows_Foundation\"`"]
    #[cfg(all(feature = "Microsoft_UI_Input", feature = "Windows_Foundation"))]
    pub fn Cumulative(&self) -> ::windows_core::Result<super::super::Input::ManipulationDelta> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Cumulative)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Input\"`, `\"Windows_Foundation\"`"]
    #[cfg(all(feature = "Microsoft_UI_Input", feature = "Windows_Foundation"))]
    pub fn Velocities(
        &self,
    ) -> ::windows_core::Result<super::super::Input::ManipulationVelocities> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Velocities)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
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
    #[doc = "Required features: `\"Microsoft_UI_Input\"`"]
    #[cfg(feature = "Microsoft_UI_Input")]
    pub fn PointerDeviceType(
        &self,
    ) -> ::windows_core::Result<super::super::Input::PointerDeviceType> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PointerDeviceType)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
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
impl ::windows_core::RuntimeType for ManipulationCompletedRoutedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for ManipulationCompletedRoutedEventArgs {
    type Vtable = IManipulationCompletedRoutedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ManipulationCompletedRoutedEventArgs {
    const IID: ::windows_core::GUID =
        <IManipulationCompletedRoutedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ManipulationCompletedRoutedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.ManipulationCompletedRoutedEventArgs";
}
::windows_core::imp::interface_hierarchy!(
    ManipulationCompletedRoutedEventArgs,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<super::RoutedEventArgs> for ManipulationCompletedRoutedEventArgs {}
unsafe impl ::core::marker::Send for ManipulationCompletedRoutedEventArgs {}
unsafe impl ::core::marker::Sync for ManipulationCompletedRoutedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ManipulationDeltaRoutedEventArgs(::windows_core::IUnknown);
impl ManipulationDeltaRoutedEventArgs {
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
            ManipulationDeltaRoutedEventArgs,
            ::windows_core::imp::IGenericFactory,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Container(&self) -> ::windows_core::Result<super::UIElement> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Container)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn Position(&self) -> ::windows_core::Result<::windows::Foundation::Point> {
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
    pub fn IsInertial(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsInertial)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Input\"`, `\"Windows_Foundation\"`"]
    #[cfg(all(feature = "Microsoft_UI_Input", feature = "Windows_Foundation"))]
    pub fn Delta(&self) -> ::windows_core::Result<super::super::Input::ManipulationDelta> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Delta)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Input\"`, `\"Windows_Foundation\"`"]
    #[cfg(all(feature = "Microsoft_UI_Input", feature = "Windows_Foundation"))]
    pub fn Cumulative(&self) -> ::windows_core::Result<super::super::Input::ManipulationDelta> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Cumulative)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Input\"`, `\"Windows_Foundation\"`"]
    #[cfg(all(feature = "Microsoft_UI_Input", feature = "Windows_Foundation"))]
    pub fn Velocities(
        &self,
    ) -> ::windows_core::Result<super::super::Input::ManipulationVelocities> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Velocities)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
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
    #[doc = "Required features: `\"Microsoft_UI_Input\"`"]
    #[cfg(feature = "Microsoft_UI_Input")]
    pub fn PointerDeviceType(
        &self,
    ) -> ::windows_core::Result<super::super::Input::PointerDeviceType> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PointerDeviceType)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn Complete(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).Complete)(::windows_core::Interface::as_raw(
                this,
            ))
            .ok()
        }
    }
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
impl ::windows_core::RuntimeType for ManipulationDeltaRoutedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for ManipulationDeltaRoutedEventArgs {
    type Vtable = IManipulationDeltaRoutedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ManipulationDeltaRoutedEventArgs {
    const IID: ::windows_core::GUID =
        <IManipulationDeltaRoutedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ManipulationDeltaRoutedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.ManipulationDeltaRoutedEventArgs";
}
::windows_core::imp::interface_hierarchy!(
    ManipulationDeltaRoutedEventArgs,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<super::RoutedEventArgs> for ManipulationDeltaRoutedEventArgs {}
unsafe impl ::core::marker::Send for ManipulationDeltaRoutedEventArgs {}
unsafe impl ::core::marker::Sync for ManipulationDeltaRoutedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ManipulationInertiaStartingRoutedEventArgs(::windows_core::IUnknown);
impl ManipulationInertiaStartingRoutedEventArgs {
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
            ManipulationInertiaStartingRoutedEventArgs,
            ::windows_core::imp::IGenericFactory,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Container(&self) -> ::windows_core::Result<super::UIElement> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Container)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn ExpansionBehavior(&self) -> ::windows_core::Result<InertiaExpansionBehavior> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ExpansionBehavior)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetExpansionBehavior<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<InertiaExpansionBehavior>,
    {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetExpansionBehavior)(
                ::windows_core::Interface::as_raw(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn RotationBehavior(&self) -> ::windows_core::Result<InertiaRotationBehavior> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RotationBehavior)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetRotationBehavior<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<InertiaRotationBehavior>,
    {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetRotationBehavior)(
                ::windows_core::Interface::as_raw(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn TranslationBehavior(&self) -> ::windows_core::Result<InertiaTranslationBehavior> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TranslationBehavior)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetTranslationBehavior<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<InertiaTranslationBehavior>,
    {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetTranslationBehavior)(
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
    #[doc = "Required features: `\"Microsoft_UI_Input\"`"]
    #[cfg(feature = "Microsoft_UI_Input")]
    pub fn PointerDeviceType(
        &self,
    ) -> ::windows_core::Result<super::super::Input::PointerDeviceType> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PointerDeviceType)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Input\"`, `\"Windows_Foundation\"`"]
    #[cfg(all(feature = "Microsoft_UI_Input", feature = "Windows_Foundation"))]
    pub fn Delta(&self) -> ::windows_core::Result<super::super::Input::ManipulationDelta> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Delta)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Input\"`, `\"Windows_Foundation\"`"]
    #[cfg(all(feature = "Microsoft_UI_Input", feature = "Windows_Foundation"))]
    pub fn Cumulative(&self) -> ::windows_core::Result<super::super::Input::ManipulationDelta> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Cumulative)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Input\"`, `\"Windows_Foundation\"`"]
    #[cfg(all(feature = "Microsoft_UI_Input", feature = "Windows_Foundation"))]
    pub fn Velocities(
        &self,
    ) -> ::windows_core::Result<super::super::Input::ManipulationVelocities> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Velocities)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
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
impl ::windows_core::RuntimeType for ManipulationInertiaStartingRoutedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for ManipulationInertiaStartingRoutedEventArgs {
    type Vtable = IManipulationInertiaStartingRoutedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ManipulationInertiaStartingRoutedEventArgs {
    const IID: ::windows_core::GUID =
        <IManipulationInertiaStartingRoutedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ManipulationInertiaStartingRoutedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.ManipulationInertiaStartingRoutedEventArgs";
}
::windows_core::imp::interface_hierarchy!(
    ManipulationInertiaStartingRoutedEventArgs,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<super::RoutedEventArgs>
    for ManipulationInertiaStartingRoutedEventArgs
{
}
unsafe impl ::core::marker::Send for ManipulationInertiaStartingRoutedEventArgs {}
unsafe impl ::core::marker::Sync for ManipulationInertiaStartingRoutedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ManipulationPivot(::windows_core::IUnknown);
impl ManipulationPivot {
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
            ManipulationPivot,
            ::windows_core::imp::IGenericFactory,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
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
    pub fn Radius(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Radius)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetRadius(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetRadius)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn CreateInstanceWithCenterAndRadius(
        center: ::windows::Foundation::Point,
        radius: f64,
    ) -> ::windows_core::Result<ManipulationPivot> {
        Self::IManipulationPivotFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstanceWithCenterAndRadius)(
                ::windows_core::Interface::as_raw(this),
                center,
                radius,
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IManipulationPivotFactory<
        R,
        F: FnOnce(&IManipulationPivotFactory) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<
            ManipulationPivot,
            IManipulationPivotFactory,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for ManipulationPivot {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for ManipulationPivot {
    type Vtable = IManipulationPivot_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ManipulationPivot {
    const IID: ::windows_core::GUID = <IManipulationPivot as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ManipulationPivot {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.ManipulationPivot";
}
::windows_core::imp::interface_hierarchy!(
    ManipulationPivot,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for ManipulationPivot {}
unsafe impl ::core::marker::Sync for ManipulationPivot {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ManipulationStartedRoutedEventArgs(::windows_core::IUnknown);
impl ManipulationStartedRoutedEventArgs {
    pub fn Container(&self) -> ::windows_core::Result<super::UIElement> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Container)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn Position(&self) -> ::windows_core::Result<::windows::Foundation::Point> {
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
    #[doc = "Required features: `\"Microsoft_UI_Input\"`"]
    #[cfg(feature = "Microsoft_UI_Input")]
    pub fn PointerDeviceType(
        &self,
    ) -> ::windows_core::Result<super::super::Input::PointerDeviceType> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PointerDeviceType)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Input\"`, `\"Windows_Foundation\"`"]
    #[cfg(all(feature = "Microsoft_UI_Input", feature = "Windows_Foundation"))]
    pub fn Cumulative(&self) -> ::windows_core::Result<super::super::Input::ManipulationDelta> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Cumulative)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn Complete(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).Complete)(::windows_core::Interface::as_raw(
                this,
            ))
            .ok()
        }
    }
    pub unsafe fn CreateInstance<P0>(
        baseinterface: P0,
        innerinterface: &mut ::core::option::Option<::windows_core::IInspectable>,
    ) -> ::windows_core::Result<ManipulationStartedRoutedEventArgs>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        Self::IManipulationStartedRoutedEventArgsFactory(|this| unsafe {
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
    #[doc(hidden)]
    pub fn IManipulationStartedRoutedEventArgsFactory<
        R,
        F: FnOnce(&IManipulationStartedRoutedEventArgsFactory) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<
            ManipulationStartedRoutedEventArgs,
            IManipulationStartedRoutedEventArgsFactory,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for ManipulationStartedRoutedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for ManipulationStartedRoutedEventArgs {
    type Vtable = IManipulationStartedRoutedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ManipulationStartedRoutedEventArgs {
    const IID: ::windows_core::GUID =
        <IManipulationStartedRoutedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ManipulationStartedRoutedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.ManipulationStartedRoutedEventArgs";
}
::windows_core::imp::interface_hierarchy!(
    ManipulationStartedRoutedEventArgs,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<super::RoutedEventArgs> for ManipulationStartedRoutedEventArgs {}
unsafe impl ::core::marker::Send for ManipulationStartedRoutedEventArgs {}
unsafe impl ::core::marker::Sync for ManipulationStartedRoutedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ManipulationStartingRoutedEventArgs(::windows_core::IUnknown);
impl ManipulationStartingRoutedEventArgs {
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
            ManipulationStartingRoutedEventArgs,
            ::windows_core::imp::IGenericFactory,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Mode(&self) -> ::windows_core::Result<ManipulationModes> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Mode)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetMode(&self, value: ManipulationModes) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetMode)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn Container(&self) -> ::windows_core::Result<super::UIElement> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Container)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetContainer<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::UIElement>,
    {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetContainer)(
                ::windows_core::Interface::as_raw(this),
                value.try_into_param()?.abi(),
            )
            .ok()
        }
    }
    pub fn Pivot(&self) -> ::windows_core::Result<ManipulationPivot> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Pivot)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetPivot<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<ManipulationPivot>,
    {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetPivot)(
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
impl ::windows_core::RuntimeType for ManipulationStartingRoutedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for ManipulationStartingRoutedEventArgs {
    type Vtable = IManipulationStartingRoutedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ManipulationStartingRoutedEventArgs {
    const IID: ::windows_core::GUID =
        <IManipulationStartingRoutedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ManipulationStartingRoutedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.ManipulationStartingRoutedEventArgs";
}
::windows_core::imp::interface_hierarchy!(
    ManipulationStartingRoutedEventArgs,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<super::RoutedEventArgs> for ManipulationStartingRoutedEventArgs {}
unsafe impl ::core::marker::Send for ManipulationStartingRoutedEventArgs {}
unsafe impl ::core::marker::Sync for ManipulationStartingRoutedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct NoFocusCandidateFoundEventArgs(::windows_core::IUnknown);
impl NoFocusCandidateFoundEventArgs {
    pub fn Direction(&self) -> ::windows_core::Result<FocusNavigationDirection> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Direction)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
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
    pub fn InputDevice(&self) -> ::windows_core::Result<FocusInputDeviceKind> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).InputDevice)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
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
impl ::windows_core::RuntimeType for NoFocusCandidateFoundEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for NoFocusCandidateFoundEventArgs {
    type Vtable = INoFocusCandidateFoundEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for NoFocusCandidateFoundEventArgs {
    const IID: ::windows_core::GUID =
        <INoFocusCandidateFoundEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for NoFocusCandidateFoundEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.NoFocusCandidateFoundEventArgs";
}
::windows_core::imp::interface_hierarchy!(
    NoFocusCandidateFoundEventArgs,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<super::RoutedEventArgs> for NoFocusCandidateFoundEventArgs {}
unsafe impl ::core::marker::Send for NoFocusCandidateFoundEventArgs {}
unsafe impl ::core::marker::Sync for NoFocusCandidateFoundEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct Pointer(::windows_core::IUnknown);
impl Pointer {
    pub fn PointerId(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PointerId)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Input\"`"]
    #[cfg(feature = "Microsoft_UI_Input")]
    pub fn PointerDeviceType(
        &self,
    ) -> ::windows_core::Result<super::super::Input::PointerDeviceType> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PointerDeviceType)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn IsInContact(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsInContact)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn IsInRange(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsInRange)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for Pointer {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for Pointer {
    type Vtable = IPointer_Vtbl;
}
unsafe impl ::windows_core::ComInterface for Pointer {
    const IID: ::windows_core::GUID = <IPointer as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for Pointer {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.Pointer";
}
::windows_core::imp::interface_hierarchy!(
    Pointer,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for Pointer {}
unsafe impl ::core::marker::Sync for Pointer {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct PointerRoutedEventArgs(::windows_core::IUnknown);
impl PointerRoutedEventArgs {
    pub fn Pointer(&self) -> ::windows_core::Result<Pointer> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Pointer)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_System\"`"]
    #[cfg(feature = "Windows_System")]
    pub fn KeyModifiers(&self) -> ::windows_core::Result<::windows::System::VirtualKeyModifiers> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).KeyModifiers)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
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
    pub fn IsGenerated(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsGenerated)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Input\"`"]
    #[cfg(feature = "Microsoft_UI_Input")]
    pub fn GetCurrentPoint<P0>(
        &self,
        relativeto: P0,
    ) -> ::windows_core::Result<super::super::Input::PointerPoint>
    where
        P0: ::windows_core::TryIntoParam<super::UIElement>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetCurrentPoint)(
                ::windows_core::Interface::as_raw(this),
                relativeto.try_into_param()?.abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Input\"`, `\"Windows_Foundation_Collections\"`"]
    #[cfg(all(
        feature = "Microsoft_UI_Input",
        feature = "Windows_Foundation_Collections"
    ))]
    pub fn GetIntermediatePoints<P0>(
        &self,
        relativeto: P0,
    ) -> ::windows_core::Result<
        ::windows::Foundation::Collections::IVector<super::super::Input::PointerPoint>,
    >
    where
        P0: ::windows_core::TryIntoParam<super::UIElement>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetIntermediatePoints)(
                ::windows_core::Interface::as_raw(this),
                relativeto.try_into_param()?.abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
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
impl ::windows_core::RuntimeType for PointerRoutedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for PointerRoutedEventArgs {
    type Vtable = IPointerRoutedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PointerRoutedEventArgs {
    const IID: ::windows_core::GUID =
        <IPointerRoutedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PointerRoutedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.PointerRoutedEventArgs";
}
::windows_core::imp::interface_hierarchy!(
    PointerRoutedEventArgs,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<super::RoutedEventArgs> for PointerRoutedEventArgs {}
unsafe impl ::core::marker::Send for PointerRoutedEventArgs {}
unsafe impl ::core::marker::Sync for PointerRoutedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ProcessKeyboardAcceleratorEventArgs(::windows_core::IUnknown);
impl ProcessKeyboardAcceleratorEventArgs {
    #[doc = "Required features: `\"Windows_System\"`"]
    #[cfg(feature = "Windows_System")]
    pub fn Key(&self) -> ::windows_core::Result<::windows::System::VirtualKey> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Key)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_System\"`"]
    #[cfg(feature = "Windows_System")]
    pub fn Modifiers(&self) -> ::windows_core::Result<::windows::System::VirtualKeyModifiers> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Modifiers)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
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
impl ::windows_core::RuntimeType for ProcessKeyboardAcceleratorEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for ProcessKeyboardAcceleratorEventArgs {
    type Vtable = IProcessKeyboardAcceleratorEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ProcessKeyboardAcceleratorEventArgs {
    const IID: ::windows_core::GUID =
        <IProcessKeyboardAcceleratorEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ProcessKeyboardAcceleratorEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.ProcessKeyboardAcceleratorEventArgs";
}
::windows_core::imp::interface_hierarchy!(
    ProcessKeyboardAcceleratorEventArgs,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for ProcessKeyboardAcceleratorEventArgs {}
unsafe impl ::core::marker::Sync for ProcessKeyboardAcceleratorEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct RightTappedRoutedEventArgs(::windows_core::IUnknown);
impl RightTappedRoutedEventArgs {
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
            RightTappedRoutedEventArgs,
            ::windows_core::imp::IGenericFactory,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "Required features: `\"Microsoft_UI_Input\"`"]
    #[cfg(feature = "Microsoft_UI_Input")]
    pub fn PointerDeviceType(
        &self,
    ) -> ::windows_core::Result<super::super::Input::PointerDeviceType> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PointerDeviceType)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
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
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn GetPosition<P0>(
        &self,
        relativeto: P0,
    ) -> ::windows_core::Result<::windows::Foundation::Point>
    where
        P0: ::windows_core::TryIntoParam<super::UIElement>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetPosition)(
                ::windows_core::Interface::as_raw(this),
                relativeto.try_into_param()?.abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
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
impl ::windows_core::RuntimeType for RightTappedRoutedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for RightTappedRoutedEventArgs {
    type Vtable = IRightTappedRoutedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for RightTappedRoutedEventArgs {
    const IID: ::windows_core::GUID =
        <IRightTappedRoutedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for RightTappedRoutedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.RightTappedRoutedEventArgs";
}
::windows_core::imp::interface_hierarchy!(
    RightTappedRoutedEventArgs,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<super::RoutedEventArgs> for RightTappedRoutedEventArgs {}
unsafe impl ::core::marker::Send for RightTappedRoutedEventArgs {}
unsafe impl ::core::marker::Sync for RightTappedRoutedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct StandardUICommand(::windows_core::IUnknown);
impl StandardUICommand {
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn CanExecuteChanged<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::EventHandler<::windows_core::IInspectable>,
        >,
    {
        let this = &::windows_core::ComInterface::cast::<ICommand>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CanExecuteChanged)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveCanExecuteChanged(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ICommand>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveCanExecuteChanged)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn CanExecute<P0>(&self, parameter: P0) -> ::windows_core::Result<bool>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = &::windows_core::ComInterface::cast::<ICommand>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CanExecute)(
                ::windows_core::Interface::as_raw(this),
                parameter.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn Execute<P0>(&self, parameter: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = &::windows_core::ComInterface::cast::<ICommand>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).Execute)(
                ::windows_core::Interface::as_raw(this),
                parameter.into_param().abi(),
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
    pub fn Kind(&self) -> ::windows_core::Result<StandardUICommandKind> {
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
    pub fn SetKind(&self, value: StandardUICommandKind) -> ::windows_core::Result<()> {
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
    ) -> ::windows_core::Result<StandardUICommand>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        Self::IStandardUICommandFactory(|this| unsafe {
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
    pub unsafe fn CreateInstanceWithKind<P0>(
        kind: StandardUICommandKind,
        baseinterface: P0,
        innerinterface: &mut ::core::option::Option<::windows_core::IInspectable>,
    ) -> ::windows_core::Result<StandardUICommand>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        Self::IStandardUICommandFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstanceWithKind)(
                ::windows_core::Interface::as_raw(this),
                kind,
                baseinterface.into_param().abi(),
                innerinterface as *mut _ as _,
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn KindProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IStandardUICommandStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).KindProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn Label(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IXamlUICommand>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Label)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetLabel(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IXamlUICommand>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetLabel)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Controls\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Controls")]
    pub fn IconSource(&self) -> ::windows_core::Result<super::Controls::IconSource> {
        let this = &::windows_core::ComInterface::cast::<IXamlUICommand>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IconSource)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Controls\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Controls")]
    pub fn SetIconSource<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::Controls::IconSource>,
    {
        let this = &::windows_core::ComInterface::cast::<IXamlUICommand>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetIconSource)(
                ::windows_core::Interface::as_raw(this),
                value.try_into_param()?.abi(),
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation_Collections\"`"]
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub fn KeyboardAccelerators(
        &self,
    ) -> ::windows_core::Result<::windows::Foundation::Collections::IVector<KeyboardAccelerator>>
    {
        let this = &::windows_core::ComInterface::cast::<IXamlUICommand>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).KeyboardAccelerators)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn AccessKey(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IXamlUICommand>(self)?;
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
        let this = &::windows_core::ComInterface::cast::<IXamlUICommand>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetAccessKey)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn Description(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IXamlUICommand>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Description)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetDescription(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IXamlUICommand>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetDescription)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn Command(&self) -> ::windows_core::Result<ICommand> {
        let this = &::windows_core::ComInterface::cast::<IXamlUICommand>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Command)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetCommand<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<ICommand>,
    {
        let this = &::windows_core::ComInterface::cast::<IXamlUICommand>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetCommand)(
                ::windows_core::Interface::as_raw(this),
                value.try_into_param()?.abi(),
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn ExecuteRequested<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<XamlUICommand, ExecuteRequestedEventArgs>,
        >,
    {
        let this = &::windows_core::ComInterface::cast::<IXamlUICommand>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ExecuteRequested)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveExecuteRequested(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IXamlUICommand>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveExecuteRequested)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn CanExecuteRequested<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<XamlUICommand, CanExecuteRequestedEventArgs>,
        >,
    {
        let this = &::windows_core::ComInterface::cast::<IXamlUICommand>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CanExecuteRequested)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveCanExecuteRequested(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IXamlUICommand>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveCanExecuteRequested)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn NotifyCanExecuteChanged(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IXamlUICommand>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).NotifyCanExecuteChanged)(
                ::windows_core::Interface::as_raw(this),
            )
            .ok()
        }
    }
    #[doc(hidden)]
    pub fn IStandardUICommandFactory<
        R,
        F: FnOnce(&IStandardUICommandFactory) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<
            StandardUICommand,
            IStandardUICommandFactory,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IStandardUICommandStatics<
        R,
        F: FnOnce(&IStandardUICommandStatics) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<
            StandardUICommand,
            IStandardUICommandStatics,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for StandardUICommand {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for StandardUICommand {
    type Vtable = IStandardUICommand_Vtbl;
}
unsafe impl ::windows_core::ComInterface for StandardUICommand {
    const IID: ::windows_core::GUID = <IStandardUICommand as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for StandardUICommand {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.StandardUICommand";
}
::windows_core::imp::interface_hierarchy!(
    StandardUICommand,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<ICommand> for StandardUICommand {}
impl ::windows_core::CanTryInto<XamlUICommand> for StandardUICommand {}
impl ::windows_core::CanTryInto<super::DependencyObject> for StandardUICommand {}
unsafe impl ::core::marker::Send for StandardUICommand {}
unsafe impl ::core::marker::Sync for StandardUICommand {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct TappedRoutedEventArgs(::windows_core::IUnknown);
impl TappedRoutedEventArgs {
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
            TappedRoutedEventArgs,
            ::windows_core::imp::IGenericFactory,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
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
    #[doc = "Required features: `\"Microsoft_UI_Input\"`"]
    #[cfg(feature = "Microsoft_UI_Input")]
    pub fn PointerDeviceType(
        &self,
    ) -> ::windows_core::Result<super::super::Input::PointerDeviceType> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PointerDeviceType)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
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
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn GetPosition<P0>(
        &self,
        relativeto: P0,
    ) -> ::windows_core::Result<::windows::Foundation::Point>
    where
        P0: ::windows_core::TryIntoParam<super::UIElement>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetPosition)(
                ::windows_core::Interface::as_raw(this),
                relativeto.try_into_param()?.abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for TappedRoutedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for TappedRoutedEventArgs {
    type Vtable = ITappedRoutedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for TappedRoutedEventArgs {
    const IID: ::windows_core::GUID = <ITappedRoutedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for TappedRoutedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.TappedRoutedEventArgs";
}
::windows_core::imp::interface_hierarchy!(
    TappedRoutedEventArgs,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<super::RoutedEventArgs> for TappedRoutedEventArgs {}
unsafe impl ::core::marker::Send for TappedRoutedEventArgs {}
unsafe impl ::core::marker::Sync for TappedRoutedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct XamlUICommand(::windows_core::IUnknown);
impl XamlUICommand {
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn CanExecuteChanged<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::EventHandler<::windows_core::IInspectable>,
        >,
    {
        let this = &::windows_core::ComInterface::cast::<ICommand>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CanExecuteChanged)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveCanExecuteChanged(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ICommand>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveCanExecuteChanged)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn CanExecute<P0>(&self, parameter: P0) -> ::windows_core::Result<bool>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = &::windows_core::ComInterface::cast::<ICommand>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CanExecute)(
                ::windows_core::Interface::as_raw(this),
                parameter.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn Execute<P0>(&self, parameter: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = &::windows_core::ComInterface::cast::<ICommand>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).Execute)(
                ::windows_core::Interface::as_raw(this),
                parameter.into_param().abi(),
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
    pub fn Label(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Label)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetLabel(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetLabel)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Controls\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Controls")]
    pub fn IconSource(&self) -> ::windows_core::Result<super::Controls::IconSource> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IconSource)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Controls\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Controls")]
    pub fn SetIconSource<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::Controls::IconSource>,
    {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetIconSource)(
                ::windows_core::Interface::as_raw(this),
                value.try_into_param()?.abi(),
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation_Collections\"`"]
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub fn KeyboardAccelerators(
        &self,
    ) -> ::windows_core::Result<::windows::Foundation::Collections::IVector<KeyboardAccelerator>>
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).KeyboardAccelerators)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
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
    pub fn Description(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Description)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetDescription(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetDescription)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn Command(&self) -> ::windows_core::Result<ICommand> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Command)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetCommand<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<ICommand>,
    {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetCommand)(
                ::windows_core::Interface::as_raw(this),
                value.try_into_param()?.abi(),
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn ExecuteRequested<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<XamlUICommand, ExecuteRequestedEventArgs>,
        >,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ExecuteRequested)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveExecuteRequested(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveExecuteRequested)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn CanExecuteRequested<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<XamlUICommand, CanExecuteRequestedEventArgs>,
        >,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CanExecuteRequested)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveCanExecuteRequested(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveCanExecuteRequested)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn NotifyCanExecuteChanged(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).NotifyCanExecuteChanged)(
                ::windows_core::Interface::as_raw(this),
            )
            .ok()
        }
    }
    pub unsafe fn CreateInstance<P0>(
        baseinterface: P0,
        innerinterface: &mut ::core::option::Option<::windows_core::IInspectable>,
    ) -> ::windows_core::Result<XamlUICommand>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        Self::IXamlUICommandFactory(|this| unsafe {
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
    pub fn LabelProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IXamlUICommandStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LabelProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn IconSourceProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IXamlUICommandStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IconSourceProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn KeyboardAcceleratorsProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IXamlUICommandStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).KeyboardAcceleratorsProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn AccessKeyProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IXamlUICommandStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AccessKeyProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn DescriptionProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IXamlUICommandStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DescriptionProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn CommandProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IXamlUICommandStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CommandProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IXamlUICommandFactory<
        R,
        F: FnOnce(&IXamlUICommandFactory) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<XamlUICommand, IXamlUICommandFactory> =
            ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IXamlUICommandStatics<
        R,
        F: FnOnce(&IXamlUICommandStatics) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<XamlUICommand, IXamlUICommandStatics> =
            ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for XamlUICommand {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for XamlUICommand {
    type Vtable = IXamlUICommand_Vtbl;
}
unsafe impl ::windows_core::ComInterface for XamlUICommand {
    const IID: ::windows_core::GUID = <IXamlUICommand as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for XamlUICommand {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.XamlUICommand";
}
::windows_core::imp::interface_hierarchy!(
    XamlUICommand,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<ICommand> for XamlUICommand {}
impl ::windows_core::CanTryInto<super::DependencyObject> for XamlUICommand {}
unsafe impl ::core::marker::Send for XamlUICommand {}
unsafe impl ::core::marker::Sync for XamlUICommand {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FocusInputDeviceKind(pub i32);
impl FocusInputDeviceKind {
    pub const None: Self = Self(0i32);
    pub const Mouse: Self = Self(1i32);
    pub const Touch: Self = Self(2i32);
    pub const Pen: Self = Self(3i32);
    pub const Keyboard: Self = Self(4i32);
    pub const GameController: Self = Self(5i32);
}
impl ::core::marker::Copy for FocusInputDeviceKind {}
impl ::core::clone::Clone for FocusInputDeviceKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FocusInputDeviceKind {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for FocusInputDeviceKind {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for FocusInputDeviceKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FocusInputDeviceKind").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for FocusInputDeviceKind {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::from_slice(
            b"enum(Microsoft.UI.Xaml.Input.FocusInputDeviceKind;i4)",
        );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FocusNavigationDirection(pub i32);
impl FocusNavigationDirection {
    pub const Next: Self = Self(0i32);
    pub const Previous: Self = Self(1i32);
    pub const Up: Self = Self(2i32);
    pub const Down: Self = Self(3i32);
    pub const Left: Self = Self(4i32);
    pub const Right: Self = Self(5i32);
    pub const None: Self = Self(6i32);
}
impl ::core::marker::Copy for FocusNavigationDirection {}
impl ::core::clone::Clone for FocusNavigationDirection {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FocusNavigationDirection {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for FocusNavigationDirection {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for FocusNavigationDirection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FocusNavigationDirection").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for FocusNavigationDirection {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::from_slice(
            b"enum(Microsoft.UI.Xaml.Input.FocusNavigationDirection;i4)",
        );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct InputScopeNameValue(pub i32);
impl InputScopeNameValue {
    pub const Default: Self = Self(0i32);
    pub const Url: Self = Self(1i32);
    pub const EmailSmtpAddress: Self = Self(5i32);
    pub const PersonalFullName: Self = Self(7i32);
    pub const CurrencyAmountAndSymbol: Self = Self(20i32);
    pub const CurrencyAmount: Self = Self(21i32);
    pub const DateMonthNumber: Self = Self(23i32);
    pub const DateDayNumber: Self = Self(24i32);
    pub const DateYear: Self = Self(25i32);
    pub const Digits: Self = Self(28i32);
    pub const Number: Self = Self(29i32);
    pub const Password: Self = Self(31i32);
    pub const TelephoneNumber: Self = Self(32i32);
    pub const TelephoneCountryCode: Self = Self(33i32);
    pub const TelephoneAreaCode: Self = Self(34i32);
    pub const TelephoneLocalNumber: Self = Self(35i32);
    pub const TimeHour: Self = Self(37i32);
    pub const TimeMinutesOrSeconds: Self = Self(38i32);
    pub const NumberFullWidth: Self = Self(39i32);
    pub const AlphanumericHalfWidth: Self = Self(40i32);
    pub const AlphanumericFullWidth: Self = Self(41i32);
    pub const Hiragana: Self = Self(44i32);
    pub const KatakanaHalfWidth: Self = Self(45i32);
    pub const KatakanaFullWidth: Self = Self(46i32);
    pub const Hanja: Self = Self(47i32);
    pub const HangulHalfWidth: Self = Self(48i32);
    pub const HangulFullWidth: Self = Self(49i32);
    pub const Search: Self = Self(50i32);
    pub const Formula: Self = Self(51i32);
    pub const SearchIncremental: Self = Self(52i32);
    pub const ChineseHalfWidth: Self = Self(53i32);
    pub const ChineseFullWidth: Self = Self(54i32);
    pub const NativeScript: Self = Self(55i32);
    pub const Text: Self = Self(57i32);
    pub const Chat: Self = Self(58i32);
    pub const NameOrPhoneNumber: Self = Self(59i32);
    pub const EmailNameOrAddress: Self = Self(60i32);
    pub const Maps: Self = Self(62i32);
    pub const NumericPassword: Self = Self(63i32);
    pub const NumericPin: Self = Self(64i32);
    pub const AlphanumericPin: Self = Self(65i32);
    pub const FormulaNumber: Self = Self(67i32);
    pub const ChatWithoutEmoji: Self = Self(68i32);
}
impl ::core::marker::Copy for InputScopeNameValue {}
impl ::core::clone::Clone for InputScopeNameValue {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for InputScopeNameValue {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for InputScopeNameValue {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for InputScopeNameValue {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InputScopeNameValue").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for InputScopeNameValue {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::from_slice(
            b"enum(Microsoft.UI.Xaml.Input.InputScopeNameValue;i4)",
        );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KeyTipPlacementMode(pub i32);
impl KeyTipPlacementMode {
    pub const Auto: Self = Self(0i32);
    pub const Bottom: Self = Self(1i32);
    pub const Top: Self = Self(2i32);
    pub const Left: Self = Self(3i32);
    pub const Right: Self = Self(4i32);
    pub const Center: Self = Self(5i32);
    pub const Hidden: Self = Self(6i32);
}
impl ::core::marker::Copy for KeyTipPlacementMode {}
impl ::core::clone::Clone for KeyTipPlacementMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KeyTipPlacementMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for KeyTipPlacementMode {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for KeyTipPlacementMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KeyTipPlacementMode").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for KeyTipPlacementMode {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::from_slice(
            b"enum(Microsoft.UI.Xaml.Input.KeyTipPlacementMode;i4)",
        );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KeyboardAcceleratorPlacementMode(pub i32);
impl KeyboardAcceleratorPlacementMode {
    pub const Auto: Self = Self(0i32);
    pub const Hidden: Self = Self(1i32);
}
impl ::core::marker::Copy for KeyboardAcceleratorPlacementMode {}
impl ::core::clone::Clone for KeyboardAcceleratorPlacementMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KeyboardAcceleratorPlacementMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for KeyboardAcceleratorPlacementMode {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for KeyboardAcceleratorPlacementMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KeyboardAcceleratorPlacementMode").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for KeyboardAcceleratorPlacementMode {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::from_slice(
            b"enum(Microsoft.UI.Xaml.Input.KeyboardAcceleratorPlacementMode;i4)",
        );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KeyboardNavigationMode(pub i32);
impl KeyboardNavigationMode {
    pub const Local: Self = Self(0i32);
    pub const Cycle: Self = Self(1i32);
    pub const Once: Self = Self(2i32);
}
impl ::core::marker::Copy for KeyboardNavigationMode {}
impl ::core::clone::Clone for KeyboardNavigationMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KeyboardNavigationMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for KeyboardNavigationMode {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for KeyboardNavigationMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KeyboardNavigationMode").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for KeyboardNavigationMode {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::from_slice(
            b"enum(Microsoft.UI.Xaml.Input.KeyboardNavigationMode;i4)",
        );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ManipulationModes(pub u32);
impl ManipulationModes {
    pub const None: Self = Self(0u32);
    pub const TranslateX: Self = Self(1u32);
    pub const TranslateY: Self = Self(2u32);
    pub const TranslateRailsX: Self = Self(4u32);
    pub const TranslateRailsY: Self = Self(8u32);
    pub const Rotate: Self = Self(16u32);
    pub const Scale: Self = Self(32u32);
    pub const TranslateInertia: Self = Self(64u32);
    pub const RotateInertia: Self = Self(128u32);
    pub const ScaleInertia: Self = Self(256u32);
    pub const All: Self = Self(65535u32);
    pub const System: Self = Self(65536u32);
}
impl ::core::marker::Copy for ManipulationModes {}
impl ::core::clone::Clone for ManipulationModes {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ManipulationModes {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for ManipulationModes {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for ManipulationModes {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ManipulationModes").field(&self.0).finish()
    }
}
impl ManipulationModes {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for ManipulationModes {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for ManipulationModes {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for ManipulationModes {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for ManipulationModes {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for ManipulationModes {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::windows_core::RuntimeType for ManipulationModes {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::from_slice(
            b"enum(Microsoft.UI.Xaml.Input.ManipulationModes;u4)",
        );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct StandardUICommandKind(pub i32);
impl StandardUICommandKind {
    pub const None: Self = Self(0i32);
    pub const Cut: Self = Self(1i32);
    pub const Copy: Self = Self(2i32);
    pub const Paste: Self = Self(3i32);
    pub const SelectAll: Self = Self(4i32);
    pub const Delete: Self = Self(5i32);
    pub const Share: Self = Self(6i32);
    pub const Save: Self = Self(7i32);
    pub const Open: Self = Self(8i32);
    pub const Close: Self = Self(9i32);
    pub const Pause: Self = Self(10i32);
    pub const Play: Self = Self(11i32);
    pub const Stop: Self = Self(12i32);
    pub const Forward: Self = Self(13i32);
    pub const Backward: Self = Self(14i32);
    pub const Undo: Self = Self(15i32);
    pub const Redo: Self = Self(16i32);
}
impl ::core::marker::Copy for StandardUICommandKind {}
impl ::core::clone::Clone for StandardUICommandKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for StandardUICommandKind {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for StandardUICommandKind {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for StandardUICommandKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StandardUICommandKind").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for StandardUICommandKind {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::from_slice(
            b"enum(Microsoft.UI.Xaml.Input.StandardUICommandKind;i4)",
        );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct XYFocusKeyboardNavigationMode(pub i32);
impl XYFocusKeyboardNavigationMode {
    pub const Auto: Self = Self(0i32);
    pub const Enabled: Self = Self(1i32);
    pub const Disabled: Self = Self(2i32);
}
impl ::core::marker::Copy for XYFocusKeyboardNavigationMode {}
impl ::core::clone::Clone for XYFocusKeyboardNavigationMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for XYFocusKeyboardNavigationMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for XYFocusKeyboardNavigationMode {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for XYFocusKeyboardNavigationMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XYFocusKeyboardNavigationMode").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for XYFocusKeyboardNavigationMode {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::from_slice(
            b"enum(Microsoft.UI.Xaml.Input.XYFocusKeyboardNavigationMode;i4)",
        );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct XYFocusNavigationStrategy(pub i32);
impl XYFocusNavigationStrategy {
    pub const Auto: Self = Self(0i32);
    pub const Projection: Self = Self(1i32);
    pub const NavigationDirectionDistance: Self = Self(2i32);
    pub const RectilinearDistance: Self = Self(3i32);
}
impl ::core::marker::Copy for XYFocusNavigationStrategy {}
impl ::core::clone::Clone for XYFocusNavigationStrategy {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for XYFocusNavigationStrategy {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for XYFocusNavigationStrategy {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for XYFocusNavigationStrategy {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XYFocusNavigationStrategy").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for XYFocusNavigationStrategy {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::from_slice(
            b"enum(Microsoft.UI.Xaml.Input.XYFocusNavigationStrategy;i4)",
        );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct XYFocusNavigationStrategyOverride(pub i32);
impl XYFocusNavigationStrategyOverride {
    pub const None: Self = Self(0i32);
    pub const Auto: Self = Self(1i32);
    pub const Projection: Self = Self(2i32);
    pub const NavigationDirectionDistance: Self = Self(3i32);
    pub const RectilinearDistance: Self = Self(4i32);
}
impl ::core::marker::Copy for XYFocusNavigationStrategyOverride {}
impl ::core::clone::Clone for XYFocusNavigationStrategyOverride {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for XYFocusNavigationStrategyOverride {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for XYFocusNavigationStrategyOverride {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for XYFocusNavigationStrategyOverride {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XYFocusNavigationStrategyOverride").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for XYFocusNavigationStrategyOverride {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::from_slice(
            b"enum(Microsoft.UI.Xaml.Input.XYFocusNavigationStrategyOverride;i4)",
        );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct DoubleTappedEventHandler(pub ::windows_core::IUnknown);
impl DoubleTappedEventHandler {
    pub fn new<
        F: FnMut(
                ::core::option::Option<&::windows_core::IInspectable>,
                ::core::option::Option<&DoubleTappedRoutedEventArgs>,
            ) -> ::windows_core::Result<()>
            + ::core::marker::Send
            + 'static,
    >(
        invoke: F,
    ) -> Self {
        let com = DoubleTappedEventHandlerBox::<F> {
            vtable: &DoubleTappedEventHandlerBox::<F>::VTABLE,
            count: ::windows_core::imp::RefCount::new(1),
            invoke,
        };
        unsafe { ::core::mem::transmute(::std::boxed::Box::new(com)) }
    }
    pub fn Invoke<P0, P1>(&self, sender: P0, e: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
        P1: ::windows_core::IntoParam<DoubleTappedRoutedEventArgs>,
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
struct DoubleTappedEventHandlerBox<
    F: FnMut(
            ::core::option::Option<&::windows_core::IInspectable>,
            ::core::option::Option<&DoubleTappedRoutedEventArgs>,
        ) -> ::windows_core::Result<()>
        + ::core::marker::Send
        + 'static,
> {
    vtable: *const DoubleTappedEventHandler_Vtbl,
    invoke: F,
    count: ::windows_core::imp::RefCount,
}
impl<
        F: FnMut(
                ::core::option::Option<&::windows_core::IInspectable>,
                ::core::option::Option<&DoubleTappedRoutedEventArgs>,
            ) -> ::windows_core::Result<()>
            + ::core::marker::Send
            + 'static,
    > DoubleTappedEventHandlerBox<F>
{
    const VTABLE: DoubleTappedEventHandler_Vtbl = DoubleTappedEventHandler_Vtbl {
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
        *interface = if *iid == <DoubleTappedEventHandler as ::windows_core::ComInterface>::IID
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
unsafe impl ::windows_core::Interface for DoubleTappedEventHandler {
    type Vtable = DoubleTappedEventHandler_Vtbl;
}
unsafe impl ::windows_core::ComInterface for DoubleTappedEventHandler {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xf7a501b9_e277_5611_87b0_0e0607622183);
}
impl ::windows_core::RuntimeType for DoubleTappedEventHandler {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct DoubleTappedEventHandler_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        sender: *mut ::core::ffi::c_void,
        e: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct HoldingEventHandler(pub ::windows_core::IUnknown);
impl HoldingEventHandler {
    pub fn new<
        F: FnMut(
                ::core::option::Option<&::windows_core::IInspectable>,
                ::core::option::Option<&HoldingRoutedEventArgs>,
            ) -> ::windows_core::Result<()>
            + ::core::marker::Send
            + 'static,
    >(
        invoke: F,
    ) -> Self {
        let com = HoldingEventHandlerBox::<F> {
            vtable: &HoldingEventHandlerBox::<F>::VTABLE,
            count: ::windows_core::imp::RefCount::new(1),
            invoke,
        };
        unsafe { ::core::mem::transmute(::std::boxed::Box::new(com)) }
    }
    pub fn Invoke<P0, P1>(&self, sender: P0, e: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
        P1: ::windows_core::IntoParam<HoldingRoutedEventArgs>,
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
struct HoldingEventHandlerBox<
    F: FnMut(
            ::core::option::Option<&::windows_core::IInspectable>,
            ::core::option::Option<&HoldingRoutedEventArgs>,
        ) -> ::windows_core::Result<()>
        + ::core::marker::Send
        + 'static,
> {
    vtable: *const HoldingEventHandler_Vtbl,
    invoke: F,
    count: ::windows_core::imp::RefCount,
}
impl<
        F: FnMut(
                ::core::option::Option<&::windows_core::IInspectable>,
                ::core::option::Option<&HoldingRoutedEventArgs>,
            ) -> ::windows_core::Result<()>
            + ::core::marker::Send
            + 'static,
    > HoldingEventHandlerBox<F>
{
    const VTABLE: HoldingEventHandler_Vtbl = HoldingEventHandler_Vtbl {
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
        *interface = if *iid == <HoldingEventHandler as ::windows_core::ComInterface>::IID
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
unsafe impl ::windows_core::Interface for HoldingEventHandler {
    type Vtable = HoldingEventHandler_Vtbl;
}
unsafe impl ::windows_core::ComInterface for HoldingEventHandler {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xfe23c5bd_4984_56b6_b92b_fc9d1216b24e);
}
impl ::windows_core::RuntimeType for HoldingEventHandler {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct HoldingEventHandler_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        sender: *mut ::core::ffi::c_void,
        e: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct KeyEventHandler(pub ::windows_core::IUnknown);
impl KeyEventHandler {
    pub fn new<
        F: FnMut(
                ::core::option::Option<&::windows_core::IInspectable>,
                ::core::option::Option<&KeyRoutedEventArgs>,
            ) -> ::windows_core::Result<()>
            + ::core::marker::Send
            + 'static,
    >(
        invoke: F,
    ) -> Self {
        let com = KeyEventHandlerBox::<F> {
            vtable: &KeyEventHandlerBox::<F>::VTABLE,
            count: ::windows_core::imp::RefCount::new(1),
            invoke,
        };
        unsafe { ::core::mem::transmute(::std::boxed::Box::new(com)) }
    }
    pub fn Invoke<P0, P1>(&self, sender: P0, e: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
        P1: ::windows_core::IntoParam<KeyRoutedEventArgs>,
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
struct KeyEventHandlerBox<
    F: FnMut(
            ::core::option::Option<&::windows_core::IInspectable>,
            ::core::option::Option<&KeyRoutedEventArgs>,
        ) -> ::windows_core::Result<()>
        + ::core::marker::Send
        + 'static,
> {
    vtable: *const KeyEventHandler_Vtbl,
    invoke: F,
    count: ::windows_core::imp::RefCount,
}
impl<
        F: FnMut(
                ::core::option::Option<&::windows_core::IInspectable>,
                ::core::option::Option<&KeyRoutedEventArgs>,
            ) -> ::windows_core::Result<()>
            + ::core::marker::Send
            + 'static,
    > KeyEventHandlerBox<F>
{
    const VTABLE: KeyEventHandler_Vtbl = KeyEventHandler_Vtbl {
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
        *interface = if *iid == <KeyEventHandler as ::windows_core::ComInterface>::IID
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
unsafe impl ::windows_core::Interface for KeyEventHandler {
    type Vtable = KeyEventHandler_Vtbl;
}
unsafe impl ::windows_core::ComInterface for KeyEventHandler {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xdb68e7cc_9a2b_527d_9989_25284daccc03);
}
impl ::windows_core::RuntimeType for KeyEventHandler {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct KeyEventHandler_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        sender: *mut ::core::ffi::c_void,
        e: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ManipulationCompletedEventHandler(pub ::windows_core::IUnknown);
impl ManipulationCompletedEventHandler {
    pub fn new<
        F: FnMut(
                ::core::option::Option<&::windows_core::IInspectable>,
                ::core::option::Option<&ManipulationCompletedRoutedEventArgs>,
            ) -> ::windows_core::Result<()>
            + ::core::marker::Send
            + 'static,
    >(
        invoke: F,
    ) -> Self {
        let com = ManipulationCompletedEventHandlerBox::<F> {
            vtable: &ManipulationCompletedEventHandlerBox::<F>::VTABLE,
            count: ::windows_core::imp::RefCount::new(1),
            invoke,
        };
        unsafe { ::core::mem::transmute(::std::boxed::Box::new(com)) }
    }
    pub fn Invoke<P0, P1>(&self, sender: P0, e: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
        P1: ::windows_core::IntoParam<ManipulationCompletedRoutedEventArgs>,
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
struct ManipulationCompletedEventHandlerBox<
    F: FnMut(
            ::core::option::Option<&::windows_core::IInspectable>,
            ::core::option::Option<&ManipulationCompletedRoutedEventArgs>,
        ) -> ::windows_core::Result<()>
        + ::core::marker::Send
        + 'static,
> {
    vtable: *const ManipulationCompletedEventHandler_Vtbl,
    invoke: F,
    count: ::windows_core::imp::RefCount,
}
impl<
        F: FnMut(
                ::core::option::Option<&::windows_core::IInspectable>,
                ::core::option::Option<&ManipulationCompletedRoutedEventArgs>,
            ) -> ::windows_core::Result<()>
            + ::core::marker::Send
            + 'static,
    > ManipulationCompletedEventHandlerBox<F>
{
    const VTABLE: ManipulationCompletedEventHandler_Vtbl = ManipulationCompletedEventHandler_Vtbl {
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
        *interface = if *iid
            == <ManipulationCompletedEventHandler as ::windows_core::ComInterface>::IID
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
unsafe impl ::windows_core::Interface for ManipulationCompletedEventHandler {
    type Vtable = ManipulationCompletedEventHandler_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ManipulationCompletedEventHandler {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xd51df8db_71cd_5bfd_8426_767218ee55ec);
}
impl ::windows_core::RuntimeType for ManipulationCompletedEventHandler {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ManipulationCompletedEventHandler_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        sender: *mut ::core::ffi::c_void,
        e: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ManipulationDeltaEventHandler(pub ::windows_core::IUnknown);
impl ManipulationDeltaEventHandler {
    pub fn new<
        F: FnMut(
                ::core::option::Option<&::windows_core::IInspectable>,
                ::core::option::Option<&ManipulationDeltaRoutedEventArgs>,
            ) -> ::windows_core::Result<()>
            + ::core::marker::Send
            + 'static,
    >(
        invoke: F,
    ) -> Self {
        let com = ManipulationDeltaEventHandlerBox::<F> {
            vtable: &ManipulationDeltaEventHandlerBox::<F>::VTABLE,
            count: ::windows_core::imp::RefCount::new(1),
            invoke,
        };
        unsafe { ::core::mem::transmute(::std::boxed::Box::new(com)) }
    }
    pub fn Invoke<P0, P1>(&self, sender: P0, e: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
        P1: ::windows_core::IntoParam<ManipulationDeltaRoutedEventArgs>,
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
struct ManipulationDeltaEventHandlerBox<
    F: FnMut(
            ::core::option::Option<&::windows_core::IInspectable>,
            ::core::option::Option<&ManipulationDeltaRoutedEventArgs>,
        ) -> ::windows_core::Result<()>
        + ::core::marker::Send
        + 'static,
> {
    vtable: *const ManipulationDeltaEventHandler_Vtbl,
    invoke: F,
    count: ::windows_core::imp::RefCount,
}
impl<
        F: FnMut(
                ::core::option::Option<&::windows_core::IInspectable>,
                ::core::option::Option<&ManipulationDeltaRoutedEventArgs>,
            ) -> ::windows_core::Result<()>
            + ::core::marker::Send
            + 'static,
    > ManipulationDeltaEventHandlerBox<F>
{
    const VTABLE: ManipulationDeltaEventHandler_Vtbl = ManipulationDeltaEventHandler_Vtbl {
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
        *interface = if *iid == <ManipulationDeltaEventHandler as ::windows_core::ComInterface>::IID
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
unsafe impl ::windows_core::Interface for ManipulationDeltaEventHandler {
    type Vtable = ManipulationDeltaEventHandler_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ManipulationDeltaEventHandler {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x83f2d4ce_105f_5392_a38a_b7467b7c2ea5);
}
impl ::windows_core::RuntimeType for ManipulationDeltaEventHandler {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ManipulationDeltaEventHandler_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        sender: *mut ::core::ffi::c_void,
        e: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ManipulationInertiaStartingEventHandler(pub ::windows_core::IUnknown);
impl ManipulationInertiaStartingEventHandler {
    pub fn new<
        F: FnMut(
                ::core::option::Option<&::windows_core::IInspectable>,
                ::core::option::Option<&ManipulationInertiaStartingRoutedEventArgs>,
            ) -> ::windows_core::Result<()>
            + ::core::marker::Send
            + 'static,
    >(
        invoke: F,
    ) -> Self {
        let com = ManipulationInertiaStartingEventHandlerBox::<F> {
            vtable: &ManipulationInertiaStartingEventHandlerBox::<F>::VTABLE,
            count: ::windows_core::imp::RefCount::new(1),
            invoke,
        };
        unsafe { ::core::mem::transmute(::std::boxed::Box::new(com)) }
    }
    pub fn Invoke<P0, P1>(&self, sender: P0, e: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
        P1: ::windows_core::IntoParam<ManipulationInertiaStartingRoutedEventArgs>,
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
struct ManipulationInertiaStartingEventHandlerBox<
    F: FnMut(
            ::core::option::Option<&::windows_core::IInspectable>,
            ::core::option::Option<&ManipulationInertiaStartingRoutedEventArgs>,
        ) -> ::windows_core::Result<()>
        + ::core::marker::Send
        + 'static,
> {
    vtable: *const ManipulationInertiaStartingEventHandler_Vtbl,
    invoke: F,
    count: ::windows_core::imp::RefCount,
}
impl<
        F: FnMut(
                ::core::option::Option<&::windows_core::IInspectable>,
                ::core::option::Option<&ManipulationInertiaStartingRoutedEventArgs>,
            ) -> ::windows_core::Result<()>
            + ::core::marker::Send
            + 'static,
    > ManipulationInertiaStartingEventHandlerBox<F>
{
    const VTABLE: ManipulationInertiaStartingEventHandler_Vtbl =
        ManipulationInertiaStartingEventHandler_Vtbl {
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
        *interface = if *iid
            == <ManipulationInertiaStartingEventHandler as ::windows_core::ComInterface>::IID
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
unsafe impl ::windows_core::Interface for ManipulationInertiaStartingEventHandler {
    type Vtable = ManipulationInertiaStartingEventHandler_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ManipulationInertiaStartingEventHandler {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x5de296bd_6f1c_5f60_9180_10705282576c);
}
impl ::windows_core::RuntimeType for ManipulationInertiaStartingEventHandler {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ManipulationInertiaStartingEventHandler_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        sender: *mut ::core::ffi::c_void,
        e: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ManipulationStartedEventHandler(pub ::windows_core::IUnknown);
impl ManipulationStartedEventHandler {
    pub fn new<
        F: FnMut(
                ::core::option::Option<&::windows_core::IInspectable>,
                ::core::option::Option<&ManipulationStartedRoutedEventArgs>,
            ) -> ::windows_core::Result<()>
            + ::core::marker::Send
            + 'static,
    >(
        invoke: F,
    ) -> Self {
        let com = ManipulationStartedEventHandlerBox::<F> {
            vtable: &ManipulationStartedEventHandlerBox::<F>::VTABLE,
            count: ::windows_core::imp::RefCount::new(1),
            invoke,
        };
        unsafe { ::core::mem::transmute(::std::boxed::Box::new(com)) }
    }
    pub fn Invoke<P0, P1>(&self, sender: P0, e: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
        P1: ::windows_core::TryIntoParam<ManipulationStartedRoutedEventArgs>,
    {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).Invoke)(
                ::windows_core::Interface::as_raw(this),
                sender.into_param().abi(),
                e.try_into_param()?.abi(),
            )
            .ok()
        }
    }
}
#[repr(C)]
struct ManipulationStartedEventHandlerBox<
    F: FnMut(
            ::core::option::Option<&::windows_core::IInspectable>,
            ::core::option::Option<&ManipulationStartedRoutedEventArgs>,
        ) -> ::windows_core::Result<()>
        + ::core::marker::Send
        + 'static,
> {
    vtable: *const ManipulationStartedEventHandler_Vtbl,
    invoke: F,
    count: ::windows_core::imp::RefCount,
}
impl<
        F: FnMut(
                ::core::option::Option<&::windows_core::IInspectable>,
                ::core::option::Option<&ManipulationStartedRoutedEventArgs>,
            ) -> ::windows_core::Result<()>
            + ::core::marker::Send
            + 'static,
    > ManipulationStartedEventHandlerBox<F>
{
    const VTABLE: ManipulationStartedEventHandler_Vtbl = ManipulationStartedEventHandler_Vtbl {
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
        *interface = if *iid
            == <ManipulationStartedEventHandler as ::windows_core::ComInterface>::IID
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
unsafe impl ::windows_core::Interface for ManipulationStartedEventHandler {
    type Vtable = ManipulationStartedEventHandler_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ManipulationStartedEventHandler {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x41060669_304c_53ac_9d43_bc311235aae4);
}
impl ::windows_core::RuntimeType for ManipulationStartedEventHandler {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ManipulationStartedEventHandler_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        sender: *mut ::core::ffi::c_void,
        e: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ManipulationStartingEventHandler(pub ::windows_core::IUnknown);
impl ManipulationStartingEventHandler {
    pub fn new<
        F: FnMut(
                ::core::option::Option<&::windows_core::IInspectable>,
                ::core::option::Option<&ManipulationStartingRoutedEventArgs>,
            ) -> ::windows_core::Result<()>
            + ::core::marker::Send
            + 'static,
    >(
        invoke: F,
    ) -> Self {
        let com = ManipulationStartingEventHandlerBox::<F> {
            vtable: &ManipulationStartingEventHandlerBox::<F>::VTABLE,
            count: ::windows_core::imp::RefCount::new(1),
            invoke,
        };
        unsafe { ::core::mem::transmute(::std::boxed::Box::new(com)) }
    }
    pub fn Invoke<P0, P1>(&self, sender: P0, e: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
        P1: ::windows_core::IntoParam<ManipulationStartingRoutedEventArgs>,
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
struct ManipulationStartingEventHandlerBox<
    F: FnMut(
            ::core::option::Option<&::windows_core::IInspectable>,
            ::core::option::Option<&ManipulationStartingRoutedEventArgs>,
        ) -> ::windows_core::Result<()>
        + ::core::marker::Send
        + 'static,
> {
    vtable: *const ManipulationStartingEventHandler_Vtbl,
    invoke: F,
    count: ::windows_core::imp::RefCount,
}
impl<
        F: FnMut(
                ::core::option::Option<&::windows_core::IInspectable>,
                ::core::option::Option<&ManipulationStartingRoutedEventArgs>,
            ) -> ::windows_core::Result<()>
            + ::core::marker::Send
            + 'static,
    > ManipulationStartingEventHandlerBox<F>
{
    const VTABLE: ManipulationStartingEventHandler_Vtbl = ManipulationStartingEventHandler_Vtbl {
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
        *interface = if *iid
            == <ManipulationStartingEventHandler as ::windows_core::ComInterface>::IID
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
unsafe impl ::windows_core::Interface for ManipulationStartingEventHandler {
    type Vtable = ManipulationStartingEventHandler_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ManipulationStartingEventHandler {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x44f528f1_f0e4_505c_a0bb_0c4839b29df5);
}
impl ::windows_core::RuntimeType for ManipulationStartingEventHandler {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ManipulationStartingEventHandler_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        sender: *mut ::core::ffi::c_void,
        e: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct PointerEventHandler(pub ::windows_core::IUnknown);
impl PointerEventHandler {
    pub fn new<
        F: FnMut(
                ::core::option::Option<&::windows_core::IInspectable>,
                ::core::option::Option<&PointerRoutedEventArgs>,
            ) -> ::windows_core::Result<()>
            + ::core::marker::Send
            + 'static,
    >(
        invoke: F,
    ) -> Self {
        let com = PointerEventHandlerBox::<F> {
            vtable: &PointerEventHandlerBox::<F>::VTABLE,
            count: ::windows_core::imp::RefCount::new(1),
            invoke,
        };
        unsafe { ::core::mem::transmute(::std::boxed::Box::new(com)) }
    }
    pub fn Invoke<P0, P1>(&self, sender: P0, e: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
        P1: ::windows_core::IntoParam<PointerRoutedEventArgs>,
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
struct PointerEventHandlerBox<
    F: FnMut(
            ::core::option::Option<&::windows_core::IInspectable>,
            ::core::option::Option<&PointerRoutedEventArgs>,
        ) -> ::windows_core::Result<()>
        + ::core::marker::Send
        + 'static,
> {
    vtable: *const PointerEventHandler_Vtbl,
    invoke: F,
    count: ::windows_core::imp::RefCount,
}
impl<
        F: FnMut(
                ::core::option::Option<&::windows_core::IInspectable>,
                ::core::option::Option<&PointerRoutedEventArgs>,
            ) -> ::windows_core::Result<()>
            + ::core::marker::Send
            + 'static,
    > PointerEventHandlerBox<F>
{
    const VTABLE: PointerEventHandler_Vtbl = PointerEventHandler_Vtbl {
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
        *interface = if *iid == <PointerEventHandler as ::windows_core::ComInterface>::IID
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
unsafe impl ::windows_core::Interface for PointerEventHandler {
    type Vtable = PointerEventHandler_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PointerEventHandler {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xa48a71e1_8bb4_5597_9e31_903a3f6a04fb);
}
impl ::windows_core::RuntimeType for PointerEventHandler {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct PointerEventHandler_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        sender: *mut ::core::ffi::c_void,
        e: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct RightTappedEventHandler(pub ::windows_core::IUnknown);
impl RightTappedEventHandler {
    pub fn new<
        F: FnMut(
                ::core::option::Option<&::windows_core::IInspectable>,
                ::core::option::Option<&RightTappedRoutedEventArgs>,
            ) -> ::windows_core::Result<()>
            + ::core::marker::Send
            + 'static,
    >(
        invoke: F,
    ) -> Self {
        let com = RightTappedEventHandlerBox::<F> {
            vtable: &RightTappedEventHandlerBox::<F>::VTABLE,
            count: ::windows_core::imp::RefCount::new(1),
            invoke,
        };
        unsafe { ::core::mem::transmute(::std::boxed::Box::new(com)) }
    }
    pub fn Invoke<P0, P1>(&self, sender: P0, e: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
        P1: ::windows_core::IntoParam<RightTappedRoutedEventArgs>,
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
struct RightTappedEventHandlerBox<
    F: FnMut(
            ::core::option::Option<&::windows_core::IInspectable>,
            ::core::option::Option<&RightTappedRoutedEventArgs>,
        ) -> ::windows_core::Result<()>
        + ::core::marker::Send
        + 'static,
> {
    vtable: *const RightTappedEventHandler_Vtbl,
    invoke: F,
    count: ::windows_core::imp::RefCount,
}
impl<
        F: FnMut(
                ::core::option::Option<&::windows_core::IInspectable>,
                ::core::option::Option<&RightTappedRoutedEventArgs>,
            ) -> ::windows_core::Result<()>
            + ::core::marker::Send
            + 'static,
    > RightTappedEventHandlerBox<F>
{
    const VTABLE: RightTappedEventHandler_Vtbl = RightTappedEventHandler_Vtbl {
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
        *interface = if *iid == <RightTappedEventHandler as ::windows_core::ComInterface>::IID
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
unsafe impl ::windows_core::Interface for RightTappedEventHandler {
    type Vtable = RightTappedEventHandler_Vtbl;
}
unsafe impl ::windows_core::ComInterface for RightTappedEventHandler {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x5070e32f_3dc7_56cf_8fdd_de1b40d0b472);
}
impl ::windows_core::RuntimeType for RightTappedEventHandler {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct RightTappedEventHandler_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        sender: *mut ::core::ffi::c_void,
        e: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct TappedEventHandler(pub ::windows_core::IUnknown);
impl TappedEventHandler {
    pub fn new<
        F: FnMut(
                ::core::option::Option<&::windows_core::IInspectable>,
                ::core::option::Option<&TappedRoutedEventArgs>,
            ) -> ::windows_core::Result<()>
            + ::core::marker::Send
            + 'static,
    >(
        invoke: F,
    ) -> Self {
        let com = TappedEventHandlerBox::<F> {
            vtable: &TappedEventHandlerBox::<F>::VTABLE,
            count: ::windows_core::imp::RefCount::new(1),
            invoke,
        };
        unsafe { ::core::mem::transmute(::std::boxed::Box::new(com)) }
    }
    pub fn Invoke<P0, P1>(&self, sender: P0, e: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
        P1: ::windows_core::IntoParam<TappedRoutedEventArgs>,
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
struct TappedEventHandlerBox<
    F: FnMut(
            ::core::option::Option<&::windows_core::IInspectable>,
            ::core::option::Option<&TappedRoutedEventArgs>,
        ) -> ::windows_core::Result<()>
        + ::core::marker::Send
        + 'static,
> {
    vtable: *const TappedEventHandler_Vtbl,
    invoke: F,
    count: ::windows_core::imp::RefCount,
}
impl<
        F: FnMut(
                ::core::option::Option<&::windows_core::IInspectable>,
                ::core::option::Option<&TappedRoutedEventArgs>,
            ) -> ::windows_core::Result<()>
            + ::core::marker::Send
            + 'static,
    > TappedEventHandlerBox<F>
{
    const VTABLE: TappedEventHandler_Vtbl = TappedEventHandler_Vtbl {
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
        *interface = if *iid == <TappedEventHandler as ::windows_core::ComInterface>::IID
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
unsafe impl ::windows_core::Interface for TappedEventHandler {
    type Vtable = TappedEventHandler_Vtbl;
}
unsafe impl ::windows_core::ComInterface for TappedEventHandler {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xb60074f3_125b_534e_8f9c_9769bd3f0f64);
}
impl ::windows_core::RuntimeType for TappedEventHandler {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct TappedEventHandler_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        sender: *mut ::core::ffi::c_void,
        e: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
