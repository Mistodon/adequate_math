use matrix::{ Mat4 };
use vector::*;


pub fn ortho_projection(aspect: f32, size: f32, near: f32, far: f32) -> Mat4<f32>
{
    let inv_width = 1.0 / (aspect * size);
    let inv_height = 1.0 / size;

    Mat4([
         [inv_width, 0.0, 0.0, 0.0],
         [0.0, inv_height, 0.0, 0.0],
         [0.0, 0.0, 2.0 / (far - near), 0.0],
         [0.0, 0.0, -(far + near) / (far - near), 1.0]
    ])
}


pub fn axis_rotation(axis: Vec3<f32>, angle: f32) -> Mat4<f32>
{
    let (x, y, z) = axis.norm().as_tuple();
    let (s, c) = angle.sin_cos();
    let ic = 1.0 - c;

    Mat4([
         [(c + x * x * ic), (y * x * ic + z * s), (z * x * ic - y * s), 0.0],
         [(x * y * ic - z * s), (c + y * y * ic), (z * y * ic + x * s), 0.0],
         [(x * z * ic + y * s), (y * z * ic - x * s), (c + z * z * ic), 0.0],
         [0.0, 0.0, 0.0, 1.0],
    ])

}


pub fn euler_rotation(angles: Vec3<f32>) -> Mat4<f32>
{
    let (x, y, z) = angles.as_tuple();
    let (sx, cx) = x.sin_cos();
    let (sy, cy) = y.sin_cos();
    let (sz, cz) = z.sin_cos();

    Mat4([
         [cz * cy, sz * cy, -sy, 0.0],
         [cz * sy * sx - sz * cx, sz * sy * sx + cz * cx, cy * sx, 0.0],
         [cz * sy * cx + sz * sx, sz * sy * cx - cz * sx, cy * cx, 0.0],
         [0.0, 0.0, 0.0, 1.0]
    ])
}


pub fn look_rotation(forward: Vec3<f32>, up: Vec3<f32>) -> Mat4<f32>
{
    let forward = forward.norm();
    let up = up.norm();
    let right = up.cross(forward);
    let up = forward.cross(right);

    Mat4([right.extend(0.0).0, up.extend(0.0).0, forward.extend(0.0).0, [0.0, 0.0, 0.0, 1.0]])
}


#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn orthographic_projection()
    {
        let m = ortho_projection(1.0, 4.0, 0.0, 4.0);
        let p = vec4(4.0, 4.0, 4.0, 1.0);
        assert_eq!(m * p, vec4(1.0, 1.0, 1.0, 1.0));
    }

    #[test]
    fn rotation_around_axis()
    {
        let m = axis_rotation(vec3(0.0, 1.0, 0.0), ::std::f32::consts::PI / 2.0);
        let x = vec4(1.0, 0.0, 0.0, 1.0);
        let y = vec4(0.0, 1.0, 0.0, 1.0);
        let z = vec4(0.0, 0.0, 1.0, 1.0);
        let mx = (m * x).map(f32::round);
        let my = (m * y).map(f32::round);
        let mz = (m * z).map(f32::round);
        assert_eq!(mx, vec4(0.0, 0.0, -1.0, 1.0));
        assert_eq!(my, vec4(0.0, 1.0, 0.0, 1.0));
        assert_eq!(mz, vec4(1.0, 0.0, 0.0, 1.0));
    }

    #[test]
    fn euler_rotation_of_vector()
    {
        let a = ::std::f32::consts::PI / 2.0;

        let u = vec4(1.0, 0.0, 0.0, 1.0);

        let mx = euler_rotation(vec3(  a, 0.0, 0.0));
        let my = euler_rotation(vec3(0.0,   a, 0.0));
        let mz = euler_rotation(vec3(0.0, 0.0,   a));

        let ux = (mx * u).map(f32::round);
        let uy = (my * u).map(f32::round);
        let uz = (mz * u).map(f32::round);

        assert_eq!(ux, vec4(1.0, 0.0, 0.0, 1.0));
        assert_eq!(uy, vec4(0.0, 0.0, -1.0, 1.0));
        assert_eq!(uz, vec4(0.0, 1.0, 0.0, 1.0));
    }

    #[test]
    fn look_at_rotation()
    {
        let m = look_rotation(vec3(1.0, 0.0, 0.0), vec3(0.0, 1.0, 0.0));

        let x = vec4(1.0, 0.0, 0.0, 1.0);
        let y = vec4(0.0, 1.0, 0.0, 1.0);
        let z = vec4(0.0, 0.0, 1.0, 1.0);

        let mx = (m * x).map(f32::round);
        let my = (m * y).map(f32::round);
        let mz = (m * z).map(f32::round);

        assert_eq!(mx, vec4(0.0, 0.0, -1.0, 1.0));
        assert_eq!(my, vec4(0.0, 1.0, 0.0, 1.0));
        assert_eq!(mz, vec4(1.0, 0.0, 0.0, 1.0));
    }
}
