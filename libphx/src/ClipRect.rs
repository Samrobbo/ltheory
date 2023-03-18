use crate::internal::Memory::*;
use crate::Common::*;
use crate::Math::IVec2;
use crate::Math::Vec3;
use crate::Viewport::*;
use crate::GL::gl;
use libc;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ClipRect {
    pub x: f32,
    pub y: f32,
    pub sx: f32,
    pub sy: f32,
    pub enabled: bool,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ClipRectTransform {
    pub tx: f32,
    pub ty: f32,
    pub sx: f32,
    pub sy: f32,
}
static mut transform: [ClipRectTransform; 128] = [ClipRectTransform {
    tx: 0.,
    ty: 0.,
    sx: 0.,
    sy: 0.,
}; 128];

static mut transformIndex: i32 = -1_i32;

static mut rect: [ClipRect; 128] = [ClipRect {
    x: 0.,
    y: 0.,
    sx: 0.,
    sy: 0.,
    enabled: false,
}; 128];

static mut rectIndex: i32 = -1_i32;

#[inline]
unsafe extern "C" fn TransformRect(
    mut x: *mut f32,
    mut y: *mut f32,
    mut sx: *mut f32,
    mut sy: *mut f32,
) {
    if transformIndex >= 0_i32 {
        let mut curr: *mut ClipRectTransform =
            transform.as_mut_ptr().offset(transformIndex as isize);
        *x = (*curr).sx * *x + (*curr).tx;
        *y = (*curr).sy * *y + (*curr).ty;
        *sx *= (*curr).sx;
        *sy *= (*curr).sy;
    }
}

#[no_mangle]
pub unsafe extern "C" fn ClipRect_Activate(mut this: *mut ClipRect) {
    if !this.is_null() && (*this).enabled as i32 != 0 {
        let mut vpSize: IVec2 = IVec2 { x: 0, y: 0 };
        Viewport_GetSize(&mut vpSize);
        gl::Enable(gl::SCISSOR_TEST);
        let mut x: f32 = (*this).x;
        let mut y: f32 = (*this).y;
        let mut sx: f32 = (*this).sx;
        let mut sy: f32 = (*this).sy;
        TransformRect(&mut x, &mut y, &mut sx, &mut sy);
        gl::Scissor(x as i32, vpSize.y - (y + sy) as i32, sx as i32, sy as i32);
    } else {
        gl::Disable(gl::SCISSOR_TEST);
    };
}

#[no_mangle]
pub unsafe extern "C" fn ClipRect_Push(mut x: f32, mut y: f32, mut sx: f32, mut sy: f32) {
    if rectIndex + 1_i32 >= 128_i32 {
        Fatal(b"ClipRect_Push: Maximum stack depth exceeded\0" as *const u8 as *const libc::c_char);
    }
    rectIndex += 1;
    let mut curr: *mut ClipRect = rect.as_mut_ptr().offset(rectIndex as isize);
    (*curr).x = x;
    (*curr).y = y;
    (*curr).sx = sx;
    (*curr).sy = sy;
    (*curr).enabled = true;
    ClipRect_Activate(curr);
}

#[no_mangle]
pub unsafe extern "C" fn ClipRect_PushCombined(mut x: f32, mut y: f32, mut sx: f32, mut sy: f32) {
    let mut curr: *mut ClipRect = rect.as_mut_ptr().offset(rectIndex as isize);
    if rectIndex >= 0_i32 && (*curr).enabled as i32 != 0 {
        let mut maxX: f32 = x + sx;
        let mut maxY: f32 = y + sy;
        x = f64::max(x as f64, (*curr).x as f64) as f32;
        y = f64::max(y as f64, (*curr).y as f64) as f32;
        ClipRect_Push(
            x,
            y,
            (f64::min(maxX as f64, ((*curr).x + (*curr).sx) as f64) - x as f64) as f32,
            (f64::min(maxY as f64, ((*curr).y + (*curr).sy) as f64) - y as f64) as f32,
        );
    } else {
        ClipRect_Push(x, y, sx, sy);
    };
}

#[no_mangle]
pub unsafe extern "C" fn ClipRect_PushDisabled() {
    if rectIndex + 1_i32 >= 128_i32 {
        Fatal(b"ClipRect_Push: Maximum stack depth exceeded\0" as *const u8 as *const libc::c_char);
    }
    rectIndex += 1;
    let mut curr: *mut ClipRect = rect.as_mut_ptr().offset(rectIndex as isize);
    (*curr).enabled = false;
    ClipRect_Activate(curr);
}

#[no_mangle]
pub unsafe extern "C" fn ClipRect_PushTransform(
    mut tx: f32,
    mut ty: f32,
    mut sx: f32,
    mut sy: f32,
) {
    if transformIndex + 1_i32 >= 128_i32 {
        Fatal(
            b"ClipRect_PushTransform: Maximum stack depth exceeded\0" as *const u8
                as *const libc::c_char,
        );
    }
    transformIndex += 1;
    let mut curr: *mut ClipRectTransform = transform.as_mut_ptr().offset(transformIndex as isize);
    (*curr).tx = tx;
    (*curr).ty = ty;
    (*curr).sx = sx;
    (*curr).sy = sy;
    if rectIndex >= 0_i32 {
        ClipRect_Activate(rect.as_mut_ptr().offset(rectIndex as isize));
    }
}

#[no_mangle]
pub unsafe extern "C" fn ClipRect_Pop() {
    if rectIndex < 0_i32 {
        Fatal(
            b"ClipRect_Pop: Attempting to pop an empty stack\0" as *const u8 as *const libc::c_char,
        );
    }
    rectIndex -= 1;
    ClipRect_Activate(if rectIndex >= 0_i32 {
        rect.as_mut_ptr().offset(rectIndex as isize)
    } else {
        std::ptr::null_mut()
    });
}

#[no_mangle]
pub unsafe extern "C" fn ClipRect_PopTransform() {
    if transformIndex < 0_i32 {
        Fatal(
            b"ClipRect_PopTransform: Attempting to pop an empty stack\0" as *const u8
                as *const libc::c_char,
        );
    }
    transformIndex -= 1;
    if rectIndex >= 0_i32 {
        ClipRect_Activate(rect.as_mut_ptr().offset(rectIndex as isize));
    }
}
