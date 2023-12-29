#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDispatcherExitDeferral(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDispatcherExitDeferral {
    type Vtable = IDispatcherExitDeferral_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDispatcherExitDeferral {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x910b5aac_3310_563e_8418_f3005579729e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDispatcherExitDeferral_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Complete:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDispatcherQueue(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDispatcherQueue {
    type Vtable = IDispatcherQueue_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDispatcherQueue {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xf6ebf8fa_be1c_5bf6_a467_73da28738ae8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDispatcherQueue_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateTimer: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub TryEnqueue: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        callback: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub TryEnqueueWithPriority: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        priority: DispatcherQueuePriority,
        callback: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_Foundation")]
    pub ShutdownStarting: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    ShutdownStarting: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub RemoveShutdownStarting: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RemoveShutdownStarting: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub ShutdownCompleted: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    ShutdownCompleted: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub RemoveShutdownCompleted: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RemoveShutdownCompleted: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDispatcherQueue2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDispatcherQueue2 {
    type Vtable = IDispatcherQueue2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDispatcherQueue2 {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x0cf48751_f1ac_59b8_ba52_6ce7a1444d6f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDispatcherQueue2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub HasThreadAccess: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDispatcherQueue3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDispatcherQueue3 {
    type Vtable = IDispatcherQueue3_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDispatcherQueue3 {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x14a7a175_5c27_5a35_b079_21960cf764a8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDispatcherQueue3_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub EnqueueEventLoopExit:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub EnsureSystemDispatcherQueue:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub RunEventLoop:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub RunEventLoopWithOptions: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        options: DispatcherRunOptions,
        deferral: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_Foundation")]
    pub FrameworkShutdownStarting: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    FrameworkShutdownStarting: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub RemoveFrameworkShutdownStarting: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RemoveFrameworkShutdownStarting: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub FrameworkShutdownCompleted: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    FrameworkShutdownCompleted: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub RemoveFrameworkShutdownCompleted: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RemoveFrameworkShutdownCompleted: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDispatcherQueueController(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDispatcherQueueController {
    type Vtable = IDispatcherQueueController_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDispatcherQueueController {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xbce8178d_2183_584c_9e5b_f9366f6ae484);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDispatcherQueueController_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub DispatcherQueue: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_Foundation")]
    pub ShutdownQueueAsync: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    ShutdownQueueAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDispatcherQueueController2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDispatcherQueueController2 {
    type Vtable = IDispatcherQueueController2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDispatcherQueueController2 {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x4c68ee2a_1cb1_5591_a3a2_9b590b8f8b9a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDispatcherQueueController2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ShutdownQueue:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDispatcherQueueControllerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDispatcherQueueControllerStatics {
    type Vtable = IDispatcherQueueControllerStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDispatcherQueueControllerStatics {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xf18d6145_722b_593d_bcf2_a61e713f0037);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDispatcherQueueControllerStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateOnDedicatedThread: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub CreateOnCurrentThread: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDispatcherQueueShutdownStartingEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDispatcherQueueShutdownStartingEventArgs {
    type Vtable = IDispatcherQueueShutdownStartingEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDispatcherQueueShutdownStartingEventArgs {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x32519be5_072b_5660_a70e_8835c9b8157d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDispatcherQueueShutdownStartingEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
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
pub struct IDispatcherQueueStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDispatcherQueueStatics {
    type Vtable = IDispatcherQueueStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDispatcherQueueStatics {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xcd3382ea_a455_5124_b63a_ca40d34ca23c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDispatcherQueueStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub GetForCurrentThread: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDispatcherQueueTimer(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDispatcherQueueTimer {
    type Vtable = IDispatcherQueueTimer_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDispatcherQueueTimer {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xad4d63fd_88fe_541f_ac11_bf2dc1ed2ce5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDispatcherQueueTimer_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Windows_Foundation")]
    pub Interval: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::TimeSpan,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    Interval: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub SetInterval: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::windows::Foundation::TimeSpan,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    SetInterval: usize,
    pub IsRunning: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub IsRepeating: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetIsRepeating: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_Foundation")]
    pub Tick: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    Tick: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub RemoveTick: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RemoveTick: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct DispatcherExitDeferral(::windows_core::IUnknown);
impl DispatcherExitDeferral {
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
            DispatcherExitDeferral,
            ::windows_core::imp::IGenericFactory,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
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
}
impl ::windows_core::RuntimeType for DispatcherExitDeferral {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for DispatcherExitDeferral {
    type Vtable = IDispatcherExitDeferral_Vtbl;
}
unsafe impl ::windows_core::ComInterface for DispatcherExitDeferral {
    const IID: ::windows_core::GUID =
        <IDispatcherExitDeferral as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for DispatcherExitDeferral {
    const NAME: &'static str = "Microsoft.UI.Dispatching.DispatcherExitDeferral";
}
::windows_core::imp::interface_hierarchy!(
    DispatcherExitDeferral,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for DispatcherExitDeferral {}
unsafe impl ::core::marker::Sync for DispatcherExitDeferral {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct DispatcherQueue(::windows_core::IUnknown);
impl DispatcherQueue {
    pub fn CreateTimer(&self) -> ::windows_core::Result<DispatcherQueueTimer> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateTimer)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn TryEnqueue<P0>(&self, callback: P0) -> ::windows_core::Result<bool>
    where
        P0: ::windows_core::IntoParam<DispatcherQueueHandler>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TryEnqueue)(
                ::windows_core::Interface::as_raw(this),
                callback.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn TryEnqueueWithPriority<P0>(
        &self,
        priority: DispatcherQueuePriority,
        callback: P0,
    ) -> ::windows_core::Result<bool>
    where
        P0: ::windows_core::IntoParam<DispatcherQueueHandler>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TryEnqueueWithPriority)(
                ::windows_core::Interface::as_raw(this),
                priority,
                callback.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn ShutdownStarting<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<
                DispatcherQueue,
                DispatcherQueueShutdownStartingEventArgs,
            >,
        >,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ShutdownStarting)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveShutdownStarting(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveShutdownStarting)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn ShutdownCompleted<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<DispatcherQueue, ::windows_core::IInspectable>,
        >,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ShutdownCompleted)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveShutdownCompleted(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveShutdownCompleted)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn HasThreadAccess(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IDispatcherQueue2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HasThreadAccess)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn EnqueueEventLoopExit(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IDispatcherQueue3>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).EnqueueEventLoopExit)(
                ::windows_core::Interface::as_raw(this),
            )
            .ok()
        }
    }
    pub fn EnsureSystemDispatcherQueue(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IDispatcherQueue3>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).EnsureSystemDispatcherQueue)(
                ::windows_core::Interface::as_raw(this),
            )
            .ok()
        }
    }
    pub fn RunEventLoop(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IDispatcherQueue3>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).RunEventLoop)(
                ::windows_core::Interface::as_raw(this),
            )
            .ok()
        }
    }
    pub fn RunEventLoopWithOptions<P0>(
        &self,
        options: DispatcherRunOptions,
        deferral: P0,
    ) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<DispatcherExitDeferral>,
    {
        let this = &::windows_core::ComInterface::cast::<IDispatcherQueue3>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).RunEventLoopWithOptions)(
                ::windows_core::Interface::as_raw(this),
                options,
                deferral.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn FrameworkShutdownStarting<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<
                DispatcherQueue,
                DispatcherQueueShutdownStartingEventArgs,
            >,
        >,
    {
        let this = &::windows_core::ComInterface::cast::<IDispatcherQueue3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FrameworkShutdownStarting)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveFrameworkShutdownStarting(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IDispatcherQueue3>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveFrameworkShutdownStarting)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn FrameworkShutdownCompleted<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<DispatcherQueue, ::windows_core::IInspectable>,
        >,
    {
        let this = &::windows_core::ComInterface::cast::<IDispatcherQueue3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FrameworkShutdownCompleted)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveFrameworkShutdownCompleted(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IDispatcherQueue3>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveFrameworkShutdownCompleted)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn GetForCurrentThread() -> ::windows_core::Result<DispatcherQueue> {
        Self::IDispatcherQueueStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetForCurrentThread)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IDispatcherQueueStatics<
        R,
        F: FnOnce(&IDispatcherQueueStatics) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<DispatcherQueue, IDispatcherQueueStatics> =
            ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for DispatcherQueue {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for DispatcherQueue {
    type Vtable = IDispatcherQueue_Vtbl;
}
unsafe impl ::windows_core::ComInterface for DispatcherQueue {
    const IID: ::windows_core::GUID = <IDispatcherQueue as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for DispatcherQueue {
    const NAME: &'static str = "Microsoft.UI.Dispatching.DispatcherQueue";
}
::windows_core::imp::interface_hierarchy!(
    DispatcherQueue,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for DispatcherQueue {}
unsafe impl ::core::marker::Sync for DispatcherQueue {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct DispatcherQueueController(::windows_core::IUnknown);
impl DispatcherQueueController {
    pub fn DispatcherQueue(&self) -> ::windows_core::Result<DispatcherQueue> {
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
    pub fn ShutdownQueueAsync(
        &self,
    ) -> ::windows_core::Result<::windows::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ShutdownQueueAsync)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn ShutdownQueue(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IDispatcherQueueController2>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).ShutdownQueue)(
                ::windows_core::Interface::as_raw(this),
            )
            .ok()
        }
    }
    pub fn CreateOnDedicatedThread() -> ::windows_core::Result<DispatcherQueueController> {
        Self::IDispatcherQueueControllerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateOnDedicatedThread)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn CreateOnCurrentThread() -> ::windows_core::Result<DispatcherQueueController> {
        Self::IDispatcherQueueControllerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateOnCurrentThread)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IDispatcherQueueControllerStatics<
        R,
        F: FnOnce(&IDispatcherQueueControllerStatics) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<
            DispatcherQueueController,
            IDispatcherQueueControllerStatics,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for DispatcherQueueController {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for DispatcherQueueController {
    type Vtable = IDispatcherQueueController_Vtbl;
}
unsafe impl ::windows_core::ComInterface for DispatcherQueueController {
    const IID: ::windows_core::GUID =
        <IDispatcherQueueController as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for DispatcherQueueController {
    const NAME: &'static str = "Microsoft.UI.Dispatching.DispatcherQueueController";
}
::windows_core::imp::interface_hierarchy!(
    DispatcherQueueController,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for DispatcherQueueController {}
unsafe impl ::core::marker::Sync for DispatcherQueueController {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct DispatcherQueueShutdownStartingEventArgs(::windows_core::IUnknown);
impl DispatcherQueueShutdownStartingEventArgs {
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
impl ::windows_core::RuntimeType for DispatcherQueueShutdownStartingEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for DispatcherQueueShutdownStartingEventArgs {
    type Vtable = IDispatcherQueueShutdownStartingEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for DispatcherQueueShutdownStartingEventArgs {
    const IID: ::windows_core::GUID =
        <IDispatcherQueueShutdownStartingEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for DispatcherQueueShutdownStartingEventArgs {
    const NAME: &'static str = "Microsoft.UI.Dispatching.DispatcherQueueShutdownStartingEventArgs";
}
::windows_core::imp::interface_hierarchy!(
    DispatcherQueueShutdownStartingEventArgs,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for DispatcherQueueShutdownStartingEventArgs {}
unsafe impl ::core::marker::Sync for DispatcherQueueShutdownStartingEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct DispatcherQueueTimer(::windows_core::IUnknown);
impl DispatcherQueueTimer {
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn Interval(&self) -> ::windows_core::Result<::windows::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Interval)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn SetInterval(
        &self,
        value: ::windows::Foundation::TimeSpan,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetInterval)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsRunning(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsRunning)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn IsRepeating(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsRepeating)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetIsRepeating(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetIsRepeating)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
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
    pub fn Tick<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<
                DispatcherQueueTimer,
                ::windows_core::IInspectable,
            >,
        >,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Tick)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveTick(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveTick)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
}
impl ::windows_core::RuntimeType for DispatcherQueueTimer {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for DispatcherQueueTimer {
    type Vtable = IDispatcherQueueTimer_Vtbl;
}
unsafe impl ::windows_core::ComInterface for DispatcherQueueTimer {
    const IID: ::windows_core::GUID = <IDispatcherQueueTimer as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for DispatcherQueueTimer {
    const NAME: &'static str = "Microsoft.UI.Dispatching.DispatcherQueueTimer";
}
::windows_core::imp::interface_hierarchy!(
    DispatcherQueueTimer,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for DispatcherQueueTimer {}
unsafe impl ::core::marker::Sync for DispatcherQueueTimer {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DispatcherQueuePriority(pub i32);
impl DispatcherQueuePriority {
    pub const Low: Self = Self(-10i32);
    pub const Normal: Self = Self(0i32);
    pub const High: Self = Self(10i32);
}
impl ::core::marker::Copy for DispatcherQueuePriority {}
impl ::core::clone::Clone for DispatcherQueuePriority {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DispatcherQueuePriority {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for DispatcherQueuePriority {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for DispatcherQueuePriority {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DispatcherQueuePriority").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for DispatcherQueuePriority {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::from_slice(
            b"enum(Microsoft.UI.Dispatching.DispatcherQueuePriority;i4)",
        );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DispatcherRunOptions(pub u32);
impl DispatcherRunOptions {
    pub const None: Self = Self(0u32);
    pub const ContinueOnQuit: Self = Self(1u32);
    pub const QuitOnlyLocalLoop: Self = Self(2u32);
}
impl ::core::marker::Copy for DispatcherRunOptions {}
impl ::core::clone::Clone for DispatcherRunOptions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DispatcherRunOptions {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for DispatcherRunOptions {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for DispatcherRunOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DispatcherRunOptions").field(&self.0).finish()
    }
}
impl DispatcherRunOptions {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for DispatcherRunOptions {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for DispatcherRunOptions {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for DispatcherRunOptions {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for DispatcherRunOptions {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for DispatcherRunOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::windows_core::RuntimeType for DispatcherRunOptions {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::from_slice(
            b"enum(Microsoft.UI.Dispatching.DispatcherRunOptions;u4)",
        );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct DispatcherQueueHandler(pub ::windows_core::IUnknown);
impl DispatcherQueueHandler {
    pub fn new<F: FnMut() -> ::windows_core::Result<()> + ::core::marker::Send + 'static>(
        invoke: F,
    ) -> Self {
        let com = DispatcherQueueHandlerBox::<F> {
            vtable: &DispatcherQueueHandlerBox::<F>::VTABLE,
            count: ::windows_core::imp::RefCount::new(1),
            invoke,
        };
        unsafe { ::core::mem::transmute(::std::boxed::Box::new(com)) }
    }
    pub fn Invoke(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).Invoke)(::windows_core::Interface::as_raw(
                this,
            ))
            .ok()
        }
    }
}
#[repr(C)]
struct DispatcherQueueHandlerBox<
    F: FnMut() -> ::windows_core::Result<()> + ::core::marker::Send + 'static,
> {
    vtable: *const DispatcherQueueHandler_Vtbl,
    invoke: F,
    count: ::windows_core::imp::RefCount,
}
impl<F: FnMut() -> ::windows_core::Result<()> + ::core::marker::Send + 'static>
    DispatcherQueueHandlerBox<F>
{
    const VTABLE: DispatcherQueueHandler_Vtbl = DispatcherQueueHandler_Vtbl {
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
        *interface = if *iid == <DispatcherQueueHandler as ::windows_core::ComInterface>::IID
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
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        ((*this).invoke)().into()
    }
}
unsafe impl ::windows_core::Interface for DispatcherQueueHandler {
    type Vtable = DispatcherQueueHandler_Vtbl;
}
unsafe impl ::windows_core::ComInterface for DispatcherQueueHandler {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x2e0872a9_4e29_5f14_b688_fb96d5f9d5f8);
}
impl ::windows_core::RuntimeType for DispatcherQueueHandler {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct DispatcherQueueHandler_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Invoke:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
