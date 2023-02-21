use crate::internal::Memory::*;
use glam::Vec2;
use glam::Vec3;
use libc;
extern "C" {
    fn Hash_XX64(buf: *const libc::c_void, len: i32, seed: u64) -> u64;
    fn cos(_: f64) -> f64;
    fn sin(_: f64) -> f64;
    fn log(_: f64) -> f64;
    fn ldexp(_: f64, _: i32) -> f64;
    fn sqrt(_: f64) -> f64;
    fn floor(_: f64) -> f64;
    fn TimeStamp_Get() -> TimeStamp;
}
pub type cstr = *const libc::c_char;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RNG {
    pub seed: u64,
    pub state: [u64; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Quat {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Vec4f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}
pub type TimeStamp = u64;

#[inline]
unsafe extern "C" fn Floor(mut t: f64) -> f64 {
    return floor(t);
}
#[inline]
unsafe extern "C" fn Log(mut t: f64) -> f64 {
    return log(t);
}
#[inline]
unsafe extern "C" fn Max(mut a: f64, mut b: f64) -> f64 {
    return if a > b { a } else { b };
}
#[inline]
unsafe extern "C" fn Round(mut t: f64) -> f64 {
    return Floor(t + 0.5f64);
}
#[inline]
unsafe extern "C" fn Sqrt(mut t: f64) -> f64 {
    return sqrt(t);
}
#[inline]
unsafe extern "C" fn Cos(mut t: f64) -> f64 {
    return cos(t);
}
#[inline]
unsafe extern "C" fn Sin(mut t: f64) -> f64 {
    return sin(t);
}
#[inline]
unsafe extern "C" fn Random_SplitMix64(mut state: *mut u64) -> u64 {
    *state = (*state as u64).wrapping_add(0x9e3779b97f4a7c15 as u64) as u64;
    let mut z: u64 = *state;
    z = (z ^ z >> 30 as i32).wrapping_mul(0xbf58476d1ce4e5b9 as u64);
    z = (z ^ z >> 27 as i32).wrapping_mul(0x94d049bb133111eb as u64);
    return z ^ z >> 31 as i32;
}
#[inline]
unsafe extern "C" fn rotl(x: u64, mut k: i32) -> u64 {
    return x << k | x >> 64 as i32 - k;
}
#[inline]
unsafe extern "C" fn Random_Xoroshiro128(mut state0: *mut u64, mut state1: *mut u64) -> u64 {
    let s0: u64 = *state0;
    let mut s1: u64 = *state1;
    let result: u64 = s0.wrapping_add(s1);
    s1 ^= s0;
    *state0 = rotl(s0, 55 as i32) ^ s1 ^ s1 << 14 as i32;
    *state1 = rotl(s1, 36 as i32);
    return result;
}

#[inline]
unsafe extern "C" fn RNG_Next64(mut this: *mut RNG) -> u64 {
    return Random_Xoroshiro128(
        &mut *((*this).state).as_mut_ptr().offset(0),
        &mut *((*this).state).as_mut_ptr().offset(1),
    );
}
#[inline]
unsafe extern "C" fn RNG_Next32(mut this: *mut RNG) -> u32 {
    return (RNG_Next64(this) & 0xffffffff as u32 as u64) as u32;
}
#[inline]
unsafe extern "C" fn RNG_Init(mut this: *mut RNG) {
    let mut seed: u64 = (*this).seed;
    let mut i: i32 = 0 as i32;
    while i < 64 as i32 {
        seed = Random_SplitMix64(&mut seed);
        i += 1;
    }
    (*this).state[0] = Random_SplitMix64(&mut seed);
    (*this).state[1] = Random_SplitMix64(&mut seed);
    let mut i_0: i32 = 0 as i32;
    while i_0 < 64 as i32 {
        RNG_Next64(this);
        i_0 += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn RNG_Create(mut seed: u64) -> *mut RNG {
    let mut this: *mut RNG = MemAlloc(::core::mem::size_of::<RNG>()) as *mut RNG;
    (*this).seed = seed;
    RNG_Init(this);
    return this;
}
#[no_mangle]
pub unsafe extern "C" fn RNG_FromStr(mut s: cstr) -> *mut RNG {
    return RNG_Create(Hash_XX64(
        s as *const libc::c_void,
        StrLen(s) as i32,
        0 as i32 as u64,
    ));
}
#[no_mangle]
pub unsafe extern "C" fn RNG_FromTime() -> *mut RNG {
    return RNG_Create(TimeStamp_Get());
}
#[no_mangle]
pub unsafe extern "C" fn RNG_Free(mut this: *mut RNG) {
    MemFree(this as *const libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn RNG_Rewind(mut this: *mut RNG) {
    RNG_Init(this);
}
#[no_mangle]
pub unsafe extern "C" fn RNG_Chance(mut this: *mut RNG, mut probability: f64) -> bool {
    return RNG_GetUniform(this) < probability;
}
#[no_mangle]
pub unsafe extern "C" fn RNG_Get31(mut this: *mut RNG) -> i32 {
    let mut i: u32 = RNG_Next32(this) & 0x7fffffff as u32;
    return *(&mut i as *mut u32 as *mut i32);
}
#[no_mangle]
pub unsafe extern "C" fn RNG_Get32(mut this: *mut RNG) -> u32 {
    return RNG_Next32(this);
}
#[no_mangle]
pub unsafe extern "C" fn RNG_Get64(mut this: *mut RNG) -> u64 {
    return RNG_Next64(this);
}
#[no_mangle]
pub unsafe extern "C" fn RNG_GetAngle(mut this: *mut RNG) -> f64 {
    return 6.28318531f32 as f64 * RNG_GetUniform(this);
}
#[no_mangle]
pub unsafe extern "C" fn RNG_GetInt(mut this: *mut RNG, mut lower: i32, mut upper: i32) -> i32 {
    let mut t: f64 = RNG_GetUniform(this);
    return Round(lower as f64 + t * (upper - lower) as f64) as i32;
}
#[no_mangle]
pub unsafe extern "C" fn RNG_GetRNG(mut this: *mut RNG) -> *mut RNG {
    return RNG_Create(RNG_Get64(this));
}
#[no_mangle]
pub unsafe extern "C" fn RNG_GetUniform(mut this: *mut RNG) -> f64 {
    return ldexp(RNG_Next32(this) as f64, -(32 as i32));
}
#[no_mangle]
pub unsafe extern "C" fn RNG_GetUniformRange(
    mut this: *mut RNG,
    mut lower: f64,
    mut upper: f64,
) -> f64 {
    let mut t: f64 = ldexp(RNG_Next32(this) as f64, -(32 as i32));
    return lower + t * (upper - lower);
}
#[no_mangle]
pub unsafe extern "C" fn RNG_GetErlang(mut this: *mut RNG, mut k: i32) -> f64 {
    let mut sum: f64 = 0.0f64;
    let mut i: i32 = 0 as i32;
    while i < k {
        sum += RNG_GetExp(this);
        i += 1;
    }
    return sum / k as f64;
}
#[no_mangle]
pub unsafe extern "C" fn RNG_GetExp(mut this: *mut RNG) -> f64 {
    return -Log(Max(
        1.0f64 - RNG_GetUniform(this),
        2.2204460492503131e-16f64,
    ));
}
#[no_mangle]
pub unsafe extern "C" fn RNG_GetGaussian(mut this: *mut RNG) -> f64 {
    let mut angle: f64 = RNG_GetAngle(this);
    let mut radius: f64 = 1.0f64 - RNG_GetUniform(this);
    return Cos(angle) * Sqrt(-2.0f64 * Log(radius));
}
#[no_mangle]
pub unsafe extern "C" fn RNG_GetAxis2(mut this: *mut RNG, mut out: *mut Vec2) {
    *out = Vec2::new(0.0f32, 0.0f32);
    let mut axis: i32 = RNG_GetInt(this, 0 as i32, 3 as i32);
    match axis {
        0 => {
            (*out).x = 1.0f32;
            return;
        }
        1 => {
            (*out).x = -1.0f32;
            return;
        }
        2 => {
            (*out).y = 1.0f32;
            return;
        }
        3 => {
            (*out).y = -1.0f32;
            return;
        }
        _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn RNG_GetAxis3(mut this: *mut RNG, mut out: *mut Vec3) {
    *out = Vec3::ZERO;
    let mut axis: i32 = RNG_GetInt(this, 0 as i32, 5 as i32);
    match axis {
        0 => {
            (*out).x = 1.0f32;
            return;
        }
        1 => {
            (*out).x = -1.0f32;
            return;
        }
        2 => {
            (*out).y = 1.0f32;
            return;
        }
        3 => {
            (*out).y = -1.0f32;
            return;
        }
        4 => {
            (*out).z = 1.0f32;
            return;
        }
        5 => {
            (*out).z = -1.0f32;
            return;
        }
        _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn RNG_GetDir2(mut this: *mut RNG, mut out: *mut Vec2) {
    let mut angle: f64 = RNG_GetAngle(this);
    *out = Vec2::new(Cos(angle) as f32, Sin(angle) as f32);
}
#[no_mangle]
pub unsafe extern "C" fn RNG_GetDir3(mut this: *mut RNG, mut out: *mut Vec3) {
    loop {
        let mut x: f64 = 2.0f64 * RNG_GetUniform(this) - 1.0f64;
        let mut y: f64 = 2.0f64 * RNG_GetUniform(this) - 1.0f64;
        let mut z: f64 = 2.0f64 * RNG_GetUniform(this) - 1.0f64;
        let mut m2: f64 = x * x + y * y + z * z;
        if m2 <= 1.0f64 && m2 > 1e-6f64 {
            m2 = Sqrt(m2);
            (*out).x = (x / m2) as f32;
            (*out).y = (y / m2) as f32;
            (*out).z = (z / m2) as f32;
            return;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn RNG_GetDisc(mut this: *mut RNG, mut out: *mut Vec2) {
    loop {
        let mut x: f64 = 2.0f64 * RNG_GetUniform(this) - 1.0f64;
        let mut y: f64 = 2.0f64 * RNG_GetUniform(this) - 1.0f64;
        if x * x + y * y <= 1.0f64 {
            (*out).x = x as f32;
            (*out).y = y as f32;
            return;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn RNG_GetSign(mut this: *mut RNG) -> f64 {
    return if RNG_GetUniform(this) > 0.5f64 {
        1.0f64
    } else {
        -1.0f64
    };
}
#[no_mangle]
pub unsafe extern "C" fn RNG_GetSphere(mut this: *mut RNG, mut out: *mut Vec3) {
    loop {
        let mut x: f64 = 2.0f64 * RNG_GetUniform(this) - 1.0f64;
        let mut y: f64 = 2.0f64 * RNG_GetUniform(this) - 1.0f64;
        let mut z: f64 = 2.0f64 * RNG_GetUniform(this) - 1.0f64;
        if x * x + y * y + z * z <= 1.0f64 {
            (*out).x = x as f32;
            (*out).y = y as f32;
            (*out).z = z as f32;
            return;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn RNG_GetVec2(
    mut this: *mut RNG,
    mut out: *mut Vec2,
    mut lower: f64,
    mut upper: f64,
) {
    (*out).x = RNG_GetUniformRange(this, lower, upper) as f32;
    (*out).y = RNG_GetUniformRange(this, lower, upper) as f32;
}
#[no_mangle]
pub unsafe extern "C" fn RNG_GetVec3(
    mut this: *mut RNG,
    mut out: *mut Vec3,
    mut lower: f64,
    mut upper: f64,
) {
    (*out).x = RNG_GetUniformRange(this, lower, upper) as f32;
    (*out).y = RNG_GetUniformRange(this, lower, upper) as f32;
    (*out).z = RNG_GetUniformRange(this, lower, upper) as f32;
}
#[no_mangle]
pub unsafe extern "C" fn RNG_GetVec4(
    mut this: *mut RNG,
    mut out: *mut Vec4f,
    mut lower: f64,
    mut upper: f64,
) {
    (*out).x = RNG_GetUniformRange(this, lower, upper) as f32;
    (*out).y = RNG_GetUniformRange(this, lower, upper) as f32;
    (*out).z = RNG_GetUniformRange(this, lower, upper) as f32;
    (*out).w = RNG_GetUniformRange(this, lower, upper) as f32;
}
#[no_mangle]
pub unsafe extern "C" fn RNG_GetQuat(mut this: *mut RNG, mut out: *mut Quat) {
    let mut p0 = Vec2::ZERO;
    let mut p1 = Vec2::ZERO;
    RNG_GetDisc(this, &mut p0);
    RNG_GetDisc(this, &mut p1);
    let mut d0 = p0.length_squared() as f64;
    let mut d1 = p1.length_squared() as f64 + 2.2204460492503131e-16f64;
    let mut s = Sqrt((1.0f64 - d0) / d1);
    (*out).x = p0.y;
    (*out).y = (p1.x as f64 * s) as f32;
    (*out).z = (p1.y as f64 * s) as f32;
    (*out).w = p0.x;
}
