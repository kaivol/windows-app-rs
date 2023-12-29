#[doc = "Required features: `\"Microsoft_UI_Composition\"`"]
#[cfg(feature = "Microsoft_UI_Composition")]
pub trait IBrushOverrides_Impl: Sized {
    fn PopulatePropertyInfoOverride(
        &self,
        propertyname: &::windows_core::HSTRING,
        animationpropertyinfo: ::core::option::Option<
            &super::super::Composition::AnimationPropertyInfo,
        >,
    ) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Microsoft_UI_Composition")]
impl ::windows_core::RuntimeName for IBrushOverrides {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.IBrushOverrides";
}
#[cfg(feature = "Microsoft_UI_Composition")]
impl IBrushOverrides_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IBrushOverrides_Impl,
        const OFFSET: isize,
    >() -> IBrushOverrides_Vtbl {
        unsafe extern "system" fn PopulatePropertyInfoOverride<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IBrushOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            propertyname: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
            animationpropertyinfo: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PopulatePropertyInfoOverride(
                ::core::mem::transmute(&propertyname),
                ::windows_core::from_raw_borrowed(&animationpropertyinfo),
            )
            .into()
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<Identity, IBrushOverrides, OFFSET>(),
            PopulatePropertyInfoOverride: PopulatePropertyInfoOverride::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IBrushOverrides as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Windows_Foundation\"`"]
#[cfg(feature = "Windows_Foundation")]
pub trait IGeneralTransformOverrides_Impl: Sized {
    fn InverseCore(&self) -> ::windows_core::Result<GeneralTransform>;
    fn TryTransformCore(
        &self,
        inpoint: &::windows::Foundation::Point,
        outpoint: &mut ::windows::Foundation::Point,
    ) -> ::windows_core::Result<bool>;
    fn TransformBoundsCore(
        &self,
        rect: &::windows::Foundation::Rect,
    ) -> ::windows_core::Result<::windows::Foundation::Rect>;
}
#[cfg(feature = "Windows_Foundation")]
impl ::windows_core::RuntimeName for IGeneralTransformOverrides {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.IGeneralTransformOverrides";
}
#[cfg(feature = "Windows_Foundation")]
impl IGeneralTransformOverrides_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IGeneralTransformOverrides_Impl,
        const OFFSET: isize,
    >() -> IGeneralTransformOverrides_Vtbl {
        unsafe extern "system" fn InverseCore<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IGeneralTransformOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.InverseCore() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryTransformCore<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IGeneralTransformOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            inpoint: ::windows::Foundation::Point,
            outpoint: *mut ::windows::Foundation::Point,
            result__: *mut bool,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.TryTransformCore(
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
        unsafe extern "system" fn TransformBoundsCore<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IGeneralTransformOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            rect: ::windows::Foundation::Rect,
            result__: *mut ::windows::Foundation::Rect,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.TransformBoundsCore(::core::mem::transmute(&rect)) {
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
                IGeneralTransformOverrides,
                OFFSET,
            >(),
            InverseCore: InverseCore::<Identity, Impl, OFFSET>,
            TryTransformCore: TryTransformCore::<Identity, Impl, OFFSET>,
            TransformBoundsCore: TransformBoundsCore::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IGeneralTransformOverrides as ::windows_core::ComInterface>::IID
    }
}
pub trait ISurfaceImageSourceManagerNative_Impl: Sized {
    fn FlushAllSurfacesWithDevice(
        &self,
        device: ::core::option::Option<&::windows_core::IUnknown>,
    ) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for ISurfaceImageSourceManagerNative {}
impl ISurfaceImageSourceManagerNative_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: ISurfaceImageSourceManagerNative_Impl,
        const OFFSET: isize,
    >() -> ISurfaceImageSourceManagerNative_Vtbl {
        unsafe extern "system" fn FlushAllSurfacesWithDevice<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: ISurfaceImageSourceManagerNative_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            device: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.FlushAllSurfacesWithDevice(::windows_core::from_raw_borrowed(&device))
                .into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            FlushAllSurfacesWithDevice: FlushAllSurfacesWithDevice::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <ISurfaceImageSourceManagerNative as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Windows_Win32_Foundation\"`, `\"Windows_Win32_Graphics_Dxgi\"`"]
#[cfg(all(
    feature = "Windows_Win32_Foundation",
    feature = "Windows_Win32_Graphics_Dxgi"
))]
pub trait ISurfaceImageSourceNative_Impl: Sized {
    fn SetDevice(
        &self,
        device: ::core::option::Option<&::windows::Win32::Graphics::Dxgi::IDXGIDevice>,
    ) -> ::windows_core::Result<()>;
    fn BeginDraw(
        &self,
        updaterect: &::windows::Win32::Foundation::RECT,
        surface: *mut ::core::option::Option<::windows::Win32::Graphics::Dxgi::IDXGISurface>,
        offset: *mut ::windows::Win32::Foundation::POINT,
    ) -> ::windows_core::Result<()>;
    fn EndDraw(&self) -> ::windows_core::Result<()>;
}
#[cfg(all(
    feature = "Windows_Win32_Foundation",
    feature = "Windows_Win32_Graphics_Dxgi"
))]
impl ::windows_core::RuntimeName for ISurfaceImageSourceNative {}
#[cfg(all(
    feature = "Windows_Win32_Foundation",
    feature = "Windows_Win32_Graphics_Dxgi"
))]
impl ISurfaceImageSourceNative_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: ISurfaceImageSourceNative_Impl,
        const OFFSET: isize,
    >() -> ISurfaceImageSourceNative_Vtbl {
        unsafe extern "system" fn SetDevice<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: ISurfaceImageSourceNative_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            device: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDevice(::windows_core::from_raw_borrowed(&device)).into()
        }
        unsafe extern "system" fn BeginDraw<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: ISurfaceImageSourceNative_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            updaterect: ::windows::Win32::Foundation::RECT,
            surface: *mut *mut ::core::ffi::c_void,
            offset: *mut ::windows::Win32::Foundation::POINT,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.BeginDraw(
                ::core::mem::transmute(&updaterect),
                ::core::mem::transmute_copy(&surface),
                ::core::mem::transmute_copy(&offset),
            )
            .into()
        }
        unsafe extern "system" fn EndDraw<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: ISurfaceImageSourceNative_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndDraw().into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetDevice: SetDevice::<Identity, Impl, OFFSET>,
            BeginDraw: BeginDraw::<Identity, Impl, OFFSET>,
            EndDraw: EndDraw::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <ISurfaceImageSourceNative as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Windows_Win32_Foundation\"`"]
