use crate::internal::Memory::*;
use crate::Common::*;
use crate::Math::Vec3;
use crate::Math::{IVec2, IVec3};
use crate::Metric::*;
use crate::Profiler::*;
use crate::CubeFace::*;
use crate::Tex2D::*;
use crate::Tex3D::*;
use crate::TexCube::*;
use crate::TexFormat::*;
use crate::TexFormat::*;
use crate::Viewport::*;
use crate::GL::gl;
use libc;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct FBO {
    pub handle: u32,
    pub colorIndex: i32,
    pub sx: i32,
    pub sy: i32,
    pub depth: bool,
}

static mut fboIndex: i32 = -1_i32;

static mut fboStack: [FBO; 16] = [FBO {
    handle: 0,
    colorIndex: 0,
    sx: 0,
    sy: 0,
    depth: false,
}; 16];

#[inline]
unsafe extern "C" fn GetActive() -> *mut FBO {
    fboStack.as_mut_ptr().offset(fboIndex as isize)
}

#[inline]
unsafe extern "C" fn SetDrawBuffers(mut count: i32) {
    static mut bufs: [gl::types::GLenum; 4] = [
        gl::COLOR_ATTACHMENT0 as gl::types::GLenum,
        gl::COLOR_ATTACHMENT1 as gl::types::GLenum,
        gl::COLOR_ATTACHMENT2 as gl::types::GLenum,
        gl::COLOR_ATTACHMENT3 as gl::types::GLenum,
    ];
    gl::DrawBuffers(count, bufs.as_ptr());
}

#[no_mangle]
pub unsafe extern "C" fn RenderTarget_Push(mut sx: i32, mut sy: i32) {
    Profiler_Begin(
        (*::core::mem::transmute::<&[u8; 18], &[libc::c_char; 18]>(b"RenderTarget_Push\0"))
            .as_ptr(),
    );
    if fboIndex + 1_i32 >= 16_i32 {
        Fatal(
            b"RenderTarget_Push: Maximum stack depth exceeded\0" as *const u8
                as *const libc::c_char,
        );
    }
    fboIndex += 1;
    let mut this: *mut FBO = GetActive();
    (*this).handle = 0_u32;
    (*this).colorIndex = 0_i32;
    (*this).sx = sx;
    (*this).sy = sy;
    (*this).depth = false;
    Metric_Inc(0x7_i32);
    gl::GenFramebuffers(1_i32, &mut (*this).handle);
    gl::BindFramebuffer(gl::FRAMEBUFFER, (*this).handle);
    Viewport_Push(0_i32, 0_i32, sx, sy, false);
    Profiler_End();
}

#[no_mangle]
pub unsafe extern "C" fn RenderTarget_Pop() {
    Profiler_Begin(
        (*::core::mem::transmute::<&[u8; 17], &[libc::c_char; 17]>(b"RenderTarget_Pop\0")).as_ptr(),
    );
    if fboIndex < 0_i32 {
        Fatal(
            b"RenderTarget_Pop: Attempting to pop an empty stack\0" as *const u8
                as *const libc::c_char,
        );
    }
    let mut i: u32 = 0;
    while i < 4 {
        gl::FramebufferTexture2D(
            gl::FRAMEBUFFER,
            gl::COLOR_ATTACHMENT0 + i,
            gl::TEXTURE_2D,
            0,
            0,
        );
        i += 1;
    }
    gl::FramebufferTexture2D(
        gl::FRAMEBUFFER,
        gl::DEPTH_ATTACHMENT,
        gl::TEXTURE_2D,
        0,
        0_i32,
    );
    gl::DeleteFramebuffers(
        1_i32,
        &mut (*fboStack.as_mut_ptr().offset(fboIndex as isize)).handle,
    );
    fboIndex -= 1;
    Metric_Inc(0x7_i32);
    if fboIndex >= 0_i32 {
        gl::BindFramebuffer(
            gl::FRAMEBUFFER,
            (*GetActive()).handle,
        );
    } else {
        gl::BindFramebuffer(
            gl::FRAMEBUFFER,
            0,
        );
    }
    Viewport_Pop();
    Profiler_End();
}

#[no_mangle]
pub unsafe extern "C" fn RenderTarget_BindTex2D(mut this: *mut Tex2D) {
    RenderTarget_BindTex2DLevel(this, 0_i32);
}

