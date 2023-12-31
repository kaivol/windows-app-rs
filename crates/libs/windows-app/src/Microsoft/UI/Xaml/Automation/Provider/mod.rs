#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IAnnotationProvider(::windows_core::IUnknown);
impl IAnnotationProvider {
    pub fn AnnotationTypeId(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AnnotationTypeId)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn AnnotationTypeName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AnnotationTypeName)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn Author(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Author)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn DateTime(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DateTime)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn Target(&self) -> ::windows_core::Result<IRawElementProviderSimple> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Target)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(
    IAnnotationProvider,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::RuntimeType for IAnnotationProvider {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_interface::<Self>();
}
unsafe impl ::windows_core::Interface for IAnnotationProvider {
    type Vtable = IAnnotationProvider_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IAnnotationProvider {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x546ab18e_986d_5deb_8f2a_2d9303a43006);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAnnotationProvider_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub AnnotationTypeId: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut i32,
    ) -> ::windows_core::HRESULT,
    pub AnnotationTypeName: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub Author: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub DateTime: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub Target: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICustomNavigationProvider(::windows_core::IUnknown);
impl ICustomNavigationProvider {
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Automation_Peers\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Automation_Peers")]
    pub fn NavigateCustom(
        &self,
        direction: super::Peers::AutomationNavigationDirection,
    ) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NavigateCustom)(
                ::windows_core::Interface::as_raw(this),
                direction,
                &mut result__,
            )
            .from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(
    ICustomNavigationProvider,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::RuntimeType for ICustomNavigationProvider {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_interface::<Self>();
}
unsafe impl ::windows_core::Interface for ICustomNavigationProvider {
    type Vtable = ICustomNavigationProvider_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICustomNavigationProvider {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xcad51322_faa9_5a2b_90f0_b762c46178b3);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICustomNavigationProvider_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Microsoft_UI_Xaml_Automation_Peers")]
    pub NavigateCustom: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        direction: super::Peers::AutomationNavigationDirection,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Microsoft_UI_Xaml_Automation_Peers"))]
    NavigateCustom: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDockProvider(::windows_core::IUnknown);
impl IDockProvider {
    pub fn DockPosition(&self) -> ::windows_core::Result<super::DockPosition> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DockPosition)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetDockPosition(&self, dockposition: super::DockPosition) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetDockPosition)(
                ::windows_core::Interface::as_raw(this),
                dockposition,
            )
            .ok()
        }
    }
}
::windows_core::imp::interface_hierarchy!(
    IDockProvider,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::RuntimeType for IDockProvider {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_interface::<Self>();
}
unsafe impl ::windows_core::Interface for IDockProvider {
    type Vtable = IDockProvider_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDockProvider {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x9882b971_70ea_5c6d_a818_7a7ab68c6f3b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDockProvider_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub DockPosition: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::DockPosition,
    ) -> ::windows_core::HRESULT,
    pub SetDockPosition: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        dockposition: super::DockPosition,
    ) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDragProvider(::windows_core::IUnknown);
impl IDragProvider {
    pub fn IsGrabbed(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsGrabbed)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn DropEffect(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DropEffect)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn DropEffects(
        &self,
    ) -> ::windows_core::Result<::windows_core::Array<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows_core::Interface::vtable(this).DropEffects)(
                ::windows_core::Interface::as_raw(this),
                ::windows_core::Array::<::windows_core::HSTRING>::set_abi_len(
                    ::std::mem::transmute(&mut result__),
                ),
                result__.as_mut_ptr() as *mut _ as _,
            )
            .and_then(|| result__.assume_init())
        }
    }
    pub fn GetGrabbedItems(
        &self,
    ) -> ::windows_core::Result<::windows_core::Array<IRawElementProviderSimple>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows_core::Interface::vtable(this).GetGrabbedItems)(
                ::windows_core::Interface::as_raw(this),
                ::windows_core::Array::<IRawElementProviderSimple>::set_abi_len(
                    ::std::mem::transmute(&mut result__),
                ),
                result__.as_mut_ptr() as *mut _ as _,
            )
            .and_then(|| result__.assume_init())
        }
    }
}
::windows_core::imp::interface_hierarchy!(
    IDragProvider,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::RuntimeType for IDragProvider {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_interface::<Self>();
}
unsafe impl ::windows_core::Interface for IDragProvider {
    type Vtable = IDragProvider_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDragProvider {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xc60bb643_a356_5132_a258_ffba6c7480f2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDragProvider_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub IsGrabbed: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub DropEffect: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub DropEffects: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result_size__: *mut u32,
        result__: *mut *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub GetGrabbedItems: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result_size__: *mut u32,
        result__: *mut *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDropTargetProvider(::windows_core::IUnknown);
impl IDropTargetProvider {
    pub fn DropEffect(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DropEffect)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn DropEffects(
        &self,
    ) -> ::windows_core::Result<::windows_core::Array<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows_core::Interface::vtable(this).DropEffects)(
                ::windows_core::Interface::as_raw(this),
                ::windows_core::Array::<::windows_core::HSTRING>::set_abi_len(
                    ::std::mem::transmute(&mut result__),
                ),
                result__.as_mut_ptr() as *mut _ as _,
            )
            .and_then(|| result__.assume_init())
        }
    }
}
::windows_core::imp::interface_hierarchy!(
    IDropTargetProvider,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::RuntimeType for IDropTargetProvider {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_interface::<Self>();
}
unsafe impl ::windows_core::Interface for IDropTargetProvider {
    type Vtable = IDropTargetProvider_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDropTargetProvider {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x9b2a9f3d_bbb1_510d_99e8_0e0ae14a6e3b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDropTargetProvider_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub DropEffect: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub DropEffects: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result_size__: *mut u32,
        result__: *mut *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IExpandCollapseProvider(::windows_core::IUnknown);
impl IExpandCollapseProvider {
    pub fn ExpandCollapseState(&self) -> ::windows_core::Result<super::ExpandCollapseState> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ExpandCollapseState)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn Collapse(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).Collapse)(::windows_core::Interface::as_raw(
                this,
            ))
            .ok()
        }
    }
    pub fn Expand(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).Expand)(::windows_core::Interface::as_raw(
                this,
            ))
            .ok()
        }
    }
}
::windows_core::imp::interface_hierarchy!(
    IExpandCollapseProvider,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::RuntimeType for IExpandCollapseProvider {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_interface::<Self>();
}
unsafe impl ::windows_core::Interface for IExpandCollapseProvider {
    type Vtable = IExpandCollapseProvider_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IExpandCollapseProvider {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x6cef349c_b181_5d0b_b297_c3b0166120c3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IExpandCollapseProvider_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ExpandCollapseState: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::ExpandCollapseState,
    ) -> ::windows_core::HRESULT,
    pub Collapse:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Expand:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IGridItemProvider(::windows_core::IUnknown);
impl IGridItemProvider {
    pub fn Column(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Column)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn ColumnSpan(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ColumnSpan)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn ContainingGrid(&self) -> ::windows_core::Result<IRawElementProviderSimple> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ContainingGrid)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn Row(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Row)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn RowSpan(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RowSpan)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(
    IGridItemProvider,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::RuntimeType for IGridItemProvider {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_interface::<Self>();
}
unsafe impl ::windows_core::Interface for IGridItemProvider {
    type Vtable = IGridItemProvider_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IGridItemProvider {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xd2557a0e_6909_5170_a680_60728df339b4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGridItemProvider_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Column: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut i32,
    ) -> ::windows_core::HRESULT,
    pub ColumnSpan: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut i32,
    ) -> ::windows_core::HRESULT,
    pub ContainingGrid: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub Row: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut i32,
    ) -> ::windows_core::HRESULT,
    pub RowSpan: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut i32,
    ) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IGridProvider(::windows_core::IUnknown);
impl IGridProvider {
    pub fn ColumnCount(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ColumnCount)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn RowCount(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RowCount)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn GetItem(
        &self,
        row: i32,
        column: i32,
    ) -> ::windows_core::Result<IRawElementProviderSimple> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetItem)(
                ::windows_core::Interface::as_raw(this),
                row,
                column,
                &mut result__,
            )
            .from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(
    IGridProvider,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::RuntimeType for IGridProvider {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_interface::<Self>();
}
unsafe impl ::windows_core::Interface for IGridProvider {
    type Vtable = IGridProvider_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IGridProvider {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x50992d5e_d225_56e9_a25a_78c372e81955);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGridProvider_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ColumnCount: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut i32,
    ) -> ::windows_core::HRESULT,
    pub RowCount: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut i32,
    ) -> ::windows_core::HRESULT,
    pub GetItem: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        row: i32,
        column: i32,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IIRawElementProviderSimple(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IIRawElementProviderSimple {
    type Vtable = IIRawElementProviderSimple_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IIRawElementProviderSimple {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xf90bc239_ade2_55c9_a838_a3b0579763c5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIRawElementProviderSimple_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IInvokeProvider(::windows_core::IUnknown);
impl IInvokeProvider {
    pub fn Invoke(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).Invoke)(::windows_core::Interface::as_raw(
                this,
            ))
            .ok()
        }
    }
}
::windows_core::imp::interface_hierarchy!(
    IInvokeProvider,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::RuntimeType for IInvokeProvider {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_interface::<Self>();
}
unsafe impl ::windows_core::Interface for IInvokeProvider {
    type Vtable = IInvokeProvider_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IInvokeProvider {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x02481105_3378_544d_b4e1_a1b368afbc02);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInvokeProvider_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Invoke:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IItemContainerProvider(::windows_core::IUnknown);
impl IItemContainerProvider {
    pub fn FindItemByProperty<P0, P1, P2>(
        &self,
        startafter: P0,
        automationproperty: P1,
        value: P2,
    ) -> ::windows_core::Result<IRawElementProviderSimple>
    where
        P0: ::windows_core::IntoParam<IRawElementProviderSimple>,
        P1: ::windows_core::IntoParam<super::AutomationProperty>,
        P2: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FindItemByProperty)(
                ::windows_core::Interface::as_raw(this),
                startafter.into_param().abi(),
                automationproperty.into_param().abi(),
                value.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(
    IItemContainerProvider,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::RuntimeType for IItemContainerProvider {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_interface::<Self>();
}
unsafe impl ::windows_core::Interface for IItemContainerProvider {
    type Vtable = IItemContainerProvider_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IItemContainerProvider {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xad297363_694e_5885_997d_a2d6dff415a7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IItemContainerProvider_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub FindItemByProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        startafter: *mut ::core::ffi::c_void,
        automationproperty: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMultipleViewProvider(::windows_core::IUnknown);
impl IMultipleViewProvider {
    pub fn CurrentView(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CurrentView)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn GetSupportedViews(&self) -> ::windows_core::Result<::windows_core::Array<i32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows_core::Interface::vtable(this).GetSupportedViews)(
                ::windows_core::Interface::as_raw(this),
                ::windows_core::Array::<i32>::set_abi_len(::std::mem::transmute(&mut result__)),
                result__.as_mut_ptr() as *mut _ as _,
            )
            .and_then(|| result__.assume_init())
        }
    }
    pub fn GetViewName(&self, viewid: i32) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetViewName)(
                ::windows_core::Interface::as_raw(this),
                viewid,
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetCurrentView(&self, viewid: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetCurrentView)(
                ::windows_core::Interface::as_raw(this),
                viewid,
            )
            .ok()
        }
    }
}
::windows_core::imp::interface_hierarchy!(
    IMultipleViewProvider,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::RuntimeType for IMultipleViewProvider {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_interface::<Self>();
}
unsafe impl ::windows_core::Interface for IMultipleViewProvider {
    type Vtable = IMultipleViewProvider_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMultipleViewProvider {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x60be5484_3d8f_51fd_beab_423422ee1e03);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMultipleViewProvider_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CurrentView: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut i32,
    ) -> ::windows_core::HRESULT,
    pub GetSupportedViews: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result_size__: *mut u32,
        result__: *mut *mut i32,
    ) -> ::windows_core::HRESULT,
    pub GetViewName: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        viewid: i32,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub SetCurrentView: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        viewid: i32,
    ) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IObjectModelProvider(::windows_core::IUnknown);
impl IObjectModelProvider {
    pub fn GetUnderlyingObjectModel(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetUnderlyingObjectModel)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(
    IObjectModelProvider,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::RuntimeType for IObjectModelProvider {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_interface::<Self>();
}
unsafe impl ::windows_core::Interface for IObjectModelProvider {
    type Vtable = IObjectModelProvider_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IObjectModelProvider {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x92953ed0_4bd8_5624_8e3d_78d45fde9cf2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IObjectModelProvider_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub GetUnderlyingObjectModel: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IRangeValueProvider(::windows_core::IUnknown);
impl IRangeValueProvider {
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
    pub fn LargeChange(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LargeChange)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn Maximum(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Maximum)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn Minimum(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Minimum)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SmallChange(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SmallChange)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn Value(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Value)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetValue(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetValue)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
}
::windows_core::imp::interface_hierarchy!(
    IRangeValueProvider,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::RuntimeType for IRangeValueProvider {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_interface::<Self>();
}
unsafe impl ::windows_core::Interface for IRangeValueProvider {
    type Vtable = IRangeValueProvider_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IRangeValueProvider {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x729ae414_1e8f_5020_82bb_bb574d145fd8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRangeValueProvider_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub IsReadOnly: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub LargeChange: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub Maximum: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub Minimum: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub SmallChange: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub Value: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub SetValue: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
    ) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IScrollItemProvider(::windows_core::IUnknown);
impl IScrollItemProvider {
    pub fn ScrollIntoView(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).ScrollIntoView)(
                ::windows_core::Interface::as_raw(this),
            )
            .ok()
        }
    }
}
::windows_core::imp::interface_hierarchy!(
    IScrollItemProvider,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::RuntimeType for IScrollItemProvider {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_interface::<Self>();
}
unsafe impl ::windows_core::Interface for IScrollItemProvider {
    type Vtable = IScrollItemProvider_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IScrollItemProvider {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x8a6fb8eb_e5f1_58eb_8e72_8b95f236fc47);
}
#[repr(C)]
#[doc(hidden)]
pub struct IScrollItemProvider_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ScrollIntoView:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IScrollProvider(::windows_core::IUnknown);
impl IScrollProvider {
    pub fn HorizontallyScrollable(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HorizontallyScrollable)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn HorizontalScrollPercent(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HorizontalScrollPercent)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn HorizontalViewSize(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HorizontalViewSize)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn VerticallyScrollable(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).VerticallyScrollable)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn VerticalScrollPercent(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).VerticalScrollPercent)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn VerticalViewSize(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).VerticalViewSize)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn Scroll(
        &self,
        horizontalamount: super::ScrollAmount,
        verticalamount: super::ScrollAmount,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).Scroll)(
                ::windows_core::Interface::as_raw(this),
                horizontalamount,
                verticalamount,
            )
            .ok()
        }
    }
    pub fn SetScrollPercent(
        &self,
        horizontalpercent: f64,
        verticalpercent: f64,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetScrollPercent)(
                ::windows_core::Interface::as_raw(this),
                horizontalpercent,
                verticalpercent,
            )
            .ok()
        }
    }
}
::windows_core::imp::interface_hierarchy!(
    IScrollProvider,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::RuntimeType for IScrollProvider {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_interface::<Self>();
}
unsafe impl ::windows_core::Interface for IScrollProvider {
    type Vtable = IScrollProvider_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IScrollProvider {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x7e2e5af3_ff50_5365_bcfe_ef424b2fd590);
}
#[repr(C)]
#[doc(hidden)]
pub struct IScrollProvider_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub HorizontallyScrollable: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub HorizontalScrollPercent: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub HorizontalViewSize: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub VerticallyScrollable: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub VerticalScrollPercent: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub VerticalViewSize: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub Scroll: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        horizontalamount: super::ScrollAmount,
        verticalamount: super::ScrollAmount,
    ) -> ::windows_core::HRESULT,
    pub SetScrollPercent: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        horizontalpercent: f64,
        verticalpercent: f64,
    ) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ISelectionItemProvider(::windows_core::IUnknown);
