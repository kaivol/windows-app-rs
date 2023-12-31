pub trait ITextElementOverrides_Impl: Sized {
    fn OnDisconnectVisualChildren(&self) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for ITextElementOverrides {
    const NAME: &'static str = "Microsoft.UI.Xaml.Documents.ITextElementOverrides";
}
impl ITextElementOverrides_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: ITextElementOverrides_Impl,
        const OFFSET: isize,
    >() -> ITextElementOverrides_Vtbl {
        unsafe extern "system" fn OnDisconnectVisualChildren<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: ITextElementOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnDisconnectVisualChildren().into()
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<Identity, ITextElementOverrides, OFFSET>(
            ),
            OnDisconnectVisualChildren: OnDisconnectVisualChildren::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <ITextElementOverrides as ::windows_core::ComInterface>::IID
    }
}
