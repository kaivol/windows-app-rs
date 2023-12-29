#[doc = "Required features: `\"Microsoft_UI_Composition\"`, `\"Windows_Foundation_Numerics\"`"]
#[cfg(all(
    feature = "Microsoft_UI_Composition",
    feature = "Windows_Foundation_Numerics"
))]
pub trait IAnimatedVisual_Impl: Sized + ::windows::Foundation::IClosable_Impl {
    fn RootVisual(&self) -> ::windows_core::Result<super::super::Composition::Visual>;
    fn Size(&self) -> ::windows_core::Result<::windows::Foundation::Numerics::Vector2>;
    fn Duration(&self) -> ::windows_core::Result<::windows::Foundation::TimeSpan>;
}
#[cfg(all(
    feature = "Microsoft_UI_Composition",
    feature = "Windows_Foundation_Numerics"
))]
impl ::windows_core::RuntimeName for IAnimatedVisual {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.IAnimatedVisual";
}
#[cfg(all(
    feature = "Microsoft_UI_Composition",
    feature = "Windows_Foundation_Numerics"
))]
impl IAnimatedVisual_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IAnimatedVisual_Impl,
        const OFFSET: isize,
    >() -> IAnimatedVisual_Vtbl {
        unsafe extern "system" fn RootVisual<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IAnimatedVisual_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RootVisual() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Size<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IAnimatedVisual_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::Foundation::Numerics::Vector2,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Size() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Duration<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IAnimatedVisual_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::Foundation::TimeSpan,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Duration() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<Identity, IAnimatedVisual, OFFSET>(),
            RootVisual: RootVisual::<Identity, Impl, OFFSET>,
            Size: Size::<Identity, Impl, OFFSET>,
            Duration: Duration::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IAnimatedVisual as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Microsoft_UI_Composition\"`, `\"Windows_Foundation_Numerics\"`"]
#[cfg(all(
    feature = "Microsoft_UI_Composition",
    feature = "Windows_Foundation_Numerics"
))]
pub trait IAnimatedVisual2_Impl:
    Sized + IAnimatedVisual_Impl + ::windows::Foundation::IClosable_Impl
{
    fn CreateAnimations(&self) -> ::windows_core::Result<()>;
    fn DestroyAnimations(&self) -> ::windows_core::Result<()>;
}
#[cfg(all(
    feature = "Microsoft_UI_Composition",
    feature = "Windows_Foundation_Numerics"
))]
impl ::windows_core::RuntimeName for IAnimatedVisual2 {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.IAnimatedVisual2";
}
#[cfg(all(
    feature = "Microsoft_UI_Composition",
    feature = "Windows_Foundation_Numerics"
))]
impl IAnimatedVisual2_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IAnimatedVisual2_Impl,
        const OFFSET: isize,
    >() -> IAnimatedVisual2_Vtbl {
        unsafe extern "system" fn CreateAnimations<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IAnimatedVisual2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateAnimations().into()
        }
        unsafe extern "system" fn DestroyAnimations<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IAnimatedVisual2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DestroyAnimations().into()
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<Identity, IAnimatedVisual2, OFFSET>(),
            CreateAnimations: CreateAnimations::<Identity, Impl, OFFSET>,
            DestroyAnimations: DestroyAnimations::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IAnimatedVisual2 as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Microsoft_UI_Composition\"`"]
#[cfg(feature = "Microsoft_UI_Composition")]
pub trait IAnimatedVisualSource_Impl: Sized {
    fn TryCreateAnimatedVisual(
        &self,
        compositor: ::core::option::Option<&super::super::Composition::Compositor>,
        diagnostics: &mut ::core::option::Option<::windows_core::IInspectable>,
    ) -> ::windows_core::Result<IAnimatedVisual>;
}
#[cfg(feature = "Microsoft_UI_Composition")]
impl ::windows_core::RuntimeName for IAnimatedVisualSource {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.IAnimatedVisualSource";
}
#[cfg(feature = "Microsoft_UI_Composition")]
impl IAnimatedVisualSource_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IAnimatedVisualSource_Impl,
        const OFFSET: isize,
    >() -> IAnimatedVisualSource_Vtbl {
        unsafe extern "system" fn TryCreateAnimatedVisual<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IAnimatedVisualSource_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            compositor: *mut ::core::ffi::c_void,
            diagnostics: *mut *mut ::core::ffi::c_void,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.TryCreateAnimatedVisual(
                ::windows_core::from_raw_borrowed(&compositor),
                ::core::mem::transmute_copy(&diagnostics),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<Identity, IAnimatedVisualSource, OFFSET>(
            ),
            TryCreateAnimatedVisual: TryCreateAnimatedVisual::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IAnimatedVisualSource as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Microsoft_UI_Composition\"`, `\"Windows_Foundation_Collections\"`, `\"Windows_UI\"`"]
#[cfg(all(
    feature = "Microsoft_UI_Composition",
    feature = "Windows_Foundation_Collections",
    feature = "Windows_UI"
))]
pub trait IAnimatedVisualSource2_Impl: Sized + IAnimatedVisualSource_Impl {
    fn Markers(
        &self,
    ) -> ::windows_core::Result<
        ::windows::Foundation::Collections::IMapView<::windows_core::HSTRING, f64>,
    >;
    fn SetColorProperty(
        &self,
        propertyname: &::windows_core::HSTRING,
        value: &::windows::UI::Color,
    ) -> ::windows_core::Result<()>;
}
#[cfg(all(
    feature = "Microsoft_UI_Composition",
    feature = "Windows_Foundation_Collections",
    feature = "Windows_UI"
))]
impl ::windows_core::RuntimeName for IAnimatedVisualSource2 {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.IAnimatedVisualSource2";
}
#[cfg(all(
    feature = "Microsoft_UI_Composition",
    feature = "Windows_Foundation_Collections",
    feature = "Windows_UI"
))]
impl IAnimatedVisualSource2_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IAnimatedVisualSource2_Impl,
        const OFFSET: isize,
    >() -> IAnimatedVisualSource2_Vtbl {
        unsafe extern "system" fn Markers<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IAnimatedVisualSource2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Markers() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetColorProperty<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IAnimatedVisualSource2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            propertyname: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
            value: ::windows::UI::Color,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetColorProperty(
                ::core::mem::transmute(&propertyname),
                ::core::mem::transmute(&value),
            )
            .into()
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<
                Identity,
                IAnimatedVisualSource2,
                OFFSET,
            >(),
            Markers: Markers::<Identity, Impl, OFFSET>,
            SetColorProperty: SetColorProperty::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IAnimatedVisualSource2 as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Microsoft_UI_Composition\"`"]
