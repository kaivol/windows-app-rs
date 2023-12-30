#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IAddPagesEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAddPagesEventArgs {
    type Vtable = IAddPagesEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IAddPagesEventArgs {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xa69f3cb3_6b74_5ee8_b034_188098a98c5d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAddPagesEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Windows_Graphics_Printing")]
    pub PrintTaskOptions: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Graphics_Printing"))]
    PrintTaskOptions: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IGetPreviewPageEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGetPreviewPageEventArgs {
    type Vtable = IGetPreviewPageEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IGetPreviewPageEventArgs {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xa68fbe17_f577_597f_b3ab_b379447149e6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGetPreviewPageEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub PageNumber: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut i32,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPaginateEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPaginateEventArgs {
    type Vtable = IPaginateEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPaginateEventArgs {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x6499c196_11a9_5ef8_91cb_52fb963bf172);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPaginateEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Windows_Graphics_Printing")]
    pub PrintTaskOptions: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Graphics_Printing"))]
    PrintTaskOptions: usize,
    pub CurrentPreviewPageNumber: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut i32,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPrintDocument(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPrintDocument {
    type Vtable = IPrintDocument_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPrintDocument {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x1e40f1fc_5d33_5fe9_ba3e_954c0d161524);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintDocument_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Windows_Graphics_Printing")]
    pub DocumentSource: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Graphics_Printing"))]
    DocumentSource: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub Paginate: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    Paginate: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub RemovePaginate: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RemovePaginate: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub GetPreviewPage: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    GetPreviewPage: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub RemoveGetPreviewPage: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RemoveGetPreviewPage: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub AddPages: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    AddPages: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub RemoveAddPages: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RemoveAddPages: usize,
    pub AddPage: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        pagevisual: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub AddPagesComplete:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetPreviewPageCount: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        count: i32,
        r#type: PreviewPageCountType,
    ) -> ::windows_core::HRESULT,
    pub SetPreviewPage: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        pagenumber: i32,
        pagevisual: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub InvalidatePreview:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPrintDocumentFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPrintDocumentFactory {
    type Vtable = IPrintDocumentFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPrintDocumentFactory {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xc4c1bc12_84d1_539c_b416_d7e54c45ab58);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintDocumentFactory_Vtbl {
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
pub struct IPrintDocumentStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPrintDocumentStatics {
    type Vtable = IPrintDocumentStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPrintDocumentStatics {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x8975e4bc_8fc8_5e8f_a55a_bf71b9a827b7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintDocumentStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub DocumentSourceProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct AddPagesEventArgs(::windows_core::IUnknown);
impl AddPagesEventArgs {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<
            AddPagesEventArgs,
            ::windows_core::imp::IGenericFactory,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "Required features: `\"Windows_Graphics_Printing\"`"]
    #[cfg(feature = "Windows_Graphics_Printing")]
    pub fn PrintTaskOptions(
        &self,
    ) -> ::windows_core::Result<::windows::Graphics::Printing::PrintTaskOptions> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PrintTaskOptions)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for AddPagesEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for AddPagesEventArgs {
    type Vtable = IAddPagesEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for AddPagesEventArgs {
    const IID: ::windows_core::GUID = <IAddPagesEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for AddPagesEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Printing.AddPagesEventArgs";
}
::windows_core::imp::interface_hierarchy!(
    AddPagesEventArgs,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for AddPagesEventArgs {}
unsafe impl ::core::marker::Sync for AddPagesEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct GetPreviewPageEventArgs(::windows_core::IUnknown);
impl GetPreviewPageEventArgs {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<
            GetPreviewPageEventArgs,
            ::windows_core::imp::IGenericFactory,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn PageNumber(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PageNumber)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for GetPreviewPageEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for GetPreviewPageEventArgs {
    type Vtable = IGetPreviewPageEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for GetPreviewPageEventArgs {
    const IID: ::windows_core::GUID =
        <IGetPreviewPageEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for GetPreviewPageEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Printing.GetPreviewPageEventArgs";
}
::windows_core::imp::interface_hierarchy!(
    GetPreviewPageEventArgs,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for GetPreviewPageEventArgs {}
unsafe impl ::core::marker::Sync for GetPreviewPageEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct PaginateEventArgs(::windows_core::IUnknown);
impl PaginateEventArgs {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<
            PaginateEventArgs,
            ::windows_core::imp::IGenericFactory,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "Required features: `\"Windows_Graphics_Printing\"`"]
    #[cfg(feature = "Windows_Graphics_Printing")]
    pub fn PrintTaskOptions(
        &self,
    ) -> ::windows_core::Result<::windows::Graphics::Printing::PrintTaskOptions> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PrintTaskOptions)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn CurrentPreviewPageNumber(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CurrentPreviewPageNumber)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for PaginateEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for PaginateEventArgs {
    type Vtable = IPaginateEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PaginateEventArgs {
    const IID: ::windows_core::GUID = <IPaginateEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PaginateEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Printing.PaginateEventArgs";
}
::windows_core::imp::interface_hierarchy!(
    PaginateEventArgs,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for PaginateEventArgs {}
unsafe impl ::core::marker::Sync for PaginateEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct PrintDocument(::windows_core::IUnknown);
impl PrintDocument {
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
    #[doc = "Required features: `\"Windows_Graphics_Printing\"`"]
    #[cfg(feature = "Windows_Graphics_Printing")]
    pub fn DocumentSource(
        &self,
    ) -> ::windows_core::Result<::windows::Graphics::Printing::IPrintDocumentSource> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DocumentSource)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn Paginate<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<PaginateEventHandler>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Paginate)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemovePaginate(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).RemovePaginate)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn GetPreviewPage<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<GetPreviewPageEventHandler>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetPreviewPage)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveGetPreviewPage(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveGetPreviewPage)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn AddPages<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<AddPagesEventHandler>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AddPages)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveAddPages(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveAddPages)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn AddPage<P0>(&self, pagevisual: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::UIElement>,
    {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).AddPage)(
                ::windows_core::Interface::as_raw(this),
                pagevisual.try_into_param()?.abi(),
            )
            .ok()
        }
    }
    pub fn AddPagesComplete(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).AddPagesComplete)(
                ::windows_core::Interface::as_raw(this),
            )
            .ok()
        }
    }
    pub fn SetPreviewPageCount(
        &self,
        count: i32,
        r#type: PreviewPageCountType,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetPreviewPageCount)(
                ::windows_core::Interface::as_raw(this),
                count,
                r#type,
            )
            .ok()
        }
    }
    pub fn SetPreviewPage<P0>(&self, pagenumber: i32, pagevisual: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::UIElement>,
    {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetPreviewPage)(
                ::windows_core::Interface::as_raw(this),
                pagenumber,
                pagevisual.try_into_param()?.abi(),
            )
            .ok()
        }
    }
    pub fn InvalidatePreview(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).InvalidatePreview)(
                ::windows_core::Interface::as_raw(this),
            )
            .ok()
        }
    }
    pub unsafe fn CreateInstance<P0>(
        baseinterface: P0,
        innerinterface: &mut ::core::option::Option<::windows_core::IInspectable>,
    ) -> ::windows_core::Result<PrintDocument>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        Self::IPrintDocumentFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstance)(
                ::windows_core::Interface::as_raw(this),
                baseinterface.into_param().abi(),
                innerinterface as *mut _ as _,
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn DocumentSourceProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IPrintDocumentStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DocumentSourceProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPrintDocumentFactory<
        R,
        F: FnOnce(&IPrintDocumentFactory) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<PrintDocument, IPrintDocumentFactory> =
            ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IPrintDocumentStatics<
        R,
        F: FnOnce(&IPrintDocumentStatics) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<PrintDocument, IPrintDocumentStatics> =
            ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for PrintDocument {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for PrintDocument {
    type Vtable = IPrintDocument_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PrintDocument {
    const IID: ::windows_core::GUID = <IPrintDocument as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PrintDocument {
    const NAME: &'static str = "Microsoft.UI.Xaml.Printing.PrintDocument";
}
::windows_core::imp::interface_hierarchy!(
    PrintDocument,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<super::DependencyObject> for PrintDocument {}
unsafe impl ::core::marker::Send for PrintDocument {}
unsafe impl ::core::marker::Sync for PrintDocument {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PreviewPageCountType(pub i32);
impl PreviewPageCountType {
    pub const Final: Self = Self(0i32);
    pub const Intermediate: Self = Self(1i32);
}
impl ::core::marker::Copy for PreviewPageCountType {}
impl ::core::clone::Clone for PreviewPageCountType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PreviewPageCountType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for PreviewPageCountType {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for PreviewPageCountType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PreviewPageCountType").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for PreviewPageCountType {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::from_slice(
            b"enum(Microsoft.UI.Xaml.Printing.PreviewPageCountType;i4)",
        );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct AddPagesEventHandler(pub ::windows_core::IUnknown);
impl AddPagesEventHandler {
    pub fn new<
        F: FnMut(
                ::core::option::Option<&::windows_core::IInspectable>,
                ::core::option::Option<&AddPagesEventArgs>,
            ) -> ::windows_core::Result<()>
            + ::core::marker::Send
            + 'static,
    >(
        invoke: F,
    ) -> Self {
        let com = AddPagesEventHandlerBox::<F> {
            vtable: &AddPagesEventHandlerBox::<F>::VTABLE,
            count: ::windows_core::imp::RefCount::new(1),
            invoke,
        };
        unsafe { ::core::mem::transmute(::std::boxed::Box::new(com)) }
    }
    pub fn Invoke<P0, P1>(&self, sender: P0, e: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
        P1: ::windows_core::IntoParam<AddPagesEventArgs>,
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
struct AddPagesEventHandlerBox<
    F: FnMut(
            ::core::option::Option<&::windows_core::IInspectable>,
            ::core::option::Option<&AddPagesEventArgs>,
        ) -> ::windows_core::Result<()>
        + ::core::marker::Send
        + 'static,
> {
    vtable: *const AddPagesEventHandler_Vtbl,
    invoke: F,
    count: ::windows_core::imp::RefCount,
}
impl<
        F: FnMut(
                ::core::option::Option<&::windows_core::IInspectable>,
                ::core::option::Option<&AddPagesEventArgs>,
            ) -> ::windows_core::Result<()>
            + ::core::marker::Send
            + 'static,
    > AddPagesEventHandlerBox<F>
{
    const VTABLE: AddPagesEventHandler_Vtbl = AddPagesEventHandler_Vtbl {
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
        *interface = if *iid == <AddPagesEventHandler as ::windows_core::ComInterface>::IID
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
unsafe impl ::windows_core::Interface for AddPagesEventHandler {
    type Vtable = AddPagesEventHandler_Vtbl;
}
unsafe impl ::windows_core::ComInterface for AddPagesEventHandler {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xed77566a_bd03_5118_b7c3_d9cea64307dd);
}
impl ::windows_core::RuntimeType for AddPagesEventHandler {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct AddPagesEventHandler_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        sender: *mut ::core::ffi::c_void,
        e: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct GetPreviewPageEventHandler(pub ::windows_core::IUnknown);
impl GetPreviewPageEventHandler {
    pub fn new<
        F: FnMut(
                ::core::option::Option<&::windows_core::IInspectable>,
                ::core::option::Option<&GetPreviewPageEventArgs>,
            ) -> ::windows_core::Result<()>
            + ::core::marker::Send
            + 'static,
    >(
        invoke: F,
    ) -> Self {
        let com = GetPreviewPageEventHandlerBox::<F> {
            vtable: &GetPreviewPageEventHandlerBox::<F>::VTABLE,
            count: ::windows_core::imp::RefCount::new(1),
            invoke,
        };
        unsafe { ::core::mem::transmute(::std::boxed::Box::new(com)) }
    }
    pub fn Invoke<P0, P1>(&self, sender: P0, e: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
        P1: ::windows_core::IntoParam<GetPreviewPageEventArgs>,
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
struct GetPreviewPageEventHandlerBox<
    F: FnMut(
            ::core::option::Option<&::windows_core::IInspectable>,
            ::core::option::Option<&GetPreviewPageEventArgs>,
        ) -> ::windows_core::Result<()>
        + ::core::marker::Send
        + 'static,
> {
    vtable: *const GetPreviewPageEventHandler_Vtbl,
    invoke: F,
    count: ::windows_core::imp::RefCount,
}
impl<
        F: FnMut(
                ::core::option::Option<&::windows_core::IInspectable>,
                ::core::option::Option<&GetPreviewPageEventArgs>,
            ) -> ::windows_core::Result<()>
            + ::core::marker::Send
            + 'static,
    > GetPreviewPageEventHandlerBox<F>
{
    const VTABLE: GetPreviewPageEventHandler_Vtbl = GetPreviewPageEventHandler_Vtbl {
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
        *interface = if *iid == <GetPreviewPageEventHandler as ::windows_core::ComInterface>::IID
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
unsafe impl ::windows_core::Interface for GetPreviewPageEventHandler {
    type Vtable = GetPreviewPageEventHandler_Vtbl;
}
unsafe impl ::windows_core::ComInterface for GetPreviewPageEventHandler {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x1c801689_a018_5574_9109_bcef62176da2);
}
impl ::windows_core::RuntimeType for GetPreviewPageEventHandler {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct GetPreviewPageEventHandler_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        sender: *mut ::core::ffi::c_void,
        e: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct PaginateEventHandler(pub ::windows_core::IUnknown);
impl PaginateEventHandler {
    pub fn new<
        F: FnMut(
                ::core::option::Option<&::windows_core::IInspectable>,
                ::core::option::Option<&PaginateEventArgs>,
            ) -> ::windows_core::Result<()>
            + ::core::marker::Send
            + 'static,
    >(
        invoke: F,
    ) -> Self {
        let com = PaginateEventHandlerBox::<F> {
            vtable: &PaginateEventHandlerBox::<F>::VTABLE,
            count: ::windows_core::imp::RefCount::new(1),
            invoke,
        };
        unsafe { ::core::mem::transmute(::std::boxed::Box::new(com)) }
    }
    pub fn Invoke<P0, P1>(&self, sender: P0, e: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
        P1: ::windows_core::IntoParam<PaginateEventArgs>,
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
struct PaginateEventHandlerBox<
    F: FnMut(
            ::core::option::Option<&::windows_core::IInspectable>,
            ::core::option::Option<&PaginateEventArgs>,
        ) -> ::windows_core::Result<()>
        + ::core::marker::Send
        + 'static,
> {
    vtable: *const PaginateEventHandler_Vtbl,
    invoke: F,
    count: ::windows_core::imp::RefCount,
}
impl<
        F: FnMut(
                ::core::option::Option<&::windows_core::IInspectable>,
                ::core::option::Option<&PaginateEventArgs>,
            ) -> ::windows_core::Result<()>
            + ::core::marker::Send
            + 'static,
    > PaginateEventHandlerBox<F>
{
    const VTABLE: PaginateEventHandler_Vtbl = PaginateEventHandler_Vtbl {
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
        *interface = if *iid == <PaginateEventHandler as ::windows_core::ComInterface>::IID
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
unsafe impl ::windows_core::Interface for PaginateEventHandler {
    type Vtable = PaginateEventHandler_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PaginateEventHandler {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xc291876c_343a_5b9b_a36c_8415ba4cd9dd);
}
impl ::windows_core::RuntimeType for PaginateEventHandler {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct PaginateEventHandler_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        sender: *mut ::core::ffi::c_void,
        e: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
