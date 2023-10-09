//! Math utilities and the custom types it uses.

pub mod matrix;
pub mod vector;

pub fn normalize(vector: vector::Vec3) -> vector::Vec3 {
    let wrap = |mut value: f32| -> f32 {
        while value < -180.0 {
            value += 360.0;
        }
        while value > 180.0 {
            value -= 360.0;
        }

        value
    };

    vector::Vec3::new(wrap(vector.x), wrap(vector.y), 0.0)
}

pub fn calc_angle(source: vector::Vec3, dest: vector::Vec3) -> vector::Vec3 {
    let mut res = vector::Vec3::empty();
    let delta = source - dest;

    let hyp = (delta.x * delta.x + delta.y * delta.y).sqrt();

    res.x = f32::to_degrees((delta.z / hyp).atan());
    res.y = f32::to_degrees((delta.y / delta.x).atan());
    res.z = 0.0;

    if delta.x >= 0.0 {
        res.y += 180.0;
    }

    normalize(res)
}

pub fn angle_vectors(to_vector: vector::Vec3) -> vector::Vec3 {
    let (sy, cy) = to_vector.x.to_radians().sin_cos();
    let (sp, cp) = to_vector.y.to_radians().sin_cos();

    vector::Vec3::new(cp * cy, cp * sy, -sp)
}

pub fn get_fov(source: vector::Vec3, dest: vector::Vec3, current: vector::Vec3) -> f32 {
    let delta = normalize(calc_angle(source, dest) - current);
    let mut fov = delta.x - delta.y;

    if fov < 0.0 {
        fov *= -1.0;
    }

    fov
}

pub fn get_player_distance(nearest: vector::Vec3, new: vector::Vec3) -> f32 {
    let mut dist = (((nearest.x - new.x).powi(2)
        + (nearest.y - new.y).powi(2)
        + (nearest.z - new.z).powi(2)) as f32)
        .sqrt();
    dist *= 0.01905;
    dist
}
