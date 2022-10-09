#[doc = "*Required features: `\"UI_Xaml_Interop\"`*"]
#[repr(transparent)]
pub struct IBindableIterable(::windows::core::IUnknown);
impl IBindableIterable {
    pub fn First(&self) -> ::windows::core::Result<IBindableIterator> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).First)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<IBindableIterator>(result__)
        }
    }
}
::windows::core::interface_hierarchy!(
    IBindableIterable,
    ::windows::core::IUnknown,
    ::windows::core::IInspectable
);
impl ::core::clone::Clone for IBindableIterable {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IBindableIterable {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IBindableIterable {}
impl ::core::fmt::Debug for IBindableIterable {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IBindableIterable").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IBindableIterable {
    const SIGNATURE: ::windows::core::ConstBuffer =
        ::windows::core::ConstBuffer::from_slice(b"{036d2c08-df29-41af-8aa2-d774be62ba6f}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for IBindableIterable {
    type Vtable = IBindableIterable_Vtbl;
}
unsafe impl ::windows::core::Interface for IBindableIterable {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x036d2c08_df29_41af_8aa2_d774be62ba6f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBindableIterable_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub First: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Xaml_Interop\"`*"]
#[repr(transparent)]
pub struct IBindableIterator(::windows::core::IUnknown);
impl IBindableIterator {
    pub fn Current(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Current)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn HasCurrent(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).HasCurrent)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn MoveNext(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MoveNext)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        }
    }
}
::windows::core::interface_hierarchy!(
    IBindableIterator,
    ::windows::core::IUnknown,
    ::windows::core::IInspectable
);
impl ::core::clone::Clone for IBindableIterator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IBindableIterator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IBindableIterator {}
impl ::core::fmt::Debug for IBindableIterator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IBindableIterator").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IBindableIterator {
    const SIGNATURE: ::windows::core::ConstBuffer =
        ::windows::core::ConstBuffer::from_slice(b"{6a1d6c07-076d-49f2-8314-f52c9c9a8331}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for IBindableIterator {
    type Vtable = IBindableIterator_Vtbl;
}
unsafe impl ::windows::core::Interface for IBindableIterator {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x6a1d6c07_076d_49f2_8314_f52c9c9a8331);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBindableIterator_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Current: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub HasCurrent: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub MoveNext: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Xaml_Interop\"`*"]
#[repr(transparent)]
pub struct IBindableObservableVector(::windows::core::IUnknown);
impl IBindableObservableVector {
    pub fn VectorChanged(
        &self,
        handler: &BindableVectorChangedEventHandler,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).VectorChanged)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(handler),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveVectorChanged(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).RemoveVectorChanged)(
                ::windows::core::Vtable::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn First(&self) -> ::windows::core::Result<IBindableIterator> {
        let this = &::windows::core::Interface::cast::<IBindableIterable>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).First)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<IBindableIterator>(result__)
        }
    }
    pub fn GetAt(&self, index: u32) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IBindableVector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetAt)(
                ::windows::core::Vtable::as_raw(this),
                index,
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn Size(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<IBindableVector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Size)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<u32>(result__)
        }
    }
    pub fn GetView(&self) -> ::windows::core::Result<IBindableVectorView> {
        let this = &::windows::core::Interface::cast::<IBindableVector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetView)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<IBindableVectorView>(result__)
        }
    }
    pub fn IndexOf<'a, P0>(&self, value: P0, index: &mut u32) -> ::windows::core::Result<bool>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IInspectable>>,
    {
        let this = &::windows::core::Interface::cast::<IBindableVector>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IndexOf)(
                ::windows::core::Vtable::as_raw(this),
                value.into().abi(),
                index,
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetAt<'a, P0>(&self, index: u32, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IInspectable>>,
    {
        let this = &::windows::core::Interface::cast::<IBindableVector>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetAt)(
                ::windows::core::Vtable::as_raw(this),
                index,
                value.into().abi(),
            )
            .ok()
        }
    }
    pub fn InsertAt<'a, P0>(&self, index: u32, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IInspectable>>,
    {
        let this = &::windows::core::Interface::cast::<IBindableVector>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).InsertAt)(
                ::windows::core::Vtable::as_raw(this),
                index,
                value.into().abi(),
            )
            .ok()
        }
    }
    pub fn RemoveAt(&self, index: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IBindableVector>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).RemoveAt)(
                ::windows::core::Vtable::as_raw(this),
                index,
            )
            .ok()
        }
    }
    pub fn Append<'a, P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IInspectable>>,
    {
        let this = &::windows::core::Interface::cast::<IBindableVector>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).Append)(
                ::windows::core::Vtable::as_raw(this),
                value.into().abi(),
            )
            .ok()
        }
    }
    pub fn RemoveAtEnd(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IBindableVector>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).RemoveAtEnd)(::windows::core::Vtable::as_raw(
                this,
            ))
            .ok()
        }
    }
    pub fn Clear(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IBindableVector>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).Clear)(::windows::core::Vtable::as_raw(this))
                .ok()
        }
    }
}
::windows::core::interface_hierarchy!(
    IBindableObservableVector,
    ::windows::core::IUnknown,
    ::windows::core::IInspectable
);
impl ::core::convert::TryFrom<IBindableObservableVector> for IBindableIterable {
    type Error = ::windows::core::Error;
    fn try_from(value: IBindableObservableVector) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IBindableObservableVector> for IBindableIterable {
    type Error = ::windows::core::Error;
    fn try_from(value: &IBindableObservableVector) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&IBindableObservableVector>
    for ::windows::core::InParam<'a, IBindableIterable>
{
    type Error = ::windows::core::Error;
    fn try_from(value: &IBindableObservableVector) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<IBindableObservableVector> for IBindableVector {
    type Error = ::windows::core::Error;
    fn try_from(value: IBindableObservableVector) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IBindableObservableVector> for IBindableVector {
    type Error = ::windows::core::Error;
    fn try_from(value: &IBindableObservableVector) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&IBindableObservableVector>
    for ::windows::core::InParam<'a, IBindableVector>
{
    type Error = ::windows::core::Error;
    fn try_from(value: &IBindableObservableVector) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::clone::Clone for IBindableObservableVector {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IBindableObservableVector {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IBindableObservableVector {}
impl ::core::fmt::Debug for IBindableObservableVector {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IBindableObservableVector").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IBindableObservableVector {
    const SIGNATURE: ::windows::core::ConstBuffer =
        ::windows::core::ConstBuffer::from_slice(b"{fe1eb536-7e7f-4f90-ac9a-474984aae512}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for IBindableObservableVector {
    type Vtable = IBindableObservableVector_Vtbl;
}
unsafe impl ::windows::core::Interface for IBindableObservableVector {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xfe1eb536_7e7f_4f90_ac9a_474984aae512);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBindableObservableVector_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub VectorChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub RemoveVectorChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Xaml_Interop\"`*"]
#[repr(transparent)]
pub struct IBindableVector(::windows::core::IUnknown);
impl IBindableVector {
    pub fn GetAt(&self, index: u32) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetAt)(
                ::windows::core::Vtable::as_raw(this),
                index,
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn Size(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Size)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<u32>(result__)
        }
    }
    pub fn GetView(&self) -> ::windows::core::Result<IBindableVectorView> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetView)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<IBindableVectorView>(result__)
        }
    }
    pub fn IndexOf<'a, P0>(&self, value: P0, index: &mut u32) -> ::windows::core::Result<bool>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IndexOf)(
                ::windows::core::Vtable::as_raw(this),
                value.into().abi(),
                index,
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetAt<'a, P0>(&self, index: u32, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IInspectable>>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetAt)(
                ::windows::core::Vtable::as_raw(this),
                index,
                value.into().abi(),
            )
            .ok()
        }
    }
    pub fn InsertAt<'a, P0>(&self, index: u32, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IInspectable>>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).InsertAt)(
                ::windows::core::Vtable::as_raw(this),
                index,
                value.into().abi(),
            )
            .ok()
        }
    }
    pub fn RemoveAt(&self, index: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).RemoveAt)(
                ::windows::core::Vtable::as_raw(this),
                index,
            )
            .ok()
        }
    }
    pub fn Append<'a, P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IInspectable>>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).Append)(
                ::windows::core::Vtable::as_raw(this),
                value.into().abi(),
            )
            .ok()
        }
    }
    pub fn RemoveAtEnd(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).RemoveAtEnd)(::windows::core::Vtable::as_raw(
                this,
            ))
            .ok()
        }
    }
    pub fn Clear(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).Clear)(::windows::core::Vtable::as_raw(this))
                .ok()
        }
    }
    pub fn First(&self) -> ::windows::core::Result<IBindableIterator> {
        let this = &::windows::core::Interface::cast::<IBindableIterable>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).First)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<IBindableIterator>(result__)
        }
    }
}
::windows::core::interface_hierarchy!(
    IBindableVector,
    ::windows::core::IUnknown,
    ::windows::core::IInspectable
);
impl ::core::convert::TryFrom<IBindableVector> for IBindableIterable {
    type Error = ::windows::core::Error;
    fn try_from(value: IBindableVector) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IBindableVector> for IBindableIterable {
    type Error = ::windows::core::Error;
    fn try_from(value: &IBindableVector) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&IBindableVector>
    for ::windows::core::InParam<'a, IBindableIterable>
{
    type Error = ::windows::core::Error;
    fn try_from(value: &IBindableVector) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::clone::Clone for IBindableVector {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IBindableVector {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IBindableVector {}
impl ::core::fmt::Debug for IBindableVector {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IBindableVector").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IBindableVector {
    const SIGNATURE: ::windows::core::ConstBuffer =
        ::windows::core::ConstBuffer::from_slice(b"{393de7de-6fd0-4c0d-bb71-47244a113e93}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for IBindableVector {
    type Vtable = IBindableVector_Vtbl;
}
unsafe impl ::windows::core::Interface for IBindableVector {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x393de7de_6fd0_4c0d_bb71_47244a113e93);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBindableVector_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetAt: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        index: u32,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub Size: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut u32,
    ) -> ::windows::core::HRESULT,
    pub GetView: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub IndexOf: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
        index: *mut u32,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub SetAt: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        index: u32,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub InsertAt: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        index: u32,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub RemoveAt: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        index: u32,
    ) -> ::windows::core::HRESULT,
    pub Append: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub RemoveAtEnd:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Clear:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Xaml_Interop\"`*"]
#[repr(transparent)]
pub struct IBindableVectorView(::windows::core::IUnknown);
impl IBindableVectorView {
    pub fn GetAt(&self, index: u32) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetAt)(
                ::windows::core::Vtable::as_raw(this),
                index,
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn Size(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Size)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<u32>(result__)
        }
    }
    pub fn IndexOf<'a, P0>(&self, value: P0, index: &mut u32) -> ::windows::core::Result<bool>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IndexOf)(
                ::windows::core::Vtable::as_raw(this),
                value.into().abi(),
                index,
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn First(&self) -> ::windows::core::Result<IBindableIterator> {
        let this = &::windows::core::Interface::cast::<IBindableIterable>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).First)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<IBindableIterator>(result__)
        }
    }
}
::windows::core::interface_hierarchy!(
    IBindableVectorView,
    ::windows::core::IUnknown,
    ::windows::core::IInspectable
);
impl ::core::convert::TryFrom<IBindableVectorView> for IBindableIterable {
    type Error = ::windows::core::Error;
    fn try_from(value: IBindableVectorView) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IBindableVectorView> for IBindableIterable {
    type Error = ::windows::core::Error;
    fn try_from(value: &IBindableVectorView) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&IBindableVectorView>
    for ::windows::core::InParam<'a, IBindableIterable>
{
    type Error = ::windows::core::Error;
    fn try_from(value: &IBindableVectorView) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::clone::Clone for IBindableVectorView {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IBindableVectorView {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IBindableVectorView {}
impl ::core::fmt::Debug for IBindableVectorView {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IBindableVectorView").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IBindableVectorView {
    const SIGNATURE: ::windows::core::ConstBuffer =
        ::windows::core::ConstBuffer::from_slice(b"{346dd6e7-976e-4bc3-815d-ece243bc0f33}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for IBindableVectorView {
    type Vtable = IBindableVectorView_Vtbl;
}
unsafe impl ::windows::core::Interface for IBindableVectorView {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x346dd6e7_976e_4bc3_815d_ece243bc0f33);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBindableVectorView_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetAt: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        index: u32,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub Size: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut u32,
    ) -> ::windows::core::HRESULT,
    pub IndexOf: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
        index: *mut u32,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Xaml_Interop\"`*"]
#[repr(transparent)]
pub struct INotifyCollectionChanged(::windows::core::IUnknown);
impl INotifyCollectionChanged {
    pub fn CollectionChanged(
        &self,
        handler: &NotifyCollectionChangedEventHandler,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CollectionChanged)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(handler),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveCollectionChanged(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).RemoveCollectionChanged)(
                ::windows::core::Vtable::as_raw(this),
                token,
            )
            .ok()
        }
    }
}
::windows::core::interface_hierarchy!(
    INotifyCollectionChanged,
    ::windows::core::IUnknown,
    ::windows::core::IInspectable
);
impl ::core::clone::Clone for INotifyCollectionChanged {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for INotifyCollectionChanged {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INotifyCollectionChanged {}
impl ::core::fmt::Debug for INotifyCollectionChanged {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INotifyCollectionChanged").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for INotifyCollectionChanged {
    const SIGNATURE: ::windows::core::ConstBuffer =
        ::windows::core::ConstBuffer::from_slice(b"{530155e1-28a5-5693-87ce-30724d95a06d}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for INotifyCollectionChanged {
    type Vtable = INotifyCollectionChanged_Vtbl;
}
unsafe impl ::windows::core::Interface for INotifyCollectionChanged {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x530155e1_28a5_5693_87ce_30724d95a06d);
}
#[repr(C)]
#[doc(hidden)]
pub struct INotifyCollectionChanged_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CollectionChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub RemoveCollectionChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct INotifyCollectionChangedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for INotifyCollectionChangedEventArgs {
    type Vtable = INotifyCollectionChangedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for INotifyCollectionChangedEventArgs {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xda049ff2_d2e0_5fe8_8c7b_f87f26060b6f);
}
#[repr(C)]
#[doc(hidden)]
pub struct INotifyCollectionChangedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Action: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut NotifyCollectionChangedAction,
    ) -> ::windows::core::HRESULT,
    pub NewItems: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub OldItems: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub NewStartingIndex: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut i32,
    ) -> ::windows::core::HRESULT,
    pub OldStartingIndex: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut i32,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct INotifyCollectionChangedEventArgsFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for INotifyCollectionChangedEventArgsFactory {
    type Vtable = INotifyCollectionChangedEventArgsFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for INotifyCollectionChangedEventArgsFactory {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x5108eba4_4892_5a20_8374_a96815e0fd27);
}
#[repr(C)]
#[doc(hidden)]
pub struct INotifyCollectionChangedEventArgsFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
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
    ) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Xaml_Interop\"`*"]
