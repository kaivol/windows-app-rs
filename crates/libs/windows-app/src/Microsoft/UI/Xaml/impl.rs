pub trait IApplicationOverrides_Impl: Sized {
    fn OnLaunched(
        &self,
        args: ::core::option::Option<&LaunchActivatedEventArgs>,
    ) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IApplicationOverrides {
    const NAME: &'static str = "Microsoft.UI.Xaml.IApplicationOverrides";
}
impl IApplicationOverrides_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IApplicationOverrides_Impl,
        const OFFSET: isize,
    >() -> IApplicationOverrides_Vtbl {
        unsafe extern "system" fn OnLaunched<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IApplicationOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            args: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnLaunched(::windows_core::from_raw_borrowed(&args)).into()
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<Identity, IApplicationOverrides, OFFSET>(
            ),
            OnLaunched: OnLaunched::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IApplicationOverrides as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Microsoft_UI_Xaml_Controls\"`"]
#[cfg(feature = "Microsoft_UI_Xaml_Controls")]
pub trait IDataTemplateExtension_Impl: Sized {
    fn ResetTemplate(&self) -> ::windows_core::Result<()>;
    fn ProcessBinding(&self, phase: u32) -> ::windows_core::Result<bool>;
    fn ProcessBindings(
        &self,
        arg: ::core::option::Option<&Controls::ContainerContentChangingEventArgs>,
    ) -> ::windows_core::Result<i32>;
}
#[cfg(feature = "Microsoft_UI_Xaml_Controls")]
impl ::windows_core::RuntimeName for IDataTemplateExtension {
    const NAME: &'static str = "Microsoft.UI.Xaml.IDataTemplateExtension";
}
#[cfg(feature = "Microsoft_UI_Xaml_Controls")]
impl IDataTemplateExtension_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IDataTemplateExtension_Impl,
        const OFFSET: isize,
    >() -> IDataTemplateExtension_Vtbl {
        unsafe extern "system" fn ResetTemplate<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IDataTemplateExtension_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ResetTemplate().into()
        }
        unsafe extern "system" fn ProcessBinding<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IDataTemplateExtension_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            phase: u32,
            result__: *mut bool,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ProcessBinding(phase) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProcessBindings<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IDataTemplateExtension_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            arg: *mut ::core::ffi::c_void,
            result__: *mut i32,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ProcessBindings(::windows_core::from_raw_borrowed(&arg)) {
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
                IDataTemplateExtension,
                OFFSET,
            >(),
            ResetTemplate: ResetTemplate::<Identity, Impl, OFFSET>,
            ProcessBinding: ProcessBinding::<Identity, Impl, OFFSET>,
            ProcessBindings: ProcessBindings::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IDataTemplateExtension as ::windows_core::ComInterface>::IID
    }
}
pub trait IElementFactory_Impl: Sized {
    fn GetElement(
        &self,
        args: ::core::option::Option<&ElementFactoryGetArgs>,
    ) -> ::windows_core::Result<UIElement>;
    fn RecycleElement(
        &self,
        args: ::core::option::Option<&ElementFactoryRecycleArgs>,
    ) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IElementFactory {
    const NAME: &'static str = "Microsoft.UI.Xaml.IElementFactory";
}
impl IElementFactory_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IElementFactory_Impl,
        const OFFSET: isize,
    >() -> IElementFactory_Vtbl {
        unsafe extern "system" fn GetElement<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IElementFactory_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            args: *mut ::core::ffi::c_void,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetElement(::windows_core::from_raw_borrowed(&args)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RecycleElement<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IElementFactory_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            args: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RecycleElement(::windows_core::from_raw_borrowed(&args)).into()
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<Identity, IElementFactory, OFFSET>(),
            GetElement: GetElement::<Identity, Impl, OFFSET>,
            RecycleElement: RecycleElement::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IElementFactory as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Windows_Foundation\"`"]
#[cfg(feature = "Windows_Foundation")]
pub trait IFrameworkElementOverrides_Impl: Sized {
    fn MeasureOverride(
        &self,
        availablesize: &::windows::Foundation::Size,
    ) -> ::windows_core::Result<::windows::Foundation::Size>;
    fn ArrangeOverride(
        &self,
        finalsize: &::windows::Foundation::Size,
    ) -> ::windows_core::Result<::windows::Foundation::Size>;
    fn OnApplyTemplate(&self) -> ::windows_core::Result<()>;
    fn GoToElementStateCore(
        &self,
        statename: &::windows_core::HSTRING,
        usetransitions: bool,
    ) -> ::windows_core::Result<bool>;
}
#[cfg(feature = "Windows_Foundation")]
impl ::windows_core::RuntimeName for IFrameworkElementOverrides {
    const NAME: &'static str = "Microsoft.UI.Xaml.IFrameworkElementOverrides";
}
#[cfg(feature = "Windows_Foundation")]
impl IFrameworkElementOverrides_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IFrameworkElementOverrides_Impl,
        const OFFSET: isize,
    >() -> IFrameworkElementOverrides_Vtbl {
        unsafe extern "system" fn MeasureOverride<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFrameworkElementOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            availablesize: ::windows::Foundation::Size,
            result__: *mut ::windows::Foundation::Size,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MeasureOverride(::core::mem::transmute(&availablesize)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ArrangeOverride<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFrameworkElementOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            finalsize: ::windows::Foundation::Size,
            result__: *mut ::windows::Foundation::Size,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ArrangeOverride(::core::mem::transmute(&finalsize)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnApplyTemplate<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFrameworkElementOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnApplyTemplate().into()
        }
        unsafe extern "system" fn GoToElementStateCore<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IFrameworkElementOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            statename: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
            usetransitions: bool,
            result__: *mut bool,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GoToElementStateCore(::core::mem::transmute(&statename), usetransitions) {
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
                IFrameworkElementOverrides,
                OFFSET,
            >(),
            MeasureOverride: MeasureOverride::<Identity, Impl, OFFSET>,
            ArrangeOverride: ArrangeOverride::<Identity, Impl, OFFSET>,
            OnApplyTemplate: OnApplyTemplate::<Identity, Impl, OFFSET>,
            GoToElementStateCore: GoToElementStateCore::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IFrameworkElementOverrides as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Microsoft_UI_Composition\"`, `\"Microsoft_UI_Xaml_Automation_Peers\"`, `\"Microsoft_UI_Xaml_Input\"`, `\"Windows_Foundation_Collections\"`"]
#[cfg(all(
    feature = "Microsoft_UI_Composition",
    feature = "Microsoft_UI_Xaml_Automation_Peers",
    feature = "Microsoft_UI_Xaml_Input",
    feature = "Windows_Foundation_Collections"
))]
pub trait IUIElementOverrides_Impl: Sized {
    fn OnCreateAutomationPeer(&self) -> ::windows_core::Result<Automation::Peers::AutomationPeer>;
    fn OnDisconnectVisualChildren(&self) -> ::windows_core::Result<()>;
    fn FindSubElementsForTouchTargeting(
        &self,
        point: &::windows::Foundation::Point,
        boundingrect: &::windows::Foundation::Rect,
    ) -> ::windows_core::Result<
        ::windows::Foundation::Collections::IIterable<
            ::windows::Foundation::Collections::IIterable<::windows::Foundation::Point>,
        >,
    >;
    fn GetChildrenInTabFocusOrder(
        &self,
    ) -> ::windows_core::Result<::windows::Foundation::Collections::IIterable<DependencyObject>>;
    fn OnKeyboardAcceleratorInvoked(
        &self,
        args: ::core::option::Option<&Input::KeyboardAcceleratorInvokedEventArgs>,
    ) -> ::windows_core::Result<()>;
    fn OnProcessKeyboardAccelerators(
        &self,
        args: ::core::option::Option<&Input::ProcessKeyboardAcceleratorEventArgs>,
    ) -> ::windows_core::Result<()>;
    fn OnBringIntoViewRequested(
        &self,
        e: ::core::option::Option<&BringIntoViewRequestedEventArgs>,
    ) -> ::windows_core::Result<()>;
    fn PopulatePropertyInfoOverride(
        &self,
        propertyname: &::windows_core::HSTRING,
        animationpropertyinfo: ::core::option::Option<&super::Composition::AnimationPropertyInfo>,
    ) -> ::windows_core::Result<()>;
}
#[cfg(all(
    feature = "Microsoft_UI_Composition",
    feature = "Microsoft_UI_Xaml_Automation_Peers",
    feature = "Microsoft_UI_Xaml_Input",
    feature = "Windows_Foundation_Collections"
))]
impl ::windows_core::RuntimeName for IUIElementOverrides {
    const NAME: &'static str = "Microsoft.UI.Xaml.IUIElementOverrides";
}
#[cfg(all(
    feature = "Microsoft_UI_Composition",
    feature = "Microsoft_UI_Xaml_Automation_Peers",
    feature = "Microsoft_UI_Xaml_Input",
    feature = "Windows_Foundation_Collections"
))]
impl IUIElementOverrides_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IUIElementOverrides_Impl,
        const OFFSET: isize,
    >() -> IUIElementOverrides_Vtbl {
        unsafe extern "system" fn OnCreateAutomationPeer<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IUIElementOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.OnCreateAutomationPeer() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnDisconnectVisualChildren<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IUIElementOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnDisconnectVisualChildren().into()
        }
        unsafe extern "system" fn FindSubElementsForTouchTargeting<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IUIElementOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            point: ::windows::Foundation::Point,
            boundingrect: ::windows::Foundation::Rect,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FindSubElementsForTouchTargeting(
                ::core::mem::transmute(&point),
                ::core::mem::transmute(&boundingrect),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetChildrenInTabFocusOrder<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IUIElementOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetChildrenInTabFocusOrder() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnKeyboardAcceleratorInvoked<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IUIElementOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            args: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnKeyboardAcceleratorInvoked(::windows_core::from_raw_borrowed(&args))
                .into()
        }
        unsafe extern "system" fn OnProcessKeyboardAccelerators<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IUIElementOverrides_Impl,
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
        unsafe extern "system" fn OnBringIntoViewRequested<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IUIElementOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            e: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnBringIntoViewRequested(::windows_core::from_raw_borrowed(&e)).into()
        }
        unsafe extern "system" fn PopulatePropertyInfoOverride<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IUIElementOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            propertyname: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
            animationpropertyinfo: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PopulatePropertyInfoOverride(
                ::core::mem::transmute(&propertyname),
                ::windows_core::from_raw_borrowed(&animationpropertyinfo),
            )
            .into()
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<Identity, IUIElementOverrides, OFFSET>(
            ),
            OnCreateAutomationPeer: OnCreateAutomationPeer::<Identity, Impl, OFFSET>,
            OnDisconnectVisualChildren: OnDisconnectVisualChildren::<Identity, Impl, OFFSET>,
            FindSubElementsForTouchTargeting: FindSubElementsForTouchTargeting::<
                Identity,
                Impl,
                OFFSET,
            >,
            GetChildrenInTabFocusOrder: GetChildrenInTabFocusOrder::<Identity, Impl, OFFSET>,
            OnKeyboardAcceleratorInvoked: OnKeyboardAcceleratorInvoked::<Identity, Impl, OFFSET>,
            OnProcessKeyboardAccelerators: OnProcessKeyboardAccelerators::<Identity, Impl, OFFSET>,
            OnBringIntoViewRequested: OnBringIntoViewRequested::<Identity, Impl, OFFSET>,
            PopulatePropertyInfoOverride: PopulatePropertyInfoOverride::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IUIElementOverrides as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Microsoft_UI_Xaml_Controls\"`"]
