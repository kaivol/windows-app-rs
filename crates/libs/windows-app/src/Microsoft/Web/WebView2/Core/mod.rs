#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct CoreWebView2Certificate_Manual(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for CoreWebView2Certificate_Manual {
    type Vtable = CoreWebView2Certificate_Manual_Vtbl;
}
unsafe impl ::windows_core::ComInterface for CoreWebView2Certificate_Manual {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x4b9b0fe5_0ad9_5594_81e7_b18ecc0636de);
}
#[repr(C)]
#[doc(hidden)]
pub struct CoreWebView2Certificate_Manual_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Windows_Security_Cryptography_Certificates")]
    pub ToCertificate: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Security_Cryptography_Certificates"))]
    ToCertificate: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct CoreWebView2ClientCertificate_Manual(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for CoreWebView2ClientCertificate_Manual {
    type Vtable = CoreWebView2ClientCertificate_Manual_Vtbl;
}
unsafe impl ::windows_core::ComInterface for CoreWebView2ClientCertificate_Manual {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xfaefefc2_20c3_5d86_8a74_f6d87d6ff8fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct CoreWebView2ClientCertificate_Manual_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Windows_Security_Cryptography_Certificates")]
    pub ToCertificate: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Security_Cryptography_Certificates"))]
    ToCertificate: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct CoreWebView2Profile_Manual(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for CoreWebView2Profile_Manual {
    type Vtable = CoreWebView2Profile_Manual_Vtbl;
}
unsafe impl ::windows_core::ComInterface for CoreWebView2Profile_Manual {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xb42bfab4_c4bf_5469_89ac_cadc69e3b0f5);
}
#[repr(C)]
#[doc(hidden)]
pub struct CoreWebView2Profile_Manual_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Windows_Foundation")]
    pub ClearBrowsingDataAsync: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        datakinds: CoreWebView2BrowsingDataKinds,
        starttime: ::windows::Foundation::DateTime,
        endtime: ::windows::Foundation::DateTime,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    ClearBrowsingDataAsync: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub ClearBrowsingDataAsync2: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    ClearBrowsingDataAsync2: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct CoreWebView2Profile_Manual2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for CoreWebView2Profile_Manual2 {
    type Vtable = CoreWebView2Profile_Manual2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for CoreWebView2Profile_Manual2 {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x6e62815a_6269_5756_92c3_f08afe17649c);
}
#[repr(C)]
#[doc(hidden)]
pub struct CoreWebView2Profile_Manual2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub GetNonDefaultPermissionSettingsAsync: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    )
        -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation_Collections"))]
    GetNonDefaultPermissionSettingsAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICoreWebView2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreWebView2 {
    type Vtable = ICoreWebView2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICoreWebView2 {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x3a3f559a_e5e9_5338_bb67_4eb0504a4f14);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Settings: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub Source: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub BrowserProcessId: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut u32,
    ) -> ::windows_core::HRESULT,
    pub CanGoBack: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub CanGoForward: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub DocumentTitle: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub ContainsFullScreenElement: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_Foundation")]
    pub NavigationStarting: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    NavigationStarting: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub RemoveNavigationStarting: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RemoveNavigationStarting: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub ContentLoading: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    ContentLoading: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub RemoveContentLoading: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RemoveContentLoading: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub SourceChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    SourceChanged: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub RemoveSourceChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RemoveSourceChanged: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub HistoryChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    HistoryChanged: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub RemoveHistoryChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RemoveHistoryChanged: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub NavigationCompleted: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    NavigationCompleted: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub RemoveNavigationCompleted: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RemoveNavigationCompleted: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub FrameNavigationStarting: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    FrameNavigationStarting: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub RemoveFrameNavigationStarting: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RemoveFrameNavigationStarting: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub FrameNavigationCompleted: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    FrameNavigationCompleted: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub RemoveFrameNavigationCompleted: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RemoveFrameNavigationCompleted: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub ScriptDialogOpening: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    ScriptDialogOpening: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub RemoveScriptDialogOpening: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RemoveScriptDialogOpening: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub PermissionRequested: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    PermissionRequested: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub RemovePermissionRequested: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RemovePermissionRequested: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub ProcessFailed: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    ProcessFailed: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub RemoveProcessFailed: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RemoveProcessFailed: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub WebMessageReceived: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    WebMessageReceived: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub RemoveWebMessageReceived: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RemoveWebMessageReceived: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub NewWindowRequested: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    NewWindowRequested: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub RemoveNewWindowRequested: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RemoveNewWindowRequested: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub DocumentTitleChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    DocumentTitleChanged: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub RemoveDocumentTitleChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RemoveDocumentTitleChanged: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub ContainsFullScreenElementChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    ContainsFullScreenElementChanged: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub RemoveContainsFullScreenElementChanged:
        unsafe extern "system" fn(
            this: *mut ::core::ffi::c_void,
            token: ::windows::Foundation::EventRegistrationToken,
        ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RemoveContainsFullScreenElementChanged: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub WebResourceRequested: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    WebResourceRequested: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub RemoveWebResourceRequested: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RemoveWebResourceRequested: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub WindowCloseRequested: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    WindowCloseRequested: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub RemoveWindowCloseRequested: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RemoveWindowCloseRequested: usize,
    pub Navigate: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        uri: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub NavigateToString: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        htmlcontent: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_Foundation")]
    pub AddScriptToExecuteOnDocumentCreatedAsync:
        unsafe extern "system" fn(
            this: *mut ::core::ffi::c_void,
            javascript: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    AddScriptToExecuteOnDocumentCreatedAsync: usize,
    pub RemoveScriptToExecuteOnDocumentCreated:
        unsafe extern "system" fn(
            this: *mut ::core::ffi::c_void,
            id: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_Foundation")]
    pub ExecuteScriptAsync: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        javascript: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    ExecuteScriptAsync: usize,
    #[cfg(all(feature = "Windows_Foundation", feature = "Windows_Storage_Streams"))]
    pub CapturePreviewAsync: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        imageformat: CoreWebView2CapturePreviewImageFormat,
        imagestream: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Windows_Foundation", feature = "Windows_Storage_Streams")))]
    CapturePreviewAsync: usize,
    pub Reload:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub PostWebMessageAsJson: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        webmessageasjson: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub PostWebMessageAsString: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        webmessageasstring: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_Foundation")]
    pub CallDevToolsProtocolMethodAsync: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        methodname: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        parametersasjson: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    CallDevToolsProtocolMethodAsync: usize,
    pub GoBack:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GoForward:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetDevToolsProtocolEventReceiver: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        eventname: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub AddHostObjectToScript: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        name: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        rawobject: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub RemoveHostObjectFromScript: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        name: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub OpenDevToolsWindow:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub AddWebResourceRequestedFilter: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        uri: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        resourcecontext: CoreWebView2WebResourceContext,
    ) -> ::windows_core::HRESULT,
    pub RemoveWebResourceRequestedFilter: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        uri: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        resourcecontext: CoreWebView2WebResourceContext,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICoreWebView2AcceleratorKeyPressedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreWebView2AcceleratorKeyPressedEventArgs {
    type Vtable = ICoreWebView2AcceleratorKeyPressedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICoreWebView2AcceleratorKeyPressedEventArgs {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x41a56100_92a5_59d1_9e71_9222e33ae38b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2AcceleratorKeyPressedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub KeyEventKind: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut CoreWebView2KeyEventKind,
    ) -> ::windows_core::HRESULT,
    pub VirtualKey: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut u32,
    ) -> ::windows_core::HRESULT,
    pub KeyEventLParam: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut i32,
    ) -> ::windows_core::HRESULT,
    pub PhysicalKeyStatus: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut CoreWebView2PhysicalKeyStatus,
    ) -> ::windows_core::HRESULT,
    pub Handled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICoreWebView2BasicAuthenticationRequestedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreWebView2BasicAuthenticationRequestedEventArgs {
    type Vtable = ICoreWebView2BasicAuthenticationRequestedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICoreWebView2BasicAuthenticationRequestedEventArgs {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x4b16330c_4ca5_555e_af21_164334405f63);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2BasicAuthenticationRequestedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Uri: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub Challenge: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub Response: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub Cancel: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetCancel: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_Foundation")]
    pub GetDeferral: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    GetDeferral: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICoreWebView2BasicAuthenticationResponse(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreWebView2BasicAuthenticationResponse {
    type Vtable = ICoreWebView2BasicAuthenticationResponse_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICoreWebView2BasicAuthenticationResponse {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x08df33b9_6e38_5962_9ffd_caab3c30fbc1);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2BasicAuthenticationResponse_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub UserName: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub SetUserName: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub Password: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub SetPassword: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICoreWebView2BrowserProcessExitedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreWebView2BrowserProcessExitedEventArgs {
    type Vtable = ICoreWebView2BrowserProcessExitedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICoreWebView2BrowserProcessExitedEventArgs {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x79963f77_1484_5a46_b91f_dfc5c1a0ce14);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2BrowserProcessExitedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub BrowserProcessExitKind: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut CoreWebView2BrowserProcessExitKind,
    ) -> ::windows_core::HRESULT,
    pub BrowserProcessId: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut u32,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICoreWebView2Certificate(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreWebView2Certificate {
    type Vtable = ICoreWebView2Certificate_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICoreWebView2Certificate {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x414a3b75_1bc1_55e1_9926_268c0a3462c7);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2Certificate_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Subject: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub Issuer: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub ValidFrom: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub ValidTo: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub DerEncodedSerialNumber: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub PemEncodedIssuerCertificateChain: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation_Collections"))]
    PemEncodedIssuerCertificateChain: usize,
    pub ToPemEncoding: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICoreWebView2ClientCertificate(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreWebView2ClientCertificate {
    type Vtable = ICoreWebView2ClientCertificate_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICoreWebView2ClientCertificate {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x091b39f2_68df_52b4_8fb0_fd3561af41f2);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2ClientCertificate_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Subject: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub Issuer: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub ValidFrom: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub ValidTo: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub DerEncodedSerialNumber: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub PemEncodedIssuerCertificateChain: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation_Collections"))]
    PemEncodedIssuerCertificateChain: usize,
    pub Kind: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut CoreWebView2ClientCertificateKind,
    ) -> ::windows_core::HRESULT,
    pub ToPemEncoding: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICoreWebView2ClientCertificateRequestedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreWebView2ClientCertificateRequestedEventArgs {
    type Vtable = ICoreWebView2ClientCertificateRequestedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICoreWebView2ClientCertificateRequestedEventArgs {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x93287b55_31f9_55a0_b68b_d9841d7e1bf4);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2ClientCertificateRequestedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Host: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub Port: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut i32,
    ) -> ::windows_core::HRESULT,
    pub IsProxy: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub AllowedCertificateAuthorities: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation_Collections"))]
    AllowedCertificateAuthorities: usize,
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub MutuallyTrustedCertificates: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation_Collections"))]
    MutuallyTrustedCertificates: usize,
    pub SelectedCertificate: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetSelectedCertificate: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub Cancel: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetCancel: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
    pub Handled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_Foundation")]
    pub GetDeferral: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    GetDeferral: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICoreWebView2CompositionController(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreWebView2CompositionController {
    type Vtable = ICoreWebView2CompositionController_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICoreWebView2CompositionController {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x31bbb153_11b2_58e8_9beb_69f5c8e14420);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2CompositionController_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub RootVisualTarget: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetRootVisualTarget: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_Foundation")]
    pub CursorChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    CursorChanged: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub RemoveCursorChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RemoveCursorChanged: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub SendMouseInput: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        eventkind: CoreWebView2MouseEventKind,
        virtualkeys: CoreWebView2MouseEventVirtualKeys,
        mousedata: u32,
        point: ::windows::Foundation::Point,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    SendMouseInput: usize,
    pub SendPointerInput: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        eventkind: CoreWebView2PointerEventKind,
        pointerinfo: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_UI_Core")]
    pub Cursor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI_Core"))]
    Cursor: usize,
    #[cfg(feature = "Windows_ApplicationModel_DataTransfer_DragDrop_Core")]
    pub DragEnter: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        draginfo: *mut ::core::ffi::c_void,
        draguioverride: *mut ::core::ffi::c_void,
        result__: *mut ::windows::ApplicationModel::DataTransfer::DataPackageOperation,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_ApplicationModel_DataTransfer_DragDrop_Core"))]
    DragEnter: usize,
    #[cfg(feature = "Windows_ApplicationModel_DataTransfer_DragDrop_Core")]
    pub DragOver: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        draginfo: *mut ::core::ffi::c_void,
        draguioverride: *mut ::core::ffi::c_void,
        result__: *mut ::windows::ApplicationModel::DataTransfer::DataPackageOperation,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_ApplicationModel_DataTransfer_DragDrop_Core"))]
    DragOver: usize,
    #[cfg(feature = "Windows_ApplicationModel_DataTransfer_DragDrop_Core")]
    pub Drop: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        draginfo: *mut ::core::ffi::c_void,
        result__: *mut ::windows::ApplicationModel::DataTransfer::DataPackageOperation,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_ApplicationModel_DataTransfer_DragDrop_Core"))]
    Drop: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICoreWebView2CompositionController2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreWebView2CompositionController2 {
    type Vtable = ICoreWebView2CompositionController2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICoreWebView2CompositionController2 {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x8cef61b9_fa55_547d_aae6_7bcaed4249a2);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2CompositionController2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICoreWebView2CompositionController3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreWebView2CompositionController3 {
    type Vtable = ICoreWebView2CompositionController3_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICoreWebView2CompositionController3 {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xbbbecdcf_0f03_50f0_8f85_9cbf6c9bbe10);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2CompositionController3_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub DragLeave:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICoreWebView2CompositionControllerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreWebView2CompositionControllerStatics {
    type Vtable = ICoreWebView2CompositionControllerStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICoreWebView2CompositionControllerStatics {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x4df0ab1f_7f2a_573b_b81a_b9b531224736);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2CompositionControllerStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICoreWebView2CompositionControllerStatics2_Manual(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreWebView2CompositionControllerStatics2_Manual {
    type Vtable = ICoreWebView2CompositionControllerStatics2_Manual_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICoreWebView2CompositionControllerStatics2_Manual {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x48a321e7_4f40_526e_837e_1eb0c477b69d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2CompositionControllerStatics2_Manual_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICoreWebView2ContentLoadingEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreWebView2ContentLoadingEventArgs {
    type Vtable = ICoreWebView2ContentLoadingEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICoreWebView2ContentLoadingEventArgs {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x6cf95373_946c_5dae_9b3e_0fe23d5aa29f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2ContentLoadingEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub IsErrorPage: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub NavigationId: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut u64,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICoreWebView2ContextMenuItem(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreWebView2ContextMenuItem {
    type Vtable = ICoreWebView2ContextMenuItem_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICoreWebView2ContextMenuItem {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x2a65706f_941a_52cd_8651_a165586b0abf);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2ContextMenuItem_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Name: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub Label: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub CommandId: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut i32,
    ) -> ::windows_core::HRESULT,
    pub ShortcutKeyDescription: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_Storage_Streams")]
    pub Icon: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Storage_Streams"))]
    Icon: usize,
    pub Kind: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut CoreWebView2ContextMenuItemKind,
    ) -> ::windows_core::HRESULT,
    pub IsEnabled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetIsEnabled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
    pub IsChecked: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetIsChecked: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub Children: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation_Collections"))]
    Children: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub CustomItemSelected: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    CustomItemSelected: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub RemoveCustomItemSelected: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RemoveCustomItemSelected: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICoreWebView2ContextMenuRequestedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreWebView2ContextMenuRequestedEventArgs {
    type Vtable = ICoreWebView2ContextMenuRequestedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICoreWebView2ContextMenuRequestedEventArgs {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xd77bdd8c_9b3e_596e_ae80_320c0df4ecbc);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2ContextMenuRequestedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub MenuItems: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation_Collections"))]
    MenuItems: usize,
    pub ContextMenuTarget: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_Foundation")]
    pub Location: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::Point,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    Location: usize,
    pub SelectedCommandId: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut i32,
    ) -> ::windows_core::HRESULT,
    pub SetSelectedCommandId: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: i32,
    ) -> ::windows_core::HRESULT,
    pub Handled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_Foundation")]
    pub GetDeferral: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    GetDeferral: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICoreWebView2ContextMenuTarget(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreWebView2ContextMenuTarget {
    type Vtable = ICoreWebView2ContextMenuTarget_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICoreWebView2ContextMenuTarget {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x41e24e6a_4612_5bd9_8e61_e9280615205e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2ContextMenuTarget_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Kind: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut CoreWebView2ContextMenuTargetKind,
    ) -> ::windows_core::HRESULT,
    pub IsEditable: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub IsRequestedForMainFrame: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub PageUri: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub FrameUri: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub HasLinkUri: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub LinkUri: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub HasLinkText: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub LinkText: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub HasSourceUri: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SourceUri: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub HasSelection: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SelectionText: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICoreWebView2Controller(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreWebView2Controller {
    type Vtable = ICoreWebView2Controller_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICoreWebView2Controller {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xa588121c_53bf_590e_80e5_29d729cbd743);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2Controller_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub IsVisible: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetIsVisible: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_Foundation")]
    pub Bounds: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::Rect,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    Bounds: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub SetBounds: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::windows::Foundation::Rect,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    SetBounds: usize,
    pub ZoomFactor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub SetZoomFactor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
    ) -> ::windows_core::HRESULT,
    pub ParentWindow: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetParentWindow: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub CoreWebView2: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_Foundation")]
    pub ZoomFactorChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    ZoomFactorChanged: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub RemoveZoomFactorChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RemoveZoomFactorChanged: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub MoveFocusRequested: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    MoveFocusRequested: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub RemoveMoveFocusRequested: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RemoveMoveFocusRequested: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub GotFocus: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    GotFocus: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub RemoveGotFocus: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RemoveGotFocus: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub LostFocus: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    LostFocus: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub RemoveLostFocus: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RemoveLostFocus: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub AcceleratorKeyPressed: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    AcceleratorKeyPressed: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub RemoveAcceleratorKeyPressed: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RemoveAcceleratorKeyPressed: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub SetBoundsAndZoomFactor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        bounds: ::windows::Foundation::Rect,
        zoomfactor: f64,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    SetBoundsAndZoomFactor: usize,
    pub MoveFocus: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        reason: CoreWebView2MoveFocusReason,
    ) -> ::windows_core::HRESULT,
    pub NotifyParentWindowPositionChanged:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Close: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICoreWebView2Controller2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreWebView2Controller2 {
    type Vtable = ICoreWebView2Controller2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICoreWebView2Controller2 {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x0069c40b_2e8a_513f_9d9d_e0c2b64f7200);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2Controller2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Windows_UI")]
    pub DefaultBackgroundColor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI"))]
    DefaultBackgroundColor: usize,
    #[cfg(feature = "Windows_UI")]
    pub SetDefaultBackgroundColor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::windows::UI::Color,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI"))]
    SetDefaultBackgroundColor: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICoreWebView2Controller3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreWebView2Controller3 {
    type Vtable = ICoreWebView2Controller3_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICoreWebView2Controller3 {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xe5bae214_791a_5d13_9b76_a257d9fda2ac);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2Controller3_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub RasterizationScale: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub SetRasterizationScale: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
    ) -> ::windows_core::HRESULT,
    pub ShouldDetectMonitorScaleChanges: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetShouldDetectMonitorScaleChanges: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    )
        -> ::windows_core::HRESULT,
    pub BoundsMode: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut CoreWebView2BoundsMode,
    ) -> ::windows_core::HRESULT,
    pub SetBoundsMode: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: CoreWebView2BoundsMode,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_Foundation")]
    pub RasterizationScaleChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RasterizationScaleChanged: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub RemoveRasterizationScaleChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RemoveRasterizationScaleChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICoreWebView2Controller4(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreWebView2Controller4 {
    type Vtable = ICoreWebView2Controller4_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICoreWebView2Controller4 {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x94e2862d_4638_54ba_92cf_e31a31499b78);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2Controller4_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub AllowExternalDrop: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetAllowExternalDrop: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICoreWebView2ControllerFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreWebView2ControllerFactory {
    type Vtable = ICoreWebView2ControllerFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICoreWebView2ControllerFactory {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x270b2c5b_c3a9_53d8_a5ca_262ea9ea62e8);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2ControllerFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICoreWebView2ControllerOptions(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreWebView2ControllerOptions {
    type Vtable = ICoreWebView2ControllerOptions_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICoreWebView2ControllerOptions {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x3337e821_3606_5a0e_8e2f_0c1e57d743f7);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2ControllerOptions_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ProfileName: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub SetProfileName: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub IsInPrivateModeEnabled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetIsInPrivateModeEnabled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICoreWebView2ControllerOptions2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreWebView2ControllerOptions2 {
    type Vtable = ICoreWebView2ControllerOptions2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICoreWebView2ControllerOptions2 {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x41b69e93_cc17_5c7d_a0c8_fa21c27aadb6);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2ControllerOptions2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ScriptLocale: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub SetScriptLocale: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICoreWebView2ControllerWindowReference(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreWebView2ControllerWindowReference {
    type Vtable = ICoreWebView2ControllerWindowReference_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICoreWebView2ControllerWindowReference {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x0feddad4_48a3_5cc4_9f61_e7adfd1e9c76);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2ControllerWindowReference_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub WindowHandle: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut u64,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_UI_Core")]
    pub CoreWindow: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI_Core"))]
    CoreWindow: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICoreWebView2ControllerWindowReferenceStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreWebView2ControllerWindowReferenceStatics {
    type Vtable = ICoreWebView2ControllerWindowReferenceStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICoreWebView2ControllerWindowReferenceStatics {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xddf6ebf1_ebc6_5a34_9008_661c3a2eb767);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2ControllerWindowReferenceStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateFromWindowHandle: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        windowhandle: u64,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_UI_Core")]
    pub CreateFromCoreWindow: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        corewindow: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_UI_Core"))]
    CreateFromCoreWindow: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICoreWebView2Cookie(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreWebView2Cookie {
    type Vtable = ICoreWebView2Cookie_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICoreWebView2Cookie {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x52f670fe_8ca2_5aad_aedb_25f7903b7038);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2Cookie_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Name: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub Value: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub SetValue: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub Domain: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub Path: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub Expires: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub SetExpires: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
    ) -> ::windows_core::HRESULT,
    pub IsHttpOnly: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetIsHttpOnly: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
    pub SameSite: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut CoreWebView2CookieSameSiteKind,
    ) -> ::windows_core::HRESULT,
    pub SetSameSite: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: CoreWebView2CookieSameSiteKind,
    ) -> ::windows_core::HRESULT,
    pub IsSecure: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetIsSecure: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
    pub IsSession: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICoreWebView2CookieManager(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreWebView2CookieManager {
    type Vtable = ICoreWebView2CookieManager_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICoreWebView2CookieManager {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x4098f516_adca_5563_aaa5_d7affd847aa3);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2CookieManager_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateCookie: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        name: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        value: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        domain: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        path: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub CopyCookie: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        cookieparam: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub AddOrUpdateCookie: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        cookie: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub DeleteCookie: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        cookie: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub DeleteCookies: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        name: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        uri: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub DeleteCookiesWithDomainAndPath: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        name: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        domain: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        path: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub DeleteAllCookies:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICoreWebView2CookieManager_Manual(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreWebView2CookieManager_Manual {
    type Vtable = ICoreWebView2CookieManager_Manual_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICoreWebView2CookieManager_Manual {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x9bcca0ea_7225_577a_bb23_c7c98023154e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2CookieManager_Manual_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub GetCookiesAsync: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        uri: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation_Collections"))]
    GetCookiesAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICoreWebView2CustomSchemeRegistration(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreWebView2CustomSchemeRegistration {
    type Vtable = ICoreWebView2CustomSchemeRegistration_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICoreWebView2CustomSchemeRegistration {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x16dc60d9_ddec_5c3d_bc1f_4408d1875af1);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2CustomSchemeRegistration_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub TreatAsSecure: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut i32,
    ) -> ::windows_core::HRESULT,
    pub SetTreatAsSecure: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: i32,
    ) -> ::windows_core::HRESULT,
    pub HasAuthorityComponent: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetHasAuthorityComponent: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICoreWebView2DOMContentLoadedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreWebView2DOMContentLoadedEventArgs {
    type Vtable = ICoreWebView2DOMContentLoadedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICoreWebView2DOMContentLoadedEventArgs {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xc474d0a3_24ac_59fc_b78b_da7562a6a052);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2DOMContentLoadedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub NavigationId: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut u64,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICoreWebView2DevToolsProtocolEventReceivedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreWebView2DevToolsProtocolEventReceivedEventArgs {
    type Vtable = ICoreWebView2DevToolsProtocolEventReceivedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICoreWebView2DevToolsProtocolEventReceivedEventArgs {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xb6a4b41d_fd18_59fa_923a_c57555d960ce);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2DevToolsProtocolEventReceivedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ParameterObjectAsJson: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICoreWebView2DevToolsProtocolEventReceivedEventArgs2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreWebView2DevToolsProtocolEventReceivedEventArgs2 {
    type Vtable = ICoreWebView2DevToolsProtocolEventReceivedEventArgs2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICoreWebView2DevToolsProtocolEventReceivedEventArgs2 {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x221728ba_635e_50d2_bd3c_fd22f4113978);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2DevToolsProtocolEventReceivedEventArgs2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SessionId: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICoreWebView2DevToolsProtocolEventReceiver(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreWebView2DevToolsProtocolEventReceiver {
    type Vtable = ICoreWebView2DevToolsProtocolEventReceiver_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICoreWebView2DevToolsProtocolEventReceiver {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xb2a2be79_65fc_5537_8715_3d92bf31090b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2DevToolsProtocolEventReceiver_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Windows_Foundation")]
    pub DevToolsProtocolEventReceived: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    DevToolsProtocolEventReceived: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub RemoveDevToolsProtocolEventReceived: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    )
        -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RemoveDevToolsProtocolEventReceived: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICoreWebView2DispatchAdapter(::windows_core::IUnknown);
impl ICoreWebView2DispatchAdapter {
    pub fn WrapNamedObject<P0>(
        &self,
        name: &::windows_core::HSTRING,
        adapter: P0,
    ) -> ::windows_core::Result<::windows_core::IInspectable>
    where
        P0: ::windows_core::TryIntoParam<ICoreWebView2DispatchAdapter>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).WrapNamedObject)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(name),
                adapter.try_into_param()?.abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn WrapObject<P0, P1>(
        &self,
        unwrapped: P0,
        adapter: P1,
    ) -> ::windows_core::Result<::windows_core::IInspectable>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
        P1: ::windows_core::TryIntoParam<ICoreWebView2DispatchAdapter>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).WrapObject)(
                ::windows_core::Interface::as_raw(this),
                unwrapped.into_param().abi(),
                adapter.try_into_param()?.abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn UnwrapObject<P0>(
        &self,
        wrapped: P0,
    ) -> ::windows_core::Result<::windows_core::IInspectable>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).UnwrapObject)(
                ::windows_core::Interface::as_raw(this),
                wrapped.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn Clean(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).Clean)(::windows_core::Interface::as_raw(this))
                .ok()
        }
    }
}
::windows_core::imp::interface_hierarchy!(
    ICoreWebView2DispatchAdapter,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::RuntimeType for ICoreWebView2DispatchAdapter {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_interface::<Self>();
}
unsafe impl ::windows_core::Interface for ICoreWebView2DispatchAdapter {
    type Vtable = ICoreWebView2DispatchAdapter_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICoreWebView2DispatchAdapter {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x7888a42d_18f3_5966_80cb_8cc25351bd0a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2DispatchAdapter_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub WrapNamedObject: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        name: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        adapter: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub WrapObject: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        unwrapped: *mut ::core::ffi::c_void,
        adapter: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub UnwrapObject: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        wrapped: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub Clean: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICoreWebView2DownloadOperation(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreWebView2DownloadOperation {
    type Vtable = ICoreWebView2DownloadOperation_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICoreWebView2DownloadOperation {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xafe73e6b_e760_5a06_9bf6_1e743c13cd2d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2DownloadOperation_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Uri: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub ContentDisposition: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub MimeType: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub TotalBytesToReceive: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut i64,
    ) -> ::windows_core::HRESULT,
    pub BytesReceived: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut i64,
    ) -> ::windows_core::HRESULT,
    pub EstimatedEndTime: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub ResultFilePath: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub State: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut CoreWebView2DownloadState,
    ) -> ::windows_core::HRESULT,
    pub InterruptReason: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut CoreWebView2DownloadInterruptReason,
    ) -> ::windows_core::HRESULT,
    pub CanResume: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_Foundation")]
    pub BytesReceivedChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    BytesReceivedChanged: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub RemoveBytesReceivedChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RemoveBytesReceivedChanged: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub EstimatedEndTimeChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    EstimatedEndTimeChanged: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub RemoveEstimatedEndTimeChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RemoveEstimatedEndTimeChanged: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub StateChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    StateChanged: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub RemoveStateChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RemoveStateChanged: usize,
    pub Cancel:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Pause: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Resume:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICoreWebView2DownloadStartingEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreWebView2DownloadStartingEventArgs {
    type Vtable = ICoreWebView2DownloadStartingEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICoreWebView2DownloadStartingEventArgs {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x45d982ba_9256_5b35_b023_26a438599110);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2DownloadStartingEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub DownloadOperation: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub Cancel: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetCancel: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
    pub ResultFilePath: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub SetResultFilePath: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub Handled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_Foundation")]
    pub GetDeferral: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    GetDeferral: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICoreWebView2Environment(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreWebView2Environment {
    type Vtable = ICoreWebView2Environment_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICoreWebView2Environment {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xd8cc7831_b783_556b_b9ce_899c1e95d585);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2Environment_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub BrowserVersionString: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_Foundation")]
    pub NewBrowserVersionAvailable: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    NewBrowserVersionAvailable: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub RemoveNewBrowserVersionAvailable: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RemoveNewBrowserVersionAvailable: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub CreateCoreWebView2ControllerAsync: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        parentwindow: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    )
        -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    CreateCoreWebView2ControllerAsync: usize,
    #[cfg(feature = "Windows_Storage_Streams")]
    pub CreateWebResourceResponse: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        content: *mut ::core::ffi::c_void,
        statuscode: i32,
        reasonphrase: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        headers: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Storage_Streams"))]
    CreateWebResourceResponse: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICoreWebView2Environment10(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreWebView2Environment10 {
    type Vtable = ICoreWebView2Environment10_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICoreWebView2Environment10 {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xc224e69c_1efd_5ecc_adc8_2b52e7b97ce5);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2Environment10_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateCoreWebView2ControllerOptions: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    )
        -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICoreWebView2Environment11(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreWebView2Environment11 {
    type Vtable = ICoreWebView2Environment11_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICoreWebView2Environment11 {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xda23d64c_8b61_5b6c_8581_f6a688abd7cd);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2Environment11_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub FailureReportFolderPath: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICoreWebView2Environment12(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreWebView2Environment12 {
    type Vtable = ICoreWebView2Environment12_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICoreWebView2Environment12 {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x82531ddb_be63_5254_812f_880d9f0ec54e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2Environment12_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateSharedBuffer: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        size: u64,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICoreWebView2Environment2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreWebView2Environment2 {
    type Vtable = ICoreWebView2Environment2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICoreWebView2Environment2 {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x0b634668_1017_5fc7_9921_f1f51866a8c0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2Environment2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Windows_Storage_Streams")]
    pub CreateWebResourceRequest: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        uri: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        method: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        postdata: *mut ::core::ffi::c_void,
        headers: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Storage_Streams"))]
    CreateWebResourceRequest: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICoreWebView2Environment3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreWebView2Environment3 {
    type Vtable = ICoreWebView2Environment3_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICoreWebView2Environment3 {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x5e33f46c_c0b9_5126_8840_17f9c11b3a8a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2Environment3_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Windows_Foundation")]
    pub CreateCoreWebView2CompositionControllerAsync:
        unsafe extern "system" fn(
            this: *mut ::core::ffi::c_void,
            parentwindow: *mut ::core::ffi::c_void,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    CreateCoreWebView2CompositionControllerAsync: usize,
    pub CreateCoreWebView2PointerInfo: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICoreWebView2Environment4(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreWebView2Environment4 {
    type Vtable = ICoreWebView2Environment4_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICoreWebView2Environment4 {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x6db697da_eebd_5818_8790_1fe57ef319e2);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2Environment4_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICoreWebView2Environment5(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreWebView2Environment5 {
    type Vtable = ICoreWebView2Environment5_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICoreWebView2Environment5 {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xf33399af_e4d3_59dc_ac38_8397aadcedb1);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2Environment5_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Windows_Foundation")]
    pub BrowserProcessExited: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    BrowserProcessExited: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub RemoveBrowserProcessExited: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RemoveBrowserProcessExited: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICoreWebView2Environment6(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreWebView2Environment6 {
    type Vtable = ICoreWebView2Environment6_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICoreWebView2Environment6 {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x965d538f_5958_5d98_8972_f622021df402);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2Environment6_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreatePrintSettings: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICoreWebView2Environment7(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreWebView2Environment7 {
    type Vtable = ICoreWebView2Environment7_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICoreWebView2Environment7 {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xe1f44fe2_fc54_5383_a383_c87e1da96b83);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2Environment7_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub UserDataFolder: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICoreWebView2Environment8(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreWebView2Environment8 {
    type Vtable = ICoreWebView2Environment8_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICoreWebView2Environment8 {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xdb67c807_d0db_5980_a3a9_75ef8f63d6f6);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2Environment8_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Windows_Foundation")]
    pub ProcessInfosChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    ProcessInfosChanged: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub RemoveProcessInfosChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RemoveProcessInfosChanged: usize,
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub GetProcessInfos: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation_Collections"))]
    GetProcessInfos: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICoreWebView2Environment9(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreWebView2Environment9 {
    type Vtable = ICoreWebView2Environment9_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICoreWebView2Environment9 {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xc8213ec7_7dc9_5468_a88b_15c6b7144478);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2Environment9_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Windows_Storage_Streams")]
    pub CreateContextMenuItem: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        label: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        iconstream: *mut ::core::ffi::c_void,
        kind: CoreWebView2ContextMenuItemKind,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Storage_Streams"))]
    CreateContextMenuItem: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICoreWebView2EnvironmentOptions(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreWebView2EnvironmentOptions {
    type Vtable = ICoreWebView2EnvironmentOptions_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICoreWebView2EnvironmentOptions {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x25d6dc39_0062_5735_8b09_a6f535f19e97);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2EnvironmentOptions_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub AdditionalBrowserArguments: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub SetAdditionalBrowserArguments: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub Language: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub SetLanguage: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub TargetCompatibleBrowserVersion: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub SetTargetCompatibleBrowserVersion: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    )
        -> ::windows_core::HRESULT,
    pub AllowSingleSignOnUsingOSPrimaryAccount:
        unsafe extern "system" fn(
            this: *mut ::core::ffi::c_void,
            result__: *mut bool,
        ) -> ::windows_core::HRESULT,
    pub SetAllowSingleSignOnUsingOSPrimaryAccount:
        unsafe extern "system" fn(
            this: *mut ::core::ffi::c_void,
            value: bool,
        ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICoreWebView2EnvironmentOptions2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreWebView2EnvironmentOptions2 {
    type Vtable = ICoreWebView2EnvironmentOptions2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICoreWebView2EnvironmentOptions2 {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xe77350fb_77a1_56f7_be95_eb7f8a7a3072);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2EnvironmentOptions2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ExclusiveUserDataFolderAccess: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetExclusiveUserDataFolderAccess: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICoreWebView2EnvironmentOptions3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreWebView2EnvironmentOptions3 {
    type Vtable = ICoreWebView2EnvironmentOptions3_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICoreWebView2EnvironmentOptions3 {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x48ab919d_2444_5e8c_a6f6_aba840d6c5ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2EnvironmentOptions3_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub IsCustomCrashReportingEnabled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetIsCustomCrashReportingEnabled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICoreWebView2EnvironmentOptions4(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreWebView2EnvironmentOptions4 {
    type Vtable = ICoreWebView2EnvironmentOptions4_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICoreWebView2EnvironmentOptions4 {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xa2cb850f_cd14_5a7d_9c98_53fd51ec9858);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2EnvironmentOptions4_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICoreWebView2EnvironmentOptions5(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreWebView2EnvironmentOptions5 {
    type Vtable = ICoreWebView2EnvironmentOptions5_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICoreWebView2EnvironmentOptions5 {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x36b1ca6c_e06c_5050_8ef9_247c5a7aa9c9);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2EnvironmentOptions5_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub EnableTrackingPrevention: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetEnableTrackingPrevention: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICoreWebView2EnvironmentOptions_Manual(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreWebView2EnvironmentOptions_Manual {
    type Vtable = ICoreWebView2EnvironmentOptions_Manual_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICoreWebView2EnvironmentOptions_Manual {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x1f104443_ea93_5a37_b791_34e6a31172ed);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2EnvironmentOptions_Manual_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICoreWebView2EnvironmentStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreWebView2EnvironmentStatics {
    type Vtable = ICoreWebView2EnvironmentStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICoreWebView2EnvironmentStatics {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x0e33f804_f20b_5635_8491_162aaa27517b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2EnvironmentStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Windows_Foundation")]
    pub CreateAsync: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    CreateAsync: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub CreateWithOptionsAsync: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        browserexecutablefolder: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        userdatafolder: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        options: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    CreateWithOptionsAsync: usize,
    pub GetAvailableBrowserVersionString: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub GetAvailableBrowserVersionString2: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        browserexecutablefolder: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    )
        -> ::windows_core::HRESULT,
    pub CompareBrowserVersionString: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        browserversionstring1: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        browserversionstring2: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        result__: *mut i32,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICoreWebView2Environment_Manual(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreWebView2Environment_Manual {
    type Vtable = ICoreWebView2Environment_Manual_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICoreWebView2Environment_Manual {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xf51cfabe_73ad_5635_a935_6386aef9238e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2Environment_Manual_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Windows_Foundation")]
    pub CreateCoreWebView2ControllerAsync: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        parentwindow: *mut ::core::ffi::c_void,
        options: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    )
        -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    CreateCoreWebView2ControllerAsync: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub CreateCoreWebView2CompositionControllerAsync:
        unsafe extern "system" fn(
            this: *mut ::core::ffi::c_void,
            parentwindow: *mut ::core::ffi::c_void,
            options: *mut ::core::ffi::c_void,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    CreateCoreWebView2CompositionControllerAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICoreWebView2File(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreWebView2File {
    type Vtable = ICoreWebView2File_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICoreWebView2File {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xcab45512_9594_50f1_ac3c_9cc103b574a3);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2File_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Path: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICoreWebView2Frame(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreWebView2Frame {
    type Vtable = ICoreWebView2Frame_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICoreWebView2Frame {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x02ffcbf9_19e7_5bb8_8273_346420fb1503);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2Frame_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Name: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_Foundation")]
    pub NameChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    NameChanged: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub RemoveNameChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RemoveNameChanged: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub Destroyed: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    Destroyed: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub RemoveDestroyed: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RemoveDestroyed: usize,
    pub RemoveHostObjectFromScript: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        name: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub IsDestroyed: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut i32,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICoreWebView2Frame2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreWebView2Frame2 {
    type Vtable = ICoreWebView2Frame2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICoreWebView2Frame2 {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x33dbc9c9_a103_56e3_b722_363814200320);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2Frame2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Windows_Foundation")]
    pub NavigationStarting: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    NavigationStarting: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub RemoveNavigationStarting: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RemoveNavigationStarting: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub ContentLoading: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    ContentLoading: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub RemoveContentLoading: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RemoveContentLoading: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub NavigationCompleted: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    NavigationCompleted: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub RemoveNavigationCompleted: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RemoveNavigationCompleted: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub DOMContentLoaded: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    DOMContentLoaded: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub RemoveDOMContentLoaded: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RemoveDOMContentLoaded: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub WebMessageReceived: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    WebMessageReceived: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub RemoveWebMessageReceived: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RemoveWebMessageReceived: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub ExecuteScriptAsync: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        javascript: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    ExecuteScriptAsync: usize,
    pub PostWebMessageAsJson: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        webmessageasjson: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub PostWebMessageAsString: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        webmessageasstring: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICoreWebView2Frame3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreWebView2Frame3 {
    type Vtable = ICoreWebView2Frame3_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICoreWebView2Frame3 {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x6545dac4_1666_50a5_bbe8_ec04842a466f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2Frame3_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Windows_Foundation")]
    pub PermissionRequested: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    PermissionRequested: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub RemovePermissionRequested: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RemovePermissionRequested: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICoreWebView2Frame4(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreWebView2Frame4 {
    type Vtable = ICoreWebView2Frame4_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICoreWebView2Frame4 {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xd46cd758_64b9_543e_a7b8_cac9b4c059a7);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2Frame4_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub PostSharedBufferToScript: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        sharedbuffer: *mut ::core::ffi::c_void,
        access: CoreWebView2SharedBufferAccess,
        additionaldataasjson: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICoreWebView2FrameCreatedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreWebView2FrameCreatedEventArgs {
    type Vtable = ICoreWebView2FrameCreatedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICoreWebView2FrameCreatedEventArgs {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x527b01b8_fc6d_5543_8dce_96cdfdb32c81);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2FrameCreatedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Frame: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICoreWebView2FrameInfo(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreWebView2FrameInfo {
    type Vtable = ICoreWebView2FrameInfo_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICoreWebView2FrameInfo {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xf9b82e06_73f3_513b_bc2c_445ddedba976);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2FrameInfo_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Name: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub Source: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICoreWebView2HttpHeadersCollectionIterator(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreWebView2HttpHeadersCollectionIterator {
    type Vtable = ICoreWebView2HttpHeadersCollectionIterator_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICoreWebView2HttpHeadersCollectionIterator {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xadf264ee_d980_5f48_a60e_8705de046608);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2HttpHeadersCollectionIterator_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICoreWebView2HttpRequestHeaders(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreWebView2HttpRequestHeaders {
    type Vtable = ICoreWebView2HttpRequestHeaders_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICoreWebView2HttpRequestHeaders {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xdc2226c7_3515_55bb_bcb2_57b78f86b91d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2HttpRequestHeaders_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub GetHeader: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        name: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub GetHeaders: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        name: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub Contains: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        name: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetHeader: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        name: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        value: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub RemoveHeader: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        name: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICoreWebView2HttpResponseHeaders(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreWebView2HttpResponseHeaders {
    type Vtable = ICoreWebView2HttpResponseHeaders_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICoreWebView2HttpResponseHeaders {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xf3d383e9_747f_5574_8662_9a6b920cecd4);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2HttpResponseHeaders_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub AppendHeader: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        name: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        value: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub Contains: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        name: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub GetHeader: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        name: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub GetHeaders: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        name: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICoreWebView2LaunchingExternalUriSchemeEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreWebView2LaunchingExternalUriSchemeEventArgs {
    type Vtable = ICoreWebView2LaunchingExternalUriSchemeEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICoreWebView2LaunchingExternalUriSchemeEventArgs {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x6ab44f8d_ec6a_56a1_ae3c_9c55dff6cbc6);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2LaunchingExternalUriSchemeEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Uri: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub InitiatingOrigin: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub IsUserInitiated: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub Cancel: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetCancel: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_Foundation")]
    pub GetDeferral: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    GetDeferral: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICoreWebView2MoveFocusRequestedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreWebView2MoveFocusRequestedEventArgs {
    type Vtable = ICoreWebView2MoveFocusRequestedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICoreWebView2MoveFocusRequestedEventArgs {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x2e29103b_ecdd_5c1d_b288_3f066d608919);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2MoveFocusRequestedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Reason: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut CoreWebView2MoveFocusReason,
    ) -> ::windows_core::HRESULT,
    pub Handled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICoreWebView2NavigationCompletedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreWebView2NavigationCompletedEventArgs {
    type Vtable = ICoreWebView2NavigationCompletedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICoreWebView2NavigationCompletedEventArgs {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x4865e238_036a_5664_95a3_447ec44cf498);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2NavigationCompletedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub IsSuccess: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub WebErrorStatus: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut CoreWebView2WebErrorStatus,
    ) -> ::windows_core::HRESULT,
    pub NavigationId: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut u64,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICoreWebView2NavigationCompletedEventArgs2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreWebView2NavigationCompletedEventArgs2 {
    type Vtable = ICoreWebView2NavigationCompletedEventArgs2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICoreWebView2NavigationCompletedEventArgs2 {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x6e4d3c33_a6e2_5896_90c5_68b4b5e55b40);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2NavigationCompletedEventArgs2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub HttpStatusCode: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut i32,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICoreWebView2NavigationStartingEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreWebView2NavigationStartingEventArgs {
    type Vtable = ICoreWebView2NavigationStartingEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICoreWebView2NavigationStartingEventArgs {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x548d23d3_fea3_5616_bd05_ae08066c86d3);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2NavigationStartingEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Uri: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub IsUserInitiated: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub IsRedirected: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub RequestHeaders: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub Cancel: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetCancel: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
    pub NavigationId: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut u64,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICoreWebView2NavigationStartingEventArgs2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreWebView2NavigationStartingEventArgs2 {
    type Vtable = ICoreWebView2NavigationStartingEventArgs2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICoreWebView2NavigationStartingEventArgs2 {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xd7a3824e_7654_5c4b_b069_e6501634d84c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2NavigationStartingEventArgs2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub AdditionalAllowedFrameAncestors: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub SetAdditionalAllowedFrameAncestors: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    )
        -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICoreWebView2NewWindowRequestedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreWebView2NewWindowRequestedEventArgs {
    type Vtable = ICoreWebView2NewWindowRequestedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICoreWebView2NewWindowRequestedEventArgs {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xe6e013ba_aec8_532e_9ac9_1590af7b25ec);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2NewWindowRequestedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Uri: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub NewWindow: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetNewWindow: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub Handled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
    pub IsUserInitiated: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub WindowFeatures: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_Foundation")]
    pub GetDeferral: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    GetDeferral: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICoreWebView2NewWindowRequestedEventArgs2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreWebView2NewWindowRequestedEventArgs2 {
    type Vtable = ICoreWebView2NewWindowRequestedEventArgs2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICoreWebView2NewWindowRequestedEventArgs2 {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xf4806259_e63a_5c0b_a02c_5f10e11094f4);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2NewWindowRequestedEventArgs2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Name: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICoreWebView2PermissionRequestedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreWebView2PermissionRequestedEventArgs {
    type Vtable = ICoreWebView2PermissionRequestedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICoreWebView2PermissionRequestedEventArgs {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x118bdd9b_cef1_5910_929e_c1a321328239);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2PermissionRequestedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Uri: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub PermissionKind: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut CoreWebView2PermissionKind,
    ) -> ::windows_core::HRESULT,
    pub IsUserInitiated: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub State: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut CoreWebView2PermissionState,
    ) -> ::windows_core::HRESULT,
    pub SetState: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: CoreWebView2PermissionState,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_Foundation")]
    pub GetDeferral: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    GetDeferral: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICoreWebView2PermissionRequestedEventArgs2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreWebView2PermissionRequestedEventArgs2 {
    type Vtable = ICoreWebView2PermissionRequestedEventArgs2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICoreWebView2PermissionRequestedEventArgs2 {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xa6652fba_ebe5_5891_addc_cb37da8f7e66);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2PermissionRequestedEventArgs2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Handled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICoreWebView2PermissionRequestedEventArgs3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreWebView2PermissionRequestedEventArgs3 {
    type Vtable = ICoreWebView2PermissionRequestedEventArgs3_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICoreWebView2PermissionRequestedEventArgs3 {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x200e8bcc_bc11_5beb_aa7a_79d4c95d73aa);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2PermissionRequestedEventArgs3_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SavesInProfile: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetSavesInProfile: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICoreWebView2PermissionSetting(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreWebView2PermissionSetting {
    type Vtable = ICoreWebView2PermissionSetting_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICoreWebView2PermissionSetting {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xb4158d0b_8ef8_575f_8e99_5fe02e8b579e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2PermissionSetting_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub PermissionKind: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut CoreWebView2PermissionKind,
    ) -> ::windows_core::HRESULT,
    pub PermissionOrigin: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub PermissionState: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut CoreWebView2PermissionState,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICoreWebView2PointerInfo(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreWebView2PointerInfo {
    type Vtable = ICoreWebView2PointerInfo_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICoreWebView2PointerInfo {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xc3860e0d_c018_5a84_bc06_9f8f7b275dff);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2PointerInfo_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub PointerKind: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut u32,
    ) -> ::windows_core::HRESULT,
    pub SetPointerKind: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: u32,
    ) -> ::windows_core::HRESULT,
    pub PointerId: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut u32,
    ) -> ::windows_core::HRESULT,
    pub SetPointerId: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: u32,
    ) -> ::windows_core::HRESULT,
    pub FrameId: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut u32,
    ) -> ::windows_core::HRESULT,
    pub SetFrameId: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: u32,
    ) -> ::windows_core::HRESULT,
    pub PointerFlags: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut u32,
    ) -> ::windows_core::HRESULT,
    pub SetPointerFlags: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: u32,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_Foundation")]
    pub PointerDeviceRect: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::Rect,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    PointerDeviceRect: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub SetPointerDeviceRect: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::windows::Foundation::Rect,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    SetPointerDeviceRect: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub DisplayRect: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::Rect,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    DisplayRect: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub SetDisplayRect: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::windows::Foundation::Rect,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    SetDisplayRect: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub PixelLocation: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::Point,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    PixelLocation: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub SetPixelLocation: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::windows::Foundation::Point,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    SetPixelLocation: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub HimetricLocation: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::Point,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    HimetricLocation: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub SetHimetricLocation: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::windows::Foundation::Point,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    SetHimetricLocation: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub PixelLocationRaw: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::Point,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    PixelLocationRaw: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub SetPixelLocationRaw: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::windows::Foundation::Point,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    SetPixelLocationRaw: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub HimetricLocationRaw: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::Point,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    HimetricLocationRaw: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub SetHimetricLocationRaw: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::windows::Foundation::Point,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    SetHimetricLocationRaw: usize,
    pub Time: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut u32,
    ) -> ::windows_core::HRESULT,
    pub SetTime: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: u32,
    ) -> ::windows_core::HRESULT,
    pub HistoryCount: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut u32,
    ) -> ::windows_core::HRESULT,
    pub SetHistoryCount: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: u32,
    ) -> ::windows_core::HRESULT,
    pub InputData: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut i32,
    ) -> ::windows_core::HRESULT,
    pub SetInputData: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: i32,
    ) -> ::windows_core::HRESULT,
    pub KeyStates: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut u32,
    ) -> ::windows_core::HRESULT,
    pub SetKeyStates: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: u32,
    ) -> ::windows_core::HRESULT,
    pub PerformanceCount: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut u64,
    ) -> ::windows_core::HRESULT,
    pub SetPerformanceCount: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: u64,
    ) -> ::windows_core::HRESULT,
    pub ButtonChangeKind: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut i32,
    ) -> ::windows_core::HRESULT,
    pub SetButtonChangeKind: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: i32,
    ) -> ::windows_core::HRESULT,
    pub PenFlags: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut u32,
    ) -> ::windows_core::HRESULT,
    pub SetPenFlags: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: u32,
    ) -> ::windows_core::HRESULT,
    pub PenMask: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut u32,
    ) -> ::windows_core::HRESULT,
    pub SetPenMask: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: u32,
    ) -> ::windows_core::HRESULT,
    pub PenPressure: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut u32,
    ) -> ::windows_core::HRESULT,
    pub SetPenPressure: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: u32,
    ) -> ::windows_core::HRESULT,
    pub PenRotation: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut u32,
    ) -> ::windows_core::HRESULT,
    pub SetPenRotation: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: u32,
    ) -> ::windows_core::HRESULT,
    pub PenTiltX: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut i32,
    ) -> ::windows_core::HRESULT,
    pub SetPenTiltX: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: i32,
    ) -> ::windows_core::HRESULT,
    pub PenTiltY: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut i32,
    ) -> ::windows_core::HRESULT,
    pub SetPenTiltY: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: i32,
    ) -> ::windows_core::HRESULT,
    pub TouchFlags: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut u32,
    ) -> ::windows_core::HRESULT,
    pub SetTouchFlags: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: u32,
    ) -> ::windows_core::HRESULT,
    pub TouchMask: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut u32,
    ) -> ::windows_core::HRESULT,
    pub SetTouchMask: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: u32,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_Foundation")]
    pub TouchContact: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::Rect,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    TouchContact: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub SetTouchContact: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::windows::Foundation::Rect,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    SetTouchContact: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub TouchContactRaw: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::Rect,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    TouchContactRaw: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub SetTouchContactRaw: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::windows::Foundation::Rect,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    SetTouchContactRaw: usize,
    pub TouchOrientation: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut u32,
    ) -> ::windows_core::HRESULT,
    pub SetTouchOrientation: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: u32,
    ) -> ::windows_core::HRESULT,
    pub TouchPressure: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut u32,
    ) -> ::windows_core::HRESULT,
    pub SetTouchPressure: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: u32,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICoreWebView2PrintSettings(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreWebView2PrintSettings {
    type Vtable = ICoreWebView2PrintSettings_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICoreWebView2PrintSettings {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x9c75c8c0_ef3d_58a8_9a8c_18eed9fd0f16);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2PrintSettings_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Orientation: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut CoreWebView2PrintOrientation,
    ) -> ::windows_core::HRESULT,
    pub SetOrientation: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: CoreWebView2PrintOrientation,
    ) -> ::windows_core::HRESULT,
    pub ScaleFactor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub SetScaleFactor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
    ) -> ::windows_core::HRESULT,
    pub PageWidth: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub SetPageWidth: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
    ) -> ::windows_core::HRESULT,
    pub PageHeight: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub SetPageHeight: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
    ) -> ::windows_core::HRESULT,
    pub MarginTop: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub SetMarginTop: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
    ) -> ::windows_core::HRESULT,
    pub MarginBottom: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub SetMarginBottom: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
    ) -> ::windows_core::HRESULT,
    pub MarginLeft: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub SetMarginLeft: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
    ) -> ::windows_core::HRESULT,
    pub MarginRight: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f64,
    ) -> ::windows_core::HRESULT,
    pub SetMarginRight: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f64,
    ) -> ::windows_core::HRESULT,
    pub ShouldPrintBackgrounds: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetShouldPrintBackgrounds: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
    pub ShouldPrintSelectionOnly: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetShouldPrintSelectionOnly: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
    pub ShouldPrintHeaderAndFooter: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetShouldPrintHeaderAndFooter: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
    pub HeaderTitle: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub SetHeaderTitle: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub FooterUri: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub SetFooterUri: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICoreWebView2PrintSettings2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreWebView2PrintSettings2 {
    type Vtable = ICoreWebView2PrintSettings2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICoreWebView2PrintSettings2 {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xd2a97895_ca6e_57fc_905d_c6f77a081768);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2PrintSettings2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub PageRanges: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub SetPageRanges: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub PagesPerSide: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut i32,
    ) -> ::windows_core::HRESULT,
    pub SetPagesPerSide: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: i32,
    ) -> ::windows_core::HRESULT,
    pub Copies: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut i32,
    ) -> ::windows_core::HRESULT,
    pub SetCopies: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: i32,
    ) -> ::windows_core::HRESULT,
    pub Collation: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut CoreWebView2PrintCollation,
    ) -> ::windows_core::HRESULT,
    pub SetCollation: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: CoreWebView2PrintCollation,
    ) -> ::windows_core::HRESULT,
    pub ColorMode: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut CoreWebView2PrintColorMode,
    ) -> ::windows_core::HRESULT,
    pub SetColorMode: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: CoreWebView2PrintColorMode,
    ) -> ::windows_core::HRESULT,
    pub Duplex: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut CoreWebView2PrintDuplex,
    ) -> ::windows_core::HRESULT,
    pub SetDuplex: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: CoreWebView2PrintDuplex,
    ) -> ::windows_core::HRESULT,
    pub MediaSize: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut CoreWebView2PrintMediaSize,
    ) -> ::windows_core::HRESULT,
    pub SetMediaSize: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: CoreWebView2PrintMediaSize,
    ) -> ::windows_core::HRESULT,
    pub PrinterName: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub SetPrinterName: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICoreWebView2PrivatePartial(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreWebView2PrivatePartial {
    type Vtable = ICoreWebView2PrivatePartial_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICoreWebView2PrivatePartial {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x2850f27c_0c9d_5cdc_b356_18f5b97d9fcf);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2PrivatePartial_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICoreWebView2PrivatePartialController(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreWebView2PrivatePartialController {
    type Vtable = ICoreWebView2PrivatePartialController_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICoreWebView2PrivatePartialController {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x527f6678_8629_5c2a_bc3b_8d5c95e2b9bc);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2PrivatePartialController_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub IsBrowserHitTransparent: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICoreWebView2ProcessFailedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreWebView2ProcessFailedEventArgs {
    type Vtable = ICoreWebView2ProcessFailedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICoreWebView2ProcessFailedEventArgs {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x25a8f8c9_d944_539d_afa3_24172b48ef47);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2ProcessFailedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ProcessFailedKind: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut CoreWebView2ProcessFailedKind,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICoreWebView2ProcessFailedEventArgs2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreWebView2ProcessFailedEventArgs2 {
    type Vtable = ICoreWebView2ProcessFailedEventArgs2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICoreWebView2ProcessFailedEventArgs2 {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xc5d9c952_b456_5dc7_9f76_fde967484af5);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2ProcessFailedEventArgs2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Reason: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut CoreWebView2ProcessFailedReason,
    ) -> ::windows_core::HRESULT,
    pub ExitCode: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut i32,
    ) -> ::windows_core::HRESULT,
    pub ProcessDescription: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub FrameInfosForFailedProcess: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation_Collections"))]
    FrameInfosForFailedProcess: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICoreWebView2ProcessInfo(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreWebView2ProcessInfo {
    type Vtable = ICoreWebView2ProcessInfo_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICoreWebView2ProcessInfo {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xb6ec37e1_23eb_5924_b346_e837890aa9d5);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2ProcessInfo_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ProcessId: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut i32,
    ) -> ::windows_core::HRESULT,
    pub Kind: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut CoreWebView2ProcessKind,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICoreWebView2Profile(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreWebView2Profile {
    type Vtable = ICoreWebView2Profile_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICoreWebView2Profile {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xd4bdd25c_a2db_5c03_9659_abdeb9793621);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2Profile_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ProfileName: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub IsInPrivateModeEnabled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub ProfilePath: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub DefaultDownloadFolderPath: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub SetDefaultDownloadFolderPath: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub PreferredColorScheme: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut CoreWebView2PreferredColorScheme,
    ) -> ::windows_core::HRESULT,
    pub SetPreferredColorScheme: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: CoreWebView2PreferredColorScheme,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICoreWebView2Profile2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreWebView2Profile2 {
    type Vtable = ICoreWebView2Profile2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICoreWebView2Profile2 {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x93d21e18_1b06_59d0_9687_10f4844b016d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2Profile2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Windows_Foundation")]
    pub ClearBrowsingDataAsync: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        datakinds: CoreWebView2BrowsingDataKinds,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    ClearBrowsingDataAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICoreWebView2Profile3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreWebView2Profile3 {
    type Vtable = ICoreWebView2Profile3_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICoreWebView2Profile3 {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x507ed587_c511_5e47_be5b_fc9ccdf179b6);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2Profile3_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub PreferredTrackingPreventionLevel: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut CoreWebView2TrackingPreventionLevel,
    ) -> ::windows_core::HRESULT,
    pub SetPreferredTrackingPreventionLevel: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: CoreWebView2TrackingPreventionLevel,
    )
        -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICoreWebView2Profile4(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreWebView2Profile4 {
    type Vtable = ICoreWebView2Profile4_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICoreWebView2Profile4 {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xeeae109a_f641_5a5b_942f_9922594ffb4d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2Profile4_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Windows_Foundation")]
    pub SetPermissionStateAsync: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        permissionkind: CoreWebView2PermissionKind,
        origin: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        state: CoreWebView2PermissionState,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    SetPermissionStateAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICoreWebView2Profile5(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreWebView2Profile5 {
    type Vtable = ICoreWebView2Profile5_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICoreWebView2Profile5 {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xc9aac8f7_e502_5485_b033_99e4940ee0f1);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2Profile5_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CookieManager: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICoreWebView2Profile6(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreWebView2Profile6 {
    type Vtable = ICoreWebView2Profile6_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICoreWebView2Profile6 {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xc16a4665_9d44_5768_94a3_69b3976fc3d6);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2Profile6_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub IsPasswordAutosaveEnabled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetIsPasswordAutosaveEnabled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
    pub IsGeneralAutofillEnabled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetIsGeneralAutofillEnabled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICoreWebView2ScriptDialogOpeningEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreWebView2ScriptDialogOpeningEventArgs {
    type Vtable = ICoreWebView2ScriptDialogOpeningEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICoreWebView2ScriptDialogOpeningEventArgs {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xa4315212_c7eb_568a_86e4_c61e31ba6cda);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2ScriptDialogOpeningEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Uri: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub Kind: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut CoreWebView2ScriptDialogKind,
    ) -> ::windows_core::HRESULT,
    pub Message: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub DefaultText: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub ResultText: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub SetResultText: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub Accept:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_Foundation")]
    pub GetDeferral: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    GetDeferral: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICoreWebView2ServerCertificateErrorDetectedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreWebView2ServerCertificateErrorDetectedEventArgs {
    type Vtable = ICoreWebView2ServerCertificateErrorDetectedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICoreWebView2ServerCertificateErrorDetectedEventArgs {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x90fdc703_5a9e_56f6_a422_7c114c736420);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2ServerCertificateErrorDetectedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ErrorStatus: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut CoreWebView2WebErrorStatus,
    ) -> ::windows_core::HRESULT,
    pub RequestUri: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub ServerCertificate: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub Action: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut CoreWebView2ServerCertificateErrorAction,
    ) -> ::windows_core::HRESULT,
    pub SetAction: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: CoreWebView2ServerCertificateErrorAction,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_Foundation")]
    pub GetDeferral: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    GetDeferral: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICoreWebView2Settings(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreWebView2Settings {
    type Vtable = ICoreWebView2Settings_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICoreWebView2Settings {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x003b325e_74cd_52dd_8024_ebb8be38e48e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2Settings_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub IsScriptEnabled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetIsScriptEnabled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
    pub IsWebMessageEnabled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetIsWebMessageEnabled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
    pub AreDefaultScriptDialogsEnabled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetAreDefaultScriptDialogsEnabled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    )
        -> ::windows_core::HRESULT,
    pub IsStatusBarEnabled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetIsStatusBarEnabled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
    pub AreDevToolsEnabled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetAreDevToolsEnabled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
    pub AreDefaultContextMenusEnabled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetAreDefaultContextMenusEnabled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
    pub AreHostObjectsAllowed: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetAreHostObjectsAllowed: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
    pub IsZoomControlEnabled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetIsZoomControlEnabled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
    pub IsBuiltInErrorPageEnabled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetIsBuiltInErrorPageEnabled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICoreWebView2Settings2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreWebView2Settings2 {
    type Vtable = ICoreWebView2Settings2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICoreWebView2Settings2 {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x377d3480_fdb2_56e7_bade_507d352887e9);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2Settings2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub UserAgent: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub SetUserAgent: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICoreWebView2Settings3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreWebView2Settings3 {
    type Vtable = ICoreWebView2Settings3_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICoreWebView2Settings3 {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x52200f01_5309_5b2e_a03c_3d2677591940);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2Settings3_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub AreBrowserAcceleratorKeysEnabled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetAreBrowserAcceleratorKeysEnabled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    )
        -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICoreWebView2Settings4(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreWebView2Settings4 {
    type Vtable = ICoreWebView2Settings4_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICoreWebView2Settings4 {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xd6a955f0_daef_5a6a_a6f6_c72f0ede7620);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2Settings4_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub IsPasswordAutosaveEnabled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetIsPasswordAutosaveEnabled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
    pub IsGeneralAutofillEnabled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetIsGeneralAutofillEnabled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICoreWebView2Settings5(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreWebView2Settings5 {
    type Vtable = ICoreWebView2Settings5_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICoreWebView2Settings5 {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xafc42b23_4839_5d73_acf7_e0335631abf5);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2Settings5_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub IsPinchZoomEnabled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetIsPinchZoomEnabled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICoreWebView2Settings6(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreWebView2Settings6 {
    type Vtable = ICoreWebView2Settings6_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICoreWebView2Settings6 {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x3fe4ae85_0540_5bf1_b4d9_99ec57aa64f5);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2Settings6_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub IsSwipeNavigationEnabled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetIsSwipeNavigationEnabled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICoreWebView2Settings7(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreWebView2Settings7 {
    type Vtable = ICoreWebView2Settings7_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICoreWebView2Settings7 {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x688027cd_9f84_59e8_8d5c_91123df24b92);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2Settings7_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub HiddenPdfToolbarItems: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut CoreWebView2PdfToolbarItems,
    ) -> ::windows_core::HRESULT,
    pub SetHiddenPdfToolbarItems: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: CoreWebView2PdfToolbarItems,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICoreWebView2Settings8(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreWebView2Settings8 {
    type Vtable = ICoreWebView2Settings8_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICoreWebView2Settings8 {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x956f1a8f_3198_5577_b250_7d91d17f7eed);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2Settings8_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub IsReputationCheckingRequired: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetIsReputationCheckingRequired: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICoreWebView2Settings_Manual(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreWebView2Settings_Manual {
    type Vtable = ICoreWebView2Settings_Manual_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICoreWebView2Settings_Manual {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x0a538c87_e000_511c_87ca_ded3413d03da);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2Settings_Manual_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub HostObjectDispatchAdapter: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetHostObjectDispatchAdapter: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICoreWebView2SharedBuffer(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreWebView2SharedBuffer {
    type Vtable = ICoreWebView2SharedBuffer_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICoreWebView2SharedBuffer {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x2907cc84_f843_5959_8734_f871766f8f13);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2SharedBuffer_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Size: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut u64,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_Storage_Streams")]
    pub OpenStream: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Storage_Streams"))]
    OpenStream: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICoreWebView2SharedBuffer_Manual(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreWebView2SharedBuffer_Manual {
    type Vtable = ICoreWebView2SharedBuffer_Manual_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICoreWebView2SharedBuffer_Manual {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x1aa4e213_ace3_5f74_a2ae_c6489ceb3239);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2SharedBuffer_Manual_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Windows_Foundation")]
    pub Buffer: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    Buffer: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICoreWebView2SourceChangedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreWebView2SourceChangedEventArgs {
    type Vtable = ICoreWebView2SourceChangedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICoreWebView2SourceChangedEventArgs {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xca437b2c_6a18_5552_b749_b198f8cc34d9);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2SourceChangedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub IsNewDocument: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICoreWebView2WebMessageReceivedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreWebView2WebMessageReceivedEventArgs {
    type Vtable = ICoreWebView2WebMessageReceivedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICoreWebView2WebMessageReceivedEventArgs {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xeb066159_b725_5d5b_adc8_f5d7b9290304);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2WebMessageReceivedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Source: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub WebMessageAsJson: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub TryGetWebMessageAsString: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICoreWebView2WebMessageReceivedEventArgs2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreWebView2WebMessageReceivedEventArgs2 {
    type Vtable = ICoreWebView2WebMessageReceivedEventArgs2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICoreWebView2WebMessageReceivedEventArgs2 {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x71dc5fa0_08a0_5dea_9363_799df5021452);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2WebMessageReceivedEventArgs2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub AdditionalObjects: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation_Collections"))]
    AdditionalObjects: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICoreWebView2WebResourceRequest(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreWebView2WebResourceRequest {
    type Vtable = ICoreWebView2WebResourceRequest_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICoreWebView2WebResourceRequest {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x5c742259_67d2_5df2_8382_0f201b4d7197);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2WebResourceRequest_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Uri: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub SetUri: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub Method: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub SetMethod: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_Storage_Streams")]
    pub Content: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Storage_Streams"))]
    Content: usize,
    #[cfg(feature = "Windows_Storage_Streams")]
    pub SetContent: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Storage_Streams"))]
    SetContent: usize,
    pub Headers: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICoreWebView2WebResourceRequestedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreWebView2WebResourceRequestedEventArgs {
    type Vtable = ICoreWebView2WebResourceRequestedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICoreWebView2WebResourceRequestedEventArgs {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x577f1fc4_c943_54a9_9700_bd469b48bd41);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2WebResourceRequestedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Request: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub Response: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub SetResponse: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub ResourceContext: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut CoreWebView2WebResourceContext,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_Foundation")]
    pub GetDeferral: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    GetDeferral: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICoreWebView2WebResourceResponse(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreWebView2WebResourceResponse {
    type Vtable = ICoreWebView2WebResourceResponse_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICoreWebView2WebResourceResponse {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x14621923_e485_5f44_8f5d_bd4243bc398f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2WebResourceResponse_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Windows_Storage_Streams")]
    pub Content: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Storage_Streams"))]
    Content: usize,
    #[cfg(feature = "Windows_Storage_Streams")]
    pub SetContent: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Storage_Streams"))]
    SetContent: usize,
    pub Headers: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub StatusCode: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut i32,
    ) -> ::windows_core::HRESULT,
    pub SetStatusCode: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: i32,
    ) -> ::windows_core::HRESULT,
    pub ReasonPhrase: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    pub SetReasonPhrase: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICoreWebView2WebResourceResponseReceivedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreWebView2WebResourceResponseReceivedEventArgs {
    type Vtable = ICoreWebView2WebResourceResponseReceivedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICoreWebView2WebResourceResponseReceivedEventArgs {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x12424671_9711_54f4_bcdf_5f307add6ec2);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2WebResourceResponseReceivedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Request: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub Response: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICoreWebView2WebResourceResponseView(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreWebView2WebResourceResponseView {
    type Vtable = ICoreWebView2WebResourceResponseView_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICoreWebView2WebResourceResponseView {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x33ee060b_b578_5698_b541_fef87fe7fe72);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2WebResourceResponseView_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Headers: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub StatusCode: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut i32,
    ) -> ::windows_core::HRESULT,
    pub ReasonPhrase: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Windows_Foundation", feature = "Windows_Storage_Streams"))]
    pub GetContentAsync: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Windows_Foundation", feature = "Windows_Storage_Streams")))]
    GetContentAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICoreWebView2WindowFeatures(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreWebView2WindowFeatures {
    type Vtable = ICoreWebView2WindowFeatures_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICoreWebView2WindowFeatures {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xee8686d6_056f_5e06_824f_4e2a24c1c1d6);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2WindowFeatures_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub HasPosition: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub HasSize: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub Left: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut u32,
    ) -> ::windows_core::HRESULT,
    pub Top: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut u32,
    ) -> ::windows_core::HRESULT,
    pub Height: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut u32,
    ) -> ::windows_core::HRESULT,
    pub Width: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut u32,
    ) -> ::windows_core::HRESULT,
    pub ShouldDisplayMenuBar: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub ShouldDisplayStatus: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub ShouldDisplayToolbar: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub ShouldDisplayScrollBars: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICoreWebView2_10(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreWebView2_10 {
    type Vtable = ICoreWebView2_10_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICoreWebView2_10 {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xa7b20434_970f_54b1_aa63_3c42671fa9ab);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2_10_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Windows_Foundation")]
    pub BasicAuthenticationRequested: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    BasicAuthenticationRequested: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub RemoveBasicAuthenticationRequested: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    )
        -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RemoveBasicAuthenticationRequested: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICoreWebView2_11(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreWebView2_11 {
    type Vtable = ICoreWebView2_11_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICoreWebView2_11 {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xc00acbb1_ae32_501f_ad19_9d0ac32d6142);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2_11_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Windows_Foundation")]
    pub ContextMenuRequested: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    ContextMenuRequested: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub RemoveContextMenuRequested: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RemoveContextMenuRequested: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub CallDevToolsProtocolMethodForSessionAsync:
        unsafe extern "system" fn(
            this: *mut ::core::ffi::c_void,
            sessionid: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
            methodname: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
            parametersasjson: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    CallDevToolsProtocolMethodForSessionAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICoreWebView2_12(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreWebView2_12 {
    type Vtable = ICoreWebView2_12_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICoreWebView2_12 {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xdbbbe9a1_18d3_5f67_b362_0f4ae937d754);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2_12_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub StatusBarText: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_Foundation")]
    pub StatusBarTextChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    StatusBarTextChanged: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub RemoveStatusBarTextChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RemoveStatusBarTextChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICoreWebView2_13(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreWebView2_13 {
    type Vtable = ICoreWebView2_13_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICoreWebView2_13 {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x314b5846_dbc7_5de4_a792_647ea0f3296a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2_13_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Profile: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICoreWebView2_14(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreWebView2_14 {
    type Vtable = ICoreWebView2_14_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICoreWebView2_14 {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xa7647b24_3b1e_50a9_be24_6e8ac63fe491);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2_14_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Windows_Foundation")]
    pub ServerCertificateErrorDetected: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    ServerCertificateErrorDetected: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub RemoveServerCertificateErrorDetected: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    )
        -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RemoveServerCertificateErrorDetected: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub ClearServerCertificateErrorActionsAsync:
        unsafe extern "system" fn(
            this: *mut ::core::ffi::c_void,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    ClearServerCertificateErrorActionsAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICoreWebView2_15(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreWebView2_15 {
    type Vtable = ICoreWebView2_15_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICoreWebView2_15 {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x4443f532_d2ba_5ae2_a9b3_8de62bd5d4a9);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2_15_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub FaviconUri: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_Foundation")]
    pub FaviconChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    FaviconChanged: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub RemoveFaviconChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RemoveFaviconChanged: usize,
    #[cfg(all(feature = "Windows_Foundation", feature = "Windows_Storage_Streams"))]
    pub GetFaviconAsync: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        format: CoreWebView2FaviconImageFormat,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Windows_Foundation", feature = "Windows_Storage_Streams")))]
    GetFaviconAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICoreWebView2_16(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreWebView2_16 {
    type Vtable = ICoreWebView2_16_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICoreWebView2_16 {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x61d0a57c_6c4f_50ff_a137_314b0099a2b8);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2_16_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Windows_Foundation")]
    pub PrintAsync: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        printsettings: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    PrintAsync: usize,
    pub ShowPrintUI: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        printdialogkind: CoreWebView2PrintDialogKind,
    ) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Windows_Foundation", feature = "Windows_Storage_Streams"))]
    pub PrintToPdfStreamAsync: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        printsettings: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Windows_Foundation", feature = "Windows_Storage_Streams")))]
    PrintToPdfStreamAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICoreWebView2_17(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreWebView2_17 {
    type Vtable = ICoreWebView2_17_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICoreWebView2_17 {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x2345f9db_5488_559a_82af_9086cc4f7988);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2_17_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub PostSharedBufferToScript: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        sharedbuffer: *mut ::core::ffi::c_void,
        access: CoreWebView2SharedBufferAccess,
        additionaldataasjson: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICoreWebView2_18(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreWebView2_18 {
    type Vtable = ICoreWebView2_18_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICoreWebView2_18 {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x94f52e61_9d75_5a81_acd3_830ff29ce6f7);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2_18_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Windows_Foundation")]
    pub LaunchingExternalUriScheme: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    LaunchingExternalUriScheme: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub RemoveLaunchingExternalUriScheme: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RemoveLaunchingExternalUriScheme: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICoreWebView2_19(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreWebView2_19 {
    type Vtable = ICoreWebView2_19_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICoreWebView2_19 {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x35a94a5c_e027_5dc5_8c2b_c2fc7d589159);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2_19_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub MemoryUsageTargetLevel: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut CoreWebView2MemoryUsageTargetLevel,
    ) -> ::windows_core::HRESULT,
    pub SetMemoryUsageTargetLevel: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: CoreWebView2MemoryUsageTargetLevel,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICoreWebView2_2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreWebView2_2 {
    type Vtable = ICoreWebView2_2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICoreWebView2_2 {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x578cb133_2873_5408_bd9e_389bbe9fa7fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2_2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CookieManager: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    pub Environment: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_Foundation")]
    pub WebResourceResponseReceived: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    WebResourceResponseReceived: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub RemoveWebResourceResponseReceived: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    )
        -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RemoveWebResourceResponseReceived: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub DOMContentLoaded: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    DOMContentLoaded: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub RemoveDOMContentLoaded: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RemoveDOMContentLoaded: usize,
    pub NavigateWithWebResourceRequest: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        request: *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICoreWebView2_3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreWebView2_3 {
    type Vtable = ICoreWebView2_3_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICoreWebView2_3 {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xa8c76ae7_6170_5dfe_8f00_79cd76a9b4d9);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2_3_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub IsSuspended: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_Foundation")]
    pub TrySuspendAsync: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    TrySuspendAsync: usize,
    pub Resume:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetVirtualHostNameToFolderMapping: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        hostname: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        folderpath: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        accesskind: CoreWebView2HostResourceAccessKind,
    )
        -> ::windows_core::HRESULT,
    pub ClearVirtualHostNameToFolderMapping: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        hostname: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
    )
        -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICoreWebView2_4(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreWebView2_4 {
    type Vtable = ICoreWebView2_4_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICoreWebView2_4 {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x4ac595ce_1502_5775_b2c8_22c11a369c25);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2_4_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Windows_Foundation")]
    pub FrameCreated: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    FrameCreated: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub RemoveFrameCreated: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RemoveFrameCreated: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub DownloadStarting: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    DownloadStarting: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub RemoveDownloadStarting: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RemoveDownloadStarting: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICoreWebView2_5(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreWebView2_5 {
    type Vtable = ICoreWebView2_5_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICoreWebView2_5 {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xdd6af643_220c_5dc6_b0a8_22c41e472595);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2_5_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Windows_Foundation")]
    pub ClientCertificateRequested: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    ClientCertificateRequested: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub RemoveClientCertificateRequested: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RemoveClientCertificateRequested: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICoreWebView2_6(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreWebView2_6 {
    type Vtable = ICoreWebView2_6_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICoreWebView2_6 {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x92b34b96_853d_5bb6_ac52_30297ce805f1);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2_6_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub OpenTaskManagerWindow:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICoreWebView2_7(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreWebView2_7 {
    type Vtable = ICoreWebView2_7_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICoreWebView2_7 {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xf9b7107a_2e09_5596_a033_911ba12315f7);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2_7_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Windows_Foundation")]
    pub PrintToPdfAsync: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        resultfilepath: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        printsettings: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    PrintToPdfAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICoreWebView2_8(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreWebView2_8 {
    type Vtable = ICoreWebView2_8_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICoreWebView2_8 {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0xaa2503c0_8d1c_5a3d_b898_f55f7595268a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2_8_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub IsMuted: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub SetIsMuted: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows_core::HRESULT,
    pub IsDocumentPlayingAudio: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_Foundation")]
    pub IsMutedChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    IsMutedChanged: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub RemoveIsMutedChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RemoveIsMutedChanged: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub IsDocumentPlayingAudioChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    IsDocumentPlayingAudioChanged: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub RemoveIsDocumentPlayingAudioChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    )
        -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RemoveIsDocumentPlayingAudioChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICoreWebView2_9(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICoreWebView2_9 {
    type Vtable = ICoreWebView2_9_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICoreWebView2_9 {
    const IID: ::windows_core::GUID =
        ::windows_core::GUID::from_u128(0x64b2ec16_0b29_5216_bf86_e575c88f7031);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWebView2_9_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub IsDefaultDownloadDialogOpen: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows_core::HRESULT,
    pub DefaultDownloadDialogCornerAlignment: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut CoreWebView2DefaultDownloadDialogCornerAlignment,
    )
        -> ::windows_core::HRESULT,
    pub SetDefaultDownloadDialogCornerAlignment:
        unsafe extern "system" fn(
            this: *mut ::core::ffi::c_void,
            value: CoreWebView2DefaultDownloadDialogCornerAlignment,
        ) -> ::windows_core::HRESULT,
    #[cfg(feature = "Windows_Foundation")]
    pub DefaultDownloadDialogMargin: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::Point,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    DefaultDownloadDialogMargin: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub SetDefaultDownloadDialogMargin: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::windows::Foundation::Point,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    SetDefaultDownloadDialogMargin: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub IsDefaultDownloadDialogOpenChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    )
        -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    IsDefaultDownloadDialogOpenChanged: usize,
    #[cfg(feature = "Windows_Foundation")]
    pub RemoveIsDefaultDownloadDialogOpenChanged:
        unsafe extern "system" fn(
            this: *mut ::core::ffi::c_void,
            token: ::windows::Foundation::EventRegistrationToken,
        ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Windows_Foundation"))]
    RemoveIsDefaultDownloadDialogOpenChanged: usize,
    pub OpenDefaultDownloadDialog:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CloseDefaultDownloadDialog:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct CoreWebView2(::windows_core::IUnknown);
impl CoreWebView2 {
    pub fn Settings(&self) -> ::windows_core::Result<CoreWebView2Settings> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Settings)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn Source(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
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
    pub fn BrowserProcessId(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).BrowserProcessId)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn CanGoBack(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CanGoBack)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn CanGoForward(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CanGoForward)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn DocumentTitle(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DocumentTitle)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn ContainsFullScreenElement(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ContainsFullScreenElement)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn NavigationStarting<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<
                CoreWebView2,
                CoreWebView2NavigationStartingEventArgs,
            >,
        >,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NavigationStarting)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveNavigationStarting(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveNavigationStarting)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn ContentLoading<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<
                CoreWebView2,
                CoreWebView2ContentLoadingEventArgs,
            >,
        >,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ContentLoading)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveContentLoading(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveContentLoading)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn SourceChanged<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<
                CoreWebView2,
                CoreWebView2SourceChangedEventArgs,
            >,
        >,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SourceChanged)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveSourceChanged(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveSourceChanged)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn HistoryChanged<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<CoreWebView2, ::windows_core::IInspectable>,
        >,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HistoryChanged)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveHistoryChanged(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveHistoryChanged)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn NavigationCompleted<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<
                CoreWebView2,
                CoreWebView2NavigationCompletedEventArgs,
            >,
        >,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NavigationCompleted)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveNavigationCompleted(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveNavigationCompleted)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn FrameNavigationStarting<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<
                CoreWebView2,
                CoreWebView2NavigationStartingEventArgs,
            >,
        >,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FrameNavigationStarting)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveFrameNavigationStarting(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveFrameNavigationStarting)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn FrameNavigationCompleted<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<
                CoreWebView2,
                CoreWebView2NavigationCompletedEventArgs,
            >,
        >,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FrameNavigationCompleted)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveFrameNavigationCompleted(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveFrameNavigationCompleted)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn ScriptDialogOpening<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<
                CoreWebView2,
                CoreWebView2ScriptDialogOpeningEventArgs,
            >,
        >,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ScriptDialogOpening)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveScriptDialogOpening(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveScriptDialogOpening)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn PermissionRequested<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<
                CoreWebView2,
                CoreWebView2PermissionRequestedEventArgs,
            >,
        >,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PermissionRequested)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemovePermissionRequested(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).RemovePermissionRequested)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn ProcessFailed<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<
                CoreWebView2,
                CoreWebView2ProcessFailedEventArgs,
            >,
        >,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ProcessFailed)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveProcessFailed(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveProcessFailed)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn WebMessageReceived<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<
                CoreWebView2,
                CoreWebView2WebMessageReceivedEventArgs,
            >,
        >,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).WebMessageReceived)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveWebMessageReceived(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveWebMessageReceived)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn NewWindowRequested<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<
                CoreWebView2,
                CoreWebView2NewWindowRequestedEventArgs,
            >,
        >,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NewWindowRequested)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveNewWindowRequested(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveNewWindowRequested)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn DocumentTitleChanged<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<CoreWebView2, ::windows_core::IInspectable>,
        >,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DocumentTitleChanged)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveDocumentTitleChanged(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveDocumentTitleChanged)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn ContainsFullScreenElementChanged<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<CoreWebView2, ::windows_core::IInspectable>,
        >,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ContainsFullScreenElementChanged)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveContainsFullScreenElementChanged(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveContainsFullScreenElementChanged)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn WebResourceRequested<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<
                CoreWebView2,
                CoreWebView2WebResourceRequestedEventArgs,
            >,
        >,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).WebResourceRequested)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveWebResourceRequested(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveWebResourceRequested)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn WindowCloseRequested<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<CoreWebView2, ::windows_core::IInspectable>,
        >,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).WindowCloseRequested)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveWindowCloseRequested(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveWindowCloseRequested)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn Navigate(&self, uri: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).Navigate)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(uri),
            )
            .ok()
        }
    }
    pub fn NavigateToString(
        &self,
        htmlcontent: &::windows_core::HSTRING,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).NavigateToString)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(htmlcontent),
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn AddScriptToExecuteOnDocumentCreatedAsync(
        &self,
        javascript: &::windows_core::HSTRING,
    ) -> ::windows_core::Result<::windows::Foundation::IAsyncOperation<::windows_core::HSTRING>>
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AddScriptToExecuteOnDocumentCreatedAsync)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(javascript),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn RemoveScriptToExecuteOnDocumentCreated(
        &self,
        id: &::windows_core::HSTRING,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveScriptToExecuteOnDocumentCreated)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(id),
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn ExecuteScriptAsync(
        &self,
        javascript: &::windows_core::HSTRING,
    ) -> ::windows_core::Result<::windows::Foundation::IAsyncOperation<::windows_core::HSTRING>>
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ExecuteScriptAsync)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(javascript),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`, `\"Windows_Storage_Streams\"`"]
    #[cfg(all(feature = "Windows_Foundation", feature = "Windows_Storage_Streams"))]
    pub fn CapturePreviewAsync<P0>(
        &self,
        imageformat: CoreWebView2CapturePreviewImageFormat,
        imagestream: P0,
    ) -> ::windows_core::Result<::windows::Foundation::IAsyncAction>
    where
        P0: ::windows_core::TryIntoParam<::windows::Storage::Streams::IRandomAccessStream>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CapturePreviewAsync)(
                ::windows_core::Interface::as_raw(this),
                imageformat,
                imagestream.try_into_param()?.abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn Reload(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).Reload)(::windows_core::Interface::as_raw(
                this,
            ))
            .ok()
        }
    }
    pub fn PostWebMessageAsJson(
        &self,
        webmessageasjson: &::windows_core::HSTRING,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).PostWebMessageAsJson)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(webmessageasjson),
            )
            .ok()
        }
    }
    pub fn PostWebMessageAsString(
        &self,
        webmessageasstring: &::windows_core::HSTRING,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).PostWebMessageAsString)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(webmessageasstring),
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn CallDevToolsProtocolMethodAsync(
        &self,
        methodname: &::windows_core::HSTRING,
        parametersasjson: &::windows_core::HSTRING,
    ) -> ::windows_core::Result<::windows::Foundation::IAsyncOperation<::windows_core::HSTRING>>
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CallDevToolsProtocolMethodAsync)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(methodname),
                ::core::mem::transmute_copy(parametersasjson),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn GoBack(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).GoBack)(::windows_core::Interface::as_raw(
                this,
            ))
            .ok()
        }
    }
    pub fn GoForward(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).GoForward)(::windows_core::Interface::as_raw(
                this,
            ))
            .ok()
        }
    }
    pub fn GetDevToolsProtocolEventReceiver(
        &self,
        eventname: &::windows_core::HSTRING,
    ) -> ::windows_core::Result<CoreWebView2DevToolsProtocolEventReceiver> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetDevToolsProtocolEventReceiver)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(eventname),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn Stop(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).Stop)(::windows_core::Interface::as_raw(this))
                .ok()
        }
    }
    pub fn AddHostObjectToScript<P0>(
        &self,
        name: &::windows_core::HSTRING,
        rawobject: P0,
    ) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).AddHostObjectToScript)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(name),
                rawobject.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn RemoveHostObjectFromScript(
        &self,
        name: &::windows_core::HSTRING,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveHostObjectFromScript)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(name),
            )
            .ok()
        }
    }
    pub fn OpenDevToolsWindow(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).OpenDevToolsWindow)(
                ::windows_core::Interface::as_raw(this),
            )
            .ok()
        }
    }
    pub fn AddWebResourceRequestedFilter(
        &self,
        uri: &::windows_core::HSTRING,
        resourcecontext: CoreWebView2WebResourceContext,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).AddWebResourceRequestedFilter)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(uri),
                resourcecontext,
            )
            .ok()
        }
    }
    pub fn RemoveWebResourceRequestedFilter(
        &self,
        uri: &::windows_core::HSTRING,
        resourcecontext: CoreWebView2WebResourceContext,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveWebResourceRequestedFilter)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(uri),
                resourcecontext,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn BasicAuthenticationRequested<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<
                CoreWebView2,
                CoreWebView2BasicAuthenticationRequestedEventArgs,
            >,
        >,
    {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2_10>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).BasicAuthenticationRequested)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveBasicAuthenticationRequested(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2_10>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveBasicAuthenticationRequested)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn ContextMenuRequested<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<
                CoreWebView2,
                CoreWebView2ContextMenuRequestedEventArgs,
            >,
        >,
    {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2_11>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ContextMenuRequested)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveContextMenuRequested(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2_11>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveContextMenuRequested)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn CallDevToolsProtocolMethodForSessionAsync(
        &self,
        sessionid: &::windows_core::HSTRING,
        methodname: &::windows_core::HSTRING,
        parametersasjson: &::windows_core::HSTRING,
    ) -> ::windows_core::Result<::windows::Foundation::IAsyncOperation<::windows_core::HSTRING>>
    {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2_11>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CallDevToolsProtocolMethodForSessionAsync)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(sessionid),
                ::core::mem::transmute_copy(methodname),
                ::core::mem::transmute_copy(parametersasjson),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn StatusBarText(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2_12>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StatusBarText)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn StatusBarTextChanged<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<CoreWebView2, ::windows_core::IInspectable>,
        >,
    {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2_12>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StatusBarTextChanged)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveStatusBarTextChanged(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2_12>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveStatusBarTextChanged)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn Profile(&self) -> ::windows_core::Result<CoreWebView2Profile> {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2_13>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Profile)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn ServerCertificateErrorDetected<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<
                CoreWebView2,
                CoreWebView2ServerCertificateErrorDetectedEventArgs,
            >,
        >,
    {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2_14>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ServerCertificateErrorDetected)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveServerCertificateErrorDetected(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2_14>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveServerCertificateErrorDetected)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn ClearServerCertificateErrorActionsAsync(
        &self,
    ) -> ::windows_core::Result<::windows::Foundation::IAsyncAction> {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2_14>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ClearServerCertificateErrorActionsAsync)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn FaviconUri(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2_15>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FaviconUri)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn FaviconChanged<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<CoreWebView2, ::windows_core::IInspectable>,
        >,
    {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2_15>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FaviconChanged)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveFaviconChanged(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2_15>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveFaviconChanged)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`, `\"Windows_Storage_Streams\"`"]
    #[cfg(all(feature = "Windows_Foundation", feature = "Windows_Storage_Streams"))]
    pub fn GetFaviconAsync(
        &self,
        format: CoreWebView2FaviconImageFormat,
    ) -> ::windows_core::Result<
        ::windows::Foundation::IAsyncOperation<::windows::Storage::Streams::IRandomAccessStream>,
    > {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2_15>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetFaviconAsync)(
                ::windows_core::Interface::as_raw(this),
                format,
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn PrintAsync<P0>(
        &self,
        printsettings: P0,
    ) -> ::windows_core::Result<::windows::Foundation::IAsyncOperation<CoreWebView2PrintStatus>>
    where
        P0: ::windows_core::IntoParam<CoreWebView2PrintSettings>,
    {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2_16>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PrintAsync)(
                ::windows_core::Interface::as_raw(this),
                printsettings.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn ShowPrintUI(
        &self,
        printdialogkind: CoreWebView2PrintDialogKind,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2_16>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).ShowPrintUI)(
                ::windows_core::Interface::as_raw(this),
                printdialogkind,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`, `\"Windows_Storage_Streams\"`"]
    #[cfg(all(feature = "Windows_Foundation", feature = "Windows_Storage_Streams"))]
    pub fn PrintToPdfStreamAsync<P0>(
        &self,
        printsettings: P0,
    ) -> ::windows_core::Result<
        ::windows::Foundation::IAsyncOperation<::windows::Storage::Streams::IRandomAccessStream>,
    >
    where
        P0: ::windows_core::IntoParam<CoreWebView2PrintSettings>,
    {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2_16>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PrintToPdfStreamAsync)(
                ::windows_core::Interface::as_raw(this),
                printsettings.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn PostSharedBufferToScript<P0>(
        &self,
        sharedbuffer: P0,
        access: CoreWebView2SharedBufferAccess,
        additionaldataasjson: &::windows_core::HSTRING,
    ) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<CoreWebView2SharedBuffer>,
    {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2_17>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).PostSharedBufferToScript)(
                ::windows_core::Interface::as_raw(this),
                sharedbuffer.into_param().abi(),
                access,
                ::core::mem::transmute_copy(additionaldataasjson),
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn LaunchingExternalUriScheme<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<
                CoreWebView2,
                CoreWebView2LaunchingExternalUriSchemeEventArgs,
            >,
        >,
    {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2_18>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LaunchingExternalUriScheme)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveLaunchingExternalUriScheme(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2_18>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveLaunchingExternalUriScheme)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn MemoryUsageTargetLevel(
        &self,
    ) -> ::windows_core::Result<CoreWebView2MemoryUsageTargetLevel> {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2_19>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MemoryUsageTargetLevel)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetMemoryUsageTargetLevel(
        &self,
        value: CoreWebView2MemoryUsageTargetLevel,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2_19>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetMemoryUsageTargetLevel)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn CookieManager(&self) -> ::windows_core::Result<CoreWebView2CookieManager> {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2_2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CookieManager)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn Environment(&self) -> ::windows_core::Result<CoreWebView2Environment> {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2_2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Environment)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn WebResourceResponseReceived<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<
                CoreWebView2,
                CoreWebView2WebResourceResponseReceivedEventArgs,
            >,
        >,
    {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2_2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).WebResourceResponseReceived)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveWebResourceResponseReceived(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2_2>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveWebResourceResponseReceived)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn DOMContentLoaded<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<
                CoreWebView2,
                CoreWebView2DOMContentLoadedEventArgs,
            >,
        >,
    {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2_2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DOMContentLoaded)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveDOMContentLoaded(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2_2>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveDOMContentLoaded)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn NavigateWithWebResourceRequest<P0>(&self, request: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<CoreWebView2WebResourceRequest>,
    {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2_2>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).NavigateWithWebResourceRequest)(
                ::windows_core::Interface::as_raw(this),
                request.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn IsSuspended(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2_3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsSuspended)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn TrySuspendAsync(
        &self,
    ) -> ::windows_core::Result<::windows::Foundation::IAsyncOperation<bool>> {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2_3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TrySuspendAsync)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn Resume(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2_3>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).Resume)(::windows_core::Interface::as_raw(
                this,
            ))
            .ok()
        }
    }
    pub fn SetVirtualHostNameToFolderMapping(
        &self,
        hostname: &::windows_core::HSTRING,
        folderpath: &::windows_core::HSTRING,
        accesskind: CoreWebView2HostResourceAccessKind,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2_3>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetVirtualHostNameToFolderMapping)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(hostname),
                ::core::mem::transmute_copy(folderpath),
                accesskind,
            )
            .ok()
        }
    }
    pub fn ClearVirtualHostNameToFolderMapping(
        &self,
        hostname: &::windows_core::HSTRING,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2_3>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).ClearVirtualHostNameToFolderMapping)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(hostname),
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn FrameCreated<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<
                CoreWebView2,
                CoreWebView2FrameCreatedEventArgs,
            >,
        >,
    {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2_4>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FrameCreated)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveFrameCreated(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2_4>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveFrameCreated)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn DownloadStarting<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<
                CoreWebView2,
                CoreWebView2DownloadStartingEventArgs,
            >,
        >,
    {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2_4>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DownloadStarting)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveDownloadStarting(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2_4>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveDownloadStarting)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn ClientCertificateRequested<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<
                CoreWebView2,
                CoreWebView2ClientCertificateRequestedEventArgs,
            >,
        >,
    {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2_5>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ClientCertificateRequested)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveClientCertificateRequested(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2_5>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveClientCertificateRequested)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn OpenTaskManagerWindow(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2_6>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).OpenTaskManagerWindow)(
                ::windows_core::Interface::as_raw(this),
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn PrintToPdfAsync<P0>(
        &self,
        resultfilepath: &::windows_core::HSTRING,
        printsettings: P0,
    ) -> ::windows_core::Result<::windows::Foundation::IAsyncOperation<bool>>
    where
        P0: ::windows_core::IntoParam<CoreWebView2PrintSettings>,
    {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2_7>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PrintToPdfAsync)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(resultfilepath),
                printsettings.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn IsMuted(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2_8>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsMuted)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetIsMuted(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2_8>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetIsMuted)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsDocumentPlayingAudio(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2_8>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsDocumentPlayingAudio)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn IsMutedChanged<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<CoreWebView2, ::windows_core::IInspectable>,
        >,
    {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2_8>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsMutedChanged)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveIsMutedChanged(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2_8>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveIsMutedChanged)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn IsDocumentPlayingAudioChanged<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<CoreWebView2, ::windows_core::IInspectable>,
        >,
    {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2_8>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsDocumentPlayingAudioChanged)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveIsDocumentPlayingAudioChanged(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2_8>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveIsDocumentPlayingAudioChanged)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn IsDefaultDownloadDialogOpen(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2_9>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsDefaultDownloadDialogOpen)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn DefaultDownloadDialogCornerAlignment(
        &self,
    ) -> ::windows_core::Result<CoreWebView2DefaultDownloadDialogCornerAlignment> {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2_9>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DefaultDownloadDialogCornerAlignment)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetDefaultDownloadDialogCornerAlignment(
        &self,
        value: CoreWebView2DefaultDownloadDialogCornerAlignment,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2_9>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetDefaultDownloadDialogCornerAlignment)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn DefaultDownloadDialogMargin(
        &self,
    ) -> ::windows_core::Result<::windows::Foundation::Point> {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2_9>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DefaultDownloadDialogMargin)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn SetDefaultDownloadDialogMargin(
        &self,
        value: ::windows::Foundation::Point,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2_9>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetDefaultDownloadDialogMargin)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn IsDefaultDownloadDialogOpenChanged<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<CoreWebView2, ::windows_core::IInspectable>,
        >,
    {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2_9>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsDefaultDownloadDialogOpenChanged)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveIsDefaultDownloadDialogOpenChanged(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2_9>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveIsDefaultDownloadDialogOpenChanged)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn OpenDefaultDownloadDialog(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2_9>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).OpenDefaultDownloadDialog)(
                ::windows_core::Interface::as_raw(this),
            )
            .ok()
        }
    }
    pub fn CloseDefaultDownloadDialog(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2_9>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).CloseDefaultDownloadDialog)(
                ::windows_core::Interface::as_raw(this),
            )
            .ok()
        }
    }
}
impl ::windows_core::RuntimeType for CoreWebView2 {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for CoreWebView2 {
    type Vtable = ICoreWebView2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for CoreWebView2 {
    const IID: ::windows_core::GUID = <ICoreWebView2 as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for CoreWebView2 {
    const NAME: &'static str = "Microsoft.Web.WebView2.Core.CoreWebView2";
}
::windows_core::imp::interface_hierarchy!(
    CoreWebView2,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for CoreWebView2 {}
unsafe impl ::core::marker::Sync for CoreWebView2 {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct CoreWebView2AcceleratorKeyPressedEventArgs(::windows_core::IUnknown);
impl CoreWebView2AcceleratorKeyPressedEventArgs {
    pub fn KeyEventKind(&self) -> ::windows_core::Result<CoreWebView2KeyEventKind> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).KeyEventKind)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn VirtualKey(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).VirtualKey)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn KeyEventLParam(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).KeyEventLParam)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn PhysicalKeyStatus(&self) -> ::windows_core::Result<CoreWebView2PhysicalKeyStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PhysicalKeyStatus)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn Handled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Handled)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetHandled)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
}
impl ::windows_core::RuntimeType for CoreWebView2AcceleratorKeyPressedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for CoreWebView2AcceleratorKeyPressedEventArgs {
    type Vtable = ICoreWebView2AcceleratorKeyPressedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for CoreWebView2AcceleratorKeyPressedEventArgs {
    const IID: ::windows_core::GUID =
        <ICoreWebView2AcceleratorKeyPressedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for CoreWebView2AcceleratorKeyPressedEventArgs {
    const NAME: &'static str =
        "Microsoft.Web.WebView2.Core.CoreWebView2AcceleratorKeyPressedEventArgs";
}
::windows_core::imp::interface_hierarchy!(
    CoreWebView2AcceleratorKeyPressedEventArgs,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for CoreWebView2AcceleratorKeyPressedEventArgs {}
unsafe impl ::core::marker::Sync for CoreWebView2AcceleratorKeyPressedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct CoreWebView2BasicAuthenticationRequestedEventArgs(::windows_core::IUnknown);
impl CoreWebView2BasicAuthenticationRequestedEventArgs {
    pub fn Uri(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Uri)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn Challenge(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Challenge)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn Response(&self) -> ::windows_core::Result<CoreWebView2BasicAuthenticationResponse> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Response)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
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
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn GetDeferral(&self) -> ::windows_core::Result<::windows::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetDeferral)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for CoreWebView2BasicAuthenticationRequestedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for CoreWebView2BasicAuthenticationRequestedEventArgs {
    type Vtable = ICoreWebView2BasicAuthenticationRequestedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for CoreWebView2BasicAuthenticationRequestedEventArgs {
    const IID: ::windows_core::GUID =
        <ICoreWebView2BasicAuthenticationRequestedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for CoreWebView2BasicAuthenticationRequestedEventArgs {
    const NAME: &'static str =
        "Microsoft.Web.WebView2.Core.CoreWebView2BasicAuthenticationRequestedEventArgs";
}
::windows_core::imp::interface_hierarchy!(
    CoreWebView2BasicAuthenticationRequestedEventArgs,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for CoreWebView2BasicAuthenticationRequestedEventArgs {}
unsafe impl ::core::marker::Sync for CoreWebView2BasicAuthenticationRequestedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct CoreWebView2BasicAuthenticationResponse(::windows_core::IUnknown);
impl CoreWebView2BasicAuthenticationResponse {
    pub fn UserName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).UserName)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetUserName(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetUserName)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn Password(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Password)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetPassword(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetPassword)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
}
impl ::windows_core::RuntimeType for CoreWebView2BasicAuthenticationResponse {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for CoreWebView2BasicAuthenticationResponse {
    type Vtable = ICoreWebView2BasicAuthenticationResponse_Vtbl;
}
unsafe impl ::windows_core::ComInterface for CoreWebView2BasicAuthenticationResponse {
    const IID: ::windows_core::GUID =
        <ICoreWebView2BasicAuthenticationResponse as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for CoreWebView2BasicAuthenticationResponse {
    const NAME: &'static str =
        "Microsoft.Web.WebView2.Core.CoreWebView2BasicAuthenticationResponse";
}
::windows_core::imp::interface_hierarchy!(
    CoreWebView2BasicAuthenticationResponse,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for CoreWebView2BasicAuthenticationResponse {}
unsafe impl ::core::marker::Sync for CoreWebView2BasicAuthenticationResponse {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct CoreWebView2BrowserProcessExitedEventArgs(::windows_core::IUnknown);
impl CoreWebView2BrowserProcessExitedEventArgs {
    pub fn BrowserProcessExitKind(
        &self,
    ) -> ::windows_core::Result<CoreWebView2BrowserProcessExitKind> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).BrowserProcessExitKind)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn BrowserProcessId(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).BrowserProcessId)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for CoreWebView2BrowserProcessExitedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for CoreWebView2BrowserProcessExitedEventArgs {
    type Vtable = ICoreWebView2BrowserProcessExitedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for CoreWebView2BrowserProcessExitedEventArgs {
    const IID: ::windows_core::GUID =
        <ICoreWebView2BrowserProcessExitedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for CoreWebView2BrowserProcessExitedEventArgs {
    const NAME: &'static str =
        "Microsoft.Web.WebView2.Core.CoreWebView2BrowserProcessExitedEventArgs";
}
::windows_core::imp::interface_hierarchy!(
    CoreWebView2BrowserProcessExitedEventArgs,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for CoreWebView2BrowserProcessExitedEventArgs {}
unsafe impl ::core::marker::Sync for CoreWebView2BrowserProcessExitedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct CoreWebView2Certificate(::windows_core::IUnknown);
impl CoreWebView2Certificate {
    #[doc = "Required features: `\"Windows_Security_Cryptography_Certificates\"`"]
    #[cfg(feature = "Windows_Security_Cryptography_Certificates")]
    pub fn ToCertificate(
        &self,
    ) -> ::windows_core::Result<::windows::Security::Cryptography::Certificates::Certificate> {
        let this = &::windows_core::ComInterface::cast::<CoreWebView2Certificate_Manual>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ToCertificate)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn Subject(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Subject)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn Issuer(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Issuer)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn ValidFrom(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ValidFrom)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn ValidTo(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ValidTo)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn DerEncodedSerialNumber(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DerEncodedSerialNumber)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn DisplayName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DisplayName)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation_Collections\"`"]
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub fn PemEncodedIssuerCertificateChain(
        &self,
    ) -> ::windows_core::Result<
        ::windows::Foundation::Collections::IVectorView<::windows_core::HSTRING>,
    > {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PemEncodedIssuerCertificateChain)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn ToPemEncoding(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ToPemEncoding)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for CoreWebView2Certificate {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for CoreWebView2Certificate {
    type Vtable = ICoreWebView2Certificate_Vtbl;
}
unsafe impl ::windows_core::ComInterface for CoreWebView2Certificate {
    const IID: ::windows_core::GUID =
        <ICoreWebView2Certificate as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for CoreWebView2Certificate {
    const NAME: &'static str = "Microsoft.Web.WebView2.Core.CoreWebView2Certificate";
}
::windows_core::imp::interface_hierarchy!(
    CoreWebView2Certificate,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for CoreWebView2Certificate {}
unsafe impl ::core::marker::Sync for CoreWebView2Certificate {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct CoreWebView2ClientCertificate(::windows_core::IUnknown);
impl CoreWebView2ClientCertificate {
    #[doc = "Required features: `\"Windows_Security_Cryptography_Certificates\"`"]
    #[cfg(feature = "Windows_Security_Cryptography_Certificates")]
    pub fn ToCertificate(
        &self,
    ) -> ::windows_core::Result<::windows::Security::Cryptography::Certificates::Certificate> {
        let this =
            &::windows_core::ComInterface::cast::<CoreWebView2ClientCertificate_Manual>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ToCertificate)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn Subject(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Subject)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn Issuer(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Issuer)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn ValidFrom(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ValidFrom)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn ValidTo(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ValidTo)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn DerEncodedSerialNumber(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DerEncodedSerialNumber)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn DisplayName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DisplayName)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation_Collections\"`"]
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub fn PemEncodedIssuerCertificateChain(
        &self,
    ) -> ::windows_core::Result<
        ::windows::Foundation::Collections::IVectorView<::windows_core::HSTRING>,
    > {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PemEncodedIssuerCertificateChain)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn Kind(&self) -> ::windows_core::Result<CoreWebView2ClientCertificateKind> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn ToPemEncoding(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ToPemEncoding)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for CoreWebView2ClientCertificate {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for CoreWebView2ClientCertificate {
    type Vtable = ICoreWebView2ClientCertificate_Vtbl;
}
unsafe impl ::windows_core::ComInterface for CoreWebView2ClientCertificate {
    const IID: ::windows_core::GUID =
        <ICoreWebView2ClientCertificate as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for CoreWebView2ClientCertificate {
    const NAME: &'static str = "Microsoft.Web.WebView2.Core.CoreWebView2ClientCertificate";
}
::windows_core::imp::interface_hierarchy!(
    CoreWebView2ClientCertificate,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for CoreWebView2ClientCertificate {}
unsafe impl ::core::marker::Sync for CoreWebView2ClientCertificate {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct CoreWebView2ClientCertificateRequestedEventArgs(::windows_core::IUnknown);
impl CoreWebView2ClientCertificateRequestedEventArgs {
    pub fn Host(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Host)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn Port(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Port)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn IsProxy(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsProxy)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation_Collections\"`"]
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub fn AllowedCertificateAuthorities(
        &self,
    ) -> ::windows_core::Result<
        ::windows::Foundation::Collections::IVectorView<::windows_core::HSTRING>,
    > {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AllowedCertificateAuthorities)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation_Collections\"`"]
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub fn MutuallyTrustedCertificates(
        &self,
    ) -> ::windows_core::Result<
        ::windows::Foundation::Collections::IVectorView<CoreWebView2ClientCertificate>,
    > {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MutuallyTrustedCertificates)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SelectedCertificate(&self) -> ::windows_core::Result<CoreWebView2ClientCertificate> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SelectedCertificate)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetSelectedCertificate<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<CoreWebView2ClientCertificate>,
    {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetSelectedCertificate)(
                ::windows_core::Interface::as_raw(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
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
    pub fn Handled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Handled)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetHandled)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn GetDeferral(&self) -> ::windows_core::Result<::windows::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetDeferral)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for CoreWebView2ClientCertificateRequestedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for CoreWebView2ClientCertificateRequestedEventArgs {
    type Vtable = ICoreWebView2ClientCertificateRequestedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for CoreWebView2ClientCertificateRequestedEventArgs {
    const IID: ::windows_core::GUID =
        <ICoreWebView2ClientCertificateRequestedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for CoreWebView2ClientCertificateRequestedEventArgs {
    const NAME: &'static str =
        "Microsoft.Web.WebView2.Core.CoreWebView2ClientCertificateRequestedEventArgs";
}
::windows_core::imp::interface_hierarchy!(
    CoreWebView2ClientCertificateRequestedEventArgs,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for CoreWebView2ClientCertificateRequestedEventArgs {}
unsafe impl ::core::marker::Sync for CoreWebView2ClientCertificateRequestedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct CoreWebView2CompositionController(::windows_core::IUnknown);
impl CoreWebView2CompositionController {
    pub fn RootVisualTarget(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RootVisualTarget)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetRootVisualTarget<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetRootVisualTarget)(
                ::windows_core::Interface::as_raw(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn CursorChanged<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<
                CoreWebView2CompositionController,
                ::windows_core::IInspectable,
            >,
        >,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CursorChanged)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveCursorChanged(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveCursorChanged)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn SendMouseInput(
        &self,
        eventkind: CoreWebView2MouseEventKind,
        virtualkeys: CoreWebView2MouseEventVirtualKeys,
        mousedata: u32,
        point: ::windows::Foundation::Point,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SendMouseInput)(
                ::windows_core::Interface::as_raw(this),
                eventkind,
                virtualkeys,
                mousedata,
                point,
            )
            .ok()
        }
    }
    pub fn SendPointerInput<P0>(
        &self,
        eventkind: CoreWebView2PointerEventKind,
        pointerinfo: P0,
    ) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<CoreWebView2PointerInfo>,
    {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SendPointerInput)(
                ::windows_core::Interface::as_raw(this),
                eventkind,
                pointerinfo.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_UI_Core\"`"]
    #[cfg(feature = "Windows_UI_Core")]
    pub fn Cursor(&self) -> ::windows_core::Result<::windows::UI::Core::CoreCursor> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Cursor)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_ApplicationModel_DataTransfer_DragDrop_Core\"`"]
    #[cfg(feature = "Windows_ApplicationModel_DataTransfer_DragDrop_Core")]
    pub fn DragEnter<P0, P1>(
        &self,
        draginfo: P0,
        draguioverride: P1,
    ) -> ::windows_core::Result<::windows::ApplicationModel::DataTransfer::DataPackageOperation>
    where
        P0: ::windows_core::IntoParam<
            ::windows::ApplicationModel::DataTransfer::DragDrop::Core::CoreDragInfo,
        >,
        P1: ::windows_core::IntoParam<
            ::windows::ApplicationModel::DataTransfer::DragDrop::Core::CoreDragUIOverride,
        >,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DragEnter)(
                ::windows_core::Interface::as_raw(this),
                draginfo.into_param().abi(),
                draguioverride.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_ApplicationModel_DataTransfer_DragDrop_Core\"`"]
    #[cfg(feature = "Windows_ApplicationModel_DataTransfer_DragDrop_Core")]
    pub fn DragOver<P0, P1>(
        &self,
        draginfo: P0,
        draguioverride: P1,
    ) -> ::windows_core::Result<::windows::ApplicationModel::DataTransfer::DataPackageOperation>
    where
        P0: ::windows_core::IntoParam<
            ::windows::ApplicationModel::DataTransfer::DragDrop::Core::CoreDragInfo,
        >,
        P1: ::windows_core::IntoParam<
            ::windows::ApplicationModel::DataTransfer::DragDrop::Core::CoreDragUIOverride,
        >,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DragOver)(
                ::windows_core::Interface::as_raw(this),
                draginfo.into_param().abi(),
                draguioverride.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_ApplicationModel_DataTransfer_DragDrop_Core\"`"]
    #[cfg(feature = "Windows_ApplicationModel_DataTransfer_DragDrop_Core")]
    pub fn Drop<P0>(
        &self,
        draginfo: P0,
    ) -> ::windows_core::Result<::windows::ApplicationModel::DataTransfer::DataPackageOperation>
    where
        P0: ::windows_core::IntoParam<
            ::windows::ApplicationModel::DataTransfer::DragDrop::Core::CoreDragInfo,
        >,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Drop)(
                ::windows_core::Interface::as_raw(this),
                draginfo.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn DragLeave(&self) -> ::windows_core::Result<()> {
        let this =
            &::windows_core::ComInterface::cast::<ICoreWebView2CompositionController3>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).DragLeave)(::windows_core::Interface::as_raw(
                this,
            ))
            .ok()
        }
    }
    pub fn IsVisible(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2Controller>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsVisible)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetIsVisible(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2Controller>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetIsVisible)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn Bounds(&self) -> ::windows_core::Result<::windows::Foundation::Rect> {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2Controller>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Bounds)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn SetBounds(&self, value: ::windows::Foundation::Rect) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2Controller>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetBounds)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn ZoomFactor(&self) -> ::windows_core::Result<f64> {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2Controller>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ZoomFactor)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetZoomFactor(&self, value: f64) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2Controller>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetZoomFactor)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn ParentWindow(&self) -> ::windows_core::Result<CoreWebView2ControllerWindowReference> {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2Controller>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ParentWindow)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetParentWindow<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<CoreWebView2ControllerWindowReference>,
    {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2Controller>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetParentWindow)(
                ::windows_core::Interface::as_raw(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn CoreWebView2(&self) -> ::windows_core::Result<CoreWebView2> {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2Controller>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CoreWebView2)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn ZoomFactorChanged<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<
                CoreWebView2Controller,
                ::windows_core::IInspectable,
            >,
        >,
    {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2Controller>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ZoomFactorChanged)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveZoomFactorChanged(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2Controller>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveZoomFactorChanged)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn MoveFocusRequested<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<
                CoreWebView2Controller,
                CoreWebView2MoveFocusRequestedEventArgs,
            >,
        >,
    {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2Controller>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MoveFocusRequested)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveMoveFocusRequested(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2Controller>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveMoveFocusRequested)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn GotFocus<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<
                CoreWebView2Controller,
                ::windows_core::IInspectable,
            >,
        >,
    {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2Controller>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GotFocus)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveGotFocus(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2Controller>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveGotFocus)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn LostFocus<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<
                CoreWebView2Controller,
                ::windows_core::IInspectable,
            >,
        >,
    {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2Controller>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LostFocus)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveLostFocus(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2Controller>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveLostFocus)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn AcceleratorKeyPressed<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<
                CoreWebView2Controller,
                CoreWebView2AcceleratorKeyPressedEventArgs,
            >,
        >,
    {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2Controller>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AcceleratorKeyPressed)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveAcceleratorKeyPressed(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2Controller>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveAcceleratorKeyPressed)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn SetBoundsAndZoomFactor(
        &self,
        bounds: ::windows::Foundation::Rect,
        zoomfactor: f64,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2Controller>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetBoundsAndZoomFactor)(
                ::windows_core::Interface::as_raw(this),
                bounds,
                zoomfactor,
            )
            .ok()
        }
    }
    pub fn MoveFocus(&self, reason: CoreWebView2MoveFocusReason) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2Controller>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).MoveFocus)(
                ::windows_core::Interface::as_raw(this),
                reason,
            )
            .ok()
        }
    }
    pub fn NotifyParentWindowPositionChanged(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2Controller>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).NotifyParentWindowPositionChanged)(
                ::windows_core::Interface::as_raw(this),
            )
            .ok()
        }
    }
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2Controller>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this))
                .ok()
        }
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn DefaultBackgroundColor(&self) -> ::windows_core::Result<::windows::UI::Color> {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2Controller2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DefaultBackgroundColor)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn SetDefaultBackgroundColor(
        &self,
        value: ::windows::UI::Color,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2Controller2>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetDefaultBackgroundColor)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn RasterizationScale(&self) -> ::windows_core::Result<f64> {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2Controller3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RasterizationScale)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetRasterizationScale(&self, value: f64) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2Controller3>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetRasterizationScale)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn ShouldDetectMonitorScaleChanges(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2Controller3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ShouldDetectMonitorScaleChanges)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetShouldDetectMonitorScaleChanges(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2Controller3>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetShouldDetectMonitorScaleChanges)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn BoundsMode(&self) -> ::windows_core::Result<CoreWebView2BoundsMode> {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2Controller3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).BoundsMode)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetBoundsMode(&self, value: CoreWebView2BoundsMode) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2Controller3>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetBoundsMode)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RasterizationScaleChanged<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<
                CoreWebView2Controller,
                ::windows_core::IInspectable,
            >,
        >,
    {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2Controller3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RasterizationScaleChanged)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveRasterizationScaleChanged(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2Controller3>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveRasterizationScaleChanged)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn AllowExternalDrop(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2Controller4>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AllowExternalDrop)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetAllowExternalDrop(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2Controller4>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetAllowExternalDrop)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsBrowserHitTransparent(&self) -> ::windows_core::Result<bool> {
        let this =
            &::windows_core::ComInterface::cast::<ICoreWebView2PrivatePartialController>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsBrowserHitTransparent)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for CoreWebView2CompositionController {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for CoreWebView2CompositionController {
    type Vtable = ICoreWebView2CompositionController_Vtbl;
}
unsafe impl ::windows_core::ComInterface for CoreWebView2CompositionController {
    const IID: ::windows_core::GUID =
        <ICoreWebView2CompositionController as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for CoreWebView2CompositionController {
    const NAME: &'static str = "Microsoft.Web.WebView2.Core.CoreWebView2CompositionController";
}
::windows_core::imp::interface_hierarchy!(
    CoreWebView2CompositionController,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<CoreWebView2Controller> for CoreWebView2CompositionController {}
unsafe impl ::core::marker::Send for CoreWebView2CompositionController {}
unsafe impl ::core::marker::Sync for CoreWebView2CompositionController {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct CoreWebView2ContentLoadingEventArgs(::windows_core::IUnknown);
impl CoreWebView2ContentLoadingEventArgs {
    pub fn IsErrorPage(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsErrorPage)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn NavigationId(&self) -> ::windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NavigationId)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for CoreWebView2ContentLoadingEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for CoreWebView2ContentLoadingEventArgs {
    type Vtable = ICoreWebView2ContentLoadingEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for CoreWebView2ContentLoadingEventArgs {
    const IID: ::windows_core::GUID =
        <ICoreWebView2ContentLoadingEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for CoreWebView2ContentLoadingEventArgs {
    const NAME: &'static str = "Microsoft.Web.WebView2.Core.CoreWebView2ContentLoadingEventArgs";
}
::windows_core::imp::interface_hierarchy!(
    CoreWebView2ContentLoadingEventArgs,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for CoreWebView2ContentLoadingEventArgs {}
unsafe impl ::core::marker::Sync for CoreWebView2ContentLoadingEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct CoreWebView2ContextMenuItem(::windows_core::IUnknown);
impl CoreWebView2ContextMenuItem {
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
    pub fn Label(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Label)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn CommandId(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CommandId)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn ShortcutKeyDescription(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ShortcutKeyDescription)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Storage_Streams\"`"]
    #[cfg(feature = "Windows_Storage_Streams")]
    pub fn Icon(&self) -> ::windows_core::Result<::windows::Storage::Streams::IRandomAccessStream> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Icon)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn Kind(&self) -> ::windows_core::Result<CoreWebView2ContextMenuItemKind> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn IsEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsEnabled)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetIsEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetIsEnabled)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsChecked(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsChecked)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetIsChecked(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetIsChecked)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation_Collections\"`"]
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub fn Children(
        &self,
    ) -> ::windows_core::Result<
        ::windows::Foundation::Collections::IVector<CoreWebView2ContextMenuItem>,
    > {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Children)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn CustomItemSelected<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<
                CoreWebView2ContextMenuItem,
                ::windows_core::IInspectable,
            >,
        >,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CustomItemSelected)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveCustomItemSelected(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveCustomItemSelected)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
}
impl ::windows_core::RuntimeType for CoreWebView2ContextMenuItem {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for CoreWebView2ContextMenuItem {
    type Vtable = ICoreWebView2ContextMenuItem_Vtbl;
}
unsafe impl ::windows_core::ComInterface for CoreWebView2ContextMenuItem {
    const IID: ::windows_core::GUID =
        <ICoreWebView2ContextMenuItem as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for CoreWebView2ContextMenuItem {
    const NAME: &'static str = "Microsoft.Web.WebView2.Core.CoreWebView2ContextMenuItem";
}
::windows_core::imp::interface_hierarchy!(
    CoreWebView2ContextMenuItem,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for CoreWebView2ContextMenuItem {}
unsafe impl ::core::marker::Sync for CoreWebView2ContextMenuItem {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct CoreWebView2ContextMenuRequestedEventArgs(::windows_core::IUnknown);
impl CoreWebView2ContextMenuRequestedEventArgs {
    #[doc = "Required features: `\"Windows_Foundation_Collections\"`"]
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub fn MenuItems(
        &self,
    ) -> ::windows_core::Result<
        ::windows::Foundation::Collections::IVector<CoreWebView2ContextMenuItem>,
    > {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MenuItems)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn ContextMenuTarget(&self) -> ::windows_core::Result<CoreWebView2ContextMenuTarget> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ContextMenuTarget)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn Location(&self) -> ::windows_core::Result<::windows::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Location)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SelectedCommandId(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SelectedCommandId)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetSelectedCommandId(&self, value: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetSelectedCommandId)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn Handled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Handled)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetHandled)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn GetDeferral(&self) -> ::windows_core::Result<::windows::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetDeferral)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for CoreWebView2ContextMenuRequestedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for CoreWebView2ContextMenuRequestedEventArgs {
    type Vtable = ICoreWebView2ContextMenuRequestedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for CoreWebView2ContextMenuRequestedEventArgs {
    const IID: ::windows_core::GUID =
        <ICoreWebView2ContextMenuRequestedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for CoreWebView2ContextMenuRequestedEventArgs {
    const NAME: &'static str =
        "Microsoft.Web.WebView2.Core.CoreWebView2ContextMenuRequestedEventArgs";
}
::windows_core::imp::interface_hierarchy!(
    CoreWebView2ContextMenuRequestedEventArgs,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for CoreWebView2ContextMenuRequestedEventArgs {}
unsafe impl ::core::marker::Sync for CoreWebView2ContextMenuRequestedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct CoreWebView2ContextMenuTarget(::windows_core::IUnknown);
impl CoreWebView2ContextMenuTarget {
    pub fn Kind(&self) -> ::windows_core::Result<CoreWebView2ContextMenuTargetKind> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn IsEditable(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsEditable)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn IsRequestedForMainFrame(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsRequestedForMainFrame)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn PageUri(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PageUri)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn FrameUri(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FrameUri)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn HasLinkUri(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HasLinkUri)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn LinkUri(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LinkUri)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn HasLinkText(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HasLinkText)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn LinkText(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LinkText)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn HasSourceUri(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HasSourceUri)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SourceUri(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SourceUri)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn HasSelection(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HasSelection)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SelectionText(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SelectionText)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for CoreWebView2ContextMenuTarget {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for CoreWebView2ContextMenuTarget {
    type Vtable = ICoreWebView2ContextMenuTarget_Vtbl;
}
unsafe impl ::windows_core::ComInterface for CoreWebView2ContextMenuTarget {
    const IID: ::windows_core::GUID =
        <ICoreWebView2ContextMenuTarget as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for CoreWebView2ContextMenuTarget {
    const NAME: &'static str = "Microsoft.Web.WebView2.Core.CoreWebView2ContextMenuTarget";
}
::windows_core::imp::interface_hierarchy!(
    CoreWebView2ContextMenuTarget,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for CoreWebView2ContextMenuTarget {}
unsafe impl ::core::marker::Sync for CoreWebView2ContextMenuTarget {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct CoreWebView2Controller(::windows_core::IUnknown);
impl CoreWebView2Controller {
    pub fn IsVisible(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsVisible)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetIsVisible(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetIsVisible)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn Bounds(&self) -> ::windows_core::Result<::windows::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Bounds)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn SetBounds(&self, value: ::windows::Foundation::Rect) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetBounds)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn ZoomFactor(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ZoomFactor)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetZoomFactor(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetZoomFactor)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn ParentWindow(&self) -> ::windows_core::Result<CoreWebView2ControllerWindowReference> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ParentWindow)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetParentWindow<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<CoreWebView2ControllerWindowReference>,
    {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetParentWindow)(
                ::windows_core::Interface::as_raw(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn CoreWebView2(&self) -> ::windows_core::Result<CoreWebView2> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CoreWebView2)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn ZoomFactorChanged<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<
                CoreWebView2Controller,
                ::windows_core::IInspectable,
            >,
        >,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ZoomFactorChanged)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveZoomFactorChanged(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveZoomFactorChanged)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn MoveFocusRequested<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<
                CoreWebView2Controller,
                CoreWebView2MoveFocusRequestedEventArgs,
            >,
        >,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MoveFocusRequested)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveMoveFocusRequested(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveMoveFocusRequested)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn GotFocus<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<
                CoreWebView2Controller,
                ::windows_core::IInspectable,
            >,
        >,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GotFocus)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveGotFocus(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveGotFocus)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn LostFocus<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<
                CoreWebView2Controller,
                ::windows_core::IInspectable,
            >,
        >,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LostFocus)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveLostFocus(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveLostFocus)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn AcceleratorKeyPressed<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<
                CoreWebView2Controller,
                CoreWebView2AcceleratorKeyPressedEventArgs,
            >,
        >,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AcceleratorKeyPressed)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveAcceleratorKeyPressed(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveAcceleratorKeyPressed)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn SetBoundsAndZoomFactor(
        &self,
        bounds: ::windows::Foundation::Rect,
        zoomfactor: f64,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetBoundsAndZoomFactor)(
                ::windows_core::Interface::as_raw(this),
                bounds,
                zoomfactor,
            )
            .ok()
        }
    }
    pub fn MoveFocus(&self, reason: CoreWebView2MoveFocusReason) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).MoveFocus)(
                ::windows_core::Interface::as_raw(this),
                reason,
            )
            .ok()
        }
    }
    pub fn NotifyParentWindowPositionChanged(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).NotifyParentWindowPositionChanged)(
                ::windows_core::Interface::as_raw(this),
            )
            .ok()
        }
    }
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this))
                .ok()
        }
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn DefaultBackgroundColor(&self) -> ::windows_core::Result<::windows::UI::Color> {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2Controller2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DefaultBackgroundColor)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn SetDefaultBackgroundColor(
        &self,
        value: ::windows::UI::Color,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2Controller2>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetDefaultBackgroundColor)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn RasterizationScale(&self) -> ::windows_core::Result<f64> {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2Controller3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RasterizationScale)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetRasterizationScale(&self, value: f64) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2Controller3>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetRasterizationScale)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn ShouldDetectMonitorScaleChanges(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2Controller3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ShouldDetectMonitorScaleChanges)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetShouldDetectMonitorScaleChanges(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2Controller3>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetShouldDetectMonitorScaleChanges)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn BoundsMode(&self) -> ::windows_core::Result<CoreWebView2BoundsMode> {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2Controller3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).BoundsMode)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetBoundsMode(&self, value: CoreWebView2BoundsMode) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2Controller3>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetBoundsMode)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RasterizationScaleChanged<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<
                CoreWebView2Controller,
                ::windows_core::IInspectable,
            >,
        >,
    {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2Controller3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RasterizationScaleChanged)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveRasterizationScaleChanged(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2Controller3>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveRasterizationScaleChanged)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn AllowExternalDrop(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2Controller4>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AllowExternalDrop)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetAllowExternalDrop(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2Controller4>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetAllowExternalDrop)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsBrowserHitTransparent(&self) -> ::windows_core::Result<bool> {
        let this =
            &::windows_core::ComInterface::cast::<ICoreWebView2PrivatePartialController>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsBrowserHitTransparent)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for CoreWebView2Controller {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for CoreWebView2Controller {
    type Vtable = ICoreWebView2Controller_Vtbl;
}
unsafe impl ::windows_core::ComInterface for CoreWebView2Controller {
    const IID: ::windows_core::GUID =
        <ICoreWebView2Controller as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for CoreWebView2Controller {
    const NAME: &'static str = "Microsoft.Web.WebView2.Core.CoreWebView2Controller";
}
::windows_core::imp::interface_hierarchy!(
    CoreWebView2Controller,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for CoreWebView2Controller {}
unsafe impl ::core::marker::Sync for CoreWebView2Controller {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct CoreWebView2ControllerOptions(::windows_core::IUnknown);
impl CoreWebView2ControllerOptions {
    pub fn ProfileName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ProfileName)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetProfileName(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetProfileName)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn IsInPrivateModeEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsInPrivateModeEnabled)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetIsInPrivateModeEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetIsInPrivateModeEnabled)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn ScriptLocale(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2ControllerOptions2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ScriptLocale)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetScriptLocale(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2ControllerOptions2>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetScriptLocale)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
}
impl ::windows_core::RuntimeType for CoreWebView2ControllerOptions {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for CoreWebView2ControllerOptions {
    type Vtable = ICoreWebView2ControllerOptions_Vtbl;
}
unsafe impl ::windows_core::ComInterface for CoreWebView2ControllerOptions {
    const IID: ::windows_core::GUID =
        <ICoreWebView2ControllerOptions as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for CoreWebView2ControllerOptions {
    const NAME: &'static str = "Microsoft.Web.WebView2.Core.CoreWebView2ControllerOptions";
}
::windows_core::imp::interface_hierarchy!(
    CoreWebView2ControllerOptions,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for CoreWebView2ControllerOptions {}
unsafe impl ::core::marker::Sync for CoreWebView2ControllerOptions {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct CoreWebView2ControllerWindowReference(::windows_core::IUnknown);
impl CoreWebView2ControllerWindowReference {
    pub fn WindowHandle(&self) -> ::windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).WindowHandle)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_UI_Core\"`"]
    #[cfg(feature = "Windows_UI_Core")]
    pub fn CoreWindow(&self) -> ::windows_core::Result<::windows::UI::Core::CoreWindow> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CoreWindow)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn CreateFromWindowHandle(
        windowhandle: u64,
    ) -> ::windows_core::Result<CoreWebView2ControllerWindowReference> {
        Self::ICoreWebView2ControllerWindowReferenceStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromWindowHandle)(
                ::windows_core::Interface::as_raw(this),
                windowhandle,
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_UI_Core\"`"]
    #[cfg(feature = "Windows_UI_Core")]
    pub fn CreateFromCoreWindow<P0>(
        corewindow: P0,
    ) -> ::windows_core::Result<CoreWebView2ControllerWindowReference>
    where
        P0: ::windows_core::IntoParam<::windows::UI::Core::CoreWindow>,
    {
        Self::ICoreWebView2ControllerWindowReferenceStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromCoreWindow)(
                ::windows_core::Interface::as_raw(this),
                corewindow.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ICoreWebView2ControllerWindowReferenceStatics<
        R,
        F: FnOnce(&ICoreWebView2ControllerWindowReferenceStatics) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<
            CoreWebView2ControllerWindowReference,
            ICoreWebView2ControllerWindowReferenceStatics,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for CoreWebView2ControllerWindowReference {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for CoreWebView2ControllerWindowReference {
    type Vtable = ICoreWebView2ControllerWindowReference_Vtbl;
}
unsafe impl ::windows_core::ComInterface for CoreWebView2ControllerWindowReference {
    const IID: ::windows_core::GUID =
        <ICoreWebView2ControllerWindowReference as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for CoreWebView2ControllerWindowReference {
    const NAME: &'static str = "Microsoft.Web.WebView2.Core.CoreWebView2ControllerWindowReference";
}
::windows_core::imp::interface_hierarchy!(
    CoreWebView2ControllerWindowReference,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for CoreWebView2ControllerWindowReference {}
unsafe impl ::core::marker::Sync for CoreWebView2ControllerWindowReference {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct CoreWebView2Cookie(::windows_core::IUnknown);
impl CoreWebView2Cookie {
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
    pub fn Domain(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Domain)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn Path(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
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
    pub fn Expires(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Expires)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetExpires(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetExpires)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsHttpOnly(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsHttpOnly)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetIsHttpOnly(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetIsHttpOnly)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn SameSite(&self) -> ::windows_core::Result<CoreWebView2CookieSameSiteKind> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SameSite)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetSameSite(&self, value: CoreWebView2CookieSameSiteKind) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetSameSite)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsSecure(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsSecure)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetIsSecure(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetIsSecure)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsSession(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsSession)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for CoreWebView2Cookie {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for CoreWebView2Cookie {
    type Vtable = ICoreWebView2Cookie_Vtbl;
}
unsafe impl ::windows_core::ComInterface for CoreWebView2Cookie {
    const IID: ::windows_core::GUID = <ICoreWebView2Cookie as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for CoreWebView2Cookie {
    const NAME: &'static str = "Microsoft.Web.WebView2.Core.CoreWebView2Cookie";
}
::windows_core::imp::interface_hierarchy!(
    CoreWebView2Cookie,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for CoreWebView2Cookie {}
unsafe impl ::core::marker::Sync for CoreWebView2Cookie {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct CoreWebView2CookieManager(::windows_core::IUnknown);
impl CoreWebView2CookieManager {
    pub fn CreateCookie(
        &self,
        name: &::windows_core::HSTRING,
        value: &::windows_core::HSTRING,
        domain: &::windows_core::HSTRING,
        path: &::windows_core::HSTRING,
    ) -> ::windows_core::Result<CoreWebView2Cookie> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateCookie)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(name),
                ::core::mem::transmute_copy(value),
                ::core::mem::transmute_copy(domain),
                ::core::mem::transmute_copy(path),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn CopyCookie<P0>(&self, cookieparam: P0) -> ::windows_core::Result<CoreWebView2Cookie>
    where
        P0: ::windows_core::IntoParam<CoreWebView2Cookie>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CopyCookie)(
                ::windows_core::Interface::as_raw(this),
                cookieparam.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn AddOrUpdateCookie<P0>(&self, cookie: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<CoreWebView2Cookie>,
    {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).AddOrUpdateCookie)(
                ::windows_core::Interface::as_raw(this),
                cookie.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn DeleteCookie<P0>(&self, cookie: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<CoreWebView2Cookie>,
    {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).DeleteCookie)(
                ::windows_core::Interface::as_raw(this),
                cookie.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn DeleteCookies(
        &self,
        name: &::windows_core::HSTRING,
        uri: &::windows_core::HSTRING,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).DeleteCookies)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(name),
                ::core::mem::transmute_copy(uri),
            )
            .ok()
        }
    }
    pub fn DeleteCookiesWithDomainAndPath(
        &self,
        name: &::windows_core::HSTRING,
        domain: &::windows_core::HSTRING,
        path: &::windows_core::HSTRING,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).DeleteCookiesWithDomainAndPath)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(name),
                ::core::mem::transmute_copy(domain),
                ::core::mem::transmute_copy(path),
            )
            .ok()
        }
    }
    pub fn DeleteAllCookies(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).DeleteAllCookies)(
                ::windows_core::Interface::as_raw(this),
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation_Collections\"`"]
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub fn GetCookiesAsync(
        &self,
        uri: &::windows_core::HSTRING,
    ) -> ::windows_core::Result<
        ::windows::Foundation::IAsyncOperation<
            ::windows::Foundation::Collections::IVectorView<CoreWebView2Cookie>,
        >,
    > {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2CookieManager_Manual>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetCookiesAsync)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(uri),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for CoreWebView2CookieManager {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for CoreWebView2CookieManager {
    type Vtable = ICoreWebView2CookieManager_Vtbl;
}
unsafe impl ::windows_core::ComInterface for CoreWebView2CookieManager {
    const IID: ::windows_core::GUID =
        <ICoreWebView2CookieManager as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for CoreWebView2CookieManager {
    const NAME: &'static str = "Microsoft.Web.WebView2.Core.CoreWebView2CookieManager";
}
::windows_core::imp::interface_hierarchy!(
    CoreWebView2CookieManager,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for CoreWebView2CookieManager {}
unsafe impl ::core::marker::Sync for CoreWebView2CookieManager {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct CoreWebView2CustomSchemeRegistration(::windows_core::IUnknown);
impl CoreWebView2CustomSchemeRegistration {
    pub fn TreatAsSecure(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TreatAsSecure)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetTreatAsSecure(&self, value: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetTreatAsSecure)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn HasAuthorityComponent(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HasAuthorityComponent)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetHasAuthorityComponent(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetHasAuthorityComponent)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
}
impl ::windows_core::RuntimeType for CoreWebView2CustomSchemeRegistration {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for CoreWebView2CustomSchemeRegistration {
    type Vtable = ICoreWebView2CustomSchemeRegistration_Vtbl;
}
unsafe impl ::windows_core::ComInterface for CoreWebView2CustomSchemeRegistration {
    const IID: ::windows_core::GUID =
        <ICoreWebView2CustomSchemeRegistration as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for CoreWebView2CustomSchemeRegistration {
    const NAME: &'static str = "Microsoft.Web.WebView2.Core.CoreWebView2CustomSchemeRegistration";
}
::windows_core::imp::interface_hierarchy!(
    CoreWebView2CustomSchemeRegistration,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for CoreWebView2CustomSchemeRegistration {}
unsafe impl ::core::marker::Sync for CoreWebView2CustomSchemeRegistration {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct CoreWebView2DOMContentLoadedEventArgs(::windows_core::IUnknown);
impl CoreWebView2DOMContentLoadedEventArgs {
    pub fn NavigationId(&self) -> ::windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NavigationId)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for CoreWebView2DOMContentLoadedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for CoreWebView2DOMContentLoadedEventArgs {
    type Vtable = ICoreWebView2DOMContentLoadedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for CoreWebView2DOMContentLoadedEventArgs {
    const IID: ::windows_core::GUID =
        <ICoreWebView2DOMContentLoadedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for CoreWebView2DOMContentLoadedEventArgs {
    const NAME: &'static str = "Microsoft.Web.WebView2.Core.CoreWebView2DOMContentLoadedEventArgs";
}
::windows_core::imp::interface_hierarchy!(
    CoreWebView2DOMContentLoadedEventArgs,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for CoreWebView2DOMContentLoadedEventArgs {}
unsafe impl ::core::marker::Sync for CoreWebView2DOMContentLoadedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct CoreWebView2DevToolsProtocolEventReceivedEventArgs(::windows_core::IUnknown);
impl CoreWebView2DevToolsProtocolEventReceivedEventArgs {
    pub fn ParameterObjectAsJson(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ParameterObjectAsJson)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SessionId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<
            ICoreWebView2DevToolsProtocolEventReceivedEventArgs2,
        >(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SessionId)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for CoreWebView2DevToolsProtocolEventReceivedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for CoreWebView2DevToolsProtocolEventReceivedEventArgs {
    type Vtable = ICoreWebView2DevToolsProtocolEventReceivedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for CoreWebView2DevToolsProtocolEventReceivedEventArgs {
    const IID: ::windows_core::GUID =
        <ICoreWebView2DevToolsProtocolEventReceivedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for CoreWebView2DevToolsProtocolEventReceivedEventArgs {
    const NAME: &'static str =
        "Microsoft.Web.WebView2.Core.CoreWebView2DevToolsProtocolEventReceivedEventArgs";
}
::windows_core::imp::interface_hierarchy!(
    CoreWebView2DevToolsProtocolEventReceivedEventArgs,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for CoreWebView2DevToolsProtocolEventReceivedEventArgs {}
unsafe impl ::core::marker::Sync for CoreWebView2DevToolsProtocolEventReceivedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct CoreWebView2DevToolsProtocolEventReceiver(::windows_core::IUnknown);
impl CoreWebView2DevToolsProtocolEventReceiver {
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn DevToolsProtocolEventReceived<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<
                CoreWebView2,
                CoreWebView2DevToolsProtocolEventReceivedEventArgs,
            >,
        >,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DevToolsProtocolEventReceived)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveDevToolsProtocolEventReceived(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveDevToolsProtocolEventReceived)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
}
impl ::windows_core::RuntimeType for CoreWebView2DevToolsProtocolEventReceiver {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for CoreWebView2DevToolsProtocolEventReceiver {
    type Vtable = ICoreWebView2DevToolsProtocolEventReceiver_Vtbl;
}
unsafe impl ::windows_core::ComInterface for CoreWebView2DevToolsProtocolEventReceiver {
    const IID: ::windows_core::GUID =
        <ICoreWebView2DevToolsProtocolEventReceiver as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for CoreWebView2DevToolsProtocolEventReceiver {
    const NAME: &'static str =
        "Microsoft.Web.WebView2.Core.CoreWebView2DevToolsProtocolEventReceiver";
}
::windows_core::imp::interface_hierarchy!(
    CoreWebView2DevToolsProtocolEventReceiver,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for CoreWebView2DevToolsProtocolEventReceiver {}
unsafe impl ::core::marker::Sync for CoreWebView2DevToolsProtocolEventReceiver {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct CoreWebView2DownloadOperation(::windows_core::IUnknown);
impl CoreWebView2DownloadOperation {
    pub fn Uri(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Uri)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn ContentDisposition(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ContentDisposition)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn MimeType(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MimeType)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn TotalBytesToReceive(&self) -> ::windows_core::Result<i64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TotalBytesToReceive)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn BytesReceived(&self) -> ::windows_core::Result<i64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).BytesReceived)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn EstimatedEndTime(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).EstimatedEndTime)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn ResultFilePath(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ResultFilePath)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn State(&self) -> ::windows_core::Result<CoreWebView2DownloadState> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).State)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn InterruptReason(&self) -> ::windows_core::Result<CoreWebView2DownloadInterruptReason> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).InterruptReason)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn CanResume(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CanResume)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn BytesReceivedChanged<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<
                CoreWebView2DownloadOperation,
                ::windows_core::IInspectable,
            >,
        >,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).BytesReceivedChanged)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveBytesReceivedChanged(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveBytesReceivedChanged)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn EstimatedEndTimeChanged<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<
                CoreWebView2DownloadOperation,
                ::windows_core::IInspectable,
            >,
        >,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).EstimatedEndTimeChanged)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveEstimatedEndTimeChanged(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveEstimatedEndTimeChanged)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn StateChanged<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<
                CoreWebView2DownloadOperation,
                ::windows_core::IInspectable,
            >,
        >,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StateChanged)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveStateChanged(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveStateChanged)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn Cancel(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).Cancel)(::windows_core::Interface::as_raw(
                this,
            ))
            .ok()
        }
    }
    pub fn Pause(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).Pause)(::windows_core::Interface::as_raw(this))
                .ok()
        }
    }
    pub fn Resume(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).Resume)(::windows_core::Interface::as_raw(
                this,
            ))
            .ok()
        }
    }
}
impl ::windows_core::RuntimeType for CoreWebView2DownloadOperation {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for CoreWebView2DownloadOperation {
    type Vtable = ICoreWebView2DownloadOperation_Vtbl;
}
unsafe impl ::windows_core::ComInterface for CoreWebView2DownloadOperation {
    const IID: ::windows_core::GUID =
        <ICoreWebView2DownloadOperation as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for CoreWebView2DownloadOperation {
    const NAME: &'static str = "Microsoft.Web.WebView2.Core.CoreWebView2DownloadOperation";
}
::windows_core::imp::interface_hierarchy!(
    CoreWebView2DownloadOperation,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for CoreWebView2DownloadOperation {}
unsafe impl ::core::marker::Sync for CoreWebView2DownloadOperation {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct CoreWebView2DownloadStartingEventArgs(::windows_core::IUnknown);
impl CoreWebView2DownloadStartingEventArgs {
    pub fn DownloadOperation(&self) -> ::windows_core::Result<CoreWebView2DownloadOperation> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DownloadOperation)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
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
    pub fn ResultFilePath(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ResultFilePath)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetResultFilePath(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetResultFilePath)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn Handled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Handled)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetHandled)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn GetDeferral(&self) -> ::windows_core::Result<::windows::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetDeferral)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for CoreWebView2DownloadStartingEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for CoreWebView2DownloadStartingEventArgs {
    type Vtable = ICoreWebView2DownloadStartingEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for CoreWebView2DownloadStartingEventArgs {
    const IID: ::windows_core::GUID =
        <ICoreWebView2DownloadStartingEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for CoreWebView2DownloadStartingEventArgs {
    const NAME: &'static str = "Microsoft.Web.WebView2.Core.CoreWebView2DownloadStartingEventArgs";
}
::windows_core::imp::interface_hierarchy!(
    CoreWebView2DownloadStartingEventArgs,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for CoreWebView2DownloadStartingEventArgs {}
unsafe impl ::core::marker::Sync for CoreWebView2DownloadStartingEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct CoreWebView2Environment(::windows_core::IUnknown);
impl CoreWebView2Environment {
    pub fn BrowserVersionString(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).BrowserVersionString)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn NewBrowserVersionAvailable<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<
                CoreWebView2Environment,
                ::windows_core::IInspectable,
            >,
        >,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NewBrowserVersionAvailable)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveNewBrowserVersionAvailable(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveNewBrowserVersionAvailable)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn CreateCoreWebView2ControllerAsync<P0>(
        &self,
        parentwindow: P0,
    ) -> ::windows_core::Result<::windows::Foundation::IAsyncOperation<CoreWebView2Controller>>
    where
        P0: ::windows_core::IntoParam<CoreWebView2ControllerWindowReference>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateCoreWebView2ControllerAsync)(
                ::windows_core::Interface::as_raw(this),
                parentwindow.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Storage_Streams\"`"]
    #[cfg(feature = "Windows_Storage_Streams")]
    pub fn CreateWebResourceResponse<P0>(
        &self,
        content: P0,
        statuscode: i32,
        reasonphrase: &::windows_core::HSTRING,
        headers: &::windows_core::HSTRING,
    ) -> ::windows_core::Result<CoreWebView2WebResourceResponse>
    where
        P0: ::windows_core::TryIntoParam<::windows::Storage::Streams::IRandomAccessStream>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateWebResourceResponse)(
                ::windows_core::Interface::as_raw(this),
                content.try_into_param()?.abi(),
                statuscode,
                ::core::mem::transmute_copy(reasonphrase),
                ::core::mem::transmute_copy(headers),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn CreateCoreWebView2ControllerOptions(
        &self,
    ) -> ::windows_core::Result<CoreWebView2ControllerOptions> {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2Environment10>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateCoreWebView2ControllerOptions)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn FailureReportFolderPath(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2Environment11>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FailureReportFolderPath)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn CreateSharedBuffer(
        &self,
        size: u64,
    ) -> ::windows_core::Result<CoreWebView2SharedBuffer> {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2Environment12>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateSharedBuffer)(
                ::windows_core::Interface::as_raw(this),
                size,
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Storage_Streams\"`"]
    #[cfg(feature = "Windows_Storage_Streams")]
    pub fn CreateWebResourceRequest<P0>(
        &self,
        uri: &::windows_core::HSTRING,
        method: &::windows_core::HSTRING,
        postdata: P0,
        headers: &::windows_core::HSTRING,
    ) -> ::windows_core::Result<CoreWebView2WebResourceRequest>
    where
        P0: ::windows_core::TryIntoParam<::windows::Storage::Streams::IRandomAccessStream>,
    {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2Environment2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateWebResourceRequest)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(uri),
                ::core::mem::transmute_copy(method),
                postdata.try_into_param()?.abi(),
                ::core::mem::transmute_copy(headers),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn CreateCoreWebView2CompositionControllerAsync<P0>(
        &self,
        parentwindow: P0,
    ) -> ::windows_core::Result<
        ::windows::Foundation::IAsyncOperation<CoreWebView2CompositionController>,
    >
    where
        P0: ::windows_core::IntoParam<CoreWebView2ControllerWindowReference>,
    {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2Environment3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateCoreWebView2CompositionControllerAsync)(
                ::windows_core::Interface::as_raw(this),
                parentwindow.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn CreateCoreWebView2PointerInfo(&self) -> ::windows_core::Result<CoreWebView2PointerInfo> {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2Environment3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateCoreWebView2PointerInfo)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn BrowserProcessExited<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<
                CoreWebView2Environment,
                CoreWebView2BrowserProcessExitedEventArgs,
            >,
        >,
    {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2Environment5>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).BrowserProcessExited)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveBrowserProcessExited(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2Environment5>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveBrowserProcessExited)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn CreatePrintSettings(&self) -> ::windows_core::Result<CoreWebView2PrintSettings> {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2Environment6>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreatePrintSettings)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn UserDataFolder(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2Environment7>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).UserDataFolder)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn ProcessInfosChanged<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<
                CoreWebView2Environment,
                ::windows_core::IInspectable,
            >,
        >,
    {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2Environment8>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ProcessInfosChanged)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveProcessInfosChanged(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2Environment8>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveProcessInfosChanged)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation_Collections\"`"]
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub fn GetProcessInfos(
        &self,
    ) -> ::windows_core::Result<
        ::windows::Foundation::Collections::IVectorView<CoreWebView2ProcessInfo>,
    > {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2Environment8>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetProcessInfos)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Storage_Streams\"`"]
    #[cfg(feature = "Windows_Storage_Streams")]
    pub fn CreateContextMenuItem<P0>(
        &self,
        label: &::windows_core::HSTRING,
        iconstream: P0,
        kind: CoreWebView2ContextMenuItemKind,
    ) -> ::windows_core::Result<CoreWebView2ContextMenuItem>
    where
        P0: ::windows_core::TryIntoParam<::windows::Storage::Streams::IRandomAccessStream>,
    {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2Environment9>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateContextMenuItem)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(label),
                iconstream.try_into_param()?.abi(),
                kind,
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn CreateAsync(
    ) -> ::windows_core::Result<::windows::Foundation::IAsyncOperation<CoreWebView2Environment>>
    {
        Self::ICoreWebView2EnvironmentStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateAsync)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn CreateWithOptionsAsync<P0>(
        browserexecutablefolder: &::windows_core::HSTRING,
        userdatafolder: &::windows_core::HSTRING,
        options: P0,
    ) -> ::windows_core::Result<::windows::Foundation::IAsyncOperation<CoreWebView2Environment>>
    where
        P0: ::windows_core::IntoParam<CoreWebView2EnvironmentOptions>,
    {
        Self::ICoreWebView2EnvironmentStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateWithOptionsAsync)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(browserexecutablefolder),
                ::core::mem::transmute_copy(userdatafolder),
                options.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn GetAvailableBrowserVersionString() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICoreWebView2EnvironmentStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetAvailableBrowserVersionString)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn GetAvailableBrowserVersionString2(
        browserexecutablefolder: &::windows_core::HSTRING,
    ) -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICoreWebView2EnvironmentStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetAvailableBrowserVersionString2)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(browserexecutablefolder),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn CompareBrowserVersionString(
        browserversionstring1: &::windows_core::HSTRING,
        browserversionstring2: &::windows_core::HSTRING,
    ) -> ::windows_core::Result<i32> {
        Self::ICoreWebView2EnvironmentStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CompareBrowserVersionString)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(browserversionstring1),
                ::core::mem::transmute_copy(browserversionstring2),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn CreateCoreWebView2ControllerAsync2<P0, P1>(
        &self,
        parentwindow: P0,
        options: P1,
    ) -> ::windows_core::Result<::windows::Foundation::IAsyncOperation<CoreWebView2Controller>>
    where
        P0: ::windows_core::IntoParam<CoreWebView2ControllerWindowReference>,
        P1: ::windows_core::IntoParam<CoreWebView2ControllerOptions>,
    {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2Environment_Manual>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateCoreWebView2ControllerAsync)(
                ::windows_core::Interface::as_raw(this),
                parentwindow.into_param().abi(),
                options.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn CreateCoreWebView2CompositionControllerAsync2<P0, P1>(
        &self,
        parentwindow: P0,
        options: P1,
    ) -> ::windows_core::Result<
        ::windows::Foundation::IAsyncOperation<CoreWebView2CompositionController>,
    >
    where
        P0: ::windows_core::IntoParam<CoreWebView2ControllerWindowReference>,
        P1: ::windows_core::IntoParam<CoreWebView2ControllerOptions>,
    {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2Environment_Manual>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateCoreWebView2CompositionControllerAsync)(
                ::windows_core::Interface::as_raw(this),
                parentwindow.into_param().abi(),
                options.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc(hidden)]
    pub fn ICoreWebView2EnvironmentStatics<
        R,
        F: FnOnce(&ICoreWebView2EnvironmentStatics) -> ::windows_core::Result<R>,
    >(
        callback: F,
    ) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<
            CoreWebView2Environment,
            ICoreWebView2EnvironmentStatics,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for CoreWebView2Environment {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for CoreWebView2Environment {
    type Vtable = ICoreWebView2Environment_Vtbl;
}
unsafe impl ::windows_core::ComInterface for CoreWebView2Environment {
    const IID: ::windows_core::GUID =
        <ICoreWebView2Environment as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for CoreWebView2Environment {
    const NAME: &'static str = "Microsoft.Web.WebView2.Core.CoreWebView2Environment";
}
::windows_core::imp::interface_hierarchy!(
    CoreWebView2Environment,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for CoreWebView2Environment {}
unsafe impl ::core::marker::Sync for CoreWebView2Environment {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct CoreWebView2EnvironmentOptions(::windows_core::IUnknown);
impl CoreWebView2EnvironmentOptions {
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
            CoreWebView2EnvironmentOptions,
            ::windows_core::imp::IGenericFactory,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn AdditionalBrowserArguments(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AdditionalBrowserArguments)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetAdditionalBrowserArguments(
        &self,
        value: &::windows_core::HSTRING,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetAdditionalBrowserArguments)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn Language(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Language)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetLanguage(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetLanguage)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn TargetCompatibleBrowserVersion(
        &self,
    ) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TargetCompatibleBrowserVersion)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetTargetCompatibleBrowserVersion(
        &self,
        value: &::windows_core::HSTRING,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetTargetCompatibleBrowserVersion)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn AllowSingleSignOnUsingOSPrimaryAccount(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AllowSingleSignOnUsingOSPrimaryAccount)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetAllowSingleSignOnUsingOSPrimaryAccount(
        &self,
        value: bool,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetAllowSingleSignOnUsingOSPrimaryAccount)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn ExclusiveUserDataFolderAccess(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2EnvironmentOptions2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ExclusiveUserDataFolderAccess)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetExclusiveUserDataFolderAccess(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2EnvironmentOptions2>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetExclusiveUserDataFolderAccess)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsCustomCrashReportingEnabled(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2EnvironmentOptions3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsCustomCrashReportingEnabled)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetIsCustomCrashReportingEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2EnvironmentOptions3>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetIsCustomCrashReportingEnabled)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn EnableTrackingPrevention(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2EnvironmentOptions5>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).EnableTrackingPrevention)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetEnableTrackingPrevention(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2EnvironmentOptions5>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetEnableTrackingPrevention)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
}
impl ::windows_core::RuntimeType for CoreWebView2EnvironmentOptions {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for CoreWebView2EnvironmentOptions {
    type Vtable = ICoreWebView2EnvironmentOptions_Vtbl;
}
unsafe impl ::windows_core::ComInterface for CoreWebView2EnvironmentOptions {
    const IID: ::windows_core::GUID =
        <ICoreWebView2EnvironmentOptions as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for CoreWebView2EnvironmentOptions {
    const NAME: &'static str = "Microsoft.Web.WebView2.Core.CoreWebView2EnvironmentOptions";
}
::windows_core::imp::interface_hierarchy!(
    CoreWebView2EnvironmentOptions,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for CoreWebView2EnvironmentOptions {}
unsafe impl ::core::marker::Sync for CoreWebView2EnvironmentOptions {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct CoreWebView2File(::windows_core::IUnknown);
impl CoreWebView2File {
    pub fn Path(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
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
}
impl ::windows_core::RuntimeType for CoreWebView2File {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for CoreWebView2File {
    type Vtable = ICoreWebView2File_Vtbl;
}
unsafe impl ::windows_core::ComInterface for CoreWebView2File {
    const IID: ::windows_core::GUID = <ICoreWebView2File as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for CoreWebView2File {
    const NAME: &'static str = "Microsoft.Web.WebView2.Core.CoreWebView2File";
}
::windows_core::imp::interface_hierarchy!(
    CoreWebView2File,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for CoreWebView2File {}
unsafe impl ::core::marker::Sync for CoreWebView2File {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct CoreWebView2Frame(::windows_core::IUnknown);
impl CoreWebView2Frame {
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
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn NameChanged<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<
                CoreWebView2Frame,
                ::windows_core::IInspectable,
            >,
        >,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NameChanged)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveNameChanged(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveNameChanged)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn Destroyed<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<
                CoreWebView2Frame,
                ::windows_core::IInspectable,
            >,
        >,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Destroyed)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveDestroyed(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveDestroyed)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn RemoveHostObjectFromScript(
        &self,
        name: &::windows_core::HSTRING,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveHostObjectFromScript)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(name),
            )
            .ok()
        }
    }
    pub fn IsDestroyed(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsDestroyed)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn NavigationStarting<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<
                CoreWebView2Frame,
                CoreWebView2NavigationStartingEventArgs,
            >,
        >,
    {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2Frame2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NavigationStarting)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveNavigationStarting(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2Frame2>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveNavigationStarting)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn ContentLoading<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<
                CoreWebView2Frame,
                CoreWebView2ContentLoadingEventArgs,
            >,
        >,
    {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2Frame2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ContentLoading)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveContentLoading(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2Frame2>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveContentLoading)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn NavigationCompleted<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<
                CoreWebView2Frame,
                CoreWebView2NavigationCompletedEventArgs,
            >,
        >,
    {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2Frame2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NavigationCompleted)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveNavigationCompleted(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2Frame2>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveNavigationCompleted)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn DOMContentLoaded<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<
                CoreWebView2Frame,
                CoreWebView2DOMContentLoadedEventArgs,
            >,
        >,
    {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2Frame2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DOMContentLoaded)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveDOMContentLoaded(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2Frame2>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveDOMContentLoaded)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn WebMessageReceived<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<
                CoreWebView2Frame,
                CoreWebView2WebMessageReceivedEventArgs,
            >,
        >,
    {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2Frame2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).WebMessageReceived)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemoveWebMessageReceived(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2Frame2>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveWebMessageReceived)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn ExecuteScriptAsync(
        &self,
        javascript: &::windows_core::HSTRING,
    ) -> ::windows_core::Result<::windows::Foundation::IAsyncOperation<::windows_core::HSTRING>>
    {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2Frame2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ExecuteScriptAsync)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(javascript),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn PostWebMessageAsJson(
        &self,
        webmessageasjson: &::windows_core::HSTRING,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2Frame2>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).PostWebMessageAsJson)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(webmessageasjson),
            )
            .ok()
        }
    }
    pub fn PostWebMessageAsString(
        &self,
        webmessageasstring: &::windows_core::HSTRING,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2Frame2>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).PostWebMessageAsString)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(webmessageasstring),
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn PermissionRequested<P0>(
        &self,
        handler: P0,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<
            ::windows::Foundation::TypedEventHandler<
                CoreWebView2Frame,
                CoreWebView2PermissionRequestedEventArgs,
            >,
        >,
    {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2Frame3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PermissionRequested)(
                ::windows_core::Interface::as_raw(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn RemovePermissionRequested(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2Frame3>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).RemovePermissionRequested)(
                ::windows_core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn PostSharedBufferToScript<P0>(
        &self,
        sharedbuffer: P0,
        access: CoreWebView2SharedBufferAccess,
        additionaldataasjson: &::windows_core::HSTRING,
    ) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<CoreWebView2SharedBuffer>,
    {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2Frame4>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).PostSharedBufferToScript)(
                ::windows_core::Interface::as_raw(this),
                sharedbuffer.into_param().abi(),
                access,
                ::core::mem::transmute_copy(additionaldataasjson),
            )
            .ok()
        }
    }
}
impl ::windows_core::RuntimeType for CoreWebView2Frame {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for CoreWebView2Frame {
    type Vtable = ICoreWebView2Frame_Vtbl;
}
unsafe impl ::windows_core::ComInterface for CoreWebView2Frame {
    const IID: ::windows_core::GUID = <ICoreWebView2Frame as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for CoreWebView2Frame {
    const NAME: &'static str = "Microsoft.Web.WebView2.Core.CoreWebView2Frame";
}
::windows_core::imp::interface_hierarchy!(
    CoreWebView2Frame,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for CoreWebView2Frame {}
unsafe impl ::core::marker::Sync for CoreWebView2Frame {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct CoreWebView2FrameCreatedEventArgs(::windows_core::IUnknown);
impl CoreWebView2FrameCreatedEventArgs {
    pub fn Frame(&self) -> ::windows_core::Result<CoreWebView2Frame> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Frame)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for CoreWebView2FrameCreatedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for CoreWebView2FrameCreatedEventArgs {
    type Vtable = ICoreWebView2FrameCreatedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for CoreWebView2FrameCreatedEventArgs {
    const IID: ::windows_core::GUID =
        <ICoreWebView2FrameCreatedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for CoreWebView2FrameCreatedEventArgs {
    const NAME: &'static str = "Microsoft.Web.WebView2.Core.CoreWebView2FrameCreatedEventArgs";
}
::windows_core::imp::interface_hierarchy!(
    CoreWebView2FrameCreatedEventArgs,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for CoreWebView2FrameCreatedEventArgs {}
unsafe impl ::core::marker::Sync for CoreWebView2FrameCreatedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct CoreWebView2FrameInfo(::windows_core::IUnknown);
impl CoreWebView2FrameInfo {
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
    pub fn Source(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
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
}
impl ::windows_core::RuntimeType for CoreWebView2FrameInfo {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for CoreWebView2FrameInfo {
    type Vtable = ICoreWebView2FrameInfo_Vtbl;
}
unsafe impl ::windows_core::ComInterface for CoreWebView2FrameInfo {
    const IID: ::windows_core::GUID = <ICoreWebView2FrameInfo as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for CoreWebView2FrameInfo {
    const NAME: &'static str = "Microsoft.Web.WebView2.Core.CoreWebView2FrameInfo";
}
::windows_core::imp::interface_hierarchy!(
    CoreWebView2FrameInfo,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for CoreWebView2FrameInfo {}
unsafe impl ::core::marker::Sync for CoreWebView2FrameInfo {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct CoreWebView2HttpHeadersCollectionIterator(::windows_core::IUnknown);
impl CoreWebView2HttpHeadersCollectionIterator {
    #[doc = "Required features: `\"Windows_Foundation_Collections\"`"]
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub fn Current(
        &self,
    ) -> ::windows_core::Result<
        ::windows::Foundation::Collections::IKeyValuePair<
            ::windows_core::HSTRING,
            ::windows_core::HSTRING,
        >,
    > {
        let this = &::windows_core::ComInterface::cast::<
            ::windows::Foundation::Collections::IIterator<
                ::windows::Foundation::Collections::IKeyValuePair<
                    ::windows_core::HSTRING,
                    ::windows_core::HSTRING,
                >,
            >,
        >(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Current)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation_Collections\"`"]
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub fn HasCurrent(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<
            ::windows::Foundation::Collections::IIterator<
                ::windows::Foundation::Collections::IKeyValuePair<
                    ::windows_core::HSTRING,
                    ::windows_core::HSTRING,
                >,
            >,
        >(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HasCurrent)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation_Collections\"`"]
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub fn MoveNext(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<
            ::windows::Foundation::Collections::IIterator<
                ::windows::Foundation::Collections::IKeyValuePair<
                    ::windows_core::HSTRING,
                    ::windows_core::HSTRING,
                >,
            >,
        >(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MoveNext)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation_Collections\"`"]
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub fn GetMany(
        &self,
        items: &mut [::core::option::Option<
            ::windows::Foundation::Collections::IKeyValuePair<
                ::windows_core::HSTRING,
                ::windows_core::HSTRING,
            >,
        >],
    ) -> ::windows_core::Result<u32> {
        let this = &::windows_core::ComInterface::cast::<
            ::windows::Foundation::Collections::IIterator<
                ::windows::Foundation::Collections::IKeyValuePair<
                    ::windows_core::HSTRING,
                    ::windows_core::HSTRING,
                >,
            >,
        >(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetMany)(
                ::windows_core::Interface::as_raw(this),
                items.len().try_into().unwrap(),
                ::core::mem::transmute_copy(&items),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for CoreWebView2HttpHeadersCollectionIterator {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for CoreWebView2HttpHeadersCollectionIterator {
    type Vtable = ICoreWebView2HttpHeadersCollectionIterator_Vtbl;
}
unsafe impl ::windows_core::ComInterface for CoreWebView2HttpHeadersCollectionIterator {
    const IID: ::windows_core::GUID =
        <ICoreWebView2HttpHeadersCollectionIterator as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for CoreWebView2HttpHeadersCollectionIterator {
    const NAME: &'static str =
        "Microsoft.Web.WebView2.Core.CoreWebView2HttpHeadersCollectionIterator";
}
::windows_core::imp::interface_hierarchy!(
    CoreWebView2HttpHeadersCollectionIterator,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
#[cfg(feature = "Windows_Foundation_Collections")]
impl
    ::windows_core::CanTryInto<
        ::windows::Foundation::Collections::IIterator<
            ::windows::Foundation::Collections::IKeyValuePair<
                ::windows_core::HSTRING,
                ::windows_core::HSTRING,
            >,
        >,
    > for CoreWebView2HttpHeadersCollectionIterator
{
}
unsafe impl ::core::marker::Send for CoreWebView2HttpHeadersCollectionIterator {}
unsafe impl ::core::marker::Sync for CoreWebView2HttpHeadersCollectionIterator {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct CoreWebView2HttpRequestHeaders(::windows_core::IUnknown);
impl CoreWebView2HttpRequestHeaders {
    pub fn GetHeader(
        &self,
        name: &::windows_core::HSTRING,
    ) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetHeader)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(name),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn GetHeaders(
        &self,
        name: &::windows_core::HSTRING,
    ) -> ::windows_core::Result<CoreWebView2HttpHeadersCollectionIterator> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetHeaders)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(name),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn Contains(&self, name: &::windows_core::HSTRING) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Contains)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(name),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetHeader(
        &self,
        name: &::windows_core::HSTRING,
        value: &::windows_core::HSTRING,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetHeader)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(name),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn RemoveHeader(&self, name: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).RemoveHeader)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(name),
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation_Collections\"`"]
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub fn First(
        &self,
    ) -> ::windows_core::Result<
        ::windows::Foundation::Collections::IIterator<
            ::windows::Foundation::Collections::IKeyValuePair<
                ::windows_core::HSTRING,
                ::windows_core::HSTRING,
            >,
        >,
    > {
        let this = &::windows_core::ComInterface::cast::<
            ::windows::Foundation::Collections::IIterable<
                ::windows::Foundation::Collections::IKeyValuePair<
                    ::windows_core::HSTRING,
                    ::windows_core::HSTRING,
                >,
            >,
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
}
impl ::windows_core::RuntimeType for CoreWebView2HttpRequestHeaders {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for CoreWebView2HttpRequestHeaders {
    type Vtable = ICoreWebView2HttpRequestHeaders_Vtbl;
}
unsafe impl ::windows_core::ComInterface for CoreWebView2HttpRequestHeaders {
    const IID: ::windows_core::GUID =
        <ICoreWebView2HttpRequestHeaders as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for CoreWebView2HttpRequestHeaders {
    const NAME: &'static str = "Microsoft.Web.WebView2.Core.CoreWebView2HttpRequestHeaders";
}
#[cfg(feature = "Windows_Foundation_Collections")]
impl ::core::iter::IntoIterator for CoreWebView2HttpRequestHeaders {
    type Item = ::windows::Foundation::Collections::IKeyValuePair<
        ::windows_core::HSTRING,
        ::windows_core::HSTRING,
    >;
    type IntoIter = ::windows::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Windows_Foundation_Collections")]
impl ::core::iter::IntoIterator for &CoreWebView2HttpRequestHeaders {
    type Item = ::windows::Foundation::Collections::IKeyValuePair<
        ::windows_core::HSTRING,
        ::windows_core::HSTRING,
    >;
    type IntoIter = ::windows::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
::windows_core::imp::interface_hierarchy!(
    CoreWebView2HttpRequestHeaders,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
#[cfg(feature = "Windows_Foundation_Collections")]
impl
    ::windows_core::CanTryInto<
        ::windows::Foundation::Collections::IIterable<
            ::windows::Foundation::Collections::IKeyValuePair<
                ::windows_core::HSTRING,
                ::windows_core::HSTRING,
            >,
        >,
    > for CoreWebView2HttpRequestHeaders
{
}
unsafe impl ::core::marker::Send for CoreWebView2HttpRequestHeaders {}
unsafe impl ::core::marker::Sync for CoreWebView2HttpRequestHeaders {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct CoreWebView2HttpResponseHeaders(::windows_core::IUnknown);
impl CoreWebView2HttpResponseHeaders {
    pub fn AppendHeader(
        &self,
        name: &::windows_core::HSTRING,
        value: &::windows_core::HSTRING,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).AppendHeader)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(name),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn Contains(&self, name: &::windows_core::HSTRING) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Contains)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(name),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn GetHeader(
        &self,
        name: &::windows_core::HSTRING,
    ) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetHeader)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(name),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn GetHeaders(
        &self,
        name: &::windows_core::HSTRING,
    ) -> ::windows_core::Result<CoreWebView2HttpHeadersCollectionIterator> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetHeaders)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(name),
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
        ::windows::Foundation::Collections::IIterator<
            ::windows::Foundation::Collections::IKeyValuePair<
                ::windows_core::HSTRING,
                ::windows_core::HSTRING,
            >,
        >,
    > {
        let this = &::windows_core::ComInterface::cast::<
            ::windows::Foundation::Collections::IIterable<
                ::windows::Foundation::Collections::IKeyValuePair<
                    ::windows_core::HSTRING,
                    ::windows_core::HSTRING,
                >,
            >,
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
}
impl ::windows_core::RuntimeType for CoreWebView2HttpResponseHeaders {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for CoreWebView2HttpResponseHeaders {
    type Vtable = ICoreWebView2HttpResponseHeaders_Vtbl;
}
unsafe impl ::windows_core::ComInterface for CoreWebView2HttpResponseHeaders {
    const IID: ::windows_core::GUID =
        <ICoreWebView2HttpResponseHeaders as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for CoreWebView2HttpResponseHeaders {
    const NAME: &'static str = "Microsoft.Web.WebView2.Core.CoreWebView2HttpResponseHeaders";
}
#[cfg(feature = "Windows_Foundation_Collections")]
impl ::core::iter::IntoIterator for CoreWebView2HttpResponseHeaders {
    type Item = ::windows::Foundation::Collections::IKeyValuePair<
        ::windows_core::HSTRING,
        ::windows_core::HSTRING,
    >;
    type IntoIter = ::windows::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Windows_Foundation_Collections")]
impl ::core::iter::IntoIterator for &CoreWebView2HttpResponseHeaders {
    type Item = ::windows::Foundation::Collections::IKeyValuePair<
        ::windows_core::HSTRING,
        ::windows_core::HSTRING,
    >;
    type IntoIter = ::windows::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
::windows_core::imp::interface_hierarchy!(
    CoreWebView2HttpResponseHeaders,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
#[cfg(feature = "Windows_Foundation_Collections")]
impl
    ::windows_core::CanTryInto<
        ::windows::Foundation::Collections::IIterable<
            ::windows::Foundation::Collections::IKeyValuePair<
                ::windows_core::HSTRING,
                ::windows_core::HSTRING,
            >,
        >,
    > for CoreWebView2HttpResponseHeaders
{
}
unsafe impl ::core::marker::Send for CoreWebView2HttpResponseHeaders {}
unsafe impl ::core::marker::Sync for CoreWebView2HttpResponseHeaders {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct CoreWebView2LaunchingExternalUriSchemeEventArgs(::windows_core::IUnknown);
impl CoreWebView2LaunchingExternalUriSchemeEventArgs {
    pub fn Uri(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Uri)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn InitiatingOrigin(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).InitiatingOrigin)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn IsUserInitiated(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsUserInitiated)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
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
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn GetDeferral(&self) -> ::windows_core::Result<::windows::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetDeferral)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for CoreWebView2LaunchingExternalUriSchemeEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for CoreWebView2LaunchingExternalUriSchemeEventArgs {
    type Vtable = ICoreWebView2LaunchingExternalUriSchemeEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for CoreWebView2LaunchingExternalUriSchemeEventArgs {
    const IID: ::windows_core::GUID =
        <ICoreWebView2LaunchingExternalUriSchemeEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for CoreWebView2LaunchingExternalUriSchemeEventArgs {
    const NAME: &'static str =
        "Microsoft.Web.WebView2.Core.CoreWebView2LaunchingExternalUriSchemeEventArgs";
}
::windows_core::imp::interface_hierarchy!(
    CoreWebView2LaunchingExternalUriSchemeEventArgs,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for CoreWebView2LaunchingExternalUriSchemeEventArgs {}
unsafe impl ::core::marker::Sync for CoreWebView2LaunchingExternalUriSchemeEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct CoreWebView2MoveFocusRequestedEventArgs(::windows_core::IUnknown);
impl CoreWebView2MoveFocusRequestedEventArgs {
    pub fn Reason(&self) -> ::windows_core::Result<CoreWebView2MoveFocusReason> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Reason)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn Handled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Handled)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetHandled)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
}
impl ::windows_core::RuntimeType for CoreWebView2MoveFocusRequestedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for CoreWebView2MoveFocusRequestedEventArgs {
    type Vtable = ICoreWebView2MoveFocusRequestedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for CoreWebView2MoveFocusRequestedEventArgs {
    const IID: ::windows_core::GUID =
        <ICoreWebView2MoveFocusRequestedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for CoreWebView2MoveFocusRequestedEventArgs {
    const NAME: &'static str =
        "Microsoft.Web.WebView2.Core.CoreWebView2MoveFocusRequestedEventArgs";
}
::windows_core::imp::interface_hierarchy!(
    CoreWebView2MoveFocusRequestedEventArgs,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for CoreWebView2MoveFocusRequestedEventArgs {}
unsafe impl ::core::marker::Sync for CoreWebView2MoveFocusRequestedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct CoreWebView2NavigationCompletedEventArgs(::windows_core::IUnknown);
impl CoreWebView2NavigationCompletedEventArgs {
    pub fn IsSuccess(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsSuccess)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn WebErrorStatus(&self) -> ::windows_core::Result<CoreWebView2WebErrorStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).WebErrorStatus)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn NavigationId(&self) -> ::windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NavigationId)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn HttpStatusCode(&self) -> ::windows_core::Result<i32> {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2NavigationCompletedEventArgs2>(
            self,
        )?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HttpStatusCode)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for CoreWebView2NavigationCompletedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for CoreWebView2NavigationCompletedEventArgs {
    type Vtable = ICoreWebView2NavigationCompletedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for CoreWebView2NavigationCompletedEventArgs {
    const IID: ::windows_core::GUID =
        <ICoreWebView2NavigationCompletedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for CoreWebView2NavigationCompletedEventArgs {
    const NAME: &'static str =
        "Microsoft.Web.WebView2.Core.CoreWebView2NavigationCompletedEventArgs";
}
::windows_core::imp::interface_hierarchy!(
    CoreWebView2NavigationCompletedEventArgs,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for CoreWebView2NavigationCompletedEventArgs {}
unsafe impl ::core::marker::Sync for CoreWebView2NavigationCompletedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct CoreWebView2NavigationStartingEventArgs(::windows_core::IUnknown);
impl CoreWebView2NavigationStartingEventArgs {
    pub fn Uri(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Uri)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn IsUserInitiated(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsUserInitiated)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn IsRedirected(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsRedirected)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn RequestHeaders(&self) -> ::windows_core::Result<CoreWebView2HttpRequestHeaders> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RequestHeaders)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
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
    pub fn NavigationId(&self) -> ::windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NavigationId)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn AdditionalAllowedFrameAncestors(
        &self,
    ) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this =
            &::windows_core::ComInterface::cast::<ICoreWebView2NavigationStartingEventArgs2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AdditionalAllowedFrameAncestors)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetAdditionalAllowedFrameAncestors(
        &self,
        value: &::windows_core::HSTRING,
    ) -> ::windows_core::Result<()> {
        let this =
            &::windows_core::ComInterface::cast::<ICoreWebView2NavigationStartingEventArgs2>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetAdditionalAllowedFrameAncestors)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
}
impl ::windows_core::RuntimeType for CoreWebView2NavigationStartingEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for CoreWebView2NavigationStartingEventArgs {
    type Vtable = ICoreWebView2NavigationStartingEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for CoreWebView2NavigationStartingEventArgs {
    const IID: ::windows_core::GUID =
        <ICoreWebView2NavigationStartingEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for CoreWebView2NavigationStartingEventArgs {
    const NAME: &'static str =
        "Microsoft.Web.WebView2.Core.CoreWebView2NavigationStartingEventArgs";
}
::windows_core::imp::interface_hierarchy!(
    CoreWebView2NavigationStartingEventArgs,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for CoreWebView2NavigationStartingEventArgs {}
unsafe impl ::core::marker::Sync for CoreWebView2NavigationStartingEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct CoreWebView2NewWindowRequestedEventArgs(::windows_core::IUnknown);
impl CoreWebView2NewWindowRequestedEventArgs {
    pub fn Uri(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Uri)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn NewWindow(&self) -> ::windows_core::Result<CoreWebView2> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NewWindow)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetNewWindow<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<CoreWebView2>,
    {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetNewWindow)(
                ::windows_core::Interface::as_raw(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn Handled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Handled)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetHandled)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsUserInitiated(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsUserInitiated)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn WindowFeatures(&self) -> ::windows_core::Result<CoreWebView2WindowFeatures> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).WindowFeatures)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn GetDeferral(&self) -> ::windows_core::Result<::windows::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetDeferral)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn Name(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this =
            &::windows_core::ComInterface::cast::<ICoreWebView2NewWindowRequestedEventArgs2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Name)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for CoreWebView2NewWindowRequestedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for CoreWebView2NewWindowRequestedEventArgs {
    type Vtable = ICoreWebView2NewWindowRequestedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for CoreWebView2NewWindowRequestedEventArgs {
    const IID: ::windows_core::GUID =
        <ICoreWebView2NewWindowRequestedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for CoreWebView2NewWindowRequestedEventArgs {
    const NAME: &'static str =
        "Microsoft.Web.WebView2.Core.CoreWebView2NewWindowRequestedEventArgs";
}
::windows_core::imp::interface_hierarchy!(
    CoreWebView2NewWindowRequestedEventArgs,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for CoreWebView2NewWindowRequestedEventArgs {}
unsafe impl ::core::marker::Sync for CoreWebView2NewWindowRequestedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct CoreWebView2PermissionRequestedEventArgs(::windows_core::IUnknown);
impl CoreWebView2PermissionRequestedEventArgs {
    pub fn Uri(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Uri)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn PermissionKind(&self) -> ::windows_core::Result<CoreWebView2PermissionKind> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PermissionKind)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn IsUserInitiated(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsUserInitiated)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn State(&self) -> ::windows_core::Result<CoreWebView2PermissionState> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).State)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetState(&self, value: CoreWebView2PermissionState) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetState)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn GetDeferral(&self) -> ::windows_core::Result<::windows::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetDeferral)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn Handled(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2PermissionRequestedEventArgs2>(
            self,
        )?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Handled)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2PermissionRequestedEventArgs2>(
            self,
        )?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetHandled)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn SavesInProfile(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2PermissionRequestedEventArgs3>(
            self,
        )?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SavesInProfile)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetSavesInProfile(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2PermissionRequestedEventArgs3>(
            self,
        )?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetSavesInProfile)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
}
impl ::windows_core::RuntimeType for CoreWebView2PermissionRequestedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for CoreWebView2PermissionRequestedEventArgs {
    type Vtable = ICoreWebView2PermissionRequestedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for CoreWebView2PermissionRequestedEventArgs {
    const IID: ::windows_core::GUID =
        <ICoreWebView2PermissionRequestedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for CoreWebView2PermissionRequestedEventArgs {
    const NAME: &'static str =
        "Microsoft.Web.WebView2.Core.CoreWebView2PermissionRequestedEventArgs";
}
::windows_core::imp::interface_hierarchy!(
    CoreWebView2PermissionRequestedEventArgs,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for CoreWebView2PermissionRequestedEventArgs {}
unsafe impl ::core::marker::Sync for CoreWebView2PermissionRequestedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct CoreWebView2PermissionSetting(::windows_core::IUnknown);
impl CoreWebView2PermissionSetting {
    pub fn PermissionKind(&self) -> ::windows_core::Result<CoreWebView2PermissionKind> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PermissionKind)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn PermissionOrigin(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PermissionOrigin)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn PermissionState(&self) -> ::windows_core::Result<CoreWebView2PermissionState> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PermissionState)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for CoreWebView2PermissionSetting {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for CoreWebView2PermissionSetting {
    type Vtable = ICoreWebView2PermissionSetting_Vtbl;
}
unsafe impl ::windows_core::ComInterface for CoreWebView2PermissionSetting {
    const IID: ::windows_core::GUID =
        <ICoreWebView2PermissionSetting as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for CoreWebView2PermissionSetting {
    const NAME: &'static str = "Microsoft.Web.WebView2.Core.CoreWebView2PermissionSetting";
}
::windows_core::imp::interface_hierarchy!(
    CoreWebView2PermissionSetting,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for CoreWebView2PermissionSetting {}
unsafe impl ::core::marker::Sync for CoreWebView2PermissionSetting {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct CoreWebView2PointerInfo(::windows_core::IUnknown);
impl CoreWebView2PointerInfo {
    pub fn PointerKind(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PointerKind)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetPointerKind(&self, value: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetPointerKind)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn PointerId(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PointerId)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetPointerId(&self, value: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetPointerId)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn FrameId(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FrameId)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetFrameId(&self, value: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetFrameId)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn PointerFlags(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PointerFlags)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetPointerFlags(&self, value: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetPointerFlags)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn PointerDeviceRect(&self) -> ::windows_core::Result<::windows::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PointerDeviceRect)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn SetPointerDeviceRect(
        &self,
        value: ::windows::Foundation::Rect,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetPointerDeviceRect)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn DisplayRect(&self) -> ::windows_core::Result<::windows::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DisplayRect)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn SetDisplayRect(&self, value: ::windows::Foundation::Rect) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetDisplayRect)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn PixelLocation(&self) -> ::windows_core::Result<::windows::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PixelLocation)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn SetPixelLocation(
        &self,
        value: ::windows::Foundation::Point,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetPixelLocation)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn HimetricLocation(&self) -> ::windows_core::Result<::windows::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HimetricLocation)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn SetHimetricLocation(
        &self,
        value: ::windows::Foundation::Point,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetHimetricLocation)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn PixelLocationRaw(&self) -> ::windows_core::Result<::windows::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PixelLocationRaw)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn SetPixelLocationRaw(
        &self,
        value: ::windows::Foundation::Point,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetPixelLocationRaw)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn HimetricLocationRaw(&self) -> ::windows_core::Result<::windows::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HimetricLocationRaw)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn SetHimetricLocationRaw(
        &self,
        value: ::windows::Foundation::Point,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetHimetricLocationRaw)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn Time(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Time)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetTime(&self, value: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetTime)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn HistoryCount(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HistoryCount)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetHistoryCount(&self, value: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetHistoryCount)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn InputData(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).InputData)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetInputData(&self, value: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetInputData)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn KeyStates(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).KeyStates)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetKeyStates(&self, value: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetKeyStates)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn PerformanceCount(&self) -> ::windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PerformanceCount)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetPerformanceCount(&self, value: u64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetPerformanceCount)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn ButtonChangeKind(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ButtonChangeKind)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetButtonChangeKind(&self, value: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetButtonChangeKind)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn PenFlags(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PenFlags)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetPenFlags(&self, value: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetPenFlags)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn PenMask(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PenMask)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetPenMask(&self, value: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetPenMask)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn PenPressure(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PenPressure)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetPenPressure(&self, value: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetPenPressure)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn PenRotation(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PenRotation)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetPenRotation(&self, value: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetPenRotation)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn PenTiltX(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PenTiltX)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetPenTiltX(&self, value: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetPenTiltX)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn PenTiltY(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PenTiltY)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetPenTiltY(&self, value: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetPenTiltY)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn TouchFlags(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TouchFlags)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetTouchFlags(&self, value: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetTouchFlags)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn TouchMask(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TouchMask)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetTouchMask(&self, value: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetTouchMask)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn TouchContact(&self) -> ::windows_core::Result<::windows::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TouchContact)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn SetTouchContact(
        &self,
        value: ::windows::Foundation::Rect,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetTouchContact)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn TouchContactRaw(&self) -> ::windows_core::Result<::windows::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TouchContactRaw)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn SetTouchContactRaw(
        &self,
        value: ::windows::Foundation::Rect,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetTouchContactRaw)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn TouchOrientation(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TouchOrientation)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetTouchOrientation(&self, value: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetTouchOrientation)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn TouchPressure(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TouchPressure)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetTouchPressure(&self, value: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetTouchPressure)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
}
impl ::windows_core::RuntimeType for CoreWebView2PointerInfo {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for CoreWebView2PointerInfo {
    type Vtable = ICoreWebView2PointerInfo_Vtbl;
}
unsafe impl ::windows_core::ComInterface for CoreWebView2PointerInfo {
    const IID: ::windows_core::GUID =
        <ICoreWebView2PointerInfo as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for CoreWebView2PointerInfo {
    const NAME: &'static str = "Microsoft.Web.WebView2.Core.CoreWebView2PointerInfo";
}
::windows_core::imp::interface_hierarchy!(
    CoreWebView2PointerInfo,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for CoreWebView2PointerInfo {}
unsafe impl ::core::marker::Sync for CoreWebView2PointerInfo {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct CoreWebView2PrintSettings(::windows_core::IUnknown);
impl CoreWebView2PrintSettings {
    pub fn Orientation(&self) -> ::windows_core::Result<CoreWebView2PrintOrientation> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Orientation)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetOrientation(
        &self,
        value: CoreWebView2PrintOrientation,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetOrientation)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn ScaleFactor(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ScaleFactor)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetScaleFactor(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetScaleFactor)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn PageWidth(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PageWidth)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetPageWidth(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetPageWidth)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn PageHeight(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PageHeight)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetPageHeight(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetPageHeight)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn MarginTop(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MarginTop)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetMarginTop(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetMarginTop)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn MarginBottom(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MarginBottom)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetMarginBottom(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetMarginBottom)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn MarginLeft(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MarginLeft)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetMarginLeft(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetMarginLeft)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn MarginRight(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MarginRight)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetMarginRight(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetMarginRight)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn ShouldPrintBackgrounds(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ShouldPrintBackgrounds)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetShouldPrintBackgrounds(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetShouldPrintBackgrounds)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn ShouldPrintSelectionOnly(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ShouldPrintSelectionOnly)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetShouldPrintSelectionOnly(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetShouldPrintSelectionOnly)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn ShouldPrintHeaderAndFooter(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ShouldPrintHeaderAndFooter)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetShouldPrintHeaderAndFooter(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetShouldPrintHeaderAndFooter)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn HeaderTitle(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HeaderTitle)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetHeaderTitle(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetHeaderTitle)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn FooterUri(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FooterUri)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetFooterUri(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetFooterUri)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn PageRanges(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2PrintSettings2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PageRanges)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetPageRanges(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2PrintSettings2>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetPageRanges)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn PagesPerSide(&self) -> ::windows_core::Result<i32> {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2PrintSettings2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PagesPerSide)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetPagesPerSide(&self, value: i32) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2PrintSettings2>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetPagesPerSide)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn Copies(&self) -> ::windows_core::Result<i32> {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2PrintSettings2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Copies)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetCopies(&self, value: i32) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2PrintSettings2>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetCopies)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn Collation(&self) -> ::windows_core::Result<CoreWebView2PrintCollation> {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2PrintSettings2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Collation)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetCollation(&self, value: CoreWebView2PrintCollation) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2PrintSettings2>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetCollation)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn ColorMode(&self) -> ::windows_core::Result<CoreWebView2PrintColorMode> {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2PrintSettings2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ColorMode)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetColorMode(&self, value: CoreWebView2PrintColorMode) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2PrintSettings2>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetColorMode)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn Duplex(&self) -> ::windows_core::Result<CoreWebView2PrintDuplex> {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2PrintSettings2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Duplex)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetDuplex(&self, value: CoreWebView2PrintDuplex) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2PrintSettings2>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetDuplex)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn MediaSize(&self) -> ::windows_core::Result<CoreWebView2PrintMediaSize> {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2PrintSettings2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MediaSize)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetMediaSize(&self, value: CoreWebView2PrintMediaSize) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2PrintSettings2>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetMediaSize)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn PrinterName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2PrintSettings2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PrinterName)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetPrinterName(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2PrintSettings2>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetPrinterName)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
}
impl ::windows_core::RuntimeType for CoreWebView2PrintSettings {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for CoreWebView2PrintSettings {
    type Vtable = ICoreWebView2PrintSettings_Vtbl;
}
unsafe impl ::windows_core::ComInterface for CoreWebView2PrintSettings {
    const IID: ::windows_core::GUID =
        <ICoreWebView2PrintSettings as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for CoreWebView2PrintSettings {
    const NAME: &'static str = "Microsoft.Web.WebView2.Core.CoreWebView2PrintSettings";
}
::windows_core::imp::interface_hierarchy!(
    CoreWebView2PrintSettings,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for CoreWebView2PrintSettings {}
unsafe impl ::core::marker::Sync for CoreWebView2PrintSettings {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct CoreWebView2ProcessFailedEventArgs(::windows_core::IUnknown);
impl CoreWebView2ProcessFailedEventArgs {
    pub fn ProcessFailedKind(&self) -> ::windows_core::Result<CoreWebView2ProcessFailedKind> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ProcessFailedKind)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn Reason(&self) -> ::windows_core::Result<CoreWebView2ProcessFailedReason> {
        let this =
            &::windows_core::ComInterface::cast::<ICoreWebView2ProcessFailedEventArgs2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Reason)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn ExitCode(&self) -> ::windows_core::Result<i32> {
        let this =
            &::windows_core::ComInterface::cast::<ICoreWebView2ProcessFailedEventArgs2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ExitCode)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn ProcessDescription(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this =
            &::windows_core::ComInterface::cast::<ICoreWebView2ProcessFailedEventArgs2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ProcessDescription)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation_Collections\"`"]
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub fn FrameInfosForFailedProcess(
        &self,
    ) -> ::windows_core::Result<
        ::windows::Foundation::Collections::IVectorView<CoreWebView2FrameInfo>,
    > {
        let this =
            &::windows_core::ComInterface::cast::<ICoreWebView2ProcessFailedEventArgs2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FrameInfosForFailedProcess)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for CoreWebView2ProcessFailedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for CoreWebView2ProcessFailedEventArgs {
    type Vtable = ICoreWebView2ProcessFailedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for CoreWebView2ProcessFailedEventArgs {
    const IID: ::windows_core::GUID =
        <ICoreWebView2ProcessFailedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for CoreWebView2ProcessFailedEventArgs {
    const NAME: &'static str = "Microsoft.Web.WebView2.Core.CoreWebView2ProcessFailedEventArgs";
}
::windows_core::imp::interface_hierarchy!(
    CoreWebView2ProcessFailedEventArgs,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for CoreWebView2ProcessFailedEventArgs {}
unsafe impl ::core::marker::Sync for CoreWebView2ProcessFailedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct CoreWebView2ProcessInfo(::windows_core::IUnknown);
impl CoreWebView2ProcessInfo {
    pub fn ProcessId(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ProcessId)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn Kind(&self) -> ::windows_core::Result<CoreWebView2ProcessKind> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for CoreWebView2ProcessInfo {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for CoreWebView2ProcessInfo {
    type Vtable = ICoreWebView2ProcessInfo_Vtbl;
}
unsafe impl ::windows_core::ComInterface for CoreWebView2ProcessInfo {
    const IID: ::windows_core::GUID =
        <ICoreWebView2ProcessInfo as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for CoreWebView2ProcessInfo {
    const NAME: &'static str = "Microsoft.Web.WebView2.Core.CoreWebView2ProcessInfo";
}
::windows_core::imp::interface_hierarchy!(
    CoreWebView2ProcessInfo,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for CoreWebView2ProcessInfo {}
unsafe impl ::core::marker::Sync for CoreWebView2ProcessInfo {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct CoreWebView2Profile(::windows_core::IUnknown);
impl CoreWebView2Profile {
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn ClearBrowsingDataAsync(
        &self,
        datakinds: CoreWebView2BrowsingDataKinds,
        starttime: ::windows::Foundation::DateTime,
        endtime: ::windows::Foundation::DateTime,
    ) -> ::windows_core::Result<::windows::Foundation::IAsyncAction> {
        let this = &::windows_core::ComInterface::cast::<CoreWebView2Profile_Manual>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ClearBrowsingDataAsync)(
                ::windows_core::Interface::as_raw(this),
                datakinds,
                starttime,
                endtime,
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn ClearBrowsingDataAsync2(
        &self,
    ) -> ::windows_core::Result<::windows::Foundation::IAsyncAction> {
        let this = &::windows_core::ComInterface::cast::<CoreWebView2Profile_Manual>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ClearBrowsingDataAsync2)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation_Collections\"`"]
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub fn GetNonDefaultPermissionSettingsAsync(
        &self,
    ) -> ::windows_core::Result<
        ::windows::Foundation::IAsyncOperation<
            ::windows::Foundation::Collections::IVectorView<CoreWebView2PermissionSetting>,
        >,
    > {
        let this = &::windows_core::ComInterface::cast::<CoreWebView2Profile_Manual2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetNonDefaultPermissionSettingsAsync)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn ProfileName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ProfileName)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn IsInPrivateModeEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsInPrivateModeEnabled)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn ProfilePath(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ProfilePath)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn DefaultDownloadFolderPath(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DefaultDownloadFolderPath)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetDefaultDownloadFolderPath(
        &self,
        value: &::windows_core::HSTRING,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetDefaultDownloadFolderPath)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn PreferredColorScheme(&self) -> ::windows_core::Result<CoreWebView2PreferredColorScheme> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreferredColorScheme)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetPreferredColorScheme(
        &self,
        value: CoreWebView2PreferredColorScheme,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetPreferredColorScheme)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn ClearBrowsingDataAsync2(
        &self,
        datakinds: CoreWebView2BrowsingDataKinds,
    ) -> ::windows_core::Result<::windows::Foundation::IAsyncAction> {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2Profile2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ClearBrowsingDataAsync)(
                ::windows_core::Interface::as_raw(this),
                datakinds,
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn PreferredTrackingPreventionLevel(
        &self,
    ) -> ::windows_core::Result<CoreWebView2TrackingPreventionLevel> {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2Profile3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreferredTrackingPreventionLevel)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetPreferredTrackingPreventionLevel(
        &self,
        value: CoreWebView2TrackingPreventionLevel,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2Profile3>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetPreferredTrackingPreventionLevel)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn SetPermissionStateAsync(
        &self,
        permissionkind: CoreWebView2PermissionKind,
        origin: &::windows_core::HSTRING,
        state: CoreWebView2PermissionState,
    ) -> ::windows_core::Result<::windows::Foundation::IAsyncAction> {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2Profile4>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SetPermissionStateAsync)(
                ::windows_core::Interface::as_raw(this),
                permissionkind,
                ::core::mem::transmute_copy(origin),
                state,
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn CookieManager(&self) -> ::windows_core::Result<CoreWebView2CookieManager> {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2Profile5>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CookieManager)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn IsPasswordAutosaveEnabled(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2Profile6>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsPasswordAutosaveEnabled)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetIsPasswordAutosaveEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2Profile6>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetIsPasswordAutosaveEnabled)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsGeneralAutofillEnabled(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2Profile6>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsGeneralAutofillEnabled)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetIsGeneralAutofillEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2Profile6>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetIsGeneralAutofillEnabled)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
}
impl ::windows_core::RuntimeType for CoreWebView2Profile {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for CoreWebView2Profile {
    type Vtable = ICoreWebView2Profile_Vtbl;
}
unsafe impl ::windows_core::ComInterface for CoreWebView2Profile {
    const IID: ::windows_core::GUID = <ICoreWebView2Profile as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for CoreWebView2Profile {
    const NAME: &'static str = "Microsoft.Web.WebView2.Core.CoreWebView2Profile";
}
::windows_core::imp::interface_hierarchy!(
    CoreWebView2Profile,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for CoreWebView2Profile {}
unsafe impl ::core::marker::Sync for CoreWebView2Profile {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct CoreWebView2ScriptDialogOpeningEventArgs(::windows_core::IUnknown);
impl CoreWebView2ScriptDialogOpeningEventArgs {
    pub fn Uri(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Uri)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn Kind(&self) -> ::windows_core::Result<CoreWebView2ScriptDialogKind> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn Message(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Message)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn DefaultText(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DefaultText)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn ResultText(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ResultText)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetResultText(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetResultText)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn Accept(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).Accept)(::windows_core::Interface::as_raw(
                this,
            ))
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn GetDeferral(&self) -> ::windows_core::Result<::windows::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetDeferral)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for CoreWebView2ScriptDialogOpeningEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for CoreWebView2ScriptDialogOpeningEventArgs {
    type Vtable = ICoreWebView2ScriptDialogOpeningEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for CoreWebView2ScriptDialogOpeningEventArgs {
    const IID: ::windows_core::GUID =
        <ICoreWebView2ScriptDialogOpeningEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for CoreWebView2ScriptDialogOpeningEventArgs {
    const NAME: &'static str =
        "Microsoft.Web.WebView2.Core.CoreWebView2ScriptDialogOpeningEventArgs";
}
::windows_core::imp::interface_hierarchy!(
    CoreWebView2ScriptDialogOpeningEventArgs,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for CoreWebView2ScriptDialogOpeningEventArgs {}
unsafe impl ::core::marker::Sync for CoreWebView2ScriptDialogOpeningEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct CoreWebView2ServerCertificateErrorDetectedEventArgs(::windows_core::IUnknown);
impl CoreWebView2ServerCertificateErrorDetectedEventArgs {
    pub fn ErrorStatus(&self) -> ::windows_core::Result<CoreWebView2WebErrorStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ErrorStatus)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn RequestUri(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RequestUri)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn ServerCertificate(&self) -> ::windows_core::Result<CoreWebView2Certificate> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ServerCertificate)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn Action(&self) -> ::windows_core::Result<CoreWebView2ServerCertificateErrorAction> {
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
    pub fn SetAction(
        &self,
        value: CoreWebView2ServerCertificateErrorAction,
    ) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetAction)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn GetDeferral(&self) -> ::windows_core::Result<::windows::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetDeferral)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for CoreWebView2ServerCertificateErrorDetectedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for CoreWebView2ServerCertificateErrorDetectedEventArgs {
    type Vtable = ICoreWebView2ServerCertificateErrorDetectedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for CoreWebView2ServerCertificateErrorDetectedEventArgs {
    const IID: ::windows_core::GUID =
        <ICoreWebView2ServerCertificateErrorDetectedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for CoreWebView2ServerCertificateErrorDetectedEventArgs {
    const NAME: &'static str =
        "Microsoft.Web.WebView2.Core.CoreWebView2ServerCertificateErrorDetectedEventArgs";
}
::windows_core::imp::interface_hierarchy!(
    CoreWebView2ServerCertificateErrorDetectedEventArgs,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for CoreWebView2ServerCertificateErrorDetectedEventArgs {}
unsafe impl ::core::marker::Sync for CoreWebView2ServerCertificateErrorDetectedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct CoreWebView2Settings(::windows_core::IUnknown);
impl CoreWebView2Settings {
    pub fn IsScriptEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsScriptEnabled)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetIsScriptEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetIsScriptEnabled)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsWebMessageEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsWebMessageEnabled)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetIsWebMessageEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetIsWebMessageEnabled)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn AreDefaultScriptDialogsEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AreDefaultScriptDialogsEnabled)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetAreDefaultScriptDialogsEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetAreDefaultScriptDialogsEnabled)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsStatusBarEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsStatusBarEnabled)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetIsStatusBarEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetIsStatusBarEnabled)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn AreDevToolsEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AreDevToolsEnabled)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetAreDevToolsEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetAreDevToolsEnabled)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn AreDefaultContextMenusEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AreDefaultContextMenusEnabled)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetAreDefaultContextMenusEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetAreDefaultContextMenusEnabled)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn AreHostObjectsAllowed(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AreHostObjectsAllowed)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetAreHostObjectsAllowed(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetAreHostObjectsAllowed)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsZoomControlEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsZoomControlEnabled)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetIsZoomControlEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetIsZoomControlEnabled)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsBuiltInErrorPageEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsBuiltInErrorPageEnabled)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetIsBuiltInErrorPageEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetIsBuiltInErrorPageEnabled)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn UserAgent(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2Settings2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).UserAgent)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetUserAgent(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2Settings2>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetUserAgent)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn AreBrowserAcceleratorKeysEnabled(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2Settings3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AreBrowserAcceleratorKeysEnabled)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetAreBrowserAcceleratorKeysEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2Settings3>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetAreBrowserAcceleratorKeysEnabled)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsPasswordAutosaveEnabled(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2Settings4>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsPasswordAutosaveEnabled)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetIsPasswordAutosaveEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2Settings4>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetIsPasswordAutosaveEnabled)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsGeneralAutofillEnabled(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2Settings4>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsGeneralAutofillEnabled)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetIsGeneralAutofillEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2Settings4>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetIsGeneralAutofillEnabled)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsPinchZoomEnabled(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2Settings5>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsPinchZoomEnabled)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetIsPinchZoomEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2Settings5>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetIsPinchZoomEnabled)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsSwipeNavigationEnabled(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2Settings6>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsSwipeNavigationEnabled)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetIsSwipeNavigationEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2Settings6>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetIsSwipeNavigationEnabled)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn HiddenPdfToolbarItems(&self) -> ::windows_core::Result<CoreWebView2PdfToolbarItems> {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2Settings7>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HiddenPdfToolbarItems)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetHiddenPdfToolbarItems(
        &self,
        value: CoreWebView2PdfToolbarItems,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2Settings7>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetHiddenPdfToolbarItems)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsReputationCheckingRequired(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2Settings8>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsReputationCheckingRequired)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetIsReputationCheckingRequired(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2Settings8>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetIsReputationCheckingRequired)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn HostObjectDispatchAdapter(
        &self,
    ) -> ::windows_core::Result<ICoreWebView2DispatchAdapter> {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2Settings_Manual>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HostObjectDispatchAdapter)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetHostObjectDispatchAdapter<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<ICoreWebView2DispatchAdapter>,
    {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2Settings_Manual>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetHostObjectDispatchAdapter)(
                ::windows_core::Interface::as_raw(this),
                value.try_into_param()?.abi(),
            )
            .ok()
        }
    }
}
impl ::windows_core::RuntimeType for CoreWebView2Settings {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for CoreWebView2Settings {
    type Vtable = ICoreWebView2Settings_Vtbl;
}
unsafe impl ::windows_core::ComInterface for CoreWebView2Settings {
    const IID: ::windows_core::GUID = <ICoreWebView2Settings as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for CoreWebView2Settings {
    const NAME: &'static str = "Microsoft.Web.WebView2.Core.CoreWebView2Settings";
}
::windows_core::imp::interface_hierarchy!(
    CoreWebView2Settings,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for CoreWebView2Settings {}
unsafe impl ::core::marker::Sync for CoreWebView2Settings {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct CoreWebView2SharedBuffer(::windows_core::IUnknown);
impl CoreWebView2SharedBuffer {
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<::windows::Foundation::IClosable>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this))
                .ok()
        }
    }
    pub fn Size(&self) -> ::windows_core::Result<u64> {
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
    #[doc = "Required features: `\"Windows_Storage_Streams\"`"]
    #[cfg(feature = "Windows_Storage_Streams")]
    pub fn OpenStream(
        &self,
    ) -> ::windows_core::Result<::windows::Storage::Streams::IRandomAccessStream> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OpenStream)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn Buffer(&self) -> ::windows_core::Result<::windows::Foundation::IMemoryBufferReference> {
        let this = &::windows_core::ComInterface::cast::<ICoreWebView2SharedBuffer_Manual>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Buffer)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for CoreWebView2SharedBuffer {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for CoreWebView2SharedBuffer {
    type Vtable = ICoreWebView2SharedBuffer_Vtbl;
}
unsafe impl ::windows_core::ComInterface for CoreWebView2SharedBuffer {
    const IID: ::windows_core::GUID =
        <ICoreWebView2SharedBuffer as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for CoreWebView2SharedBuffer {
    const NAME: &'static str = "Microsoft.Web.WebView2.Core.CoreWebView2SharedBuffer";
}
::windows_core::imp::interface_hierarchy!(
    CoreWebView2SharedBuffer,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
#[cfg(feature = "Windows_Foundation")]
impl ::windows_core::CanTryInto<::windows::Foundation::IClosable> for CoreWebView2SharedBuffer {}
unsafe impl ::core::marker::Send for CoreWebView2SharedBuffer {}
unsafe impl ::core::marker::Sync for CoreWebView2SharedBuffer {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct CoreWebView2SourceChangedEventArgs(::windows_core::IUnknown);
impl CoreWebView2SourceChangedEventArgs {
    pub fn IsNewDocument(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsNewDocument)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for CoreWebView2SourceChangedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for CoreWebView2SourceChangedEventArgs {
    type Vtable = ICoreWebView2SourceChangedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for CoreWebView2SourceChangedEventArgs {
    const IID: ::windows_core::GUID =
        <ICoreWebView2SourceChangedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for CoreWebView2SourceChangedEventArgs {
    const NAME: &'static str = "Microsoft.Web.WebView2.Core.CoreWebView2SourceChangedEventArgs";
}
::windows_core::imp::interface_hierarchy!(
    CoreWebView2SourceChangedEventArgs,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for CoreWebView2SourceChangedEventArgs {}
unsafe impl ::core::marker::Sync for CoreWebView2SourceChangedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct CoreWebView2WebMessageReceivedEventArgs(::windows_core::IUnknown);
impl CoreWebView2WebMessageReceivedEventArgs {
    pub fn Source(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
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
    pub fn WebMessageAsJson(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).WebMessageAsJson)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn TryGetWebMessageAsString(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TryGetWebMessageAsString)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation_Collections\"`"]
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub fn AdditionalObjects(
        &self,
    ) -> ::windows_core::Result<
        ::windows::Foundation::Collections::IVectorView<::windows_core::IInspectable>,
    > {
        let this =
            &::windows_core::ComInterface::cast::<ICoreWebView2WebMessageReceivedEventArgs2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AdditionalObjects)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for CoreWebView2WebMessageReceivedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for CoreWebView2WebMessageReceivedEventArgs {
    type Vtable = ICoreWebView2WebMessageReceivedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for CoreWebView2WebMessageReceivedEventArgs {
    const IID: ::windows_core::GUID =
        <ICoreWebView2WebMessageReceivedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for CoreWebView2WebMessageReceivedEventArgs {
    const NAME: &'static str =
        "Microsoft.Web.WebView2.Core.CoreWebView2WebMessageReceivedEventArgs";
}
::windows_core::imp::interface_hierarchy!(
    CoreWebView2WebMessageReceivedEventArgs,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for CoreWebView2WebMessageReceivedEventArgs {}
unsafe impl ::core::marker::Sync for CoreWebView2WebMessageReceivedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct CoreWebView2WebResourceRequest(::windows_core::IUnknown);
impl CoreWebView2WebResourceRequest {
    pub fn Uri(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Uri)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetUri(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetUri)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn Method(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Method)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetMethod(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetMethod)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    #[doc = "Required features: `\"Windows_Storage_Streams\"`"]
    #[cfg(feature = "Windows_Storage_Streams")]
    pub fn Content(
        &self,
    ) -> ::windows_core::Result<::windows::Storage::Streams::IRandomAccessStream> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Content)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Storage_Streams\"`"]
    #[cfg(feature = "Windows_Storage_Streams")]
    pub fn SetContent<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<::windows::Storage::Streams::IRandomAccessStream>,
    {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetContent)(
                ::windows_core::Interface::as_raw(this),
                value.try_into_param()?.abi(),
            )
            .ok()
        }
    }
    pub fn Headers(&self) -> ::windows_core::Result<CoreWebView2HttpRequestHeaders> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Headers)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for CoreWebView2WebResourceRequest {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for CoreWebView2WebResourceRequest {
    type Vtable = ICoreWebView2WebResourceRequest_Vtbl;
}
unsafe impl ::windows_core::ComInterface for CoreWebView2WebResourceRequest {
    const IID: ::windows_core::GUID =
        <ICoreWebView2WebResourceRequest as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for CoreWebView2WebResourceRequest {
    const NAME: &'static str = "Microsoft.Web.WebView2.Core.CoreWebView2WebResourceRequest";
}
::windows_core::imp::interface_hierarchy!(
    CoreWebView2WebResourceRequest,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for CoreWebView2WebResourceRequest {}
unsafe impl ::core::marker::Sync for CoreWebView2WebResourceRequest {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct CoreWebView2WebResourceRequestedEventArgs(::windows_core::IUnknown);
impl CoreWebView2WebResourceRequestedEventArgs {
    pub fn Request(&self) -> ::windows_core::Result<CoreWebView2WebResourceRequest> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Request)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn Response(&self) -> ::windows_core::Result<CoreWebView2WebResourceResponse> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Response)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetResponse<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<CoreWebView2WebResourceResponse>,
    {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetResponse)(
                ::windows_core::Interface::as_raw(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn ResourceContext(&self) -> ::windows_core::Result<CoreWebView2WebResourceContext> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ResourceContext)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`"]
    #[cfg(feature = "Windows_Foundation")]
    pub fn GetDeferral(&self) -> ::windows_core::Result<::windows::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetDeferral)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for CoreWebView2WebResourceRequestedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for CoreWebView2WebResourceRequestedEventArgs {
    type Vtable = ICoreWebView2WebResourceRequestedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for CoreWebView2WebResourceRequestedEventArgs {
    const IID: ::windows_core::GUID =
        <ICoreWebView2WebResourceRequestedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for CoreWebView2WebResourceRequestedEventArgs {
    const NAME: &'static str =
        "Microsoft.Web.WebView2.Core.CoreWebView2WebResourceRequestedEventArgs";
}
::windows_core::imp::interface_hierarchy!(
    CoreWebView2WebResourceRequestedEventArgs,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for CoreWebView2WebResourceRequestedEventArgs {}
unsafe impl ::core::marker::Sync for CoreWebView2WebResourceRequestedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct CoreWebView2WebResourceResponse(::windows_core::IUnknown);
impl CoreWebView2WebResourceResponse {
    #[doc = "Required features: `\"Windows_Storage_Streams\"`"]
    #[cfg(feature = "Windows_Storage_Streams")]
    pub fn Content(
        &self,
    ) -> ::windows_core::Result<::windows::Storage::Streams::IRandomAccessStream> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Content)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Storage_Streams\"`"]
    #[cfg(feature = "Windows_Storage_Streams")]
    pub fn SetContent<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<::windows::Storage::Streams::IRandomAccessStream>,
    {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetContent)(
                ::windows_core::Interface::as_raw(this),
                value.try_into_param()?.abi(),
            )
            .ok()
        }
    }
    pub fn Headers(&self) -> ::windows_core::Result<CoreWebView2HttpResponseHeaders> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Headers)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn StatusCode(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StatusCode)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetStatusCode(&self, value: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetStatusCode)(
                ::windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn ReasonPhrase(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ReasonPhrase)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetReasonPhrase(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe {
            (::windows_core::Interface::vtable(this).SetReasonPhrase)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
}
impl ::windows_core::RuntimeType for CoreWebView2WebResourceResponse {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for CoreWebView2WebResourceResponse {
    type Vtable = ICoreWebView2WebResourceResponse_Vtbl;
}
unsafe impl ::windows_core::ComInterface for CoreWebView2WebResourceResponse {
    const IID: ::windows_core::GUID =
        <ICoreWebView2WebResourceResponse as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for CoreWebView2WebResourceResponse {
    const NAME: &'static str = "Microsoft.Web.WebView2.Core.CoreWebView2WebResourceResponse";
}
::windows_core::imp::interface_hierarchy!(
    CoreWebView2WebResourceResponse,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for CoreWebView2WebResourceResponse {}
unsafe impl ::core::marker::Sync for CoreWebView2WebResourceResponse {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct CoreWebView2WebResourceResponseReceivedEventArgs(::windows_core::IUnknown);
impl CoreWebView2WebResourceResponseReceivedEventArgs {
    pub fn Request(&self) -> ::windows_core::Result<CoreWebView2WebResourceRequest> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Request)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn Response(&self) -> ::windows_core::Result<CoreWebView2WebResourceResponseView> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Response)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for CoreWebView2WebResourceResponseReceivedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for CoreWebView2WebResourceResponseReceivedEventArgs {
    type Vtable = ICoreWebView2WebResourceResponseReceivedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for CoreWebView2WebResourceResponseReceivedEventArgs {
    const IID: ::windows_core::GUID =
        <ICoreWebView2WebResourceResponseReceivedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for CoreWebView2WebResourceResponseReceivedEventArgs {
    const NAME: &'static str =
        "Microsoft.Web.WebView2.Core.CoreWebView2WebResourceResponseReceivedEventArgs";
}
::windows_core::imp::interface_hierarchy!(
    CoreWebView2WebResourceResponseReceivedEventArgs,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for CoreWebView2WebResourceResponseReceivedEventArgs {}
unsafe impl ::core::marker::Sync for CoreWebView2WebResourceResponseReceivedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct CoreWebView2WebResourceResponseView(::windows_core::IUnknown);
impl CoreWebView2WebResourceResponseView {
    pub fn Headers(&self) -> ::windows_core::Result<CoreWebView2HttpResponseHeaders> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Headers)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn StatusCode(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StatusCode)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn ReasonPhrase(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ReasonPhrase)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation\"`, `\"Windows_Storage_Streams\"`"]
    #[cfg(all(feature = "Windows_Foundation", feature = "Windows_Storage_Streams"))]
    pub fn GetContentAsync(
        &self,
    ) -> ::windows_core::Result<
        ::windows::Foundation::IAsyncOperation<::windows::Storage::Streams::IRandomAccessStream>,
    > {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetContentAsync)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for CoreWebView2WebResourceResponseView {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for CoreWebView2WebResourceResponseView {
    type Vtable = ICoreWebView2WebResourceResponseView_Vtbl;
}
unsafe impl ::windows_core::ComInterface for CoreWebView2WebResourceResponseView {
    const IID: ::windows_core::GUID =
        <ICoreWebView2WebResourceResponseView as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for CoreWebView2WebResourceResponseView {
    const NAME: &'static str = "Microsoft.Web.WebView2.Core.CoreWebView2WebResourceResponseView";
}
::windows_core::imp::interface_hierarchy!(
    CoreWebView2WebResourceResponseView,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for CoreWebView2WebResourceResponseView {}
unsafe impl ::core::marker::Sync for CoreWebView2WebResourceResponseView {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct CoreWebView2WindowFeatures(::windows_core::IUnknown);
impl CoreWebView2WindowFeatures {
    pub fn HasPosition(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HasPosition)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn HasSize(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HasSize)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn Left(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Left)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn Top(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Top)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn Height(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Height)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn Width(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Width)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn ShouldDisplayMenuBar(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ShouldDisplayMenuBar)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn ShouldDisplayStatus(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ShouldDisplayStatus)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn ShouldDisplayToolbar(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ShouldDisplayToolbar)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn ShouldDisplayScrollBars(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ShouldDisplayScrollBars)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for CoreWebView2WindowFeatures {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for CoreWebView2WindowFeatures {
    type Vtable = ICoreWebView2WindowFeatures_Vtbl;
}
unsafe impl ::windows_core::ComInterface for CoreWebView2WindowFeatures {
    const IID: ::windows_core::GUID =
        <ICoreWebView2WindowFeatures as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for CoreWebView2WindowFeatures {
    const NAME: &'static str = "Microsoft.Web.WebView2.Core.CoreWebView2WindowFeatures";
}
::windows_core::imp::interface_hierarchy!(
    CoreWebView2WindowFeatures,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
unsafe impl ::core::marker::Send for CoreWebView2WindowFeatures {}
unsafe impl ::core::marker::Sync for CoreWebView2WindowFeatures {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CoreWebView2BoundsMode(pub i32);
impl CoreWebView2BoundsMode {
    pub const UseRawPixels: Self = Self(0i32);
    pub const UseRasterizationScale: Self = Self(1i32);
}
impl ::core::marker::Copy for CoreWebView2BoundsMode {}
impl ::core::clone::Clone for CoreWebView2BoundsMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CoreWebView2BoundsMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for CoreWebView2BoundsMode {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for CoreWebView2BoundsMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreWebView2BoundsMode").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for CoreWebView2BoundsMode {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::from_slice(
            b"enum(Microsoft.Web.WebView2.Core.CoreWebView2BoundsMode;i4)",
        );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CoreWebView2BrowserProcessExitKind(pub i32);
impl CoreWebView2BrowserProcessExitKind {
    pub const Normal: Self = Self(0i32);
    pub const Failed: Self = Self(1i32);
}
impl ::core::marker::Copy for CoreWebView2BrowserProcessExitKind {}
impl ::core::clone::Clone for CoreWebView2BrowserProcessExitKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CoreWebView2BrowserProcessExitKind {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for CoreWebView2BrowserProcessExitKind {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for CoreWebView2BrowserProcessExitKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreWebView2BrowserProcessExitKind").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for CoreWebView2BrowserProcessExitKind {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::from_slice(
            b"enum(Microsoft.Web.WebView2.Core.CoreWebView2BrowserProcessExitKind;i4)",
        );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CoreWebView2BrowsingDataKinds(pub u32);
impl CoreWebView2BrowsingDataKinds {
    pub const FileSystems: Self = Self(1u32);
    pub const IndexedDb: Self = Self(2u32);
    pub const LocalStorage: Self = Self(4u32);
    pub const WebSql: Self = Self(8u32);
    pub const CacheStorage: Self = Self(16u32);
    pub const AllDomStorage: Self = Self(32u32);
    pub const Cookies: Self = Self(64u32);
    pub const AllSite: Self = Self(128u32);
    pub const DiskCache: Self = Self(256u32);
    pub const DownloadHistory: Self = Self(512u32);
    pub const GeneralAutofill: Self = Self(1024u32);
    pub const PasswordAutosave: Self = Self(2048u32);
    pub const BrowsingHistory: Self = Self(4096u32);
    pub const Settings: Self = Self(8192u32);
    pub const AllProfile: Self = Self(16384u32);
}
impl ::core::marker::Copy for CoreWebView2BrowsingDataKinds {}
impl ::core::clone::Clone for CoreWebView2BrowsingDataKinds {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CoreWebView2BrowsingDataKinds {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for CoreWebView2BrowsingDataKinds {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for CoreWebView2BrowsingDataKinds {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreWebView2BrowsingDataKinds").field(&self.0).finish()
    }
}
impl CoreWebView2BrowsingDataKinds {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for CoreWebView2BrowsingDataKinds {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CoreWebView2BrowsingDataKinds {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CoreWebView2BrowsingDataKinds {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CoreWebView2BrowsingDataKinds {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CoreWebView2BrowsingDataKinds {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::windows_core::RuntimeType for CoreWebView2BrowsingDataKinds {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::from_slice(
            b"enum(Microsoft.Web.WebView2.Core.CoreWebView2BrowsingDataKinds;u4)",
        );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CoreWebView2CapturePreviewImageFormat(pub i32);
impl CoreWebView2CapturePreviewImageFormat {
    pub const Png: Self = Self(0i32);
    pub const Jpeg: Self = Self(1i32);
}
impl ::core::marker::Copy for CoreWebView2CapturePreviewImageFormat {}
impl ::core::clone::Clone for CoreWebView2CapturePreviewImageFormat {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CoreWebView2CapturePreviewImageFormat {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for CoreWebView2CapturePreviewImageFormat {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for CoreWebView2CapturePreviewImageFormat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreWebView2CapturePreviewImageFormat").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for CoreWebView2CapturePreviewImageFormat {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::from_slice(
            b"enum(Microsoft.Web.WebView2.Core.CoreWebView2CapturePreviewImageFormat;i4)",
        );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CoreWebView2ClientCertificateKind(pub i32);
impl CoreWebView2ClientCertificateKind {
    pub const SmartCard: Self = Self(0i32);
    pub const Pin: Self = Self(1i32);
    pub const Other: Self = Self(2i32);
}
impl ::core::marker::Copy for CoreWebView2ClientCertificateKind {}
impl ::core::clone::Clone for CoreWebView2ClientCertificateKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CoreWebView2ClientCertificateKind {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for CoreWebView2ClientCertificateKind {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for CoreWebView2ClientCertificateKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreWebView2ClientCertificateKind").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for CoreWebView2ClientCertificateKind {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::from_slice(
            b"enum(Microsoft.Web.WebView2.Core.CoreWebView2ClientCertificateKind;i4)",
        );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CoreWebView2ContextMenuItemKind(pub i32);
impl CoreWebView2ContextMenuItemKind {
    pub const Command: Self = Self(0i32);
    pub const CheckBox: Self = Self(1i32);
    pub const Radio: Self = Self(2i32);
    pub const Separator: Self = Self(3i32);
    pub const Submenu: Self = Self(4i32);
}
impl ::core::marker::Copy for CoreWebView2ContextMenuItemKind {}
impl ::core::clone::Clone for CoreWebView2ContextMenuItemKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CoreWebView2ContextMenuItemKind {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for CoreWebView2ContextMenuItemKind {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for CoreWebView2ContextMenuItemKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreWebView2ContextMenuItemKind").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for CoreWebView2ContextMenuItemKind {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::from_slice(
            b"enum(Microsoft.Web.WebView2.Core.CoreWebView2ContextMenuItemKind;i4)",
        );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CoreWebView2ContextMenuTargetKind(pub i32);
impl CoreWebView2ContextMenuTargetKind {
    pub const Page: Self = Self(0i32);
    pub const Image: Self = Self(1i32);
    pub const SelectedText: Self = Self(2i32);
    pub const Audio: Self = Self(3i32);
    pub const Video: Self = Self(4i32);
}
impl ::core::marker::Copy for CoreWebView2ContextMenuTargetKind {}
impl ::core::clone::Clone for CoreWebView2ContextMenuTargetKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CoreWebView2ContextMenuTargetKind {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for CoreWebView2ContextMenuTargetKind {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for CoreWebView2ContextMenuTargetKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreWebView2ContextMenuTargetKind").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for CoreWebView2ContextMenuTargetKind {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::from_slice(
            b"enum(Microsoft.Web.WebView2.Core.CoreWebView2ContextMenuTargetKind;i4)",
        );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CoreWebView2CookieSameSiteKind(pub i32);
impl CoreWebView2CookieSameSiteKind {
    pub const None: Self = Self(0i32);
    pub const Lax: Self = Self(1i32);
    pub const Strict: Self = Self(2i32);
}
impl ::core::marker::Copy for CoreWebView2CookieSameSiteKind {}
impl ::core::clone::Clone for CoreWebView2CookieSameSiteKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CoreWebView2CookieSameSiteKind {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for CoreWebView2CookieSameSiteKind {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for CoreWebView2CookieSameSiteKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreWebView2CookieSameSiteKind").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for CoreWebView2CookieSameSiteKind {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::from_slice(
            b"enum(Microsoft.Web.WebView2.Core.CoreWebView2CookieSameSiteKind;i4)",
        );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CoreWebView2DefaultDownloadDialogCornerAlignment(pub i32);
impl CoreWebView2DefaultDownloadDialogCornerAlignment {
    pub const TopLeft: Self = Self(0i32);
    pub const TopRight: Self = Self(1i32);
    pub const BottomLeft: Self = Self(2i32);
    pub const BottomRight: Self = Self(3i32);
}
impl ::core::marker::Copy for CoreWebView2DefaultDownloadDialogCornerAlignment {}
impl ::core::clone::Clone for CoreWebView2DefaultDownloadDialogCornerAlignment {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CoreWebView2DefaultDownloadDialogCornerAlignment {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for CoreWebView2DefaultDownloadDialogCornerAlignment {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for CoreWebView2DefaultDownloadDialogCornerAlignment {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreWebView2DefaultDownloadDialogCornerAlignment")
            .field(&self.0)
            .finish()
    }
}
impl ::windows_core::RuntimeType for CoreWebView2DefaultDownloadDialogCornerAlignment {
    const SIGNATURE : ::windows_core::imp::ConstBuffer =::windows_core::imp::ConstBuffer::from_slice ( b"enum(Microsoft.Web.WebView2.Core.CoreWebView2DefaultDownloadDialogCornerAlignment;i4)" ) ;
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CoreWebView2DownloadInterruptReason(pub i32);
impl CoreWebView2DownloadInterruptReason {
    pub const None: Self = Self(0i32);
    pub const FileFailed: Self = Self(1i32);
    pub const FileAccessDenied: Self = Self(2i32);
    pub const FileNoSpace: Self = Self(3i32);
    pub const FileNameTooLong: Self = Self(4i32);
    pub const FileTooLarge: Self = Self(5i32);
    pub const FileMalicious: Self = Self(6i32);
    pub const FileTransientError: Self = Self(7i32);
    pub const FileBlockedByPolicy: Self = Self(8i32);
    pub const FileSecurityCheckFailed: Self = Self(9i32);
    pub const FileTooShort: Self = Self(10i32);
    pub const FileHashMismatch: Self = Self(11i32);
    pub const NetworkFailed: Self = Self(12i32);
    pub const NetworkTimeout: Self = Self(13i32);
    pub const NetworkDisconnected: Self = Self(14i32);
    pub const NetworkServerDown: Self = Self(15i32);
    pub const NetworkInvalidRequest: Self = Self(16i32);
    pub const ServerFailed: Self = Self(17i32);
    pub const ServerNoRange: Self = Self(18i32);
    pub const ServerBadContent: Self = Self(19i32);
    pub const ServerUnauthorized: Self = Self(20i32);
    pub const ServerCertificateProblem: Self = Self(21i32);
    pub const ServerForbidden: Self = Self(22i32);
    pub const ServerUnexpectedResponse: Self = Self(23i32);
    pub const ServerContentLengthMismatch: Self = Self(24i32);
    pub const ServerCrossOriginRedirect: Self = Self(25i32);
    pub const UserCanceled: Self = Self(26i32);
    pub const UserShutdown: Self = Self(27i32);
    pub const UserPaused: Self = Self(28i32);
    pub const DownloadProcessCrashed: Self = Self(29i32);
}
impl ::core::marker::Copy for CoreWebView2DownloadInterruptReason {}
impl ::core::clone::Clone for CoreWebView2DownloadInterruptReason {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CoreWebView2DownloadInterruptReason {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for CoreWebView2DownloadInterruptReason {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for CoreWebView2DownloadInterruptReason {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreWebView2DownloadInterruptReason").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for CoreWebView2DownloadInterruptReason {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::from_slice(
            b"enum(Microsoft.Web.WebView2.Core.CoreWebView2DownloadInterruptReason;i4)",
        );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CoreWebView2DownloadState(pub i32);
impl CoreWebView2DownloadState {
    pub const InProgress: Self = Self(0i32);
    pub const Interrupted: Self = Self(1i32);
    pub const Completed: Self = Self(2i32);
}
impl ::core::marker::Copy for CoreWebView2DownloadState {}
impl ::core::clone::Clone for CoreWebView2DownloadState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CoreWebView2DownloadState {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for CoreWebView2DownloadState {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for CoreWebView2DownloadState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreWebView2DownloadState").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for CoreWebView2DownloadState {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::from_slice(
            b"enum(Microsoft.Web.WebView2.Core.CoreWebView2DownloadState;i4)",
        );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CoreWebView2FaviconImageFormat(pub i32);
impl CoreWebView2FaviconImageFormat {
    pub const Png: Self = Self(0i32);
    pub const Jpeg: Self = Self(1i32);
}
impl ::core::marker::Copy for CoreWebView2FaviconImageFormat {}
impl ::core::clone::Clone for CoreWebView2FaviconImageFormat {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CoreWebView2FaviconImageFormat {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for CoreWebView2FaviconImageFormat {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for CoreWebView2FaviconImageFormat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreWebView2FaviconImageFormat").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for CoreWebView2FaviconImageFormat {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::from_slice(
            b"enum(Microsoft.Web.WebView2.Core.CoreWebView2FaviconImageFormat;i4)",
        );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CoreWebView2HostResourceAccessKind(pub i32);
impl CoreWebView2HostResourceAccessKind {
    pub const Deny: Self = Self(0i32);
    pub const Allow: Self = Self(1i32);
    pub const DenyCors: Self = Self(2i32);
}
impl ::core::marker::Copy for CoreWebView2HostResourceAccessKind {}
impl ::core::clone::Clone for CoreWebView2HostResourceAccessKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CoreWebView2HostResourceAccessKind {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for CoreWebView2HostResourceAccessKind {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for CoreWebView2HostResourceAccessKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreWebView2HostResourceAccessKind").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for CoreWebView2HostResourceAccessKind {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::from_slice(
            b"enum(Microsoft.Web.WebView2.Core.CoreWebView2HostResourceAccessKind;i4)",
        );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CoreWebView2KeyEventKind(pub i32);
impl CoreWebView2KeyEventKind {
    pub const KeyDown: Self = Self(0i32);
    pub const KeyUp: Self = Self(1i32);
    pub const SystemKeyDown: Self = Self(2i32);
    pub const SystemKeyUp: Self = Self(3i32);
}
impl ::core::marker::Copy for CoreWebView2KeyEventKind {}
impl ::core::clone::Clone for CoreWebView2KeyEventKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CoreWebView2KeyEventKind {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for CoreWebView2KeyEventKind {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for CoreWebView2KeyEventKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreWebView2KeyEventKind").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for CoreWebView2KeyEventKind {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::from_slice(
            b"enum(Microsoft.Web.WebView2.Core.CoreWebView2KeyEventKind;i4)",
        );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CoreWebView2MemoryUsageTargetLevel(pub i32);
impl CoreWebView2MemoryUsageTargetLevel {
    pub const Normal: Self = Self(0i32);
    pub const Low: Self = Self(1i32);
}
impl ::core::marker::Copy for CoreWebView2MemoryUsageTargetLevel {}
impl ::core::clone::Clone for CoreWebView2MemoryUsageTargetLevel {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CoreWebView2MemoryUsageTargetLevel {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for CoreWebView2MemoryUsageTargetLevel {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for CoreWebView2MemoryUsageTargetLevel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreWebView2MemoryUsageTargetLevel").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for CoreWebView2MemoryUsageTargetLevel {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::from_slice(
            b"enum(Microsoft.Web.WebView2.Core.CoreWebView2MemoryUsageTargetLevel;i4)",
        );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CoreWebView2MouseEventKind(pub i32);
impl CoreWebView2MouseEventKind {
    pub const HorizontalWheel: Self = Self(526i32);
    pub const LeftButtonDoubleClick: Self = Self(515i32);
    pub const LeftButtonDown: Self = Self(513i32);
    pub const LeftButtonUp: Self = Self(514i32);
    pub const Leave: Self = Self(675i32);
    pub const MiddleButtonDoubleClick: Self = Self(521i32);
    pub const MiddleButtonDown: Self = Self(519i32);
    pub const MiddleButtonUp: Self = Self(520i32);
    pub const Move: Self = Self(512i32);
    pub const RightButtonDoubleClick: Self = Self(518i32);
    pub const RightButtonDown: Self = Self(516i32);
    pub const RightButtonUp: Self = Self(517i32);
    pub const Wheel: Self = Self(522i32);
    pub const XButtonDoubleClick: Self = Self(525i32);
    pub const XButtonDown: Self = Self(523i32);
    pub const XButtonUp: Self = Self(524i32);
}
impl ::core::marker::Copy for CoreWebView2MouseEventKind {}
impl ::core::clone::Clone for CoreWebView2MouseEventKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CoreWebView2MouseEventKind {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for CoreWebView2MouseEventKind {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for CoreWebView2MouseEventKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreWebView2MouseEventKind").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for CoreWebView2MouseEventKind {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::from_slice(
            b"enum(Microsoft.Web.WebView2.Core.CoreWebView2MouseEventKind;i4)",
        );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CoreWebView2MouseEventVirtualKeys(pub u32);
impl CoreWebView2MouseEventVirtualKeys {
    pub const None: Self = Self(0u32);
    pub const LeftButton: Self = Self(1u32);
    pub const RightButton: Self = Self(2u32);
    pub const Shift: Self = Self(4u32);
    pub const Control: Self = Self(8u32);
    pub const MiddleButton: Self = Self(16u32);
    pub const XButton1: Self = Self(32u32);
    pub const XButton2: Self = Self(64u32);
}
impl ::core::marker::Copy for CoreWebView2MouseEventVirtualKeys {}
impl ::core::clone::Clone for CoreWebView2MouseEventVirtualKeys {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CoreWebView2MouseEventVirtualKeys {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for CoreWebView2MouseEventVirtualKeys {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for CoreWebView2MouseEventVirtualKeys {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreWebView2MouseEventVirtualKeys").field(&self.0).finish()
    }
}
impl CoreWebView2MouseEventVirtualKeys {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for CoreWebView2MouseEventVirtualKeys {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CoreWebView2MouseEventVirtualKeys {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CoreWebView2MouseEventVirtualKeys {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CoreWebView2MouseEventVirtualKeys {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CoreWebView2MouseEventVirtualKeys {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::windows_core::RuntimeType for CoreWebView2MouseEventVirtualKeys {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::from_slice(
            b"enum(Microsoft.Web.WebView2.Core.CoreWebView2MouseEventVirtualKeys;u4)",
        );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CoreWebView2MoveFocusReason(pub i32);
impl CoreWebView2MoveFocusReason {
    pub const Programmatic: Self = Self(0i32);
    pub const Next: Self = Self(1i32);
    pub const Previous: Self = Self(2i32);
}
impl ::core::marker::Copy for CoreWebView2MoveFocusReason {}
impl ::core::clone::Clone for CoreWebView2MoveFocusReason {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CoreWebView2MoveFocusReason {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for CoreWebView2MoveFocusReason {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for CoreWebView2MoveFocusReason {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreWebView2MoveFocusReason").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for CoreWebView2MoveFocusReason {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::from_slice(
            b"enum(Microsoft.Web.WebView2.Core.CoreWebView2MoveFocusReason;i4)",
        );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CoreWebView2PdfToolbarItems(pub u32);
impl CoreWebView2PdfToolbarItems {
    pub const None: Self = Self(0u32);
    pub const Save: Self = Self(1u32);
    pub const Print: Self = Self(2u32);
    pub const SaveAs: Self = Self(4u32);
    pub const ZoomIn: Self = Self(8u32);
    pub const ZoomOut: Self = Self(16u32);
    pub const Rotate: Self = Self(32u32);
    pub const FitPage: Self = Self(64u32);
    pub const PageLayout: Self = Self(128u32);
    pub const Bookmarks: Self = Self(256u32);
    pub const PageSelector: Self = Self(512u32);
    pub const Search: Self = Self(1024u32);
    pub const FullScreen: Self = Self(2048u32);
    pub const MoreSettings: Self = Self(4096u32);
}
impl ::core::marker::Copy for CoreWebView2PdfToolbarItems {}
impl ::core::clone::Clone for CoreWebView2PdfToolbarItems {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CoreWebView2PdfToolbarItems {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for CoreWebView2PdfToolbarItems {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for CoreWebView2PdfToolbarItems {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreWebView2PdfToolbarItems").field(&self.0).finish()
    }
}
impl CoreWebView2PdfToolbarItems {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for CoreWebView2PdfToolbarItems {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CoreWebView2PdfToolbarItems {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CoreWebView2PdfToolbarItems {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CoreWebView2PdfToolbarItems {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CoreWebView2PdfToolbarItems {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::windows_core::RuntimeType for CoreWebView2PdfToolbarItems {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::from_slice(
            b"enum(Microsoft.Web.WebView2.Core.CoreWebView2PdfToolbarItems;u4)",
        );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CoreWebView2PermissionKind(pub i32);
impl CoreWebView2PermissionKind {
    pub const UnknownPermission: Self = Self(0i32);
    pub const Microphone: Self = Self(1i32);
    pub const Camera: Self = Self(2i32);
    pub const Geolocation: Self = Self(3i32);
    pub const Notifications: Self = Self(4i32);
    pub const OtherSensors: Self = Self(5i32);
    pub const ClipboardRead: Self = Self(6i32);
    pub const MultipleAutomaticDownloads: Self = Self(7i32);
    pub const FileReadWrite: Self = Self(8i32);
    pub const Autoplay: Self = Self(9i32);
    pub const LocalFonts: Self = Self(10i32);
    pub const MidiSystemExclusiveMessages: Self = Self(11i32);
    pub const WindowManagement: Self = Self(12i32);
}
impl ::core::marker::Copy for CoreWebView2PermissionKind {}
impl ::core::clone::Clone for CoreWebView2PermissionKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CoreWebView2PermissionKind {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for CoreWebView2PermissionKind {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for CoreWebView2PermissionKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreWebView2PermissionKind").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for CoreWebView2PermissionKind {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::from_slice(
            b"enum(Microsoft.Web.WebView2.Core.CoreWebView2PermissionKind;i4)",
        );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CoreWebView2PermissionState(pub i32);
impl CoreWebView2PermissionState {
    pub const Default: Self = Self(0i32);
    pub const Allow: Self = Self(1i32);
    pub const Deny: Self = Self(2i32);
}
impl ::core::marker::Copy for CoreWebView2PermissionState {}
impl ::core::clone::Clone for CoreWebView2PermissionState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CoreWebView2PermissionState {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for CoreWebView2PermissionState {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for CoreWebView2PermissionState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreWebView2PermissionState").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for CoreWebView2PermissionState {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::from_slice(
            b"enum(Microsoft.Web.WebView2.Core.CoreWebView2PermissionState;i4)",
        );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CoreWebView2PointerEventKind(pub i32);
impl CoreWebView2PointerEventKind {
    pub const Activate: Self = Self(587i32);
    pub const Down: Self = Self(582i32);
    pub const Enter: Self = Self(585i32);
    pub const Leave: Self = Self(586i32);
    pub const Up: Self = Self(583i32);
    pub const Update: Self = Self(581i32);
}
impl ::core::marker::Copy for CoreWebView2PointerEventKind {}
impl ::core::clone::Clone for CoreWebView2PointerEventKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CoreWebView2PointerEventKind {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for CoreWebView2PointerEventKind {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for CoreWebView2PointerEventKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreWebView2PointerEventKind").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for CoreWebView2PointerEventKind {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::from_slice(
            b"enum(Microsoft.Web.WebView2.Core.CoreWebView2PointerEventKind;i4)",
        );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CoreWebView2PreferredColorScheme(pub i32);
impl CoreWebView2PreferredColorScheme {
    pub const Auto: Self = Self(0i32);
    pub const Light: Self = Self(1i32);
    pub const Dark: Self = Self(2i32);
}
impl ::core::marker::Copy for CoreWebView2PreferredColorScheme {}
impl ::core::clone::Clone for CoreWebView2PreferredColorScheme {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CoreWebView2PreferredColorScheme {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for CoreWebView2PreferredColorScheme {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for CoreWebView2PreferredColorScheme {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreWebView2PreferredColorScheme").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for CoreWebView2PreferredColorScheme {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::from_slice(
            b"enum(Microsoft.Web.WebView2.Core.CoreWebView2PreferredColorScheme;i4)",
        );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CoreWebView2PrintCollation(pub i32);
impl CoreWebView2PrintCollation {
    pub const Default: Self = Self(0i32);
    pub const Collated: Self = Self(1i32);
    pub const Uncollated: Self = Self(2i32);
}
impl ::core::marker::Copy for CoreWebView2PrintCollation {}
impl ::core::clone::Clone for CoreWebView2PrintCollation {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CoreWebView2PrintCollation {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for CoreWebView2PrintCollation {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for CoreWebView2PrintCollation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreWebView2PrintCollation").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for CoreWebView2PrintCollation {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::from_slice(
            b"enum(Microsoft.Web.WebView2.Core.CoreWebView2PrintCollation;i4)",
        );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CoreWebView2PrintColorMode(pub i32);
impl CoreWebView2PrintColorMode {
    pub const Default: Self = Self(0i32);
    pub const Color: Self = Self(1i32);
    pub const Grayscale: Self = Self(2i32);
}
impl ::core::marker::Copy for CoreWebView2PrintColorMode {}
impl ::core::clone::Clone for CoreWebView2PrintColorMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CoreWebView2PrintColorMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for CoreWebView2PrintColorMode {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for CoreWebView2PrintColorMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreWebView2PrintColorMode").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for CoreWebView2PrintColorMode {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::from_slice(
            b"enum(Microsoft.Web.WebView2.Core.CoreWebView2PrintColorMode;i4)",
        );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CoreWebView2PrintDialogKind(pub i32);
impl CoreWebView2PrintDialogKind {
    pub const Browser: Self = Self(0i32);
    pub const System: Self = Self(1i32);
}
impl ::core::marker::Copy for CoreWebView2PrintDialogKind {}
impl ::core::clone::Clone for CoreWebView2PrintDialogKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CoreWebView2PrintDialogKind {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for CoreWebView2PrintDialogKind {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for CoreWebView2PrintDialogKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreWebView2PrintDialogKind").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for CoreWebView2PrintDialogKind {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::from_slice(
            b"enum(Microsoft.Web.WebView2.Core.CoreWebView2PrintDialogKind;i4)",
        );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CoreWebView2PrintDuplex(pub i32);
impl CoreWebView2PrintDuplex {
    pub const Default: Self = Self(0i32);
    pub const OneSided: Self = Self(1i32);
    pub const TwoSidedLongEdge: Self = Self(2i32);
    pub const TwoSidedShortEdge: Self = Self(3i32);
}
impl ::core::marker::Copy for CoreWebView2PrintDuplex {}
impl ::core::clone::Clone for CoreWebView2PrintDuplex {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CoreWebView2PrintDuplex {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for CoreWebView2PrintDuplex {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for CoreWebView2PrintDuplex {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreWebView2PrintDuplex").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for CoreWebView2PrintDuplex {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::from_slice(
            b"enum(Microsoft.Web.WebView2.Core.CoreWebView2PrintDuplex;i4)",
        );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CoreWebView2PrintMediaSize(pub i32);
impl CoreWebView2PrintMediaSize {
    pub const Default: Self = Self(0i32);
    pub const Custom: Self = Self(1i32);
}
impl ::core::marker::Copy for CoreWebView2PrintMediaSize {}
impl ::core::clone::Clone for CoreWebView2PrintMediaSize {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CoreWebView2PrintMediaSize {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for CoreWebView2PrintMediaSize {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for CoreWebView2PrintMediaSize {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreWebView2PrintMediaSize").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for CoreWebView2PrintMediaSize {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::from_slice(
            b"enum(Microsoft.Web.WebView2.Core.CoreWebView2PrintMediaSize;i4)",
        );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CoreWebView2PrintOrientation(pub i32);
impl CoreWebView2PrintOrientation {
    pub const Portrait: Self = Self(0i32);
    pub const Landscape: Self = Self(1i32);
}
impl ::core::marker::Copy for CoreWebView2PrintOrientation {}
impl ::core::clone::Clone for CoreWebView2PrintOrientation {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CoreWebView2PrintOrientation {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for CoreWebView2PrintOrientation {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for CoreWebView2PrintOrientation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreWebView2PrintOrientation").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for CoreWebView2PrintOrientation {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::from_slice(
            b"enum(Microsoft.Web.WebView2.Core.CoreWebView2PrintOrientation;i4)",
        );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CoreWebView2PrintStatus(pub i32);
impl CoreWebView2PrintStatus {
    pub const Succeeded: Self = Self(0i32);
    pub const PrinterUnavailable: Self = Self(1i32);
    pub const OtherError: Self = Self(2i32);
}
impl ::core::marker::Copy for CoreWebView2PrintStatus {}
impl ::core::clone::Clone for CoreWebView2PrintStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CoreWebView2PrintStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for CoreWebView2PrintStatus {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for CoreWebView2PrintStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreWebView2PrintStatus").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for CoreWebView2PrintStatus {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::from_slice(
            b"enum(Microsoft.Web.WebView2.Core.CoreWebView2PrintStatus;i4)",
        );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CoreWebView2ProcessFailedKind(pub i32);
impl CoreWebView2ProcessFailedKind {
    pub const BrowserProcessExited: Self = Self(0i32);
    pub const RenderProcessExited: Self = Self(1i32);
    pub const RenderProcessUnresponsive: Self = Self(2i32);
    pub const FrameRenderProcessExited: Self = Self(3i32);
    pub const UtilityProcessExited: Self = Self(4i32);
    pub const SandboxHelperProcessExited: Self = Self(5i32);
    pub const GpuProcessExited: Self = Self(6i32);
    pub const PpapiPluginProcessExited: Self = Self(7i32);
    pub const PpapiBrokerProcessExited: Self = Self(8i32);
    pub const UnknownProcessExited: Self = Self(9i32);
}
impl ::core::marker::Copy for CoreWebView2ProcessFailedKind {}
impl ::core::clone::Clone for CoreWebView2ProcessFailedKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CoreWebView2ProcessFailedKind {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for CoreWebView2ProcessFailedKind {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for CoreWebView2ProcessFailedKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreWebView2ProcessFailedKind").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for CoreWebView2ProcessFailedKind {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::from_slice(
            b"enum(Microsoft.Web.WebView2.Core.CoreWebView2ProcessFailedKind;i4)",
        );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CoreWebView2ProcessFailedReason(pub i32);
impl CoreWebView2ProcessFailedReason {
    pub const Unexpected: Self = Self(0i32);
    pub const Unresponsive: Self = Self(1i32);
    pub const Terminated: Self = Self(2i32);
    pub const Crashed: Self = Self(3i32);
    pub const LaunchFailed: Self = Self(4i32);
    pub const OutOfMemory: Self = Self(5i32);
    pub const ProfileDeleted: Self = Self(6i32);
}
impl ::core::marker::Copy for CoreWebView2ProcessFailedReason {}
impl ::core::clone::Clone for CoreWebView2ProcessFailedReason {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CoreWebView2ProcessFailedReason {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for CoreWebView2ProcessFailedReason {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for CoreWebView2ProcessFailedReason {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreWebView2ProcessFailedReason").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for CoreWebView2ProcessFailedReason {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::from_slice(
            b"enum(Microsoft.Web.WebView2.Core.CoreWebView2ProcessFailedReason;i4)",
        );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CoreWebView2ProcessKind(pub i32);
impl CoreWebView2ProcessKind {
    pub const Browser: Self = Self(0i32);
    pub const Renderer: Self = Self(1i32);
    pub const Utility: Self = Self(2i32);
    pub const SandboxHelper: Self = Self(3i32);
    pub const Gpu: Self = Self(4i32);
    pub const PpapiPlugin: Self = Self(5i32);
    pub const PpapiBroker: Self = Self(6i32);
}
impl ::core::marker::Copy for CoreWebView2ProcessKind {}
impl ::core::clone::Clone for CoreWebView2ProcessKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CoreWebView2ProcessKind {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for CoreWebView2ProcessKind {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for CoreWebView2ProcessKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreWebView2ProcessKind").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for CoreWebView2ProcessKind {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::from_slice(
            b"enum(Microsoft.Web.WebView2.Core.CoreWebView2ProcessKind;i4)",
        );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CoreWebView2ScriptDialogKind(pub i32);
impl CoreWebView2ScriptDialogKind {
    pub const Alert: Self = Self(0i32);
    pub const Confirm: Self = Self(1i32);
    pub const Prompt: Self = Self(2i32);
    pub const Beforeunload: Self = Self(3i32);
}
impl ::core::marker::Copy for CoreWebView2ScriptDialogKind {}
impl ::core::clone::Clone for CoreWebView2ScriptDialogKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CoreWebView2ScriptDialogKind {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for CoreWebView2ScriptDialogKind {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for CoreWebView2ScriptDialogKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreWebView2ScriptDialogKind").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for CoreWebView2ScriptDialogKind {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::from_slice(
            b"enum(Microsoft.Web.WebView2.Core.CoreWebView2ScriptDialogKind;i4)",
        );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CoreWebView2ServerCertificateErrorAction(pub i32);
impl CoreWebView2ServerCertificateErrorAction {
    pub const AlwaysAllow: Self = Self(0i32);
    pub const Cancel: Self = Self(1i32);
    pub const Default: Self = Self(2i32);
}
impl ::core::marker::Copy for CoreWebView2ServerCertificateErrorAction {}
impl ::core::clone::Clone for CoreWebView2ServerCertificateErrorAction {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CoreWebView2ServerCertificateErrorAction {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for CoreWebView2ServerCertificateErrorAction {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for CoreWebView2ServerCertificateErrorAction {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreWebView2ServerCertificateErrorAction")
            .field(&self.0)
            .finish()
    }
}
impl ::windows_core::RuntimeType for CoreWebView2ServerCertificateErrorAction {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::from_slice(
            b"enum(Microsoft.Web.WebView2.Core.CoreWebView2ServerCertificateErrorAction;i4)",
        );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CoreWebView2SharedBufferAccess(pub i32);
impl CoreWebView2SharedBufferAccess {
    pub const ReadOnly: Self = Self(0i32);
    pub const ReadWrite: Self = Self(1i32);
}
impl ::core::marker::Copy for CoreWebView2SharedBufferAccess {}
impl ::core::clone::Clone for CoreWebView2SharedBufferAccess {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CoreWebView2SharedBufferAccess {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for CoreWebView2SharedBufferAccess {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for CoreWebView2SharedBufferAccess {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreWebView2SharedBufferAccess").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for CoreWebView2SharedBufferAccess {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::from_slice(
            b"enum(Microsoft.Web.WebView2.Core.CoreWebView2SharedBufferAccess;i4)",
        );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CoreWebView2TrackingPreventionLevel(pub i32);
impl CoreWebView2TrackingPreventionLevel {
    pub const None: Self = Self(0i32);
    pub const Basic: Self = Self(1i32);
    pub const Balanced: Self = Self(2i32);
    pub const Strict: Self = Self(3i32);
}
impl ::core::marker::Copy for CoreWebView2TrackingPreventionLevel {}
impl ::core::clone::Clone for CoreWebView2TrackingPreventionLevel {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CoreWebView2TrackingPreventionLevel {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for CoreWebView2TrackingPreventionLevel {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for CoreWebView2TrackingPreventionLevel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreWebView2TrackingPreventionLevel").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for CoreWebView2TrackingPreventionLevel {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::from_slice(
            b"enum(Microsoft.Web.WebView2.Core.CoreWebView2TrackingPreventionLevel;i4)",
        );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CoreWebView2WebErrorStatus(pub i32);
impl CoreWebView2WebErrorStatus {
    pub const Unknown: Self = Self(0i32);
    pub const CertificateCommonNameIsIncorrect: Self = Self(1i32);
    pub const CertificateExpired: Self = Self(2i32);
    pub const ClientCertificateContainsErrors: Self = Self(3i32);
    pub const CertificateRevoked: Self = Self(4i32);
    pub const CertificateIsInvalid: Self = Self(5i32);
    pub const ServerUnreachable: Self = Self(6i32);
    pub const Timeout: Self = Self(7i32);
    pub const ErrorHttpInvalidServerResponse: Self = Self(8i32);
    pub const ConnectionAborted: Self = Self(9i32);
    pub const ConnectionReset: Self = Self(10i32);
    pub const Disconnected: Self = Self(11i32);
    pub const CannotConnect: Self = Self(12i32);
    pub const HostNameNotResolved: Self = Self(13i32);
    pub const OperationCanceled: Self = Self(14i32);
    pub const RedirectFailed: Self = Self(15i32);
    pub const UnexpectedError: Self = Self(16i32);
    pub const ValidAuthenticationCredentialsRequired: Self = Self(17i32);
    pub const ValidProxyAuthenticationRequired: Self = Self(18i32);
}
impl ::core::marker::Copy for CoreWebView2WebErrorStatus {}
impl ::core::clone::Clone for CoreWebView2WebErrorStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CoreWebView2WebErrorStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for CoreWebView2WebErrorStatus {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for CoreWebView2WebErrorStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreWebView2WebErrorStatus").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for CoreWebView2WebErrorStatus {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::from_slice(
            b"enum(Microsoft.Web.WebView2.Core.CoreWebView2WebErrorStatus;i4)",
        );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CoreWebView2WebResourceContext(pub i32);
impl CoreWebView2WebResourceContext {
    pub const All: Self = Self(0i32);
    pub const Document: Self = Self(1i32);
    pub const Stylesheet: Self = Self(2i32);
    pub const Image: Self = Self(3i32);
    pub const Media: Self = Self(4i32);
    pub const Font: Self = Self(5i32);
    pub const Script: Self = Self(6i32);
    pub const XmlHttpRequest: Self = Self(7i32);
    pub const Fetch: Self = Self(8i32);
    pub const TextTrack: Self = Self(9i32);
    pub const EventSource: Self = Self(10i32);
    pub const Websocket: Self = Self(11i32);
    pub const Manifest: Self = Self(12i32);
    pub const SignedExchange: Self = Self(13i32);
    pub const Ping: Self = Self(14i32);
    pub const CspViolationReport: Self = Self(15i32);
    pub const Other: Self = Self(16i32);
}
impl ::core::marker::Copy for CoreWebView2WebResourceContext {}
impl ::core::clone::Clone for CoreWebView2WebResourceContext {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CoreWebView2WebResourceContext {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for CoreWebView2WebResourceContext {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for CoreWebView2WebResourceContext {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreWebView2WebResourceContext").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for CoreWebView2WebResourceContext {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::from_slice(
            b"enum(Microsoft.Web.WebView2.Core.CoreWebView2WebResourceContext;i4)",
        );
}
#[repr(C)]
pub struct CoreWebView2PhysicalKeyStatus {
    pub RepeatCount: u32,
    pub ScanCode: u32,
    pub IsExtendedKey: i32,
    pub IsMenuKeyDown: i32,
    pub WasKeyDown: i32,
    pub IsKeyReleased: i32,
}
impl ::core::marker::Copy for CoreWebView2PhysicalKeyStatus {}
impl ::core::clone::Clone for CoreWebView2PhysicalKeyStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CoreWebView2PhysicalKeyStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CoreWebView2PhysicalKeyStatus")
            .field("RepeatCount", &self.RepeatCount)
            .field("ScanCode", &self.ScanCode)
            .field("IsExtendedKey", &self.IsExtendedKey)
            .field("IsMenuKeyDown", &self.IsMenuKeyDown)
            .field("WasKeyDown", &self.WasKeyDown)
            .field("IsKeyReleased", &self.IsKeyReleased)
            .finish()
    }
}
impl ::windows_core::TypeKind for CoreWebView2PhysicalKeyStatus {
    type TypeKind = ::windows_core::CopyType;
}
impl ::windows_core::RuntimeType for CoreWebView2PhysicalKeyStatus {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::from_slice(
            b"struct(Microsoft.Web.WebView2.Core.CoreWebView2PhysicalKeyStatus;u4;u4;i4;i4;i4;i4)",
        );
}
impl ::core::cmp::PartialEq for CoreWebView2PhysicalKeyStatus {
    fn eq(&self, other: &Self) -> bool {
        self.RepeatCount == other.RepeatCount
            && self.ScanCode == other.ScanCode
            && self.IsExtendedKey == other.IsExtendedKey
            && self.IsMenuKeyDown == other.IsMenuKeyDown
            && self.WasKeyDown == other.WasKeyDown
            && self.IsKeyReleased == other.IsKeyReleased
    }
}
impl ::core::cmp::Eq for CoreWebView2PhysicalKeyStatus {}
impl ::core::default::Default for CoreWebView2PhysicalKeyStatus {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
