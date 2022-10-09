#[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
#[repr(transparent)]
pub struct IComponentConnector(::windows::core::IUnknown);
impl IComponentConnector {
    pub fn Connect<'a, P0>(&self, connectionid: i32, target: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IInspectable>>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).Connect)(
                ::windows::core::Vtable::as_raw(this),
                connectionid,
                target.into().abi(),
            )
            .ok()
        }
    }
    pub fn GetBindingConnector<'a, P0>(
        &self,
        connectionid: i32,
        target: P0,
    ) -> ::windows::core::Result<IComponentConnector>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetBindingConnector)(
                ::windows::core::Vtable::as_raw(this),
                connectionid,
                target.into().abi(),
                result__.as_mut_ptr(),
            )
            .from_abi::<IComponentConnector>(result__)
        }
    }
}
::windows::core::interface_hierarchy!(
    IComponentConnector,
    ::windows::core::IUnknown,
    ::windows::core::IInspectable
);
impl ::core::clone::Clone for IComponentConnector {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IComponentConnector {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IComponentConnector {}
impl ::core::fmt::Debug for IComponentConnector {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IComponentConnector").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IComponentConnector {
    const SIGNATURE: ::windows::core::ConstBuffer =
        ::windows::core::ConstBuffer::from_slice(b"{ad401812-b091-51d0-b915-2d682cd2af10}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for IComponentConnector {
    type Vtable = IComponentConnector_Vtbl;
}
unsafe impl ::windows::core::Interface for IComponentConnector {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xad401812_b091_51d0_b915_2d682cd2af10);
}
#[repr(C)]
#[doc(hidden)]
pub struct IComponentConnector_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Connect: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        connectionid: i32,
        target: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub GetBindingConnector: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        connectionid: i32,
        target: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
#[repr(transparent)]
pub struct IDataTemplateComponent(::windows::core::IUnknown);
impl IDataTemplateComponent {
    pub fn Recycle(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).Recycle)(::windows::core::Vtable::as_raw(this))
                .ok()
        }
    }
    pub fn ProcessBindings<'a, P0>(
        &self,
        item: P0,
        itemindex: i32,
        phase: i32,
        nextphase: &mut i32,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IInspectable>>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).ProcessBindings)(
                ::windows::core::Vtable::as_raw(this),
                item.into().abi(),
                itemindex,
                phase,
                nextphase,
            )
            .ok()
        }
    }
}
::windows::core::interface_hierarchy!(
    IDataTemplateComponent,
    ::windows::core::IUnknown,
    ::windows::core::IInspectable
);
impl ::core::clone::Clone for IDataTemplateComponent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDataTemplateComponent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDataTemplateComponent {}
impl ::core::fmt::Debug for IDataTemplateComponent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDataTemplateComponent").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IDataTemplateComponent {
    const SIGNATURE: ::windows::core::ConstBuffer =
        ::windows::core::ConstBuffer::from_slice(b"{1743ddf7-38ba-58c9-a2a6-b0ae28713bee}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for IDataTemplateComponent {
    type Vtable = IDataTemplateComponent_Vtbl;
}
unsafe impl ::windows::core::Interface for IDataTemplateComponent {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x1743ddf7_38ba_58c9_a2a6_b0ae28713bee);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataTemplateComponent_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Recycle:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ProcessBindings: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        item: *mut ::core::ffi::c_void,
        itemindex: i32,
        phase: i32,
        nextphase: *mut i32,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMarkupExtension(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IMarkupExtension {
    type Vtable = IMarkupExtension_Vtbl;
}
unsafe impl ::windows::core::Interface for IMarkupExtension {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xc355371e_091d_5136_af4a_baf5e00616bd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMarkupExtension_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMarkupExtensionFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IMarkupExtensionFactory {
    type Vtable = IMarkupExtensionFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for IMarkupExtensionFactory {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x20651afa_5f3a_5f0c_adb1_b6551f53a6a0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMarkupExtensionFactory_Vtbl {
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
pub struct IMarkupExtensionOverrides(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IMarkupExtensionOverrides {
    type Vtable = IMarkupExtensionOverrides_Vtbl;
}
unsafe impl ::windows::core::Interface for IMarkupExtensionOverrides {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xa12aa575_5d31_5b68_a30f_8495412a351d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMarkupExtensionOverrides_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ProvideValue: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub ProvideValueWithIXamlServiceProvider: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        serviceprovider: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    )
        -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
#[repr(transparent)]
pub struct IProvideValueTarget(::windows::core::IUnknown);
impl IProvideValueTarget {
    pub fn TargetObject(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TargetObject)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn TargetProperty(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TargetProperty)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
}
::windows::core::interface_hierarchy!(
    IProvideValueTarget,
    ::windows::core::IUnknown,
    ::windows::core::IInspectable
);
impl ::core::clone::Clone for IProvideValueTarget {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IProvideValueTarget {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IProvideValueTarget {}
impl ::core::fmt::Debug for IProvideValueTarget {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IProvideValueTarget").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IProvideValueTarget {
    const SIGNATURE: ::windows::core::ConstBuffer =
        ::windows::core::ConstBuffer::from_slice(b"{3f01ff68-3efd-591d-a506-de13fcaabd83}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for IProvideValueTarget {
    type Vtable = IProvideValueTarget_Vtbl;
}
unsafe impl ::windows::core::Interface for IProvideValueTarget {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x3f01ff68_3efd_591d_a506_de13fcaabd83);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProvideValueTarget_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub TargetObject: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub TargetProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IProvideValueTargetProperty(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IProvideValueTargetProperty {
    type Vtable = IProvideValueTargetProperty_Vtbl;
}
unsafe impl ::windows::core::Interface for IProvideValueTargetProperty {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xce777b1f_b42e_59d1_870d_12fdf0629133);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProvideValueTargetProperty_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Name: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub Type: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::core::mem::ManuallyDrop<crate::core::TypeName>,
    ) -> ::windows::core::HRESULT,
    pub DeclaringType: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::core::mem::ManuallyDrop<crate::core::TypeName>,
    ) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
#[repr(transparent)]
pub struct IRootObjectProvider(::windows::core::IUnknown);
impl IRootObjectProvider {
    pub fn RootObject(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RootObject)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
}
::windows::core::interface_hierarchy!(
    IRootObjectProvider,
    ::windows::core::IUnknown,
    ::windows::core::IInspectable
);
impl ::core::clone::Clone for IRootObjectProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IRootObjectProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRootObjectProvider {}
impl ::core::fmt::Debug for IRootObjectProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRootObjectProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IRootObjectProvider {
    const SIGNATURE: ::windows::core::ConstBuffer =
        ::windows::core::ConstBuffer::from_slice(b"{13d63599-352f-5eb8-81c1-bc62fb12d6da}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for IRootObjectProvider {
    type Vtable = IRootObjectProvider_Vtbl;
}
unsafe impl ::windows::core::Interface for IRootObjectProvider {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x13d63599_352f_5eb8_81c1_bc62fb12d6da);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRootObjectProvider_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub RootObject: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
#[repr(transparent)]
pub struct IUriContext(::windows::core::IUnknown);
impl IUriContext {
    pub fn BaseUri(&self) -> ::windows::core::Result<::windows::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).BaseUri)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::Uri>(result__)
        }
    }
}
::windows::core::interface_hierarchy!(
    IUriContext,
    ::windows::core::IUnknown,
    ::windows::core::IInspectable
);
impl ::core::clone::Clone for IUriContext {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUriContext {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUriContext {}
impl ::core::fmt::Debug for IUriContext {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUriContext").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IUriContext {
    const SIGNATURE: ::windows::core::ConstBuffer =
        ::windows::core::ConstBuffer::from_slice(b"{fb8605f6-8f05-52ee-a01c-3a9e118a6ea2}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for IUriContext {
    type Vtable = IUriContext_Vtbl;
}
unsafe impl ::windows::core::Interface for IUriContext {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xfb8605f6_8f05_52ee_a01c_3a9e118a6ea2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUriContext_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub BaseUri: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXamlBinaryWriter(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IXamlBinaryWriter {
    type Vtable = IXamlBinaryWriter_Vtbl;
}
unsafe impl ::windows::core::Interface for IXamlBinaryWriter {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x8fb45e3b_e689_55bf_aa11_d83b1c1cdda1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlBinaryWriter_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXamlBinaryWriterStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IXamlBinaryWriterStatics {
    type Vtable = IXamlBinaryWriterStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IXamlBinaryWriterStatics {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x774907fc_c846_517f_abcc_c3f7e8c3ffc9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlBinaryWriterStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Write: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        inputstreams: *mut ::core::ffi::c_void,
        outputstreams: *mut ::core::ffi::c_void,
        xamlmetadataprovider: *mut ::core::ffi::c_void,
        result__: *mut XamlBinaryWriterErrorInformation,
    ) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
#[repr(transparent)]
pub struct IXamlBindScopeDiagnostics(::windows::core::IUnknown);
impl IXamlBindScopeDiagnostics {
    pub fn Disable(&self, linenumber: i32, columnnumber: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).Disable)(
                ::windows::core::Vtable::as_raw(this),
                linenumber,
                columnnumber,
            )
            .ok()
        }
    }
}
::windows::core::interface_hierarchy!(
    IXamlBindScopeDiagnostics,
    ::windows::core::IUnknown,
    ::windows::core::IInspectable
);
impl ::core::clone::Clone for IXamlBindScopeDiagnostics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IXamlBindScopeDiagnostics {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXamlBindScopeDiagnostics {}
impl ::core::fmt::Debug for IXamlBindScopeDiagnostics {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXamlBindScopeDiagnostics").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IXamlBindScopeDiagnostics {
    const SIGNATURE: ::windows::core::ConstBuffer =
        ::windows::core::ConstBuffer::from_slice(b"{3ea84e4e-fdfe-55a8-a561-edf5697846d7}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for IXamlBindScopeDiagnostics {
    type Vtable = IXamlBindScopeDiagnostics_Vtbl;
}
unsafe impl ::windows::core::Interface for IXamlBindScopeDiagnostics {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x3ea84e4e_fdfe_55a8_a561_edf5697846d7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlBindScopeDiagnostics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Disable: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        linenumber: i32,
        columnnumber: i32,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXamlBindingHelper(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IXamlBindingHelper {
    type Vtable = IXamlBindingHelper_Vtbl;
}
unsafe impl ::windows::core::Interface for IXamlBindingHelper {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x607a9bf2_5a6d_5c89_a756_bb44f24f28f8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlBindingHelper_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXamlBindingHelperStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IXamlBindingHelperStatics {
    type Vtable = IXamlBindingHelperStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IXamlBindingHelperStatics {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x93c7dad3_f9c2_5372_84dc_9e9c4661d083);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlBindingHelperStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub DataTemplateComponentProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub GetDataTemplateComponent: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub SetDataTemplateComponent: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub SuspendRendering: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        target: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub ResumeRendering: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        target: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub ConvertValue: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        r#type: ::core::mem::ManuallyDrop<crate::core::TypeName>,
        value: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub SetPropertyFromString: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        dependencyobject: *mut ::core::ffi::c_void,
        propertytoset: *mut ::core::ffi::c_void,
        value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub SetPropertyFromBoolean: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        dependencyobject: *mut ::core::ffi::c_void,
        propertytoset: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows::core::HRESULT,
    pub SetPropertyFromChar16: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        dependencyobject: *mut ::core::ffi::c_void,
        propertytoset: *mut ::core::ffi::c_void,
        value: u16,
    ) -> ::windows::core::HRESULT,
    pub SetPropertyFromDateTime: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        dependencyobject: *mut ::core::ffi::c_void,
        propertytoset: *mut ::core::ffi::c_void,
        value: ::windows::Foundation::DateTime,
    ) -> ::windows::core::HRESULT,
    pub SetPropertyFromDouble: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        dependencyobject: *mut ::core::ffi::c_void,
        propertytoset: *mut ::core::ffi::c_void,
        value: f64,
    ) -> ::windows::core::HRESULT,
    pub SetPropertyFromInt32: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        dependencyobject: *mut ::core::ffi::c_void,
        propertytoset: *mut ::core::ffi::c_void,
        value: i32,
    ) -> ::windows::core::HRESULT,
    pub SetPropertyFromUInt32: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        dependencyobject: *mut ::core::ffi::c_void,
        propertytoset: *mut ::core::ffi::c_void,
        value: u32,
    ) -> ::windows::core::HRESULT,
    pub SetPropertyFromInt64: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        dependencyobject: *mut ::core::ffi::c_void,
        propertytoset: *mut ::core::ffi::c_void,
        value: i64,
    ) -> ::windows::core::HRESULT,
    pub SetPropertyFromUInt64: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        dependencyobject: *mut ::core::ffi::c_void,
        propertytoset: *mut ::core::ffi::c_void,
        value: u64,
    ) -> ::windows::core::HRESULT,
    pub SetPropertyFromSingle: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        dependencyobject: *mut ::core::ffi::c_void,
        propertytoset: *mut ::core::ffi::c_void,
        value: f32,
    ) -> ::windows::core::HRESULT,
    pub SetPropertyFromPoint: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        dependencyobject: *mut ::core::ffi::c_void,
        propertytoset: *mut ::core::ffi::c_void,
        value: ::windows::Foundation::Point,
    ) -> ::windows::core::HRESULT,
    pub SetPropertyFromRect: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        dependencyobject: *mut ::core::ffi::c_void,
        propertytoset: *mut ::core::ffi::c_void,
        value: ::windows::Foundation::Rect,
    ) -> ::windows::core::HRESULT,
    pub SetPropertyFromSize: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        dependencyobject: *mut ::core::ffi::c_void,
        propertytoset: *mut ::core::ffi::c_void,
        value: ::windows::Foundation::Size,
    ) -> ::windows::core::HRESULT,
    pub SetPropertyFromTimeSpan: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        dependencyobject: *mut ::core::ffi::c_void,
        propertytoset: *mut ::core::ffi::c_void,
        value: ::windows::Foundation::TimeSpan,
    ) -> ::windows::core::HRESULT,
    pub SetPropertyFromByte: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        dependencyobject: *mut ::core::ffi::c_void,
        propertytoset: *mut ::core::ffi::c_void,
        value: u8,
    ) -> ::windows::core::HRESULT,
    pub SetPropertyFromUri: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        dependencyobject: *mut ::core::ffi::c_void,
        propertytoset: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub SetPropertyFromObject: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        dependencyobject: *mut ::core::ffi::c_void,
        propertytoset: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXamlMarkupHelper(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IXamlMarkupHelper {
    type Vtable = IXamlMarkupHelper_Vtbl;
}
unsafe impl ::windows::core::Interface for IXamlMarkupHelper {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xcd677310_3b06_5a13_b31a_401849570858);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlMarkupHelper_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXamlMarkupHelperStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IXamlMarkupHelperStatics {
    type Vtable = IXamlMarkupHelperStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IXamlMarkupHelperStatics {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xd9a0f6e3_c6cc_5cb6_8999_85788701f339);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlMarkupHelperStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub UnloadObject: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
#[repr(transparent)]
pub struct IXamlMember(::windows::core::IUnknown);
impl IXamlMember {
    pub fn IsAttachable(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsAttachable)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn IsDependencyProperty(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsDependencyProperty)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn IsReadOnly(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsReadOnly)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Name)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn TargetType(&self) -> ::windows::core::Result<IXamlType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TargetType)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<IXamlType>(result__)
        }
    }
    pub fn Type(&self) -> ::windows::core::Result<IXamlType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Type)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<IXamlType>(result__)
        }
    }
    pub fn GetValue<'a, P0>(
        &self,
        instance: P0,
    ) -> ::windows::core::Result<::windows::core::IInspectable>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetValue)(
                ::windows::core::Vtable::as_raw(this),
                instance.into().abi(),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn SetValue<'a, P0, P1>(&self, instance: P0, value: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IInspectable>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IInspectable>>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetValue)(
                ::windows::core::Vtable::as_raw(this),
                instance.into().abi(),
                value.into().abi(),
            )
            .ok()
        }
    }
}
::windows::core::interface_hierarchy!(
    IXamlMember,
    ::windows::core::IUnknown,
    ::windows::core::IInspectable
);
impl ::core::clone::Clone for IXamlMember {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IXamlMember {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXamlMember {}
impl ::core::fmt::Debug for IXamlMember {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXamlMember").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IXamlMember {
    const SIGNATURE: ::windows::core::ConstBuffer =
        ::windows::core::ConstBuffer::from_slice(b"{bf3a2913-5c63-50ec-8660-61809be7b9b9}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for IXamlMember {
    type Vtable = IXamlMember_Vtbl;
}
unsafe impl ::windows::core::Interface for IXamlMember {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xbf3a2913_5c63_50ec_8660_61809be7b9b9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlMember_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub IsAttachable: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub IsDependencyProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub IsReadOnly: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub Name: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub TargetType: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub Type: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub GetValue: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        instance: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub SetValue: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        instance: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
#[repr(transparent)]
pub struct IXamlMetadataProvider(::windows::core::IUnknown);
impl IXamlMetadataProvider {
    pub fn GetXamlType<'a, P0>(&self, r#type: P0) -> ::windows::core::Result<IXamlType>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, crate::core::TypeName>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetXamlType)(
                ::windows::core::Vtable::as_raw(this),
                r#type.into().abi(),
                result__.as_mut_ptr(),
            )
            .from_abi::<IXamlType>(result__)
        }
    }
    pub fn GetXamlTypeByFullName(
        &self,
        fullname: &::windows::core::HSTRING,
    ) -> ::windows::core::Result<IXamlType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetXamlTypeByFullName)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(fullname),
                result__.as_mut_ptr(),
            )
            .from_abi::<IXamlType>(result__)
        }
    }
    pub fn GetXmlnsDefinitions(
        &self,
    ) -> ::windows::core::Result<::windows::core::Array<XmlnsDefinition>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetXmlnsDefinitions)(
                ::windows::core::Vtable::as_raw(this),
                ::windows::core::Array::<XmlnsDefinition>::set_abi_len(result__.assume_init_mut()),
                result__.as_mut_ptr() as *mut _ as _,
            )
            .and_then(|| result__.assume_init())
        }
    }
}
::windows::core::interface_hierarchy!(
    IXamlMetadataProvider,
    ::windows::core::IUnknown,
    ::windows::core::IInspectable
);
impl ::core::clone::Clone for IXamlMetadataProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IXamlMetadataProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXamlMetadataProvider {}
impl ::core::fmt::Debug for IXamlMetadataProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXamlMetadataProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IXamlMetadataProvider {
    const SIGNATURE: ::windows::core::ConstBuffer =
        ::windows::core::ConstBuffer::from_slice(b"{a96251f0-2214-5d53-8746-ce99a2593cd7}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for IXamlMetadataProvider {
    type Vtable = IXamlMetadataProvider_Vtbl;
}
unsafe impl ::windows::core::Interface for IXamlMetadataProvider {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xa96251f0_2214_5d53_8746_ce99a2593cd7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlMetadataProvider_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetXamlType: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        r#type: ::core::mem::ManuallyDrop<crate::core::TypeName>,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub GetXamlTypeByFullName: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        fullname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub GetXmlnsDefinitions: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result_size__: *mut u32,
        result__: *mut *mut ::core::mem::ManuallyDrop<XmlnsDefinition>,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXamlReader(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IXamlReader {
    type Vtable = IXamlReader_Vtbl;
}
unsafe impl ::windows::core::Interface for IXamlReader {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x54ce54c8_38c6_50d9_ac98_4b03eddbde9f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlReader_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXamlReaderStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IXamlReaderStatics {
    type Vtable = IXamlReaderStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IXamlReaderStatics {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x82a4cd9e_435e_5aeb_8c4f_300cece45cae);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlReaderStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Load: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        xaml: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub LoadWithInitialTemplateValidation: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        xaml: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        result__: *mut *mut ::core::ffi::c_void,
    )
        -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
#[repr(transparent)]
pub struct IXamlType(::windows::core::IUnknown);
impl IXamlType {
    pub fn BaseType(&self) -> ::windows::core::Result<IXamlType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).BaseType)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<IXamlType>(result__)
        }
    }
    pub fn ContentProperty(&self) -> ::windows::core::Result<IXamlMember> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ContentProperty)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<IXamlMember>(result__)
        }
    }
    pub fn FullName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FullName)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn IsArray(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsArray)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn IsCollection(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsCollection)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn IsConstructible(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsConstructible)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn IsDictionary(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsDictionary)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn IsMarkupExtension(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsMarkupExtension)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn IsBindable(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsBindable)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn ItemType(&self) -> ::windows::core::Result<IXamlType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ItemType)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<IXamlType>(result__)
        }
    }
    pub fn KeyType(&self) -> ::windows::core::Result<IXamlType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).KeyType)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<IXamlType>(result__)
        }
    }
    pub fn BoxedType(&self) -> ::windows::core::Result<IXamlType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).BoxedType)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<IXamlType>(result__)
        }
    }
    pub fn UnderlyingType(&self) -> ::windows::core::Result<crate::core::TypeName> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).UnderlyingType)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<crate::core::TypeName>(result__)
        }
    }
    pub fn ActivateInstance(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ActivateInstance)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn CreateFromString(
        &self,
        value: &::windows::core::HSTRING,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateFromString)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(value),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn GetMember(
        &self,
        name: &::windows::core::HSTRING,
    ) -> ::windows::core::Result<IXamlMember> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetMember)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(name),
                result__.as_mut_ptr(),
            )
            .from_abi::<IXamlMember>(result__)
        }
    }
    pub fn AddToVector<'a, P0, P1>(&self, instance: P0, value: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IInspectable>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IInspectable>>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).AddToVector)(
                ::windows::core::Vtable::as_raw(this),
                instance.into().abi(),
                value.into().abi(),
            )
            .ok()
        }
    }
    pub fn AddToMap<'a, P0, P1, P2>(
        &self,
        instance: P0,
        key: P1,
        value: P2,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IInspectable>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IInspectable>>,
        P2: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IInspectable>>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).AddToMap)(
                ::windows::core::Vtable::as_raw(this),
                instance.into().abi(),
                key.into().abi(),
                value.into().abi(),
            )
            .ok()
        }
    }
    pub fn RunInitializer(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).RunInitializer)(::windows::core::Vtable::as_raw(
                this,
            ))
            .ok()
        }
    }
}
::windows::core::interface_hierarchy!(
    IXamlType,
    ::windows::core::IUnknown,
    ::windows::core::IInspectable
);
impl ::core::clone::Clone for IXamlType {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IXamlType {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXamlType {}
impl ::core::fmt::Debug for IXamlType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXamlType").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IXamlType {
    const SIGNATURE: ::windows::core::ConstBuffer =
        ::windows::core::ConstBuffer::from_slice(b"{d24219df-7ec9-57f1-a27b-6af251d9c5bc}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for IXamlType {
    type Vtable = IXamlType_Vtbl;
}
unsafe impl ::windows::core::Interface for IXamlType {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xd24219df_7ec9_57f1_a27b_6af251d9c5bc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlType_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub BaseType: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub ContentProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub FullName: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub IsArray: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub IsCollection: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub IsConstructible: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub IsDictionary: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub IsMarkupExtension: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub IsBindable: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub ItemType: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub KeyType: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub BoxedType: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub UnderlyingType: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::core::mem::ManuallyDrop<crate::core::TypeName>,
    ) -> ::windows::core::HRESULT,
    pub ActivateInstance: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub CreateFromString: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub GetMember: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub AddToVector: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        instance: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub AddToMap: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        instance: *mut ::core::ffi::c_void,
        key: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub RunInitializer:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
#[repr(transparent)]
pub struct IXamlTypeResolver(::windows::core::IUnknown);
impl IXamlTypeResolver {
    pub fn Resolve(
        &self,
        qualifiedtypename: &::windows::core::HSTRING,
    ) -> ::windows::core::Result<crate::core::TypeName> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Resolve)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(qualifiedtypename),
                result__.as_mut_ptr(),
            )
            .from_abi::<crate::core::TypeName>(result__)
        }
    }
}
::windows::core::interface_hierarchy!(
    IXamlTypeResolver,
    ::windows::core::IUnknown,
    ::windows::core::IInspectable
);
impl ::core::clone::Clone for IXamlTypeResolver {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IXamlTypeResolver {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXamlTypeResolver {}
impl ::core::fmt::Debug for IXamlTypeResolver {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXamlTypeResolver").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IXamlTypeResolver {
    const SIGNATURE: ::windows::core::ConstBuffer =
        ::windows::core::ConstBuffer::from_slice(b"{3fa15615-cacf-547f-b1ed-89dae8c67452}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for IXamlTypeResolver {
    type Vtable = IXamlTypeResolver_Vtbl;
}
unsafe impl ::windows::core::Interface for IXamlTypeResolver {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x3fa15615_cacf_547f_b1ed_89dae8c67452);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlTypeResolver_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Resolve: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        qualifiedtypename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        result__: *mut ::core::mem::ManuallyDrop<crate::core::TypeName>,
    ) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
#[repr(transparent)]
pub struct MarkupExtension(::windows::core::IUnknown);
impl MarkupExtension {
    pub fn new() -> ::windows::core::Result<MarkupExtension> {
        Self::IMarkupExtensionFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateInstance)(
                ::windows::core::Vtable::as_raw(this),
                ::core::ptr::null_mut(),
                &mut ::core::option::Option::<::windows::core::IInspectable>::None as *mut _ as _,
                result__.as_mut_ptr(),
            )
            .from_abi::<MarkupExtension>(result__)
        })
    }
    pub fn compose<T>(compose: T) -> ::windows::core::Result<MarkupExtension>
    where
        T: ::windows::core::Compose,
    {
        Self::IMarkupExtensionFactory(|this| unsafe {
            let (derived__, base__) = ::windows::core::Compose::compose(compose);
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateInstance)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(&derived__),
                base__ as *mut _ as _,
                result__.as_mut_ptr(),
            )
            .from_abi::<MarkupExtension>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IMarkupExtensionFactory<
        R,
        F: FnOnce(&IMarkupExtensionFactory) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<MarkupExtension, IMarkupExtensionFactory> =
            ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for MarkupExtension {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MarkupExtension {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MarkupExtension {}
impl ::core::fmt::Debug for MarkupExtension {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MarkupExtension").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MarkupExtension {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Xaml.Markup.MarkupExtension;{c355371e-091d-5136-af4a-baf5e00616bd})",
    );
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for MarkupExtension {
    type Vtable = IMarkupExtension_Vtbl;
}
unsafe impl ::windows::core::Interface for MarkupExtension {
    const IID: ::windows::core::GUID = <IMarkupExtension as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MarkupExtension {
    const NAME: &'static str = "Microsoft.UI.Xaml.Markup.MarkupExtension";
}
::windows::core::interface_hierarchy!(
    MarkupExtension,
    ::windows::core::IUnknown,
    ::windows::core::IInspectable
);
unsafe impl ::core::marker::Send for MarkupExtension {}
unsafe impl ::core::marker::Sync for MarkupExtension {}
#[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
#[repr(transparent)]
pub struct ProvideValueTargetProperty(::windows::core::IUnknown);
impl ProvideValueTargetProperty {
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
            ProvideValueTargetProperty,
            ::windows::core::IGenericFactory,
        > = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Name)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn Type(&self) -> ::windows::core::Result<crate::core::TypeName> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Type)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<crate::core::TypeName>(result__)
        }
    }
    pub fn DeclaringType(&self) -> ::windows::core::Result<crate::core::TypeName> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DeclaringType)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<crate::core::TypeName>(result__)
        }
    }
}
impl ::core::clone::Clone for ProvideValueTargetProperty {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ProvideValueTargetProperty {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ProvideValueTargetProperty {}
impl ::core::fmt::Debug for ProvideValueTargetProperty {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ProvideValueTargetProperty").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ProvideValueTargetProperty {
    const SIGNATURE : ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice ( b"rc(Microsoft.UI.Xaml.Markup.ProvideValueTargetProperty;{ce777b1f-b42e-59d1-870d-12fdf0629133})" ) ;
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ProvideValueTargetProperty {
    type Vtable = IProvideValueTargetProperty_Vtbl;
}
unsafe impl ::windows::core::Interface for ProvideValueTargetProperty {
    const IID: ::windows::core::GUID =
        <IProvideValueTargetProperty as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ProvideValueTargetProperty {
    const NAME: &'static str = "Microsoft.UI.Xaml.Markup.ProvideValueTargetProperty";
}
::windows::core::interface_hierarchy!(
    ProvideValueTargetProperty,
    ::windows::core::IUnknown,
    ::windows::core::IInspectable
);
unsafe impl ::core::marker::Send for ProvideValueTargetProperty {}
unsafe impl ::core::marker::Sync for ProvideValueTargetProperty {}
#[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
#[repr(transparent)]
pub struct XamlBinaryWriter(::windows::core::IUnknown);
impl XamlBinaryWriter {
    pub fn Write<'a, P0, E0, P1, E1, P2, E2>(
        inputstreams: P0,
        outputstreams: P1,
        xamlmetadataprovider: P2,
    ) -> ::windows::core::Result<XamlBinaryWriterErrorInformation>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<
                'a,
                ::windows::Foundation::Collections::IVector<
                    ::windows::Storage::Streams::IRandomAccessStream,
                >,
            >,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
        P1: ::std::convert::TryInto<
            ::windows::core::InParam<
                'a,
                ::windows::Foundation::Collections::IVector<
                    ::windows::Storage::Streams::IRandomAccessStream,
                >,
            >,
            Error = E1,
        >,
        E1: ::std::convert::Into<::windows::core::Error>,
        P2: ::std::convert::TryInto<
            ::windows::core::InParam<'a, IXamlMetadataProvider>,
            Error = E2,
        >,
        E2: ::std::convert::Into<::windows::core::Error>,
    {
        Self::IXamlBinaryWriterStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Write)(
                ::windows::core::Vtable::as_raw(this),
                inputstreams.try_into().map_err(|e| e.into())?.abi(),
                outputstreams.try_into().map_err(|e| e.into())?.abi(),
                xamlmetadataprovider.try_into().map_err(|e| e.into())?.abi(),
                result__.as_mut_ptr(),
            )
            .from_abi::<XamlBinaryWriterErrorInformation>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IXamlBinaryWriterStatics<
        R,
        F: FnOnce(&IXamlBinaryWriterStatics) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<XamlBinaryWriter, IXamlBinaryWriterStatics> =
            ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for XamlBinaryWriter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for XamlBinaryWriter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for XamlBinaryWriter {}
impl ::core::fmt::Debug for XamlBinaryWriter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XamlBinaryWriter").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for XamlBinaryWriter {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Xaml.Markup.XamlBinaryWriter;{8fb45e3b-e689-55bf-aa11-d83b1c1cdda1})",
    );
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for XamlBinaryWriter {
    type Vtable = IXamlBinaryWriter_Vtbl;
}
unsafe impl ::windows::core::Interface for XamlBinaryWriter {
    const IID: ::windows::core::GUID = <IXamlBinaryWriter as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for XamlBinaryWriter {
    const NAME: &'static str = "Microsoft.UI.Xaml.Markup.XamlBinaryWriter";
}
::windows::core::interface_hierarchy!(
    XamlBinaryWriter,
    ::windows::core::IUnknown,
    ::windows::core::IInspectable
);
unsafe impl ::core::marker::Send for XamlBinaryWriter {}
unsafe impl ::core::marker::Sync for XamlBinaryWriter {}
#[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
#[repr(transparent)]
pub struct XamlBindingHelper(::windows::core::IUnknown);
impl XamlBindingHelper {
    pub fn DataTemplateComponentProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IXamlBindingHelperStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DataTemplateComponentProperty)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn GetDataTemplateComponent<'a, P0>(
        element: P0,
    ) -> ::windows::core::Result<IDataTemplateComponent>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::DependencyObject>>,
    {
        Self::IXamlBindingHelperStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDataTemplateComponent)(
                ::windows::core::Vtable::as_raw(this),
                element.into().abi(),
                result__.as_mut_ptr(),
            )
            .from_abi::<IDataTemplateComponent>(result__)
        })
    }
    pub fn SetDataTemplateComponent<'a, P0, P1, E1>(
        element: P0,
        value: P1,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::DependencyObject>>,
        P1: ::std::convert::TryInto<
            ::windows::core::InParam<'a, IDataTemplateComponent>,
            Error = E1,
        >,
        E1: ::std::convert::Into<::windows::core::Error>,
    {
        Self::IXamlBindingHelperStatics(|this| unsafe {
            (::windows::core::Vtable::vtable(this).SetDataTemplateComponent)(
                ::windows::core::Vtable::as_raw(this),
                element.into().abi(),
                value.try_into().map_err(|e| e.into())?.abi(),
            )
            .ok()
        })
    }
    pub fn SuspendRendering<'a, P0>(target: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::UIElement>>,
    {
        Self::IXamlBindingHelperStatics(|this| unsafe {
            (::windows::core::Vtable::vtable(this).SuspendRendering)(
                ::windows::core::Vtable::as_raw(this),
                target.into().abi(),
            )
            .ok()
        })
    }
    pub fn ResumeRendering<'a, P0>(target: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::UIElement>>,
    {
        Self::IXamlBindingHelperStatics(|this| unsafe {
            (::windows::core::Vtable::vtable(this).ResumeRendering)(
                ::windows::core::Vtable::as_raw(this),
                target.into().abi(),
            )
            .ok()
        })
    }
    pub fn ConvertValue<'a, P0, P1>(
        r#type: P0,
        value: P1,
    ) -> ::windows::core::Result<::windows::core::IInspectable>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, crate::core::TypeName>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IInspectable>>,
    {
        Self::IXamlBindingHelperStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ConvertValue)(
                ::windows::core::Vtable::as_raw(this),
                r#type.into().abi(),
                value.into().abi(),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        })
    }
    pub fn SetPropertyFromString<'a, P0>(
        dependencyobject: P0,
        propertytoset: &super::DependencyProperty,
        value: &::windows::core::HSTRING,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IInspectable>>,
    {
        Self::IXamlBindingHelperStatics(|this| unsafe {
            (::windows::core::Vtable::vtable(this).SetPropertyFromString)(
                ::windows::core::Vtable::as_raw(this),
                dependencyobject.into().abi(),
                ::core::mem::transmute_copy(propertytoset),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        })
    }
    pub fn SetPropertyFromBoolean<'a, P0>(
        dependencyobject: P0,
        propertytoset: &super::DependencyProperty,
        value: bool,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IInspectable>>,
    {
        Self::IXamlBindingHelperStatics(|this| unsafe {
            (::windows::core::Vtable::vtable(this).SetPropertyFromBoolean)(
                ::windows::core::Vtable::as_raw(this),
                dependencyobject.into().abi(),
                ::core::mem::transmute_copy(propertytoset),
                value,
            )
            .ok()
        })
    }
    pub fn SetPropertyFromChar16<'a, P0>(
        dependencyobject: P0,
        propertytoset: &super::DependencyProperty,
        value: u16,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IInspectable>>,
    {
        Self::IXamlBindingHelperStatics(|this| unsafe {
            (::windows::core::Vtable::vtable(this).SetPropertyFromChar16)(
                ::windows::core::Vtable::as_raw(this),
                dependencyobject.into().abi(),
                ::core::mem::transmute_copy(propertytoset),
                value,
            )
            .ok()
        })
    }
    pub fn SetPropertyFromDateTime<'a, P0>(
        dependencyobject: P0,
        propertytoset: &super::DependencyProperty,
        value: ::windows::Foundation::DateTime,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IInspectable>>,
    {
        Self::IXamlBindingHelperStatics(|this| unsafe {
            (::windows::core::Vtable::vtable(this).SetPropertyFromDateTime)(
                ::windows::core::Vtable::as_raw(this),
                dependencyobject.into().abi(),
                ::core::mem::transmute_copy(propertytoset),
                value,
            )
            .ok()
        })
    }
    pub fn SetPropertyFromDouble<'a, P0>(
        dependencyobject: P0,
        propertytoset: &super::DependencyProperty,
        value: f64,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IInspectable>>,
    {
        Self::IXamlBindingHelperStatics(|this| unsafe {
            (::windows::core::Vtable::vtable(this).SetPropertyFromDouble)(
                ::windows::core::Vtable::as_raw(this),
                dependencyobject.into().abi(),
                ::core::mem::transmute_copy(propertytoset),
                value,
            )
            .ok()
        })
    }
    pub fn SetPropertyFromInt32<'a, P0>(
        dependencyobject: P0,
        propertytoset: &super::DependencyProperty,
        value: i32,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IInspectable>>,
    {
        Self::IXamlBindingHelperStatics(|this| unsafe {
            (::windows::core::Vtable::vtable(this).SetPropertyFromInt32)(
                ::windows::core::Vtable::as_raw(this),
                dependencyobject.into().abi(),
                ::core::mem::transmute_copy(propertytoset),
                value,
            )
            .ok()
        })
    }
    pub fn SetPropertyFromUInt32<'a, P0>(
        dependencyobject: P0,
        propertytoset: &super::DependencyProperty,
        value: u32,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IInspectable>>,
    {
        Self::IXamlBindingHelperStatics(|this| unsafe {
            (::windows::core::Vtable::vtable(this).SetPropertyFromUInt32)(
                ::windows::core::Vtable::as_raw(this),
                dependencyobject.into().abi(),
                ::core::mem::transmute_copy(propertytoset),
                value,
            )
            .ok()
        })
    }
    pub fn SetPropertyFromInt64<'a, P0>(
        dependencyobject: P0,
        propertytoset: &super::DependencyProperty,
        value: i64,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IInspectable>>,
    {
        Self::IXamlBindingHelperStatics(|this| unsafe {
            (::windows::core::Vtable::vtable(this).SetPropertyFromInt64)(
                ::windows::core::Vtable::as_raw(this),
                dependencyobject.into().abi(),
                ::core::mem::transmute_copy(propertytoset),
                value,
            )
            .ok()
        })
    }
    pub fn SetPropertyFromUInt64<'a, P0>(
        dependencyobject: P0,
        propertytoset: &super::DependencyProperty,
        value: u64,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IInspectable>>,
    {
        Self::IXamlBindingHelperStatics(|this| unsafe {
            (::windows::core::Vtable::vtable(this).SetPropertyFromUInt64)(
                ::windows::core::Vtable::as_raw(this),
                dependencyobject.into().abi(),
                ::core::mem::transmute_copy(propertytoset),
                value,
            )
            .ok()
        })
    }
    pub fn SetPropertyFromSingle<'a, P0>(
        dependencyobject: P0,
        propertytoset: &super::DependencyProperty,
        value: f32,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IInspectable>>,
    {
        Self::IXamlBindingHelperStatics(|this| unsafe {
            (::windows::core::Vtable::vtable(this).SetPropertyFromSingle)(
                ::windows::core::Vtable::as_raw(this),
                dependencyobject.into().abi(),
                ::core::mem::transmute_copy(propertytoset),
                value,
            )
            .ok()
        })
    }
    pub fn SetPropertyFromPoint<'a, P0>(
        dependencyobject: P0,
        propertytoset: &super::DependencyProperty,
        value: ::windows::Foundation::Point,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IInspectable>>,
    {
        Self::IXamlBindingHelperStatics(|this| unsafe {
            (::windows::core::Vtable::vtable(this).SetPropertyFromPoint)(
                ::windows::core::Vtable::as_raw(this),
                dependencyobject.into().abi(),
                ::core::mem::transmute_copy(propertytoset),
                value,
            )
            .ok()
        })
    }
    pub fn SetPropertyFromRect<'a, P0>(
        dependencyobject: P0,
        propertytoset: &super::DependencyProperty,
        value: ::windows::Foundation::Rect,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IInspectable>>,
    {
        Self::IXamlBindingHelperStatics(|this| unsafe {
            (::windows::core::Vtable::vtable(this).SetPropertyFromRect)(
                ::windows::core::Vtable::as_raw(this),
                dependencyobject.into().abi(),
                ::core::mem::transmute_copy(propertytoset),
                value,
            )
            .ok()
        })
    }
    pub fn SetPropertyFromSize<'a, P0>(
        dependencyobject: P0,
        propertytoset: &super::DependencyProperty,
        value: ::windows::Foundation::Size,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IInspectable>>,
    {
        Self::IXamlBindingHelperStatics(|this| unsafe {
            (::windows::core::Vtable::vtable(this).SetPropertyFromSize)(
                ::windows::core::Vtable::as_raw(this),
                dependencyobject.into().abi(),
                ::core::mem::transmute_copy(propertytoset),
                value,
            )
            .ok()
        })
    }
    pub fn SetPropertyFromTimeSpan<'a, P0>(
        dependencyobject: P0,
        propertytoset: &super::DependencyProperty,
        value: ::windows::Foundation::TimeSpan,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IInspectable>>,
    {
        Self::IXamlBindingHelperStatics(|this| unsafe {
            (::windows::core::Vtable::vtable(this).SetPropertyFromTimeSpan)(
                ::windows::core::Vtable::as_raw(this),
                dependencyobject.into().abi(),
                ::core::mem::transmute_copy(propertytoset),
                value,
            )
            .ok()
        })
    }
    pub fn SetPropertyFromByte<'a, P0>(
        dependencyobject: P0,
        propertytoset: &super::DependencyProperty,
        value: u8,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IInspectable>>,
    {
        Self::IXamlBindingHelperStatics(|this| unsafe {
            (::windows::core::Vtable::vtable(this).SetPropertyFromByte)(
                ::windows::core::Vtable::as_raw(this),
                dependencyobject.into().abi(),
                ::core::mem::transmute_copy(propertytoset),
                value,
            )
            .ok()
        })
    }
    pub fn SetPropertyFromUri<'a, P0>(
        dependencyobject: P0,
        propertytoset: &super::DependencyProperty,
        value: &::windows::Foundation::Uri,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IInspectable>>,
    {
        Self::IXamlBindingHelperStatics(|this| unsafe {
            (::windows::core::Vtable::vtable(this).SetPropertyFromUri)(
                ::windows::core::Vtable::as_raw(this),
                dependencyobject.into().abi(),
                ::core::mem::transmute_copy(propertytoset),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        })
    }
    pub fn SetPropertyFromObject<'a, P0, P1>(
        dependencyobject: P0,
        propertytoset: &super::DependencyProperty,
        value: P1,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IInspectable>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IInspectable>>,
    {
        Self::IXamlBindingHelperStatics(|this| unsafe {
            (::windows::core::Vtable::vtable(this).SetPropertyFromObject)(
                ::windows::core::Vtable::as_raw(this),
                dependencyobject.into().abi(),
                ::core::mem::transmute_copy(propertytoset),
                value.into().abi(),
            )
            .ok()
        })
    }
    #[doc(hidden)]
    pub fn IXamlBindingHelperStatics<
        R,
        F: FnOnce(&IXamlBindingHelperStatics) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<XamlBindingHelper, IXamlBindingHelperStatics> =
            ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for XamlBindingHelper {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for XamlBindingHelper {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for XamlBindingHelper {}
impl ::core::fmt::Debug for XamlBindingHelper {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XamlBindingHelper").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for XamlBindingHelper {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Xaml.Markup.XamlBindingHelper;{607a9bf2-5a6d-5c89-a756-bb44f24f28f8})",
    );
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for XamlBindingHelper {
    type Vtable = IXamlBindingHelper_Vtbl;
}
unsafe impl ::windows::core::Interface for XamlBindingHelper {
    const IID: ::windows::core::GUID = <IXamlBindingHelper as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for XamlBindingHelper {
    const NAME: &'static str = "Microsoft.UI.Xaml.Markup.XamlBindingHelper";
}
::windows::core::interface_hierarchy!(
    XamlBindingHelper,
    ::windows::core::IUnknown,
    ::windows::core::IInspectable
);
unsafe impl ::core::marker::Send for XamlBindingHelper {}
unsafe impl ::core::marker::Sync for XamlBindingHelper {}
#[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
#[repr(transparent)]
pub struct XamlMarkupHelper(::windows::core::IUnknown);
impl XamlMarkupHelper {
    pub fn UnloadObject<'a, P0>(element: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::DependencyObject>>,
    {
        Self::IXamlMarkupHelperStatics(|this| unsafe {
            (::windows::core::Vtable::vtable(this).UnloadObject)(
                ::windows::core::Vtable::as_raw(this),
                element.into().abi(),
            )
            .ok()
        })
    }
    #[doc(hidden)]
    pub fn IXamlMarkupHelperStatics<
        R,
        F: FnOnce(&IXamlMarkupHelperStatics) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<XamlMarkupHelper, IXamlMarkupHelperStatics> =
            ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for XamlMarkupHelper {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for XamlMarkupHelper {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for XamlMarkupHelper {}
impl ::core::fmt::Debug for XamlMarkupHelper {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XamlMarkupHelper").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for XamlMarkupHelper {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Xaml.Markup.XamlMarkupHelper;{cd677310-3b06-5a13-b31a-401849570858})",
    );
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for XamlMarkupHelper {
    type Vtable = IXamlMarkupHelper_Vtbl;
}
unsafe impl ::windows::core::Interface for XamlMarkupHelper {
    const IID: ::windows::core::GUID = <IXamlMarkupHelper as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for XamlMarkupHelper {
    const NAME: &'static str = "Microsoft.UI.Xaml.Markup.XamlMarkupHelper";
}
::windows::core::interface_hierarchy!(
    XamlMarkupHelper,
    ::windows::core::IUnknown,
    ::windows::core::IInspectable
);
unsafe impl ::core::marker::Send for XamlMarkupHelper {}
unsafe impl ::core::marker::Sync for XamlMarkupHelper {}
#[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
#[repr(transparent)]
pub struct XamlReader(::windows::core::IUnknown);
impl XamlReader {
    pub fn Load(
        xaml: &::windows::core::HSTRING,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        Self::IXamlReaderStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Load)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(xaml),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        })
    }
    pub fn LoadWithInitialTemplateValidation(
        xaml: &::windows::core::HSTRING,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        Self::IXamlReaderStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LoadWithInitialTemplateValidation)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(xaml),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IXamlReaderStatics<R, F: FnOnce(&IXamlReaderStatics) -> ::windows::core::Result<R>>(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<XamlReader, IXamlReaderStatics> =
            ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for XamlReader {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for XamlReader {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for XamlReader {}
impl ::core::fmt::Debug for XamlReader {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XamlReader").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for XamlReader {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Xaml.Markup.XamlReader;{54ce54c8-38c6-50d9-ac98-4b03eddbde9f})",
    );
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for XamlReader {
    type Vtable = IXamlReader_Vtbl;
}
unsafe impl ::windows::core::Interface for XamlReader {
    const IID: ::windows::core::GUID = <IXamlReader as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for XamlReader {
    const NAME: &'static str = "Microsoft.UI.Xaml.Markup.XamlReader";
}
::windows::core::interface_hierarchy!(
    XamlReader,
    ::windows::core::IUnknown,
    ::windows::core::IInspectable
);
unsafe impl ::core::marker::Send for XamlReader {}
unsafe impl ::core::marker::Sync for XamlReader {}
#[repr(C)]
#[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
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
unsafe impl ::windows::core::Abi for XamlBinaryWriterErrorInformation {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for XamlBinaryWriterErrorInformation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"struct(Microsoft.UI.Xaml.Markup.XamlBinaryWriterErrorInformation;u4;u4;u4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
impl ::core::cmp::PartialEq for XamlBinaryWriterErrorInformation {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            ::windows::core::memcmp(
                self as *const _ as _,
                other as *const _ as _,
                core::mem::size_of::<XamlBinaryWriterErrorInformation>(),
            ) == 0
        }
    }
}
impl ::core::cmp::Eq for XamlBinaryWriterErrorInformation {}
impl ::core::default::Default for XamlBinaryWriterErrorInformation {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"UI_Xaml_Markup\"`*"]
pub struct XmlnsDefinition {
    pub XmlNamespace: ::windows::core::HSTRING,
    pub Namespace: ::windows::core::HSTRING,
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
unsafe impl ::windows::core::Abi for XmlnsDefinition {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
unsafe impl ::windows::core::RuntimeType for XmlnsDefinition {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"struct(Microsoft.UI.Xaml.Markup.XmlnsDefinition;string;string)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(from.clone())
    }
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
