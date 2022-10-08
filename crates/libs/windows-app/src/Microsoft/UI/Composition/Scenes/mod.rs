#[doc(hidden)]
#[repr(transparent)]
pub struct ISceneBoundingBox(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ISceneBoundingBox {
    type Vtable = ISceneBoundingBox_Vtbl;
}
unsafe impl ::windows::core::Interface for ISceneBoundingBox {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x39fb48e0_216a_5608_9186_6ba9f98b5c67);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceneBoundingBox_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Center: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::Numerics::Vector3,
    ) -> ::windows::core::HRESULT,
    pub Extents: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::Numerics::Vector3,
    ) -> ::windows::core::HRESULT,
    pub Max: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::Numerics::Vector3,
    ) -> ::windows::core::HRESULT,
    pub Min: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::Numerics::Vector3,
    ) -> ::windows::core::HRESULT,
    pub Size: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::Numerics::Vector3,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISceneComponent(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ISceneComponent {
    type Vtable = ISceneComponent_Vtbl;
}
unsafe impl ::windows::core::Interface for ISceneComponent {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xf73361cb_8027_50e2_98ee_b2e3ea050a54);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceneComponent_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ComponentType: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut SceneComponentType,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISceneComponentCollection(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ISceneComponentCollection {
    type Vtable = ISceneComponentCollection_Vtbl;
}
unsafe impl ::windows::core::Interface for ISceneComponentCollection {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xe4b21c71_87e2_5aeb_85be_884e8302273e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceneComponentCollection_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISceneComponentFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ISceneComponentFactory {
    type Vtable = ISceneComponentFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for ISceneComponentFactory {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x254088b0_babf_503d_9a66_0d86af5f7303);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceneComponentFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISceneMaterial(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ISceneMaterial {
    type Vtable = ISceneMaterial_Vtbl;
}
unsafe impl ::windows::core::Interface for ISceneMaterial {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x042142a7_bf6b_57ad_badc_f581f38edb48);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceneMaterial_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISceneMaterialFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ISceneMaterialFactory {
    type Vtable = ISceneMaterialFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for ISceneMaterialFactory {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x25747893_8748_5f60_969f_318fa0b735ca);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceneMaterialFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISceneMaterialInput(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ISceneMaterialInput {
    type Vtable = ISceneMaterialInput_Vtbl;
}
unsafe impl ::windows::core::Interface for ISceneMaterialInput {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x446bdade_719b_5db4_b699_f226d0062a2e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceneMaterialInput_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISceneMaterialInputFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ISceneMaterialInputFactory {
    type Vtable = ISceneMaterialInputFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for ISceneMaterialInputFactory {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xb4dabd1d_58c0_5710_928a_bc49b0735694);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceneMaterialInputFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISceneMesh(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ISceneMesh {
    type Vtable = ISceneMesh_Vtbl;
}
unsafe impl ::windows::core::Interface for ISceneMesh {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x5cf846aa_f53f_555e_a3ad_f5bc52ca32fb);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceneMesh_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Bounds: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(feature = "Graphics_DirectX")]
    pub PrimitiveTopology: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::super::super::Graphics::DirectX::DirectXPrimitiveTopology,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX"))]
    PrimitiveTopology: usize,
    #[cfg(feature = "Graphics_DirectX")]
    pub SetPrimitiveTopology: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: super::super::super::Graphics::DirectX::DirectXPrimitiveTopology,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX"))]
    SetPrimitiveTopology: usize,
    #[cfg(feature = "Graphics_DirectX")]
    pub FillMeshAttribute: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        semantic: SceneAttributeSemantic,
        format: super::super::super::Graphics::DirectX::DirectXPixelFormat,
        memory: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX"))]
    FillMeshAttribute: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISceneMeshMaterialAttributeMap(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ISceneMeshMaterialAttributeMap {
    type Vtable = ISceneMeshMaterialAttributeMap_Vtbl;
}
unsafe impl ::windows::core::Interface for ISceneMeshMaterialAttributeMap {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x2360c457_edae_5660_bedc_89096582ed70);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceneMeshMaterialAttributeMap_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISceneMeshRendererComponent(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ISceneMeshRendererComponent {
    type Vtable = ISceneMeshRendererComponent_Vtbl;
}
unsafe impl ::windows::core::Interface for ISceneMeshRendererComponent {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xd2be85a0_70a8_5c62_84d8_8ba55e4c64a9);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceneMeshRendererComponent_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Material: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub SetMaterial: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub Mesh: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub SetMesh: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub UVMappings: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISceneMeshRendererComponentStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ISceneMeshRendererComponentStatics {
    type Vtable = ISceneMeshRendererComponentStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for ISceneMeshRendererComponentStatics {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xc54f8c5a_a104_5cfa_89dc_13edaa6e3d88);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceneMeshRendererComponentStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        compositor: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISceneMeshStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ISceneMeshStatics {
    type Vtable = ISceneMeshStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for ISceneMeshStatics {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x29c52125_964b_5315_80f9_3893713290f5);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceneMeshStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        compositor: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISceneMetallicRoughnessMaterial(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ISceneMetallicRoughnessMaterial {
    type Vtable = ISceneMetallicRoughnessMaterial_Vtbl;
}
unsafe impl ::windows::core::Interface for ISceneMetallicRoughnessMaterial {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x0a4afcf4_7bae_5702_9b85_8bc849f39987);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceneMetallicRoughnessMaterial_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub BaseColorInput: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub SetBaseColorInput: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub BaseColorFactor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::Numerics::Vector4,
    ) -> ::windows::core::HRESULT,
    pub SetBaseColorFactor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::windows::Foundation::Numerics::Vector4,
    ) -> ::windows::core::HRESULT,
    pub MetallicFactor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f32,
    ) -> ::windows::core::HRESULT,
    pub SetMetallicFactor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f32,
    ) -> ::windows::core::HRESULT,
    pub MetallicRoughnessInput: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub SetMetallicRoughnessInput: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub RoughnessFactor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f32,
    ) -> ::windows::core::HRESULT,
    pub SetRoughnessFactor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f32,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISceneMetallicRoughnessMaterialStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ISceneMetallicRoughnessMaterialStatics {
    type Vtable = ISceneMetallicRoughnessMaterialStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for ISceneMetallicRoughnessMaterialStatics {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xdffd2043_ab3c_57a0_8e13_6f09725e970f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceneMetallicRoughnessMaterialStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        compositor: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISceneModelTransform(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ISceneModelTransform {
    type Vtable = ISceneModelTransform_Vtbl;
}
unsafe impl ::windows::core::Interface for ISceneModelTransform {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x3f05555f_0f67_576e_9d8a_93c1f250c29f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceneModelTransform_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Orientation: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::Numerics::Quaternion,
    ) -> ::windows::core::HRESULT,
    pub SetOrientation: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::windows::Foundation::Numerics::Quaternion,
    ) -> ::windows::core::HRESULT,
    pub RotationAngle: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f32,
    ) -> ::windows::core::HRESULT,
    pub SetRotationAngle: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f32,
    ) -> ::windows::core::HRESULT,
    pub RotationAngleInDegrees: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f32,
    ) -> ::windows::core::HRESULT,
    pub SetRotationAngleInDegrees: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f32,
    ) -> ::windows::core::HRESULT,
    pub RotationAxis: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::Numerics::Vector3,
    ) -> ::windows::core::HRESULT,
    pub SetRotationAxis: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::windows::Foundation::Numerics::Vector3,
    ) -> ::windows::core::HRESULT,
    pub Scale: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::Numerics::Vector3,
    ) -> ::windows::core::HRESULT,
    pub SetScale: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::windows::Foundation::Numerics::Vector3,
    ) -> ::windows::core::HRESULT,
    pub Translation: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::Numerics::Vector3,
    ) -> ::windows::core::HRESULT,
    pub SetTranslation: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::windows::Foundation::Numerics::Vector3,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISceneNode(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ISceneNode {
    type Vtable = ISceneNode_Vtbl;
}
unsafe impl ::windows::core::Interface for ISceneNode {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xa1bce140_79c2_59e6_9b68_63b1bab0e2a6);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceneNode_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Children: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub Components: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub Parent: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub Transform: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub FindFirstComponentOfType: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: SceneComponentType,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISceneNodeCollection(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ISceneNodeCollection {
    type Vtable = ISceneNodeCollection_Vtbl;
}
unsafe impl ::windows::core::Interface for ISceneNodeCollection {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xf219b68e_5666_5c6c_aa4e_08db07fd6bcf);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceneNodeCollection_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISceneNodeStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ISceneNodeStatics {
    type Vtable = ISceneNodeStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for ISceneNodeStatics {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x801c4394_4198_5da1_ac39_6e8a44b5ce57);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceneNodeStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        compositor: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISceneObject(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ISceneObject {
    type Vtable = ISceneObject_Vtbl;
}
unsafe impl ::windows::core::Interface for ISceneObject {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x4333e514_4fc7_521e_8bca_11c51fbcaf1e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceneObject_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISceneObjectFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ISceneObjectFactory {
    type Vtable = ISceneObjectFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for ISceneObjectFactory {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xee797f7d_77db_5c4c_b6f5_c1930fad85c5);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceneObjectFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IScenePbrMaterial(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IScenePbrMaterial {
    type Vtable = IScenePbrMaterial_Vtbl;
}
unsafe impl ::windows::core::Interface for IScenePbrMaterial {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x295d0725_56fe_5954_8057_3f4ca7515b36);
}
#[repr(C)]
#[doc(hidden)]
pub struct IScenePbrMaterial_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub AlphaCutoff: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f32,
    ) -> ::windows::core::HRESULT,
    pub SetAlphaCutoff: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f32,
    ) -> ::windows::core::HRESULT,
    pub AlphaMode: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut SceneAlphaMode,
    ) -> ::windows::core::HRESULT,
    pub SetAlphaMode: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: SceneAlphaMode,
    ) -> ::windows::core::HRESULT,
    pub EmissiveInput: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub SetEmissiveInput: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub EmissiveFactor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::Numerics::Vector3,
    ) -> ::windows::core::HRESULT,
    pub SetEmissiveFactor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::windows::Foundation::Numerics::Vector3,
    ) -> ::windows::core::HRESULT,
    pub IsDoubleSided: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub SetIsDoubleSided: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows::core::HRESULT,
    pub NormalInput: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub SetNormalInput: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub NormalScale: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f32,
    ) -> ::windows::core::HRESULT,
    pub SetNormalScale: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f32,
    ) -> ::windows::core::HRESULT,
    pub OcclusionInput: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub SetOcclusionInput: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub OcclusionStrength: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f32,
    ) -> ::windows::core::HRESULT,
    pub SetOcclusionStrength: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f32,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IScenePbrMaterialFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IScenePbrMaterialFactory {
    type Vtable = IScenePbrMaterialFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for IScenePbrMaterialFactory {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x9e34d32a_e30c_51f5_84ac_6467950605ca);
}
#[repr(C)]
#[doc(hidden)]
pub struct IScenePbrMaterialFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISceneRendererComponent(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ISceneRendererComponent {
    type Vtable = ISceneRendererComponent_Vtbl;
}
unsafe impl ::windows::core::Interface for ISceneRendererComponent {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x6bab8030_89c1_5dbc_a48e_1805ddf9cdd1);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceneRendererComponent_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISceneRendererComponentFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ISceneRendererComponentFactory {
    type Vtable = ISceneRendererComponentFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for ISceneRendererComponentFactory {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x3ccac1d6_6a0f_582e_bb1a_10ebc1e405ca);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceneRendererComponentFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISceneSurfaceMaterialInput(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ISceneSurfaceMaterialInput {
    type Vtable = ISceneSurfaceMaterialInput_Vtbl;
}
unsafe impl ::windows::core::Interface for ISceneSurfaceMaterialInput {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xb9854b4f_286c_50cd_a734_491a251d5fd3);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceneSurfaceMaterialInput_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub BitmapInterpolationMode: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::CompositionBitmapInterpolationMode,
    ) -> ::windows::core::HRESULT,
    pub SetBitmapInterpolationMode: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: super::CompositionBitmapInterpolationMode,
    ) -> ::windows::core::HRESULT,
    pub Surface: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub SetSurface: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub WrappingUMode: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut SceneWrappingMode,
    ) -> ::windows::core::HRESULT,
    pub SetWrappingUMode: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: SceneWrappingMode,
    ) -> ::windows::core::HRESULT,
    pub WrappingVMode: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut SceneWrappingMode,
    ) -> ::windows::core::HRESULT,
    pub SetWrappingVMode: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: SceneWrappingMode,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISceneSurfaceMaterialInputStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ISceneSurfaceMaterialInputStatics {
    type Vtable = ISceneSurfaceMaterialInputStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for ISceneSurfaceMaterialInputStatics {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x8e1ba937_ad60_51bc_8256_ca62c4b2ae92);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceneSurfaceMaterialInputStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        compositor: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISceneVisual(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ISceneVisual {
    type Vtable = ISceneVisual_Vtbl;
}
unsafe impl ::windows::core::Interface for ISceneVisual {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x0144d7ad_6a7d_59cb_a0f9_74a04e85352c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceneVisual_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Root: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub SetRoot: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISceneVisualStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ISceneVisualStatics {
    type Vtable = ISceneVisualStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for ISceneVisualStatics {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x7b8da6d1_5bd8_5095_9264_e5572653ea07);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceneVisualStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        compositor: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Composition_Scenes\"`*"]
#[repr(transparent)]
pub struct SceneBoundingBox(::windows::core::IUnknown);
impl SceneBoundingBox {
    pub fn PopulatePropertyInfo(
        &self,
        propertyname: &::windows::core::HSTRING,
        propertyinfo: &super::AnimationPropertyInfo,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).PopulatePropertyInfo)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(propertyname),
                ::core::mem::transmute_copy(propertyinfo),
            )
            .ok()
        }
    }
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<::windows::Foundation::IClosable>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).Close)(::windows::core::Vtable::as_raw(this))
                .ok()
        }
    }
    pub fn Compositor(&self) -> ::windows::core::Result<super::Compositor> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Compositor)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::Compositor>(result__)
        }
    }
    pub fn Properties(&self) -> ::windows::core::Result<super::CompositionPropertySet> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Properties)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::CompositionPropertySet>(result__)
        }
    }
    pub fn StartAnimation<'a, P0>(
        &self,
        propertyname: &::windows::core::HSTRING,
        animation: P0,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::CompositionAnimation>>,
    {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).StartAnimation)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(propertyname),
                animation.into().abi(),
            )
            .ok()
        }
    }
    pub fn StopAnimation(
        &self,
        propertyname: &::windows::core::HSTRING,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).StopAnimation)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(propertyname),
            )
            .ok()
        }
    }
    pub fn Comment(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Comment)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetComment(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetComment)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn ImplicitAnimations(
        &self,
    ) -> ::windows::core::Result<super::ImplicitAnimationCollection> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ImplicitAnimations)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::ImplicitAnimationCollection>(result__)
        }
    }
    pub fn SetImplicitAnimations(
        &self,
        value: &super::ImplicitAnimationCollection,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetImplicitAnimations)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn StartAnimationGroup<'a, P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<'a, super::ICompositionAnimationBase>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).StartAnimationGroup)(
                ::windows::core::Vtable::as_raw(this),
                value.try_into().map_err(|e| e.into())?.abi(),
            )
            .ok()
        }
    }
    pub fn StopAnimationGroup<'a, P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<'a, super::ICompositionAnimationBase>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).StopAnimationGroup)(
                ::windows::core::Vtable::as_raw(this),
                value.try_into().map_err(|e| e.into())?.abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Dispatching\"`*"]
    #[cfg(feature = "UI_Dispatching")]
    pub fn DispatcherQueue(
        &self,
    ) -> ::windows::core::Result<super::super::Dispatching::DispatcherQueue> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DispatcherQueue)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::super::Dispatching::DispatcherQueue>(result__)
        }
    }
    pub fn TryGetAnimationController(
        &self,
        propertyname: &::windows::core::HSTRING,
    ) -> ::windows::core::Result<super::AnimationController> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TryGetAnimationController)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(propertyname),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::AnimationController>(result__)
        }
    }
    pub fn StartAnimationWithController<'a, P0>(
        &self,
        propertyname: &::windows::core::HSTRING,
        animation: P0,
        animationcontroller: &super::AnimationController,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::CompositionAnimation>>,
    {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject5>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).StartAnimationWithController)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(propertyname),
                animation.into().abi(),
                ::core::mem::transmute_copy(animationcontroller),
            )
            .ok()
        }
    }
    pub fn Center(&self) -> ::windows::core::Result<::windows::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Center)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::Numerics::Vector3>(result__)
        }
    }
    pub fn Extents(&self) -> ::windows::core::Result<::windows::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Extents)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::Numerics::Vector3>(result__)
        }
    }
    pub fn Max(&self) -> ::windows::core::Result<::windows::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Max)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::Numerics::Vector3>(result__)
        }
    }
    pub fn Min(&self) -> ::windows::core::Result<::windows::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Min)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::Numerics::Vector3>(result__)
        }
    }
    pub fn Size(&self) -> ::windows::core::Result<::windows::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Size)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::Numerics::Vector3>(result__)
        }
    }
}
impl ::core::clone::Clone for SceneBoundingBox {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SceneBoundingBox {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SceneBoundingBox {}
impl ::core::fmt::Debug for SceneBoundingBox {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SceneBoundingBox").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SceneBoundingBox {
    const SIGNATURE : ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice ( b"rc(Microsoft.UI.Composition.Scenes.SceneBoundingBox;{39fb48e0-216a-5608-9186-6ba9f98b5c67})" ) ;
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for SceneBoundingBox {
    type Vtable = ISceneBoundingBox_Vtbl;
}
unsafe impl ::windows::core::Interface for SceneBoundingBox {
    const IID: ::windows::core::GUID = <ISceneBoundingBox as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for SceneBoundingBox {
    const NAME: &'static str = "Microsoft.UI.Composition.Scenes.SceneBoundingBox";
}
::windows::core::interface_hierarchy!(
    SceneBoundingBox,
    ::windows::core::IUnknown,
    ::windows::core::IInspectable
);
impl ::core::convert::TryFrom<SceneBoundingBox> for super::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: SceneBoundingBox) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SceneBoundingBox> for super::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: &SceneBoundingBox) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&SceneBoundingBox>
    for ::windows::core::InParam<'a, super::IAnimationObject>
{
    type Error = ::windows::core::Error;
    fn try_from(value: &SceneBoundingBox) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<SceneBoundingBox> for ::windows::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: SceneBoundingBox) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SceneBoundingBox> for ::windows::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &SceneBoundingBox) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&SceneBoundingBox>
    for ::windows::core::InParam<'a, ::windows::Foundation::IClosable>
{
    type Error = ::windows::core::Error;
    fn try_from(value: &SceneBoundingBox) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::From<SceneBoundingBox> for SceneObject {
    fn from(value: SceneBoundingBox) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&SceneBoundingBox> for SceneObject {
    fn from(value: &SceneBoundingBox) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::core::convert::From<&SceneBoundingBox> for ::windows::core::InParam<'a, SceneObject> {
    fn from(value: &SceneBoundingBox) -> Self {
        ::windows::core::InParam::owned(value.into())
    }
}
impl ::core::convert::From<SceneBoundingBox> for super::CompositionObject {
    fn from(value: SceneBoundingBox) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&SceneBoundingBox> for super::CompositionObject {
    fn from(value: &SceneBoundingBox) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::core::convert::From<&SceneBoundingBox>
    for ::windows::core::InParam<'a, super::CompositionObject>
{
    fn from(value: &SceneBoundingBox) -> Self {
        ::windows::core::InParam::owned(value.into())
    }
}
unsafe impl ::core::marker::Send for SceneBoundingBox {}
unsafe impl ::core::marker::Sync for SceneBoundingBox {}
#[doc = "*Required features: `\"UI_Composition_Scenes\"`*"]
#[repr(transparent)]
pub struct SceneComponent(::windows::core::IUnknown);
impl SceneComponent {
    pub fn PopulatePropertyInfo(
        &self,
        propertyname: &::windows::core::HSTRING,
        propertyinfo: &super::AnimationPropertyInfo,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).PopulatePropertyInfo)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(propertyname),
                ::core::mem::transmute_copy(propertyinfo),
            )
            .ok()
        }
    }
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<::windows::Foundation::IClosable>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).Close)(::windows::core::Vtable::as_raw(this))
                .ok()
        }
    }
    pub fn Compositor(&self) -> ::windows::core::Result<super::Compositor> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Compositor)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::Compositor>(result__)
        }
    }
    pub fn Properties(&self) -> ::windows::core::Result<super::CompositionPropertySet> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Properties)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::CompositionPropertySet>(result__)
        }
    }
    pub fn StartAnimation<'a, P0>(
        &self,
        propertyname: &::windows::core::HSTRING,
        animation: P0,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::CompositionAnimation>>,
    {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).StartAnimation)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(propertyname),
                animation.into().abi(),
            )
            .ok()
        }
    }
    pub fn StopAnimation(
        &self,
        propertyname: &::windows::core::HSTRING,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).StopAnimation)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(propertyname),
            )
            .ok()
        }
    }
    pub fn Comment(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Comment)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetComment(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetComment)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn ImplicitAnimations(
        &self,
    ) -> ::windows::core::Result<super::ImplicitAnimationCollection> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ImplicitAnimations)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::ImplicitAnimationCollection>(result__)
        }
    }
    pub fn SetImplicitAnimations(
        &self,
        value: &super::ImplicitAnimationCollection,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetImplicitAnimations)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn StartAnimationGroup<'a, P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<'a, super::ICompositionAnimationBase>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).StartAnimationGroup)(
                ::windows::core::Vtable::as_raw(this),
                value.try_into().map_err(|e| e.into())?.abi(),
            )
            .ok()
        }
    }
    pub fn StopAnimationGroup<'a, P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<'a, super::ICompositionAnimationBase>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).StopAnimationGroup)(
                ::windows::core::Vtable::as_raw(this),
                value.try_into().map_err(|e| e.into())?.abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Dispatching\"`*"]
    #[cfg(feature = "UI_Dispatching")]
    pub fn DispatcherQueue(
        &self,
    ) -> ::windows::core::Result<super::super::Dispatching::DispatcherQueue> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DispatcherQueue)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::super::Dispatching::DispatcherQueue>(result__)
        }
    }
    pub fn TryGetAnimationController(
        &self,
        propertyname: &::windows::core::HSTRING,
    ) -> ::windows::core::Result<super::AnimationController> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TryGetAnimationController)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(propertyname),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::AnimationController>(result__)
        }
    }
    pub fn StartAnimationWithController<'a, P0>(
        &self,
        propertyname: &::windows::core::HSTRING,
        animation: P0,
        animationcontroller: &super::AnimationController,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::CompositionAnimation>>,
    {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject5>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).StartAnimationWithController)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(propertyname),
                animation.into().abi(),
                ::core::mem::transmute_copy(animationcontroller),
            )
            .ok()
        }
    }
    pub fn ComponentType(&self) -> ::windows::core::Result<SceneComponentType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ComponentType)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<SceneComponentType>(result__)
        }
    }
}
impl ::core::clone::Clone for SceneComponent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SceneComponent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SceneComponent {}
impl ::core::fmt::Debug for SceneComponent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SceneComponent").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SceneComponent {
    const SIGNATURE : ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice ( b"rc(Microsoft.UI.Composition.Scenes.SceneComponent;{f73361cb-8027-50e2-98ee-b2e3ea050a54})" ) ;
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for SceneComponent {
    type Vtable = ISceneComponent_Vtbl;
}
unsafe impl ::windows::core::Interface for SceneComponent {
    const IID: ::windows::core::GUID = <ISceneComponent as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for SceneComponent {
    const NAME: &'static str = "Microsoft.UI.Composition.Scenes.SceneComponent";
}
::windows::core::interface_hierarchy!(
    SceneComponent,
    ::windows::core::IUnknown,
    ::windows::core::IInspectable
);
impl ::core::convert::TryFrom<SceneComponent> for super::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: SceneComponent) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SceneComponent> for super::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: &SceneComponent) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&SceneComponent>
    for ::windows::core::InParam<'a, super::IAnimationObject>
{
    type Error = ::windows::core::Error;
    fn try_from(value: &SceneComponent) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<SceneComponent> for ::windows::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: SceneComponent) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SceneComponent> for ::windows::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &SceneComponent) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&SceneComponent>
    for ::windows::core::InParam<'a, ::windows::Foundation::IClosable>
{
    type Error = ::windows::core::Error;
    fn try_from(value: &SceneComponent) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::From<SceneComponent> for SceneObject {
    fn from(value: SceneComponent) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&SceneComponent> for SceneObject {
    fn from(value: &SceneComponent) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::core::convert::From<&SceneComponent> for ::windows::core::InParam<'a, SceneObject> {
    fn from(value: &SceneComponent) -> Self {
        ::windows::core::InParam::owned(value.into())
    }
}
impl ::core::convert::From<SceneComponent> for super::CompositionObject {
    fn from(value: SceneComponent) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&SceneComponent> for super::CompositionObject {
    fn from(value: &SceneComponent) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::core::convert::From<&SceneComponent>
    for ::windows::core::InParam<'a, super::CompositionObject>
{
    fn from(value: &SceneComponent) -> Self {
        ::windows::core::InParam::owned(value.into())
    }
}
unsafe impl ::core::marker::Send for SceneComponent {}
unsafe impl ::core::marker::Sync for SceneComponent {}
#[doc = "*Required features: `\"UI_Composition_Scenes\"`*"]
#[repr(transparent)]
pub struct SceneComponentCollection(::windows::core::IUnknown);
impl SceneComponentCollection {
    pub fn PopulatePropertyInfo(
        &self,
        propertyname: &::windows::core::HSTRING,
        propertyinfo: &super::AnimationPropertyInfo,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).PopulatePropertyInfo)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(propertyname),
                ::core::mem::transmute_copy(propertyinfo),
            )
            .ok()
        }
    }
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<::windows::Foundation::IClosable>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).Close)(::windows::core::Vtable::as_raw(this))
                .ok()
        }
    }
    pub fn Compositor(&self) -> ::windows::core::Result<super::Compositor> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Compositor)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::Compositor>(result__)
        }
    }
    pub fn Properties(&self) -> ::windows::core::Result<super::CompositionPropertySet> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Properties)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::CompositionPropertySet>(result__)
        }
    }
    pub fn StartAnimation<'a, P0>(
        &self,
        propertyname: &::windows::core::HSTRING,
        animation: P0,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::CompositionAnimation>>,
    {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).StartAnimation)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(propertyname),
                animation.into().abi(),
            )
            .ok()
        }
    }
    pub fn StopAnimation(
        &self,
        propertyname: &::windows::core::HSTRING,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).StopAnimation)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(propertyname),
            )
            .ok()
        }
    }
    pub fn Comment(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Comment)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetComment(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetComment)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn ImplicitAnimations(
        &self,
    ) -> ::windows::core::Result<super::ImplicitAnimationCollection> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ImplicitAnimations)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::ImplicitAnimationCollection>(result__)
        }
    }
    pub fn SetImplicitAnimations(
        &self,
        value: &super::ImplicitAnimationCollection,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetImplicitAnimations)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn StartAnimationGroup<'a, P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<'a, super::ICompositionAnimationBase>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).StartAnimationGroup)(
                ::windows::core::Vtable::as_raw(this),
                value.try_into().map_err(|e| e.into())?.abi(),
            )
            .ok()
        }
    }
    pub fn StopAnimationGroup<'a, P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<'a, super::ICompositionAnimationBase>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).StopAnimationGroup)(
                ::windows::core::Vtable::as_raw(this),
                value.try_into().map_err(|e| e.into())?.abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Dispatching\"`*"]
    #[cfg(feature = "UI_Dispatching")]
    pub fn DispatcherQueue(
        &self,
    ) -> ::windows::core::Result<super::super::Dispatching::DispatcherQueue> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DispatcherQueue)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::super::Dispatching::DispatcherQueue>(result__)
        }
    }
    pub fn TryGetAnimationController(
        &self,
        propertyname: &::windows::core::HSTRING,
    ) -> ::windows::core::Result<super::AnimationController> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TryGetAnimationController)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(propertyname),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::AnimationController>(result__)
        }
    }
    pub fn StartAnimationWithController<'a, P0>(
        &self,
        propertyname: &::windows::core::HSTRING,
        animation: P0,
        animationcontroller: &super::AnimationController,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::CompositionAnimation>>,
    {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject5>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).StartAnimationWithController)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(propertyname),
                animation.into().abi(),
                ::core::mem::transmute_copy(animationcontroller),
            )
            .ok()
        }
    }
    pub fn First(
        &self,
    ) -> ::windows::core::Result<::windows::Foundation::Collections::IIterator<SceneComponent>>
    {
        let this = &::windows::core::Interface::cast::<
            ::windows::Foundation::Collections::IIterable<SceneComponent>,
        >(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).First)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::Collections::IIterator<SceneComponent>>(result__)
        }
    }
    pub fn GetAt(&self, index: u32) -> ::windows::core::Result<SceneComponent> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetAt)(
                ::windows::core::Vtable::as_raw(this),
                index,
                result__.as_mut_ptr(),
            )
            .from_abi::<SceneComponent>(result__)
        }
    }
    pub fn Size(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Size)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<u32>(result__)
        }
    }
    pub fn GetView(
        &self,
    ) -> ::windows::core::Result<::windows::Foundation::Collections::IVectorView<SceneComponent>>
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetView)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::Collections::IVectorView<SceneComponent>>(result__)
        }
    }
    pub fn IndexOf<'a, P0>(&self, value: P0, index: &mut u32) -> ::windows::core::Result<bool>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, SceneComponent>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IndexOf)(
                ::windows::core::Vtable::as_raw(this),
                value.into().abi(),
                index,
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetAt<'a, P0>(&self, index: u32, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, SceneComponent>>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetAt)(
                ::windows::core::Vtable::as_raw(this),
                index,
                value.into().abi(),
            )
            .ok()
        }
    }
    pub fn InsertAt<'a, P0>(&self, index: u32, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, SceneComponent>>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).InsertAt)(
                ::windows::core::Vtable::as_raw(this),
                index,
                value.into().abi(),
            )
            .ok()
        }
    }
    pub fn RemoveAt(&self, index: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).RemoveAt)(
                ::windows::core::Vtable::as_raw(this),
                index,
            )
            .ok()
        }
    }
    pub fn Append<'a, P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, SceneComponent>>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).Append)(
                ::windows::core::Vtable::as_raw(this),
                value.into().abi(),
            )
            .ok()
        }
    }
    pub fn RemoveAtEnd(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).RemoveAtEnd)(::windows::core::Vtable::as_raw(
                this,
            ))
            .ok()
        }
    }
    pub fn Clear(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).Clear)(::windows::core::Vtable::as_raw(this))
                .ok()
        }
    }
    pub fn GetMany(
        &self,
        startindex: u32,
        items: &mut [::core::option::Option<SceneComponent>],
    ) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetMany)(
                ::windows::core::Vtable::as_raw(this),
                startindex,
                items.len() as u32,
                ::core::mem::transmute_copy(&items),
                result__.as_mut_ptr(),
            )
            .from_abi::<u32>(result__)
        }
    }
    pub fn ReplaceAll(
        &self,
        items: &[::core::option::Option<SceneComponent>],
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).ReplaceAll)(
                ::windows::core::Vtable::as_raw(this),
                items.len() as u32,
                ::core::mem::transmute(items.as_ptr()),
            )
            .ok()
        }
    }
}
impl ::core::clone::Clone for SceneComponentCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SceneComponentCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SceneComponentCollection {}
impl ::core::fmt::Debug for SceneComponentCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SceneComponentCollection").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SceneComponentCollection {
    const SIGNATURE : ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice ( b"rc(Microsoft.UI.Composition.Scenes.SceneComponentCollection;pinterface({913337e9-11a1-4345-a3a2-4e7f956e222d};rc(Microsoft.UI.Composition.Scenes.SceneComponent;{f73361cb-8027-50e2-98ee-b2e3ea050a54})))" ) ;
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for SceneComponentCollection {
    type Vtable = ::windows::Foundation::Collections::IVector_Vtbl<SceneComponent>;
}
unsafe impl ::windows::core::Interface for SceneComponentCollection {
    const IID : ::windows::core::GUID = < ::windows::Foundation::Collections:: IVector :: < SceneComponent > as::windows::core::Interface >::IID ;
}
impl ::windows::core::RuntimeName for SceneComponentCollection {
    const NAME: &'static str = "Microsoft.UI.Composition.Scenes.SceneComponentCollection";
}
impl ::core::iter::IntoIterator for SceneComponentCollection {
    type Item = SceneComponent;
    type IntoIter = ::windows::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
impl ::core::iter::IntoIterator for &SceneComponentCollection {
    type Item = SceneComponent;
    type IntoIter = ::windows::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::windows::Foundation::Collections::VectorIterator::new(
            ::core::convert::TryInto::try_into(self).ok(),
        )
    }
}
::windows::core::interface_hierarchy!(
    SceneComponentCollection,
    ::windows::core::IUnknown,
    ::windows::core::IInspectable
);
impl ::core::convert::TryFrom<SceneComponentCollection> for super::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: SceneComponentCollection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SceneComponentCollection> for super::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: &SceneComponentCollection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&SceneComponentCollection>
    for ::windows::core::InParam<'a, super::IAnimationObject>
{
    type Error = ::windows::core::Error;
    fn try_from(value: &SceneComponentCollection) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<SceneComponentCollection> for ::windows::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: SceneComponentCollection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SceneComponentCollection> for ::windows::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &SceneComponentCollection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&SceneComponentCollection>
    for ::windows::core::InParam<'a, ::windows::Foundation::IClosable>
{
    type Error = ::windows::core::Error;
    fn try_from(value: &SceneComponentCollection) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<SceneComponentCollection>
    for ::windows::Foundation::Collections::IIterable<SceneComponent>
{
    type Error = ::windows::core::Error;
    fn try_from(value: SceneComponentCollection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SceneComponentCollection>
    for ::windows::Foundation::Collections::IIterable<SceneComponent>
{
    type Error = ::windows::core::Error;
    fn try_from(value: &SceneComponentCollection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&SceneComponentCollection>
    for ::windows::core::InParam<'a, ::windows::Foundation::Collections::IIterable<SceneComponent>>
{
    type Error = ::windows::core::Error;
    fn try_from(value: &SceneComponentCollection) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<SceneComponentCollection>
    for ::windows::Foundation::Collections::IVector<SceneComponent>
{
    type Error = ::windows::core::Error;
    fn try_from(value: SceneComponentCollection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SceneComponentCollection>
    for ::windows::Foundation::Collections::IVector<SceneComponent>
{
    type Error = ::windows::core::Error;
    fn try_from(value: &SceneComponentCollection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&SceneComponentCollection>
    for ::windows::core::InParam<'a, ::windows::Foundation::Collections::IVector<SceneComponent>>
{
    type Error = ::windows::core::Error;
    fn try_from(value: &SceneComponentCollection) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::From<SceneComponentCollection> for SceneObject {
    fn from(value: SceneComponentCollection) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&SceneComponentCollection> for SceneObject {
    fn from(value: &SceneComponentCollection) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::core::convert::From<&SceneComponentCollection>
    for ::windows::core::InParam<'a, SceneObject>
{
    fn from(value: &SceneComponentCollection) -> Self {
        ::windows::core::InParam::owned(value.into())
    }
}
impl ::core::convert::From<SceneComponentCollection> for super::CompositionObject {
    fn from(value: SceneComponentCollection) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&SceneComponentCollection> for super::CompositionObject {
    fn from(value: &SceneComponentCollection) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::core::convert::From<&SceneComponentCollection>
    for ::windows::core::InParam<'a, super::CompositionObject>
{
    fn from(value: &SceneComponentCollection) -> Self {
        ::windows::core::InParam::owned(value.into())
    }
}
unsafe impl ::core::marker::Send for SceneComponentCollection {}
unsafe impl ::core::marker::Sync for SceneComponentCollection {}
#[doc = "*Required features: `\"UI_Composition_Scenes\"`*"]
#[repr(transparent)]
pub struct SceneMaterial(::windows::core::IUnknown);
impl SceneMaterial {
    pub fn PopulatePropertyInfo(
        &self,
        propertyname: &::windows::core::HSTRING,
        propertyinfo: &super::AnimationPropertyInfo,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).PopulatePropertyInfo)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(propertyname),
                ::core::mem::transmute_copy(propertyinfo),
            )
            .ok()
        }
    }
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<::windows::Foundation::IClosable>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).Close)(::windows::core::Vtable::as_raw(this))
                .ok()
        }
    }
    pub fn Compositor(&self) -> ::windows::core::Result<super::Compositor> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Compositor)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::Compositor>(result__)
        }
    }
    pub fn Properties(&self) -> ::windows::core::Result<super::CompositionPropertySet> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Properties)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::CompositionPropertySet>(result__)
        }
    }
    pub fn StartAnimation<'a, P0>(
        &self,
        propertyname: &::windows::core::HSTRING,
        animation: P0,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::CompositionAnimation>>,
    {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).StartAnimation)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(propertyname),
                animation.into().abi(),
            )
            .ok()
        }
    }
    pub fn StopAnimation(
        &self,
        propertyname: &::windows::core::HSTRING,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).StopAnimation)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(propertyname),
            )
            .ok()
        }
    }
    pub fn Comment(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Comment)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetComment(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetComment)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn ImplicitAnimations(
        &self,
    ) -> ::windows::core::Result<super::ImplicitAnimationCollection> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ImplicitAnimations)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::ImplicitAnimationCollection>(result__)
        }
    }
    pub fn SetImplicitAnimations(
        &self,
        value: &super::ImplicitAnimationCollection,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetImplicitAnimations)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn StartAnimationGroup<'a, P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<'a, super::ICompositionAnimationBase>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).StartAnimationGroup)(
                ::windows::core::Vtable::as_raw(this),
                value.try_into().map_err(|e| e.into())?.abi(),
            )
            .ok()
        }
    }
    pub fn StopAnimationGroup<'a, P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<'a, super::ICompositionAnimationBase>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).StopAnimationGroup)(
                ::windows::core::Vtable::as_raw(this),
                value.try_into().map_err(|e| e.into())?.abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Dispatching\"`*"]
    #[cfg(feature = "UI_Dispatching")]
    pub fn DispatcherQueue(
        &self,
    ) -> ::windows::core::Result<super::super::Dispatching::DispatcherQueue> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DispatcherQueue)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::super::Dispatching::DispatcherQueue>(result__)
        }
    }
    pub fn TryGetAnimationController(
        &self,
        propertyname: &::windows::core::HSTRING,
    ) -> ::windows::core::Result<super::AnimationController> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TryGetAnimationController)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(propertyname),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::AnimationController>(result__)
        }
    }
    pub fn StartAnimationWithController<'a, P0>(
        &self,
        propertyname: &::windows::core::HSTRING,
        animation: P0,
        animationcontroller: &super::AnimationController,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::CompositionAnimation>>,
    {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject5>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).StartAnimationWithController)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(propertyname),
                animation.into().abi(),
                ::core::mem::transmute_copy(animationcontroller),
            )
            .ok()
        }
    }
}
impl ::core::clone::Clone for SceneMaterial {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SceneMaterial {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SceneMaterial {}
impl ::core::fmt::Debug for SceneMaterial {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SceneMaterial").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SceneMaterial {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Composition.Scenes.SceneMaterial;{042142a7-bf6b-57ad-badc-f581f38edb48})",
    );
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for SceneMaterial {
    type Vtable = ISceneMaterial_Vtbl;
}
unsafe impl ::windows::core::Interface for SceneMaterial {
    const IID: ::windows::core::GUID = <ISceneMaterial as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for SceneMaterial {
    const NAME: &'static str = "Microsoft.UI.Composition.Scenes.SceneMaterial";
}
::windows::core::interface_hierarchy!(
    SceneMaterial,
    ::windows::core::IUnknown,
    ::windows::core::IInspectable
);
impl ::core::convert::TryFrom<SceneMaterial> for super::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: SceneMaterial) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SceneMaterial> for super::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: &SceneMaterial) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&SceneMaterial>
    for ::windows::core::InParam<'a, super::IAnimationObject>
{
    type Error = ::windows::core::Error;
    fn try_from(value: &SceneMaterial) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<SceneMaterial> for ::windows::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: SceneMaterial) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SceneMaterial> for ::windows::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &SceneMaterial) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&SceneMaterial>
    for ::windows::core::InParam<'a, ::windows::Foundation::IClosable>
{
    type Error = ::windows::core::Error;
    fn try_from(value: &SceneMaterial) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::From<SceneMaterial> for SceneObject {
    fn from(value: SceneMaterial) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&SceneMaterial> for SceneObject {
    fn from(value: &SceneMaterial) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::core::convert::From<&SceneMaterial> for ::windows::core::InParam<'a, SceneObject> {
    fn from(value: &SceneMaterial) -> Self {
        ::windows::core::InParam::owned(value.into())
    }
}
impl ::core::convert::From<SceneMaterial> for super::CompositionObject {
    fn from(value: SceneMaterial) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&SceneMaterial> for super::CompositionObject {
    fn from(value: &SceneMaterial) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::core::convert::From<&SceneMaterial>
    for ::windows::core::InParam<'a, super::CompositionObject>
{
    fn from(value: &SceneMaterial) -> Self {
        ::windows::core::InParam::owned(value.into())
    }
}
unsafe impl ::core::marker::Send for SceneMaterial {}
unsafe impl ::core::marker::Sync for SceneMaterial {}
#[doc = "*Required features: `\"UI_Composition_Scenes\"`*"]
#[repr(transparent)]
pub struct SceneMaterialInput(::windows::core::IUnknown);
impl SceneMaterialInput {
    pub fn PopulatePropertyInfo(
        &self,
        propertyname: &::windows::core::HSTRING,
        propertyinfo: &super::AnimationPropertyInfo,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).PopulatePropertyInfo)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(propertyname),
                ::core::mem::transmute_copy(propertyinfo),
            )
            .ok()
        }
    }
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<::windows::Foundation::IClosable>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).Close)(::windows::core::Vtable::as_raw(this))
                .ok()
        }
    }
    pub fn Compositor(&self) -> ::windows::core::Result<super::Compositor> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Compositor)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::Compositor>(result__)
        }
    }
    pub fn Properties(&self) -> ::windows::core::Result<super::CompositionPropertySet> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Properties)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::CompositionPropertySet>(result__)
        }
    }
    pub fn StartAnimation<'a, P0>(
        &self,
        propertyname: &::windows::core::HSTRING,
        animation: P0,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::CompositionAnimation>>,
    {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).StartAnimation)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(propertyname),
                animation.into().abi(),
            )
            .ok()
        }
    }
    pub fn StopAnimation(
        &self,
        propertyname: &::windows::core::HSTRING,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).StopAnimation)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(propertyname),
            )
            .ok()
        }
    }
    pub fn Comment(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Comment)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetComment(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetComment)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn ImplicitAnimations(
        &self,
    ) -> ::windows::core::Result<super::ImplicitAnimationCollection> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ImplicitAnimations)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::ImplicitAnimationCollection>(result__)
        }
    }
    pub fn SetImplicitAnimations(
        &self,
        value: &super::ImplicitAnimationCollection,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetImplicitAnimations)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn StartAnimationGroup<'a, P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<'a, super::ICompositionAnimationBase>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).StartAnimationGroup)(
                ::windows::core::Vtable::as_raw(this),
                value.try_into().map_err(|e| e.into())?.abi(),
            )
            .ok()
        }
    }
    pub fn StopAnimationGroup<'a, P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<'a, super::ICompositionAnimationBase>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).StopAnimationGroup)(
                ::windows::core::Vtable::as_raw(this),
                value.try_into().map_err(|e| e.into())?.abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Dispatching\"`*"]
    #[cfg(feature = "UI_Dispatching")]
    pub fn DispatcherQueue(
        &self,
    ) -> ::windows::core::Result<super::super::Dispatching::DispatcherQueue> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DispatcherQueue)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::super::Dispatching::DispatcherQueue>(result__)
        }
    }
    pub fn TryGetAnimationController(
        &self,
        propertyname: &::windows::core::HSTRING,
    ) -> ::windows::core::Result<super::AnimationController> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TryGetAnimationController)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(propertyname),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::AnimationController>(result__)
        }
    }
    pub fn StartAnimationWithController<'a, P0>(
        &self,
        propertyname: &::windows::core::HSTRING,
        animation: P0,
        animationcontroller: &super::AnimationController,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::CompositionAnimation>>,
    {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject5>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).StartAnimationWithController)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(propertyname),
                animation.into().abi(),
                ::core::mem::transmute_copy(animationcontroller),
            )
            .ok()
        }
    }
}
impl ::core::clone::Clone for SceneMaterialInput {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SceneMaterialInput {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SceneMaterialInput {}
impl ::core::fmt::Debug for SceneMaterialInput {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SceneMaterialInput").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SceneMaterialInput {
    const SIGNATURE : ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice ( b"rc(Microsoft.UI.Composition.Scenes.SceneMaterialInput;{446bdade-719b-5db4-b699-f226d0062a2e})" ) ;
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for SceneMaterialInput {
    type Vtable = ISceneMaterialInput_Vtbl;
}
unsafe impl ::windows::core::Interface for SceneMaterialInput {
    const IID: ::windows::core::GUID = <ISceneMaterialInput as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for SceneMaterialInput {
    const NAME: &'static str = "Microsoft.UI.Composition.Scenes.SceneMaterialInput";
}
::windows::core::interface_hierarchy!(
    SceneMaterialInput,
    ::windows::core::IUnknown,
    ::windows::core::IInspectable
);
impl ::core::convert::TryFrom<SceneMaterialInput> for super::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: SceneMaterialInput) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SceneMaterialInput> for super::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: &SceneMaterialInput) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&SceneMaterialInput>
    for ::windows::core::InParam<'a, super::IAnimationObject>
{
    type Error = ::windows::core::Error;
    fn try_from(value: &SceneMaterialInput) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<SceneMaterialInput> for ::windows::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: SceneMaterialInput) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SceneMaterialInput> for ::windows::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &SceneMaterialInput) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&SceneMaterialInput>
    for ::windows::core::InParam<'a, ::windows::Foundation::IClosable>
{
    type Error = ::windows::core::Error;
    fn try_from(value: &SceneMaterialInput) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::From<SceneMaterialInput> for SceneObject {
    fn from(value: SceneMaterialInput) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&SceneMaterialInput> for SceneObject {
    fn from(value: &SceneMaterialInput) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::core::convert::From<&SceneMaterialInput> for ::windows::core::InParam<'a, SceneObject> {
    fn from(value: &SceneMaterialInput) -> Self {
        ::windows::core::InParam::owned(value.into())
    }
}
impl ::core::convert::From<SceneMaterialInput> for super::CompositionObject {
    fn from(value: SceneMaterialInput) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&SceneMaterialInput> for super::CompositionObject {
    fn from(value: &SceneMaterialInput) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::core::convert::From<&SceneMaterialInput>
    for ::windows::core::InParam<'a, super::CompositionObject>
{
    fn from(value: &SceneMaterialInput) -> Self {
        ::windows::core::InParam::owned(value.into())
    }
}
unsafe impl ::core::marker::Send for SceneMaterialInput {}
unsafe impl ::core::marker::Sync for SceneMaterialInput {}
#[doc = "*Required features: `\"UI_Composition_Scenes\"`*"]
#[repr(transparent)]
pub struct SceneMesh(::windows::core::IUnknown);
impl SceneMesh {
    pub fn PopulatePropertyInfo(
        &self,
        propertyname: &::windows::core::HSTRING,
        propertyinfo: &super::AnimationPropertyInfo,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).PopulatePropertyInfo)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(propertyname),
                ::core::mem::transmute_copy(propertyinfo),
            )
            .ok()
        }
    }
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<::windows::Foundation::IClosable>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).Close)(::windows::core::Vtable::as_raw(this))
                .ok()
        }
    }
    pub fn Compositor(&self) -> ::windows::core::Result<super::Compositor> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Compositor)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::Compositor>(result__)
        }
    }
    pub fn Properties(&self) -> ::windows::core::Result<super::CompositionPropertySet> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Properties)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::CompositionPropertySet>(result__)
        }
    }
    pub fn StartAnimation<'a, P0>(
        &self,
        propertyname: &::windows::core::HSTRING,
        animation: P0,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::CompositionAnimation>>,
    {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).StartAnimation)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(propertyname),
                animation.into().abi(),
            )
            .ok()
        }
    }
    pub fn StopAnimation(
        &self,
        propertyname: &::windows::core::HSTRING,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).StopAnimation)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(propertyname),
            )
            .ok()
        }
    }
    pub fn Comment(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Comment)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetComment(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetComment)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn ImplicitAnimations(
        &self,
    ) -> ::windows::core::Result<super::ImplicitAnimationCollection> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ImplicitAnimations)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::ImplicitAnimationCollection>(result__)
        }
    }
    pub fn SetImplicitAnimations(
        &self,
        value: &super::ImplicitAnimationCollection,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetImplicitAnimations)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn StartAnimationGroup<'a, P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<'a, super::ICompositionAnimationBase>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).StartAnimationGroup)(
                ::windows::core::Vtable::as_raw(this),
                value.try_into().map_err(|e| e.into())?.abi(),
            )
            .ok()
        }
    }
    pub fn StopAnimationGroup<'a, P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<'a, super::ICompositionAnimationBase>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).StopAnimationGroup)(
                ::windows::core::Vtable::as_raw(this),
                value.try_into().map_err(|e| e.into())?.abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Dispatching\"`*"]
    #[cfg(feature = "UI_Dispatching")]
    pub fn DispatcherQueue(
        &self,
    ) -> ::windows::core::Result<super::super::Dispatching::DispatcherQueue> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DispatcherQueue)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::super::Dispatching::DispatcherQueue>(result__)
        }
    }
    pub fn TryGetAnimationController(
        &self,
        propertyname: &::windows::core::HSTRING,
    ) -> ::windows::core::Result<super::AnimationController> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TryGetAnimationController)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(propertyname),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::AnimationController>(result__)
        }
    }
    pub fn StartAnimationWithController<'a, P0>(
        &self,
        propertyname: &::windows::core::HSTRING,
        animation: P0,
        animationcontroller: &super::AnimationController,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::CompositionAnimation>>,
    {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject5>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).StartAnimationWithController)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(propertyname),
                animation.into().abi(),
                ::core::mem::transmute_copy(animationcontroller),
            )
            .ok()
        }
    }
    pub fn Bounds(&self) -> ::windows::core::Result<SceneBoundingBox> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Bounds)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<SceneBoundingBox>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_DirectX\"`*"]
    #[cfg(feature = "Graphics_DirectX")]
    pub fn PrimitiveTopology(
        &self,
    ) -> ::windows::core::Result<super::super::super::Graphics::DirectX::DirectXPrimitiveTopology>
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PrimitiveTopology)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::super::super::Graphics::DirectX::DirectXPrimitiveTopology>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_DirectX\"`*"]
    #[cfg(feature = "Graphics_DirectX")]
    pub fn SetPrimitiveTopology(
        &self,
        value: super::super::super::Graphics::DirectX::DirectXPrimitiveTopology,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetPrimitiveTopology)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"Graphics_DirectX\"`*"]
    #[cfg(feature = "Graphics_DirectX")]
    pub fn FillMeshAttribute(
        &self,
        semantic: SceneAttributeSemantic,
        format: super::super::super::Graphics::DirectX::DirectXPixelFormat,
        memory: &::windows::Foundation::MemoryBuffer,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).FillMeshAttribute)(
                ::windows::core::Vtable::as_raw(this),
                semantic,
                format,
                ::core::mem::transmute_copy(memory),
            )
            .ok()
        }
    }
    pub fn Create(compositor: &super::Compositor) -> ::windows::core::Result<SceneMesh> {
        Self::ISceneMeshStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Create)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(compositor),
                result__.as_mut_ptr(),
            )
            .from_abi::<SceneMesh>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ISceneMeshStatics<R, F: FnOnce(&ISceneMeshStatics) -> ::windows::core::Result<R>>(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<SceneMesh, ISceneMeshStatics> =
            ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for SceneMesh {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SceneMesh {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SceneMesh {}
impl ::core::fmt::Debug for SceneMesh {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SceneMesh").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SceneMesh {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Composition.Scenes.SceneMesh;{5cf846aa-f53f-555e-a3ad-f5bc52ca32fb})",
    );
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for SceneMesh {
    type Vtable = ISceneMesh_Vtbl;
}
unsafe impl ::windows::core::Interface for SceneMesh {
    const IID: ::windows::core::GUID = <ISceneMesh as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for SceneMesh {
    const NAME: &'static str = "Microsoft.UI.Composition.Scenes.SceneMesh";
}
::windows::core::interface_hierarchy!(
    SceneMesh,
    ::windows::core::IUnknown,
    ::windows::core::IInspectable
);
impl ::core::convert::TryFrom<SceneMesh> for super::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: SceneMesh) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SceneMesh> for super::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: &SceneMesh) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&SceneMesh>
    for ::windows::core::InParam<'a, super::IAnimationObject>
{
    type Error = ::windows::core::Error;
    fn try_from(value: &SceneMesh) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<SceneMesh> for ::windows::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: SceneMesh) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SceneMesh> for ::windows::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &SceneMesh) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&SceneMesh>
    for ::windows::core::InParam<'a, ::windows::Foundation::IClosable>
{
    type Error = ::windows::core::Error;
    fn try_from(value: &SceneMesh) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::From<SceneMesh> for SceneObject {
    fn from(value: SceneMesh) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&SceneMesh> for SceneObject {
    fn from(value: &SceneMesh) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::core::convert::From<&SceneMesh> for ::windows::core::InParam<'a, SceneObject> {
    fn from(value: &SceneMesh) -> Self {
        ::windows::core::InParam::owned(value.into())
    }
}
impl ::core::convert::From<SceneMesh> for super::CompositionObject {
    fn from(value: SceneMesh) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&SceneMesh> for super::CompositionObject {
    fn from(value: &SceneMesh) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::core::convert::From<&SceneMesh>
    for ::windows::core::InParam<'a, super::CompositionObject>
{
    fn from(value: &SceneMesh) -> Self {
        ::windows::core::InParam::owned(value.into())
    }
}
unsafe impl ::core::marker::Send for SceneMesh {}
unsafe impl ::core::marker::Sync for SceneMesh {}
#[doc = "*Required features: `\"UI_Composition_Scenes\"`*"]
#[repr(transparent)]
pub struct SceneMeshMaterialAttributeMap(::windows::core::IUnknown);
impl SceneMeshMaterialAttributeMap {
    pub fn PopulatePropertyInfo(
        &self,
        propertyname: &::windows::core::HSTRING,
        propertyinfo: &super::AnimationPropertyInfo,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).PopulatePropertyInfo)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(propertyname),
                ::core::mem::transmute_copy(propertyinfo),
            )
            .ok()
        }
    }
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<::windows::Foundation::IClosable>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).Close)(::windows::core::Vtable::as_raw(this))
                .ok()
        }
    }
    pub fn Compositor(&self) -> ::windows::core::Result<super::Compositor> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Compositor)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::Compositor>(result__)
        }
    }
    pub fn Properties(&self) -> ::windows::core::Result<super::CompositionPropertySet> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Properties)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::CompositionPropertySet>(result__)
        }
    }
    pub fn StartAnimation<'a, P0>(
        &self,
        propertyname: &::windows::core::HSTRING,
        animation: P0,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::CompositionAnimation>>,
    {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).StartAnimation)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(propertyname),
                animation.into().abi(),
            )
            .ok()
        }
    }
    pub fn StopAnimation(
        &self,
        propertyname: &::windows::core::HSTRING,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).StopAnimation)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(propertyname),
            )
            .ok()
        }
    }
    pub fn Comment(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Comment)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetComment(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetComment)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn ImplicitAnimations(
        &self,
    ) -> ::windows::core::Result<super::ImplicitAnimationCollection> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ImplicitAnimations)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::ImplicitAnimationCollection>(result__)
        }
    }
    pub fn SetImplicitAnimations(
        &self,
        value: &super::ImplicitAnimationCollection,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetImplicitAnimations)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn StartAnimationGroup<'a, P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<'a, super::ICompositionAnimationBase>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).StartAnimationGroup)(
                ::windows::core::Vtable::as_raw(this),
                value.try_into().map_err(|e| e.into())?.abi(),
            )
            .ok()
        }
    }
    pub fn StopAnimationGroup<'a, P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<'a, super::ICompositionAnimationBase>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).StopAnimationGroup)(
                ::windows::core::Vtable::as_raw(this),
                value.try_into().map_err(|e| e.into())?.abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Dispatching\"`*"]
    #[cfg(feature = "UI_Dispatching")]
    pub fn DispatcherQueue(
        &self,
    ) -> ::windows::core::Result<super::super::Dispatching::DispatcherQueue> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DispatcherQueue)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::super::Dispatching::DispatcherQueue>(result__)
        }
    }
    pub fn TryGetAnimationController(
        &self,
        propertyname: &::windows::core::HSTRING,
    ) -> ::windows::core::Result<super::AnimationController> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TryGetAnimationController)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(propertyname),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::AnimationController>(result__)
        }
    }
    pub fn StartAnimationWithController<'a, P0>(
        &self,
        propertyname: &::windows::core::HSTRING,
        animation: P0,
        animationcontroller: &super::AnimationController,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::CompositionAnimation>>,
    {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject5>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).StartAnimationWithController)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(propertyname),
                animation.into().abi(),
                ::core::mem::transmute_copy(animationcontroller),
            )
            .ok()
        }
    }
    pub fn First(
        &self,
    ) -> ::windows::core::Result<
        ::windows::Foundation::Collections::IIterator<
            ::windows::Foundation::Collections::IKeyValuePair<
                ::windows::core::HSTRING,
                SceneAttributeSemantic,
            >,
        >,
    > {
        let this = &::windows::core::Interface::cast::<
            ::windows::Foundation::Collections::IIterable<
                ::windows::Foundation::Collections::IKeyValuePair<
                    ::windows::core::HSTRING,
                    SceneAttributeSemantic,
                >,
            >,
        >(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).First)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::Collections::IIterator<
                ::windows::Foundation::Collections::IKeyValuePair<
                    ::windows::core::HSTRING,
                    SceneAttributeSemantic,
                >,
            >>(result__)
        }
    }
    pub fn Lookup(
        &self,
        key: &::windows::core::HSTRING,
    ) -> ::windows::core::Result<SceneAttributeSemantic> {
        let this = &::windows::core::Interface::cast::<
            ::windows::Foundation::Collections::IMap<
                ::windows::core::HSTRING,
                SceneAttributeSemantic,
            >,
        >(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Lookup)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(key),
                result__.as_mut_ptr(),
            )
            .from_abi::<SceneAttributeSemantic>(result__)
        }
    }
    pub fn Size(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<
            ::windows::Foundation::Collections::IMap<
                ::windows::core::HSTRING,
                SceneAttributeSemantic,
            >,
        >(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Size)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<u32>(result__)
        }
    }
    pub fn HasKey(&self, key: &::windows::core::HSTRING) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<
            ::windows::Foundation::Collections::IMap<
                ::windows::core::HSTRING,
                SceneAttributeSemantic,
            >,
        >(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).HasKey)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(key),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn GetView(
        &self,
    ) -> ::windows::core::Result<
        ::windows::Foundation::Collections::IMapView<
            ::windows::core::HSTRING,
            SceneAttributeSemantic,
        >,
    > {
        let this = &::windows::core::Interface::cast::<
            ::windows::Foundation::Collections::IMap<
                ::windows::core::HSTRING,
                SceneAttributeSemantic,
            >,
        >(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetView)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::Collections::IMapView<
                ::windows::core::HSTRING,
                SceneAttributeSemantic,
            >>(result__)
        }
    }
    pub fn Insert(
        &self,
        key: &::windows::core::HSTRING,
        value: SceneAttributeSemantic,
    ) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<
            ::windows::Foundation::Collections::IMap<
                ::windows::core::HSTRING,
                SceneAttributeSemantic,
            >,
        >(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Insert)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(key),
                value,
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn Remove(&self, key: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<
            ::windows::Foundation::Collections::IMap<
                ::windows::core::HSTRING,
                SceneAttributeSemantic,
            >,
        >(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).Remove)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(key),
            )
            .ok()
        }
    }
    pub fn Clear(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<
            ::windows::Foundation::Collections::IMap<
                ::windows::core::HSTRING,
                SceneAttributeSemantic,
            >,
        >(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).Clear)(::windows::core::Vtable::as_raw(this))
                .ok()
        }
    }
}
impl ::core::clone::Clone for SceneMeshMaterialAttributeMap {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SceneMeshMaterialAttributeMap {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SceneMeshMaterialAttributeMap {}
impl ::core::fmt::Debug for SceneMeshMaterialAttributeMap {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SceneMeshMaterialAttributeMap").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SceneMeshMaterialAttributeMap {
    const SIGNATURE : ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice ( b"rc(Microsoft.UI.Composition.Scenes.SceneMeshMaterialAttributeMap;{2360c457-edae-5660-bedc-89096582ed70})" ) ;
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for SceneMeshMaterialAttributeMap {
    type Vtable = ISceneMeshMaterialAttributeMap_Vtbl;
}
unsafe impl ::windows::core::Interface for SceneMeshMaterialAttributeMap {
    const IID: ::windows::core::GUID =
        <ISceneMeshMaterialAttributeMap as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for SceneMeshMaterialAttributeMap {
    const NAME: &'static str = "Microsoft.UI.Composition.Scenes.SceneMeshMaterialAttributeMap";
}
impl ::core::iter::IntoIterator for SceneMeshMaterialAttributeMap {
    type Item = ::windows::Foundation::Collections::IKeyValuePair<
        ::windows::core::HSTRING,
        SceneAttributeSemantic,
    >;
    type IntoIter = ::windows::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
impl ::core::iter::IntoIterator for &SceneMeshMaterialAttributeMap {
    type Item = ::windows::Foundation::Collections::IKeyValuePair<
        ::windows::core::HSTRING,
        SceneAttributeSemantic,
    >;
    type IntoIter = ::windows::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
::windows::core::interface_hierarchy!(
    SceneMeshMaterialAttributeMap,
    ::windows::core::IUnknown,
    ::windows::core::IInspectable
);
impl ::core::convert::TryFrom<SceneMeshMaterialAttributeMap> for super::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: SceneMeshMaterialAttributeMap) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SceneMeshMaterialAttributeMap> for super::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: &SceneMeshMaterialAttributeMap) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&SceneMeshMaterialAttributeMap>
    for ::windows::core::InParam<'a, super::IAnimationObject>
{
    type Error = ::windows::core::Error;
    fn try_from(value: &SceneMeshMaterialAttributeMap) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<SceneMeshMaterialAttributeMap> for ::windows::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: SceneMeshMaterialAttributeMap) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SceneMeshMaterialAttributeMap> for ::windows::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &SceneMeshMaterialAttributeMap) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&SceneMeshMaterialAttributeMap>
    for ::windows::core::InParam<'a, ::windows::Foundation::IClosable>
{
    type Error = ::windows::core::Error;
    fn try_from(value: &SceneMeshMaterialAttributeMap) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<SceneMeshMaterialAttributeMap>
    for ::windows::Foundation::Collections::IIterable<
        ::windows::Foundation::Collections::IKeyValuePair<
            ::windows::core::HSTRING,
            SceneAttributeSemantic,
        >,
    >
{
    type Error = ::windows::core::Error;
    fn try_from(value: SceneMeshMaterialAttributeMap) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SceneMeshMaterialAttributeMap>
    for ::windows::Foundation::Collections::IIterable<
        ::windows::Foundation::Collections::IKeyValuePair<
            ::windows::core::HSTRING,
            SceneAttributeSemantic,
        >,
    >
{
    type Error = ::windows::core::Error;
    fn try_from(value: &SceneMeshMaterialAttributeMap) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&SceneMeshMaterialAttributeMap>
    for ::windows::core::InParam<
        'a,
        ::windows::Foundation::Collections::IIterable<
            ::windows::Foundation::Collections::IKeyValuePair<
                ::windows::core::HSTRING,
                SceneAttributeSemantic,
            >,
        >,
    >
{
    type Error = ::windows::core::Error;
    fn try_from(value: &SceneMeshMaterialAttributeMap) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<SceneMeshMaterialAttributeMap>
    for ::windows::Foundation::Collections::IMap<::windows::core::HSTRING, SceneAttributeSemantic>
{
    type Error = ::windows::core::Error;
    fn try_from(value: SceneMeshMaterialAttributeMap) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SceneMeshMaterialAttributeMap>
    for ::windows::Foundation::Collections::IMap<::windows::core::HSTRING, SceneAttributeSemantic>
{
    type Error = ::windows::core::Error;
    fn try_from(value: &SceneMeshMaterialAttributeMap) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&SceneMeshMaterialAttributeMap>
    for ::windows::core::InParam<
        'a,
        ::windows::Foundation::Collections::IMap<::windows::core::HSTRING, SceneAttributeSemantic>,
    >
{
    type Error = ::windows::core::Error;
    fn try_from(value: &SceneMeshMaterialAttributeMap) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::From<SceneMeshMaterialAttributeMap> for SceneObject {
    fn from(value: SceneMeshMaterialAttributeMap) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&SceneMeshMaterialAttributeMap> for SceneObject {
    fn from(value: &SceneMeshMaterialAttributeMap) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::core::convert::From<&SceneMeshMaterialAttributeMap>
    for ::windows::core::InParam<'a, SceneObject>
{
    fn from(value: &SceneMeshMaterialAttributeMap) -> Self {
        ::windows::core::InParam::owned(value.into())
    }
}
impl ::core::convert::From<SceneMeshMaterialAttributeMap> for super::CompositionObject {
    fn from(value: SceneMeshMaterialAttributeMap) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&SceneMeshMaterialAttributeMap> for super::CompositionObject {
    fn from(value: &SceneMeshMaterialAttributeMap) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::core::convert::From<&SceneMeshMaterialAttributeMap>
    for ::windows::core::InParam<'a, super::CompositionObject>
{
    fn from(value: &SceneMeshMaterialAttributeMap) -> Self {
        ::windows::core::InParam::owned(value.into())
    }
}
unsafe impl ::core::marker::Send for SceneMeshMaterialAttributeMap {}
unsafe impl ::core::marker::Sync for SceneMeshMaterialAttributeMap {}
#[doc = "*Required features: `\"UI_Composition_Scenes\"`*"]
#[repr(transparent)]
pub struct SceneMeshRendererComponent(::windows::core::IUnknown);
impl SceneMeshRendererComponent {
    pub fn PopulatePropertyInfo(
        &self,
        propertyname: &::windows::core::HSTRING,
        propertyinfo: &super::AnimationPropertyInfo,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).PopulatePropertyInfo)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(propertyname),
                ::core::mem::transmute_copy(propertyinfo),
            )
            .ok()
        }
    }
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<::windows::Foundation::IClosable>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).Close)(::windows::core::Vtable::as_raw(this))
                .ok()
        }
    }
    pub fn Compositor(&self) -> ::windows::core::Result<super::Compositor> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Compositor)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::Compositor>(result__)
        }
    }
    pub fn Properties(&self) -> ::windows::core::Result<super::CompositionPropertySet> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Properties)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::CompositionPropertySet>(result__)
        }
    }
    pub fn StartAnimation<'a, P0>(
        &self,
        propertyname: &::windows::core::HSTRING,
        animation: P0,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::CompositionAnimation>>,
    {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).StartAnimation)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(propertyname),
                animation.into().abi(),
            )
            .ok()
        }
    }
    pub fn StopAnimation(
        &self,
        propertyname: &::windows::core::HSTRING,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).StopAnimation)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(propertyname),
            )
            .ok()
        }
    }
    pub fn Comment(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Comment)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetComment(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetComment)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn ImplicitAnimations(
        &self,
    ) -> ::windows::core::Result<super::ImplicitAnimationCollection> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ImplicitAnimations)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::ImplicitAnimationCollection>(result__)
        }
    }
    pub fn SetImplicitAnimations(
        &self,
        value: &super::ImplicitAnimationCollection,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetImplicitAnimations)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn StartAnimationGroup<'a, P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<'a, super::ICompositionAnimationBase>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).StartAnimationGroup)(
                ::windows::core::Vtable::as_raw(this),
                value.try_into().map_err(|e| e.into())?.abi(),
            )
            .ok()
        }
    }
    pub fn StopAnimationGroup<'a, P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<'a, super::ICompositionAnimationBase>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).StopAnimationGroup)(
                ::windows::core::Vtable::as_raw(this),
                value.try_into().map_err(|e| e.into())?.abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Dispatching\"`*"]
    #[cfg(feature = "UI_Dispatching")]
    pub fn DispatcherQueue(
        &self,
    ) -> ::windows::core::Result<super::super::Dispatching::DispatcherQueue> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DispatcherQueue)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::super::Dispatching::DispatcherQueue>(result__)
        }
    }
    pub fn TryGetAnimationController(
        &self,
        propertyname: &::windows::core::HSTRING,
    ) -> ::windows::core::Result<super::AnimationController> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TryGetAnimationController)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(propertyname),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::AnimationController>(result__)
        }
    }
    pub fn StartAnimationWithController<'a, P0>(
        &self,
        propertyname: &::windows::core::HSTRING,
        animation: P0,
        animationcontroller: &super::AnimationController,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::CompositionAnimation>>,
    {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject5>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).StartAnimationWithController)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(propertyname),
                animation.into().abi(),
                ::core::mem::transmute_copy(animationcontroller),
            )
            .ok()
        }
    }
    pub fn ComponentType(&self) -> ::windows::core::Result<SceneComponentType> {
        let this = &::windows::core::Interface::cast::<ISceneComponent>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ComponentType)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<SceneComponentType>(result__)
        }
    }
    pub fn Material(&self) -> ::windows::core::Result<SceneMaterial> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Material)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<SceneMaterial>(result__)
        }
    }
    pub fn SetMaterial<'a, P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, SceneMaterial>>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetMaterial)(
                ::windows::core::Vtable::as_raw(this),
                value.into().abi(),
            )
            .ok()
        }
    }
    pub fn Mesh(&self) -> ::windows::core::Result<SceneMesh> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Mesh)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<SceneMesh>(result__)
        }
    }
    pub fn SetMesh(&self, value: &SceneMesh) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetMesh)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn UVMappings(&self) -> ::windows::core::Result<SceneMeshMaterialAttributeMap> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).UVMappings)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<SceneMeshMaterialAttributeMap>(result__)
        }
    }
    pub fn Create(
        compositor: &super::Compositor,
    ) -> ::windows::core::Result<SceneMeshRendererComponent> {
        Self::ISceneMeshRendererComponentStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Create)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(compositor),
                result__.as_mut_ptr(),
            )
            .from_abi::<SceneMeshRendererComponent>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ISceneMeshRendererComponentStatics<
        R,
        F: FnOnce(&ISceneMeshRendererComponentStatics) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<
            SceneMeshRendererComponent,
            ISceneMeshRendererComponentStatics,
        > = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for SceneMeshRendererComponent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SceneMeshRendererComponent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SceneMeshRendererComponent {}
