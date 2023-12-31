pub trait IComponentConnector_Impl: Sized {
    fn Connect(
        &self,
        connectionid: i32,
        target: ::core::option::Option<&::windows_core::IInspectable>,
    ) -> ::windows_core::Result<()>;
    fn GetBindingConnector(
        &self,
        connectionid: i32,
        target: ::core::option::Option<&::windows_core::IInspectable>,
    ) -> ::windows_core::Result<IComponentConnector>;
}
impl ::windows_core::RuntimeName for IComponentConnector {
    const NAME: &'static str = "Microsoft.UI.Xaml.Markup.IComponentConnector";
}
impl IComponentConnector_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IComponentConnector_Impl,
        const OFFSET: isize,
    >() -> IComponentConnector_Vtbl {
        unsafe extern "system" fn Connect<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IComponentConnector_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            connectionid: i32,
            target: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Connect(connectionid, ::windows_core::from_raw_borrowed(&target)).into()
        }
        unsafe extern "system" fn GetBindingConnector<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IComponentConnector_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            connectionid: i32,
            target: *mut ::core::ffi::c_void,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetBindingConnector(connectionid, ::windows_core::from_raw_borrowed(&target))
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
            base__: ::windows_core::IInspectable_Vtbl::new::<Identity, IComponentConnector, OFFSET>(
            ),
            Connect: Connect::<Identity, Impl, OFFSET>,
            GetBindingConnector: GetBindingConnector::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IComponentConnector as ::windows_core::ComInterface>::IID
    }
}
pub trait IDataTemplateComponent_Impl: Sized {
    fn Recycle(&self) -> ::windows_core::Result<()>;
    fn ProcessBindings(
        &self,
        item: ::core::option::Option<&::windows_core::IInspectable>,
        itemindex: i32,
        phase: i32,
        nextphase: &mut i32,
    ) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IDataTemplateComponent {
    const NAME: &'static str = "Microsoft.UI.Xaml.Markup.IDataTemplateComponent";
}
impl IDataTemplateComponent_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IDataTemplateComponent_Impl,
        const OFFSET: isize,
    >() -> IDataTemplateComponent_Vtbl {
        unsafe extern "system" fn Recycle<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IDataTemplateComponent_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Recycle().into()
        }
        unsafe extern "system" fn ProcessBindings<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IDataTemplateComponent_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            item: *mut ::core::ffi::c_void,
            itemindex: i32,
            phase: i32,
            nextphase: *mut i32,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ProcessBindings(
                ::windows_core::from_raw_borrowed(&item),
                itemindex,
                phase,
                ::core::mem::transmute_copy(&nextphase),
            )
            .into()
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<
                Identity,
                IDataTemplateComponent,
                OFFSET,
            >(),
            Recycle: Recycle::<Identity, Impl, OFFSET>,
            ProcessBindings: ProcessBindings::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IDataTemplateComponent as ::windows_core::ComInterface>::IID
    }
}
pub trait IMarkupExtensionOverrides_Impl: Sized {
    fn ProvideValue(&self) -> ::windows_core::Result<::windows_core::IInspectable>;
    fn ProvideValueWithIXamlServiceProvider(
        &self,
        serviceprovider: ::core::option::Option<&super::IXamlServiceProvider>,
    ) -> ::windows_core::Result<::windows_core::IInspectable>;
}
impl ::windows_core::RuntimeName for IMarkupExtensionOverrides {
    const NAME: &'static str = "Microsoft.UI.Xaml.Markup.IMarkupExtensionOverrides";
}
impl IMarkupExtensionOverrides_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IMarkupExtensionOverrides_Impl,
        const OFFSET: isize,
    >() -> IMarkupExtensionOverrides_Vtbl {
        unsafe extern "system" fn ProvideValue<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IMarkupExtensionOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ProvideValue() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProvideValueWithIXamlServiceProvider<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IMarkupExtensionOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            serviceprovider: *mut ::core::ffi::c_void,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ProvideValueWithIXamlServiceProvider(::windows_core::from_raw_borrowed(
                &serviceprovider,
            )) {
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
                IMarkupExtensionOverrides,
                OFFSET,
            >(),
            ProvideValue: ProvideValue::<Identity, Impl, OFFSET>,
            ProvideValueWithIXamlServiceProvider: ProvideValueWithIXamlServiceProvider::<
                Identity,
                Impl,
                OFFSET,
            >,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IMarkupExtensionOverrides as ::windows_core::ComInterface>::IID
    }
}
pub trait IProvideValueTarget_Impl: Sized {
    fn TargetObject(&self) -> ::windows_core::Result<::windows_core::IInspectable>;
    fn TargetProperty(&self) -> ::windows_core::Result<::windows_core::IInspectable>;
}
impl ::windows_core::RuntimeName for IProvideValueTarget {
    const NAME: &'static str = "Microsoft.UI.Xaml.Markup.IProvideValueTarget";
}
impl IProvideValueTarget_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IProvideValueTarget_Impl,
        const OFFSET: isize,
    >() -> IProvideValueTarget_Vtbl {
        unsafe extern "system" fn TargetObject<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IProvideValueTarget_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.TargetObject() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TargetProperty<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IProvideValueTarget_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.TargetProperty() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<Identity, IProvideValueTarget, OFFSET>(
            ),
            TargetObject: TargetObject::<Identity, Impl, OFFSET>,
            TargetProperty: TargetProperty::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IProvideValueTarget as ::windows_core::ComInterface>::IID
    }
}
pub trait IRootObjectProvider_Impl: Sized {
    fn RootObject(&self) -> ::windows_core::Result<::windows_core::IInspectable>;
}
impl ::windows_core::RuntimeName for IRootObjectProvider {
    const NAME: &'static str = "Microsoft.UI.Xaml.Markup.IRootObjectProvider";
}
impl IRootObjectProvider_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IRootObjectProvider_Impl,
        const OFFSET: isize,
    >() -> IRootObjectProvider_Vtbl {
        unsafe extern "system" fn RootObject<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IRootObjectProvider_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RootObject() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<Identity, IRootObjectProvider, OFFSET>(
            ),
            RootObject: RootObject::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IRootObjectProvider as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Windows_Foundation\"`"]