#[repr(transparent)]
pub struct NotifyCollectionChangedEventArgs(::windows::core::IUnknown);
impl NotifyCollectionChangedEventArgs {
    pub fn Action(&self) -> ::windows::core::Result<NotifyCollectionChangedAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Action)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<NotifyCollectionChangedAction>(result__)
        }
    }
    pub fn NewItems(&self) -> ::windows::core::Result<IBindableVector> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NewItems)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<IBindableVector>(result__)
        }
    }
    pub fn OldItems(&self) -> ::windows::core::Result<IBindableVector> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).OldItems)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<IBindableVector>(result__)
        }
    }
    pub fn NewStartingIndex(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NewStartingIndex)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<i32>(result__)
        }
    }
    pub fn OldStartingIndex(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).OldStartingIndex)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<i32>(result__)
        }
    }
    pub fn CreateInstanceWithAllParameters<'a, P0, E0, P1, E1>(
        action: NotifyCollectionChangedAction,
        newitems: P0,
        olditems: P1,
        newindex: i32,
        oldindex: i32,
    ) -> ::windows::core::Result<NotifyCollectionChangedEventArgs>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, IBindableVector>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
        P1: ::std::convert::TryInto<::windows::core::InParam<'a, IBindableVector>, Error = E1>,
        E1: ::std::convert::Into<::windows::core::Error>,
    {
        Self::INotifyCollectionChangedEventArgsFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateInstanceWithAllParameters)(
                ::windows::core::Vtable::as_raw(this),
                action,
                newitems.try_into().map_err(|e| e.into())?.abi(),
                olditems.try_into().map_err(|e| e.into())?.abi(),
                newindex,
                oldindex,
                ::core::ptr::null_mut(),
                &mut ::core::option::Option::<::windows::core::IInspectable>::None as *mut _ as _,
                result__.as_mut_ptr(),
            )
            .from_abi::<NotifyCollectionChangedEventArgs>(result__)
        })
    }
    pub fn CreateInstanceWithAllParameters_compose<'a, P0, E0, P1, E1, T>(
        action: NotifyCollectionChangedAction,
        newitems: P0,
        olditems: P1,
        newindex: i32,
        oldindex: i32,
        compose: T,
    ) -> ::windows::core::Result<NotifyCollectionChangedEventArgs>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, IBindableVector>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
        P1: ::std::convert::TryInto<::windows::core::InParam<'a, IBindableVector>, Error = E1>,
        E1: ::std::convert::Into<::windows::core::Error>,
        T: ::windows::core::Compose,
    {
        Self::INotifyCollectionChangedEventArgsFactory(|this| unsafe {
            let (derived__, base__) = ::windows::core::Compose::compose(compose);
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateInstanceWithAllParameters)(
                ::windows::core::Vtable::as_raw(this),
                action,
                newitems.try_into().map_err(|e| e.into())?.abi(),
                olditems.try_into().map_err(|e| e.into())?.abi(),
                newindex,
                oldindex,
                ::core::mem::transmute_copy(&derived__),
                base__ as *mut _ as _,
                result__.as_mut_ptr(),
            )
            .from_abi::<NotifyCollectionChangedEventArgs>(result__)
        })
    }
    #[doc(hidden)]
    pub fn INotifyCollectionChangedEventArgsFactory<
        R,
        F: FnOnce(&INotifyCollectionChangedEventArgsFactory) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<
            NotifyCollectionChangedEventArgs,
            INotifyCollectionChangedEventArgsFactory,
        > = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for NotifyCollectionChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for NotifyCollectionChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for NotifyCollectionChangedEventArgs {}
