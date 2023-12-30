#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IBinding(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBinding {
    type Vtable = IBinding_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IBinding {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x501ea0e8_edd4_59de_8845_76af2eabbe00);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBinding_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Path: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetPath: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub Mode: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut BindingMode,
    ) -> ::windows_core::HRESULT,
    pub SetMode: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: BindingMode,
    ) -> ::windows_core::HRESULT,
    pub Source: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetSource: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub RelativeSource: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetRelativeSource: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub ElementName: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub SetElementName: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub Converter: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetConverter: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub ConverterParameter: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetConverterParameter: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub ConverterLanguage: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub SetConverterLanguage: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub FallbackValue: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetFallbackValue: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub TargetNullValue: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetTargetNullValue: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub UpdateSourceTrigger: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut UpdateSourceTrigger,
    ) -> ::windows_core::HRESULT,
    pub SetUpdateSourceTrigger: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: UpdateSourceTrigger,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IBindingBase(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBindingBase {
    type Vtable = IBindingBase_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IBindingBase {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x91ddd141_5944_50ef_b85e_218e463f7a73);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBindingBase_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IBindingBaseFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBindingBaseFactory {
    type Vtable = IBindingBaseFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IBindingBaseFactory {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xc8a866c5_f6f3_5f7a_9592_d385af48bd8f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBindingBaseFactory_Vtbl {
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
pub struct IBindingExpression(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBindingExpression {
    type Vtable = IBindingExpression_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IBindingExpression {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x4c023916_37bc_5b07_bc9d_15c547bd9b26);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBindingExpression_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub DataItem: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub ParentBinding: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub UpdateSource:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IBindingExpressionBase(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBindingExpressionBase {
    type Vtable = IBindingExpressionBase_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IBindingExpressionBase {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x8825e5a9_d9a3_5e87_bcd8_c63133d29029);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBindingExpressionBase_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IBindingExpressionBaseFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBindingExpressionBaseFactory {
    type Vtable = IBindingExpressionBaseFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IBindingExpressionBaseFactory {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x41d643b9_2629_5451_a716_596c0848b5dc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBindingExpressionBaseFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IBindingExpressionFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBindingExpressionFactory {
    type Vtable = IBindingExpressionFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IBindingExpressionFactory {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x086cae14_81a1_588b_b619_05ee84c0f089);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBindingExpressionFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IBindingFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBindingFactory {
    type Vtable = IBindingFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IBindingFactory {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xcb2de749_b115_5f67_b64a_797d54885d5c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBindingFactory_Vtbl {
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
pub struct IBindingOperations(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBindingOperations {
    type Vtable = IBindingOperations_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IBindingOperations {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x9a319b95_aabe_5075_b227_8eb07e443d8b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBindingOperations_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IBindingOperationsStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBindingOperationsStatics {
    type Vtable = IBindingOperationsStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IBindingOperationsStatics {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x1e1bdbd3_fca5_5c85_b87d_b504cd8fa8ac);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBindingOperationsStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SetBinding: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        target: *mut ::core::ffi::c_void,
        dp: *mut ::core::ffi::c_void,
        binding: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICollectionView(::windows_core::IUnknown);
impl ICollectionView {
    pub fn CurrentItem(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CurrentItem)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn CurrentPosition(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CurrentPosition)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn IsCurrentAfterLast(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsCurrentAfterLast)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn IsCurrentBeforeFirst(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsCurrentBeforeFirst)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation_Collections\"`"]
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub fn CollectionGroups(
        &self,
    ) -> ::windows_core::Result<
        ::windows::Foundation::Collections::IObservableVector<::windows_core::IInspectable>,
    > {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CollectionGroups)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn HasMoreItems(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HasMoreItems)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn CurrentChanged<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::EventHandler<::windows_core::IInspectable>,
        >,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CurrentChanged)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveCurrentChanged(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveCurrentChanged)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn CurrentChanging<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<CurrentChangingEventHandler>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CurrentChanging)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveCurrentChanging(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveCurrentChanging)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn MoveCurrentTo<P0>(&self, item: P0) -> ::windows_core::Result<bool>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MoveCurrentTo)(
                ::windows_core::Interface::as_raw(this),
                item.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn MoveCurrentToPosition(&self, index: i32) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MoveCurrentToPosition)(
                ::windows_core::Interface::as_raw(this),
                index,
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn MoveCurrentToFirst(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MoveCurrentToFirst)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn MoveCurrentToLast(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MoveCurrentToLast)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn MoveCurrentToNext(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MoveCurrentToNext)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn MoveCurrentToPrevious(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MoveCurrentToPrevious)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn LoadMoreItemsAsync(
        &self,
        count: u32,
    ) -> ::windows_core::Result<::windows::Foundation::IAsyncOperation<LoadMoreItemsResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LoadMoreItemsAsync)(
                ::windows_core::Interface::as_raw(this),
                count,
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation_Collections\"`"]
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub fn First(
        &self,
    ) -> ::windows_core::Result<
        ::windows::Foundation::Collections::IIterator<::windows_core::IInspectable>,
    > {
        let this = &::windows_core::ComInterface::cast::<
            ::windows::Foundation::Collections::IIterable<::windows_core::IInspectable>,
        >(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).First)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation_Collections\"`"]
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub fn VectorChanged<P0>(
        &self,
        vhnd: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::Collections::VectorChangedEventHandler<
                ::windows_core::IInspectable,
            >,
        >,
    {
        let this = &::windows_core::ComInterface::cast::<
            ::windows::Foundation::Collections::IObservableVector<::windows_core::IInspectable>,
        >(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).VectorChanged)(
                ::windows_core::Interface::as_raw(this),
                vhnd.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation_Collections\"`"]
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub fn RemoveVectorChanged(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<
            ::windows::Foundation::Collections::IObservableVector<::windows_core::IInspectable>,
        >(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveVectorChanged)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation_Collections\"`"]
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub fn GetAt(&self, index: u32) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = &::windows_core::ComInterface::cast::<
            ::windows::Foundation::Collections::IVector<::windows_core::IInspectable>,
        >(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetAt)(
                ::windows_core::Interface::as_raw(this),
                index,
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation_Collections\"`"]
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub fn Size(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::ComInterface::cast::<
            ::windows::Foundation::Collections::IVector<::windows_core::IInspectable>,
        >(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Size)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation_Collections\"`"]
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub fn GetView(
        &self,
    ) -> ::windows_core::Result<
        ::windows::Foundation::Collections::IVectorView<::windows_core::IInspectable>,
    > {
        let this = &::windows_core::ComInterface::cast::<
            ::windows::Foundation::Collections::IVector<::windows_core::IInspectable>,
        >(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetView)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation_Collections\"`"]
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub fn IndexOf<P0>(&self, value: P0, index: &mut u32) -> ::windows_core::Result<bool>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = &::windows_core::ComInterface::cast::<
            ::windows::Foundation::Collections::IVector<::windows_core::IInspectable>,
        >(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IndexOf)(
                ::windows_core::Interface::as_raw(this),
                value.into_param().abi(),
                index,
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation_Collections\"`"]
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub fn SetAt<P0>(&self, index: u32, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = &::windows_core::ComInterface::cast::<
            ::windows::Foundation::Collections::IVector<::windows_core::IInspectable>,
        >(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetAt)(
                ::windows_core::Interface::as_raw(this),
                index,
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation_Collections\"`"]
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub fn InsertAt<P0>(&self, index: u32, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = &::windows_core::ComInterface::cast::<
            ::windows::Foundation::Collections::IVector<::windows_core::IInspectable>,
        >(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).InsertAt)(
                ::windows_core::Interface::as_raw(this),
                index,
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation_Collections\"`"]
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub fn RemoveAt(&self, index: u32) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<
            ::windows::Foundation::Collections::IVector<::windows_core::IInspectable>,
        >(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveAt)(
                ::windows_core::Interface::as_raw(this),
                index,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation_Collections\"`"]
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub fn Append<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = &::windows_core::ComInterface::cast::<
            ::windows::Foundation::Collections::IVector<::windows_core::IInspectable>,
        >(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).Append)(
                ::windows_core::Interface::as_raw(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation_Collections\"`"]
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub fn RemoveAtEnd(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<
            ::windows::Foundation::Collections::IVector<::windows_core::IInspectable>,
        >(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveAtEnd)(
                ::windows_core::Interface::as_raw(this),
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation_Collections\"`"]
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub fn Clear(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<
            ::windows::Foundation::Collections::IVector<::windows_core::IInspectable>,
        >(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).Clear)(::windows_core::Interface::as_raw(this))
                .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation_Collections\"`"]
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub fn GetMany(
        &self,
        startindex: u32,
        items: &mut [::core::option::Option<::windows_core::IInspectable>],
    ) -> ::windows_core::Result<u32> {
        let this = &::windows_core::ComInterface::cast::<
            ::windows::Foundation::Collections::IVector<::windows_core::IInspectable>,
        >(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetMany)(
                ::windows_core::Interface::as_raw(this),
                startindex,
                items.len().try_into().unwrap(),
                ::core::mem::transmute_copy(&items),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation_Collections\"`"]
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub fn ReplaceAll(
        &self,
        items: &[::core::option::Option<::windows_core::IInspectable>],
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<
            ::windows::Foundation::Collections::IVector<::windows_core::IInspectable>,
        >(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).ReplaceAll)(
                ::windows_core::Interface::as_raw(this),
                items.len().try_into().unwrap(),
                ::core::mem::transmute(items.as_ptr()),
            )
            .ok()
        }
    }
}
::windows_core::imp::interface_hierarchy!(
    ICollectionView,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
#[cfg(feature = "Windows_Foundation_Collections")]
impl
    ::windows_core::CanTryInto<
        ::windows::Foundation::Collections::IIterable<::windows_core::IInspectable>,
    > for ICollectionView
{
}
#[cfg(feature = "Windows_Foundation_Collections")]
impl
    ::windows_core::CanTryInto<
        ::windows::Foundation::Collections::IObservableVector<::windows_core::IInspectable>,
    > for ICollectionView
{
}
#[cfg(feature = "Windows_Foundation_Collections")]
impl
    ::windows_core::CanTryInto<
        ::windows::Foundation::Collections::IVector<::windows_core::IInspectable>,
    > for ICollectionView
{
}
impl ::windows_core::RuntimeType for ICollectionView {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[cfg(feature = "Windows_Foundation_Collections")]
impl ::core::iter::IntoIterator for ICollectionView {
    type Item = ::windows_core::IInspectable;
    type IntoIter = ::windows::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Windows_Foundation_Collections")]
impl ::core::iter::IntoIterator for &ICollectionView {
    type Item = ::windows_core::IInspectable;
    type IntoIter = ::windows::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::windows::Foundation::Collections::VectorIterator::new(
            ::windows_core::ComInterface::cast(self).ok(),
        )
    }
}
unsafe impl ::windows_core::Interface for ICollectionView {
    type Vtable = ICollectionView_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICollectionView {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xf8bb90d8_e008_5d65_8c97_7bb790a4230c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICollectionView_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CurrentItem: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub CurrentPosition: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut i32,
    ) -> ::windows_core::HRESULT,
    pub IsCurrentAfterLast: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub IsCurrentBeforeFirst: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub CollectionGroups: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation_Collections"))]
    CollectionGroups: usize,
    pub HasMoreItems: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_Foundation")]
    pub CurrentChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    CurrentChanged: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub RemoveCurrentChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RemoveCurrentChanged: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub CurrentChanging: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    CurrentChanging: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub RemoveCurrentChanging: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RemoveCurrentChanging: usize,
    pub MoveCurrentTo: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        item: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub MoveCurrentToPosition: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        index: i32,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub MoveCurrentToFirst: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub MoveCurrentToLast: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub MoveCurrentToNext: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub MoveCurrentToPrevious: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_Foundation")]
    pub LoadMoreItemsAsync: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        count: u32,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    LoadMoreItemsAsync: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICollectionViewFactory(::windows_core::IUnknown);
impl ICollectionViewFactory {
    pub fn CreateView(&self) -> ::windows_core::Result<ICollectionView> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateView)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(
    ICollectionViewFactory,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::RuntimeType for ICollectionViewFactory {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_interface::<Self>();
}
unsafe impl ::windows_core::Interface for ICollectionViewFactory {
    type Vtable = ICollectionViewFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICollectionViewFactory {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xd971f795_5728_5bef_9602_43f2c4250e56);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICollectionViewFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateView: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICollectionViewGroup(::windows_core::IUnknown);
impl ICollectionViewGroup {
    pub fn Group(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Group)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation_Collections\"`"]
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub fn GroupItems(
        &self,
    ) -> ::windows_core::Result<
        ::windows::Foundation::Collections::IObservableVector<::windows_core::IInspectable>,
    > {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GroupItems)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(
    ICollectionViewGroup,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::RuntimeType for ICollectionViewGroup {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_interface::<Self>();
}
unsafe impl ::windows_core::Interface for ICollectionViewGroup {
    type Vtable = ICollectionViewGroup_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICollectionViewGroup {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x96a08da8_be38_5ae0_903d_6fb6111e61f5);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICollectionViewGroup_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Group: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub GroupItems: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation_Collections"))]
    GroupItems: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICollectionViewSource(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICollectionViewSource {
    type Vtable = ICollectionViewSource_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICollectionViewSource {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xa45e3b3a_f31e_5bbb_8a7c_70cf5c64bc3f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICollectionViewSource_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Source: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetSource: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub View: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub IsSourceGrouped: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetIsSourceGrouped: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
    pub ItemsPath: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetItemsPath: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICollectionViewSourceStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICollectionViewSourceStatics {
    type Vtable = ICollectionViewSourceStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICollectionViewSourceStatics {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xe282f10f_d4b1_5769_8a11_30f739e6113b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICollectionViewSourceStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SourceProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub ViewProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub IsSourceGroupedProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub ItemsPathProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICurrentChangingEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICurrentChangingEventArgs {
    type Vtable = ICurrentChangingEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICurrentChangingEventArgs {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x63e42ed6_e14a_51ea_9cb1_72f9c907dc80);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICurrentChangingEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Cancel: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetCancel: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
    pub IsCancelable: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICurrentChangingEventArgsFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICurrentChangingEventArgsFactory {
    type Vtable = ICurrentChangingEventArgsFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICurrentChangingEventArgsFactory {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x3670f48a_ac2c_5352_8a4b_6b977a08e5f8);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICurrentChangingEventArgsFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        baseinterface: *mut ::core::ffi::c_void,
        innerinterface: *mut *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub CreateWithCancelableParameter: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        iscancelable: bool,
        baseinterface: *mut ::core::ffi::c_void,
        innerinterface: *mut *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICustomProperty(::windows_core::IUnknown);
impl ICustomProperty {
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
    pub fn GetValue<P0>(&self, target: P0) -> ::windows_core::Result<::windows_core::IInspectable>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetValue)(
                ::windows_core::Interface::as_raw(this),
                target.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetValue<P0, P1>(&self, target: P0, value: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
        P1: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetValue)(
                ::windows_core::Interface::as_raw(this),
                target.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn GetIndexedValue<P0, P1>(
        &self,
        target: P0,
        index: P1,
    ) -> ::windows_core::Result<::windows_core::IInspectable>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
        P1: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetIndexedValue)(
                ::windows_core::Interface::as_raw(this),
                target.into_param().abi(),
                index.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetIndexedValue<P0, P1, P2>(
        &self,
        target: P0,
        value: P1,
        index: P2,
    ) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
        P1: ::windows_core::IntoParam<::windows_core::IInspectable>,
        P2: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetIndexedValue)(
                ::windows_core::Interface::as_raw(this),
                target.into_param().abi(),
                value.into_param().abi(),
                index.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn CanWrite(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CanWrite)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn CanRead(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CanRead)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(
    ICustomProperty,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::RuntimeType for ICustomProperty {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_interface::<Self>();
}
unsafe impl ::windows_core::Interface for ICustomProperty {
    type Vtable = ICustomProperty_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICustomProperty {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x30da92c0_23e8_42a0_ae7c_734a0e5d2782);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICustomProperty_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Windows_UI_Xaml_Interop")]
    pub Type: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<
            super::super::super::super::Windows::UI::Xaml::Interop::TypeName,
        >,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI_Xaml_Interop"))]
    Type: usize,
    pub Name: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub GetValue: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        target: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetValue: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        target: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub GetIndexedValue: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        target: *mut ::core::ffi::c_void,
        index: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetIndexedValue: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        target: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
        index: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub CanWrite: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub CanRead: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICustomPropertyProvider(::windows_core::IUnknown);
impl ICustomPropertyProvider {
    pub fn GetCustomProperty(
        &self,
        name: &::windows_core::HSTRING,
    ) -> ::windows_core::Result<ICustomProperty> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetCustomProperty)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(name),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_UI_Xaml_Interop\"`"]
    #[cfg(feature = "Windows_UI_Xaml_Interop")]
    pub fn GetIndexedProperty<P0>(
        &self,
        name: &::windows_core::HSTRING,
        r#type: P0,
    ) -> ::windows_core::Result<ICustomProperty>
    where
        P0: ::windows_core::IntoParam<
            super::super::super::super::Windows::UI::Xaml::Interop::TypeName,
        >,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetIndexedProperty)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(name),
                r#type.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn GetStringRepresentation(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetStringRepresentation)(
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
}
::windows_core::imp::interface_hierarchy!(
    ICustomPropertyProvider,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::RuntimeType for ICustomPropertyProvider {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_interface::<Self>();
}
unsafe impl ::windows_core::Interface for ICustomPropertyProvider {
    type Vtable = ICustomPropertyProvider_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICustomPropertyProvider {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x7c925755_3e48_42b4_8677_76372267033f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICustomPropertyProvider_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub GetCustomProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        name: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_UI_Xaml_Interop")]
    pub GetIndexedProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        name: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        r#type: ::std::mem::MaybeUninit<
            super::super::super::super::Windows::UI::Xaml::Interop::TypeName,
        >,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI_Xaml_Interop"))]
    GetIndexedProperty: usize,
    pub GetStringRepresentation: unsafe extern "system" fn(
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
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDataErrorsChangedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDataErrorsChangedEventArgs {
    type Vtable = IDataErrorsChangedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDataErrorsChangedEventArgs {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xd026dd64_5f26_5f15_a86a_0dec8a431796);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataErrorsChangedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub PropertyName: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub SetPropertyName: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDataErrorsChangedEventArgsFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDataErrorsChangedEventArgsFactory {
    type Vtable = IDataErrorsChangedEventArgsFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDataErrorsChangedEventArgsFactory {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x62d0bd1e_b85f_5fcc_842a_7cb0dda37fe5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataErrorsChangedEventArgsFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        name: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IItemIndexRange(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IItemIndexRange {
    type Vtable = IItemIndexRange_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IItemIndexRange {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xeba09846_2554_5b86_ac17_614f05105fa2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IItemIndexRange_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub FirstIndex: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut i32,
    ) -> ::windows_core::HRESULT,
    pub Length: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut u32,
    ) -> ::windows_core::HRESULT,
    pub LastIndex: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut i32,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IItemIndexRangeFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IItemIndexRangeFactory {
    type Vtable = IItemIndexRangeFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IItemIndexRangeFactory {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x9fc73213_eda0_5238_aa2c_401c9921f0f9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IItemIndexRangeFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        firstindex: i32,
        length: u32,
        baseinterface: *mut ::core::ffi::c_void,
        innerinterface: *mut *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IItemsRangeInfo(::windows_core::IUnknown);
impl IItemsRangeInfo {
    #[doc = "Required features: `\"Windows_Foundation_Collections\"`"]
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub fn RangesChanged<P0, P1>(
        &self,
        visiblerange: P0,
        trackeditems: P1,
    ) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<ItemIndexRange>,
        P1: ::windows_core::TryIntoParam<
            ::windows::Foundation::Collections::IVectorView<ItemIndexRange>,
        >,
    {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).RangesChanged)(
                ::windows_core::Interface::as_raw(this),
                visiblerange.try_into_param()?.abi(),
                trackeditems.try_into_param()?.abi(),
            )
            .ok()
        }
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
}
::windows_core::imp::interface_hierarchy!(
    IItemsRangeInfo,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
#[cfg(feature = "Windows_Foundation")]
impl ::windows_core::CanTryInto<::windows::Foundation::IClosable> for IItemsRangeInfo {}
impl ::windows_core::RuntimeType for IItemsRangeInfo {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_interface::<Self>();
}
unsafe impl ::windows_core::Interface for IItemsRangeInfo {
    type Vtable = IItemsRangeInfo_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IItemsRangeInfo {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xb8376d08_85fb_563b_8273_39ef2d138256);
}
#[repr(C)]
#[doc(hidden)]
pub struct IItemsRangeInfo_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub RangesChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        visiblerange: *mut ::core::ffi::c_void,
        trackeditems: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation_Collections"))]
    RangesChanged: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct INotifyDataErrorInfo(::windows_core::IUnknown);
impl INotifyDataErrorInfo {
    pub fn HasErrors(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HasErrors)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn ErrorsChanged<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::EventHandler<DataErrorsChangedEventArgs>,
        >,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ErrorsChanged)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveErrorsChanged(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveErrorsChanged)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation_Collections\"`"]
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub fn GetErrors(
        &self,
        propertyname: &::windows_core::HSTRING,
    ) -> ::windows_core::Result<
        ::windows::Foundation::Collections::IIterable<::windows_core::IInspectable>,
    > {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetErrors)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(propertyname),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(
    INotifyDataErrorInfo,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::RuntimeType for INotifyDataErrorInfo {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_interface::<Self>();
}
unsafe impl ::windows_core::Interface for INotifyDataErrorInfo {
    type Vtable = INotifyDataErrorInfo_Vtbl;
}
unsafe impl ::windows_core::ComInterface for INotifyDataErrorInfo {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x0ee6c2cc_273e_567d_bc0a_1dd87ee51eba);
}
#[repr(C)]
#[doc(hidden)]
pub struct INotifyDataErrorInfo_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub HasErrors: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_Foundation")]
    pub ErrorsChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    ErrorsChanged: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub RemoveErrorsChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RemoveErrorsChanged: usize,
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub GetErrors: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        propertyname: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation_Collections"))]
    GetErrors: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct INotifyPropertyChanged(::windows_core::IUnknown);
impl INotifyPropertyChanged {
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn PropertyChanged<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<PropertyChangedEventHandler>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PropertyChanged)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemovePropertyChanged(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).RemovePropertyChanged)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
}
::windows_core::imp::interface_hierarchy!(
    INotifyPropertyChanged,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::RuntimeType for INotifyPropertyChanged {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_interface::<Self>();
}
unsafe impl ::windows_core::Interface for INotifyPropertyChanged {
    type Vtable = INotifyPropertyChanged_Vtbl;
}
unsafe impl ::windows_core::ComInterface for INotifyPropertyChanged {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x90b17601_b065_586e_83d9_9adc3a695284);
}
#[repr(C)]
#[doc(hidden)]
pub struct INotifyPropertyChanged_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Windows_Foundation")]
    pub PropertyChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    PropertyChanged: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub RemovePropertyChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RemovePropertyChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPropertyChangedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPropertyChangedEventArgs {
    type Vtable = IPropertyChangedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPropertyChangedEventArgs {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x63d0c952_396b_54f4_af8c_ba8724a427bf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertyChangedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub PropertyName: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPropertyChangedEventArgsFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPropertyChangedEventArgsFactory {
    type Vtable = IPropertyChangedEventArgsFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPropertyChangedEventArgsFactory {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x7c0c27a8_0b41_5070_b160_fc9ae960a36c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertyChangedEventArgsFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        name: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        baseinterface: *mut ::core::ffi::c_void,
        innerinterface: *mut *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IRelativeSource(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRelativeSource {
    type Vtable = IRelativeSource_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IRelativeSource {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x7ffc8126_5dd8_58bb_b686_c71eddea07b2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRelativeSource_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Mode: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut RelativeSourceMode,
    ) -> ::windows_core::HRESULT,
    pub SetMode: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: RelativeSourceMode,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IRelativeSourceFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRelativeSourceFactory {
    type Vtable = IRelativeSourceFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IRelativeSourceFactory {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x8518522c_85e3_5ae1_b9e9_28ea43c2051e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRelativeSourceFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        baseinterface: *mut ::core::ffi::c_void,
        innerinterface: *mut *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ISelectionInfo(::windows_core::IUnknown);
impl ISelectionInfo {
    pub fn SelectRange<P0>(&self, itemindexrange: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<ItemIndexRange>,
    {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SelectRange)(
                ::windows_core::Interface::as_raw(this),
                itemindexrange.try_into_param()?.abi(),
            )
            .ok()
        }
    }
    pub fn DeselectRange<P0>(&self, itemindexrange: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<ItemIndexRange>,
    {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).DeselectRange)(
                ::windows_core::Interface::as_raw(this),
                itemindexrange.try_into_param()?.abi(),
            )
            .ok()
        }
    }
    pub fn IsSelected(&self, index: i32) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsSelected)(
                ::windows_core::Interface::as_raw(this),
                index,
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation_Collections\"`"]
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub fn GetSelectedRanges(
        &self,
    ) -> ::windows_core::Result<::windows::Foundation::Collections::IVectorView<ItemIndexRange>>
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetSelectedRanges)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(
    ISelectionInfo,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::RuntimeType for ISelectionInfo {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_interface::<Self>();
}
unsafe impl ::windows_core::Interface for ISelectionInfo {
    type Vtable = ISelectionInfo_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ISelectionInfo {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x1b84c26b_9532_5803_935b_a03bf7e875dc);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISelectionInfo_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SelectRange: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        itemindexrange: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub DeselectRange: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        itemindexrange: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub IsSelected: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        index: i32,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub GetSelectedRanges: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation_Collections"))]
    GetSelectedRanges: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ISupportIncrementalLoading(::windows_core::IUnknown);
impl ISupportIncrementalLoading {
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn LoadMoreItemsAsync(
        &self,
        count: u32,
    ) -> ::windows_core::Result<::windows::Foundation::IAsyncOperation<LoadMoreItemsResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LoadMoreItemsAsync)(
                ::windows_core::Interface::as_raw(this),
                count,
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn HasMoreItems(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HasMoreItems)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(
    ISupportIncrementalLoading,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::RuntimeType for ISupportIncrementalLoading {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_interface::<Self>();
}
unsafe impl ::windows_core::Interface for ISupportIncrementalLoading {
    type Vtable = ISupportIncrementalLoading_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ISupportIncrementalLoading {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xd8f9b586_a64a_5ff8_868e_204e144f2cf4);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISupportIncrementalLoading_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Windows_Foundation")]
    pub LoadMoreItemsAsync: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        count: u32,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    LoadMoreItemsAsync: usize,
    pub HasMoreItems: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IValueConverter(::windows_core::IUnknown);
impl IValueConverter {
    #[doc = "Required features: `\"Windows_UI_Xaml_Interop\"`"]
    #[cfg(feature = "Windows_UI_Xaml_Interop")]
    pub fn Convert<P0, P1, P2>(
        &self,
        value: P0,
        targettype: P1,
        parameter: P2,
        language: &::windows_core::HSTRING,
    ) -> ::windows_core::Result<::windows_core::IInspectable>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
        P1: ::windows_core::IntoParam<
            super::super::super::super::Windows::UI::Xaml::Interop::TypeName,
        >,
        P2: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Convert)(
                ::windows_core::Interface::as_raw(this),
                value.into_param().abi(),
                targettype.into_param().abi(),
                parameter.into_param().abi(),
                ::core::mem::transmute_copy(language),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_UI_Xaml_Interop\"`"]
    #[cfg(feature = "Windows_UI_Xaml_Interop")]
    pub fn ConvertBack<P0, P1, P2>(
        &self,
        value: P0,
        targettype: P1,
        parameter: P2,
        language: &::windows_core::HSTRING,
    ) -> ::windows_core::Result<::windows_core::IInspectable>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
        P1: ::windows_core::IntoParam<
            super::super::super::super::Windows::UI::Xaml::Interop::TypeName,
        >,
        P2: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ConvertBack)(
                ::windows_core::Interface::as_raw(this),
                value.into_param().abi(),
                targettype.into_param().abi(),
                parameter.into_param().abi(),
                ::core::mem::transmute_copy(language),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(
    IValueConverter,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::RuntimeType for IValueConverter {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_interface::<Self>();
}
unsafe impl ::windows_core::Interface for IValueConverter {
    type Vtable = IValueConverter_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IValueConverter {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xafdd2bff_10f5_5173_b7c0_3590bd96cb35);
}
#[repr(C)]
#[doc(hidden)]
pub struct IValueConverter_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Windows_UI_Xaml_Interop")]
    pub Convert: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
        targettype: ::std::mem::MaybeUninit<
            super::super::super::super::Windows::UI::Xaml::Interop::TypeName,
        >,
        parameter: *mut ::core::ffi::c_void,
        language: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI_Xaml_Interop"))]
    Convert: usize,
    #[cfg(feature = "Windows_UI_Xaml_Interop")]
    pub ConvertBack: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
        targettype: ::std::mem::MaybeUninit<
            super::super::super::super::Windows::UI::Xaml::Interop::TypeName,
        >,
        parameter: *mut ::core::ffi::c_void,
        language: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI_Xaml_Interop"))]
    ConvertBack: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct Binding(::windows_core::IUnknown);
impl Binding {
    pub fn Path(&self) -> ::windows_core::Result<super::PropertyPath> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Path)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetPath<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::PropertyPath>,
    {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetPath)(
                ::windows_core::Interface::as_raw(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn Mode(&self) -> ::windows_core::Result<BindingMode> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Mode)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetMode(&self, value: BindingMode) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetMode)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn Source(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Source)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetSource<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetSource)(
                ::windows_core::Interface::as_raw(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn RelativeSource(&self) -> ::windows_core::Result<RelativeSource> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RelativeSource)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetRelativeSource<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<RelativeSource>,
    {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetRelativeSource)(
                ::windows_core::Interface::as_raw(this),
                value.try_into_param()?.abi(),
            )
            .ok()
        }
    }
    pub fn ElementName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ElementName)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetElementName(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetElementName)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn Converter(&self) -> ::windows_core::Result<IValueConverter> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Converter)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetConverter<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<IValueConverter>,
    {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetConverter)(
                ::windows_core::Interface::as_raw(this),
                value.try_into_param()?.abi(),
            )
            .ok()
        }
    }
    pub fn ConverterParameter(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ConverterParameter)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetConverterParameter<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetConverterParameter)(
                ::windows_core::Interface::as_raw(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn ConverterLanguage(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ConverterLanguage)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetConverterLanguage(
        &self,
        value: &::windows_core::HSTRING,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetConverterLanguage)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn FallbackValue(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FallbackValue)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetFallbackValue<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetFallbackValue)(
                ::windows_core::Interface::as_raw(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn TargetNullValue(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TargetNullValue)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetTargetNullValue<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetTargetNullValue)(
                ::windows_core::Interface::as_raw(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn UpdateSourceTrigger(&self) -> ::windows_core::Result<UpdateSourceTrigger> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).UpdateSourceTrigger)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetUpdateSourceTrigger(&self, value: UpdateSourceTrigger) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetUpdateSourceTrigger)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub unsafe fn CreateInstance<P0>(
        baseinterface: P0,
        innerinterface: &mut ::core::option::Option<::windows_core::IInspectable>,
    ) -> ::windows_core::Result<Binding>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        Self::IBindingFactory(|this| unsafe {
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
    #[doc(hidden)]
    pub fn IBindingFactory<R, F: FnOnce(&IBindingFactory) -> ::windows_core::Result<R>>(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<Binding, IBindingFactory> =
            ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for Binding {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for Binding {
    type Vtable = IBinding_Vtbl;
}
unsafe impl ::windows_core::ComInterface for Binding {
    const IID: ::windows_core::GUID = <IBinding as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for Binding {
    const NAME: &'static str = "Microsoft.UI.Xaml.Data.Binding";
}
::windows_core::imp::interface_hierarchy!(
    Binding,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<BindingBase> for Binding {}
impl ::windows_core::CanTryInto<super::DependencyObject> for Binding {}
unsafe impl ::core::marker::Send for Binding {}
unsafe impl ::core::marker::Sync for Binding {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct BindingBase(::windows_core::IUnknown);
impl BindingBase {
    pub unsafe fn CreateInstance<P0>(
        baseinterface: P0,
        innerinterface: &mut ::core::option::Option<::windows_core::IInspectable>,
    ) -> ::windows_core::Result<BindingBase>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        Self::IBindingBaseFactory(|this| unsafe {
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
    #[doc(hidden)]
    pub fn IBindingBaseFactory<R, F: FnOnce(&IBindingBaseFactory) -> ::windows_core::Result<R>>(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<BindingBase, IBindingBaseFactory> =
            ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for BindingBase {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for BindingBase {
    type Vtable = IBindingBase_Vtbl;
}
unsafe impl ::windows_core::ComInterface for BindingBase {
    const IID: ::windows_core::GUID = <IBindingBase as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for BindingBase {
    const NAME: &'static str = "Microsoft.UI.Xaml.Data.BindingBase";
}
::windows_core::imp::interface_hierarchy!(
    BindingBase,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<super::DependencyObject> for BindingBase {}
unsafe impl ::core::marker::Send for BindingBase {}
unsafe impl ::core::marker::Sync for BindingBase {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct BindingExpression(::windows_core::IUnknown);
impl BindingExpression {
    pub fn DataItem(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DataItem)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn ParentBinding(&self) -> ::windows_core::Result<Binding> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ParentBinding)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn UpdateSource(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).UpdateSource)(
                ::windows_core::Interface::as_raw(this),
            )
            .ok()
        }
    }
}
impl ::windows_core::RuntimeType for BindingExpression {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for BindingExpression {
    type Vtable = IBindingExpression_Vtbl;
}
unsafe impl ::windows_core::ComInterface for BindingExpression {
    const IID: ::windows_core::GUID = <IBindingExpression as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for BindingExpression {
    const NAME: &'static str = "Microsoft.UI.Xaml.Data.BindingExpression";
}
::windows_core::imp::interface_hierarchy!(
    BindingExpression,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<BindingExpressionBase> for BindingExpression {}
unsafe impl ::core::marker::Send for BindingExpression {}
unsafe impl ::core::marker::Sync for BindingExpression {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct BindingExpressionBase(::windows_core::IUnknown);
impl BindingExpressionBase {}
impl ::windows_core::RuntimeType for BindingExpressionBase {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for BindingExpressionBase {
    type Vtable = IBindingExpressionBase_Vtbl;
}
unsafe impl ::windows_core::ComInterface for BindingExpressionBase {
    const IID: ::windows_core::GUID = <IBindingExpressionBase as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for BindingExpressionBase {
    const NAME: &'static str = "Microsoft.UI.Xaml.Data.BindingExpressionBase";
}
::windows_core::imp::interface_hierarchy!(
    BindingExpressionBase,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for BindingExpressionBase {}
unsafe impl ::core::marker::Sync for BindingExpressionBase {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct BindingOperations(::windows_core::IUnknown);
impl BindingOperations {
    pub fn SetBinding<P0, P1, P2>(target: P0, dp: P1, binding: P2) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::DependencyObject>,
        P1: ::windows_core::IntoParam<super::DependencyProperty>,
        P2: ::windows_core::TryIntoParam<BindingBase>,
    {
        Self::IBindingOperationsStatics(|this| unsafe {
            (::windows_core::Interface::vtable(this).SetBinding)(
                ::windows_core::Interface::as_raw(this),
                target.try_into_param()?.abi(),
                dp.into_param().abi(),
                binding.try_into_param()?.abi(),
            )
            .ok()
        })
    }
    #[doc(hidden)]
    pub fn IBindingOperationsStatics<
        R,
        F: FnOnce(&IBindingOperationsStatics) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<
            BindingOperations,
            IBindingOperationsStatics,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for BindingOperations {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for BindingOperations {
    type Vtable = IBindingOperations_Vtbl;
}
unsafe impl ::windows_core::ComInterface for BindingOperations {
    const IID: ::windows_core::GUID = <IBindingOperations as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for BindingOperations {
    const NAME: &'static str = "Microsoft.UI.Xaml.Data.BindingOperations";
}
::windows_core::imp::interface_hierarchy!(
    BindingOperations,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for BindingOperations {}
unsafe impl ::core::marker::Sync for BindingOperations {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct CollectionViewSource(::windows_core::IUnknown);
impl CollectionViewSource {
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
            CollectionViewSource,
            ::windows_core::imp::IGenericFactory,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Source(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Source)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetSource<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetSource)(
                ::windows_core::Interface::as_raw(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn View(&self) -> ::windows_core::Result<ICollectionView> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).View)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn IsSourceGrouped(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsSourceGrouped)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetIsSourceGrouped(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetIsSourceGrouped)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn ItemsPath(&self) -> ::windows_core::Result<super::PropertyPath> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ItemsPath)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetItemsPath<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::PropertyPath>,
    {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetItemsPath)(
                ::windows_core::Interface::as_raw(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn SourceProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::ICollectionViewSourceStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SourceProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn ViewProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::ICollectionViewSourceStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ViewProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn IsSourceGroupedProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::ICollectionViewSourceStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsSourceGroupedProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn ItemsPathProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::ICollectionViewSourceStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ItemsPathProperty)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
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
    #[doc(hidden)]
    pub fn ICollectionViewSourceStatics<
        R,
        F: FnOnce(&ICollectionViewSourceStatics) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<
            CollectionViewSource,
            ICollectionViewSourceStatics,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for CollectionViewSource {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for CollectionViewSource {
    type Vtable = ICollectionViewSource_Vtbl;
}
unsafe impl ::windows_core::ComInterface for CollectionViewSource {
    const IID: ::windows_core::GUID = <ICollectionViewSource as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for CollectionViewSource {
    const NAME: &'static str = "Microsoft.UI.Xaml.Data.CollectionViewSource";
}
::windows_core::imp::interface_hierarchy!(
    CollectionViewSource,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<super::DependencyObject> for CollectionViewSource {}
unsafe impl ::core::marker::Send for CollectionViewSource {}
unsafe impl ::core::marker::Sync for CollectionViewSource {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct CurrentChangingEventArgs(::windows_core::IUnknown);
impl CurrentChangingEventArgs {
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
    pub fn IsCancelable(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsCancelable)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub unsafe fn CreateInstance<P0>(
        baseinterface: P0,
        innerinterface: &mut ::core::option::Option<::windows_core::IInspectable>,
    ) -> ::windows_core::Result<CurrentChangingEventArgs>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        Self::ICurrentChangingEventArgsFactory(|this| unsafe {
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
    pub unsafe fn CreateWithCancelableParameter<P0>(
        iscancelable: bool,
        baseinterface: P0,
        innerinterface: &mut ::core::option::Option<::windows_core::IInspectable>,
    ) -> ::windows_core::Result<CurrentChangingEventArgs>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        Self::ICurrentChangingEventArgsFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateWithCancelableParameter)(
                ::windows_core::Interface::as_raw(this),
                iscancelable,
                baseinterface.into_param().abi(),
                innerinterface as *mut _ as _,
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ICurrentChangingEventArgsFactory<
        R,
        F: FnOnce(&ICurrentChangingEventArgsFactory) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<
            CurrentChangingEventArgs,
            ICurrentChangingEventArgsFactory,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for CurrentChangingEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for CurrentChangingEventArgs {
    type Vtable = ICurrentChangingEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for CurrentChangingEventArgs {
    const IID: ::windows_core::GUID =
        <ICurrentChangingEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for CurrentChangingEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Data.CurrentChangingEventArgs";
}
::windows_core::imp::interface_hierarchy!(
    CurrentChangingEventArgs,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for CurrentChangingEventArgs {}
unsafe impl ::core::marker::Sync for CurrentChangingEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct DataErrorsChangedEventArgs(::windows_core::IUnknown);
impl DataErrorsChangedEventArgs {
    pub fn PropertyName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PropertyName)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetPropertyName(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetPropertyName)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn CreateInstance(
        name: &::windows_core::HSTRING,
    ) -> ::windows_core::Result<DataErrorsChangedEventArgs> {
        Self::IDataErrorsChangedEventArgsFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstance)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(name),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IDataErrorsChangedEventArgsFactory<
        R,
        F: FnOnce(&IDataErrorsChangedEventArgsFactory) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<
            DataErrorsChangedEventArgs,
            IDataErrorsChangedEventArgsFactory,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for DataErrorsChangedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for DataErrorsChangedEventArgs {
    type Vtable = IDataErrorsChangedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for DataErrorsChangedEventArgs {
    const IID: ::windows_core::GUID =
        <IDataErrorsChangedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for DataErrorsChangedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Data.DataErrorsChangedEventArgs";
}
::windows_core::imp::interface_hierarchy!(
    DataErrorsChangedEventArgs,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for DataErrorsChangedEventArgs {}
unsafe impl ::core::marker::Sync for DataErrorsChangedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ItemIndexRange(::windows_core::IUnknown);
impl ItemIndexRange {
    pub fn FirstIndex(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FirstIndex)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn Length(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Length)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn LastIndex(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LastIndex)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub unsafe fn CreateInstance<P0>(
        firstindex: i32,
        length: u32,
        baseinterface: P0,
        innerinterface: &mut ::core::option::Option<::windows_core::IInspectable>,
    ) -> ::windows_core::Result<ItemIndexRange>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        Self::IItemIndexRangeFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstance)(
                ::windows_core::Interface::as_raw(this),
                firstindex,
                length,
                baseinterface.into_param().abi(),
                innerinterface as *mut _ as _,
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IItemIndexRangeFactory<
        R,
        F: FnOnce(&IItemIndexRangeFactory) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<ItemIndexRange, IItemIndexRangeFactory> =
            ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for ItemIndexRange {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for ItemIndexRange {
    type Vtable = IItemIndexRange_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ItemIndexRange {
    const IID: ::windows_core::GUID = <IItemIndexRange as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ItemIndexRange {
    const NAME: &'static str = "Microsoft.UI.Xaml.Data.ItemIndexRange";
}
::windows_core::imp::interface_hierarchy!(
    ItemIndexRange,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for ItemIndexRange {}
unsafe impl ::core::marker::Sync for ItemIndexRange {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct PropertyChangedEventArgs(::windows_core::IUnknown);
impl PropertyChangedEventArgs {
    pub fn PropertyName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PropertyName)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub unsafe fn CreateInstance<P0>(
        name: &::windows_core::HSTRING,
        baseinterface: P0,
        innerinterface: &mut ::core::option::Option<::windows_core::IInspectable>,
    ) -> ::windows_core::Result<PropertyChangedEventArgs>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        Self::IPropertyChangedEventArgsFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstance)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(name),
                baseinterface.into_param().abi(),
                innerinterface as *mut _ as _,
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPropertyChangedEventArgsFactory<
        R,
        F: FnOnce(&IPropertyChangedEventArgsFactory) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<
            PropertyChangedEventArgs,
            IPropertyChangedEventArgsFactory,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for PropertyChangedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for PropertyChangedEventArgs {
    type Vtable = IPropertyChangedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PropertyChangedEventArgs {
    const IID: ::windows_core::GUID =
        <IPropertyChangedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PropertyChangedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Data.PropertyChangedEventArgs";
}
::windows_core::imp::interface_hierarchy!(
    PropertyChangedEventArgs,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for PropertyChangedEventArgs {}
unsafe impl ::core::marker::Sync for PropertyChangedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct RelativeSource(::windows_core::IUnknown);
impl RelativeSource {
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
    pub fn Mode(&self) -> ::windows_core::Result<RelativeSourceMode> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Mode)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetMode(&self, value: RelativeSourceMode) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetMode)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub unsafe fn CreateInstance<P0>(
        baseinterface: P0,
        innerinterface: &mut ::core::option::Option<::windows_core::IInspectable>,
    ) -> ::windows_core::Result<RelativeSource>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        Self::IRelativeSourceFactory(|this| unsafe {
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
    #[doc(hidden)]
    pub fn IRelativeSourceFactory<
        R,
        F: FnOnce(&IRelativeSourceFactory) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<RelativeSource, IRelativeSourceFactory> =
            ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for RelativeSource {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for RelativeSource {
    type Vtable = IRelativeSource_Vtbl;
}
unsafe impl ::windows_core::ComInterface for RelativeSource {
    const IID: ::windows_core::GUID = <IRelativeSource as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for RelativeSource {
    const NAME: &'static str = "Microsoft.UI.Xaml.Data.RelativeSource";
}
::windows_core::imp::interface_hierarchy!(
    RelativeSource,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<super::DependencyObject> for RelativeSource {}
unsafe impl ::core::marker::Send for RelativeSource {}
unsafe impl ::core::marker::Sync for RelativeSource {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct BindingMode(pub i32);
impl BindingMode {
    pub const OneWay: Self = Self(1i32);
    pub const OneTime: Self = Self(2i32);
    pub const TwoWay: Self = Self(3i32);
}
impl ::core::marker::Copy for BindingMode {}
impl ::core::clone::Clone for BindingMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for BindingMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for BindingMode {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for BindingMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BindingMode").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for BindingMode {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::from_slice(
            b"enum(Microsoft.UI.Xaml.Data.BindingMode;i4)",
        );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RelativeSourceMode(pub i32);
impl RelativeSourceMode {
    pub const None: Self = Self(0i32);
    pub const TemplatedParent: Self = Self(1i32);
    pub const Self_: Self = Self(2i32);
}
impl ::core::marker::Copy for RelativeSourceMode {}
impl ::core::clone::Clone for RelativeSourceMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RelativeSourceMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for RelativeSourceMode {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for RelativeSourceMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RelativeSourceMode").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for RelativeSourceMode {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::from_slice(
            b"enum(Microsoft.UI.Xaml.Data.RelativeSourceMode;i4)",
        );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct UpdateSourceTrigger(pub i32);
impl UpdateSourceTrigger {
    pub const Default: Self = Self(0i32);
    pub const PropertyChanged: Self = Self(1i32);
    pub const Explicit: Self = Self(2i32);
    pub const LostFocus: Self = Self(3i32);
}
impl ::core::marker::Copy for UpdateSourceTrigger {}
impl ::core::clone::Clone for UpdateSourceTrigger {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UpdateSourceTrigger {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for UpdateSourceTrigger {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for UpdateSourceTrigger {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UpdateSourceTrigger").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for UpdateSourceTrigger {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::from_slice(
            b"enum(Microsoft.UI.Xaml.Data.UpdateSourceTrigger;i4)",
        );
}
#[repr(C)]
pub struct LoadMoreItemsResult {
    pub Count: u32,
}
impl ::core::marker::Copy for LoadMoreItemsResult {}
impl ::core::clone::Clone for LoadMoreItemsResult {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for LoadMoreItemsResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LoadMoreItemsResult").field("Count", &self.Count).finish()
    }
}
impl ::windows_core::TypeKind for LoadMoreItemsResult {
    type TypeKind = ::windows_core::CopyType;
}
impl ::windows_core::RuntimeType for LoadMoreItemsResult {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::from_slice(
            b"struct(Microsoft.UI.Xaml.Data.LoadMoreItemsResult;u4)",
        );
}
impl ::core::cmp::PartialEq for LoadMoreItemsResult {
    fn eq(&self, other: &Self) -> bool {
        self.Count == other.Count
    }
}
impl ::core::cmp::Eq for LoadMoreItemsResult {}
impl ::core::default::Default for LoadMoreItemsResult {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct CurrentChangingEventHandler(pub ::windows_core::IUnknown);
impl CurrentChangingEventHandler {
    pub fn new<
        F: FnMut(
                ::core::option::Option<&::windows_core::IInspectable>,
                ::core::option::Option<&CurrentChangingEventArgs>,
            ) -> ::windows_core::Result<()>
            + ::core::marker::Send
            + 'static,
    >(
        invoke: F,
    ) -> Self {
        let com = CurrentChangingEventHandlerBox::<F> {
            vtable: &CurrentChangingEventHandlerBox::<F>::VTABLE,
            count: ::windows_core::imp::RefCount::new(1),
            invoke,
        };
        unsafe { ::core::mem::transmute(::std::boxed::Box::new(com)) }
    }
    pub fn Invoke<P0, P1>(&self, sender: P0, e: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
        P1: ::windows_core::TryIntoParam<CurrentChangingEventArgs>,
    {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).Invoke)(
                ::windows_core::Interface::as_raw(this),
                sender.into_param().abi(),
                e.try_into_param()?.abi(),
            )
            .ok()
        }
    }
}
#[repr(C)]
struct CurrentChangingEventHandlerBox<
    F: FnMut(
            ::core::option::Option<&::windows_core::IInspectable>,
            ::core::option::Option<&CurrentChangingEventArgs>,
        ) -> ::windows_core::Result<()>
        + ::core::marker::Send
        + 'static,
> {
    vtable: *const CurrentChangingEventHandler_Vtbl,
    invoke: F,
    count: ::windows_core::imp::RefCount,
}
impl<
        F: FnMut(
                ::core::option::Option<&::windows_core::IInspectable>,
                ::core::option::Option<&CurrentChangingEventArgs>,
            ) -> ::windows_core::Result<()>
            + ::core::marker::Send
            + 'static,
    > CurrentChangingEventHandlerBox<F>
{
    const VTABLE: CurrentChangingEventHandler_Vtbl = CurrentChangingEventHandler_Vtbl {
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
        *interface = if *iid == <CurrentChangingEventHandler as ::windows_core::ComInterface>::IID
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
unsafe impl ::windows_core::Interface for CurrentChangingEventHandler {
    type Vtable = CurrentChangingEventHandler_Vtbl;
}
unsafe impl ::windows_core::ComInterface for CurrentChangingEventHandler {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x3d2a98dd_95b3_5fd5_93b4_a1a2599f225c);
}
impl ::windows_core::RuntimeType for CurrentChangingEventHandler {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct CurrentChangingEventHandler_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        sender: *mut ::core::ffi::c_void,
        e: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct PropertyChangedEventHandler(pub ::windows_core::IUnknown);
impl PropertyChangedEventHandler {
    pub fn new<
        F: FnMut(
                ::core::option::Option<&::windows_core::IInspectable>,
                ::core::option::Option<&PropertyChangedEventArgs>,
            ) -> ::windows_core::Result<()>
            + ::core::marker::Send
            + 'static,
    >(
        invoke: F,
    ) -> Self {
        let com = PropertyChangedEventHandlerBox::<F> {
            vtable: &PropertyChangedEventHandlerBox::<F>::VTABLE,
            count: ::windows_core::imp::RefCount::new(1),
            invoke,
        };
        unsafe { ::core::mem::transmute(::std::boxed::Box::new(com)) }
    }
    pub fn Invoke<P0, P1>(&self, sender: P0, e: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
        P1: ::windows_core::TryIntoParam<PropertyChangedEventArgs>,
    {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).Invoke)(
                ::windows_core::Interface::as_raw(this),
                sender.into_param().abi(),
                e.try_into_param()?.abi(),
            )
            .ok()
        }
    }
}
#[repr(C)]
struct PropertyChangedEventHandlerBox<
    F: FnMut(
            ::core::option::Option<&::windows_core::IInspectable>,
            ::core::option::Option<&PropertyChangedEventArgs>,
        ) -> ::windows_core::Result<()>
        + ::core::marker::Send
        + 'static,
> {
    vtable: *const PropertyChangedEventHandler_Vtbl,
    invoke: F,
    count: ::windows_core::imp::RefCount,
}
impl<
        F: FnMut(
                ::core::option::Option<&::windows_core::IInspectable>,
                ::core::option::Option<&PropertyChangedEventArgs>,
            ) -> ::windows_core::Result<()>
            + ::core::marker::Send
            + 'static,
    > PropertyChangedEventHandlerBox<F>
{
    const VTABLE: PropertyChangedEventHandler_Vtbl = PropertyChangedEventHandler_Vtbl {
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
        *interface = if *iid == <PropertyChangedEventHandler as ::windows_core::ComInterface>::IID
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
unsafe impl ::windows_core::Interface for PropertyChangedEventHandler {
    type Vtable = PropertyChangedEventHandler_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PropertyChangedEventHandler {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xe3de52f6_1e32_5da6_bb2d_b5b6096c962d);
}
impl ::windows_core::RuntimeType for PropertyChangedEventHandler {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct PropertyChangedEventHandler_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        sender: *mut ::core::ffi::c_void,
        e: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
