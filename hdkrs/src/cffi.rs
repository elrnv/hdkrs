#![allow(clippy::cyclomatic_complexity)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use gut::io::{
    obj::*,
    vtk::{
        convert_pointcloud_to_vtk_format, convert_polymesh_to_vtk_format,
        convert_tetmesh_to_vtk_format, convert_vtk_to_polymesh,
        convert_vtk_to_tetmesh, model::Vtk,
        VTKPolyExportStyle,
    },
};
use gut::mesh::{attrib, topology as topo, Attrib, PointCloud, PolyMesh, TetMesh, VertexPositions};
use gut::{self, NumCells, NumFaces};
use hashbrown::hash_map::Iter;
pub use std::os::raw::{c_char, c_double, c_float, c_int, c_schar, c_void};
pub use libc::size_t;
use std::any::TypeId;
use std::ffi::{CStr, CString};
use std::slice;
use std::sync::Arc;

/// A Rust polygon mesh struct.
pub struct HR_PolyMesh {
    pub mesh: PolyMesh<f64>,
}

/// A Rust tetmesh struct.
pub struct HR_TetMesh {
    pub mesh: TetMesh<f64>,
}

/// A Rust pointcloud struct.
pub struct HR_PointCloud {
    pub mesh: PointCloud<f64>,
}

#[repr(C)]
pub enum HRCookResultTag {
    HR_SUCCESS,
    HR_WARNING,
    HR_ERROR,
}

/// Result for C interop.
#[repr(C)]
pub struct HR_CookResult {
    pub message: *mut c_char,
    pub tag: HRCookResultTag,
}

#[no_mangle]
pub unsafe extern "C" fn hr_free_result(res: HR_CookResult) {
    let _ = CString::from_raw(res.message);
}

pub enum HR_VertexIndex {}
pub enum HR_FaceIndex {}
pub enum HR_CellIndex {}
pub enum HR_FaceVertexIndex {}
pub enum HR_CellVertexIndex {}

#[repr(C)]
pub struct HR_PointArray {
    capacity: size_t,
    size: size_t,
    array: *mut [f64; 3],
}

#[repr(C)]
pub struct HR_IndexArray {
    capacity: size_t,
    size: size_t,
    array: *mut size_t,
}

macro_rules! get_points_impl {
    ($mesh:ident) => {{
        assert!(!$mesh.is_null());
        let mut pts: Vec<[f64; 3]> = (*$mesh).mesh.vertex_positions().to_vec();

        let arr = HR_PointArray {
            capacity: pts.capacity(),
            size: pts.len(),
            array: pts.as_mut_slice().as_mut_ptr(),
        };

        ::std::mem::forget(pts);

        arr
    }};
}

#[no_mangle]
pub unsafe extern "C" fn hr_get_pointcloud_points(ptcloud: *const HR_PointCloud) -> HR_PointArray {
    get_points_impl!(ptcloud)
}

#[no_mangle]
pub unsafe extern "C" fn hr_get_tetmesh_points(mesh: *const HR_TetMesh) -> HR_PointArray {
    get_points_impl!(mesh)
}

#[no_mangle]
pub unsafe extern "C" fn hr_get_polymesh_points(mesh: *const HR_PolyMesh) -> HR_PointArray {
    get_points_impl!(mesh)
}

#[no_mangle]
pub unsafe extern "C" fn hr_get_tetmesh_indices(mesh: *const HR_TetMesh) -> HR_IndexArray {
    assert!(!mesh.is_null());
    let mut indices = Vec::new();

    for cell in (*mesh).mesh.cell_iter() {
        for &idx in cell.iter() {
            indices.push(idx);
        }
    }

    let arr = HR_IndexArray {
        capacity: indices.capacity(),
        size: indices.len(),
        array: indices.as_mut_slice().as_mut_ptr(),
    };

    ::std::mem::forget(indices);

    arr
}

/// Polygon mesh indices is a contiguous set of polygon indices, each in the form:
/// `n, i_1, i_2, ..., i_n` where `n` is the number of sides on a polygon.
#[no_mangle]
pub unsafe extern "C" fn hr_get_polymesh_indices(mesh: *const HR_PolyMesh) -> HR_IndexArray {
    assert!(!mesh.is_null());
    let mut indices = Vec::new();

    for poly in (*mesh).mesh.face_iter() {
        indices.push(poly.len());
        for &idx in poly.iter() {
            indices.push(idx);
        }
    }

    let arr = HR_IndexArray {
        capacity: indices.capacity(),
        size: indices.len(),
        array: indices.as_mut_slice().as_mut_ptr(),
    };

    ::std::mem::forget(indices);

    arr
}

#[no_mangle]
pub unsafe extern "C" fn hr_free_point_array(arr: HR_PointArray) {
    let _ = Vec::from_raw_parts(arr.array, arr.size, arr.capacity);
}

#[no_mangle]
pub unsafe extern "C" fn hr_free_index_array(arr: HR_IndexArray) {
    let _ = Vec::from_raw_parts(arr.array, arr.size, arr.capacity);
}

// Required for cbindgen to produce these opaque structs.
mod missing_structs {
    #[allow(dead_code)]
    struct HR_AttribIter;

    #[allow(dead_code)]
    struct CString;
}

