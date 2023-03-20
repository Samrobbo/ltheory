use crate::internal::Memory::*;
use crate::Common::*;
use crate::Math::{Vec2, Vec3, Vec4};
use crate::Matrix::*;
use crate::Shader::*;
use crate::Tex1D::*;
use crate::Tex2D::*;
use crate::Tex3D::*;
use crate::TexCube::*;
use crate::GL::gl;
use libc;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ShaderState {
    pub _refCount: u32,
    pub shader: *mut Shader,
    pub elems_size: i32,
    pub elems_capacity: i32,
    pub elems_data: *mut Elem,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Elem {
    pub type_0: u32,
    pub index: i32,
    pub data: C2RustUnnamed,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub asFloat: f32,
    pub asFloat2: Vec2,
    pub asFloat3: Vec3,
    pub asFloat4: Vec4,
    pub asInt: i32,
    pub asMatrix: *mut Matrix,
    pub asTex1D: *mut Tex1D,
    pub asTex2D: *mut Tex2D,
    pub asTex3D: *mut Tex3D,
    pub asTexCube: *mut TexCube,
}

#[no_mangle]
pub static ElemType_Float: u32 = 1;

#[no_mangle]
pub static ElemType_Float2: u32 = 2;

#[no_mangle]
pub static ElemType_Float3: u32 = 3;

#[no_mangle]
pub static ElemType_Float4: u32 = 4;

#[no_mangle]
pub static ElemType_Int: u32 = 5;

#[no_mangle]
pub static ElemType_Matrix: u32 = 6;

#[no_mangle]
pub static ElemType_Tex1D: u32 = 7;

#[no_mangle]
pub static ElemType_Tex2D: u32 = 8;

#[no_mangle]
pub static ElemType_Tex3D: u32 = 9;

#[no_mangle]
pub static ElemType_TexCube: u32 = 10;

#[no_mangle]
pub unsafe extern "C" fn ShaderState_Create(mut shader: *mut Shader) -> *mut ShaderState {
    let mut this = MemNew!(ShaderState);
    (*this)._refCount = 1;
    (*this).elems_capacity = 0;
    (*this).elems_size = 0;
    (*this).elems_data = std::ptr::null_mut();
    Shader_Acquire(shader);
    (*this).shader = shader;
    this
}

#[no_mangle]
pub unsafe extern "C" fn ShaderState_Acquire(mut this: *mut ShaderState) {
    (*this)._refCount = ((*this)._refCount).wrapping_add(1);
}

#[no_mangle]
pub unsafe extern "C" fn ShaderState_Free(mut this: *mut ShaderState) {
    if !this.is_null() && {
        (*this)._refCount = ((*this)._refCount).wrapping_sub(1);
        (*this)._refCount <= 0
    } {
        let mut e: *mut Elem = (*this).elems_data;
        let mut __iterend: *mut Elem = ((*this).elems_data).offset((*this).elems_size as isize);
        while e < __iterend {
            match (*e).type_0 {
                7 => {
                    Tex1D_Free((*e).data.asTex1D);
                }
                8 => {
                    Tex2D_Free((*e).data.asTex2D);
                }
                9 => {
                    Tex3D_Free((*e).data.asTex3D);
                }
                10 => {
                    TexCube_Free((*e).data.asTexCube);
                }
                _ => {}
            }
            e = e.offset(1);
        }
        Shader_Free((*this).shader);
        MemFree((*this).elems_data as *const _);
        MemFree(this as *const _);
    }
}

#[no_mangle]
pub unsafe extern "C" fn ShaderState_FromShaderLoad(
    mut vertName: *const libc::c_char,
    mut fragName: *const libc::c_char,
) -> *mut ShaderState {
    let mut shader: *mut Shader = Shader_Load(vertName, fragName);
    let mut this: *mut ShaderState = ShaderState_Create(shader);
    Shader_Free(shader);
    this
}

#[no_mangle]
pub unsafe extern "C" fn ShaderState_SetFloat(
    mut this: *mut ShaderState,
    mut name: *const libc::c_char,
    mut x: f32,
) {
    let mut elem: Elem = Elem {
        type_0: ElemType_Float,
        index: Shader_GetVariable((*this).shader, name),
        data: C2RustUnnamed { asFloat: 0. },
    };
    elem.data.asFloat = x;
    if ((*this).elems_capacity == (*this).elems_size) as libc::c_long != 0 {
        (*this).elems_capacity = if (*this).elems_capacity != 0 {
            (*this).elems_capacity * 2
        } else {
            1
        };
        let mut elemSize: usize = std::mem::size_of::<Elem>();
        let mut pData: *mut *mut libc::c_void =
            &mut (*this).elems_data as *mut *mut Elem as *mut *mut libc::c_void;
        *pData = MemRealloc(
            (*this).elems_data as *mut _,
            ((*this).elems_capacity as usize).wrapping_mul(elemSize),
        );
    }
    let fresh0 = (*this).elems_size;
    (*this).elems_size += 1;
    *((*this).elems_data).offset(fresh0 as isize) = elem;
}

#[no_mangle]
pub unsafe extern "C" fn ShaderState_SetFloat2(
    mut this: *mut ShaderState,
    mut name: *const libc::c_char,
    mut x: f32,
    mut y: f32,
) {
    let mut elem: Elem = Elem {
        type_0: ElemType_Float2,
        index: Shader_GetVariable((*this).shader, name),
        data: C2RustUnnamed { asFloat: 0. },
    };
    elem.data.asFloat2 = Vec2::new(x, y);
    if ((*this).elems_capacity == (*this).elems_size) as libc::c_long != 0 {
        (*this).elems_capacity = if (*this).elems_capacity != 0 {
            (*this).elems_capacity * 2
        } else {
            1
        };
        let mut elemSize: usize = std::mem::size_of::<Elem>();
        let mut pData: *mut *mut libc::c_void =
            &mut (*this).elems_data as *mut *mut Elem as *mut *mut libc::c_void;
        *pData = MemRealloc(
            (*this).elems_data as *mut _,
            ((*this).elems_capacity as usize).wrapping_mul(elemSize),
        );
    }
    let fresh1 = (*this).elems_size;
    (*this).elems_size += 1;
    *((*this).elems_data).offset(fresh1 as isize) = elem;
}

#[no_mangle]
pub unsafe extern "C" fn ShaderState_SetFloat3(
    mut this: *mut ShaderState,
    mut name: *const libc::c_char,
    mut x: f32,
    mut y: f32,
    mut z: f32,
) {
    let mut elem: Elem = Elem {
        type_0: ElemType_Float3,
        index: Shader_GetVariable((*this).shader, name),
        data: C2RustUnnamed { asFloat: 0. },
    };
    elem.data.asFloat3 = Vec3::new(x, y, z);
    if ((*this).elems_capacity == (*this).elems_size) as libc::c_long != 0 {
        (*this).elems_capacity = if (*this).elems_capacity != 0 {
            (*this).elems_capacity * 2
        } else {
            1
        };
        let mut elemSize: usize = std::mem::size_of::<Elem>();
        let mut pData: *mut *mut libc::c_void =
            &mut (*this).elems_data as *mut *mut Elem as *mut *mut libc::c_void;
        *pData = MemRealloc(
            (*this).elems_data as *mut _,
            ((*this).elems_capacity as usize).wrapping_mul(elemSize),
        );
    }
    let fresh2 = (*this).elems_size;
    (*this).elems_size += 1;
    *((*this).elems_data).offset(fresh2 as isize) = elem;
}

#[no_mangle]
pub unsafe extern "C" fn ShaderState_SetFloat4(
    mut this: *mut ShaderState,
    mut name: *const libc::c_char,
    mut x: f32,
    mut y: f32,
    mut z: f32,
    mut w: f32,
) {
    let mut elem: Elem = Elem {
        type_0: ElemType_Float4,
        index: Shader_GetVariable((*this).shader, name),
        data: C2RustUnnamed { asFloat: 0. },
    };
    elem.data.asFloat4 = Vec4::new(x, y, z, w);
    if ((*this).elems_capacity == (*this).elems_size) as libc::c_long != 0 {
        (*this).elems_capacity = if (*this).elems_capacity != 0 {
            (*this).elems_capacity * 2
        } else {
            1
        };
        let mut elemSize: usize = std::mem::size_of::<Elem>();
        let mut pData: *mut *mut libc::c_void =
            &mut (*this).elems_data as *mut *mut Elem as *mut *mut libc::c_void;
        *pData = MemRealloc(
            (*this).elems_data as *mut _,
            ((*this).elems_capacity as usize).wrapping_mul(elemSize),
        );
    }
    let fresh3 = (*this).elems_size;
    (*this).elems_size += 1;
    *((*this).elems_data).offset(fresh3 as isize) = elem;
}

#[no_mangle]
pub unsafe extern "C" fn ShaderState_SetInt(
    mut this: *mut ShaderState,
    mut name: *const libc::c_char,
    mut x: i32,
) {
    let mut elem: Elem = Elem {
        type_0: ElemType_Int,
        index: Shader_GetVariable((*this).shader, name),
        data: C2RustUnnamed { asFloat: 0. },
    };
    elem.data.asInt = x;
    if ((*this).elems_capacity == (*this).elems_size) as libc::c_long != 0 {
        (*this).elems_capacity = if (*this).elems_capacity != 0 {
            (*this).elems_capacity * 2
        } else {
            1
        };
        let mut elemSize: usize = std::mem::size_of::<Elem>();
        let mut pData: *mut *mut libc::c_void =
            &mut (*this).elems_data as *mut *mut Elem as *mut *mut libc::c_void;
        *pData = MemRealloc(
            (*this).elems_data as *mut _,
            ((*this).elems_capacity as usize).wrapping_mul(elemSize),
        );
    }
    let fresh4 = (*this).elems_size;
    (*this).elems_size += 1;
    *((*this).elems_data).offset(fresh4 as isize) = elem;
}

#[no_mangle]
pub unsafe extern "C" fn ShaderState_SetMatrix(
    mut this: *mut ShaderState,
    mut name: *const libc::c_char,
    mut x: *mut Matrix,
) {
    let mut elem: Elem = Elem {
        type_0: ElemType_Matrix,
        index: Shader_GetVariable((*this).shader, name),
        data: C2RustUnnamed { asFloat: 0. },
    };
    elem.data.asMatrix = x;
    if ((*this).elems_capacity == (*this).elems_size) as libc::c_long != 0 {
        (*this).elems_capacity = if (*this).elems_capacity != 0 {
            (*this).elems_capacity * 2
        } else {
            1
        };
        let mut elemSize: usize = std::mem::size_of::<Elem>();
        let mut pData: *mut *mut libc::c_void =
            &mut (*this).elems_data as *mut *mut Elem as *mut *mut libc::c_void;
        *pData = MemRealloc(
            (*this).elems_data as *mut _,
            ((*this).elems_capacity as usize).wrapping_mul(elemSize),
        );
    }
    let fresh5 = (*this).elems_size;
    (*this).elems_size += 1;
    *((*this).elems_data).offset(fresh5 as isize) = elem;
}

#[no_mangle]
pub unsafe extern "C" fn ShaderState_SetTex1D(
    mut this: *mut ShaderState,
    mut name: *const libc::c_char,
    mut x: *mut Tex1D,
) {
    Tex1D_Acquire(x);
    let mut elem: Elem = Elem {
        type_0: ElemType_Tex1D,
        index: Shader_GetVariable((*this).shader, name),
        data: C2RustUnnamed { asFloat: 0. },
    };
    elem.data.asTex1D = x;
    if ((*this).elems_capacity == (*this).elems_size) as libc::c_long != 0 {
        (*this).elems_capacity = if (*this).elems_capacity != 0 {
            (*this).elems_capacity * 2
        } else {
            1
        };
        let mut elemSize: usize = std::mem::size_of::<Elem>();
        let mut pData: *mut *mut libc::c_void =
            &mut (*this).elems_data as *mut *mut Elem as *mut *mut libc::c_void;
        *pData = MemRealloc(
            (*this).elems_data as *mut _,
            ((*this).elems_capacity as usize).wrapping_mul(elemSize),
        );
    }
    let fresh6 = (*this).elems_size;
    (*this).elems_size += 1;
    *((*this).elems_data).offset(fresh6 as isize) = elem;
}

#[no_mangle]
pub unsafe extern "C" fn ShaderState_SetTex2D(
    mut this: *mut ShaderState,
    mut name: *const libc::c_char,
    mut x: *mut Tex2D,
) {
    Tex2D_Acquire(x);
    let mut elem: Elem = Elem {
        type_0: ElemType_Tex2D,
        index: Shader_GetVariable((*this).shader, name),
        data: C2RustUnnamed { asFloat: 0. },
    };
    elem.data.asTex2D = x;
    if ((*this).elems_capacity == (*this).elems_size) as libc::c_long != 0 {
        (*this).elems_capacity = if (*this).elems_capacity != 0 {
            (*this).elems_capacity * 2
        } else {
            1
        };
        let mut elemSize: usize = std::mem::size_of::<Elem>();
        let mut pData: *mut *mut libc::c_void =
            &mut (*this).elems_data as *mut *mut Elem as *mut *mut libc::c_void;
        *pData = MemRealloc(
            (*this).elems_data as *mut _,
            ((*this).elems_capacity as usize).wrapping_mul(elemSize),
        );
    }
    let fresh7 = (*this).elems_size;
    (*this).elems_size += 1;
    *((*this).elems_data).offset(fresh7 as isize) = elem;
}

#[no_mangle]
pub unsafe extern "C" fn ShaderState_SetTex3D(
    mut this: *mut ShaderState,
    mut name: *const libc::c_char,
    mut x: *mut Tex3D,
) {
    Tex3D_Acquire(x);
    let mut elem: Elem = Elem {
        type_0: ElemType_Tex3D,
        index: Shader_GetVariable((*this).shader, name),
        data: C2RustUnnamed { asFloat: 0. },
    };
    elem.data.asTex3D = x;
    if ((*this).elems_capacity == (*this).elems_size) as libc::c_long != 0 {
        (*this).elems_capacity = if (*this).elems_capacity != 0 {
            (*this).elems_capacity * 2
        } else {
            1
        };
        let mut elemSize: usize = std::mem::size_of::<Elem>();
        let mut pData: *mut *mut libc::c_void =
            &mut (*this).elems_data as *mut *mut Elem as *mut *mut libc::c_void;
        *pData = MemRealloc(
            (*this).elems_data as *mut _,
            ((*this).elems_capacity as usize).wrapping_mul(elemSize),
        );
    }
    let fresh8 = (*this).elems_size;
    (*this).elems_size += 1;
    *((*this).elems_data).offset(fresh8 as isize) = elem;
}

#[no_mangle]
pub unsafe extern "C" fn ShaderState_SetTexCube(
    mut this: *mut ShaderState,
    mut name: *const libc::c_char,
    mut x: *mut TexCube,
) {
    TexCube_Acquire(x);
    let mut elem: Elem = Elem {
        type_0: ElemType_TexCube,
        index: Shader_GetVariable((*this).shader, name),
        data: C2RustUnnamed { asFloat: 0. },
    };
    elem.data.asTexCube = x;
    if ((*this).elems_capacity == (*this).elems_size) as libc::c_long != 0 {
        (*this).elems_capacity = if (*this).elems_capacity != 0 {
            (*this).elems_capacity * 2
        } else {
            1
        };
        let mut elemSize: usize = std::mem::size_of::<Elem>();
        let mut pData: *mut *mut libc::c_void =
            &mut (*this).elems_data as *mut *mut Elem as *mut *mut libc::c_void;
        *pData = MemRealloc(
            (*this).elems_data as *mut _,
            ((*this).elems_capacity as usize).wrapping_mul(elemSize),
        );
    }
    let fresh9 = (*this).elems_size;
    (*this).elems_size += 1;
    *((*this).elems_data).offset(fresh9 as isize) = elem;
}

#[no_mangle]
pub unsafe extern "C" fn ShaderState_Start(mut this: *mut ShaderState) {
    Shader_Start((*this).shader);
    let mut e: *mut Elem = (*this).elems_data;
    let mut __iterend: *mut Elem = ((*this).elems_data).offset((*this).elems_size as isize);
    while e < __iterend {
        match (*e).type_0 {
            1 => {
                gl::Uniform1f((*e).index, (*e).data.asFloat);
            }
            2 => {
                gl::Uniform2f((*e).index, (*e).data.asFloat2.x, (*e).data.asFloat2.y);
            }
            3 => {
                gl::Uniform3f(
                    (*e).index,
                    (*e).data.asFloat3.x,
                    (*e).data.asFloat3.y,
                    (*e).data.asFloat3.z,
                );
            }
            4 => {
                gl::Uniform4f(
                    (*e).index,
                    (*e).data.asFloat4.x,
                    (*e).data.asFloat4.y,
                    (*e).data.asFloat4.z,
                    (*e).data.asFloat4.w,
                );
            }
            5 => {
                gl::Uniform1i((*e).index, (*e).data.asInt);
            }
            6 => {
                Shader_ISetMatrix((*e).index, (*e).data.asMatrix);
            }
            7 => {
                Shader_ISetTex1D((*e).index, (*e).data.asTex1D);
            }
            8 => {
                Shader_ISetTex2D((*e).index, (*e).data.asTex2D);
            }
            9 => {
                Shader_ISetTex3D((*e).index, (*e).data.asTex3D);
            }
            10 => {
                Shader_ISetTexCube((*e).index, (*e).data.asTexCube);
            }
            _ => {
                Fatal(
                    b"ShaderState_Start: Encountered invalid opcode\0" as *const u8
                        as *const libc::c_char,
                );
            }
        }
        e = e.offset(1);
    }
}

#[no_mangle]
pub unsafe extern "C" fn ShaderState_Stop(mut this: *mut ShaderState) {
    Shader_Stop((*this).shader);
}
