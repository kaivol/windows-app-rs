#[doc = "Required features: `\"Windows_Foundation\"`, `\"Windows_UI_Composition\"`, `\"Windows_UI_Core\"`"]
#[cfg(all(
    feature = "Windows_Foundation",
    feature = "Windows_UI_Composition",
    feature = "Windows_UI_Core"
))]
pub trait ISystemBackdropController_Impl: Sized + ::windows::Foundation::IClosable_Impl {
    fn SetTargetWithWindowId(
        &self,
        windowid: &super::super::WindowId,
        desktopwindowtarget: ::core::option::Option<&::windows::UI::Composition::CompositionTarget>,
    ) -> ::windows_core::Result<bool>;
    fn SetTargetWithCoreWindow(
        &self,
        corewindow: ::core::option::Option<&::windows::UI::Core::CoreWindow>,
        compositiontarget: ::core::option::Option<&::windows::UI::Composition::CompositionTarget>,
    ) -> ::windows_core::Result<bool>;
}
#[cfg(all(
    feature = "Windows_Foundation",
    feature = "Windows_UI_Composition",
    feature = "Windows_UI_Core"
))]
impl ::windows_core::RuntimeName for ISystemBackdropController {
    const NAME: &'static str = "Microsoft.UI.Composition.SystemBackdrops.ISystemBackdropController";
}
#[cfg(all(
    feature = "Windows_Foundation",
    feature = "Windows_UI_Composition",
    feature = "Windows_UI_Core"
))]
impl ISystemBackdropController_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: ISystemBackdropController_Impl,
        const OFFSET: isize,
    >() -> ISystemBackdropController_Vtbl {
        unsafe extern "system" fn SetTargetWithWindowId<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: ISystemBackdropController_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            windowid: super::super::WindowId,
            desktopwindowtarget: *mut ::core::ffi::c_void,
            result__: *mut bool,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SetTargetWithWindowId(
                ::core::mem::transmute(&windowid),
                ::windows_core::from_raw_borrowed(&desktopwindowtarget),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTargetWithCoreWindow<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: ISystemBackdropController_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            corewindow: *mut ::core::ffi::c_void,
            compositiontarget: *mut ::core::ffi::c_void,
            result__: *mut bool,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SetTargetWithCoreWindow(
                ::windows_core::from_raw_borrowed(&corewindow),
                ::windows_core::from_raw_borrowed(&compositiontarget),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<
                Identity,
                ISystemBackdropController,
                OFFSET,
            >(),
            SetTargetWithWindowId: SetTargetWithWindowId::<Identity, Impl, OFFSET>,
            SetTargetWithCoreWindow: SetTargetWithCoreWindow::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <ISystemBackdropController as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Windows_Foundation\"`, `\"Windows_UI_Composition\"`, `\"Windows_UI_Core\"`"]
#[cfg(all(
    feature = "Windows_Foundation",
    feature = "Windows_UI_Composition",
    feature = "Windows_UI_Core"
))]
pub trait ISystemBackdropControllerWithTargets_Impl:
    Sized + ::windows::Foundation::IClosable_Impl + ISystemBackdropController_Impl
{
    fn State(&self) -> ::windows_core::Result<SystemBackdropState>;
    fn AddSystemBackdropTarget(
        &self,
        systembackdroptarget: ::core::option::Option<&super::ICompositionSupportsSystemBackdrop>,
    ) -> ::windows_core::Result<bool>;
    fn RemoveAllSystemBackdropTargets(&self) -> ::windows_core::Result<()>;
    fn RemoveSystemBackdropTarget(
        &self,
        systembackdroptarget: ::core::option::Option<&super::ICompositionSupportsSystemBackdrop>,
    ) -> ::windows_core::Result<bool>;
    fn SetSystemBackdropConfiguration(
        &self,
        configuration: ::core::option::Option<&SystemBackdropConfiguration>,
    ) -> ::windows_core::Result<()>;
    fn StateChanged(
        &self,
        handler: ::core::option::Option<
            &::windows::Foundation::TypedEventHandler<
                ISystemBackdropControllerWithTargets,
                ::windows_core::IInspectable,
            >,
        >,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>;
    fn RemoveStateChanged(
        &self,
        token: &::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()>;
}
#[cfg(all(
    feature = "Windows_Foundation",
    feature = "Windows_UI_Composition",
    feature = "Windows_UI_Core"
))]
impl ::windows_core::RuntimeName for ISystemBackdropControllerWithTargets {
    const NAME: &'static str =
        "Microsoft.UI.Composition.SystemBackdrops.ISystemBackdropControllerWithTargets";
}
#[cfg(all(
    feature = "Windows_Foundation",
    feature = "Windows_UI_Composition",
    feature = "Windows_UI_Core"
))]
impl ISystemBackdropControllerWithTargets_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: ISystemBackdropControllerWithTargets_Impl,
        const OFFSET: isize,
    >() -> ISystemBackdropControllerWithTargets_Vtbl {
        unsafe extern "system" fn State<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: ISystemBackdropControllerWithTargets_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut SystemBackdropState,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.State() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddSystemBackdropTarget<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: ISystemBackdropControllerWithTargets_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            systembackdroptarget: *mut ::core::ffi::c_void,
            result__: *mut bool,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this
                .AddSystemBackdropTarget(::windows_core::from_raw_borrowed(&systembackdroptarget))
            {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAllSystemBackdropTargets<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: ISystemBackdropControllerWithTargets_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveAllSystemBackdropTargets().into()
        }
        unsafe extern "system" fn RemoveSystemBackdropTarget<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: ISystemBackdropControllerWithTargets_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            systembackdroptarget: *mut ::core::ffi::c_void,
            result__: *mut bool,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RemoveSystemBackdropTarget(::windows_core::from_raw_borrowed(
                &systembackdroptarget,
            )) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSystemBackdropConfiguration<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: ISystemBackdropControllerWithTargets_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            configuration: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSystemBackdropConfiguration(::windows_core::from_raw_borrowed(&configuration))
                .into()
        }
        unsafe extern "system" fn StateChanged<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: ISystemBackdropControllerWithTargets_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            handler: *mut ::core::ffi::c_void,
            result__: *mut ::windows::Foundation::EventRegistrationToken,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.StateChanged(::windows_core::from_raw_borrowed(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveStateChanged<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: ISystemBackdropControllerWithTargets_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            token: ::windows::Foundation::EventRegistrationToken,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveStateChanged(::core::mem::transmute(&token)).into()
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<
                Identity,
                ISystemBackdropControllerWithTargets,
                OFFSET,
            >(),
            State: State::<Identity, Impl, OFFSET>,
            AddSystemBackdropTarget: AddSystemBackdropTarget::<Identity, Impl, OFFSET>,
            RemoveAllSystemBackdropTargets: RemoveAllSystemBackdropTargets::<Identity, Impl, OFFSET>,
            RemoveSystemBackdropTarget: RemoveSystemBackdropTarget::<Identity, Impl, OFFSET>,
            SetSystemBackdropConfiguration: SetSystemBackdropConfiguration::<Identity, Impl, OFFSET>,
            StateChanged: StateChanged::<Identity, Impl, OFFSET>,
            RemoveStateChanged: RemoveStateChanged::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <ISystemBackdropControllerWithTargets as ::windows_core::ComInterface>::IID
    }
}
