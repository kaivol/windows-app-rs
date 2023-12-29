#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ISecurityDescriptorHelpersStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISecurityDescriptorHelpersStatics {
    type Vtable = ISecurityDescriptorHelpersStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ISecurityDescriptorHelpersStatics {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x14fa9e8d_59f0_5017_852f_3ae24fd5ebb1);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISecurityDescriptorHelpersStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub GetSddlForAppContainerNames: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        accessRequests_array_size: u32,
        accessrequests: *const ::std::mem::MaybeUninit<AppContainerNameAndAccess>,
        principalstringsid: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        principalaccessmask: u32,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub GetSecurityDescriptorBytesFromAppContainerNames:
        unsafe extern "system" fn(
            this: *mut ::core::ffi::c_void,
            accessRequests_array_size: u32,
            accessrequests: *const ::std::mem::MaybeUninit<AppContainerNameAndAccess>,
            principalstringsid: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
            principalaccessmask: u32,
            result_size__: *mut u32,
            result__: *mut *mut u8,
        ) -> ::windows_core::HRESULT,
}
pub struct SecurityDescriptorHelpers;
impl SecurityDescriptorHelpers {
    pub fn GetSddlForAppContainerNames(
        accessrequests: &[AppContainerNameAndAccess],
        principalstringsid: &::windows_core::HSTRING,
        principalaccessmask: u32,
    ) -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ISecurityDescriptorHelpersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetSddlForAppContainerNames)(
                ::windows_core::Interface::as_raw(this),
                accessrequests.len().try_into().unwrap(),
                ::core::mem::transmute(accessrequests.as_ptr()),
                ::core::mem::transmute_copy(principalstringsid),
                principalaccessmask,
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn GetSecurityDescriptorBytesFromAppContainerNames(
        accessrequests: &[AppContainerNameAndAccess],
        principalstringsid: &::windows_core::HSTRING,
        principalaccessmask: u32,
    ) -> ::windows_core::Result<::windows_core::Array<u8>> {
        Self::ISecurityDescriptorHelpersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows_core::Interface::vtable(this)
                .GetSecurityDescriptorBytesFromAppContainerNames)(
                ::windows_core::Interface::as_raw(this),
                accessrequests.len().try_into().unwrap(),
                ::core::mem::transmute(accessrequests.as_ptr()),
                ::core::mem::transmute_copy(principalstringsid),
                principalaccessmask,
                ::windows_core::Array::<u8>::set_abi_len(::std::mem::transmute(&mut result__)),
                result__.as_mut_ptr() as *mut _ as _,
            )
            .and_then(|| result__.assume_init())
        })
    }
    #[doc(hidden)]
    pub fn ISecurityDescriptorHelpersStatics<
        R,
        F: FnOnce(&ISecurityDescriptorHelpersStatics) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<
            SecurityDescriptorHelpers,
            ISecurityDescriptorHelpersStatics,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeName for SecurityDescriptorHelpers {
    const NAME: &'static str = "Microsoft.Windows.Security.AccessControl.SecurityDescriptorHelpers";
}
#[repr(C)]
pub struct AppContainerNameAndAccess {
    pub appContainerName: ::windows_core::HSTRING,
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
impl ::windows_core::TypeKind for AppContainerNameAndAccess {
    type TypeKind = ::windows_core::ValueType;
}
impl ::windows_core::RuntimeType for AppContainerNameAndAccess {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::from_slice(
            b"struct(Microsoft.Windows.Security.AccessControl.AppContainerNameAndAccess;string;u4)",
        );
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
