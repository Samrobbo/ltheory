use crate::internal::Memory::*;
use crate::Common::*;
use crate::Bytes::*;
use crate::ClipRect::*;
use crate::CubeFace::*;
use crate::DataFormat::*;
use crate::Draw::*;
use crate::GLMatrix::*;
use crate::Math::Vec3;
use crate::PixelFormat::*;
use crate::RenderState::*;
use crate::RenderTarget::*;
use crate::Shader::*;
use crate::ShaderState::*;
use crate::Tex2D::*;
use crate::Tex2D_Load::*;
use crate::Tex2D_Save::*;
use crate::TexFormat::*;
use crate::TimeStamp::*;
use crate::GL::gl;
use libc;

extern "C" {
    static mut __glewGenerateMipmap: PFNGLGENERATEMIPMAPPROC;
}

pub type uchar = libc::c_uchar;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct TexCube {
    pub _refCount: u32,
    pub handle: u32,
    pub size: i32,
    pub format: TexFormat,
}
pub type TexFormat = i32;
pub type CubeFace = i32;
pub type DataFormat = i32;
pub type PixelFormat = i32;
pub type TexFilter = i32;
pub type TimeStamp = u64;
pub type GLenum = u32;
pub type GLu32 = u32;
pub type GLint = i32;
pub type GLsizei = i32;
pub type PFNGLGENERATEMIPMAPPROC = Option<unsafe extern "C" fn(GLenum) -> ()>;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Face {
    pub face: CubeFace,
    pub look: Vec3,
    pub up: Vec3,
}

static mut kFaces: [Face; 6] = [
    Face {
        face: 0x8515_i32,
        look: Vec3::new(1.0f32, 0.0f32, 0.0f32),
        up: Vec3::new(0.0f32, 1.0f32, 0.0f32),
    },
    Face {
        face: 0x8516_i32,
        look: Vec3::new(-1.0f32, 0.0f32, 0.0f32),
        up: Vec3::new(0.0f32, 1.0f32, 0.0f32),
    },
    Face {
        face: 0x8517_i32,
        look: Vec3::new(0.0f32, 1.0f32, 0.0f32),
        up: Vec3::new(0.0f32, 0.0f32, -1.0f32),
    },
    Face {
        face: 0x8518_i32,
        look: Vec3::new(0.0f32, -1.0f32, 0.0f32),
        up: Vec3::new(0.0f32, 0.0f32, 1.0f32),
    },
    Face {
        face: 0x8519_i32,
        look: Vec3::new(0.0f32, 0.0f32, 1.0f32),
        up: Vec3::new(0.0f32, 1.0f32, 0.0f32),
    },
    Face {
        face: 0x851a_i32,
        look: Vec3::new(0.0f32, 0.0f32, -1.0f32),
        up: Vec3::new(0.0f32, 1.0f32, 0.0f32),
    },
];

static mut kFaceExt: [*const libc::c_char; 6] = [
    b"px\0" as *const u8 as *const libc::c_char,
    b"py\0" as *const u8 as *const libc::c_char,
    b"pz\0" as *const u8 as *const libc::c_char,
    b"nx\0" as *const u8 as *const libc::c_char,
    b"ny\0" as *const u8 as *const libc::c_char,
    b"nz\0" as *const u8 as *const libc::c_char,
];

