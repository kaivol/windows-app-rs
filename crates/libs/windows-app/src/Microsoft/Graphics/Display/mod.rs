#[doc(hidden)]
#[repr(transparent)]
pub struct IDisplayAdvancedColorInfo(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IDisplayAdvancedColorInfo {
    type Vtable = IDisplayAdvancedColorInfo_Vtbl;
}
unsafe impl ::windows::core::Interface for IDisplayAdvancedColorInfo {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xb44f0f47_7065_5175_ba3e_714489c85a3e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDisplayAdvancedColorInfo_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CurrentAdvancedColorKind: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut DisplayAdvancedColorKind,
    ) -> ::windows::core::HRESULT,
    pub RedPrimary: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::Point,
    ) -> ::windows::core::HRESULT,
    pub GreenPrimary: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::Point,
    ) -> ::windows::core::HRESULT,
    pub BluePrimary: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::Point,
    ) -> ::windows::core::HRESULT,
    pub WhitePoint: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::Point,
    ) -> ::windows::core::HRESULT,
    pub MaxLuminanceInNits: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows::core::HRESULT,
    pub MinLuminanceInNits: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows::core::HRESULT,
    pub MaxAverageFullFrameLuminanceInNits: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    )
        -> ::windows::core::HRESULT,
    pub SdrWhiteLevelInNits: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows::core::HRESULT,
    pub IsHdrMetadataFormatCurrentlySupported:
        unsafe extern "system" fn(
            this: *mut ::core::ffi::c_void,
            format: DisplayHdrMetadataFormat,
            result__: *mut bool,
        ) -> ::windows::core::HRESULT,
    pub IsAdvancedColorKindAvailable: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        kind: DisplayAdvancedColorKind,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDisplayInformation(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IDisplayInformation {
    type Vtable = IDisplayInformation_Vtbl;
}
unsafe impl ::windows::core::Interface for IDisplayInformation {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xf0d58d4f_84ce_5b27_b222_4f8f7dc0aaeb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDisplayInformation_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "UI_Dispatching")]
    pub DispatcherQueue: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))]
    DispatcherQueue: usize,
    pub IsStereoEnabled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub IsStereoEnabledChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub RemoveIsStereoEnabledChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub GetColorProfileAsync: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub GetColorProfile: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub ColorProfileChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub RemoveColorProfileChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub GetAdvancedColorInfo: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub AdvancedColorInfoChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub RemoveAdvancedColorInfoChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub Destroyed: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub RemoveDestroyed: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDisplayInformationStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IDisplayInformationStatics {
    type Vtable = IDisplayInformationStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IDisplayInformationStatics {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x2de85048_37fa_56c0_ac30_47e2044d7ea8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDisplayInformationStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "UI")]
    pub CreateForWindowId: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        windowid: super::super::UI::WindowId,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI"))]
    CreateForWindowId: usize,
    #[cfg(feature = "UI")]
    pub CreateForDisplayId: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        displayid: super::super::UI::DisplayId,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI"))]
    CreateForDisplayId: usize,
}
#[doc = "*Required features: `\"Graphics_Display\"`*"]
#[repr(transparent)]
pub struct DisplayAdvancedColorInfo(::windows::core::IUnknown);
impl DisplayAdvancedColorInfo {
    pub fn CurrentAdvancedColorKind(&self) -> ::windows::core::Result<DisplayAdvancedColorKind> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CurrentAdvancedColorKind)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<DisplayAdvancedColorKind>(result__)
        }
    }
    pub fn RedPrimary(&self) -> ::windows::core::Result<::windows::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RedPrimary)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::Point>(result__)
        }
    }
    pub fn GreenPrimary(&self) -> ::windows::core::Result<::windows::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GreenPrimary)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::Point>(result__)
        }
    }
    pub fn BluePrimary(&self) -> ::windows::core::Result<::windows::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).BluePrimary)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::Point>(result__)
        }
    }
    pub fn WhitePoint(&self) -> ::windows::core::Result<::windows::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).WhitePoint)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::Point>(result__)
        }
    }
    pub fn MaxLuminanceInNits(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MaxLuminanceInNits)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn MinLuminanceInNits(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MinLuminanceInNits)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn MaxAverageFullFrameLuminanceInNits(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MaxAverageFullFrameLuminanceInNits)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SdrWhiteLevelInNits(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SdrWhiteLevelInNits)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn IsHdrMetadataFormatCurrentlySupported(
        &self,
        format: DisplayHdrMetadataFormat,
    ) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsHdrMetadataFormatCurrentlySupported)(
                ::windows::core::Vtable::as_raw(this),
                format,
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn IsAdvancedColorKindAvailable(
        &self,
        kind: DisplayAdvancedColorKind,
    ) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsAdvancedColorKindAvailable)(
                ::windows::core::Vtable::as_raw(this),
                kind,
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for DisplayAdvancedColorInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DisplayAdvancedColorInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DisplayAdvancedColorInfo {}
impl ::core::fmt::Debug for DisplayAdvancedColorInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DisplayAdvancedColorInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DisplayAdvancedColorInfo {
    const SIGNATURE : ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice ( b"rc(Microsoft.Graphics.Display.DisplayAdvancedColorInfo;{b44f0f47-7065-5175-ba3e-714489c85a3e})" ) ;
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for DisplayAdvancedColorInfo {
    type Vtable = IDisplayAdvancedColorInfo_Vtbl;
}
unsafe impl ::windows::core::Interface for DisplayAdvancedColorInfo {
    const IID: ::windows::core::GUID =
        <IDisplayAdvancedColorInfo as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for DisplayAdvancedColorInfo {
    const NAME: &'static str = "Microsoft.Graphics.Display.DisplayAdvancedColorInfo";
}
::windows::core::interface_hierarchy!(
    DisplayAdvancedColorInfo,
    ::windows::core::IUnknown,
    ::windows::core::IInspectable
);
unsafe impl ::core::marker::Send for DisplayAdvancedColorInfo {}
unsafe impl ::core::marker::Sync for DisplayAdvancedColorInfo {}
#[doc = "*Required features: `\"Graphics_Display\"`*"]
#[repr(transparent)]
pub struct DisplayInformation(::windows::core::IUnknown);
impl DisplayInformation {
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<::windows::Foundation::IClosable>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).Close)(::windows::core::Vtable::as_raw(this))
                .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Dispatching\"`*"]
    #[cfg(feature = "UI_Dispatching")]
    pub fn DispatcherQueue(
        &self,
    ) -> ::windows::core::Result<super::super::UI::Dispatching::DispatcherQueue> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DispatcherQueue)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::super::UI::Dispatching::DispatcherQueue>(result__)
        }
    }
    pub fn IsStereoEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsStereoEnabled)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn IsStereoEnabledChanged(
        &self,
        handler: &::windows::Foundation::TypedEventHandler<
            DisplayInformation,
            ::windows::core::IInspectable,
        >,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsStereoEnabledChanged)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(handler),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveIsStereoEnabledChanged(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).RemoveIsStereoEnabledChanged)(
                ::windows::core::Vtable::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn GetColorProfileAsync(
        &self,
    ) -> ::windows::core::Result<
        ::windows::Foundation::IAsyncOperation<::windows::Storage::Streams::IRandomAccessStream>,
    > {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetColorProfileAsync)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::IAsyncOperation<
                ::windows::Storage::Streams::IRandomAccessStream,
            >>(result__)
        }
    }
    pub fn GetColorProfile(
        &self,
    ) -> ::windows::core::Result<::windows::Storage::Streams::IRandomAccessStream> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetColorProfile)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Storage::Streams::IRandomAccessStream>(result__)
        }
    }
    pub fn ColorProfileChanged(
        &self,
        handler: &::windows::Foundation::TypedEventHandler<
            DisplayInformation,
            ::windows::core::IInspectable,
        >,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ColorProfileChanged)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(handler),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveColorProfileChanged(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).RemoveColorProfileChanged)(
                ::windows::core::Vtable::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn GetAdvancedColorInfo(&self) -> ::windows::core::Result<DisplayAdvancedColorInfo> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetAdvancedColorInfo)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<DisplayAdvancedColorInfo>(result__)
        }
    }
    pub fn AdvancedColorInfoChanged(
        &self,
        handler: &::windows::Foundation::TypedEventHandler<
            DisplayInformation,
            ::windows::core::IInspectable,
        >,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AdvancedColorInfoChanged)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(handler),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveAdvancedColorInfoChanged(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).RemoveAdvancedColorInfoChanged)(
                ::windows::core::Vtable::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn Destroyed(
        &self,
        handler: &::windows::Foundation::TypedEventHandler<
            DisplayInformation,
            ::windows::core::IInspectable,
        >,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Destroyed)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(handler),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveDestroyed(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).RemoveDestroyed)(
                ::windows::core::Vtable::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI\"`*"]
    #[cfg(feature = "UI")]
    pub fn CreateForWindowId(
        windowid: super::super::UI::WindowId,
    ) -> ::windows::core::Result<DisplayInformation> {
        Self::IDisplayInformationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateForWindowId)(
                ::windows::core::Vtable::as_raw(this),
                windowid,
                result__.as_mut_ptr(),
            )
            .from_abi::<DisplayInformation>(result__)
        })
    }
    #[doc = "*Required features: `\"UI\"`*"]
    #[cfg(feature = "UI")]
    pub fn CreateForDisplayId(
        displayid: super::super::UI::DisplayId,
    ) -> ::windows::core::Result<DisplayInformation> {
        Self::IDisplayInformationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateForDisplayId)(
                ::windows::core::Vtable::as_raw(this),
                displayid,
                result__.as_mut_ptr(),
            )
            .from_abi::<DisplayInformation>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IDisplayInformationStatics<
        R,
        F: FnOnce(&IDisplayInformationStatics) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<
            DisplayInformation,
            IDisplayInformationStatics,
        > = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for DisplayInformation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DisplayInformation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DisplayInformation {}
