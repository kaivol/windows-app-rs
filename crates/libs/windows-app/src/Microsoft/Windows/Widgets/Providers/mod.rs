#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWidgetActionInvokedArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWidgetActionInvokedArgs {
    type Vtable = IWidgetActionInvokedArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWidgetActionInvokedArgs {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xc593cc57_04b9_52ca_88ad_46fea21ea340);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWidgetActionInvokedArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub WidgetContext: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub Verb: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub Data: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub CustomState: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWidgetAnalyticsInfoReportedArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWidgetAnalyticsInfoReportedArgs {
    type Vtable = IWidgetAnalyticsInfoReportedArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWidgetAnalyticsInfoReportedArgs {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x1d9e5fb5_2bce_5350_87b1_d63199526639);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWidgetAnalyticsInfoReportedArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub WidgetContext: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub AnalyticsJson: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWidgetContext(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWidgetContext {
    type Vtable = IWidgetContext_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWidgetContext {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x903c518b_40bc_5bc6_88f7_af9d81c0cdc1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWidgetContext_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub DefinitionId: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub Size: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::WidgetSize,
    ) -> ::windows_core::HRESULT,
    pub IsActive: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWidgetContextChangedArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWidgetContextChangedArgs {
    type Vtable = IWidgetContextChangedArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWidgetContextChangedArgs {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x2c226d54_2252_576b_a197_370b28d25c2f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWidgetContextChangedArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub WidgetContext: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWidgetCustomizationRequestedArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWidgetCustomizationRequestedArgs {
    type Vtable = IWidgetCustomizationRequestedArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWidgetCustomizationRequestedArgs {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x41dea311_dd9b_5b8b_b493_3a30552116b8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWidgetCustomizationRequestedArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub WidgetContext: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub CustomState: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWidgetErrorInfoReportedArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWidgetErrorInfoReportedArgs {
    type Vtable = IWidgetErrorInfoReportedArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWidgetErrorInfoReportedArgs {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x30efa627_b21f_55d5_b91a_b23b4aa13645);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWidgetErrorInfoReportedArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub WidgetContext: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub ErrorJson: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWidgetInfo(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWidgetInfo {
    type Vtable = IWidgetInfo_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWidgetInfo {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xcea11f42_a020_5db5_89e2_b7dece4ae5cb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWidgetInfo_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub WidgetContext: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub Template: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub Data: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub CustomState: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_Foundation")]
    pub LastUpdateTime: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::DateTime,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    LastUpdateTime: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWidgetManager(::windows_core::IUnknown);
impl IWidgetManager {
    pub fn UpdateWidget<P0>(&self, widgetupdaterequestoptions: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<WidgetUpdateRequestOptions>,
    {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).UpdateWidget)(
                ::windows_core::Interface::as_raw(this),
                widgetupdaterequestoptions.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn GetWidgetIds(
        &self,
    ) -> ::windows_core::Result<::windows_core::Array<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows_core::Interface::vtable(this).GetWidgetIds)(
                ::windows_core::Interface::as_raw(this),
                ::windows_core::Array::<::windows_core::HSTRING>::set_abi_len(
                    ::std::mem::transmute(&mut result__),
                ),
                result__.as_mut_ptr() as *mut _ as _,
            )
            .and_then(|| result__.assume_init())
        }
    }
    pub fn GetWidgetInfo(
        &self,
        widgetid: &::windows_core::HSTRING,
    ) -> ::windows_core::Result<WidgetInfo> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetWidgetInfo)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(widgetid),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn GetWidgetInfos(&self) -> ::windows_core::Result<::windows_core::Array<WidgetInfo>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows_core::Interface::vtable(this).GetWidgetInfos)(
                ::windows_core::Interface::as_raw(this),
                ::windows_core::Array::<WidgetInfo>::set_abi_len(::std::mem::transmute(
                    &mut result__,
                )),
                result__.as_mut_ptr() as *mut _ as _,
            )
            .and_then(|| result__.assume_init())
        }
    }
    pub fn DeleteWidget(&self, widgetid: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).DeleteWidget)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(widgetid),
            )
            .ok()
        }
    }
}
::windows_core::imp::interface_hierarchy!(
    IWidgetManager,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::RuntimeType for IWidgetManager {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_interface::<Self>();
}
unsafe impl ::windows_core::Interface for IWidgetManager {
    type Vtable = IWidgetManager_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWidgetManager {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x71cb10c0_671e_48e3_b995_207940397123);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWidgetManager_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub UpdateWidget: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        widgetupdaterequestoptions: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub GetWidgetIds: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result_size__: *mut u32,
        result__: *mut *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub GetWidgetInfo: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        widgetid: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub GetWidgetInfos: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result_size__: *mut u32,
        result__: *mut *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub DeleteWidget: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        widgetid: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWidgetManagerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWidgetManagerStatics {
    type Vtable = IWidgetManagerStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWidgetManagerStatics {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x7f233b06_28e5_5e2b_8c04_a4fa747c28c7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWidgetManagerStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub GetDefault: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWidgetProvider(::windows_core::IUnknown);
impl IWidgetProvider {
    pub fn CreateWidget<P0>(&self, widgetcontext: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<WidgetContext>,
    {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).CreateWidget)(
                ::windows_core::Interface::as_raw(this),
                widgetcontext.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn DeleteWidget(
        &self,
        widgetid: &::windows_core::HSTRING,
        customstate: &::windows_core::HSTRING,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).DeleteWidget)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(widgetid),
                ::core::mem::transmute_copy(customstate),
            )
            .ok()
        }
    }
    pub fn OnActionInvoked<P0>(&self, actioninvokedargs: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<WidgetActionInvokedArgs>,
    {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).OnActionInvoked)(
                ::windows_core::Interface::as_raw(this),
                actioninvokedargs.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn OnWidgetContextChanged<P0>(&self, contextchangedargs: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<WidgetContextChangedArgs>,
    {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).OnWidgetContextChanged)(
                ::windows_core::Interface::as_raw(this),
                contextchangedargs.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn Activate<P0>(&self, widgetcontext: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<WidgetContext>,
    {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).Activate)(
                ::windows_core::Interface::as_raw(this),
                widgetcontext.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn Deactivate(&self, widgetid: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).Deactivate)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(widgetid),
            )
            .ok()
        }
    }
}
::windows_core::imp::interface_hierarchy!(
    IWidgetProvider,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::RuntimeType for IWidgetProvider {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_interface::<Self>();
}
unsafe impl ::windows_core::Interface for IWidgetProvider {
    type Vtable = IWidgetProvider_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWidgetProvider {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x5c5774cc_72a0_452d_b9ed_075c0dd25eed);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWidgetProvider_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateWidget: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        widgetcontext: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub DeleteWidget: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        widgetid: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        customstate: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub OnActionInvoked: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        actioninvokedargs: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub OnWidgetContextChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        contextchangedargs: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub Activate: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        widgetcontext: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub Deactivate: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        widgetid: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWidgetProvider2(::windows_core::IUnknown);
impl IWidgetProvider2 {
    pub fn OnCustomizationRequested<P0>(
        &self,
        customizationrequestedargs: P0,
    ) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<WidgetCustomizationRequestedArgs>,
    {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).OnCustomizationRequested)(
                ::windows_core::Interface::as_raw(this),
                customizationrequestedargs.into_param().abi(),
            )
            .ok()
        }
    }
}
::windows_core::imp::interface_hierarchy!(
    IWidgetProvider2,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::RuntimeType for IWidgetProvider2 {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_interface::<Self>();
}
unsafe impl ::windows_core::Interface for IWidgetProvider2 {
    type Vtable = IWidgetProvider2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWidgetProvider2 {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x38c3a963_dd93_479d_9276_04bf84ee1816);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWidgetProvider2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub OnCustomizationRequested: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        customizationrequestedargs: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWidgetProviderAnalytics(::windows_core::IUnknown);
impl IWidgetProviderAnalytics {
    pub fn OnAnalyticsInfoReported<P0>(&self, args: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<WidgetAnalyticsInfoReportedArgs>,
    {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).OnAnalyticsInfoReported)(
                ::windows_core::Interface::as_raw(this),
                args.into_param().abi(),
            )
            .ok()
        }
    }
}
::windows_core::imp::interface_hierarchy!(
    IWidgetProviderAnalytics,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::RuntimeType for IWidgetProviderAnalytics {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_interface::<Self>();
}
unsafe impl ::windows_core::Interface for IWidgetProviderAnalytics {
    type Vtable = IWidgetProviderAnalytics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWidgetProviderAnalytics {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x661985a5_d187_482d_9eef_6fda05d21845);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWidgetProviderAnalytics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub OnAnalyticsInfoReported: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        args: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWidgetProviderErrors(::windows_core::IUnknown);
impl IWidgetProviderErrors {
    pub fn OnErrorInfoReported<P0>(&self, args: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<WidgetErrorInfoReportedArgs>,
    {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).OnErrorInfoReported)(
                ::windows_core::Interface::as_raw(this),
                args.into_param().abi(),
            )
            .ok()
        }
    }
}
::windows_core::imp::interface_hierarchy!(
    IWidgetProviderErrors,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::RuntimeType for IWidgetProviderErrors {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_interface::<Self>();
}
unsafe impl ::windows_core::Interface for IWidgetProviderErrors {
    type Vtable = IWidgetProviderErrors_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWidgetProviderErrors {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x90c1b5f0_0d3a_4ac6_abb7_c97b367b8fcc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWidgetProviderErrors_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub OnErrorInfoReported: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        args: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWidgetUpdateRequestOptions(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWidgetUpdateRequestOptions {
    type Vtable = IWidgetUpdateRequestOptions_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWidgetUpdateRequestOptions {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xb09ca8f7_7424_5687_baaf_7dd6fa639672);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWidgetUpdateRequestOptions_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub WidgetId: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub Template: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub SetTemplate: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub Data: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub SetData: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub CustomState: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub SetCustomState: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWidgetUpdateRequestOptionsFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWidgetUpdateRequestOptionsFactory {
    type Vtable = IWidgetUpdateRequestOptionsFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWidgetUpdateRequestOptionsFactory {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xe0e00af8_1d10_57a8_9419_3f568e854daa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWidgetUpdateRequestOptionsFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        widgetid: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWidgetUpdateRequestOptionsStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWidgetUpdateRequestOptionsStatics {
    type Vtable = IWidgetUpdateRequestOptionsStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWidgetUpdateRequestOptionsStatics {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x4645b5e3_d332_5d11_82f0_3607e5df6018);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWidgetUpdateRequestOptionsStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub UnsetValue: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct WidgetActionInvokedArgs(::windows_core::IUnknown);
impl WidgetActionInvokedArgs {
    pub fn WidgetContext(&self) -> ::windows_core::Result<WidgetContext> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).WidgetContext)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn Verb(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Verb)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn Data(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Data)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn CustomState(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CustomState)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for WidgetActionInvokedArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for WidgetActionInvokedArgs {
    type Vtable = IWidgetActionInvokedArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for WidgetActionInvokedArgs {
    const IID: ::windows_core::GUID =
        <IWidgetActionInvokedArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for WidgetActionInvokedArgs {
    const NAME: &'static str = "Microsoft.Windows.Widgets.Providers.WidgetActionInvokedArgs";
}
::windows_core::imp::interface_hierarchy!(
    WidgetActionInvokedArgs,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for WidgetActionInvokedArgs {}
unsafe impl ::core::marker::Sync for WidgetActionInvokedArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct WidgetAnalyticsInfoReportedArgs(::windows_core::IUnknown);
impl WidgetAnalyticsInfoReportedArgs {
    pub fn WidgetContext(&self) -> ::windows_core::Result<WidgetContext> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).WidgetContext)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn AnalyticsJson(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AnalyticsJson)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for WidgetAnalyticsInfoReportedArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for WidgetAnalyticsInfoReportedArgs {
    type Vtable = IWidgetAnalyticsInfoReportedArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for WidgetAnalyticsInfoReportedArgs {
    const IID: ::windows_core::GUID =
        <IWidgetAnalyticsInfoReportedArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for WidgetAnalyticsInfoReportedArgs {
    const NAME: &'static str =
        "Microsoft.Windows.Widgets.Providers.WidgetAnalyticsInfoReportedArgs";
}
::windows_core::imp::interface_hierarchy!(
    WidgetAnalyticsInfoReportedArgs,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for WidgetAnalyticsInfoReportedArgs {}
unsafe impl ::core::marker::Sync for WidgetAnalyticsInfoReportedArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct WidgetContext(::windows_core::IUnknown);
impl WidgetContext {
    pub fn Id(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Id)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn DefinitionId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DefinitionId)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn Size(&self) -> ::windows_core::Result<super::WidgetSize> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Size)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn IsActive(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsActive)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for WidgetContext {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for WidgetContext {
    type Vtable = IWidgetContext_Vtbl;
}
unsafe impl ::windows_core::ComInterface for WidgetContext {
    const IID: ::windows_core::GUID = <IWidgetContext as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for WidgetContext {
    const NAME: &'static str = "Microsoft.Windows.Widgets.Providers.WidgetContext";
}
::windows_core::imp::interface_hierarchy!(
    WidgetContext,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for WidgetContext {}
unsafe impl ::core::marker::Sync for WidgetContext {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct WidgetContextChangedArgs(::windows_core::IUnknown);
impl WidgetContextChangedArgs {
    pub fn WidgetContext(&self) -> ::windows_core::Result<WidgetContext> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).WidgetContext)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for WidgetContextChangedArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for WidgetContextChangedArgs {
    type Vtable = IWidgetContextChangedArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for WidgetContextChangedArgs {
    const IID: ::windows_core::GUID =
        <IWidgetContextChangedArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for WidgetContextChangedArgs {
    const NAME: &'static str = "Microsoft.Windows.Widgets.Providers.WidgetContextChangedArgs";
}
::windows_core::imp::interface_hierarchy!(
    WidgetContextChangedArgs,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for WidgetContextChangedArgs {}
unsafe impl ::core::marker::Sync for WidgetContextChangedArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct WidgetCustomizationRequestedArgs(::windows_core::IUnknown);
impl WidgetCustomizationRequestedArgs {
    pub fn WidgetContext(&self) -> ::windows_core::Result<WidgetContext> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).WidgetContext)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn CustomState(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CustomState)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for WidgetCustomizationRequestedArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for WidgetCustomizationRequestedArgs {
    type Vtable = IWidgetCustomizationRequestedArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for WidgetCustomizationRequestedArgs {
    const IID: ::windows_core::GUID =
        <IWidgetCustomizationRequestedArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for WidgetCustomizationRequestedArgs {
    const NAME: &'static str =
        "Microsoft.Windows.Widgets.Providers.WidgetCustomizationRequestedArgs";
}
::windows_core::imp::interface_hierarchy!(
    WidgetCustomizationRequestedArgs,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for WidgetCustomizationRequestedArgs {}
unsafe impl ::core::marker::Sync for WidgetCustomizationRequestedArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct WidgetErrorInfoReportedArgs(::windows_core::IUnknown);
impl WidgetErrorInfoReportedArgs {
    pub fn WidgetContext(&self) -> ::windows_core::Result<WidgetContext> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).WidgetContext)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn ErrorJson(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ErrorJson)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for WidgetErrorInfoReportedArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for WidgetErrorInfoReportedArgs {
    type Vtable = IWidgetErrorInfoReportedArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for WidgetErrorInfoReportedArgs {
    const IID: ::windows_core::GUID =
        <IWidgetErrorInfoReportedArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for WidgetErrorInfoReportedArgs {
    const NAME: &'static str = "Microsoft.Windows.Widgets.Providers.WidgetErrorInfoReportedArgs";
}
::windows_core::imp::interface_hierarchy!(
    WidgetErrorInfoReportedArgs,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for WidgetErrorInfoReportedArgs {}
unsafe impl ::core::marker::Sync for WidgetErrorInfoReportedArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct WidgetInfo(::windows_core::IUnknown);
impl WidgetInfo {
    pub fn WidgetContext(&self) -> ::windows_core::Result<WidgetContext> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).WidgetContext)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn Template(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Template)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn Data(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Data)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn CustomState(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CustomState)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn LastUpdateTime(&self) -> ::windows_core::Result<::windows::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LastUpdateTime)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for WidgetInfo {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for WidgetInfo {
    type Vtable = IWidgetInfo_Vtbl;
}
unsafe impl ::windows_core::ComInterface for WidgetInfo {
    const IID: ::windows_core::GUID = <IWidgetInfo as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for WidgetInfo {
    const NAME: &'static str = "Microsoft.Windows.Widgets.Providers.WidgetInfo";
}
::windows_core::imp::interface_hierarchy!(
    WidgetInfo,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for WidgetInfo {}
unsafe impl ::core::marker::Sync for WidgetInfo {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct WidgetManager(::windows_core::IUnknown);
impl WidgetManager {
    pub fn UpdateWidget<P0>(&self, widgetupdaterequestoptions: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<WidgetUpdateRequestOptions>,
    {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).UpdateWidget)(
                ::windows_core::Interface::as_raw(this),
                widgetupdaterequestoptions.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn GetWidgetIds(
        &self,
    ) -> ::windows_core::Result<::windows_core::Array<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows_core::Interface::vtable(this).GetWidgetIds)(
                ::windows_core::Interface::as_raw(this),
                ::windows_core::Array::<::windows_core::HSTRING>::set_abi_len(
                    ::std::mem::transmute(&mut result__),
                ),
                result__.as_mut_ptr() as *mut _ as _,
            )
            .and_then(|| result__.assume_init())
        }
    }
    pub fn GetWidgetInfo(
        &self,
        widgetid: &::windows_core::HSTRING,
    ) -> ::windows_core::Result<WidgetInfo> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetWidgetInfo)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(widgetid),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn GetWidgetInfos(&self) -> ::windows_core::Result<::windows_core::Array<WidgetInfo>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows_core::Interface::vtable(this).GetWidgetInfos)(
                ::windows_core::Interface::as_raw(this),
                ::windows_core::Array::<WidgetInfo>::set_abi_len(::std::mem::transmute(
                    &mut result__,
                )),
                result__.as_mut_ptr() as *mut _ as _,
            )
            .and_then(|| result__.assume_init())
        }
    }
    pub fn DeleteWidget(&self, widgetid: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).DeleteWidget)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(widgetid),
            )
            .ok()
        }
    }
    pub fn GetDefault() -> ::windows_core::Result<WidgetManager> {
        Self::IWidgetManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetDefault)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IWidgetManagerStatics<
        R,
        F: FnOnce(&IWidgetManagerStatics) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<WidgetManager, IWidgetManagerStatics> =
            ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for WidgetManager {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for WidgetManager {
    type Vtable = IWidgetManager_Vtbl;
}
unsafe impl ::windows_core::ComInterface for WidgetManager {
    const IID: ::windows_core::GUID = <IWidgetManager as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for WidgetManager {
    const NAME: &'static str = "Microsoft.Windows.Widgets.Providers.WidgetManager";
}
::windows_core::imp::interface_hierarchy!(
    WidgetManager,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<IWidgetManager> for WidgetManager {}
unsafe impl ::core::marker::Send for WidgetManager {}
unsafe impl ::core::marker::Sync for WidgetManager {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct WidgetUpdateRequestOptions(::windows_core::IUnknown);
impl WidgetUpdateRequestOptions {
    pub fn WidgetId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).WidgetId)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn Template(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Template)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetTemplate(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetTemplate)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn Data(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Data)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetData(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetData)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn CustomState(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CustomState)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetCustomState(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetCustomState)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn CreateInstance(
        widgetid: &::windows_core::HSTRING,
    ) -> ::windows_core::Result<WidgetUpdateRequestOptions> {
        Self::IWidgetUpdateRequestOptionsFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstance)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(widgetid),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn UnsetValue() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IWidgetUpdateRequestOptionsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).UnsetValue)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IWidgetUpdateRequestOptionsFactory<
        R,
        F: FnOnce(&IWidgetUpdateRequestOptionsFactory) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<
            WidgetUpdateRequestOptions,
            IWidgetUpdateRequestOptionsFactory,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IWidgetUpdateRequestOptionsStatics<
        R,
        F: FnOnce(&IWidgetUpdateRequestOptionsStatics) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<
            WidgetUpdateRequestOptions,
            IWidgetUpdateRequestOptionsStatics,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for WidgetUpdateRequestOptions {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for WidgetUpdateRequestOptions {
    type Vtable = IWidgetUpdateRequestOptions_Vtbl;
}
unsafe impl ::windows_core::ComInterface for WidgetUpdateRequestOptions {
    const IID: ::windows_core::GUID =
        <IWidgetUpdateRequestOptions as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for WidgetUpdateRequestOptions {
    const NAME: &'static str = "Microsoft.Windows.Widgets.Providers.WidgetUpdateRequestOptions";
}
::windows_core::imp::interface_hierarchy!(
    WidgetUpdateRequestOptions,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for WidgetUpdateRequestOptions {}
unsafe impl ::core::marker::Sync for WidgetUpdateRequestOptions {}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
