#[doc = "Required features: `\"Windows_Foundation_Collections\"`"]
#[cfg(feature = "Windows_Foundation_Collections")]
pub trait IAutomationPeerOverrides_Impl: Sized {
    fn GetPatternCore(
        &self,
        patterninterface: PatternInterface,
    ) -> ::windows_core::Result<::windows_core::IInspectable>;
    fn GetAcceleratorKeyCore(&self) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn GetAccessKeyCore(&self) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn GetAutomationControlTypeCore(&self) -> ::windows_core::Result<AutomationControlType>;
    fn GetAutomationIdCore(&self) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn GetBoundingRectangleCore(&self) -> ::windows_core::Result<::windows::Foundation::Rect>;
    fn GetChildrenCore(
        &self,
    ) -> ::windows_core::Result<::windows::Foundation::Collections::IVector<AutomationPeer>>;
    fn NavigateCore(
        &self,
        direction: AutomationNavigationDirection,
    ) -> ::windows_core::Result<::windows_core::IInspectable>;
    fn GetClassNameCore(&self) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn GetClickablePointCore(&self) -> ::windows_core::Result<::windows::Foundation::Point>;
    fn GetHelpTextCore(&self) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn GetItemStatusCore(&self) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn GetItemTypeCore(&self) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn GetLabeledByCore(&self) -> ::windows_core::Result<AutomationPeer>;
    fn GetLocalizedControlTypeCore(&self) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn GetNameCore(&self) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn GetOrientationCore(&self) -> ::windows_core::Result<AutomationOrientation>;
    fn HasKeyboardFocusCore(&self) -> ::windows_core::Result<bool>;
    fn IsContentElementCore(&self) -> ::windows_core::Result<bool>;
    fn IsControlElementCore(&self) -> ::windows_core::Result<bool>;
    fn IsEnabledCore(&self) -> ::windows_core::Result<bool>;
    fn IsKeyboardFocusableCore(&self) -> ::windows_core::Result<bool>;
    fn IsOffscreenCore(&self) -> ::windows_core::Result<bool>;
    fn IsPasswordCore(&self) -> ::windows_core::Result<bool>;
    fn IsRequiredForFormCore(&self) -> ::windows_core::Result<bool>;
    fn SetFocusCore(&self) -> ::windows_core::Result<()>;
    fn GetPeerFromPointCore(
        &self,
        point: &::windows::Foundation::Point,
    ) -> ::windows_core::Result<AutomationPeer>;
    fn GetElementFromPointCore(
        &self,
        pointinwindowcoordinates: &::windows::Foundation::Point,
    ) -> ::windows_core::Result<::windows_core::IInspectable>;
    fn GetFocusedElementCore(&self) -> ::windows_core::Result<::windows_core::IInspectable>;
    fn GetLiveSettingCore(&self) -> ::windows_core::Result<AutomationLiveSetting>;
    fn ShowContextMenuCore(&self) -> ::windows_core::Result<()>;
    fn GetControlledPeersCore(
        &self,
    ) -> ::windows_core::Result<::windows::Foundation::Collections::IVectorView<AutomationPeer>>;
    fn GetAnnotationsCore(
        &self,
    ) -> ::windows_core::Result<::windows::Foundation::Collections::IVector<AutomationPeerAnnotation>>;
    fn GetPositionInSetCore(&self) -> ::windows_core::Result<i32>;
    fn GetSizeOfSetCore(&self) -> ::windows_core::Result<i32>;
    fn GetLevelCore(&self) -> ::windows_core::Result<i32>;
    fn GetLandmarkTypeCore(&self) -> ::windows_core::Result<AutomationLandmarkType>;
    fn GetLocalizedLandmarkTypeCore(&self) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn IsPeripheralCore(&self) -> ::windows_core::Result<bool>;
    fn IsDataValidForFormCore(&self) -> ::windows_core::Result<bool>;
    fn GetFullDescriptionCore(&self) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn GetDescribedByCore(
        &self,
    ) -> ::windows_core::Result<::windows::Foundation::Collections::IIterable<AutomationPeer>>;
    fn GetFlowsToCore(
        &self,
    ) -> ::windows_core::Result<::windows::Foundation::Collections::IIterable<AutomationPeer>>;
    fn GetFlowsFromCore(
        &self,
    ) -> ::windows_core::Result<::windows::Foundation::Collections::IIterable<AutomationPeer>>;
    fn GetCultureCore(&self) -> ::windows_core::Result<i32>;
    fn GetHeadingLevelCore(&self) -> ::windows_core::Result<AutomationHeadingLevel>;
    fn IsDialogCore(&self) -> ::windows_core::Result<bool>;
}
#[cfg(feature = "Windows_Foundation_Collections")]
impl ::windows_core::RuntimeName for IAutomationPeerOverrides {
    const NAME: &'static str = "Microsoft.UI.Xaml.Automation.Peers.IAutomationPeerOverrides";
}
#[cfg(feature = "Windows_Foundation_Collections")]
impl IAutomationPeerOverrides_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IAutomationPeerOverrides_Impl,
        const OFFSET: isize,
    >() -> IAutomationPeerOverrides_Vtbl {
        unsafe extern "system" fn GetPatternCore<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IAutomationPeerOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            patterninterface: PatternInterface,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPatternCore(patterninterface) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAcceleratorKeyCore<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IAutomationPeerOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetAcceleratorKeyCore() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAccessKeyCore<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IAutomationPeerOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetAccessKeyCore() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAutomationControlTypeCore<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IAutomationPeerOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut AutomationControlType,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetAutomationControlTypeCore() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAutomationIdCore<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IAutomationPeerOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetAutomationIdCore() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBoundingRectangleCore<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IAutomationPeerOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::Foundation::Rect,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetBoundingRectangleCore() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetChildrenCore<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IAutomationPeerOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetChildrenCore() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NavigateCore<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IAutomationPeerOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            direction: AutomationNavigationDirection,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.NavigateCore(direction) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetClassNameCore<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IAutomationPeerOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetClassNameCore() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetClickablePointCore<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IAutomationPeerOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::Foundation::Point,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetClickablePointCore() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHelpTextCore<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IAutomationPeerOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetHelpTextCore() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetItemStatusCore<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IAutomationPeerOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetItemStatusCore() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetItemTypeCore<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IAutomationPeerOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetItemTypeCore() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLabeledByCore<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IAutomationPeerOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetLabeledByCore() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLocalizedControlTypeCore<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IAutomationPeerOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetLocalizedControlTypeCore() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNameCore<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IAutomationPeerOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetNameCore() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOrientationCore<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IAutomationPeerOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut AutomationOrientation,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetOrientationCore() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HasKeyboardFocusCore<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IAutomationPeerOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut bool,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.HasKeyboardFocusCore() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsContentElementCore<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IAutomationPeerOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut bool,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsContentElementCore() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsControlElementCore<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IAutomationPeerOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut bool,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsControlElementCore() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsEnabledCore<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IAutomationPeerOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut bool,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsEnabledCore() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsKeyboardFocusableCore<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IAutomationPeerOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut bool,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsKeyboardFocusableCore() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsOffscreenCore<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IAutomationPeerOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut bool,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsOffscreenCore() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsPasswordCore<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IAutomationPeerOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut bool,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsPasswordCore() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsRequiredForFormCore<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IAutomationPeerOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut bool,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsRequiredForFormCore() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFocusCore<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IAutomationPeerOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetFocusCore().into()
        }
        unsafe extern "system" fn GetPeerFromPointCore<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IAutomationPeerOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            point: ::windows::Foundation::Point,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPeerFromPointCore(::core::mem::transmute(&point)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetElementFromPointCore<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IAutomationPeerOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            pointinwindowcoordinates: ::windows::Foundation::Point,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetElementFromPointCore(::core::mem::transmute(&pointinwindowcoordinates)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFocusedElementCore<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IAutomationPeerOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFocusedElementCore() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLiveSettingCore<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IAutomationPeerOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut AutomationLiveSetting,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetLiveSettingCore() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShowContextMenuCore<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IAutomationPeerOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ShowContextMenuCore().into()
        }
        unsafe extern "system" fn GetControlledPeersCore<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IAutomationPeerOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetControlledPeersCore() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAnnotationsCore<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IAutomationPeerOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetAnnotationsCore() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPositionInSetCore<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IAutomationPeerOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut i32,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPositionInSetCore() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSizeOfSetCore<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IAutomationPeerOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut i32,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSizeOfSetCore() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLevelCore<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IAutomationPeerOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut i32,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetLevelCore() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLandmarkTypeCore<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IAutomationPeerOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut AutomationLandmarkType,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetLandmarkTypeCore() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLocalizedLandmarkTypeCore<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IAutomationPeerOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetLocalizedLandmarkTypeCore() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsPeripheralCore<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IAutomationPeerOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut bool,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsPeripheralCore() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsDataValidForFormCore<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IAutomationPeerOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut bool,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsDataValidForFormCore() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFullDescriptionCore<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IAutomationPeerOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFullDescriptionCore() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDescribedByCore<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IAutomationPeerOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDescribedByCore() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFlowsToCore<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IAutomationPeerOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFlowsToCore() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFlowsFromCore<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IAutomationPeerOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFlowsFromCore() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCultureCore<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IAutomationPeerOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut i32,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCultureCore() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHeadingLevelCore<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IAutomationPeerOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut AutomationHeadingLevel,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetHeadingLevelCore() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsDialogCore<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IAutomationPeerOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut bool,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsDialogCore() {
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
                IAutomationPeerOverrides,
                OFFSET,
            >(),
            GetPatternCore: GetPatternCore::<Identity, Impl, OFFSET>,
            GetAcceleratorKeyCore: GetAcceleratorKeyCore::<Identity, Impl, OFFSET>,
            GetAccessKeyCore: GetAccessKeyCore::<Identity, Impl, OFFSET>,
            GetAutomationControlTypeCore: GetAutomationControlTypeCore::<Identity, Impl, OFFSET>,
            GetAutomationIdCore: GetAutomationIdCore::<Identity, Impl, OFFSET>,
            GetBoundingRectangleCore: GetBoundingRectangleCore::<Identity, Impl, OFFSET>,
            GetChildrenCore: GetChildrenCore::<Identity, Impl, OFFSET>,
            NavigateCore: NavigateCore::<Identity, Impl, OFFSET>,
            GetClassNameCore: GetClassNameCore::<Identity, Impl, OFFSET>,
            GetClickablePointCore: GetClickablePointCore::<Identity, Impl, OFFSET>,
            GetHelpTextCore: GetHelpTextCore::<Identity, Impl, OFFSET>,
            GetItemStatusCore: GetItemStatusCore::<Identity, Impl, OFFSET>,
            GetItemTypeCore: GetItemTypeCore::<Identity, Impl, OFFSET>,
            GetLabeledByCore: GetLabeledByCore::<Identity, Impl, OFFSET>,
            GetLocalizedControlTypeCore: GetLocalizedControlTypeCore::<Identity, Impl, OFFSET>,
            GetNameCore: GetNameCore::<Identity, Impl, OFFSET>,
            GetOrientationCore: GetOrientationCore::<Identity, Impl, OFFSET>,
            HasKeyboardFocusCore: HasKeyboardFocusCore::<Identity, Impl, OFFSET>,
            IsContentElementCore: IsContentElementCore::<Identity, Impl, OFFSET>,
            IsControlElementCore: IsControlElementCore::<Identity, Impl, OFFSET>,
            IsEnabledCore: IsEnabledCore::<Identity, Impl, OFFSET>,
            IsKeyboardFocusableCore: IsKeyboardFocusableCore::<Identity, Impl, OFFSET>,
            IsOffscreenCore: IsOffscreenCore::<Identity, Impl, OFFSET>,
            IsPasswordCore: IsPasswordCore::<Identity, Impl, OFFSET>,
            IsRequiredForFormCore: IsRequiredForFormCore::<Identity, Impl, OFFSET>,
            SetFocusCore: SetFocusCore::<Identity, Impl, OFFSET>,
            GetPeerFromPointCore: GetPeerFromPointCore::<Identity, Impl, OFFSET>,
            GetElementFromPointCore: GetElementFromPointCore::<Identity, Impl, OFFSET>,
            GetFocusedElementCore: GetFocusedElementCore::<Identity, Impl, OFFSET>,
            GetLiveSettingCore: GetLiveSettingCore::<Identity, Impl, OFFSET>,
            ShowContextMenuCore: ShowContextMenuCore::<Identity, Impl, OFFSET>,
            GetControlledPeersCore: GetControlledPeersCore::<Identity, Impl, OFFSET>,
            GetAnnotationsCore: GetAnnotationsCore::<Identity, Impl, OFFSET>,
            GetPositionInSetCore: GetPositionInSetCore::<Identity, Impl, OFFSET>,
            GetSizeOfSetCore: GetSizeOfSetCore::<Identity, Impl, OFFSET>,
            GetLevelCore: GetLevelCore::<Identity, Impl, OFFSET>,
            GetLandmarkTypeCore: GetLandmarkTypeCore::<Identity, Impl, OFFSET>,
            GetLocalizedLandmarkTypeCore: GetLocalizedLandmarkTypeCore::<Identity, Impl, OFFSET>,
            IsPeripheralCore: IsPeripheralCore::<Identity, Impl, OFFSET>,
            IsDataValidForFormCore: IsDataValidForFormCore::<Identity, Impl, OFFSET>,
            GetFullDescriptionCore: GetFullDescriptionCore::<Identity, Impl, OFFSET>,
            GetDescribedByCore: GetDescribedByCore::<Identity, Impl, OFFSET>,
            GetFlowsToCore: GetFlowsToCore::<Identity, Impl, OFFSET>,
            GetFlowsFromCore: GetFlowsFromCore::<Identity, Impl, OFFSET>,
            GetCultureCore: GetCultureCore::<Identity, Impl, OFFSET>,
            GetHeadingLevelCore: GetHeadingLevelCore::<Identity, Impl, OFFSET>,
            IsDialogCore: IsDialogCore::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IAutomationPeerOverrides as ::windows_core::ComInterface>::IID
    }
}
pub trait IItemsControlAutomationPeerOverrides_Impl: Sized {
    fn OnCreateItemAutomationPeer(
        &self,
        item: ::core::option::Option<&::windows_core::IInspectable>,
    ) -> ::windows_core::Result<ItemAutomationPeer>;
}
impl ::windows_core::RuntimeName for IItemsControlAutomationPeerOverrides {
    const NAME: &'static str =
        "Microsoft.UI.Xaml.Automation.Peers.IItemsControlAutomationPeerOverrides";
}
impl IItemsControlAutomationPeerOverrides_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IItemsControlAutomationPeerOverrides_Impl,
        const OFFSET: isize,
    >() -> IItemsControlAutomationPeerOverrides_Vtbl {
        unsafe extern "system" fn OnCreateItemAutomationPeer<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IItemsControlAutomationPeerOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            item: *mut ::core::ffi::c_void,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.OnCreateItemAutomationPeer(::windows_core::from_raw_borrowed(&item)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<
                Identity,
                IItemsControlAutomationPeerOverrides,
                OFFSET,
            >(),
            OnCreateItemAutomationPeer: OnCreateItemAutomationPeer::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IItemsControlAutomationPeerOverrides as ::windows_core::ComInterface>::IID
    }
}
