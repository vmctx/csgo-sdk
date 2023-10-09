#[repr(i32)]
pub enum Bones {
    Head = 8,
    Neck = 7,
    Spine0 = 3,
    Spine1 = 4,
    Spine2 = 5,
    Spine3 = 6,
    LegUpperL = 65,
    LegLowerL = 66,
    LegUpperR = 72,
    LegLowerR = 73,
    FootL = 69,
    FootR = 76,
}

pub fn get_bone_list() -> &'static [i32; 12] {
    static BONES: [i32; 12] = [
        Bones::Head as i32,
        Bones::Neck as i32,
        Bones::Spine0 as i32,
        Bones::Spine1 as i32,
        Bones::Spine2 as i32,
        Bones::Spine3 as i32,
        Bones::LegUpperL as i32,
        Bones::LegLowerL as i32,
        Bones::LegUpperR as i32,
        Bones::LegLowerR as i32,
        Bones::FootL as i32,
        Bones::FootR as i32,
    ];

    &BONES
}
