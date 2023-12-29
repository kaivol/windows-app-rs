#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPenDeviceInteropStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPenDeviceInteropStatics {
    type Vtable = IPenDeviceInteropStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPenDeviceInteropStatics {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xc2a59f2a_e077_5d30_a1bd_cf84dd09ee39);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPenDeviceInteropStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Windows_Devices_Input")]
    pub FromPointerPoint: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        pointerpoint: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Devices_Input"))]
    FromPointerPoint: usize,
}
pub struct PenDeviceInterop;
impl PenDeviceInterop {
    #[doc = "Required features: `\"Windows_Devices_Input\"`"]
    #[cfg(feature = "Windows_Devices_Input")]
    pub fn FromPointerPoint<P0>(
        pointerpoint: P0,
    ) -> ::windows_core::Result<::windows::Devices::Input::PenDevice>
    where
        P0: ::windows_core::IntoParam<super::PointerPoint>,
    {
        Self::IPenDeviceInteropStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FromPointerPoint)(
                ::windows_core::Interface::as_raw(this),
                pointerpoint.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPenDeviceInteropStatics<
        R,
        F: FnOnce(&IPenDeviceInteropStatics) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<
            PenDeviceInterop,
            IPenDeviceInteropStatics,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeName for PenDeviceInterop {
    const NAME: &'static str = "Microsoft.UI.Input.Interop.PenDeviceInterop";
}
