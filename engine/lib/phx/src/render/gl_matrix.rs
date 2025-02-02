use super::*;
use crate::math::*;

/* NOTE : LoadMatrix expects column-major memory layout, but we use row-major,
 *        hence the need for transpositions when taking a Matrix*. */

#[no_mangle]
pub unsafe extern "C" fn GLMatrix_Clear() {
    gl::LoadIdentity();
}

#[no_mangle]
pub unsafe extern "C" fn GLMatrix_Load(matrix: &mut Matrix) {
    let m: &[f32; 16] = &matrix.m;
    let transpose: [f32; 16] = [
        m[0], m[4], m[8], m[12], m[1], m[5], m[9], m[13], m[2], m[6], m[10], m[14], m[3], m[7],
        m[11], m[15],
    ];
    gl::LoadMatrixf(transpose.as_ptr());
}

#[no_mangle]
pub unsafe extern "C" fn GLMatrix_LookAt(eye: *const DVec3, at: *const DVec3, up: *const DVec3) {
    let z = (*at - *eye).normalize();
    let x = DVec3::cross(z, (*up).normalize()).normalize();
    let y = DVec3::cross(x, z);

    /* TODO : Yet another sign flip. Sigh. */
    let mut m: [f64; 16] = [
        x.x, y.x, -z.x, 0.0, x.y, y.y, -z.y, 0.0, x.z, y.z, -z.z, 0.0, 0.0, 0.0, 0.0, 1.0,
    ];

    gl::MultMatrixd(m.as_mut_ptr());
    gl::Translated(-(*eye).x, -(*eye).y, -(*eye).z);
}

#[no_mangle]
pub unsafe extern "C" fn GLMatrix_ModeP() {
    gl::MatrixMode(gl::PROJECTION);
}

#[no_mangle]
pub unsafe extern "C" fn GLMatrix_ModeWV() {
    gl::MatrixMode(gl::MODELVIEW);
}

#[no_mangle]
pub unsafe extern "C" fn GLMatrix_Mult(matrix: &mut Matrix) {
    let m: &[f32; 16] = &matrix.m;
    let transpose: [f32; 16] = [
        m[0], m[4], m[8], m[12], m[1], m[5], m[9], m[13], m[2], m[6], m[10], m[14], m[3], m[7],
        m[11], m[15],
    ];
    gl::MultMatrixf(transpose.as_ptr());
}

#[no_mangle]
pub unsafe extern "C" fn GLMatrix_Perspective(fovy: f64, aspect: f64, z0: f64, z1: f64) {
    let rads: f64 = std::f32::consts::PI as f64 * fovy / 360.0f64;
    let cot: f64 = 1.0f64 / f64::tan(rads);
    let dz: f64 = z1 - z0;
    let nf: f64 = -2.0f64 * (z0 * z1) / dz;
    let mut m: [f64; 16] = [
        cot / aspect,
        0.0,
        0.0,
        0.0,
        0.0,
        cot,
        0.0,
        0.0,
        0.0,
        0.0,
        -(z0 + z1) / dz,
        -1.0f64,
        0.0,
        0.0,
        nf,
        0.0,
    ];
    gl::MultMatrixd(m.as_mut_ptr());
}

#[no_mangle]
pub unsafe extern "C" fn GLMatrix_Pop() {
    gl::PopMatrix();
}

#[no_mangle]
pub unsafe extern "C" fn GLMatrix_Push() {
    gl::PushMatrix();
}

#[no_mangle]
pub unsafe extern "C" fn GLMatrix_PushClear() {
    gl::PushMatrix();
    gl::LoadIdentity();
}

#[no_mangle]
pub unsafe extern "C" fn GLMatrix_Get() -> Option<Box<Matrix>> {
    let mut matrixMode: gl::types::GLint = 0;
    gl::GetIntegerv(gl::MATRIX_MODE, &mut matrixMode);

    match matrixMode as u32 {
        gl::MODELVIEW => {
            matrixMode = gl::MODELVIEW_MATRIX as i32;
        }
        gl::PROJECTION => {
            matrixMode = gl::PROJECTION_MATRIX as i32;
        }
        gl::COLOR | gl::TEXTURE | _ => return None,
    }

    let mut matrix: Box<Matrix> = Matrix_Identity();
    gl::GetFloatv(matrixMode as gl::types::GLenum, matrix.m.as_mut_ptr());
    Some(matrix)
}

#[no_mangle]
pub unsafe extern "C" fn GLMatrix_RotateX(angle: f64) {
    gl::Rotated(angle, 1.0f64, 0.0f64, 0.0f64);
}

#[no_mangle]
pub unsafe extern "C" fn GLMatrix_RotateY(angle: f64) {
    gl::Rotated(angle, 0.0f64, 1.0f64, 0.0f64);
}

#[no_mangle]
pub unsafe extern "C" fn GLMatrix_RotateZ(angle: f64) {
    gl::Rotated(angle, 0.0f64, 0.0f64, 1.0f64);
}

#[no_mangle]
pub unsafe extern "C" fn GLMatrix_Scale(x: f64, y: f64, z: f64) {
    gl::Scaled(x, y, z);
}

#[no_mangle]
pub unsafe extern "C" fn GLMatrix_Translate(x: f64, y: f64, z: f64) {
    gl::Translated(x, y, z);
}
