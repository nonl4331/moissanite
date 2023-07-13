use crate::prelude::*;

pub unsafe fn cornell_box(scale: f32) {
    let vo = VERTICES.len();
    let no = NORMALS.len();
    let mo = MATERIALS.len();

    VERTICES.extend([
        Vec3::new(1.0, 1.0, 1.0) * scale,
        Vec3::new(-1.0, 1.0, 1.0) * scale,
        Vec3::new(1.0, -1.0, 1.0) * scale,
        Vec3::new(-1.0, -1.0, 1.0) * scale,
        Vec3::new(1.0, 1.0, -1.0) * scale,
        Vec3::new(-1.0, 1.0, -1.0) * scale,
        Vec3::new(1.0, -1.0, -1.0) * scale,
        Vec3::new(-1.0, -1.0, -1.0) * scale,
    ]);
    NORMALS.extend([
        Vec3::new(1.0, 0.0, 0.0),
        Vec3::new(-1.0, 0.0, 0.0),
        Vec3::new(0.0, -1.0, 0.0),
        Vec3::new(0.0, 0.0, 1.0),
        Vec3::new(0.0, 0.0, -1.0),
    ]);
    MATERIALS.extend([
        // white
        Mat::SpectralReflectanceDistribution(SpectralReflectanceDistribution::new([
            0.0, 0.445, 0.723, 0.767, 0.729, 0.735, 0.733, 0.728, 0.754, 0.740, 0.731, 0.730,
            0.760, 0.737, 0.0, 0.0,
        ])),
        // green
        Mat::SpectralReflectanceDistribution(SpectralReflectanceDistribution::new([
            0.0, 0.096, 0.097, 0.101, 0.125, 0.343, 0.481, 0.373, 0.266, 0.160, 0.121, 0.117,
            0.139, 0.159, 0.0, 0.0,
        ])),
        // red
        Mat::SpectralReflectanceDistribution(SpectralReflectanceDistribution::new([
            0.0, 0.046, 0.055, 0.061, 0.060, 0.056, 0.059, 0.063, 0.090, 0.287, 0.584, 0.610,
            0.628, 0.642, 0.0, 0.0,
        ])),
    ]);
    TRIANGLES.extend([
        Triangle::new([3 + vo, 1 + vo, 5 + vo], [no, no, no], 2 + mo),
        Triangle::new([5 + vo, 7 + vo, 3 + vo], [no, no, no], 2 + mo),
        Triangle::new([vo, 2 + vo, 6 + vo], [1 + no, 1 + no, 1 + no], 1 + mo),
        Triangle::new([6 + vo, 4 + vo, vo], [1 + no, 1 + no, 1 + no], 1 + mo),
        Triangle::new([1 + vo, vo, 4 + vo], [2 + no, 2 + no, 2 + no], mo),
        Triangle::new([4 + vo, 5 + vo, 1 + vo], [2 + no, 2 + no, 2 + no], mo),
        Triangle::new([5 + vo, 4 + vo, 6 + vo], [3 + no, 3 + no, 3 + no], mo),
        Triangle::new([6 + vo, 7 + vo, 5 + vo], [3 + no, 3 + no, 3 + no], mo),
        Triangle::new([1 + vo, vo, 2 + vo], [4 + no, 4 + no, 4 + no], mo),
        Triangle::new([2 + vo, 3 + vo, 1 + vo], [4 + no, 4 + no, 4 + no], mo),
    ]);
}