#[cfg(feature = "Microsoft_UI_Composition")]
pub trait IAnimatedVisualSource3_Impl: Sized {
    fn TryCreateAnimatedVisual(
        &self,
        compositor: ::core::option::Option<&super::super::Composition::Compositor>,
        diagnostics: &mut ::core::option::Option<::windows_core::IInspectable>,
        createanimations: bool,
    ) -> ::windows_core::Result<IAnimatedVisual2>;
}
#[cfg(feature = "Microsoft_UI_Composition")]
impl ::windows_core::RuntimeName for IAnimatedVisualSource3 {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.IAnimatedVisualSource3";
}
#[cfg(feature = "Microsoft_UI_Composition")]
impl IAnimatedVisualSource3_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IAnimatedVisualSource3_Impl,
        const OFFSET: isize,
    >() -> IAnimatedVisualSource3_Vtbl {
        unsafe extern "system" fn TryCreateAnimatedVisual<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IAnimatedVisualSource3_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            compositor: *mut ::core::ffi::c_void,
            diagnostics: *mut *mut ::core::ffi::c_void,
            createanimations: bool,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.TryCreateAnimatedVisual(
                ::windows_core::from_raw_borrowed(&compositor),
                ::core::mem::transmute_copy(&diagnostics),
                createanimations,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<
                Identity,
                IAnimatedVisualSource3,
                OFFSET,
            >(),
            TryCreateAnimatedVisual: TryCreateAnimatedVisual::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IAnimatedVisualSource3 as ::windows_core::ComInterface>::IID
    }
}
pub trait IAppBarOverrides_Impl: Sized {
    fn OnClosed(
        &self,
        e: ::core::option::Option<&::windows_core::IInspectable>,
    ) -> ::windows_core::Result<()>;
    fn OnOpened(
        &self,
        e: ::core::option::Option<&::windows_core::IInspectable>,
    ) -> ::windows_core::Result<()>;
    fn OnClosing(
        &self,
        e: ::core::option::Option<&::windows_core::IInspectable>,
    ) -> ::windows_core::Result<()>;
    fn OnOpening(
        &self,
        e: ::core::option::Option<&::windows_core::IInspectable>,
    ) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IAppBarOverrides {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.IAppBarOverrides";
}
impl IAppBarOverrides_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IAppBarOverrides_Impl,
        const OFFSET: isize,
    >() -> IAppBarOverrides_Vtbl {
        unsafe extern "system" fn OnClosed<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IAppBarOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            e: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnClosed(::windows_core::from_raw_borrowed(&e)).into()
        }
        unsafe extern "system" fn OnOpened<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IAppBarOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            e: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnOpened(::windows_core::from_raw_borrowed(&e)).into()
        }
        unsafe extern "system" fn OnClosing<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IAppBarOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            e: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnClosing(::windows_core::from_raw_borrowed(&e)).into()
        }
        unsafe extern "system" fn OnOpening<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IAppBarOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            e: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnOpening(::windows_core::from_raw_borrowed(&e)).into()
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<Identity, IAppBarOverrides, OFFSET>(),
            OnClosed: OnClosed::<Identity, Impl, OFFSET>,
            OnOpened: OnOpened::<Identity, Impl, OFFSET>,
            OnClosing: OnClosing::<Identity, Impl, OFFSET>,
            OnOpening: OnOpening::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IAppBarOverrides as ::windows_core::ComInterface>::IID
    }
}
pub trait IComboBoxOverrides_Impl: Sized {
    fn OnDropDownClosed(
        &self,
        e: ::core::option::Option<&::windows_core::IInspectable>,
    ) -> ::windows_core::Result<()>;
    fn OnDropDownOpened(
        &self,
        e: ::core::option::Option<&::windows_core::IInspectable>,
    ) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IComboBoxOverrides {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.IComboBoxOverrides";
}
impl IComboBoxOverrides_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IComboBoxOverrides_Impl,
        const OFFSET: isize,
    >() -> IComboBoxOverrides_Vtbl {
        unsafe extern "system" fn OnDropDownClosed<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IComboBoxOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            e: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnDropDownClosed(::windows_core::from_raw_borrowed(&e)).into()
        }
        unsafe extern "system" fn OnDropDownOpened<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IComboBoxOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            e: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnDropDownOpened(::windows_core::from_raw_borrowed(&e)).into()
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<Identity, IComboBoxOverrides, OFFSET>(
            ),
            OnDropDownClosed: OnDropDownClosed::<Identity, Impl, OFFSET>,
            OnDropDownOpened: OnDropDownOpened::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IComboBoxOverrides as ::windows_core::ComInterface>::IID
    }
}
pub trait ICommandBarElement_Impl: Sized {
    fn IsCompact(&self) -> ::windows_core::Result<bool>;
    fn SetIsCompact(&self, value: bool) -> ::windows_core::Result<()>;
    fn IsInOverflow(&self) -> ::windows_core::Result<bool>;
    fn DynamicOverflowOrder(&self) -> ::windows_core::Result<i32>;
    fn SetDynamicOverflowOrder(&self, value: i32) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for ICommandBarElement {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.ICommandBarElement";
}
impl ICommandBarElement_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: ICommandBarElement_Impl,
        const OFFSET: isize,
    >() -> ICommandBarElement_Vtbl {
        unsafe extern "system" fn IsCompact<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: ICommandBarElement_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut bool,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsCompact() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsCompact<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: ICommandBarElement_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            value: bool,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetIsCompact(value).into()
        }
        unsafe extern "system" fn IsInOverflow<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: ICommandBarElement_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut bool,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsInOverflow() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DynamicOverflowOrder<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: ICommandBarElement_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut i32,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DynamicOverflowOrder() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDynamicOverflowOrder<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: ICommandBarElement_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            value: i32,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDynamicOverflowOrder(value).into()
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<Identity, ICommandBarElement, OFFSET>(
            ),
            IsCompact: IsCompact::<Identity, Impl, OFFSET>,
            SetIsCompact: SetIsCompact::<Identity, Impl, OFFSET>,
            IsInOverflow: IsInOverflow::<Identity, Impl, OFFSET>,
            DynamicOverflowOrder: DynamicOverflowOrder::<Identity, Impl, OFFSET>,
            SetDynamicOverflowOrder: SetDynamicOverflowOrder::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <ICommandBarElement as ::windows_core::ComInterface>::IID
    }
}
pub trait IContentControlOverrides_Impl: Sized {
    fn OnContentChanged(
        &self,
        oldcontent: ::core::option::Option<&::windows_core::IInspectable>,
        newcontent: ::core::option::Option<&::windows_core::IInspectable>,
    ) -> ::windows_core::Result<()>;
    fn OnContentTemplateChanged(
        &self,
        oldcontenttemplate: ::core::option::Option<&super::DataTemplate>,
        newcontenttemplate: ::core::option::Option<&super::DataTemplate>,
    ) -> ::windows_core::Result<()>;
    fn OnContentTemplateSelectorChanged(
        &self,
        oldcontenttemplateselector: ::core::option::Option<&DataTemplateSelector>,
        newcontenttemplateselector: ::core::option::Option<&DataTemplateSelector>,
    ) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IContentControlOverrides {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.IContentControlOverrides";
}
impl IContentControlOverrides_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IContentControlOverrides_Impl,
        const OFFSET: isize,
    >() -> IContentControlOverrides_Vtbl {
        unsafe extern "system" fn OnContentChanged<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IContentControlOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            oldcontent: *mut ::core::ffi::c_void,
            newcontent: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnContentChanged(
                ::windows_core::from_raw_borrowed(&oldcontent),
                ::windows_core::from_raw_borrowed(&newcontent),
            )
            .into()
        }
        unsafe extern "system" fn OnContentTemplateChanged<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IContentControlOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            oldcontenttemplate: *mut ::core::ffi::c_void,
            newcontenttemplate: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnContentTemplateChanged(
                ::windows_core::from_raw_borrowed(&oldcontenttemplate),
                ::windows_core::from_raw_borrowed(&newcontenttemplate),
            )
            .into()
        }
        unsafe extern "system" fn OnContentTemplateSelectorChanged<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IContentControlOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            oldcontenttemplateselector: *mut ::core::ffi::c_void,
            newcontenttemplateselector: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnContentTemplateSelectorChanged(
                ::windows_core::from_raw_borrowed(&oldcontenttemplateselector),
                ::windows_core::from_raw_borrowed(&newcontenttemplateselector),
            )
            .into()
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<
                Identity,
                IContentControlOverrides,
                OFFSET,
            >(),
            OnContentChanged: OnContentChanged::<Identity, Impl, OFFSET>,
            OnContentTemplateChanged: OnContentTemplateChanged::<Identity, Impl, OFFSET>,
            OnContentTemplateSelectorChanged: OnContentTemplateSelectorChanged::<
                Identity,
                Impl,
                OFFSET,
            >,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IContentControlOverrides as ::windows_core::ComInterface>::IID
    }
}
pub trait IContentPresenterOverrides_Impl: Sized {
    fn OnContentTemplateChanged(
        &self,
        oldcontenttemplate: ::core::option::Option<&super::DataTemplate>,
        newcontenttemplate: ::core::option::Option<&super::DataTemplate>,
    ) -> ::windows_core::Result<()>;
    fn OnContentTemplateSelectorChanged(
        &self,
        oldcontenttemplateselector: ::core::option::Option<&DataTemplateSelector>,
        newcontenttemplateselector: ::core::option::Option<&DataTemplateSelector>,
    ) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IContentPresenterOverrides {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.IContentPresenterOverrides";
}
impl IContentPresenterOverrides_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IContentPresenterOverrides_Impl,
        const OFFSET: isize,
    >() -> IContentPresenterOverrides_Vtbl {
        unsafe extern "system" fn OnContentTemplateChanged<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IContentPresenterOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            oldcontenttemplate: *mut ::core::ffi::c_void,
            newcontenttemplate: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnContentTemplateChanged(
                ::windows_core::from_raw_borrowed(&oldcontenttemplate),
                ::windows_core::from_raw_borrowed(&newcontenttemplate),
            )
            .into()
        }
        unsafe extern "system" fn OnContentTemplateSelectorChanged<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IContentPresenterOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            oldcontenttemplateselector: *mut ::core::ffi::c_void,
            newcontenttemplateselector: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnContentTemplateSelectorChanged(
                ::windows_core::from_raw_borrowed(&oldcontenttemplateselector),
                ::windows_core::from_raw_borrowed(&newcontenttemplateselector),
            )
            .into()
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<
                Identity,
                IContentPresenterOverrides,
                OFFSET,
            >(),
            OnContentTemplateChanged: OnContentTemplateChanged::<Identity, Impl, OFFSET>,
            OnContentTemplateSelectorChanged: OnContentTemplateSelectorChanged::<
                Identity,
                Impl,
                OFFSET,
            >,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IContentPresenterOverrides as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Microsoft_UI_Xaml_Input\"`"]