impl ISelectionItemProvider {
    pub fn IsSelected(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsSelected)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SelectionContainer(&self) -> ::windows_core::Result<IRawElementProviderSimple> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SelectionContainer)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn AddToSelection(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).AddToSelection)(
                ::windows_core::Interface::as_raw(this),
            )
            .ok()
        }
    }
    pub fn RemoveFromSelection(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveFromSelection)(
                ::windows_core::Interface::as_raw(this),
            )
            .ok()
        }
    }
    pub fn Select(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).Select)(::windows_core::Interface::as_raw(
                this,
            ))
            .ok()
        }
    }
}
::windows_core::imp::interface_hierarchy!(
    ISelectionItemProvider,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::RuntimeType for ISelectionItemProvider {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_interface::<Self>();
}
unsafe impl ::windows_core::Interface for ISelectionItemProvider {
    type Vtable = ISelectionItemProvider_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ISelectionItemProvider {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xc9dfdd81_d4ac_5d31_be7f_24fab16060e4);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISelectionItemProvider_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub IsSelected: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SelectionContainer: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub AddToSelection:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub RemoveFromSelection:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Select:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ISelectionProvider(::windows_core::IUnknown);
impl ISelectionProvider {
    pub fn CanSelectMultiple(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CanSelectMultiple)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn IsSelectionRequired(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsSelectionRequired)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn GetSelection(
        &self,
    ) -> ::windows_core::Result<::windows_core::Array<IRawElementProviderSimple>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows_core::Interface::vtable(this).GetSelection)(
                ::windows_core::Interface::as_raw(this),
                ::windows_core::Array::<IRawElementProviderSimple>::set_abi_len(
                    ::std::mem::transmute(&mut result__),
                ),
                result__.as_mut_ptr() as *mut _ as _,
            )
            .and_then(|| result__.assume_init())
        }
    }
}
::windows_core::imp::interface_hierarchy!(
    ISelectionProvider,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::RuntimeType for ISelectionProvider {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_interface::<Self>();
}
unsafe impl ::windows_core::Interface for ISelectionProvider {
    type Vtable = ISelectionProvider_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ISelectionProvider {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x80d56d4e_0052_541f_9411_9d1778b3bfca);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISelectionProvider_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CanSelectMultiple: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub IsSelectionRequired: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub GetSelection: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result_size__: *mut u32,
        result__: *mut *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ISpreadsheetItemProvider(::windows_core::IUnknown);
impl ISpreadsheetItemProvider {
    pub fn Formula(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Formula)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn GetAnnotationObjects(
        &self,
    ) -> ::windows_core::Result<::windows_core::Array<IRawElementProviderSimple>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows_core::Interface::vtable(this).GetAnnotationObjects)(
                ::windows_core::Interface::as_raw(this),
                ::windows_core::Array::<IRawElementProviderSimple>::set_abi_len(
                    ::std::mem::transmute(&mut result__),
                ),
                result__.as_mut_ptr() as *mut _ as _,
            )
            .and_then(|| result__.assume_init())
        }
    }
    pub fn GetAnnotationTypes(
        &self,
    ) -> ::windows_core::Result<::windows_core::Array<super::AnnotationType>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows_core::Interface::vtable(this).GetAnnotationTypes)(
                ::windows_core::Interface::as_raw(this),
                ::windows_core::Array::<super::AnnotationType>::set_abi_len(::std::mem::transmute(
                    &mut result__,
                )),
                result__.as_mut_ptr() as *mut _ as _,
            )
            .and_then(|| result__.assume_init())
        }
    }
}
::windows_core::imp::interface_hierarchy!(
    ISpreadsheetItemProvider,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::RuntimeType for ISpreadsheetItemProvider {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_interface::<Self>();
}
unsafe impl ::windows_core::Interface for ISpreadsheetItemProvider {
    type Vtable = ISpreadsheetItemProvider_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ISpreadsheetItemProvider {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x51c1ce89_b21f_592c_8768_0accdefd5738);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpreadsheetItemProvider_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Formula: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub GetAnnotationObjects: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result_size__: *mut u32,
        result__: *mut *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub GetAnnotationTypes: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result_size__: *mut u32,
        result__: *mut *mut super::AnnotationType,
    ) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ISpreadsheetProvider(::windows_core::IUnknown);
impl ISpreadsheetProvider {
    pub fn GetItemByName(
        &self,
        name: &::windows_core::HSTRING,
    ) -> ::windows_core::Result<IRawElementProviderSimple> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetItemByName)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(name),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(
    ISpreadsheetProvider,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::RuntimeType for ISpreadsheetProvider {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_interface::<Self>();
}
unsafe impl ::windows_core::Interface for ISpreadsheetProvider {
    type Vtable = ISpreadsheetProvider_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ISpreadsheetProvider {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x1ff41bac_d9e3_5e48_b5f8_9eab0fb2d9d8);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpreadsheetProvider_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub GetItemByName: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        name: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IStylesProvider(::windows_core::IUnknown);
impl IStylesProvider {
    pub fn ExtendedProperties(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ExtendedProperties)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn FillColor(&self) -> ::windows_core::Result<::windows::UI::Color> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FillColor)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn FillPatternColor(&self) -> ::windows_core::Result<::windows::UI::Color> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FillPatternColor)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn FillPatternStyle(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FillPatternStyle)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn Shape(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Shape)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn StyleId(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StyleId)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn StyleName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StyleName)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(
    IStylesProvider,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::RuntimeType for IStylesProvider {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_interface::<Self>();
}
unsafe impl ::windows_core::Interface for IStylesProvider {
    type Vtable = IStylesProvider_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IStylesProvider {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xd8895839_0048_54de_9c1f_152de6665e80);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStylesProvider_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ExtendedProperties: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_UI")]
    pub FillColor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI"))]
    FillColor: usize,
    #[cfg(feature = "Windows_UI")]
    pub FillPatternColor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI"))]
    FillPatternColor: usize,
    pub FillPatternStyle: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub Shape: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub StyleId: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut i32,
    ) -> ::windows_core::HRESULT,
    pub StyleName: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ISynchronizedInputProvider(::windows_core::IUnknown);
impl ISynchronizedInputProvider {
    pub fn Cancel(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).Cancel)(::windows_core::Interface::as_raw(
                this,
            ))
            .ok()
        }
    }
    pub fn StartListening(
        &self,
        inputtype: super::SynchronizedInputType,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).StartListening)(
                ::windows_core::Interface::as_raw(this),
                inputtype,
            )
            .ok()
        }
    }
}
::windows_core::imp::interface_hierarchy!(
    ISynchronizedInputProvider,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::RuntimeType for ISynchronizedInputProvider {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_interface::<Self>();
}
unsafe impl ::windows_core::Interface for ISynchronizedInputProvider {
    type Vtable = ISynchronizedInputProvider_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ISynchronizedInputProvider {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xc5615613_936d_5289_a190_e82057e0ff5a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISynchronizedInputProvider_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Cancel:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub StartListening: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        inputtype: super::SynchronizedInputType,
    ) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ITableItemProvider(::windows_core::IUnknown);
impl ITableItemProvider {
    pub fn GetColumnHeaderItems(
        &self,
    ) -> ::windows_core::Result<::windows_core::Array<IRawElementProviderSimple>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows_core::Interface::vtable(this).GetColumnHeaderItems)(
                ::windows_core::Interface::as_raw(this),
                ::windows_core::Array::<IRawElementProviderSimple>::set_abi_len(
                    ::std::mem::transmute(&mut result__),
                ),
                result__.as_mut_ptr() as *mut _ as _,
            )
            .and_then(|| result__.assume_init())
        }
    }
    pub fn GetRowHeaderItems(
        &self,
    ) -> ::windows_core::Result<::windows_core::Array<IRawElementProviderSimple>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows_core::Interface::vtable(this).GetRowHeaderItems)(
                ::windows_core::Interface::as_raw(this),
                ::windows_core::Array::<IRawElementProviderSimple>::set_abi_len(
                    ::std::mem::transmute(&mut result__),
                ),
                result__.as_mut_ptr() as *mut _ as _,
            )
            .and_then(|| result__.assume_init())
        }
    }
}
::windows_core::imp::interface_hierarchy!(
    ITableItemProvider,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::RuntimeType for ITableItemProvider {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_interface::<Self>();
}
unsafe impl ::windows_core::Interface for ITableItemProvider {
    type Vtable = ITableItemProvider_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ITableItemProvider {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x6ce6f038_54d4_5553_a4ad_03cbcf358197);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITableItemProvider_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub GetColumnHeaderItems: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result_size__: *mut u32,
        result__: *mut *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub GetRowHeaderItems: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result_size__: *mut u32,
        result__: *mut *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ITableProvider(::windows_core::IUnknown);
impl ITableProvider {
    pub fn RowOrColumnMajor(&self) -> ::windows_core::Result<super::RowOrColumnMajor> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RowOrColumnMajor)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn GetColumnHeaders(
        &self,
    ) -> ::windows_core::Result<::windows_core::Array<IRawElementProviderSimple>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows_core::Interface::vtable(this).GetColumnHeaders)(
                ::windows_core::Interface::as_raw(this),
                ::windows_core::Array::<IRawElementProviderSimple>::set_abi_len(
                    ::std::mem::transmute(&mut result__),
                ),
                result__.as_mut_ptr() as *mut _ as _,
            )
            .and_then(|| result__.assume_init())
        }
    }
    pub fn GetRowHeaders(
        &self,
    ) -> ::windows_core::Result<::windows_core::Array<IRawElementProviderSimple>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows_core::Interface::vtable(this).GetRowHeaders)(
                ::windows_core::Interface::as_raw(this),
                ::windows_core::Array::<IRawElementProviderSimple>::set_abi_len(
                    ::std::mem::transmute(&mut result__),
                ),
                result__.as_mut_ptr() as *mut _ as _,
            )
            .and_then(|| result__.assume_init())
        }
    }
}
::windows_core::imp::interface_hierarchy!(
    ITableProvider,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::RuntimeType for ITableProvider {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_interface::<Self>();
}
unsafe impl ::windows_core::Interface for ITableProvider {
    type Vtable = ITableProvider_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ITableProvider {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x9aba6724_b22d_5db8_8abb_81f911f18af2);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITableProvider_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub RowOrColumnMajor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::RowOrColumnMajor,
    ) -> ::windows_core::HRESULT,
    pub GetColumnHeaders: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result_size__: *mut u32,
        result__: *mut *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub GetRowHeaders: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result_size__: *mut u32,
        result__: *mut *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ITextChildProvider(::windows_core::IUnknown);
impl ITextChildProvider {
    pub fn TextContainer(&self) -> ::windows_core::Result<IRawElementProviderSimple> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TextContainer)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn TextRange(&self) -> ::windows_core::Result<ITextRangeProvider> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TextRange)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(
    ITextChildProvider,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::RuntimeType for ITextChildProvider {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_interface::<Self>();
}
unsafe impl ::windows_core::Interface for ITextChildProvider {
    type Vtable = ITextChildProvider_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ITextChildProvider {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x7c72e55f_f75d_5522_aeb5_c1f82c32933b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextChildProvider_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub TextContainer: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub TextRange: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ITextEditProvider(::windows_core::IUnknown);
impl ITextEditProvider {
    pub fn GetActiveComposition(&self) -> ::windows_core::Result<ITextRangeProvider> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetActiveComposition)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn GetConversionTarget(&self) -> ::windows_core::Result<ITextRangeProvider> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetConversionTarget)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn DocumentRange(&self) -> ::windows_core::Result<ITextRangeProvider> {
        let this = &::windows_core::ComInterface::cast::<ITextProvider>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DocumentRange)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SupportedTextSelection(&self) -> ::windows_core::Result<super::SupportedTextSelection> {
        let this = &::windows_core::ComInterface::cast::<ITextProvider>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SupportedTextSelection)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn GetSelection(
        &self,
    ) -> ::windows_core::Result<::windows_core::Array<ITextRangeProvider>> {
        let this = &::windows_core::ComInterface::cast::<ITextProvider>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows_core::Interface::vtable(this).GetSelection)(
                ::windows_core::Interface::as_raw(this),
                ::windows_core::Array::<ITextRangeProvider>::set_abi_len(::std::mem::transmute(
                    &mut result__,
                )),
                result__.as_mut_ptr() as *mut _ as _,
            )
            .and_then(|| result__.assume_init())
        }
    }
    pub fn GetVisibleRanges(
        &self,
    ) -> ::windows_core::Result<::windows_core::Array<ITextRangeProvider>> {
        let this = &::windows_core::ComInterface::cast::<ITextProvider>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows_core::Interface::vtable(this).GetVisibleRanges)(
                ::windows_core::Interface::as_raw(this),
                ::windows_core::Array::<ITextRangeProvider>::set_abi_len(::std::mem::transmute(
                    &mut result__,
                )),
                result__.as_mut_ptr() as *mut _ as _,
            )
            .and_then(|| result__.assume_init())
        }
    }
    pub fn RangeFromChild<P0>(&self, childelement: P0) -> ::windows_core::Result<ITextRangeProvider>
    where
        P0: ::windows_core::IntoParam<IRawElementProviderSimple>,
    {
        let this = &::windows_core::ComInterface::cast::<ITextProvider>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RangeFromChild)(
                ::windows_core::Interface::as_raw(this),
                childelement.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RangeFromPoint(
        &self,
        screenlocation: ::windows::Foundation::Point,
    ) -> ::windows_core::Result<ITextRangeProvider> {
        let this = &::windows_core::ComInterface::cast::<ITextProvider>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RangeFromPoint)(
                ::windows_core::Interface::as_raw(this),
                screenlocation,
                &mut result__,
            )
            .from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(
    ITextEditProvider,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<ITextProvider> for ITextEditProvider {}
impl ::windows_core::RuntimeType for ITextEditProvider {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_interface::<Self>();
}
unsafe impl ::windows_core::Interface for ITextEditProvider {
    type Vtable = ITextEditProvider_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ITextEditProvider {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x7f09bbe8_bea7_5dd3_ba6b_28dbb402fad4);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextEditProvider_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub GetActiveComposition: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub GetConversionTarget: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ITextProvider(::windows_core::IUnknown);
impl ITextProvider {
    pub fn DocumentRange(&self) -> ::windows_core::Result<ITextRangeProvider> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DocumentRange)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SupportedTextSelection(&self) -> ::windows_core::Result<super::SupportedTextSelection> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SupportedTextSelection)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn GetSelection(
        &self,
    ) -> ::windows_core::Result<::windows_core::Array<ITextRangeProvider>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows_core::Interface::vtable(this).GetSelection)(
                ::windows_core::Interface::as_raw(this),
                ::windows_core::Array::<ITextRangeProvider>::set_abi_len(::std::mem::transmute(
                    &mut result__,
                )),
                result__.as_mut_ptr() as *mut _ as _,
            )
            .and_then(|| result__.assume_init())
        }
    }
    pub fn GetVisibleRanges(
        &self,
    ) -> ::windows_core::Result<::windows_core::Array<ITextRangeProvider>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows_core::Interface::vtable(this).GetVisibleRanges)(
                ::windows_core::Interface::as_raw(this),
                ::windows_core::Array::<ITextRangeProvider>::set_abi_len(::std::mem::transmute(
                    &mut result__,
                )),
                result__.as_mut_ptr() as *mut _ as _,
            )
            .and_then(|| result__.assume_init())
        }
    }
    pub fn RangeFromChild<P0>(&self, childelement: P0) -> ::windows_core::Result<ITextRangeProvider>
    where
        P0: ::windows_core::IntoParam<IRawElementProviderSimple>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RangeFromChild)(
                ::windows_core::Interface::as_raw(this),
                childelement.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RangeFromPoint(
        &self,
        screenlocation: ::windows::Foundation::Point,
    ) -> ::windows_core::Result<ITextRangeProvider> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RangeFromPoint)(
                ::windows_core::Interface::as_raw(this),
                screenlocation,
                &mut result__,
            )
            .from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(
    ITextProvider,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::RuntimeType for ITextProvider {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_interface::<Self>();
}
unsafe impl ::windows_core::Interface for ITextProvider {
    type Vtable = ITextProvider_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ITextProvider {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x37e7dce6_fe7a_56a7_a47a_9462872c67ef);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextProvider_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub DocumentRange: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SupportedTextSelection: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::SupportedTextSelection,
    ) -> ::windows_core::HRESULT,
    pub GetSelection: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result_size__: *mut u32,
        result__: *mut *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub GetVisibleRanges: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result_size__: *mut u32,
        result__: *mut *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub RangeFromChild: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        childelement: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_Foundation")]
    pub RangeFromPoint: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        screenlocation: ::windows::Foundation::Point,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RangeFromPoint: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ITextProvider2(::windows_core::IUnknown);
impl ITextProvider2 {
    pub fn RangeFromAnnotation<P0>(
        &self,
        annotationelement: P0,
    ) -> ::windows_core::Result<ITextRangeProvider>
    where
        P0: ::windows_core::IntoParam<IRawElementProviderSimple>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RangeFromAnnotation)(
                ::windows_core::Interface::as_raw(this),
                annotationelement.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn GetCaretRange(&self, isactive: &mut bool) -> ::windows_core::Result<ITextRangeProvider> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetCaretRange)(
                ::windows_core::Interface::as_raw(this),
                isactive,
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn DocumentRange(&self) -> ::windows_core::Result<ITextRangeProvider> {
        let this = &::windows_core::ComInterface::cast::<ITextProvider>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DocumentRange)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SupportedTextSelection(&self) -> ::windows_core::Result<super::SupportedTextSelection> {
        let this = &::windows_core::ComInterface::cast::<ITextProvider>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SupportedTextSelection)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn GetSelection(
        &self,
    ) -> ::windows_core::Result<::windows_core::Array<ITextRangeProvider>> {
        let this = &::windows_core::ComInterface::cast::<ITextProvider>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows_core::Interface::vtable(this).GetSelection)(
                ::windows_core::Interface::as_raw(this),
                ::windows_core::Array::<ITextRangeProvider>::set_abi_len(::std::mem::transmute(
                    &mut result__,
                )),
                result__.as_mut_ptr() as *mut _ as _,
            )
            .and_then(|| result__.assume_init())
        }
    }
    pub fn GetVisibleRanges(
        &self,
    ) -> ::windows_core::Result<::windows_core::Array<ITextRangeProvider>> {
        let this = &::windows_core::ComInterface::cast::<ITextProvider>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows_core::Interface::vtable(this).GetVisibleRanges)(
                ::windows_core::Interface::as_raw(this),
                ::windows_core::Array::<ITextRangeProvider>::set_abi_len(::std::mem::transmute(
                    &mut result__,
                )),
                result__.as_mut_ptr() as *mut _ as _,
            )
            .and_then(|| result__.assume_init())
        }
    }
    pub fn RangeFromChild<P0>(&self, childelement: P0) -> ::windows_core::Result<ITextRangeProvider>
    where
        P0: ::windows_core::IntoParam<IRawElementProviderSimple>,
    {
        let this = &::windows_core::ComInterface::cast::<ITextProvider>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RangeFromChild)(
                ::windows_core::Interface::as_raw(this),
                childelement.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RangeFromPoint(
        &self,
        screenlocation: ::windows::Foundation::Point,
    ) -> ::windows_core::Result<ITextRangeProvider> {
        let this = &::windows_core::ComInterface::cast::<ITextProvider>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RangeFromPoint)(
                ::windows_core::Interface::as_raw(this),
                screenlocation,
                &mut result__,
            )
            .from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(
    ITextProvider2,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<ITextProvider> for ITextProvider2 {}
impl ::windows_core::RuntimeType for ITextProvider2 {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_interface::<Self>();
}
unsafe impl ::windows_core::Interface for ITextProvider2 {
    type Vtable = ITextProvider2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ITextProvider2 {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x6844f012_c7e6_5763_ba04_5b6db910cd34);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextProvider2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub RangeFromAnnotation: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        annotationelement: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub GetCaretRange: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        isactive: *mut bool,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ITextRangeProvider(::windows_core::IUnknown);
impl ITextRangeProvider {
    pub fn Clone(&self) -> ::windows_core::Result<ITextRangeProvider> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Clone)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn Compare<P0>(&self, textrangeprovider: P0) -> ::windows_core::Result<bool>
    where
        P0: ::windows_core::TryIntoParam<ITextRangeProvider>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Compare)(
                ::windows_core::Interface::as_raw(this),
                textrangeprovider.try_into_param()?.abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Automation_Text\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Automation_Text")]
    pub fn CompareEndpoints<P0>(
        &self,
        endpoint: super::Text::TextPatternRangeEndpoint,
        textrangeprovider: P0,
        targetendpoint: super::Text::TextPatternRangeEndpoint,
    ) -> ::windows_core::Result<i32>
    where
        P0: ::windows_core::TryIntoParam<ITextRangeProvider>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CompareEndpoints)(
                ::windows_core::Interface::as_raw(this),
                endpoint,
                textrangeprovider.try_into_param()?.abi(),
                targetendpoint,
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Automation_Text\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Automation_Text")]
    pub fn ExpandToEnclosingUnit(&self, unit: super::Text::TextUnit) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).ExpandToEnclosingUnit)(
                ::windows_core::Interface::as_raw(this),
                unit,
            )
            .ok()
        }
    }
    pub fn FindAttribute<P0>(
        &self,
        attributeid: i32,
        value: P0,
        backward: bool,
    ) -> ::windows_core::Result<ITextRangeProvider>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FindAttribute)(
                ::windows_core::Interface::as_raw(this),
                attributeid,
                value.into_param().abi(),
                backward,
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn FindText(
        &self,
        text: &::windows_core::HSTRING,
        backward: bool,
        ignorecase: bool,
    ) -> ::windows_core::Result<ITextRangeProvider> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FindText)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(text),
                backward,
                ignorecase,
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn GetAttributeValue(
        &self,
        attributeid: i32,
    ) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetAttributeValue)(
                ::windows_core::Interface::as_raw(this),
                attributeid,
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn GetBoundingRectangles(
        &self,
        returnvalue: &mut ::windows_core::Array<f64>,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).GetBoundingRectangles)(
                ::windows_core::Interface::as_raw(this),
                returnvalue.set_abi_len(),
                returnvalue as *mut _ as _,
            )
            .ok()
        }
    }
    pub fn GetEnclosingElement(&self) -> ::windows_core::Result<IRawElementProviderSimple> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetEnclosingElement)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn GetText(&self, maxlength: i32) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetText)(
                ::windows_core::Interface::as_raw(this),
                maxlength,
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Automation_Text\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Automation_Text")]
    pub fn Move(&self, unit: super::Text::TextUnit, count: i32) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Move)(
                ::windows_core::Interface::as_raw(this),
                unit,
                count,
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Automation_Text\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Automation_Text")]
    pub fn MoveEndpointByUnit(
        &self,
        endpoint: super::Text::TextPatternRangeEndpoint,
        unit: super::Text::TextUnit,
        count: i32,
    ) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MoveEndpointByUnit)(
                ::windows_core::Interface::as_raw(this),
                endpoint,
                unit,
                count,
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Automation_Text\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Automation_Text")]
    pub fn MoveEndpointByRange<P0>(
        &self,
        endpoint: super::Text::TextPatternRangeEndpoint,
        textrangeprovider: P0,
        targetendpoint: super::Text::TextPatternRangeEndpoint,
    ) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<ITextRangeProvider>,
    {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).MoveEndpointByRange)(
                ::windows_core::Interface::as_raw(this),
                endpoint,
                textrangeprovider.try_into_param()?.abi(),
                targetendpoint,
            )
            .ok()
        }
    }
    pub fn Select(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).Select)(::windows_core::Interface::as_raw(
                this,
            ))
            .ok()
        }
    }
    pub fn AddToSelection(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).AddToSelection)(
                ::windows_core::Interface::as_raw(this),
            )
            .ok()
        }
    }
    pub fn RemoveFromSelection(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveFromSelection)(
                ::windows_core::Interface::as_raw(this),
            )
            .ok()
        }
    }
    pub fn ScrollIntoView(&self, aligntotop: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).ScrollIntoView)(
                ::windows_core::Interface::as_raw(this),
                aligntotop,
            )
            .ok()
        }
    }
    pub fn GetChildren(
        &self,
    ) -> ::windows_core::Result<::windows_core::Array<IRawElementProviderSimple>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows_core::Interface::vtable(this).GetChildren)(
                ::windows_core::Interface::as_raw(this),
                ::windows_core::Array::<IRawElementProviderSimple>::set_abi_len(
                    ::std::mem::transmute(&mut result__),
                ),
                result__.as_mut_ptr() as *mut _ as _,
            )
            .and_then(|| result__.assume_init())
        }
    }
}
::windows_core::imp::interface_hierarchy!(
    ITextRangeProvider,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::RuntimeType for ITextRangeProvider {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_interface::<Self>();
}
unsafe impl ::windows_core::Interface for ITextRangeProvider {
    type Vtable = ITextRangeProvider_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ITextRangeProvider {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x84210361_6ce2_5084_bf3b_28afa6e9851f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextRangeProvider_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Clone: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub Compare: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        textrangeprovider: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Microsoft_UI_Xaml_Automation_Text")]
    pub CompareEndpoints: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        endpoint: super::Text::TextPatternRangeEndpoint,
        textrangeprovider: *mut ::core::ffi::c_void,
        targetendpoint: super::Text::TextPatternRangeEndpoint,
        result__: *mut i32,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Microsoft_UI_Xaml_Automation_Text"))]
    CompareEndpoints: usize,
    #[cfg(feature = "Microsoft_UI_Xaml_Automation_Text")]
    pub ExpandToEnclosingUnit: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        unit: super::Text::TextUnit,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Microsoft_UI_Xaml_Automation_Text"))]
    ExpandToEnclosingUnit: usize,
    pub FindAttribute: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        attributeid: i32,
        value: *mut ::core::ffi::c_void,
        backward: bool,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub FindText: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        text: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        backward: bool,
        ignorecase: bool,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub GetAttributeValue: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        attributeid: i32,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub GetBoundingRectangles: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        returnValue_array_size: *mut u32,
        returnvalue: *mut *mut f64,
    ) -> ::windows_core::HRESULT,
    pub GetEnclosingElement: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub GetText: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        maxlength: i32,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Microsoft_UI_Xaml_Automation_Text")]
    pub Move: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        unit: super::Text::TextUnit,
        count: i32,
        result__: *mut i32,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Microsoft_UI_Xaml_Automation_Text"))]
    Move: usize,
    #[cfg(feature = "Microsoft_UI_Xaml_Automation_Text")]
    pub MoveEndpointByUnit: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        endpoint: super::Text::TextPatternRangeEndpoint,
        unit: super::Text::TextUnit,
        count: i32,
        result__: *mut i32,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Microsoft_UI_Xaml_Automation_Text"))]
    MoveEndpointByUnit: usize,
    #[cfg(feature = "Microsoft_UI_Xaml_Automation_Text")]
    pub MoveEndpointByRange: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        endpoint: super::Text::TextPatternRangeEndpoint,
        textrangeprovider: *mut ::core::ffi::c_void,
        targetendpoint: super::Text::TextPatternRangeEndpoint,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Microsoft_UI_Xaml_Automation_Text"))]
    MoveEndpointByRange: usize,
    pub Select:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub AddToSelection:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub RemoveFromSelection:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub ScrollIntoView: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        aligntotop: bool,
    ) -> ::windows_core::HRESULT,
    pub GetChildren: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result_size__: *mut u32,
        result__: *mut *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ITextRangeProvider2(::windows_core::IUnknown);
impl ITextRangeProvider2 {
    pub fn ShowContextMenu(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).ShowContextMenu)(
                ::windows_core::Interface::as_raw(this),
            )
            .ok()
        }
    }
    pub fn Clone(&self) -> ::windows_core::Result<ITextRangeProvider> {
        let this = &::windows_core::ComInterface::cast::<ITextRangeProvider>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Clone)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn Compare<P0>(&self, textrangeprovider: P0) -> ::windows_core::Result<bool>
    where
        P0: ::windows_core::TryIntoParam<ITextRangeProvider>,
    {
        let this = &::windows_core::ComInterface::cast::<ITextRangeProvider>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Compare)(
                ::windows_core::Interface::as_raw(this),
                textrangeprovider.try_into_param()?.abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Automation_Text\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Automation_Text")]
    pub fn CompareEndpoints<P0>(
        &self,
        endpoint: super::Text::TextPatternRangeEndpoint,
        textrangeprovider: P0,
        targetendpoint: super::Text::TextPatternRangeEndpoint,
    ) -> ::windows_core::Result<i32>
    where
        P0: ::windows_core::TryIntoParam<ITextRangeProvider>,
    {
        let this = &::windows_core::ComInterface::cast::<ITextRangeProvider>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CompareEndpoints)(
                ::windows_core::Interface::as_raw(this),
                endpoint,
                textrangeprovider.try_into_param()?.abi(),
                targetendpoint,
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Automation_Text\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Automation_Text")]
    pub fn ExpandToEnclosingUnit(&self, unit: super::Text::TextUnit) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextRangeProvider>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).ExpandToEnclosingUnit)(
                ::windows_core::Interface::as_raw(this),
                unit,
            )
            .ok()
        }
    }
    pub fn FindAttribute<P0>(
        &self,
        attributeid: i32,
        value: P0,
        backward: bool,
    ) -> ::windows_core::Result<ITextRangeProvider>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = &::windows_core::ComInterface::cast::<ITextRangeProvider>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FindAttribute)(
                ::windows_core::Interface::as_raw(this),
                attributeid,
                value.into_param().abi(),
                backward,
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn FindText(
        &self,
        text: &::windows_core::HSTRING,
        backward: bool,
        ignorecase: bool,
    ) -> ::windows_core::Result<ITextRangeProvider> {
        let this = &::windows_core::ComInterface::cast::<ITextRangeProvider>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FindText)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(text),
                backward,
                ignorecase,
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn GetAttributeValue(
        &self,
        attributeid: i32,
    ) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = &::windows_core::ComInterface::cast::<ITextRangeProvider>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetAttributeValue)(
                ::windows_core::Interface::as_raw(this),
                attributeid,
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn GetBoundingRectangles(
        &self,
        returnvalue: &mut ::windows_core::Array<f64>,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextRangeProvider>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).GetBoundingRectangles)(
                ::windows_core::Interface::as_raw(this),
                returnvalue.set_abi_len(),
                returnvalue as *mut _ as _,
            )
            .ok()
        }
    }
    pub fn GetEnclosingElement(&self) -> ::windows_core::Result<IRawElementProviderSimple> {
        let this = &::windows_core::ComInterface::cast::<ITextRangeProvider>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetEnclosingElement)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn GetText(&self, maxlength: i32) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<ITextRangeProvider>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetText)(
                ::windows_core::Interface::as_raw(this),
                maxlength,
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Automation_Text\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Automation_Text")]
    pub fn Move(&self, unit: super::Text::TextUnit, count: i32) -> ::windows_core::Result<i32> {
        let this = &::windows_core::ComInterface::cast::<ITextRangeProvider>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Move)(
                ::windows_core::Interface::as_raw(this),
                unit,
                count,
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Automation_Text\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Automation_Text")]
    pub fn MoveEndpointByUnit(
        &self,
        endpoint: super::Text::TextPatternRangeEndpoint,
        unit: super::Text::TextUnit,
        count: i32,
    ) -> ::windows_core::Result<i32> {
        let this = &::windows_core::ComInterface::cast::<ITextRangeProvider>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MoveEndpointByUnit)(
                ::windows_core::Interface::as_raw(this),
                endpoint,
                unit,
                count,
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Xaml_Automation_Text\"`"]
    #[cfg(feature = "Microsoft_UI_Xaml_Automation_Text")]
    pub fn MoveEndpointByRange<P0>(
        &self,
        endpoint: super::Text::TextPatternRangeEndpoint,
        textrangeprovider: P0,
        targetendpoint: super::Text::TextPatternRangeEndpoint,
    ) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<ITextRangeProvider>,
    {
        let this = &::windows_core::ComInterface::cast::<ITextRangeProvider>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).MoveEndpointByRange)(
                ::windows_core::Interface::as_raw(this),
                endpoint,
                textrangeprovider.try_into_param()?.abi(),
                targetendpoint,
            )
            .ok()
        }
    }
    pub fn Select(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextRangeProvider>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).Select)(::windows_core::Interface::as_raw(
                this,
            ))
            .ok()
        }
    }
    pub fn AddToSelection(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextRangeProvider>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).AddToSelection)(
                ::windows_core::Interface::as_raw(this),
            )
            .ok()
        }
    }
    pub fn RemoveFromSelection(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextRangeProvider>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveFromSelection)(
                ::windows_core::Interface::as_raw(this),
            )
            .ok()
        }
    }
    pub fn ScrollIntoView(&self, aligntotop: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITextRangeProvider>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).ScrollIntoView)(
                ::windows_core::Interface::as_raw(this),
                aligntotop,
            )
            .ok()
        }
    }
    pub fn GetChildren(
        &self,
    ) -> ::windows_core::Result<::windows_core::Array<IRawElementProviderSimple>> {
        let this = &::windows_core::ComInterface::cast::<ITextRangeProvider>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows_core::Interface::vtable(this).GetChildren)(
                ::windows_core::Interface::as_raw(this),
                ::windows_core::Array::<IRawElementProviderSimple>::set_abi_len(
                    ::std::mem::transmute(&mut result__),
                ),
                result__.as_mut_ptr() as *mut _ as _,
            )
            .and_then(|| result__.assume_init())
        }
    }
}
::windows_core::imp::interface_hierarchy!(
    ITextRangeProvider2,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<ITextRangeProvider> for ITextRangeProvider2 {}
impl ::windows_core::RuntimeType for ITextRangeProvider2 {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_interface::<Self>();
}
unsafe impl ::windows_core::Interface for ITextRangeProvider2 {
    type Vtable = ITextRangeProvider2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ITextRangeProvider2 {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x34d4a80e_36bb_5362_a53b_490428a8b367);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextRangeProvider2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ShowContextMenu:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IToggleProvider(::windows_core::IUnknown);
impl IToggleProvider {
    pub fn ToggleState(&self) -> ::windows_core::Result<super::ToggleState> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ToggleState)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn Toggle(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).Toggle)(::windows_core::Interface::as_raw(
                this,
            ))
            .ok()
        }
    }
}
::windows_core::imp::interface_hierarchy!(
    IToggleProvider,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::RuntimeType for IToggleProvider {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_interface::<Self>();
}
unsafe impl ::windows_core::Interface for IToggleProvider {
    type Vtable = IToggleProvider_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IToggleProvider {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x021080c2_30a9_52ef_bc32_2b79847b6ba7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IToggleProvider_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ToggleState: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::ToggleState,
    ) -> ::windows_core::HRESULT,
    pub Toggle:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ITransformProvider(::windows_core::IUnknown);
impl ITransformProvider {
    pub fn CanMove(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CanMove)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn CanResize(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CanResize)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn CanRotate(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CanRotate)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn Move(&self, x: f64, y: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).Move)(
                ::windows_core::Interface::as_raw(this),
                x,
                y,
            )
            .ok()
        }
    }
    pub fn Resize(&self, width: f64, height: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).Resize)(
                ::windows_core::Interface::as_raw(this),
                width,
                height,
            )
            .ok()
        }
    }
    pub fn Rotate(&self, degrees: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).Rotate)(
                ::windows_core::Interface::as_raw(this),
                degrees,
            )
            .ok()
        }
    }
}
::windows_core::imp::interface_hierarchy!(
    ITransformProvider,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::RuntimeType for ITransformProvider {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_interface::<Self>();
}
unsafe impl ::windows_core::Interface for ITransformProvider {
    type Vtable = ITransformProvider_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ITransformProvider {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x6fd76988_8f52_5ef2_a826_9c8c4951c911);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransformProvider_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CanMove: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub CanResize: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub CanRotate: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub Move: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        x: f64,
        y: f64,
    ) -> ::windows_core::HRESULT,
    pub Resize: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        width: f64,
        height: f64,
    ) -> ::windows_core::HRESULT,
    pub Rotate: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        degrees: f64,
    ) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ITransformProvider2(::windows_core::IUnknown);
