#[doc = "Required features: `\"Windows_Foundation\"`"]
#[cfg(feature = "Windows_Foundation")]
pub trait ICommand_Impl: Sized {
    fn CanExecuteChanged(
        &self,
        handler: ::core::option::Option<
            &::windows::Foundation::EventHandler<::windows_core::IInspectable>,
        >,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>;
    fn RemoveCanExecuteChanged(
        &self,
        token: &::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()>;
    fn CanExecute(
        &self,
        parameter: ::core::option::Option<&::windows_core::IInspectable>,
    ) -> ::windows_core::Result<bool>;
    fn Execute(
        &self,
        parameter: ::core::option::Option<&::windows_core::IInspectable>,
    ) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Windows_Foundation")]
impl ::windows_core::RuntimeName for ICommand {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.ICommand";
}
#[cfg(feature = "Windows_Foundation")]
impl ICommand_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: ICommand_Impl,
        const OFFSET: isize,
    >() -> ICommand_Vtbl {
        unsafe extern "system" fn CanExecuteChanged<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: ICommand_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            handler: *mut ::core::ffi::c_void,
            result__: *mut ::windows::Foundation::EventRegistrationToken,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CanExecuteChanged(::windows_core::from_raw_borrowed(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveCanExecuteChanged<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: ICommand_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            token: ::windows::Foundation::EventRegistrationToken,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveCanExecuteChanged(::core::mem::transmute(&token)).into()
        }
        unsafe extern "system" fn CanExecute<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: ICommand_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            parameter: *mut ::core::ffi::c_void,
            result__: *mut bool,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CanExecute(::windows_core::from_raw_borrowed(&parameter)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Execute<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: ICommand_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            parameter: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Execute(::windows_core::from_raw_borrowed(&parameter)).into()
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<Identity, ICommand, OFFSET>(),
            CanExecuteChanged: CanExecuteChanged::<Identity, Impl, OFFSET>,
            RemoveCanExecuteChanged: RemoveCanExecuteChanged::<Identity, Impl, OFFSET>,
            CanExecute: CanExecute::<Identity, Impl, OFFSET>,
            Execute: Execute::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <ICommand as ::windows_core::ComInterface>::IID
    }
}
