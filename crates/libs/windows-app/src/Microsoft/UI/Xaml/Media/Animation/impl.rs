pub trait INavigationTransitionInfoOverrides_Impl: Sized {
    fn GetNavigationStateCore(&self) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn SetNavigationStateCore(
        &self,
        navigationstate: &::windows_core::HSTRING,
    ) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for INavigationTransitionInfoOverrides {
    const NAME: &'static str =
        "Microsoft.UI.Xaml.Media.Animation.INavigationTransitionInfoOverrides";
}
impl INavigationTransitionInfoOverrides_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: INavigationTransitionInfoOverrides_Impl,
        const OFFSET: isize,
    >() -> INavigationTransitionInfoOverrides_Vtbl {
        unsafe extern "system" fn GetNavigationStateCore<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: INavigationTransitionInfoOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetNavigationStateCore() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNavigationStateCore<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: INavigationTransitionInfoOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            navigationstate: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetNavigationStateCore(::core::mem::transmute(&navigationstate)).into()
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<
                Identity,
                INavigationTransitionInfoOverrides,
                OFFSET,
            >(),
            GetNavigationStateCore: GetNavigationStateCore::<Identity, Impl, OFFSET>,
            SetNavigationStateCore: SetNavigationStateCore::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <INavigationTransitionInfoOverrides as ::windows_core::ComInterface>::IID
    }
}
