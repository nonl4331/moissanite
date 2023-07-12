use crate::prelude::*;

#[allow(dead_code)]
pub unsafe fn load_obj(path: &str, scale: f32, offset: Vec3) {
    let (models, _) = tobj::load_obj(
        path,
        &tobj::LoadOptions {
            triangulate: true,
            ignore_points: true,
            ignore_lines: true,
            ..Default::default()
        },
    )
    .unwrap();

    let mut total = 0;

    for m in models.iter() {
        let mesh = &m.mesh;

        let vo = VERTICES.len();
        let no = NORMALS.len();

        // load vertices
        for j in 0..mesh.positions.len() / 3 {
            let i = j * 3;
            VERTICES.push(
                Vec3::new(
                    mesh.positions[i] * scale,
                    mesh.positions[i + 1] * scale,
                    mesh.positions[i + 2] * scale,
                ) + offset,
            )
        }

        // load normals
        for j in 0..mesh.normals.len() / 3 {
            let i = j * 3;
            NORMALS.push(Vec3::new(
                mesh.normals[i],
                mesh.normals[i + 1],
                mesh.normals[i + 2],
            ))
        }

        // create triangles
        let ilen = mesh.indices.len();
        let nilen = mesh.normal_indices.len();
        assert_eq!(ilen % 3, 0);
        assert_eq!(ilen, nilen);

        for j in 0..ilen / 3 {
            let i = j * 3;
            TRIANGLES.push(Triangle::new(
                [
                    mesh.indices[i] as usize + vo,
                    mesh.indices[i + 1] as usize + vo,
                    mesh.indices[i + 2] as usize + vo,
                ],
                [
                    mesh.normal_indices[i] as usize + no,
                    mesh.normal_indices[i + 1] as usize + no,
                    mesh.normal_indices[i + 2] as usize + no,
                ],
                0,
                0,
            ));
        }

        total += mesh.indices.len() / 3;
    }

    log::info!("loaded {total} triangles");
}
