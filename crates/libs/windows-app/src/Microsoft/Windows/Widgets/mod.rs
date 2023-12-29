#[cfg(feature = "Microsoft_Windows_Widgets_Providers")]
#[doc = "Required features: `\"Microsoft_Windows_Widgets_Providers\"`"]
pub mod Providers;
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WidgetSize(pub i32);
impl WidgetSize {
    pub const Small: Self = Self(0i32);
    pub const Medium: Self = Self(1i32);
    pub const Large: Self = Self(2i32);
}
impl ::core::marker::Copy for WidgetSize {}
impl ::core::clone::Clone for WidgetSize {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WidgetSize {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for WidgetSize {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for WidgetSize {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WidgetSize").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for WidgetSize {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::from_slice(
            b"enum(Microsoft.Windows.Widgets.WidgetSize;i4)",
        );
}
