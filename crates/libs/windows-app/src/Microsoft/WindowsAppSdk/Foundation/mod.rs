#[doc = "*Required features: `\"WindowsAppSdk_Foundation\"`*"]
#[inline]
pub unsafe fn MddBootstrapInitialize<'a, P0>(
    majorminorversion: u32,
    versiontag: P0,
    minversion: ::windows::Win32::Storage::Packaging::Appx::PACKAGE_VERSION,
) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[link(name = "microsoft.windowsappruntime.bootstrap")]
    extern "system" {
        fn MddBootstrapInitialize(
            majorminorversion: u32,
            versiontag: ::windows::core::PCWSTR,
            minversion: ::windows::Win32::Storage::Packaging::Appx::PACKAGE_VERSION,
        ) -> ::windows::core::HRESULT;
    }
    MddBootstrapInitialize(majorminorversion, versiontag.into(), ::core::mem::transmute(minversion))
        .ok()
}
#[doc = "*Required features: `\"WindowsAppSdk_Foundation\"`*"]
#[inline]
pub unsafe fn MddBootstrapInitialize2<'a, P0>(
    majorminorversion: u32,
    versiontag: P0,
    minversion: ::windows::Win32::Storage::Packaging::Appx::PACKAGE_VERSION,
    options: MddBootstrapInitializeOptions,
) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[link(name = "microsoft.windowsappruntime.bootstrap")]
    extern "system" {
        fn MddBootstrapInitialize2(
            majorminorversion: u32,
            versiontag: ::windows::core::PCWSTR,
            minversion: ::windows::Win32::Storage::Packaging::Appx::PACKAGE_VERSION,
            options: MddBootstrapInitializeOptions,
        ) -> ::windows::core::HRESULT;
    }
    MddBootstrapInitialize2(
        majorminorversion,
        versiontag.into(),
        ::core::mem::transmute(minversion),
        options,
    )
    .ok()
}
#[doc = "*Required features: `\"WindowsAppSdk_Foundation\"`*"]
#[inline]
pub unsafe fn MddBootstrapShutdown() {
    #[link(name = "microsoft.windowsappruntime.bootstrap")]
    extern "system" {
        fn MddBootstrapShutdown();
    }
    MddBootstrapShutdown()
}
#[doc = "*Required features: `\"WindowsAppSdk_Foundation\"`*"]
pub const WASR_IDENTITY_PUBLISHER: &str =
    "CN=Microsoft Corporation, O=Microsoft Corporation, L=Redmond, S=Washington, C=US";
#[doc = "*Required features: `\"WindowsAppSdk_Foundation\"`*"]
pub const WASR_IDENTITY_PUBLISHERID: &str = "8wekyb3d8bbwe";
#[doc = "*Required features: `\"WindowsAppSdk_Foundation\"`*"]
pub const WASR_IDENTITY_PUBLISHERID_W: &str = "8wekyb3d8bbwe";
#[doc = "*Required features: `\"WindowsAppSdk_Foundation\"`*"]
pub const WASR_IDENTITY_PUBLISHER_W: &str =
    "CN=Microsoft Corporation, O=Microsoft Corporation, L=Redmond, S=Washington, C=US";
#[doc = "*Required features: `\"WindowsAppSdk_Foundation\"`*"]
pub const WASR_PKG_DDLM_ARM64_FAMILY: &str =
    "Microsoft.WinAppRuntime.DDLM.2000.609.1413.0-a6-p1_8wekyb3d8bbwe";
#[doc = "*Required features: `\"WindowsAppSdk_Foundation\"`*"]
pub const WASR_PKG_DDLM_ARM64_FAMILY_W: &str =
    "Microsoft.WinAppRuntime.DDLM.2000.609.1413.0-a6-p1_8wekyb3d8bbwe";
#[doc = "*Required features: `\"WindowsAppSdk_Foundation\"`*"]
pub const WASR_PKG_DDLM_X64_FAMILY: &str =
    "Microsoft.WinAppRuntime.DDLM.2000.609.1413.0-x6-p1_8wekyb3d8bbwe";
