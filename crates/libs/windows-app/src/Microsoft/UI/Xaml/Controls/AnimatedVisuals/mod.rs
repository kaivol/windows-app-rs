#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct AnimatedAcceptVisualSource(::windows_core::IUnknown);
impl AnimatedAcceptVisualSource {
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
            AnimatedAcceptVisualSource,
            ::windows_core::imp::IGenericFactory,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "Required features: `\"Microsoft_UI_Composition\"`"]
    #[cfg(feature = "Microsoft_UI_Composition")]
    pub fn TryCreateAnimatedVisual<P0>(
        &self,
        compositor: P0,
        diagnostics: &mut ::core::option::Option<::windows_core::IInspectable>,
    ) -> ::windows_core::Result<super::IAnimatedVisual>
    where
        P0: ::windows_core::IntoParam<super::super::super::Composition::Compositor>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TryCreateAnimatedVisual)(
                ::windows_core::Interface::as_raw(this),
                compositor.into_param().abi(),
                diagnostics as *mut _ as _,
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation_Collections\"`"]
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub fn Markers(
        &self,
    ) -> ::windows_core::Result<
        ::windows::Foundation::Collections::IMapView<::windows_core::HSTRING, f64>,
    > {
        let this = &::windows_core::ComInterface::cast::<super::IAnimatedVisualSource2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Markers)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn SetColorProperty(
        &self,
        propertyname: &::windows_core::HSTRING,
        value: ::windows::UI::Color,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::IAnimatedVisualSource2>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetColorProperty)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(propertyname),
                value,
            )
            .ok()
        }
    }
}
impl ::windows_core::RuntimeType for AnimatedAcceptVisualSource {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for AnimatedAcceptVisualSource {
    type Vtable = super::IAnimatedVisualSource_Vtbl;
}
unsafe impl ::windows_core::ComInterface for AnimatedAcceptVisualSource {
    const IID: ::windows_core::GUID =
        <super::IAnimatedVisualSource as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for AnimatedAcceptVisualSource {
    const NAME: &'static str =
        "Microsoft.UI.Xaml.Controls.AnimatedVisuals.AnimatedAcceptVisualSource";
}
::windows_core::imp::interface_hierarchy!(
    AnimatedAcceptVisualSource,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<super::IAnimatedVisualSource> for AnimatedAcceptVisualSource {}
impl ::windows_core::CanTryInto<super::IAnimatedVisualSource2> for AnimatedAcceptVisualSource {}
unsafe impl ::core::marker::Send for AnimatedAcceptVisualSource {}
unsafe impl ::core::marker::Sync for AnimatedAcceptVisualSource {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct AnimatedBackVisualSource(::windows_core::IUnknown);
impl AnimatedBackVisualSource {
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
            AnimatedBackVisualSource,
            ::windows_core::imp::IGenericFactory,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "Required features: `\"Microsoft_UI_Composition\"`"]
    #[cfg(feature = "Microsoft_UI_Composition")]
    pub fn TryCreateAnimatedVisual<P0>(
        &self,
        compositor: P0,
        diagnostics: &mut ::core::option::Option<::windows_core::IInspectable>,
    ) -> ::windows_core::Result<super::IAnimatedVisual>
    where
        P0: ::windows_core::IntoParam<super::super::super::Composition::Compositor>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TryCreateAnimatedVisual)(
                ::windows_core::Interface::as_raw(this),
                compositor.into_param().abi(),
                diagnostics as *mut _ as _,
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation_Collections\"`"]
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub fn Markers(
        &self,
    ) -> ::windows_core::Result<
        ::windows::Foundation::Collections::IMapView<::windows_core::HSTRING, f64>,
    > {
        let this = &::windows_core::ComInterface::cast::<super::IAnimatedVisualSource2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Markers)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn SetColorProperty(
        &self,
        propertyname: &::windows_core::HSTRING,
        value: ::windows::UI::Color,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::IAnimatedVisualSource2>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetColorProperty)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(propertyname),
                value,
            )
            .ok()
        }
    }
}
impl ::windows_core::RuntimeType for AnimatedBackVisualSource {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for AnimatedBackVisualSource {
    type Vtable = super::IAnimatedVisualSource_Vtbl;
}
unsafe impl ::windows_core::ComInterface for AnimatedBackVisualSource {
    const IID: ::windows_core::GUID =
        <super::IAnimatedVisualSource as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for AnimatedBackVisualSource {
    const NAME: &'static str =
        "Microsoft.UI.Xaml.Controls.AnimatedVisuals.AnimatedBackVisualSource";
}
::windows_core::imp::interface_hierarchy!(
    AnimatedBackVisualSource,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<super::IAnimatedVisualSource> for AnimatedBackVisualSource {}
impl ::windows_core::CanTryInto<super::IAnimatedVisualSource2> for AnimatedBackVisualSource {}
unsafe impl ::core::marker::Send for AnimatedBackVisualSource {}
unsafe impl ::core::marker::Sync for AnimatedBackVisualSource {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct AnimatedChevronDownSmallVisualSource(::windows_core::IUnknown);
impl AnimatedChevronDownSmallVisualSource {
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
            AnimatedChevronDownSmallVisualSource,
            ::windows_core::imp::IGenericFactory,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "Required features: `\"Microsoft_UI_Composition\"`"]
    #[cfg(feature = "Microsoft_UI_Composition")]
    pub fn TryCreateAnimatedVisual<P0>(
        &self,
        compositor: P0,
        diagnostics: &mut ::core::option::Option<::windows_core::IInspectable>,
    ) -> ::windows_core::Result<super::IAnimatedVisual>
    where
        P0: ::windows_core::IntoParam<super::super::super::Composition::Compositor>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TryCreateAnimatedVisual)(
                ::windows_core::Interface::as_raw(this),
                compositor.into_param().abi(),
                diagnostics as *mut _ as _,
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation_Collections\"`"]
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub fn Markers(
        &self,
    ) -> ::windows_core::Result<
        ::windows::Foundation::Collections::IMapView<::windows_core::HSTRING, f64>,
    > {
        let this = &::windows_core::ComInterface::cast::<super::IAnimatedVisualSource2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Markers)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn SetColorProperty(
        &self,
        propertyname: &::windows_core::HSTRING,
        value: ::windows::UI::Color,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::IAnimatedVisualSource2>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetColorProperty)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(propertyname),
                value,
            )
            .ok()
        }
    }
}
impl ::windows_core::RuntimeType for AnimatedChevronDownSmallVisualSource {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for AnimatedChevronDownSmallVisualSource {
    type Vtable = super::IAnimatedVisualSource_Vtbl;
}
unsafe impl ::windows_core::ComInterface for AnimatedChevronDownSmallVisualSource {
    const IID: ::windows_core::GUID =
        <super::IAnimatedVisualSource as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for AnimatedChevronDownSmallVisualSource {
    const NAME: &'static str =
        "Microsoft.UI.Xaml.Controls.AnimatedVisuals.AnimatedChevronDownSmallVisualSource";
}
::windows_core::imp::interface_hierarchy!(
    AnimatedChevronDownSmallVisualSource,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<super::IAnimatedVisualSource>
    for AnimatedChevronDownSmallVisualSource
{
}
impl ::windows_core::CanTryInto<super::IAnimatedVisualSource2>
    for AnimatedChevronDownSmallVisualSource
{
}
unsafe impl ::core::marker::Send for AnimatedChevronDownSmallVisualSource {}
unsafe impl ::core::marker::Sync for AnimatedChevronDownSmallVisualSource {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct AnimatedChevronRightDownSmallVisualSource(::windows_core::IUnknown);
impl AnimatedChevronRightDownSmallVisualSource {
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
            AnimatedChevronRightDownSmallVisualSource,
            ::windows_core::imp::IGenericFactory,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "Required features: `\"Microsoft_UI_Composition\"`"]
    #[cfg(feature = "Microsoft_UI_Composition")]
    pub fn TryCreateAnimatedVisual<P0>(
        &self,
        compositor: P0,
        diagnostics: &mut ::core::option::Option<::windows_core::IInspectable>,
    ) -> ::windows_core::Result<super::IAnimatedVisual>
    where
        P0: ::windows_core::IntoParam<super::super::super::Composition::Compositor>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TryCreateAnimatedVisual)(
                ::windows_core::Interface::as_raw(this),
                compositor.into_param().abi(),
                diagnostics as *mut _ as _,
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation_Collections\"`"]
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub fn Markers(
        &self,
    ) -> ::windows_core::Result<
        ::windows::Foundation::Collections::IMapView<::windows_core::HSTRING, f64>,
    > {
        let this = &::windows_core::ComInterface::cast::<super::IAnimatedVisualSource2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Markers)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn SetColorProperty(
        &self,
        propertyname: &::windows_core::HSTRING,
        value: ::windows::UI::Color,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::IAnimatedVisualSource2>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetColorProperty)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(propertyname),
                value,
            )
            .ok()
        }
    }
}
impl ::windows_core::RuntimeType for AnimatedChevronRightDownSmallVisualSource {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for AnimatedChevronRightDownSmallVisualSource {
    type Vtable = super::IAnimatedVisualSource_Vtbl;
}
unsafe impl ::windows_core::ComInterface for AnimatedChevronRightDownSmallVisualSource {
    const IID: ::windows_core::GUID =
        <super::IAnimatedVisualSource as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for AnimatedChevronRightDownSmallVisualSource {
    const NAME: &'static str =
        "Microsoft.UI.Xaml.Controls.AnimatedVisuals.AnimatedChevronRightDownSmallVisualSource";
}
::windows_core::imp::interface_hierarchy!(
    AnimatedChevronRightDownSmallVisualSource,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<super::IAnimatedVisualSource>
    for AnimatedChevronRightDownSmallVisualSource
{
}
impl ::windows_core::CanTryInto<super::IAnimatedVisualSource2>
    for AnimatedChevronRightDownSmallVisualSource
{
}
unsafe impl ::core::marker::Send for AnimatedChevronRightDownSmallVisualSource {}
unsafe impl ::core::marker::Sync for AnimatedChevronRightDownSmallVisualSource {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct AnimatedChevronUpDownSmallVisualSource(::windows_core::IUnknown);
impl AnimatedChevronUpDownSmallVisualSource {
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
            AnimatedChevronUpDownSmallVisualSource,
            ::windows_core::imp::IGenericFactory,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "Required features: `\"Microsoft_UI_Composition\"`"]
    #[cfg(feature = "Microsoft_UI_Composition")]
    pub fn TryCreateAnimatedVisual<P0>(
        &self,
        compositor: P0,
        diagnostics: &mut ::core::option::Option<::windows_core::IInspectable>,
    ) -> ::windows_core::Result<super::IAnimatedVisual>
    where
        P0: ::windows_core::IntoParam<super::super::super::Composition::Compositor>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TryCreateAnimatedVisual)(
                ::windows_core::Interface::as_raw(this),
                compositor.into_param().abi(),
                diagnostics as *mut _ as _,
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation_Collections\"`"]
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub fn Markers(
        &self,
    ) -> ::windows_core::Result<
        ::windows::Foundation::Collections::IMapView<::windows_core::HSTRING, f64>,
    > {
        let this = &::windows_core::ComInterface::cast::<super::IAnimatedVisualSource2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Markers)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn SetColorProperty(
        &self,
        propertyname: &::windows_core::HSTRING,
        value: ::windows::UI::Color,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::IAnimatedVisualSource2>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetColorProperty)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(propertyname),
                value,
            )
            .ok()
        }
    }
}
impl ::windows_core::RuntimeType for AnimatedChevronUpDownSmallVisualSource {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for AnimatedChevronUpDownSmallVisualSource {
    type Vtable = super::IAnimatedVisualSource_Vtbl;
}
unsafe impl ::windows_core::ComInterface for AnimatedChevronUpDownSmallVisualSource {
    const IID: ::windows_core::GUID =
        <super::IAnimatedVisualSource as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for AnimatedChevronUpDownSmallVisualSource {
    const NAME: &'static str =
        "Microsoft.UI.Xaml.Controls.AnimatedVisuals.AnimatedChevronUpDownSmallVisualSource";
}
::windows_core::imp::interface_hierarchy!(
    AnimatedChevronUpDownSmallVisualSource,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<super::IAnimatedVisualSource>
    for AnimatedChevronUpDownSmallVisualSource
{
}
impl ::windows_core::CanTryInto<super::IAnimatedVisualSource2>
    for AnimatedChevronUpDownSmallVisualSource
{
}
unsafe impl ::core::marker::Send for AnimatedChevronUpDownSmallVisualSource {}
unsafe impl ::core::marker::Sync for AnimatedChevronUpDownSmallVisualSource {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct AnimatedFindVisualSource(::windows_core::IUnknown);
impl AnimatedFindVisualSource {
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
            AnimatedFindVisualSource,
            ::windows_core::imp::IGenericFactory,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "Required features: `\"Microsoft_UI_Composition\"`"]
    #[cfg(feature = "Microsoft_UI_Composition")]
    pub fn TryCreateAnimatedVisual<P0>(
        &self,
        compositor: P0,
        diagnostics: &mut ::core::option::Option<::windows_core::IInspectable>,
    ) -> ::windows_core::Result<super::IAnimatedVisual>
    where
        P0: ::windows_core::IntoParam<super::super::super::Composition::Compositor>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TryCreateAnimatedVisual)(
                ::windows_core::Interface::as_raw(this),
                compositor.into_param().abi(),
                diagnostics as *mut _ as _,
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation_Collections\"`"]
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub fn Markers(
        &self,
    ) -> ::windows_core::Result<
        ::windows::Foundation::Collections::IMapView<::windows_core::HSTRING, f64>,
    > {
        let this = &::windows_core::ComInterface::cast::<super::IAnimatedVisualSource2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Markers)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn SetColorProperty(
        &self,
        propertyname: &::windows_core::HSTRING,
        value: ::windows::UI::Color,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::IAnimatedVisualSource2>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetColorProperty)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(propertyname),
                value,
            )
            .ok()
        }
    }
}
impl ::windows_core::RuntimeType for AnimatedFindVisualSource {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for AnimatedFindVisualSource {
    type Vtable = super::IAnimatedVisualSource_Vtbl;
}
unsafe impl ::windows_core::ComInterface for AnimatedFindVisualSource {
    const IID: ::windows_core::GUID =
        <super::IAnimatedVisualSource as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for AnimatedFindVisualSource {
    const NAME: &'static str =
        "Microsoft.UI.Xaml.Controls.AnimatedVisuals.AnimatedFindVisualSource";
}
::windows_core::imp::interface_hierarchy!(
    AnimatedFindVisualSource,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<super::IAnimatedVisualSource> for AnimatedFindVisualSource {}
impl ::windows_core::CanTryInto<super::IAnimatedVisualSource2> for AnimatedFindVisualSource {}
unsafe impl ::core::marker::Send for AnimatedFindVisualSource {}
unsafe impl ::core::marker::Sync for AnimatedFindVisualSource {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct AnimatedGlobalNavigationButtonVisualSource(::windows_core::IUnknown);
impl AnimatedGlobalNavigationButtonVisualSource {
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
            AnimatedGlobalNavigationButtonVisualSource,
            ::windows_core::imp::IGenericFactory,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "Required features: `\"Microsoft_UI_Composition\"`"]
    #[cfg(feature = "Microsoft_UI_Composition")]
    pub fn TryCreateAnimatedVisual<P0>(
        &self,
        compositor: P0,
        diagnostics: &mut ::core::option::Option<::windows_core::IInspectable>,
    ) -> ::windows_core::Result<super::IAnimatedVisual>
    where
        P0: ::windows_core::IntoParam<super::super::super::Composition::Compositor>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TryCreateAnimatedVisual)(
                ::windows_core::Interface::as_raw(this),
                compositor.into_param().abi(),
                diagnostics as *mut _ as _,
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation_Collections\"`"]
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub fn Markers(
        &self,
    ) -> ::windows_core::Result<
        ::windows::Foundation::Collections::IMapView<::windows_core::HSTRING, f64>,
    > {
        let this = &::windows_core::ComInterface::cast::<super::IAnimatedVisualSource2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Markers)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn SetColorProperty(
        &self,
        propertyname: &::windows_core::HSTRING,
        value: ::windows::UI::Color,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::IAnimatedVisualSource2>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetColorProperty)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(propertyname),
                value,
            )
            .ok()
        }
    }
}
impl ::windows_core::RuntimeType for AnimatedGlobalNavigationButtonVisualSource {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for AnimatedGlobalNavigationButtonVisualSource {
    type Vtable = super::IAnimatedVisualSource_Vtbl;
}
unsafe impl ::windows_core::ComInterface for AnimatedGlobalNavigationButtonVisualSource {
    const IID: ::windows_core::GUID =
        <super::IAnimatedVisualSource as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for AnimatedGlobalNavigationButtonVisualSource {
    const NAME: &'static str =
        "Microsoft.UI.Xaml.Controls.AnimatedVisuals.AnimatedGlobalNavigationButtonVisualSource";
}
::windows_core::imp::interface_hierarchy!(
    AnimatedGlobalNavigationButtonVisualSource,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<super::IAnimatedVisualSource>
    for AnimatedGlobalNavigationButtonVisualSource
{
}
impl ::windows_core::CanTryInto<super::IAnimatedVisualSource2>
    for AnimatedGlobalNavigationButtonVisualSource
{
}
unsafe impl ::core::marker::Send for AnimatedGlobalNavigationButtonVisualSource {}
unsafe impl ::core::marker::Sync for AnimatedGlobalNavigationButtonVisualSource {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct AnimatedSettingsVisualSource(::windows_core::IUnknown);
impl AnimatedSettingsVisualSource {
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
            AnimatedSettingsVisualSource,
            ::windows_core::imp::IGenericFactory,
        > = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "Required features: `\"Microsoft_UI_Composition\"`"]
    #[cfg(feature = "Microsoft_UI_Composition")]
    pub fn TryCreateAnimatedVisual<P0>(
        &self,
        compositor: P0,
        diagnostics: &mut ::core::option::Option<::windows_core::IInspectable>,
    ) -> ::windows_core::Result<super::IAnimatedVisual>
    where
        P0: ::windows_core::IntoParam<super::super::super::Composition::Compositor>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TryCreateAnimatedVisual)(
                ::windows_core::Interface::as_raw(this),
                compositor.into_param().abi(),
                diagnostics as *mut _ as _,
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_Foundation_Collections\"`"]
    #[cfg(feature = "Windows_Foundation_Collections")]
    pub fn Markers(
        &self,
    ) -> ::windows_core::Result<
        ::windows::Foundation::Collections::IMapView<::windows_core::HSTRING, f64>,
    > {
        let this = &::windows_core::ComInterface::cast::<super::IAnimatedVisualSource2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Markers)(
                ::windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Windows_UI\"`"]
    #[cfg(feature = "Windows_UI")]
    pub fn SetColorProperty(
        &self,
        propertyname: &::windows_core::HSTRING,
        value: ::windows::UI::Color,
    ) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::IAnimatedVisualSource2>(self)?;
        unsafe {
            (::windows_core::Interface::vtable(this).SetColorProperty)(
                ::windows_core::Interface::as_raw(this),
                ::core::mem::transmute_copy(propertyname),
                value,
            )
            .ok()
        }
    }
}
impl ::windows_core::RuntimeType for AnimatedSettingsVisualSource {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for AnimatedSettingsVisualSource {
    type Vtable = super::IAnimatedVisualSource_Vtbl;
}
unsafe impl ::windows_core::ComInterface for AnimatedSettingsVisualSource {
    const IID: ::windows_core::GUID =
        <super::IAnimatedVisualSource as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for AnimatedSettingsVisualSource {
    const NAME: &'static str =
        "Microsoft.UI.Xaml.Controls.AnimatedVisuals.AnimatedSettingsVisualSource";
}
::windows_core::imp::interface_hierarchy!(
    AnimatedSettingsVisualSource,
    ::windows_core::IUnknown,
    ::windows_core::IInspectable
);
impl ::windows_core::CanTryInto<super::IAnimatedVisualSource> for AnimatedSettingsVisualSource {}
impl ::windows_core::CanTryInto<super::IAnimatedVisualSource2> for AnimatedSettingsVisualSource {}
unsafe impl ::core::marker::Send for AnimatedSettingsVisualSource {}
unsafe impl ::core::marker::Sync for AnimatedSettingsVisualSource {}