impl ::core::fmt::Debug for DisplayInformation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DisplayInformation").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DisplayInformation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.Graphics.Display.DisplayInformation;{f0d58d4f-84ce-5b27-b222-4f8f7dc0aaeb})",
    );
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for DisplayInformation {
    type Vtable = IDisplayInformation_Vtbl;
}
unsafe impl ::windows::core::Interface for DisplayInformation {
    const IID: ::windows::core::GUID = <IDisplayInformation as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for DisplayInformation {
    const NAME: &'static str = "Microsoft.Graphics.Display.DisplayInformation";
}
::windows::core::interface_hierarchy!(
    DisplayInformation,
    ::windows::core::IUnknown,
    ::windows::core::IInspectable
);
impl ::core::convert::TryFrom<DisplayInformation> for ::windows::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: DisplayInformation) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&DisplayInformation> for ::windows::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &DisplayInformation) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&DisplayInformation>
    for ::windows::core::InParam<'a, ::windows::Foundation::IClosable>
{
    type Error = ::windows::core::Error;
    fn try_from(value: &DisplayInformation) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for DisplayInformation {}
unsafe impl ::core::marker::Sync for DisplayInformation {}
#[doc = "*Required features: `\"Graphics_Display\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DisplayAdvancedColorKind(pub i32);
impl DisplayAdvancedColorKind {
    pub const StandardDynamicRange: Self = Self(0i32);
    pub const WideColorGamut: Self = Self(1i32);
    pub const HighDynamicRange: Self = Self(2i32);
}
impl ::core::marker::Copy for DisplayAdvancedColorKind {}
impl ::core::clone::Clone for DisplayAdvancedColorKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DisplayAdvancedColorKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DisplayAdvancedColorKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for DisplayAdvancedColorKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DisplayAdvancedColorKind").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DisplayAdvancedColorKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.Graphics.Display.DisplayAdvancedColorKind;i4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Graphics_Display\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DisplayHdrMetadataFormat(pub i32);
impl DisplayHdrMetadataFormat {
    pub const Hdr10: Self = Self(0i32);
    pub const Hdr10Plus: Self = Self(1i32);
}
impl ::core::marker::Copy for DisplayHdrMetadataFormat {}
impl ::core::clone::Clone for DisplayHdrMetadataFormat {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DisplayHdrMetadataFormat {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DisplayHdrMetadataFormat {
    type Abi = Self;
}
impl ::core::fmt::Debug for DisplayHdrMetadataFormat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DisplayHdrMetadataFormat").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DisplayHdrMetadataFormat {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.Graphics.Display.DisplayHdrMetadataFormat;i4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