#[inline]
unsafe extern "C" fn TexCube_InitParameters() {
    gl::TexParameteri(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
    gl::TexParameteri(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
    gl::TexParameteri(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
    gl::TexParameteri(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
}

#[no_mangle]
pub unsafe extern "C" fn TexCube_Create(mut size: i32, mut format: TexFormat) -> *mut TexCube {
    if !TexFormat_IsValid(format) {
        Fatal(
            b"TexCube_Create: Invalid texture format requested\0" as *const u8
                as *const libc::c_char,
        );
    }
    if TexFormat_IsDepth(format) {
        Fatal(
            b"TexCube_Create: Cannot create cubemap with depth format\0" as *const u8
                as *const libc::c_char,
        );
    }

    let mut this: *mut TexCube = MemAlloc(::core::mem::size_of::<TexCube>()) as *mut TexCube;
    (*this)._refCount = 1_i32 as u32;
    gl::GenTextures(1_i32, &mut (*this).handle);
    (*this).size = size;
    (*this).format = format;
    gl::BindTexture(gl::TEXTURE_CUBE_MAP, (*this).handle);
    gl::TexImage2D(
        gl::TEXTURE_CUBE_MAP_POSITIVE_X,
        0_i32,
        format,
        size,
        size,
        0_i32,
        gl::RED,
        gl::BYTE,
        std::ptr::null(),
    );
    gl::TexImage2D(
        gl::TEXTURE_CUBE_MAP_POSITIVE_Y,
        0_i32,
        format,
        size,
        size,
        0_i32,
        gl::RED,
        gl::BYTE,
        std::ptr::null(),
    );
    gl::TexImage2D(
        gl::TEXTURE_CUBE_MAP_POSITIVE_Z,
        0_i32,
        format,
        size,
        size,
        0_i32,
        gl::RED,
        gl::BYTE,
        std::ptr::null(),
    );
    gl::TexImage2D(
        gl::TEXTURE_CUBE_MAP_NEGATIVE_X,
        0_i32,
        format,
        size,
        size,
        0_i32,
        gl::RED,
        gl::BYTE,
        std::ptr::null(),
    );
    gl::TexImage2D(
        gl::TEXTURE_CUBE_MAP_NEGATIVE_Y,
        0_i32,
        format,
        size,
        size,
        0_i32,
        gl::RED,
        gl::BYTE,
        std::ptr::null(),
    );
    gl::TexImage2D(
        gl::TEXTURE_CUBE_MAP_NEGATIVE_Z,
        0_i32,
        format,
        size,
        size,
        0_i32,
        gl::RED,
        gl::BYTE,
        std::ptr::null(),
    );
    TexCube_InitParameters();
    gl::BindTexture(gl::TEXTURE_CUBE_MAP, 0);
    this
}

#[no_mangle]
pub unsafe extern "C" fn TexCube_Acquire(mut this: *mut TexCube) {
    (*this)._refCount = ((*this)._refCount).wrapping_add(1);
}

#[no_mangle]
pub unsafe extern "C" fn TexCube_Clear(
    mut this: *mut TexCube,
    r: f32,
    g: f32,
    b: f32,
    a: f32,
) {
    for i in 0..6 {
        let mut face: Face = kFaces[i as usize];
        RenderTarget_Push((*this).size, (*this).size);
        RenderTarget_BindTexCube(this, face.face);
        Draw_Clear(r, g, b, a);
        RenderTarget_Pop();
    }
}

#[no_mangle]
pub unsafe extern "C" fn TexCube_Free(mut this: *mut TexCube) {
    if !this.is_null() && {
        (*this)._refCount = ((*this)._refCount).wrapping_sub(1);
        (*this)._refCount <= 0_i32 as u32
    } {
        gl::DeleteTextures(1_i32, &mut (*this).handle);
        MemFree(this as *const libc::c_void);
    }
}

#[no_mangle]
pub unsafe extern "C" fn TexCube_Load(mut path: *const libc::c_char) -> *mut TexCube {
    let mut this: *mut TexCube = MemAlloc(::core::mem::size_of::<TexCube>()) as *mut TexCube;
    gl::GenTextures(1_i32, &mut (*this).handle);
    gl::BindTexture(gl::TEXTURE_CUBE_MAP, (*this).handle);

    let mut components: i32 = 0_i32;
    let mut dataLayout: i32 = 0_i32;

    for i in 0..6 {
        let mut facePath: *const libc::c_char = StrAdd3(
            path,
            kFaceExt[i as usize],
            b".jpg\0" as *const u8 as *const libc::c_char,
        );
        let mut sx: i32 = 0;
        let mut sy: i32 = 0;
        let mut lcomponents: i32 = 0;
        let mut data: *mut uchar = Tex2D_LoadRaw(facePath, &mut sx, &mut sy, &mut lcomponents);
        if data.is_null() {
            Fatal(
                b"TexCube_Load failed to load cubemap face from '%s'\0" as *const u8
                    as *const libc::c_char,
                facePath,
            );
        }
        if sx != sy {
            Fatal(
                b"TexCube_Load loaded cubemap face is not square\0" as *const u8
                    as *const libc::c_char,
            );
        }
        if i != 0 {
            if sx != (*this).size || sy != (*this).size {
                Fatal(
                    b"TexCube_Load loaded cubemap faces have different resolutions\0" as *const u8
                        as *const libc::c_char,
                );
            }
            if lcomponents != components {
                Fatal(
                    b"TexCube_Load loaded cubemap faces have different number of components\0"
                        as *const u8 as *const libc::c_char,
                );
            }
        } else {
            components = lcomponents;
            (*this).size = sx;
            (*this).format = if components == 4_i32 {
                TexFormat_RGBA8
            } else if components == 3_i32 {
                TexFormat_RGB8
            } else if components == 2_i32 {
                TexFormat_RG8
            } else {
                TexFormat_R8
            };
            dataLayout = if components == 4_i32 {
                gl::RGBA
            } else if components == 3_i32 {
                gl::RGB
            } else if components == 2_i32 {
                gl::RG
            } else {
                gl::RED
            } as i32;
        }

        gl::TexImage2D(
            kFaces[i as usize].face as GLenum,
            0_i32,
            (*this).format,
            (*this).size,
            (*this).size,
            0_i32,
            dataLayout as GLenum,
            gl::UNSIGNED_BYTE,
            data as *const libc::c_void,
        );
        MemFree(facePath as *const libc::c_void);
        MemFree(data as *const libc::c_void);
    }

    TexCube_InitParameters();
    gl::BindTexture(gl::TEXTURE_CUBE_MAP, 0);
    this
}

#[no_mangle]
pub unsafe extern "C" fn TexCube_GetData(
    mut this: *mut TexCube,
    mut data: *mut libc::c_void,
    mut face: CubeFace,
    mut level: i32,
    mut pf: PixelFormat,
    mut df: DataFormat,
) {
    gl::BindTexture(gl::TEXTURE_CUBE_MAP, (*this).handle);
    gl::GetTexImage(face as GLenum, level, pf as GLenum, df as GLenum, data);
    gl::BindTexture(gl::TEXTURE_CUBE_MAP, 0);
}

#[no_mangle]
pub unsafe extern "C" fn TexCube_GetDataBytes(
    mut this: *mut TexCube,
    mut face: CubeFace,
    mut level: i32,
    mut pf: PixelFormat,
    mut df: DataFormat,
) -> *mut Bytes {
    let mut size: i32 = (*this).size * (*this).size;
    size *= DataFormat_GetSize(df);
    size *= PixelFormat_Components(pf);
    let mut data: *mut Bytes = Bytes_Create(size as u32);
    TexCube_GetData(this, Bytes_GetData(data), face, level, pf, df);
    Bytes_Rewind(data);
    data
}

#[no_mangle]
pub unsafe extern "C" fn TexCube_GetFormat(this: *mut TexCube) -> TexFormat {
    (*this).format
}

#[no_mangle]
pub unsafe extern "C" fn TexCube_GetHandle(this: *mut TexCube) -> u32 {
    (*this).handle
}

#[no_mangle]
pub unsafe extern "C" fn TexCube_GetSize(this: *mut TexCube) -> i32 {
    (*this).size
}

#[no_mangle]
pub unsafe extern "C" fn TexCube_Generate(mut this: *mut TexCube, mut state: *mut ShaderState) {
    GLMatrix_ModeP();
    GLMatrix_Push();
    GLMatrix_Clear();
    GLMatrix_ModeWV();
    GLMatrix_Push();
    GLMatrix_Clear();
    RenderState_PushAllDefaults();
    ShaderState_Start(state);

    for i in 0..6 {
        let mut face: Face = kFaces[i as usize];
        let mut size: i32 = (*this).size;
        let mut fSize: f32 = (*this).size as f32;
        RenderTarget_Push(size, size);
        RenderTarget_BindTexCube(this, face.face);
        Draw_Clear(0.0f32, 0.0f32, 0.0f32, 1.0f32);
        Shader_SetFloat3(
            b"cubeLook\0" as *const u8 as *const libc::c_char,
            face.look.x,
            face.look.y,
            face.look.z,
        );
        Shader_SetFloat3(
            b"cubeUp\0" as *const u8 as *const libc::c_char,
            face.up.x,
            face.up.y,
            face.up.z,
        );
        Shader_SetFloat(b"cubeSize\0" as *const u8 as *const libc::c_char, fSize);

        let mut j: i32 = 1_i32;
        let mut jobSize: i32 = 1_i32;
        while j <= size {
            let mut time: TimeStamp = TimeStamp_Get();
            ClipRect_Push(0.0f32, (j - 1_i32) as f32, size as f32, jobSize as f32);
            Draw_Rect(0.0f32, 0.0f32, fSize, fSize);
            Draw_Flush();
            ClipRect_Pop();

            j += jobSize;
            let mut elapsed: f64 = TimeStamp_GetElapsed(time);
            jobSize = f64::max(
                1_f64,
                f64::floor(0.25f64 * jobSize as f64 / elapsed + 0.5f64) as i32 as f64,
            ) as i32;
            jobSize = f64::min(jobSize as f64, (size - j + 1_i32) as f64) as i32;
        }

        RenderTarget_Pop();
    }

    ShaderState_Stop(state);
    RenderState_PopAll();
    GLMatrix_ModeP();
    GLMatrix_Pop();
    GLMatrix_ModeWV();
    GLMatrix_Pop();
}

#[no_mangle]
pub unsafe extern "C" fn TexCube_GenMipmap(mut this: *mut TexCube) {
    gl::BindTexture(gl::TEXTURE_CUBE_MAP, (*this).handle);
    __glewGenerateMipmap.expect("non-null function pointer")(gl::TEXTURE_CUBE_MAP);
    gl::BindTexture(gl::TEXTURE_CUBE_MAP, 0);
}

#[no_mangle]
pub unsafe extern "C" fn TexCube_SetData(
    mut this: *mut TexCube,
    mut data: *const libc::c_void,
    mut face: CubeFace,
    mut level: i32,
    mut pf: PixelFormat,
    mut df: DataFormat,
) {
    gl::BindTexture(gl::TEXTURE_CUBE_MAP, (*this).handle);
    gl::TexImage2D(
        face as GLenum,
        level,
        (*this).format,
        (*this).size,
        (*this).size,
        0_i32,
        pf as GLenum,
        df as GLenum,
        data,
    );
    gl::BindTexture(gl::TEXTURE_CUBE_MAP, 0);
}

#[no_mangle]
pub unsafe extern "C" fn TexCube_SetDataBytes(
    mut this: *mut TexCube,
    mut data: *mut Bytes,
    mut face: CubeFace,
    mut level: i32,
    mut pf: PixelFormat,
    mut df: DataFormat,
) {
    TexCube_SetData(this, Bytes_GetData(data), face, level, pf, df);
}

#[no_mangle]
pub unsafe extern "C" fn TexCube_SetMagFilter(mut this: *mut TexCube, mut filter: TexFilter) {
    gl::BindTexture(gl::TEXTURE_CUBE_MAP, (*this).handle);
    gl::TexParameteri(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_MAG_FILTER, filter);
    gl::BindTexture(gl::TEXTURE_CUBE_MAP, 0);
}

#[no_mangle]
pub unsafe extern "C" fn TexCube_SetMinFilter(mut this: *mut TexCube, mut filter: TexFilter) {
    gl::BindTexture(gl::TEXTURE_CUBE_MAP, (*this).handle);
    gl::TexParameteri(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_MIN_FILTER, filter);
    gl::BindTexture(gl::TEXTURE_CUBE_MAP, 0);
}

#[no_mangle]
pub unsafe extern "C" fn TexCube_Save(mut this: *mut TexCube, mut path: *const libc::c_char) {
    TexCube_SaveLevel(this, path, 0);
}

#[no_mangle]
pub unsafe extern "C" fn TexCube_SaveLevel(
    mut this: *mut TexCube,
    mut path: *const libc::c_char,
    mut level: i32,
) {
    let mut size: i32 = (*this).size >> level;
    gl::BindTexture(gl::TEXTURE_CUBE_MAP, (*this).handle);
    let mut buffer: *mut uchar =
        MemAlloc((::core::mem::size_of::<uchar>()).wrapping_mul((4_i32 * size * size) as usize))
            as *mut uchar;
    for i in 0..6 {
        let mut face: CubeFace = kFaces[i as usize].face;
        let mut facePath: *const libc::c_char = StrAdd3(
            path,
            kFaceExt[i as usize],
            b".png\0" as *const u8 as *const libc::c_char,
        );
        gl::GetTexImage(
            face as GLenum,
            level,
            gl::RGBA,
            gl::UNSIGNED_BYTE,
            buffer as *mut libc::c_void,
        );
        Tex2D_Save_Png(facePath, size, size, 4_i32, buffer);
        MemFree(facePath as *const libc::c_void);
    }
    MemFree(buffer as *const libc::c_void);
    gl::BindTexture(gl::TEXTURE_CUBE_MAP, 0);
}
