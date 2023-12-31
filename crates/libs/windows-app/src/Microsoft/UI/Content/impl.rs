#[doc = "Required features: `\"Microsoft_UI_Dispatching\"`, `\"Windows_Foundation\"`"]
#[cfg(all(feature = "Microsoft_UI_Dispatching", feature = "Windows_Foundation"))]
pub trait IContentSiteBridge_Impl: Sized + ::windows::Foundation::IClosable_Impl {
    fn DispatcherQueue(&self) -> ::windows_core::Result<super::Dispatching::DispatcherQueue>;
    fn LayoutDirectionOverride(
        &self,
    ) -> ::windows_core::Result<::windows::Foundation::IReference<ContentLayoutDirection>>;
    fn SetLayoutDirectionOverride(
        &self,
        value: ::core::option::Option<&::windows::Foundation::IReference<ContentLayoutDirection>>,
    ) -> ::windows_core::Result<()>;
    fn OverrideScale(&self) -> ::windows_core::Result<f32>;
    fn SetOverrideScale(&self, value: f32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Microsoft_UI_Dispatching", feature = "Windows_Foundation"))]
impl ::windows_core::RuntimeName for IContentSiteBridge {
    const NAME: &'static str = "Microsoft.UI.Content.IContentSiteBridge";
}
#[cfg(all(feature = "Microsoft_UI_Dispatching", feature = "Windows_Foundation"))]
impl IContentSiteBridge_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IContentSiteBridge_Impl,
        const OFFSET: isize,
    >() -> IContentSiteBridge_Vtbl {
        unsafe extern "system" fn DispatcherQueue<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IContentSiteBridge_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DispatcherQueue() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LayoutDirectionOverride<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IContentSiteBridge_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LayoutDirectionOverride() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLayoutDirectionOverride<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IContentSiteBridge_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            value: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetLayoutDirectionOverride(::windows_core::from_raw_borrowed(&value))
                .into()
        }
        unsafe extern "system" fn OverrideScale<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IContentSiteBridge_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut f32,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.OverrideScale() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOverrideScale<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IContentSiteBridge_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            value: f32,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetOverrideScale(value).into()
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<Identity, IContentSiteBridge, OFFSET>(
            ),
            DispatcherQueue: DispatcherQueue::<Identity, Impl, OFFSET>,
            LayoutDirectionOverride: LayoutDirectionOverride::<Identity, Impl, OFFSET>,
            SetLayoutDirectionOverride: SetLayoutDirectionOverride::<Identity, Impl, OFFSET>,
            OverrideScale: OverrideScale::<Identity, Impl, OFFSET>,
            SetOverrideScale: SetOverrideScale::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IContentSiteBridge as ::windows_core::ComInterface>::IID
    }
}
