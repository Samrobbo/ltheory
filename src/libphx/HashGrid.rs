use ::libc;
use super::internal::Memory::*;
extern "C" {
    pub type MemPool;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn floor(_: libc::c_double) -> libc::c_double;
    fn Hash_XX64(buf: *const libc::c_void, len: libc::c_int, seed: uint64) -> uint64;
    fn MemPool_Create(cellSize: uint32, blockSize: uint32) -> *mut MemPool;
    fn MemPool_Free(_: *mut MemPool);
    fn MemPool_Alloc(_: *mut MemPool) -> *mut libc::c_void;
    fn MemPool_Clear(_: *mut MemPool);
    fn MemPool_Dealloc(_: *mut MemPool, _: *mut libc::c_void);
    fn Profiler_Begin(_: cstr);
    fn Profiler_End();
}
pub type int32_t = libc::c_int;
pub type uint32_t = libc::c_uint;
pub type uint64_t = libc::c_ulonglong;
pub type __darwin_size_t = libc::c_ulong;
pub type size_t = __darwin_size_t;
pub type cstr = *const libc::c_char;
pub type int32 = int32_t;
pub type uint32 = uint32_t;
pub type uint64 = uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HashGrid {
    pub version: uint64,
    pub cells: *mut HashGridCell,
    pub elemPool: *mut MemPool,
    pub cellCount: uint32,
    pub cellSize: libc::c_float,
    pub mask: uint32,
    pub results_size: int32,
    pub results_capacity: int32,
    pub results_data: *mut *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HashGridCell {
    pub version: uint64,
    pub elems_size: int32,
    pub elems_capacity: int32,
    pub elems_data: *mut *mut HashGridElem,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HashGridElem {
    pub version: uint64,
    pub object: *mut libc::c_void,
    pub lower: [int32; 3],
    pub upper: [int32; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Box3f {
    pub lower: Vec3f,
    pub upper: Vec3f,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Vec3f {
    pub x: libc::c_float,
    pub y: libc::c_float,
    pub z: libc::c_float,
}



#[inline]
unsafe extern "C" fn Floor(mut t: libc::c_double) -> libc::c_double {
    return floor(t);
}
#[inline]
unsafe extern "C" fn Maxi(mut a: libc::c_int, mut b: libc::c_int) -> libc::c_int {
    return if a > b { a } else { b };
}
#[inline]
unsafe extern "C" fn Mini(mut a: libc::c_int, mut b: libc::c_int) -> libc::c_int {
    return if a < b { a } else { b };
}
#[no_mangle]
pub unsafe extern "C" fn HashGrid_Create(
    mut cellSize: libc::c_float,
    mut cellCount: uint32,
) -> *mut HashGrid {
    let mut logCount: uint32 = 0 as libc::c_int as uint32;
    while cellCount > 1 as libc::c_int as libc::c_uint {
        cellCount = (cellCount as libc::c_uint)
            .wrapping_div(2 as libc::c_int as libc::c_uint) as uint32 as uint32;
        logCount = logCount.wrapping_add(1);
    }
    cellCount = ((1 as libc::c_int) << logCount) as uint32;
    let mut self_0: *mut HashGrid = MemAlloc(
        ::core::mem::size_of::<HashGrid>() as usize,
    ) as *mut HashGrid;
    (*self_0).version = 0 as libc::c_int as uint64;
    (*self_0)
        .cells = MemAllocZero(
        (::core::mem::size_of::<HashGridCell>())
            .wrapping_mul(cellCount as usize),
    ) as *mut HashGridCell;
    (*self_0)
        .elemPool = MemPool_Create(
        ::core::mem::size_of::<HashGridElem>() as usize as uint32,
        (0x1000 as libc::c_uint as usize)
            .wrapping_div(::core::mem::size_of::<HashGridElem>())
            as uint32,
    );
    (*self_0).cellCount = cellCount;
    (*self_0).cellSize = cellSize;
    (*self_0).mask = (((1 as libc::c_int) << logCount) - 1 as libc::c_int) as uint32;
    (*self_0).results_capacity = 0 as libc::c_int;
    (*self_0).results_size = 0 as libc::c_int;
    (*self_0).results_data = 0 as *mut *mut libc::c_void;
    let mut i: uint32 = 0 as libc::c_int as uint32;
    while i < cellCount {
        (*((*self_0).cells).offset(i as isize)).elems_capacity = 0 as libc::c_int;
        (*((*self_0).cells).offset(i as isize)).elems_size = 0 as libc::c_int;
        let ref mut fresh0 = (*((*self_0).cells).offset(i as isize)).elems_data;
        *fresh0 = 0 as *mut *mut HashGridElem;
        i = i.wrapping_add(1);
    }
    return self_0;
}
#[no_mangle]
pub unsafe extern "C" fn HashGrid_Free(mut self_0: *mut HashGrid) {
    MemFree((*self_0).results_data as *const libc::c_void);
    let mut i: uint32 = 0 as libc::c_int as uint32;
    while i < (*self_0).cellCount {
        MemFree(
            (*((*self_0).cells).offset(i as isize)).elems_data as *const libc::c_void,
        );
        i = i.wrapping_add(1);
    }
    MemPool_Free((*self_0).elemPool);
    MemFree((*self_0).cells as *const libc::c_void);
    MemFree(self_0 as *const libc::c_void);
}
#[inline]
unsafe extern "C" fn HashGrid_GetCell(
    mut self_0: *mut HashGrid,
    mut x: int32,
    mut y: int32,
    mut z: int32,
) -> *mut HashGridCell {
    let mut p: [int32; 3] = [x, y, z];
    let mut hash: uint64 = Hash_XX64(
        p.as_mut_ptr() as *const libc::c_void,
        ::core::mem::size_of::<[int32; 3]>() as libc::c_ulong as libc::c_int,
        0 as libc::c_ulonglong,
    );
    return ((*self_0).cells)
        .offset((hash & (*self_0).mask as libc::c_ulonglong) as isize);
}
unsafe extern "C" fn HashGrid_AddElem(
    mut self_0: *mut HashGrid,
    mut elem: *mut HashGridElem,
) {
    (*self_0).version = ((*self_0).version).wrapping_add(1);
    let mut x: int32 = (*elem).lower[0 as libc::c_int as usize];
    while x <= (*elem).upper[0 as libc::c_int as usize] {
        let mut y: int32 = (*elem).lower[1 as libc::c_int as usize];
        while y <= (*elem).upper[1 as libc::c_int as usize] {
            let mut z: int32 = (*elem).lower[2 as libc::c_int as usize];
            while z <= (*elem).upper[2 as libc::c_int as usize] {
                let mut cell: *mut HashGridCell = HashGrid_GetCell(self_0, x, y, z);
                if (*cell).version != (*self_0).version {
                    (*cell).version = (*self_0).version;
                    if ((*cell).elems_capacity == (*cell).elems_size) as libc::c_int
                        as libc::c_long != 0
                    {
                        (*cell)
                            .elems_capacity = if (*cell).elems_capacity != 0 {
                            (*cell).elems_capacity * 2 as libc::c_int
                        } else {
                            1 as libc::c_int
                        };
                        let mut elemSize: usize = ::core::mem::size_of::<
                            *mut HashGridElem,
                        >();
                        let mut pData: *mut *mut libc::c_void = &mut (*cell).elems_data
                            as *mut *mut *mut HashGridElem as *mut *mut libc::c_void;
                        *pData = MemRealloc(
                            (*cell).elems_data as *mut libc::c_void,
                            ((*cell).elems_capacity as usize)
                                .wrapping_mul(elemSize),
                        );
                    }
                    let fresh1 = (*cell).elems_size;
                    (*cell).elems_size = (*cell).elems_size + 1;
                    let ref mut fresh2 = *((*cell).elems_data).offset(fresh1 as isize);
                    *fresh2 = elem;
                }
                z += 1;
            }
            y += 1;
        }
        x += 1;
    }
}
unsafe extern "C" fn HashGrid_RemoveElem(
    mut self_0: *mut HashGrid,
    mut elem: *mut HashGridElem,
) {
    (*self_0).version = ((*self_0).version).wrapping_add(1);
    let mut x: int32 = (*elem).lower[0 as libc::c_int as usize];
    while x <= (*elem).upper[0 as libc::c_int as usize] {
        let mut y: int32 = (*elem).lower[1 as libc::c_int as usize];
        while y <= (*elem).upper[1 as libc::c_int as usize] {
            let mut z: int32 = (*elem).lower[2 as libc::c_int as usize];
            while z <= (*elem).upper[2 as libc::c_int as usize] {
                let mut cell: *mut HashGridCell = HashGrid_GetCell(self_0, x, y, z);
                if (*cell).version != (*self_0).version {
                    (*cell).version = (*self_0).version;
                    let mut _i: int32 = 0 as libc::c_int;
                    while _i < (*cell).elems_size {
                        if (*((*cell).elems_data).offset(_i as isize) == elem)
                            as libc::c_int as libc::c_long != 0
                        {
                            (*cell).elems_size -= 1;
                            let ref mut fresh3 = *((*cell).elems_data)
                                .offset(_i as isize);
                            *fresh3 = *((*cell).elems_data)
                                .offset((*cell).elems_size as isize);
                            break;
                        } else {
                            _i += 1;
                        }
                    }
                }
                z += 1;
            }
            y += 1;
        }
        x += 1;
    }
}
#[inline]
unsafe extern "C" fn HashGrid_ToLocal(
    mut self_0: *mut HashGrid,
    mut x: libc::c_float,
) -> int32 {
    return Floor((x / (*self_0).cellSize) as libc::c_double) as int32;
}
#[no_mangle]
pub unsafe extern "C" fn HashGrid_Add(
    mut self_0: *mut HashGrid,
    mut object: *mut libc::c_void,
    mut box_0: *const Box3f,
) -> *mut HashGridElem {
    let mut elem: *mut HashGridElem = MemPool_Alloc((*self_0).elemPool)
        as *mut HashGridElem;
    (*elem).object = object;
    (*elem)
        .lower[0 as libc::c_int as usize] = HashGrid_ToLocal(self_0, (*box_0).lower.x);
    (*elem)
        .lower[1 as libc::c_int as usize] = HashGrid_ToLocal(self_0, (*box_0).lower.y);
    (*elem)
        .lower[2 as libc::c_int as usize] = HashGrid_ToLocal(self_0, (*box_0).lower.z);
    (*elem)
        .upper[0 as libc::c_int as usize] = HashGrid_ToLocal(self_0, (*box_0).upper.x);
    (*elem)
        .upper[1 as libc::c_int as usize] = HashGrid_ToLocal(self_0, (*box_0).upper.y);
    (*elem)
        .upper[2 as libc::c_int as usize] = HashGrid_ToLocal(self_0, (*box_0).upper.z);
    HashGrid_AddElem(self_0, elem);
    return elem;
}
#[no_mangle]
pub unsafe extern "C" fn HashGrid_Clear(mut self_0: *mut HashGrid) {
    (*self_0).version = 0 as libc::c_int as uint64;
    let mut i: uint32 = 0 as libc::c_int as uint32;
    while i < (*self_0).cellCount {
        (*((*self_0).cells).offset(i as isize)).elems_size = 0 as libc::c_int;
        (*((*self_0).cells).offset(i as isize)).version = 0 as libc::c_int as uint64;
        i = i.wrapping_add(1);
    }
    MemPool_Clear((*self_0).elemPool);
    (*self_0).results_size = 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn HashGrid_Remove(
    mut self_0: *mut HashGrid,
    mut elem: *mut HashGridElem,
) {
    HashGrid_RemoveElem(self_0, elem);
    MemPool_Dealloc((*self_0).elemPool, elem as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn HashGrid_Update(
    mut self_0: *mut HashGrid,
    mut elem: *mut HashGridElem,
    mut box_0: *const Box3f,
) {
    Profiler_Begin(
        (*::core::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"HashGrid_Update\0"))
            .as_ptr(),
    );
    let mut lower: [int32; 3] = [
        HashGrid_ToLocal(self_0, (*box_0).lower.x),
        HashGrid_ToLocal(self_0, (*box_0).lower.y),
        HashGrid_ToLocal(self_0, (*box_0).lower.z),
    ];
    let mut upper: [int32; 3] = [
        HashGrid_ToLocal(self_0, (*box_0).upper.x),
        HashGrid_ToLocal(self_0, (*box_0).upper.y),
        HashGrid_ToLocal(self_0, (*box_0).upper.z),
    ];
    if lower[0 as libc::c_int as usize] == (*elem).lower[0 as libc::c_int as usize]
        && upper[0 as libc::c_int as usize] == (*elem).upper[0 as libc::c_int as usize]
        && lower[1 as libc::c_int as usize] == (*elem).lower[1 as libc::c_int as usize]
        && upper[1 as libc::c_int as usize] == (*elem).upper[1 as libc::c_int as usize]
        && lower[2 as libc::c_int as usize] == (*elem).lower[2 as libc::c_int as usize]
        && upper[2 as libc::c_int as usize] == (*elem).upper[2 as libc::c_int as usize]
    {
        Profiler_End();
        return;
    }
    let mut lowerUnion: [int32; 3] = [
        Mini(lower[0 as libc::c_int as usize], (*elem).lower[0 as libc::c_int as usize]),
        Mini(lower[1 as libc::c_int as usize], (*elem).lower[1 as libc::c_int as usize]),
        Mini(lower[2 as libc::c_int as usize], (*elem).lower[2 as libc::c_int as usize]),
    ];
    let mut upperUnion: [int32; 3] = [
        Maxi(upper[0 as libc::c_int as usize], (*elem).upper[0 as libc::c_int as usize]),
        Maxi(upper[1 as libc::c_int as usize], (*elem).upper[1 as libc::c_int as usize]),
        Maxi(upper[2 as libc::c_int as usize], (*elem).upper[2 as libc::c_int as usize]),
    ];
    (*self_0).version = ((*self_0).version).wrapping_add(1);
    let mut vRemove: uint64 = (*self_0).version;
    (*self_0).version = ((*self_0).version).wrapping_add(1);
    let mut vAdd: uint64 = (*self_0).version;
    let mut x: int32 = lowerUnion[0 as libc::c_int as usize];
    while x <= upperUnion[0 as libc::c_int as usize] {
        let mut y: int32 = lowerUnion[1 as libc::c_int as usize];
        while y <= upperUnion[1 as libc::c_int as usize] {
            let mut z: int32 = lowerUnion[2 as libc::c_int as usize];
            while z <= upperUnion[2 as libc::c_int as usize] {
                let mut inPrev: bool = (*elem).lower[0 as libc::c_int as usize] <= x
                    && (*elem).lower[1 as libc::c_int as usize] <= y
                    && (*elem).lower[2 as libc::c_int as usize] <= z
                    && x <= (*elem).upper[0 as libc::c_int as usize]
                    && y <= (*elem).upper[1 as libc::c_int as usize]
                    && z <= (*elem).upper[2 as libc::c_int as usize];
                let mut inCurr: bool = lower[0 as libc::c_int as usize] <= x
                    && lower[1 as libc::c_int as usize] <= y
                    && lower[2 as libc::c_int as usize] <= z
                    && x <= upper[0 as libc::c_int as usize]
                    && y <= upper[1 as libc::c_int as usize]
                    && z <= upper[2 as libc::c_int as usize];
                if !(inPrev as libc::c_int != 0 && inCurr as libc::c_int != 0) {
                    let mut cell: *mut HashGridCell = HashGrid_GetCell(self_0, x, y, z);
                    if !((*cell).version == vAdd) {
                        if !((*cell).version == vRemove && inPrev as libc::c_int != 0) {
                            if inPrev {
                                let mut _i: int32 = 0 as libc::c_int;
                                while _i < (*cell).elems_size {
                                    if (*((*cell).elems_data).offset(_i as isize) == elem)
                                        as libc::c_int as libc::c_long != 0
                                    {
                                        (*cell).elems_size -= 1;
                                        let ref mut fresh4 = *((*cell).elems_data)
                                            .offset(_i as isize);
                                        *fresh4 = *((*cell).elems_data)
                                            .offset((*cell).elems_size as isize);
                                        break;
                                    } else {
                                        _i += 1;
                                    }
                                }
                                (*cell).version = vRemove;
                            } else {
                                if (*cell).version != vRemove {
                                    let mut _i_0: int32 = 0 as libc::c_int;
                                    while _i_0 < (*cell).elems_size {
                                        if (*((*cell).elems_data).offset(_i_0 as isize) == elem)
                                            as libc::c_int as libc::c_long != 0
                                        {
                                            (*cell).elems_size -= 1;
                                            let ref mut fresh5 = *((*cell).elems_data)
                                                .offset(_i_0 as isize);
                                            *fresh5 = *((*cell).elems_data)
                                                .offset((*cell).elems_size as isize);
                                            break;
                                        } else {
                                            _i_0 += 1;
                                        }
                                    }
                                }
                                if ((*cell).elems_capacity == (*cell).elems_size)
                                    as libc::c_int as libc::c_long != 0
                                {
                                    (*cell)
                                        .elems_capacity = if (*cell).elems_capacity != 0 {
                                        (*cell).elems_capacity * 2 as libc::c_int
                                    } else {
                                        1 as libc::c_int
                                    };
                                    let mut elemSize: usize = ::core::mem::size_of::<
                                        *mut HashGridElem,
                                    >();
                                    let mut pData: *mut *mut libc::c_void = &mut (*cell)
                                        .elems_data as *mut *mut *mut HashGridElem
                                        as *mut *mut libc::c_void;
                                    *pData = MemRealloc(
                                        (*cell).elems_data as *mut libc::c_void,
                                        ((*cell).elems_capacity as usize)
                                            .wrapping_mul(elemSize),
                                    );
                                }
                                let fresh6 = (*cell).elems_size;
                                (*cell).elems_size = (*cell).elems_size + 1;
                                let ref mut fresh7 = *((*cell).elems_data)
                                    .offset(fresh6 as isize);
                                *fresh7 = elem;
                                (*cell).version = vAdd;
                            }
                        }
                    }
                }
                z += 1;
            }
            y += 1;
        }
        x += 1;
    }
    (*elem).lower[0 as libc::c_int as usize] = lower[0 as libc::c_int as usize];
    (*elem).lower[1 as libc::c_int as usize] = lower[1 as libc::c_int as usize];
    (*elem).lower[2 as libc::c_int as usize] = lower[2 as libc::c_int as usize];
    (*elem).upper[0 as libc::c_int as usize] = upper[0 as libc::c_int as usize];
    (*elem).upper[1 as libc::c_int as usize] = upper[1 as libc::c_int as usize];
    (*elem).upper[2 as libc::c_int as usize] = upper[2 as libc::c_int as usize];
    Profiler_End();
}
#[no_mangle]
pub unsafe extern "C" fn HashGrid_GetResults(
    mut self_0: *mut HashGrid,
) -> *mut *mut libc::c_void {
    return (*self_0).results_data;
}
#[no_mangle]
pub unsafe extern "C" fn HashGrid_QueryBox(
    mut self_0: *mut HashGrid,
    mut box_0: *const Box3f,
) -> libc::c_int {
    (*self_0).results_size = 0 as libc::c_int;
    (*self_0).version = ((*self_0).version).wrapping_add(1);
    let mut lower: [int32; 3] = [
        HashGrid_ToLocal(self_0, (*box_0).lower.x),
        HashGrid_ToLocal(self_0, (*box_0).lower.y),
        HashGrid_ToLocal(self_0, (*box_0).lower.z),
    ];
    let mut upper: [int32; 3] = [
        HashGrid_ToLocal(self_0, (*box_0).upper.x),
        HashGrid_ToLocal(self_0, (*box_0).upper.y),
        HashGrid_ToLocal(self_0, (*box_0).upper.z),
    ];
    let mut x: int32 = lower[0 as libc::c_int as usize];
    while x <= upper[0 as libc::c_int as usize] {
        let mut y: int32 = lower[1 as libc::c_int as usize];
        while y <= upper[1 as libc::c_int as usize] {
            let mut z: int32 = lower[2 as libc::c_int as usize];
            while z <= upper[2 as libc::c_int as usize] {
                let mut cell: *mut HashGridCell = HashGrid_GetCell(self_0, x, y, z);
                if (*cell).version != (*self_0).version {
                    (*cell).version = (*self_0).version;
                    let mut i: int32 = 0 as libc::c_int;
                    while i < (*cell).elems_size {
                        let mut elem: *mut HashGridElem = *((*cell).elems_data)
                            .offset(i as isize);
                        if (*elem).version != (*self_0).version {
                            (*elem).version = (*self_0).version;
                            if ((*self_0).results_capacity == (*self_0).results_size)
                                as libc::c_int as libc::c_long != 0
                            {
                                (*self_0)
                                    .results_capacity = if (*self_0).results_capacity != 0 {
                                    (*self_0).results_capacity * 2 as libc::c_int
                                } else {
                                    1 as libc::c_int
                                };
                                let mut elemSize: usize = ::core::mem::size_of::<
                                    *mut libc::c_void,
                                >();
                                let mut pData: *mut *mut libc::c_void = &mut (*self_0)
                                    .results_data as *mut *mut *mut libc::c_void
                                    as *mut *mut libc::c_void;
                                *pData = MemRealloc(
                                    (*self_0).results_data as *mut libc::c_void,
                                    ((*self_0).results_capacity as usize)
                                        .wrapping_mul(elemSize),
                                );
                            }
                            let fresh8 = (*self_0).results_size;
                            (*self_0).results_size = (*self_0).results_size + 1;
                            let ref mut fresh9 = *((*self_0).results_data)
                                .offset(fresh8 as isize);
                            *fresh9 = (*elem).object;
                        }
                        i += 1;
                    }
                }
                z += 1;
            }
            y += 1;
        }
        x += 1;
    }
    return (*self_0).results_size;
}
#[no_mangle]
pub unsafe extern "C" fn HashGrid_QueryPoint(
    mut self_0: *mut HashGrid,
    mut p: *const Vec3f,
) -> libc::c_int {
    (*self_0).results_size = 0 as libc::c_int;
    let mut cell: *mut HashGridCell = HashGrid_GetCell(
        self_0,
        HashGrid_ToLocal(self_0, (*p).x),
        HashGrid_ToLocal(self_0, (*p).y),
        HashGrid_ToLocal(self_0, (*p).z),
    );
    let mut i: int32 = 0 as libc::c_int;
    while i < (*cell).elems_size {
        let mut elem: *mut HashGridElem = *((*cell).elems_data).offset(i as isize);
        if ((*self_0).results_capacity == (*self_0).results_size) as libc::c_int
            as libc::c_long != 0
        {
            (*self_0)
                .results_capacity = if (*self_0).results_capacity != 0 {
                (*self_0).results_capacity * 2 as libc::c_int
            } else {
                1 as libc::c_int
            };
            let mut elemSize: usize = ::core::mem::size_of::<*mut libc::c_void>();
            let mut pData: *mut *mut libc::c_void = &mut (*self_0).results_data
                as *mut *mut *mut libc::c_void as *mut *mut libc::c_void;
            *pData = MemRealloc(
                (*self_0).results_data as *mut libc::c_void,
                ((*self_0).results_capacity as usize).wrapping_mul(elemSize as usize),
            );
        }
        let fresh10 = (*self_0).results_size;
        (*self_0).results_size = (*self_0).results_size + 1;
        let ref mut fresh11 = *((*self_0).results_data).offset(fresh10 as isize);
        *fresh11 = (*elem).object;
        i += 1;
    }
    return (*self_0).results_size;
}
