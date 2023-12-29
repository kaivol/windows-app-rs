#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IBindableIterable(::windows_core::IUnknown);
impl IBindableIterable {
    pub fn First(&self) -> ::windows_core::Result<IBindableIterator> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).First)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(
    IBindableIterable,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::RuntimeType for IBindableIterable {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_interface::<Self>();
}
unsafe impl ::windows_core::Interface for IBindableIterable {
    type Vtable = IBindableIterable_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IBindableIterable {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x036d2c08_df29_41af_8aa2_d774be62ba6f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBindableIterable_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub First: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IBindableIterator(::windows_core::IUnknown);
impl IBindableIterator {
    pub fn Current(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Current)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn HasCurrent(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HasCurrent)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn MoveNext(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MoveNext)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(
    IBindableIterator,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::RuntimeType for IBindableIterator {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_interface::<Self>();
}
unsafe impl ::windows_core::Interface for IBindableIterator {
    type Vtable = IBindableIterator_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IBindableIterator {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x6a1d6c07_076d_49f2_8314_f52c9c9a8331);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBindableIterator_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Current: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub HasCurrent: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub MoveNext: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IBindableObservableVector(::windows_core::IUnknown);
impl IBindableObservableVector {
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn VectorChanged<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<BindableVectorChangedEventHandler>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).VectorChanged)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveVectorChanged(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveVectorChanged)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn First(&self) -> ::windows_core::Result<IBindableIterator> {
        let this = &::windows_core::ComInterface::cast::<IBindableIterable>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).First)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn GetAt(&self, index: u32) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = &::windows_core::ComInterface::cast::<IBindableVector>(self)?;
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
    pub fn Size(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::ComInterface::cast::<IBindableVector>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Size)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn GetView(&self) -> ::windows_core::Result<IBindableVectorView> {
        let this = &::windows_core::ComInterface::cast::<IBindableVector>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetView)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn IndexOf<P0>(&self, value: P0, index: &mut u32) -> ::windows_core::Result<bool>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = &::windows_core::ComInterface::cast::<IBindableVector>(self)?;
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
    pub fn SetAt<P0>(&self, index: u32, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = &::windows_core::ComInterface::cast::<IBindableVector>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetAt)(
                ::windows_core::Interface::as_raw(this),
                index,
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn InsertAt<P0>(&self, index: u32, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = &::windows_core::ComInterface::cast::<IBindableVector>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).InsertAt)(
                ::windows_core::Interface::as_raw(this),
                index,
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn RemoveAt(&self, index: u32) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IBindableVector>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveAt)(
                ::windows_core::Interface::as_raw(this),
                index,
            )
            .ok()
        }
    }
    pub fn Append<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = &::windows_core::ComInterface::cast::<IBindableVector>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).Append)(
                ::windows_core::Interface::as_raw(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn RemoveAtEnd(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IBindableVector>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveAtEnd)(
                ::windows_core::Interface::as_raw(this),
            )
            .ok()
        }
    }
    pub fn Clear(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IBindableVector>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).Clear)(::windows_core::Interface::as_raw(this))
                .ok()
        }
    }
}
::windows_core::imp::interface_hierarchy!(
    IBindableObservableVector,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<IBindableIterable> for IBindableObservableVector {}
impl ::windows_core::CanTryInto<IBindableVector> for IBindableObservableVector {}
impl ::windows_core::RuntimeType for IBindableObservableVector {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_interface::<Self>();
}
unsafe impl ::windows_core::Interface for IBindableObservableVector {
    type Vtable = IBindableObservableVector_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IBindableObservableVector {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xfe1eb536_7e7f_4f90_ac9a_474984aae512);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBindableObservableVector_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Windows_Foundation")]
    pub VectorChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    VectorChanged: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub RemoveVectorChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RemoveVectorChanged: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IBindableVector(::windows_core::IUnknown);
impl IBindableVector {
    pub fn GetAt(&self, index: u32) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = self;
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
    pub fn Size(&self) -> ::windows_core::Result<u32> {
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
    pub fn GetView(&self) -> ::windows_core::Result<IBindableVectorView> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetView)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn IndexOf<P0>(&self, value: P0, index: &mut u32) -> ::windows_core::Result<bool>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = self;
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
    pub fn SetAt<P0>(&self, index: u32, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetAt)(
                ::windows_core::Interface::as_raw(this),
                index,
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn InsertAt<P0>(&self, index: u32, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).InsertAt)(
                ::windows_core::Interface::as_raw(this),
                index,
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn RemoveAt(&self, index: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveAt)(
                ::windows_core::Interface::as_raw(this),
                index,
            )
            .ok()
        }
    }
    pub fn Append<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).Append)(
                ::windows_core::Interface::as_raw(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn RemoveAtEnd(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveAtEnd)(
                ::windows_core::Interface::as_raw(this),
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
    pub fn First(&self) -> ::windows_core::Result<IBindableIterator> {
        let this = &::windows_core::ComInterface::cast::<IBindableIterable>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).First)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(
    IBindableVector,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<IBindableIterable> for IBindableVector {}
impl ::windows_core::RuntimeType for IBindableVector {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_interface::<Self>();
}
unsafe impl ::windows_core::Interface for IBindableVector {
    type Vtable = IBindableVector_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IBindableVector {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x393de7de_6fd0_4c0d_bb71_47244a113e93);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBindableVector_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub GetAt: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        index: u32,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub Size: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut u32,
    ) -> ::windows_core::HRESULT,
    pub GetView: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub IndexOf: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
        index: *mut u32,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetAt: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        index: u32,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub InsertAt: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        index: u32,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub RemoveAt: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        index: u32,
    ) -> ::windows_core::HRESULT,
    pub Append: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub RemoveAtEnd:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Clear: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IBindableVectorView(::windows_core::IUnknown);
impl IBindableVectorView {
    pub fn GetAt(&self, index: u32) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = self;
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
    pub fn Size(&self) -> ::windows_core::Result<u32> {
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
    pub fn IndexOf<P0>(&self, value: P0, index: &mut u32) -> ::windows_core::Result<bool>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = self;
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
    pub fn First(&self) -> ::windows_core::Result<IBindableIterator> {
        let this = &::windows_core::ComInterface::cast::<IBindableIterable>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).First)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(
    IBindableVectorView,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<IBindableIterable> for IBindableVectorView {}
impl ::windows_core::RuntimeType for IBindableVectorView {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_interface::<Self>();
}
unsafe impl ::windows_core::Interface for IBindableVectorView {
    type Vtable = IBindableVectorView_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IBindableVectorView {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x346dd6e7_976e_4bc3_815d_ece243bc0f33);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBindableVectorView_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub GetAt: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        index: u32,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub Size: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut u32,
    ) -> ::windows_core::HRESULT,
    pub IndexOf: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
        index: *mut u32,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct INotifyCollectionChanged(::windows_core::IUnknown);
impl INotifyCollectionChanged {
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn CollectionChanged<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<NotifyCollectionChangedEventHandler>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CollectionChanged)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveCollectionChanged(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveCollectionChanged)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
}
::windows_core::imp::interface_hierarchy!(
    INotifyCollectionChanged,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::RuntimeType for INotifyCollectionChanged {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_interface::<Self>();
}
unsafe impl ::windows_core::Interface for INotifyCollectionChanged {
    type Vtable = INotifyCollectionChanged_Vtbl;
}
unsafe impl ::windows_core::ComInterface for INotifyCollectionChanged {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x530155e1_28a5_5693_87ce_30724d95a06d);
}
#[repr(C)]
#[doc(hidden)]
pub struct INotifyCollectionChanged_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Windows_Foundation")]
    pub CollectionChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    CollectionChanged: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub RemoveCollectionChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RemoveCollectionChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct INotifyCollectionChangedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for INotifyCollectionChangedEventArgs {
    type Vtable = INotifyCollectionChangedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for INotifyCollectionChangedEventArgs {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xda049ff2_d2e0_5fe8_8c7b_f87f26060b6f);
}
#[repr(C)]
#[doc(hidden)]
pub struct INotifyCollectionChangedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Action: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut NotifyCollectionChangedAction,
    ) -> ::windows_core::HRESULT,
    pub NewItems: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub OldItems: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub NewStartingIndex: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut i32,
    ) -> ::windows_core::HRESULT,
    pub OldStartingIndex: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut i32,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct INotifyCollectionChangedEventArgsFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for INotifyCollectionChangedEventArgsFactory {
    type Vtable = INotifyCollectionChangedEventArgsFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for INotifyCollectionChangedEventArgsFactory {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x5108eba4_4892_5a20_8374_a96815e0fd27);
}
#[repr(C)]
#[doc(hidden)]
pub struct INotifyCollectionChangedEventArgsFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateInstanceWithAllParameters: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        action: NotifyCollectionChangedAction,
        newitems: *mut ::core::ffi::c_void,
        olditems: *mut ::core::ffi::c_void,
        newindex: i32,
        oldindex: i32,
        baseinterface: *mut ::core::ffi::c_void,
        innerinterface: *mut *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct NotifyCollectionChangedEventArgs(::windows_core::IUnknown);
impl NotifyCollectionChangedEventArgs {
    pub fn Action(&self) -> ::windows_core::Result<NotifyCollectionChangedAction> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Action)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn NewItems(&self) -> ::windows_core::Result<IBindableVector> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NewItems)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn OldItems(&self) -> ::windows_core::Result<IBindableVector> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OldItems)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn NewStartingIndex(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NewStartingIndex)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn OldStartingIndex(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OldStartingIndex)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for NotifyCollectionChangedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for NotifyCollectionChangedEventArgs {
    type Vtable = INotifyCollectionChangedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for NotifyCollectionChangedEventArgs {
    const IID: ::windows_core::GUID =
        <INotifyCollectionChangedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for NotifyCollectionChangedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Interop.NotifyCollectionChangedEventArgs";
}
::windows_core::imp::interface_hierarchy!(
    NotifyCollectionChangedEventArgs,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for NotifyCollectionChangedEventArgs {}
unsafe impl ::core::marker::Sync for NotifyCollectionChangedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NotifyCollectionChangedAction(pub i32);
impl NotifyCollectionChangedAction {
    pub const Add: Self = Self(0i32);
    pub const Remove: Self = Self(1i32);
    pub const Replace: Self = Self(2i32);
    pub const Move: Self = Self(3i32);
    pub const Reset: Self = Self(4i32);
}
impl ::core::marker::Copy for NotifyCollectionChangedAction {}
impl ::core::clone::Clone for NotifyCollectionChangedAction {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NotifyCollectionChangedAction {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for NotifyCollectionChangedAction {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for NotifyCollectionChangedAction {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NotifyCollectionChangedAction").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for NotifyCollectionChangedAction {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::from_slice(
            b"enum(Microsoft.UI.Xaml.Interop.NotifyCollectionChangedAction;i4)",
        );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct BindableVectorChangedEventHandler(pub ::windows_core::IUnknown);
impl BindableVectorChangedEventHandler {
    pub fn new<
        F: FnMut(
                ::core::option::Option<&IBindableObservableVector>,
                ::core::option::Option<&::windows_core::IInspectable>,
            ) -> ::windows_core::Result<()>
            + ::core::marker::Send
            + 'static,
    >(
        invoke: F,
    ) -> Self {
        let com = BindableVectorChangedEventHandlerBox::<F> {
            vtable: &BindableVectorChangedEventHandlerBox::<F>::VTABLE,
            count: ::windows_core::imp::RefCount::new(1),
            invoke,
        };
        unsafe { ::core::mem::transmute(::std::boxed::Box::new(com)) }
    }
    pub fn Invoke<P0, P1>(&self, vector: P0, e: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<IBindableObservableVector>,
        P1: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).Invoke)(
                ::windows_core::Interface::as_raw(this),
                vector.try_into_param()?.abi(),
                e.into_param().abi(),
            )
            .ok()
        }
    }
}
#[repr(C)]
struct BindableVectorChangedEventHandlerBox<
    F: FnMut(
            ::core::option::Option<&IBindableObservableVector>,
            ::core::option::Option<&::windows_core::IInspectable>,
        ) -> ::windows_core::Result<()>
        + ::core::marker::Send
        + 'static,
> {
    vtable: *const BindableVectorChangedEventHandler_Vtbl,
    invoke: F,
    count: ::windows_core::imp::RefCount,
}
impl<
        F: FnMut(
                ::core::option::Option<&IBindableObservableVector>,
                ::core::option::Option<&::windows_core::IInspectable>,
            ) -> ::windows_core::Result<()>
            + ::core::marker::Send
            + 'static,
    > BindableVectorChangedEventHandlerBox<F>
{
    const VTABLE: BindableVectorChangedEventHandler_Vtbl = BindableVectorChangedEventHandler_Vtbl {
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
        *interface = if *iid
            == <BindableVectorChangedEventHandler as ::windows_core::ComInterface>::IID
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
        vector: *mut ::core::ffi::c_void,
        e: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        ((*this).invoke)(
            ::windows_core::from_raw_borrowed(&vector),
            ::windows_core::from_raw_borrowed(&e),
        )
        .into()
    }
}
unsafe impl ::windows_core::Interface for BindableVectorChangedEventHandler {
    type Vtable = BindableVectorChangedEventHandler_Vtbl;
}
unsafe impl ::windows_core::ComInterface for BindableVectorChangedEventHandler {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x624cd4e1_d007_43b1_9c03_af4d3e6258c4);
}
impl ::windows_core::RuntimeType for BindableVectorChangedEventHandler {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct BindableVectorChangedEventHandler_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        vector: *mut ::core::ffi::c_void,
        e: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct NotifyCollectionChangedEventHandler(pub ::windows_core::IUnknown);
impl NotifyCollectionChangedEventHandler {
    pub fn new<
        F: FnMut(
                ::core::option::Option<&::windows_core::IInspectable>,
                ::core::option::Option<&NotifyCollectionChangedEventArgs>,
            ) -> ::windows_core::Result<()>
            + ::core::marker::Send
            + 'static,
    >(
        invoke: F,
    ) -> Self {
        let com = NotifyCollectionChangedEventHandlerBox::<F> {
            vtable: &NotifyCollectionChangedEventHandlerBox::<F>::VTABLE,
            count: ::windows_core::imp::RefCount::new(1),
            invoke,
        };
        unsafe { ::core::mem::transmute(::std::boxed::Box::new(com)) }
    }
    pub fn Invoke<P0, P1>(&self, sender: P0, e: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
        P1: ::windows_core::TryIntoParam<NotifyCollectionChangedEventArgs>,
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
struct NotifyCollectionChangedEventHandlerBox<
    F: FnMut(
            ::core::option::Option<&::windows_core::IInspectable>,
            ::core::option::Option<&NotifyCollectionChangedEventArgs>,
        ) -> ::windows_core::Result<()>
        + ::core::marker::Send
        + 'static,
> {
    vtable: *const NotifyCollectionChangedEventHandler_Vtbl,
    invoke: F,
    count: ::windows_core::imp::RefCount,
}
impl<
        F: FnMut(
                ::core::option::Option<&::windows_core::IInspectable>,
                ::core::option::Option<&NotifyCollectionChangedEventArgs>,
            ) -> ::windows_core::Result<()>
            + ::core::marker::Send
            + 'static,
    > NotifyCollectionChangedEventHandlerBox<F>
{
    const VTABLE: NotifyCollectionChangedEventHandler_Vtbl =
        NotifyCollectionChangedEventHandler_Vtbl {
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
        *interface = if *iid
            == <NotifyCollectionChangedEventHandler as ::windows_core::ComInterface>::IID
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
unsafe impl ::windows_core::Interface for NotifyCollectionChangedEventHandler {
    type Vtable = NotifyCollectionChangedEventHandler_Vtbl;
}
unsafe impl ::windows_core::ComInterface for NotifyCollectionChangedEventHandler {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x8b0909dc_2005_5d93_bf8a_725f017baa8d);
}
impl ::windows_core::RuntimeType for NotifyCollectionChangedEventHandler {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct NotifyCollectionChangedEventHandler_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        sender: *mut ::core::ffi::c_void,
        e: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
