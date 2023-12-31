#[doc = "Required features: `\"Windows_Win32_Storage_Packaging_Appx\"`"]
#[cfg(feature = "Windows_Win32_Storage_Packaging_Appx")]
#[inline]
pub unsafe fn MddBootstrapInitialize<P0>(
    majorminorversion: u32,
    versiontag: P0,
    minversion: ::windows::Win32::Storage::Packaging::Appx::PACKAGE_VERSION,
) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    #[link(name = "microsoft.windowsappruntime.bootstrap")]
    extern "system" {
        pub fn MddBootstrapInitialize(
            majorminorversion: u32,
            versiontag: ::windows_core::PCWSTR,
            minversion: ::windows::Win32::Storage::Packaging::Appx::PACKAGE_VERSION,
        ) -> ::windows_core::HRESULT;
    }
    MddBootstrapInitialize(
        majorminorversion,
        versiontag.into_param().abi(),
        ::core::mem::transmute(minversion),
    )
    .ok()
}
#[doc = "Required features: `\"Windows_Win32_Storage_Packaging_Appx\"`"]
#[cfg(feature = "Windows_Win32_Storage_Packaging_Appx")]
#[inline]
pub unsafe fn MddBootstrapInitialize2<P0>(
    majorminorversion: u32,
    versiontag: P0,
    minversion: ::windows::Win32::Storage::Packaging::Appx::PACKAGE_VERSION,
    options: MddBootstrapInitializeOptions,
) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    #[link(name = "microsoft.windowsappruntime.bootstrap")]
    extern "system" {
        pub fn MddBootstrapInitialize2(
            majorminorversion: u32,
            versiontag: ::windows_core::PCWSTR,
            minversion: ::windows::Win32::Storage::Packaging::Appx::PACKAGE_VERSION,
            options: MddBootstrapInitializeOptions,
        ) -> ::windows_core::HRESULT;
    }
    MddBootstrapInitialize2(
        majorminorversion,
        versiontag.into_param().abi(),
        ::core::mem::transmute(minversion),
        options,
    )
    .ok()
}
#[inline]
pub unsafe fn MddBootstrapShutdown() {
    #[link(name = "microsoft.windowsappruntime.bootstrap")]
    extern "system" {
        pub fn MddBootstrapShutdown();
    }
    MddBootstrapShutdown()
}
pub const MddBootstrapInitializeOptions_None: MddBootstrapInitializeOptions =
    MddBootstrapInitializeOptions(0i32);
pub const MddBootstrapInitializeOptions_OnError_DebugBreak: MddBootstrapInitializeOptions =
    MddBootstrapInitializeOptions(1i32);
pub const MddBootstrapInitializeOptions_OnError_DebugBreak_IfDebuggerAttached:
    MddBootstrapInitializeOptions = MddBootstrapInitializeOptions(2i32);
pub const MddBootstrapInitializeOptions_OnError_FailFast: MddBootstrapInitializeOptions =
    MddBootstrapInitializeOptions(4i32);
pub const MddBootstrapInitializeOptions_OnNoMatch_ShowUI: MddBootstrapInitializeOptions =
    MddBootstrapInitializeOptions(8i32);
pub const MddBootstrapInitializeOptions_OnPackageIdentity_NOOP: MddBootstrapInitializeOptions =
    MddBootstrapInitializeOptions(16i32);
pub const WASR_IDENTITY_PUBLISHER: ::windows_core::PCSTR = ::windows_core::s!(
    "CN=Microsoft Corporation, O=Microsoft Corporation, L=Redmond, S=Washington, C=US"
);
pub const WASR_IDENTITY_PUBLISHERID: ::windows_core::PCSTR = ::windows_core::s!("8wekyb3d8bbwe");
pub const WASR_IDENTITY_PUBLISHERID_W: ::windows_core::PCWSTR = ::windows_core::w!("8wekyb3d8bbwe");
pub const WASR_IDENTITY_PUBLISHER_W: ::windows_core::PCWSTR = ::windows_core::w!(
    "CN=Microsoft Corporation, O=Microsoft Corporation, L=Redmond, S=Washington, C=US"
);
pub const WASR_PKG_DDLM_ARM64_FAMILY: ::windows_core::PCSTR =
    ::windows_core::s!("Microsoft.WinAppRuntime.DDLM.4000.1049.117.0-a6_8wekyb3d8bbwe");
