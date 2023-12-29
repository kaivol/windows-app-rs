#[doc = "Required features: `\"Windows_Foundation_Collections\"`"]
#[cfg(feature = "Windows_Foundation_Collections")]
pub trait IResourceContext_Impl: Sized {
    fn QualifierValues(
        &self,
    ) -> ::windows_core::Result<
        ::windows::Foundation::Collections::IMap<::windows_core::HSTRING, ::windows_core::HSTRING>,
    >;
}
#[cfg(feature = "Windows_Foundation_Collections")]
impl ::windows_core::RuntimeName for IResourceContext {
    const NAME: &'static str = "Microsoft.Windows.ApplicationModel.Resources.IResourceContext";
}
#[cfg(feature = "Windows_Foundation_Collections")]
impl IResourceContext_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IResourceContext_Impl,
        const OFFSET: isize,
    >() -> IResourceContext_Vtbl {
        unsafe extern "system" fn QualifierValues<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IResourceContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.QualifierValues() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<Identity, IResourceContext, OFFSET>(),
            QualifierValues: QualifierValues::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IResourceContext as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Windows_Foundation\"`"]
#[cfg(feature = "Windows_Foundation")]
pub trait IResourceManager_Impl: Sized {
    fn MainResourceMap(&self) -> ::windows_core::Result<ResourceMap>;
    fn CreateResourceContext(&self) -> ::windows_core::Result<ResourceContext>;
    fn ResourceNotFound(
        &self,
        handler: ::core::option::Option<
            &::windows::Foundation::TypedEventHandler<ResourceManager, ResourceNotFoundEventArgs>,
        >,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>;
    fn RemoveResourceNotFound(
        &self,
        token: &::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Windows_Foundation")]
impl ::windows_core::RuntimeName for IResourceManager {
    const NAME: &'static str = "Microsoft.Windows.ApplicationModel.Resources.IResourceManager";
}
#[cfg(feature = "Windows_Foundation")]
impl IResourceManager_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IResourceManager_Impl,
        const OFFSET: isize,
    >() -> IResourceManager_Vtbl {
        unsafe extern "system" fn MainResourceMap<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IResourceManager_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MainResourceMap() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateResourceContext<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IResourceManager_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateResourceContext() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResourceNotFound<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IResourceManager_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            handler: *mut ::core::ffi::c_void,
            result__: *mut ::windows::Foundation::EventRegistrationToken,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ResourceNotFound(::windows_core::from_raw_borrowed(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveResourceNotFound<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IResourceManager_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            token: ::windows::Foundation::EventRegistrationToken,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveResourceNotFound(::core::mem::transmute(&token)).into()
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<Identity, IResourceManager, OFFSET>(),
            MainResourceMap: MainResourceMap::<Identity, Impl, OFFSET>,
            CreateResourceContext: CreateResourceContext::<Identity, Impl, OFFSET>,
            ResourceNotFound: ResourceNotFound::<Identity, Impl, OFFSET>,
            RemoveResourceNotFound: RemoveResourceNotFound::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IResourceManager as ::windows_core::ComInterface>::IID
    }
}
