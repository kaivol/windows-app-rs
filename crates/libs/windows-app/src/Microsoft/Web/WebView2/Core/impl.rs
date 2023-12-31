pub trait ICoreWebView2DispatchAdapter_Impl: Sized {
    fn WrapNamedObject(
        &self,
        name: &::windows_core::HSTRING,
        adapter: ::core::option::Option<&ICoreWebView2DispatchAdapter>,
    ) -> ::windows_core::Result<::windows_core::IInspectable>;
    fn WrapObject(
        &self,
        unwrapped: ::core::option::Option<&::windows_core::IInspectable>,
        adapter: ::core::option::Option<&ICoreWebView2DispatchAdapter>,
    ) -> ::windows_core::Result<::windows_core::IInspectable>;
    fn UnwrapObject(
        &self,
        wrapped: ::core::option::Option<&::windows_core::IInspectable>,
    ) -> ::windows_core::Result<::windows_core::IInspectable>;
    fn Clean(&self) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for ICoreWebView2DispatchAdapter {
    const NAME: &'static str = "Microsoft.Web.WebView2.Core.ICoreWebView2DispatchAdapter";
}
impl ICoreWebView2DispatchAdapter_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: ICoreWebView2DispatchAdapter_Impl,
        const OFFSET: isize,
    >() -> ICoreWebView2DispatchAdapter_Vtbl {
        unsafe extern "system" fn WrapNamedObject<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: ICoreWebView2DispatchAdapter_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            name: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
            adapter: *mut ::core::ffi::c_void,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.WrapNamedObject(
                ::core::mem::transmute(&name),
                ::windows_core::from_raw_borrowed(&adapter),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WrapObject<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: ICoreWebView2DispatchAdapter_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            unwrapped: *mut ::core::ffi::c_void,
            adapter: *mut ::core::ffi::c_void,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.WrapObject(
                ::windows_core::from_raw_borrowed(&unwrapped),
                ::windows_core::from_raw_borrowed(&adapter),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnwrapObject<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: ICoreWebView2DispatchAdapter_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            wrapped: *mut ::core::ffi::c_void,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.UnwrapObject(::windows_core::from_raw_borrowed(&wrapped)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clean<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: ICoreWebView2DispatchAdapter_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Clean().into()
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<
                Identity,
                ICoreWebView2DispatchAdapter,
                OFFSET,
            >(),
            WrapNamedObject: WrapNamedObject::<Identity, Impl, OFFSET>,
            WrapObject: WrapObject::<Identity, Impl, OFFSET>,
            UnwrapObject: UnwrapObject::<Identity, Impl, OFFSET>,
            Clean: Clean::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <ICoreWebView2DispatchAdapter as ::windows_core::ComInterface>::IID
    }
}
