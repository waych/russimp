use russimp_sys::{aiScene, aiImportFile, aiReleaseImport, aiPostProcessSteps_aiProcess_CalcTangentSpace, aiPostProcessSteps_aiProcess_Triangulate, aiPostProcessSteps_aiProcess_JoinIdenticalVertices, aiPostProcessSteps_aiProcess_SortByPType, aiPostProcessSteps_aiProcess_MakeLeftHanded, aiPostProcessSteps_aiProcess_RemoveComponent, aiPostProcessSteps_aiProcess_GenNormals, aiPostProcessSteps_aiProcess_GenSmoothNormals, aiPostProcessSteps_aiProcess_SplitLargeMeshes, aiPostProcessSteps_aiProcess_PreTransformVertices, aiPostProcessSteps_aiProcess_LimitBoneWeights, aiPostProcessSteps_aiProcess_ValidateDataStructure, aiPostProcessSteps_aiProcess_ImproveCacheLocality, aiPostProcessSteps_aiProcess_RemoveRedundantMaterials, aiPostProcessSteps_aiProcess_FixInfacingNormals, aiPostProcessSteps_aiProcess_FindDegenerates, aiPostProcessSteps_aiProcess_FindInvalidData, aiPostProcessSteps_aiProcess_GenUVCoords, aiPostProcessSteps_aiProcess_TransformUVCoords, aiPostProcessSteps_aiProcess_FindInstances, aiPostProcessSteps_aiProcess_OptimizeMeshes, aiPostProcessSteps_aiProcess_OptimizeGraph, aiPostProcessSteps_aiProcess_FlipUVs, aiPostProcessSteps_aiProcess_FlipWindingOrder, aiPostProcessSteps_aiProcess_SplitByBoneCount, aiPostProcessSteps_aiProcess_Debone, aiPostProcessSteps_aiProcess_GlobalScale, aiPostProcessSteps_aiProcess_EmbedTextures, aiPostProcessSteps_aiProcess_ForceGenNormals, aiPostProcessSteps_aiProcess_DropNormals, aiPostProcessSteps_aiProcess_GenBoundingBoxes, aiGetErrorString, aiMaterial, aiAnimation, aiCamera, aiLight, aiMesh};

use std::ffi::{CString, CStr};
use std::ops::BitOr;
use crate::{RussimpError, Russult, FromRawVec};
use crate::material::Material;
use crate::animation::Animation;
use crate::camera::Camera;
use crate::light::Light;
use crate::mesh::Mesh;
use crate::metadata::MetaData;
use crate::node::Node;

pub struct Scene {
    scene: *const aiScene
}

#[repr(u32)]
pub enum PostProcessSteps {
    CalcTangentSpace = aiPostProcessSteps_aiProcess_CalcTangentSpace,
    JoinIdenticalVertices = aiPostProcessSteps_aiProcess_JoinIdenticalVertices,
    MakeLeftHanded = aiPostProcessSteps_aiProcess_MakeLeftHanded,
    Triangulate = aiPostProcessSteps_aiProcess_Triangulate,
    RemoveComponent = aiPostProcessSteps_aiProcess_RemoveComponent,
    GenNormals = aiPostProcessSteps_aiProcess_GenNormals,
    GenSmoothNormals = aiPostProcessSteps_aiProcess_GenSmoothNormals,
    SplitLargeMeshes = aiPostProcessSteps_aiProcess_SplitLargeMeshes,
    PreTransformVertices = aiPostProcessSteps_aiProcess_PreTransformVertices,
    LimitBoneWeights = aiPostProcessSteps_aiProcess_LimitBoneWeights,
    ValidateDataStructure = aiPostProcessSteps_aiProcess_ValidateDataStructure,
    ImproveCacheLocality = aiPostProcessSteps_aiProcess_ImproveCacheLocality,
    RemoveRedundantMaterials = aiPostProcessSteps_aiProcess_RemoveRedundantMaterials,
    FixInfacingNormals = aiPostProcessSteps_aiProcess_FixInfacingNormals,
    SortByPType = aiPostProcessSteps_aiProcess_SortByPType,
    FindDegenerates = aiPostProcessSteps_aiProcess_FindDegenerates,
    FindInvalidData = aiPostProcessSteps_aiProcess_FindInvalidData,
    GenUVCoords = aiPostProcessSteps_aiProcess_GenUVCoords,
    TransformUVCoords = aiPostProcessSteps_aiProcess_TransformUVCoords,
    FindInstances = aiPostProcessSteps_aiProcess_FindInstances,
    OptimizeMeshes = aiPostProcessSteps_aiProcess_OptimizeMeshes,
    OptimizeGraph = aiPostProcessSteps_aiProcess_OptimizeGraph,
    FlipUVs = aiPostProcessSteps_aiProcess_FlipUVs,
    FlipWindingOrder = aiPostProcessSteps_aiProcess_FlipWindingOrder,
    SplitByBoneCount = aiPostProcessSteps_aiProcess_SplitByBoneCount,
    Debone = aiPostProcessSteps_aiProcess_Debone,
    GlobalScale = aiPostProcessSteps_aiProcess_GlobalScale,
    EmbedTextures = aiPostProcessSteps_aiProcess_EmbedTextures,
    ForceGenNormals = aiPostProcessSteps_aiProcess_ForceGenNormals,
    DropNormals = aiPostProcessSteps_aiProcess_DropNormals,
    GenBoundingBoxes = aiPostProcessSteps_aiProcess_GenBoundingBoxes,
}

