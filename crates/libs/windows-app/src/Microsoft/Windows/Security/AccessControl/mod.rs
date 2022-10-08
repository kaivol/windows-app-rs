#[doc(hidden)]
#[repr(transparent)]
pub struct ISecurityDescriptorHelpersStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ISecurityDescriptorHelpersStatics {
    type Vtable = ISecurityDescriptorHelpersStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for ISecurityDescriptorHelpersStatics {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x14fa9e8d_59f0_5017_852f_3ae24fd5ebb1);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISecurityDescriptorHelpersStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetSddlForAppContainerNames: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        accessRequests_array_size: u32,
        accessrequests: *const ::core::mem::ManuallyDrop<AppContainerNameAndAccess>,
        principalstringsid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        principalaccessmask: u32,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub GetSecurityDescriptorBytesFromAppContainerNames:
        unsafe extern "system" fn(
            this: *mut ::core::ffi::c_void,
            accessRequests_array_size: u32,
            accessrequests: *const ::core::mem::ManuallyDrop<AppContainerNameAndAccess>,
            principalstringsid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
            principalaccessmask: u32,
            result_size__: *mut u32,
            result__: *mut *mut u8,
        ) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Windows_Security_AccessControl\"`*"]
pub struct SecurityDescriptorHelpers;
impl SecurityDescriptorHelpers {
    pub fn GetSddlForAppContainerNames(
        accessrequests: &[AppContainerNameAndAccess],
        principalstringsid: &::windows::core::HSTRING,
        principalaccessmask: u32,
    ) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ISecurityDescriptorHelpersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetSddlForAppContainerNames)(
                ::windows::core::Vtable::as_raw(this),
                accessrequests.len() as u32,
                ::core::mem::transmute(accessrequests.as_ptr()),
                ::core::mem::transmute_copy(principalstringsid),
                principalaccessmask,
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn GetSecurityDescriptorBytesFromAppContainerNames(
        accessrequests: &[AppContainerNameAndAccess],
        principalstringsid: &::windows::core::HSTRING,
        principalaccessmask: u32,
    ) -> ::windows::core::Result<::windows::core::Array<u8>> {
        Self::ISecurityDescriptorHelpersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetSecurityDescriptorBytesFromAppContainerNames)(
                ::windows::core::Vtable::as_raw(this),
                accessrequests.len() as u32,
                ::core::mem::transmute(accessrequests.as_ptr()),
                ::core::mem::transmute_copy(principalstringsid),
                principalaccessmask,
                ::windows::core::Array::<u8>::set_abi_len(result__.assume_init_mut()),
                result__.as_mut_ptr() as *mut _ as _,
            )
            .and_then(|| result__.assume_init())
        })
    }
    #[doc(hidden)]
    pub fn ISecurityDescriptorHelpersStatics<
        R,
        F: FnOnce(&ISecurityDescriptorHelpersStatics) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<
            SecurityDescriptorHelpers,
            ISecurityDescriptorHelpersStatics,
        > = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for SecurityDescriptorHelpers {
    const NAME: &'static str = "Microsoft.Windows.Security.AccessControl.SecurityDescriptorHelpers";
}
#[repr(C)]
#[doc = "*Required features: `\"Windows_Security_AccessControl\"`*"]
pub struct AppContainerNameAndAccess {
    pub appContainerName: ::windows::core::HSTRING,
    pub accessMask: u32,
}
impl ::core::clone::Clone for AppContainerNameAndAccess {
    fn clone(&self) -> Self {
        Self {
            appContainerName: self.appContainerName.clone(),
            accessMask: self.accessMask,
        }
    }
}
impl ::core::fmt::Debug for AppContainerNameAndAccess {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AppContainerNameAndAccess")
            .field("appContainerName", &self.appContainerName)
            .field("accessMask", &self.accessMask)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for AppContainerNameAndAccess {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
unsafe impl ::windows::core::RuntimeType for AppContainerNameAndAccess {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"struct(Microsoft.Windows.Security.AccessControl.AppContainerNameAndAccess;string;u4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(from.clone())
    }
}
impl ::core::cmp::PartialEq for AppContainerNameAndAccess {
    fn eq(&self, other: &Self) -> bool {
        self.appContainerName == other.appContainerName && self.accessMask == other.accessMask
    }
}
impl ::core::cmp::Eq for AppContainerNameAndAccess {}
impl ::core::default::Default for AppContainerNameAndAccess {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