impl ITransformProvider2 {
    pub fn CanZoom(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CanZoom)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn ZoomLevel(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ZoomLevel)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn MaxZoom(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MaxZoom)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn MinZoom(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MinZoom)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn Zoom(&self, zoom: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).Zoom)(
                ::windows_core::Interface::as_raw(this),
                zoom,
            )
            .ok()
        }
    }
    pub fn ZoomByUnit(&self, zoomunit: super::ZoomUnit) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).ZoomByUnit)(
                ::windows_core::Interface::as_raw(this),
                zoomunit,
            )
            .ok()
        }
    }
    pub fn CanMove(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<ITransformProvider>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CanMove)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn CanResize(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<ITransformProvider>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CanResize)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn CanRotate(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<ITransformProvider>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CanRotate)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn Move(&self, x: f64, y: f64) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITransformProvider>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).Move)(
                ::windows_core::Interface::as_raw(this),
                x,
                y,
            )
            .ok()
        }
    }
    pub fn Resize(&self, width: f64, height: f64) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITransformProvider>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).Resize)(
                ::windows_core::Interface::as_raw(this),
                width,
                height,
            )
            .ok()
        }
    }
    pub fn Rotate(&self, degrees: f64) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITransformProvider>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).Rotate)(
                ::windows_core::Interface::as_raw(this),
                degrees,
            )
            .ok()
        }
    }
}
::windows_core::imp::interface_hierarchy!(
    ITransformProvider2,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<ITransformProvider> for ITransformProvider2 {}
impl ::windows_core::RuntimeType for ITransformProvider2 {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_interface::<Self>();
}
unsafe impl ::windows_core::Interface for ITransformProvider2 {
    type Vtable = ITransformProvider2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ITransformProvider2 {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x7d91d02d_8401_5cf8_bbc4_47391a524215);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransformProvider2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CanZoom: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub ZoomLevel: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub MaxZoom: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub MinZoom: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub Zoom: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        zoom: f64,
    ) -> ::windows_core::HRESULT,
    pub ZoomByUnit: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        zoomunit: super::ZoomUnit,
    ) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IValueProvider(::windows_core::IUnknown);