#[cfg(feature = "Windows_Win32_Foundation")]
pub trait ISurfaceImageSourceNativeWithD2D_Impl: Sized {
    fn SetDevice(
        &self,
        device: ::core::option::Option<&::windows_core::IUnknown>,
    ) -> ::windows_core::Result<()>;
    fn BeginDraw(
        &self,
        updaterect: *const ::windows::Win32::Foundation::RECT,
        iid: *const ::windows_core::GUID,
        updateobject: *mut *mut ::core::ffi::c_void,
        offset: *mut ::windows::Win32::Foundation::POINT,
    ) -> ::windows_core::Result<()>;
    fn EndDraw(&self) -> ::windows_core::Result<()>;
    fn SuspendDraw(&self) -> ::windows_core::Result<()>;
    fn ResumeDraw(&self) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Windows_Win32_Foundation")]
impl ::windows_core::RuntimeName for ISurfaceImageSourceNativeWithD2D {}
#[cfg(feature = "Windows_Win32_Foundation")]
impl ISurfaceImageSourceNativeWithD2D_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: ISurfaceImageSourceNativeWithD2D_Impl,
        const OFFSET: isize,
    >() -> ISurfaceImageSourceNativeWithD2D_Vtbl {
        unsafe extern "system" fn SetDevice<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: ISurfaceImageSourceNativeWithD2D_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            device: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDevice(::windows_core::from_raw_borrowed(&device)).into()
        }
        unsafe extern "system" fn BeginDraw<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: ISurfaceImageSourceNativeWithD2D_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            updaterect: *const ::windows::Win32::Foundation::RECT,
            iid: *const ::windows_core::GUID,
            updateobject: *mut *mut ::core::ffi::c_void,
            offset: *mut ::windows::Win32::Foundation::POINT,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.BeginDraw(
                ::core::mem::transmute_copy(&updaterect),
                ::core::mem::transmute_copy(&iid),
                ::core::mem::transmute_copy(&updateobject),
                ::core::mem::transmute_copy(&offset),
            )
            .into()
        }
        unsafe extern "system" fn EndDraw<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: ISurfaceImageSourceNativeWithD2D_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndDraw().into()
        }
        unsafe extern "system" fn SuspendDraw<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: ISurfaceImageSourceNativeWithD2D_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SuspendDraw().into()
        }
        unsafe extern "system" fn ResumeDraw<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: ISurfaceImageSourceNativeWithD2D_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ResumeDraw().into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetDevice: SetDevice::<Identity, Impl, OFFSET>,
            BeginDraw: BeginDraw::<Identity, Impl, OFFSET>,
            EndDraw: EndDraw::<Identity, Impl, OFFSET>,
            SuspendDraw: SuspendDraw::<Identity, Impl, OFFSET>,
            ResumeDraw: ResumeDraw::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <ISurfaceImageSourceNativeWithD2D as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Windows_Win32_Graphics_Dxgi\"`"]