#[cfg(feature = "Microsoft_UI_Xaml_Controls")]
pub trait IVisualStateManagerOverrides_Impl: Sized {
    fn GoToStateCore(
        &self,
        control: ::core::option::Option<&Controls::Control>,
        templateroot: ::core::option::Option<&FrameworkElement>,
        statename: &::windows_core::HSTRING,
        group: ::core::option::Option<&VisualStateGroup>,
        state: ::core::option::Option<&VisualState>,
        usetransitions: bool,
    ) -> ::windows_core::Result<bool>;
}
#[cfg(feature = "Microsoft_UI_Xaml_Controls")]
impl ::windows_core::RuntimeName for IVisualStateManagerOverrides {
    const NAME: &'static str = "Microsoft.UI.Xaml.IVisualStateManagerOverrides";
}
#[cfg(feature = "Microsoft_UI_Xaml_Controls")]
impl IVisualStateManagerOverrides_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IVisualStateManagerOverrides_Impl,
        const OFFSET: isize,
    >() -> IVisualStateManagerOverrides_Vtbl {
        unsafe extern "system" fn GoToStateCore<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IVisualStateManagerOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            control: *mut ::core::ffi::c_void,
            templateroot: *mut ::core::ffi::c_void,
            statename: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
            group: *mut ::core::ffi::c_void,
            state: *mut ::core::ffi::c_void,
            usetransitions: bool,
            result__: *mut bool,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GoToStateCore(
                ::windows_core::from_raw_borrowed(&control),
                ::windows_core::from_raw_borrowed(&templateroot),
                ::core::mem::transmute(&statename),
                ::windows_core::from_raw_borrowed(&group),
                ::windows_core::from_raw_borrowed(&state),
                usetransitions,
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
                IVisualStateManagerOverrides,
                OFFSET,
            >(),
            GoToStateCore: GoToStateCore::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IVisualStateManagerOverrides as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Windows_Win32_Foundation\"`"]
#[cfg(feature = "Windows_Win32_Foundation")]
pub trait IWindowNative_Impl: Sized {
    fn WindowHandle(
        &self,
        hwnd: *mut ::windows::Win32::Foundation::HWND,
    ) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Windows_Win32_Foundation")]
impl ::windows_core::RuntimeName for IWindowNative {}
#[cfg(feature = "Windows_Win32_Foundation")]
impl IWindowNative_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IWindowNative_Impl,
        const OFFSET: isize,
    >() -> IWindowNative_Vtbl {
        unsafe extern "system" fn WindowHandle<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IWindowNative_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            hwnd: *mut ::windows::Win32::Foundation::HWND,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.WindowHandle(::core::mem::transmute_copy(&hwnd)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            WindowHandle: WindowHandle::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IWindowNative as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Windows_UI_Xaml_Interop\"`"]
#[cfg(feature = "Windows_UI_Xaml_Interop")]
pub trait IXamlServiceProvider_Impl: Sized {
    fn GetService(
        &self,
        r#type: &super::super::super::Windows::UI::Xaml::Interop::TypeName,
    ) -> ::windows_core::Result<::windows_core::IInspectable>;
}
#[cfg(feature = "Windows_UI_Xaml_Interop")]
impl ::windows_core::RuntimeName for IXamlServiceProvider {
    const NAME: &'static str = "Microsoft.UI.Xaml.IXamlServiceProvider";
}
#[cfg(feature = "Windows_UI_Xaml_Interop")]
impl IXamlServiceProvider_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IXamlServiceProvider_Impl,
        const OFFSET: isize,
    >() -> IXamlServiceProvider_Vtbl {
        unsafe extern "system" fn GetService<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IXamlServiceProvider_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            r#type: ::std::mem::MaybeUninit<
                super::super::super::Windows::UI::Xaml::Interop::TypeName,
            >,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetService(::core::mem::transmute(&r#type)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<Identity, IXamlServiceProvider, OFFSET>(
            ),
            GetService: GetService::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IXamlServiceProvider as ::windows_core::ComInterface>::IID
    }
}