pub enum HR_AttribIter<'a> {
    Vertex(Iter<'a, String, attrib::Attribute<topo::VertexIndex>>),
    Face(Iter<'a, String, attrib::Attribute<topo::FaceIndex>>),
    Cell(Iter<'a, String, attrib::Attribute<topo::CellIndex>>),
    FaceVertex(Iter<'a, String, attrib::Attribute<topo::FaceVertexIndex>>),
    CellVertex(Iter<'a, String, attrib::Attribute<topo::CellVertexIndex>>),
    None,
}

// Another workaround for ffi. To ensure the lifetime can be elided in HR_AttribIter in the return
// results for the two functions below, we need to pass a parameter with a lifetime parameter.
// Otherwise cbindgen doesn't know how to ignore lifetime parameters on returned types for some
// reason.
pub struct HR_Dummy<'a> {
    p: ::std::marker::PhantomData<&'a u32>,
}

#[no_mangle]
pub unsafe extern "C" fn hr_pointcloud_attrib_iter(
    mesh_ptr: *const HR_PointCloud,
    loc: HRAttribLocation,
    _d: *const HR_Dummy,
) -> *mut HR_AttribIter {
    assert!(!mesh_ptr.is_null());

    let mesh = &(*mesh_ptr);

    let iter = Box::new(match loc {
        HRAttribLocation::HR_VERTEX => {
            HR_AttribIter::Vertex(mesh.mesh.attrib_dict::<topo::VertexIndex>().iter())
        }
        _ => return ::std::ptr::null_mut::<HR_AttribIter>(),
    });

    Box::into_raw(iter)
}

#[no_mangle]
pub unsafe extern "C" fn hr_tetmesh_attrib_iter(
    mesh_ptr: *const HR_TetMesh,
    loc: HRAttribLocation,
    _d: *const HR_Dummy,
) -> *mut HR_AttribIter {
    assert!(!mesh_ptr.is_null());

    let mesh = &(*mesh_ptr);

    let iter = Box::new(match loc {
        HRAttribLocation::HR_VERTEX => {
            HR_AttribIter::Vertex(mesh.mesh.attrib_dict::<topo::VertexIndex>().iter())
        }
        HRAttribLocation::HR_CELL => {
            HR_AttribIter::Cell(mesh.mesh.attrib_dict::<topo::CellIndex>().iter())
        }
        HRAttribLocation::HR_CELLVERTEX => {
            HR_AttribIter::CellVertex(mesh.mesh.attrib_dict::<topo::CellVertexIndex>().iter())
        }
        _ => return ::std::ptr::null_mut::<HR_AttribIter>(),
    });

    Box::into_raw(iter)
}

#[no_mangle]
pub unsafe extern "C" fn hr_polymesh_attrib_iter(
    mesh_ptr: *const HR_PolyMesh,
    loc: HRAttribLocation,
    _d: *const HR_Dummy,
) -> *mut HR_AttribIter {
    assert!(!mesh_ptr.is_null());

    let mesh = &(*mesh_ptr);

    let iter = Box::new(match loc {
        HRAttribLocation::HR_VERTEX => {
            HR_AttribIter::Vertex(mesh.mesh.attrib_dict::<topo::VertexIndex>().iter())
        }
        HRAttribLocation::HR_FACE => {
            HR_AttribIter::Face(mesh.mesh.attrib_dict::<topo::FaceIndex>().iter())
        }
        HRAttribLocation::HR_FACEVERTEX => {
            HR_AttribIter::FaceVertex(mesh.mesh.attrib_dict::<topo::FaceVertexIndex>().iter())
        }
        _ => return ::std::ptr::null_mut::<HR_AttribIter>(),
    });

    Box::into_raw(iter)
}

#[no_mangle]
pub unsafe extern "C" fn hr_free_attrib_iter(data: *mut HR_AttribIter) {
    let _ = Box::from_raw(data);
}

/// Wrapper around the `attrib::HR_Attribute` struct to eliminate generics for ffi.
#[derive(Debug)]
#[repr(C)]
pub enum HR_AttribData<'a> {
    Vertex(&'a attrib::Attribute<topo::VertexIndex>),
    Face(&'a attrib::Attribute<topo::FaceIndex>),
    Cell(&'a attrib::Attribute<topo::CellIndex>),
    FaceVertex(&'a attrib::Attribute<topo::FaceVertexIndex>),
    CellVertex(&'a attrib::Attribute<topo::CellVertexIndex>),
    None,
}

/// Opaque type to store data about a particular attribute. This struct owns the string it
/// contains thus it must be freed when done.
#[derive(Debug)]
pub struct HR_Attribute<'a> {
    name: CString,
    data: HR_AttribData<'a>,
}

/// Produces an `HR_Attribute` struct that references the next available attribute data.
#[no_mangle]
pub unsafe extern "C" fn hr_attrib_iter_next(iter_ptr: *mut HR_AttribIter) -> *mut HR_Attribute {
    assert!(!iter_ptr.is_null());

    let null = ::std::ptr::null::<HR_Attribute>() as *mut HR_Attribute;

    match *iter_ptr {
        HR_AttribIter::Vertex(ref mut iter) => iter.next().map_or(null, |(k, v)| {
            Box::into_raw(Box::new(HR_Attribute {
                name: CString::new(k.as_str()).unwrap(),
                data: HR_AttribData::Vertex(v),
            }))
        }),
        HR_AttribIter::Face(ref mut iter) => iter.next().map_or(null, |(k, v)| {
            Box::into_raw(Box::new(HR_Attribute {
                name: CString::new(k.as_str()).unwrap(),
                data: HR_AttribData::Face(v),
            }))
        }),
        HR_AttribIter::Cell(ref mut iter) => iter.next().map_or(null, |(k, v)| {
            Box::into_raw(Box::new(HR_Attribute {
                name: CString::new(k.as_str()).unwrap(),
                data: HR_AttribData::Cell(v),
            }))
        }),
        HR_AttribIter::FaceVertex(ref mut iter) => iter.next().map_or(null, |(k, v)| {
            Box::into_raw(Box::new(HR_Attribute {
                name: CString::new(k.as_str()).unwrap(),
                data: HR_AttribData::FaceVertex(v),
            }))
        }),
        HR_AttribIter::CellVertex(ref mut iter) => iter.next().map_or(null, |(k, v)| {
            Box::into_raw(Box::new(HR_Attribute {
                name: CString::new(k.as_str()).unwrap(),
                data: HR_AttribData::CellVertex(v),
            }))
        }),
        HR_AttribIter::None => null,
    }
}

#[no_mangle]
pub unsafe extern "C" fn hr_free_attribute(attrib: *mut HR_Attribute) {
    let _ = Box::from_raw(attrib);
}

#[no_mangle]
pub unsafe extern "C" fn hr_attrib_name(data: *const HR_Attribute) -> *const c_char {
    if data.is_null() {
        std::ptr::null()
    } else {
        (*data).name.as_ptr()
    }
}

#[repr(C)]
pub enum HRDataType {
    HR_I8,
    HR_I32,
    HR_I64,
    HR_F32,
    HR_F64,
    HR_STR,
    HR_UNSUPPORTED,
}

macro_rules! impl_supported_types {
    ($var:ident, $type:ty) => {
        $var == TypeId::of::<$type>()
    };
    ($var:ident, $type:ty, $($array_sizes:expr),*) => {
        $var == TypeId::of::<$type>() ||
            $($var == TypeId::of::<[$type;$array_sizes]>())||*
    }
}

macro_rules! impl_supported_sizes {
    ($var:ident, $($types:ty),*) => {
        $($var == TypeId::of::<$types>())||*
    };
    ($var:ident, $array_size:expr, $($types:ty),*) => {
        $($var == TypeId::of::<[$types;$array_size]>())||*
    }
}

macro_rules! cast_to_vec {
    ($type:ident, $data:ident) => {
        (*$data)
            .direct_clone_into_vec::<$type>()
            .unwrap_or(Vec::new())
    };
    ($type:ident, $data:ident, $tuple_size:expr) => {
        (*$data)
            .direct_clone_into_vec::<[$type; $tuple_size]>()
            .unwrap_or(Vec::new())
            .iter()
            .flat_map(|x| x.iter().cloned())
            .collect()
    };
}

fn attrib_type_id<I>(attrib: &attrib::Attribute<I>) -> HRDataType {
    match attrib.data.element_type_id() {
        x if impl_supported_types!(
            x, i8, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16
        ) =>
        {
            HRDataType::HR_I8
        }
        x if impl_supported_types!(
            x, i32, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16
        ) =>
        {
            HRDataType::HR_I32
        }
        x if impl_supported_types!(
            x, i64, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16
        ) =>
        {
            HRDataType::HR_I64
        }
        x if impl_supported_types!(
            x, f32, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16
        ) =>
        {
            HRDataType::HR_F32
        }
        x if impl_supported_types!(
            x, f64, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16
        ) =>
        {
            HRDataType::HR_F64
        }
        // Currently we only support a single string, supporting tuples would require a refactor of
        // the gut attribute system.
        x if impl_supported_types!(x, Arc<String>) => HRDataType::HR_STR,
        _ => HRDataType::HR_UNSUPPORTED,
    }
}

fn attrib_flat_array<I, T: 'static + Clone>(attrib: &attrib::Attribute<I>) -> (Vec<T>, usize) {
    let tuple_size = match attrib.data.element_type_id() {
        x if impl_supported_sizes!(x, i8, i32, i64, f32, f64, Arc<String>) => 1,
        x if impl_supported_sizes!(x, 1, i8, i32, i64, f32, f64) => 1,
        x if impl_supported_sizes!(x, 2, i8, i32, i64, f32, f64) => 2,
        x if impl_supported_sizes!(x, 3, i8, i32, i64, f32, f64) => 3,
        x if impl_supported_sizes!(x, 4, i8, i32, i64, f32, f64) => 4,
        x if impl_supported_sizes!(x, 5, i8, i32, i64, f32, f64) => 5,
        x if impl_supported_sizes!(x, 6, i8, i32, i64, f32, f64) => 6,
        x if impl_supported_sizes!(x, 7, i8, i32, i64, f32, f64) => 7,
        x if impl_supported_sizes!(x, 8, i8, i32, i64, f32, f64) => 8,
        x if impl_supported_sizes!(x, 9, i8, i32, i64, f32, f64) => 9,
        x if impl_supported_sizes!(x, 10, i8, i32, i64, f32, f64) => 10,
        x if impl_supported_sizes!(x, 11, i8, i32, i64, f32, f64) => 11,
        x if impl_supported_sizes!(x, 12, i8, i32, i64, f32, f64) => 12,
        x if impl_supported_sizes!(x, 13, i8, i32, i64, f32, f64) => 13,
        x if impl_supported_sizes!(x, 14, i8, i32, i64, f32, f64) => 14,
        x if impl_supported_sizes!(x, 15, i8, i32, i64, f32, f64) => 15,
        x if impl_supported_sizes!(x, 16, i8, i32, i64, f32, f64) => 16,
        _ => 0,
    };
    // Strings are stored in indirect attributes and cannot be directly cast
    if let Ok(indirect_str_iter) = attrib.indirect_iter::<String>() {
        if TypeId::of::<T>() == TypeId::of::<*mut i8>() {
            assert_eq!(std::mem::size_of::<T>(), std::mem::size_of::<*mut i8>());
            // The following is safe because we check that T is indeed *mut i8 so there is no
            // ambiguity here. This can probably be refactored at the cost of some code verbosity.
            return (
                indirect_str_iter
                    .map(|s| unsafe {
                        std::mem::transmute_copy(&CString::new(s.as_str()).unwrap().into_raw())
                    })
                    .collect(),
                tuple_size,
            );
        }
    }

    let flat_vec = match tuple_size {
        1 => cast_to_vec!(T, attrib),
        2 => cast_to_vec!(T, attrib, 2),
        3 => cast_to_vec!(T, attrib, 3),
        4 => cast_to_vec!(T, attrib, 4),
        5 => cast_to_vec!(T, attrib, 5),
        6 => cast_to_vec!(T, attrib, 6),
        7 => cast_to_vec!(T, attrib, 7),
        8 => cast_to_vec!(T, attrib, 8),
        9 => cast_to_vec!(T, attrib, 9),
        10 => cast_to_vec!(T, attrib, 10),
        11 => cast_to_vec!(T, attrib, 11),
        12 => cast_to_vec!(T, attrib, 12),
        13 => cast_to_vec!(T, attrib, 13),
        14 => cast_to_vec!(T, attrib, 14),
        15 => cast_to_vec!(T, attrib, 15),
        16 => cast_to_vec!(T, attrib, 16),
        _ => Vec::new(),
    };

    (flat_vec, tuple_size)
}

#[no_mangle]
pub unsafe extern "C" fn hr_attrib_data_type(attrib: *const HR_Attribute) -> HRDataType {
    if attrib.is_null() {
        HRDataType::HR_UNSUPPORTED
    } else {
        match (*attrib).data {
            HR_AttribData::Vertex(a) => attrib_type_id(a),
            HR_AttribData::Face(a) => attrib_type_id(a),
            HR_AttribData::Cell(a) => attrib_type_id(a),
            HR_AttribData::FaceVertex(a) => attrib_type_id(a),
            HR_AttribData::CellVertex(a) => attrib_type_id(a),
            HR_AttribData::None => HRDataType::HR_UNSUPPORTED,
        }
    }
}

#[repr(C)]
pub struct HR_AttribArrayI8 {
    capacity: size_t,
    size: size_t,
    tuple_size: size_t,
    array: *mut i8,
}
#[repr(C)]
pub struct HR_AttribArrayI32 {
    capacity: size_t,
    size: size_t,
    tuple_size: size_t,
    array: *mut i32,
}
#[repr(C)]
pub struct HR_AttribArrayI64 {
    capacity: size_t,
    size: size_t,
    tuple_size: size_t,
    array: *mut i64,
}
#[repr(C)]
pub struct HR_AttribArrayF32 {
    capacity: size_t,
    size: size_t,
    tuple_size: size_t,
    array: *mut f32,
}
#[repr(C)]
pub struct HR_AttribArrayF64 {
    capacity: size_t,
    size: size_t,
    tuple_size: size_t,
    array: *mut f64,
}

#[repr(C)]
pub struct HR_AttribArrayStr {
    capacity: size_t,
    size: size_t,
    tuple_size: size_t,
    array: *mut *mut c_char,
}

macro_rules! impl_get_attrib_data {
    (_impl_make_array $array_name:ident, $vec:ident, $tuple_size:ident) => {{
        let arr = $array_name {
            capacity: $vec.capacity(),
            size: $vec.len(),
            tuple_size: $tuple_size,
            array: $vec.as_mut_ptr(),
        };

        ::std::mem::forget($vec);

        arr
    }};
    ($array_name:ident, $attrib_data:ident) => {{
        let (mut vec, tuple_size) = if $attrib_data.is_null() {
            (Vec::new(), 0)
        } else {
            match (*$attrib_data).data {
                HR_AttribData::Vertex(data) => attrib_flat_array(data),
                HR_AttribData::Face(data) => attrib_flat_array(data),
                HR_AttribData::Cell(data) => attrib_flat_array(data),
                HR_AttribData::FaceVertex(data) => attrib_flat_array(data),
                HR_AttribData::CellVertex(data) => attrib_flat_array(data),
                HR_AttribData::None => (Vec::new(), 0),
            }
        };

        impl_get_attrib_data!(_impl_make_array $array_name, vec, tuple_size)
    }};
}

#[no_mangle]
pub unsafe extern "C" fn hr_attrib_data_i8(attrib: *mut HR_Attribute) -> HR_AttribArrayI8 {
    impl_get_attrib_data!(HR_AttribArrayI8, attrib)
}
#[no_mangle]
pub unsafe extern "C" fn hr_attrib_data_i32(attrib: *mut HR_Attribute) -> HR_AttribArrayI32 {
    impl_get_attrib_data!(HR_AttribArrayI32, attrib)
}
#[no_mangle]
pub unsafe extern "C" fn hr_attrib_data_i64(attrib: *mut HR_Attribute) -> HR_AttribArrayI64 {
    impl_get_attrib_data!(HR_AttribArrayI64, attrib)
}
#[no_mangle]
pub unsafe extern "C" fn hr_attrib_data_f32(attrib: *mut HR_Attribute) -> HR_AttribArrayF32 {
    impl_get_attrib_data!(HR_AttribArrayF32, attrib)
}
#[no_mangle]
pub unsafe extern "C" fn hr_attrib_data_f64(attrib: *mut HR_Attribute) -> HR_AttribArrayF64 {
    impl_get_attrib_data!(HR_AttribArrayF64, attrib)
}
#[no_mangle]
pub unsafe extern "C" fn hr_attrib_data_str(attrib: *mut HR_Attribute) -> HR_AttribArrayStr {
    impl_get_attrib_data!(HR_AttribArrayStr, attrib)
}

#[no_mangle]
pub unsafe extern "C" fn hr_free_attrib_data_i8(arr: HR_AttribArrayI8) {
    let _ = Vec::from_raw_parts(arr.array, arr.size, arr.capacity);
}

#[no_mangle]
pub unsafe extern "C" fn hr_free_attrib_data_i32(arr: HR_AttribArrayI32) {
    let _ = Vec::from_raw_parts(arr.array, arr.size, arr.capacity);
}

#[no_mangle]
pub unsafe extern "C" fn hr_free_attrib_data_i64(arr: HR_AttribArrayI64) {
    let _ = Vec::from_raw_parts(arr.array, arr.size, arr.capacity);
}

#[no_mangle]
pub unsafe extern "C" fn hr_free_attrib_data_f32(arr: HR_AttribArrayF32) {
    let _ = Vec::from_raw_parts(arr.array, arr.size, arr.capacity);
}

#[no_mangle]
pub unsafe extern "C" fn hr_free_attrib_data_f64(arr: HR_AttribArrayF64) {
    let _ = Vec::from_raw_parts(arr.array, arr.size, arr.capacity);
}

#[no_mangle]
pub unsafe extern "C" fn hr_free_attrib_data_str(arr: HR_AttribArrayStr) {
    for i in 0..arr.size as isize {
        let _ = CString::from_raw(*arr.array.offset(i));
    }

    let _ = Vec::from_raw_parts(arr.array, arr.size, arr.capacity);
}

#[no_mangle]
pub unsafe extern "C" fn hr_free_iter(iter: *mut HR_AttribIter) {
    if !iter.is_null() {
        let _ = Box::from_raw(iter);
    }
}

#[repr(C)]
#[derive(Debug)]
pub enum HRAttribLocation {
    HR_VERTEX,
    HR_FACE,
    HR_CELL,
    HR_FACEVERTEX,
    HR_CELLVERTEX,
}

#[no_mangle]
pub unsafe extern "C" fn hr_make_pointcloud(
    ncoords: size_t,
    coords: *const c_double,
) -> *mut HR_PointCloud {
    // check invariants
    assert!(
        ncoords % 3 == 0,
        "Given coordinate array size is not a multiple of 3."
    );
    let verts = ptr_to_vec_of_triples((ncoords / 3) as usize, coords);
    let ptcloud = Box::new(HR_PointCloud {
        mesh: gut::mesh::PointCloud::new(verts),
    });
    Box::into_raw(ptcloud)
}

macro_rules! make_mesh_impl {
    ($hr_mesh:ident, $mesh_ty:ident, $ncoords:ident, $coords:ident, $convert:expr) => {{
        // check invariants
        assert!(
            $ncoords % 3 == 0,
            "Given coordinate array size is not a multiple of 3."
        );

        let indices = $convert;
        let verts = ptr_to_vec_of_triples(($ncoords / 3) as usize, $coords);

        let mesh = Box::new($hr_mesh {
            mesh: gut::mesh::$mesh_ty::new(verts, indices),
        });

        Box::into_raw(mesh)
    }};
}

#[no_mangle]
pub unsafe extern "C" fn hr_make_polymesh(
    ncoords: size_t,
    coords: *const c_double,
    nindices: size_t,
    indices: *const size_t,
) -> *mut HR_PolyMesh {
    make_mesh_impl!(
        HR_PolyMesh,
        PolyMesh,
        ncoords,
        coords,
        slice::from_raw_parts(indices, nindices)
    )
}

#[no_mangle]
pub unsafe extern "C" fn hr_make_tetmesh(
    ncoords: size_t,
    coords: *const c_double,
    nindices: size_t,
    indices: *const size_t,
) -> *mut HR_TetMesh {
    make_mesh_impl!(
        HR_TetMesh,
        TetMesh,
        ncoords,
        coords,
        bytemuck::cast_slice(slice::from_raw_parts(indices, nindices)).to_vec()
    )
}

#[no_mangle]
pub unsafe extern "C" fn hr_free_pointcloud(mesh: *mut HR_PointCloud) {
    if !mesh.is_null() {
        let _ = Box::from_raw(mesh);
    }
}

#[no_mangle]
pub unsafe extern "C" fn hr_free_tetmesh(mesh: *mut HR_TetMesh) {
    if !mesh.is_null() {
        let _ = Box::from_raw(mesh);
    }
}

#[no_mangle]
pub unsafe extern "C" fn hr_free_polymesh(mesh: *mut HR_PolyMesh) {
    if !mesh.is_null() {
        let _ = Box::from_raw(mesh);
    }
}

/// Helper macro for converting C-style data to a `Vec<[T;$tuple_size]>`
/// `data` is the const pointer of to the data to be copied.
/// `size` is the number of elements returned in the vector.
macro_rules! ptr_to_vec_of_arrays {
    ($size:ident, $data:ident, $ty:ty) => {{
        //slice::from_raw_parts($data, $size).to_vec()
        slice::from_raw_parts($data, $size).to_vec()
    }};
    ($size:ident, $data:ident, $ty:ty, $tuple_size:expr) => {{
        bytemuck::cast_slice::<_, [$ty; $tuple_size]>(slice::from_raw_parts($data, $size)).to_vec()
        //assert!($size % $tuple_size == 0, "Wrong tuple size for array.");
        //let nelem = $size / $tuple_size;
        //let mut data = Vec::with_capacity(nelem);
        //for i in 0..nelem as isize {
        //    let mut s: [$ty; $tuple_size] = ::std::mem::uninitialized();
        //    for k in 0..$tuple_size {
        //        s[k] = (*$data.offset($tuple_size * i + k as isize)).clone();
        //    }

        //    data.push(s);
        //}
        //data
    }};
}

#[derive(Debug, PartialEq)]
enum Error {
    Attrib(attrib::Error),
    Internal,
}

impl From<attrib::Error> for Error {
    fn from(a: attrib::Error) -> Self {
        Error::Attrib(a)
    }
}

macro_rules! impl_add_attrib {
    (_impl HR_PointCloud, $data_type:ty, $mesh:ident,
     $len:ident, $data:ident, $name:ident, $loc:ident) => {
        let vec = ptr_to_vec_of_arrays!($len, $data, $data_type);
        impl_add_attrib!(_impl_points $mesh, $loc, $name, vec);
    };
    (_impl HR_PolyMesh, $data_type:ty, $mesh:ident,
     $len:ident, $data:ident, $name:ident, $loc:ident) => {
        let vec = ptr_to_vec_of_arrays!($len, $data, $data_type);
        impl_add_attrib!(_impl_surface $mesh, $loc, $name, vec);
    };
    (_impl HR_TetMesh, $data_type:ty, $mesh:ident,
     $len:ident, $data:ident, $name:ident, $loc:ident) => {
        let vec = ptr_to_vec_of_arrays!($len, $data, $data_type);
        impl_add_attrib!(_impl_volume $mesh, $loc, $name, vec);
    };
    (_impl HR_PointCloud, $data_type:ty, $tuple_size:expr, $mesh:ident,
     $len:ident, $data:ident, $name:ident, $loc:ident) => {
        let vec = ptr_to_vec_of_arrays!($len, $data, $data_type, $tuple_size);
        impl_add_attrib!(_impl_points $mesh, $loc, $name, vec);
    };
    (_impl HR_PolyMesh, $data_type:ty, $tuple_size:expr, $mesh:ident,
     $len:ident, $data:ident, $name:ident, $loc:ident) => {
        let vec = ptr_to_vec_of_arrays!($len, $data, $data_type, $tuple_size);
        impl_add_attrib!(_impl_surface $mesh, $loc, $name, vec);
    };
    (_impl HR_TetMesh, $data_type:ty, $tuple_size:expr, $mesh:ident,
     $len:ident, $data:ident, $name:ident, $loc:ident) => {
        let vec = ptr_to_vec_of_arrays!($len, $data, $data_type, $tuple_size);
        impl_add_attrib!(_impl_volume $mesh, $loc, $name, vec);
    };
    // Points only attributes
    (_impl_points $mesh:ident, $loc:ident, $name:ident, $vec:ident) => {
        {
            if let HRAttribLocation::HR_VERTEX = $loc {
                if let Err(error) = (*$mesh).mesh.add_attrib_data::<_,topo::VertexIndex>($name, $vec) {
                    println!("Warning: failed to add attribute \"{}\" at {:?}, with error: {:?}", $name, $loc, error);
                }
            }
        }
    };
    // Surface type meshes like tri- or quad-meshes typically have face attributes but no
    // cell attributes.
    (_impl_surface $mesh:ident, $loc:ident, $name:ident, $vec:ident) => {
        {
            match $loc {
                HRAttribLocation::HR_VERTEX => {
                    if let Err(error) = (*$mesh).mesh.add_attrib_data::<_,topo::VertexIndex>($name, $vec) {
                        println!("Warning: failed to add attribute \"{}\" at {:?}, with error: {:?}", $name, $loc, error);
                    }
                },
                HRAttribLocation::HR_FACE => {
                    if let Err(error) = (*$mesh).mesh.add_attrib_data::<_,topo::FaceIndex>($name, $vec) {
                        println!("Warning: failed to add attribute \"{}\" at {:?}, with error: {:?}", $name, $loc, error);
                    }
                },
                HRAttribLocation::HR_FACEVERTEX => {
                    if let Err(error) = (*$mesh).mesh.add_attrib_data::<_,topo::FaceVertexIndex>($name, $vec) {
                        println!("Warning: failed to add attribute \"{}\" at {:?}, with error: {:?}", $name, $loc, error);
                    }
                },
                _ => (),
            };
        }
    };
    // Volume type meshes like tet and hex meshes have cell attributes.
    (_impl_volume $mesh:ident, $loc:ident, $name:ident, $vec:ident) => {
        {
            match $loc {
                HRAttribLocation::HR_VERTEX => {
                    if let Err(error) = (*$mesh).mesh.add_attrib_data::<_,topo::VertexIndex>($name, $vec) {
                        println!("Warning: failed to add attribute \"{}\" at {:?}, with error: {:?}", $name, $loc, error);
                    }
                },
                HRAttribLocation::HR_CELL => {
                    if let Err(error) = (*$mesh).mesh.add_attrib_data::<_,topo::CellIndex>($name, $vec) {
                        println!("Warning: failed to add attribute \"{}\" at {:?}, with error: {:?}", $name, $loc, error);
                    }
                },
                HRAttribLocation::HR_CELLVERTEX => {
                    if let Err(error) = (*$mesh).mesh.add_attrib_data::<_,topo::CellVertexIndex>($name, $vec) {
                        println!("Warning: failed to add attribute \"{}\" at {:?}, with error: {:?}", $name, $loc, error);
                    }
                },
                _ => (),
            };
        }
    };
    // Main implemnetation of the add attribute function.
    ($mtype:ident, $mesh:ident,
     $loc:ident, $name:ident, $tuple_size:ident,
     $n:ident, $data:ident: $dty:ty) => {
        assert!(!$mesh.is_null(), "Can't add attributes to a null pointer.");

        if let Ok(name_str) = CStr::from_ptr($name).to_str() {
            match $tuple_size {
                1 => {impl_add_attrib!(_impl $mtype, $dty, $mesh, $n, $data, name_str, $loc);},
                2 => {impl_add_attrib!(_impl $mtype, $dty, 2, $mesh, $n, $data, name_str, $loc);},
                3 => {impl_add_attrib!(_impl $mtype, $dty, 3, $mesh, $n, $data, name_str, $loc);},
                4 => {impl_add_attrib!(_impl $mtype, $dty, 4, $mesh, $n, $data, name_str, $loc);},
                5 => {impl_add_attrib!(_impl $mtype, $dty, 5, $mesh, $n, $data, name_str, $loc);},
                6 => {impl_add_attrib!(_impl $mtype, $dty, 6, $mesh, $n, $data, name_str, $loc);},
                7 => {impl_add_attrib!(_impl $mtype, $dty, 7, $mesh, $n, $data, name_str, $loc);},
                8 => {impl_add_attrib!(_impl $mtype, $dty, 8, $mesh, $n, $data, name_str, $loc);},
                9 => {impl_add_attrib!(_impl $mtype, $dty, 9, $mesh, $n, $data, name_str, $loc);},
                10 => {impl_add_attrib!(_impl $mtype, $dty, 10, $mesh, $n, $data, name_str, $loc);},
                11 => {impl_add_attrib!(_impl $mtype, $dty, 11, $mesh, $n, $data, name_str, $loc);},
                12 => {impl_add_attrib!(_impl $mtype, $dty, 12, $mesh, $n, $data, name_str, $loc);},
                13 => {impl_add_attrib!(_impl $mtype, $dty, 13, $mesh, $n, $data, name_str, $loc);},
                14 => {impl_add_attrib!(_impl $mtype, $dty, 14, $mesh, $n, $data, name_str, $loc);},
                15 => {impl_add_attrib!(_impl $mtype, $dty, 15, $mesh, $n, $data, name_str, $loc);},
                16 => {impl_add_attrib!(_impl $mtype, $dty, 16, $mesh, $n, $data, name_str, $loc);},
                _ => (),
            }
        }
    };
    // *** String Attributes ***
    // Points only attributes
    (_impl_str_points $mesh:ident, $loc:ident, $name:ident, $update_fn:expr) => {
        {
            if let HRAttribLocation::HR_VERTEX = $loc {
                (*$mesh).mesh.add_indirect_attrib::<_, topo::VertexIndex>($name, String::new())
                    .and_then(|(attrib, cache)| { attrib.indirect_update_with($update_fn, cache)?; Ok(()) })
                    .map_err(Error::from)
            } else {
                Err(Error::Internal)
            }
        }
    };
    // Surface type meshes like tri- or quad-meshes typically have face attributes but no
    // cell attributes.
    (_impl_str_surface $mesh:ident, $loc:ident, $name:ident, $update_fn:expr) => {
        {
            match $loc {
                HRAttribLocation::HR_VERTEX => {
                    (*$mesh).mesh.add_indirect_attrib::<_, topo::VertexIndex>($name, String::new())
                        .and_then(|(attrib, cache)| { attrib.indirect_update_with($update_fn, cache)?; Ok(()) })
                        .map_err(Error::from)
                },
                HRAttribLocation::HR_FACE => {
                    (*$mesh).mesh.add_indirect_attrib::<_, topo::FaceIndex>($name, String::new())
                        .and_then(|(attrib, cache)| { attrib.indirect_update_with($update_fn, cache)?; Ok(()) })
                        .map_err(Error::from)
                },
                HRAttribLocation::HR_FACEVERTEX => {
                    (*$mesh).mesh.add_indirect_attrib::<_, topo::FaceVertexIndex>($name, String::new())
                        .and_then(|(attrib, cache)| { attrib.indirect_update_with($update_fn, cache)?; Ok(()) })
                        .map_err(Error::from)
                },
                _ => Err(Error::Internal),
            }
        }
    };
    // Volume type meshes like tet and hex meshes have cell attributes.
    (_impl_str_volume $mesh:ident, $loc:ident, $name:ident, $update_fn:expr) => {
        {
            match $loc {
                HRAttribLocation::HR_VERTEX => {
                    (*$mesh).mesh.add_indirect_attrib::<_, topo::VertexIndex>($name, String::new())
                        .and_then(|(attrib, cache)| { attrib.indirect_update_with($update_fn, cache)?; Ok(()) })
                        .map_err(Error::from)
                },
                HRAttribLocation::HR_CELL => {
                    (*$mesh).mesh.add_indirect_attrib::<_, topo::CellIndex>($name, String::new())
                        .and_then(|(attrib, cache)| { attrib.indirect_update_with($update_fn, cache)?; Ok(()) })
                        .map_err(Error::from)
                },
                HRAttribLocation::HR_CELLVERTEX => {
                    (*$mesh).mesh.add_indirect_attrib::<_, topo::CellVertexIndex>($name, String::new())
                        .and_then(|(attrib, cache)| { attrib.indirect_update_with($update_fn, cache)?; Ok(()) })
                        .map_err(Error::from)
                },
                _ => Err(Error::Internal),
            }
        }
    };
    // Helpers for the implementation for string attributes below.
    (_impl_str HR_PointCloud, $mesh:ident, $loc:ident, $name:ident, $update_fn:expr) => {
        impl_add_attrib!(_impl_str_points $mesh, $loc, $name, $update_fn)
    };
    (_impl_str HR_PolyMesh, $mesh:ident, $loc:ident, $name:ident, $update_fn:expr) => {
        impl_add_attrib!(_impl_str_surface $mesh, $loc, $name, $update_fn)
    };
    (_impl_str HR_TetMesh, $mesh:ident, $loc:ident, $name:ident, $update_fn:expr) => {
        impl_add_attrib!(_impl_str_volume $mesh, $loc, $name, $update_fn)
    };
    // Implementation for string attributes
    ($mesh_type:ident, $mesh:ident,
     $loc:ident, $name:ident, $tuple_size:ident,
     $nstrings:ident, $strings:ident, $len:ident, $data:ident) => {
        assert!(!$mesh.is_null(), "Can't add attributes to a null pointer.");
        assert!($tuple_size == 1, "Only 1 dimensional string attributes currently supported.");
        if $tuple_size != 1 {
            return;
        }

        if let Ok(name_str) = CStr::from_ptr($name).to_str() {
            let indices = slice::from_raw_parts($data, $len);
            let update = |i: usize, _: &Arc<_>| {
                let idx = indices[i];
                if idx >= 0 {
                    assert!(idx < $nstrings as i64);
                    let cstr = *$strings.offset(idx as isize);
                    if let Ok(s) = CStr::from_ptr(cstr).to_str() {
                        return Some(std::sync::Arc::new(String::from(s)));
                    }
                }
                None
            };

            let res = impl_add_attrib!(_impl_str $mesh_type, $mesh, $loc, name_str, update);
            match res {
                Err(error) =>
                    println!("Warning: failed to add string attribute \"{}\" at {:?}, with error: {:?}", name_str, $loc, error),
                _ => {}
            }
        }
    }
}

/// If the given mesh is null, this function will panic.
#[no_mangle]
pub unsafe extern "C" fn hr_add_pointcloud_attrib_f32(
    mesh: *mut HR_PointCloud,
    loc: HRAttribLocation,
    name: *const c_char,
    tuple_size: size_t,
    len: size_t,
    data: *const c_float,
) {
    impl_add_attrib!(
        HR_PointCloud,
        mesh,
        loc,
        name,
        tuple_size,
        len,
        data: c_float
    );
}

/// If the given mesh is null, this function will panic.
#[no_mangle]
pub unsafe extern "C" fn hr_add_pointcloud_attrib_f64(
    mesh: *mut HR_PointCloud,
    loc: HRAttribLocation,
    name: *const c_char,
    tuple_size: size_t,
    len: size_t,
    data: *const c_double,
) {
    impl_add_attrib!(
        HR_PointCloud,
        mesh,
        loc,
        name,
        tuple_size,
        len,
        data: c_double
    );
}

/// If the given mesh is null, this function will panic.
#[no_mangle]
pub unsafe extern "C" fn hr_add_pointcloud_attrib_i8(
    mesh: *mut HR_PointCloud,
    loc: HRAttribLocation,
    name: *const c_char,
    tuple_size: size_t,
    len: size_t,
    data: *const c_schar,
) {
    impl_add_attrib!(
        HR_PointCloud,
        mesh,
        loc,
        name,
        tuple_size,
        len,
        data: c_schar
    );
}

/// If the given mesh is null, this function will panic.
#[no_mangle]
pub unsafe extern "C" fn hr_add_pointcloud_attrib_i32(
    mesh: *mut HR_PointCloud,
    loc: HRAttribLocation,
    name: *const c_char,
    tuple_size: size_t,
    len: size_t,
    data: *const c_int,
) {
    impl_add_attrib!(HR_PointCloud, mesh, loc, name, tuple_size, len, data: c_int);
}

/// If the given mesh is null, this function will panic.
#[no_mangle]
pub unsafe extern "C" fn hr_add_pointcloud_attrib_i64(
    mesh: *mut HR_PointCloud,
    loc: HRAttribLocation,
    name: *const c_char,
    tuple_size: size_t,
    len: size_t,
    data: *const i64,
) {
    impl_add_attrib!(HR_PointCloud, mesh, loc, name, tuple_size, len, data: i64);
}

/// If the given mesh is null, this function will panic.
#[no_mangle]
pub unsafe extern "C" fn hr_add_polymesh_attrib_f32(
    mesh: *mut HR_PolyMesh,
    loc: HRAttribLocation,
    name: *const c_char,
    tuple_size: size_t,
    len: size_t,
    data: *const c_float,
) {
    impl_add_attrib!(HR_PolyMesh, mesh, loc, name, tuple_size, len, data: c_float);
}

/// If the given mesh is null, this function will panic.
#[no_mangle]
pub unsafe extern "C" fn hr_add_polymesh_attrib_f64(
    mesh: *mut HR_PolyMesh,
    loc: HRAttribLocation,
    name: *const c_char,
    tuple_size: size_t,
    len: size_t,
    data: *const c_double,
) {
    impl_add_attrib!(
        HR_PolyMesh,
        mesh,
        loc,
        name,
        tuple_size,
        len,
        data: c_double
    );
}

/// If the given mesh is null, this function will panic.
#[no_mangle]
pub unsafe extern "C" fn hr_add_polymesh_attrib_i8(
    mesh: *mut HR_PolyMesh,
    loc: HRAttribLocation,
    name: *const c_char,
    tuple_size: size_t,
    len: size_t,
    data: *const c_schar,
) {
    impl_add_attrib!(HR_PolyMesh, mesh, loc, name, tuple_size, len, data: c_schar);
}

/// If the given mesh is null, this function will panic.
#[no_mangle]
pub unsafe extern "C" fn hr_add_polymesh_attrib_i32(
    mesh: *mut HR_PolyMesh,
    loc: HRAttribLocation,
    name: *const c_char,
    tuple_size: size_t,
    len: size_t,
    data: *const c_int,
) {
    impl_add_attrib!(HR_PolyMesh, mesh, loc, name, tuple_size, len, data: c_int);
}

/// If the given mesh is null, this function will panic.
#[no_mangle]
pub unsafe extern "C" fn hr_add_polymesh_attrib_i64(
    mesh: *mut HR_PolyMesh,
    loc: HRAttribLocation,
    name: *const c_char,
    tuple_size: size_t,
    len: size_t,
    data: *const i64,
) {
    impl_add_attrib!(HR_PolyMesh, mesh, loc, name, tuple_size, len, data: i64);
}

/// If the given mesh is null, this function will panic.
#[no_mangle]
pub unsafe extern "C" fn hr_add_tetmesh_attrib_f32(
    mesh: *mut HR_TetMesh,
    loc: HRAttribLocation,
    name: *const c_char,
    tuple_size: size_t,
    len: size_t,
    data: *const c_float,
) {
    impl_add_attrib!(HR_TetMesh, mesh, loc, name, tuple_size, len, data: c_float);
}

/// If the given mesh is null, this function will panic.
#[no_mangle]
pub unsafe extern "C" fn hr_add_tetmesh_attrib_f64(
    mesh: *mut HR_TetMesh,
    loc: HRAttribLocation,
    name: *const c_char,
    tuple_size: size_t,
    len: size_t,
    data: *const c_double,
) {
    impl_add_attrib!(HR_TetMesh, mesh, loc, name, tuple_size, len, data: c_double);
}

/// If the given mesh is null, this function will panic.
#[no_mangle]
pub unsafe extern "C" fn hr_add_tetmesh_attrib_i8(
    mesh: *mut HR_TetMesh,
    loc: HRAttribLocation,
    name: *const c_char,
    tuple_size: size_t,
    len: size_t,
    data: *const c_schar,
) {
    impl_add_attrib!(HR_TetMesh, mesh, loc, name, tuple_size, len, data: c_schar);
}

/// If the given mesh is null, this function will panic.
#[no_mangle]
pub unsafe extern "C" fn hr_add_tetmesh_attrib_i32(
    mesh: *mut HR_TetMesh,
    loc: HRAttribLocation,
    name: *const c_char,
    tuple_size: size_t,
    len: size_t,
    data: *const c_int,
) {
    impl_add_attrib!(HR_TetMesh, mesh, loc, name, tuple_size, len, data: c_int);
}

/// If the given mesh is null, this function will panic.
#[no_mangle]
pub unsafe extern "C" fn hr_add_tetmesh_attrib_i64(
    mesh: *mut HR_TetMesh,
    loc: HRAttribLocation,
    name: *const c_char,
    tuple_size: size_t,
    len: size_t,
    data: *const i64,
) {
    impl_add_attrib!(HR_TetMesh, mesh, loc, name, tuple_size, len, data: i64);
}

/// If the given mesh is null, this function will panic.
#[no_mangle]
pub unsafe extern "C" fn hr_add_pointcloud_attrib_str(
    mesh: *mut HR_PointCloud,
    loc: HRAttribLocation,
    name: *const c_char,
    tuple_size: size_t,
    nstrings: size_t,
    strings: *const *const c_char,
    len: size_t,
    data: *const i64,
) {
    impl_add_attrib!(
        HR_PointCloud,
        mesh,
        loc,
        name,
        tuple_size,
        nstrings,
        strings,
        len,
        data
    );
}

/// If the given mesh is null, this function will panic.
#[no_mangle]
pub unsafe extern "C" fn hr_add_polymesh_attrib_str(
    mesh: *mut HR_PolyMesh,
    loc: HRAttribLocation,
    name: *const c_char,
    tuple_size: size_t,
    nstrings: size_t,
    strings: *const *const c_char,
    len: size_t,
    data: *const i64,
) {
    impl_add_attrib!(
        HR_PolyMesh,
        mesh,
        loc,
        name,
        tuple_size,
        nstrings,
        strings,
        len,
        data
    );
}

/// If the given mesh is null, this function will panic.
#[no_mangle]
pub unsafe extern "C" fn hr_add_tetmesh_attrib_str(
    mesh: *mut HR_TetMesh,
    loc: HRAttribLocation,
    name: *const c_char,
    tuple_size: size_t,
    nstrings: size_t,
    strings: *const *const c_char,
    len: size_t,
    data: *const i64,
) {
    impl_add_attrib!(HR_TetMesh, mesh, loc, name, tuple_size, nstrings, strings, len, data);
}

/// Helper routine for converting C-style data to `[T;3]`s.
/// `num` is the number of arrays to output, which means that `data_ptr` must point to an array of
/// `n*3` elements.
unsafe fn ptr_to_vec_of_triples<T: Copy>(num_elem: usize, data_ptr: *const T) -> Vec<[T; 3]> {
    let mut data = Vec::with_capacity(num_elem);
    for i in 0..num_elem as isize {
        data.push([
            *data_ptr.offset(3 * i),
            *data_ptr.offset(3 * i + 1),
            *data_ptr.offset(3 * i + 2),
        ]);
    }
    data
}

/*
 * Buffer stuff
 */
#[repr(C)]
pub struct HR_ByteBuffer {
    data: *const c_char,
    size: usize,
}

impl Default for HR_ByteBuffer {
    fn default() -> Self {
        HR_ByteBuffer {
            data: std::ptr::null(),
            size: 0,
        }
    }
}

impl From<Vec<u8>> for HR_ByteBuffer {
    fn from(v: Vec<u8>) -> Self {
        let boxed_data = v.into_boxed_slice();
        let size = boxed_data.len();

        HR_ByteBuffer {
            data: Box::into_raw(boxed_data) as *const c_char,
            size,
        }
    }
}

impl HR_ByteBuffer {
    fn write_legacy_vtk(vtk: Vtk) -> Self {
        let mut vec_data = Vec::<u8>::new();
        vtk.write_legacy(&mut vec_data).expect("Failed to write Vtk data to byte buffer");
        vec_data.into()
    }
    fn write_xml_vtk(vtk: Vtk) -> Self {
        let mut vec_data = Vec::<u8>::new();
        vtk.write_xml(&mut vec_data).expect("Failed to write Vtk data to byte buffer");
        vec_data.into()
    }
}

impl From<ObjData> for HR_ByteBuffer {
    fn from(obj: ObjData) -> Self {
        let mut vec_data = Vec::<u8>::new();
        obj.write_to_buf(&mut vec_data)
            .expect("Failed to write Obj data to byte buffer");
        vec_data.into()
    }
}

#[no_mangle]
pub unsafe extern "C" fn hr_free_byte_buffer(buf: HR_ByteBuffer) {
    if !buf.data.is_null() && buf.size > 0 {
        let slice = slice::from_raw_parts_mut(buf.data as *mut u8, buf.size);
        let _: Box<[u8]> = Box::from_raw(slice as *mut [u8]);
    }
}

/// Write the given HR_PolyMesh as a polygon mesh in XML VTK format returned through an appropriately sized
/// `HR_ByteBuffer`.
#[no_mangle]
pub unsafe extern "C" fn hr_make_polymesh_vtp_buffer(mesh: *const HR_PolyMesh) -> HR_ByteBuffer {
    // check invariants
    assert!(!mesh.is_null());

    convert_polymesh_to_vtk_format(&(*mesh).mesh, VTKPolyExportStyle::PolyData)
        .map(HR_ByteBuffer::write_xml_vtk)
        .unwrap_or_else(|_| Default::default())
}

/// Write the given HR_PointCloud as a polygon mesh in XML VTK format returned through an appropriately sized
/// `HR_ByteBuffer`.
#[no_mangle]
pub unsafe extern "C" fn hr_make_pointcloud_vtp_buffer(
    mesh: *const HR_PointCloud,
) -> HR_ByteBuffer {
    // check invariants
    assert!(!mesh.is_null());

    convert_pointcloud_to_vtk_format(&(*mesh).mesh, VTKPolyExportStyle::PolyData)
        .map(HR_ByteBuffer::write_xml_vtk)
        .unwrap_or_else(|_| Default::default())
}

/// Write the given HR_TetMesh as an unstructured grid in XML VTK format returned through an appropriately sized
/// `HR_ByteBuffer`.
#[no_mangle]
pub unsafe extern "C" fn hr_make_tetmesh_vtu_buffer(mesh: *const HR_TetMesh) -> HR_ByteBuffer {
    // check invariants
    assert!(!mesh.is_null());

    convert_tetmesh_to_vtk_format(&(*mesh).mesh)
        .map(HR_ByteBuffer::write_xml_vtk)
        .unwrap_or_else(|_| Default::default())
}

/// Write the given HR_PolyMesh as an unstructured grid in XML VTK format returned through an appropriately sized
/// `HR_ByteBuffer`.
#[no_mangle]
pub unsafe extern "C" fn hr_make_polymesh_vtu_buffer(mesh: *const HR_PolyMesh) -> HR_ByteBuffer {
    // check invariants
    assert!(!mesh.is_null());

    convert_polymesh_to_vtk_format(&(*mesh).mesh, VTKPolyExportStyle::UnstructuredGrid)
        .map(HR_ByteBuffer::write_xml_vtk)
        .unwrap_or_else(|_| Default::default())
}

/// Write the given HR_PointCloud as an unstructured grid in XML VTK format returned through an appropriately sized
/// `HR_ByteBuffer`.
#[no_mangle]
pub unsafe extern "C" fn hr_make_pointcloud_vtu_buffer(
    mesh: *const HR_PointCloud,
) -> HR_ByteBuffer {
    // check invariants
    assert!(!mesh.is_null());

    convert_pointcloud_to_vtk_format(&(*mesh).mesh, VTKPolyExportStyle::UnstructuredGrid)
        .map(HR_ByteBuffer::write_xml_vtk)
        .unwrap_or_else(|_| Default::default())
}

/// Write the given HR_TetMesh into a binary VTK format returned through an appropriately sized
/// `HR_ByteBuffer`.
#[no_mangle]
pub unsafe extern "C" fn hr_make_tetmesh_vtk_buffer(mesh: *const HR_TetMesh) -> HR_ByteBuffer {
    // check invariants
    assert!(!mesh.is_null());

    convert_tetmesh_to_vtk_format(&(*mesh).mesh)
        .map(HR_ByteBuffer::write_legacy_vtk)
        .unwrap_or_else(|_| Default::default())
}

/// Write the given HR_PolyMesh into a binary VTK format returned through an appropriately sized
/// `HR_ByteBuffer`.
#[no_mangle]
pub unsafe extern "C" fn hr_make_polymesh_vtk_buffer(mesh: *const HR_PolyMesh) -> HR_ByteBuffer {
    // check invariants
    assert!(!mesh.is_null());

    convert_polymesh_to_vtk_format(&(*mesh).mesh, VTKPolyExportStyle::PolyData)
        .map(HR_ByteBuffer::write_legacy_vtk)
        .unwrap_or_else(|_| Default::default())
}

/// Write the given HR_PointCloud into a binary VTK format returned through an appropriately sized
/// `HR_ByteBuffer`.
#[no_mangle]
pub unsafe extern "C" fn hr_make_pointcloud_vtk_buffer(
    mesh: *const HR_PointCloud,
) -> HR_ByteBuffer {
    // check invariants
    assert!(!mesh.is_null());

    convert_pointcloud_to_vtk_format(&(*mesh).mesh, VTKPolyExportStyle::PolyData)
        .map(HR_ByteBuffer::write_legacy_vtk)
        .unwrap_or_else(|_| Default::default())
}

/// Write the given `HR_PolyMesh` into the Obj format returned through an appropriately sized
/// `HR_ByteBuffer`.
#[no_mangle]
pub unsafe extern "C" fn hr_make_polymesh_obj_buffer(mesh: *const HR_PolyMesh) -> HR_ByteBuffer {
    // check invariants
    assert!(!mesh.is_null());

    convert_polymesh_to_obj_format(&(*mesh).mesh)
        .map(From::from)
        .unwrap_or_else(|_| Default::default())
}

/// Write the given `HR_PointCloud` into the Obj format returned through an appropriately sized
/// `HR_ByteBuffer`.
#[no_mangle]
pub unsafe extern "C" fn hr_make_pointcloud_obj_buffer(
    mesh: *const HR_PointCloud,
) -> HR_ByteBuffer {
    // check invariants
    assert!(!mesh.is_null());

    convert_pointcloud_to_obj_format(&(*mesh).mesh)
        .map(From::from)
        .unwrap_or_else(|_| Default::default())
}

#[derive(Debug)]
#[repr(C)]
pub enum HRMeshType {
    HR_TETMESH,
    HR_POLYMESH,
    HR_NONE,
}

#[derive(Debug)]
#[repr(C)]
pub struct HR_Mesh {
    tetmesh: *mut HR_TetMesh,
    polymesh: *mut HR_PolyMesh,
    tag: HRMeshType,
}

impl Default for HR_Mesh {
    fn default() -> Self {
        HR_Mesh {
            tetmesh: ::std::ptr::null_mut(),
            polymesh: ::std::ptr::null_mut(),
            tag: HRMeshType::HR_NONE,
        }
    }
}

/// Helper to convert the given VTK data set into a valid `HR_Mesh` type.
///
/// In case of failure `None` is returned.
fn convert_vtk_polymesh_to_hr_mesh(vtk: Vtk) -> Option<HR_Mesh> {
    if let Ok(mesh) = convert_vtk_to_polymesh(vtk) {
        if mesh.num_faces() > 0 {
            let polymesh = Box::new(HR_PolyMesh { mesh });
            return Some(HR_Mesh {
                tag: HRMeshType::HR_POLYMESH,
                polymesh: Box::into_raw(polymesh),
                ..HR_Mesh::default()
            });
        }
    }
    None
}

/// Parse a given byte array into a HR_PolyMesh depending on what is stored in the
/// buffer assuming polygon VTK format.
#[no_mangle]
pub unsafe extern "C" fn hr_parse_vtp_mesh(data: *const c_char, size: size_t) -> HR_Mesh {
    if data.is_null() || size == 0 {
        return HR_Mesh::default();
    }

    let slice = slice::from_raw_parts_mut(data as *mut u8, size);

    if let Ok(vtk) = Vtk::parse_xml(&*slice) {
        convert_vtk_polymesh_to_hr_mesh(vtk).unwrap_or_else(HR_Mesh::default)
    } else {
        HR_Mesh::default()
    }
}

/// Parse a given byte array into a HR_TetMesh or a HR_PolyMesh depending on what is stored in the
/// buffer assuming unstructured grid VTK format.
#[no_mangle]
pub unsafe extern "C" fn hr_parse_vtu_mesh(data: *const c_char, size: size_t) -> HR_Mesh {
    if data.is_null() || size == 0 {
        return HR_Mesh::default();
    }

    let slice = slice::from_raw_parts_mut(data as *mut u8, size);

    if let Ok(vtk) = Vtk::parse_xml(&*slice) {
        if let Ok(mesh) = convert_vtk_to_tetmesh(vtk.clone()) {
            if mesh.num_cells() > 0 {
                let tetmesh = Box::new(HR_TetMesh { mesh });
                return HR_Mesh {
                    tag: HRMeshType::HR_TETMESH,
                    tetmesh: Box::into_raw(tetmesh),
                    ..HR_Mesh::default()
                };
            }
        }

        return convert_vtk_polymesh_to_hr_mesh(vtk).unwrap_or_else(HR_Mesh::default);
    }
    HR_Mesh::default()
}

/// Parse a given byte array into a HR_TetMesh or a HR_PolyMesh depending on what is stored in the
/// buffer assuming VTK format.
#[no_mangle]
pub unsafe extern "C" fn hr_parse_vtk_mesh(data: *const c_char, size: size_t) -> HR_Mesh {
    if data.is_null() || size == 0 {
        return HR_Mesh::default();
    }

    let slice = slice::from_raw_parts_mut(data as *mut u8, size);

    if let Ok(vtk) = Vtk::parse_legacy_be(&*slice) {
        if let Ok(mesh) = convert_vtk_to_tetmesh(vtk.clone()) {
            if mesh.num_cells() > 0 {
                let tetmesh = Box::new(HR_TetMesh { mesh });
                return HR_Mesh {
                    tag: HRMeshType::HR_TETMESH,
                    tetmesh: Box::into_raw(tetmesh),
                    ..HR_Mesh::default()
                };
            }
        }

        return convert_vtk_polymesh_to_hr_mesh(vtk).unwrap_or_else(HR_Mesh::default);
    }
    HR_Mesh::default()
}

/// Parse a given byte array into a HR_PolyMesh assuming obj format.
#[no_mangle]
pub unsafe extern "C" fn hr_parse_obj_mesh(data: *const c_char, size: size_t) -> HR_Mesh {
    if data.is_null() || size == 0 {
        return HR_Mesh::default();
    }

    let slice = slice::from_raw_parts_mut(data as *mut u8, size);

    if let Ok(obj_data) = ObjData::load_buf_with_config(&*slice, LoadConfig { strict: false }) {
        if let Ok(mesh) = convert_obj_to_polymesh(obj_data) {
            if mesh.num_faces() > 0 {
                let polymesh = Box::new(HR_PolyMesh { mesh });
                return HR_Mesh {
                    tag: HRMeshType::HR_POLYMESH,
                    polymesh: Box::into_raw(polymesh),
                    ..HR_Mesh::default()
                };
            }
        }
    }
    HR_Mesh::default()
}
