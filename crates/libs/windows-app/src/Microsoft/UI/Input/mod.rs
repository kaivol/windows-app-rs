#[cfg(feature = "Microsoft_UI_Input_DragDrop")]
#[doc = "Required features: `\"Microsoft_UI_Input_DragDrop\"`"]
pub mod DragDrop;
#[cfg(feature = "Microsoft_UI_Input_Interop")]
#[doc = "Required features: `\"Microsoft_UI_Input_Interop\"`"]
pub mod Interop;
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICharacterReceivedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICharacterReceivedEventArgs {
    type Vtable = ICharacterReceivedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICharacterReceivedEventArgs {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x36122718_9263_592b_8d87_8f86543ffc95);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICharacterReceivedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Handled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
    pub KeyCode: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut u32,
    ) -> ::windows_core::HRESULT,
    pub KeyStatus: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut PhysicalKeyStatus,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IContextMenuKeyEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IContextMenuKeyEventArgs {
    type Vtable = IContextMenuKeyEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IContextMenuKeyEventArgs {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xf6025762_9426_541a_b647_037abdbecefc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContextMenuKeyEventArgs_Vtbl {
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
pub struct ICrossSlidingEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICrossSlidingEventArgs {
    type Vtable = ICrossSlidingEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICrossSlidingEventArgs {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x7679641f_ba9f_543c_a7c8_6229a98f89ef);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICrossSlidingEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CrossSlidingState: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut CrossSlidingState,
    ) -> ::windows_core::HRESULT,
    pub PointerDeviceType: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut PointerDeviceType,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_Foundation")]
    pub Position: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::Point,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    Position: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDraggingEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDraggingEventArgs {
    type Vtable = IDraggingEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDraggingEventArgs {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x3efb1b75_3d3b_550e_963d_0828ca76128a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDraggingEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub DraggingState: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut DraggingState,
    ) -> ::windows_core::HRESULT,
    pub PointerDeviceType: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut PointerDeviceType,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_Foundation")]
    pub Position: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::Point,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    Position: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IFocusChangedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFocusChangedEventArgs {
    type Vtable = IFocusChangedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IFocusChangedEventArgs {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xa039b115_dbdf_594c_9b86_da6aa05c9fa2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFocusChangedEventArgs_Vtbl {
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
pub struct IGestureRecognizer(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGestureRecognizer {
    type Vtable = IGestureRecognizer_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IGestureRecognizer {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xcda89afc_6bd0_595c_ba37_545fce5bf016);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGestureRecognizer_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub AutoProcessInertia: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetAutoProcessInertia: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
    pub CrossSlideExact: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetCrossSlideExact: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
    pub CrossSlideHorizontally: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetCrossSlideHorizontally: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
    pub CrossSlideThresholds: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut CrossSlideThresholds,
    ) -> ::windows_core::HRESULT,
    pub SetCrossSlideThresholds: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: CrossSlideThresholds,
    ) -> ::windows_core::HRESULT,
    pub GestureSettings: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut GestureSettings,
    ) -> ::windows_core::HRESULT,
    pub SetGestureSettings: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: GestureSettings,
    ) -> ::windows_core::HRESULT,
    pub IsActive: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub IsInertial: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_Foundation")]
    pub PivotCenter: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::Point,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    PivotCenter: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub SetPivotCenter: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::windows::Foundation::Point,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    SetPivotCenter: usize,
    pub PivotRadius: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f32,
    ) -> ::windows_core::HRESULT,
    pub SetPivotRadius: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f32,
    ) -> ::windows_core::HRESULT,
    pub InertiaExpansionDeceleration: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f32,
    ) -> ::windows_core::HRESULT,
    pub SetInertiaExpansionDeceleration: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f32,
    ) -> ::windows_core::HRESULT,
    pub InertiaExpansion: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f32,
    ) -> ::windows_core::HRESULT,
    pub SetInertiaExpansion: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f32,
    ) -> ::windows_core::HRESULT,
    pub InertiaRotationAngle: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f32,
    ) -> ::windows_core::HRESULT,
    pub SetInertiaRotationAngle: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f32,
    ) -> ::windows_core::HRESULT,
    pub InertiaRotationDeceleration: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f32,
    ) -> ::windows_core::HRESULT,
    pub SetInertiaRotationDeceleration: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f32,
    ) -> ::windows_core::HRESULT,
    pub InertiaTranslationDeceleration: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f32,
    ) -> ::windows_core::HRESULT,
    pub SetInertiaTranslationDeceleration: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f32,
    )
        -> ::windows_core::HRESULT,
    pub InertiaTranslationDisplacement: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f32,
    ) -> ::windows_core::HRESULT,
    pub SetInertiaTranslationDisplacement: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f32,
    )
        -> ::windows_core::HRESULT,
    pub ManipulationExact: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetManipulationExact: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
    pub MouseWheelParameters: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub ShowGestureFeedback: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetShowGestureFeedback: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
    pub CanBeDoubleTap: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub CompleteGesture:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub ProcessDownEvent: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub ProcessMoveEvents: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation_Collections"))]
    ProcessMoveEvents: usize,
    pub ProcessMouseWheelEvent: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
        isshiftkeydown: bool,
        iscontrolkeydown: bool,
    ) -> ::windows_core::HRESULT,
    pub ProcessInertia:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub ProcessUpEvent: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_Foundation")]
    pub Tapped: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    Tapped: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub RemoveTapped: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RemoveTapped: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub RightTapped: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RightTapped: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub RemoveRightTapped: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RemoveRightTapped: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub Holding: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    Holding: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub RemoveHolding: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RemoveHolding: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub Dragging: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    Dragging: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub RemoveDragging: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RemoveDragging: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub ManipulationStarted: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    ManipulationStarted: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub RemoveManipulationStarted: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RemoveManipulationStarted: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub ManipulationUpdated: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    ManipulationUpdated: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub RemoveManipulationUpdated: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RemoveManipulationUpdated: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub ManipulationInertiaStarting: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    ManipulationInertiaStarting: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub RemoveManipulationInertiaStarting: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    )
        -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RemoveManipulationInertiaStarting: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub ManipulationCompleted: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    ManipulationCompleted: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub RemoveManipulationCompleted: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RemoveManipulationCompleted: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub CrossSliding: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    CrossSliding: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub RemoveCrossSliding: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RemoveCrossSliding: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IHoldingEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHoldingEventArgs {
    type Vtable = IHoldingEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IHoldingEventArgs {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x8e449e85_d223_533c_b0b2_bf7c6d10c2db);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHoldingEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub HoldingState: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut HoldingState,
    ) -> ::windows_core::HRESULT,
    pub PointerDeviceType: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut PointerDeviceType,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_Foundation")]
    pub Position: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::Point,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    Position: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IInputActivationListener(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IInputActivationListener {
    type Vtable = IInputActivationListener_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IInputActivationListener {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x3b818627_6ce7_5e0d_a0f5_6684fd1aec78);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputActivationListener_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub State: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut InputActivationState,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_Foundation")]
    pub InputActivationChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    InputActivationChanged: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub RemoveInputActivationChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RemoveInputActivationChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IInputActivationListenerActivationChangedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IInputActivationListenerActivationChangedEventArgs {
    type Vtable = IInputActivationListenerActivationChangedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IInputActivationListenerActivationChangedEventArgs {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x7978526b_00b6_5303_8f7d_55bef36da786);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputActivationListenerActivationChangedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IInputActivationListenerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IInputActivationListenerStatics {
    type Vtable = IInputActivationListenerStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IInputActivationListenerStatics {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xc4249843_f053_5c99_9d51_720ade94224d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputActivationListenerStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub GetForWindowId: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        windowid: super::WindowId,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IInputActivationListenerStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IInputActivationListenerStatics2 {
    type Vtable = IInputActivationListenerStatics2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IInputActivationListenerStatics2 {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x7ea26120_9636_5292_a7b1_56544ac51a22);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputActivationListenerStatics2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Microsoft_UI_Content")]
    pub GetForIsland: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        island: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Microsoft_UI_Content"))]
    GetForIsland: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IInputCursor(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IInputCursor {
    type Vtable = IInputCursor_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IInputCursor {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x359b15f9_19c2_5714_8432_75176826406b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputCursor_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IInputCursorFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IInputCursorFactory {
    type Vtable = IInputCursorFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IInputCursorFactory {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x2f47647b_4be0_53e9_be7e_c38d5459db6b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputCursorFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IInputCursorStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IInputCursorStatics {
    type Vtable = IInputCursorStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IInputCursorStatics {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x92f6a552_099f_55fb_8c31_e450284c9643);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputCursorStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Windows_UI_Core")]
    pub CreateFromCoreCursor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        cursor: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI_Core"))]
    CreateFromCoreCursor: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IInputCustomCursor(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IInputCustomCursor {
    type Vtable = IInputCustomCursor_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IInputCustomCursor {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x5486f042_7e1a_5dc8_8041_e47b609a5ba1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputCustomCursor_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IInputCustomCursorFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IInputCustomCursorFactory {
    type Vtable = IInputCustomCursorFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IInputCustomCursorFactory {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x6f402882_66e0_57d3_89d0_aa5e2ff917bc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputCustomCursorFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IInputDesktopNamedResourceCursor(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IInputDesktopNamedResourceCursor {
    type Vtable = IInputDesktopNamedResourceCursor_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IInputDesktopNamedResourceCursor {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xf40ea93b_0ed7_5b3a_bfe2_14e2b5ad88a3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputDesktopNamedResourceCursor_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ModuleName: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub ResourceName: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IInputDesktopNamedResourceCursorStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IInputDesktopNamedResourceCursorStatics {
    type Vtable = IInputDesktopNamedResourceCursorStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IInputDesktopNamedResourceCursorStatics {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xe8b6d5aa_898b_5e69_b01f_383a0943e3e4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputDesktopNamedResourceCursorStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        resourcename: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub CreateFromModule: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        modulename: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        resourcename: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IInputDesktopResourceCursor(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IInputDesktopResourceCursor {
    type Vtable = IInputDesktopResourceCursor_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IInputDesktopResourceCursor {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x1df2777f_7c90_58fc_a7a3_d5736c6510fd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputDesktopResourceCursor_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ModuleName: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub ResourceId: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut u32,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IInputDesktopResourceCursorStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IInputDesktopResourceCursorStatics {
    type Vtable = IInputDesktopResourceCursorStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IInputDesktopResourceCursorStatics {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xf440dc37_a0b6_56eb_bcec_b024f2233d47);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputDesktopResourceCursorStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        resourceid: u32,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub CreateFromModule: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        modulename: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        resourceid: u32,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IInputFocusChangedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IInputFocusChangedEventArgs {
    type Vtable = IInputFocusChangedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IInputFocusChangedEventArgs {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xd85b1b7a_045d_5a1b_9966_ebc0b3d47567);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputFocusChangedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IInputFocusController(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IInputFocusController {
    type Vtable = IInputFocusController_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IInputFocusController {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x8dfdc26c_8b8d_515d_8ddd_4685b3a540e9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputFocusController_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub HasFocus: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub TrySetFocus: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
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
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IInputFocusControllerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IInputFocusControllerStatics {
    type Vtable = IInputFocusControllerStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IInputFocusControllerStatics {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xaeb311da_da9b_5a1b_92f4_83ddde933e00);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputFocusControllerStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Microsoft_UI_Content")]
    pub GetForIsland: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        island: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Microsoft_UI_Content"))]
    GetForIsland: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IInputKeyboardSource(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IInputKeyboardSource {
    type Vtable = IInputKeyboardSource_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IInputKeyboardSource {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xed61b906_16ad_5df7_a550_5e6f7d2229f7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputKeyboardSource_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IInputKeyboardSource2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IInputKeyboardSource2 {
    type Vtable = IInputKeyboardSource2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IInputKeyboardSource2 {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x79d1c9b6_b3c9_5ec2_8a5b_707088787f78);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputKeyboardSource2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Windows_System")]
    pub GetCurrentKeyState: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        virtualkey: ::windows::System::VirtualKey,
        result__: *mut VirtualKeyStates,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_System"))]
    GetCurrentKeyState: usize,
    #[cfg(feature = "Windows_System")]
    pub GetKeyState: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        virtualkey: ::windows::System::VirtualKey,
        result__: *mut VirtualKeyStates,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_System"))]
    GetKeyState: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub CharacterReceived: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    CharacterReceived: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub RemoveCharacterReceived: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RemoveCharacterReceived: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub ContextMenuKey: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    ContextMenuKey: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub RemoveContextMenuKey: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RemoveContextMenuKey: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub KeyDown: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    KeyDown: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub RemoveKeyDown: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RemoveKeyDown: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub KeyUp: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    KeyUp: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub RemoveKeyUp: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RemoveKeyUp: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub SystemKeyDown: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    SystemKeyDown: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub RemoveSystemKeyDown: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RemoveSystemKeyDown: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub SystemKeyUp: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    SystemKeyUp: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub RemoveSystemKeyUp: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RemoveSystemKeyUp: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IInputKeyboardSourceStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IInputKeyboardSourceStatics {
    type Vtable = IInputKeyboardSourceStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IInputKeyboardSourceStatics {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xf4e1563d_8c2e_5bcd_b784_47adeaa3cd7e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputKeyboardSourceStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Windows_System", feature = "Windows_UI_Core"))]
    pub GetKeyStateForCurrentThread: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        virtualkey: ::windows::System::VirtualKey,
        result__: *mut ::windows::UI::Core::CoreVirtualKeyStates,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Windows_System", feature = "Windows_UI_Core")))]
    GetKeyStateForCurrentThread: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IInputKeyboardSourceStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IInputKeyboardSourceStatics2 {
    type Vtable = IInputKeyboardSourceStatics2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IInputKeyboardSourceStatics2 {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x8857518c_2899_5f11_9b64_0ad83234824b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputKeyboardSourceStatics2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Microsoft_UI_Content")]
    pub GetForIsland: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        island: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Microsoft_UI_Content"))]
    GetForIsland: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IInputLightDismissAction(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IInputLightDismissAction {
    type Vtable = IInputLightDismissAction_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IInputLightDismissAction {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xe8a39502_a860_502f_8c10_3646d43aecf1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputLightDismissAction_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Windows_Foundation")]
    pub Dismissed: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    Dismissed: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub RemoveDismissed: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RemoveDismissed: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IInputLightDismissActionStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IInputLightDismissActionStatics {
    type Vtable = IInputLightDismissActionStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IInputLightDismissActionStatics {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xed9b8def_6496_5169_984d_d44b4e690623);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputLightDismissActionStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub GetForWindowId: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        windowid: super::WindowId,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IInputLightDismissEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IInputLightDismissEventArgs {
    type Vtable = IInputLightDismissEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IInputLightDismissEventArgs {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x078660ee_07ca_5808_b982_e6e899cf098c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputLightDismissEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IInputNonClientPointerSource(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IInputNonClientPointerSource {
    type Vtable = IInputNonClientPointerSource_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IInputNonClientPointerSource {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x471732b4_3d07_5104_b192_ebacf71e86df);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputNonClientPointerSource_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Microsoft_UI_Dispatching")]
    pub DispatcherQueue: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Microsoft_UI_Dispatching"))]
    DispatcherQueue: usize,
    pub ClearAllRegionRects:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub ClearRegionRects: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        region: NonClientRegionKind,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_Graphics")]
    pub GetRegionRects: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        region: NonClientRegionKind,
        result_size__: *mut u32,
        result__: *mut *mut ::windows::Graphics::RectInt32,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Graphics"))]
    GetRegionRects: usize,
    #[cfg(feature = "Windows_Graphics")]
    pub SetRegionRects: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        region: NonClientRegionKind,
        rects_array_size: u32,
        rects: *const ::windows::Graphics::RectInt32,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Graphics"))]
    SetRegionRects: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub CaptionTapped: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    CaptionTapped: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub RemoveCaptionTapped: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RemoveCaptionTapped: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub PointerEntered: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    PointerEntered: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub RemovePointerEntered: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RemovePointerEntered: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub PointerExited: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    PointerExited: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub RemovePointerExited: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RemovePointerExited: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub PointerMoved: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    PointerMoved: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub RemovePointerMoved: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RemovePointerMoved: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub PointerPressed: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    PointerPressed: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub RemovePointerPressed: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RemovePointerPressed: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub PointerReleased: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    PointerReleased: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub RemovePointerReleased: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RemovePointerReleased: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub RegionsChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RegionsChanged: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub RemoveRegionsChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RemoveRegionsChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IInputNonClientPointerSourceStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IInputNonClientPointerSourceStatics {
    type Vtable = IInputNonClientPointerSourceStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IInputNonClientPointerSourceStatics {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x7d0b775c_1903_5dc7_bd2f_7a4b31f0cff2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputNonClientPointerSourceStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub GetForWindowId: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        windowid: super::WindowId,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IInputObject(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IInputObject {
    type Vtable = IInputObject_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IInputObject {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x42edbc88_d386_544d_b1b8_68617fe68282);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputObject_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
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
pub struct IInputObjectFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IInputObjectFactory {
    type Vtable = IInputObjectFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IInputObjectFactory {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xf7786bc2_b0b8_5961_9a57_ae199d452106);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputObjectFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IInputPointerSource(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IInputPointerSource {
    type Vtable = IInputPointerSource_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IInputPointerSource {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x6a6c2764_c3f4_5be5_8447_c9a98766c240);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputPointerSource_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Cursor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetCursor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub DeviceKinds: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut InputPointerSourceDeviceKinds,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_Foundation")]
    pub PointerCaptureLost: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    PointerCaptureLost: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub RemovePointerCaptureLost: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RemovePointerCaptureLost: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub PointerEntered: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    PointerEntered: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub RemovePointerEntered: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RemovePointerEntered: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub PointerExited: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    PointerExited: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub RemovePointerExited: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RemovePointerExited: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub PointerMoved: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    PointerMoved: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub RemovePointerMoved: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RemovePointerMoved: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub PointerPressed: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    PointerPressed: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub RemovePointerPressed: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RemovePointerPressed: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub PointerReleased: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    PointerReleased: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub RemovePointerReleased: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RemovePointerReleased: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub PointerRoutedAway: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    PointerRoutedAway: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub RemovePointerRoutedAway: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RemovePointerRoutedAway: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub PointerRoutedReleased: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    PointerRoutedReleased: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub RemovePointerRoutedReleased: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RemovePointerRoutedReleased: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub PointerRoutedTo: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    PointerRoutedTo: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub RemovePointerRoutedTo: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RemovePointerRoutedTo: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub PointerWheelChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    PointerWheelChanged: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub RemovePointerWheelChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RemovePointerWheelChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IInputPointerSourceStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IInputPointerSourceStatics {
    type Vtable = IInputPointerSourceStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IInputPointerSourceStatics {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xe8a19fd1_a914_533f_9b0f_6bf0065e6781);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputPointerSourceStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Microsoft_UI_Content")]
    pub GetForIsland: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        island: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Microsoft_UI_Content"))]
    GetForIsland: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IInputPreTranslateKeyboardSource(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IInputPreTranslateKeyboardSource {
    type Vtable = IInputPreTranslateKeyboardSource_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IInputPreTranslateKeyboardSource {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x2f327feb_b7e7_5e37_a0cc_37dcabe76588);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputPreTranslateKeyboardSource_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IInputPreTranslateKeyboardSourceStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IInputPreTranslateKeyboardSourceStatics {
    type Vtable = IInputPreTranslateKeyboardSourceStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IInputPreTranslateKeyboardSourceStatics {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x23d584d2_af8c_5a8a_806f_2ba9c5b1a5ec);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputPreTranslateKeyboardSourceStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Microsoft_UI_Content")]
    pub GetForIsland: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        island: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Microsoft_UI_Content"))]
    GetForIsland: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IInputSystemCursor(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IInputSystemCursor {
    type Vtable = IInputSystemCursor_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IInputSystemCursor {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x59f538e7_c500_59ab_8b54_0bc6100fd49e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputSystemCursor_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CursorShape: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut InputSystemCursorShape,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IInputSystemCursorStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IInputSystemCursorStatics {
    type Vtable = IInputSystemCursorStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IInputSystemCursorStatics {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xd3860bb6_698a_5814_aedd_c2fa8bba5a02);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputSystemCursorStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        r#type: InputSystemCursorShape,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IKeyEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IKeyEventArgs {
    type Vtable = IKeyEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IKeyEventArgs {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x40d5bb74_977e_5194_8039_9f6c44427bbb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKeyEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Handled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
    pub KeyStatus: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut PhysicalKeyStatus,
    ) -> ::windows_core::HRESULT,
    pub Timestamp: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut u64,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_System")]
    pub VirtualKey: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::System::VirtualKey,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_System"))]
    VirtualKey: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IManipulationCompletedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IManipulationCompletedEventArgs {
    type Vtable = IManipulationCompletedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IManipulationCompletedEventArgs {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x0e0249d4_46e4_5559_aee3_fa45ce2a7f56);
}
#[repr(C)]
#[doc(hidden)]
pub struct IManipulationCompletedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Windows_Foundation")]
    pub Cumulative: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ManipulationDelta,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    Cumulative: usize,
    pub PointerDeviceType: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut PointerDeviceType,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_Foundation")]
    pub Position: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::Point,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    Position: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub Velocities: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ManipulationVelocities,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    Velocities: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IManipulationInertiaStartingEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IManipulationInertiaStartingEventArgs {
    type Vtable = IManipulationInertiaStartingEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IManipulationInertiaStartingEventArgs {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xacf9ef71_6e15_56ab_9260_f0d3ce5f66e8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IManipulationInertiaStartingEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Windows_Foundation")]
    pub Cumulative: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ManipulationDelta,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    Cumulative: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub Delta: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ManipulationDelta,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    Delta: usize,
    pub PointerDeviceType: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut PointerDeviceType,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_Foundation")]
    pub Position: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::Point,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    Position: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub Velocities: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ManipulationVelocities,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    Velocities: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IManipulationStartedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IManipulationStartedEventArgs {
    type Vtable = IManipulationStartedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IManipulationStartedEventArgs {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x4a616613_eef1_5f1b_a768_0775478d49d4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IManipulationStartedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Windows_Foundation")]
    pub Cumulative: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ManipulationDelta,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    Cumulative: usize,
    pub PointerDeviceType: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut PointerDeviceType,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_Foundation")]
    pub Position: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::Point,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    Position: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IManipulationUpdatedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IManipulationUpdatedEventArgs {
    type Vtable = IManipulationUpdatedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IManipulationUpdatedEventArgs {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x406e1961_0c98_5fc0_b3d8_116492ef0053);
}
#[repr(C)]
#[doc(hidden)]
pub struct IManipulationUpdatedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Windows_Foundation")]
    pub Cumulative: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ManipulationDelta,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    Cumulative: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub Delta: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ManipulationDelta,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    Delta: usize,
    pub PointerDeviceType: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut PointerDeviceType,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_Foundation")]
    pub Position: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::Point,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    Position: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub Velocities: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ManipulationVelocities,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    Velocities: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMouseWheelParameters(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMouseWheelParameters {
    type Vtable = IMouseWheelParameters_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMouseWheelParameters {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x6d98be40_1d56_51d1_aa0d_f325439cd009);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMouseWheelParameters_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Windows_Foundation")]
    pub CharTranslation: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::Point,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    CharTranslation: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub SetCharTranslation: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::windows::Foundation::Point,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    SetCharTranslation: usize,
    pub DeltaScale: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f32,
    ) -> ::windows_core::HRESULT,
    pub SetDeltaScale: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f32,
    ) -> ::windows_core::HRESULT,
    pub DeltaRotationAngle: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f32,
    ) -> ::windows_core::HRESULT,
    pub SetDeltaRotationAngle: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f32,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_Foundation")]
    pub PageTranslation: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::Point,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    PageTranslation: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub SetPageTranslation: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::windows::Foundation::Point,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    SetPageTranslation: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct INonClientCaptionTappedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for INonClientCaptionTappedEventArgs {
    type Vtable = INonClientCaptionTappedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for INonClientCaptionTappedEventArgs {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x3d173531_991f_5753_b7e0_14a121c3cd2d);
}
#[repr(C)]
#[doc(hidden)]
pub struct INonClientCaptionTappedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Windows_Foundation")]
    pub Point: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::Point,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    Point: usize,
    pub PointerDeviceType: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut PointerDeviceType,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct INonClientPointerEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for INonClientPointerEventArgs {
    type Vtable = INonClientPointerEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for INonClientPointerEventArgs {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xa5b44aec_b797_505a_a129_ae4e5271c73c);
}
#[repr(C)]
#[doc(hidden)]
pub struct INonClientPointerEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Windows_Foundation")]
    pub Point: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::Point,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    Point: usize,
    pub PointerDeviceType: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut PointerDeviceType,
    ) -> ::windows_core::HRESULT,
    pub RegionKind: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut NonClientRegionKind,
    ) -> ::windows_core::HRESULT,
    pub IsPointInRegion: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct INonClientRegionsChangedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for INonClientRegionsChangedEventArgs {
    type Vtable = INonClientRegionsChangedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for INonClientRegionsChangedEventArgs {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xfe97ee95_1824_51b2_b8eb_10ff0665ce23);
}
#[repr(C)]
#[doc(hidden)]
pub struct INonClientRegionsChangedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ChangedRegions: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result_size__: *mut u32,
        result__: *mut *mut NonClientRegionKind,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPointerEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPointerEventArgs {
    type Vtable = IPointerEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPointerEventArgs {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x865b188c_2ed5_5df8_829f_ac0701d5c51a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPointerEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CurrentPoint: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub Handled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_System")]
    pub KeyModifiers: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::System::VirtualKeyModifiers,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_System"))]
    KeyModifiers: usize,
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub GetIntermediatePoints: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation_Collections"))]
    GetIntermediatePoints: usize,
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub GetIntermediateTransformedPoints: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        transform: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation_Collections"))]
    GetIntermediateTransformedPoints: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPointerPoint(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPointerPoint {
    type Vtable = IPointerPoint_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPointerPoint {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x0d430ee6_252c_59a4_b2a2_d44264dc6a40);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPointerPoint_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub FrameId: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut u32,
    ) -> ::windows_core::HRESULT,
    pub IsInContact: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub PointerDeviceType: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut PointerDeviceType,
    ) -> ::windows_core::HRESULT,
    pub PointerId: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut u32,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_Foundation")]
    pub Position: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::Point,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    Position: usize,
    pub Properties: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub Timestamp: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut u64,
    ) -> ::windows_core::HRESULT,
    pub GetTransformedPoint: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        transform: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPointerPointProperties(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPointerPointProperties {
    type Vtable = IPointerPointProperties_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPointerPointProperties {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xd760ed77_4b10_57a5_b3cc_d9bf3413e996);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPointerPointProperties_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Windows_Foundation")]
    pub ContactRect: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::Rect,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    ContactRect: usize,
    pub IsBarrelButtonPressed: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub IsCanceled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub IsEraser: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub IsHorizontalMouseWheel: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub IsInRange: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub IsInverted: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub IsLeftButtonPressed: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub IsMiddleButtonPressed: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub IsPrimary: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub IsRightButtonPressed: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub IsXButton1Pressed: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub IsXButton2Pressed: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub MouseWheelDelta: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut i32,
    ) -> ::windows_core::HRESULT,
    pub Orientation: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f32,
    ) -> ::windows_core::HRESULT,
    pub PointerUpdateKind: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut PointerUpdateKind,
    ) -> ::windows_core::HRESULT,
    pub Pressure: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f32,
    ) -> ::windows_core::HRESULT,
    pub TouchConfidence: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub Twist: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f32,
    ) -> ::windows_core::HRESULT,
    pub XTilt: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f32,
    ) -> ::windows_core::HRESULT,
    pub YTilt: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f32,
    ) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPointerPointTransform(::windows_core::IUnknown);
impl IPointerPointTransform {
    pub fn Inverse(&self) -> ::windows_core::Result<IPointerPointTransform> {
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
    pub fn TryTransformBounds(
        &self,
        inrect: ::windows::Foundation::Rect,
        outrect: &mut ::windows::Foundation::Rect,
    ) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TryTransformBounds)(
                ::windows_core::Interface::as_raw(this),
                inrect,
                outrect,
                &mut result__,
            )
            .from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(
    IPointerPointTransform,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::RuntimeType for IPointerPointTransform {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_interface::<Self>();
}
unsafe impl ::windows_core::Interface for IPointerPointTransform {
    type Vtable = IPointerPointTransform_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPointerPointTransform {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xdb4791bc_994d_54c7_92ef_66ea1de9b43c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPointerPointTransform_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Inverse: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
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
    pub TryTransformBounds: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        inrect: ::windows::Foundation::Rect,
        outrect: *mut ::windows::Foundation::Rect,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    TryTransformBounds: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPointerPredictor(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPointerPredictor {
    type Vtable = IPointerPredictor_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPointerPredictor {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x12c100ec_2100_565f_a60c_f1187f438828);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPointerPredictor_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Windows_Foundation")]
    pub PredictionTime: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::TimeSpan,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    PredictionTime: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub SetPredictionTime: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::windows::Foundation::TimeSpan,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    SetPredictionTime: usize,
    pub GetPredictedPoints: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        point: *mut ::core::ffi::c_void,
        result_size__: *mut u32,
        result__: *mut *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPointerPredictorStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPointerPredictorStatics {
    type Vtable = IPointerPredictorStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPointerPredictorStatics {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x78a8ef30_3e5c_55cd_8f85_65ac09b1a987);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPointerPredictorStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateForInputPointerSource: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        inputpointersource: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IRightTappedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRightTappedEventArgs {
    type Vtable = IRightTappedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IRightTappedEventArgs {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x8ff73b39_887e_50a4_8500_77953039dcb4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRightTappedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub PointerDeviceType: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut PointerDeviceType,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_Foundation")]
    pub Position: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::Point,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    Position: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ITappedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITappedEventArgs {
    type Vtable = ITappedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ITappedEventArgs {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xc3a01bb5_6076_5e0f_871a_9d94a6a8f82b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITappedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub PointerDeviceType: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut PointerDeviceType,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_Foundation")]
    pub Position: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::Point,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    Position: usize,
    pub TapCount: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut u32,
    ) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct CharacterReceivedEventArgs(::windows_core::IUnknown);
impl CharacterReceivedEventArgs {
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
    pub fn KeyCode(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).KeyCode)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn KeyStatus(&self) -> ::windows_core::Result<PhysicalKeyStatus> {
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
}
impl ::windows_core::RuntimeType for CharacterReceivedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for CharacterReceivedEventArgs {
    type Vtable = ICharacterReceivedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for CharacterReceivedEventArgs {
    const IID: ::windows_core::GUID =
        <ICharacterReceivedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for CharacterReceivedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Input.CharacterReceivedEventArgs";
}
::windows_core::imp::interface_hierarchy!(
    CharacterReceivedEventArgs,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for CharacterReceivedEventArgs {}
unsafe impl ::core::marker::Sync for CharacterReceivedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ContextMenuKeyEventArgs(::windows_core::IUnknown);
impl ContextMenuKeyEventArgs {
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
impl ::windows_core::RuntimeType for ContextMenuKeyEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for ContextMenuKeyEventArgs {
    type Vtable = IContextMenuKeyEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ContextMenuKeyEventArgs {
    const IID: ::windows_core::GUID =
        <IContextMenuKeyEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ContextMenuKeyEventArgs {
    const NAME: &'static str = "Microsoft.UI.Input.ContextMenuKeyEventArgs";
}
::windows_core::imp::interface_hierarchy!(
    ContextMenuKeyEventArgs,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for ContextMenuKeyEventArgs {}
unsafe impl ::core::marker::Sync for ContextMenuKeyEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct CrossSlidingEventArgs(::windows_core::IUnknown);
impl CrossSlidingEventArgs {
    pub fn CrossSlidingState(&self) -> ::windows_core::Result<CrossSlidingState> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CrossSlidingState)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn PointerDeviceType(&self) -> ::windows_core::Result<PointerDeviceType> {
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
}
impl ::windows_core::RuntimeType for CrossSlidingEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for CrossSlidingEventArgs {
    type Vtable = ICrossSlidingEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for CrossSlidingEventArgs {
    const IID: ::windows_core::GUID = <ICrossSlidingEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for CrossSlidingEventArgs {
    const NAME: &'static str = "Microsoft.UI.Input.CrossSlidingEventArgs";
}
::windows_core::imp::interface_hierarchy!(
    CrossSlidingEventArgs,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for CrossSlidingEventArgs {}
unsafe impl ::core::marker::Sync for CrossSlidingEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct DraggingEventArgs(::windows_core::IUnknown);
impl DraggingEventArgs {
    pub fn DraggingState(&self) -> ::windows_core::Result<DraggingState> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DraggingState)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn PointerDeviceType(&self) -> ::windows_core::Result<PointerDeviceType> {
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
}
impl ::windows_core::RuntimeType for DraggingEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for DraggingEventArgs {
    type Vtable = IDraggingEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for DraggingEventArgs {
    const IID: ::windows_core::GUID = <IDraggingEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for DraggingEventArgs {
    const NAME: &'static str = "Microsoft.UI.Input.DraggingEventArgs";
}
::windows_core::imp::interface_hierarchy!(
    DraggingEventArgs,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for DraggingEventArgs {}
unsafe impl ::core::marker::Sync for DraggingEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct FocusChangedEventArgs(::windows_core::IUnknown);
impl FocusChangedEventArgs {
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
impl ::windows_core::RuntimeType for FocusChangedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for FocusChangedEventArgs {
    type Vtable = IFocusChangedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for FocusChangedEventArgs {
    const IID: ::windows_core::GUID = <IFocusChangedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for FocusChangedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Input.FocusChangedEventArgs";
}
::windows_core::imp::interface_hierarchy!(
    FocusChangedEventArgs,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for FocusChangedEventArgs {}
unsafe impl ::core::marker::Sync for FocusChangedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct GestureRecognizer(::windows_core::IUnknown);
impl GestureRecognizer {
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
            GestureRecognizer,
            ::windows_core::imp::IGenericFactory,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn AutoProcessInertia(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AutoProcessInertia)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetAutoProcessInertia(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetAutoProcessInertia)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn CrossSlideExact(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CrossSlideExact)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetCrossSlideExact(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetCrossSlideExact)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn CrossSlideHorizontally(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CrossSlideHorizontally)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetCrossSlideHorizontally(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetCrossSlideHorizontally)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn CrossSlideThresholds(&self) -> ::windows_core::Result<CrossSlideThresholds> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CrossSlideThresholds)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetCrossSlideThresholds(
        &self,
        value: CrossSlideThresholds,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetCrossSlideThresholds)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn GestureSettings(&self) -> ::windows_core::Result<GestureSettings> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GestureSettings)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetGestureSettings(&self, value: GestureSettings) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetGestureSettings)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsActive(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsActive)(
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
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn PivotCenter(&self) -> ::windows_core::Result<::windows::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PivotCenter)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn SetPivotCenter(
        &self,
        value: ::windows::Foundation::Point,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetPivotCenter)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn PivotRadius(&self) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PivotRadius)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetPivotRadius(&self, value: f32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetPivotRadius)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn InertiaExpansionDeceleration(&self) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).InertiaExpansionDeceleration)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetInertiaExpansionDeceleration(&self, value: f32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetInertiaExpansionDeceleration)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn InertiaExpansion(&self) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).InertiaExpansion)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetInertiaExpansion(&self, value: f32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetInertiaExpansion)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn InertiaRotationAngle(&self) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).InertiaRotationAngle)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetInertiaRotationAngle(&self, value: f32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetInertiaRotationAngle)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn InertiaRotationDeceleration(&self) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).InertiaRotationDeceleration)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetInertiaRotationDeceleration(&self, value: f32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetInertiaRotationDeceleration)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn InertiaTranslationDeceleration(&self) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).InertiaTranslationDeceleration)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetInertiaTranslationDeceleration(&self, value: f32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetInertiaTranslationDeceleration)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn InertiaTranslationDisplacement(&self) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).InertiaTranslationDisplacement)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetInertiaTranslationDisplacement(&self, value: f32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetInertiaTranslationDisplacement)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn ManipulationExact(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ManipulationExact)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetManipulationExact(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetManipulationExact)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn MouseWheelParameters(&self) -> ::windows_core::Result<MouseWheelParameters> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MouseWheelParameters)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn ShowGestureFeedback(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ShowGestureFeedback)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetShowGestureFeedback(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetShowGestureFeedback)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn CanBeDoubleTap<P0>(&self, value: P0) -> ::windows_core::Result<bool>
    where
        P0: ::windows_core::IntoParam<PointerPoint>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CanBeDoubleTap)(
                ::windows_core::Interface::as_raw(this),
                value.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn CompleteGesture(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).CompleteGesture)(
                ::windows_core::Interface::as_raw(this),
            )
            .ok()
        }
    }
    pub fn ProcessDownEvent<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<PointerPoint>,
    {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).ProcessDownEvent)(
                ::windows_core::Interface::as_raw(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation_Collections\"`"]
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub fn ProcessMoveEvents<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<::windows::Foundation::Collections::IVector<PointerPoint>>,
    {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).ProcessMoveEvents)(
                ::windows_core::Interface::as_raw(this),
                value.try_into_param()?.abi(),
            )
            .ok()
        }
    }
    pub fn ProcessMouseWheelEvent<P0>(
        &self,
        value: P0,
        isshiftkeydown: bool,
        iscontrolkeydown: bool,
    ) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<PointerPoint>,
    {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).ProcessMouseWheelEvent)(
                ::windows_core::Interface::as_raw(this),
                value.into_param().abi(),
                isshiftkeydown,
                iscontrolkeydown,
            )
            .ok()
        }
    }
    pub fn ProcessInertia(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).ProcessInertia)(
                ::windows_core::Interface::as_raw(this),
            )
            .ok()
        }
    }
    pub fn ProcessUpEvent<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<PointerPoint>,
    {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).ProcessUpEvent)(
                ::windows_core::Interface::as_raw(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn Tapped<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<GestureRecognizer, TappedEventArgs>,
        >,
    {
        let this = self;
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
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveTapped)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RightTapped<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<GestureRecognizer, RightTappedEventArgs>,
        >,
    {
        let this = self;
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
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveRightTapped)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn Holding<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<GestureRecognizer, HoldingEventArgs>,
        >,
    {
        let this = self;
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
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveHolding)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn Dragging<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<GestureRecognizer, DraggingEventArgs>,
        >,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Dragging)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveDragging(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveDragging)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn ManipulationStarted<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<
                GestureRecognizer,
                ManipulationStartedEventArgs,
            >,
        >,
    {
        let this = self;
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
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveManipulationStarted)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn ManipulationUpdated<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<
                GestureRecognizer,
                ManipulationUpdatedEventArgs,
            >,
        >,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ManipulationUpdated)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveManipulationUpdated(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveManipulationUpdated)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn ManipulationInertiaStarting<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<
                GestureRecognizer,
                ManipulationInertiaStartingEventArgs,
            >,
        >,
    {
        let this = self;
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
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveManipulationInertiaStarting)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn ManipulationCompleted<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<
                GestureRecognizer,
                ManipulationCompletedEventArgs,
            >,
        >,
    {
        let this = self;
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
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveManipulationCompleted)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn CrossSliding<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<GestureRecognizer, CrossSlidingEventArgs>,
        >,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CrossSliding)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveCrossSliding(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveCrossSliding)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
}
impl ::windows_core::RuntimeType for GestureRecognizer {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for GestureRecognizer {
    type Vtable = IGestureRecognizer_Vtbl;
}
unsafe impl ::windows_core::ComInterface for GestureRecognizer {
    const IID: ::windows_core::GUID = <IGestureRecognizer as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for GestureRecognizer {
    const NAME: &'static str = "Microsoft.UI.Input.GestureRecognizer";
}
::windows_core::imp::interface_hierarchy!(
    GestureRecognizer,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for GestureRecognizer {}
unsafe impl ::core::marker::Sync for GestureRecognizer {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct HoldingEventArgs(::windows_core::IUnknown);
impl HoldingEventArgs {
    pub fn HoldingState(&self) -> ::windows_core::Result<HoldingState> {
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
    pub fn PointerDeviceType(&self) -> ::windows_core::Result<PointerDeviceType> {
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
}
impl ::windows_core::RuntimeType for HoldingEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for HoldingEventArgs {
    type Vtable = IHoldingEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for HoldingEventArgs {
    const IID: ::windows_core::GUID = <IHoldingEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for HoldingEventArgs {
    const NAME: &'static str = "Microsoft.UI.Input.HoldingEventArgs";
}
::windows_core::imp::interface_hierarchy!(
    HoldingEventArgs,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for HoldingEventArgs {}
unsafe impl ::core::marker::Sync for HoldingEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct InputActivationListener(::windows_core::IUnknown);
impl InputActivationListener {
    pub fn State(&self) -> ::windows_core::Result<InputActivationState> {
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
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn InputActivationChanged<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<
                InputActivationListener,
                InputActivationListenerActivationChangedEventArgs,
            >,
        >,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).InputActivationChanged)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveInputActivationChanged(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveInputActivationChanged)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn GetForWindowId(
        windowid: super::WindowId,
    ) -> ::windows_core::Result<InputActivationListener> {
        Self::IInputActivationListenerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetForWindowId)(
                ::windows_core::Interface::as_raw(this),
                windowid,
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Microsoft_UI_Content\"`"]
    #[cfg(feature = "Microsoft_UI_Content")]
    pub fn GetForIsland<P0>(island: P0) -> ::windows_core::Result<InputActivationListener>
    where
        P0: ::windows_core::TryIntoParam<super::Content::ContentIsland>,
    {
        Self::IInputActivationListenerStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetForIsland)(
                ::windows_core::Interface::as_raw(this),
                island.try_into_param()?.abi(),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Microsoft_UI_Dispatching\"`"]
    #[cfg(feature = "Microsoft_UI_Dispatching")]
    pub fn DispatcherQueue(&self) -> ::windows_core::Result<super::Dispatching::DispatcherQueue> {
        let this = &::windows_core::ComInterface::cast::<IInputObject>(self)?;
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
    pub fn IInputActivationListenerStatics<
        R,
        F: FnOnce(&IInputActivationListenerStatics) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<
            InputActivationListener,
            IInputActivationListenerStatics,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IInputActivationListenerStatics2<
        R,
        F: FnOnce(&IInputActivationListenerStatics2) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<
            InputActivationListener,
            IInputActivationListenerStatics2,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for InputActivationListener {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for InputActivationListener {
    type Vtable = IInputActivationListener_Vtbl;
}
unsafe impl ::windows_core::ComInterface for InputActivationListener {
    const IID: ::windows_core::GUID =
        <IInputActivationListener as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for InputActivationListener {
    const NAME: &'static str = "Microsoft.UI.Input.InputActivationListener";
}
::windows_core::imp::interface_hierarchy!(
    InputActivationListener,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<InputObject> for InputActivationListener {}
unsafe impl ::core::marker::Send for InputActivationListener {}
unsafe impl ::core::marker::Sync for InputActivationListener {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct InputActivationListenerActivationChangedEventArgs(::windows_core::IUnknown);
impl InputActivationListenerActivationChangedEventArgs {}
impl ::windows_core::RuntimeType for InputActivationListenerActivationChangedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for InputActivationListenerActivationChangedEventArgs {
    type Vtable = IInputActivationListenerActivationChangedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for InputActivationListenerActivationChangedEventArgs {
    const IID: ::windows_core::GUID =
        <IInputActivationListenerActivationChangedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for InputActivationListenerActivationChangedEventArgs {
    const NAME: &'static str =
        "Microsoft.UI.Input.InputActivationListenerActivationChangedEventArgs";
}
::windows_core::imp::interface_hierarchy!(
    InputActivationListenerActivationChangedEventArgs,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for InputActivationListenerActivationChangedEventArgs {}
unsafe impl ::core::marker::Sync for InputActivationListenerActivationChangedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct InputCursor(::windows_core::IUnknown);
impl InputCursor {
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<::windows::Foundation::IClosable>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this))
                .ok()
        }
    }
    #[doc = "Required features: `\"Windows_UI_Core\"`"]
    #[cfg(feature = "Windows_UI_Core")]
    pub fn CreateFromCoreCursor<P0>(cursor: P0) -> ::windows_core::Result<InputCursor>
    where
        P0: ::windows_core::IntoParam<::windows::UI::Core::CoreCursor>,
    {
        Self::IInputCursorStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromCoreCursor)(
                ::windows_core::Interface::as_raw(this),
                cursor.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IInputCursorStatics<R, F: FnOnce(&IInputCursorStatics) -> ::windows_core::Result<R>>(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<InputCursor, IInputCursorStatics> =
            ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for InputCursor {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for InputCursor {
    type Vtable = IInputCursor_Vtbl;
}
unsafe impl ::windows_core::ComInterface for InputCursor {
    const IID: ::windows_core::GUID = <IInputCursor as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for InputCursor {
    const NAME: &'static str = "Microsoft.UI.Input.InputCursor";
}
::windows_core::imp::interface_hierarchy!(
    InputCursor,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
#[cfg(feature = "Windows_Foundation")]
impl ::windows_core::CanTryInto<::windows::Foundation::IClosable> for InputCursor {}
unsafe impl ::core::marker::Send for InputCursor {}
unsafe impl ::core::marker::Sync for InputCursor {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct InputCustomCursor(::windows_core::IUnknown);
impl InputCustomCursor {
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
impl ::windows_core::RuntimeType for InputCustomCursor {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for InputCustomCursor {
    type Vtable = IInputCustomCursor_Vtbl;
}
unsafe impl ::windows_core::ComInterface for InputCustomCursor {
    const IID: ::windows_core::GUID = <IInputCustomCursor as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for InputCustomCursor {
    const NAME: &'static str = "Microsoft.UI.Input.InputCustomCursor";
}
::windows_core::imp::interface_hierarchy!(
    InputCustomCursor,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
#[cfg(feature = "Windows_Foundation")]
impl ::windows_core::CanTryInto<::windows::Foundation::IClosable> for InputCustomCursor {}
impl ::windows_core::CanTryInto<InputCursor> for InputCustomCursor {}
unsafe impl ::core::marker::Send for InputCustomCursor {}
unsafe impl ::core::marker::Sync for InputCustomCursor {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct InputDesktopNamedResourceCursor(::windows_core::IUnknown);
impl InputDesktopNamedResourceCursor {
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<::windows::Foundation::IClosable>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this))
                .ok()
        }
    }
    pub fn ModuleName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ModuleName)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn ResourceName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ResourceName)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn Create(
        resourcename: &::windows_core::HSTRING,
    ) -> ::windows_core::Result<InputDesktopNamedResourceCursor> {
        Self::IInputDesktopNamedResourceCursorStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Create)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(resourcename),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn CreateFromModule(
        modulename: &::windows_core::HSTRING,
        resourcename: &::windows_core::HSTRING,
    ) -> ::windows_core::Result<InputDesktopNamedResourceCursor> {
        Self::IInputDesktopNamedResourceCursorStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromModule)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(modulename),
                ::core::mem::transmute_copy(resourcename),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IInputDesktopNamedResourceCursorStatics<
        R,
        F: FnOnce(&IInputDesktopNamedResourceCursorStatics) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<
            InputDesktopNamedResourceCursor,
            IInputDesktopNamedResourceCursorStatics,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for InputDesktopNamedResourceCursor {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for InputDesktopNamedResourceCursor {
    type Vtable = IInputDesktopNamedResourceCursor_Vtbl;
}
unsafe impl ::windows_core::ComInterface for InputDesktopNamedResourceCursor {
    const IID: ::windows_core::GUID =
        <IInputDesktopNamedResourceCursor as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for InputDesktopNamedResourceCursor {
    const NAME: &'static str = "Microsoft.UI.Input.InputDesktopNamedResourceCursor";
}
::windows_core::imp::interface_hierarchy!(
    InputDesktopNamedResourceCursor,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
#[cfg(feature = "Windows_Foundation")]
impl ::windows_core::CanTryInto<::windows::Foundation::IClosable>
    for InputDesktopNamedResourceCursor
{
}
impl ::windows_core::CanTryInto<InputCursor> for InputDesktopNamedResourceCursor {}
unsafe impl ::core::marker::Send for InputDesktopNamedResourceCursor {}
unsafe impl ::core::marker::Sync for InputDesktopNamedResourceCursor {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct InputDesktopResourceCursor(::windows_core::IUnknown);
impl InputDesktopResourceCursor {
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<::windows::Foundation::IClosable>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this))
                .ok()
        }
    }
    pub fn ModuleName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ModuleName)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn ResourceId(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ResourceId)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn Create(resourceid: u32) -> ::windows_core::Result<InputDesktopResourceCursor> {
        Self::IInputDesktopResourceCursorStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Create)(
                ::windows_core::Interface::as_raw(this),
                resourceid,
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn CreateFromModule(
        modulename: &::windows_core::HSTRING,
        resourceid: u32,
    ) -> ::windows_core::Result<InputDesktopResourceCursor> {
        Self::IInputDesktopResourceCursorStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromModule)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(modulename),
                resourceid,
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IInputDesktopResourceCursorStatics<
        R,
        F: FnOnce(&IInputDesktopResourceCursorStatics) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<
            InputDesktopResourceCursor,
            IInputDesktopResourceCursorStatics,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for InputDesktopResourceCursor {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for InputDesktopResourceCursor {
    type Vtable = IInputDesktopResourceCursor_Vtbl;
}
unsafe impl ::windows_core::ComInterface for InputDesktopResourceCursor {
    const IID: ::windows_core::GUID =
        <IInputDesktopResourceCursor as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for InputDesktopResourceCursor {
    const NAME: &'static str = "Microsoft.UI.Input.InputDesktopResourceCursor";
}
::windows_core::imp::interface_hierarchy!(
    InputDesktopResourceCursor,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
#[cfg(feature = "Windows_Foundation")]
impl ::windows_core::CanTryInto<::windows::Foundation::IClosable> for InputDesktopResourceCursor {}
impl ::windows_core::CanTryInto<InputCursor> for InputDesktopResourceCursor {}
unsafe impl ::core::marker::Send for InputDesktopResourceCursor {}
unsafe impl ::core::marker::Sync for InputDesktopResourceCursor {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct InputFocusChangedEventArgs(::windows_core::IUnknown);
impl InputFocusChangedEventArgs {}
impl ::windows_core::RuntimeType for InputFocusChangedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for InputFocusChangedEventArgs {
    type Vtable = IInputFocusChangedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for InputFocusChangedEventArgs {
    const IID: ::windows_core::GUID =
        <IInputFocusChangedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for InputFocusChangedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Input.InputFocusChangedEventArgs";
}
::windows_core::imp::interface_hierarchy!(
    InputFocusChangedEventArgs,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for InputFocusChangedEventArgs {}
unsafe impl ::core::marker::Sync for InputFocusChangedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct InputFocusController(::windows_core::IUnknown);
impl InputFocusController {
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
    pub fn TrySetFocus(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TrySetFocus)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
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
            ::windows::Foundation::TypedEventHandler<InputFocusController, FocusChangedEventArgs>,
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
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn LostFocus<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<InputFocusController, FocusChangedEventArgs>,
        >,
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
    #[doc = "Required features: `\"Microsoft_UI_Content\"`"]
    #[cfg(feature = "Microsoft_UI_Content")]
    pub fn GetForIsland<P0>(island: P0) -> ::windows_core::Result<InputFocusController>
    where
        P0: ::windows_core::TryIntoParam<super::Content::ContentIsland>,
    {
        Self::IInputFocusControllerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetForIsland)(
                ::windows_core::Interface::as_raw(this),
                island.try_into_param()?.abi(),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Microsoft_UI_Dispatching\"`"]
    #[cfg(feature = "Microsoft_UI_Dispatching")]
    pub fn DispatcherQueue(&self) -> ::windows_core::Result<super::Dispatching::DispatcherQueue> {
        let this = &::windows_core::ComInterface::cast::<IInputObject>(self)?;
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
    pub fn IInputFocusControllerStatics<
        R,
        F: FnOnce(&IInputFocusControllerStatics) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<
            InputFocusController,
            IInputFocusControllerStatics,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for InputFocusController {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for InputFocusController {
    type Vtable = IInputFocusController_Vtbl;
}
unsafe impl ::windows_core::ComInterface for InputFocusController {
    const IID: ::windows_core::GUID = <IInputFocusController as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for InputFocusController {
    const NAME: &'static str = "Microsoft.UI.Input.InputFocusController";
}
::windows_core::imp::interface_hierarchy!(
    InputFocusController,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<InputObject> for InputFocusController {}
unsafe impl ::core::marker::Send for InputFocusController {}
unsafe impl ::core::marker::Sync for InputFocusController {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct InputKeyboardSource(::windows_core::IUnknown);
impl InputKeyboardSource {
    #[doc = "Required features: `\"Windows_System\"`"]
    #[cfg(feature = "Windows_System")]
    pub fn GetCurrentKeyState(
        &self,
        virtualkey: ::windows::System::VirtualKey,
    ) -> ::windows_core::Result<VirtualKeyStates> {
        let this = &::windows_core::ComInterface::cast::<IInputKeyboardSource2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetCurrentKeyState)(
                ::windows_core::Interface::as_raw(this),
                virtualkey,
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_System\"`"]
    #[cfg(feature = "Windows_System")]
    pub fn GetKeyState(
        &self,
        virtualkey: ::windows::System::VirtualKey,
    ) -> ::windows_core::Result<VirtualKeyStates> {
        let this = &::windows_core::ComInterface::cast::<IInputKeyboardSource2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetKeyState)(
                ::windows_core::Interface::as_raw(this),
                virtualkey,
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn CharacterReceived<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<
                InputKeyboardSource,
                CharacterReceivedEventArgs,
            >,
        >,
    {
        let this = &::windows_core::ComInterface::cast::<IInputKeyboardSource2>(self)?;
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
        let this = &::windows_core::ComInterface::cast::<IInputKeyboardSource2>(self)?;
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
    pub fn ContextMenuKey<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<InputKeyboardSource, ContextMenuKeyEventArgs>,
        >,
    {
        let this = &::windows_core::ComInterface::cast::<IInputKeyboardSource2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ContextMenuKey)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveContextMenuKey(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IInputKeyboardSource2>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveContextMenuKey)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn KeyDown<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<InputKeyboardSource, KeyEventArgs>,
        >,
    {
        let this = &::windows_core::ComInterface::cast::<IInputKeyboardSource2>(self)?;
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
        let this = &::windows_core::ComInterface::cast::<IInputKeyboardSource2>(self)?;
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
    pub fn KeyUp<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<InputKeyboardSource, KeyEventArgs>,
        >,
    {
        let this = &::windows_core::ComInterface::cast::<IInputKeyboardSource2>(self)?;
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
        let this = &::windows_core::ComInterface::cast::<IInputKeyboardSource2>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveKeyUp)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn SystemKeyDown<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<InputKeyboardSource, KeyEventArgs>,
        >,
    {
        let this = &::windows_core::ComInterface::cast::<IInputKeyboardSource2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SystemKeyDown)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveSystemKeyDown(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IInputKeyboardSource2>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveSystemKeyDown)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn SystemKeyUp<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<InputKeyboardSource, KeyEventArgs>,
        >,
    {
        let this = &::windows_core::ComInterface::cast::<IInputKeyboardSource2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SystemKeyUp)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveSystemKeyUp(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IInputKeyboardSource2>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveSystemKeyUp)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_System\"`, `\"Windows_UI_Core\"`"]
    #[cfg(all(feature = "Windows_System", feature = "Windows_UI_Core"))]
    pub fn GetKeyStateForCurrentThread(
        virtualkey: ::windows::System::VirtualKey,
    ) -> ::windows_core::Result<::windows::UI::Core::CoreVirtualKeyStates> {
        Self::IInputKeyboardSourceStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetKeyStateForCurrentThread)(
                ::windows_core::Interface::as_raw(this),
                virtualkey,
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Microsoft_UI_Content\"`"]
    #[cfg(feature = "Microsoft_UI_Content")]
    pub fn GetForIsland<P0>(island: P0) -> ::windows_core::Result<InputKeyboardSource>
    where
        P0: ::windows_core::TryIntoParam<super::Content::ContentIsland>,
    {
        Self::IInputKeyboardSourceStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetForIsland)(
                ::windows_core::Interface::as_raw(this),
                island.try_into_param()?.abi(),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Microsoft_UI_Dispatching\"`"]
    #[cfg(feature = "Microsoft_UI_Dispatching")]
    pub fn DispatcherQueue(&self) -> ::windows_core::Result<super::Dispatching::DispatcherQueue> {
        let this = &::windows_core::ComInterface::cast::<IInputObject>(self)?;
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
    pub fn IInputKeyboardSourceStatics<
        R,
        F: FnOnce(&IInputKeyboardSourceStatics) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<
            InputKeyboardSource,
            IInputKeyboardSourceStatics,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IInputKeyboardSourceStatics2<
        R,
        F: FnOnce(&IInputKeyboardSourceStatics2) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<
            InputKeyboardSource,
            IInputKeyboardSourceStatics2,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for InputKeyboardSource {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for InputKeyboardSource {
    type Vtable = IInputKeyboardSource_Vtbl;
}
unsafe impl ::windows_core::ComInterface for InputKeyboardSource {
    const IID: ::windows_core::GUID = <IInputKeyboardSource as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for InputKeyboardSource {
    const NAME: &'static str = "Microsoft.UI.Input.InputKeyboardSource";
}
::windows_core::imp::interface_hierarchy!(
    InputKeyboardSource,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<InputObject> for InputKeyboardSource {}
unsafe impl ::core::marker::Send for InputKeyboardSource {}
unsafe impl ::core::marker::Sync for InputKeyboardSource {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct InputLightDismissAction(::windows_core::IUnknown);
impl InputLightDismissAction {
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn Dismissed<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<
                InputLightDismissAction,
                InputLightDismissEventArgs,
            >,
        >,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Dismissed)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveDismissed(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveDismissed)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn GetForWindowId(
        windowid: super::WindowId,
    ) -> ::windows_core::Result<InputLightDismissAction> {
        Self::IInputLightDismissActionStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetForWindowId)(
                ::windows_core::Interface::as_raw(this),
                windowid,
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Microsoft_UI_Dispatching\"`"]
    #[cfg(feature = "Microsoft_UI_Dispatching")]
    pub fn DispatcherQueue(&self) -> ::windows_core::Result<super::Dispatching::DispatcherQueue> {
        let this = &::windows_core::ComInterface::cast::<IInputObject>(self)?;
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
    pub fn IInputLightDismissActionStatics<
        R,
        F: FnOnce(&IInputLightDismissActionStatics) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<
            InputLightDismissAction,
            IInputLightDismissActionStatics,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for InputLightDismissAction {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for InputLightDismissAction {
    type Vtable = IInputLightDismissAction_Vtbl;
}
unsafe impl ::windows_core::ComInterface for InputLightDismissAction {
    const IID: ::windows_core::GUID =
        <IInputLightDismissAction as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for InputLightDismissAction {
    const NAME: &'static str = "Microsoft.UI.Input.InputLightDismissAction";
}
::windows_core::imp::interface_hierarchy!(
    InputLightDismissAction,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<InputObject> for InputLightDismissAction {}
unsafe impl ::core::marker::Send for InputLightDismissAction {}
unsafe impl ::core::marker::Sync for InputLightDismissAction {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct InputLightDismissEventArgs(::windows_core::IUnknown);
impl InputLightDismissEventArgs {}
impl ::windows_core::RuntimeType for InputLightDismissEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for InputLightDismissEventArgs {
    type Vtable = IInputLightDismissEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for InputLightDismissEventArgs {
    const IID: ::windows_core::GUID =
        <IInputLightDismissEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for InputLightDismissEventArgs {
    const NAME: &'static str = "Microsoft.UI.Input.InputLightDismissEventArgs";
}
::windows_core::imp::interface_hierarchy!(
    InputLightDismissEventArgs,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for InputLightDismissEventArgs {}
unsafe impl ::core::marker::Sync for InputLightDismissEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct InputNonClientPointerSource(::windows_core::IUnknown);
impl InputNonClientPointerSource {
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
    pub fn ClearAllRegionRects(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).ClearAllRegionRects)(
                ::windows_core::Interface::as_raw(this),
            )
            .ok()
        }
    }
    pub fn ClearRegionRects(&self, region: NonClientRegionKind) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).ClearRegionRects)(
                ::windows_core::Interface::as_raw(this),
                region,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Graphics\"`"]
    #[cfg(feature = "Windows_Graphics")]
    pub fn GetRegionRects(
        &self,
        region: NonClientRegionKind,
    ) -> ::windows_core::Result<::windows_core::Array<::windows::Graphics::RectInt32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows_core::Interface::vtable(this).GetRegionRects)(
                ::windows_core::Interface::as_raw(this),
                region,
                ::windows_core::Array::<::windows::Graphics::RectInt32>::set_abi_len(
                    ::std::mem::transmute(&mut result__),
                ),
                result__.as_mut_ptr() as *mut _ as _,
            )
            .and_then(|| result__.assume_init())
        }
    }
    #[doc = "Required features: `\"Windows_Graphics\"`"]
    #[cfg(feature = "Windows_Graphics")]
    pub fn SetRegionRects(
        &self,
        region: NonClientRegionKind,
        rects: &[::windows::Graphics::RectInt32],
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetRegionRects)(
                ::windows_core::Interface::as_raw(this),
                region,
                rects.len().try_into().unwrap(),
                rects.as_ptr(),
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn CaptionTapped<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<
                InputNonClientPointerSource,
                NonClientCaptionTappedEventArgs,
            >,
        >,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CaptionTapped)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveCaptionTapped(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveCaptionTapped)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn PointerEntered<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<
                InputNonClientPointerSource,
                NonClientPointerEventArgs,
            >,
        >,
    {
        let this = self;
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
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).RemovePointerEntered)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn PointerExited<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<
                InputNonClientPointerSource,
                NonClientPointerEventArgs,
            >,
        >,
    {
        let this = self;
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
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).RemovePointerExited)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn PointerMoved<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<
                InputNonClientPointerSource,
                NonClientPointerEventArgs,
            >,
        >,
    {
        let this = self;
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
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).RemovePointerMoved)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn PointerPressed<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<
                InputNonClientPointerSource,
                NonClientPointerEventArgs,
            >,
        >,
    {
        let this = self;
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
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).RemovePointerPressed)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn PointerReleased<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<
                InputNonClientPointerSource,
                NonClientPointerEventArgs,
            >,
        >,
    {
        let this = self;
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
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).RemovePointerReleased)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RegionsChanged<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<
                InputNonClientPointerSource,
                NonClientRegionsChangedEventArgs,
            >,
        >,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RegionsChanged)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveRegionsChanged(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveRegionsChanged)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn GetForWindowId(
        windowid: super::WindowId,
    ) -> ::windows_core::Result<InputNonClientPointerSource> {
        Self::IInputNonClientPointerSourceStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetForWindowId)(
                ::windows_core::Interface::as_raw(this),
                windowid,
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IInputNonClientPointerSourceStatics<
        R,
        F: FnOnce(&IInputNonClientPointerSourceStatics) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<
            InputNonClientPointerSource,
            IInputNonClientPointerSourceStatics,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for InputNonClientPointerSource {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for InputNonClientPointerSource {
    type Vtable = IInputNonClientPointerSource_Vtbl;
}
unsafe impl ::windows_core::ComInterface for InputNonClientPointerSource {
    const IID: ::windows_core::GUID =
        <IInputNonClientPointerSource as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for InputNonClientPointerSource {
    const NAME: &'static str = "Microsoft.UI.Input.InputNonClientPointerSource";
}
::windows_core::imp::interface_hierarchy!(
    InputNonClientPointerSource,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for InputNonClientPointerSource {}
unsafe impl ::core::marker::Sync for InputNonClientPointerSource {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct InputObject(::windows_core::IUnknown);
impl InputObject {
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
}
impl ::windows_core::RuntimeType for InputObject {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for InputObject {
    type Vtable = IInputObject_Vtbl;
}
unsafe impl ::windows_core::ComInterface for InputObject {
    const IID: ::windows_core::GUID = <IInputObject as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for InputObject {
    const NAME: &'static str = "Microsoft.UI.Input.InputObject";
}
::windows_core::imp::interface_hierarchy!(
    InputObject,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for InputObject {}
unsafe impl ::core::marker::Sync for InputObject {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct InputPointerSource(::windows_core::IUnknown);
impl InputPointerSource {
    #[doc = "Required features: `\"Microsoft_UI_Dispatching\"`"]
    #[cfg(feature = "Microsoft_UI_Dispatching")]
    pub fn DispatcherQueue(&self) -> ::windows_core::Result<super::Dispatching::DispatcherQueue> {
        let this = &::windows_core::ComInterface::cast::<IInputObject>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DispatcherQueue)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn Cursor(&self) -> ::windows_core::Result<InputCursor> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Cursor)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetCursor<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<InputCursor>,
    {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetCursor)(
                ::windows_core::Interface::as_raw(this),
                value.try_into_param()?.abi(),
            )
            .ok()
        }
    }
    pub fn DeviceKinds(&self) -> ::windows_core::Result<InputPointerSourceDeviceKinds> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DeviceKinds)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn PointerCaptureLost<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<InputPointerSource, PointerEventArgs>,
        >,
    {
        let this = self;
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
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).RemovePointerCaptureLost)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn PointerEntered<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<InputPointerSource, PointerEventArgs>,
        >,
    {
        let this = self;
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
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).RemovePointerEntered)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn PointerExited<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<InputPointerSource, PointerEventArgs>,
        >,
    {
        let this = self;
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
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).RemovePointerExited)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn PointerMoved<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<InputPointerSource, PointerEventArgs>,
        >,
    {
        let this = self;
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
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).RemovePointerMoved)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn PointerPressed<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<InputPointerSource, PointerEventArgs>,
        >,
    {
        let this = self;
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
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).RemovePointerPressed)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn PointerReleased<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<InputPointerSource, PointerEventArgs>,
        >,
    {
        let this = self;
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
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).RemovePointerReleased)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn PointerRoutedAway<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<InputPointerSource, PointerEventArgs>,
        >,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PointerRoutedAway)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemovePointerRoutedAway(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).RemovePointerRoutedAway)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn PointerRoutedReleased<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<InputPointerSource, PointerEventArgs>,
        >,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PointerRoutedReleased)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemovePointerRoutedReleased(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).RemovePointerRoutedReleased)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn PointerRoutedTo<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<InputPointerSource, PointerEventArgs>,
        >,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PointerRoutedTo)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemovePointerRoutedTo(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).RemovePointerRoutedTo)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn PointerWheelChanged<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<InputPointerSource, PointerEventArgs>,
        >,
    {
        let this = self;
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
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).RemovePointerWheelChanged)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Content\"`"]
    #[cfg(feature = "Microsoft_UI_Content")]
    pub fn GetForIsland<P0>(island: P0) -> ::windows_core::Result<InputPointerSource>
    where
        P0: ::windows_core::TryIntoParam<super::Content::ContentIsland>,
    {
        Self::IInputPointerSourceStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetForIsland)(
                ::windows_core::Interface::as_raw(this),
                island.try_into_param()?.abi(),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IInputPointerSourceStatics<
        R,
        F: FnOnce(&IInputPointerSourceStatics) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<
            InputPointerSource,
            IInputPointerSourceStatics,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for InputPointerSource {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for InputPointerSource {
    type Vtable = IInputPointerSource_Vtbl;
}
unsafe impl ::windows_core::ComInterface for InputPointerSource {
    const IID: ::windows_core::GUID = <IInputPointerSource as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for InputPointerSource {
    const NAME: &'static str = "Microsoft.UI.Input.InputPointerSource";
}
::windows_core::imp::interface_hierarchy!(
    InputPointerSource,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<InputObject> for InputPointerSource {}
unsafe impl ::core::marker::Send for InputPointerSource {}
unsafe impl ::core::marker::Sync for InputPointerSource {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct InputPreTranslateKeyboardSource(::windows_core::IUnknown);
impl InputPreTranslateKeyboardSource {
    #[doc = "Required features: `\"Microsoft_UI_Dispatching\"`"]
    #[cfg(feature = "Microsoft_UI_Dispatching")]
    pub fn DispatcherQueue(&self) -> ::windows_core::Result<super::Dispatching::DispatcherQueue> {
        let this = &::windows_core::ComInterface::cast::<IInputObject>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DispatcherQueue)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Content\"`"]
    #[cfg(feature = "Microsoft_UI_Content")]
    pub fn GetForIsland<P0>(island: P0) -> ::windows_core::Result<InputPreTranslateKeyboardSource>
    where
        P0: ::windows_core::TryIntoParam<super::Content::ContentIsland>,
    {
        Self::IInputPreTranslateKeyboardSourceStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetForIsland)(
                ::windows_core::Interface::as_raw(this),
                island.try_into_param()?.abi(),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IInputPreTranslateKeyboardSourceStatics<
        R,
        F: FnOnce(&IInputPreTranslateKeyboardSourceStatics) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<
            InputPreTranslateKeyboardSource,
            IInputPreTranslateKeyboardSourceStatics,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for InputPreTranslateKeyboardSource {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for InputPreTranslateKeyboardSource {
    type Vtable = IInputPreTranslateKeyboardSource_Vtbl;
}
unsafe impl ::windows_core::ComInterface for InputPreTranslateKeyboardSource {
    const IID: ::windows_core::GUID =
        <IInputPreTranslateKeyboardSource as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for InputPreTranslateKeyboardSource {
    const NAME: &'static str = "Microsoft.UI.Input.InputPreTranslateKeyboardSource";
}
::windows_core::imp::interface_hierarchy!(
    InputPreTranslateKeyboardSource,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<InputObject> for InputPreTranslateKeyboardSource {}
unsafe impl ::core::marker::Send for InputPreTranslateKeyboardSource {}
unsafe impl ::core::marker::Sync for InputPreTranslateKeyboardSource {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct InputSystemCursor(::windows_core::IUnknown);
impl InputSystemCursor {
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<::windows::Foundation::IClosable>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this))
                .ok()
        }
    }
    pub fn CursorShape(&self) -> ::windows_core::Result<InputSystemCursorShape> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CursorShape)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn Create(r#type: InputSystemCursorShape) -> ::windows_core::Result<InputSystemCursor> {
        Self::IInputSystemCursorStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Create)(
                ::windows_core::Interface::as_raw(this),
                r#type,
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IInputSystemCursorStatics<
        R,
        F: FnOnce(&IInputSystemCursorStatics) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<
            InputSystemCursor,
            IInputSystemCursorStatics,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for InputSystemCursor {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for InputSystemCursor {
    type Vtable = IInputSystemCursor_Vtbl;
}
unsafe impl ::windows_core::ComInterface for InputSystemCursor {
    const IID: ::windows_core::GUID = <IInputSystemCursor as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for InputSystemCursor {
    const NAME: &'static str = "Microsoft.UI.Input.InputSystemCursor";
}
::windows_core::imp::interface_hierarchy!(
    InputSystemCursor,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
#[cfg(feature = "Windows_Foundation")]
impl ::windows_core::CanTryInto<::windows::Foundation::IClosable> for InputSystemCursor {}
impl ::windows_core::CanTryInto<InputCursor> for InputSystemCursor {}
unsafe impl ::core::marker::Send for InputSystemCursor {}
unsafe impl ::core::marker::Sync for InputSystemCursor {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct KeyEventArgs(::windows_core::IUnknown);
impl KeyEventArgs {
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
    pub fn KeyStatus(&self) -> ::windows_core::Result<PhysicalKeyStatus> {
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
    pub fn Timestamp(&self) -> ::windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Timestamp)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_System\"`"]
    #[cfg(feature = "Windows_System")]
    pub fn VirtualKey(&self) -> ::windows_core::Result<::windows::System::VirtualKey> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).VirtualKey)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for KeyEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for KeyEventArgs {
    type Vtable = IKeyEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for KeyEventArgs {
    const IID: ::windows_core::GUID = <IKeyEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for KeyEventArgs {
    const NAME: &'static str = "Microsoft.UI.Input.KeyEventArgs";
}
::windows_core::imp::interface_hierarchy!(
    KeyEventArgs,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for KeyEventArgs {}
unsafe impl ::core::marker::Sync for KeyEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ManipulationCompletedEventArgs(::windows_core::IUnknown);
impl ManipulationCompletedEventArgs {
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn Cumulative(&self) -> ::windows_core::Result<ManipulationDelta> {
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
    pub fn PointerDeviceType(&self) -> ::windows_core::Result<PointerDeviceType> {
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
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn Velocities(&self) -> ::windows_core::Result<ManipulationVelocities> {
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
}
impl ::windows_core::RuntimeType for ManipulationCompletedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for ManipulationCompletedEventArgs {
    type Vtable = IManipulationCompletedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ManipulationCompletedEventArgs {
    const IID: ::windows_core::GUID =
        <IManipulationCompletedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ManipulationCompletedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Input.ManipulationCompletedEventArgs";
}
::windows_core::imp::interface_hierarchy!(
    ManipulationCompletedEventArgs,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for ManipulationCompletedEventArgs {}
unsafe impl ::core::marker::Sync for ManipulationCompletedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ManipulationInertiaStartingEventArgs(::windows_core::IUnknown);
impl ManipulationInertiaStartingEventArgs {
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn Cumulative(&self) -> ::windows_core::Result<ManipulationDelta> {
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
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn Delta(&self) -> ::windows_core::Result<ManipulationDelta> {
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
    pub fn PointerDeviceType(&self) -> ::windows_core::Result<PointerDeviceType> {
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
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn Velocities(&self) -> ::windows_core::Result<ManipulationVelocities> {
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
}
impl ::windows_core::RuntimeType for ManipulationInertiaStartingEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for ManipulationInertiaStartingEventArgs {
    type Vtable = IManipulationInertiaStartingEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ManipulationInertiaStartingEventArgs {
    const IID: ::windows_core::GUID =
        <IManipulationInertiaStartingEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ManipulationInertiaStartingEventArgs {
    const NAME: &'static str = "Microsoft.UI.Input.ManipulationInertiaStartingEventArgs";
}
::windows_core::imp::interface_hierarchy!(
    ManipulationInertiaStartingEventArgs,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for ManipulationInertiaStartingEventArgs {}
unsafe impl ::core::marker::Sync for ManipulationInertiaStartingEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ManipulationStartedEventArgs(::windows_core::IUnknown);
impl ManipulationStartedEventArgs {
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn Cumulative(&self) -> ::windows_core::Result<ManipulationDelta> {
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
    pub fn PointerDeviceType(&self) -> ::windows_core::Result<PointerDeviceType> {
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
}
impl ::windows_core::RuntimeType for ManipulationStartedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for ManipulationStartedEventArgs {
    type Vtable = IManipulationStartedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ManipulationStartedEventArgs {
    const IID: ::windows_core::GUID =
        <IManipulationStartedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ManipulationStartedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Input.ManipulationStartedEventArgs";
}
::windows_core::imp::interface_hierarchy!(
    ManipulationStartedEventArgs,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for ManipulationStartedEventArgs {}
unsafe impl ::core::marker::Sync for ManipulationStartedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ManipulationUpdatedEventArgs(::windows_core::IUnknown);
impl ManipulationUpdatedEventArgs {
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn Cumulative(&self) -> ::windows_core::Result<ManipulationDelta> {
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
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn Delta(&self) -> ::windows_core::Result<ManipulationDelta> {
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
    pub fn PointerDeviceType(&self) -> ::windows_core::Result<PointerDeviceType> {
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
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn Velocities(&self) -> ::windows_core::Result<ManipulationVelocities> {
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
}
impl ::windows_core::RuntimeType for ManipulationUpdatedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for ManipulationUpdatedEventArgs {
    type Vtable = IManipulationUpdatedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ManipulationUpdatedEventArgs {
    const IID: ::windows_core::GUID =
        <IManipulationUpdatedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ManipulationUpdatedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Input.ManipulationUpdatedEventArgs";
}
::windows_core::imp::interface_hierarchy!(
    ManipulationUpdatedEventArgs,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for ManipulationUpdatedEventArgs {}
unsafe impl ::core::marker::Sync for ManipulationUpdatedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct MouseWheelParameters(::windows_core::IUnknown);
impl MouseWheelParameters {
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn CharTranslation(&self) -> ::windows_core::Result<::windows::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CharTranslation)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn SetCharTranslation(
        &self,
        value: ::windows::Foundation::Point,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetCharTranslation)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn DeltaScale(&self) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DeltaScale)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetDeltaScale(&self, value: f32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetDeltaScale)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn DeltaRotationAngle(&self) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DeltaRotationAngle)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetDeltaRotationAngle(&self, value: f32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetDeltaRotationAngle)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn PageTranslation(&self) -> ::windows_core::Result<::windows::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PageTranslation)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn SetPageTranslation(
        &self,
        value: ::windows::Foundation::Point,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetPageTranslation)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
}
impl ::windows_core::RuntimeType for MouseWheelParameters {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for MouseWheelParameters {
    type Vtable = IMouseWheelParameters_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MouseWheelParameters {
    const IID: ::windows_core::GUID = <IMouseWheelParameters as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MouseWheelParameters {
    const NAME: &'static str = "Microsoft.UI.Input.MouseWheelParameters";
}
::windows_core::imp::interface_hierarchy!(
    MouseWheelParameters,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for MouseWheelParameters {}
unsafe impl ::core::marker::Sync for MouseWheelParameters {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct NonClientCaptionTappedEventArgs(::windows_core::IUnknown);
impl NonClientCaptionTappedEventArgs {
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
    pub fn PointerDeviceType(&self) -> ::windows_core::Result<PointerDeviceType> {
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
}
impl ::windows_core::RuntimeType for NonClientCaptionTappedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for NonClientCaptionTappedEventArgs {
    type Vtable = INonClientCaptionTappedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for NonClientCaptionTappedEventArgs {
    const IID: ::windows_core::GUID =
        <INonClientCaptionTappedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for NonClientCaptionTappedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Input.NonClientCaptionTappedEventArgs";
}
::windows_core::imp::interface_hierarchy!(
    NonClientCaptionTappedEventArgs,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for NonClientCaptionTappedEventArgs {}
unsafe impl ::core::marker::Sync for NonClientCaptionTappedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct NonClientPointerEventArgs(::windows_core::IUnknown);
impl NonClientPointerEventArgs {
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
    pub fn PointerDeviceType(&self) -> ::windows_core::Result<PointerDeviceType> {
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
    pub fn RegionKind(&self) -> ::windows_core::Result<NonClientRegionKind> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RegionKind)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn IsPointInRegion(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsPointInRegion)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for NonClientPointerEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for NonClientPointerEventArgs {
    type Vtable = INonClientPointerEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for NonClientPointerEventArgs {
    const IID: ::windows_core::GUID =
        <INonClientPointerEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for NonClientPointerEventArgs {
    const NAME: &'static str = "Microsoft.UI.Input.NonClientPointerEventArgs";
}
::windows_core::imp::interface_hierarchy!(
    NonClientPointerEventArgs,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for NonClientPointerEventArgs {}
unsafe impl ::core::marker::Sync for NonClientPointerEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct NonClientRegionsChangedEventArgs(::windows_core::IUnknown);
impl NonClientRegionsChangedEventArgs {
    pub fn ChangedRegions(
        &self,
    ) -> ::windows_core::Result<::windows_core::Array<NonClientRegionKind>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows_core::Interface::vtable(this).ChangedRegions)(
                ::windows_core::Interface::as_raw(this),
                ::windows_core::Array::<NonClientRegionKind>::set_abi_len(::std::mem::transmute(
                    &mut result__,
                )),
                result__.as_mut_ptr() as *mut _ as _,
            )
            .and_then(|| result__.assume_init())
        }
    }
}
impl ::windows_core::RuntimeType for NonClientRegionsChangedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for NonClientRegionsChangedEventArgs {
    type Vtable = INonClientRegionsChangedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for NonClientRegionsChangedEventArgs {
    const IID: ::windows_core::GUID =
        <INonClientRegionsChangedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for NonClientRegionsChangedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Input.NonClientRegionsChangedEventArgs";
}
::windows_core::imp::interface_hierarchy!(
    NonClientRegionsChangedEventArgs,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for NonClientRegionsChangedEventArgs {}
unsafe impl ::core::marker::Sync for NonClientRegionsChangedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct PointerEventArgs(::windows_core::IUnknown);
impl PointerEventArgs {
    pub fn CurrentPoint(&self) -> ::windows_core::Result<PointerPoint> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CurrentPoint)(
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
    #[doc = "Required features: `\"Windows_Foundation_Collections\"`"]
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub fn GetIntermediatePoints(
        &self,
    ) -> ::windows_core::Result<::windows::Foundation::Collections::IVector<PointerPoint>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetIntermediatePoints)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation_Collections\"`"]
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub fn GetIntermediateTransformedPoints<P0>(
        &self,
        transform: P0,
    ) -> ::windows_core::Result<::windows::Foundation::Collections::IVector<PointerPoint>>
    where
        P0: ::windows_core::TryIntoParam<IPointerPointTransform>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetIntermediateTransformedPoints)(
                ::windows_core::Interface::as_raw(this),
                transform.try_into_param()?.abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for PointerEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for PointerEventArgs {
    type Vtable = IPointerEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PointerEventArgs {
    const IID: ::windows_core::GUID = <IPointerEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PointerEventArgs {
    const NAME: &'static str = "Microsoft.UI.Input.PointerEventArgs";
}
::windows_core::imp::interface_hierarchy!(
    PointerEventArgs,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for PointerEventArgs {}
unsafe impl ::core::marker::Sync for PointerEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct PointerPoint(::windows_core::IUnknown);
impl PointerPoint {
    pub fn FrameId(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FrameId)(
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
    pub fn PointerDeviceType(&self) -> ::windows_core::Result<PointerDeviceType> {
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
    pub fn Properties(&self) -> ::windows_core::Result<PointerPointProperties> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Properties)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn Timestamp(&self) -> ::windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Timestamp)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn GetTransformedPoint<P0>(&self, transform: P0) -> ::windows_core::Result<PointerPoint>
    where
        P0: ::windows_core::TryIntoParam<IPointerPointTransform>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetTransformedPoint)(
                ::windows_core::Interface::as_raw(this),
                transform.try_into_param()?.abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for PointerPoint {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for PointerPoint {
    type Vtable = IPointerPoint_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PointerPoint {
    const IID: ::windows_core::GUID = <IPointerPoint as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PointerPoint {
    const NAME: &'static str = "Microsoft.UI.Input.PointerPoint";
}
::windows_core::imp::interface_hierarchy!(
    PointerPoint,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for PointerPoint {}
unsafe impl ::core::marker::Sync for PointerPoint {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct PointerPointProperties(::windows_core::IUnknown);
impl PointerPointProperties {
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn ContactRect(&self) -> ::windows_core::Result<::windows::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ContactRect)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn IsBarrelButtonPressed(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsBarrelButtonPressed)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn IsCanceled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsCanceled)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn IsEraser(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsEraser)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn IsHorizontalMouseWheel(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsHorizontalMouseWheel)(
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
    pub fn IsInverted(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsInverted)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn IsLeftButtonPressed(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsLeftButtonPressed)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn IsMiddleButtonPressed(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsMiddleButtonPressed)(
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
    pub fn IsRightButtonPressed(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsRightButtonPressed)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn IsXButton1Pressed(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsXButton1Pressed)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn IsXButton2Pressed(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsXButton2Pressed)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn MouseWheelDelta(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MouseWheelDelta)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn Orientation(&self) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Orientation)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn PointerUpdateKind(&self) -> ::windows_core::Result<PointerUpdateKind> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PointerUpdateKind)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn Pressure(&self) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Pressure)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn TouchConfidence(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TouchConfidence)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn Twist(&self) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Twist)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn XTilt(&self) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).XTilt)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn YTilt(&self) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).YTilt)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for PointerPointProperties {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for PointerPointProperties {
    type Vtable = IPointerPointProperties_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PointerPointProperties {
    const IID: ::windows_core::GUID =
        <IPointerPointProperties as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PointerPointProperties {
    const NAME: &'static str = "Microsoft.UI.Input.PointerPointProperties";
}
::windows_core::imp::interface_hierarchy!(
    PointerPointProperties,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for PointerPointProperties {}
unsafe impl ::core::marker::Sync for PointerPointProperties {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct PointerPredictor(::windows_core::IUnknown);
impl PointerPredictor {
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
    pub fn PredictionTime(&self) -> ::windows_core::Result<::windows::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PredictionTime)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn SetPredictionTime(
        &self,
        value: ::windows::Foundation::TimeSpan,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetPredictionTime)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn GetPredictedPoints<P0>(
        &self,
        point: P0,
    ) -> ::windows_core::Result<::windows_core::Array<PointerPoint>>
    where
        P0: ::windows_core::IntoParam<PointerPoint>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows_core::Interface::vtable(this).GetPredictedPoints)(
                ::windows_core::Interface::as_raw(this),
                point.into_param().abi(),
                ::windows_core::Array::<PointerPoint>::set_abi_len(::std::mem::transmute(
                    &mut result__,
                )),
                result__.as_mut_ptr() as *mut _ as _,
            )
            .and_then(|| result__.assume_init())
        }
    }
    pub fn CreateForInputPointerSource<P0>(
        inputpointersource: P0,
    ) -> ::windows_core::Result<PointerPredictor>
    where
        P0: ::windows_core::IntoParam<InputPointerSource>,
    {
        Self::IPointerPredictorStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateForInputPointerSource)(
                ::windows_core::Interface::as_raw(this),
                inputpointersource.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPointerPredictorStatics<
        R,
        F: FnOnce(&IPointerPredictorStatics) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<
            PointerPredictor,
            IPointerPredictorStatics,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for PointerPredictor {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for PointerPredictor {
    type Vtable = IPointerPredictor_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PointerPredictor {
    const IID: ::windows_core::GUID = <IPointerPredictor as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PointerPredictor {
    const NAME: &'static str = "Microsoft.UI.Input.PointerPredictor";
}
::windows_core::imp::interface_hierarchy!(
    PointerPredictor,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
#[cfg(feature = "Windows_Foundation")]
impl ::windows_core::CanTryInto<::windows::Foundation::IClosable> for PointerPredictor {}
unsafe impl ::core::marker::Send for PointerPredictor {}
unsafe impl ::core::marker::Sync for PointerPredictor {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct RightTappedEventArgs(::windows_core::IUnknown);
impl RightTappedEventArgs {
    pub fn PointerDeviceType(&self) -> ::windows_core::Result<PointerDeviceType> {
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
}
impl ::windows_core::RuntimeType for RightTappedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for RightTappedEventArgs {
    type Vtable = IRightTappedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for RightTappedEventArgs {
    const IID: ::windows_core::GUID = <IRightTappedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for RightTappedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Input.RightTappedEventArgs";
}
::windows_core::imp::interface_hierarchy!(
    RightTappedEventArgs,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for RightTappedEventArgs {}
unsafe impl ::core::marker::Sync for RightTappedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct TappedEventArgs(::windows_core::IUnknown);
impl TappedEventArgs {
    pub fn PointerDeviceType(&self) -> ::windows_core::Result<PointerDeviceType> {
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
    pub fn TapCount(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TapCount)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for TappedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for TappedEventArgs {
    type Vtable = ITappedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for TappedEventArgs {
    const IID: ::windows_core::GUID = <ITappedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for TappedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Input.TappedEventArgs";
}
::windows_core::imp::interface_hierarchy!(
    TappedEventArgs,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for TappedEventArgs {}
unsafe impl ::core::marker::Sync for TappedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CrossSlidingState(pub i32);
impl CrossSlidingState {
    pub const Started: Self = Self(0i32);
    pub const Dragging: Self = Self(1i32);
    pub const Selecting: Self = Self(2i32);
    pub const SelectSpeedBumping: Self = Self(3i32);
    pub const SpeedBumping: Self = Self(4i32);
    pub const Rearranging: Self = Self(5i32);
    pub const Completed: Self = Self(6i32);
}
impl ::core::marker::Copy for CrossSlidingState {}
impl ::core::clone::Clone for CrossSlidingState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CrossSlidingState {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for CrossSlidingState {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for CrossSlidingState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CrossSlidingState").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for CrossSlidingState {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::from_slice(
            b"enum(Microsoft.UI.Input.CrossSlidingState;i4)",
        );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DraggingState(pub i32);
impl DraggingState {
    pub const Started: Self = Self(0i32);
    pub const Continuing: Self = Self(1i32);
    pub const Completed: Self = Self(2i32);
}
impl ::core::marker::Copy for DraggingState {}
impl ::core::clone::Clone for DraggingState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DraggingState {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for DraggingState {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for DraggingState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DraggingState").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for DraggingState {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::from_slice(b"enum(Microsoft.UI.Input.DraggingState;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct GestureSettings(pub u32);
impl GestureSettings {
    pub const None: Self = Self(0u32);
    pub const Tap: Self = Self(1u32);
    pub const DoubleTap: Self = Self(2u32);
    pub const Hold: Self = Self(4u32);
    pub const HoldWithMouse: Self = Self(8u32);
    pub const RightTap: Self = Self(16u32);
    pub const Drag: Self = Self(32u32);
    pub const ManipulationTranslateX: Self = Self(64u32);
    pub const ManipulationTranslateY: Self = Self(128u32);
    pub const ManipulationTranslateRailsX: Self = Self(256u32);
    pub const ManipulationTranslateRailsY: Self = Self(512u32);
    pub const ManipulationRotate: Self = Self(1024u32);
    pub const ManipulationScale: Self = Self(2048u32);
    pub const ManipulationTranslateInertia: Self = Self(4096u32);
    pub const ManipulationRotateInertia: Self = Self(8192u32);
    pub const ManipulationScaleInertia: Self = Self(16384u32);
    pub const CrossSlide: Self = Self(32768u32);
    pub const ManipulationMultipleFingerPanning: Self = Self(65536u32);
}
impl ::core::marker::Copy for GestureSettings {}
impl ::core::clone::Clone for GestureSettings {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GestureSettings {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for GestureSettings {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for GestureSettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GestureSettings").field(&self.0).finish()
    }
}
impl GestureSettings {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for GestureSettings {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for GestureSettings {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for GestureSettings {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for GestureSettings {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for GestureSettings {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::windows_core::RuntimeType for GestureSettings {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::from_slice(
            b"enum(Microsoft.UI.Input.GestureSettings;u4)",
        );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HoldingState(pub i32);
impl HoldingState {
    pub const Started: Self = Self(0i32);
    pub const Completed: Self = Self(1i32);
    pub const Canceled: Self = Self(2i32);
}
impl ::core::marker::Copy for HoldingState {}
impl ::core::clone::Clone for HoldingState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for HoldingState {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for HoldingState {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for HoldingState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HoldingState").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for HoldingState {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::from_slice(b"enum(Microsoft.UI.Input.HoldingState;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct InputActivationState(pub i32);
impl InputActivationState {
    pub const None: Self = Self(0i32);
    pub const Deactivated: Self = Self(1i32);
    pub const Activated: Self = Self(2i32);
}
impl ::core::marker::Copy for InputActivationState {}
impl ::core::clone::Clone for InputActivationState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for InputActivationState {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for InputActivationState {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for InputActivationState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InputActivationState").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for InputActivationState {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::from_slice(
            b"enum(Microsoft.UI.Input.InputActivationState;i4)",
        );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct InputPointerSourceDeviceKinds(pub u32);
impl InputPointerSourceDeviceKinds {
    pub const None: Self = Self(0u32);
    pub const Touch: Self = Self(1u32);
    pub const Pen: Self = Self(2u32);
    pub const Mouse: Self = Self(4u32);
}
impl ::core::marker::Copy for InputPointerSourceDeviceKinds {}
impl ::core::clone::Clone for InputPointerSourceDeviceKinds {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for InputPointerSourceDeviceKinds {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for InputPointerSourceDeviceKinds {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for InputPointerSourceDeviceKinds {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InputPointerSourceDeviceKinds").field(&self.0).finish()
    }
}
impl InputPointerSourceDeviceKinds {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for InputPointerSourceDeviceKinds {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for InputPointerSourceDeviceKinds {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for InputPointerSourceDeviceKinds {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for InputPointerSourceDeviceKinds {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for InputPointerSourceDeviceKinds {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::windows_core::RuntimeType for InputPointerSourceDeviceKinds {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::from_slice(
            b"enum(Microsoft.UI.Input.InputPointerSourceDeviceKinds;u4)",
        );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct InputSystemCursorShape(pub i32);
impl InputSystemCursorShape {
    pub const Arrow: Self = Self(0i32);
    pub const Cross: Self = Self(1i32);
    pub const Hand: Self = Self(3i32);
    pub const Help: Self = Self(4i32);
    pub const IBeam: Self = Self(5i32);
    pub const SizeAll: Self = Self(6i32);
    pub const SizeNortheastSouthwest: Self = Self(7i32);
    pub const SizeNorthSouth: Self = Self(8i32);
    pub const SizeNorthwestSoutheast: Self = Self(9i32);
    pub const SizeWestEast: Self = Self(10i32);
    pub const UniversalNo: Self = Self(11i32);
    pub const UpArrow: Self = Self(12i32);
    pub const Wait: Self = Self(13i32);
    pub const Pin: Self = Self(14i32);
    pub const Person: Self = Self(15i32);
    pub const AppStarting: Self = Self(16i32);
}
impl ::core::marker::Copy for InputSystemCursorShape {}
impl ::core::clone::Clone for InputSystemCursorShape {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for InputSystemCursorShape {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for InputSystemCursorShape {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for InputSystemCursorShape {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InputSystemCursorShape").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for InputSystemCursorShape {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::from_slice(
            b"enum(Microsoft.UI.Input.InputSystemCursorShape;i4)",
        );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NonClientRegionKind(pub i32);
impl NonClientRegionKind {
    pub const Close: Self = Self(0i32);
    pub const Maximize: Self = Self(1i32);
    pub const Minimize: Self = Self(2i32);
    pub const Icon: Self = Self(3i32);
    pub const Caption: Self = Self(4i32);
    pub const TopBorder: Self = Self(5i32);
    pub const LeftBorder: Self = Self(6i32);
    pub const BottomBorder: Self = Self(7i32);
    pub const RightBorder: Self = Self(8i32);
    pub const Passthrough: Self = Self(9i32);
}
impl ::core::marker::Copy for NonClientRegionKind {}
impl ::core::clone::Clone for NonClientRegionKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NonClientRegionKind {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for NonClientRegionKind {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for NonClientRegionKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NonClientRegionKind").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for NonClientRegionKind {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::from_slice(
            b"enum(Microsoft.UI.Input.NonClientRegionKind;i4)",
        );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PointerDeviceType(pub i32);
impl PointerDeviceType {
    pub const Touch: Self = Self(0i32);
    pub const Pen: Self = Self(1i32);
    pub const Mouse: Self = Self(2i32);
    pub const Touchpad: Self = Self(3i32);
}
impl ::core::marker::Copy for PointerDeviceType {}
impl ::core::clone::Clone for PointerDeviceType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PointerDeviceType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for PointerDeviceType {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for PointerDeviceType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PointerDeviceType").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for PointerDeviceType {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::from_slice(
            b"enum(Microsoft.UI.Input.PointerDeviceType;i4)",
        );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PointerUpdateKind(pub i32);
impl PointerUpdateKind {
    pub const Other: Self = Self(0i32);
    pub const LeftButtonPressed: Self = Self(1i32);
    pub const LeftButtonReleased: Self = Self(2i32);
    pub const RightButtonPressed: Self = Self(3i32);
    pub const RightButtonReleased: Self = Self(4i32);
    pub const MiddleButtonPressed: Self = Self(5i32);
    pub const MiddleButtonReleased: Self = Self(6i32);
    pub const XButton1Pressed: Self = Self(7i32);
    pub const XButton1Released: Self = Self(8i32);
    pub const XButton2Pressed: Self = Self(9i32);
    pub const XButton2Released: Self = Self(10i32);
}
impl ::core::marker::Copy for PointerUpdateKind {}
impl ::core::clone::Clone for PointerUpdateKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PointerUpdateKind {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for PointerUpdateKind {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for PointerUpdateKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PointerUpdateKind").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for PointerUpdateKind {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::from_slice(
            b"enum(Microsoft.UI.Input.PointerUpdateKind;i4)",
        );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VirtualKeyStates(pub u32);
impl VirtualKeyStates {
    pub const None: Self = Self(0u32);
    pub const Down: Self = Self(1u32);
    pub const Locked: Self = Self(2u32);
}
impl ::core::marker::Copy for VirtualKeyStates {}
impl ::core::clone::Clone for VirtualKeyStates {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VirtualKeyStates {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for VirtualKeyStates {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for VirtualKeyStates {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VirtualKeyStates").field(&self.0).finish()
    }
}
impl VirtualKeyStates {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for VirtualKeyStates {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for VirtualKeyStates {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for VirtualKeyStates {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for VirtualKeyStates {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for VirtualKeyStates {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::windows_core::RuntimeType for VirtualKeyStates {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::from_slice(
            b"enum(Microsoft.UI.Input.VirtualKeyStates;u4)",
        );
}
#[repr(C)]
pub struct CrossSlideThresholds {
    pub SelectionStart: f32,
    pub SpeedBumpStart: f32,
    pub SpeedBumpEnd: f32,
    pub RearrangeStart: f32,
}
impl ::core::marker::Copy for CrossSlideThresholds {}
impl ::core::clone::Clone for CrossSlideThresholds {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CrossSlideThresholds {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CrossSlideThresholds")
            .field("SelectionStart", &self.SelectionStart)
            .field("SpeedBumpStart", &self.SpeedBumpStart)
            .field("SpeedBumpEnd", &self.SpeedBumpEnd)
            .field("RearrangeStart", &self.RearrangeStart)
            .finish()
    }
}
impl ::windows_core::TypeKind for CrossSlideThresholds {
    type TypeKind = ::windows_core::CopyType;
}
impl ::windows_core::RuntimeType for CrossSlideThresholds {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::from_slice(
            b"struct(Microsoft.UI.Input.CrossSlideThresholds;f4;f4;f4;f4)",
        );
}
impl ::core::cmp::PartialEq for CrossSlideThresholds {
    fn eq(&self, other: &Self) -> bool {
        self.SelectionStart == other.SelectionStart
            && self.SpeedBumpStart == other.SpeedBumpStart
            && self.SpeedBumpEnd == other.SpeedBumpEnd
            && self.RearrangeStart == other.RearrangeStart
    }
}
impl ::core::cmp::Eq for CrossSlideThresholds {}
impl ::core::default::Default for CrossSlideThresholds {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Windows_Foundation\"`"]
#[cfg(feature = "Windows_Foundation")]
pub struct ManipulationDelta {
    pub Translation: ::windows::Foundation::Point,
    pub Scale: f32,
    pub Rotation: f32,
    pub Expansion: f32,
}
#[cfg(feature = "Windows_Foundation")]
impl ::core::marker::Copy for ManipulationDelta {}
#[cfg(feature = "Windows_Foundation")]
impl ::core::clone::Clone for ManipulationDelta {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Windows_Foundation")]
impl ::core::fmt::Debug for ManipulationDelta {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ManipulationDelta")
            .field("Translation", &self.Translation)
            .field("Scale", &self.Scale)
            .field("Rotation", &self.Rotation)
            .field("Expansion", &self.Expansion)
            .finish()
    }
}
#[cfg(feature = "Windows_Foundation")]
impl ::windows_core::TypeKind for ManipulationDelta {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Windows_Foundation")]
impl ::windows_core::RuntimeType for ManipulationDelta {
    const SIGNATURE : ::windows_core::imp::ConstBuffer =::windows_core::imp::ConstBuffer::from_slice ( b"struct(Microsoft.UI.Input.ManipulationDelta;struct(Windows.Foundation.Point;f4;f4);f4;f4;f4)" ) ;
}
#[cfg(feature = "Windows_Foundation")]
impl ::core::cmp::PartialEq for ManipulationDelta {
    fn eq(&self, other: &Self) -> bool {
        self.Translation == other.Translation
            && self.Scale == other.Scale
            && self.Rotation == other.Rotation
            && self.Expansion == other.Expansion
    }
}
#[cfg(feature = "Windows_Foundation")]
impl ::core::cmp::Eq for ManipulationDelta {}
#[cfg(feature = "Windows_Foundation")]
impl ::core::default::Default for ManipulationDelta {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Windows_Foundation\"`"]
#[cfg(feature = "Windows_Foundation")]
pub struct ManipulationVelocities {
    pub Linear: ::windows::Foundation::Point,
    pub Angular: f32,
    pub Expansion: f32,
}
#[cfg(feature = "Windows_Foundation")]
impl ::core::marker::Copy for ManipulationVelocities {}
#[cfg(feature = "Windows_Foundation")]
impl ::core::clone::Clone for ManipulationVelocities {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Windows_Foundation")]
impl ::core::fmt::Debug for ManipulationVelocities {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ManipulationVelocities")
            .field("Linear", &self.Linear)
            .field("Angular", &self.Angular)
            .field("Expansion", &self.Expansion)
            .finish()
    }
}
#[cfg(feature = "Windows_Foundation")]
impl ::windows_core::TypeKind for ManipulationVelocities {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Windows_Foundation")]
impl ::windows_core::RuntimeType for ManipulationVelocities {
    const SIGNATURE : ::windows_core::imp::ConstBuffer =::windows_core::imp::ConstBuffer::from_slice ( b"struct(Microsoft.UI.Input.ManipulationVelocities;struct(Windows.Foundation.Point;f4;f4);f4;f4)" ) ;
}
#[cfg(feature = "Windows_Foundation")]
impl ::core::cmp::PartialEq for ManipulationVelocities {
    fn eq(&self, other: &Self) -> bool {
        self.Linear == other.Linear
            && self.Angular == other.Angular
            && self.Expansion == other.Expansion
    }
}
#[cfg(feature = "Windows_Foundation")]
impl ::core::cmp::Eq for ManipulationVelocities {}
#[cfg(feature = "Windows_Foundation")]
impl ::core::default::Default for ManipulationVelocities {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct PhysicalKeyStatus {
    pub RepeatCount: u32,
    pub ScanCode: u32,
    pub IsExtendedKey: bool,
    pub IsMenuKeyDown: bool,
    pub WasKeyDown: bool,
    pub IsKeyReleased: bool,
}
impl ::core::marker::Copy for PhysicalKeyStatus {}
impl ::core::clone::Clone for PhysicalKeyStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PhysicalKeyStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PhysicalKeyStatus")
            .field("RepeatCount", &self.RepeatCount)
            .field("ScanCode", &self.ScanCode)
            .field("IsExtendedKey", &self.IsExtendedKey)
            .field("IsMenuKeyDown", &self.IsMenuKeyDown)
            .field("WasKeyDown", &self.WasKeyDown)
            .field("IsKeyReleased", &self.IsKeyReleased)
            .finish()
    }
}
impl ::windows_core::TypeKind for PhysicalKeyStatus {
    type TypeKind = ::windows_core::CopyType;
}
impl ::windows_core::RuntimeType for PhysicalKeyStatus {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::from_slice(
            b"struct(Microsoft.UI.Input.PhysicalKeyStatus;u4;u4;b1;b1;b1;b1)",
        );
}
impl ::core::cmp::PartialEq for PhysicalKeyStatus {
    fn eq(&self, other: &Self) -> bool {
        self.RepeatCount == other.RepeatCount
            && self.ScanCode == other.ScanCode
            && self.IsExtendedKey == other.IsExtendedKey
            && self.IsMenuKeyDown == other.IsMenuKeyDown
            && self.WasKeyDown == other.WasKeyDown
            && self.IsKeyReleased == other.IsKeyReleased
    }
}
impl ::core::cmp::Eq for PhysicalKeyStatus {}
impl ::core::default::Default for PhysicalKeyStatus {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