impl IValueProvider {
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
    pub fn Value(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Value)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetValue(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetValue)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
}
::windows_core::imp::interface_hierarchy!(
    IValueProvider,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::RuntimeType for IValueProvider {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_interface::<Self>();
}
unsafe impl ::windows_core::Interface for IValueProvider {
    type Vtable = IValueProvider_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IValueProvider {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x984f11cf_4611_588e_b52e_b96a12322c71);
}
#[repr(C)]
#[doc(hidden)]
pub struct IValueProvider_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub IsReadOnly: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub Value: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub SetValue: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IVirtualizedItemProvider(::windows_core::IUnknown);
impl IVirtualizedItemProvider {
    pub fn Realize(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).Realize)(::windows_core::Interface::as_raw(
                this,
            ))
            .ok()
        }
    }
}
::windows_core::imp::interface_hierarchy!(
    IVirtualizedItemProvider,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::RuntimeType for IVirtualizedItemProvider {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_interface::<Self>();
}
unsafe impl ::windows_core::Interface for IVirtualizedItemProvider {
    type Vtable = IVirtualizedItemProvider_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IVirtualizedItemProvider {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x098f858a_2e63_5985_ab87_f8ebdb1c5740);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVirtualizedItemProvider_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Realize:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWindowProvider(::windows_core::IUnknown);
impl IWindowProvider {
    pub fn IsModal(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsModal)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn IsTopmost(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsTopmost)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn Maximizable(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Maximizable)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn Minimizable(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Minimizable)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn InteractionState(&self) -> ::windows_core::Result<super::WindowInteractionState> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).InteractionState)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn VisualState(&self) -> ::windows_core::Result<super::WindowVisualState> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).VisualState)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this))
                .ok()
        }
    }
    pub fn SetVisualState(&self, state: super::WindowVisualState) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetVisualState)(
                ::windows_core::Interface::as_raw(this),
                state,
            )
            .ok()
        }
    }
    pub fn WaitForInputIdle(&self, milliseconds: i32) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).WaitForInputIdle)(
                ::windows_core::Interface::as_raw(this),
                milliseconds,
                &mut result__,
            )
            .from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(
    IWindowProvider,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::RuntimeType for IWindowProvider {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_interface::<Self>();
}
unsafe impl ::windows_core::Interface for IWindowProvider {
    type Vtable = IWindowProvider_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWindowProvider {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x83f1df99_9ddf_575e_a651_2ee657fd16e0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowProvider_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub IsModal: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub IsTopmost: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub Maximizable: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub Minimizable: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub InteractionState: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::WindowInteractionState,
    ) -> ::windows_core::HRESULT,
    pub VisualState: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::WindowVisualState,
    ) -> ::windows_core::HRESULT,
    pub Close: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetVisualState: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        state: super::WindowVisualState,
    ) -> ::windows_core::HRESULT,
    pub WaitForInputIdle: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        milliseconds: i32,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IRawElementProviderSimple(::windows_core::IUnknown);