#[cfg(feature = "Windows_Win32_Graphics_Dxgi")]
pub trait ISwapChainBackgroundPanelNative_Impl: Sized {
    fn SetSwapChain(
        &self,
        swapchain: ::core::option::Option<&::windows::Win32::Graphics::Dxgi::IDXGISwapChain>,
    ) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Windows_Win32_Graphics_Dxgi")]
impl ::windows_core::RuntimeName for ISwapChainBackgroundPanelNative {}
#[cfg(feature = "Windows_Win32_Graphics_Dxgi")]
impl ISwapChainBackgroundPanelNative_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: ISwapChainBackgroundPanelNative_Impl,
        const OFFSET: isize,
    >() -> ISwapChainBackgroundPanelNative_Vtbl {
        unsafe extern "system" fn SetSwapChain<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: ISwapChainBackgroundPanelNative_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            swapchain: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSwapChain(::windows_core::from_raw_borrowed(&swapchain)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetSwapChain: SetSwapChain::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <ISwapChainBackgroundPanelNative as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Windows_Win32_Graphics_Dxgi\"`"]
#[cfg(feature = "Windows_Win32_Graphics_Dxgi")]
pub trait ISwapChainPanelNative_Impl: Sized {
    fn SetSwapChain(
        &self,
        swapchain: ::core::option::Option<&::windows::Win32::Graphics::Dxgi::IDXGISwapChain>,
    ) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Windows_Win32_Graphics_Dxgi")]
impl ::windows_core::RuntimeName for ISwapChainPanelNative {}
#[cfg(feature = "Windows_Win32_Graphics_Dxgi")]
impl ISwapChainPanelNative_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: ISwapChainPanelNative_Impl,
        const OFFSET: isize,
    >() -> ISwapChainPanelNative_Vtbl {
        unsafe extern "system" fn SetSwapChain<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: ISwapChainPanelNative_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            swapchain: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSwapChain(::windows_core::from_raw_borrowed(&swapchain)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetSwapChain: SetSwapChain::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <ISwapChainPanelNative as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Windows_Win32_Foundation\"`, `\"Windows_Win32_Graphics_Dxgi\"`"]
#[cfg(all(
    feature = "Windows_Win32_Foundation",
    feature = "Windows_Win32_Graphics_Dxgi"
))]
pub trait ISwapChainPanelNative2_Impl: Sized + ISwapChainPanelNative_Impl {
    fn SetSwapChainHandle(
        &self,
        swapchainhandle: ::windows::Win32::Foundation::HANDLE,
    ) -> ::windows_core::Result<()>;
}
#[cfg(all(
    feature = "Windows_Win32_Foundation",
    feature = "Windows_Win32_Graphics_Dxgi"
))]
impl ::windows_core::RuntimeName for ISwapChainPanelNative2 {}
#[cfg(all(
    feature = "Windows_Win32_Foundation",
    feature = "Windows_Win32_Graphics_Dxgi"
))]
impl ISwapChainPanelNative2_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: ISwapChainPanelNative2_Impl,
        const OFFSET: isize,
    >() -> ISwapChainPanelNative2_Vtbl {
        unsafe extern "system" fn SetSwapChainHandle<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: ISwapChainPanelNative2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            swapchainhandle: ::windows::Win32::Foundation::HANDLE,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSwapChainHandle(::core::mem::transmute_copy(&swapchainhandle)).into()
        }
        Self {
            base__: ISwapChainPanelNative_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetSwapChainHandle: SetSwapChainHandle::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <ISwapChainPanelNative2 as ::windows_core::ComInterface>::IID
            || *iid == <ISwapChainPanelNative as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Microsoft_UI_Composition\"`"]
