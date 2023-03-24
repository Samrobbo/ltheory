use crate::internal::Memory::*;
use crate::Common::*;
use crate::Intersect::*;
use crate::LineSegment::*;
use crate::Math::DVec3;
use crate::Math::Vec3;
use crate::Math::Vec3_Validate;
use crate::Plane::*;
use crate::Triangle::*;
use libc;

#[derive(Clone)]
#[repr(C)]
pub struct Polygon {
    pub vertices: Vec<Vec3>,
}

#[no_mangle]
pub unsafe extern "C" fn Polygon_ToPlane(mut polygon: *mut Polygon, mut out: *mut Plane) {
    let mut v: &Vec<Vec3> = &(*polygon).vertices;
    
    let mut n: DVec3 = DVec3 {
        x: 0.0,
        y: 0.,
        z: 0.,
    };
    let mut centroid = DVec3::ZERO;

    let vCurAsF32 = v[v.len() - 1];
    let mut vCur = DVec3::new(vCurAsF32.x as f64, vCurAsF32.y as f64, vCurAsF32.z as f64);
    let mut i: usize = 0;
    while i < v.len() {
        let vPrev: DVec3 = vCur;
        let vCurAsF32 = v[i];
        vCur = DVec3::new(vCurAsF32.x as f64, vCurAsF32.y as f64, vCurAsF32.z as f64);

        n.x += (vPrev.y - vCur.y) * (vPrev.z + vCur.z);
        n.y += (vPrev.z - vCur.z) * (vPrev.x + vCur.x);
        n.z += (vPrev.x - vCur.x) * (vPrev.y + vCur.y);
        centroid += vCur;
        i += 1;
    }
    n = n.normalize();
    centroid /= v.len() as f64;

    (*out).n = Vec3::new(n.x as f32, n.y as f32, n.z as f32);
    (*out).d = DVec3::dot(centroid, n) as f32;

    // CHECK2(Assert(PointsInPlane(out, polygon)));
}

#[no_mangle]
pub unsafe extern "C" fn Polygon_ToPlaneFast(mut polygon: *mut Polygon, mut out: *mut Plane) {
    // NOTE: Doesn't normalize n and uses v[0] as the center.

    let v: &Vec<Vec3> = &(*polygon).vertices;

    let mut n: Vec3 = Vec3::new(0.0f32, 0., 0.);
    let mut i: usize = v.len() - 1;
    let mut j: usize = 0;
    while j < v.len() {
        n.x += (v[i].y - v[j].y) * (v[i].z + v[j].z);
        n.y += (v[i].z - v[j].z) * (v[i].x + v[j].x);
        n.z += (v[i].x - v[j].x) * (v[i].y + v[j].y);

        i = j;
        j += 1;
    }

    (*out).n = n;
    (*out).d = Vec3::dot(v[0], n);

    // CHECK2(Assert(PointsInPlane(out, polygon)));
}

