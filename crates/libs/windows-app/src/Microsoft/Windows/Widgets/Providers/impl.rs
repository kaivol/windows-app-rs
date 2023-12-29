pub trait IWidgetManager_Impl: Sized {
    fn UpdateWidget(
        &self,
        widgetupdaterequestoptions: ::core::option::Option<&WidgetUpdateRequestOptions>,
    ) -> ::windows_core::Result<()>;
    fn GetWidgetIds(
        &self,
    ) -> ::windows_core::Result<::windows_core::Array<::windows_core::HSTRING>>;
    fn GetWidgetInfo(
        &self,
        widgetid: &::windows_core::HSTRING,
    ) -> ::windows_core::Result<WidgetInfo>;
    fn GetWidgetInfos(&self) -> ::windows_core::Result<::windows_core::Array<WidgetInfo>>;
    fn DeleteWidget(&self, widgetid: &::windows_core::HSTRING) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IWidgetManager {
    const NAME: &'static str = "Microsoft.Windows.Widgets.Providers.IWidgetManager";
}
impl IWidgetManager_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IWidgetManager_Impl,
        const OFFSET: isize,
    >() -> IWidgetManager_Vtbl {
        unsafe extern "system" fn UpdateWidget<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IWidgetManager_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            widgetupdaterequestoptions: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UpdateWidget(::windows_core::from_raw_borrowed(&widgetupdaterequestoptions))
                .into()
        }
        unsafe extern "system" fn GetWidgetIds<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IWidgetManager_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result_size__: *mut u32,
            result__: *mut *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetWidgetIds() {
                ::core::result::Result::Ok(ok__) => {
                    let (ok_data__, ok_data_len__) = ok__.into_abi();
                    ::core::ptr::write(result__, ok_data__);
                    ::core::ptr::write(result_size__, ok_data_len__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWidgetInfo<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IWidgetManager_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            widgetid: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetWidgetInfo(::core::mem::transmute(&widgetid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWidgetInfos<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IWidgetManager_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result_size__: *mut u32,
            result__: *mut *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetWidgetInfos() {
                ::core::result::Result::Ok(ok__) => {
                    let (ok_data__, ok_data_len__) = ok__.into_abi();
                    ::core::ptr::write(result__, ok_data__);
                    ::core::ptr::write(result_size__, ok_data_len__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteWidget<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IWidgetManager_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            widgetid: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteWidget(::core::mem::transmute(&widgetid)).into()
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<Identity, IWidgetManager, OFFSET>(),
            UpdateWidget: UpdateWidget::<Identity, Impl, OFFSET>,
            GetWidgetIds: GetWidgetIds::<Identity, Impl, OFFSET>,
            GetWidgetInfo: GetWidgetInfo::<Identity, Impl, OFFSET>,
            GetWidgetInfos: GetWidgetInfos::<Identity, Impl, OFFSET>,
            DeleteWidget: DeleteWidget::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IWidgetManager as ::windows_core::ComInterface>::IID
    }
}
pub trait IWidgetProvider_Impl: Sized {
    fn CreateWidget(
        &self,
        widgetcontext: ::core::option::Option<&WidgetContext>,
    ) -> ::windows_core::Result<()>;
    fn DeleteWidget(
        &self,
        widgetid: &::windows_core::HSTRING,
        customstate: &::windows_core::HSTRING,
    ) -> ::windows_core::Result<()>;
    fn OnActionInvoked(
        &self,
        actioninvokedargs: ::core::option::Option<&WidgetActionInvokedArgs>,
    ) -> ::windows_core::Result<()>;
    fn OnWidgetContextChanged(
        &self,
        contextchangedargs: ::core::option::Option<&WidgetContextChangedArgs>,
    ) -> ::windows_core::Result<()>;
    fn Activate(
        &self,
        widgetcontext: ::core::option::Option<&WidgetContext>,
    ) -> ::windows_core::Result<()>;
    fn Deactivate(&self, widgetid: &::windows_core::HSTRING) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IWidgetProvider {
    const NAME: &'static str = "Microsoft.Windows.Widgets.Providers.IWidgetProvider";
}
impl IWidgetProvider_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IWidgetProvider_Impl,
        const OFFSET: isize,
    >() -> IWidgetProvider_Vtbl {
        unsafe extern "system" fn CreateWidget<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IWidgetProvider_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            widgetcontext: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateWidget(::windows_core::from_raw_borrowed(&widgetcontext)).into()
        }
        unsafe extern "system" fn DeleteWidget<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IWidgetProvider_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            widgetid: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
            customstate: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteWidget(
                ::core::mem::transmute(&widgetid),
                ::core::mem::transmute(&customstate),
            )
            .into()
        }
        unsafe extern "system" fn OnActionInvoked<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IWidgetProvider_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            actioninvokedargs: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnActionInvoked(::windows_core::from_raw_borrowed(&actioninvokedargs))
                .into()
        }
        unsafe extern "system" fn OnWidgetContextChanged<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IWidgetProvider_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            contextchangedargs: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnWidgetContextChanged(::windows_core::from_raw_borrowed(&contextchangedargs))
                .into()
        }
        unsafe extern "system" fn Activate<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IWidgetProvider_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            widgetcontext: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Activate(::windows_core::from_raw_borrowed(&widgetcontext)).into()
        }
        unsafe extern "system" fn Deactivate<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IWidgetProvider_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            widgetid: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Deactivate(::core::mem::transmute(&widgetid)).into()
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<Identity, IWidgetProvider, OFFSET>(),
            CreateWidget: CreateWidget::<Identity, Impl, OFFSET>,
            DeleteWidget: DeleteWidget::<Identity, Impl, OFFSET>,
            OnActionInvoked: OnActionInvoked::<Identity, Impl, OFFSET>,
            OnWidgetContextChanged: OnWidgetContextChanged::<Identity, Impl, OFFSET>,
            Activate: Activate::<Identity, Impl, OFFSET>,
            Deactivate: Deactivate::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IWidgetProvider as ::windows_core::ComInterface>::IID
    }
}
pub trait IWidgetProvider2_Impl: Sized {
    fn OnCustomizationRequested(
        &self,
        customizationrequestedargs: ::core::option::Option<&WidgetCustomizationRequestedArgs>,
    ) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IWidgetProvider2 {
    const NAME: &'static str = "Microsoft.Windows.Widgets.Providers.IWidgetProvider2";
}
impl IWidgetProvider2_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IWidgetProvider2_Impl,
        const OFFSET: isize,
    >() -> IWidgetProvider2_Vtbl {
        unsafe extern "system" fn OnCustomizationRequested<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IWidgetProvider2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            customizationrequestedargs: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnCustomizationRequested(::windows_core::from_raw_borrowed(
                &customizationrequestedargs,
            ))
            .into()
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<Identity, IWidgetProvider2, OFFSET>(),
            OnCustomizationRequested: OnCustomizationRequested::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IWidgetProvider2 as ::windows_core::ComInterface>::IID
    }
}
pub trait IWidgetProviderAnalytics_Impl: Sized {
    fn OnAnalyticsInfoReported(
        &self,
        args: ::core::option::Option<&WidgetAnalyticsInfoReportedArgs>,
    ) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IWidgetProviderAnalytics {
    const NAME: &'static str = "Microsoft.Windows.Widgets.Providers.IWidgetProviderAnalytics";
}
impl IWidgetProviderAnalytics_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IWidgetProviderAnalytics_Impl,
        const OFFSET: isize,
    >() -> IWidgetProviderAnalytics_Vtbl {
        unsafe extern "system" fn OnAnalyticsInfoReported<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IWidgetProviderAnalytics_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            args: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnAnalyticsInfoReported(::windows_core::from_raw_borrowed(&args)).into()
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<
                Identity,
                IWidgetProviderAnalytics,
                OFFSET,
            >(),
            OnAnalyticsInfoReported: OnAnalyticsInfoReported::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IWidgetProviderAnalytics as ::windows_core::ComInterface>::IID
    }
}
pub trait IWidgetProviderErrors_Impl: Sized {
    fn OnErrorInfoReported(
        &self,
        args: ::core::option::Option<&WidgetErrorInfoReportedArgs>,
    ) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IWidgetProviderErrors {
    const NAME: &'static str = "Microsoft.Windows.Widgets.Providers.IWidgetProviderErrors";
}
impl IWidgetProviderErrors_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IWidgetProviderErrors_Impl,
        const OFFSET: isize,
    >() -> IWidgetProviderErrors_Vtbl {
        unsafe extern "system" fn OnErrorInfoReported<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IWidgetProviderErrors_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            args: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnErrorInfoReported(::windows_core::from_raw_borrowed(&args)).into()
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<Identity, IWidgetProviderErrors, OFFSET>(
            ),
            OnErrorInfoReported: OnErrorInfoReported::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IWidgetProviderErrors as ::windows_core::ComInterface>::IID
    }
}
