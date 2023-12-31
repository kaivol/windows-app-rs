#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPowerManagerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPowerManagerStatics {
    type Vtable = IPowerManagerStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPowerManagerStatics {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xfa3554cc_be1c_534c_bff8_72df78e9f4a4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPowerManagerStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub EnergySaverStatus: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut EnergySaverStatus,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_Foundation")]
    pub EnergySaverStatusChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    EnergySaverStatusChanged: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub RemoveEnergySaverStatusChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RemoveEnergySaverStatusChanged: usize,
    pub BatteryStatus: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut BatteryStatus,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_Foundation")]
    pub BatteryStatusChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    BatteryStatusChanged: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub RemoveBatteryStatusChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RemoveBatteryStatusChanged: usize,
    pub PowerSupplyStatus: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut PowerSupplyStatus,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_Foundation")]
    pub PowerSupplyStatusChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    PowerSupplyStatusChanged: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub RemovePowerSupplyStatusChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RemovePowerSupplyStatusChanged: usize,
    pub RemainingChargePercent: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut i32,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_Foundation")]
    pub RemainingChargePercentChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RemainingChargePercentChanged: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub RemoveRemainingChargePercentChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    )
        -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RemoveRemainingChargePercentChanged: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub RemainingDischargeTime: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::TimeSpan,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RemainingDischargeTime: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub RemainingDischargeTimeChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RemainingDischargeTimeChanged: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub RemoveRemainingDischargeTimeChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    )
        -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RemoveRemainingDischargeTimeChanged: usize,
    pub PowerSourceKind: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut PowerSourceKind,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_Foundation")]
    pub PowerSourceKindChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    PowerSourceKindChanged: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub RemovePowerSourceKindChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RemovePowerSourceKindChanged: usize,
    pub DisplayStatus: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut DisplayStatus,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_Foundation")]
    pub DisplayStatusChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    DisplayStatusChanged: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub RemoveDisplayStatusChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RemoveDisplayStatusChanged: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub SystemIdleStatusChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    SystemIdleStatusChanged: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub RemoveSystemIdleStatusChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RemoveSystemIdleStatusChanged: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub EffectivePowerMode: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    EffectivePowerMode: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub EffectivePowerModeChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    EffectivePowerModeChanged: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub RemoveEffectivePowerModeChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RemoveEffectivePowerModeChanged: usize,
    pub UserPresenceStatus: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut UserPresenceStatus,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_Foundation")]
    pub UserPresenceStatusChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    UserPresenceStatusChanged: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub RemoveUserPresenceStatusChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RemoveUserPresenceStatusChanged: usize,
    pub SystemSuspendStatus: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut SystemSuspendStatus,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_Foundation")]
    pub SystemSuspendStatusChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    SystemSuspendStatusChanged: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub RemoveSystemSuspendStatusChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RemoveSystemSuspendStatusChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPowerManagerStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPowerManagerStatics2 {
    type Vtable = IPowerManagerStatics2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPowerManagerStatics2 {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x61f3cc25_65b4_5def_9c9b_990cef3b0833);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPowerManagerStatics2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub EffectivePowerMode2: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut EffectivePowerMode,
    ) -> ::windows_core::HRESULT,
}
pub struct PowerManager;
impl PowerManager {
    pub fn EnergySaverStatus() -> ::windows_core::Result<EnergySaverStatus> {
        Self::IPowerManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).EnergySaverStatus)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn EnergySaverStatusChanged<P0>(
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::EventHandler<::windows_core::IInspectable>,
        >,
    {
        Self::IPowerManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).EnergySaverStatusChanged)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveEnergySaverStatusChanged(
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        Self::IPowerManagerStatics(|this| unsafe {
            (::windows_core::Interface::vtable(this).RemoveEnergySaverStatusChanged)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        })
    }
    pub fn BatteryStatus() -> ::windows_core::Result<BatteryStatus> {
        Self::IPowerManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).BatteryStatus)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn BatteryStatusChanged<P0>(
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::EventHandler<::windows_core::IInspectable>,
        >,
    {
        Self::IPowerManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).BatteryStatusChanged)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveBatteryStatusChanged(
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        Self::IPowerManagerStatics(|this| unsafe {
            (::windows_core::Interface::vtable(this).RemoveBatteryStatusChanged)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        })
    }
    pub fn PowerSupplyStatus() -> ::windows_core::Result<PowerSupplyStatus> {
        Self::IPowerManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PowerSupplyStatus)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn PowerSupplyStatusChanged<P0>(
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::EventHandler<::windows_core::IInspectable>,
        >,
    {
        Self::IPowerManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PowerSupplyStatusChanged)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemovePowerSupplyStatusChanged(
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        Self::IPowerManagerStatics(|this| unsafe {
            (::windows_core::Interface::vtable(this).RemovePowerSupplyStatusChanged)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        })
    }
    pub fn RemainingChargePercent() -> ::windows_core::Result<i32> {
        Self::IPowerManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RemainingChargePercent)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemainingChargePercentChanged<P0>(
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::EventHandler<::windows_core::IInspectable>,
        >,
    {
        Self::IPowerManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RemainingChargePercentChanged)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveRemainingChargePercentChanged(
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        Self::IPowerManagerStatics(|this| unsafe {
            (::windows_core::Interface::vtable(this).RemoveRemainingChargePercentChanged)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        })
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemainingDischargeTime() -> ::windows_core::Result<::windows::Foundation::TimeSpan> {
        Self::IPowerManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RemainingDischargeTime)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemainingDischargeTimeChanged<P0>(
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::EventHandler<::windows_core::IInspectable>,
        >,
    {
        Self::IPowerManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RemainingDischargeTimeChanged)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveRemainingDischargeTimeChanged(
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        Self::IPowerManagerStatics(|this| unsafe {
            (::windows_core::Interface::vtable(this).RemoveRemainingDischargeTimeChanged)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        })
    }
    pub fn PowerSourceKind() -> ::windows_core::Result<PowerSourceKind> {
        Self::IPowerManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PowerSourceKind)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn PowerSourceKindChanged<P0>(
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::EventHandler<::windows_core::IInspectable>,
        >,
    {
        Self::IPowerManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PowerSourceKindChanged)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemovePowerSourceKindChanged(
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        Self::IPowerManagerStatics(|this| unsafe {
            (::windows_core::Interface::vtable(this).RemovePowerSourceKindChanged)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        })
    }
    pub fn DisplayStatus() -> ::windows_core::Result<DisplayStatus> {
        Self::IPowerManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DisplayStatus)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn DisplayStatusChanged<P0>(
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::EventHandler<::windows_core::IInspectable>,
        >,
    {
        Self::IPowerManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DisplayStatusChanged)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveDisplayStatusChanged(
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        Self::IPowerManagerStatics(|this| unsafe {
            (::windows_core::Interface::vtable(this).RemoveDisplayStatusChanged)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        })
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn SystemIdleStatusChanged<P0>(
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::EventHandler<::windows_core::IInspectable>,
        >,
    {
        Self::IPowerManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SystemIdleStatusChanged)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveSystemIdleStatusChanged(
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        Self::IPowerManagerStatics(|this| unsafe {
            (::windows_core::Interface::vtable(this).RemoveSystemIdleStatusChanged)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        })
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn EffectivePowerMode(
    ) -> ::windows_core::Result<::windows::Foundation::IAsyncOperation<EffectivePowerMode>> {
        Self::IPowerManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).EffectivePowerMode)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn EffectivePowerModeChanged<P0>(
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::EventHandler<::windows_core::IInspectable>,
        >,
    {
        Self::IPowerManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).EffectivePowerModeChanged)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveEffectivePowerModeChanged(
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        Self::IPowerManagerStatics(|this| unsafe {
            (::windows_core::Interface::vtable(this).RemoveEffectivePowerModeChanged)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        })
    }
    pub fn UserPresenceStatus() -> ::windows_core::Result<UserPresenceStatus> {
        Self::IPowerManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).UserPresenceStatus)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn UserPresenceStatusChanged<P0>(
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::EventHandler<::windows_core::IInspectable>,
        >,
    {
        Self::IPowerManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).UserPresenceStatusChanged)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveUserPresenceStatusChanged(
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        Self::IPowerManagerStatics(|this| unsafe {
            (::windows_core::Interface::vtable(this).RemoveUserPresenceStatusChanged)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        })
    }
    pub fn SystemSuspendStatus() -> ::windows_core::Result<SystemSuspendStatus> {
        Self::IPowerManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SystemSuspendStatus)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn SystemSuspendStatusChanged<P0>(
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::EventHandler<::windows_core::IInspectable>,
        >,
    {
        Self::IPowerManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SystemSuspendStatusChanged)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveSystemSuspendStatusChanged(
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        Self::IPowerManagerStatics(|this| unsafe {
            (::windows_core::Interface::vtable(this).RemoveSystemSuspendStatusChanged)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        })
    }
    pub fn EffectivePowerMode2() -> ::windows_core::Result<EffectivePowerMode> {
        Self::IPowerManagerStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).EffectivePowerMode2)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPowerManagerStatics<
        R,
        F: FnOnce(&IPowerManagerStatics) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<PowerManager, IPowerManagerStatics> =
            ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IPowerManagerStatics2<
        R,
        F: FnOnce(&IPowerManagerStatics2) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<PowerManager, IPowerManagerStatics2> =
            ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeName for PowerManager {
    const NAME: &'static str = "Microsoft.Windows.System.Power.PowerManager";
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct BatteryStatus(pub i32);
impl BatteryStatus {
    pub const NotPresent: Self = Self(0i32);
    pub const Discharging: Self = Self(1i32);
    pub const Idle: Self = Self(2i32);
    pub const Charging: Self = Self(3i32);
}
impl ::core::marker::Copy for BatteryStatus {}
impl ::core::clone::Clone for BatteryStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for BatteryStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for BatteryStatus {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for BatteryStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BatteryStatus").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for BatteryStatus {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::from_slice(
            b"enum(Microsoft.Windows.System.Power.BatteryStatus;i4)",
        );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DisplayStatus(pub i32);
impl DisplayStatus {
    pub const Off: Self = Self(0i32);
    pub const On: Self = Self(1i32);
    pub const Dimmed: Self = Self(2i32);
}
impl ::core::marker::Copy for DisplayStatus {}
impl ::core::clone::Clone for DisplayStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DisplayStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for DisplayStatus {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for DisplayStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DisplayStatus").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for DisplayStatus {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::from_slice(
            b"enum(Microsoft.Windows.System.Power.DisplayStatus;i4)",
        );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct EffectivePowerMode(pub i32);
impl EffectivePowerMode {
    pub const BatterySaver: Self = Self(0i32);
    pub const BetterBattery: Self = Self(1i32);
    pub const Balanced: Self = Self(2i32);
    pub const HighPerformance: Self = Self(3i32);
    pub const MaxPerformance: Self = Self(4i32);
    pub const GameMode: Self = Self(5i32);
    pub const MixedReality: Self = Self(6i32);
}
impl ::core::marker::Copy for EffectivePowerMode {}
impl ::core::clone::Clone for EffectivePowerMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EffectivePowerMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for EffectivePowerMode {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for EffectivePowerMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EffectivePowerMode").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for EffectivePowerMode {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::from_slice(
            b"enum(Microsoft.Windows.System.Power.EffectivePowerMode;i4)",
        );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct EnergySaverStatus(pub i32);
impl EnergySaverStatus {
    pub const Uninitialized: Self = Self(0i32);
    pub const Disabled: Self = Self(1i32);
    pub const Off: Self = Self(2i32);
    pub const On: Self = Self(3i32);
}
impl ::core::marker::Copy for EnergySaverStatus {}
impl ::core::clone::Clone for EnergySaverStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EnergySaverStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for EnergySaverStatus {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for EnergySaverStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EnergySaverStatus").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for EnergySaverStatus {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::from_slice(
            b"enum(Microsoft.Windows.System.Power.EnergySaverStatus;i4)",
        );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PowerSourceKind(pub i32);
impl PowerSourceKind {
    pub const AC: Self = Self(0i32);
    pub const DC: Self = Self(1i32);
}
impl ::core::marker::Copy for PowerSourceKind {}
impl ::core::clone::Clone for PowerSourceKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PowerSourceKind {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for PowerSourceKind {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for PowerSourceKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PowerSourceKind").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for PowerSourceKind {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::from_slice(
            b"enum(Microsoft.Windows.System.Power.PowerSourceKind;i4)",
        );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PowerSupplyStatus(pub i32);
impl PowerSupplyStatus {
    pub const NotPresent: Self = Self(0i32);
    pub const Inadequate: Self = Self(1i32);
    pub const Adequate: Self = Self(2i32);
}
impl ::core::marker::Copy for PowerSupplyStatus {}
impl ::core::clone::Clone for PowerSupplyStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PowerSupplyStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for PowerSupplyStatus {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for PowerSupplyStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PowerSupplyStatus").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for PowerSupplyStatus {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::from_slice(
            b"enum(Microsoft.Windows.System.Power.PowerSupplyStatus;i4)",
        );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SystemSuspendStatus(pub i32);
impl SystemSuspendStatus {
    pub const Uninitialized: Self = Self(0i32);
    pub const Entering: Self = Self(1i32);
    pub const AutoResume: Self = Self(2i32);
    pub const ManualResume: Self = Self(3i32);
}
impl ::core::marker::Copy for SystemSuspendStatus {}
impl ::core::clone::Clone for SystemSuspendStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SystemSuspendStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for SystemSuspendStatus {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for SystemSuspendStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SystemSuspendStatus").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for SystemSuspendStatus {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::from_slice(
            b"enum(Microsoft.Windows.System.Power.SystemSuspendStatus;i4)",
        );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct UserPresenceStatus(pub i32);
impl UserPresenceStatus {
    pub const Present: Self = Self(0i32);
    pub const Absent: Self = Self(1i32);
}
impl ::core::marker::Copy for UserPresenceStatus {}
impl ::core::clone::Clone for UserPresenceStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UserPresenceStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for UserPresenceStatus {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for UserPresenceStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserPresenceStatus").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for UserPresenceStatus {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::from_slice(
            b"enum(Microsoft.Windows.System.Power.UserPresenceStatus;i4)",
        );
}