impl ::core::fmt::Debug for NotifyCollectionChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NotifyCollectionChangedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for NotifyCollectionChangedEventArgs {
    const SIGNATURE : ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice ( b"rc(Microsoft.UI.Xaml.Interop.NotifyCollectionChangedEventArgs;{da049ff2-d2e0-5fe8-8c7b-f87f26060b6f})" ) ;
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for NotifyCollectionChangedEventArgs {
    type Vtable = INotifyCollectionChangedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for NotifyCollectionChangedEventArgs {
    const IID: ::windows::core::GUID =
        <INotifyCollectionChangedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for NotifyCollectionChangedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Interop.NotifyCollectionChangedEventArgs";
}
::windows::core::interface_hierarchy!(
    NotifyCollectionChangedEventArgs,
    ::windows::core::IUnknown,
    ::windows::core::IInspectable
);
unsafe impl ::core::marker::Send for NotifyCollectionChangedEventArgs {}
unsafe impl ::core::marker::Sync for NotifyCollectionChangedEventArgs {}
#[doc = "*Required features: `\"UI_Xaml_Interop\"`*"]
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
unsafe impl ::windows::core::Abi for NotifyCollectionChangedAction {
    type Abi = Self;
}
impl ::core::fmt::Debug for NotifyCollectionChangedAction {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NotifyCollectionChangedAction").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for NotifyCollectionChangedAction {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Xaml.Interop.NotifyCollectionChangedAction;i4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_Xaml_Interop\"`*"]
#[repr(transparent)]
pub struct BindableVectorChangedEventHandler(pub ::windows::core::IUnknown);
impl BindableVectorChangedEventHandler {
    pub fn new<
        F: FnMut(
                &::core::option::Option<IBindableObservableVector>,
                &::core::option::Option<::windows::core::IInspectable>,
            ) -> ::windows::core::Result<()>
            + ::core::marker::Send
            + 'static,
    >(
        invoke: F,
    ) -> Self {
        let com = BindableVectorChangedEventHandlerBox::<F> {
            vtable: &BindableVectorChangedEventHandlerBox::<F>::VTABLE,
            count: ::windows::core::RefCount::new(1),
            invoke,
        };
        unsafe { ::core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    pub fn Invoke<'a, P0, E0, P1>(&self, vector: P0, e: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<'a, IBindableObservableVector>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IInspectable>>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).Invoke)(
                ::windows::core::Vtable::as_raw(this),
                vector.try_into().map_err(|e| e.into())?.abi(),
                e.into().abi(),
            )
            .ok()
        }
    }
}
#[repr(C)]
struct BindableVectorChangedEventHandlerBox<
    F: FnMut(
            &::core::option::Option<IBindableObservableVector>,
            &::core::option::Option<::windows::core::IInspectable>,
        ) -> ::windows::core::Result<()>
        + ::core::marker::Send
        + 'static,
