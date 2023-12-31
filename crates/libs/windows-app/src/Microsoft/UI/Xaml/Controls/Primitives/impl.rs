#[doc = "Required features: `\"Microsoft_UI_Xaml_Input\"`"]
#[cfg(feature = "Microsoft_UI_Xaml_Input")]
pub trait IFlyoutBaseOverrides_Impl: Sized {
    fn CreatePresenter(&self) -> ::windows_core::Result<super::Control>;
    fn OnProcessKeyboardAccelerators(
        &self,
        args: ::core::option::Option<&super::super::Input::ProcessKeyboardAcceleratorEventArgs>,
    ) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Microsoft_UI_Xaml_Input")]
impl ::windows_core::RuntimeName for IFlyoutBaseOverrides {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.Primitives.IFlyoutBaseOverrides";
}
#[cfg(feature = "Microsoft_UI_Xaml_Input")]
impl IFlyoutBaseOverrides_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IFlyoutBaseOverrides_Impl,
        const OFFSET: isize,
    >() -> IFlyoutBaseOverrides_Vtbl {
        unsafe extern "system" fn CreatePresenter<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFlyoutBaseOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreatePresenter() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnProcessKeyboardAccelerators<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFlyoutBaseOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            args: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnProcessKeyboardAccelerators(::windows_core::from_raw_borrowed(&args))
                .into()
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<Identity, IFlyoutBaseOverrides, OFFSET>(
            ),
            CreatePresenter: CreatePresenter::<Identity, Impl, OFFSET>,
            OnProcessKeyboardAccelerators: OnProcessKeyboardAccelerators::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IFlyoutBaseOverrides as ::windows_core::ComInterface>::IID
    }
}
pub trait IPickerFlyoutBaseOverrides_Impl: Sized {
    fn OnConfirmed(&self) -> ::windows_core::Result<()>;
    fn ShouldShowConfirmationButtons(&self) -> ::windows_core::Result<bool>;
}
impl ::windows_core::RuntimeName for IPickerFlyoutBaseOverrides {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.Primitives.IPickerFlyoutBaseOverrides";
}
impl IPickerFlyoutBaseOverrides_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IPickerFlyoutBaseOverrides_Impl,
        const OFFSET: isize,
    >() -> IPickerFlyoutBaseOverrides_Vtbl {
        unsafe extern "system" fn OnConfirmed<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IPickerFlyoutBaseOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnConfirmed().into()
        }
        unsafe extern "system" fn ShouldShowConfirmationButtons<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IPickerFlyoutBaseOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut bool,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ShouldShowConfirmationButtons() {
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
                IPickerFlyoutBaseOverrides,
                OFFSET,
            >(),
            OnConfirmed: OnConfirmed::<Identity, Impl, OFFSET>,
            ShouldShowConfirmationButtons: ShouldShowConfirmationButtons::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IPickerFlyoutBaseOverrides as ::windows_core::ComInterface>::IID
    }
}
pub trait IRangeBaseOverrides_Impl: Sized {
    fn OnMinimumChanged(&self, oldminimum: f64, newminimum: f64) -> ::windows_core::Result<()>;
    fn OnMaximumChanged(&self, oldmaximum: f64, newmaximum: f64) -> ::windows_core::Result<()>;
    fn OnValueChanged(&self, oldvalue: f64, newvalue: f64) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IRangeBaseOverrides {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.Primitives.IRangeBaseOverrides";
}
impl IRangeBaseOverrides_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IRangeBaseOverrides_Impl,
        const OFFSET: isize,
    >() -> IRangeBaseOverrides_Vtbl {
        unsafe extern "system" fn OnMinimumChanged<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IRangeBaseOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            oldminimum: f64,
            newminimum: f64,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnMinimumChanged(oldminimum, newminimum).into()
        }
        unsafe extern "system" fn OnMaximumChanged<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IRangeBaseOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            oldmaximum: f64,
            newmaximum: f64,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnMaximumChanged(oldmaximum, newmaximum).into()
        }
        unsafe extern "system" fn OnValueChanged<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IRangeBaseOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            oldvalue: f64,
            newvalue: f64,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnValueChanged(oldvalue, newvalue).into()
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<Identity, IRangeBaseOverrides, OFFSET>(
            ),
            OnMinimumChanged: OnMinimumChanged::<Identity, Impl, OFFSET>,
            OnMaximumChanged: OnMaximumChanged::<Identity, Impl, OFFSET>,
            OnValueChanged: OnValueChanged::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IRangeBaseOverrides as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Microsoft_UI_Composition\"`, `\"Windows_Foundation_Numerics\"`"]
