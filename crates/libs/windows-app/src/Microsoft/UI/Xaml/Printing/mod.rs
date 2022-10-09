#[doc(hidden)]
#[repr(transparent)]
pub struct IAddPagesEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IAddPagesEventArgs {
    type Vtable = IAddPagesEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IAddPagesEventArgs {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xa69f3cb3_6b74_5ee8_b034_188098a98c5d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAddPagesEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub PrintTaskOptions: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGetPreviewPageEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IGetPreviewPageEventArgs {
    type Vtable = IGetPreviewPageEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IGetPreviewPageEventArgs {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xa68fbe17_f577_597f_b3ab_b379447149e6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGetPreviewPageEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub PageNumber: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut i32,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPaginateEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IPaginateEventArgs {
    type Vtable = IPaginateEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IPaginateEventArgs {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x6499c196_11a9_5ef8_91cb_52fb963bf172);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPaginateEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub PrintTaskOptions: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub CurrentPreviewPageNumber: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut i32,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintDocument(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IPrintDocument {
    type Vtable = IPrintDocument_Vtbl;
}
unsafe impl ::windows::core::Interface for IPrintDocument {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x1e40f1fc_5d33_5fe9_ba3e_954c0d161524);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintDocument_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub DocumentSource: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub Paginate: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub RemovePaginate: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub GetPreviewPage: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub RemoveGetPreviewPage: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub AddPages: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub RemoveAddPages: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub AddPage: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        pagevisual: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub AddPagesComplete:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetPreviewPageCount: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        count: i32,
        r#type: PreviewPageCountType,
    ) -> ::windows::core::HRESULT,
    pub SetPreviewPage: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        pagenumber: i32,
        pagevisual: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub InvalidatePreview:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintDocumentFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IPrintDocumentFactory {
    type Vtable = IPrintDocumentFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for IPrintDocumentFactory {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xc4c1bc12_84d1_539c_b416_d7e54c45ab58);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintDocumentFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        baseinterface: *mut ::core::ffi::c_void,
        innerinterface: *mut *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintDocumentStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IPrintDocumentStatics {
    type Vtable = IPrintDocumentStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IPrintDocumentStatics {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x8975e4bc_8fc8_5e8f_a55a_bf71b9a827b7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintDocumentStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub DocumentSourceProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Xaml_Printing\"`*"]
#[repr(transparent)]
pub struct AddPagesEventArgs(::windows::core::IUnknown);
impl AddPagesEventArgs {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<
            AddPagesEventArgs,
            ::windows::core::IGenericFactory,
        > = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn PrintTaskOptions(
        &self,
    ) -> ::windows::core::Result<::windows::Graphics::Printing::PrintTaskOptions> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PrintTaskOptions)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Graphics::Printing::PrintTaskOptions>(result__)
        }
    }
}
impl ::core::clone::Clone for AddPagesEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AddPagesEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AddPagesEventArgs {}
impl ::core::fmt::Debug for AddPagesEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AddPagesEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AddPagesEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Xaml.Printing.AddPagesEventArgs;{a69f3cb3-6b74-5ee8-b034-188098a98c5d})",
    );
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for AddPagesEventArgs {
    type Vtable = IAddPagesEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for AddPagesEventArgs {
    const IID: ::windows::core::GUID = <IAddPagesEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AddPagesEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Printing.AddPagesEventArgs";
}
::windows::core::interface_hierarchy!(
    AddPagesEventArgs,
    ::windows::core::IUnknown,
    ::windows::core::IInspectable
);
unsafe impl ::core::marker::Send for AddPagesEventArgs {}
unsafe impl ::core::marker::Sync for AddPagesEventArgs {}
#[doc = "*Required features: `\"UI_Xaml_Printing\"`*"]
#[repr(transparent)]
pub struct GetPreviewPageEventArgs(::windows::core::IUnknown);
impl GetPreviewPageEventArgs {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<
            GetPreviewPageEventArgs,
            ::windows::core::IGenericFactory,
        > = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn PageNumber(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PageNumber)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<i32>(result__)
        }
    }
}
impl ::core::clone::Clone for GetPreviewPageEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for GetPreviewPageEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GetPreviewPageEventArgs {}
impl ::core::fmt::Debug for GetPreviewPageEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GetPreviewPageEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for GetPreviewPageEventArgs {
    const SIGNATURE : ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice ( b"rc(Microsoft.UI.Xaml.Printing.GetPreviewPageEventArgs;{a68fbe17-f577-597f-b3ab-b379447149e6})" ) ;
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for GetPreviewPageEventArgs {
    type Vtable = IGetPreviewPageEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for GetPreviewPageEventArgs {
    const IID: ::windows::core::GUID =
        <IGetPreviewPageEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for GetPreviewPageEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Printing.GetPreviewPageEventArgs";
}
::windows::core::interface_hierarchy!(
    GetPreviewPageEventArgs,
    ::windows::core::IUnknown,
    ::windows::core::IInspectable
);
unsafe impl ::core::marker::Send for GetPreviewPageEventArgs {}
unsafe impl ::core::marker::Sync for GetPreviewPageEventArgs {}
#[doc = "*Required features: `\"UI_Xaml_Printing\"`*"]
#[repr(transparent)]
pub struct PaginateEventArgs(::windows::core::IUnknown);
impl PaginateEventArgs {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<
            PaginateEventArgs,
            ::windows::core::IGenericFactory,
        > = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn PrintTaskOptions(
        &self,
    ) -> ::windows::core::Result<::windows::Graphics::Printing::PrintTaskOptions> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PrintTaskOptions)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Graphics::Printing::PrintTaskOptions>(result__)
        }
    }
    pub fn CurrentPreviewPageNumber(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CurrentPreviewPageNumber)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<i32>(result__)
        }
    }
}
impl ::core::clone::Clone for PaginateEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PaginateEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PaginateEventArgs {}
impl ::core::fmt::Debug for PaginateEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PaginateEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PaginateEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Xaml.Printing.PaginateEventArgs;{6499c196-11a9-5ef8-91cb-52fb963bf172})",
    );
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for PaginateEventArgs {
    type Vtable = IPaginateEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for PaginateEventArgs {
    const IID: ::windows::core::GUID = <IPaginateEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PaginateEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Printing.PaginateEventArgs";
}
::windows::core::interface_hierarchy!(
    PaginateEventArgs,
    ::windows::core::IUnknown,
    ::windows::core::IInspectable
);
unsafe impl ::core::marker::Send for PaginateEventArgs {}
unsafe impl ::core::marker::Sync for PaginateEventArgs {}
#[doc = "*Required features: `\"UI_Xaml_Printing\"`*"]
#[repr(transparent)]
pub struct PrintDocument(::windows::core::IUnknown);
impl PrintDocument {
    pub fn GetValue(
        &self,
        dp: &super::DependencyProperty,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetValue)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(dp),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn SetValue<'a, P0>(
        &self,
        dp: &super::DependencyProperty,
        value: P0,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IInspectable>>,
    {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetValue)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(dp),
                value.into().abi(),
            )
            .ok()
        }
    }
    pub fn ClearValue(&self, dp: &super::DependencyProperty) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).ClearValue)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(dp),
            )
            .ok()
        }
    }
    pub fn ReadLocalValue(
        &self,
        dp: &super::DependencyProperty,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ReadLocalValue)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(dp),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn GetAnimationBaseValue(
        &self,
        dp: &super::DependencyProperty,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetAnimationBaseValue)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(dp),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn RegisterPropertyChangedCallback(
        &self,
        dp: &super::DependencyProperty,
        callback: &super::DependencyPropertyChangedCallback,
    ) -> ::windows::core::Result<i64> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RegisterPropertyChangedCallback)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(dp),
                ::core::mem::transmute_copy(callback),
                result__.as_mut_ptr(),
            )
            .from_abi::<i64>(result__)
        }
    }
    pub fn UnregisterPropertyChangedCallback(
        &self,
        dp: &super::DependencyProperty,
        token: i64,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).UnregisterPropertyChangedCallback)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(dp),
                token,
            )
            .ok()
        }
    }
    pub fn Dispatcher(&self) -> ::windows::core::Result<::windows::UI::Core::CoreDispatcher> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Dispatcher)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Core::CoreDispatcher>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Dispatching\"`*"]
    #[cfg(feature = "UI_Dispatching")]
    pub fn DispatcherQueue(
        &self,
    ) -> ::windows::core::Result<super::super::Dispatching::DispatcherQueue> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DispatcherQueue)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::super::Dispatching::DispatcherQueue>(result__)
        }
    }
    pub fn DocumentSource(
        &self,
    ) -> ::windows::core::Result<::windows::Graphics::Printing::IPrintDocumentSource> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DocumentSource)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Graphics::Printing::IPrintDocumentSource>(result__)
        }
    }
    pub fn Paginate(
        &self,
        handler: &PaginateEventHandler,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Paginate)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(handler),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemovePaginate(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).RemovePaginate)(
                ::windows::core::Vtable::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn GetPreviewPage(
        &self,
        handler: &GetPreviewPageEventHandler,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetPreviewPage)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(handler),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveGetPreviewPage(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).RemoveGetPreviewPage)(
                ::windows::core::Vtable::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn AddPages(
        &self,
        handler: &AddPagesEventHandler,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AddPages)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(handler),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveAddPages(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).RemoveAddPages)(
                ::windows::core::Vtable::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn AddPage<'a, P0>(&self, pagevisual: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::UIElement>>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).AddPage)(
                ::windows::core::Vtable::as_raw(this),
                pagevisual.into().abi(),
            )
            .ok()
        }
    }
    pub fn AddPagesComplete(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).AddPagesComplete)(
                ::windows::core::Vtable::as_raw(this),
            )
            .ok()
        }
    }
    pub fn SetPreviewPageCount(
        &self,
        count: i32,
        r#type: PreviewPageCountType,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetPreviewPageCount)(
                ::windows::core::Vtable::as_raw(this),
                count,
                r#type,
            )
            .ok()
        }
    }
    pub fn SetPreviewPage<'a, P0>(
        &self,
        pagenumber: i32,
        pagevisual: P0,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::UIElement>>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetPreviewPage)(
                ::windows::core::Vtable::as_raw(this),
                pagenumber,
                pagevisual.into().abi(),
            )
            .ok()
        }
    }
    pub fn InvalidatePreview(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).InvalidatePreview)(
                ::windows::core::Vtable::as_raw(this),
            )
            .ok()
        }
    }
    pub fn new() -> ::windows::core::Result<PrintDocument> {
        Self::IPrintDocumentFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateInstance)(
                ::windows::core::Vtable::as_raw(this),
                ::core::ptr::null_mut(),
                &mut ::core::option::Option::<::windows::core::IInspectable>::None as *mut _ as _,
                result__.as_mut_ptr(),
            )
            .from_abi::<PrintDocument>(result__)
        })
    }
    pub fn compose<T>(compose: T) -> ::windows::core::Result<PrintDocument>
    where
        T: ::windows::core::Compose,
    {
        Self::IPrintDocumentFactory(|this| unsafe {
            let (derived__, base__) = ::windows::core::Compose::compose(compose);
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateInstance)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(&derived__),
                base__ as *mut _ as _,
                result__.as_mut_ptr(),
            )
            .from_abi::<PrintDocument>(result__)
        })
    }
    pub fn DocumentSourceProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IPrintDocumentStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DocumentSourceProperty)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPrintDocumentFactory<
        R,
        F: FnOnce(&IPrintDocumentFactory) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<PrintDocument, IPrintDocumentFactory> =
            ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IPrintDocumentStatics<
        R,
        F: FnOnce(&IPrintDocumentStatics) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<PrintDocument, IPrintDocumentStatics> =
            ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for PrintDocument {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PrintDocument {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PrintDocument {}
impl ::core::fmt::Debug for PrintDocument {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintDocument").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PrintDocument {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Xaml.Printing.PrintDocument;{1e40f1fc-5d33-5fe9-ba3e-954c0d161524})",
    );
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for PrintDocument {
    type Vtable = IPrintDocument_Vtbl;
}
unsafe impl ::windows::core::Interface for PrintDocument {
    const IID: ::windows::core::GUID = <IPrintDocument as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PrintDocument {
    const NAME: &'static str = "Microsoft.UI.Xaml.Printing.PrintDocument";
}
::windows::core::interface_hierarchy!(
    PrintDocument,
    ::windows::core::IUnknown,
    ::windows::core::IInspectable
);
impl ::core::convert::From<PrintDocument> for super::DependencyObject {
    fn from(value: PrintDocument) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&PrintDocument> for super::DependencyObject {
    fn from(value: &PrintDocument) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::core::convert::From<&PrintDocument>
    for ::windows::core::InParam<'a, super::DependencyObject>
{
    fn from(value: &PrintDocument) -> Self {
        ::windows::core::InParam::owned(value.into())
    }
}
unsafe impl ::core::marker::Send for PrintDocument {}
unsafe impl ::core::marker::Sync for PrintDocument {}
#[doc = "*Required features: `\"UI_Xaml_Printing\"`*"]
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
unsafe impl ::windows::core::Abi for PreviewPageCountType {
    type Abi = Self;
}
impl ::core::fmt::Debug for PreviewPageCountType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PreviewPageCountType").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PreviewPageCountType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Xaml.Printing.PreviewPageCountType;i4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_Xaml_Printing\"`*"]
#[repr(transparent)]
pub struct AddPagesEventHandler(pub ::windows::core::IUnknown);
impl AddPagesEventHandler {
    pub fn new<
        F: FnMut(
                &::core::option::Option<::windows::core::IInspectable>,
                &::core::option::Option<AddPagesEventArgs>,
            ) -> ::windows::core::Result<()>
            + ::core::marker::Send
            + 'static,
    >(
        invoke: F,
    ) -> Self {
        let com = AddPagesEventHandlerBox::<F> {
            vtable: &AddPagesEventHandlerBox::<F>::VTABLE,
            count: ::windows::core::RefCount::new(1),
            invoke,
        };
        unsafe { ::core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    pub fn Invoke<'a, P0>(&self, sender: P0, e: &AddPagesEventArgs) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IInspectable>>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).Invoke)(
                ::windows::core::Vtable::as_raw(this),
                sender.into().abi(),
                ::core::mem::transmute_copy(e),
            )
            .ok()
        }
    }
}
#[repr(C)]
struct AddPagesEventHandlerBox<
    F: FnMut(
            &::core::option::Option<::windows::core::IInspectable>,
            &::core::option::Option<AddPagesEventArgs>,
        ) -> ::windows::core::Result<()>
        + ::core::marker::Send
        + 'static,
> {
    vtable: *const AddPagesEventHandler_Vtbl,
    invoke: F,
    count: ::windows::core::RefCount,
}
impl<
        F: FnMut(
                &::core::option::Option<::windows::core::IInspectable>,
                &::core::option::Option<AddPagesEventArgs>,
            ) -> ::windows::core::Result<()>
            + ::core::marker::Send
            + 'static,
    > AddPagesEventHandlerBox<F>
{
    const VTABLE: AddPagesEventHandler_Vtbl = AddPagesEventHandler_Vtbl {
        base__: ::windows::core::IUnknown_Vtbl {
            QueryInterface: Self::QueryInterface,
            AddRef: Self::AddRef,
            Release: Self::Release,
        },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn QueryInterface(
        this: *mut ::core::ffi::c_void,
        iid: &::windows::core::GUID,
        interface: *mut *const ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        *interface = if iid == &<AddPagesEventHandler as ::windows::core::Interface>::IID
            || iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID
            || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID
        {
            &mut (*this).vtable as *mut _ as _
        } else {
            ::core::ptr::null_mut()
        };
        if (*interface).is_null() {
            ::windows::core::HRESULT(-2147467262)
        } else {
            (*this).count.add_ref();
            ::windows::core::HRESULT(0)
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
            let _ = ::windows::core::alloc::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(
        this: *mut ::core::ffi::c_void,
        sender: *mut ::core::ffi::c_void,
        e: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        ((*this).invoke)(::core::mem::transmute(&sender), ::core::mem::transmute(&e)).into()
    }
}
impl ::core::clone::Clone for AddPagesEventHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AddPagesEventHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AddPagesEventHandler {}
impl ::core::fmt::Debug for AddPagesEventHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AddPagesEventHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for AddPagesEventHandler {
    type Vtable = AddPagesEventHandler_Vtbl;
}
unsafe impl ::windows::core::Interface for AddPagesEventHandler {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xed77566a_bd03_5118_b7c3_d9cea64307dd);
}
unsafe impl ::windows::core::RuntimeType for AddPagesEventHandler {
    const SIGNATURE: ::windows::core::ConstBuffer =
        ::windows::core::ConstBuffer::from_slice(b"{ed77566a-bd03-5118-b7c3-d9cea64307dd}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct AddPagesEventHandler_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        sender: *mut ::core::ffi::c_void,
        e: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Xaml_Printing\"`*"]
#[repr(transparent)]
pub struct GetPreviewPageEventHandler(pub ::windows::core::IUnknown);
impl GetPreviewPageEventHandler {
    pub fn new<
        F: FnMut(
                &::core::option::Option<::windows::core::IInspectable>,
                &::core::option::Option<GetPreviewPageEventArgs>,
            ) -> ::windows::core::Result<()>
            + ::core::marker::Send
            + 'static,
    >(
        invoke: F,
    ) -> Self {
        let com = GetPreviewPageEventHandlerBox::<F> {
            vtable: &GetPreviewPageEventHandlerBox::<F>::VTABLE,
            count: ::windows::core::RefCount::new(1),
            invoke,
        };
        unsafe { ::core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    pub fn Invoke<'a, P0>(
        &self,
        sender: P0,
        e: &GetPreviewPageEventArgs,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IInspectable>>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).Invoke)(
                ::windows::core::Vtable::as_raw(this),
                sender.into().abi(),
                ::core::mem::transmute_copy(e),
            )
            .ok()
        }
    }
}
#[repr(C)]
struct GetPreviewPageEventHandlerBox<
    F: FnMut(
            &::core::option::Option<::windows::core::IInspectable>,
            &::core::option::Option<GetPreviewPageEventArgs>,
        ) -> ::windows::core::Result<()>
        + ::core::marker::Send
        + 'static,
> {
    vtable: *const GetPreviewPageEventHandler_Vtbl,
    invoke: F,
    count: ::windows::core::RefCount,
}
impl<
        F: FnMut(
                &::core::option::Option<::windows::core::IInspectable>,
                &::core::option::Option<GetPreviewPageEventArgs>,
            ) -> ::windows::core::Result<()>
            + ::core::marker::Send
            + 'static,
    > GetPreviewPageEventHandlerBox<F>
{
    const VTABLE: GetPreviewPageEventHandler_Vtbl = GetPreviewPageEventHandler_Vtbl {
        base__: ::windows::core::IUnknown_Vtbl {
            QueryInterface: Self::QueryInterface,
            AddRef: Self::AddRef,
            Release: Self::Release,
        },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn QueryInterface(
        this: *mut ::core::ffi::c_void,
        iid: &::windows::core::GUID,
        interface: *mut *const ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        *interface = if iid == &<GetPreviewPageEventHandler as ::windows::core::Interface>::IID
            || iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID
            || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID
        {
            &mut (*this).vtable as *mut _ as _
        } else {
            ::core::ptr::null_mut()
        };
        if (*interface).is_null() {
            ::windows::core::HRESULT(-2147467262)
        } else {
            (*this).count.add_ref();
            ::windows::core::HRESULT(0)
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
            let _ = ::windows::core::alloc::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(
        this: *mut ::core::ffi::c_void,
        sender: *mut ::core::ffi::c_void,
        e: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        ((*this).invoke)(::core::mem::transmute(&sender), ::core::mem::transmute(&e)).into()
    }
}
impl ::core::clone::Clone for GetPreviewPageEventHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for GetPreviewPageEventHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GetPreviewPageEventHandler {}
impl ::core::fmt::Debug for GetPreviewPageEventHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GetPreviewPageEventHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for GetPreviewPageEventHandler {
    type Vtable = GetPreviewPageEventHandler_Vtbl;
}
unsafe impl ::windows::core::Interface for GetPreviewPageEventHandler {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x1c801689_a018_5574_9109_bcef62176da2);
}
unsafe impl ::windows::core::RuntimeType for GetPreviewPageEventHandler {
    const SIGNATURE: ::windows::core::ConstBuffer =
        ::windows::core::ConstBuffer::from_slice(b"{1c801689-a018-5574-9109-bcef62176da2}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct GetPreviewPageEventHandler_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        sender: *mut ::core::ffi::c_void,
        e: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Xaml_Printing\"`*"]
#[repr(transparent)]
pub struct PaginateEventHandler(pub ::windows::core::IUnknown);
impl PaginateEventHandler {
    pub fn new<
        F: FnMut(
                &::core::option::Option<::windows::core::IInspectable>,
                &::core::option::Option<PaginateEventArgs>,
            ) -> ::windows::core::Result<()>
            + ::core::marker::Send
            + 'static,
    >(
        invoke: F,
    ) -> Self {
        let com = PaginateEventHandlerBox::<F> {
            vtable: &PaginateEventHandlerBox::<F>::VTABLE,
            count: ::windows::core::RefCount::new(1),
            invoke,
        };
        unsafe { ::core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    pub fn Invoke<'a, P0>(&self, sender: P0, e: &PaginateEventArgs) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IInspectable>>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).Invoke)(
                ::windows::core::Vtable::as_raw(this),
                sender.into().abi(),
                ::core::mem::transmute_copy(e),
            )
            .ok()
        }
    }
}
#[repr(C)]
struct PaginateEventHandlerBox<
    F: FnMut(
            &::core::option::Option<::windows::core::IInspectable>,
            &::core::option::Option<PaginateEventArgs>,
        ) -> ::windows::core::Result<()>
        + ::core::marker::Send
        + 'static,
> {
    vtable: *const PaginateEventHandler_Vtbl,
    invoke: F,
    count: ::windows::core::RefCount,
}
impl<
        F: FnMut(
                &::core::option::Option<::windows::core::IInspectable>,
                &::core::option::Option<PaginateEventArgs>,
            ) -> ::windows::core::Result<()>
            + ::core::marker::Send
            + 'static,
    > PaginateEventHandlerBox<F>
{
    const VTABLE: PaginateEventHandler_Vtbl = PaginateEventHandler_Vtbl {
        base__: ::windows::core::IUnknown_Vtbl {
            QueryInterface: Self::QueryInterface,
            AddRef: Self::AddRef,
            Release: Self::Release,
        },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn QueryInterface(
        this: *mut ::core::ffi::c_void,
        iid: &::windows::core::GUID,
        interface: *mut *const ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        *interface = if iid == &<PaginateEventHandler as ::windows::core::Interface>::IID
            || iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID
            || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID
        {
            &mut (*this).vtable as *mut _ as _
        } else {
            ::core::ptr::null_mut()
        };
        if (*interface).is_null() {
            ::windows::core::HRESULT(-2147467262)
        } else {
            (*this).count.add_ref();
            ::windows::core::HRESULT(0)
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
            let _ = ::windows::core::alloc::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(
        this: *mut ::core::ffi::c_void,
        sender: *mut ::core::ffi::c_void,
        e: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        ((*this).invoke)(::core::mem::transmute(&sender), ::core::mem::transmute(&e)).into()
    }
}
impl ::core::clone::Clone for PaginateEventHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PaginateEventHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PaginateEventHandler {}
impl ::core::fmt::Debug for PaginateEventHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PaginateEventHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for PaginateEventHandler {
    type Vtable = PaginateEventHandler_Vtbl;
}
unsafe impl ::windows::core::Interface for PaginateEventHandler {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xc291876c_343a_5b9b_a36c_8415ba4cd9dd);
}
unsafe impl ::windows::core::RuntimeType for PaginateEventHandler {
    const SIGNATURE: ::windows::core::ConstBuffer =
        ::windows::core::ConstBuffer::from_slice(b"{c291876c-343a-5b9b-a36c-8415ba4cd9dd}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct PaginateEventHandler_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        sender: *mut ::core::ffi::c_void,
        e: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