> {
    vtable: *const BindableVectorChangedEventHandler_Vtbl,
    invoke: F,
    count: ::windows::core::RefCount,
}
impl<
        F: FnMut(
                &::core::option::Option<IBindableObservableVector>,
                &::core::option::Option<::windows::core::IInspectable>,
            ) -> ::windows::core::Result<()>
            + ::core::marker::Send
            + 'static,
    > BindableVectorChangedEventHandlerBox<F>
{
    const VTABLE: BindableVectorChangedEventHandler_Vtbl = BindableVectorChangedEventHandler_Vtbl {
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
        *interface = if iid
            == &<BindableVectorChangedEventHandler as ::windows::core::Interface>::IID
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
        vector: *mut ::core::ffi::c_void,
        e: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        ((*this).invoke)(::core::mem::transmute(&vector), ::core::mem::transmute(&e)).into()
    }
}
impl ::core::clone::Clone for BindableVectorChangedEventHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BindableVectorChangedEventHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BindableVectorChangedEventHandler {}
impl ::core::fmt::Debug for BindableVectorChangedEventHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BindableVectorChangedEventHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for BindableVectorChangedEventHandler {
    type Vtable = BindableVectorChangedEventHandler_Vtbl;
}
unsafe impl ::windows::core::Interface for BindableVectorChangedEventHandler {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x624cd4e1_d007_43b1_9c03_af4d3e6258c4);
}
unsafe impl ::windows::core::RuntimeType for BindableVectorChangedEventHandler {
    const SIGNATURE: ::windows::core::ConstBuffer =
        ::windows::core::ConstBuffer::from_slice(b"{624cd4e1-d007-43b1-9c03-af4d3e6258c4}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct BindableVectorChangedEventHandler_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        vector: *mut ::core::ffi::c_void,
        e: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Xaml_Interop\"`*"]
#[repr(transparent)]
pub struct NotifyCollectionChangedEventHandler(pub ::windows::core::IUnknown);
impl NotifyCollectionChangedEventHandler {
    pub fn new<
        F: FnMut(
                &::core::option::Option<::windows::core::IInspectable>,
                &::core::option::Option<NotifyCollectionChangedEventArgs>,
            ) -> ::windows::core::Result<()>
            + ::core::marker::Send
            + 'static,
    >(
        invoke: F,
    ) -> Self {
        let com = NotifyCollectionChangedEventHandlerBox::<F> {
            vtable: &NotifyCollectionChangedEventHandlerBox::<F>::VTABLE,
            count: ::windows::core::RefCount::new(1),
            invoke,
        };
        unsafe { ::core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    pub fn Invoke<'a, P0, P1>(&self, sender: P0, e: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IInspectable>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, NotifyCollectionChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).Invoke)(
                ::windows::core::Vtable::as_raw(this),
                sender.into().abi(),
                e.into().abi(),
            )
            .ok()
        }
    }
}
#[repr(C)]
struct NotifyCollectionChangedEventHandlerBox<
    F: FnMut(
            &::core::option::Option<::windows::core::IInspectable>,
            &::core::option::Option<NotifyCollectionChangedEventArgs>,
        ) -> ::windows::core::Result<()>
        + ::core::marker::Send
        + 'static,
