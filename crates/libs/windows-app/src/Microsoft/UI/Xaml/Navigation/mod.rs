#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IFrameNavigationOptions(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFrameNavigationOptions {
    type Vtable = IFrameNavigationOptions_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IFrameNavigationOptions {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x390de593_14cf_5312_af99_6cd8d59ec5d5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFrameNavigationOptions_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub IsNavigationStackEnabled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetIsNavigationStackEnabled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Microsoft_UI_Xaml_Media_Animation")]
    pub TransitionInfoOverride: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Microsoft_UI_Xaml_Media_Animation"))]
    TransitionInfoOverride: usize,
    #[cfg(feature = "Microsoft_UI_Xaml_Media_Animation")]
    pub SetTransitionInfoOverride: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Microsoft_UI_Xaml_Media_Animation"))]
    SetTransitionInfoOverride: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IFrameNavigationOptionsFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFrameNavigationOptionsFactory {
    type Vtable = IFrameNavigationOptionsFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IFrameNavigationOptionsFactory {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xddf3f748_7127_5cee_9f79_ac281a234632);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFrameNavigationOptionsFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        baseinterface: *mut ::core::ffi::c_void,
        innerinterface: *mut *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct INavigatingCancelEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for INavigatingCancelEventArgs {
    type Vtable = INavigatingCancelEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for INavigatingCancelEventArgs {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x172fde12_e06f_5df6_930e_5facf7b3fbe7);
}
#[repr(C)]
#[doc(hidden)]
pub struct INavigatingCancelEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Cancel: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetCancel: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
    pub NavigationMode: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut NavigationMode,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_UI_Xaml_Interop")]
    pub SourcePageType: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<
            super::super::super::super::Windows::UI::Xaml::Interop::TypeName,
        >,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI_Xaml_Interop"))]
    SourcePageType: usize,
    pub Parameter: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Microsoft_UI_Xaml_Media_Animation")]
    pub NavigationTransitionInfo: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Microsoft_UI_Xaml_Media_Animation"))]
    NavigationTransitionInfo: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct INavigationEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for INavigationEventArgs {
    type Vtable = INavigationEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for INavigationEventArgs {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x876b70b4_2923_5785_9cea_2e44aa0761bd);
}
#[repr(C)]
#[doc(hidden)]
pub struct INavigationEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Content: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub Parameter: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Microsoft_UI_Xaml_Media_Animation")]
    pub NavigationTransitionInfo: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Microsoft_UI_Xaml_Media_Animation"))]
    NavigationTransitionInfo: usize,
    #[cfg(feature = "Windows_UI_Xaml_Interop")]
    pub SourcePageType: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<
            super::super::super::super::Windows::UI::Xaml::Interop::TypeName,
        >,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI_Xaml_Interop"))]
    SourcePageType: usize,
    pub NavigationMode: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut NavigationMode,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_Foundation")]
    pub Uri: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    Uri: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub SetUri: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    SetUri: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct INavigationFailedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for INavigationFailedEventArgs {
    type Vtable = INavigationFailedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for INavigationFailedEventArgs {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xf808f9a0_130c_5974_87f8_4433271a35a9);
}
#[repr(C)]
#[doc(hidden)]
pub struct INavigationFailedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Exception: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows_core::HRESULT,
    ) -> ::windows_core::HRESULT,
    pub Handled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_UI_Xaml_Interop")]
    pub SourcePageType: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<
            super::super::super::super::Windows::UI::Xaml::Interop::TypeName,
        >,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI_Xaml_Interop"))]
    SourcePageType: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPageStackEntry(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPageStackEntry {
    type Vtable = IPageStackEntry_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPageStackEntry {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xd591f56e_4262_5c91_9d79_29165cd82100);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPageStackEntry_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Windows_UI_Xaml_Interop")]
    pub SourcePageType: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<
            super::super::super::super::Windows::UI::Xaml::Interop::TypeName,
        >,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI_Xaml_Interop"))]
    SourcePageType: usize,
    pub Parameter: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Microsoft_UI_Xaml_Media_Animation")]
    pub NavigationTransitionInfo: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Microsoft_UI_Xaml_Media_Animation"))]
    NavigationTransitionInfo: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPageStackEntryFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPageStackEntryFactory {
    type Vtable = IPageStackEntryFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPageStackEntryFactory {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x7e5a9469_6108_5e92_a499_5ee9f065a68a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPageStackEntryFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(all(
        feature = "Microsoft_UI_Xaml_Media_Animation",
        feature = "Windows_UI_Xaml_Interop"
    ))]
    pub CreateInstance: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        sourcepagetype: ::std::mem::MaybeUninit<
            super::super::super::super::Windows::UI::Xaml::Interop::TypeName,
        >,
        parameter: *mut ::core::ffi::c_void,
        navigationtransitioninfo: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(all(
        feature = "Microsoft_UI_Xaml_Media_Animation",
        feature = "Windows_UI_Xaml_Interop"
    )))]
    CreateInstance: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPageStackEntryStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPageStackEntryStatics {
    type Vtable = IPageStackEntryStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPageStackEntryStatics {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x2f1d4cb7_923b_59bb_bfc4_750933f28385);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPageStackEntryStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SourcePageTypeProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct FrameNavigationOptions(::windows_core::IUnknown);
impl FrameNavigationOptions {
    pub fn IsNavigationStackEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsNavigationStackEnabled)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetIsNavigationStackEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetIsNavigationStackEnabled)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Media_Animation\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Media_Animation")]
    pub fn TransitionInfoOverride(
        &self,
    ) -> ::windows_core::Result<super::Media::Animation::NavigationTransitionInfo> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TransitionInfoOverride)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Media_Animation\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Media_Animation")]
    pub fn SetTransitionInfoOverride<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::Media::Animation::NavigationTransitionInfo>,
    {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetTransitionInfoOverride)(
                ::windows_core::Interface::as_raw(this),
                value.try_into_param()?.abi(),
            )
            .ok()
        }
    }
}
impl ::windows_core::RuntimeType for FrameNavigationOptions {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for FrameNavigationOptions {
    type Vtable = IFrameNavigationOptions_Vtbl;
}
unsafe impl ::windows_core::ComInterface for FrameNavigationOptions {
    const IID: ::windows_core::GUID =
        <IFrameNavigationOptions as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for FrameNavigationOptions {
    const NAME: &'static str = "Microsoft.UI.Xaml.Navigation.FrameNavigationOptions";
}
::windows_core::imp::interface_hierarchy!(
    FrameNavigationOptions,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for FrameNavigationOptions {}
unsafe impl ::core::marker::Sync for FrameNavigationOptions {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct NavigatingCancelEventArgs(::windows_core::IUnknown);
impl NavigatingCancelEventArgs {
    pub fn Cancel(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Cancel)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetCancel(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetCancel)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn NavigationMode(&self) -> ::windows_core::Result<NavigationMode> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NavigationMode)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_UI_Xaml_Interop\"`"]
    #[cfg(feature = "Windows_UI_Xaml_Interop")]
    pub fn SourcePageType(
        &self,
    ) -> ::windows_core::Result<super::super::super::super::Windows::UI::Xaml::Interop::TypeName>
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SourcePageType)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn Parameter(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Parameter)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Media_Animation\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Media_Animation")]
    pub fn NavigationTransitionInfo(
        &self,
    ) -> ::windows_core::Result<super::Media::Animation::NavigationTransitionInfo> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NavigationTransitionInfo)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for NavigatingCancelEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for NavigatingCancelEventArgs {
    type Vtable = INavigatingCancelEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for NavigatingCancelEventArgs {
    const IID: ::windows_core::GUID =
        <INavigatingCancelEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for NavigatingCancelEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Navigation.NavigatingCancelEventArgs";
}
::windows_core::imp::interface_hierarchy!(
    NavigatingCancelEventArgs,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for NavigatingCancelEventArgs {}
unsafe impl ::core::marker::Sync for NavigatingCancelEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct NavigationEventArgs(::windows_core::IUnknown);
impl NavigationEventArgs {
    pub fn Content(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Content)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn Parameter(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Parameter)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Media_Animation\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Media_Animation")]
    pub fn NavigationTransitionInfo(
        &self,
    ) -> ::windows_core::Result<super::Media::Animation::NavigationTransitionInfo> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NavigationTransitionInfo)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_UI_Xaml_Interop\"`"]
    #[cfg(feature = "Windows_UI_Xaml_Interop")]
    pub fn SourcePageType(
        &self,
    ) -> ::windows_core::Result<super::super::super::super::Windows::UI::Xaml::Interop::TypeName>
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SourcePageType)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn NavigationMode(&self) -> ::windows_core::Result<NavigationMode> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NavigationMode)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn Uri(&self) -> ::windows_core::Result<::windows::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Uri)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn SetUri<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows::Foundation::Uri>,
    {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetUri)(
                ::windows_core::Interface::as_raw(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
}
impl ::windows_core::RuntimeType for NavigationEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for NavigationEventArgs {
    type Vtable = INavigationEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for NavigationEventArgs {
    const IID: ::windows_core::GUID = <INavigationEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for NavigationEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Navigation.NavigationEventArgs";
}
::windows_core::imp::interface_hierarchy!(
    NavigationEventArgs,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for NavigationEventArgs {}
unsafe impl ::core::marker::Sync for NavigationEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct NavigationFailedEventArgs(::windows_core::IUnknown);
impl NavigationFailedEventArgs {
    pub fn Exception(&self) -> ::windows_core::Result<::windows_core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Exception)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn Handled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Handled)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetHandled)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_UI_Xaml_Interop\"`"]
    #[cfg(feature = "Windows_UI_Xaml_Interop")]
    pub fn SourcePageType(
        &self,
    ) -> ::windows_core::Result<super::super::super::super::Windows::UI::Xaml::Interop::TypeName>
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SourcePageType)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for NavigationFailedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for NavigationFailedEventArgs {
    type Vtable = INavigationFailedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for NavigationFailedEventArgs {
    const IID: ::windows_core::GUID =
        <INavigationFailedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for NavigationFailedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Navigation.NavigationFailedEventArgs";
}
::windows_core::imp::interface_hierarchy!(
    NavigationFailedEventArgs,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for NavigationFailedEventArgs {}
unsafe impl ::core::marker::Sync for NavigationFailedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct PageStackEntry(::windows_core::IUnknown);
impl PageStackEntry {
    pub fn GetValue<P0>(&self, dp: P0) -> ::windows_core::Result<::windows_core::IInspectable>
    where
        P0: ::windows_core::IntoParam<super::DependencyProperty>,
    {
        let this = &::windows_core::ComInterface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetValue)(
                ::windows_core::Interface::as_raw(this),
                dp.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetValue<P0, P1>(&self, dp: P0, value: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::DependencyProperty>,
        P1: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = &::windows_core::ComInterface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetValue)(
                ::windows_core::Interface::as_raw(this),
                dp.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn ClearValue<P0>(&self, dp: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::DependencyProperty>,
    {
        let this = &::windows_core::ComInterface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).ClearValue)(
                ::windows_core::Interface::as_raw(this),
                dp.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn ReadLocalValue<P0>(&self, dp: P0) -> ::windows_core::Result<::windows_core::IInspectable>
    where
        P0: ::windows_core::IntoParam<super::DependencyProperty>,
    {
        let this = &::windows_core::ComInterface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ReadLocalValue)(
                ::windows_core::Interface::as_raw(this),
                dp.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn GetAnimationBaseValue<P0>(
        &self,
        dp: P0,
    ) -> ::windows_core::Result<::windows_core::IInspectable>
    where
        P0: ::windows_core::IntoParam<super::DependencyProperty>,
    {
        let this = &::windows_core::ComInterface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetAnimationBaseValue)(
                ::windows_core::Interface::as_raw(this),
                dp.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn RegisterPropertyChangedCallback<P0, P1>(
        &self,
        dp: P0,
        callback: P1,
    ) -> ::windows_core::Result<i64>
    where
        P0: ::windows_core::IntoParam<super::DependencyProperty>,
        P1: ::windows_core::IntoParam<super::DependencyPropertyChangedCallback>,
    {
        let this = &::windows_core::ComInterface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RegisterPropertyChangedCallback)(
                ::windows_core::Interface::as_raw(this),
                dp.into_param().abi(),
                callback.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn UnregisterPropertyChangedCallback<P0>(
        &self,
        dp: P0,
        token: i64,
    ) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::DependencyProperty>,
    {
        let this = &::windows_core::ComInterface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).UnregisterPropertyChangedCallback)(
                ::windows_core::Interface::as_raw(this),
                dp.into_param().abi(),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_UI_Core\"`"]
    #[cfg(feature = "Windows_UI_Core")]
    pub fn Dispatcher(&self) -> ::windows_core::Result<::windows::UI::Core::CoreDispatcher> {
        let this = &::windows_core::ComInterface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Dispatcher)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Dispatching\"`"]
    #[cfg(feature = "Microsoft_UI_Dispatching")]
    pub fn DispatcherQueue(
        &self,
    ) -> ::windows_core::Result<super::super::Dispatching::DispatcherQueue> {
        let this = &::windows_core::ComInterface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DispatcherQueue)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_UI_Xaml_Interop\"`"]
    #[cfg(feature = "Windows_UI_Xaml_Interop")]
    pub fn SourcePageType(
        &self,
    ) -> ::windows_core::Result<super::super::super::super::Windows::UI::Xaml::Interop::TypeName>
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SourcePageType)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn Parameter(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Parameter)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Media_Animation\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Media_Animation")]
    pub fn NavigationTransitionInfo(
        &self,
    ) -> ::windows_core::Result<super::Media::Animation::NavigationTransitionInfo> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NavigationTransitionInfo)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Media_Animation\"`, `\"Windows_UI_Xaml_Interop\"`"]
    #[cfg(all(
        feature = "Microsoft_UI_Xaml_Media_Animation",
        feature = "Windows_UI_Xaml_Interop"
    ))]
    pub fn CreateInstance<P0, P1, P2>(
        sourcepagetype: P0,
        parameter: P1,
        navigationtransitioninfo: P2,
    ) -> ::windows_core::Result<PageStackEntry>
    where
        P0: ::windows_core::IntoParam<
            super::super::super::super::Windows::UI::Xaml::Interop::TypeName,
        >,
        P1: ::windows_core::IntoParam<::windows_core::IInspectable>,
        P2: ::windows_core::TryIntoParam<super::Media::Animation::NavigationTransitionInfo>,
    {
        Self::IPageStackEntryFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstance)(
                ::windows_core::Interface::as_raw(this),
                sourcepagetype.into_param().abi(),
                parameter.into_param().abi(),
                navigationtransitioninfo.try_into_param()?.abi(),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn SourcePageTypeProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IPageStackEntryStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SourcePageTypeProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPageStackEntryFactory<
        R,
        F: FnOnce(&IPageStackEntryFactory) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<PageStackEntry, IPageStackEntryFactory> =
            ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IPageStackEntryStatics<
        R,
        F: FnOnce(&IPageStackEntryStatics) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<PageStackEntry, IPageStackEntryStatics> =
            ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for PageStackEntry {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for PageStackEntry {
    type Vtable = IPageStackEntry_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PageStackEntry {
    const IID: ::windows_core::GUID = <IPageStackEntry as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PageStackEntry {
    const NAME: &'static str = "Microsoft.UI.Xaml.Navigation.PageStackEntry";
}
::windows_core::imp::interface_hierarchy!(
    PageStackEntry,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<super::DependencyObject> for PageStackEntry {}
unsafe impl ::core::marker::Send for PageStackEntry {}
unsafe impl ::core::marker::Sync for PageStackEntry {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NavigationCacheMode(pub i32);
impl NavigationCacheMode {
    pub const Disabled: Self = Self(0i32);
    pub const Required: Self = Self(1i32);
    pub const Enabled: Self = Self(2i32);
}
impl ::core::marker::Copy for NavigationCacheMode {}
impl ::core::clone::Clone for NavigationCacheMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NavigationCacheMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for NavigationCacheMode {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for NavigationCacheMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NavigationCacheMode").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for NavigationCacheMode {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::from_slice(
            b"enum(Microsoft.UI.Xaml.Navigation.NavigationCacheMode;i4)",
        );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NavigationMode(pub i32);
impl NavigationMode {
    pub const New: Self = Self(0i32);
    pub const Back: Self = Self(1i32);
    pub const Forward: Self = Self(2i32);
    pub const Refresh: Self = Self(3i32);
}
impl ::core::marker::Copy for NavigationMode {}
impl ::core::clone::Clone for NavigationMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NavigationMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for NavigationMode {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for NavigationMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NavigationMode").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for NavigationMode {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::from_slice(
            b"enum(Microsoft.UI.Xaml.Navigation.NavigationMode;i4)",
        );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct NavigatedEventHandler(pub ::windows_core::IUnknown);
impl NavigatedEventHandler {
    pub fn new<
        F: FnMut(
                ::core::option::Option<&::windows_core::IInspectable>,
                ::core::option::Option<&NavigationEventArgs>,
            ) -> ::windows_core::Result<()>
            + ::core::marker::Send
            + 'static,
    >(
        invoke: F,
    ) -> Self {
        let com = NavigatedEventHandlerBox::<F> {
            vtable: &NavigatedEventHandlerBox::<F>::VTABLE,
            count: ::windows_core::imp::RefCount::new(1),
            invoke,
        };
        unsafe { ::core::mem::transmute(::std::boxed::Box::new(com)) }
    }
    pub fn Invoke<P0, P1>(&self, sender: P0, e: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
        P1: ::windows_core::IntoParam<NavigationEventArgs>,
    {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).Invoke)(
                ::windows_core::Interface::as_raw(this),
                sender.into_param().abi(),
                e.into_param().abi(),
            )
            .ok()
        }
    }
}
#[repr(C)]
struct NavigatedEventHandlerBox<
    F: FnMut(
            ::core::option::Option<&::windows_core::IInspectable>,
            ::core::option::Option<&NavigationEventArgs>,
        ) -> ::windows_core::Result<()>
        + ::core::marker::Send
        + 'static,
> {
    vtable: *const NavigatedEventHandler_Vtbl,
    invoke: F,
    count: ::windows_core::imp::RefCount,
}
impl<
        F: FnMut(
                ::core::option::Option<&::windows_core::IInspectable>,
                ::core::option::Option<&NavigationEventArgs>,
            ) -> ::windows_core::Result<()>
            + ::core::marker::Send
            + 'static,
    > NavigatedEventHandlerBox<F>
{
    const VTABLE: NavigatedEventHandler_Vtbl = NavigatedEventHandler_Vtbl {
        base__: ::windows_core::IUnknown_Vtbl {
            QueryInterface: Self::QueryInterface,
            AddRef: Self::AddRef,
            Release: Self::Release,
        },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn QueryInterface(
        this: *mut ::core::ffi::c_void,
        iid: *const ::windows_core::GUID,
        interface: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        if iid.is_null() || interface.is_null() {
            return ::windows_core::HRESULT(-2147467261);
        }
        *interface = if *iid == <NavigatedEventHandler as ::windows_core::ComInterface>::IID
            || *iid == <::windows_core::IUnknown as ::windows_core::ComInterface>::IID
            || *iid == <::windows_core::imp::IAgileObject as ::windows_core::ComInterface>::IID
        {
            &mut (*this).vtable as *mut _ as _
        } else {
            ::core::ptr::null_mut()
        };
        if (*interface).is_null() {
            ::windows_core::HRESULT(-2147467262)
        } else {
            (*this).count.add_ref();
            ::windows_core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: *mut ::core::ffi::c_void) -> u32 {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: *mut ::core::ffi::c_void) -> u32 {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            let _ = ::std::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(
        this: *mut ::core::ffi::c_void,
        sender: *mut ::core::ffi::c_void,
        e: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        ((*this).invoke)(
            ::windows_core::from_raw_borrowed(&sender),
            ::windows_core::from_raw_borrowed(&e),
        )
        .into()
    }
}
unsafe impl ::windows_core::Interface for NavigatedEventHandler {
    type Vtable = NavigatedEventHandler_Vtbl;
}
unsafe impl ::windows_core::ComInterface for NavigatedEventHandler {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x8631b517_6d8e_58ee_82fe_d4034d1bd7c1);
}
impl ::windows_core::RuntimeType for NavigatedEventHandler {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct NavigatedEventHandler_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        sender: *mut ::core::ffi::c_void,
        e: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct NavigatingCancelEventHandler(pub ::windows_core::IUnknown);
impl NavigatingCancelEventHandler {
    pub fn new<
        F: FnMut(
                ::core::option::Option<&::windows_core::IInspectable>,
                ::core::option::Option<&NavigatingCancelEventArgs>,
            ) -> ::windows_core::Result<()>
            + ::core::marker::Send
            + 'static,
    >(
        invoke: F,
    ) -> Self {
        let com = NavigatingCancelEventHandlerBox::<F> {
            vtable: &NavigatingCancelEventHandlerBox::<F>::VTABLE,
            count: ::windows_core::imp::RefCount::new(1),
            invoke,
        };
        unsafe { ::core::mem::transmute(::std::boxed::Box::new(com)) }
    }
    pub fn Invoke<P0, P1>(&self, sender: P0, e: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
        P1: ::windows_core::IntoParam<NavigatingCancelEventArgs>,
    {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).Invoke)(
                ::windows_core::Interface::as_raw(this),
                sender.into_param().abi(),
                e.into_param().abi(),
            )
            .ok()
        }
    }
}
#[repr(C)]
struct NavigatingCancelEventHandlerBox<
    F: FnMut(
            ::core::option::Option<&::windows_core::IInspectable>,
            ::core::option::Option<&NavigatingCancelEventArgs>,
        ) -> ::windows_core::Result<()>
        + ::core::marker::Send
        + 'static,
> {
    vtable: *const NavigatingCancelEventHandler_Vtbl,
    invoke: F,
    count: ::windows_core::imp::RefCount,
}
impl<
        F: FnMut(
                ::core::option::Option<&::windows_core::IInspectable>,
                ::core::option::Option<&NavigatingCancelEventArgs>,
            ) -> ::windows_core::Result<()>
            + ::core::marker::Send
            + 'static,
    > NavigatingCancelEventHandlerBox<F>
{
    const VTABLE: NavigatingCancelEventHandler_Vtbl = NavigatingCancelEventHandler_Vtbl {
        base__: ::windows_core::IUnknown_Vtbl {
            QueryInterface: Self::QueryInterface,
            AddRef: Self::AddRef,
            Release: Self::Release,
        },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn QueryInterface(
        this: *mut ::core::ffi::c_void,
        iid: *const ::windows_core::GUID,
        interface: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        if iid.is_null() || interface.is_null() {
            return ::windows_core::HRESULT(-2147467261);
        }
        *interface = if *iid == <NavigatingCancelEventHandler as ::windows_core::ComInterface>::IID
            || *iid == <::windows_core::IUnknown as ::windows_core::ComInterface>::IID
            || *iid == <::windows_core::imp::IAgileObject as ::windows_core::ComInterface>::IID
        {
            &mut (*this).vtable as *mut _ as _
        } else {
            ::core::ptr::null_mut()
        };
        if (*interface).is_null() {
            ::windows_core::HRESULT(-2147467262)
        } else {
            (*this).count.add_ref();
            ::windows_core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: *mut ::core::ffi::c_void) -> u32 {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: *mut ::core::ffi::c_void) -> u32 {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            let _ = ::std::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(
        this: *mut ::core::ffi::c_void,
        sender: *mut ::core::ffi::c_void,
        e: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        ((*this).invoke)(
            ::windows_core::from_raw_borrowed(&sender),
            ::windows_core::from_raw_borrowed(&e),
        )
        .into()
    }
}
unsafe impl ::windows_core::Interface for NavigatingCancelEventHandler {
    type Vtable = NavigatingCancelEventHandler_Vtbl;
}
unsafe impl ::windows_core::ComInterface for NavigatingCancelEventHandler {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xfcae1401_ec94_565f_9f48_7c4b6272b3b1);
}
impl ::windows_core::RuntimeType for NavigatingCancelEventHandler {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct NavigatingCancelEventHandler_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        sender: *mut ::core::ffi::c_void,
        e: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct NavigationFailedEventHandler(pub ::windows_core::IUnknown);
impl NavigationFailedEventHandler {
    pub fn new<
        F: FnMut(
                ::core::option::Option<&::windows_core::IInspectable>,
                ::core::option::Option<&NavigationFailedEventArgs>,
            ) -> ::windows_core::Result<()>
            + ::core::marker::Send
            + 'static,
    >(
        invoke: F,
    ) -> Self {
        let com = NavigationFailedEventHandlerBox::<F> {
            vtable: &NavigationFailedEventHandlerBox::<F>::VTABLE,
            count: ::windows_core::imp::RefCount::new(1),
            invoke,
        };
        unsafe { ::core::mem::transmute(::std::boxed::Box::new(com)) }
    }
    pub fn Invoke<P0, P1>(&self, sender: P0, e: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
        P1: ::windows_core::IntoParam<NavigationFailedEventArgs>,
    {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).Invoke)(
                ::windows_core::Interface::as_raw(this),
                sender.into_param().abi(),
                e.into_param().abi(),
            )
            .ok()
        }
    }
}
#[repr(C)]
struct NavigationFailedEventHandlerBox<
    F: FnMut(
            ::core::option::Option<&::windows_core::IInspectable>,
            ::core::option::Option<&NavigationFailedEventArgs>,
        ) -> ::windows_core::Result<()>
        + ::core::marker::Send
        + 'static,
> {
    vtable: *const NavigationFailedEventHandler_Vtbl,
    invoke: F,
    count: ::windows_core::imp::RefCount,
}
impl<
        F: FnMut(
                ::core::option::Option<&::windows_core::IInspectable>,
                ::core::option::Option<&NavigationFailedEventArgs>,
            ) -> ::windows_core::Result<()>
            + ::core::marker::Send
            + 'static,
    > NavigationFailedEventHandlerBox<F>
{
    const VTABLE: NavigationFailedEventHandler_Vtbl = NavigationFailedEventHandler_Vtbl {
        base__: ::windows_core::IUnknown_Vtbl {
            QueryInterface: Self::QueryInterface,
            AddRef: Self::AddRef,
            Release: Self::Release,
        },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn QueryInterface(
        this: *mut ::core::ffi::c_void,
        iid: *const ::windows_core::GUID,
        interface: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        if iid.is_null() || interface.is_null() {
            return ::windows_core::HRESULT(-2147467261);
        }
        *interface = if *iid == <NavigationFailedEventHandler as ::windows_core::ComInterface>::IID
            || *iid == <::windows_core::IUnknown as ::windows_core::ComInterface>::IID
            || *iid == <::windows_core::imp::IAgileObject as ::windows_core::ComInterface>::IID
        {
            &mut (*this).vtable as *mut _ as _
        } else {
            ::core::ptr::null_mut()
        };
        if (*interface).is_null() {
            ::windows_core::HRESULT(-2147467262)
        } else {
            (*this).count.add_ref();
            ::windows_core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: *mut ::core::ffi::c_void) -> u32 {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: *mut ::core::ffi::c_void) -> u32 {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            let _ = ::std::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(
        this: *mut ::core::ffi::c_void,
        sender: *mut ::core::ffi::c_void,
        e: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        ((*this).invoke)(
            ::windows_core::from_raw_borrowed(&sender),
            ::windows_core::from_raw_borrowed(&e),
        )
        .into()
    }
}
unsafe impl ::windows_core::Interface for NavigationFailedEventHandler {
    type Vtable = NavigationFailedEventHandler_Vtbl;
}
unsafe impl ::windows_core::ComInterface for NavigationFailedEventHandler {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x97ca2b56_d6eb_5fd2_a675_a339640eedba);
}
impl ::windows_core::RuntimeType for NavigationFailedEventHandler {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct NavigationFailedEventHandler_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        sender: *mut ::core::ffi::c_void,
        e: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct NavigationStoppedEventHandler(pub ::windows_core::IUnknown);
impl NavigationStoppedEventHandler {
    pub fn new<
        F: FnMut(
                ::core::option::Option<&::windows_core::IInspectable>,
                ::core::option::Option<&NavigationEventArgs>,
            ) -> ::windows_core::Result<()>
            + ::core::marker::Send
            + 'static,
    >(
        invoke: F,
    ) -> Self {
        let com = NavigationStoppedEventHandlerBox::<F> {
            vtable: &NavigationStoppedEventHandlerBox::<F>::VTABLE,
            count: ::windows_core::imp::RefCount::new(1),
            invoke,
        };
        unsafe { ::core::mem::transmute(::std::boxed::Box::new(com)) }
    }
    pub fn Invoke<P0, P1>(&self, sender: P0, e: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
        P1: ::windows_core::IntoParam<NavigationEventArgs>,
    {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).Invoke)(
                ::windows_core::Interface::as_raw(this),
                sender.into_param().abi(),
                e.into_param().abi(),
            )
            .ok()
        }
    }
}
#[repr(C)]
struct NavigationStoppedEventHandlerBox<
    F: FnMut(
            ::core::option::Option<&::windows_core::IInspectable>,
            ::core::option::Option<&NavigationEventArgs>,
        ) -> ::windows_core::Result<()>
        + ::core::marker::Send
        + 'static,
> {
    vtable: *const NavigationStoppedEventHandler_Vtbl,
    invoke: F,
    count: ::windows_core::imp::RefCount,
}
impl<
        F: FnMut(
                ::core::option::Option<&::windows_core::IInspectable>,
                ::core::option::Option<&NavigationEventArgs>,
            ) -> ::windows_core::Result<()>
            + ::core::marker::Send
            + 'static,
    > NavigationStoppedEventHandlerBox<F>
{
    const VTABLE: NavigationStoppedEventHandler_Vtbl = NavigationStoppedEventHandler_Vtbl {
        base__: ::windows_core::IUnknown_Vtbl {
            QueryInterface: Self::QueryInterface,
            AddRef: Self::AddRef,
            Release: Self::Release,
        },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn QueryInterface(
        this: *mut ::core::ffi::c_void,
        iid: *const ::windows_core::GUID,
        interface: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        if iid.is_null() || interface.is_null() {
            return ::windows_core::HRESULT(-2147467261);
        }
        *interface = if *iid == <NavigationStoppedEventHandler as ::windows_core::ComInterface>::IID
            || *iid == <::windows_core::IUnknown as ::windows_core::ComInterface>::IID
            || *iid == <::windows_core::imp::IAgileObject as ::windows_core::ComInterface>::IID
        {
            &mut (*this).vtable as *mut _ as _
        } else {
            ::core::ptr::null_mut()
        };
        if (*interface).is_null() {
            ::windows_core::HRESULT(-2147467262)
        } else {
            (*this).count.add_ref();
            ::windows_core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: *mut ::core::ffi::c_void) -> u32 {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: *mut ::core::ffi::c_void) -> u32 {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            let _ = ::std::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(
        this: *mut ::core::ffi::c_void,
        sender: *mut ::core::ffi::c_void,
        e: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        ((*this).invoke)(
            ::windows_core::from_raw_borrowed(&sender),
            ::windows_core::from_raw_borrowed(&e),
        )
        .into()
    }
}
unsafe impl ::windows_core::Interface for NavigationStoppedEventHandler {
    type Vtable = NavigationStoppedEventHandler_Vtbl;
}
unsafe impl ::windows_core::ComInterface for NavigationStoppedEventHandler {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xb9e796a6_7ffe_5a63_aef4_cbc331663b66);
}
impl ::windows_core::RuntimeType for NavigationStoppedEventHandler {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct NavigationStoppedEventHandler_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        sender: *mut ::core::ffi::c_void,
        e: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
