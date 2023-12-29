#[doc = "Required features: `\"Windows_Foundation\"`"]
#[cfg(feature = "Windows_Foundation")]
pub trait IPointerPointTransform_Impl: Sized {
    fn Inverse(&self) -> ::windows_core::Result<IPointerPointTransform>;
    fn TryTransform(
        &self,
        inpoint: &::windows::Foundation::Point,
        outpoint: &mut ::windows::Foundation::Point,
    ) -> ::windows_core::Result<bool>;
    fn TryTransformBounds(
        &self,
        inrect: &::windows::Foundation::Rect,
        outrect: &mut ::windows::Foundation::Rect,
    ) -> ::windows_core::Result<bool>;
}
#[cfg(feature = "Windows_Foundation")]
impl ::windows_core::RuntimeName for IPointerPointTransform {
    const NAME: &'static str = "Microsoft.UI.Input.IPointerPointTransform";
}
#[cfg(feature = "Windows_Foundation")]
impl IPointerPointTransform_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IPointerPointTransform_Impl,
        const OFFSET: isize,
    >() -> IPointerPointTransform_Vtbl {
        unsafe extern "system" fn Inverse<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IPointerPointTransform_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Inverse() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryTransform<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IPointerPointTransform_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            inpoint: ::windows::Foundation::Point,
            outpoint: *mut ::windows::Foundation::Point,
            result__: *mut bool,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.TryTransform(
                ::core::mem::transmute(&inpoint),
                ::core::mem::transmute_copy(&outpoint),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryTransformBounds<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IPointerPointTransform_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            inrect: ::windows::Foundation::Rect,
            outrect: *mut ::windows::Foundation::Rect,
            result__: *mut bool,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.TryTransformBounds(
                ::core::mem::transmute(&inrect),
                ::core::mem::transmute_copy(&outrect),
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
                IPointerPointTransform,
                OFFSET,
            >(),
            Inverse: Inverse::<Identity, Impl, OFFSET>,
            TryTransform: TryTransform::<Identity, Impl, OFFSET>,
            TryTransformBounds: TryTransformBounds::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IPointerPointTransform as ::windows_core::ComInterface>::IID
    }
}
