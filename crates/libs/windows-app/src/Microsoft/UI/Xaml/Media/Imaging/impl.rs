#[doc = "Required features: `\"Windows_ApplicationModel_Background\"`"]
#[cfg(feature = "Windows_ApplicationModel_Background")]
pub trait IXamlRenderingBackgroundTaskOverrides_Impl: Sized {
    fn OnRun(
        &self,
        taskinstance: ::core::option::Option<
            &::windows::ApplicationModel::Background::IBackgroundTaskInstance,
        >,
    ) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Windows_ApplicationModel_Background")]
impl ::windows_core::RuntimeName for IXamlRenderingBackgroundTaskOverrides {
    const NAME: &'static str =
        "Microsoft.UI.Xaml.Media.Imaging.IXamlRenderingBackgroundTaskOverrides";
}
#[cfg(feature = "Windows_ApplicationModel_Background")]
impl IXamlRenderingBackgroundTaskOverrides_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IXamlRenderingBackgroundTaskOverrides_Impl,
        const OFFSET: isize,
    >() -> IXamlRenderingBackgroundTaskOverrides_Vtbl {
        unsafe extern "system" fn OnRun<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IXamlRenderingBackgroundTaskOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            taskinstance: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnRun(::windows_core::from_raw_borrowed(&taskinstance)).into()
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<
                Identity,
                IXamlRenderingBackgroundTaskOverrides,
                OFFSET,
            >(),
            OnRun: OnRun::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IXamlRenderingBackgroundTaskOverrides as ::windows_core::ComInterface>::IID
    }
}
