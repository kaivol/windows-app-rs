pub trait IBindableIterable_Impl: Sized {
    fn First(&self) -> ::windows_core::Result<IBindableIterator>;
}
impl ::windows_core::RuntimeName for IBindableIterable {
    const NAME: &'static str = "Microsoft.UI.Xaml.Interop.IBindableIterable";
}
impl IBindableIterable_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IBindableIterable_Impl,
        const OFFSET: isize,
    >() -> IBindableIterable_Vtbl {
        unsafe extern "system" fn First<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IBindableIterable_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.First() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<Identity, IBindableIterable, OFFSET>(),
            First: First::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IBindableIterable as ::windows_core::ComInterface>::IID
    }
}
pub trait IBindableIterator_Impl: Sized {
    fn Current(&self) -> ::windows_core::Result<::windows_core::IInspectable>;
    fn HasCurrent(&self) -> ::windows_core::Result<bool>;
    fn MoveNext(&self) -> ::windows_core::Result<bool>;
}
impl ::windows_core::RuntimeName for IBindableIterator {
    const NAME: &'static str = "Microsoft.UI.Xaml.Interop.IBindableIterator";
}
impl IBindableIterator_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IBindableIterator_Impl,
        const OFFSET: isize,
    >() -> IBindableIterator_Vtbl {
        unsafe extern "system" fn Current<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IBindableIterator_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Current() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HasCurrent<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IBindableIterator_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut bool,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.HasCurrent() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveNext<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IBindableIterator_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut bool,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MoveNext() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<Identity, IBindableIterator, OFFSET>(),
            Current: Current::<Identity, Impl, OFFSET>,
            HasCurrent: HasCurrent::<Identity, Impl, OFFSET>,
            MoveNext: MoveNext::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IBindableIterator as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Windows_Foundation\"`"]