pub const WASR_PKG_DDLM_ARM64_FAMILY_W: ::windows_core::PCWSTR =
    ::windows_core::w!("Microsoft.WinAppRuntime.DDLM.4000.1049.117.0-a6_8wekyb3d8bbwe");
pub const WASR_PKG_DDLM_X64_FAMILY: ::windows_core::PCSTR =
    ::windows_core::s!("Microsoft.WinAppRuntime.DDLM.4000.1049.117.0-x6_8wekyb3d8bbwe");
pub const WASR_PKG_DDLM_X64_FAMILY_W: ::windows_core::PCWSTR =
    ::windows_core::w!("Microsoft.WinAppRuntime.DDLM.4000.1049.117.0-x6_8wekyb3d8bbwe");
pub const WASR_PKG_DDLM_X86_FAMILY: ::windows_core::PCSTR =
    ::windows_core::s!("Microsoft.WinAppRuntime.DDLM.4000.1049.117.0-x8_8wekyb3d8bbwe");
pub const WASR_PKG_DDLM_X86_FAMILY_W: ::windows_core::PCWSTR =
    ::windows_core::w!("Microsoft.WinAppRuntime.DDLM.4000.1049.117.0-x8_8wekyb3d8bbwe");
pub const WASR_PKG_FRAMEWORK_FAMILY: ::windows_core::PCSTR =
    ::windows_core::s!("Microsoft.WindowsAppRuntime.1.4_8wekyb3d8bbwe");
pub const WASR_PKG_FRAMEWORK_FAMILY_W: ::windows_core::PCWSTR =
    ::windows_core::w!("Microsoft.WindowsAppRuntime.1.4_8wekyb3d8bbwe");
pub const WASR_PKG_MAIN_FAMILY: ::windows_core::PCSTR =
    ::windows_core::s!("MicrosoftCorporationII.WinAppRuntime.Main.1.4_8wekyb3d8bbwe");
pub const WASR_PKG_MAIN_FAMILY_W: ::windows_core::PCWSTR =
    ::windows_core::w!("MicrosoftCorporationII.WinAppRuntime.Main.1.4_8wekyb3d8bbwe");
pub const WASR_PKG_SINGLETON_FAMILY: ::windows_core::PCSTR =
    ::windows_core::s!("MicrosoftCorporationII.WinAppRuntime.Singleton_8wekyb3d8bbwe");
pub const WASR_PKG_SINGLETON_FAMILY_W: ::windows_core::PCWSTR =
    ::windows_core::w!("MicrosoftCorporationII.WinAppRuntime.Singleton_8wekyb3d8bbwe");
pub const WASR_VERSION_BUILD: u16 = 117u16;
pub const WASR_VERSION_DOTQUADSTRING: ::windows_core::PCSTR = ::windows_core::s!("4000.1049.117.0");
pub const WASR_VERSION_DOTQUADSTRING_W: ::windows_core::PCWSTR =
    ::windows_core::w!("4000.1049.117.0");