#[doc = "*Required features: `\"WindowsAppSdk_Foundation\"`*"]
pub const WASR_PKG_DDLM_X64_FAMILY_W: &str =
    "Microsoft.WinAppRuntime.DDLM.2000.609.1413.0-x6-p1_8wekyb3d8bbwe";
#[doc = "*Required features: `\"WindowsAppSdk_Foundation\"`*"]
pub const WASR_PKG_DDLM_X86_FAMILY: &str =
    "Microsoft.WinAppRuntime.DDLM.2000.609.1413.0-x8-p1_8wekyb3d8bbwe";
#[doc = "*Required features: `\"WindowsAppSdk_Foundation\"`*"]
pub const WASR_PKG_DDLM_X86_FAMILY_W: &str =
    "Microsoft.WinAppRuntime.DDLM.2000.609.1413.0-x8-p1_8wekyb3d8bbwe";
#[doc = "*Required features: `\"WindowsAppSdk_Foundation\"`*"]
pub const WASR_PKG_FRAMEWORK_FAMILY: &str =
    "Microsoft.WindowsAppRuntime.1.2-preview1_8wekyb3d8bbwe";
#[doc = "*Required features: `\"WindowsAppSdk_Foundation\"`*"]
pub const WASR_PKG_FRAMEWORK_FAMILY_W: &str =
    "Microsoft.WindowsAppRuntime.1.2-preview1_8wekyb3d8bbwe";
#[doc = "*Required features: `\"WindowsAppSdk_Foundation\"`*"]
pub const WASR_PKG_MAIN_FAMILY: &str =
    "MicrosoftCorporationII.WinAppRuntime.Main.1.2-p1_8wekyb3d8bbwe";
#[doc = "*Required features: `\"WindowsAppSdk_Foundation\"`*"]
pub const WASR_PKG_MAIN_FAMILY_W: &str =
    "MicrosoftCorporationII.WinAppRuntime.Main.1.2-p1_8wekyb3d8bbwe";
#[doc = "*Required features: `\"WindowsAppSdk_Foundation\"`*"]
pub const WASR_PKG_SINGLETON_FAMILY: &str =
    "MicrosoftCorporationII.WinAppRuntime.Singleton-p1_8wekyb3d8bbwe";
#[doc = "*Required features: `\"WindowsAppSdk_Foundation\"`*"]
pub const WASR_PKG_SINGLETON_FAMILY_W: &str =
    "MicrosoftCorporationII.WinAppRuntime.Singleton-p1_8wekyb3d8bbwe";
