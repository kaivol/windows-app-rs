#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDragDropManager(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDragDropManager {
    type Vtable = IDragDropManager_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDragDropManager {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x4fea9efc_b073_5fbe_9c95_a4113ef6393f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDragDropManager_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub AreConcurrentOperationsEnabled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetAreConcurrentOperationsEnabled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    )
        -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_Foundation")]
    pub TargetRequested: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    TargetRequested: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub RemoveTargetRequested: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RemoveTargetRequested: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDragDropManagerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDragDropManagerStatics {
    type Vtable = IDragDropManagerStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDragDropManagerStatics {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x5587c863_57d7_5d0f_8ea9_e5dcf06a0f83);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDragDropManagerStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Microsoft_UI_Content")]
    pub GetForIsland: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        content: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Microsoft_UI_Content"))]
    GetForIsland: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDragInfo(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDragInfo {
    type Vtable = IDragInfo_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDragInfo {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x7507d891_62a8_5a79_a880_ac7353d001ec);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDragInfo_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Windows_ApplicationModel_DataTransfer")]
    pub AllowedOperations: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::ApplicationModel::DataTransfer::DataPackageOperation,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_ApplicationModel_DataTransfer"))]
    AllowedOperations: usize,
    #[cfg(feature = "Windows_ApplicationModel_DataTransfer")]
    pub Data: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_ApplicationModel_DataTransfer"))]
    Data: usize,
    pub Modifiers: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut DragDropModifiers,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_Foundation")]
    pub Position: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::Point,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    Position: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDragOperation(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDragOperation {
    type Vtable = IDragOperation_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDragOperation {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xef122288_7984_53d3_8488_133dcd3de793);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDragOperation_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Windows_ApplicationModel_DataTransfer")]
    pub AllowedOperations: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::ApplicationModel::DataTransfer::DataPackageOperation,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_ApplicationModel_DataTransfer"))]
    AllowedOperations: usize,
    #[cfg(feature = "Windows_ApplicationModel_DataTransfer")]
    pub SetAllowedOperations: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::windows::ApplicationModel::DataTransfer::DataPackageOperation,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_ApplicationModel_DataTransfer"))]
    SetAllowedOperations: usize,
    #[cfg(feature = "Windows_ApplicationModel_DataTransfer")]
    pub Data: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_ApplicationModel_DataTransfer"))]
    Data: usize,
    pub DragUIContentMode: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut DragUIContentMode,
    ) -> ::windows_core::HRESULT,
    pub SetDragUIContentMode: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: DragUIContentMode,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_Graphics_Imaging")]
    pub SetDragUIContentFromSoftwareBitmap: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        bitmap: *mut ::core::ffi::c_void,
    )
        -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Graphics_Imaging"))]
    SetDragUIContentFromSoftwareBitmap: usize,
    #[cfg(all(feature = "Windows_Foundation", feature = "Windows_Graphics_Imaging"))]
    pub SetDragUIContentFromSoftwareBitmap2: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        bitmap: *mut ::core::ffi::c_void,
        anchorpoint: ::windows::Foundation::Point,
    )
        -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Windows_Foundation", feature = "Windows_Graphics_Imaging")))]
    SetDragUIContentFromSoftwareBitmap2: usize,
    #[cfg(all(
        feature = "Windows_ApplicationModel_DataTransfer",
        feature = "Windows_Foundation"
    ))]
    pub StartAsync: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        initialtarget: *mut ::core::ffi::c_void,
        initialpointerpoint: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(all(
        feature = "Windows_ApplicationModel_DataTransfer",
        feature = "Windows_Foundation"
    )))]
    StartAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDragUIOverride(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDragUIOverride {
    type Vtable = IDragUIOverride_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDragUIOverride {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x8432fbac_a17f_5a95_8f56_fb432280b54d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDragUIOverride_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Caption: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub SetCaption: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub IsCaptionVisible: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetIsCaptionVisible: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
    pub IsContentVisible: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetIsContentVisible: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
    pub IsGlyphVisible: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetIsGlyphVisible: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
    pub Clear: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_Graphics_Imaging")]
    pub SetContentFromSoftwareBitmap: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        bitmap: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Graphics_Imaging"))]
    SetContentFromSoftwareBitmap: usize,
    #[cfg(all(feature = "Windows_Foundation", feature = "Windows_Graphics_Imaging"))]
    pub SetContentFromSoftwareBitmap2: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        bitmap: *mut ::core::ffi::c_void,
        anchorpoint: ::windows::Foundation::Point,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Windows_Foundation", feature = "Windows_Graphics_Imaging")))]
    SetContentFromSoftwareBitmap2: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDropOperationTarget(::windows_core::IUnknown);
impl IDropOperationTarget {
    #[doc = "Required features: `\"Windows_ApplicationModel_DataTransfer\"`, `\"Windows_Foundation\"`"]
    #[cfg(all(
        feature = "Windows_ApplicationModel_DataTransfer",
        feature = "Windows_Foundation"
    ))]
    pub fn DropAsync<P0>(
        &self,
        draginfo: P0,
    ) -> ::windows_core::Result<
        ::windows::Foundation::IAsyncOperation<
            ::windows::ApplicationModel::DataTransfer::DataPackageOperation,
        >,
    >
    where
        P0: ::windows_core::IntoParam<DragInfo>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DropAsync)(
                ::windows_core::Interface::as_raw(this),
                draginfo.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_ApplicationModel_DataTransfer\"`, `\"Windows_Foundation\"`"]
    #[cfg(all(
        feature = "Windows_ApplicationModel_DataTransfer",
        feature = "Windows_Foundation"
    ))]
    pub fn EnterAsync<P0, P1>(
        &self,
        draginfo: P0,
        draguioverride: P1,
    ) -> ::windows_core::Result<
        ::windows::Foundation::IAsyncOperation<
            ::windows::ApplicationModel::DataTransfer::DataPackageOperation,
        >,
    >
    where
        P0: ::windows_core::IntoParam<DragInfo>,
        P1: ::windows_core::IntoParam<DragUIOverride>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).EnterAsync)(
                ::windows_core::Interface::as_raw(this),
                draginfo.into_param().abi(),
                draguioverride.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn LeaveAsync<P0>(
        &self,
        draginfo: P0,
    ) -> ::windows_core::Result<::windows::Foundation::IAsyncAction>
    where
        P0: ::windows_core::IntoParam<DragInfo>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LeaveAsync)(
                ::windows_core::Interface::as_raw(this),
                draginfo.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_ApplicationModel_DataTransfer\"`, `\"Windows_Foundation\"`"]
    #[cfg(all(
        feature = "Windows_ApplicationModel_DataTransfer",
        feature = "Windows_Foundation"
    ))]
    pub fn OverAsync<P0, P1>(
        &self,
        draginfo: P0,
        draguioverride: P1,
    ) -> ::windows_core::Result<
        ::windows::Foundation::IAsyncOperation<
            ::windows::ApplicationModel::DataTransfer::DataPackageOperation,
        >,
    >
    where
        P0: ::windows_core::IntoParam<DragInfo>,
        P1: ::windows_core::IntoParam<DragUIOverride>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OverAsync)(
                ::windows_core::Interface::as_raw(this),
                draginfo.into_param().abi(),
                draguioverride.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(
    IDropOperationTarget,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::RuntimeType for IDropOperationTarget {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_interface::<Self>();
}
unsafe impl ::windows_core::Interface for IDropOperationTarget {
    type Vtable = IDropOperationTarget_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDropOperationTarget {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x1c2707d9_0065_53c7_bbfb_50850378caf3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDropOperationTarget_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(all(
        feature = "Windows_ApplicationModel_DataTransfer",
        feature = "Windows_Foundation"
    ))]
    pub DropAsync: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        draginfo: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(all(
        feature = "Windows_ApplicationModel_DataTransfer",
        feature = "Windows_Foundation"
    )))]
    DropAsync: usize,
    #[cfg(all(
        feature = "Windows_ApplicationModel_DataTransfer",
        feature = "Windows_Foundation"
    ))]
    pub EnterAsync: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        draginfo: *mut ::core::ffi::c_void,
        draguioverride: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(all(
        feature = "Windows_ApplicationModel_DataTransfer",
        feature = "Windows_Foundation"
    )))]
    EnterAsync: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub LeaveAsync: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        draginfo: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    LeaveAsync: usize,
    #[cfg(all(
        feature = "Windows_ApplicationModel_DataTransfer",
        feature = "Windows_Foundation"
    ))]
    pub OverAsync: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        draginfo: *mut ::core::ffi::c_void,
        draguioverride: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(all(
        feature = "Windows_ApplicationModel_DataTransfer",
        feature = "Windows_Foundation"
    )))]
    OverAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDropOperationTargetRequestedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDropOperationTargetRequestedEventArgs {
    type Vtable = IDropOperationTargetRequestedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDropOperationTargetRequestedEventArgs {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xf61c5b62_720e_59ff_ad0b_e77fc5b8a4a3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDropOperationTargetRequestedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SetTarget: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        target: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct DragDropManager(::windows_core::IUnknown);
impl DragDropManager {
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<::windows::Foundation::IClosable>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this))
                .ok()
        }
    }
    pub fn AreConcurrentOperationsEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AreConcurrentOperationsEnabled)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetAreConcurrentOperationsEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetAreConcurrentOperationsEnabled)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn TargetRequested<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<
                DragDropManager,
                DropOperationTargetRequestedEventArgs,
            >,
        >,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TargetRequested)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveTargetRequested(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveTargetRequested)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Microsoft_UI_Content\"`"]
    #[cfg(feature = "Microsoft_UI_Content")]
    pub fn GetForIsland<P0>(content: P0) -> ::windows_core::Result<DragDropManager>
    where
        P0: ::windows_core::TryIntoParam<super::super::Content::ContentIsland>,
    {
        Self::IDragDropManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetForIsland)(
                ::windows_core::Interface::as_raw(this),
                content.try_into_param()?.abi(),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IDragDropManagerStatics<
        R,
        F: FnOnce(&IDragDropManagerStatics) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<DragDropManager, IDragDropManagerStatics> =
            ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for DragDropManager {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for DragDropManager {
    type Vtable = IDragDropManager_Vtbl;
}
unsafe impl ::windows_core::ComInterface for DragDropManager {
    const IID: ::windows_core::GUID = <IDragDropManager as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for DragDropManager {
    const NAME: &'static str = "Microsoft.UI.Input.DragDrop.DragDropManager";
}
::windows_core::imp::interface_hierarchy!(
    DragDropManager,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
#[cfg(feature = "Windows_Foundation")]
impl ::windows_core::CanTryInto<::windows::Foundation::IClosable> for DragDropManager {}
unsafe impl ::core::marker::Send for DragDropManager {}
unsafe impl ::core::marker::Sync for DragDropManager {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct DragInfo(::windows_core::IUnknown);
impl DragInfo {
    #[doc = "Required features: `\"Windows_ApplicationModel_DataTransfer\"`"]
    #[cfg(feature = "Windows_ApplicationModel_DataTransfer")]
    pub fn AllowedOperations(
        &self,
    ) -> ::windows_core::Result<::windows::ApplicationModel::DataTransfer::DataPackageOperation>
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AllowedOperations)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_ApplicationModel_DataTransfer\"`"]
    #[cfg(feature = "Windows_ApplicationModel_DataTransfer")]
    pub fn Data(
        &self,
    ) -> ::windows_core::Result<::windows::ApplicationModel::DataTransfer::DataPackageView> {
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
    pub fn Modifiers(&self) -> ::windows_core::Result<DragDropModifiers> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Modifiers)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn Position(&self) -> ::windows_core::Result<::windows::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Position)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for DragInfo {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for DragInfo {
    type Vtable = IDragInfo_Vtbl;
}
unsafe impl ::windows_core::ComInterface for DragInfo {
    const IID: ::windows_core::GUID = <IDragInfo as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for DragInfo {
    const NAME: &'static str = "Microsoft.UI.Input.DragDrop.DragInfo";
}
::windows_core::imp::interface_hierarchy!(
    DragInfo,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for DragInfo {}
unsafe impl ::core::marker::Sync for DragInfo {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct DragOperation(::windows_core::IUnknown);
impl DragOperation {
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
            DragOperation,
            ::windows_core::imp::IGenericFactory,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<::windows::Foundation::IClosable>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this))
                .ok()
        }
    }
    #[doc = "Required features: `\"Windows_ApplicationModel_DataTransfer\"`"]
    #[cfg(feature = "Windows_ApplicationModel_DataTransfer")]
    pub fn AllowedOperations(
        &self,
    ) -> ::windows_core::Result<::windows::ApplicationModel::DataTransfer::DataPackageOperation>
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AllowedOperations)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_ApplicationModel_DataTransfer\"`"]
    #[cfg(feature = "Windows_ApplicationModel_DataTransfer")]
    pub fn SetAllowedOperations(
        &self,
        value: ::windows::ApplicationModel::DataTransfer::DataPackageOperation,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetAllowedOperations)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_ApplicationModel_DataTransfer\"`"]
    #[cfg(feature = "Windows_ApplicationModel_DataTransfer")]
    pub fn Data(
        &self,
    ) -> ::windows_core::Result<::windows::ApplicationModel::DataTransfer::DataPackage> {
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
    pub fn DragUIContentMode(&self) -> ::windows_core::Result<DragUIContentMode> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DragUIContentMode)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetDragUIContentMode(&self, value: DragUIContentMode) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetDragUIContentMode)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Graphics_Imaging\"`"]
    #[cfg(feature = "Windows_Graphics_Imaging")]
    pub fn SetDragUIContentFromSoftwareBitmap<P0>(&self, bitmap: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows::Graphics::Imaging::SoftwareBitmap>,
    {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetDragUIContentFromSoftwareBitmap)(
                ::windows_core::Interface::as_raw(this),
                bitmap.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`, `\"Windows_Graphics_Imaging\"`"]
    #[cfg(all(feature = "Windows_Foundation", feature = "Windows_Graphics_Imaging"))]
    pub fn SetDragUIContentFromSoftwareBitmap2<P0>(
        &self,
        bitmap: P0,
        anchorpoint: ::windows::Foundation::Point,
    ) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows::Graphics::Imaging::SoftwareBitmap>,
    {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetDragUIContentFromSoftwareBitmap2)(
                ::windows_core::Interface::as_raw(this),
                bitmap.into_param().abi(),
                anchorpoint,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_ApplicationModel_DataTransfer\"`, `\"Windows_Foundation\"`"]
    #[cfg(all(
        feature = "Windows_ApplicationModel_DataTransfer",
        feature = "Windows_Foundation"
    ))]
    pub fn StartAsync<P0, P1>(
        &self,
        initialtarget: P0,
        initialpointerpoint: P1,
    ) -> ::windows_core::Result<
        ::windows::Foundation::IAsyncOperation<
            ::windows::ApplicationModel::DataTransfer::DataPackageOperation,
        >,
    >
    where
        P0: ::windows_core::IntoParam<DragDropManager>,
        P1: ::windows_core::IntoParam<super::PointerPoint>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StartAsync)(
                ::windows_core::Interface::as_raw(this),
                initialtarget.into_param().abi(),
                initialpointerpoint.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for DragOperation {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for DragOperation {
    type Vtable = IDragOperation_Vtbl;
}
unsafe impl ::windows_core::ComInterface for DragOperation {
    const IID: ::windows_core::GUID = <IDragOperation as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for DragOperation {
    const NAME: &'static str = "Microsoft.UI.Input.DragDrop.DragOperation";
}
::windows_core::imp::interface_hierarchy!(
    DragOperation,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
#[cfg(feature = "Windows_Foundation")]
impl ::windows_core::CanTryInto<::windows::Foundation::IClosable> for DragOperation {}
unsafe impl ::core::marker::Send for DragOperation {}
unsafe impl ::core::marker::Sync for DragOperation {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct DragUIOverride(::windows_core::IUnknown);
impl DragUIOverride {
    pub fn Caption(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Caption)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetCaption(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetCaption)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn IsCaptionVisible(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsCaptionVisible)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetIsCaptionVisible(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetIsCaptionVisible)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsContentVisible(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsContentVisible)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetIsContentVisible(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetIsContentVisible)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsGlyphVisible(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsGlyphVisible)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetIsGlyphVisible(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetIsGlyphVisible)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn Clear(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).Clear)(::windows_core::Interface::as_raw(this))
                .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Graphics_Imaging\"`"]
    #[cfg(feature = "Windows_Graphics_Imaging")]
    pub fn SetContentFromSoftwareBitmap<P0>(&self, bitmap: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows::Graphics::Imaging::SoftwareBitmap>,
    {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetContentFromSoftwareBitmap)(
                ::windows_core::Interface::as_raw(this),
                bitmap.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`, `\"Windows_Graphics_Imaging\"`"]
    #[cfg(all(feature = "Windows_Foundation", feature = "Windows_Graphics_Imaging"))]
    pub fn SetContentFromSoftwareBitmap2<P0>(
        &self,
        bitmap: P0,
        anchorpoint: ::windows::Foundation::Point,
    ) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows::Graphics::Imaging::SoftwareBitmap>,
    {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetContentFromSoftwareBitmap2)(
                ::windows_core::Interface::as_raw(this),
                bitmap.into_param().abi(),
                anchorpoint,
            )
            .ok()
        }
    }
}
impl ::windows_core::RuntimeType for DragUIOverride {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for DragUIOverride {
    type Vtable = IDragUIOverride_Vtbl;
}
unsafe impl ::windows_core::ComInterface for DragUIOverride {
    const IID: ::windows_core::GUID = <IDragUIOverride as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for DragUIOverride {
    const NAME: &'static str = "Microsoft.UI.Input.DragDrop.DragUIOverride";
}
::windows_core::imp::interface_hierarchy!(
    DragUIOverride,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for DragUIOverride {}
unsafe impl ::core::marker::Sync for DragUIOverride {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct DropOperationTargetRequestedEventArgs(::windows_core::IUnknown);
impl DropOperationTargetRequestedEventArgs {
    pub fn SetTarget<P0>(&self, target: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<IDropOperationTarget>,
    {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetTarget)(
                ::windows_core::Interface::as_raw(this),
                target.try_into_param()?.abi(),
            )
            .ok()
        }
    }
}
impl ::windows_core::RuntimeType for DropOperationTargetRequestedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for DropOperationTargetRequestedEventArgs {
    type Vtable = IDropOperationTargetRequestedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for DropOperationTargetRequestedEventArgs {
    const IID: ::windows_core::GUID =
        <IDropOperationTargetRequestedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for DropOperationTargetRequestedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Input.DragDrop.DropOperationTargetRequestedEventArgs";
}
::windows_core::imp::interface_hierarchy!(
    DropOperationTargetRequestedEventArgs,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for DropOperationTargetRequestedEventArgs {}
unsafe impl ::core::marker::Sync for DropOperationTargetRequestedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DragDropModifiers(pub u32);
impl DragDropModifiers {
    pub const None: Self = Self(0u32);
    pub const Shift: Self = Self(1u32);
    pub const Control: Self = Self(2u32);
    pub const Alt: Self = Self(4u32);
    pub const LeftButton: Self = Self(8u32);
    pub const MiddleButton: Self = Self(16u32);
    pub const RightButton: Self = Self(32u32);
}
impl ::core::marker::Copy for DragDropModifiers {}
impl ::core::clone::Clone for DragDropModifiers {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DragDropModifiers {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for DragDropModifiers {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for DragDropModifiers {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DragDropModifiers").field(&self.0).finish()
    }
}
impl DragDropModifiers {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for DragDropModifiers {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for DragDropModifiers {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for DragDropModifiers {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for DragDropModifiers {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for DragDropModifiers {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::windows_core::RuntimeType for DragDropModifiers {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::from_slice(
            b"enum(Microsoft.UI.Input.DragDrop.DragDropModifiers;u4)",
        );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DragUIContentMode(pub i32);
impl DragUIContentMode {
    pub const Auto: Self = Self(0i32);
    pub const Deferred: Self = Self(1i32);
}
impl ::core::marker::Copy for DragUIContentMode {}
impl ::core::clone::Clone for DragUIContentMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DragUIContentMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for DragUIContentMode {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for DragUIContentMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DragUIContentMode").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for DragUIContentMode {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::from_slice(
            b"enum(Microsoft.UI.Input.DragDrop.DragUIContentMode;i4)",
        );
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