#[cfg(feature = "Windows_Foundation")]
pub trait IBindableObservableVector_Impl:
    Sized + IBindableIterable_Impl + IBindableVector_Impl
{
    fn VectorChanged(
        &self,
        handler: ::core::option::Option<&BindableVectorChangedEventHandler>,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>;
    fn RemoveVectorChanged(
        &self,
        token: &::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Windows_Foundation")]
impl ::windows_core::RuntimeName for IBindableObservableVector {
    const NAME: &'static str = "Microsoft.UI.Xaml.Interop.IBindableObservableVector";
}
#[cfg(feature = "Windows_Foundation")]
impl IBindableObservableVector_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IBindableObservableVector_Impl,
        const OFFSET: isize,
    >() -> IBindableObservableVector_Vtbl {
        unsafe extern "system" fn VectorChanged<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IBindableObservableVector_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            handler: *mut ::core::ffi::c_void,
            result__: *mut ::windows::Foundation::EventRegistrationToken,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.VectorChanged(::windows_core::from_raw_borrowed(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveVectorChanged<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IBindableObservableVector_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            token: ::windows::Foundation::EventRegistrationToken,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveVectorChanged(::core::mem::transmute(&token)).into()
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<
                Identity,
                IBindableObservableVector,
                OFFSET,
            >(),
            VectorChanged: VectorChanged::<Identity, Impl, OFFSET>,
            RemoveVectorChanged: RemoveVectorChanged::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IBindableObservableVector as ::windows_core::ComInterface>::IID
    }
}
pub trait IBindableVector_Impl: Sized + IBindableIterable_Impl {
    fn GetAt(&self, index: u32) -> ::windows_core::Result<::windows_core::IInspectable>;
    fn Size(&self) -> ::windows_core::Result<u32>;
    fn GetView(&self) -> ::windows_core::Result<IBindableVectorView>;
    fn IndexOf(
        &self,
        value: ::core::option::Option<&::windows_core::IInspectable>,
        index: &mut u32,
    ) -> ::windows_core::Result<bool>;
    fn SetAt(
        &self,
        index: u32,
        value: ::core::option::Option<&::windows_core::IInspectable>,
    ) -> ::windows_core::Result<()>;
    fn InsertAt(
        &self,
        index: u32,
        value: ::core::option::Option<&::windows_core::IInspectable>,
    ) -> ::windows_core::Result<()>;
    fn RemoveAt(&self, index: u32) -> ::windows_core::Result<()>;
    fn Append(
        &self,
        value: ::core::option::Option<&::windows_core::IInspectable>,
    ) -> ::windows_core::Result<()>;
    fn RemoveAtEnd(&self) -> ::windows_core::Result<()>;
    fn Clear(&self) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IBindableVector {
    const NAME: &'static str = "Microsoft.UI.Xaml.Interop.IBindableVector";
}
impl IBindableVector_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IBindableVector_Impl,
        const OFFSET: isize,
    >() -> IBindableVector_Vtbl {
        unsafe extern "system" fn GetAt<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IBindableVector_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            index: u32,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetAt(index) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Size<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IBindableVector_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut u32,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Size() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetView<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IBindableVector_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetView() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IndexOf<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IBindableVector_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            value: *mut ::core::ffi::c_void,
            index: *mut u32,
            result__: *mut bool,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IndexOf(
                ::windows_core::from_raw_borrowed(&value),
                ::core::mem::transmute_copy(&index),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAt<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IBindableVector_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            index: u32,
            value: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAt(index, ::windows_core::from_raw_borrowed(&value)).into()
        }
        unsafe extern "system" fn InsertAt<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IBindableVector_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            index: u32,
            value: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.InsertAt(index, ::windows_core::from_raw_borrowed(&value)).into()
        }
        unsafe extern "system" fn RemoveAt<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IBindableVector_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            index: u32,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveAt(index).into()
        }
        unsafe extern "system" fn Append<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IBindableVector_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            value: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Append(::windows_core::from_raw_borrowed(&value)).into()
        }
        unsafe extern "system" fn RemoveAtEnd<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IBindableVector_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveAtEnd().into()
        }
        unsafe extern "system" fn Clear<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IBindableVector_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Clear().into()
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<Identity, IBindableVector, OFFSET>(),
            GetAt: GetAt::<Identity, Impl, OFFSET>,
            Size: Size::<Identity, Impl, OFFSET>,
            GetView: GetView::<Identity, Impl, OFFSET>,
            IndexOf: IndexOf::<Identity, Impl, OFFSET>,
            SetAt: SetAt::<Identity, Impl, OFFSET>,
            InsertAt: InsertAt::<Identity, Impl, OFFSET>,
            RemoveAt: RemoveAt::<Identity, Impl, OFFSET>,
            Append: Append::<Identity, Impl, OFFSET>,
            RemoveAtEnd: RemoveAtEnd::<Identity, Impl, OFFSET>,
            Clear: Clear::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IBindableVector as ::windows_core::ComInterface>::IID
    }
}
pub trait IBindableVectorView_Impl: Sized + IBindableIterable_Impl {
    fn GetAt(&self, index: u32) -> ::windows_core::Result<::windows_core::IInspectable>;
    fn Size(&self) -> ::windows_core::Result<u32>;
    fn IndexOf(
        &self,
        value: ::core::option::Option<&::windows_core::IInspectable>,
        index: &mut u32,
    ) -> ::windows_core::Result<bool>;
}
impl ::windows_core::RuntimeName for IBindableVectorView {
    const NAME: &'static str = "Microsoft.UI.Xaml.Interop.IBindableVectorView";
}
impl IBindableVectorView_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IBindableVectorView_Impl,
        const OFFSET: isize,
    >() -> IBindableVectorView_Vtbl {
        unsafe extern "system" fn GetAt<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IBindableVectorView_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            index: u32,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetAt(index) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Size<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IBindableVectorView_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut u32,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Size() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IndexOf<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IBindableVectorView_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            value: *mut ::core::ffi::c_void,
            index: *mut u32,
            result__: *mut bool,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IndexOf(
                ::windows_core::from_raw_borrowed(&value),
                ::core::mem::transmute_copy(&index),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<Identity, IBindableVectorView, OFFSET>(
            ),
            GetAt: GetAt::<Identity, Impl, OFFSET>,
            Size: Size::<Identity, Impl, OFFSET>,
            IndexOf: IndexOf::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IBindableVectorView as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Windows_Foundation\"`"]
#[cfg(feature = "Windows_Foundation")]
pub trait INotifyCollectionChanged_Impl: Sized {
    fn CollectionChanged(
        &self,
        handler: ::core::option::Option<&NotifyCollectionChangedEventHandler>,
    ) -> ::windows_core::Result<::windows::Foundation::EventRegistrationToken>;
    fn RemoveCollectionChanged(
        &self,
        token: &::windows::Foundation::EventRegistrationToken,
    ) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Windows_Foundation")]
impl ::windows_core::RuntimeName for INotifyCollectionChanged {
    const NAME: &'static str = "Microsoft.UI.Xaml.Interop.INotifyCollectionChanged";
}
#[cfg(feature = "Windows_Foundation")]
impl INotifyCollectionChanged_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: INotifyCollectionChanged_Impl,
        const OFFSET: isize,
    >() -> INotifyCollectionChanged_Vtbl {
        unsafe extern "system" fn CollectionChanged<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: INotifyCollectionChanged_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            handler: *mut ::core::ffi::c_void,
            result__: *mut ::windows::Foundation::EventRegistrationToken,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CollectionChanged(::windows_core::from_raw_borrowed(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveCollectionChanged<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: INotifyCollectionChanged_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            token: ::windows::Foundation::EventRegistrationToken,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveCollectionChanged(::core::mem::transmute(&token)).into()
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<
                Identity,
                INotifyCollectionChanged,
                OFFSET,
            >(),
            CollectionChanged: CollectionChanged::<Identity, Impl, OFFSET>,
            RemoveCollectionChanged: RemoveCollectionChanged::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <INotifyCollectionChanged as ::windows_core::ComInterface>::IID
    }
}
