#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IComponentConnector(::windows_core::IUnknown);
impl IComponentConnector {
    pub fn Connect<P0>(&self, connectionid: i32, target: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).Connect)(
                ::windows_core::Interface::as_raw(this),
                connectionid,
                target.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn GetBindingConnector<P0>(
        &self,
        connectionid: i32,
        target: P0,
    ) -> ::windows_core::Result<IComponentConnector>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetBindingConnector)(
                ::windows_core::Interface::as_raw(this),
                connectionid,
                target.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(
    IComponentConnector,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::RuntimeType for IComponentConnector {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_interface::<Self>();
}
unsafe impl ::windows_core::Interface for IComponentConnector {
    type Vtable = IComponentConnector_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IComponentConnector {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xad401812_b091_51d0_b915_2d682cd2af10);
}
#[repr(C)]
#[doc(hidden)]
pub struct IComponentConnector_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Connect: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        connectionid: i32,
        target: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub GetBindingConnector: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        connectionid: i32,
        target: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDataTemplateComponent(::windows_core::IUnknown);
impl IDataTemplateComponent {
    pub fn Recycle(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).Recycle)(::windows_core::Interface::as_raw(
                this,
            ))
            .ok()
        }
    }
    pub fn ProcessBindings<P0>(
        &self,
        item: P0,
        itemindex: i32,
        phase: i32,
        nextphase: &mut i32,
    ) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).ProcessBindings)(
                ::windows_core::Interface::as_raw(this),
                item.into_param().abi(),
                itemindex,
                phase,
                nextphase,
            )
            .ok()
        }
    }
}
::windows_core::imp::interface_hierarchy!(
    IDataTemplateComponent,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::RuntimeType for IDataTemplateComponent {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_interface::<Self>();
}
unsafe impl ::windows_core::Interface for IDataTemplateComponent {
    type Vtable = IDataTemplateComponent_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDataTemplateComponent {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x1743ddf7_38ba_58c9_a2a6_b0ae28713bee);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataTemplateComponent_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Recycle:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub ProcessBindings: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        item: *mut ::core::ffi::c_void,
        itemindex: i32,
        phase: i32,
        nextphase: *mut i32,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMarkupExtension(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMarkupExtension {
    type Vtable = IMarkupExtension_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMarkupExtension {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xc355371e_091d_5136_af4a_baf5e00616bd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMarkupExtension_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMarkupExtensionFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMarkupExtensionFactory {
    type Vtable = IMarkupExtensionFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMarkupExtensionFactory {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x20651afa_5f3a_5f0c_adb1_b6551f53a6a0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMarkupExtensionFactory_Vtbl {
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
pub struct IMarkupExtensionOverrides(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMarkupExtensionOverrides {
    type Vtable = IMarkupExtensionOverrides_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMarkupExtensionOverrides {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xa12aa575_5d31_5b68_a30f_8495412a351d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMarkupExtensionOverrides_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ProvideValue: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub ProvideValueWithIXamlServiceProvider: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        serviceprovider: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    )
        -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IProvideValueTarget(::windows_core::IUnknown);
impl IProvideValueTarget {
    pub fn TargetObject(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TargetObject)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn TargetProperty(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TargetProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(
    IProvideValueTarget,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::RuntimeType for IProvideValueTarget {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_interface::<Self>();
}
unsafe impl ::windows_core::Interface for IProvideValueTarget {
    type Vtable = IProvideValueTarget_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IProvideValueTarget {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x3f01ff68_3efd_591d_a506_de13fcaabd83);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProvideValueTarget_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub TargetObject: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub TargetProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IProvideValueTargetProperty(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IProvideValueTargetProperty {
    type Vtable = IProvideValueTargetProperty_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IProvideValueTargetProperty {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xce777b1f_b42e_59d1_870d_12fdf0629133);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProvideValueTargetProperty_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Name: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_UI_Xaml_Interop")]
    pub Type: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<
            super::super::super::super::Windows::UI::Xaml::Interop::TypeName,
        >,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI_Xaml_Interop"))]
    Type: usize,
    #[cfg(feature = "Windows_UI_Xaml_Interop")]
    pub DeclaringType: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<
            super::super::super::super::Windows::UI::Xaml::Interop::TypeName,
        >,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI_Xaml_Interop"))]
    DeclaringType: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IRootObjectProvider(::windows_core::IUnknown);
impl IRootObjectProvider {
    pub fn RootObject(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RootObject)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(
    IRootObjectProvider,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::RuntimeType for IRootObjectProvider {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_interface::<Self>();
}
unsafe impl ::windows_core::Interface for IRootObjectProvider {
    type Vtable = IRootObjectProvider_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IRootObjectProvider {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x13d63599_352f_5eb8_81c1_bc62fb12d6da);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRootObjectProvider_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub RootObject: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IUriContext(::windows_core::IUnknown);
impl IUriContext {
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn BaseUri(&self) -> ::windows_core::Result<::windows::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).BaseUri)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(
    IUriContext,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::RuntimeType for IUriContext {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_interface::<Self>();
}
unsafe impl ::windows_core::Interface for IUriContext {
    type Vtable = IUriContext_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IUriContext {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xfb8605f6_8f05_52ee_a01c_3a9e118a6ea2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUriContext_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Windows_Foundation")]
    pub BaseUri: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    BaseUri: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IXamlBinaryWriter(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXamlBinaryWriter {
    type Vtable = IXamlBinaryWriter_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IXamlBinaryWriter {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x8fb45e3b_e689_55bf_aa11_d83b1c1cdda1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlBinaryWriter_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IXamlBinaryWriterStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXamlBinaryWriterStatics {
    type Vtable = IXamlBinaryWriterStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IXamlBinaryWriterStatics {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x774907fc_c846_517f_abcc_c3f7e8c3ffc9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlBinaryWriterStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(all(
        feature = "Windows_Foundation_Collections",
        feature = "Windows_Storage_Streams"
    ))]
    pub Write: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        inputstreams: *mut ::core::ffi::c_void,
        outputstreams: *mut ::core::ffi::c_void,
        xamlmetadataprovider: *mut ::core::ffi::c_void,
        result__: *mut XamlBinaryWriterErrorInformation,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(all(
        feature = "Windows_Foundation_Collections",
        feature = "Windows_Storage_Streams"
    )))]
    Write: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IXamlBindScopeDiagnostics(::windows_core::IUnknown);
impl IXamlBindScopeDiagnostics {
    pub fn Disable(&self, linenumber: i32, columnnumber: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).Disable)(
                ::windows_core::Interface::as_raw(this),
                linenumber,
                columnnumber,
            )
            .ok()
        }
    }
}
::windows_core::imp::interface_hierarchy!(
    IXamlBindScopeDiagnostics,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::RuntimeType for IXamlBindScopeDiagnostics {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_interface::<Self>();
}
unsafe impl ::windows_core::Interface for IXamlBindScopeDiagnostics {
    type Vtable = IXamlBindScopeDiagnostics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IXamlBindScopeDiagnostics {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x3ea84e4e_fdfe_55a8_a561_edf5697846d7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlBindScopeDiagnostics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Disable: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        linenumber: i32,
        columnnumber: i32,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IXamlBindingHelper(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXamlBindingHelper {
    type Vtable = IXamlBindingHelper_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IXamlBindingHelper {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x607a9bf2_5a6d_5c89_a756_bb44f24f28f8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlBindingHelper_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IXamlBindingHelperStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXamlBindingHelperStatics {
    type Vtable = IXamlBindingHelperStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IXamlBindingHelperStatics {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x93c7dad3_f9c2_5372_84dc_9e9c4661d083);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlBindingHelperStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub DataTemplateComponentProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub GetDataTemplateComponent: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetDataTemplateComponent: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SuspendRendering: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        target: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub ResumeRendering: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        target: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_UI_Xaml_Interop")]
    pub ConvertValue: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        r#type: ::std::mem::MaybeUninit<
            super::super::super::super::Windows::UI::Xaml::Interop::TypeName,
        >,
        value: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI_Xaml_Interop"))]
    ConvertValue: usize,
    pub SetPropertyFromString: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        dependencyobject: *mut ::core::ffi::c_void,
        propertytoset: *mut ::core::ffi::c_void,
        value: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub SetPropertyFromBoolean: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        dependencyobject: *mut ::core::ffi::c_void,
        propertytoset: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
    pub SetPropertyFromChar16: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        dependencyobject: *mut ::core::ffi::c_void,
        propertytoset: *mut ::core::ffi::c_void,
        value: u16,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_Foundation")]
    pub SetPropertyFromDateTime: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        dependencyobject: *mut ::core::ffi::c_void,
        propertytoset: *mut ::core::ffi::c_void,
        value: ::windows::Foundation::DateTime,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    SetPropertyFromDateTime: usize,
    pub SetPropertyFromDouble: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        dependencyobject: *mut ::core::ffi::c_void,
        propertytoset: *mut ::core::ffi::c_void,
        value: f64,
    ) -> ::windows_core::HRESULT,
    pub SetPropertyFromInt32: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        dependencyobject: *mut ::core::ffi::c_void,
        propertytoset: *mut ::core::ffi::c_void,
        value: i32,
    ) -> ::windows_core::HRESULT,
    pub SetPropertyFromUInt32: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        dependencyobject: *mut ::core::ffi::c_void,
        propertytoset: *mut ::core::ffi::c_void,
        value: u32,
    ) -> ::windows_core::HRESULT,
    pub SetPropertyFromInt64: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        dependencyobject: *mut ::core::ffi::c_void,
        propertytoset: *mut ::core::ffi::c_void,
        value: i64,
    ) -> ::windows_core::HRESULT,
    pub SetPropertyFromUInt64: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        dependencyobject: *mut ::core::ffi::c_void,
        propertytoset: *mut ::core::ffi::c_void,
        value: u64,
    ) -> ::windows_core::HRESULT,
    pub SetPropertyFromSingle: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        dependencyobject: *mut ::core::ffi::c_void,
        propertytoset: *mut ::core::ffi::c_void,
        value: f32,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_Foundation")]
    pub SetPropertyFromPoint: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        dependencyobject: *mut ::core::ffi::c_void,
        propertytoset: *mut ::core::ffi::c_void,
        value: ::windows::Foundation::Point,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    SetPropertyFromPoint: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub SetPropertyFromRect: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        dependencyobject: *mut ::core::ffi::c_void,
        propertytoset: *mut ::core::ffi::c_void,
        value: ::windows::Foundation::Rect,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    SetPropertyFromRect: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub SetPropertyFromSize: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        dependencyobject: *mut ::core::ffi::c_void,
        propertytoset: *mut ::core::ffi::c_void,
        value: ::windows::Foundation::Size,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    SetPropertyFromSize: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub SetPropertyFromTimeSpan: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        dependencyobject: *mut ::core::ffi::c_void,
        propertytoset: *mut ::core::ffi::c_void,
        value: ::windows::Foundation::TimeSpan,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    SetPropertyFromTimeSpan: usize,
    pub SetPropertyFromByte: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        dependencyobject: *mut ::core::ffi::c_void,
        propertytoset: *mut ::core::ffi::c_void,
        value: u8,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_Foundation")]
    pub SetPropertyFromUri: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        dependencyobject: *mut ::core::ffi::c_void,
        propertytoset: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    SetPropertyFromUri: usize,
    pub SetPropertyFromObject: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        dependencyobject: *mut ::core::ffi::c_void,
        propertytoset: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IXamlMarkupHelper(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXamlMarkupHelper {
    type Vtable = IXamlMarkupHelper_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IXamlMarkupHelper {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xcd677310_3b06_5a13_b31a_401849570858);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlMarkupHelper_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IXamlMarkupHelperStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXamlMarkupHelperStatics {
    type Vtable = IXamlMarkupHelperStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IXamlMarkupHelperStatics {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xd9a0f6e3_c6cc_5cb6_8999_85788701f339);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlMarkupHelperStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub UnloadObject: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IXamlMember(::windows_core::IUnknown);
impl IXamlMember {
    pub fn IsAttachable(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsAttachable)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn IsDependencyProperty(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsDependencyProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn IsReadOnly(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsReadOnly)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn Name(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Name)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn TargetType(&self) -> ::windows_core::Result<IXamlType> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TargetType)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn Type(&self) -> ::windows_core::Result<IXamlType> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Type)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn GetValue<P0>(&self, instance: P0) -> ::windows_core::Result<::windows_core::IInspectable>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetValue)(
                ::windows_core::Interface::as_raw(this),
                instance.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetValue<P0, P1>(&self, instance: P0, value: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
        P1: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetValue)(
                ::windows_core::Interface::as_raw(this),
                instance.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        }
    }
}
::windows_core::imp::interface_hierarchy!(
    IXamlMember,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::RuntimeType for IXamlMember {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_interface::<Self>();
}
unsafe impl ::windows_core::Interface for IXamlMember {
    type Vtable = IXamlMember_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IXamlMember {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xbf3a2913_5c63_50ec_8660_61809be7b9b9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlMember_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub IsAttachable: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub IsDependencyProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub IsReadOnly: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub Name: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub TargetType: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub Type: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub GetValue: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        instance: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetValue: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        instance: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IXamlMetadataProvider(::windows_core::IUnknown);
impl IXamlMetadataProvider {
    #[doc = "Required features: `\"Windows_UI_Xaml_Interop\"`"]
    #[cfg(feature = "Windows_UI_Xaml_Interop")]
    pub fn GetXamlType<P0>(&self, r#type: P0) -> ::windows_core::Result<IXamlType>
    where
        P0: ::windows_core::IntoParam<
            super::super::super::super::Windows::UI::Xaml::Interop::TypeName,
        >,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetXamlType)(
                ::windows_core::Interface::as_raw(this),
                r#type.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn GetXamlTypeByFullName(
        &self,
        fullname: &::windows_core::HSTRING,
    ) -> ::windows_core::Result<IXamlType> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetXamlTypeByFullName)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(fullname),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn GetXmlnsDefinitions(
        &self,
    ) -> ::windows_core::Result<::windows_core::Array<XmlnsDefinition>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows_core::Interface::vtable(this).GetXmlnsDefinitions)(
                ::windows_core::Interface::as_raw(this),
                ::windows_core::Array::<XmlnsDefinition>::set_abi_len(::std::mem::transmute(
                    &mut result__,
                )),
                result__.as_mut_ptr() as *mut _ as _,
            )
            .and_then(|| result__.assume_init())
        }
    }
}
::windows_core::imp::interface_hierarchy!(
    IXamlMetadataProvider,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::RuntimeType for IXamlMetadataProvider {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_interface::<Self>();
}
unsafe impl ::windows_core::Interface for IXamlMetadataProvider {
    type Vtable = IXamlMetadataProvider_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IXamlMetadataProvider {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xa96251f0_2214_5d53_8746_ce99a2593cd7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlMetadataProvider_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Windows_UI_Xaml_Interop")]
    pub GetXamlType: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        r#type: ::std::mem::MaybeUninit<
            super::super::super::super::Windows::UI::Xaml::Interop::TypeName,
        >,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI_Xaml_Interop"))]
    GetXamlType: usize,
    pub GetXamlTypeByFullName: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        fullname: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub GetXmlnsDefinitions: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result_size__: *mut u32,
        result__: *mut *mut ::std::mem::MaybeUninit<XmlnsDefinition>,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IXamlReader(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXamlReader {
    type Vtable = IXamlReader_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IXamlReader {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x54ce54c8_38c6_50d9_ac98_4b03eddbde9f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlReader_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IXamlReaderStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXamlReaderStatics {
    type Vtable = IXamlReaderStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IXamlReaderStatics {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x82a4cd9e_435e_5aeb_8c4f_300cece45cae);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlReaderStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Load: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        xaml: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub LoadWithInitialTemplateValidation: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        xaml: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        result__: *mut *mut ::core::ffi::c_void,
    )
        -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IXamlType(::windows_core::IUnknown);
impl IXamlType {
    pub fn BaseType(&self) -> ::windows_core::Result<IXamlType> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).BaseType)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn ContentProperty(&self) -> ::windows_core::Result<IXamlMember> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ContentProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn FullName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FullName)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn IsArray(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsArray)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn IsCollection(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsCollection)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn IsConstructible(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsConstructible)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn IsDictionary(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsDictionary)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn IsMarkupExtension(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsMarkupExtension)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn IsBindable(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsBindable)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn ItemType(&self) -> ::windows_core::Result<IXamlType> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ItemType)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn KeyType(&self) -> ::windows_core::Result<IXamlType> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).KeyType)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn BoxedType(&self) -> ::windows_core::Result<IXamlType> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).BoxedType)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_UI_Xaml_Interop\"`"]
    #[cfg(feature = "Windows_UI_Xaml_Interop")]
    pub fn UnderlyingType(
        &self,
    ) -> ::windows_core::Result<super::super::super::super::Windows::UI::Xaml::Interop::TypeName>
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).UnderlyingType)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn ActivateInstance(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ActivateInstance)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn CreateFromString(
        &self,
        value: &::windows_core::HSTRING,
    ) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromString)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(value),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn GetMember(&self, name: &::windows_core::HSTRING) -> ::windows_core::Result<IXamlMember> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetMember)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(name),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn AddToVector<P0, P1>(&self, instance: P0, value: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
        P1: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).AddToVector)(
                ::windows_core::Interface::as_raw(this),
                instance.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn AddToMap<P0, P1, P2>(
        &self,
        instance: P0,
        key: P1,
        value: P2,
    ) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
        P1: ::windows_core::IntoParam<::windows_core::IInspectable>,
        P2: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).AddToMap)(
                ::windows_core::Interface::as_raw(this),
                instance.into_param().abi(),
                key.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn RunInitializer(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).RunInitializer)(
                ::windows_core::Interface::as_raw(this),
            )
            .ok()
        }
    }
}
::windows_core::imp::interface_hierarchy!(
    IXamlType,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::RuntimeType for IXamlType {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_interface::<Self>();
}
unsafe impl ::windows_core::Interface for IXamlType {
    type Vtable = IXamlType_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IXamlType {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xd24219df_7ec9_57f1_a27b_6af251d9c5bc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlType_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub BaseType: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub ContentProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub FullName: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub IsArray: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub IsCollection: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub IsConstructible: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub IsDictionary: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub IsMarkupExtension: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub IsBindable: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub ItemType: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub KeyType: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub BoxedType: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_UI_Xaml_Interop")]
    pub UnderlyingType: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<
            super::super::super::super::Windows::UI::Xaml::Interop::TypeName,
        >,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI_Xaml_Interop"))]
    UnderlyingType: usize,
    pub ActivateInstance: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub CreateFromString: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub GetMember: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        name: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub AddToVector: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        instance: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub AddToMap: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        instance: *mut ::core::ffi::c_void,
        key: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub RunInitializer:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IXamlTypeResolver(::windows_core::IUnknown);
impl IXamlTypeResolver {
    #[doc = "Required features: `\"Windows_UI_Xaml_Interop\"`"]
    #[cfg(feature = "Windows_UI_Xaml_Interop")]
    pub fn Resolve(
        &self,
        qualifiedtypename: &::windows_core::HSTRING,
    ) -> ::windows_core::Result<super::super::super::super::Windows::UI::Xaml::Interop::TypeName>
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Resolve)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(qualifiedtypename),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(
    IXamlTypeResolver,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::RuntimeType for IXamlTypeResolver {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_interface::<Self>();
}
unsafe impl ::windows_core::Interface for IXamlTypeResolver {
    type Vtable = IXamlTypeResolver_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IXamlTypeResolver {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x3fa15615_cacf_547f_b1ed_89dae8c67452);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlTypeResolver_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Windows_UI_Xaml_Interop")]
    pub Resolve: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        qualifiedtypename: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        result__: *mut ::std::mem::MaybeUninit<
            super::super::super::super::Windows::UI::Xaml::Interop::TypeName,
        >,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI_Xaml_Interop"))]
    Resolve: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct MarkupExtension(::windows_core::IUnknown);
impl MarkupExtension {
    pub fn ProvideValue(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = &::windows_core::ComInterface::cast::<IMarkupExtensionOverrides>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ProvideValue)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn ProvideValueWithIXamlServiceProvider<P0>(
        &self,
        serviceprovider: P0,
    ) -> ::windows_core::Result<::windows_core::IInspectable>
    where
        P0: ::windows_core::TryIntoParam<super::IXamlServiceProvider>,
    {
        let this = &::windows_core::ComInterface::cast::<IMarkupExtensionOverrides>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ProvideValueWithIXamlServiceProvider)(
                ::windows_core::Interface::as_raw(this),
                serviceprovider.try_into_param()?.abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for MarkupExtension {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for MarkupExtension {
    type Vtable = IMarkupExtension_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MarkupExtension {
    const IID: ::windows_core::GUID = <IMarkupExtension as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MarkupExtension {
    const NAME: &'static str = "Microsoft.UI.Xaml.Markup.MarkupExtension";
}
::windows_core::imp::interface_hierarchy!(
    MarkupExtension,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for MarkupExtension {}
unsafe impl ::core::marker::Sync for MarkupExtension {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ProvideValueTargetProperty(::windows_core::IUnknown);
impl ProvideValueTargetProperty {
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
            ProvideValueTargetProperty,
            ::windows_core::imp::IGenericFactory,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Name(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Name)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_UI_Xaml_Interop\"`"]
    #[cfg(feature = "Windows_UI_Xaml_Interop")]
    pub fn Type(
        &self,
    ) -> ::windows_core::Result<super::super::super::super::Windows::UI::Xaml::Interop::TypeName>
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Type)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_UI_Xaml_Interop\"`"]
    #[cfg(feature = "Windows_UI_Xaml_Interop")]
    pub fn DeclaringType(
        &self,
    ) -> ::windows_core::Result<super::super::super::super::Windows::UI::Xaml::Interop::TypeName>
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DeclaringType)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for ProvideValueTargetProperty {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for ProvideValueTargetProperty {
    type Vtable = IProvideValueTargetProperty_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ProvideValueTargetProperty {
    const IID: ::windows_core::GUID =
        <IProvideValueTargetProperty as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ProvideValueTargetProperty {
    const NAME: &'static str = "Microsoft.UI.Xaml.Markup.ProvideValueTargetProperty";
}
::windows_core::imp::interface_hierarchy!(
    ProvideValueTargetProperty,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for ProvideValueTargetProperty {}
unsafe impl ::core::marker::Sync for ProvideValueTargetProperty {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct XamlBinaryWriter(::windows_core::IUnknown);
impl XamlBinaryWriter {
    #[doc = "Required features: `\"Windows_Foundation_Collections\"`, `\"Windows_Storage_Streams\"`"]
    #[cfg(all(
        feature = "Windows_Foundation_Collections",
        feature = "Windows_Storage_Streams"
    ))]
    pub fn Write<P0, P1, P2>(
        inputstreams: P0,
        outputstreams: P1,
        xamlmetadataprovider: P2,
    ) -> ::windows_core::Result<XamlBinaryWriterErrorInformation>
    where
        P0: ::windows_core::TryIntoParam<
            ::windows::Foundation::Collections::IVector<
                ::windows::Storage::Streams::IRandomAccessStream,
            >,
        >,
        P1: ::windows_core::TryIntoParam<
            ::windows::Foundation::Collections::IVector<
                ::windows::Storage::Streams::IRandomAccessStream,
            >,
        >,
        P2: ::windows_core::TryIntoParam<IXamlMetadataProvider>,
    {
        Self::IXamlBinaryWriterStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Write)(
                ::windows_core::Interface::as_raw(this),
                inputstreams.try_into_param()?.abi(),
                outputstreams.try_into_param()?.abi(),
                xamlmetadataprovider.try_into_param()?.abi(),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IXamlBinaryWriterStatics<
        R,
        F: FnOnce(&IXamlBinaryWriterStatics) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<
            XamlBinaryWriter,
            IXamlBinaryWriterStatics,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for XamlBinaryWriter {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for XamlBinaryWriter {
    type Vtable = IXamlBinaryWriter_Vtbl;
}
unsafe impl ::windows_core::ComInterface for XamlBinaryWriter {
    const IID: ::windows_core::GUID = <IXamlBinaryWriter as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for XamlBinaryWriter {
    const NAME: &'static str = "Microsoft.UI.Xaml.Markup.XamlBinaryWriter";
}
::windows_core::imp::interface_hierarchy!(
    XamlBinaryWriter,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for XamlBinaryWriter {}
unsafe impl ::core::marker::Sync for XamlBinaryWriter {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct XamlBindingHelper(::windows_core::IUnknown);
impl XamlBindingHelper {
    pub fn DataTemplateComponentProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IXamlBindingHelperStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DataTemplateComponentProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn GetDataTemplateComponent<P0>(
        element: P0,
    ) -> ::windows_core::Result<IDataTemplateComponent>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        Self::IXamlBindingHelperStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetDataTemplateComponent)(
                ::windows_core::Interface::as_raw(this),
                element.try_into_param()?.abi(),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn SetDataTemplateComponent<P0, P1>(element: P0, value: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
        P1: ::windows_core::TryIntoParam<IDataTemplateComponent>,
    {
        Self::IXamlBindingHelperStatics(|this| unsafe {
            (::windows_core::Interface::vtable(this).SetDataTemplateComponent)(
                ::windows_core::Interface::as_raw(this),
                element.try_into_param()?.abi(),
                value.try_into_param()?.abi(),
            )
            .ok()
        })
    }
    pub fn SuspendRendering<P0>(target: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::UIElement>,
    {
        Self::IXamlBindingHelperStatics(|this| unsafe {
            (::windows_core::Interface::vtable(this).SuspendRendering)(
                ::windows_core::Interface::as_raw(this),
                target.try_into_param()?.abi(),
            )
            .ok()
        })
    }
    pub fn ResumeRendering<P0>(target: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::UIElement>,
    {
        Self::IXamlBindingHelperStatics(|this| unsafe {
            (::windows_core::Interface::vtable(this).ResumeRendering)(
                ::windows_core::Interface::as_raw(this),
                target.try_into_param()?.abi(),
            )
            .ok()
        })
    }
    #[doc = "Required features: `\"Windows_UI_Xaml_Interop\"`"]
    #[cfg(feature = "Windows_UI_Xaml_Interop")]
    pub fn ConvertValue<P0, P1>(
        r#type: P0,
        value: P1,
    ) -> ::windows_core::Result<::windows_core::IInspectable>
    where
        P0: ::windows_core::IntoParam<
            super::super::super::super::Windows::UI::Xaml::Interop::TypeName,
        >,
        P1: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        Self::IXamlBindingHelperStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ConvertValue)(
                ::windows_core::Interface::as_raw(this),
                r#type.into_param().abi(),
                value.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn SetPropertyFromString<P0, P1>(
        dependencyobject: P0,
        propertytoset: P1,
        value: &::windows_core::HSTRING,
    ) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
        P1: ::windows_core::IntoParam<super::DependencyProperty>,
    {
        Self::IXamlBindingHelperStatics(|this| unsafe {
            (::windows_core::Interface::vtable(this).SetPropertyFromString)(
                ::windows_core::Interface::as_raw(this),
                dependencyobject.into_param().abi(),
                propertytoset.into_param().abi(),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        })
    }
    pub fn SetPropertyFromBoolean<P0, P1>(
        dependencyobject: P0,
        propertytoset: P1,
        value: bool,
    ) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
        P1: ::windows_core::IntoParam<super::DependencyProperty>,
    {
        Self::IXamlBindingHelperStatics(|this| unsafe {
            (::windows_core::Interface::vtable(this).SetPropertyFromBoolean)(
                ::windows_core::Interface::as_raw(this),
                dependencyobject.into_param().abi(),
                propertytoset.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    pub fn SetPropertyFromChar16<P0, P1>(
        dependencyobject: P0,
        propertytoset: P1,
        value: u16,
    ) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
        P1: ::windows_core::IntoParam<super::DependencyProperty>,
    {
        Self::IXamlBindingHelperStatics(|this| unsafe {
            (::windows_core::Interface::vtable(this).SetPropertyFromChar16)(
                ::windows_core::Interface::as_raw(this),
                dependencyobject.into_param().abi(),
                propertytoset.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn SetPropertyFromDateTime<P0, P1>(
        dependencyobject: P0,
        propertytoset: P1,
        value: ::windows::Foundation::DateTime,
    ) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
        P1: ::windows_core::IntoParam<super::DependencyProperty>,
    {
        Self::IXamlBindingHelperStatics(|this| unsafe {
            (::windows_core::Interface::vtable(this).SetPropertyFromDateTime)(
                ::windows_core::Interface::as_raw(this),
                dependencyobject.into_param().abi(),
                propertytoset.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    pub fn SetPropertyFromDouble<P0, P1>(
        dependencyobject: P0,
        propertytoset: P1,
        value: f64,
    ) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
        P1: ::windows_core::IntoParam<super::DependencyProperty>,
    {
        Self::IXamlBindingHelperStatics(|this| unsafe {
            (::windows_core::Interface::vtable(this).SetPropertyFromDouble)(
                ::windows_core::Interface::as_raw(this),
                dependencyobject.into_param().abi(),
                propertytoset.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    pub fn SetPropertyFromInt32<P0, P1>(
        dependencyobject: P0,
        propertytoset: P1,
        value: i32,
    ) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
        P1: ::windows_core::IntoParam<super::DependencyProperty>,
    {
        Self::IXamlBindingHelperStatics(|this| unsafe {
            (::windows_core::Interface::vtable(this).SetPropertyFromInt32)(
                ::windows_core::Interface::as_raw(this),
                dependencyobject.into_param().abi(),
                propertytoset.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    pub fn SetPropertyFromUInt32<P0, P1>(
        dependencyobject: P0,
        propertytoset: P1,
        value: u32,
    ) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
        P1: ::windows_core::IntoParam<super::DependencyProperty>,
    {
        Self::IXamlBindingHelperStatics(|this| unsafe {
            (::windows_core::Interface::vtable(this).SetPropertyFromUInt32)(
                ::windows_core::Interface::as_raw(this),
                dependencyobject.into_param().abi(),
                propertytoset.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    pub fn SetPropertyFromInt64<P0, P1>(
        dependencyobject: P0,
        propertytoset: P1,
        value: i64,
    ) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
        P1: ::windows_core::IntoParam<super::DependencyProperty>,
    {
        Self::IXamlBindingHelperStatics(|this| unsafe {
            (::windows_core::Interface::vtable(this).SetPropertyFromInt64)(
                ::windows_core::Interface::as_raw(this),
                dependencyobject.into_param().abi(),
                propertytoset.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    pub fn SetPropertyFromUInt64<P0, P1>(
        dependencyobject: P0,
        propertytoset: P1,
        value: u64,
    ) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
        P1: ::windows_core::IntoParam<super::DependencyProperty>,
    {
        Self::IXamlBindingHelperStatics(|this| unsafe {
            (::windows_core::Interface::vtable(this).SetPropertyFromUInt64)(
                ::windows_core::Interface::as_raw(this),
                dependencyobject.into_param().abi(),
                propertytoset.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    pub fn SetPropertyFromSingle<P0, P1>(
        dependencyobject: P0,
        propertytoset: P1,
        value: f32,
    ) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
        P1: ::windows_core::IntoParam<super::DependencyProperty>,
    {
        Self::IXamlBindingHelperStatics(|this| unsafe {
            (::windows_core::Interface::vtable(this).SetPropertyFromSingle)(
                ::windows_core::Interface::as_raw(this),
                dependencyobject.into_param().abi(),
                propertytoset.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn SetPropertyFromPoint<P0, P1>(
        dependencyobject: P0,
        propertytoset: P1,
        value: ::windows::Foundation::Point,
    ) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
        P1: ::windows_core::IntoParam<super::DependencyProperty>,
    {
        Self::IXamlBindingHelperStatics(|this| unsafe {
            (::windows_core::Interface::vtable(this).SetPropertyFromPoint)(
                ::windows_core::Interface::as_raw(this),
                dependencyobject.into_param().abi(),
                propertytoset.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn SetPropertyFromRect<P0, P1>(
        dependencyobject: P0,
        propertytoset: P1,
        value: ::windows::Foundation::Rect,
    ) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
        P1: ::windows_core::IntoParam<super::DependencyProperty>,
    {
        Self::IXamlBindingHelperStatics(|this| unsafe {
            (::windows_core::Interface::vtable(this).SetPropertyFromRect)(
                ::windows_core::Interface::as_raw(this),
                dependencyobject.into_param().abi(),
                propertytoset.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn SetPropertyFromSize<P0, P1>(
        dependencyobject: P0,
        propertytoset: P1,
        value: ::windows::Foundation::Size,
    ) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
        P1: ::windows_core::IntoParam<super::DependencyProperty>,
    {
        Self::IXamlBindingHelperStatics(|this| unsafe {
            (::windows_core::Interface::vtable(this).SetPropertyFromSize)(
                ::windows_core::Interface::as_raw(this),
                dependencyobject.into_param().abi(),
                propertytoset.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn SetPropertyFromTimeSpan<P0, P1>(
        dependencyobject: P0,
        propertytoset: P1,
        value: ::windows::Foundation::TimeSpan,
    ) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
        P1: ::windows_core::IntoParam<super::DependencyProperty>,
    {
        Self::IXamlBindingHelperStatics(|this| unsafe {
            (::windows_core::Interface::vtable(this).SetPropertyFromTimeSpan)(
                ::windows_core::Interface::as_raw(this),
                dependencyobject.into_param().abi(),
                propertytoset.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    pub fn SetPropertyFromByte<P0, P1>(
        dependencyobject: P0,
        propertytoset: P1,
        value: u8,
    ) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
        P1: ::windows_core::IntoParam<super::DependencyProperty>,
    {
        Self::IXamlBindingHelperStatics(|this| unsafe {
            (::windows_core::Interface::vtable(this).SetPropertyFromByte)(
                ::windows_core::Interface::as_raw(this),
                dependencyobject.into_param().abi(),
                propertytoset.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn SetPropertyFromUri<P0, P1, P2>(
        dependencyobject: P0,
        propertytoset: P1,
        value: P2,
    ) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
        P1: ::windows_core::IntoParam<super::DependencyProperty>,
        P2: ::windows_core::IntoParam<::windows::Foundation::Uri>,
    {
        Self::IXamlBindingHelperStatics(|this| unsafe {
            (::windows_core::Interface::vtable(this).SetPropertyFromUri)(
                ::windows_core::Interface::as_raw(this),
                dependencyobject.into_param().abi(),
                propertytoset.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        })
    }
    pub fn SetPropertyFromObject<P0, P1, P2>(
        dependencyobject: P0,
        propertytoset: P1,
        value: P2,
    ) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
        P1: ::windows_core::IntoParam<super::DependencyProperty>,
        P2: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        Self::IXamlBindingHelperStatics(|this| unsafe {
            (::windows_core::Interface::vtable(this).SetPropertyFromObject)(
                ::windows_core::Interface::as_raw(this),
                dependencyobject.into_param().abi(),
                propertytoset.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        })
    }
    #[doc(hidden)]
    pub fn IXamlBindingHelperStatics<
        R,
        F: FnOnce(&IXamlBindingHelperStatics) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<
            XamlBindingHelper,
            IXamlBindingHelperStatics,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for XamlBindingHelper {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for XamlBindingHelper {
    type Vtable = IXamlBindingHelper_Vtbl;
}
unsafe impl ::windows_core::ComInterface for XamlBindingHelper {
    const IID: ::windows_core::GUID = <IXamlBindingHelper as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for XamlBindingHelper {
    const NAME: &'static str = "Microsoft.UI.Xaml.Markup.XamlBindingHelper";
}
::windows_core::imp::interface_hierarchy!(
    XamlBindingHelper,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for XamlBindingHelper {}
unsafe impl ::core::marker::Sync for XamlBindingHelper {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct XamlMarkupHelper(::windows_core::IUnknown);
impl XamlMarkupHelper {
    pub fn UnloadObject<P0>(element: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
    {
        Self::IXamlMarkupHelperStatics(|this| unsafe {
            (::windows_core::Interface::vtable(this).UnloadObject)(
                ::windows_core::Interface::as_raw(this),
                element.try_into_param()?.abi(),
            )
            .ok()
        })
    }
    #[doc(hidden)]
    pub fn IXamlMarkupHelperStatics<
        R,
        F: FnOnce(&IXamlMarkupHelperStatics) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<
            XamlMarkupHelper,
            IXamlMarkupHelperStatics,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for XamlMarkupHelper {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for XamlMarkupHelper {
    type Vtable = IXamlMarkupHelper_Vtbl;
}
unsafe impl ::windows_core::ComInterface for XamlMarkupHelper {
    const IID: ::windows_core::GUID = <IXamlMarkupHelper as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for XamlMarkupHelper {
    const NAME: &'static str = "Microsoft.UI.Xaml.Markup.XamlMarkupHelper";
}
::windows_core::imp::interface_hierarchy!(
    XamlMarkupHelper,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for XamlMarkupHelper {}
unsafe impl ::core::marker::Sync for XamlMarkupHelper {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct XamlReader(::windows_core::IUnknown);
impl XamlReader {
    pub fn Load(
        xaml: &::windows_core::HSTRING,
    ) -> ::windows_core::Result<::windows_core::IInspectable> {
        Self::IXamlReaderStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Load)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(xaml),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn LoadWithInitialTemplateValidation(
        xaml: &::windows_core::HSTRING,
    ) -> ::windows_core::Result<::windows_core::IInspectable> {
        Self::IXamlReaderStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LoadWithInitialTemplateValidation)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(xaml),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IXamlReaderStatics<R, F: FnOnce(&IXamlReaderStatics) -> ::windows_core::Result<R>>(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<XamlReader, IXamlReaderStatics> =
            ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for XamlReader {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for XamlReader {
    type Vtable = IXamlReader_Vtbl;
}
unsafe impl ::windows_core::ComInterface for XamlReader {
    const IID: ::windows_core::GUID = <IXamlReader as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for XamlReader {
    const NAME: &'static str = "Microsoft.UI.Xaml.Markup.XamlReader";
}
::windows_core::imp::interface_hierarchy!(
    XamlReader,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for XamlReader {}
unsafe impl ::core::marker::Sync for XamlReader {}
#[repr(C)]
pub struct XamlBinaryWriterErrorInformation {
    pub InputStreamIndex: u32,
    pub LineNumber: u32,
    pub LinePosition: u32,
}
impl ::core::marker::Copy for XamlBinaryWriterErrorInformation {}
impl ::core::clone::Clone for XamlBinaryWriterErrorInformation {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for XamlBinaryWriterErrorInformation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("XamlBinaryWriterErrorInformation")
            .field("InputStreamIndex", &self.InputStreamIndex)
            .field("LineNumber", &self.LineNumber)
            .field("LinePosition", &self.LinePosition)
            .finish()
    }
}
impl ::windows_core::TypeKind for XamlBinaryWriterErrorInformation {
    type TypeKind = ::windows_core::CopyType;
}
impl ::windows_core::RuntimeType for XamlBinaryWriterErrorInformation {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::from_slice(
            b"struct(Microsoft.UI.Xaml.Markup.XamlBinaryWriterErrorInformation;u4;u4;u4)",
        );
}
impl ::core::cmp::PartialEq for XamlBinaryWriterErrorInformation {
    fn eq(&self, other: &Self) -> bool {
        self.InputStreamIndex == other.InputStreamIndex
            && self.LineNumber == other.LineNumber
            && self.LinePosition == other.LinePosition
    }
}
impl ::core::cmp::Eq for XamlBinaryWriterErrorInformation {}
impl ::core::default::Default for XamlBinaryWriterErrorInformation {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct XmlnsDefinition {
    pub XmlNamespace: ::windows_core::HSTRING,
    pub Namespace: ::windows_core::HSTRING,
}
impl ::core::clone::Clone for XmlnsDefinition {
    fn clone(&self) -> Self {
        Self {
            XmlNamespace: self.XmlNamespace.clone(),
            Namespace: self.Namespace.clone(),
        }
    }
}
impl ::core::fmt::Debug for XmlnsDefinition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("XmlnsDefinition")
            .field("XmlNamespace", &self.XmlNamespace)
            .field("Namespace", &self.Namespace)
            .finish()
    }
}
impl ::windows_core::TypeKind for XmlnsDefinition {
    type TypeKind = ::windows_core::ValueType;
}
impl ::windows_core::RuntimeType for XmlnsDefinition {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::from_slice(
            b"struct(Microsoft.UI.Xaml.Markup.XmlnsDefinition;string;string)",
        );
}
impl ::core::cmp::PartialEq for XmlnsDefinition {
    fn eq(&self, other: &Self) -> bool {
        self.XmlNamespace == other.XmlNamespace && self.Namespace == other.Namespace
    }
}
impl ::core::cmp::Eq for XmlnsDefinition {}
impl ::core::default::Default for XmlnsDefinition {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
