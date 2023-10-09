use crate::utils::math;
use crate::utils::math::vector::Vec3;

interface!(
    IPrediction,
    set_local_view_angles_virtual[13](view_angles: &Vec3) -> ()
);

impl IPrediction {
    pub fn set_local_view_angles(&self, mut view_angles: Vec3) {
        view_angles = math::normalize(view_angles);
        view_angles.x = view_angles.x.clamp(-89.0, 89.0);
        view_angles.y = view_angles.y.clamp(-180.0, 180.0);
        view_angles.z = 0.0;

        self.set_local_view_angles_virtual(&view_angles);
    }
}