#[cfg(feature = "Microsoft_UI_Xaml_Input")]
pub trait IControlOverrides_Impl: Sized {
    fn OnPointerEntered(
        &self,
        e: ::core::option::Option<&super::Input::PointerRoutedEventArgs>,
    ) -> ::windows_core::Result<()>;
    fn OnPointerPressed(
        &self,
        e: ::core::option::Option<&super::Input::PointerRoutedEventArgs>,
    ) -> ::windows_core::Result<()>;
    fn OnPointerMoved(
        &self,
        e: ::core::option::Option<&super::Input::PointerRoutedEventArgs>,
    ) -> ::windows_core::Result<()>;
    fn OnPointerReleased(
        &self,
        e: ::core::option::Option<&super::Input::PointerRoutedEventArgs>,
    ) -> ::windows_core::Result<()>;
    fn OnPointerExited(
        &self,
        e: ::core::option::Option<&super::Input::PointerRoutedEventArgs>,
    ) -> ::windows_core::Result<()>;
    fn OnPointerCaptureLost(
        &self,
        e: ::core::option::Option<&super::Input::PointerRoutedEventArgs>,
    ) -> ::windows_core::Result<()>;
    fn OnPointerCanceled(
        &self,
        e: ::core::option::Option<&super::Input::PointerRoutedEventArgs>,
    ) -> ::windows_core::Result<()>;
    fn OnPointerWheelChanged(
        &self,
        e: ::core::option::Option<&super::Input::PointerRoutedEventArgs>,
    ) -> ::windows_core::Result<()>;
    fn OnTapped(
        &self,
        e: ::core::option::Option<&super::Input::TappedRoutedEventArgs>,
    ) -> ::windows_core::Result<()>;
    fn OnDoubleTapped(
        &self,
        e: ::core::option::Option<&super::Input::DoubleTappedRoutedEventArgs>,
    ) -> ::windows_core::Result<()>;
    fn OnHolding(
        &self,
        e: ::core::option::Option<&super::Input::HoldingRoutedEventArgs>,
    ) -> ::windows_core::Result<()>;
    fn OnRightTapped(
        &self,
        e: ::core::option::Option<&super::Input::RightTappedRoutedEventArgs>,
    ) -> ::windows_core::Result<()>;
    fn OnManipulationStarting(
        &self,
        e: ::core::option::Option<&super::Input::ManipulationStartingRoutedEventArgs>,
    ) -> ::windows_core::Result<()>;
    fn OnManipulationInertiaStarting(
        &self,
        e: ::core::option::Option<&super::Input::ManipulationInertiaStartingRoutedEventArgs>,
    ) -> ::windows_core::Result<()>;
    fn OnManipulationStarted(
        &self,
        e: ::core::option::Option<&super::Input::ManipulationStartedRoutedEventArgs>,
    ) -> ::windows_core::Result<()>;
    fn OnManipulationDelta(
        &self,
        e: ::core::option::Option<&super::Input::ManipulationDeltaRoutedEventArgs>,
    ) -> ::windows_core::Result<()>;
    fn OnManipulationCompleted(
        &self,
        e: ::core::option::Option<&super::Input::ManipulationCompletedRoutedEventArgs>,
    ) -> ::windows_core::Result<()>;
    fn OnKeyUp(
        &self,
        e: ::core::option::Option<&super::Input::KeyRoutedEventArgs>,
    ) -> ::windows_core::Result<()>;
    fn OnKeyDown(
        &self,
        e: ::core::option::Option<&super::Input::KeyRoutedEventArgs>,
    ) -> ::windows_core::Result<()>;
    fn OnPreviewKeyDown(
        &self,
        e: ::core::option::Option<&super::Input::KeyRoutedEventArgs>,
    ) -> ::windows_core::Result<()>;
    fn OnPreviewKeyUp(
        &self,
        e: ::core::option::Option<&super::Input::KeyRoutedEventArgs>,
    ) -> ::windows_core::Result<()>;
    fn OnGotFocus(
        &self,
        e: ::core::option::Option<&super::RoutedEventArgs>,
    ) -> ::windows_core::Result<()>;
    fn OnLostFocus(
        &self,
        e: ::core::option::Option<&super::RoutedEventArgs>,
    ) -> ::windows_core::Result<()>;
    fn OnCharacterReceived(
        &self,
        e: ::core::option::Option<&super::Input::CharacterReceivedRoutedEventArgs>,
    ) -> ::windows_core::Result<()>;
    fn OnDragEnter(
        &self,
        e: ::core::option::Option<&super::DragEventArgs>,
    ) -> ::windows_core::Result<()>;
    fn OnDragLeave(
        &self,
        e: ::core::option::Option<&super::DragEventArgs>,
    ) -> ::windows_core::Result<()>;
    fn OnDragOver(
        &self,
        e: ::core::option::Option<&super::DragEventArgs>,
    ) -> ::windows_core::Result<()>;
    fn OnDrop(
        &self,
        e: ::core::option::Option<&super::DragEventArgs>,
    ) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Microsoft_UI_Xaml_Input")]
impl ::windows_core::RuntimeName for IControlOverrides {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.IControlOverrides";
}
#[cfg(feature = "Microsoft_UI_Xaml_Input")]
impl IControlOverrides_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IControlOverrides_Impl,
        const OFFSET: isize,
    >() -> IControlOverrides_Vtbl {
        unsafe extern "system" fn OnPointerEntered<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IControlOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            e: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnPointerEntered(::windows_core::from_raw_borrowed(&e)).into()
        }
        unsafe extern "system" fn OnPointerPressed<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IControlOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            e: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnPointerPressed(::windows_core::from_raw_borrowed(&e)).into()
        }
        unsafe extern "system" fn OnPointerMoved<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IControlOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            e: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnPointerMoved(::windows_core::from_raw_borrowed(&e)).into()
        }
        unsafe extern "system" fn OnPointerReleased<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IControlOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            e: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnPointerReleased(::windows_core::from_raw_borrowed(&e)).into()
        }
        unsafe extern "system" fn OnPointerExited<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IControlOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            e: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnPointerExited(::windows_core::from_raw_borrowed(&e)).into()
        }
        unsafe extern "system" fn OnPointerCaptureLost<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IControlOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            e: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnPointerCaptureLost(::windows_core::from_raw_borrowed(&e)).into()
        }
        unsafe extern "system" fn OnPointerCanceled<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IControlOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            e: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnPointerCanceled(::windows_core::from_raw_borrowed(&e)).into()
        }
        unsafe extern "system" fn OnPointerWheelChanged<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IControlOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            e: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnPointerWheelChanged(::windows_core::from_raw_borrowed(&e)).into()
        }
        unsafe extern "system" fn OnTapped<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IControlOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            e: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnTapped(::windows_core::from_raw_borrowed(&e)).into()
        }
        unsafe extern "system" fn OnDoubleTapped<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IControlOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            e: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnDoubleTapped(::windows_core::from_raw_borrowed(&e)).into()
        }
        unsafe extern "system" fn OnHolding<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IControlOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            e: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnHolding(::windows_core::from_raw_borrowed(&e)).into()
        }
        unsafe extern "system" fn OnRightTapped<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IControlOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            e: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnRightTapped(::windows_core::from_raw_borrowed(&e)).into()
        }
        unsafe extern "system" fn OnManipulationStarting<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IControlOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            e: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnManipulationStarting(::windows_core::from_raw_borrowed(&e)).into()
        }
        unsafe extern "system" fn OnManipulationInertiaStarting<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IControlOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            e: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnManipulationInertiaStarting(::windows_core::from_raw_borrowed(&e)).into()
        }
        unsafe extern "system" fn OnManipulationStarted<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IControlOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            e: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnManipulationStarted(::windows_core::from_raw_borrowed(&e)).into()
        }
        unsafe extern "system" fn OnManipulationDelta<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IControlOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            e: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnManipulationDelta(::windows_core::from_raw_borrowed(&e)).into()
        }
        unsafe extern "system" fn OnManipulationCompleted<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IControlOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            e: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnManipulationCompleted(::windows_core::from_raw_borrowed(&e)).into()
        }
        unsafe extern "system" fn OnKeyUp<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IControlOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            e: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnKeyUp(::windows_core::from_raw_borrowed(&e)).into()
        }
        unsafe extern "system" fn OnKeyDown<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IControlOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            e: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnKeyDown(::windows_core::from_raw_borrowed(&e)).into()
        }
        unsafe extern "system" fn OnPreviewKeyDown<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IControlOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            e: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnPreviewKeyDown(::windows_core::from_raw_borrowed(&e)).into()
        }
        unsafe extern "system" fn OnPreviewKeyUp<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IControlOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            e: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnPreviewKeyUp(::windows_core::from_raw_borrowed(&e)).into()
        }
        unsafe extern "system" fn OnGotFocus<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IControlOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            e: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnGotFocus(::windows_core::from_raw_borrowed(&e)).into()
        }
        unsafe extern "system" fn OnLostFocus<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IControlOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            e: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnLostFocus(::windows_core::from_raw_borrowed(&e)).into()
        }
        unsafe extern "system" fn OnCharacterReceived<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IControlOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            e: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnCharacterReceived(::windows_core::from_raw_borrowed(&e)).into()
        }
        unsafe extern "system" fn OnDragEnter<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IControlOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            e: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnDragEnter(::windows_core::from_raw_borrowed(&e)).into()
        }
        unsafe extern "system" fn OnDragLeave<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IControlOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            e: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnDragLeave(::windows_core::from_raw_borrowed(&e)).into()
        }
        unsafe extern "system" fn OnDragOver<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IControlOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            e: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnDragOver(::windows_core::from_raw_borrowed(&e)).into()
        }
        unsafe extern "system" fn OnDrop<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IControlOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            e: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnDrop(::windows_core::from_raw_borrowed(&e)).into()
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<Identity, IControlOverrides, OFFSET>(),
            OnPointerEntered: OnPointerEntered::<Identity, Impl, OFFSET>,
            OnPointerPressed: OnPointerPressed::<Identity, Impl, OFFSET>,
            OnPointerMoved: OnPointerMoved::<Identity, Impl, OFFSET>,
            OnPointerReleased: OnPointerReleased::<Identity, Impl, OFFSET>,
            OnPointerExited: OnPointerExited::<Identity, Impl, OFFSET>,
            OnPointerCaptureLost: OnPointerCaptureLost::<Identity, Impl, OFFSET>,
            OnPointerCanceled: OnPointerCanceled::<Identity, Impl, OFFSET>,
            OnPointerWheelChanged: OnPointerWheelChanged::<Identity, Impl, OFFSET>,
            OnTapped: OnTapped::<Identity, Impl, OFFSET>,
            OnDoubleTapped: OnDoubleTapped::<Identity, Impl, OFFSET>,
            OnHolding: OnHolding::<Identity, Impl, OFFSET>,
            OnRightTapped: OnRightTapped::<Identity, Impl, OFFSET>,
            OnManipulationStarting: OnManipulationStarting::<Identity, Impl, OFFSET>,
            OnManipulationInertiaStarting: OnManipulationInertiaStarting::<Identity, Impl, OFFSET>,
            OnManipulationStarted: OnManipulationStarted::<Identity, Impl, OFFSET>,
            OnManipulationDelta: OnManipulationDelta::<Identity, Impl, OFFSET>,
            OnManipulationCompleted: OnManipulationCompleted::<Identity, Impl, OFFSET>,
            OnKeyUp: OnKeyUp::<Identity, Impl, OFFSET>,
            OnKeyDown: OnKeyDown::<Identity, Impl, OFFSET>,
            OnPreviewKeyDown: OnPreviewKeyDown::<Identity, Impl, OFFSET>,
            OnPreviewKeyUp: OnPreviewKeyUp::<Identity, Impl, OFFSET>,
            OnGotFocus: OnGotFocus::<Identity, Impl, OFFSET>,
            OnLostFocus: OnLostFocus::<Identity, Impl, OFFSET>,
            OnCharacterReceived: OnCharacterReceived::<Identity, Impl, OFFSET>,
            OnDragEnter: OnDragEnter::<Identity, Impl, OFFSET>,
            OnDragLeave: OnDragLeave::<Identity, Impl, OFFSET>,
            OnDragOver: OnDragOver::<Identity, Impl, OFFSET>,
            OnDrop: OnDrop::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IControlOverrides as ::windows_core::ComInterface>::IID
    }
}
pub trait IDataTemplateSelectorOverrides_Impl: Sized {
    fn SelectTemplateCore(
        &self,
        item: ::core::option::Option<&::windows_core::IInspectable>,
        container: ::core::option::Option<&super::DependencyObject>,
    ) -> ::windows_core::Result<super::DataTemplate>;
    fn SelectTemplateForItemCore(
        &self,
        item: ::core::option::Option<&::windows_core::IInspectable>,
    ) -> ::windows_core::Result<super::DataTemplate>;
}
impl ::windows_core::RuntimeName for IDataTemplateSelectorOverrides {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.IDataTemplateSelectorOverrides";
}
impl IDataTemplateSelectorOverrides_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IDataTemplateSelectorOverrides_Impl,
        const OFFSET: isize,
    >() -> IDataTemplateSelectorOverrides_Vtbl {
        unsafe extern "system" fn SelectTemplateCore<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IDataTemplateSelectorOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            item: *mut ::core::ffi::c_void,
            container: *mut ::core::ffi::c_void,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SelectTemplateCore(
                ::windows_core::from_raw_borrowed(&item),
                ::windows_core::from_raw_borrowed(&container),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelectTemplateForItemCore<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IDataTemplateSelectorOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            item: *mut ::core::ffi::c_void,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SelectTemplateForItemCore(::windows_core::from_raw_borrowed(&item)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<
                Identity,
                IDataTemplateSelectorOverrides,
                OFFSET,
            >(),
            SelectTemplateCore: SelectTemplateCore::<Identity, Impl, OFFSET>,
            SelectTemplateForItemCore: SelectTemplateForItemCore::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IDataTemplateSelectorOverrides as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Microsoft_UI_Composition\"`, `\"Windows_Foundation\"`"]
#[cfg(all(feature = "Microsoft_UI_Composition", feature = "Windows_Foundation"))]
pub trait IDynamicAnimatedVisualSource_Impl: Sized + IAnimatedVisualSource_Impl {
    fn AnimatedVisualInvalidated(
        &self,
        handler: ::core::option::Option<
            &::windows::Foundation::TypedEventHandler<
                IDynamicAnimatedVisualSource,
                ::windows_core::IInspectable,
            >,
        >,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>;
    fn RemoveAnimatedVisualInvalidated(
        &self,
        token: &::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Microsoft_UI_Composition", feature = "Windows_Foundation"))]
impl ::windows_core::RuntimeName for IDynamicAnimatedVisualSource {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.IDynamicAnimatedVisualSource";
}
#[cfg(all(feature = "Microsoft_UI_Composition", feature = "Windows_Foundation"))]
impl IDynamicAnimatedVisualSource_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IDynamicAnimatedVisualSource_Impl,
        const OFFSET: isize,
    >() -> IDynamicAnimatedVisualSource_Vtbl {
        unsafe extern "system" fn AnimatedVisualInvalidated<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IDynamicAnimatedVisualSource_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            handler: *mut ::core::ffi::c_void,
            result__: *mut ::windows::Foundation::EventRegistrationToken,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AnimatedVisualInvalidated(::windows_core::from_raw_borrowed(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAnimatedVisualInvalidated<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IDynamicAnimatedVisualSource_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            token: ::windows::Foundation::EventRegistrationToken,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveAnimatedVisualInvalidated(::core::mem::transmute(&token)).into()
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<
                Identity,
                IDynamicAnimatedVisualSource,
                OFFSET,
            >(),
            AnimatedVisualInvalidated: AnimatedVisualInvalidated::<Identity, Impl, OFFSET>,
            RemoveAnimatedVisualInvalidated: RemoveAnimatedVisualInvalidated::<
                Identity,
                Impl,
                OFFSET,
            >,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IDynamicAnimatedVisualSource as ::windows_core::ComInterface>::IID
    }
}
pub trait IGroupStyleSelectorOverrides_Impl: Sized {
    fn SelectGroupStyleCore(
        &self,
        group: ::core::option::Option<&::windows_core::IInspectable>,
        level: u32,
    ) -> ::windows_core::Result<GroupStyle>;
}
impl ::windows_core::RuntimeName for IGroupStyleSelectorOverrides {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.IGroupStyleSelectorOverrides";
}
impl IGroupStyleSelectorOverrides_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IGroupStyleSelectorOverrides_Impl,
        const OFFSET: isize,
    >() -> IGroupStyleSelectorOverrides_Vtbl {
        unsafe extern "system" fn SelectGroupStyleCore<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IGroupStyleSelectorOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            group: *mut ::core::ffi::c_void,
            level: u32,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SelectGroupStyleCore(::windows_core::from_raw_borrowed(&group), level) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<
                Identity,
                IGroupStyleSelectorOverrides,
                OFFSET,
            >(),
            SelectGroupStyleCore: SelectGroupStyleCore::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IGroupStyleSelectorOverrides as ::windows_core::ComInterface>::IID
    }
}
pub trait IIconSourceOverrides_Impl: Sized {
    fn CreateIconElementCore(&self) -> ::windows_core::Result<IconElement>;
    fn GetIconElementPropertyCore(
        &self,
        iconsourceproperty: ::core::option::Option<&super::DependencyProperty>,
    ) -> ::windows_core::Result<super::DependencyProperty>;
}
impl ::windows_core::RuntimeName for IIconSourceOverrides {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.IIconSourceOverrides";
}
impl IIconSourceOverrides_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IIconSourceOverrides_Impl,
        const OFFSET: isize,
    >() -> IIconSourceOverrides_Vtbl {
        unsafe extern "system" fn CreateIconElementCore<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IIconSourceOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateIconElementCore() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIconElementPropertyCore<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IIconSourceOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            iconsourceproperty: *mut ::core::ffi::c_void,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this
                .GetIconElementPropertyCore(::windows_core::from_raw_borrowed(&iconsourceproperty))
            {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<Identity, IIconSourceOverrides, OFFSET>(
            ),
            CreateIconElementCore: CreateIconElementCore::<Identity, Impl, OFFSET>,
            GetIconElementPropertyCore: GetIconElementPropertyCore::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IIconSourceOverrides as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Windows_Foundation\"`"]
#[cfg(feature = "Windows_Foundation")]
pub trait IInsertionPanel_Impl: Sized {
    fn GetInsertionIndexes(
        &self,
        position: &::windows::Foundation::Point,
        first: &mut i32,
        second: &mut i32,
    ) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Windows_Foundation")]
impl ::windows_core::RuntimeName for IInsertionPanel {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.IInsertionPanel";
}
#[cfg(feature = "Windows_Foundation")]
impl IInsertionPanel_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IInsertionPanel_Impl,
        const OFFSET: isize,
    >() -> IInsertionPanel_Vtbl {
        unsafe extern "system" fn GetInsertionIndexes<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IInsertionPanel_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            position: ::windows::Foundation::Point,
            first: *mut i32,
            second: *mut i32,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetInsertionIndexes(
                ::core::mem::transmute(&position),
                ::core::mem::transmute_copy(&first),
                ::core::mem::transmute_copy(&second),
            )
            .into()
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<Identity, IInsertionPanel, OFFSET>(),
            GetInsertionIndexes: GetInsertionIndexes::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IInsertionPanel as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Windows_Foundation_Collections\"`"]
#[cfg(feature = "Windows_Foundation_Collections")]
pub trait IItemCollectionTransitionProviderOverrides_Impl: Sized {
    fn ShouldAnimateCore(
        &self,
        transition: ::core::option::Option<&ItemCollectionTransition>,
    ) -> ::windows_core::Result<bool>;
    fn StartTransitions(
        &self,
        transitions: ::core::option::Option<
            &::windows::Foundation::Collections::IVector<ItemCollectionTransition>,
        >,
    ) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Windows_Foundation_Collections")]
impl ::windows_core::RuntimeName for IItemCollectionTransitionProviderOverrides {
    const NAME: &'static str =
        "Microsoft.UI.Xaml.Controls.IItemCollectionTransitionProviderOverrides";
}
#[cfg(feature = "Windows_Foundation_Collections")]
impl IItemCollectionTransitionProviderOverrides_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IItemCollectionTransitionProviderOverrides_Impl,
        const OFFSET: isize,
    >() -> IItemCollectionTransitionProviderOverrides_Vtbl {
        unsafe extern "system" fn ShouldAnimateCore<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IItemCollectionTransitionProviderOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            transition: *mut ::core::ffi::c_void,
            result__: *mut bool,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ShouldAnimateCore(::windows_core::from_raw_borrowed(&transition)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartTransitions<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IItemCollectionTransitionProviderOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            transitions: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.StartTransitions(::windows_core::from_raw_borrowed(&transitions)).into()
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<
                Identity,
                IItemCollectionTransitionProviderOverrides,
                OFFSET,
            >(),
            ShouldAnimateCore: ShouldAnimateCore::<Identity, Impl, OFFSET>,
            StartTransitions: StartTransitions::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IItemCollectionTransitionProviderOverrides as ::windows_core::ComInterface>::IID
    }
}
pub trait IItemContainerMapping_Impl: Sized {
    fn ItemFromContainer(
        &self,
        container: ::core::option::Option<&super::DependencyObject>,
    ) -> ::windows_core::Result<::windows_core::IInspectable>;
    fn ContainerFromItem(
        &self,
        item: ::core::option::Option<&::windows_core::IInspectable>,
    ) -> ::windows_core::Result<super::DependencyObject>;
    fn IndexFromContainer(
        &self,
        container: ::core::option::Option<&super::DependencyObject>,
    ) -> ::windows_core::Result<i32>;
    fn ContainerFromIndex(&self, index: i32) -> ::windows_core::Result<super::DependencyObject>;
}
impl ::windows_core::RuntimeName for IItemContainerMapping {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.IItemContainerMapping";
}
impl IItemContainerMapping_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IItemContainerMapping_Impl,
        const OFFSET: isize,
    >() -> IItemContainerMapping_Vtbl {
        unsafe extern "system" fn ItemFromContainer<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IItemContainerMapping_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            container: *mut ::core::ffi::c_void,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ItemFromContainer(::windows_core::from_raw_borrowed(&container)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ContainerFromItem<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IItemContainerMapping_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            item: *mut ::core::ffi::c_void,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ContainerFromItem(::windows_core::from_raw_borrowed(&item)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IndexFromContainer<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IItemContainerMapping_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            container: *mut ::core::ffi::c_void,
            result__: *mut i32,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IndexFromContainer(::windows_core::from_raw_borrowed(&container)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ContainerFromIndex<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IItemContainerMapping_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            index: i32,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ContainerFromIndex(index) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<Identity, IItemContainerMapping, OFFSET>(
            ),
            ItemFromContainer: ItemFromContainer::<Identity, Impl, OFFSET>,
            ContainerFromItem: ContainerFromItem::<Identity, Impl, OFFSET>,
            IndexFromContainer: IndexFromContainer::<Identity, Impl, OFFSET>,
            ContainerFromIndex: ContainerFromIndex::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IItemContainerMapping as ::windows_core::ComInterface>::IID
    }
}
pub trait IItemsControlOverrides_Impl: Sized {
    fn IsItemItsOwnContainerOverride(
        &self,
        item: ::core::option::Option<&::windows_core::IInspectable>,
    ) -> ::windows_core::Result<bool>;
    fn GetContainerForItemOverride(&self) -> ::windows_core::Result<super::DependencyObject>;
    fn ClearContainerForItemOverride(
        &self,
        element: ::core::option::Option<&super::DependencyObject>,
        item: ::core::option::Option<&::windows_core::IInspectable>,
    ) -> ::windows_core::Result<()>;
    fn PrepareContainerForItemOverride(
        &self,
        element: ::core::option::Option<&super::DependencyObject>,
        item: ::core::option::Option<&::windows_core::IInspectable>,
    ) -> ::windows_core::Result<()>;
    fn OnItemsChanged(
        &self,
        e: ::core::option::Option<&::windows_core::IInspectable>,
    ) -> ::windows_core::Result<()>;
    fn OnItemContainerStyleChanged(
        &self,
        olditemcontainerstyle: ::core::option::Option<&super::Style>,
        newitemcontainerstyle: ::core::option::Option<&super::Style>,
    ) -> ::windows_core::Result<()>;
    fn OnItemContainerStyleSelectorChanged(
        &self,
        olditemcontainerstyleselector: ::core::option::Option<&StyleSelector>,
        newitemcontainerstyleselector: ::core::option::Option<&StyleSelector>,
    ) -> ::windows_core::Result<()>;
    fn OnItemTemplateChanged(
        &self,
        olditemtemplate: ::core::option::Option<&super::DataTemplate>,
        newitemtemplate: ::core::option::Option<&super::DataTemplate>,
    ) -> ::windows_core::Result<()>;
    fn OnItemTemplateSelectorChanged(
        &self,
        olditemtemplateselector: ::core::option::Option<&DataTemplateSelector>,
        newitemtemplateselector: ::core::option::Option<&DataTemplateSelector>,
    ) -> ::windows_core::Result<()>;
    fn OnGroupStyleSelectorChanged(
        &self,
        oldgroupstyleselector: ::core::option::Option<&GroupStyleSelector>,
        newgroupstyleselector: ::core::option::Option<&GroupStyleSelector>,
    ) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IItemsControlOverrides {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.IItemsControlOverrides";
}
impl IItemsControlOverrides_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IItemsControlOverrides_Impl,
        const OFFSET: isize,
    >() -> IItemsControlOverrides_Vtbl {
        unsafe extern "system" fn IsItemItsOwnContainerOverride<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IItemsControlOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            item: *mut ::core::ffi::c_void,
            result__: *mut bool,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsItemItsOwnContainerOverride(::windows_core::from_raw_borrowed(&item)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetContainerForItemOverride<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IItemsControlOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetContainerForItemOverride() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClearContainerForItemOverride<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IItemsControlOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            element: *mut ::core::ffi::c_void,
            item: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ClearContainerForItemOverride(
                ::windows_core::from_raw_borrowed(&element),
                ::windows_core::from_raw_borrowed(&item),
            )
            .into()
        }
        unsafe extern "system" fn PrepareContainerForItemOverride<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IItemsControlOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            element: *mut ::core::ffi::c_void,
            item: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PrepareContainerForItemOverride(
                ::windows_core::from_raw_borrowed(&element),
                ::windows_core::from_raw_borrowed(&item),
            )
            .into()
        }
        unsafe extern "system" fn OnItemsChanged<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IItemsControlOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            e: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnItemsChanged(::windows_core::from_raw_borrowed(&e)).into()
        }
        unsafe extern "system" fn OnItemContainerStyleChanged<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IItemsControlOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            olditemcontainerstyle: *mut ::core::ffi::c_void,
            newitemcontainerstyle: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnItemContainerStyleChanged(
                ::windows_core::from_raw_borrowed(&olditemcontainerstyle),
                ::windows_core::from_raw_borrowed(&newitemcontainerstyle),
            )
            .into()
        }
        unsafe extern "system" fn OnItemContainerStyleSelectorChanged<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IItemsControlOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            olditemcontainerstyleselector: *mut ::core::ffi::c_void,
            newitemcontainerstyleselector: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnItemContainerStyleSelectorChanged(
                ::windows_core::from_raw_borrowed(&olditemcontainerstyleselector),
                ::windows_core::from_raw_borrowed(&newitemcontainerstyleselector),
            )
            .into()
        }
        unsafe extern "system" fn OnItemTemplateChanged<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IItemsControlOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            olditemtemplate: *mut ::core::ffi::c_void,
            newitemtemplate: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnItemTemplateChanged(
                ::windows_core::from_raw_borrowed(&olditemtemplate),
                ::windows_core::from_raw_borrowed(&newitemtemplate),
            )
            .into()
        }
        unsafe extern "system" fn OnItemTemplateSelectorChanged<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IItemsControlOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            olditemtemplateselector: *mut ::core::ffi::c_void,
            newitemtemplateselector: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnItemTemplateSelectorChanged(
                ::windows_core::from_raw_borrowed(&olditemtemplateselector),
                ::windows_core::from_raw_borrowed(&newitemtemplateselector),
            )
            .into()
        }
        unsafe extern "system" fn OnGroupStyleSelectorChanged<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IItemsControlOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            oldgroupstyleselector: *mut ::core::ffi::c_void,
            newgroupstyleselector: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnGroupStyleSelectorChanged(
                ::windows_core::from_raw_borrowed(&oldgroupstyleselector),
                ::windows_core::from_raw_borrowed(&newgroupstyleselector),
            )
            .into()
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<
                Identity,
                IItemsControlOverrides,
                OFFSET,
            >(),
            IsItemItsOwnContainerOverride: IsItemItsOwnContainerOverride::<Identity, Impl, OFFSET>,
            GetContainerForItemOverride: GetContainerForItemOverride::<Identity, Impl, OFFSET>,
            ClearContainerForItemOverride: ClearContainerForItemOverride::<Identity, Impl, OFFSET>,
            PrepareContainerForItemOverride: PrepareContainerForItemOverride::<
                Identity,
                Impl,
                OFFSET,
            >,
            OnItemsChanged: OnItemsChanged::<Identity, Impl, OFFSET>,
            OnItemContainerStyleChanged: OnItemContainerStyleChanged::<Identity, Impl, OFFSET>,
            OnItemContainerStyleSelectorChanged: OnItemContainerStyleSelectorChanged::<
                Identity,
                Impl,
                OFFSET,
            >,
            OnItemTemplateChanged: OnItemTemplateChanged::<Identity, Impl, OFFSET>,
            OnItemTemplateSelectorChanged: OnItemTemplateSelectorChanged::<Identity, Impl, OFFSET>,
            OnGroupStyleSelectorChanged: OnGroupStyleSelectorChanged::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IItemsControlOverrides as ::windows_core::ComInterface>::IID
    }
}
pub trait IKeyIndexMapping_Impl: Sized {
    fn KeyFromIndex(&self, index: i32) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn IndexFromKey(&self, key: &::windows_core::HSTRING) -> ::windows_core::Result<i32>;
}
impl ::windows_core::RuntimeName for IKeyIndexMapping {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.IKeyIndexMapping";
}
impl IKeyIndexMapping_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IKeyIndexMapping_Impl,
        const OFFSET: isize,
    >() -> IKeyIndexMapping_Vtbl {
        unsafe extern "system" fn KeyFromIndex<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IKeyIndexMapping_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            index: i32,
            result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.KeyFromIndex(index) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IndexFromKey<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IKeyIndexMapping_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            key: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
            result__: *mut i32,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IndexFromKey(::core::mem::transmute(&key)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<Identity, IKeyIndexMapping, OFFSET>(),
            KeyFromIndex: KeyFromIndex::<Identity, Impl, OFFSET>,
            IndexFromKey: IndexFromKey::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IKeyIndexMapping as ::windows_core::ComInterface>::IID
    }
}
pub trait ILayoutContextOverrides_Impl: Sized {
    fn LayoutStateCore(&self) -> ::windows_core::Result<::windows_core::IInspectable>;
    fn SetLayoutStateCore(
        &self,
        value: ::core::option::Option<&::windows_core::IInspectable>,
    ) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for ILayoutContextOverrides {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.ILayoutContextOverrides";
}
impl ILayoutContextOverrides_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: ILayoutContextOverrides_Impl,
        const OFFSET: isize,
    >() -> ILayoutContextOverrides_Vtbl {
        unsafe extern "system" fn LayoutStateCore<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: ILayoutContextOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LayoutStateCore() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLayoutStateCore<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: ILayoutContextOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            value: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetLayoutStateCore(::windows_core::from_raw_borrowed(&value)).into()
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<
                Identity,
                ILayoutContextOverrides,
                OFFSET,
            >(),
            LayoutStateCore: LayoutStateCore::<Identity, Impl, OFFSET>,
            SetLayoutStateCore: SetLayoutStateCore::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <ILayoutContextOverrides as ::windows_core::ComInterface>::IID
    }
}
pub trait ILayoutOverrides_Impl: Sized {
    fn CreateDefaultItemTransitionProvider(
        &self,
    ) -> ::windows_core::Result<ItemCollectionTransitionProvider>;
}
impl ::windows_core::RuntimeName for ILayoutOverrides {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.ILayoutOverrides";
}
impl ILayoutOverrides_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: ILayoutOverrides_Impl,
        const OFFSET: isize,
    >() -> ILayoutOverrides_Vtbl {
        unsafe extern "system" fn CreateDefaultItemTransitionProvider<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: ILayoutOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateDefaultItemTransitionProvider() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<Identity, ILayoutOverrides, OFFSET>(),
            CreateDefaultItemTransitionProvider: CreateDefaultItemTransitionProvider::<
                Identity,
                Impl,
                OFFSET,
            >,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <ILayoutOverrides as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Windows_UI_Xaml_Interop\"`"]
#[cfg(feature = "Windows_UI_Xaml_Interop")]
pub trait INavigate_Impl: Sized {
    fn Navigate(
        &self,
        sourcepagetype: &super::super::super::super::Windows::UI::Xaml::Interop::TypeName,
    ) -> ::windows_core::Result<bool>;
}
#[cfg(feature = "Windows_UI_Xaml_Interop")]
impl ::windows_core::RuntimeName for INavigate {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.INavigate";
}
#[cfg(feature = "Windows_UI_Xaml_Interop")]
impl INavigate_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: INavigate_Impl,
        const OFFSET: isize,
    >() -> INavigate_Vtbl {
        unsafe extern "system" fn Navigate<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: INavigate_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            sourcepagetype: ::std::mem::MaybeUninit<
                super::super::super::super::Windows::UI::Xaml::Interop::TypeName,
            >,
            result__: *mut bool,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Navigate(::core::mem::transmute(&sourcepagetype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<Identity, INavigate, OFFSET>(),
            Navigate: Navigate::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <INavigate as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Windows_Foundation_Collections\"`"]
#[cfg(feature = "Windows_Foundation_Collections")]
pub trait INonVirtualizingLayoutContextOverrides_Impl: Sized {
    fn ChildrenCore(
        &self,
    ) -> ::windows_core::Result<::windows::Foundation::Collections::IVectorView<super::UIElement>>;
}
#[cfg(feature = "Windows_Foundation_Collections")]
impl ::windows_core::RuntimeName for INonVirtualizingLayoutContextOverrides {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.INonVirtualizingLayoutContextOverrides";
}
#[cfg(feature = "Windows_Foundation_Collections")]
impl INonVirtualizingLayoutContextOverrides_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: INonVirtualizingLayoutContextOverrides_Impl,
        const OFFSET: isize,
    >() -> INonVirtualizingLayoutContextOverrides_Vtbl {
        unsafe extern "system" fn ChildrenCore<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: INonVirtualizingLayoutContextOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ChildrenCore() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<
                Identity,
                INonVirtualizingLayoutContextOverrides,
                OFFSET,
            >(),
            ChildrenCore: ChildrenCore::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <INonVirtualizingLayoutContextOverrides as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Windows_Foundation\"`"]
#[cfg(feature = "Windows_Foundation")]
pub trait INonVirtualizingLayoutOverrides_Impl: Sized {
    fn InitializeForContextCore(
        &self,
        context: ::core::option::Option<&NonVirtualizingLayoutContext>,
    ) -> ::windows_core::Result<()>;
    fn UninitializeForContextCore(
        &self,
        context: ::core::option::Option<&NonVirtualizingLayoutContext>,
    ) -> ::windows_core::Result<()>;
    fn MeasureOverride(
        &self,
        context: ::core::option::Option<&NonVirtualizingLayoutContext>,
        availablesize: &::windows::Foundation::Size,
    ) -> ::windows_core::Result<::windows::Foundation::Size>;
    fn ArrangeOverride(
        &self,
        context: ::core::option::Option<&NonVirtualizingLayoutContext>,
        finalsize: &::windows::Foundation::Size,
    ) -> ::windows_core::Result<::windows::Foundation::Size>;
}
#[cfg(feature = "Windows_Foundation")]
impl ::windows_core::RuntimeName for INonVirtualizingLayoutOverrides {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.INonVirtualizingLayoutOverrides";
}
#[cfg(feature = "Windows_Foundation")]
impl INonVirtualizingLayoutOverrides_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: INonVirtualizingLayoutOverrides_Impl,
        const OFFSET: isize,
    >() -> INonVirtualizingLayoutOverrides_Vtbl {
        unsafe extern "system" fn InitializeForContextCore<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: INonVirtualizingLayoutOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.InitializeForContextCore(::windows_core::from_raw_borrowed(&context))
                .into()
        }
        unsafe extern "system" fn UninitializeForContextCore<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: INonVirtualizingLayoutOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UninitializeForContextCore(::windows_core::from_raw_borrowed(&context))
                .into()
        }
        unsafe extern "system" fn MeasureOverride<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: INonVirtualizingLayoutOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            availablesize: ::windows::Foundation::Size,
            result__: *mut ::windows::Foundation::Size,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MeasureOverride(
                ::windows_core::from_raw_borrowed(&context),
                ::core::mem::transmute(&availablesize),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ArrangeOverride<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: INonVirtualizingLayoutOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            finalsize: ::windows::Foundation::Size,
            result__: *mut ::windows::Foundation::Size,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ArrangeOverride(
                ::windows_core::from_raw_borrowed(&context),
                ::core::mem::transmute(&finalsize),
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
                INonVirtualizingLayoutOverrides,
                OFFSET,
            >(),
            InitializeForContextCore: InitializeForContextCore::<Identity, Impl, OFFSET>,
            UninitializeForContextCore: UninitializeForContextCore::<Identity, Impl, OFFSET>,
            MeasureOverride: MeasureOverride::<Identity, Impl, OFFSET>,
            ArrangeOverride: ArrangeOverride::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <INonVirtualizingLayoutOverrides as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Microsoft_UI_Xaml_Navigation\"`"]
#[cfg(feature = "Microsoft_UI_Xaml_Navigation")]
pub trait IPageOverrides_Impl: Sized {
    fn OnNavigatedFrom(
        &self,
        e: ::core::option::Option<&super::Navigation::NavigationEventArgs>,
    ) -> ::windows_core::Result<()>;
    fn OnNavigatedTo(
        &self,
        e: ::core::option::Option<&super::Navigation::NavigationEventArgs>,
    ) -> ::windows_core::Result<()>;
    fn OnNavigatingFrom(
        &self,
        e: ::core::option::Option<&super::Navigation::NavigatingCancelEventArgs>,
    ) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Microsoft_UI_Xaml_Navigation")]
impl ::windows_core::RuntimeName for IPageOverrides {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.IPageOverrides";
}
#[cfg(feature = "Microsoft_UI_Xaml_Navigation")]
impl IPageOverrides_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IPageOverrides_Impl,
        const OFFSET: isize,
    >() -> IPageOverrides_Vtbl {
        unsafe extern "system" fn OnNavigatedFrom<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IPageOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            e: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnNavigatedFrom(::windows_core::from_raw_borrowed(&e)).into()
        }
        unsafe extern "system" fn OnNavigatedTo<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IPageOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            e: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnNavigatedTo(::windows_core::from_raw_borrowed(&e)).into()
        }
        unsafe extern "system" fn OnNavigatingFrom<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IPageOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            e: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnNavigatingFrom(::windows_core::from_raw_borrowed(&e)).into()
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<Identity, IPageOverrides, OFFSET>(),
            OnNavigatedFrom: OnNavigatedFrom::<Identity, Impl, OFFSET>,
            OnNavigatedTo: OnNavigatedTo::<Identity, Impl, OFFSET>,
            OnNavigatingFrom: OnNavigatingFrom::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IPageOverrides as ::windows_core::ComInterface>::IID
    }
}
pub trait IScrollAnchorProvider_Impl: Sized {
    fn CurrentAnchor(&self) -> ::windows_core::Result<super::UIElement>;
    fn RegisterAnchorCandidate(
        &self,
        element: ::core::option::Option<&super::UIElement>,
    ) -> ::windows_core::Result<()>;
    fn UnregisterAnchorCandidate(
        &self,
        element: ::core::option::Option<&super::UIElement>,
    ) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IScrollAnchorProvider {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.IScrollAnchorProvider";
}
impl IScrollAnchorProvider_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IScrollAnchorProvider_Impl,
        const OFFSET: isize,
    >() -> IScrollAnchorProvider_Vtbl {
        unsafe extern "system" fn CurrentAnchor<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IScrollAnchorProvider_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentAnchor() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterAnchorCandidate<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IScrollAnchorProvider_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            element: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RegisterAnchorCandidate(::windows_core::from_raw_borrowed(&element)).into()
        }
        unsafe extern "system" fn UnregisterAnchorCandidate<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IScrollAnchorProvider_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            element: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UnregisterAnchorCandidate(::windows_core::from_raw_borrowed(&element))
                .into()
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<Identity, IScrollAnchorProvider, OFFSET>(
            ),
            CurrentAnchor: CurrentAnchor::<Identity, Impl, OFFSET>,
            RegisterAnchorCandidate: RegisterAnchorCandidate::<Identity, Impl, OFFSET>,
            UnregisterAnchorCandidate: UnregisterAnchorCandidate::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IScrollAnchorProvider as ::windows_core::ComInterface>::IID
    }
}
pub trait ISemanticZoomInformation_Impl: Sized {
    fn SemanticZoomOwner(&self) -> ::windows_core::Result<SemanticZoom>;
    fn SetSemanticZoomOwner(
        &self,
        value: ::core::option::Option<&SemanticZoom>,
    ) -> ::windows_core::Result<()>;
    fn IsActiveView(&self) -> ::windows_core::Result<bool>;
    fn SetIsActiveView(&self, value: bool) -> ::windows_core::Result<()>;
    fn IsZoomedInView(&self) -> ::windows_core::Result<bool>;
    fn SetIsZoomedInView(&self, value: bool) -> ::windows_core::Result<()>;
    fn InitializeViewChange(&self) -> ::windows_core::Result<()>;
    fn CompleteViewChange(&self) -> ::windows_core::Result<()>;
    fn MakeVisible(
        &self,
        item: ::core::option::Option<&SemanticZoomLocation>,
    ) -> ::windows_core::Result<()>;
    fn StartViewChangeFrom(
        &self,
        source: ::core::option::Option<&SemanticZoomLocation>,
        destination: ::core::option::Option<&SemanticZoomLocation>,
    ) -> ::windows_core::Result<()>;
    fn StartViewChangeTo(
        &self,
        source: ::core::option::Option<&SemanticZoomLocation>,
        destination: ::core::option::Option<&SemanticZoomLocation>,
    ) -> ::windows_core::Result<()>;
    fn CompleteViewChangeFrom(
        &self,
        source: ::core::option::Option<&SemanticZoomLocation>,
        destination: ::core::option::Option<&SemanticZoomLocation>,
    ) -> ::windows_core::Result<()>;
    fn CompleteViewChangeTo(
        &self,
        source: ::core::option::Option<&SemanticZoomLocation>,
        destination: ::core::option::Option<&SemanticZoomLocation>,
    ) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for ISemanticZoomInformation {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.ISemanticZoomInformation";
}
impl ISemanticZoomInformation_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: ISemanticZoomInformation_Impl,
        const OFFSET: isize,
    >() -> ISemanticZoomInformation_Vtbl {
        unsafe extern "system" fn SemanticZoomOwner<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: ISemanticZoomInformation_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SemanticZoomOwner() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSemanticZoomOwner<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: ISemanticZoomInformation_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            value: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSemanticZoomOwner(::windows_core::from_raw_borrowed(&value)).into()
        }
        unsafe extern "system" fn IsActiveView<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: ISemanticZoomInformation_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut bool,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsActiveView() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsActiveView<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: ISemanticZoomInformation_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            value: bool,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetIsActiveView(value).into()
        }
        unsafe extern "system" fn IsZoomedInView<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: ISemanticZoomInformation_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut bool,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsZoomedInView() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsZoomedInView<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: ISemanticZoomInformation_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            value: bool,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetIsZoomedInView(value).into()
        }
        unsafe extern "system" fn InitializeViewChange<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: ISemanticZoomInformation_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.InitializeViewChange().into()
        }
        unsafe extern "system" fn CompleteViewChange<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: ISemanticZoomInformation_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CompleteViewChange().into()
        }
        unsafe extern "system" fn MakeVisible<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: ISemanticZoomInformation_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            item: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.MakeVisible(::windows_core::from_raw_borrowed(&item)).into()
        }
        unsafe extern "system" fn StartViewChangeFrom<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: ISemanticZoomInformation_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            source: *mut ::core::ffi::c_void,
            destination: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.StartViewChangeFrom(
                ::windows_core::from_raw_borrowed(&source),
                ::windows_core::from_raw_borrowed(&destination),
            )
            .into()
        }
        unsafe extern "system" fn StartViewChangeTo<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: ISemanticZoomInformation_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            source: *mut ::core::ffi::c_void,
            destination: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.StartViewChangeTo(
                ::windows_core::from_raw_borrowed(&source),
                ::windows_core::from_raw_borrowed(&destination),
            )
            .into()
        }
        unsafe extern "system" fn CompleteViewChangeFrom<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: ISemanticZoomInformation_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            source: *mut ::core::ffi::c_void,
            destination: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CompleteViewChangeFrom(
                ::windows_core::from_raw_borrowed(&source),
                ::windows_core::from_raw_borrowed(&destination),
            )
            .into()
        }
        unsafe extern "system" fn CompleteViewChangeTo<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: ISemanticZoomInformation_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            source: *mut ::core::ffi::c_void,
            destination: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CompleteViewChangeTo(
                ::windows_core::from_raw_borrowed(&source),
                ::windows_core::from_raw_borrowed(&destination),
            )
            .into()
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<
                Identity,
                ISemanticZoomInformation,
                OFFSET,
            >(),
            SemanticZoomOwner: SemanticZoomOwner::<Identity, Impl, OFFSET>,
            SetSemanticZoomOwner: SetSemanticZoomOwner::<Identity, Impl, OFFSET>,
            IsActiveView: IsActiveView::<Identity, Impl, OFFSET>,
            SetIsActiveView: SetIsActiveView::<Identity, Impl, OFFSET>,
            IsZoomedInView: IsZoomedInView::<Identity, Impl, OFFSET>,
            SetIsZoomedInView: SetIsZoomedInView::<Identity, Impl, OFFSET>,
            InitializeViewChange: InitializeViewChange::<Identity, Impl, OFFSET>,
            CompleteViewChange: CompleteViewChange::<Identity, Impl, OFFSET>,
            MakeVisible: MakeVisible::<Identity, Impl, OFFSET>,
            StartViewChangeFrom: StartViewChangeFrom::<Identity, Impl, OFFSET>,
            StartViewChangeTo: StartViewChangeTo::<Identity, Impl, OFFSET>,
            CompleteViewChangeFrom: CompleteViewChangeFrom::<Identity, Impl, OFFSET>,
            CompleteViewChangeTo: CompleteViewChangeTo::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <ISemanticZoomInformation as ::windows_core::ComInterface>::IID
    }
}
pub trait IStyleSelectorOverrides_Impl: Sized {
    fn SelectStyleCore(
        &self,
        item: ::core::option::Option<&::windows_core::IInspectable>,
        container: ::core::option::Option<&super::DependencyObject>,
    ) -> ::windows_core::Result<super::Style>;
}
impl ::windows_core::RuntimeName for IStyleSelectorOverrides {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.IStyleSelectorOverrides";
}
impl IStyleSelectorOverrides_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IStyleSelectorOverrides_Impl,
        const OFFSET: isize,
    >() -> IStyleSelectorOverrides_Vtbl {
        unsafe extern "system" fn SelectStyleCore<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IStyleSelectorOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            item: *mut ::core::ffi::c_void,
            container: *mut ::core::ffi::c_void,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SelectStyleCore(
                ::windows_core::from_raw_borrowed(&item),
                ::windows_core::from_raw_borrowed(&container),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<
                Identity,
                IStyleSelectorOverrides,
                OFFSET,
            >(),
            SelectStyleCore: SelectStyleCore::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IStyleSelectorOverrides as ::windows_core::ComInterface>::IID
    }
}
pub trait IToggleSwitchOverrides_Impl: Sized {
    fn OnToggled(&self) -> ::windows_core::Result<()>;
    fn OnOnContentChanged(
        &self,
        oldcontent: ::core::option::Option<&::windows_core::IInspectable>,
        newcontent: ::core::option::Option<&::windows_core::IInspectable>,
    ) -> ::windows_core::Result<()>;
    fn OnOffContentChanged(
        &self,
        oldcontent: ::core::option::Option<&::windows_core::IInspectable>,
        newcontent: ::core::option::Option<&::windows_core::IInspectable>,
    ) -> ::windows_core::Result<()>;
    fn OnHeaderChanged(
        &self,
        oldcontent: ::core::option::Option<&::windows_core::IInspectable>,
        newcontent: ::core::option::Option<&::windows_core::IInspectable>,
    ) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IToggleSwitchOverrides {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.IToggleSwitchOverrides";
}
impl IToggleSwitchOverrides_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IToggleSwitchOverrides_Impl,
        const OFFSET: isize,
    >() -> IToggleSwitchOverrides_Vtbl {
        unsafe extern "system" fn OnToggled<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IToggleSwitchOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnToggled().into()
        }
        unsafe extern "system" fn OnOnContentChanged<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IToggleSwitchOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            oldcontent: *mut ::core::ffi::c_void,
            newcontent: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnOnContentChanged(
                ::windows_core::from_raw_borrowed(&oldcontent),
                ::windows_core::from_raw_borrowed(&newcontent),
            )
            .into()
        }
        unsafe extern "system" fn OnOffContentChanged<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IToggleSwitchOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            oldcontent: *mut ::core::ffi::c_void,
            newcontent: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnOffContentChanged(
                ::windows_core::from_raw_borrowed(&oldcontent),
                ::windows_core::from_raw_borrowed(&newcontent),
            )
            .into()
        }
        unsafe extern "system" fn OnHeaderChanged<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IToggleSwitchOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            oldcontent: *mut ::core::ffi::c_void,
            newcontent: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnHeaderChanged(
                ::windows_core::from_raw_borrowed(&oldcontent),
                ::windows_core::from_raw_borrowed(&newcontent),
            )
            .into()
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<
                Identity,
                IToggleSwitchOverrides,
                OFFSET,
            >(),
            OnToggled: OnToggled::<Identity, Impl, OFFSET>,
            OnOnContentChanged: OnOnContentChanged::<Identity, Impl, OFFSET>,
            OnOffContentChanged: OnOffContentChanged::<Identity, Impl, OFFSET>,
            OnHeaderChanged: OnHeaderChanged::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IToggleSwitchOverrides as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Windows_Foundation\"`"]