#[doc = "*Required features: `\"WindowsAppSdk_Foundation\"`*"]
pub const WASR_VERSION_BUILD: u16 = 1413u16;
#[doc = "*Required features: `\"WindowsAppSdk_Foundation\"`*"]
pub const WASR_VERSION_DOTQUADSTRING: &str = "2000.609.1413.0";
#[doc = "*Required features: `\"WindowsAppSdk_Foundation\"`*"]
pub const WASR_VERSION_DOTQUADSTRING_W: &str = "2000.609.1413.0";
#[doc = "*Required features: `\"WindowsAppSdk_Foundation\"`*"]
pub const WASR_VERSION_MAJOR: u16 = 2000u16;
#[doc = "*Required features: `\"WindowsAppSdk_Foundation\"`*"]
pub const WASR_VERSION_MINOR: u16 = 609u16;
#[doc = "*Required features: `\"WindowsAppSdk_Foundation\"`*"]
pub const WASR_VERSION_REVISION: u16 = 0u16;
#[doc = "*Required features: `\"WindowsAppSdk_Foundation\"`*"]
pub const WASR_VERSION_UINT64: u64 = 562952569148997632u64;
#[doc = "*Required features: `\"WindowsAppSdk_Foundation\"`*"]
pub const WAS_RELEASE_CHANNEL: &str = "preview";
#[doc = "*Required features: `\"WindowsAppSdk_Foundation\"`*"]
pub const WAS_RELEASE_CHANNEL_W: &str = "preview";
#[doc = "*Required features: `\"WindowsAppSdk_Foundation\"`*"]
pub const WAS_RELEASE_FMT_VERSION_STAG: &str = "-p1";
#[doc = "*Required features: `\"WindowsAppSdk_Foundation\"`*"]
pub const WAS_RELEASE_FMT_VERSION_STAG_W: &str = "-p1";
#[doc = "*Required features: `\"WindowsAppSdk_Foundation\"`*"]
pub const WAS_RELEASE_FMT_VERSION_TAG: &str = "-preview1";
#[doc = "*Required features: `\"WindowsAppSdk_Foundation\"`*"]
pub const WAS_RELEASE_FMT_VERSION_TAG_W: &str = "-preview1";
#[doc = "*Required features: `\"WindowsAppSdk_Foundation\"`*"]
pub const WAS_RELEASE_MAJOR: u32 = 1u32;
#[doc = "*Required features: `\"WindowsAppSdk_Foundation\"`*"]
pub const WAS_RELEASE_MAJORMINOR: u32 = 65538u32;
#[doc = "*Required features: `\"WindowsAppSdk_Foundation\"`*"]
pub const WAS_RELEASE_MINOR: u32 = 2u32;
#[doc = "*Required features: `\"WindowsAppSdk_Foundation\"`*"]
pub const WAS_RELEASE_PATCH: u32 = 0u32;
#[doc = "*Required features: `\"WindowsAppSdk_Foundation\"`*"]
pub const WAS_RELEASE_VERSION_STAG: &str = "p1";
#[doc = "*Required features: `\"WindowsAppSdk_Foundation\"`*"]
pub const WAS_RELEASE_VERSION_STAG_W: &str = "p1";
#[doc = "*Required features: `\"WindowsAppSdk_Foundation\"`*"]
pub const WAS_RELEASE_VERSION_TAG: &str = "preview1";
#[doc = "*Required features: `\"WindowsAppSdk_Foundation\"`*"]
pub const WAS_RELEASE_VERSION_TAG_W: &str = "preview1";
#[doc = "*Required features: `\"WindowsAppSdk_Foundation\"`*"]
pub const WINDOWSAPPSDK_RELEASE_CHANNEL: &str = "preview";
#[doc = "*Required features: `\"WindowsAppSdk_Foundation\"`*"]
pub const WINDOWSAPPSDK_RELEASE_CHANNEL_W: &str = "preview";
#[doc = "*Required features: `\"WindowsAppSdk_Foundation\"`*"]
pub const WINDOWSAPPSDK_RELEASE_FORMATTED_VERSION_SHORTTAG: &str = "-p1";
#[doc = "*Required features: `\"WindowsAppSdk_Foundation\"`*"]
pub const WINDOWSAPPSDK_RELEASE_FORMATTED_VERSION_SHORTTAG_W: &str = "-p1";
#[doc = "*Required features: `\"WindowsAppSdk_Foundation\"`*"]
pub const WINDOWSAPPSDK_RELEASE_FORMATTED_VERSION_TAG: &str = "-preview1";
#[doc = "*Required features: `\"WindowsAppSdk_Foundation\"`*"]
pub const WINDOWSAPPSDK_RELEASE_FORMATTED_VERSION_TAG_W: &str = "-preview1";
#[doc = "*Required features: `\"WindowsAppSdk_Foundation\"`*"]
pub const WINDOWSAPPSDK_RELEASE_MAJOR: u32 = 1u32;
#[doc = "*Required features: `\"WindowsAppSdk_Foundation\"`*"]
pub const WINDOWSAPPSDK_RELEASE_MAJORMINOR: u32 = 65538u32;
#[doc = "*Required features: `\"WindowsAppSdk_Foundation\"`*"]
pub const WINDOWSAPPSDK_RELEASE_MINOR: u32 = 2u32;
#[doc = "*Required features: `\"WindowsAppSdk_Foundation\"`*"]
pub const WINDOWSAPPSDK_RELEASE_PATCH: u32 = 0u32;
#[doc = "*Required features: `\"WindowsAppSdk_Foundation\"`*"]
pub const WINDOWSAPPSDK_RELEASE_VERSION_SHORTTAG: &str = "p1";
#[doc = "*Required features: `\"WindowsAppSdk_Foundation\"`*"]
pub const WINDOWSAPPSDK_RELEASE_VERSION_SHORTTAG_W: &str = "p1";
#[doc = "*Required features: `\"WindowsAppSdk_Foundation\"`*"]
pub const WINDOWSAPPSDK_RELEASE_VERSION_TAG: &str = "preview1";
#[doc = "*Required features: `\"WindowsAppSdk_Foundation\"`*"]
pub const WINDOWSAPPSDK_RELEASE_VERSION_TAG_W: &str = "preview1";
#[doc = "*Required features: `\"WindowsAppSdk_Foundation\"`*"]
pub const WINDOWSAPPSDK_RUNTIME_IDENTITY_PUBLISHER: &str =
    "CN=Microsoft Corporation, O=Microsoft Corporation, L=Redmond, S=Washington, C=US";
