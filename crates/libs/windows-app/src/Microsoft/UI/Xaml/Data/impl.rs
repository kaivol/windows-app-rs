#[doc = "Required features: `\"Windows_Foundation_Collections\"`"]
#[cfg(feature = "Windows_Foundation_Collections")]
pub trait ICollectionView_Impl:
    Sized
    + ::windows::Foundation::Collections::IIterable_Impl<::windows_core::IInspectable>
    + ::windows::Foundation::Collections::IObservableVector_Impl<::windows_core::IInspectable>
    + ::windows::Foundation::Collections::IVector_Impl<::windows_core::IInspectable>
{
    fn CurrentItem(&self) -> ::windows_core::Result<::windows_core::IInspectable>;
    fn CurrentPosition(&self) -> ::windows_core::Result<i32>;
    fn IsCurrentAfterLast(&self) -> ::windows_core::Result<bool>;
    fn IsCurrentBeforeFirst(&self) -> ::windows_core::Result<bool>;
    fn CollectionGroups(
        &self,
    ) -> ::windows_core::Result<
        ::windows::Foundation::Collections::IObservableVector<::windows_core::IInspectable>,
    >;
    fn HasMoreItems(&self) -> ::windows_core::Result<bool>;
    fn CurrentChanged(
        &self,
        handler: ::core::option::Option<
            &::windows::Foundation::EventHandler<::windows_core::IInspectable>,
        >,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>;
    fn RemoveCurrentChanged(
        &self,
        token: &::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()>;
    fn CurrentChanging(
        &self,
        handler: ::core::option::Option<&CurrentChangingEventHandler>,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>;
    fn RemoveCurrentChanging(
        &self,
        token: &::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()>;
    fn MoveCurrentTo(
        &self,
        item: ::core::option::Option<&::windows_core::IInspectable>,
    ) -> ::windows_core::Result<bool>;
    fn MoveCurrentToPosition(&self, index: i32) -> ::windows_core::Result<bool>;
    fn MoveCurrentToFirst(&self) -> ::windows_core::Result<bool>;
    fn MoveCurrentToLast(&self) -> ::windows_core::Result<bool>;
    fn MoveCurrentToNext(&self) -> ::windows_core::Result<bool>;
    fn MoveCurrentToPrevious(&self) -> ::windows_core::Result<bool>;
    fn LoadMoreItemsAsync(
        &self,
        count: u32,
    ) -> ::windows_core::Result<::windows::Foundation::IAsyncOperation<LoadMoreItemsResult>>;
}
#[cfg(feature = "Windows_Foundation_Collections")]
impl ::windows_core::RuntimeName for ICollectionView {
    const NAME: &'static str = "Microsoft.UI.Xaml.Data.ICollectionView";
}
#[cfg(feature = "Windows_Foundation_Collections")]
impl ICollectionView_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: ICollectionView_Impl,
        const OFFSET: isize,
    >() -> ICollectionView_Vtbl {
        unsafe extern "system" fn CurrentItem<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: ICollectionView_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentItem() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentPosition<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: ICollectionView_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut i32,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentPosition() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsCurrentAfterLast<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: ICollectionView_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut bool,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsCurrentAfterLast() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsCurrentBeforeFirst<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: ICollectionView_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut bool,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsCurrentBeforeFirst() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CollectionGroups<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: ICollectionView_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CollectionGroups() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HasMoreItems<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: ICollectionView_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut bool,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.HasMoreItems() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentChanged<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: ICollectionView_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            handler: *mut ::core::ffi::c_void,
            result__: *mut ::windows::Foundation::EventRegistrationToken,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentChanged(::windows_core::from_raw_borrowed(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveCurrentChanged<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: ICollectionView_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            token: ::windows::Foundation::EventRegistrationToken,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveCurrentChanged(::core::mem::transmute(&token)).into()
        }
        unsafe extern "system" fn CurrentChanging<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: ICollectionView_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            handler: *mut ::core::ffi::c_void,
            result__: *mut ::windows::Foundation::EventRegistrationToken,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentChanging(::windows_core::from_raw_borrowed(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveCurrentChanging<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: ICollectionView_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            token: ::windows::Foundation::EventRegistrationToken,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveCurrentChanging(::core::mem::transmute(&token)).into()
        }
        unsafe extern "system" fn MoveCurrentTo<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: ICollectionView_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            item: *mut ::core::ffi::c_void,
            result__: *mut bool,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MoveCurrentTo(::windows_core::from_raw_borrowed(&item)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveCurrentToPosition<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: ICollectionView_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            index: i32,
            result__: *mut bool,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MoveCurrentToPosition(index) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveCurrentToFirst<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: ICollectionView_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut bool,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MoveCurrentToFirst() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveCurrentToLast<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: ICollectionView_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut bool,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MoveCurrentToLast() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveCurrentToNext<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: ICollectionView_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut bool,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MoveCurrentToNext() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveCurrentToPrevious<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: ICollectionView_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut bool,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MoveCurrentToPrevious() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LoadMoreItemsAsync<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: ICollectionView_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            count: u32,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LoadMoreItemsAsync(count) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<Identity, ICollectionView, OFFSET>(),
            CurrentItem: CurrentItem::<Identity, Impl, OFFSET>,
            CurrentPosition: CurrentPosition::<Identity, Impl, OFFSET>,
            IsCurrentAfterLast: IsCurrentAfterLast::<Identity, Impl, OFFSET>,
            IsCurrentBeforeFirst: IsCurrentBeforeFirst::<Identity, Impl, OFFSET>,
            CollectionGroups: CollectionGroups::<Identity, Impl, OFFSET>,
            HasMoreItems: HasMoreItems::<Identity, Impl, OFFSET>,
            CurrentChanged: CurrentChanged::<Identity, Impl, OFFSET>,
            RemoveCurrentChanged: RemoveCurrentChanged::<Identity, Impl, OFFSET>,
            CurrentChanging: CurrentChanging::<Identity, Impl, OFFSET>,
            RemoveCurrentChanging: RemoveCurrentChanging::<Identity, Impl, OFFSET>,
            MoveCurrentTo: MoveCurrentTo::<Identity, Impl, OFFSET>,
            MoveCurrentToPosition: MoveCurrentToPosition::<Identity, Impl, OFFSET>,
            MoveCurrentToFirst: MoveCurrentToFirst::<Identity, Impl, OFFSET>,
            MoveCurrentToLast: MoveCurrentToLast::<Identity, Impl, OFFSET>,
            MoveCurrentToNext: MoveCurrentToNext::<Identity, Impl, OFFSET>,
            MoveCurrentToPrevious: MoveCurrentToPrevious::<Identity, Impl, OFFSET>,
            LoadMoreItemsAsync: LoadMoreItemsAsync::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <ICollectionView as ::windows_core::ComInterface>::IID
    }
}
pub trait ICollectionViewFactory_Impl: Sized {
    fn CreateView(&self) -> ::windows_core::Result<ICollectionView>;
}
impl ::windows_core::RuntimeName for ICollectionViewFactory {
    const NAME: &'static str = "Microsoft.UI.Xaml.Data.ICollectionViewFactory";
}
impl ICollectionViewFactory_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: ICollectionViewFactory_Impl,
        const OFFSET: isize,
    >() -> ICollectionViewFactory_Vtbl {
        unsafe extern "system" fn CreateView<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: ICollectionViewFactory_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateView() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<
                Identity,
                ICollectionViewFactory,
                OFFSET,
            >(),
            CreateView: CreateView::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <ICollectionViewFactory as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Windows_Foundation_Collections\"`"]
#[cfg(feature = "Windows_Foundation_Collections")]
pub trait ICollectionViewGroup_Impl: Sized {
    fn Group(&self) -> ::windows_core::Result<::windows_core::IInspectable>;
    fn GroupItems(
        &self,
    ) -> ::windows_core::Result<
        ::windows::Foundation::Collections::IObservableVector<::windows_core::IInspectable>,
    >;
}
#[cfg(feature = "Windows_Foundation_Collections")]
impl ::windows_core::RuntimeName for ICollectionViewGroup {
    const NAME: &'static str = "Microsoft.UI.Xaml.Data.ICollectionViewGroup";
}
#[cfg(feature = "Windows_Foundation_Collections")]
impl ICollectionViewGroup_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: ICollectionViewGroup_Impl,
        const OFFSET: isize,
    >() -> ICollectionViewGroup_Vtbl {
        unsafe extern "system" fn Group<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: ICollectionViewGroup_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Group() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GroupItems<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: ICollectionViewGroup_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GroupItems() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<Identity, ICollectionViewGroup, OFFSET>(
            ),
            Group: Group::<Identity, Impl, OFFSET>,
            GroupItems: GroupItems::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <ICollectionViewGroup as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Windows_UI_Xaml_Interop\"`"]
#[cfg(feature = "Windows_UI_Xaml_Interop")]
pub trait ICustomProperty_Impl: Sized {
    fn Type(
        &self,
    ) -> ::windows_core::Result<super::super::super::super::Windows::UI::Xaml::Interop::TypeName>;
    fn Name(&self) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn GetValue(
        &self,
        target: ::core::option::Option<&::windows_core::IInspectable>,
    ) -> ::windows_core::Result<::windows_core::IInspectable>;
    fn SetValue(
        &self,
        target: ::core::option::Option<&::windows_core::IInspectable>,
        value: ::core::option::Option<&::windows_core::IInspectable>,
    ) -> ::windows_core::Result<()>;
    fn GetIndexedValue(
        &self,
        target: ::core::option::Option<&::windows_core::IInspectable>,
        index: ::core::option::Option<&::windows_core::IInspectable>,
    ) -> ::windows_core::Result<::windows_core::IInspectable>;
    fn SetIndexedValue(
        &self,
        target: ::core::option::Option<&::windows_core::IInspectable>,
        value: ::core::option::Option<&::windows_core::IInspectable>,
        index: ::core::option::Option<&::windows_core::IInspectable>,
    ) -> ::windows_core::Result<()>;
    fn CanWrite(&self) -> ::windows_core::Result<bool>;
    fn CanRead(&self) -> ::windows_core::Result<bool>;
}
#[cfg(feature = "Windows_UI_Xaml_Interop")]
impl ::windows_core::RuntimeName for ICustomProperty {
    const NAME: &'static str = "Microsoft.UI.Xaml.Data.ICustomProperty";
}
#[cfg(feature = "Windows_UI_Xaml_Interop")]
impl ICustomProperty_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: ICustomProperty_Impl,
        const OFFSET: isize,
    >() -> ICustomProperty_Vtbl {
        unsafe extern "system" fn Type<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: ICustomProperty_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::std::mem::MaybeUninit<
                super::super::super::super::Windows::UI::Xaml::Interop::TypeName,
            >,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Type() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: ICustomProperty_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Name() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetValue<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: ICustomProperty_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            target: *mut ::core::ffi::c_void,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetValue(::windows_core::from_raw_borrowed(&target)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValue<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: ICustomProperty_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            target: *mut ::core::ffi::c_void,
            value: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetValue(
                ::windows_core::from_raw_borrowed(&target),
                ::windows_core::from_raw_borrowed(&value),
            )
            .into()
        }
        unsafe extern "system" fn GetIndexedValue<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: ICustomProperty_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            target: *mut ::core::ffi::c_void,
            index: *mut ::core::ffi::c_void,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetIndexedValue(
                ::windows_core::from_raw_borrowed(&target),
                ::windows_core::from_raw_borrowed(&index),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIndexedValue<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: ICustomProperty_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            target: *mut ::core::ffi::c_void,
            value: *mut ::core::ffi::c_void,
            index: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetIndexedValue(
                ::windows_core::from_raw_borrowed(&target),
                ::windows_core::from_raw_borrowed(&value),
                ::windows_core::from_raw_borrowed(&index),
            )
            .into()
        }
        unsafe extern "system" fn CanWrite<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: ICustomProperty_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut bool,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CanWrite() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanRead<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: ICustomProperty_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut bool,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CanRead() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<Identity, ICustomProperty, OFFSET>(),
            Type: Type::<Identity, Impl, OFFSET>,
            Name: Name::<Identity, Impl, OFFSET>,
            GetValue: GetValue::<Identity, Impl, OFFSET>,
            SetValue: SetValue::<Identity, Impl, OFFSET>,
            GetIndexedValue: GetIndexedValue::<Identity, Impl, OFFSET>,
            SetIndexedValue: SetIndexedValue::<Identity, Impl, OFFSET>,
            CanWrite: CanWrite::<Identity, Impl, OFFSET>,
            CanRead: CanRead::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <ICustomProperty as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Windows_UI_Xaml_Interop\"`"]
#[cfg(feature = "Windows_UI_Xaml_Interop")]
pub trait ICustomPropertyProvider_Impl: Sized {
    fn GetCustomProperty(
        &self,
        name: &::windows_core::HSTRING,
    ) -> ::windows_core::Result<ICustomProperty>;
    fn GetIndexedProperty(
        &self,
        name: &::windows_core::HSTRING,
        r#type: &super::super::super::super::Windows::UI::Xaml::Interop::TypeName,
    ) -> ::windows_core::Result<ICustomProperty>;
    fn GetStringRepresentation(&self) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn Type(
        &self,
    ) -> ::windows_core::Result<super::super::super::super::Windows::UI::Xaml::Interop::TypeName>;
}
#[cfg(feature = "Windows_UI_Xaml_Interop")]
impl ::windows_core::RuntimeName for ICustomPropertyProvider {
    const NAME: &'static str = "Microsoft.UI.Xaml.Data.ICustomPropertyProvider";
}
#[cfg(feature = "Windows_UI_Xaml_Interop")]
impl ICustomPropertyProvider_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: ICustomPropertyProvider_Impl,
        const OFFSET: isize,
    >() -> ICustomPropertyProvider_Vtbl {
        unsafe extern "system" fn GetCustomProperty<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: ICustomPropertyProvider_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            name: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCustomProperty(::core::mem::transmute(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIndexedProperty<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: ICustomPropertyProvider_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            name: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
            r#type: ::std::mem::MaybeUninit<
                super::super::super::super::Windows::UI::Xaml::Interop::TypeName,
            >,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this
                .GetIndexedProperty(::core::mem::transmute(&name), ::core::mem::transmute(&r#type))
            {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStringRepresentation<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: ICustomPropertyProvider_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetStringRepresentation() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Type<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: ICustomPropertyProvider_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::std::mem::MaybeUninit<
                super::super::super::super::Windows::UI::Xaml::Interop::TypeName,
            >,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Type() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<
                Identity,
                ICustomPropertyProvider,
                OFFSET,
            >(),
            GetCustomProperty: GetCustomProperty::<Identity, Impl, OFFSET>,
            GetIndexedProperty: GetIndexedProperty::<Identity, Impl, OFFSET>,
            GetStringRepresentation: GetStringRepresentation::<Identity, Impl, OFFSET>,
            Type: Type::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <ICustomPropertyProvider as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Windows_Foundation_Collections\"`"]
#[cfg(feature = "Windows_Foundation_Collections")]
pub trait IItemsRangeInfo_Impl: Sized + ::windows::Foundation::IClosable_Impl {
    fn RangesChanged(
        &self,
        visiblerange: ::core::option::Option<&ItemIndexRange>,
        trackeditems: ::core::option::Option<
            &::windows::Foundation::Collections::IVectorView<ItemIndexRange>,
        >,
    ) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Windows_Foundation_Collections")]
impl ::windows_core::RuntimeName for IItemsRangeInfo {
    const NAME: &'static str = "Microsoft.UI.Xaml.Data.IItemsRangeInfo";
}
#[cfg(feature = "Windows_Foundation_Collections")]
impl IItemsRangeInfo_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IItemsRangeInfo_Impl,
        const OFFSET: isize,
    >() -> IItemsRangeInfo_Vtbl {
        unsafe extern "system" fn RangesChanged<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IItemsRangeInfo_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            visiblerange: *mut ::core::ffi::c_void,
            trackeditems: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RangesChanged(
                ::windows_core::from_raw_borrowed(&visiblerange),
                ::windows_core::from_raw_borrowed(&trackeditems),
            )
            .into()
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<Identity, IItemsRangeInfo, OFFSET>(),
            RangesChanged: RangesChanged::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IItemsRangeInfo as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Windows_Foundation_Collections\"`"]
#[cfg(feature = "Windows_Foundation_Collections")]
pub trait INotifyDataErrorInfo_Impl: Sized {
    fn HasErrors(&self) -> ::windows_core::Result<bool>;
    fn ErrorsChanged(
        &self,
        handler: ::core::option::Option<
            &::windows::Foundation::EventHandler<DataErrorsChangedEventArgs>,
        >,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>;
    fn RemoveErrorsChanged(
        &self,
        token: &::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()>;
    fn GetErrors(
        &self,
        propertyname: &::windows_core::HSTRING,
    ) -> ::windows_core::Result<
        ::windows::Foundation::Collections::IIterable<::windows_core::IInspectable>,
    >;
}
#[cfg(feature = "Windows_Foundation_Collections")]
impl ::windows_core::RuntimeName for INotifyDataErrorInfo {
    const NAME: &'static str = "Microsoft.UI.Xaml.Data.INotifyDataErrorInfo";
}
#[cfg(feature = "Windows_Foundation_Collections")]
impl INotifyDataErrorInfo_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: INotifyDataErrorInfo_Impl,
        const OFFSET: isize,
    >() -> INotifyDataErrorInfo_Vtbl {
        unsafe extern "system" fn HasErrors<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: INotifyDataErrorInfo_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut bool,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.HasErrors() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ErrorsChanged<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: INotifyDataErrorInfo_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            handler: *mut ::core::ffi::c_void,
            result__: *mut ::windows::Foundation::EventRegistrationToken,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ErrorsChanged(::windows_core::from_raw_borrowed(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveErrorsChanged<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: INotifyDataErrorInfo_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            token: ::windows::Foundation::EventRegistrationToken,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveErrorsChanged(::core::mem::transmute(&token)).into()
        }
        unsafe extern "system" fn GetErrors<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: INotifyDataErrorInfo_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            propertyname: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetErrors(::core::mem::transmute(&propertyname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<Identity, INotifyDataErrorInfo, OFFSET>(
            ),
            HasErrors: HasErrors::<Identity, Impl, OFFSET>,
            ErrorsChanged: ErrorsChanged::<Identity, Impl, OFFSET>,
            RemoveErrorsChanged: RemoveErrorsChanged::<Identity, Impl, OFFSET>,
            GetErrors: GetErrors::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <INotifyDataErrorInfo as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Windows_Foundation\"`"]
#[cfg(feature = "Windows_Foundation")]
pub trait INotifyPropertyChanged_Impl: Sized {
    fn PropertyChanged(
        &self,
        handler: ::core::option::Option<&PropertyChangedEventHandler>,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>;
    fn RemovePropertyChanged(
        &self,
        token: &::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Windows_Foundation")]
impl ::windows_core::RuntimeName for INotifyPropertyChanged {
    const NAME: &'static str = "Microsoft.UI.Xaml.Data.INotifyPropertyChanged";
}
#[cfg(feature = "Windows_Foundation")]
impl INotifyPropertyChanged_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: INotifyPropertyChanged_Impl,
        const OFFSET: isize,
    >() -> INotifyPropertyChanged_Vtbl {
        unsafe extern "system" fn PropertyChanged<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: INotifyPropertyChanged_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            handler: *mut ::core::ffi::c_void,
            result__: *mut ::windows::Foundation::EventRegistrationToken,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PropertyChanged(::windows_core::from_raw_borrowed(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePropertyChanged<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: INotifyPropertyChanged_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            token: ::windows::Foundation::EventRegistrationToken,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemovePropertyChanged(::core::mem::transmute(&token)).into()
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<
                Identity,
                INotifyPropertyChanged,
                OFFSET,
            >(),
            PropertyChanged: PropertyChanged::<Identity, Impl, OFFSET>,
            RemovePropertyChanged: RemovePropertyChanged::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <INotifyPropertyChanged as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Windows_Foundation_Collections\"`"]
#[cfg(feature = "Windows_Foundation_Collections")]
pub trait ISelectionInfo_Impl: Sized {
    fn SelectRange(
        &self,
        itemindexrange: ::core::option::Option<&ItemIndexRange>,
    ) -> ::windows_core::Result<()>;
    fn DeselectRange(
        &self,
        itemindexrange: ::core::option::Option<&ItemIndexRange>,
    ) -> ::windows_core::Result<()>;
    fn IsSelected(&self, index: i32) -> ::windows_core::Result<bool>;
    fn GetSelectedRanges(
        &self,
    ) -> ::windows_core::Result<::windows::Foundation::Collections::IVectorView<ItemIndexRange>>;
}
#[cfg(feature = "Windows_Foundation_Collections")]
impl ::windows_core::RuntimeName for ISelectionInfo {
    const NAME: &'static str = "Microsoft.UI.Xaml.Data.ISelectionInfo";
}
#[cfg(feature = "Windows_Foundation_Collections")]
impl ISelectionInfo_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: ISelectionInfo_Impl,
        const OFFSET: isize,
    >() -> ISelectionInfo_Vtbl {
        unsafe extern "system" fn SelectRange<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: ISelectionInfo_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            itemindexrange: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SelectRange(::windows_core::from_raw_borrowed(&itemindexrange)).into()
        }
        unsafe extern "system" fn DeselectRange<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: ISelectionInfo_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            itemindexrange: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeselectRange(::windows_core::from_raw_borrowed(&itemindexrange)).into()
        }
        unsafe extern "system" fn IsSelected<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: ISelectionInfo_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            index: i32,
            result__: *mut bool,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsSelected(index) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSelectedRanges<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: ISelectionInfo_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSelectedRanges() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<Identity, ISelectionInfo, OFFSET>(),
            SelectRange: SelectRange::<Identity, Impl, OFFSET>,
            DeselectRange: DeselectRange::<Identity, Impl, OFFSET>,
            IsSelected: IsSelected::<Identity, Impl, OFFSET>,
            GetSelectedRanges: GetSelectedRanges::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <ISelectionInfo as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Windows_Foundation\"`"]
#[cfg(feature = "Windows_Foundation")]
pub trait ISupportIncrementalLoading_Impl: Sized {
    fn LoadMoreItemsAsync(
        &self,
        count: u32,
    ) -> ::windows_core::Result<::windows::Foundation::IAsyncOperation<LoadMoreItemsResult>>;
    fn HasMoreItems(&self) -> ::windows_core::Result<bool>;
}
#[cfg(feature = "Windows_Foundation")]
impl ::windows_core::RuntimeName for ISupportIncrementalLoading {
    const NAME: &'static str = "Microsoft.UI.Xaml.Data.ISupportIncrementalLoading";
}
#[cfg(feature = "Windows_Foundation")]
impl ISupportIncrementalLoading_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: ISupportIncrementalLoading_Impl,
        const OFFSET: isize,
    >() -> ISupportIncrementalLoading_Vtbl {
        unsafe extern "system" fn LoadMoreItemsAsync<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: ISupportIncrementalLoading_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            count: u32,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LoadMoreItemsAsync(count) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HasMoreItems<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: ISupportIncrementalLoading_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut bool,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.HasMoreItems() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<
                Identity,
                ISupportIncrementalLoading,
                OFFSET,
            >(),
            LoadMoreItemsAsync: LoadMoreItemsAsync::<Identity, Impl, OFFSET>,
            HasMoreItems: HasMoreItems::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <ISupportIncrementalLoading as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Windows_UI_Xaml_Interop\"`"]
#[cfg(feature = "Windows_UI_Xaml_Interop")]
pub trait IValueConverter_Impl: Sized {
    fn Convert(
        &self,
        value: ::core::option::Option<&::windows_core::IInspectable>,
        targettype: &super::super::super::super::Windows::UI::Xaml::Interop::TypeName,
        parameter: ::core::option::Option<&::windows_core::IInspectable>,
        language: &::windows_core::HSTRING,
    ) -> ::windows_core::Result<::windows_core::IInspectable>;
    fn ConvertBack(
        &self,
        value: ::core::option::Option<&::windows_core::IInspectable>,
        targettype: &super::super::super::super::Windows::UI::Xaml::Interop::TypeName,
        parameter: ::core::option::Option<&::windows_core::IInspectable>,
        language: &::windows_core::HSTRING,
    ) -> ::windows_core::Result<::windows_core::IInspectable>;
}
#[cfg(feature = "Windows_UI_Xaml_Interop")]
impl ::windows_core::RuntimeName for IValueConverter {
    const NAME: &'static str = "Microsoft.UI.Xaml.Data.IValueConverter";
}
#[cfg(feature = "Windows_UI_Xaml_Interop")]
impl IValueConverter_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IValueConverter_Impl,
        const OFFSET: isize,
    >() -> IValueConverter_Vtbl {
        unsafe extern "system" fn Convert<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IValueConverter_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            value: *mut ::core::ffi::c_void,
            targettype: ::std::mem::MaybeUninit<
                super::super::super::super::Windows::UI::Xaml::Interop::TypeName,
            >,
            parameter: *mut ::core::ffi::c_void,
            language: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Convert(
                ::windows_core::from_raw_borrowed(&value),
                ::core::mem::transmute(&targettype),
                ::windows_core::from_raw_borrowed(&parameter),
                ::core::mem::transmute(&language),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConvertBack<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IValueConverter_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            value: *mut ::core::ffi::c_void,
            targettype: ::std::mem::MaybeUninit<
                super::super::super::super::Windows::UI::Xaml::Interop::TypeName,
            >,
            parameter: *mut ::core::ffi::c_void,
            language: ::std::mem::MaybeUninit<::windows_core::HSTRING>,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ConvertBack(
                ::windows_core::from_raw_borrowed(&value),
                ::core::mem::transmute(&targettype),
                ::windows_core::from_raw_borrowed(&parameter),
                ::core::mem::transmute(&language),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<Identity, IValueConverter, OFFSET>(),
            Convert: Convert::<Identity, Impl, OFFSET>,
            ConvertBack: ConvertBack::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IValueConverter as ::windows_core::ComInterface>::IID
    }
}
