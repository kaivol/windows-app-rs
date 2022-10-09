#[doc = "*Required features: `\"UI_Xaml_Controls_AnimatedVisuals\"`*"]
#[repr(transparent)]
pub struct AnimatedAcceptVisualSource(::windows::core::IUnknown);
impl AnimatedAcceptVisualSource {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<
            AnimatedAcceptVisualSource,
            ::windows::core::IGenericFactory,
        > = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"UI_Composition\"`*"]
    #[cfg(feature = "UI_Composition")]
    pub fn TryCreateAnimatedVisual(
        &self,
        compositor: &super::super::super::Composition::Compositor,
        diagnostics: &mut ::core::option::Option<::windows::core::IInspectable>,
    ) -> ::windows::core::Result<super::IAnimatedVisual> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TryCreateAnimatedVisual)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(compositor),
                diagnostics as *mut _ as _,
                result__.as_mut_ptr(),
            )
            .from_abi::<super::IAnimatedVisual>(result__)
        }
    }
    pub fn Markers(
        &self,
    ) -> ::windows::core::Result<
        ::windows::Foundation::Collections::IMapView<::windows::core::HSTRING, f64>,
    > {
        let this = &::windows::core::Interface::cast::<super::IAnimatedVisualSource2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            ( ::windows::core::Vtable::vtable ( this ) . Markers ) ( ::windows::core::Vtable::as_raw ( this ) , result__ . as_mut_ptr ( ) ) . from_abi:: < ::windows::Foundation::Collections:: IMapView :: < :: windows::core::HSTRING , f64 > > ( result__ )
        }
    }
    pub fn SetColorProperty(
        &self,
        propertyname: &::windows::core::HSTRING,
        value: ::windows::UI::Color,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IAnimatedVisualSource2>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetColorProperty)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(propertyname),
                value,
            )
            .ok()
        }
    }
}
impl ::core::clone::Clone for AnimatedAcceptVisualSource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AnimatedAcceptVisualSource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AnimatedAcceptVisualSource {}
impl ::core::fmt::Debug for AnimatedAcceptVisualSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AnimatedAcceptVisualSource").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AnimatedAcceptVisualSource {
    const SIGNATURE : ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice ( b"rc(Microsoft.UI.Xaml.Controls.AnimatedVisuals.AnimatedAcceptVisualSource;{294765c3-70e3-555c-9657-01fc4051169d})" ) ;
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for AnimatedAcceptVisualSource {
    type Vtable = super::IAnimatedVisualSource_Vtbl;
}
unsafe impl ::windows::core::Interface for AnimatedAcceptVisualSource {
    const IID: ::windows::core::GUID =
        <super::IAnimatedVisualSource as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AnimatedAcceptVisualSource {
    const NAME: &'static str =
        "Microsoft.UI.Xaml.Controls.AnimatedVisuals.AnimatedAcceptVisualSource";
}
::windows::core::interface_hierarchy!(
    AnimatedAcceptVisualSource,
    ::windows::core::IUnknown,
    ::windows::core::IInspectable
);
impl ::core::convert::TryFrom<AnimatedAcceptVisualSource> for super::IAnimatedVisualSource {
    type Error = ::windows::core::Error;
    fn try_from(value: AnimatedAcceptVisualSource) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AnimatedAcceptVisualSource> for super::IAnimatedVisualSource {
    type Error = ::windows::core::Error;
    fn try_from(value: &AnimatedAcceptVisualSource) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&AnimatedAcceptVisualSource>
    for ::windows::core::InParam<'a, super::IAnimatedVisualSource>
{
    type Error = ::windows::core::Error;
    fn try_from(value: &AnimatedAcceptVisualSource) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<AnimatedAcceptVisualSource> for super::IAnimatedVisualSource2 {
    type Error = ::windows::core::Error;
    fn try_from(value: AnimatedAcceptVisualSource) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AnimatedAcceptVisualSource> for super::IAnimatedVisualSource2 {
    type Error = ::windows::core::Error;
    fn try_from(value: &AnimatedAcceptVisualSource) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&AnimatedAcceptVisualSource>
    for ::windows::core::InParam<'a, super::IAnimatedVisualSource2>
{
    type Error = ::windows::core::Error;
    fn try_from(value: &AnimatedAcceptVisualSource) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for AnimatedAcceptVisualSource {}
unsafe impl ::core::marker::Sync for AnimatedAcceptVisualSource {}
#[doc = "*Required features: `\"UI_Xaml_Controls_AnimatedVisuals\"`*"]
#[repr(transparent)]
pub struct AnimatedBackVisualSource(::windows::core::IUnknown);
impl AnimatedBackVisualSource {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<
            AnimatedBackVisualSource,
            ::windows::core::IGenericFactory,
        > = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"UI_Composition\"`*"]
    #[cfg(feature = "UI_Composition")]
    pub fn TryCreateAnimatedVisual(
        &self,
        compositor: &super::super::super::Composition::Compositor,
        diagnostics: &mut ::core::option::Option<::windows::core::IInspectable>,
    ) -> ::windows::core::Result<super::IAnimatedVisual> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TryCreateAnimatedVisual)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(compositor),
                diagnostics as *mut _ as _,
                result__.as_mut_ptr(),
            )
            .from_abi::<super::IAnimatedVisual>(result__)
        }
    }
    pub fn Markers(
        &self,
    ) -> ::windows::core::Result<
        ::windows::Foundation::Collections::IMapView<::windows::core::HSTRING, f64>,
    > {
        let this = &::windows::core::Interface::cast::<super::IAnimatedVisualSource2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            ( ::windows::core::Vtable::vtable ( this ) . Markers ) ( ::windows::core::Vtable::as_raw ( this ) , result__ . as_mut_ptr ( ) ) . from_abi:: < ::windows::Foundation::Collections:: IMapView :: < :: windows::core::HSTRING , f64 > > ( result__ )
        }
    }
    pub fn SetColorProperty(
        &self,
        propertyname: &::windows::core::HSTRING,
        value: ::windows::UI::Color,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IAnimatedVisualSource2>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetColorProperty)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(propertyname),
                value,
            )
            .ok()
        }
    }
}
impl ::core::clone::Clone for AnimatedBackVisualSource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AnimatedBackVisualSource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AnimatedBackVisualSource {}
impl ::core::fmt::Debug for AnimatedBackVisualSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AnimatedBackVisualSource").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AnimatedBackVisualSource {
    const SIGNATURE : ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice ( b"rc(Microsoft.UI.Xaml.Controls.AnimatedVisuals.AnimatedBackVisualSource;{294765c3-70e3-555c-9657-01fc4051169d})" ) ;
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for AnimatedBackVisualSource {
    type Vtable = super::IAnimatedVisualSource_Vtbl;
}
unsafe impl ::windows::core::Interface for AnimatedBackVisualSource {
    const IID: ::windows::core::GUID =
        <super::IAnimatedVisualSource as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AnimatedBackVisualSource {
    const NAME: &'static str =
        "Microsoft.UI.Xaml.Controls.AnimatedVisuals.AnimatedBackVisualSource";
}
::windows::core::interface_hierarchy!(
    AnimatedBackVisualSource,
    ::windows::core::IUnknown,
    ::windows::core::IInspectable
);
impl ::core::convert::TryFrom<AnimatedBackVisualSource> for super::IAnimatedVisualSource {
    type Error = ::windows::core::Error;
    fn try_from(value: AnimatedBackVisualSource) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AnimatedBackVisualSource> for super::IAnimatedVisualSource {
    type Error = ::windows::core::Error;
    fn try_from(value: &AnimatedBackVisualSource) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&AnimatedBackVisualSource>
    for ::windows::core::InParam<'a, super::IAnimatedVisualSource>
{
    type Error = ::windows::core::Error;
    fn try_from(value: &AnimatedBackVisualSource) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<AnimatedBackVisualSource> for super::IAnimatedVisualSource2 {
    type Error = ::windows::core::Error;
    fn try_from(value: AnimatedBackVisualSource) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AnimatedBackVisualSource> for super::IAnimatedVisualSource2 {
    type Error = ::windows::core::Error;
    fn try_from(value: &AnimatedBackVisualSource) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&AnimatedBackVisualSource>
    for ::windows::core::InParam<'a, super::IAnimatedVisualSource2>
{
    type Error = ::windows::core::Error;
    fn try_from(value: &AnimatedBackVisualSource) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for AnimatedBackVisualSource {}
unsafe impl ::core::marker::Sync for AnimatedBackVisualSource {}
#[doc = "*Required features: `\"UI_Xaml_Controls_AnimatedVisuals\"`*"]
#[repr(transparent)]
pub struct AnimatedChevronDownSmallVisualSource(::windows::core::IUnknown);
impl AnimatedChevronDownSmallVisualSource {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<
            AnimatedChevronDownSmallVisualSource,
            ::windows::core::IGenericFactory,
        > = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"UI_Composition\"`*"]
    #[cfg(feature = "UI_Composition")]
    pub fn TryCreateAnimatedVisual(
        &self,
        compositor: &super::super::super::Composition::Compositor,
        diagnostics: &mut ::core::option::Option<::windows::core::IInspectable>,
    ) -> ::windows::core::Result<super::IAnimatedVisual> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TryCreateAnimatedVisual)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(compositor),
                diagnostics as *mut _ as _,
                result__.as_mut_ptr(),
            )
            .from_abi::<super::IAnimatedVisual>(result__)
        }
    }
    pub fn Markers(
        &self,
    ) -> ::windows::core::Result<
        ::windows::Foundation::Collections::IMapView<::windows::core::HSTRING, f64>,
    > {
        let this = &::windows::core::Interface::cast::<super::IAnimatedVisualSource2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            ( ::windows::core::Vtable::vtable ( this ) . Markers ) ( ::windows::core::Vtable::as_raw ( this ) , result__ . as_mut_ptr ( ) ) . from_abi:: < ::windows::Foundation::Collections:: IMapView :: < :: windows::core::HSTRING , f64 > > ( result__ )
        }
    }
    pub fn SetColorProperty(
        &self,
        propertyname: &::windows::core::HSTRING,
        value: ::windows::UI::Color,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IAnimatedVisualSource2>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetColorProperty)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(propertyname),
                value,
            )
            .ok()
        }
    }
}
impl ::core::clone::Clone for AnimatedChevronDownSmallVisualSource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AnimatedChevronDownSmallVisualSource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AnimatedChevronDownSmallVisualSource {}
impl ::core::fmt::Debug for AnimatedChevronDownSmallVisualSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AnimatedChevronDownSmallVisualSource").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AnimatedChevronDownSmallVisualSource {
    const SIGNATURE : ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice ( b"rc(Microsoft.UI.Xaml.Controls.AnimatedVisuals.AnimatedChevronDownSmallVisualSource;{294765c3-70e3-555c-9657-01fc4051169d})" ) ;
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for AnimatedChevronDownSmallVisualSource {
    type Vtable = super::IAnimatedVisualSource_Vtbl;
}
unsafe impl ::windows::core::Interface for AnimatedChevronDownSmallVisualSource {
    const IID: ::windows::core::GUID =
        <super::IAnimatedVisualSource as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AnimatedChevronDownSmallVisualSource {
    const NAME: &'static str =
        "Microsoft.UI.Xaml.Controls.AnimatedVisuals.AnimatedChevronDownSmallVisualSource";
}
::windows::core::interface_hierarchy!(
    AnimatedChevronDownSmallVisualSource,
    ::windows::core::IUnknown,
    ::windows::core::IInspectable
);
impl ::core::convert::TryFrom<AnimatedChevronDownSmallVisualSource>
    for super::IAnimatedVisualSource
{
    type Error = ::windows::core::Error;
    fn try_from(value: AnimatedChevronDownSmallVisualSource) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AnimatedChevronDownSmallVisualSource>
    for super::IAnimatedVisualSource
{
    type Error = ::windows::core::Error;
    fn try_from(value: &AnimatedChevronDownSmallVisualSource) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&AnimatedChevronDownSmallVisualSource>
    for ::windows::core::InParam<'a, super::IAnimatedVisualSource>
{
    type Error = ::windows::core::Error;
    fn try_from(value: &AnimatedChevronDownSmallVisualSource) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<AnimatedChevronDownSmallVisualSource>
    for super::IAnimatedVisualSource2
{
    type Error = ::windows::core::Error;
    fn try_from(value: AnimatedChevronDownSmallVisualSource) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AnimatedChevronDownSmallVisualSource>
    for super::IAnimatedVisualSource2
{
    type Error = ::windows::core::Error;
    fn try_from(value: &AnimatedChevronDownSmallVisualSource) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&AnimatedChevronDownSmallVisualSource>
    for ::windows::core::InParam<'a, super::IAnimatedVisualSource2>
{
    type Error = ::windows::core::Error;
    fn try_from(value: &AnimatedChevronDownSmallVisualSource) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for AnimatedChevronDownSmallVisualSource {}
unsafe impl ::core::marker::Sync for AnimatedChevronDownSmallVisualSource {}
#[doc = "*Required features: `\"UI_Xaml_Controls_AnimatedVisuals\"`*"]
#[repr(transparent)]
pub struct AnimatedChevronRightDownSmallVisualSource(::windows::core::IUnknown);
impl AnimatedChevronRightDownSmallVisualSource {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<
            AnimatedChevronRightDownSmallVisualSource,
            ::windows::core::IGenericFactory,
        > = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"UI_Composition\"`*"]
    #[cfg(feature = "UI_Composition")]
    pub fn TryCreateAnimatedVisual(
        &self,
        compositor: &super::super::super::Composition::Compositor,
        diagnostics: &mut ::core::option::Option<::windows::core::IInspectable>,
    ) -> ::windows::core::Result<super::IAnimatedVisual> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TryCreateAnimatedVisual)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(compositor),
                diagnostics as *mut _ as _,
                result__.as_mut_ptr(),
            )
            .from_abi::<super::IAnimatedVisual>(result__)
        }
    }
    pub fn Markers(
        &self,
    ) -> ::windows::core::Result<
        ::windows::Foundation::Collections::IMapView<::windows::core::HSTRING, f64>,
    > {
        let this = &::windows::core::Interface::cast::<super::IAnimatedVisualSource2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            ( ::windows::core::Vtable::vtable ( this ) . Markers ) ( ::windows::core::Vtable::as_raw ( this ) , result__ . as_mut_ptr ( ) ) . from_abi:: < ::windows::Foundation::Collections:: IMapView :: < :: windows::core::HSTRING , f64 > > ( result__ )
        }
    }
    pub fn SetColorProperty(
        &self,
        propertyname: &::windows::core::HSTRING,
        value: ::windows::UI::Color,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IAnimatedVisualSource2>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetColorProperty)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(propertyname),
                value,
            )
            .ok()
        }
    }
}
impl ::core::clone::Clone for AnimatedChevronRightDownSmallVisualSource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AnimatedChevronRightDownSmallVisualSource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AnimatedChevronRightDownSmallVisualSource {}
impl ::core::fmt::Debug for AnimatedChevronRightDownSmallVisualSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AnimatedChevronRightDownSmallVisualSource")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AnimatedChevronRightDownSmallVisualSource {
    const SIGNATURE : ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice ( b"rc(Microsoft.UI.Xaml.Controls.AnimatedVisuals.AnimatedChevronRightDownSmallVisualSource;{294765c3-70e3-555c-9657-01fc4051169d})" ) ;
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for AnimatedChevronRightDownSmallVisualSource {
    type Vtable = super::IAnimatedVisualSource_Vtbl;
}
unsafe impl ::windows::core::Interface for AnimatedChevronRightDownSmallVisualSource {
    const IID: ::windows::core::GUID =
        <super::IAnimatedVisualSource as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AnimatedChevronRightDownSmallVisualSource {
    const NAME: &'static str =
        "Microsoft.UI.Xaml.Controls.AnimatedVisuals.AnimatedChevronRightDownSmallVisualSource";
}
::windows::core::interface_hierarchy!(
    AnimatedChevronRightDownSmallVisualSource,
    ::windows::core::IUnknown,
    ::windows::core::IInspectable
);
impl ::core::convert::TryFrom<AnimatedChevronRightDownSmallVisualSource>
    for super::IAnimatedVisualSource
{
    type Error = ::windows::core::Error;
    fn try_from(value: AnimatedChevronRightDownSmallVisualSource) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AnimatedChevronRightDownSmallVisualSource>
    for super::IAnimatedVisualSource
{
    type Error = ::windows::core::Error;
    fn try_from(
        value: &AnimatedChevronRightDownSmallVisualSource,
    ) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&AnimatedChevronRightDownSmallVisualSource>
    for ::windows::core::InParam<'a, super::IAnimatedVisualSource>
{
    type Error = ::windows::core::Error;
    fn try_from(
        value: &AnimatedChevronRightDownSmallVisualSource,
    ) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<AnimatedChevronRightDownSmallVisualSource>
    for super::IAnimatedVisualSource2
{
    type Error = ::windows::core::Error;
    fn try_from(value: AnimatedChevronRightDownSmallVisualSource) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AnimatedChevronRightDownSmallVisualSource>
    for super::IAnimatedVisualSource2
{
    type Error = ::windows::core::Error;
    fn try_from(
        value: &AnimatedChevronRightDownSmallVisualSource,
    ) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&AnimatedChevronRightDownSmallVisualSource>
    for ::windows::core::InParam<'a, super::IAnimatedVisualSource2>
{
    type Error = ::windows::core::Error;
    fn try_from(
        value: &AnimatedChevronRightDownSmallVisualSource,
    ) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for AnimatedChevronRightDownSmallVisualSource {}
unsafe impl ::core::marker::Sync for AnimatedChevronRightDownSmallVisualSource {}
#[doc = "*Required features: `\"UI_Xaml_Controls_AnimatedVisuals\"`*"]
#[repr(transparent)]
pub struct AnimatedChevronUpDownSmallVisualSource(::windows::core::IUnknown);
impl AnimatedChevronUpDownSmallVisualSource {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<
            AnimatedChevronUpDownSmallVisualSource,
            ::windows::core::IGenericFactory,
        > = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"UI_Composition\"`*"]
    #[cfg(feature = "UI_Composition")]
    pub fn TryCreateAnimatedVisual(
        &self,
        compositor: &super::super::super::Composition::Compositor,
        diagnostics: &mut ::core::option::Option<::windows::core::IInspectable>,
    ) -> ::windows::core::Result<super::IAnimatedVisual> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TryCreateAnimatedVisual)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(compositor),
                diagnostics as *mut _ as _,
                result__.as_mut_ptr(),
            )
            .from_abi::<super::IAnimatedVisual>(result__)
        }
    }
    pub fn Markers(
        &self,
    ) -> ::windows::core::Result<
        ::windows::Foundation::Collections::IMapView<::windows::core::HSTRING, f64>,
    > {
        let this = &::windows::core::Interface::cast::<super::IAnimatedVisualSource2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            ( ::windows::core::Vtable::vtable ( this ) . Markers ) ( ::windows::core::Vtable::as_raw ( this ) , result__ . as_mut_ptr ( ) ) . from_abi:: < ::windows::Foundation::Collections:: IMapView :: < :: windows::core::HSTRING , f64 > > ( result__ )
        }
    }
    pub fn SetColorProperty(
        &self,
        propertyname: &::windows::core::HSTRING,
        value: ::windows::UI::Color,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IAnimatedVisualSource2>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetColorProperty)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(propertyname),
                value,
            )
            .ok()
        }
    }
}
impl ::core::clone::Clone for AnimatedChevronUpDownSmallVisualSource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AnimatedChevronUpDownSmallVisualSource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AnimatedChevronUpDownSmallVisualSource {}
impl ::core::fmt::Debug for AnimatedChevronUpDownSmallVisualSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AnimatedChevronUpDownSmallVisualSource").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AnimatedChevronUpDownSmallVisualSource {
    const SIGNATURE : ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice ( b"rc(Microsoft.UI.Xaml.Controls.AnimatedVisuals.AnimatedChevronUpDownSmallVisualSource;{294765c3-70e3-555c-9657-01fc4051169d})" ) ;
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for AnimatedChevronUpDownSmallVisualSource {
    type Vtable = super::IAnimatedVisualSource_Vtbl;
}
unsafe impl ::windows::core::Interface for AnimatedChevronUpDownSmallVisualSource {
    const IID: ::windows::core::GUID =
        <super::IAnimatedVisualSource as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AnimatedChevronUpDownSmallVisualSource {
    const NAME: &'static str =
        "Microsoft.UI.Xaml.Controls.AnimatedVisuals.AnimatedChevronUpDownSmallVisualSource";
}
::windows::core::interface_hierarchy!(
    AnimatedChevronUpDownSmallVisualSource,
    ::windows::core::IUnknown,
    ::windows::core::IInspectable
);
impl ::core::convert::TryFrom<AnimatedChevronUpDownSmallVisualSource>
    for super::IAnimatedVisualSource
{
    type Error = ::windows::core::Error;
    fn try_from(value: AnimatedChevronUpDownSmallVisualSource) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AnimatedChevronUpDownSmallVisualSource>
    for super::IAnimatedVisualSource
{
    type Error = ::windows::core::Error;
    fn try_from(value: &AnimatedChevronUpDownSmallVisualSource) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&AnimatedChevronUpDownSmallVisualSource>
    for ::windows::core::InParam<'a, super::IAnimatedVisualSource>
{
    type Error = ::windows::core::Error;
    fn try_from(value: &AnimatedChevronUpDownSmallVisualSource) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<AnimatedChevronUpDownSmallVisualSource>
    for super::IAnimatedVisualSource2
{
    type Error = ::windows::core::Error;
    fn try_from(value: AnimatedChevronUpDownSmallVisualSource) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AnimatedChevronUpDownSmallVisualSource>
    for super::IAnimatedVisualSource2
{
    type Error = ::windows::core::Error;
    fn try_from(value: &AnimatedChevronUpDownSmallVisualSource) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&AnimatedChevronUpDownSmallVisualSource>
    for ::windows::core::InParam<'a, super::IAnimatedVisualSource2>
{
    type Error = ::windows::core::Error;
    fn try_from(value: &AnimatedChevronUpDownSmallVisualSource) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for AnimatedChevronUpDownSmallVisualSource {}
unsafe impl ::core::marker::Sync for AnimatedChevronUpDownSmallVisualSource {}
#[doc = "*Required features: `\"UI_Xaml_Controls_AnimatedVisuals\"`*"]
#[repr(transparent)]
pub struct AnimatedFindVisualSource(::windows::core::IUnknown);
impl AnimatedFindVisualSource {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<
            AnimatedFindVisualSource,
            ::windows::core::IGenericFactory,
        > = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"UI_Composition\"`*"]
    #[cfg(feature = "UI_Composition")]
    pub fn TryCreateAnimatedVisual(
        &self,
        compositor: &super::super::super::Composition::Compositor,
        diagnostics: &mut ::core::option::Option<::windows::core::IInspectable>,
    ) -> ::windows::core::Result<super::IAnimatedVisual> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TryCreateAnimatedVisual)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(compositor),
                diagnostics as *mut _ as _,
                result__.as_mut_ptr(),
            )
            .from_abi::<super::IAnimatedVisual>(result__)
        }
    }
    pub fn Markers(
        &self,
    ) -> ::windows::core::Result<
        ::windows::Foundation::Collections::IMapView<::windows::core::HSTRING, f64>,
    > {
        let this = &::windows::core::Interface::cast::<super::IAnimatedVisualSource2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            ( ::windows::core::Vtable::vtable ( this ) . Markers ) ( ::windows::core::Vtable::as_raw ( this ) , result__ . as_mut_ptr ( ) ) . from_abi:: < ::windows::Foundation::Collections:: IMapView :: < :: windows::core::HSTRING , f64 > > ( result__ )
        }
    }
    pub fn SetColorProperty(
        &self,
        propertyname: &::windows::core::HSTRING,
        value: ::windows::UI::Color,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IAnimatedVisualSource2>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetColorProperty)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(propertyname),
                value,
            )
            .ok()
        }
    }
}
impl ::core::clone::Clone for AnimatedFindVisualSource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AnimatedFindVisualSource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AnimatedFindVisualSource {}
impl ::core::fmt::Debug for AnimatedFindVisualSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AnimatedFindVisualSource").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AnimatedFindVisualSource {
    const SIGNATURE : ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice ( b"rc(Microsoft.UI.Xaml.Controls.AnimatedVisuals.AnimatedFindVisualSource;{294765c3-70e3-555c-9657-01fc4051169d})" ) ;
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for AnimatedFindVisualSource {
    type Vtable = super::IAnimatedVisualSource_Vtbl;
}
unsafe impl ::windows::core::Interface for AnimatedFindVisualSource {
    const IID: ::windows::core::GUID =
        <super::IAnimatedVisualSource as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AnimatedFindVisualSource {
    const NAME: &'static str =
        "Microsoft.UI.Xaml.Controls.AnimatedVisuals.AnimatedFindVisualSource";
}
::windows::core::interface_hierarchy!(
    AnimatedFindVisualSource,
    ::windows::core::IUnknown,
    ::windows::core::IInspectable
);
impl ::core::convert::TryFrom<AnimatedFindVisualSource> for super::IAnimatedVisualSource {
    type Error = ::windows::core::Error;
    fn try_from(value: AnimatedFindVisualSource) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AnimatedFindVisualSource> for super::IAnimatedVisualSource {
    type Error = ::windows::core::Error;
    fn try_from(value: &AnimatedFindVisualSource) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&AnimatedFindVisualSource>
    for ::windows::core::InParam<'a, super::IAnimatedVisualSource>
{
    type Error = ::windows::core::Error;
    fn try_from(value: &AnimatedFindVisualSource) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<AnimatedFindVisualSource> for super::IAnimatedVisualSource2 {
    type Error = ::windows::core::Error;
    fn try_from(value: AnimatedFindVisualSource) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AnimatedFindVisualSource> for super::IAnimatedVisualSource2 {
    type Error = ::windows::core::Error;
    fn try_from(value: &AnimatedFindVisualSource) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&AnimatedFindVisualSource>
    for ::windows::core::InParam<'a, super::IAnimatedVisualSource2>
{
    type Error = ::windows::core::Error;
    fn try_from(value: &AnimatedFindVisualSource) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for AnimatedFindVisualSource {}
unsafe impl ::core::marker::Sync for AnimatedFindVisualSource {}
#[doc = "*Required features: `\"UI_Xaml_Controls_AnimatedVisuals\"`*"]
#[repr(transparent)]
pub struct AnimatedGlobalNavigationButtonVisualSource(::windows::core::IUnknown);
impl AnimatedGlobalNavigationButtonVisualSource {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<
            AnimatedGlobalNavigationButtonVisualSource,
            ::windows::core::IGenericFactory,
        > = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"UI_Composition\"`*"]
    #[cfg(feature = "UI_Composition")]
    pub fn TryCreateAnimatedVisual(
        &self,
        compositor: &super::super::super::Composition::Compositor,
        diagnostics: &mut ::core::option::Option<::windows::core::IInspectable>,
    ) -> ::windows::core::Result<super::IAnimatedVisual> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TryCreateAnimatedVisual)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(compositor),
                diagnostics as *mut _ as _,
                result__.as_mut_ptr(),
            )
            .from_abi::<super::IAnimatedVisual>(result__)
        }
    }
    pub fn Markers(
        &self,
    ) -> ::windows::core::Result<
        ::windows::Foundation::Collections::IMapView<::windows::core::HSTRING, f64>,
    > {
        let this = &::windows::core::Interface::cast::<super::IAnimatedVisualSource2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            ( ::windows::core::Vtable::vtable ( this ) . Markers ) ( ::windows::core::Vtable::as_raw ( this ) , result__ . as_mut_ptr ( ) ) . from_abi:: < ::windows::Foundation::Collections:: IMapView :: < :: windows::core::HSTRING , f64 > > ( result__ )
        }
    }
    pub fn SetColorProperty(
        &self,
        propertyname: &::windows::core::HSTRING,
        value: ::windows::UI::Color,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IAnimatedVisualSource2>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetColorProperty)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(propertyname),
                value,
            )
            .ok()
        }
    }
}
impl ::core::clone::Clone for AnimatedGlobalNavigationButtonVisualSource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AnimatedGlobalNavigationButtonVisualSource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AnimatedGlobalNavigationButtonVisualSource {}
impl ::core::fmt::Debug for AnimatedGlobalNavigationButtonVisualSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AnimatedGlobalNavigationButtonVisualSource")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AnimatedGlobalNavigationButtonVisualSource {
    const SIGNATURE : ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice ( b"rc(Microsoft.UI.Xaml.Controls.AnimatedVisuals.AnimatedGlobalNavigationButtonVisualSource;{294765c3-70e3-555c-9657-01fc4051169d})" ) ;
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for AnimatedGlobalNavigationButtonVisualSource {
    type Vtable = super::IAnimatedVisualSource_Vtbl;
}
unsafe impl ::windows::core::Interface for AnimatedGlobalNavigationButtonVisualSource {
    const IID: ::windows::core::GUID =
        <super::IAnimatedVisualSource as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AnimatedGlobalNavigationButtonVisualSource {
    const NAME: &'static str =
        "Microsoft.UI.Xaml.Controls.AnimatedVisuals.AnimatedGlobalNavigationButtonVisualSource";
}
::windows::core::interface_hierarchy!(
    AnimatedGlobalNavigationButtonVisualSource,
    ::windows::core::IUnknown,
    ::windows::core::IInspectable
);
impl ::core::convert::TryFrom<AnimatedGlobalNavigationButtonVisualSource>
    for super::IAnimatedVisualSource
{
    type Error = ::windows::core::Error;
    fn try_from(
        value: AnimatedGlobalNavigationButtonVisualSource,
    ) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AnimatedGlobalNavigationButtonVisualSource>
    for super::IAnimatedVisualSource
{
    type Error = ::windows::core::Error;
    fn try_from(
        value: &AnimatedGlobalNavigationButtonVisualSource,
    ) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&AnimatedGlobalNavigationButtonVisualSource>
    for ::windows::core::InParam<'a, super::IAnimatedVisualSource>
{
    type Error = ::windows::core::Error;
    fn try_from(
        value: &AnimatedGlobalNavigationButtonVisualSource,
    ) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<AnimatedGlobalNavigationButtonVisualSource>
    for super::IAnimatedVisualSource2
{
    type Error = ::windows::core::Error;
    fn try_from(
        value: AnimatedGlobalNavigationButtonVisualSource,
    ) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AnimatedGlobalNavigationButtonVisualSource>
    for super::IAnimatedVisualSource2
{
    type Error = ::windows::core::Error;
    fn try_from(
        value: &AnimatedGlobalNavigationButtonVisualSource,
    ) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&AnimatedGlobalNavigationButtonVisualSource>
    for ::windows::core::InParam<'a, super::IAnimatedVisualSource2>
{
    type Error = ::windows::core::Error;
    fn try_from(
        value: &AnimatedGlobalNavigationButtonVisualSource,
    ) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for AnimatedGlobalNavigationButtonVisualSource {}
unsafe impl ::core::marker::Sync for AnimatedGlobalNavigationButtonVisualSource {}
#[doc = "*Required features: `\"UI_Xaml_Controls_AnimatedVisuals\"`*"]
#[repr(transparent)]
pub struct AnimatedSettingsVisualSource(::windows::core::IUnknown);
impl AnimatedSettingsVisualSource {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<
            AnimatedSettingsVisualSource,
            ::windows::core::IGenericFactory,
        > = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"UI_Composition\"`*"]
    #[cfg(feature = "UI_Composition")]
    pub fn TryCreateAnimatedVisual(
        &self,
        compositor: &super::super::super::Composition::Compositor,
        diagnostics: &mut ::core::option::Option<::windows::core::IInspectable>,
    ) -> ::windows::core::Result<super::IAnimatedVisual> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TryCreateAnimatedVisual)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(compositor),
                diagnostics as *mut _ as _,
                result__.as_mut_ptr(),
            )
            .from_abi::<super::IAnimatedVisual>(result__)
        }
    }
    pub fn Markers(
        &self,
    ) -> ::windows::core::Result<
        ::windows::Foundation::Collections::IMapView<::windows::core::HSTRING, f64>,
    > {
        let this = &::windows::core::Interface::cast::<super::IAnimatedVisualSource2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            ( ::windows::core::Vtable::vtable ( this ) . Markers ) ( ::windows::core::Vtable::as_raw ( this ) , result__ . as_mut_ptr ( ) ) . from_abi:: < ::windows::Foundation::Collections:: IMapView :: < :: windows::core::HSTRING , f64 > > ( result__ )
        }
    }
    pub fn SetColorProperty(
        &self,
        propertyname: &::windows::core::HSTRING,
        value: ::windows::UI::Color,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IAnimatedVisualSource2>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetColorProperty)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(propertyname),
                value,
            )
            .ok()
        }
    }
}
impl ::core::clone::Clone for AnimatedSettingsVisualSource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AnimatedSettingsVisualSource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AnimatedSettingsVisualSource {}
impl ::core::fmt::Debug for AnimatedSettingsVisualSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AnimatedSettingsVisualSource").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AnimatedSettingsVisualSource {
    const SIGNATURE : ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice ( b"rc(Microsoft.UI.Xaml.Controls.AnimatedVisuals.AnimatedSettingsVisualSource;{294765c3-70e3-555c-9657-01fc4051169d})" ) ;
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for AnimatedSettingsVisualSource {
    type Vtable = super::IAnimatedVisualSource_Vtbl;
}
unsafe impl ::windows::core::Interface for AnimatedSettingsVisualSource {
    const IID: ::windows::core::GUID =
        <super::IAnimatedVisualSource as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AnimatedSettingsVisualSource {
    const NAME: &'static str =
        "Microsoft.UI.Xaml.Controls.AnimatedVisuals.AnimatedSettingsVisualSource";
}
::windows::core::interface_hierarchy!(
    AnimatedSettingsVisualSource,
    ::windows::core::IUnknown,
    ::windows::core::IInspectable
);
impl ::core::convert::TryFrom<AnimatedSettingsVisualSource> for super::IAnimatedVisualSource {
    type Error = ::windows::core::Error;
    fn try_from(value: AnimatedSettingsVisualSource) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AnimatedSettingsVisualSource> for super::IAnimatedVisualSource {
    type Error = ::windows::core::Error;
    fn try_from(value: &AnimatedSettingsVisualSource) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&AnimatedSettingsVisualSource>
    for ::windows::core::InParam<'a, super::IAnimatedVisualSource>
{
    type Error = ::windows::core::Error;
    fn try_from(value: &AnimatedSettingsVisualSource) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<AnimatedSettingsVisualSource> for super::IAnimatedVisualSource2 {
    type Error = ::windows::core::Error;
    fn try_from(value: AnimatedSettingsVisualSource) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AnimatedSettingsVisualSource> for super::IAnimatedVisualSource2 {
    type Error = ::windows::core::Error;
    fn try_from(value: &AnimatedSettingsVisualSource) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&AnimatedSettingsVisualSource>
    for ::windows::core::InParam<'a, super::IAnimatedVisualSource2>
{
    type Error = ::windows::core::Error;
    fn try_from(value: &AnimatedSettingsVisualSource) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for AnimatedSettingsVisualSource {}
unsafe impl ::core::marker::Sync for AnimatedSettingsVisualSource {}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
