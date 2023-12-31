#[doc = "Required features: `\"Windows_Win32_Foundation\"`, `\"Windows_Win32_UI_WindowsAndMessaging\"`"]
#[cfg(all(
    feature = "Windows_Win32_Foundation",
    feature = "Windows_Win32_UI_WindowsAndMessaging"
))]
pub trait IDesktopWindowXamlSourceNative_Deprecated_Impl: Sized {
    fn AttachToWindow(
        &self,
        parentwnd: ::windows::Win32::Foundation::HWND,
    ) -> ::windows_core::Result<()>;
    fn WindowHandle(
        &self,
        hwnd: *mut ::windows::Win32::Foundation::HWND,
    ) -> ::windows_core::Result<()>;
    fn PreTranslateMessage(
        &self,
        message: *const ::windows::Win32::UI::WindowsAndMessaging::MSG,
        result: *mut ::windows::Win32::Foundation::BOOL,
    ) -> ::windows_core::Result<()>;
}
#[cfg(all(
    feature = "Windows_Win32_Foundation",
    feature = "Windows_Win32_UI_WindowsAndMessaging"
))]
impl ::windows_core::RuntimeName for IDesktopWindowXamlSourceNative_Deprecated {}
#[cfg(all(
    feature = "Windows_Win32_Foundation",
    feature = "Windows_Win32_UI_WindowsAndMessaging"
))]
impl IDesktopWindowXamlSourceNative_Deprecated_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IDesktopWindowXamlSourceNative_Deprecated_Impl,
        const OFFSET: isize,
    >() -> IDesktopWindowXamlSourceNative_Deprecated_Vtbl {
        unsafe extern "system" fn AttachToWindow<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IDesktopWindowXamlSourceNative_Deprecated_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            parentwnd: ::windows::Win32::Foundation::HWND,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AttachToWindow(::core::mem::transmute_copy(&parentwnd)).into()
        }
        unsafe extern "system" fn WindowHandle<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IDesktopWindowXamlSourceNative_Deprecated_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            hwnd: *mut ::windows::Win32::Foundation::HWND,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.WindowHandle(::core::mem::transmute_copy(&hwnd)).into()
        }
        unsafe extern "system" fn PreTranslateMessage<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IDesktopWindowXamlSourceNative_Deprecated_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            message: *const ::windows::Win32::UI::WindowsAndMessaging::MSG,
            result: *mut ::windows::Win32::Foundation::BOOL,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PreTranslateMessage(
                ::core::mem::transmute_copy(&message),
                ::core::mem::transmute_copy(&result),
            )
            .into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AttachToWindow: AttachToWindow::<Identity, Impl, OFFSET>,
            WindowHandle: WindowHandle::<Identity, Impl, OFFSET>,
            PreTranslateMessage: PreTranslateMessage::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IDesktopWindowXamlSourceNative_Deprecated as ::windows_core::ComInterface>::IID
    }
}
pub trait IFindReferenceTargetsCallback_Impl: Sized {
    fn FoundTrackerTarget(
        &self,
        target: ::core::option::Option<&IReferenceTrackerTarget>,
    ) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IFindReferenceTargetsCallback {}
impl IFindReferenceTargetsCallback_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IFindReferenceTargetsCallback_Impl,
        const OFFSET: isize,
    >() -> IFindReferenceTargetsCallback_Vtbl {
        unsafe extern "system" fn FoundTrackerTarget<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFindReferenceTargetsCallback_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            target: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.FoundTrackerTarget(::windows_core::from_raw_borrowed(&target)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            FoundTrackerTarget: FoundTrackerTarget::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IFindReferenceTargetsCallback as ::windows_core::ComInterface>::IID
    }
}
pub trait IReferenceTracker_Impl: Sized {
    fn ConnectFromTrackerSource(&self) -> ::windows_core::Result<()>;
    fn DisconnectFromTrackerSource(&self) -> ::windows_core::Result<()>;
    fn FindTrackerTargets(
        &self,
        callback: ::core::option::Option<&IFindReferenceTargetsCallback>,
    ) -> ::windows_core::Result<()>;
    fn GetReferenceTrackerManager(&self) -> ::windows_core::Result<IReferenceTrackerManager>;
    fn AddRefFromTrackerSource(&self) -> ::windows_core::Result<()>;
    fn ReleaseFromTrackerSource(&self) -> ::windows_core::Result<()>;
    fn PegFromTrackerSource(&self) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IReferenceTracker {}
impl IReferenceTracker_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IReferenceTracker_Impl,
        const OFFSET: isize,
    >() -> IReferenceTracker_Vtbl {
        unsafe extern "system" fn ConnectFromTrackerSource<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IReferenceTracker_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ConnectFromTrackerSource().into()
        }
        unsafe extern "system" fn DisconnectFromTrackerSource<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IReferenceTracker_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DisconnectFromTrackerSource().into()
        }
        unsafe extern "system" fn FindTrackerTargets<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IReferenceTracker_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            callback: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.FindTrackerTargets(::windows_core::from_raw_borrowed(&callback)).into()
        }
        unsafe extern "system" fn GetReferenceTrackerManager<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IReferenceTracker_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            value: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetReferenceTrackerManager() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddRefFromTrackerSource<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IReferenceTracker_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddRefFromTrackerSource().into()
        }
        unsafe extern "system" fn ReleaseFromTrackerSource<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IReferenceTracker_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReleaseFromTrackerSource().into()
        }
        unsafe extern "system" fn PegFromTrackerSource<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IReferenceTracker_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PegFromTrackerSource().into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ConnectFromTrackerSource: ConnectFromTrackerSource::<Identity, Impl, OFFSET>,
            DisconnectFromTrackerSource: DisconnectFromTrackerSource::<Identity, Impl, OFFSET>,
            FindTrackerTargets: FindTrackerTargets::<Identity, Impl, OFFSET>,
            GetReferenceTrackerManager: GetReferenceTrackerManager::<Identity, Impl, OFFSET>,
            AddRefFromTrackerSource: AddRefFromTrackerSource::<Identity, Impl, OFFSET>,
            ReleaseFromTrackerSource: ReleaseFromTrackerSource::<Identity, Impl, OFFSET>,
            PegFromTrackerSource: PegFromTrackerSource::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IReferenceTracker as ::windows_core::ComInterface>::IID
    }
}
pub trait IReferenceTrackerExtension_Impl: Sized {}
impl ::windows_core::RuntimeName for IReferenceTrackerExtension {}
impl IReferenceTrackerExtension_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IReferenceTrackerExtension_Impl,
        const OFFSET: isize,
    >() -> IReferenceTrackerExtension_Vtbl {
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IReferenceTrackerExtension as ::windows_core::ComInterface>::IID
    }
}
pub trait IReferenceTrackerHost_Impl: Sized {
    fn DisconnectUnusedReferenceSources(
        &self,
        options: __MIDL___MIDL_itf_microsoft2Eui2Examl2Ehosting2Ereferencetracker_0000_0004_0001,
    ) -> ::windows_core::Result<()>;
    fn ReleaseDisconnectedReferenceSources(&self) -> ::windows_core::Result<()>;
    fn NotifyEndOfReferenceTrackingOnThread(&self) -> ::windows_core::Result<()>;
    fn GetTrackerTarget(
        &self,
        unknown: ::core::option::Option<&::windows_core::IUnknown>,
    ) -> ::windows_core::Result<IReferenceTrackerTarget>;
    fn AddMemoryPressure(&self, bytesallocated: u64) -> ::windows_core::Result<()>;
    fn RemoveMemoryPressure(&self, bytesallocated: u64) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IReferenceTrackerHost {}
impl IReferenceTrackerHost_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IReferenceTrackerHost_Impl,
        const OFFSET: isize,
    >() -> IReferenceTrackerHost_Vtbl {
        unsafe extern "system" fn DisconnectUnusedReferenceSources<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IReferenceTrackerHost_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            options : __MIDL___MIDL_itf_microsoft2Eui2Examl2Ehosting2Ereferencetracker_0000_0004_0001,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DisconnectUnusedReferenceSources(::core::mem::transmute_copy(&options))
                .into()
        }
        unsafe extern "system" fn ReleaseDisconnectedReferenceSources<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IReferenceTrackerHost_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReleaseDisconnectedReferenceSources().into()
        }
        unsafe extern "system" fn NotifyEndOfReferenceTrackingOnThread<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IReferenceTrackerHost_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.NotifyEndOfReferenceTrackingOnThread().into()
        }
        unsafe extern "system" fn GetTrackerTarget<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IReferenceTrackerHost_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            unknown: *mut ::core::ffi::c_void,
            newreference: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetTrackerTarget(::windows_core::from_raw_borrowed(&unknown)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(newreference, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddMemoryPressure<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IReferenceTrackerHost_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            bytesallocated: u64,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddMemoryPressure(::core::mem::transmute_copy(&bytesallocated)).into()
        }
        unsafe extern "system" fn RemoveMemoryPressure<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IReferenceTrackerHost_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            bytesallocated: u64,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveMemoryPressure(::core::mem::transmute_copy(&bytesallocated)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            DisconnectUnusedReferenceSources: DisconnectUnusedReferenceSources::<
                Identity,
                Impl,
                OFFSET,
            >,
            ReleaseDisconnectedReferenceSources: ReleaseDisconnectedReferenceSources::<
                Identity,
                Impl,
                OFFSET,
            >,
            NotifyEndOfReferenceTrackingOnThread: NotifyEndOfReferenceTrackingOnThread::<
                Identity,
                Impl,
                OFFSET,
            >,
            GetTrackerTarget: GetTrackerTarget::<Identity, Impl, OFFSET>,
            AddMemoryPressure: AddMemoryPressure::<Identity, Impl, OFFSET>,
            RemoveMemoryPressure: RemoveMemoryPressure::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IReferenceTrackerHost as ::windows_core::ComInterface>::IID
    }
}
pub trait IReferenceTrackerManager_Impl: Sized {
    fn ReferenceTrackingStarted(&self) -> ::windows_core::Result<()>;
    fn FindTrackerTargetsCompleted(&self, findfailed: u8) -> ::windows_core::Result<()>;
    fn ReferenceTrackingCompleted(&self) -> ::windows_core::Result<()>;
    fn SetReferenceTrackerHost(
        &self,
        value: ::core::option::Option<&IReferenceTrackerHost>,
    ) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IReferenceTrackerManager {}
impl IReferenceTrackerManager_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IReferenceTrackerManager_Impl,
        const OFFSET: isize,
    >() -> IReferenceTrackerManager_Vtbl {
        unsafe extern "system" fn ReferenceTrackingStarted<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IReferenceTrackerManager_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReferenceTrackingStarted().into()
        }
        unsafe extern "system" fn FindTrackerTargetsCompleted<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IReferenceTrackerManager_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            findfailed: u8,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.FindTrackerTargetsCompleted(::core::mem::transmute_copy(&findfailed))
                .into()
        }
        unsafe extern "system" fn ReferenceTrackingCompleted<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IReferenceTrackerManager_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReferenceTrackingCompleted().into()
        }
        unsafe extern "system" fn SetReferenceTrackerHost<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IReferenceTrackerManager_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            value: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetReferenceTrackerHost(::windows_core::from_raw_borrowed(&value)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ReferenceTrackingStarted: ReferenceTrackingStarted::<Identity, Impl, OFFSET>,
            FindTrackerTargetsCompleted: FindTrackerTargetsCompleted::<Identity, Impl, OFFSET>,
            ReferenceTrackingCompleted: ReferenceTrackingCompleted::<Identity, Impl, OFFSET>,
            SetReferenceTrackerHost: SetReferenceTrackerHost::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IReferenceTrackerManager as ::windows_core::ComInterface>::IID
    }
}
pub trait IReferenceTrackerTarget_Impl: Sized {
    fn AddRefFromReferenceTracker(&self) -> u32;
    fn ReleaseFromReferenceTracker(&self) -> u32;
    fn Peg(&self) -> ::windows_core::Result<()>;
    fn Unpeg(&self) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IReferenceTrackerTarget {}
impl IReferenceTrackerTarget_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IReferenceTrackerTarget_Impl,
        const OFFSET: isize,
    >() -> IReferenceTrackerTarget_Vtbl {
        unsafe extern "system" fn AddRefFromReferenceTracker<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IReferenceTrackerTarget_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> u32 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddRefFromReferenceTracker()
        }
        unsafe extern "system" fn ReleaseFromReferenceTracker<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IReferenceTrackerTarget_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> u32 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReleaseFromReferenceTracker()
        }
        unsafe extern "system" fn Peg<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IReferenceTrackerTarget_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Peg().into()
        }
        unsafe extern "system" fn Unpeg<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IReferenceTrackerTarget_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Unpeg().into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddRefFromReferenceTracker: AddRefFromReferenceTracker::<Identity, Impl, OFFSET>,
            ReleaseFromReferenceTracker: ReleaseFromReferenceTracker::<Identity, Impl, OFFSET>,
            Peg: Peg::<Identity, Impl, OFFSET>,
            Unpeg: Unpeg::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IReferenceTrackerTarget as ::windows_core::ComInterface>::IID
    }
}
pub trait ITrackerOwner_Impl: Sized {
    fn CreateTrackerHandle(
        &self,
        returnvalue: *mut *mut TrackerHandle__,
    ) -> ::windows_core::Result<()>;
    fn DeleteTrackerHandle(&self, handle: *mut TrackerHandle__) -> ::windows_core::Result<()>;
    fn SetTrackerValue(
        &self,
        handle: *mut TrackerHandle__,
        value: ::core::option::Option<&::windows_core::IUnknown>,
    ) -> ::windows_core::Result<()>;
    fn TryGetSafeTrackerValue(
        &self,
        handle: *mut TrackerHandle__,
        returnvalue: *mut ::core::option::Option<::windows_core::IUnknown>,
    ) -> u8;
}
impl ::windows_core::RuntimeName for ITrackerOwner {}
impl ITrackerOwner_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: ITrackerOwner_Impl,
        const OFFSET: isize,
    >() -> ITrackerOwner_Vtbl {
        unsafe extern "system" fn CreateTrackerHandle<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: ITrackerOwner_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            returnvalue: *mut *mut TrackerHandle__,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateTrackerHandle(::core::mem::transmute_copy(&returnvalue)).into()
        }
        unsafe extern "system" fn DeleteTrackerHandle<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: ITrackerOwner_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            handle: *mut TrackerHandle__,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteTrackerHandle(::core::mem::transmute_copy(&handle)).into()
        }
        unsafe extern "system" fn SetTrackerValue<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: ITrackerOwner_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            handle: *mut TrackerHandle__,
            value: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetTrackerValue(
                ::core::mem::transmute_copy(&handle),
                ::windows_core::from_raw_borrowed(&value),
            )
            .into()
        }
        unsafe extern "system" fn TryGetSafeTrackerValue<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: ITrackerOwner_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            handle: *mut TrackerHandle__,
            returnvalue: *mut *mut ::core::ffi::c_void,
        ) -> u8 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.TryGetSafeTrackerValue(
                ::core::mem::transmute_copy(&handle),
                ::core::mem::transmute_copy(&returnvalue),
            )
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateTrackerHandle: CreateTrackerHandle::<Identity, Impl, OFFSET>,
            DeleteTrackerHandle: DeleteTrackerHandle::<Identity, Impl, OFFSET>,
            SetTrackerValue: SetTrackerValue::<Identity, Impl, OFFSET>,
            TryGetSafeTrackerValue: TryGetSafeTrackerValue::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <ITrackerOwner as ::windows_core::ComInterface>::IID
    }
}