#[cfg(feature = "Windows_Foundation")]
pub trait IUriContext_Impl: Sized {
    fn BaseUri(&self) -> ::windows_core::Result<::windows::Foundation::Uri>;
}
#[cfg(feature = "Windows_Foundation")]
impl ::windows_core::RuntimeName for IUriContext {
    const NAME: &'static str = "Microsoft.UI.Xaml.Markup.IUriContext";
}
#[cfg(feature = "Windows_Foundation")]
impl IUriContext_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IUriContext_Impl,
        const OFFSET: isize,
    >() -> IUriContext_Vtbl {
        unsafe extern "system" fn BaseUri<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IUriContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BaseUri() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<Identity, IUriContext, OFFSET>(),
            BaseUri: BaseUri::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IUriContext as ::windows_core::ComInterface>::IID
    }
}
pub trait IXamlBindScopeDiagnostics_Impl: Sized {
    fn Disable(&self, linenumber: i32, columnnumber: i32) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IXamlBindScopeDiagnostics {
    const NAME: &'static str = "Microsoft.UI.Xaml.Markup.IXamlBindScopeDiagnostics";
}
impl IXamlBindScopeDiagnostics_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IXamlBindScopeDiagnostics_Impl,
        const OFFSET: isize,
    >() -> IXamlBindScopeDiagnostics_Vtbl {
        unsafe extern "system" fn Disable<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IXamlBindScopeDiagnostics_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            linenumber: i32,
            columnnumber: i32,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Disable(linenumber, columnnumber).into()
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<
                Identity,
                IXamlBindScopeDiagnostics,
                OFFSET,
            >(),
            Disable: Disable::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IXamlBindScopeDiagnostics as ::windows_core::ComInterface>::IID
    }
}
pub trait IXamlMember_Impl: Sized {
    fn IsAttachable(&self) -> ::windows_core::Result<bool>;
    fn IsDependencyProperty(&self) -> ::windows_core::Result<bool>;
    fn IsReadOnly(&self) -> ::windows_core::Result<bool>;
    fn Name(&self) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn TargetType(&self) -> ::windows_core::Result<IXamlType>;
    fn Type(&self) -> ::windows_core::Result<IXamlType>;
    fn GetValue(
        &self,
        instance: ::core::option::Option<&::windows_core::IInspectable>,
    ) -> ::windows_core::Result<::windows_core::IInspectable>;
    fn SetValue(
        &self,
        instance: ::core::option::Option<&::windows_core::IInspectable>,
        value: ::core::option::Option<&::windows_core::IInspectable>,
    ) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IXamlMember {
    const NAME: &'static str = "Microsoft.UI.Xaml.Markup.IXamlMember";
}
impl IXamlMember_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IXamlMember_Impl,
        const OFFSET: isize,
    >() -> IXamlMember_Vtbl {
        unsafe extern "system" fn IsAttachable<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IXamlMember_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut bool,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsAttachable() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsDependencyProperty<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IXamlMember_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut bool,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsDependencyProperty() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsReadOnly<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IXamlMember_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut bool,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsReadOnly() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IXamlMember_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Name() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TargetType<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IXamlMember_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.TargetType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Type<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IXamlMember_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Type() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetValue<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IXamlMember_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            instance: *mut ::core::ffi::c_void,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetValue(::windows_core::from_raw_borrowed(&instance)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValue<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IXamlMember_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            instance: *mut ::core::ffi::c_void,
            value: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetValue(
                ::windows_core::from_raw_borrowed(&instance),
                ::windows_core::from_raw_borrowed(&value),
            )
            .into()
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<Identity, IXamlMember, OFFSET>(),
            IsAttachable: IsAttachable::<Identity, Impl, OFFSET>,
            IsDependencyProperty: IsDependencyProperty::<Identity, Impl, OFFSET>,
            IsReadOnly: IsReadOnly::<Identity, Impl, OFFSET>,
            Name: Name::<Identity, Impl, OFFSET>,
            TargetType: TargetType::<Identity, Impl, OFFSET>,
            Type: Type::<Identity, Impl, OFFSET>,
            GetValue: GetValue::<Identity, Impl, OFFSET>,
            SetValue: SetValue::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IXamlMember as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Windows_UI_Xaml_Interop\"`"]
#[cfg(feature = "Windows_UI_Xaml_Interop")]
pub trait IXamlMetadataProvider_Impl: Sized {
    fn GetXamlType(
        &self,
        r#type: &super::super::super::super::Windows::UI::Xaml::Interop::TypeName,
    ) -> ::windows_core::Result<IXamlType>;
    fn GetXamlTypeByFullName(
        &self,
        fullname: &::windows_core::HSTRING,
    ) -> ::windows_core::Result<IXamlType>;
    fn GetXmlnsDefinitions(&self)
        -> ::windows_core::Result<::windows_core::Array<XmlnsDefinition>>;
}
#[cfg(feature = "Windows_UI_Xaml_Interop")]
impl ::windows_core::RuntimeName for IXamlMetadataProvider {
    const NAME: &'static str = "Microsoft.UI.Xaml.Markup.IXamlMetadataProvider";
}
#[cfg(feature = "Windows_UI_Xaml_Interop")]
impl IXamlMetadataProvider_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IXamlMetadataProvider_Impl,
        const OFFSET: isize,
    >() -> IXamlMetadataProvider_Vtbl {
        unsafe extern "system" fn GetXamlType<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IXamlMetadataProvider_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            r#type: ::std::mem::MaybeUninit<
                super::super::super::super::Windows::UI::Xaml::Interop::TypeName,
            >,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetXamlType(::core::mem::transmute(&r#type)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetXamlTypeByFullName<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IXamlMetadataProvider_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            fullname: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetXamlTypeByFullName(::core::mem::transmute(&fullname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetXmlnsDefinitions<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IXamlMetadataProvider_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result_size__: *mut u32,
            result__: *mut *mut ::std::mem::MaybeUninit<XmlnsDefinition>,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetXmlnsDefinitions() {
                ::core::result::Result::Ok(ok__) => {
                    let (ok_data__, ok_data_len__) = ok__.into_abi();
                    ::core::ptr::write(result__, ok_data__);
                    ::core::ptr::write(result_size__, ok_data_len__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<Identity, IXamlMetadataProvider, OFFSET>(
            ),
            GetXamlType: GetXamlType::<Identity, Impl, OFFSET>,
            GetXamlTypeByFullName: GetXamlTypeByFullName::<Identity, Impl, OFFSET>,
            GetXmlnsDefinitions: GetXmlnsDefinitions::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IXamlMetadataProvider as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Windows_UI_Xaml_Interop\"`"]
#[cfg(feature = "Windows_UI_Xaml_Interop")]
pub trait IXamlType_Impl: Sized {
    fn BaseType(&self) -> ::windows_core::Result<IXamlType>;
    fn ContentProperty(&self) -> ::windows_core::Result<IXamlMember>;
    fn FullName(&self) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn IsArray(&self) -> ::windows_core::Result<bool>;
    fn IsCollection(&self) -> ::windows_core::Result<bool>;
    fn IsConstructible(&self) -> ::windows_core::Result<bool>;
    fn IsDictionary(&self) -> ::windows_core::Result<bool>;
    fn IsMarkupExtension(&self) -> ::windows_core::Result<bool>;
    fn IsBindable(&self) -> ::windows_core::Result<bool>;
    fn ItemType(&self) -> ::windows_core::Result<IXamlType>;
    fn KeyType(&self) -> ::windows_core::Result<IXamlType>;
    fn BoxedType(&self) -> ::windows_core::Result<IXamlType>;
    fn UnderlyingType(
        &self,
    ) -> ::windows_core::Result<super::super::super::super::Windows::UI::Xaml::Interop::TypeName>;
    fn ActivateInstance(&self) -> ::windows_core::Result<::windows_core::IInspectable>;
    fn CreateFromString(
        &self,
        value: &::windows_core::HSTRING,
    ) -> ::windows_core::Result<::windows_core::IInspectable>;
    fn GetMember(&self, name: &::windows_core::HSTRING) -> ::windows_core::Result<IXamlMember>;
    fn AddToVector(
        &self,
        instance: ::core::option::Option<&::windows_core::IInspectable>,
        value: ::core::option::Option<&::windows_core::IInspectable>,
    ) -> ::windows_core::Result<()>;
    fn AddToMap(
        &self,
        instance: ::core::option::Option<&::windows_core::IInspectable>,
        key: ::core::option::Option<&::windows_core::IInspectable>,
        value: ::core::option::Option<&::windows_core::IInspectable>,
    ) -> ::windows_core::Result<()>;
    fn RunInitializer(&self) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Windows_UI_Xaml_Interop")]
impl ::windows_core::RuntimeName for IXamlType {
    const NAME: &'static str = "Microsoft.UI.Xaml.Markup.IXamlType";
}
#[cfg(feature = "Windows_UI_Xaml_Interop")]
impl IXamlType_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IXamlType_Impl,
        const OFFSET: isize,
    >() -> IXamlType_Vtbl {
        unsafe extern "system" fn BaseType<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IXamlType_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BaseType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ContentProperty<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IXamlType_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ContentProperty() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FullName<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IXamlType_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FullName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsArray<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IXamlType_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut bool,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsArray() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsCollection<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IXamlType_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut bool,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsCollection() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsConstructible<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IXamlType_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut bool,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsConstructible() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsDictionary<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IXamlType_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut bool,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsDictionary() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsMarkupExtension<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IXamlType_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut bool,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsMarkupExtension() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsBindable<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IXamlType_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut bool,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsBindable() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ItemType<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IXamlType_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ItemType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn KeyType<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IXamlType_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.KeyType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BoxedType<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IXamlType_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BoxedType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnderlyingType<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IXamlType_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::std::mem::MaybeUninit<
                super::super::super::super::Windows::UI::Xaml::Interop::TypeName,
            >,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.UnderlyingType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ActivateInstance<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IXamlType_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ActivateInstance() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFromString<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IXamlType_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            value: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateFromString(::core::mem::transmute(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMember<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IXamlType_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            name: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetMember(::core::mem::transmute(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddToVector<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IXamlType_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            instance: *mut ::core::ffi::c_void,
            value: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddToVector(
                ::windows_core::from_raw_borrowed(&instance),
                ::windows_core::from_raw_borrowed(&value),
            )
            .into()
        }
        unsafe extern "system" fn AddToMap<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IXamlType_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            instance: *mut ::core::ffi::c_void,
            key: *mut ::core::ffi::c_void,
            value: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddToMap(
                ::windows_core::from_raw_borrowed(&instance),
                ::windows_core::from_raw_borrowed(&key),
                ::windows_core::from_raw_borrowed(&value),
            )
            .into()
        }
        unsafe extern "system" fn RunInitializer<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IXamlType_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RunInitializer().into()
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<Identity, IXamlType, OFFSET>(),
            BaseType: BaseType::<Identity, Impl, OFFSET>,
            ContentProperty: ContentProperty::<Identity, Impl, OFFSET>,
            FullName: FullName::<Identity, Impl, OFFSET>,
            IsArray: IsArray::<Identity, Impl, OFFSET>,
            IsCollection: IsCollection::<Identity, Impl, OFFSET>,
            IsConstructible: IsConstructible::<Identity, Impl, OFFSET>,
            IsDictionary: IsDictionary::<Identity, Impl, OFFSET>,
            IsMarkupExtension: IsMarkupExtension::<Identity, Impl, OFFSET>,
            IsBindable: IsBindable::<Identity, Impl, OFFSET>,
            ItemType: ItemType::<Identity, Impl, OFFSET>,
            KeyType: KeyType::<Identity, Impl, OFFSET>,
            BoxedType: BoxedType::<Identity, Impl, OFFSET>,
            UnderlyingType: UnderlyingType::<Identity, Impl, OFFSET>,
            ActivateInstance: ActivateInstance::<Identity, Impl, OFFSET>,
            CreateFromString: CreateFromString::<Identity, Impl, OFFSET>,
            GetMember: GetMember::<Identity, Impl, OFFSET>,
            AddToVector: AddToVector::<Identity, Impl, OFFSET>,
            AddToMap: AddToMap::<Identity, Impl, OFFSET>,
            RunInitializer: RunInitializer::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IXamlType as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Windows_UI_Xaml_Interop\"`"]
#[cfg(feature = "Windows_UI_Xaml_Interop")]
pub trait IXamlTypeResolver_Impl: Sized {
    fn Resolve(
        &self,
        qualifiedtypename: &::windows_core::HSTRING,
    ) -> ::windows_core::Result<super::super::super::super::Windows::UI::Xaml::Interop::TypeName>;
}
#[cfg(feature = "Windows_UI_Xaml_Interop")]
impl ::windows_core::RuntimeName for IXamlTypeResolver {
    const NAME: &'static str = "Microsoft.UI.Xaml.Markup.IXamlTypeResolver";
}
#[cfg(feature = "Windows_UI_Xaml_Interop")]
impl IXamlTypeResolver_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IXamlTypeResolver_Impl,
        const OFFSET: isize,
    >() -> IXamlTypeResolver_Vtbl {
        unsafe extern "system" fn Resolve<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IXamlTypeResolver_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            qualifiedtypename: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
            result__: *mut ::std::mem::MaybeUninit<
                super::super::super::super::Windows::UI::Xaml::Interop::TypeName,
            >,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Resolve(::core::mem::transmute(&qualifiedtypename)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<Identity, IXamlTypeResolver, OFFSET>(),
            Resolve: Resolve::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IXamlTypeResolver as ::windows_core::ComInterface>::IID
    }
}
