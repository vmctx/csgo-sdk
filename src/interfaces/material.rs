use libc::c_char;

#[repr(C)]
pub enum MaterialVarFlags {
    MaterialVarDebug = (1 << 0),
    MaterialVarNoDebugOverride = (1 << 1),
    MaterialVarNoDraw = (1 << 2),
    MaterialVarUseInFillrateMode = (1 << 3),
    MaterialVarVertexcolor = (1 << 4),
    MaterialVarVertexalpha = (1 << 5),
    MaterialVarSelfillum = (1 << 6),
    MaterialVarAdditive = (1 << 7),
    MaterialVarAlphatest = (1 << 8),
    MaterialVarPseudoTranslucent = (1 << 9),
    // used to mark water materials for rendering after opaques but before translucents (with alpha blending but also with depth writes)
    MaterialVarZnearer = (1 << 10),
    MaterialVarModel = (1 << 11),
    MaterialVarFlat = (1 << 12),
    MaterialVarNocull = (1 << 13),
    MaterialVarNofog = (1 << 14),
    MaterialVarIgnorez = (1 << 15),
    MaterialVarDecal = (1 << 16),
    MaterialVarEnvmapsphere = (1 << 17),
    // OBSOLETE
    MaterialVarAoprepass = (1 << 18),
    MaterialVarEnvmapcameraspace = (1 << 19),
    // OBSOLETE
    MaterialVarBasealphaenvmapmask = (1 << 20),
    MaterialVarTranslucent = (1 << 21),
    MaterialVarNormalmapalphaenvmapmask = (1 << 22),
    MaterialVarNeedsSoftwareSkinning = (1 << 23),
    // OBSOLETE
    MaterialVarOpaquetexture = (1 << 24),
    MaterialVarMultiply = (1 << 25),
    MaterialVarSuppressDecals = (1 << 26),
    MaterialVarHalflambert = (1 << 27),
    MaterialVarWireframe = (1 << 28),
    MaterialVarAllowalphatocoverage = (1 << 29),
    MaterialVarAlphaModifiedByProxy = (1 << 30),
    MaterialVarVertexfog = (1 << 31),
}

interface!(
    IMaterial,
    pub get_name[0]() -> *const c_char,
    pub get_texture_group_name[1]() -> *const c_char,
    pub find_var[11](name: *const c_char, found: *mut bool, complain: bool) -> IMaterialVar,
    pub alpha_modulate[27](alpha: f32) -> (),
    pub color_modulate[28](r: f32, g: f32, b: f32) -> (),
    pub set_material_var_flag[29](flag: MaterialVarFlags, on: bool) -> (),
    pub is_precached[70]() -> bool
);

interface!(
    IMaterialVar,
    pub get_texture[1]() -> ITexture,
    pub set_float[4](value: f32) -> (),
    pub set_int[5](value: i32) -> (),
    pub set_string[6](value: *const c_char) -> (),
    pub set_vector[10](x: f32, y: f32) -> (),
    pub set_vector_3d[11](x: f32, y: f32, z: f32) -> (),
    pub set_texture[15](texture: ITexture) -> ()
);

create_interface!(ITexture);