impl ::core::fmt::Debug for SceneMeshRendererComponent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SceneMeshRendererComponent").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SceneMeshRendererComponent {
    const SIGNATURE : ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice ( b"rc(Microsoft.UI.Composition.Scenes.SceneMeshRendererComponent;{d2be85a0-70a8-5c62-84d8-8ba55e4c64a9})" ) ;
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for SceneMeshRendererComponent {
    type Vtable = ISceneMeshRendererComponent_Vtbl;
}
unsafe impl ::windows::core::Interface for SceneMeshRendererComponent {
    const IID: ::windows::core::GUID =
        <ISceneMeshRendererComponent as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for SceneMeshRendererComponent {
    const NAME: &'static str = "Microsoft.UI.Composition.Scenes.SceneMeshRendererComponent";
}
::windows::core::interface_hierarchy!(
    SceneMeshRendererComponent,
    ::windows::core::IUnknown,
    ::windows::core::IInspectable
);
impl ::core::convert::TryFrom<SceneMeshRendererComponent> for super::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: SceneMeshRendererComponent) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SceneMeshRendererComponent> for super::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: &SceneMeshRendererComponent) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&SceneMeshRendererComponent>
    for ::windows::core::InParam<'a, super::IAnimationObject>
{
    type Error = ::windows::core::Error;
    fn try_from(value: &SceneMeshRendererComponent) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<SceneMeshRendererComponent> for ::windows::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: SceneMeshRendererComponent) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SceneMeshRendererComponent> for ::windows::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &SceneMeshRendererComponent) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&SceneMeshRendererComponent>
    for ::windows::core::InParam<'a, ::windows::Foundation::IClosable>
{
    type Error = ::windows::core::Error;
    fn try_from(value: &SceneMeshRendererComponent) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::From<SceneMeshRendererComponent> for SceneRendererComponent {
    fn from(value: SceneMeshRendererComponent) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&SceneMeshRendererComponent> for SceneRendererComponent {
    fn from(value: &SceneMeshRendererComponent) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::core::convert::From<&SceneMeshRendererComponent>
    for ::windows::core::InParam<'a, SceneRendererComponent>
{
    fn from(value: &SceneMeshRendererComponent) -> Self {
        ::windows::core::InParam::owned(value.into())
    }
}
impl ::core::convert::From<SceneMeshRendererComponent> for SceneComponent {
    fn from(value: SceneMeshRendererComponent) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&SceneMeshRendererComponent> for SceneComponent {
    fn from(value: &SceneMeshRendererComponent) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::core::convert::From<&SceneMeshRendererComponent>
    for ::windows::core::InParam<'a, SceneComponent>
{
    fn from(value: &SceneMeshRendererComponent) -> Self {
        ::windows::core::InParam::owned(value.into())
    }
}
impl ::core::convert::From<SceneMeshRendererComponent> for SceneObject {
    fn from(value: SceneMeshRendererComponent) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&SceneMeshRendererComponent> for SceneObject {
    fn from(value: &SceneMeshRendererComponent) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::core::convert::From<&SceneMeshRendererComponent>
    for ::windows::core::InParam<'a, SceneObject>
{
    fn from(value: &SceneMeshRendererComponent) -> Self {
        ::windows::core::InParam::owned(value.into())
    }
}
impl ::core::convert::From<SceneMeshRendererComponent> for super::CompositionObject {
    fn from(value: SceneMeshRendererComponent) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&SceneMeshRendererComponent> for super::CompositionObject {
    fn from(value: &SceneMeshRendererComponent) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::core::convert::From<&SceneMeshRendererComponent>
    for ::windows::core::InParam<'a, super::CompositionObject>
{
    fn from(value: &SceneMeshRendererComponent) -> Self {
        ::windows::core::InParam::owned(value.into())
    }
}
unsafe impl ::core::marker::Send for SceneMeshRendererComponent {}
unsafe impl ::core::marker::Sync for SceneMeshRendererComponent {}
#[doc = "*Required features: `\"UI_Composition_Scenes\"`*"]
#[repr(transparent)]
pub struct SceneMetallicRoughnessMaterial(::windows::core::IUnknown);
impl SceneMetallicRoughnessMaterial {
    pub fn PopulatePropertyInfo(
        &self,
        propertyname: &::windows::core::HSTRING,
        propertyinfo: &super::AnimationPropertyInfo,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).PopulatePropertyInfo)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(propertyname),
                ::core::mem::transmute_copy(propertyinfo),
            )
            .ok()
        }
    }
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<::windows::Foundation::IClosable>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).Close)(::windows::core::Vtable::as_raw(this))
                .ok()
        }
    }
    pub fn Compositor(&self) -> ::windows::core::Result<super::Compositor> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Compositor)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::Compositor>(result__)
        }
    }
    pub fn Properties(&self) -> ::windows::core::Result<super::CompositionPropertySet> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Properties)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::CompositionPropertySet>(result__)
        }
    }
    pub fn StartAnimation<'a, P0>(
        &self,
        propertyname: &::windows::core::HSTRING,
        animation: P0,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::CompositionAnimation>>,
    {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).StartAnimation)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(propertyname),
                animation.into().abi(),
            )
            .ok()
        }
    }
    pub fn StopAnimation(
        &self,
        propertyname: &::windows::core::HSTRING,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).StopAnimation)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(propertyname),
            )
            .ok()
        }
    }
    pub fn Comment(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Comment)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetComment(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetComment)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn ImplicitAnimations(
        &self,
    ) -> ::windows::core::Result<super::ImplicitAnimationCollection> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ImplicitAnimations)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::ImplicitAnimationCollection>(result__)
        }
    }
    pub fn SetImplicitAnimations(
        &self,
        value: &super::ImplicitAnimationCollection,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetImplicitAnimations)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn StartAnimationGroup<'a, P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<'a, super::ICompositionAnimationBase>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).StartAnimationGroup)(
                ::windows::core::Vtable::as_raw(this),
                value.try_into().map_err(|e| e.into())?.abi(),
            )
            .ok()
        }
    }
    pub fn StopAnimationGroup<'a, P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<'a, super::ICompositionAnimationBase>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).StopAnimationGroup)(
                ::windows::core::Vtable::as_raw(this),
                value.try_into().map_err(|e| e.into())?.abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Dispatching\"`*"]
    #[cfg(feature = "UI_Dispatching")]
    pub fn DispatcherQueue(
        &self,
    ) -> ::windows::core::Result<super::super::Dispatching::DispatcherQueue> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DispatcherQueue)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::super::Dispatching::DispatcherQueue>(result__)
        }
    }
    pub fn TryGetAnimationController(
        &self,
        propertyname: &::windows::core::HSTRING,
    ) -> ::windows::core::Result<super::AnimationController> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TryGetAnimationController)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(propertyname),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::AnimationController>(result__)
        }
    }
    pub fn StartAnimationWithController<'a, P0>(
        &self,
        propertyname: &::windows::core::HSTRING,
        animation: P0,
        animationcontroller: &super::AnimationController,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::CompositionAnimation>>,
    {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject5>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).StartAnimationWithController)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(propertyname),
                animation.into().abi(),
                ::core::mem::transmute_copy(animationcontroller),
            )
            .ok()
        }
    }
    pub fn BaseColorInput(&self) -> ::windows::core::Result<SceneMaterialInput> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).BaseColorInput)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<SceneMaterialInput>(result__)
        }
    }
    pub fn SetBaseColorInput<'a, P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, SceneMaterialInput>>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetBaseColorInput)(
                ::windows::core::Vtable::as_raw(this),
                value.into().abi(),
            )
            .ok()
        }
    }
    pub fn BaseColorFactor(
        &self,
    ) -> ::windows::core::Result<::windows::Foundation::Numerics::Vector4> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).BaseColorFactor)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::Numerics::Vector4>(result__)
        }
    }
    pub fn SetBaseColorFactor(
        &self,
        value: ::windows::Foundation::Numerics::Vector4,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetBaseColorFactor)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn MetallicFactor(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MetallicFactor)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<f32>(result__)
        }
    }
    pub fn SetMetallicFactor(&self, value: f32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetMetallicFactor)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn MetallicRoughnessInput(&self) -> ::windows::core::Result<SceneMaterialInput> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MetallicRoughnessInput)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<SceneMaterialInput>(result__)
        }
    }
    pub fn SetMetallicRoughnessInput<'a, P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, SceneMaterialInput>>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetMetallicRoughnessInput)(
                ::windows::core::Vtable::as_raw(this),
                value.into().abi(),
            )
            .ok()
        }
    }
    pub fn RoughnessFactor(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RoughnessFactor)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<f32>(result__)
        }
    }
    pub fn SetRoughnessFactor(&self, value: f32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetRoughnessFactor)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn Create(
        compositor: &super::Compositor,
    ) -> ::windows::core::Result<SceneMetallicRoughnessMaterial> {
        Self::ISceneMetallicRoughnessMaterialStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Create)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(compositor),
                result__.as_mut_ptr(),
            )
            .from_abi::<SceneMetallicRoughnessMaterial>(result__)
        })
    }
    pub fn AlphaCutoff(&self) -> ::windows::core::Result<f32> {
        let this = &::windows::core::Interface::cast::<IScenePbrMaterial>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AlphaCutoff)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<f32>(result__)
        }
    }
    pub fn SetAlphaCutoff(&self, value: f32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IScenePbrMaterial>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetAlphaCutoff)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn AlphaMode(&self) -> ::windows::core::Result<SceneAlphaMode> {
        let this = &::windows::core::Interface::cast::<IScenePbrMaterial>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AlphaMode)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<SceneAlphaMode>(result__)
        }
    }
    pub fn SetAlphaMode(&self, value: SceneAlphaMode) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IScenePbrMaterial>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetAlphaMode)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn EmissiveInput(&self) -> ::windows::core::Result<SceneMaterialInput> {
        let this = &::windows::core::Interface::cast::<IScenePbrMaterial>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).EmissiveInput)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<SceneMaterialInput>(result__)
        }
    }
    pub fn SetEmissiveInput<'a, P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, SceneMaterialInput>>,
    {
        let this = &::windows::core::Interface::cast::<IScenePbrMaterial>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetEmissiveInput)(
                ::windows::core::Vtable::as_raw(this),
                value.into().abi(),
            )
            .ok()
        }
    }
    pub fn EmissiveFactor(
        &self,
    ) -> ::windows::core::Result<::windows::Foundation::Numerics::Vector3> {
        let this = &::windows::core::Interface::cast::<IScenePbrMaterial>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).EmissiveFactor)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::Numerics::Vector3>(result__)
        }
    }
    pub fn SetEmissiveFactor(
        &self,
        value: ::windows::Foundation::Numerics::Vector3,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IScenePbrMaterial>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetEmissiveFactor)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsDoubleSided(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IScenePbrMaterial>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsDoubleSided)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetIsDoubleSided(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IScenePbrMaterial>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetIsDoubleSided)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn NormalInput(&self) -> ::windows::core::Result<SceneMaterialInput> {
        let this = &::windows::core::Interface::cast::<IScenePbrMaterial>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NormalInput)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<SceneMaterialInput>(result__)
        }
    }
    pub fn SetNormalInput<'a, P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, SceneMaterialInput>>,
    {
        let this = &::windows::core::Interface::cast::<IScenePbrMaterial>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetNormalInput)(
                ::windows::core::Vtable::as_raw(this),
                value.into().abi(),
            )
            .ok()
        }
    }
    pub fn NormalScale(&self) -> ::windows::core::Result<f32> {
        let this = &::windows::core::Interface::cast::<IScenePbrMaterial>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NormalScale)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<f32>(result__)
        }
    }
    pub fn SetNormalScale(&self, value: f32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IScenePbrMaterial>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetNormalScale)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn OcclusionInput(&self) -> ::windows::core::Result<SceneMaterialInput> {
        let this = &::windows::core::Interface::cast::<IScenePbrMaterial>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).OcclusionInput)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<SceneMaterialInput>(result__)
        }
    }
    pub fn SetOcclusionInput<'a, P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, SceneMaterialInput>>,
    {
        let this = &::windows::core::Interface::cast::<IScenePbrMaterial>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetOcclusionInput)(
                ::windows::core::Vtable::as_raw(this),
                value.into().abi(),
            )
            .ok()
        }
    }
    pub fn OcclusionStrength(&self) -> ::windows::core::Result<f32> {
        let this = &::windows::core::Interface::cast::<IScenePbrMaterial>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).OcclusionStrength)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<f32>(result__)
        }
    }
    pub fn SetOcclusionStrength(&self, value: f32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IScenePbrMaterial>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetOcclusionStrength)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc(hidden)]
    pub fn ISceneMetallicRoughnessMaterialStatics<
        R,
        F: FnOnce(&ISceneMetallicRoughnessMaterialStatics) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<
            SceneMetallicRoughnessMaterial,
            ISceneMetallicRoughnessMaterialStatics,
        > = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for SceneMetallicRoughnessMaterial {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SceneMetallicRoughnessMaterial {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SceneMetallicRoughnessMaterial {}
impl ::core::fmt::Debug for SceneMetallicRoughnessMaterial {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SceneMetallicRoughnessMaterial").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SceneMetallicRoughnessMaterial {
    const SIGNATURE : ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice ( b"rc(Microsoft.UI.Composition.Scenes.SceneMetallicRoughnessMaterial;{0a4afcf4-7bae-5702-9b85-8bc849f39987})" ) ;
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for SceneMetallicRoughnessMaterial {
    type Vtable = ISceneMetallicRoughnessMaterial_Vtbl;
}
unsafe impl ::windows::core::Interface for SceneMetallicRoughnessMaterial {
    const IID: ::windows::core::GUID =
        <ISceneMetallicRoughnessMaterial as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for SceneMetallicRoughnessMaterial {
    const NAME: &'static str = "Microsoft.UI.Composition.Scenes.SceneMetallicRoughnessMaterial";
}
::windows::core::interface_hierarchy!(
    SceneMetallicRoughnessMaterial,
    ::windows::core::IUnknown,
    ::windows::core::IInspectable
);
impl ::core::convert::TryFrom<SceneMetallicRoughnessMaterial> for super::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: SceneMetallicRoughnessMaterial) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SceneMetallicRoughnessMaterial> for super::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: &SceneMetallicRoughnessMaterial) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&SceneMetallicRoughnessMaterial>
    for ::windows::core::InParam<'a, super::IAnimationObject>
{
    type Error = ::windows::core::Error;
    fn try_from(value: &SceneMetallicRoughnessMaterial) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<SceneMetallicRoughnessMaterial> for ::windows::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: SceneMetallicRoughnessMaterial) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SceneMetallicRoughnessMaterial>
    for ::windows::Foundation::IClosable
{
    type Error = ::windows::core::Error;
    fn try_from(value: &SceneMetallicRoughnessMaterial) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&SceneMetallicRoughnessMaterial>
    for ::windows::core::InParam<'a, ::windows::Foundation::IClosable>
{
    type Error = ::windows::core::Error;
    fn try_from(value: &SceneMetallicRoughnessMaterial) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::From<SceneMetallicRoughnessMaterial> for ScenePbrMaterial {
    fn from(value: SceneMetallicRoughnessMaterial) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&SceneMetallicRoughnessMaterial> for ScenePbrMaterial {
    fn from(value: &SceneMetallicRoughnessMaterial) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::core::convert::From<&SceneMetallicRoughnessMaterial>
    for ::windows::core::InParam<'a, ScenePbrMaterial>
{
    fn from(value: &SceneMetallicRoughnessMaterial) -> Self {
        ::windows::core::InParam::owned(value.into())
    }
}
impl ::core::convert::From<SceneMetallicRoughnessMaterial> for SceneMaterial {
    fn from(value: SceneMetallicRoughnessMaterial) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&SceneMetallicRoughnessMaterial> for SceneMaterial {
    fn from(value: &SceneMetallicRoughnessMaterial) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::core::convert::From<&SceneMetallicRoughnessMaterial>
    for ::windows::core::InParam<'a, SceneMaterial>
{
    fn from(value: &SceneMetallicRoughnessMaterial) -> Self {
        ::windows::core::InParam::owned(value.into())
    }
}
impl ::core::convert::From<SceneMetallicRoughnessMaterial> for SceneObject {
    fn from(value: SceneMetallicRoughnessMaterial) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&SceneMetallicRoughnessMaterial> for SceneObject {
    fn from(value: &SceneMetallicRoughnessMaterial) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::core::convert::From<&SceneMetallicRoughnessMaterial>
    for ::windows::core::InParam<'a, SceneObject>
{
    fn from(value: &SceneMetallicRoughnessMaterial) -> Self {
        ::windows::core::InParam::owned(value.into())
    }
}
impl ::core::convert::From<SceneMetallicRoughnessMaterial> for super::CompositionObject {
    fn from(value: SceneMetallicRoughnessMaterial) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&SceneMetallicRoughnessMaterial> for super::CompositionObject {
    fn from(value: &SceneMetallicRoughnessMaterial) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::core::convert::From<&SceneMetallicRoughnessMaterial>
    for ::windows::core::InParam<'a, super::CompositionObject>
{
    fn from(value: &SceneMetallicRoughnessMaterial) -> Self {
        ::windows::core::InParam::owned(value.into())
    }
}
unsafe impl ::core::marker::Send for SceneMetallicRoughnessMaterial {}
unsafe impl ::core::marker::Sync for SceneMetallicRoughnessMaterial {}
#[doc = "*Required features: `\"UI_Composition_Scenes\"`*"]
#[repr(transparent)]
pub struct SceneModelTransform(::windows::core::IUnknown);
impl SceneModelTransform {
    pub fn PopulatePropertyInfo(
        &self,
        propertyname: &::windows::core::HSTRING,
        propertyinfo: &super::AnimationPropertyInfo,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).PopulatePropertyInfo)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(propertyname),
                ::core::mem::transmute_copy(propertyinfo),
            )
            .ok()
        }
    }
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<::windows::Foundation::IClosable>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).Close)(::windows::core::Vtable::as_raw(this))
                .ok()
        }
    }
    pub fn Compositor(&self) -> ::windows::core::Result<super::Compositor> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Compositor)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::Compositor>(result__)
        }
    }
    pub fn Properties(&self) -> ::windows::core::Result<super::CompositionPropertySet> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Properties)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::CompositionPropertySet>(result__)
        }
    }
    pub fn StartAnimation<'a, P0>(
        &self,
        propertyname: &::windows::core::HSTRING,
        animation: P0,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::CompositionAnimation>>,
    {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).StartAnimation)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(propertyname),
                animation.into().abi(),
            )
            .ok()
        }
    }
    pub fn StopAnimation(
        &self,
        propertyname: &::windows::core::HSTRING,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).StopAnimation)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(propertyname),
            )
            .ok()
        }
    }
    pub fn Comment(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Comment)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetComment(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetComment)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn ImplicitAnimations(
        &self,
    ) -> ::windows::core::Result<super::ImplicitAnimationCollection> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ImplicitAnimations)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::ImplicitAnimationCollection>(result__)
        }
    }
    pub fn SetImplicitAnimations(
        &self,
        value: &super::ImplicitAnimationCollection,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetImplicitAnimations)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn StartAnimationGroup<'a, P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<'a, super::ICompositionAnimationBase>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).StartAnimationGroup)(
                ::windows::core::Vtable::as_raw(this),
                value.try_into().map_err(|e| e.into())?.abi(),
            )
            .ok()
        }
    }
    pub fn StopAnimationGroup<'a, P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<'a, super::ICompositionAnimationBase>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).StopAnimationGroup)(
                ::windows::core::Vtable::as_raw(this),
                value.try_into().map_err(|e| e.into())?.abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Dispatching\"`*"]
    #[cfg(feature = "UI_Dispatching")]
    pub fn DispatcherQueue(
        &self,
    ) -> ::windows::core::Result<super::super::Dispatching::DispatcherQueue> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DispatcherQueue)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::super::Dispatching::DispatcherQueue>(result__)
        }
    }
    pub fn TryGetAnimationController(
        &self,
        propertyname: &::windows::core::HSTRING,
    ) -> ::windows::core::Result<super::AnimationController> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TryGetAnimationController)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(propertyname),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::AnimationController>(result__)
        }
    }
    pub fn StartAnimationWithController<'a, P0>(
        &self,
        propertyname: &::windows::core::HSTRING,
        animation: P0,
        animationcontroller: &super::AnimationController,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::CompositionAnimation>>,
    {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject5>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).StartAnimationWithController)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(propertyname),
                animation.into().abi(),
                ::core::mem::transmute_copy(animationcontroller),
            )
            .ok()
        }
    }
    pub fn Orientation(
        &self,
    ) -> ::windows::core::Result<::windows::Foundation::Numerics::Quaternion> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Orientation)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::Numerics::Quaternion>(result__)
        }
    }
    pub fn SetOrientation(
        &self,
        value: ::windows::Foundation::Numerics::Quaternion,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetOrientation)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn RotationAngle(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RotationAngle)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<f32>(result__)
        }
    }
    pub fn SetRotationAngle(&self, value: f32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetRotationAngle)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn RotationAngleInDegrees(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RotationAngleInDegrees)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<f32>(result__)
        }
    }
    pub fn SetRotationAngleInDegrees(&self, value: f32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetRotationAngleInDegrees)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn RotationAxis(
        &self,
    ) -> ::windows::core::Result<::windows::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RotationAxis)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::Numerics::Vector3>(result__)
        }
    }
    pub fn SetRotationAxis(
        &self,
        value: ::windows::Foundation::Numerics::Vector3,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetRotationAxis)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn Scale(&self) -> ::windows::core::Result<::windows::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Scale)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::Numerics::Vector3>(result__)
        }
    }
    pub fn SetScale(
        &self,
        value: ::windows::Foundation::Numerics::Vector3,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetScale)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn Translation(&self) -> ::windows::core::Result<::windows::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Translation)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::Numerics::Vector3>(result__)
        }
    }
    pub fn SetTranslation(
        &self,
        value: ::windows::Foundation::Numerics::Vector3,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetTranslation)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
}
impl ::core::clone::Clone for SceneModelTransform {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SceneModelTransform {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SceneModelTransform {}
impl ::core::fmt::Debug for SceneModelTransform {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SceneModelTransform").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SceneModelTransform {
    const SIGNATURE : ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice ( b"rc(Microsoft.UI.Composition.Scenes.SceneModelTransform;{3f05555f-0f67-576e-9d8a-93c1f250c29f})" ) ;
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for SceneModelTransform {
    type Vtable = ISceneModelTransform_Vtbl;
}
unsafe impl ::windows::core::Interface for SceneModelTransform {
    const IID: ::windows::core::GUID = <ISceneModelTransform as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for SceneModelTransform {
    const NAME: &'static str = "Microsoft.UI.Composition.Scenes.SceneModelTransform";
}
::windows::core::interface_hierarchy!(
    SceneModelTransform,
    ::windows::core::IUnknown,
    ::windows::core::IInspectable
);
impl ::core::convert::TryFrom<SceneModelTransform> for super::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: SceneModelTransform) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SceneModelTransform> for super::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: &SceneModelTransform) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&SceneModelTransform>
    for ::windows::core::InParam<'a, super::IAnimationObject>
{
    type Error = ::windows::core::Error;
    fn try_from(value: &SceneModelTransform) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<SceneModelTransform> for ::windows::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: SceneModelTransform) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SceneModelTransform> for ::windows::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &SceneModelTransform) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&SceneModelTransform>
    for ::windows::core::InParam<'a, ::windows::Foundation::IClosable>
{
    type Error = ::windows::core::Error;
    fn try_from(value: &SceneModelTransform) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::From<SceneModelTransform> for super::CompositionTransform {
    fn from(value: SceneModelTransform) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&SceneModelTransform> for super::CompositionTransform {
    fn from(value: &SceneModelTransform) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::core::convert::From<&SceneModelTransform>
    for ::windows::core::InParam<'a, super::CompositionTransform>
{
    fn from(value: &SceneModelTransform) -> Self {
        ::windows::core::InParam::owned(value.into())
    }
}
impl ::core::convert::From<SceneModelTransform> for super::CompositionObject {
    fn from(value: SceneModelTransform) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&SceneModelTransform> for super::CompositionObject {
    fn from(value: &SceneModelTransform) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::core::convert::From<&SceneModelTransform>
    for ::windows::core::InParam<'a, super::CompositionObject>
{
    fn from(value: &SceneModelTransform) -> Self {
        ::windows::core::InParam::owned(value.into())
    }
}
unsafe impl ::core::marker::Send for SceneModelTransform {}
unsafe impl ::core::marker::Sync for SceneModelTransform {}
#[doc = "*Required features: `\"UI_Composition_Scenes\"`*"]
#[repr(transparent)]
pub struct SceneNode(::windows::core::IUnknown);
impl SceneNode {
    pub fn PopulatePropertyInfo(
        &self,
        propertyname: &::windows::core::HSTRING,
        propertyinfo: &super::AnimationPropertyInfo,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).PopulatePropertyInfo)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(propertyname),
                ::core::mem::transmute_copy(propertyinfo),
            )
            .ok()
        }
    }
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<::windows::Foundation::IClosable>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).Close)(::windows::core::Vtable::as_raw(this))
                .ok()
        }
    }
    pub fn Compositor(&self) -> ::windows::core::Result<super::Compositor> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Compositor)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::Compositor>(result__)
        }
    }
    pub fn Properties(&self) -> ::windows::core::Result<super::CompositionPropertySet> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Properties)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::CompositionPropertySet>(result__)
        }
    }
    pub fn StartAnimation<'a, P0>(
        &self,
        propertyname: &::windows::core::HSTRING,
        animation: P0,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::CompositionAnimation>>,
    {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).StartAnimation)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(propertyname),
                animation.into().abi(),
            )
            .ok()
        }
    }
    pub fn StopAnimation(
        &self,
        propertyname: &::windows::core::HSTRING,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).StopAnimation)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(propertyname),
            )
            .ok()
        }
    }
    pub fn Comment(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Comment)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetComment(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetComment)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn ImplicitAnimations(
        &self,
    ) -> ::windows::core::Result<super::ImplicitAnimationCollection> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ImplicitAnimations)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::ImplicitAnimationCollection>(result__)
        }
    }
    pub fn SetImplicitAnimations(
        &self,
        value: &super::ImplicitAnimationCollection,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetImplicitAnimations)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn StartAnimationGroup<'a, P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<'a, super::ICompositionAnimationBase>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).StartAnimationGroup)(
                ::windows::core::Vtable::as_raw(this),
                value.try_into().map_err(|e| e.into())?.abi(),
            )
            .ok()
        }
    }
    pub fn StopAnimationGroup<'a, P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<'a, super::ICompositionAnimationBase>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).StopAnimationGroup)(
                ::windows::core::Vtable::as_raw(this),
                value.try_into().map_err(|e| e.into())?.abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Dispatching\"`*"]
    #[cfg(feature = "UI_Dispatching")]
    pub fn DispatcherQueue(
        &self,
    ) -> ::windows::core::Result<super::super::Dispatching::DispatcherQueue> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DispatcherQueue)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::super::Dispatching::DispatcherQueue>(result__)
        }
    }
    pub fn TryGetAnimationController(
        &self,
        propertyname: &::windows::core::HSTRING,
    ) -> ::windows::core::Result<super::AnimationController> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TryGetAnimationController)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(propertyname),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::AnimationController>(result__)
        }
    }
    pub fn StartAnimationWithController<'a, P0>(
        &self,
        propertyname: &::windows::core::HSTRING,
        animation: P0,
        animationcontroller: &super::AnimationController,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::CompositionAnimation>>,
    {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject5>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).StartAnimationWithController)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(propertyname),
                animation.into().abi(),
                ::core::mem::transmute_copy(animationcontroller),
            )
            .ok()
        }
    }
    pub fn Children(&self) -> ::windows::core::Result<SceneNodeCollection> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Children)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<SceneNodeCollection>(result__)
        }
    }
    pub fn Components(&self) -> ::windows::core::Result<SceneComponentCollection> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Components)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<SceneComponentCollection>(result__)
        }
    }
    pub fn Parent(&self) -> ::windows::core::Result<SceneNode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Parent)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<SceneNode>(result__)
        }
    }
    pub fn Transform(&self) -> ::windows::core::Result<SceneModelTransform> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Transform)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<SceneModelTransform>(result__)
        }
    }
    pub fn FindFirstComponentOfType(
        &self,
        value: SceneComponentType,
    ) -> ::windows::core::Result<SceneComponent> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FindFirstComponentOfType)(
                ::windows::core::Vtable::as_raw(this),
                value,
                result__.as_mut_ptr(),
            )
            .from_abi::<SceneComponent>(result__)
        }
    }
    pub fn Create(compositor: &super::Compositor) -> ::windows::core::Result<SceneNode> {
        Self::ISceneNodeStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Create)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(compositor),
                result__.as_mut_ptr(),
            )
            .from_abi::<SceneNode>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ISceneNodeStatics<R, F: FnOnce(&ISceneNodeStatics) -> ::windows::core::Result<R>>(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<SceneNode, ISceneNodeStatics> =
            ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for SceneNode {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SceneNode {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SceneNode {}
impl ::core::fmt::Debug for SceneNode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SceneNode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SceneNode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Composition.Scenes.SceneNode;{a1bce140-79c2-59e6-9b68-63b1bab0e2a6})",
    );
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for SceneNode {
    type Vtable = ISceneNode_Vtbl;
}
unsafe impl ::windows::core::Interface for SceneNode {
    const IID: ::windows::core::GUID = <ISceneNode as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for SceneNode {
    const NAME: &'static str = "Microsoft.UI.Composition.Scenes.SceneNode";
}
::windows::core::interface_hierarchy!(
    SceneNode,
    ::windows::core::IUnknown,
    ::windows::core::IInspectable
);
impl ::core::convert::TryFrom<SceneNode> for super::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: SceneNode) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SceneNode> for super::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: &SceneNode) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&SceneNode>
    for ::windows::core::InParam<'a, super::IAnimationObject>
{
    type Error = ::windows::core::Error;
    fn try_from(value: &SceneNode) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<SceneNode> for ::windows::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: SceneNode) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SceneNode> for ::windows::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &SceneNode) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&SceneNode>
    for ::windows::core::InParam<'a, ::windows::Foundation::IClosable>
{
    type Error = ::windows::core::Error;
    fn try_from(value: &SceneNode) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::From<SceneNode> for SceneObject {
    fn from(value: SceneNode) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&SceneNode> for SceneObject {
    fn from(value: &SceneNode) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::core::convert::From<&SceneNode> for ::windows::core::InParam<'a, SceneObject> {
    fn from(value: &SceneNode) -> Self {
        ::windows::core::InParam::owned(value.into())
    }
}
impl ::core::convert::From<SceneNode> for super::CompositionObject {
    fn from(value: SceneNode) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&SceneNode> for super::CompositionObject {
    fn from(value: &SceneNode) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::core::convert::From<&SceneNode>
    for ::windows::core::InParam<'a, super::CompositionObject>
{
    fn from(value: &SceneNode) -> Self {
        ::windows::core::InParam::owned(value.into())
    }
}
unsafe impl ::core::marker::Send for SceneNode {}
unsafe impl ::core::marker::Sync for SceneNode {}
#[doc = "*Required features: `\"UI_Composition_Scenes\"`*"]
#[repr(transparent)]
pub struct SceneNodeCollection(::windows::core::IUnknown);
impl SceneNodeCollection {
    pub fn PopulatePropertyInfo(
        &self,
        propertyname: &::windows::core::HSTRING,
        propertyinfo: &super::AnimationPropertyInfo,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).PopulatePropertyInfo)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(propertyname),
                ::core::mem::transmute_copy(propertyinfo),
            )
            .ok()
        }
    }
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<::windows::Foundation::IClosable>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).Close)(::windows::core::Vtable::as_raw(this))
                .ok()
        }
    }
    pub fn Compositor(&self) -> ::windows::core::Result<super::Compositor> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Compositor)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::Compositor>(result__)
        }
    }
    pub fn Properties(&self) -> ::windows::core::Result<super::CompositionPropertySet> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Properties)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::CompositionPropertySet>(result__)
        }
    }
    pub fn StartAnimation<'a, P0>(
        &self,
        propertyname: &::windows::core::HSTRING,
        animation: P0,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::CompositionAnimation>>,
    {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).StartAnimation)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(propertyname),
                animation.into().abi(),
            )
            .ok()
        }
    }
    pub fn StopAnimation(
        &self,
        propertyname: &::windows::core::HSTRING,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).StopAnimation)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(propertyname),
            )
            .ok()
        }
    }
    pub fn Comment(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Comment)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetComment(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetComment)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn ImplicitAnimations(
        &self,
    ) -> ::windows::core::Result<super::ImplicitAnimationCollection> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ImplicitAnimations)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::ImplicitAnimationCollection>(result__)
        }
    }
    pub fn SetImplicitAnimations(
        &self,
        value: &super::ImplicitAnimationCollection,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetImplicitAnimations)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn StartAnimationGroup<'a, P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<'a, super::ICompositionAnimationBase>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).StartAnimationGroup)(
                ::windows::core::Vtable::as_raw(this),
                value.try_into().map_err(|e| e.into())?.abi(),
            )
            .ok()
        }
    }
    pub fn StopAnimationGroup<'a, P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<'a, super::ICompositionAnimationBase>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).StopAnimationGroup)(
                ::windows::core::Vtable::as_raw(this),
                value.try_into().map_err(|e| e.into())?.abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Dispatching\"`*"]
    #[cfg(feature = "UI_Dispatching")]
    pub fn DispatcherQueue(
        &self,
    ) -> ::windows::core::Result<super::super::Dispatching::DispatcherQueue> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DispatcherQueue)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::super::Dispatching::DispatcherQueue>(result__)
        }
    }
    pub fn TryGetAnimationController(
        &self,
        propertyname: &::windows::core::HSTRING,
    ) -> ::windows::core::Result<super::AnimationController> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TryGetAnimationController)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(propertyname),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::AnimationController>(result__)
        }
    }
    pub fn StartAnimationWithController<'a, P0>(
        &self,
        propertyname: &::windows::core::HSTRING,
        animation: P0,
        animationcontroller: &super::AnimationController,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::CompositionAnimation>>,
    {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject5>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).StartAnimationWithController)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(propertyname),
                animation.into().abi(),
                ::core::mem::transmute_copy(animationcontroller),
            )
            .ok()
        }
    }
    pub fn First(
        &self,
    ) -> ::windows::core::Result<::windows::Foundation::Collections::IIterator<SceneNode>> {
        let this = &::windows::core::Interface::cast::<
            ::windows::Foundation::Collections::IIterable<SceneNode>,
        >(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).First)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::Collections::IIterator<SceneNode>>(result__)
        }
    }
    pub fn GetAt(&self, index: u32) -> ::windows::core::Result<SceneNode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetAt)(
                ::windows::core::Vtable::as_raw(this),
                index,
                result__.as_mut_ptr(),
            )
            .from_abi::<SceneNode>(result__)
        }
    }
    pub fn Size(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Size)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<u32>(result__)
        }
    }
    pub fn GetView(
        &self,
    ) -> ::windows::core::Result<::windows::Foundation::Collections::IVectorView<SceneNode>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetView)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::Collections::IVectorView<SceneNode>>(result__)
        }
    }
    pub fn IndexOf(&self, value: &SceneNode, index: &mut u32) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IndexOf)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(value),
                index,
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetAt(&self, index: u32, value: &SceneNode) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetAt)(
                ::windows::core::Vtable::as_raw(this),
                index,
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn InsertAt(&self, index: u32, value: &SceneNode) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).InsertAt)(
                ::windows::core::Vtable::as_raw(this),
                index,
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn RemoveAt(&self, index: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).RemoveAt)(
                ::windows::core::Vtable::as_raw(this),
                index,
            )
            .ok()
        }
    }
    pub fn Append(&self, value: &SceneNode) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).Append)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn RemoveAtEnd(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).RemoveAtEnd)(::windows::core::Vtable::as_raw(
                this,
            ))
            .ok()
        }
    }
    pub fn Clear(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).Clear)(::windows::core::Vtable::as_raw(this))
                .ok()
        }
    }
    pub fn GetMany(
        &self,
        startindex: u32,
        items: &mut [::core::option::Option<SceneNode>],
    ) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetMany)(
                ::windows::core::Vtable::as_raw(this),
                startindex,
                items.len() as u32,
                ::core::mem::transmute_copy(&items),
                result__.as_mut_ptr(),
            )
            .from_abi::<u32>(result__)
        }
    }
    pub fn ReplaceAll(
        &self,
        items: &[::core::option::Option<SceneNode>],
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).ReplaceAll)(
                ::windows::core::Vtable::as_raw(this),
                items.len() as u32,
                ::core::mem::transmute(items.as_ptr()),
            )
            .ok()
        }
    }
}
impl ::core::clone::Clone for SceneNodeCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SceneNodeCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SceneNodeCollection {}
impl ::core::fmt::Debug for SceneNodeCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SceneNodeCollection").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SceneNodeCollection {
    const SIGNATURE : ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice ( b"rc(Microsoft.UI.Composition.Scenes.SceneNodeCollection;pinterface({913337e9-11a1-4345-a3a2-4e7f956e222d};rc(Microsoft.UI.Composition.Scenes.SceneNode;{a1bce140-79c2-59e6-9b68-63b1bab0e2a6})))" ) ;
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for SceneNodeCollection {
    type Vtable = ::windows::Foundation::Collections::IVector_Vtbl<SceneNode>;
}
unsafe impl ::windows::core::Interface for SceneNodeCollection {
    const IID: ::windows::core::GUID =
        <::windows::Foundation::Collections::IVector<SceneNode> as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for SceneNodeCollection {
    const NAME: &'static str = "Microsoft.UI.Composition.Scenes.SceneNodeCollection";
}
impl ::core::iter::IntoIterator for SceneNodeCollection {
    type Item = SceneNode;
    type IntoIter = ::windows::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
impl ::core::iter::IntoIterator for &SceneNodeCollection {
    type Item = SceneNode;
    type IntoIter = ::windows::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::windows::Foundation::Collections::VectorIterator::new(
            ::core::convert::TryInto::try_into(self).ok(),
        )
    }
}
::windows::core::interface_hierarchy!(
    SceneNodeCollection,
    ::windows::core::IUnknown,
    ::windows::core::IInspectable
);
impl ::core::convert::TryFrom<SceneNodeCollection> for super::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: SceneNodeCollection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SceneNodeCollection> for super::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: &SceneNodeCollection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&SceneNodeCollection>
    for ::windows::core::InParam<'a, super::IAnimationObject>
{
    type Error = ::windows::core::Error;
    fn try_from(value: &SceneNodeCollection) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<SceneNodeCollection> for ::windows::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: SceneNodeCollection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SceneNodeCollection> for ::windows::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &SceneNodeCollection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&SceneNodeCollection>
    for ::windows::core::InParam<'a, ::windows::Foundation::IClosable>
{
    type Error = ::windows::core::Error;
    fn try_from(value: &SceneNodeCollection) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<SceneNodeCollection>
    for ::windows::Foundation::Collections::IIterable<SceneNode>
{
    type Error = ::windows::core::Error;
    fn try_from(value: SceneNodeCollection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SceneNodeCollection>
    for ::windows::Foundation::Collections::IIterable<SceneNode>
{
    type Error = ::windows::core::Error;
    fn try_from(value: &SceneNodeCollection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&SceneNodeCollection>
    for ::windows::core::InParam<'a, ::windows::Foundation::Collections::IIterable<SceneNode>>
{
    type Error = ::windows::core::Error;
    fn try_from(value: &SceneNodeCollection) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<SceneNodeCollection>
    for ::windows::Foundation::Collections::IVector<SceneNode>
{
    type Error = ::windows::core::Error;
    fn try_from(value: SceneNodeCollection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SceneNodeCollection>
    for ::windows::Foundation::Collections::IVector<SceneNode>
{
    type Error = ::windows::core::Error;
    fn try_from(value: &SceneNodeCollection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&SceneNodeCollection>
    for ::windows::core::InParam<'a, ::windows::Foundation::Collections::IVector<SceneNode>>
{
    type Error = ::windows::core::Error;
    fn try_from(value: &SceneNodeCollection) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::From<SceneNodeCollection> for SceneObject {
    fn from(value: SceneNodeCollection) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&SceneNodeCollection> for SceneObject {
    fn from(value: &SceneNodeCollection) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::core::convert::From<&SceneNodeCollection> for ::windows::core::InParam<'a, SceneObject> {
    fn from(value: &SceneNodeCollection) -> Self {
        ::windows::core::InParam::owned(value.into())
    }
}
impl ::core::convert::From<SceneNodeCollection> for super::CompositionObject {
    fn from(value: SceneNodeCollection) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&SceneNodeCollection> for super::CompositionObject {
    fn from(value: &SceneNodeCollection) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::core::convert::From<&SceneNodeCollection>
    for ::windows::core::InParam<'a, super::CompositionObject>
{
    fn from(value: &SceneNodeCollection) -> Self {
        ::windows::core::InParam::owned(value.into())
    }
}
unsafe impl ::core::marker::Send for SceneNodeCollection {}
unsafe impl ::core::marker::Sync for SceneNodeCollection {}
#[doc = "*Required features: `\"UI_Composition_Scenes\"`*"]
#[repr(transparent)]
pub struct SceneObject(::windows::core::IUnknown);
impl SceneObject {
    pub fn PopulatePropertyInfo(
        &self,
        propertyname: &::windows::core::HSTRING,
        propertyinfo: &super::AnimationPropertyInfo,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).PopulatePropertyInfo)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(propertyname),
                ::core::mem::transmute_copy(propertyinfo),
            )
            .ok()
        }
    }
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<::windows::Foundation::IClosable>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).Close)(::windows::core::Vtable::as_raw(this))
                .ok()
        }
    }
    pub fn Compositor(&self) -> ::windows::core::Result<super::Compositor> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Compositor)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::Compositor>(result__)
        }
    }
    pub fn Properties(&self) -> ::windows::core::Result<super::CompositionPropertySet> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Properties)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::CompositionPropertySet>(result__)
        }
    }
    pub fn StartAnimation<'a, P0>(
        &self,
        propertyname: &::windows::core::HSTRING,
        animation: P0,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::CompositionAnimation>>,
    {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).StartAnimation)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(propertyname),
                animation.into().abi(),
            )
            .ok()
        }
    }
    pub fn StopAnimation(
        &self,
        propertyname: &::windows::core::HSTRING,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).StopAnimation)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(propertyname),
            )
            .ok()
        }
    }
    pub fn Comment(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Comment)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetComment(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetComment)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn ImplicitAnimations(
        &self,
    ) -> ::windows::core::Result<super::ImplicitAnimationCollection> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ImplicitAnimations)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::ImplicitAnimationCollection>(result__)
        }
    }
    pub fn SetImplicitAnimations(
        &self,
        value: &super::ImplicitAnimationCollection,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetImplicitAnimations)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn StartAnimationGroup<'a, P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<'a, super::ICompositionAnimationBase>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).StartAnimationGroup)(
                ::windows::core::Vtable::as_raw(this),
                value.try_into().map_err(|e| e.into())?.abi(),
            )
            .ok()
        }
    }
    pub fn StopAnimationGroup<'a, P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<'a, super::ICompositionAnimationBase>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).StopAnimationGroup)(
                ::windows::core::Vtable::as_raw(this),
                value.try_into().map_err(|e| e.into())?.abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Dispatching\"`*"]
    #[cfg(feature = "UI_Dispatching")]
    pub fn DispatcherQueue(
        &self,
    ) -> ::windows::core::Result<super::super::Dispatching::DispatcherQueue> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DispatcherQueue)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::super::Dispatching::DispatcherQueue>(result__)
        }
    }
    pub fn TryGetAnimationController(
        &self,
        propertyname: &::windows::core::HSTRING,
    ) -> ::windows::core::Result<super::AnimationController> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TryGetAnimationController)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(propertyname),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::AnimationController>(result__)
        }
    }
    pub fn StartAnimationWithController<'a, P0>(
        &self,
        propertyname: &::windows::core::HSTRING,
        animation: P0,
        animationcontroller: &super::AnimationController,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::CompositionAnimation>>,
    {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject5>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).StartAnimationWithController)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(propertyname),
                animation.into().abi(),
                ::core::mem::transmute_copy(animationcontroller),
            )
            .ok()
        }
    }
}
impl ::core::clone::Clone for SceneObject {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SceneObject {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SceneObject {}
impl ::core::fmt::Debug for SceneObject {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SceneObject").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SceneObject {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Composition.Scenes.SceneObject;{4333e514-4fc7-521e-8bca-11c51fbcaf1e})",
    );
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for SceneObject {
    type Vtable = ISceneObject_Vtbl;
}
unsafe impl ::windows::core::Interface for SceneObject {
    const IID: ::windows::core::GUID = <ISceneObject as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for SceneObject {
    const NAME: &'static str = "Microsoft.UI.Composition.Scenes.SceneObject";
}
::windows::core::interface_hierarchy!(
    SceneObject,
    ::windows::core::IUnknown,
    ::windows::core::IInspectable
);
impl ::core::convert::TryFrom<SceneObject> for super::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: SceneObject) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SceneObject> for super::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: &SceneObject) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&SceneObject>
    for ::windows::core::InParam<'a, super::IAnimationObject>
{
    type Error = ::windows::core::Error;
    fn try_from(value: &SceneObject) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<SceneObject> for ::windows::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: SceneObject) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SceneObject> for ::windows::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &SceneObject) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&SceneObject>
    for ::windows::core::InParam<'a, ::windows::Foundation::IClosable>
{
    type Error = ::windows::core::Error;
    fn try_from(value: &SceneObject) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::From<SceneObject> for super::CompositionObject {
    fn from(value: SceneObject) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&SceneObject> for super::CompositionObject {
    fn from(value: &SceneObject) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::core::convert::From<&SceneObject>
    for ::windows::core::InParam<'a, super::CompositionObject>
{
    fn from(value: &SceneObject) -> Self {
        ::windows::core::InParam::owned(value.into())
    }
}
unsafe impl ::core::marker::Send for SceneObject {}
unsafe impl ::core::marker::Sync for SceneObject {}
#[doc = "*Required features: `\"UI_Composition_Scenes\"`*"]
#[repr(transparent)]
pub struct ScenePbrMaterial(::windows::core::IUnknown);
impl ScenePbrMaterial {
    pub fn PopulatePropertyInfo(
        &self,
        propertyname: &::windows::core::HSTRING,
        propertyinfo: &super::AnimationPropertyInfo,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).PopulatePropertyInfo)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(propertyname),
                ::core::mem::transmute_copy(propertyinfo),
            )
            .ok()
        }
    }
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<::windows::Foundation::IClosable>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).Close)(::windows::core::Vtable::as_raw(this))
                .ok()
        }
    }
    pub fn Compositor(&self) -> ::windows::core::Result<super::Compositor> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Compositor)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::Compositor>(result__)
        }
    }
    pub fn Properties(&self) -> ::windows::core::Result<super::CompositionPropertySet> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Properties)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::CompositionPropertySet>(result__)
        }
    }
    pub fn StartAnimation<'a, P0>(
        &self,
        propertyname: &::windows::core::HSTRING,
        animation: P0,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::CompositionAnimation>>,
    {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).StartAnimation)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(propertyname),
                animation.into().abi(),
            )
            .ok()
        }
    }
    pub fn StopAnimation(
        &self,
        propertyname: &::windows::core::HSTRING,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).StopAnimation)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(propertyname),
            )
            .ok()
        }
    }
    pub fn Comment(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Comment)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetComment(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetComment)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn ImplicitAnimations(
        &self,
    ) -> ::windows::core::Result<super::ImplicitAnimationCollection> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ImplicitAnimations)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::ImplicitAnimationCollection>(result__)
        }
    }
    pub fn SetImplicitAnimations(
        &self,
        value: &super::ImplicitAnimationCollection,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetImplicitAnimations)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn StartAnimationGroup<'a, P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<'a, super::ICompositionAnimationBase>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).StartAnimationGroup)(
                ::windows::core::Vtable::as_raw(this),
                value.try_into().map_err(|e| e.into())?.abi(),
            )
            .ok()
        }
    }
    pub fn StopAnimationGroup<'a, P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<'a, super::ICompositionAnimationBase>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).StopAnimationGroup)(
                ::windows::core::Vtable::as_raw(this),
                value.try_into().map_err(|e| e.into())?.abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Dispatching\"`*"]
    #[cfg(feature = "UI_Dispatching")]
    pub fn DispatcherQueue(
        &self,
    ) -> ::windows::core::Result<super::super::Dispatching::DispatcherQueue> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DispatcherQueue)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::super::Dispatching::DispatcherQueue>(result__)
        }
    }
    pub fn TryGetAnimationController(
        &self,
        propertyname: &::windows::core::HSTRING,
    ) -> ::windows::core::Result<super::AnimationController> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TryGetAnimationController)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(propertyname),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::AnimationController>(result__)
        }
    }
    pub fn StartAnimationWithController<'a, P0>(
        &self,
        propertyname: &::windows::core::HSTRING,
        animation: P0,
        animationcontroller: &super::AnimationController,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::CompositionAnimation>>,
    {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject5>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).StartAnimationWithController)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(propertyname),
                animation.into().abi(),
                ::core::mem::transmute_copy(animationcontroller),
            )
            .ok()
        }
    }
    pub fn AlphaCutoff(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AlphaCutoff)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<f32>(result__)
        }
    }
    pub fn SetAlphaCutoff(&self, value: f32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetAlphaCutoff)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn AlphaMode(&self) -> ::windows::core::Result<SceneAlphaMode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AlphaMode)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<SceneAlphaMode>(result__)
        }
    }
    pub fn SetAlphaMode(&self, value: SceneAlphaMode) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetAlphaMode)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn EmissiveInput(&self) -> ::windows::core::Result<SceneMaterialInput> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).EmissiveInput)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<SceneMaterialInput>(result__)
        }
    }
    pub fn SetEmissiveInput<'a, P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, SceneMaterialInput>>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetEmissiveInput)(
                ::windows::core::Vtable::as_raw(this),
                value.into().abi(),
            )
            .ok()
        }
    }
    pub fn EmissiveFactor(
        &self,
    ) -> ::windows::core::Result<::windows::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).EmissiveFactor)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::Numerics::Vector3>(result__)
        }
    }
    pub fn SetEmissiveFactor(
        &self,
        value: ::windows::Foundation::Numerics::Vector3,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetEmissiveFactor)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsDoubleSided(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsDoubleSided)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetIsDoubleSided(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetIsDoubleSided)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn NormalInput(&self) -> ::windows::core::Result<SceneMaterialInput> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NormalInput)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<SceneMaterialInput>(result__)
        }
    }
    pub fn SetNormalInput<'a, P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, SceneMaterialInput>>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetNormalInput)(
                ::windows::core::Vtable::as_raw(this),
                value.into().abi(),
            )
            .ok()
        }
    }
    pub fn NormalScale(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NormalScale)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<f32>(result__)
        }
    }
    pub fn SetNormalScale(&self, value: f32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetNormalScale)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn OcclusionInput(&self) -> ::windows::core::Result<SceneMaterialInput> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).OcclusionInput)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<SceneMaterialInput>(result__)
        }
    }
    pub fn SetOcclusionInput<'a, P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, SceneMaterialInput>>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetOcclusionInput)(
                ::windows::core::Vtable::as_raw(this),
                value.into().abi(),
            )
            .ok()
        }
    }
    pub fn OcclusionStrength(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).OcclusionStrength)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<f32>(result__)
        }
    }
    pub fn SetOcclusionStrength(&self, value: f32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetOcclusionStrength)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
}
impl ::core::clone::Clone for ScenePbrMaterial {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ScenePbrMaterial {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ScenePbrMaterial {}
impl ::core::fmt::Debug for ScenePbrMaterial {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ScenePbrMaterial").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ScenePbrMaterial {
    const SIGNATURE : ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice ( b"rc(Microsoft.UI.Composition.Scenes.ScenePbrMaterial;{295d0725-56fe-5954-8057-3f4ca7515b36})" ) ;
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ScenePbrMaterial {
    type Vtable = IScenePbrMaterial_Vtbl;
}
unsafe impl ::windows::core::Interface for ScenePbrMaterial {
    const IID: ::windows::core::GUID = <IScenePbrMaterial as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ScenePbrMaterial {
    const NAME: &'static str = "Microsoft.UI.Composition.Scenes.ScenePbrMaterial";
}
::windows::core::interface_hierarchy!(
    ScenePbrMaterial,
    ::windows::core::IUnknown,
    ::windows::core::IInspectable
);
impl ::core::convert::TryFrom<ScenePbrMaterial> for super::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: ScenePbrMaterial) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ScenePbrMaterial> for super::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: &ScenePbrMaterial) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&ScenePbrMaterial>
    for ::windows::core::InParam<'a, super::IAnimationObject>
{
    type Error = ::windows::core::Error;
    fn try_from(value: &ScenePbrMaterial) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<ScenePbrMaterial> for ::windows::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: ScenePbrMaterial) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ScenePbrMaterial> for ::windows::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &ScenePbrMaterial) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&ScenePbrMaterial>
    for ::windows::core::InParam<'a, ::windows::Foundation::IClosable>
{
    type Error = ::windows::core::Error;
    fn try_from(value: &ScenePbrMaterial) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::From<ScenePbrMaterial> for SceneMaterial {
    fn from(value: ScenePbrMaterial) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&ScenePbrMaterial> for SceneMaterial {
    fn from(value: &ScenePbrMaterial) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::core::convert::From<&ScenePbrMaterial> for ::windows::core::InParam<'a, SceneMaterial> {
    fn from(value: &ScenePbrMaterial) -> Self {
        ::windows::core::InParam::owned(value.into())
    }
}
impl ::core::convert::From<ScenePbrMaterial> for SceneObject {
    fn from(value: ScenePbrMaterial) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&ScenePbrMaterial> for SceneObject {
    fn from(value: &ScenePbrMaterial) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::core::convert::From<&ScenePbrMaterial> for ::windows::core::InParam<'a, SceneObject> {
    fn from(value: &ScenePbrMaterial) -> Self {
        ::windows::core::InParam::owned(value.into())
    }
}
impl ::core::convert::From<ScenePbrMaterial> for super::CompositionObject {
    fn from(value: ScenePbrMaterial) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&ScenePbrMaterial> for super::CompositionObject {
    fn from(value: &ScenePbrMaterial) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::core::convert::From<&ScenePbrMaterial>
    for ::windows::core::InParam<'a, super::CompositionObject>
{
    fn from(value: &ScenePbrMaterial) -> Self {
        ::windows::core::InParam::owned(value.into())
    }
}
unsafe impl ::core::marker::Send for ScenePbrMaterial {}
unsafe impl ::core::marker::Sync for ScenePbrMaterial {}
#[doc = "*Required features: `\"UI_Composition_Scenes\"`*"]
#[repr(transparent)]
pub struct SceneRendererComponent(::windows::core::IUnknown);
impl SceneRendererComponent {
    pub fn PopulatePropertyInfo(
        &self,
        propertyname: &::windows::core::HSTRING,
        propertyinfo: &super::AnimationPropertyInfo,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).PopulatePropertyInfo)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(propertyname),
                ::core::mem::transmute_copy(propertyinfo),
            )
            .ok()
        }
    }
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<::windows::Foundation::IClosable>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).Close)(::windows::core::Vtable::as_raw(this))
                .ok()
        }
    }
    pub fn Compositor(&self) -> ::windows::core::Result<super::Compositor> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Compositor)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::Compositor>(result__)
        }
    }
    pub fn Properties(&self) -> ::windows::core::Result<super::CompositionPropertySet> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Properties)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::CompositionPropertySet>(result__)
        }
    }
    pub fn StartAnimation<'a, P0>(
        &self,
        propertyname: &::windows::core::HSTRING,
        animation: P0,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::CompositionAnimation>>,
    {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).StartAnimation)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(propertyname),
                animation.into().abi(),
            )
            .ok()
        }
    }
    pub fn StopAnimation(
        &self,
        propertyname: &::windows::core::HSTRING,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).StopAnimation)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(propertyname),
            )
            .ok()
        }
    }
    pub fn Comment(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Comment)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetComment(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetComment)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn ImplicitAnimations(
        &self,
    ) -> ::windows::core::Result<super::ImplicitAnimationCollection> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ImplicitAnimations)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::ImplicitAnimationCollection>(result__)
        }
    }
    pub fn SetImplicitAnimations(
        &self,
        value: &super::ImplicitAnimationCollection,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetImplicitAnimations)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn StartAnimationGroup<'a, P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<'a, super::ICompositionAnimationBase>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).StartAnimationGroup)(
                ::windows::core::Vtable::as_raw(this),
                value.try_into().map_err(|e| e.into())?.abi(),
            )
            .ok()
        }
    }
    pub fn StopAnimationGroup<'a, P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<'a, super::ICompositionAnimationBase>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).StopAnimationGroup)(
                ::windows::core::Vtable::as_raw(this),
                value.try_into().map_err(|e| e.into())?.abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Dispatching\"`*"]
    #[cfg(feature = "UI_Dispatching")]
    pub fn DispatcherQueue(
        &self,
    ) -> ::windows::core::Result<super::super::Dispatching::DispatcherQueue> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DispatcherQueue)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::super::Dispatching::DispatcherQueue>(result__)
        }
    }
    pub fn TryGetAnimationController(
        &self,
        propertyname: &::windows::core::HSTRING,
    ) -> ::windows::core::Result<super::AnimationController> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TryGetAnimationController)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(propertyname),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::AnimationController>(result__)
        }
    }
    pub fn StartAnimationWithController<'a, P0>(
        &self,
        propertyname: &::windows::core::HSTRING,
        animation: P0,
        animationcontroller: &super::AnimationController,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::CompositionAnimation>>,
    {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject5>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).StartAnimationWithController)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(propertyname),
                animation.into().abi(),
                ::core::mem::transmute_copy(animationcontroller),
            )
            .ok()
        }
    }
    pub fn ComponentType(&self) -> ::windows::core::Result<SceneComponentType> {
        let this = &::windows::core::Interface::cast::<ISceneComponent>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ComponentType)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<SceneComponentType>(result__)
        }
    }
}
impl ::core::clone::Clone for SceneRendererComponent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SceneRendererComponent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SceneRendererComponent {}
impl ::core::fmt::Debug for SceneRendererComponent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SceneRendererComponent").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SceneRendererComponent {
    const SIGNATURE : ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice ( b"rc(Microsoft.UI.Composition.Scenes.SceneRendererComponent;{6bab8030-89c1-5dbc-a48e-1805ddf9cdd1})" ) ;
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for SceneRendererComponent {
    type Vtable = ISceneRendererComponent_Vtbl;
}
unsafe impl ::windows::core::Interface for SceneRendererComponent {
    const IID: ::windows::core::GUID = <ISceneRendererComponent as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for SceneRendererComponent {
    const NAME: &'static str = "Microsoft.UI.Composition.Scenes.SceneRendererComponent";
}
::windows::core::interface_hierarchy!(
    SceneRendererComponent,
    ::windows::core::IUnknown,
    ::windows::core::IInspectable
);
impl ::core::convert::TryFrom<SceneRendererComponent> for super::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: SceneRendererComponent) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SceneRendererComponent> for super::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: &SceneRendererComponent) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&SceneRendererComponent>
    for ::windows::core::InParam<'a, super::IAnimationObject>
{
    type Error = ::windows::core::Error;
    fn try_from(value: &SceneRendererComponent) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<SceneRendererComponent> for ::windows::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: SceneRendererComponent) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SceneRendererComponent> for ::windows::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &SceneRendererComponent) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&SceneRendererComponent>
    for ::windows::core::InParam<'a, ::windows::Foundation::IClosable>
{
    type Error = ::windows::core::Error;
    fn try_from(value: &SceneRendererComponent) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::From<SceneRendererComponent> for SceneComponent {
    fn from(value: SceneRendererComponent) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&SceneRendererComponent> for SceneComponent {
    fn from(value: &SceneRendererComponent) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::core::convert::From<&SceneRendererComponent>
    for ::windows::core::InParam<'a, SceneComponent>
{
    fn from(value: &SceneRendererComponent) -> Self {
        ::windows::core::InParam::owned(value.into())
    }
}
impl ::core::convert::From<SceneRendererComponent> for SceneObject {
    fn from(value: SceneRendererComponent) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&SceneRendererComponent> for SceneObject {
    fn from(value: &SceneRendererComponent) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::core::convert::From<&SceneRendererComponent>
    for ::windows::core::InParam<'a, SceneObject>
{
    fn from(value: &SceneRendererComponent) -> Self {
        ::windows::core::InParam::owned(value.into())
    }
}
impl ::core::convert::From<SceneRendererComponent> for super::CompositionObject {
    fn from(value: SceneRendererComponent) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&SceneRendererComponent> for super::CompositionObject {
    fn from(value: &SceneRendererComponent) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::core::convert::From<&SceneRendererComponent>
    for ::windows::core::InParam<'a, super::CompositionObject>
{
    fn from(value: &SceneRendererComponent) -> Self {
        ::windows::core::InParam::owned(value.into())
    }
}
unsafe impl ::core::marker::Send for SceneRendererComponent {}
unsafe impl ::core::marker::Sync for SceneRendererComponent {}
#[doc = "*Required features: `\"UI_Composition_Scenes\"`*"]
#[repr(transparent)]
pub struct SceneSurfaceMaterialInput(::windows::core::IUnknown);
impl SceneSurfaceMaterialInput {
    pub fn PopulatePropertyInfo(
        &self,
        propertyname: &::windows::core::HSTRING,
        propertyinfo: &super::AnimationPropertyInfo,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).PopulatePropertyInfo)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(propertyname),
                ::core::mem::transmute_copy(propertyinfo),
            )
            .ok()
        }
    }
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<::windows::Foundation::IClosable>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).Close)(::windows::core::Vtable::as_raw(this))
                .ok()
        }
    }
    pub fn Compositor(&self) -> ::windows::core::Result<super::Compositor> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Compositor)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::Compositor>(result__)
        }
    }
    pub fn Properties(&self) -> ::windows::core::Result<super::CompositionPropertySet> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Properties)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::CompositionPropertySet>(result__)
        }
    }
    pub fn StartAnimation<'a, P0>(
        &self,
        propertyname: &::windows::core::HSTRING,
        animation: P0,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::CompositionAnimation>>,
    {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).StartAnimation)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(propertyname),
                animation.into().abi(),
            )
            .ok()
        }
    }
    pub fn StopAnimation(
        &self,
        propertyname: &::windows::core::HSTRING,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).StopAnimation)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(propertyname),
            )
            .ok()
        }
    }
    pub fn Comment(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Comment)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetComment(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetComment)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn ImplicitAnimations(
        &self,
    ) -> ::windows::core::Result<super::ImplicitAnimationCollection> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ImplicitAnimations)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::ImplicitAnimationCollection>(result__)
        }
    }
    pub fn SetImplicitAnimations(
        &self,
        value: &super::ImplicitAnimationCollection,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetImplicitAnimations)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn StartAnimationGroup<'a, P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<'a, super::ICompositionAnimationBase>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).StartAnimationGroup)(
                ::windows::core::Vtable::as_raw(this),
                value.try_into().map_err(|e| e.into())?.abi(),
            )
            .ok()
        }
    }
    pub fn StopAnimationGroup<'a, P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<'a, super::ICompositionAnimationBase>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).StopAnimationGroup)(
                ::windows::core::Vtable::as_raw(this),
                value.try_into().map_err(|e| e.into())?.abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Dispatching\"`*"]
    #[cfg(feature = "UI_Dispatching")]
    pub fn DispatcherQueue(
        &self,
    ) -> ::windows::core::Result<super::super::Dispatching::DispatcherQueue> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DispatcherQueue)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::super::Dispatching::DispatcherQueue>(result__)
        }
    }
    pub fn TryGetAnimationController(
        &self,
        propertyname: &::windows::core::HSTRING,
    ) -> ::windows::core::Result<super::AnimationController> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TryGetAnimationController)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(propertyname),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::AnimationController>(result__)
        }
    }
    pub fn StartAnimationWithController<'a, P0>(
        &self,
        propertyname: &::windows::core::HSTRING,
        animation: P0,
        animationcontroller: &super::AnimationController,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::CompositionAnimation>>,
    {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject5>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).StartAnimationWithController)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(propertyname),
                animation.into().abi(),
                ::core::mem::transmute_copy(animationcontroller),
            )
            .ok()
        }
    }
    pub fn BitmapInterpolationMode(
        &self,
    ) -> ::windows::core::Result<super::CompositionBitmapInterpolationMode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).BitmapInterpolationMode)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::CompositionBitmapInterpolationMode>(result__)
        }
    }
    pub fn SetBitmapInterpolationMode(
        &self,
        value: super::CompositionBitmapInterpolationMode,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetBitmapInterpolationMode)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn Surface(&self) -> ::windows::core::Result<super::ICompositionSurface> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Surface)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::ICompositionSurface>(result__)
        }
    }
    pub fn SetSurface<'a, P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<'a, super::ICompositionSurface>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetSurface)(
                ::windows::core::Vtable::as_raw(this),
                value.try_into().map_err(|e| e.into())?.abi(),
            )
            .ok()
        }
    }
    pub fn WrappingUMode(&self) -> ::windows::core::Result<SceneWrappingMode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).WrappingUMode)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<SceneWrappingMode>(result__)
        }
    }
    pub fn SetWrappingUMode(&self, value: SceneWrappingMode) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetWrappingUMode)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn WrappingVMode(&self) -> ::windows::core::Result<SceneWrappingMode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).WrappingVMode)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<SceneWrappingMode>(result__)
        }
    }
    pub fn SetWrappingVMode(&self, value: SceneWrappingMode) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetWrappingVMode)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn Create(
        compositor: &super::Compositor,
    ) -> ::windows::core::Result<SceneSurfaceMaterialInput> {
        Self::ISceneSurfaceMaterialInputStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Create)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(compositor),
                result__.as_mut_ptr(),
            )
            .from_abi::<SceneSurfaceMaterialInput>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ISceneSurfaceMaterialInputStatics<
        R,
        F: FnOnce(&ISceneSurfaceMaterialInputStatics) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<
            SceneSurfaceMaterialInput,
            ISceneSurfaceMaterialInputStatics,
        > = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for SceneSurfaceMaterialInput {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SceneSurfaceMaterialInput {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SceneSurfaceMaterialInput {}
impl ::core::fmt::Debug for SceneSurfaceMaterialInput {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SceneSurfaceMaterialInput").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SceneSurfaceMaterialInput {
    const SIGNATURE : ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice ( b"rc(Microsoft.UI.Composition.Scenes.SceneSurfaceMaterialInput;{b9854b4f-286c-50cd-a734-491a251d5fd3})" ) ;
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for SceneSurfaceMaterialInput {
    type Vtable = ISceneSurfaceMaterialInput_Vtbl;
}
unsafe impl ::windows::core::Interface for SceneSurfaceMaterialInput {
    const IID: ::windows::core::GUID =
        <ISceneSurfaceMaterialInput as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for SceneSurfaceMaterialInput {
    const NAME: &'static str = "Microsoft.UI.Composition.Scenes.SceneSurfaceMaterialInput";
}
::windows::core::interface_hierarchy!(
    SceneSurfaceMaterialInput,
    ::windows::core::IUnknown,
    ::windows::core::IInspectable
);
impl ::core::convert::TryFrom<SceneSurfaceMaterialInput> for super::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: SceneSurfaceMaterialInput) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SceneSurfaceMaterialInput> for super::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: &SceneSurfaceMaterialInput) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&SceneSurfaceMaterialInput>
    for ::windows::core::InParam<'a, super::IAnimationObject>
{
    type Error = ::windows::core::Error;
    fn try_from(value: &SceneSurfaceMaterialInput) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<SceneSurfaceMaterialInput> for ::windows::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: SceneSurfaceMaterialInput) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SceneSurfaceMaterialInput> for ::windows::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &SceneSurfaceMaterialInput) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&SceneSurfaceMaterialInput>
    for ::windows::core::InParam<'a, ::windows::Foundation::IClosable>
{
    type Error = ::windows::core::Error;
    fn try_from(value: &SceneSurfaceMaterialInput) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::From<SceneSurfaceMaterialInput> for SceneMaterialInput {
    fn from(value: SceneSurfaceMaterialInput) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&SceneSurfaceMaterialInput> for SceneMaterialInput {
    fn from(value: &SceneSurfaceMaterialInput) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::core::convert::From<&SceneSurfaceMaterialInput>
    for ::windows::core::InParam<'a, SceneMaterialInput>
{
    fn from(value: &SceneSurfaceMaterialInput) -> Self {
        ::windows::core::InParam::owned(value.into())
    }
}
impl ::core::convert::From<SceneSurfaceMaterialInput> for SceneObject {
    fn from(value: SceneSurfaceMaterialInput) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&SceneSurfaceMaterialInput> for SceneObject {
    fn from(value: &SceneSurfaceMaterialInput) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::core::convert::From<&SceneSurfaceMaterialInput>
    for ::windows::core::InParam<'a, SceneObject>
{
    fn from(value: &SceneSurfaceMaterialInput) -> Self {
        ::windows::core::InParam::owned(value.into())
    }
}
impl ::core::convert::From<SceneSurfaceMaterialInput> for super::CompositionObject {
    fn from(value: SceneSurfaceMaterialInput) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&SceneSurfaceMaterialInput> for super::CompositionObject {
    fn from(value: &SceneSurfaceMaterialInput) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::core::convert::From<&SceneSurfaceMaterialInput>
    for ::windows::core::InParam<'a, super::CompositionObject>
{
    fn from(value: &SceneSurfaceMaterialInput) -> Self {
        ::windows::core::InParam::owned(value.into())
    }
}
unsafe impl ::core::marker::Send for SceneSurfaceMaterialInput {}
unsafe impl ::core::marker::Sync for SceneSurfaceMaterialInput {}
#[doc = "*Required features: `\"UI_Composition_Scenes\"`*"]
#[repr(transparent)]
pub struct SceneVisual(::windows::core::IUnknown);
impl SceneVisual {
    pub fn PopulatePropertyInfo(
        &self,
        propertyname: &::windows::core::HSTRING,
        propertyinfo: &super::AnimationPropertyInfo,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).PopulatePropertyInfo)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(propertyname),
                ::core::mem::transmute_copy(propertyinfo),
            )
            .ok()
        }
    }
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<::windows::Foundation::IClosable>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).Close)(::windows::core::Vtable::as_raw(this))
                .ok()
        }
    }
    pub fn Compositor(&self) -> ::windows::core::Result<super::Compositor> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Compositor)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::Compositor>(result__)
        }
    }
    pub fn Properties(&self) -> ::windows::core::Result<super::CompositionPropertySet> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Properties)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::CompositionPropertySet>(result__)
        }
    }
    pub fn StartAnimation<'a, P0>(
        &self,
        propertyname: &::windows::core::HSTRING,
        animation: P0,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::CompositionAnimation>>,
    {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).StartAnimation)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(propertyname),
                animation.into().abi(),
            )
            .ok()
        }
    }
    pub fn StopAnimation(
        &self,
        propertyname: &::windows::core::HSTRING,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).StopAnimation)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(propertyname),
            )
            .ok()
        }
    }
    pub fn Comment(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Comment)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetComment(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetComment)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn ImplicitAnimations(
        &self,
    ) -> ::windows::core::Result<super::ImplicitAnimationCollection> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ImplicitAnimations)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::ImplicitAnimationCollection>(result__)
        }
    }
    pub fn SetImplicitAnimations(
        &self,
        value: &super::ImplicitAnimationCollection,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetImplicitAnimations)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn StartAnimationGroup<'a, P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<'a, super::ICompositionAnimationBase>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).StartAnimationGroup)(
                ::windows::core::Vtable::as_raw(this),
                value.try_into().map_err(|e| e.into())?.abi(),
            )
            .ok()
        }
    }
    pub fn StopAnimationGroup<'a, P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<'a, super::ICompositionAnimationBase>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).StopAnimationGroup)(
                ::windows::core::Vtable::as_raw(this),
                value.try_into().map_err(|e| e.into())?.abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Dispatching\"`*"]
    #[cfg(feature = "UI_Dispatching")]
    pub fn DispatcherQueue(
        &self,
    ) -> ::windows::core::Result<super::super::Dispatching::DispatcherQueue> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DispatcherQueue)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::super::Dispatching::DispatcherQueue>(result__)
        }
    }
    pub fn TryGetAnimationController(
        &self,
        propertyname: &::windows::core::HSTRING,
    ) -> ::windows::core::Result<super::AnimationController> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TryGetAnimationController)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(propertyname),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::AnimationController>(result__)
        }
    }
    pub fn StartAnimationWithController<'a, P0>(
        &self,
        propertyname: &::windows::core::HSTRING,
        animation: P0,
        animationcontroller: &super::AnimationController,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::CompositionAnimation>>,
    {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject5>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).StartAnimationWithController)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(propertyname),
                animation.into().abi(),
                ::core::mem::transmute_copy(animationcontroller),
            )
            .ok()
        }
    }
    pub fn Children(&self) -> ::windows::core::Result<super::VisualCollection> {
        let this = &::windows::core::Interface::cast::<super::IContainerVisual>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Children)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::VisualCollection>(result__)
        }
    }
    pub fn Root(&self) -> ::windows::core::Result<SceneNode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Root)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<SceneNode>(result__)
        }
    }
    pub fn SetRoot(&self, value: &SceneNode) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetRoot)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn Create(compositor: &super::Compositor) -> ::windows::core::Result<SceneVisual> {
        Self::ISceneVisualStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Create)(
                ::windows::core::Vtable::as_raw(this),
                ::core::mem::transmute_copy(compositor),
                result__.as_mut_ptr(),
            )
            .from_abi::<SceneVisual>(result__)
        })
    }
    pub fn AnchorPoint(&self) -> ::windows::core::Result<::windows::Foundation::Numerics::Vector2> {
        let this = &::windows::core::Interface::cast::<super::IVisual>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AnchorPoint)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::Numerics::Vector2>(result__)
        }
    }
    pub fn SetAnchorPoint(
        &self,
        value: ::windows::Foundation::Numerics::Vector2,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IVisual>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetAnchorPoint)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn BackfaceVisibility(
        &self,
    ) -> ::windows::core::Result<super::CompositionBackfaceVisibility> {
        let this = &::windows::core::Interface::cast::<super::IVisual>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).BackfaceVisibility)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::CompositionBackfaceVisibility>(result__)
        }
    }
    pub fn SetBackfaceVisibility(
        &self,
        value: super::CompositionBackfaceVisibility,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IVisual>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetBackfaceVisibility)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn BorderMode(&self) -> ::windows::core::Result<super::CompositionBorderMode> {
        let this = &::windows::core::Interface::cast::<super::IVisual>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).BorderMode)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::CompositionBorderMode>(result__)
        }
    }
    pub fn SetBorderMode(
        &self,
        value: super::CompositionBorderMode,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IVisual>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetBorderMode)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn CenterPoint(&self) -> ::windows::core::Result<::windows::Foundation::Numerics::Vector3> {
        let this = &::windows::core::Interface::cast::<super::IVisual>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CenterPoint)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::Numerics::Vector3>(result__)
        }
    }
    pub fn SetCenterPoint(
        &self,
        value: ::windows::Foundation::Numerics::Vector3,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IVisual>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetCenterPoint)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn Clip(&self) -> ::windows::core::Result<super::CompositionClip> {
        let this = &::windows::core::Interface::cast::<super::IVisual>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Clip)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::CompositionClip>(result__)
        }
    }
    pub fn SetClip<'a, P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::CompositionClip>>,
    {
        let this = &::windows::core::Interface::cast::<super::IVisual>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetClip)(
                ::windows::core::Vtable::as_raw(this),
                value.into().abi(),
            )
            .ok()
        }
    }
    pub fn CompositeMode(&self) -> ::windows::core::Result<super::CompositionCompositeMode> {
        let this = &::windows::core::Interface::cast::<super::IVisual>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CompositeMode)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::CompositionCompositeMode>(result__)
        }
    }
    pub fn SetCompositeMode(
        &self,
        value: super::CompositionCompositeMode,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IVisual>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetCompositeMode)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsVisible(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::IVisual>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsVisible)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetIsVisible(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IVisual>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetIsVisible)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn Offset(&self) -> ::windows::core::Result<::windows::Foundation::Numerics::Vector3> {
        let this = &::windows::core::Interface::cast::<super::IVisual>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Offset)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::Numerics::Vector3>(result__)
        }
    }
    pub fn SetOffset(
        &self,
        value: ::windows::Foundation::Numerics::Vector3,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IVisual>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetOffset)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn Opacity(&self) -> ::windows::core::Result<f32> {
        let this = &::windows::core::Interface::cast::<super::IVisual>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Opacity)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<f32>(result__)
        }
    }
    pub fn SetOpacity(&self, value: f32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IVisual>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetOpacity)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn Orientation(
        &self,
    ) -> ::windows::core::Result<::windows::Foundation::Numerics::Quaternion> {
        let this = &::windows::core::Interface::cast::<super::IVisual>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Orientation)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::Numerics::Quaternion>(result__)
        }
    }
    pub fn SetOrientation(
        &self,
        value: ::windows::Foundation::Numerics::Quaternion,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IVisual>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetOrientation)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn Parent(&self) -> ::windows::core::Result<super::ContainerVisual> {
        let this = &::windows::core::Interface::cast::<super::IVisual>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Parent)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::ContainerVisual>(result__)
        }
    }
    pub fn RotationAngle(&self) -> ::windows::core::Result<f32> {
        let this = &::windows::core::Interface::cast::<super::IVisual>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RotationAngle)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<f32>(result__)
        }
    }
    pub fn SetRotationAngle(&self, value: f32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IVisual>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetRotationAngle)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn RotationAngleInDegrees(&self) -> ::windows::core::Result<f32> {
        let this = &::windows::core::Interface::cast::<super::IVisual>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RotationAngleInDegrees)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<f32>(result__)
        }
    }
    pub fn SetRotationAngleInDegrees(&self, value: f32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IVisual>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetRotationAngleInDegrees)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn RotationAxis(
        &self,
    ) -> ::windows::core::Result<::windows::Foundation::Numerics::Vector3> {
        let this = &::windows::core::Interface::cast::<super::IVisual>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RotationAxis)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::Numerics::Vector3>(result__)
        }
    }
    pub fn SetRotationAxis(
        &self,
        value: ::windows::Foundation::Numerics::Vector3,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IVisual>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetRotationAxis)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn Scale(&self) -> ::windows::core::Result<::windows::Foundation::Numerics::Vector3> {
        let this = &::windows::core::Interface::cast::<super::IVisual>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Scale)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::Numerics::Vector3>(result__)
        }
    }
    pub fn SetScale(
        &self,
        value: ::windows::Foundation::Numerics::Vector3,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IVisual>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetScale)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn Size(&self) -> ::windows::core::Result<::windows::Foundation::Numerics::Vector2> {
        let this = &::windows::core::Interface::cast::<super::IVisual>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Size)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::Numerics::Vector2>(result__)
        }
    }
    pub fn SetSize(
        &self,
        value: ::windows::Foundation::Numerics::Vector2,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IVisual>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetSize)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn TransformMatrix(
        &self,
    ) -> ::windows::core::Result<::windows::Foundation::Numerics::Matrix4x4> {
        let this = &::windows::core::Interface::cast::<super::IVisual>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TransformMatrix)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::Numerics::Matrix4x4>(result__)
        }
    }
    pub fn SetTransformMatrix(
        &self,
        value: ::windows::Foundation::Numerics::Matrix4x4,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IVisual>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetTransformMatrix)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn ParentForTransform(&self) -> ::windows::core::Result<super::Visual> {
        let this = &::windows::core::Interface::cast::<super::IVisual2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ParentForTransform)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::Visual>(result__)
        }
    }
    pub fn SetParentForTransform<'a, P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::Visual>>,
    {
        let this = &::windows::core::Interface::cast::<super::IVisual2>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetParentForTransform)(
                ::windows::core::Vtable::as_raw(this),
                value.into().abi(),
            )
            .ok()
        }
    }
    pub fn RelativeOffsetAdjustment(
        &self,
    ) -> ::windows::core::Result<::windows::Foundation::Numerics::Vector3> {
        let this = &::windows::core::Interface::cast::<super::IVisual2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RelativeOffsetAdjustment)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::Numerics::Vector3>(result__)
        }
    }
    pub fn SetRelativeOffsetAdjustment(
        &self,
        value: ::windows::Foundation::Numerics::Vector3,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IVisual2>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetRelativeOffsetAdjustment)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn RelativeSizeAdjustment(
        &self,
    ) -> ::windows::core::Result<::windows::Foundation::Numerics::Vector2> {
        let this = &::windows::core::Interface::cast::<super::IVisual2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RelativeSizeAdjustment)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::Numerics::Vector2>(result__)
        }
    }
    pub fn SetRelativeSizeAdjustment(
        &self,
        value: ::windows::Foundation::Numerics::Vector2,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IVisual2>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetRelativeSizeAdjustment)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsHitTestVisible(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::IVisual3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsHitTestVisible)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetIsHitTestVisible(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IVisual3>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetIsHitTestVisible)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsPixelSnappingEnabled(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::IVisual4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsPixelSnappingEnabled)(
                ::windows::core::Vtable::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetIsPixelSnappingEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IVisual4>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this).SetIsPixelSnappingEnabled)(
                ::windows::core::Vtable::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc(hidden)]
    pub fn ISceneVisualStatics<R, F: FnOnce(&ISceneVisualStatics) -> ::windows::core::Result<R>>(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<SceneVisual, ISceneVisualStatics> =
            ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for SceneVisual {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SceneVisual {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SceneVisual {}
impl ::core::fmt::Debug for SceneVisual {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SceneVisual").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SceneVisual {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Composition.Scenes.SceneVisual;{0144d7ad-6a7d-59cb-a0f9-74a04e85352c})",
    );
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for SceneVisual {
    type Vtable = ISceneVisual_Vtbl;
}
unsafe impl ::windows::core::Interface for SceneVisual {
    const IID: ::windows::core::GUID = <ISceneVisual as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for SceneVisual {
    const NAME: &'static str = "Microsoft.UI.Composition.Scenes.SceneVisual";
}
::windows::core::interface_hierarchy!(
    SceneVisual,
    ::windows::core::IUnknown,
    ::windows::core::IInspectable
);
impl ::core::convert::TryFrom<SceneVisual> for super::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: SceneVisual) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SceneVisual> for super::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: &SceneVisual) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&SceneVisual>
    for ::windows::core::InParam<'a, super::IAnimationObject>
{
    type Error = ::windows::core::Error;
    fn try_from(value: &SceneVisual) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<SceneVisual> for ::windows::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: SceneVisual) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SceneVisual> for ::windows::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &SceneVisual) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&SceneVisual>
    for ::windows::core::InParam<'a, ::windows::Foundation::IClosable>
{
    type Error = ::windows::core::Error;
    fn try_from(value: &SceneVisual) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::From<SceneVisual> for super::ContainerVisual {
    fn from(value: SceneVisual) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&SceneVisual> for super::ContainerVisual {
    fn from(value: &SceneVisual) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::core::convert::From<&SceneVisual>
    for ::windows::core::InParam<'a, super::ContainerVisual>
{
    fn from(value: &SceneVisual) -> Self {
        ::windows::core::InParam::owned(value.into())
    }
}
impl ::core::convert::From<SceneVisual> for super::Visual {
    fn from(value: SceneVisual) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&SceneVisual> for super::Visual {
    fn from(value: &SceneVisual) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::core::convert::From<&SceneVisual> for ::windows::core::InParam<'a, super::Visual> {
    fn from(value: &SceneVisual) -> Self {
        ::windows::core::InParam::owned(value.into())
    }
}
impl ::core::convert::From<SceneVisual> for super::CompositionObject {
    fn from(value: SceneVisual) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&SceneVisual> for super::CompositionObject {
    fn from(value: &SceneVisual) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::core::convert::From<&SceneVisual>
    for ::windows::core::InParam<'a, super::CompositionObject>
{
    fn from(value: &SceneVisual) -> Self {
        ::windows::core::InParam::owned(value.into())
    }
}
unsafe impl ::core::marker::Send for SceneVisual {}
unsafe impl ::core::marker::Sync for SceneVisual {}
#[doc = "*Required features: `\"UI_Composition_Scenes\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SceneAlphaMode(pub i32);
impl SceneAlphaMode {
    pub const Opaque: Self = Self(0i32);
    pub const AlphaTest: Self = Self(1i32);
    pub const Blend: Self = Self(2i32);
}
impl ::core::marker::Copy for SceneAlphaMode {}
impl ::core::clone::Clone for SceneAlphaMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SceneAlphaMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SceneAlphaMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for SceneAlphaMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SceneAlphaMode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SceneAlphaMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Composition.Scenes.SceneAlphaMode;i4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_Composition_Scenes\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SceneAttributeSemantic(pub i32);
impl SceneAttributeSemantic {
    pub const Index: Self = Self(0i32);
    pub const Vertex: Self = Self(1i32);
    pub const Normal: Self = Self(2i32);
    pub const TexCoord0: Self = Self(3i32);
    pub const TexCoord1: Self = Self(4i32);
    pub const Color: Self = Self(5i32);
    pub const Tangent: Self = Self(6i32);
}
impl ::core::marker::Copy for SceneAttributeSemantic {}
impl ::core::clone::Clone for SceneAttributeSemantic {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SceneAttributeSemantic {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SceneAttributeSemantic {
    type Abi = Self;
}
impl ::core::fmt::Debug for SceneAttributeSemantic {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SceneAttributeSemantic").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SceneAttributeSemantic {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Composition.Scenes.SceneAttributeSemantic;i4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_Composition_Scenes\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SceneComponentType(pub i32);
impl SceneComponentType {
    pub const MeshRendererComponent: Self = Self(0i32);
}
impl ::core::marker::Copy for SceneComponentType {}
impl ::core::clone::Clone for SceneComponentType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SceneComponentType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SceneComponentType {
    type Abi = Self;
}
impl ::core::fmt::Debug for SceneComponentType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SceneComponentType").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SceneComponentType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Composition.Scenes.SceneComponentType;i4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_Composition_Scenes\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SceneWrappingMode(pub i32);
impl SceneWrappingMode {
    pub const ClampToEdge: Self = Self(0i32);
    pub const MirroredRepeat: Self = Self(1i32);
    pub const Repeat: Self = Self(2i32);
}
impl ::core::marker::Copy for SceneWrappingMode {}
impl ::core::clone::Clone for SceneWrappingMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SceneWrappingMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SceneWrappingMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for SceneWrappingMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SceneWrappingMode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SceneWrappingMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Composition.Scenes.SceneWrappingMode;i4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
