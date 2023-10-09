#[repr(i32)]
#[derive(Clone, Copy)]
pub enum FrameStage {
    UNDEFINED = -1,
    START,
    NetUpdateStart,
    NetUpdatePostdataupdateStart,
    NetUpdatePostdataupdateEnd,
    NetUpdateEnd,
    RenderStart,
    RenderEnd,
}