impl IRawElementProviderSimple {
    pub fn GetValue<P0>(&self, dp: P0) -> ::windows_core::Result<::windows_core::IInspectable>
    where
        P0: ::windows_core::IntoParam<super::super::DependencyProperty>,
    {
        let this = &::windows_core::ComInterface::cast::<super::super::IDependencyObject>(self)?;
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
        P0: ::windows_core::IntoParam<super::super::DependencyProperty>,
        P1: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = &::windows_core::ComInterface::cast::<super::super::IDependencyObject>(self)?;
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
        P0: ::windows_core::IntoParam<super::super::DependencyProperty>,
    {
        let this = &::windows_core::ComInterface::cast::<super::super::IDependencyObject>(self)?;
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
        P0: ::windows_core::IntoParam<super::super::DependencyProperty>,
    {
        let this = &::windows_core::ComInterface::cast::<super::super::IDependencyObject>(self)?;
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
        P0: ::windows_core::IntoParam<super::super::DependencyProperty>,
    {
        let this = &::windows_core::ComInterface::cast::<super::super::IDependencyObject>(self)?;
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
        P0: ::windows_core::IntoParam<super::super::DependencyProperty>,
        P1: ::windows_core::IntoParam<super::super::DependencyPropertyChangedCallback>,
    {
        let this = &::windows_core::ComInterface::cast::<super::super::IDependencyObject>(self)?;
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
        P0: ::windows_core::IntoParam<super::super::DependencyProperty>,
    {
        let this = &::windows_core::ComInterface::cast::<super::super::IDependencyObject>(self)?;
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
        let this = &::windows_core::ComInterface::cast::<super::super::IDependencyObject>(self)?;
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
    ) -> ::windows_core::Result<super::super::super::Dispatching::DispatcherQueue> {
        let this = &::windows_core::ComInterface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DispatcherQueue)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for IRawElementProviderSimple {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for IRawElementProviderSimple {
    type Vtable = IIRawElementProviderSimple_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IRawElementProviderSimple {
    const IID: ::windows_core::GUID =
        <IIRawElementProviderSimple as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for IRawElementProviderSimple {
    const NAME: &'static str = "Microsoft.UI.Xaml.Automation.Provider.IRawElementProviderSimple";
}
::windows_core::imp::interface_hierarchy!(
    IRawElementProviderSimple,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<super::super::DependencyObject> for IRawElementProviderSimple {}
unsafe impl ::core::marker::Send for IRawElementProviderSimple {}
unsafe impl ::core::marker::Sync for IRawElementProviderSimple {}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