#[inline]
unsafe extern "C" fn Polygon_SplitImpl(
    polygon: *const Polygon,
    mut splitPlane: Plane,
    back: *mut Polygon,
    front: *mut Polygon,
) {
    if (*polygon).vertices.is_empty() {
        return
    }

    let mut a: Vec3 = *(*polygon).vertices.last().unwrap();
    let mut aSide: PointClassification = Plane_ClassifyPoint(&mut splitPlane, &mut a);
    let mut j: i32 = 0;
    while j < (*polygon).vertices.len() as i32 {
        let mut b: Vec3 = (*polygon).vertices[j as usize];
        let mut bSide: PointClassification = Plane_ClassifyPoint(&mut splitPlane, &mut b);

        if bSide == PointClassification::InFront {
            if aSide == PointClassification::Behind {
                let mut i = Vec3::ZERO;
                let mut lineSegment: LineSegment = LineSegment { p0: b, p1: a };
                let hit: bool =
                    Intersect_LineSegmentPlane(&mut lineSegment, &mut splitPlane, &mut i);
                (*front).vertices.push(i);
                (*back).vertices.push(i);

                // Assert(hit); UNUSED(hit);
                // Assert(Plane_ClassifyPoint(&splitPlane, &i) == PointClassification_Coplanar);
            }
            (*front).vertices.push(b)
        } else if bSide == PointClassification::Behind {
            if aSide == PointClassification::InFront {
                let mut i = Vec3::ZERO;
                let mut lineSegment: LineSegment = LineSegment { p0: a, p1: b };
                let mut _hit: bool =
                    Intersect_LineSegmentPlane(&mut lineSegment, &mut splitPlane, &mut i);
                (*front).vertices.push(i);
                (*back).vertices.push(i);

                // Assert(hit); UNUSED(hit);
                // Assert(Plane_ClassifyPoint(&splitPlane, &i) == PointClassification_Coplanar);
            } else if aSide == PointClassification::Coplanar {
                (*back).vertices.push(a);
            }
            (*back).vertices.push(b);
        } else {
            if aSide == PointClassification::Behind {
                (*back).vertices.push(b);
            }
            (*front).vertices.push(b);
        }
        
        a = b;
        aSide = bSide;
        j += 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn Polygon_SplitSafe(
    mut polygon: *const Polygon,
    mut splitPlane: Plane,
    mut back: *mut Polygon,
    mut front: *mut Polygon,
) {
    Polygon_SplitImpl(polygon, splitPlane, back, front);

    let polygons: [*mut Polygon; 2] = [front, back];
    let mut i: i32 = 0;
    while i < polygons.len() as i32
    {
        let polygonPart: *mut Polygon = polygons[i as usize];
        let v: &Vec<Vec3> = &(*polygonPart).vertices;

        let mut vCur: Vec3 = v[v.len() - 1];
        let mut l: usize = 0;
        while l < v.len() {
            let vPrev: Vec3 = vCur;
            vCur = v[l];

            let edgeLen: f32 = vCur.distance(vPrev);
            if (edgeLen as f64) < 0.75f64 * 1e-4f64 {
                (*back).vertices.clear();
                (*front).vertices.clear();
                for vertex in (*polygon).vertices.iter() {
                    (*back).vertices.push(*vertex);
                    (*front).vertices.push(*vertex);
                }
                return;
            }
            l += 1;
        }
        i += 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn Polygon_Split(
    mut polygon: *mut Polygon,
    mut splitPlane: Plane,
    mut back: *mut Polygon,
    mut front: *mut Polygon,
) {
    Polygon_SplitImpl(polygon, splitPlane, back, front);
}

#[no_mangle]
pub unsafe extern "C" fn Polygon_GetCentroid(mut polygon: *mut Polygon, mut out: *mut Vec3) {
    let mut centroid = Vec3::ZERO;

    for v in (*polygon).vertices.iter() {
        centroid += *v;
    }
    centroid /= (*polygon).vertices.len() as f32;

    *out = centroid;
}

pub unsafe fn Polygon_ConvexToTriangles(polygon: &Polygon, triangles: &mut Vec<Triangle>) {
    let v = &(*polygon).vertices;
    for i in 1..(v.len() - 1) {
        triangles.push(Triangle { vertices: [v[0], v[i], v[i + 1]] });
    }
}

#[no_mangle]
pub unsafe extern "C" fn Polygon_Validate(mut polygon: *mut Polygon) -> Error {
    let v: &Vec<Vec3> = &(*polygon).vertices;

    let mut vCur: Vec3 = v[v.len() - 1];
    let mut i: usize = 0;
    while i < v.len() {
        let vPrev: Vec3 = vCur;
        vCur = v[i];

        // NaN or Inf
        let e: Error = Vec3_Validate(vCur);
        if e != 0 {
            return 0x400000 | e;
        }

        // Degenerate
        let mut j: usize = i + 1;
        while j < v.len() {
            if vCur == v[j] {
                return (0x400000 | 0x40) as Error;
            }
            j += 1;
        }

        // Sliver
        /* TODO : See comment on slivers in Triangle_Validate */
        let edgeLen = vCur.distance(vPrev);
        if (edgeLen as f64) < 0.75f64 * 1e-4f64 {
            return (0x400000 | 0x8) as Error;
        }
        i += 1;
    }
    0 as Error
}
