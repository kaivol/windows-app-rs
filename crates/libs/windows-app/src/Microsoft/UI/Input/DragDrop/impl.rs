#[doc = "Required features: `\"Windows_ApplicationModel_DataTransfer\"`, `\"Windows_Foundation\"`"]
#[cfg(all(
    feature = "Windows_ApplicationModel_DataTransfer",
    feature = "Windows_Foundation"
))]
pub trait IDropOperationTarget_Impl: Sized {
    fn DropAsync(
        &self,
        draginfo: ::core::option::Option<&DragInfo>,
    ) -> ::windows_core::Result<
        ::windows::Foundation::IAsyncOperation<
            ::windows::ApplicationModel::DataTransfer::DataPackageOperation,
        >,
    >;
    fn EnterAsync(
        &self,
        draginfo: ::core::option::Option<&DragInfo>,
        draguioverride: ::core::option::Option<&DragUIOverride>,
    ) -> ::windows_core::Result<
        ::windows::Foundation::IAsyncOperation<
            ::windows::ApplicationModel::DataTransfer::DataPackageOperation,
        >,
    >;
    fn LeaveAsync(
        &self,
        draginfo: ::core::option::Option<&DragInfo>,
    ) -> ::windows_core::Result<::windows::Foundation::IAsyncAction>;
    fn OverAsync(
        &self,
        draginfo: ::core::option::Option<&DragInfo>,
        draguioverride: ::core::option::Option<&DragUIOverride>,
    ) -> ::windows_core::Result<
        ::windows::Foundation::IAsyncOperation<
            ::windows::ApplicationModel::DataTransfer::DataPackageOperation,
        >,
    >;
}
#[cfg(all(
    feature = "Windows_ApplicationModel_DataTransfer",
    feature = "Windows_Foundation"
))]
impl ::windows_core::RuntimeName for IDropOperationTarget {
    const NAME: &'static str = "Microsoft.UI.Input.DragDrop.IDropOperationTarget";
}
#[cfg(all(
    feature = "Windows_ApplicationModel_DataTransfer",
    feature = "Windows_Foundation"
))]
impl IDropOperationTarget_Vtbl {
    pub const fn new<
        Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IDropOperationTarget_Impl,
        const OFFSET: isize,
    >() -> IDropOperationTarget_Vtbl {
        unsafe extern "system" fn DropAsync<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IDropOperationTarget_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            draginfo: *mut ::core::ffi::c_void,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DropAsync(::windows_core::from_raw_borrowed(&draginfo)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnterAsync<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IDropOperationTarget_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            draginfo: *mut ::core::ffi::c_void,
            draguioverride: *mut ::core::ffi::c_void,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnterAsync(
                ::windows_core::from_raw_borrowed(&draginfo),
                ::windows_core::from_raw_borrowed(&draguioverride),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LeaveAsync<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IDropOperationTarget_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            draginfo: *mut ::core::ffi::c_void,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LeaveAsync(::windows_core::from_raw_borrowed(&draginfo)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OverAsync<
            Identity: ::windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IDropOperationTarget_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            draginfo: *mut ::core::ffi::c_void,
            draguioverride: *mut ::core::ffi::c_void,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.OverAsync(
                ::windows_core::from_raw_borrowed(&draginfo),
                ::windows_core::from_raw_borrowed(&draguioverride),
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
            base__: ::windows_core::IInspectable_Vtbl::new::<Identity, IDropOperationTarget, OFFSET>(
            ),
            DropAsync: DropAsync::<Identity, Impl, OFFSET>,
            EnterAsync: EnterAsync::<Identity, Impl, OFFSET>,
            LeaveAsync: LeaveAsync::<Identity, Impl, OFFSET>,
            OverAsync: OverAsync::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IDropOperationTarget as ::windows_core::ComInterface>::IID
    }
}
