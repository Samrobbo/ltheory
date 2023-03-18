use crate::internal::Memory::*;
use crate::Common::*;
use crate::Math::Vec3;
use crate::RenderState::*;
use std::ffi::CString;
use libc;
use sdl2_sys::*;
use crate::GL::gl;

#[no_mangle]
pub unsafe extern "C" fn OpenGL_Init() {
    static mut init: bool = false;
    if !init {
        init = true;
        gl::load_with(|s| {
            let cs = CString::new(s.as_bytes()).unwrap();
            SDL_GL_GetProcAddress(cs.as_ptr())
        });
    }
    
    gl::Disable(gl::MULTISAMPLE);
    gl::Disable(gl::CULL_FACE);
    gl::CullFace(gl::BACK);

    gl::PixelStorei(gl::PACK_ALIGNMENT, 1);
    gl::PixelStorei(gl::UNPACK_ALIGNMENT, 1);
    gl::DepthFunc(gl::LEQUAL);

    gl::Enable(gl::BLEND);
    gl::BlendFunc(gl::ONE, gl::ZERO);

    gl::Enable(gl::TEXTURE_CUBE_MAP_SEAMLESS);
    gl::Disable(gl::POINT_SMOOTH);
    gl::Disable(gl::LINE_SMOOTH);
    gl::Hint(gl::POINT_SMOOTH_HINT, gl::FASTEST);
    gl::Hint(gl::LINE_SMOOTH_HINT, gl::FASTEST);
    gl::LineWidth(2.0f32);
    
    gl::MatrixMode(gl::PROJECTION);
    gl::LoadIdentity();
    gl::MatrixMode(gl::MODELVIEW);
    gl::LoadIdentity();

    RenderState_PushAllDefaults();
}

#[no_mangle]
pub unsafe extern "C" fn OpenGL_CheckError(file: *const libc::c_char, line: i32) {
    let errorID: gl::types::GLenum = gl::GetError();
    let mut error: *const libc::c_char = std::ptr::null();
    match errorID {
        0 => return,
        1280 => {
            error = b"GL_INVALID_ENUM\0" as *const u8 as *const libc::c_char;
        }
        1281 => {
            error = b"GL_INVALID_VALUE\0" as *const u8 as *const libc::c_char;
        }
        1282 => {
            error = b"GL_INVALID_OPERATION\0" as *const u8 as *const libc::c_char;
        }
        1286 => {
            error = b"GL_INVALID_FRAMEBUFFER_OPERATION\0" as *const u8 as *const libc::c_char;
        }
        1285 => {
            error = b"GL_OUT_OF_MEMORY\0" as *const u8 as *const libc::c_char;
        }
        _ => {
            Fatal(
                b"OpenGL_CheckError: gl::GetError returned illegal error code %u at %s:%d\0"
                    as *const u8 as *const libc::c_char,
                errorID,
                file,
                line,
            );
        }
    }
    Fatal(
        b"OpenGL_CheckError: %s at %s:%d\0" as *const u8 as *const libc::c_char,
        error,
        file,
        line,
    );
}