#[cfg(feature = "Microsoft_UI_Composition")]
pub trait ISystemBackdropOverrides_Impl: Sized {
    fn OnTargetConnected(
        &self,
        connectedtarget: ::core::option::Option<
            &super::super::Composition::ICompositionSupportsSystemBackdrop,
        >,
        xamlroot: ::core::option::Option<&super::XamlRoot>,
    ) -> ::windows_core::Result<()>;
    fn OnTargetDisconnected(
        &self,
        disconnectedtarget: ::core::option::Option<
            &super::super::Composition::ICompositionSupportsSystemBackdrop,
        >,
    ) -> ::windows_core::Result<()>;
    fn OnDefaultSystemBackdropConfigurationChanged(
        &self,
        target: ::core::option::Option<
            &super::super::Composition::ICompositionSupportsSystemBackdrop,
        >,
        xamlroot: ::core::option::Option<&super::XamlRoot>,
    ) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Microsoft_UI_Composition")]
impl ::windows_core::RuntimeName for ISystemBackdropOverrides {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.ISystemBackdropOverrides";
}
#[cfg(feature = "Microsoft_UI_Composition")]
impl ISystemBackdropOverrides_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: ISystemBackdropOverrides_Impl,
        const OFFSET: isize,
    >() -> ISystemBackdropOverrides_Vtbl {
        unsafe extern "system" fn OnTargetConnected<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: ISystemBackdropOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            connectedtarget: *mut ::core::ffi::c_void,
            xamlroot: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnTargetConnected(
                ::windows_core::from_raw_borrowed(&connectedtarget),
                ::windows_core::from_raw_borrowed(&xamlroot),
            )
            .into()
        }
        unsafe extern "system" fn OnTargetDisconnected<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: ISystemBackdropOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            disconnectedtarget: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnTargetDisconnected(::windows_core::from_raw_borrowed(&disconnectedtarget))
                .into()
        }
        unsafe extern "system" fn OnDefaultSystemBackdropConfigurationChanged<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: ISystemBackdropOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            target: *mut ::core::ffi::c_void,
            xamlroot: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnDefaultSystemBackdropConfigurationChanged(
                ::windows_core::from_raw_borrowed(&target),
                ::windows_core::from_raw_borrowed(&xamlroot),
            )
            .into()
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<
                Identity,
                ISystemBackdropOverrides,
                OFFSET,
            >(),
            OnTargetConnected: OnTargetConnected::<Identity, Impl, OFFSET>,
            OnTargetDisconnected: OnTargetDisconnected::<Identity, Impl, OFFSET>,
            OnDefaultSystemBackdropConfigurationChanged:
                OnDefaultSystemBackdropConfigurationChanged::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <ISystemBackdropOverrides as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Windows_Win32_Foundation\"`, `\"Windows_Win32_Graphics_Dxgi\"`"]
