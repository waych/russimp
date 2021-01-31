use std::{
    ffi::{
        CString,
        CStr,
    },
    rc::Rc,
    cell::RefCell,
};

use derivative::Derivative;

use crate::{
    sys::{
        aiScene,
        aiPostProcessSteps_aiProcess_JoinIdenticalVertices,
        aiPostProcessSteps_aiProcess_CalcTangentSpace,
        aiPostProcessSteps_aiProcess_MakeLeftHanded,
        aiPostProcessSteps_aiProcess_Triangulate,
        aiPostProcessSteps_aiProcess_RemoveComponent,
        aiPostProcessSteps_aiProcess_GenNormals,
        aiPostProcessSteps_aiProcess_GenSmoothNormals,
        aiPostProcessSteps_aiProcess_SplitLargeMeshes,
        aiPostProcessSteps_aiProcess_PreTransformVertices,
        aiPostProcessSteps_aiProcess_LimitBoneWeights,
        aiPostProcessSteps_aiProcess_ValidateDataStructure,
        aiPostProcessSteps_aiProcess_ImproveCacheLocality,
        aiPostProcessSteps_aiProcess_RemoveRedundantMaterials,
        aiPostProcessSteps_aiProcess_FixInfacingNormals,
        aiPostProcessSteps_aiProcess_SortByPType,
        aiPostProcessSteps_aiProcess_FindDegenerates,
        aiPostProcessSteps_aiProcess_FindInvalidData,
        aiPostProcessSteps_aiProcess_GenUVCoords,
        aiPostProcessSteps_aiProcess_TransformUVCoords,
        aiPostProcessSteps_aiProcess_FindInstances,
        aiPostProcessSteps_aiProcess_OptimizeMeshes,
        aiPostProcessSteps_aiProcess_OptimizeGraph,
        aiPostProcessSteps_aiProcess_FlipWindingOrder,
        aiPostProcessSteps_aiProcess_FlipUVs,
        aiPostProcessSteps_aiProcess_SplitByBoneCount,
        aiPostProcessSteps_aiProcess_Debone,
        aiPostProcessSteps_aiProcess_GlobalScale,
        aiPostProcessSteps_aiProcess_EmbedTextures,
        aiPostProcessSteps_aiProcess_ForceGenNormals,
        aiPostProcessSteps_aiProcess_DropNormals,
        aiPostProcessSteps_aiProcess_GenBoundingBoxes,
        aiReleaseImport,
        aiImportFile,
        aiGetErrorString,
    },
    Russult,
    RussimpError,
    material::Material,
    mesh::Mesh,
    metadata::MetaData,
    animation::Animation,
    camera::Camera,
    light::Light,
    node::Node,
    texture::Texture,
    Utils
};

#[derive(Derivative)]
#[derivative(Debug)]
pub struct Scene {
    pub materials: Vec<Material>,
    pub meshes: Vec<Mesh>,
    pub metadata: Option<MetaData>,
    pub animations: Vec<Animation>,
    pub cameras: Vec<Camera>,
    pub lights: Vec<Light>,
    pub root: Option<Rc<RefCell<Node>>>,
    pub textures: Vec<Texture>,
    pub flags: u32,
}

#[derive(Derivative)]
#[derivative(Debug)]
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

impl Drop for Scene {
    fn drop(&mut self) {
        unsafe {
            aiReleaseImport(self.scene);
        }
    }
}

impl Scene {
    pub fn from(file_path: &str, flags: Vec<PostProcessSteps>) -> Russult<Scene> {
        let bitwise_flag = flags.into_iter().fold(0, |acc, x| acc | (x as u32));
        let file_path = CString::new(file_path).unwrap();

        Scene::get_scene_from_file(file_path, bitwise_flag).map_or(Err(Scene::get_error()), |scene| Ok(Self {
            materials: Utils::get_vec_from_raw(scene.mMaterials, scene.mNumMaterials),
            meshes: Utils::get_vec_from_raw(scene.mMeshes, scene.mNumMeshes),
            metadata: Utils::get_raw(scene.mMetaData),
            animations: Utils::get_vec_from_raw(scene.mAnimations, scene.mNumAnimations),
            cameras: Utils::get_vec_from_raw(scene.mCameras, scene.mNumCameras),
            lights: Utils::get_vec_from_raw(scene.mLights, scene.mNumLights),
            root: Utils::get_rc_raw(scene.mRootNode),
            textures: Utils::get_vec_from_raw(scene.mTextures, scene.mNumTextures),
            flags: scene.mFlags,
        }))
    }

    #[inline]
    fn get_scene_from_file<'a>(string: CString, flags: u32) -> Option<&'a aiScene> {
        unsafe { aiImportFile(string.as_ptr(), flags).as_ref() }
    }

    fn get_error() -> RussimpError {
        let error_buf = unsafe { aiGetErrorString() };
        let error = unsafe { CStr::from_ptr(error_buf).to_string_lossy().into_owned() };
        return RussimpError::Import(error);
    }
}

#[test]
fn importing_invalid_file_returns_error() {
    let current_directory_buf = get_model("models/box.blend");

    let scene = Scene::from(current_directory_buf.as_str(),
                            vec![PostProcessSteps::CalcTangentSpace,
                                 PostProcessSteps::Triangulate,
                                 PostProcessSteps::JoinIdenticalVertices,
                                 PostProcessSteps::SortByPType]);

    assert!(scene.is_err())
}

#[test]
fn importing_valid_file_returns_scene() {
    let current_directory_buf = get_model("models/BLEND/box.blend");

    let scene = Scene::from(current_directory_buf.as_str(),
                            vec![PostProcessSteps::CalcTangentSpace,
                                 PostProcessSteps::Triangulate,
                                 PostProcessSteps::JoinIdenticalVertices,
                                 PostProcessSteps::SortByPType]).unwrap();

    assert_eq!(8, scene.flags);
}

#[test]
fn debug_scene() {
    let box_file_path = get_model("models/BLEND/box.blend");

    let scene = Scene::from(box_file_path.as_str(),
                            vec![PostProcessSteps::CalcTangentSpace,
                                 PostProcessSteps::Triangulate,
                                 PostProcessSteps::JoinIdenticalVertices,
                                 PostProcessSteps::SortByPType]).unwrap();

    dbg!(&scene);
}