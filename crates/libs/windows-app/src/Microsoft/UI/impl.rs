#[doc = "Required features: `\"Windows_Foundation\"`"]
#[cfg(feature = "Windows_Foundation")]
pub trait IClosableNotifier_Impl: Sized {
    fn IsClosed(&self) -> ::windows_core::Result<bool>;
    fn Closed(
        &self,
        handler: ::core::option::Option<&ClosableNotifierHandler>,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>;
    fn RemoveClosed(
        &self,
        token: &::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()>;
    fn FrameworkClosed(
        &self,
        handler: ::core::option::Option<&ClosableNotifierHandler>,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>;
    fn RemoveFrameworkClosed(
        &self,
        token: &::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Windows_Foundation")]
impl ::windows_core::RuntimeName for IClosableNotifier {
    const NAME: &'static str = "Microsoft.UI.IClosableNotifier";
}
#[cfg(feature = "Windows_Foundation")]
impl IClosableNotifier_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IClosableNotifier_Impl,
        const OFFSET: isize,
    >() -> IClosableNotifier_Vtbl {
        unsafe extern "system" fn IsClosed<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IClosableNotifier_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut bool,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsClosed() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Closed<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IClosableNotifier_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            handler: *mut ::core::ffi::c_void,
            result__: *mut ::windows::Foundation::EventRegistrationToken,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Closed(::windows_core::from_raw_borrowed(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveClosed<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IClosableNotifier_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            token: ::windows::Foundation::EventRegistrationToken,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveClosed(::core::mem::transmute(&token)).into()
        }
        unsafe extern "system" fn FrameworkClosed<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IClosableNotifier_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            handler: *mut ::core::ffi::c_void,
            result__: *mut ::windows::Foundation::EventRegistrationToken,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FrameworkClosed(::windows_core::from_raw_borrowed(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveFrameworkClosed<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IClosableNotifier_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            token: ::windows::Foundation::EventRegistrationToken,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveFrameworkClosed(::core::mem::transmute(&token)).into()
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<Identity, IClosableNotifier, OFFSET>(),
            IsClosed: IsClosed::<Identity, Impl, OFFSET>,
            Closed: Closed::<Identity, Impl, OFFSET>,
            RemoveClosed: RemoveClosed::<Identity, Impl, OFFSET>,
            FrameworkClosed: FrameworkClosed::<Identity, Impl, OFFSET>,
            RemoveFrameworkClosed: RemoveFrameworkClosed::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IClosableNotifier as ::windows_core::ComInterface>::IID
    }
}