> {
    vtable: *const NotifyCollectionChangedEventHandler_Vtbl,
    invoke: F,
    count: ::windows::core::RefCount,
}
impl<
        F: FnMut(
                &::core::option::Option<::windows::core::IInspectable>,
                &::core::option::Option<NotifyCollectionChangedEventArgs>,
            ) -> ::windows::core::Result<()>
            + ::core::marker::Send
            + 'static,
    > NotifyCollectionChangedEventHandlerBox<F>
{
    const VTABLE: NotifyCollectionChangedEventHandler_Vtbl =
        NotifyCollectionChangedEventHandler_Vtbl {
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
        *interface = if iid
            == &<NotifyCollectionChangedEventHandler as ::windows::core::Interface>::IID
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
impl ::core::clone::Clone for NotifyCollectionChangedEventHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for NotifyCollectionChangedEventHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for NotifyCollectionChangedEventHandler {}
impl ::core::fmt::Debug for NotifyCollectionChangedEventHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NotifyCollectionChangedEventHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for NotifyCollectionChangedEventHandler {
    type Vtable = NotifyCollectionChangedEventHandler_Vtbl;
}
unsafe impl ::windows::core::Interface for NotifyCollectionChangedEventHandler {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x8b0909dc_2005_5d93_bf8a_725f017baa8d);
}
unsafe impl ::windows::core::RuntimeType for NotifyCollectionChangedEventHandler {
    const SIGNATURE: ::windows::core::ConstBuffer =
        ::windows::core::ConstBuffer::from_slice(b"{8b0909dc-2005-5d93-bf8a-725f017baa8d}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct NotifyCollectionChangedEventHandler_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        sender: *mut ::core::ffi::c_void,
        e: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