#[cfg(feature = "Windows_Foundation")]
pub trait IVirtualizingLayoutContextOverrides_Impl: Sized {
    fn ItemCountCore(&self) -> ::windows_core::Result<i32>;
    fn GetItemAtCore(&self, index: i32) -> ::windows_core::Result<::windows_core::IInspectable>;
    fn RealizationRectCore(&self) -> ::windows_core::Result<::windows::Foundation::Rect>;
    fn GetOrCreateElementAtCore(
        &self,
        index: i32,
        options: ElementRealizationOptions,
    ) -> ::windows_core::Result<super::UIElement>;
    fn RecycleElementCore(
        &self,
        element: ::core::option::Option<&super::UIElement>,
    ) -> ::windows_core::Result<()>;
    fn RecommendedAnchorIndexCore(&self) -> ::windows_core::Result<i32>;
    fn LayoutOriginCore(&self) -> ::windows_core::Result<::windows::Foundation::Point>;
    fn SetLayoutOriginCore(
        &self,
        value: &::windows::Foundation::Point,
    ) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Windows_Foundation")]
impl ::windows_core::RuntimeName for IVirtualizingLayoutContextOverrides {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.IVirtualizingLayoutContextOverrides";
}
#[cfg(feature = "Windows_Foundation")]
impl IVirtualizingLayoutContextOverrides_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IVirtualizingLayoutContextOverrides_Impl,
        const OFFSET: isize,
    >() -> IVirtualizingLayoutContextOverrides_Vtbl {
        unsafe extern "system" fn ItemCountCore<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IVirtualizingLayoutContextOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut i32,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ItemCountCore() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetItemAtCore<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IVirtualizingLayoutContextOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            index: i32,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetItemAtCore(index) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RealizationRectCore<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IVirtualizingLayoutContextOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::Foundation::Rect,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RealizationRectCore() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOrCreateElementAtCore<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IVirtualizingLayoutContextOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            index: i32,
            options: ElementRealizationOptions,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetOrCreateElementAtCore(index, options) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RecycleElementCore<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IVirtualizingLayoutContextOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            element: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RecycleElementCore(::windows_core::from_raw_borrowed(&element)).into()
        }
        unsafe extern "system" fn RecommendedAnchorIndexCore<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IVirtualizingLayoutContextOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut i32,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RecommendedAnchorIndexCore() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LayoutOriginCore<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IVirtualizingLayoutContextOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::Foundation::Point,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LayoutOriginCore() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLayoutOriginCore<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IVirtualizingLayoutContextOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            value: ::windows::Foundation::Point,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetLayoutOriginCore(::core::mem::transmute(&value)).into()
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<
                Identity,
                IVirtualizingLayoutContextOverrides,
                OFFSET,
            >(),
            ItemCountCore: ItemCountCore::<Identity, Impl, OFFSET>,
            GetItemAtCore: GetItemAtCore::<Identity, Impl, OFFSET>,
            RealizationRectCore: RealizationRectCore::<Identity, Impl, OFFSET>,
            GetOrCreateElementAtCore: GetOrCreateElementAtCore::<Identity, Impl, OFFSET>,
            RecycleElementCore: RecycleElementCore::<Identity, Impl, OFFSET>,
            RecommendedAnchorIndexCore: RecommendedAnchorIndexCore::<Identity, Impl, OFFSET>,
            LayoutOriginCore: LayoutOriginCore::<Identity, Impl, OFFSET>,
            SetLayoutOriginCore: SetLayoutOriginCore::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IVirtualizingLayoutContextOverrides as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Microsoft_UI_Xaml_Interop\"`, `\"Windows_Foundation\"`"]