#[no_mangle]
pub unsafe extern "C" fn RenderTarget_BindTex2DLevel(mut tex: *mut Tex2D, mut level: i32) {
    let mut this: *mut FBO = GetActive();
    let mut handle: u32 = Tex2D_GetHandle(tex);
    if TexFormat_IsColor(Tex2D_GetFormat(tex)) {
        if (*this).colorIndex >= 4_i32 {
            Fatal(
                b"RenderTarget_BindTex2D: Max color attachments exceeded\0" as *const u8
                    as *const libc::c_char,
            );
        }
        gl::FramebufferTexture2D(
            gl::FRAMEBUFFER,
            gl::COLOR_ATTACHMENT0 + (*this).colorIndex as u32,
            gl::TEXTURE_2D,
            handle,
            level,
        );
        (*this).colorIndex += 1;
        SetDrawBuffers((*this).colorIndex);
    } else {
        if (*this).depth {
            Fatal(
                b"RenderTarget_BindTex2D: Target already has a depth buffer\0" as *const u8
                    as *const libc::c_char,
            );
        }
        gl::FramebufferTexture2D(
            gl::FRAMEBUFFER,
            gl::DEPTH_ATTACHMENT,
            gl::TEXTURE_2D,
            handle,
            level,
        );
        (*this).depth = true;
    };
}

#[no_mangle]
pub unsafe extern "C" fn RenderTarget_BindTex3D(mut this: *mut Tex3D, mut layer: i32) {
    RenderTarget_BindTex3DLevel(this, layer, 0_i32);
}

#[no_mangle]
pub unsafe extern "C" fn RenderTarget_BindTex3DLevel(
    mut tex: *mut Tex3D,
    mut layer: i32,
    mut level: i32,
) {
    let mut this: *mut FBO = GetActive();
    if (*this).colorIndex >= 4_i32 {
        Fatal(
            b"RenderTarget_BindTex3D: Max color attachments exceeded\0" as *const u8
                as *const libc::c_char,
        );
    }

    let mut handle: u32 = Tex3D_GetHandle(tex);
    gl::FramebufferTexture3D(
        gl::FRAMEBUFFER,
        gl::COLOR_ATTACHMENT0 + (*this).colorIndex as u32,
        gl::TEXTURE_3D,
        handle,
        level,
        layer,
    );
    (*this).colorIndex += 1;
    SetDrawBuffers((*this).colorIndex);
}

#[no_mangle]
pub unsafe extern "C" fn RenderTarget_BindTexCube(mut this: *mut TexCube, mut face: CubeFace) {
    RenderTarget_BindTexCubeLevel(this, face, 0_i32);
}

#[no_mangle]
pub unsafe extern "C" fn RenderTarget_BindTexCubeLevel(
    mut tex: *mut TexCube,
    mut face: CubeFace,
    mut level: i32,
) {
    let mut this: *mut FBO = GetActive();
    if (*this).colorIndex >= 4_i32 {
        Fatal(
            b"RenderTarget_BindTexCubeLevel: Max color attachments exceeded\0" as *const u8
                as *const libc::c_char,
        );
    }
    let mut handle: u32 = TexCube_GetHandle(tex);

    gl::FramebufferTexture2D(
        gl::FRAMEBUFFER,
        gl::COLOR_ATTACHMENT0 + (*this).colorIndex as u32,
        face as u32,
        handle,
        level,
    );
    (*this).colorIndex += 1;
    SetDrawBuffers((*this).colorIndex);
}

#[no_mangle]
pub unsafe extern "C" fn RenderTarget_PushTex2D(mut this: *mut Tex2D) {
    RenderTarget_PushTex2DLevel(this, 0_i32);
}

#[no_mangle]
pub unsafe extern "C" fn RenderTarget_PushTex2DLevel(mut this: *mut Tex2D, mut level: i32) {
    let mut size: IVec2 = IVec2 { x: 0, y: 0 };
    Tex2D_GetSizeLevel(this, &mut size, level);
    RenderTarget_Push(size.x, size.y);
    RenderTarget_BindTex2DLevel(this, level);
}

#[no_mangle]
pub unsafe extern "C" fn RenderTarget_PushTex3D(mut this: *mut Tex3D, mut layer: i32) {
    RenderTarget_PushTex3DLevel(this, layer, 0_i32);
}

#[no_mangle]
pub unsafe extern "C" fn RenderTarget_PushTex3DLevel(
    mut this: *mut Tex3D,
    mut layer: i32,
    mut level: i32,
) {
    let mut size: IVec3 = IVec3 { x: 0, y: 0, z: 0 };
    Tex3D_GetSizeLevel(this, &mut size, level);
    RenderTarget_Push(size.x, size.y);
    RenderTarget_BindTex3DLevel(this, layer, level);
}