#[cfg(all(
    feature = "Windows_Win32_Foundation",
    feature = "Windows_Win32_Graphics_Dxgi"
))]
pub trait IVirtualSurfaceImageSourceNative_Impl: Sized + ISurfaceImageSourceNative_Impl {
    fn Invalidate(
        &self,
        updaterect: &::windows::Win32::Foundation::RECT,
    ) -> ::windows_core::Result<()>;
    fn GetUpdateRectCount(&self) -> ::windows_core::Result<u32>;
    fn GetUpdateRects(
        &self,
        updates: *mut ::windows::Win32::Foundation::RECT,
        count: u32,
    ) -> ::windows_core::Result<()>;
    fn GetVisibleBounds(&self) -> ::windows_core::Result<::windows::Win32::Foundation::RECT>;
    fn RegisterForUpdatesNeeded(
        &self,
        callback: ::core::option::Option<&IVirtualSurfaceUpdatesCallbackNative>,
    ) -> ::windows_core::Result<()>;
    fn Resize(&self, newwidth: i32, newheight: i32) -> ::windows_core::Result<()>;
}
#[cfg(all(
    feature = "Windows_Win32_Foundation",
    feature = "Windows_Win32_Graphics_Dxgi"
))]
impl ::windows_core::RuntimeName for IVirtualSurfaceImageSourceNative {}
#[cfg(all(
    feature = "Windows_Win32_Foundation",
    feature = "Windows_Win32_Graphics_Dxgi"
))]
impl IVirtualSurfaceImageSourceNative_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IVirtualSurfaceImageSourceNative_Impl,
        const OFFSET: isize,
    >() -> IVirtualSurfaceImageSourceNative_Vtbl {
        unsafe extern "system" fn Invalidate<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IVirtualSurfaceImageSourceNative_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            updaterect: ::windows::Win32::Foundation::RECT,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Invalidate(::core::mem::transmute(&updaterect)).into()
        }
        unsafe extern "system" fn GetUpdateRectCount<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IVirtualSurfaceImageSourceNative_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            count: *mut u32,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetUpdateRectCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(count, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUpdateRects<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IVirtualSurfaceImageSourceNative_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            updates: *mut ::windows::Win32::Foundation::RECT,
            count: u32,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetUpdateRects(
                ::core::mem::transmute_copy(&updates),
                ::core::mem::transmute_copy(&count),
            )
            .into()
        }
        unsafe extern "system" fn GetVisibleBounds<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IVirtualSurfaceImageSourceNative_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            bounds: *mut ::windows::Win32::Foundation::RECT,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetVisibleBounds() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(bounds, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterForUpdatesNeeded<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IVirtualSurfaceImageSourceNative_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            callback: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RegisterForUpdatesNeeded(::windows_core::from_raw_borrowed(&callback))
                .into()
        }
        unsafe extern "system" fn Resize<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IVirtualSurfaceImageSourceNative_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            newwidth: i32,
            newheight: i32,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Resize(
                ::core::mem::transmute_copy(&newwidth),
                ::core::mem::transmute_copy(&newheight),
            )
            .into()
        }
        Self {
            base__: ISurfaceImageSourceNative_Vtbl::new::<Identity, Impl, OFFSET>(),
            Invalidate: Invalidate::<Identity, Impl, OFFSET>,
            GetUpdateRectCount: GetUpdateRectCount::<Identity, Impl, OFFSET>,
            GetUpdateRects: GetUpdateRects::<Identity, Impl, OFFSET>,
            GetVisibleBounds: GetVisibleBounds::<Identity, Impl, OFFSET>,
            RegisterForUpdatesNeeded: RegisterForUpdatesNeeded::<Identity, Impl, OFFSET>,
            Resize: Resize::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IVirtualSurfaceImageSourceNative as ::windows_core::ComInterface>::IID
            || *iid == <ISurfaceImageSourceNative as ::windows_core::ComInterface>::IID
    }
}
pub trait IVirtualSurfaceUpdatesCallbackNative_Impl: Sized {
    fn UpdatesNeeded(&self) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IVirtualSurfaceUpdatesCallbackNative {}
impl IVirtualSurfaceUpdatesCallbackNative_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IVirtualSurfaceUpdatesCallbackNative_Impl,
        const OFFSET: isize,
    >() -> IVirtualSurfaceUpdatesCallbackNative_Vtbl {
        unsafe extern "system" fn UpdatesNeeded<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IVirtualSurfaceUpdatesCallbackNative_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UpdatesNeeded().into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            UpdatesNeeded: UpdatesNeeded::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IVirtualSurfaceUpdatesCallbackNative as ::windows_core::ComInterface>::IID
    }
}
pub trait IXamlCompositionBrushBaseOverrides_Impl: Sized {
    fn OnConnected(&self) -> ::windows_core::Result<()>;
    fn OnDisconnected(&self) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IXamlCompositionBrushBaseOverrides {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.IXamlCompositionBrushBaseOverrides";
}
impl IXamlCompositionBrushBaseOverrides_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IXamlCompositionBrushBaseOverrides_Impl,
        const OFFSET: isize,
    >() -> IXamlCompositionBrushBaseOverrides_Vtbl {
        unsafe extern "system" fn OnConnected<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IXamlCompositionBrushBaseOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnConnected().into()
        }
        unsafe extern "system" fn OnDisconnected<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IXamlCompositionBrushBaseOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnDisconnected().into()
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<
                Identity,
                IXamlCompositionBrushBaseOverrides,
                OFFSET,
            >(),
            OnConnected: OnConnected::<Identity, Impl, OFFSET>,
            OnDisconnected: OnDisconnected::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IXamlCompositionBrushBaseOverrides as ::windows_core::ComInterface>::IID
    }
}
pub trait IXamlLightOverrides_Impl: Sized {
    fn GetId(&self) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn OnConnected(
        &self,
        newelement: ::core::option::Option<&super::UIElement>,
    ) -> ::windows_core::Result<()>;
    fn OnDisconnected(
        &self,
        oldelement: ::core::option::Option<&super::UIElement>,
    ) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IXamlLightOverrides {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.IXamlLightOverrides";
}
impl IXamlLightOverrides_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IXamlLightOverrides_Impl,
        const OFFSET: isize,
    >() -> IXamlLightOverrides_Vtbl {
        unsafe extern "system" fn GetId<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IXamlLightOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnConnected<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IXamlLightOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            newelement: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnConnected(::windows_core::from_raw_borrowed(&newelement)).into()
        }
        unsafe extern "system" fn OnDisconnected<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IXamlLightOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            oldelement: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnDisconnected(::windows_core::from_raw_borrowed(&oldelement)).into()
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<Identity, IXamlLightOverrides, OFFSET>(
            ),
            GetId: GetId::<Identity, Impl, OFFSET>,
            OnConnected: OnConnected::<Identity, Impl, OFFSET>,
            OnDisconnected: OnDisconnected::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IXamlLightOverrides as ::windows_core::ComInterface>::IID
    }
}