pub const WASR_VERSION_MAJOR: u16 = 4000u16;
pub const WASR_VERSION_MINOR: u16 = 1049u16;
pub const WASR_VERSION_REVISION: u16 = 0u16;
pub const WASR_VERSION_UINT64: u64 = 1125904412270985216u64;
pub const WAS_RELEASE_CHANNEL: ::windows_core::PCSTR = ::windows_core::s!("stable");
pub const WAS_RELEASE_CHANNEL_W: ::windows_core::PCWSTR = ::windows_core::w!("stable");
pub const WAS_RELEASE_FMT_VERSION_STAG: ::windows_core::PCSTR = ::windows_core::s!("");
pub const WAS_RELEASE_FMT_VERSION_STAG_W: ::windows_core::PCWSTR = ::windows_core::w!("");
pub const WAS_RELEASE_FMT_VERSION_TAG: ::windows_core::PCSTR = ::windows_core::s!("");
pub const WAS_RELEASE_FMT_VERSION_TAG_W: ::windows_core::PCWSTR = ::windows_core::w!("");
pub const WAS_RELEASE_MAJOR: u32 = 1u32;
pub const WAS_RELEASE_MAJORMINOR: u32 = 65540u32;
pub const WAS_RELEASE_MINOR: u32 = 4u32;
pub const WAS_RELEASE_PATCH: u32 = 0u32;
pub const WAS_RELEASE_VERSION_STAG: ::windows_core::PCSTR = ::windows_core::s!("");
pub const WAS_RELEASE_VERSION_STAG_W: ::windows_core::PCWSTR = ::windows_core::w!("");
pub const WAS_RELEASE_VERSION_TAG: ::windows_core::PCSTR = ::windows_core::s!("");
pub const WAS_RELEASE_VERSION_TAG_W: ::windows_core::PCWSTR = ::windows_core::w!("");
pub const WINDOWSAPPSDK_RELEASE_CHANNEL: ::windows_core::PCSTR = ::windows_core::s!("stable");
pub const WINDOWSAPPSDK_RELEASE_CHANNEL_W: ::windows_core::PCWSTR = ::windows_core::w!("stable");
pub const WINDOWSAPPSDK_RELEASE_FORMATTED_VERSION_SHORTTAG: ::windows_core::PCSTR =
    ::windows_core::s!("");
pub const WINDOWSAPPSDK_RELEASE_FORMATTED_VERSION_SHORTTAG_W: ::windows_core::PCWSTR =
    ::windows_core::w!("");
pub const WINDOWSAPPSDK_RELEASE_FORMATTED_VERSION_TAG: ::windows_core::PCSTR =
    ::windows_core::s!("");
pub const WINDOWSAPPSDK_RELEASE_FORMATTED_VERSION_TAG_W: ::windows_core::PCWSTR =
    ::windows_core::w!("");
pub const WINDOWSAPPSDK_RELEASE_MAJOR: u32 = 1u32;
pub const WINDOWSAPPSDK_RELEASE_MAJORMINOR: u32 = 65540u32;
pub const WINDOWSAPPSDK_RELEASE_MINOR: u32 = 4u32;
pub const WINDOWSAPPSDK_RELEASE_PATCH: u32 = 0u32;
pub const WINDOWSAPPSDK_RELEASE_VERSION_SHORTTAG: ::windows_core::PCSTR = ::windows_core::s!("");
pub const WINDOWSAPPSDK_RELEASE_VERSION_SHORTTAG_W: ::windows_core::PCWSTR = ::windows_core::w!("");
pub const WINDOWSAPPSDK_RELEASE_VERSION_TAG: ::windows_core::PCSTR = ::windows_core::s!("");
pub const WINDOWSAPPSDK_RELEASE_VERSION_TAG_W: ::windows_core::PCWSTR = ::windows_core::w!("");
pub const WINDOWSAPPSDK_RUNTIME_IDENTITY_PUBLISHER: ::windows_core::PCSTR = ::windows_core::s!(
    "CN=Microsoft Corporation, O=Microsoft Corporation, L=Redmond, S=Washington, C=US"
);
pub const WINDOWSAPPSDK_RUNTIME_IDENTITY_PUBLISHERID: ::windows_core::PCSTR =
    ::windows_core::s!("8wekyb3d8bbwe");
pub const WINDOWSAPPSDK_RUNTIME_IDENTITY_PUBLISHERID_W: ::windows_core::PCWSTR =
    ::windows_core::w!("8wekyb3d8bbwe");
pub const WINDOWSAPPSDK_RUNTIME_IDENTITY_PUBLISHER_W: ::windows_core::PCWSTR = ::windows_core::w!(
    "CN=Microsoft Corporation, O=Microsoft Corporation, L=Redmond, S=Washington, C=US"
);
pub const WINDOWSAPPSDK_RUNTIME_PACKAGE_DDLM_ARM64_PACKAGEFAMILYNAME: ::windows_core::PCSTR =
    ::windows_core::s!("Microsoft.WinAppRuntime.DDLM.4000.1049.117.0-a6_8wekyb3d8bbwe");
pub const WINDOWSAPPSDK_RUNTIME_PACKAGE_DDLM_ARM64_PACKAGEFAMILYNAME_W: ::windows_core::PCWSTR =
    ::windows_core::w!("Microsoft.WinAppRuntime.DDLM.4000.1049.117.0-a6_8wekyb3d8bbwe");