#[cfg(all(feature = "Microsoft_UI_Xaml_Interop", feature = "Windows_Foundation"))]
pub trait IVirtualizingLayoutOverrides_Impl: Sized {
    fn InitializeForContextCore(
        &self,
        context: ::core::option::Option<&VirtualizingLayoutContext>,
    ) -> ::windows_core::Result<()>;
    fn UninitializeForContextCore(
        &self,
        context: ::core::option::Option<&VirtualizingLayoutContext>,
    ) -> ::windows_core::Result<()>;
    fn MeasureOverride(
        &self,
        context: ::core::option::Option<&VirtualizingLayoutContext>,
        availablesize: &::windows::Foundation::Size,
    ) -> ::windows_core::Result<::windows::Foundation::Size>;
    fn ArrangeOverride(
        &self,
        context: ::core::option::Option<&VirtualizingLayoutContext>,
        finalsize: &::windows::Foundation::Size,
    ) -> ::windows_core::Result<::windows::Foundation::Size>;
    fn OnItemsChangedCore(
        &self,
        context: ::core::option::Option<&VirtualizingLayoutContext>,
        source: ::core::option::Option<&::windows_core::IInspectable>,
        args: ::core::option::Option<&super::Interop::NotifyCollectionChangedEventArgs>,
    ) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Microsoft_UI_Xaml_Interop", feature = "Windows_Foundation"))]
impl ::windows_core::RuntimeName for IVirtualizingLayoutOverrides {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.IVirtualizingLayoutOverrides";
}
#[cfg(all(feature = "Microsoft_UI_Xaml_Interop", feature = "Windows_Foundation"))]
impl IVirtualizingLayoutOverrides_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IVirtualizingLayoutOverrides_Impl,
        const OFFSET: isize,
    >() -> IVirtualizingLayoutOverrides_Vtbl {
        unsafe extern "system" fn InitializeForContextCore<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IVirtualizingLayoutOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.InitializeForContextCore(::windows_core::from_raw_borrowed(&context))
                .into()
        }
        unsafe extern "system" fn UninitializeForContextCore<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IVirtualizingLayoutOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UninitializeForContextCore(::windows_core::from_raw_borrowed(&context))
                .into()
        }
        unsafe extern "system" fn MeasureOverride<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IVirtualizingLayoutOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            availablesize: ::windows::Foundation::Size,
            result__: *mut ::windows::Foundation::Size,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MeasureOverride(
                ::windows_core::from_raw_borrowed(&context),
                ::core::mem::transmute(&availablesize),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ArrangeOverride<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IVirtualizingLayoutOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            finalsize: ::windows::Foundation::Size,
            result__: *mut ::windows::Foundation::Size,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ArrangeOverride(
                ::windows_core::from_raw_borrowed(&context),
                ::core::mem::transmute(&finalsize),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnItemsChangedCore<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IVirtualizingLayoutOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            context: *mut ::core::ffi::c_void,
            source: *mut ::core::ffi::c_void,
            args: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnItemsChangedCore(
                ::windows_core::from_raw_borrowed(&context),
                ::windows_core::from_raw_borrowed(&source),
                ::windows_core::from_raw_borrowed(&args),
            )
            .into()
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<
                Identity,
                IVirtualizingLayoutOverrides,
                OFFSET,
            >(),
            InitializeForContextCore: InitializeForContextCore::<Identity, Impl, OFFSET>,
            UninitializeForContextCore: UninitializeForContextCore::<Identity, Impl, OFFSET>,
            MeasureOverride: MeasureOverride::<Identity, Impl, OFFSET>,
            ArrangeOverride: ArrangeOverride::<Identity, Impl, OFFSET>,
            OnItemsChangedCore: OnItemsChangedCore::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IVirtualizingLayoutOverrides as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Microsoft_UI_Xaml_Controls_Primitives\"`"]
#[cfg(feature = "Microsoft_UI_Xaml_Controls_Primitives")]
pub trait IVirtualizingPanelOverrides_Impl: Sized {
    fn OnItemsChanged(
        &self,
        sender: ::core::option::Option<&::windows_core::IInspectable>,
        args: ::core::option::Option<&Primitives::ItemsChangedEventArgs>,
    ) -> ::windows_core::Result<()>;
    fn OnClearChildren(&self) -> ::windows_core::Result<()>;
    fn BringIndexIntoView(&self, index: i32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Microsoft_UI_Xaml_Controls_Primitives")]
impl ::windows_core::RuntimeName for IVirtualizingPanelOverrides {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.IVirtualizingPanelOverrides";
}
#[cfg(feature = "Microsoft_UI_Xaml_Controls_Primitives")]
impl IVirtualizingPanelOverrides_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IVirtualizingPanelOverrides_Impl,
        const OFFSET: isize,
    >() -> IVirtualizingPanelOverrides_Vtbl {
        unsafe extern "system" fn OnItemsChanged<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IVirtualizingPanelOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            sender: *mut ::core::ffi::c_void,
            args: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnItemsChanged(
                ::windows_core::from_raw_borrowed(&sender),
                ::windows_core::from_raw_borrowed(&args),
            )
            .into()
        }
        unsafe extern "system" fn OnClearChildren<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IVirtualizingPanelOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnClearChildren().into()
        }
        unsafe extern "system" fn BringIndexIntoView<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IVirtualizingPanelOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            index: i32,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.BringIndexIntoView(index).into()
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<
                Identity,
                IVirtualizingPanelOverrides,
                OFFSET,
            >(),
            OnItemsChanged: OnItemsChanged::<Identity, Impl, OFFSET>,
            OnClearChildren: OnClearChildren::<Identity, Impl, OFFSET>,
            BringIndexIntoView: BringIndexIntoView::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IVirtualizingPanelOverrides as ::windows_core::ComInterface>::IID
    }
}
pub trait IVirtualizingStackPanelOverrides_Impl: Sized {
    fn OnCleanUpVirtualizedItem(
        &self,
        e: ::core::option::Option<&CleanUpVirtualizedItemEventArgs>,
    ) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IVirtualizingStackPanelOverrides {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.IVirtualizingStackPanelOverrides";
}
impl IVirtualizingStackPanelOverrides_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IVirtualizingStackPanelOverrides_Impl,
        const OFFSET: isize,
    >() -> IVirtualizingStackPanelOverrides_Vtbl {
        unsafe extern "system" fn OnCleanUpVirtualizedItem<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IVirtualizingStackPanelOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            e: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnCleanUpVirtualizedItem(::windows_core::from_raw_borrowed(&e)).into()
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<
                Identity,
                IVirtualizingStackPanelOverrides,
                OFFSET,
            >(),
            OnCleanUpVirtualizedItem: OnCleanUpVirtualizedItem::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IVirtualizingStackPanelOverrides as ::windows_core::ComInterface>::IID
    }
}