#[cfg(all(
    feature = "Microsoft_UI_Composition",
    feature = "Windows_Foundation_Numerics"
))]
pub trait IScrollController_Impl: Sized {
    fn PanningInfo(&self) -> ::windows_core::Result<IScrollControllerPanningInfo>;
    fn CanScroll(&self) -> ::windows_core::Result<bool>;
    fn IsScrollingWithMouse(&self) -> ::windows_core::Result<bool>;
    fn SetIsScrollable(&self, isscrollable: bool) -> ::windows_core::Result<()>;
    fn SetValues(
        &self,
        minoffset: f64,
        maxoffset: f64,
        offset: f64,
        viewportlength: f64,
    ) -> ::windows_core::Result<()>;
    fn GetScrollAnimation(
        &self,
        correlationid: i32,
        startposition: &::windows::Foundation::Numerics::Vector2,
        endposition: &::windows::Foundation::Numerics::Vector2,
        defaultanimation: ::core::option::Option<
            &super::super::super::Composition::CompositionAnimation,
        >,
    ) -> ::windows_core::Result<super::super::super::Composition::CompositionAnimation>;
    fn NotifyRequestedScrollCompleted(&self, correlationid: i32) -> ::windows_core::Result<()>;
    fn CanScrollChanged(
        &self,
        handler: ::core::option::Option<
            &::windows::Foundation::TypedEventHandler<
                IScrollController,
                ::windows_core::IInspectable,
            >,
        >,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>;
    fn RemoveCanScrollChanged(
        &self,
        token: &::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()>;
    fn IsScrollingWithMouseChanged(
        &self,
        handler: ::core::option::Option<
            &::windows::Foundation::TypedEventHandler<
                IScrollController,
                ::windows_core::IInspectable,
            >,
        >,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>;
    fn RemoveIsScrollingWithMouseChanged(
        &self,
        token: &::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()>;
    fn ScrollToRequested(
        &self,
        handler: ::core::option::Option<
            &::windows::Foundation::TypedEventHandler<
                IScrollController,
                ScrollControllerScrollToRequestedEventArgs,
            >,
        >,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>;
    fn RemoveScrollToRequested(
        &self,
        token: &::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()>;
    fn ScrollByRequested(
        &self,
        handler: ::core::option::Option<
            &::windows::Foundation::TypedEventHandler<
                IScrollController,
                ScrollControllerScrollByRequestedEventArgs,
            >,
        >,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>;
    fn RemoveScrollByRequested(
        &self,
        token: &::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()>;
    fn AddScrollVelocityRequested(
        &self,
        handler: ::core::option::Option<
            &::windows::Foundation::TypedEventHandler<
                IScrollController,
                ScrollControllerAddScrollVelocityRequestedEventArgs,
            >,
        >,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>;
    fn RemoveAddScrollVelocityRequested(
        &self,
        token: &::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()>;
}
#[cfg(all(
    feature = "Microsoft_UI_Composition",
    feature = "Windows_Foundation_Numerics"
))]
impl ::windows_core::RuntimeName for IScrollController {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.Primitives.IScrollController";
}
#[cfg(all(
    feature = "Microsoft_UI_Composition",
    feature = "Windows_Foundation_Numerics"
))]
impl IScrollController_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IScrollController_Impl,
        const OFFSET: isize,
    >() -> IScrollController_Vtbl {
        unsafe extern "system" fn PanningInfo<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IScrollController_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PanningInfo() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanScroll<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IScrollController_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut bool,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CanScroll() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsScrollingWithMouse<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IScrollController_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut bool,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsScrollingWithMouse() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsScrollable<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IScrollController_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            isscrollable: bool,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetIsScrollable(isscrollable).into()
        }
        unsafe extern "system" fn SetValues<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IScrollController_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            minoffset: f64,
            maxoffset: f64,
            offset: f64,
            viewportlength: f64,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetValues(minoffset, maxoffset, offset, viewportlength).into()
        }
        unsafe extern "system" fn GetScrollAnimation<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IScrollController_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            correlationid: i32,
            startposition: ::windows::Foundation::Numerics::Vector2,
            endposition: ::windows::Foundation::Numerics::Vector2,
            defaultanimation: *mut ::core::ffi::c_void,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetScrollAnimation(
                correlationid,
                ::core::mem::transmute(&startposition),
                ::core::mem::transmute(&endposition),
                ::windows_core::from_raw_borrowed(&defaultanimation),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NotifyRequestedScrollCompleted<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IScrollController_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            correlationid: i32,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.NotifyRequestedScrollCompleted(correlationid).into()
        }
        unsafe extern "system" fn CanScrollChanged<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IScrollController_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            handler: *mut ::core::ffi::c_void,
            result__: *mut ::windows::Foundation::EventRegistrationToken,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CanScrollChanged(::windows_core::from_raw_borrowed(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveCanScrollChanged<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IScrollController_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            token: ::windows::Foundation::EventRegistrationToken,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveCanScrollChanged(::core::mem::transmute(&token)).into()
        }
        unsafe extern "system" fn IsScrollingWithMouseChanged<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IScrollController_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            handler: *mut ::core::ffi::c_void,
            result__: *mut ::windows::Foundation::EventRegistrationToken,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsScrollingWithMouseChanged(::windows_core::from_raw_borrowed(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveIsScrollingWithMouseChanged<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IScrollController_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            token: ::windows::Foundation::EventRegistrationToken,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveIsScrollingWithMouseChanged(::core::mem::transmute(&token)).into()
        }
        unsafe extern "system" fn ScrollToRequested<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IScrollController_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            handler: *mut ::core::ffi::c_void,
            result__: *mut ::windows::Foundation::EventRegistrationToken,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ScrollToRequested(::windows_core::from_raw_borrowed(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveScrollToRequested<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IScrollController_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            token: ::windows::Foundation::EventRegistrationToken,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveScrollToRequested(::core::mem::transmute(&token)).into()
        }
        unsafe extern "system" fn ScrollByRequested<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IScrollController_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            handler: *mut ::core::ffi::c_void,
            result__: *mut ::windows::Foundation::EventRegistrationToken,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ScrollByRequested(::windows_core::from_raw_borrowed(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveScrollByRequested<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IScrollController_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            token: ::windows::Foundation::EventRegistrationToken,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveScrollByRequested(::core::mem::transmute(&token)).into()
        }
        unsafe extern "system" fn AddScrollVelocityRequested<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IScrollController_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            handler: *mut ::core::ffi::c_void,
            result__: *mut ::windows::Foundation::EventRegistrationToken,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AddScrollVelocityRequested(::windows_core::from_raw_borrowed(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAddScrollVelocityRequested<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IScrollController_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            token: ::windows::Foundation::EventRegistrationToken,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveAddScrollVelocityRequested(::core::mem::transmute(&token)).into()
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<Identity, IScrollController, OFFSET>(),
            PanningInfo: PanningInfo::<Identity, Impl, OFFSET>,
            CanScroll: CanScroll::<Identity, Impl, OFFSET>,
            IsScrollingWithMouse: IsScrollingWithMouse::<Identity, Impl, OFFSET>,
            SetIsScrollable: SetIsScrollable::<Identity, Impl, OFFSET>,
            SetValues: SetValues::<Identity, Impl, OFFSET>,
            GetScrollAnimation: GetScrollAnimation::<Identity, Impl, OFFSET>,
            NotifyRequestedScrollCompleted: NotifyRequestedScrollCompleted::<Identity, Impl, OFFSET>,
            CanScrollChanged: CanScrollChanged::<Identity, Impl, OFFSET>,
            RemoveCanScrollChanged: RemoveCanScrollChanged::<Identity, Impl, OFFSET>,
            IsScrollingWithMouseChanged: IsScrollingWithMouseChanged::<Identity, Impl, OFFSET>,
            RemoveIsScrollingWithMouseChanged: RemoveIsScrollingWithMouseChanged::<
                Identity,
                Impl,
                OFFSET,
            >,
            ScrollToRequested: ScrollToRequested::<Identity, Impl, OFFSET>,
            RemoveScrollToRequested: RemoveScrollToRequested::<Identity, Impl, OFFSET>,
            ScrollByRequested: ScrollByRequested::<Identity, Impl, OFFSET>,
            RemoveScrollByRequested: RemoveScrollByRequested::<Identity, Impl, OFFSET>,
            AddScrollVelocityRequested: AddScrollVelocityRequested::<Identity, Impl, OFFSET>,
            RemoveAddScrollVelocityRequested: RemoveAddScrollVelocityRequested::<
                Identity,
                Impl,
                OFFSET,
            >,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IScrollController as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Microsoft_UI_Composition\"`, `\"Windows_Foundation\"`"]
#[cfg(all(feature = "Microsoft_UI_Composition", feature = "Windows_Foundation"))]
pub trait IScrollControllerPanningInfo_Impl: Sized {
    fn IsRailEnabled(&self) -> ::windows_core::Result<bool>;
    fn PanOrientation(&self) -> ::windows_core::Result<super::Orientation>;
    fn PanningElementAncestor(&self) -> ::windows_core::Result<super::super::UIElement>;
    fn SetPanningElementExpressionAnimationSources(
        &self,
        propertyset: ::core::option::Option<
            &super::super::super::Composition::CompositionPropertySet,
        >,
        minoffsetpropertyname: &::windows_core::HSTRING,
        maxoffsetpropertyname: &::windows_core::HSTRING,
        offsetpropertyname: &::windows_core::HSTRING,
        multiplierpropertyname: &::windows_core::HSTRING,
    ) -> ::windows_core::Result<()>;
    fn Changed(
        &self,
        handler: ::core::option::Option<
            &::windows::Foundation::TypedEventHandler<
                IScrollControllerPanningInfo,
                ::windows_core::IInspectable,
            >,
        >,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>;
    fn RemoveChanged(
        &self,
        token: &::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()>;
    fn PanRequested(
        &self,
        handler: ::core::option::Option<
            &::windows::Foundation::TypedEventHandler<
                IScrollControllerPanningInfo,
                ScrollControllerPanRequestedEventArgs,
            >,
        >,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>;
    fn RemovePanRequested(
        &self,
        token: &::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Microsoft_UI_Composition", feature = "Windows_Foundation"))]
impl ::windows_core::RuntimeName for IScrollControllerPanningInfo {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.Primitives.IScrollControllerPanningInfo";
}
#[cfg(all(feature = "Microsoft_UI_Composition", feature = "Windows_Foundation"))]
impl IScrollControllerPanningInfo_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IScrollControllerPanningInfo_Impl,
        const OFFSET: isize,
    >() -> IScrollControllerPanningInfo_Vtbl {
        unsafe extern "system" fn IsRailEnabled<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IScrollControllerPanningInfo_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut bool,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsRailEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PanOrientation<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IScrollControllerPanningInfo_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut super::Orientation,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PanOrientation() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PanningElementAncestor<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IScrollControllerPanningInfo_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PanningElementAncestor() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPanningElementExpressionAnimationSources<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IScrollControllerPanningInfo_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            propertyset: *mut ::core::ffi::c_void,
            minoffsetpropertyname: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
            maxoffsetpropertyname: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
            offsetpropertyname: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
            multiplierpropertyname: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPanningElementExpressionAnimationSources(
                ::windows_core::from_raw_borrowed(&propertyset),
                ::core::mem::transmute(&minoffsetpropertyname),
                ::core::mem::transmute(&maxoffsetpropertyname),
                ::core::mem::transmute(&offsetpropertyname),
                ::core::mem::transmute(&multiplierpropertyname),
            )
            .into()
        }
        unsafe extern "system" fn Changed<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IScrollControllerPanningInfo_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            handler: *mut ::core::ffi::c_void,
            result__: *mut ::windows::Foundation::EventRegistrationToken,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Changed(::windows_core::from_raw_borrowed(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveChanged<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IScrollControllerPanningInfo_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            token: ::windows::Foundation::EventRegistrationToken,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveChanged(::core::mem::transmute(&token)).into()
        }
        unsafe extern "system" fn PanRequested<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IScrollControllerPanningInfo_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            handler: *mut ::core::ffi::c_void,
            result__: *mut ::windows::Foundation::EventRegistrationToken,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PanRequested(::windows_core::from_raw_borrowed(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePanRequested<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IScrollControllerPanningInfo_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            token: ::windows::Foundation::EventRegistrationToken,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemovePanRequested(::core::mem::transmute(&token)).into()
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<
                Identity,
                IScrollControllerPanningInfo,
                OFFSET,
            >(),
            IsRailEnabled: IsRailEnabled::<Identity, Impl, OFFSET>,
            PanOrientation: PanOrientation::<Identity, Impl, OFFSET>,
            PanningElementAncestor: PanningElementAncestor::<Identity, Impl, OFFSET>,
            SetPanningElementExpressionAnimationSources:
                SetPanningElementExpressionAnimationSources::<Identity, Impl, OFFSET>,
            Changed: Changed::<Identity, Impl, OFFSET>,
            RemoveChanged: RemoveChanged::<Identity, Impl, OFFSET>,
            PanRequested: PanRequested::<Identity, Impl, OFFSET>,
            RemovePanRequested: RemovePanRequested::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IScrollControllerPanningInfo as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Windows_Foundation_Collections\"`"]
#[cfg(feature = "Windows_Foundation_Collections")]
pub trait IScrollSnapPointsInfo_Impl: Sized {
    fn AreHorizontalSnapPointsRegular(&self) -> ::windows_core::Result<bool>;
    fn AreVerticalSnapPointsRegular(&self) -> ::windows_core::Result<bool>;
    fn HorizontalSnapPointsChanged(
        &self,
        handler: ::core::option::Option<
            &::windows::Foundation::EventHandler<::windows_core::IInspectable>,
        >,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>;
    fn RemoveHorizontalSnapPointsChanged(
        &self,
        token: &::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()>;
    fn VerticalSnapPointsChanged(
        &self,
        handler: ::core::option::Option<
            &::windows::Foundation::EventHandler<::windows_core::IInspectable>,
        >,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>;
    fn RemoveVerticalSnapPointsChanged(
        &self,
        token: &::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()>;
    fn GetIrregularSnapPoints(
        &self,
        orientation: super::Orientation,
        alignment: SnapPointsAlignment,
    ) -> ::windows_core::Result<::windows::Foundation::Collections::IVectorView<f32>>;
    fn GetRegularSnapPoints(
        &self,
        orientation: super::Orientation,
        alignment: SnapPointsAlignment,
        offset: &mut f32,
    ) -> ::windows_core::Result<f32>;
}
#[cfg(feature = "Windows_Foundation_Collections")]
impl ::windows_core::RuntimeName for IScrollSnapPointsInfo {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.Primitives.IScrollSnapPointsInfo";
}
#[cfg(feature = "Windows_Foundation_Collections")]
impl IScrollSnapPointsInfo_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IScrollSnapPointsInfo_Impl,
        const OFFSET: isize,
    >() -> IScrollSnapPointsInfo_Vtbl {
        unsafe extern "system" fn AreHorizontalSnapPointsRegular<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IScrollSnapPointsInfo_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut bool,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AreHorizontalSnapPointsRegular() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AreVerticalSnapPointsRegular<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IScrollSnapPointsInfo_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut bool,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AreVerticalSnapPointsRegular() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HorizontalSnapPointsChanged<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IScrollSnapPointsInfo_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            handler: *mut ::core::ffi::c_void,
            result__: *mut ::windows::Foundation::EventRegistrationToken,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.HorizontalSnapPointsChanged(::windows_core::from_raw_borrowed(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveHorizontalSnapPointsChanged<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IScrollSnapPointsInfo_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            token: ::windows::Foundation::EventRegistrationToken,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveHorizontalSnapPointsChanged(::core::mem::transmute(&token)).into()
        }
        unsafe extern "system" fn VerticalSnapPointsChanged<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IScrollSnapPointsInfo_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            handler: *mut ::core::ffi::c_void,
            result__: *mut ::windows::Foundation::EventRegistrationToken,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.VerticalSnapPointsChanged(::windows_core::from_raw_borrowed(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveVerticalSnapPointsChanged<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IScrollSnapPointsInfo_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            token: ::windows::Foundation::EventRegistrationToken,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveVerticalSnapPointsChanged(::core::mem::transmute(&token)).into()
        }
        unsafe extern "system" fn GetIrregularSnapPoints<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IScrollSnapPointsInfo_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            orientation: super::Orientation,
            alignment: SnapPointsAlignment,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetIrregularSnapPoints(orientation, alignment) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRegularSnapPoints<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IScrollSnapPointsInfo_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            orientation: super::Orientation,
            alignment: SnapPointsAlignment,
            offset: *mut f32,
            result__: *mut f32,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetRegularSnapPoints(
                orientation,
                alignment,
                ::core::mem::transmute_copy(&offset),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<Identity, IScrollSnapPointsInfo, OFFSET>(
            ),
            AreHorizontalSnapPointsRegular: AreHorizontalSnapPointsRegular::<Identity, Impl, OFFSET>,
            AreVerticalSnapPointsRegular: AreVerticalSnapPointsRegular::<Identity, Impl, OFFSET>,
            HorizontalSnapPointsChanged: HorizontalSnapPointsChanged::<Identity, Impl, OFFSET>,
            RemoveHorizontalSnapPointsChanged: RemoveHorizontalSnapPointsChanged::<
                Identity,
                Impl,
                OFFSET,
            >,
            VerticalSnapPointsChanged: VerticalSnapPointsChanged::<Identity, Impl, OFFSET>,
            RemoveVerticalSnapPointsChanged: RemoveVerticalSnapPointsChanged::<
                Identity,
                Impl,
                OFFSET,
            >,
            GetIrregularSnapPoints: GetIrregularSnapPoints::<Identity, Impl, OFFSET>,
            GetRegularSnapPoints: GetRegularSnapPoints::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IScrollSnapPointsInfo as ::windows_core::ComInterface>::IID
    }
}
pub trait IToggleButtonOverrides_Impl: Sized {
    fn OnToggle(&self) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IToggleButtonOverrides {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.Primitives.IToggleButtonOverrides";
}
impl IToggleButtonOverrides_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IToggleButtonOverrides_Impl,
        const OFFSET: isize,
    >() -> IToggleButtonOverrides_Vtbl {
        unsafe extern "system" fn OnToggle<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IToggleButtonOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnToggle().into()
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<
                Identity,
                IToggleButtonOverrides,
                OFFSET,
            >(),
            OnToggle: OnToggle::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IToggleButtonOverrides as ::windows_core::ComInterface>::IID
    }
}