pub const WINDOWSAPPSDK_RUNTIME_PACKAGE_DDLM_X64_PACKAGEFAMILYNAME: ::windows_core::PCSTR =
    ::windows_core::s!("Microsoft.WinAppRuntime.DDLM.4000.1049.117.0-x6_8wekyb3d8bbwe");
pub const WINDOWSAPPSDK_RUNTIME_PACKAGE_DDLM_X64_PACKAGEFAMILYNAME_W: ::windows_core::PCWSTR =
    ::windows_core::w!("Microsoft.WinAppRuntime.DDLM.4000.1049.117.0-x6_8wekyb3d8bbwe");
pub const WINDOWSAPPSDK_RUNTIME_PACKAGE_DDLM_X86_PACKAGEFAMILYNAME: ::windows_core::PCSTR =
    ::windows_core::s!("Microsoft.WinAppRuntime.DDLM.4000.1049.117.0-x8_8wekyb3d8bbwe");
pub const WINDOWSAPPSDK_RUNTIME_PACKAGE_DDLM_X86_PACKAGEFAMILYNAME_W: ::windows_core::PCWSTR =
    ::windows_core::w!("Microsoft.WinAppRuntime.DDLM.4000.1049.117.0-x8_8wekyb3d8bbwe");
pub const WINDOWSAPPSDK_RUNTIME_PACKAGE_FRAMEWORK_PACKAGEFAMILYNAME: ::windows_core::PCSTR =
    ::windows_core::s!("Microsoft.WindowsAppRuntime.1.4_8wekyb3d8bbwe");
pub const WINDOWSAPPSDK_RUNTIME_PACKAGE_FRAMEWORK_PACKAGEFAMILYNAME_W: ::windows_core::PCWSTR =
    ::windows_core::w!("Microsoft.WindowsAppRuntime.1.4_8wekyb3d8bbwe");
pub const WINDOWSAPPSDK_RUNTIME_PACKAGE_MAIN_PACKAGEFAMILYNAME: ::windows_core::PCSTR =
    ::windows_core::s!("MicrosoftCorporationII.WinAppRuntime.Main.1.4_8wekyb3d8bbwe");
pub const WINDOWSAPPSDK_RUNTIME_PACKAGE_MAIN_PACKAGEFAMILYNAME_W: ::windows_core::PCWSTR =
    ::windows_core::w!("MicrosoftCorporationII.WinAppRuntime.Main.1.4_8wekyb3d8bbwe");
pub const WINDOWSAPPSDK_RUNTIME_PACKAGE_SINGLETON_PACKAGEFAMILYNAME: ::windows_core::PCSTR =
    ::windows_core::s!("MicrosoftCorporationII.WinAppRuntime.Singleton_8wekyb3d8bbwe");
pub const WINDOWSAPPSDK_RUNTIME_PACKAGE_SINGLETON_PACKAGEFAMILYNAME_W: ::windows_core::PCWSTR =
    ::windows_core::w!("MicrosoftCorporationII.WinAppRuntime.Singleton_8wekyb3d8bbwe");
pub const WINDOWSAPPSDK_RUNTIME_VERSION_DOTQUADSTRING: ::windows_core::PCSTR =
    ::windows_core::s!("4000.1049.117.0");
pub const WINDOWSAPPSDK_RUNTIME_VERSION_DOTQUADSTRING_W: ::windows_core::PCWSTR =
    ::windows_core::w!("4000.1049.117.0");
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MddBootstrapInitializeOptions(pub i32);
impl ::core::marker::Copy for MddBootstrapInitializeOptions {}
impl ::core::clone::Clone for MddBootstrapInitializeOptions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MddBootstrapInitializeOptions {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for MddBootstrapInitializeOptions {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for MddBootstrapInitializeOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MddBootstrapInitializeOptions").field(&self.0).finish()
    }
}
impl MddBootstrapInitializeOptions {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for MddBootstrapInitializeOptions {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for MddBootstrapInitializeOptions {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for MddBootstrapInitializeOptions {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for MddBootstrapInitializeOptions {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for MddBootstrapInitializeOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