#[doc = "*Required features: `\"WindowsAppSdk_Foundation\"`*"]
pub const WINDOWSAPPSDK_RUNTIME_IDENTITY_PUBLISHERID: &str = "8wekyb3d8bbwe";
#[doc = "*Required features: `\"WindowsAppSdk_Foundation\"`*"]
pub const WINDOWSAPPSDK_RUNTIME_IDENTITY_PUBLISHERID_W: &str = "8wekyb3d8bbwe";
#[doc = "*Required features: `\"WindowsAppSdk_Foundation\"`*"]
pub const WINDOWSAPPSDK_RUNTIME_IDENTITY_PUBLISHER_W: &str =
    "CN=Microsoft Corporation, O=Microsoft Corporation, L=Redmond, S=Washington, C=US";
#[doc = "*Required features: `\"WindowsAppSdk_Foundation\"`*"]
pub const WINDOWSAPPSDK_RUNTIME_PACKAGE_DDLM_ARM64_PACKAGEFAMILYNAME: &str =
    "Microsoft.WinAppRuntime.DDLM.2000.609.1413.0-a6-p1_8wekyb3d8bbwe";
#[doc = "*Required features: `\"WindowsAppSdk_Foundation\"`*"]
pub const WINDOWSAPPSDK_RUNTIME_PACKAGE_DDLM_ARM64_PACKAGEFAMILYNAME_W: &str =
    "Microsoft.WinAppRuntime.DDLM.2000.609.1413.0-a6-p1_8wekyb3d8bbwe";
#[doc = "*Required features: `\"WindowsAppSdk_Foundation\"`*"]
pub const WINDOWSAPPSDK_RUNTIME_PACKAGE_DDLM_X64_PACKAGEFAMILYNAME: &str =
    "Microsoft.WinAppRuntime.DDLM.2000.609.1413.0-x6-p1_8wekyb3d8bbwe";
#[doc = "*Required features: `\"WindowsAppSdk_Foundation\"`*"]
pub const WINDOWSAPPSDK_RUNTIME_PACKAGE_DDLM_X64_PACKAGEFAMILYNAME_W: &str =
    "Microsoft.WinAppRuntime.DDLM.2000.609.1413.0-x6-p1_8wekyb3d8bbwe";
#[doc = "*Required features: `\"WindowsAppSdk_Foundation\"`*"]
pub const WINDOWSAPPSDK_RUNTIME_PACKAGE_DDLM_X86_PACKAGEFAMILYNAME: &str =
    "Microsoft.WinAppRuntime.DDLM.2000.609.1413.0-x8-p1_8wekyb3d8bbwe";
#[doc = "*Required features: `\"WindowsAppSdk_Foundation\"`*"]
pub const WINDOWSAPPSDK_RUNTIME_PACKAGE_DDLM_X86_PACKAGEFAMILYNAME_W: &str =
    "Microsoft.WinAppRuntime.DDLM.2000.609.1413.0-x8-p1_8wekyb3d8bbwe";
#[doc = "*Required features: `\"WindowsAppSdk_Foundation\"`*"]
pub const WINDOWSAPPSDK_RUNTIME_PACKAGE_FRAMEWORK_PACKAGEFAMILYNAME: &str =
    "Microsoft.WindowsAppRuntime.1.2-preview1_8wekyb3d8bbwe";