impl BitOr<PostProcessSteps> for PostProcessSteps {
    type Output = u32;

    fn bitor(self, rhs: PostProcessSteps) -> Self::Output {
        self | rhs
    }
}

impl BitOr<u32> for PostProcessSteps {
    type Output = ();

    fn bitor(self, rhs: u32) -> Self::Output {
        self | rhs
    }
}

impl BitOr<PostProcessSteps> for u32 {
    type Output = u32;

    fn bitor(self, rhs: PostProcessSteps) -> Self::Output {
        self | rhs
    }
}

impl Drop for Scene {
    fn drop(&mut self) {
        unsafe {
            aiReleaseImport(self.scene);
        }
    }
}

impl FromRawVec<aiMaterial, Material> for Scene {}

impl FromRawVec<aiAnimation, Animation> for Scene {}

impl FromRawVec<aiCamera, Camera> for Scene {}

impl FromRawVec<aiLight, Light> for Scene {}

impl FromRawVec<aiMesh, Mesh> for Scene {}

impl Scene {
    pub fn from(file_path: &str, flags: u32) -> Russult<Self> {
        let c_str = CString::new(file_path).unwrap();
        let scene_import: *const aiScene = unsafe { aiImportFile(c_str.as_ptr(), flags) };

        if scene_import.is_null() {
            let error_buf = unsafe { aiGetErrorString() };
            let error = unsafe { CStr::from_ptr(error_buf).to_string_lossy().into_owned() };
            return Err(RussimpError::Import(error));
        }

        Ok(Self {
            scene: scene_import
        })
    }

    pub fn get_materials(&self) -> Vec<Material> {
        Self::get_vec(unsafe { (*self.scene).mMaterials }, unsafe { (*self.scene).mNumMaterials } as usize)
    }

    pub fn get_animations(&self) -> Vec<Animation> {
        Self::get_vec(unsafe { (*self.scene).mAnimations }, unsafe { (*self.scene).mNumAnimations } as usize)
    }

    pub fn get_cameras(&self) -> Vec<Camera> {
        Self::get_vec(unsafe { (*self.scene).mCameras }, unsafe { (*self.scene).mNumCameras } as usize)
    }

    pub fn get_flags(&self) -> u32 {
        unsafe { (*self.scene).mFlags }
    }

    pub fn get_lights(&self) -> Vec<Light> {
        Self::get_vec(unsafe { (*self.scene).mLights }, unsafe { (*self.scene).mNumLights } as usize)
    }

    pub fn get_meshes(&self) -> Vec<Mesh> {
        Self::get_vec(unsafe { (*self.scene).mMeshes }, unsafe { (*self.scene).mNumMeshes } as usize)
    }

    pub fn get_meta_data(&self) -> MetaData {
        unsafe { (*self.scene).mMetaData }.into()
    }

    pub fn get_private(&self) -> Russult<String> {
        let string_raw = unsafe { CString::from_raw(unsafe { (*self.scene).mPrivate }) };

        match string_raw.into_string() {
            Ok(content) => Ok(content),
            Err(err) => Err(err.into())
        }
    }

    pub fn get_node(&self) -> Node {
        unsafe { (*self.scene).mRootNode }.into()
    }
}