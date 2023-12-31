#[inline]
pub unsafe fn MrmAllocateBuffer(size: usize) -> *mut ::core::ffi::c_void {
    #[link(name = "mrm")]
    extern "system" {
        pub fn MrmAllocateBuffer(size: usize) -> *mut ::core::ffi::c_void;
    }
    MrmAllocateBuffer(size)
}
#[inline]
pub unsafe fn MrmCreateResourceContext(
    resourcemanager: *const MrmManagerHandle__,
) -> ::windows_core::Result<*mut MrmContextHandle__> {
    #[link(name = "mrm")]
    extern "system" {
        pub fn MrmCreateResourceContext(
            resourcemanager: *const MrmManagerHandle__,
            resourcecontext: *mut *mut MrmContextHandle__,
        ) -> ::windows_core::HRESULT;
    }
    let mut result__ = ::std::mem::zeroed();
    MrmCreateResourceContext(resourcemanager, &mut result__).from_abi(result__)
}
#[inline]
pub unsafe fn MrmCreateResourceManager<P0>(
    prifilename: P0,
) -> ::windows_core::Result<*mut MrmManagerHandle__>
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    #[link(name = "mrm")]
    extern "system" {
        pub fn MrmCreateResourceManager(
            prifilename: ::windows_core::PCWSTR,
            resourcemanager: *mut *mut MrmManagerHandle__,
        ) -> ::windows_core::HRESULT;
    }
    let mut result__ = ::std::mem::zeroed();
    MrmCreateResourceManager(prifilename.into_param().abi(), &mut result__).from_abi(result__)
}
#[inline]
pub unsafe fn MrmDestroyResourceContext(
    resourcecontext: ::core::option::Option<*const MrmContextHandle__>,
) {
    #[link(name = "mrm")]
    extern "system" {
        pub fn MrmDestroyResourceContext(resourcecontext: *const MrmContextHandle__);
    }
    MrmDestroyResourceContext(::core::mem::transmute(resourcecontext.unwrap_or(::std::ptr::null())))
}
#[inline]
pub unsafe fn MrmDestroyResourceManager(
    resourcemanager: ::core::option::Option<*const MrmManagerHandle__>,
) {
    #[link(name = "mrm")]
    extern "system" {
        pub fn MrmDestroyResourceManager(resourcemanager: *const MrmManagerHandle__);
    }
    MrmDestroyResourceManager(::core::mem::transmute(resourcemanager.unwrap_or(::std::ptr::null())))
}
#[inline]
pub unsafe fn MrmFreeQualifierNamesOrValues(names: &[::windows_core::PCWSTR]) {
    #[link(name = "mrm")]
    extern "system" {
        pub fn MrmFreeQualifierNamesOrValues(size: u32, names: *const ::windows_core::PCWSTR);
    }
    MrmFreeQualifierNamesOrValues(
        names.len().try_into().unwrap(),
        ::core::mem::transmute(names.as_ptr()),
    )
}
#[inline]
pub unsafe fn MrmFreeResource(resource: ::core::option::Option<*const ::core::ffi::c_void>) {
    #[link(name = "mrm")]
    extern "system" {
        pub fn MrmFreeResource(resource: *const ::core::ffi::c_void);
    }
    MrmFreeResource(::core::mem::transmute(resource.unwrap_or(::std::ptr::null())))
}
#[inline]
pub unsafe fn MrmGetAllQualifierNames(
    resourcecontext: *const MrmContextHandle__,
    size: *mut u32,
    names: *mut *mut ::windows_core::PWSTR,
) -> ::windows_core::Result<()> {
    #[link(name = "mrm")]
    extern "system" {
        pub fn MrmGetAllQualifierNames(
            resourcecontext: *const MrmContextHandle__,
            size: *mut u32,
            names: *mut *mut ::windows_core::PWSTR,
        ) -> ::windows_core::HRESULT;
    }
    MrmGetAllQualifierNames(resourcecontext, size, names).ok()
}
#[inline]
pub unsafe fn MrmGetChildResourceMap<P0>(
    resourcemanager: *const MrmManagerHandle__,
    resourcemap: ::core::option::Option<*const MrmMapHandle__>,
    childresourcemapname: P0,
) -> ::windows_core::Result<*mut MrmMapHandle__>
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    #[link(name = "mrm")]
    extern "system" {
        pub fn MrmGetChildResourceMap(
            resourcemanager: *const MrmManagerHandle__,
            resourcemap: *const MrmMapHandle__,
            childresourcemapname: ::windows_core::PCWSTR,
            childresourcemap: *mut *mut MrmMapHandle__,
        ) -> ::windows_core::HRESULT;
    }
    let mut result__ = ::std::mem::zeroed();
    MrmGetChildResourceMap(
        resourcemanager,
        ::core::mem::transmute(resourcemap.unwrap_or(::std::ptr::null())),
        childresourcemapname.into_param().abi(),
        &mut result__,
    )
    .from_abi(result__)
}
#[inline]
pub unsafe fn MrmGetFilePathFromName<P0>(
    filename: P0,
) -> ::windows_core::Result<::windows_core::PWSTR>
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    #[link(name = "mrm")]
    extern "system" {
        pub fn MrmGetFilePathFromName(
            filename: ::windows_core::PCWSTR,
            filepath: *mut ::windows_core::PWSTR,
        ) -> ::windows_core::HRESULT;
    }
    let mut result__ = ::std::mem::zeroed();
    MrmGetFilePathFromName(filename.into_param().abi(), &mut result__).from_abi(result__)
}
#[inline]
pub unsafe fn MrmGetQualifier<P0>(
    resourcecontext: *const MrmContextHandle__,
    qualifiername: P0,
) -> ::windows_core::Result<::windows_core::PWSTR>
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    #[link(name = "mrm")]
    extern "system" {
        pub fn MrmGetQualifier(
            resourcecontext: *const MrmContextHandle__,
            qualifiername: ::windows_core::PCWSTR,
            qualifiervalue: *mut ::windows_core::PWSTR,
        ) -> ::windows_core::HRESULT;
    }
    let mut result__ = ::std::mem::zeroed();
    MrmGetQualifier(resourcecontext, qualifiername.into_param().abi(), &mut result__)
        .from_abi(result__)
}
#[inline]
pub unsafe fn MrmGetResourceCount(
    resourcemanager: *const MrmManagerHandle__,
    resourcemap: ::core::option::Option<*const MrmMapHandle__>,
) -> ::windows_core::Result<u32> {
    #[link(name = "mrm")]
    extern "system" {
        pub fn MrmGetResourceCount(
            resourcemanager: *const MrmManagerHandle__,
            resourcemap: *const MrmMapHandle__,
            count: *mut u32,
        ) -> ::windows_core::HRESULT;
    }
    let mut result__ = ::std::mem::zeroed();
    MrmGetResourceCount(
        resourcemanager,
        ::core::mem::transmute(resourcemap.unwrap_or(::std::ptr::null())),
        &mut result__,
    )
    .from_abi(result__)
}
#[inline]
pub unsafe fn MrmLoadEmbeddedResource<P0>(
    resourcemanager: *const MrmManagerHandle__,
    resourcecontext: ::core::option::Option<*const MrmContextHandle__>,
    resourcemap: ::core::option::Option<*const MrmMapHandle__>,
    resourceid: P0,
) -> ::windows_core::Result<MrmResourceData>
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    #[link(name = "mrm")]
    extern "system" {
        pub fn MrmLoadEmbeddedResource(
            resourcemanager: *const MrmManagerHandle__,
            resourcecontext: *const MrmContextHandle__,
            resourcemap: *const MrmMapHandle__,
            resourceid: ::windows_core::PCWSTR,
            data: *mut MrmResourceData,
        ) -> ::windows_core::HRESULT;
    }
    let mut result__ = ::std::mem::zeroed();
    MrmLoadEmbeddedResource(
        resourcemanager,
        ::core::mem::transmute(resourcecontext.unwrap_or(::std::ptr::null())),
        ::core::mem::transmute(resourcemap.unwrap_or(::std::ptr::null())),
        resourceid.into_param().abi(),
        &mut result__,
    )
    .from_abi(result__)
}
#[inline]
pub unsafe fn MrmLoadEmbeddedResourceFromResourceUri<P0>(
    resourcemanager: *const MrmManagerHandle__,
    resourcecontext: ::core::option::Option<*const MrmContextHandle__>,
    resourceuri: P0,
) -> ::windows_core::Result<MrmResourceData>
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    #[link(name = "mrm")]
    extern "system" {
        pub fn MrmLoadEmbeddedResourceFromResourceUri(
            resourcemanager: *const MrmManagerHandle__,
            resourcecontext: *const MrmContextHandle__,
            resourceuri: ::windows_core::PCWSTR,
            data: *mut MrmResourceData,
        ) -> ::windows_core::HRESULT;
    }
    let mut result__ = ::std::mem::zeroed();
    MrmLoadEmbeddedResourceFromResourceUri(
        resourcemanager,
        ::core::mem::transmute(resourcecontext.unwrap_or(::std::ptr::null())),
        resourceuri.into_param().abi(),
        &mut result__,
    )
    .from_abi(result__)
}
#[inline]
pub unsafe fn MrmLoadStringOrEmbeddedFromResourceUri<P0>(
    resourcemanager: *const MrmManagerHandle__,
    resourcecontext: ::core::option::Option<*const MrmContextHandle__>,
    resourceuri: P0,
    resourcetype: *mut MrmType,
    resourcestring: *mut ::windows_core::PWSTR,
    data: *mut MrmResourceData,
) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    #[link(name = "mrm")]
    extern "system" {
        pub fn MrmLoadStringOrEmbeddedFromResourceUri(
            resourcemanager: *const MrmManagerHandle__,
            resourcecontext: *const MrmContextHandle__,
            resourceuri: ::windows_core::PCWSTR,
            resourcetype: *mut MrmType,
            resourcestring: *mut ::windows_core::PWSTR,
            data: *mut MrmResourceData,
        ) -> ::windows_core::HRESULT;
    }
    MrmLoadStringOrEmbeddedFromResourceUri(
        resourcemanager,
        ::core::mem::transmute(resourcecontext.unwrap_or(::std::ptr::null())),
        resourceuri.into_param().abi(),
        resourcetype,
        resourcestring,
        data,
    )
    .ok()
}
#[inline]
pub unsafe fn MrmLoadStringOrEmbeddedResource<P0>(
    resourcemanager: *const MrmManagerHandle__,
    resourcecontext: ::core::option::Option<*const MrmContextHandle__>,
    resourcemap: ::core::option::Option<*const MrmMapHandle__>,
    resourceid: P0,
    resourcetype: *mut MrmType,
    resourcestring: *mut ::windows_core::PWSTR,
    data: *mut MrmResourceData,
) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    #[link(name = "mrm")]
    extern "system" {
        pub fn MrmLoadStringOrEmbeddedResource(
            resourcemanager: *const MrmManagerHandle__,
            resourcecontext: *const MrmContextHandle__,
            resourcemap: *const MrmMapHandle__,
            resourceid: ::windows_core::PCWSTR,
            resourcetype: *mut MrmType,
            resourcestring: *mut ::windows_core::PWSTR,
            data: *mut MrmResourceData,
        ) -> ::windows_core::HRESULT;
    }
    MrmLoadStringOrEmbeddedResource(
        resourcemanager,
        ::core::mem::transmute(resourcecontext.unwrap_or(::std::ptr::null())),
        ::core::mem::transmute(resourcemap.unwrap_or(::std::ptr::null())),
        resourceid.into_param().abi(),
        resourcetype,
        resourcestring,
        data,
    )
    .ok()
}
#[inline]
pub unsafe fn MrmLoadStringOrEmbeddedResourceByIndex(
    resourcemanager: *const MrmManagerHandle__,
    resourcecontext: ::core::option::Option<*const MrmContextHandle__>,
    resourcemap: ::core::option::Option<*const MrmMapHandle__>,
    index: u32,
    resourcetype: *mut MrmType,
    resourcename: *mut ::windows_core::PWSTR,
    resourcestring: *mut ::windows_core::PWSTR,
    data: *mut MrmResourceData,
) -> ::windows_core::Result<()> {
    #[link(name = "mrm")]
    extern "system" {
        pub fn MrmLoadStringOrEmbeddedResourceByIndex(
            resourcemanager: *const MrmManagerHandle__,
            resourcecontext: *const MrmContextHandle__,
            resourcemap: *const MrmMapHandle__,
            index: u32,
            resourcetype: *mut MrmType,
            resourcename: *mut ::windows_core::PWSTR,
            resourcestring: *mut ::windows_core::PWSTR,
            data: *mut MrmResourceData,
        ) -> ::windows_core::HRESULT;
    }
    MrmLoadStringOrEmbeddedResourceByIndex(
        resourcemanager,
        ::core::mem::transmute(resourcecontext.unwrap_or(::std::ptr::null())),
        ::core::mem::transmute(resourcemap.unwrap_or(::std::ptr::null())),
        index,
        resourcetype,
        resourcename,
        resourcestring,
        data,
    )
    .ok()
}
#[inline]
pub unsafe fn MrmLoadStringOrEmbeddedResourceByIndexWithQualifierValues(
    resourcemanager: *const MrmManagerHandle__,
    resourcecontext: ::core::option::Option<*const MrmContextHandle__>,
    resourcemap: ::core::option::Option<*const MrmMapHandle__>,
    index: u32,
    resourcetype: *mut MrmType,
    resourcename: *mut ::windows_core::PWSTR,
    resourcestring: *mut ::windows_core::PWSTR,
    data: *mut MrmResourceData,
    qualifiercount: *mut u32,
    qualifiernames: *mut *mut ::windows_core::PWSTR,
    qualifiervalues: *mut *mut ::windows_core::PWSTR,
) -> ::windows_core::Result<()> {
    #[link(name = "mrm")]
    extern "system" {
        pub fn MrmLoadStringOrEmbeddedResourceByIndexWithQualifierValues(
            resourcemanager: *const MrmManagerHandle__,
            resourcecontext: *const MrmContextHandle__,
            resourcemap: *const MrmMapHandle__,
            index: u32,
            resourcetype: *mut MrmType,
            resourcename: *mut ::windows_core::PWSTR,
            resourcestring: *mut ::windows_core::PWSTR,
            data: *mut MrmResourceData,
            qualifiercount: *mut u32,
            qualifiernames: *mut *mut ::windows_core::PWSTR,
            qualifiervalues: *mut *mut ::windows_core::PWSTR,
        ) -> ::windows_core::HRESULT;
    }
    MrmLoadStringOrEmbeddedResourceByIndexWithQualifierValues(
        resourcemanager,
        ::core::mem::transmute(resourcecontext.unwrap_or(::std::ptr::null())),
        ::core::mem::transmute(resourcemap.unwrap_or(::std::ptr::null())),
        index,
        resourcetype,
        resourcename,
        resourcestring,
        data,
        qualifiercount,
        qualifiernames,
        qualifiervalues,
    )
    .ok()
}
#[inline]
pub unsafe fn MrmLoadStringOrEmbeddedResourceWithQualifierValues<P0>(
    resourcemanager: *const MrmManagerHandle__,
    resourcecontext: ::core::option::Option<*const MrmContextHandle__>,
    resourcemap: ::core::option::Option<*const MrmMapHandle__>,
    resourceid: P0,
    resourcetype: *mut MrmType,
    resourcestring: *mut ::windows_core::PWSTR,
    data: *mut MrmResourceData,
    qualifiercount: *mut u32,
    qualifiernames: *mut *mut ::windows_core::PWSTR,
    qualifiervalues: *mut *mut ::windows_core::PWSTR,
) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    #[link(name = "mrm")]
    extern "system" {
        pub fn MrmLoadStringOrEmbeddedResourceWithQualifierValues(
            resourcemanager: *const MrmManagerHandle__,
            resourcecontext: *const MrmContextHandle__,
            resourcemap: *const MrmMapHandle__,
            resourceid: ::windows_core::PCWSTR,
            resourcetype: *mut MrmType,
            resourcestring: *mut ::windows_core::PWSTR,
            data: *mut MrmResourceData,
            qualifiercount: *mut u32,
            qualifiernames: *mut *mut ::windows_core::PWSTR,
            qualifiervalues: *mut *mut ::windows_core::PWSTR,
        ) -> ::windows_core::HRESULT;
    }
    MrmLoadStringOrEmbeddedResourceWithQualifierValues(
        resourcemanager,
        ::core::mem::transmute(resourcecontext.unwrap_or(::std::ptr::null())),
        ::core::mem::transmute(resourcemap.unwrap_or(::std::ptr::null())),
        resourceid.into_param().abi(),
        resourcetype,
        resourcestring,
        data,
        qualifiercount,
        qualifiernames,
        qualifiervalues,
    )
    .ok()
}
#[inline]
pub unsafe fn MrmLoadStringResource<P0>(
    resourcemanager: *const MrmManagerHandle__,
    resourcecontext: ::core::option::Option<*const MrmContextHandle__>,
    resourcemap: ::core::option::Option<*const MrmMapHandle__>,
    resourceid: P0,
) -> ::windows_core::Result<::windows_core::PWSTR>
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    #[link(name = "mrm")]
    extern "system" {
        pub fn MrmLoadStringResource(
            resourcemanager: *const MrmManagerHandle__,
            resourcecontext: *const MrmContextHandle__,
            resourcemap: *const MrmMapHandle__,
            resourceid: ::windows_core::PCWSTR,
            resourcestring: *mut ::windows_core::PWSTR,
        ) -> ::windows_core::HRESULT;
    }
    let mut result__ = ::std::mem::zeroed();
    MrmLoadStringResource(
        resourcemanager,
        ::core::mem::transmute(resourcecontext.unwrap_or(::std::ptr::null())),
        ::core::mem::transmute(resourcemap.unwrap_or(::std::ptr::null())),
        resourceid.into_param().abi(),
        &mut result__,
    )
    .from_abi(result__)
}
#[inline]
pub unsafe fn MrmLoadStringResourceFromResourceUri<P0>(
    resourcemanager: *const MrmManagerHandle__,
    resourcecontext: ::core::option::Option<*const MrmContextHandle__>,
    resourceuri: P0,
) -> ::windows_core::Result<::windows_core::PWSTR>
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    #[link(name = "mrm")]
    extern "system" {
        pub fn MrmLoadStringResourceFromResourceUri(
            resourcemanager: *const MrmManagerHandle__,
            resourcecontext: *const MrmContextHandle__,
            resourceuri: ::windows_core::PCWSTR,
            resourcestring: *mut ::windows_core::PWSTR,
        ) -> ::windows_core::HRESULT;
    }
    let mut result__ = ::std::mem::zeroed();
    MrmLoadStringResourceFromResourceUri(
        resourcemanager,
        ::core::mem::transmute(resourcecontext.unwrap_or(::std::ptr::null())),
        resourceuri.into_param().abi(),
        &mut result__,
    )
    .from_abi(result__)
}
#[inline]
pub unsafe fn MrmSetQualifier<P0, P1>(
    resourcecontext: *const MrmContextHandle__,
    qualifiername: P0,
    qualifiervalue: P1,
) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    #[link(name = "mrm")]
    extern "system" {
        pub fn MrmSetQualifier(
            resourcecontext: *const MrmContextHandle__,
            qualifiername: ::windows_core::PCWSTR,
            qualifiervalue: ::windows_core::PCWSTR,
        ) -> ::windows_core::HRESULT;
    }
    MrmSetQualifier(
        resourcecontext,
        qualifiername.into_param().abi(),
        qualifiervalue.into_param().abi(),
    )
    .ok()
}
pub const MrmType_Embedded: MrmType = MrmType(3i32);
pub const MrmType_Path: MrmType = MrmType(2i32);
pub const MrmType_String: MrmType = MrmType(1i32);
pub const MrmType_Unknown: MrmType = MrmType(0i32);
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MrmType(pub i32);
impl ::core::marker::Copy for MrmType {}
impl ::core::clone::Clone for MrmType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MrmType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for MrmType {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for MrmType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MrmType").field(&self.0).finish()
    }
}
#[repr(C)]
pub struct MrmContextHandle__ {
    pub unused: i32,
}
impl ::core::marker::Copy for MrmContextHandle__ {}
impl ::core::clone::Clone for MrmContextHandle__ {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MrmContextHandle__ {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MrmContextHandle__").field("unused", &self.unused).finish()
    }
}
impl ::windows_core::TypeKind for MrmContextHandle__ {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for MrmContextHandle__ {
    fn eq(&self, other: &Self) -> bool {
        self.unused == other.unused
    }
}
impl ::core::cmp::Eq for MrmContextHandle__ {}
impl ::core::default::Default for MrmContextHandle__ {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MrmManagerHandle__ {
    pub unused: i32,
}
impl ::core::marker::Copy for MrmManagerHandle__ {}
impl ::core::clone::Clone for MrmManagerHandle__ {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MrmManagerHandle__ {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MrmManagerHandle__").field("unused", &self.unused).finish()
    }
}
impl ::windows_core::TypeKind for MrmManagerHandle__ {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for MrmManagerHandle__ {
    fn eq(&self, other: &Self) -> bool {
        self.unused == other.unused
    }
}
impl ::core::cmp::Eq for MrmManagerHandle__ {}
impl ::core::default::Default for MrmManagerHandle__ {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MrmMapHandle__ {
    pub unused: i32,
}
impl ::core::marker::Copy for MrmMapHandle__ {}
impl ::core::clone::Clone for MrmMapHandle__ {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MrmMapHandle__ {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MrmMapHandle__").field("unused", &self.unused).finish()
    }
}
impl ::windows_core::TypeKind for MrmMapHandle__ {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for MrmMapHandle__ {
    fn eq(&self, other: &Self) -> bool {
        self.unused == other.unused
    }
}
impl ::core::cmp::Eq for MrmMapHandle__ {}
impl ::core::default::Default for MrmMapHandle__ {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MrmResourceData {
    pub size: u32,
    pub data: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for MrmResourceData {}
impl ::core::clone::Clone for MrmResourceData {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MrmResourceData {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MrmResourceData")
            .field("size", &self.size)
            .field("data", &self.data)
            .finish()
    }
}
impl ::windows_core::TypeKind for MrmResourceData {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for MrmResourceData {
    fn eq(&self, other: &Self) -> bool {
        self.size == other.size && self.data == other.data
    }
}
impl ::core::cmp::Eq for MrmResourceData {}
impl ::core::default::Default for MrmResourceData {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