#[doc = "*Required features: `\"WindowsAppSdk_Foundation\"`*"]
pub const WINDOWSAPPSDK_RUNTIME_PACKAGE_FRAMEWORK_PACKAGEFAMILYNAME_W: &str =
    "Microsoft.WindowsAppRuntime.1.2-preview1_8wekyb3d8bbwe";
#[doc = "*Required features: `\"WindowsAppSdk_Foundation\"`*"]
pub const WINDOWSAPPSDK_RUNTIME_PACKAGE_MAIN_PACKAGEFAMILYNAME: &str =
    "MicrosoftCorporationII.WinAppRuntime.Main.1.2-p1_8wekyb3d8bbwe";
#[doc = "*Required features: `\"WindowsAppSdk_Foundation\"`*"]
pub const WINDOWSAPPSDK_RUNTIME_PACKAGE_MAIN_PACKAGEFAMILYNAME_W: &str =
    "MicrosoftCorporationII.WinAppRuntime.Main.1.2-p1_8wekyb3d8bbwe";
#[doc = "*Required features: `\"WindowsAppSdk_Foundation\"`*"]
pub const WINDOWSAPPSDK_RUNTIME_PACKAGE_SINGLETON_PACKAGEFAMILYNAME: &str =
    "MicrosoftCorporationII.WinAppRuntime.Singleton-p1_8wekyb3d8bbwe";
#[doc = "*Required features: `\"WindowsAppSdk_Foundation\"`*"]
pub const WINDOWSAPPSDK_RUNTIME_PACKAGE_SINGLETON_PACKAGEFAMILYNAME_W: &str =
    "MicrosoftCorporationII.WinAppRuntime.Singleton-p1_8wekyb3d8bbwe";
#[doc = "*Required features: `\"WindowsAppSdk_Foundation\"`*"]
pub const WINDOWSAPPSDK_RUNTIME_VERSION_DOTQUADSTRING: &str = "2000.609.1413.0";
#[doc = "*Required features: `\"WindowsAppSdk_Foundation\"`*"]
pub const WINDOWSAPPSDK_RUNTIME_VERSION_DOTQUADSTRING_W: &str = "2000.609.1413.0";
#[doc = "*Required features: `\"WindowsAppSdk_Foundation\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MddBootstrapInitializeOptions(pub i32);
#[doc = "*Required features: `\"WindowsAppSdk_Foundation\"`*"]
pub const MddBootstrapInitializeOptions_None: MddBootstrapInitializeOptions =
    MddBootstrapInitializeOptions(0i32);
#[doc = "*Required features: `\"WindowsAppSdk_Foundation\"`*"]
pub const MddBootstrapInitializeOptions_OnError_DebugBreak: MddBootstrapInitializeOptions =
    MddBootstrapInitializeOptions(1i32);
#[doc = "*Required features: `\"WindowsAppSdk_Foundation\"`*"]
pub const MddBootstrapInitializeOptions_OnError_DebugBreak_IfDebuggerAttached:
    MddBootstrapInitializeOptions = MddBootstrapInitializeOptions(2i32);
#[doc = "*Required features: `\"WindowsAppSdk_Foundation\"`*"]
pub const MddBootstrapInitializeOptions_OnError_FailFast: MddBootstrapInitializeOptions =
    MddBootstrapInitializeOptions(4i32);
#[doc = "*Required features: `\"WindowsAppSdk_Foundation\"`*"]
pub const MddBootstrapInitializeOptions_OnNoMatch_ShowUI: MddBootstrapInitializeOptions =
    MddBootstrapInitializeOptions(8i32);
#[doc = "*Required features: `\"WindowsAppSdk_Foundation\"`*"]
pub const MddBootstrapInitializeOptions_OnPackageIdentity_NOOP: MddBootstrapInitializeOptions =
    MddBootstrapInitializeOptions(16i32);
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
unsafe impl ::windows::core::Abi for MddBootstrapInitializeOptions {
    type Abi = Self;
}
impl ::core::fmt::Debug for MddBootstrapInitializeOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MddBootstrapInitializeOptions").field(&self.0).finish()
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
