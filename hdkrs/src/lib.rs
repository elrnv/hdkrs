use std::any::TypeId;
use std::iter::Peekable;
use std::sync::Arc;

use gut::mesh::{self, attrib, topology as topo, Attrib, VertexPositions};
use hashbrown::hash_map::Iter;

pub mod interop;

#[cxx::bridge(namespace = "hdkrs")]
pub mod ffi {
    #[namespace = ""]
    unsafe extern "C++" {
        include!("hdkrs/mesh.h");
        include!("hdkrs/interrupt.h");
        type GU_Detail;
        fn impl_shared_ptr(detail: SharedPtr<GU_Detail>);
    }

    unsafe extern "C++" {
        type InterruptChecker;
        fn check_interrupt(self: Pin<&mut InterruptChecker>) -> bool;
        fn new_interrupt_checker(message: &CxxString) -> UniquePtr<InterruptChecker>;
    }

    unsafe extern "C++" {
        fn add_polymesh(detail: Pin<&mut GU_Detail>, polymesh: &PolyMesh);
        fn add_tetmesh(detail: Pin<&mut GU_Detail>, tetmesh: &TetMesh);
        fn add_pointcloud(detail: Pin<&mut GU_Detail>, ptcloud: &PointCloud);
        fn update_points(detail: Pin<&mut GU_Detail>, ptcloud: &PointCloud);

        fn build_polymesh(detail: &GU_Detail) -> Result<Box<PolyMesh>>;
        fn build_tetmesh(detail: &GU_Detail) -> Result<Box<TetMesh>>;
        fn build_pointcloud(detail: &GU_Detail) -> Result<Box<PointCloud>>;
    }
    extern "Rust" {
        type PolyMesh;
        fn get_point_coords(&self) -> Vec<f64>;
        fn get_indices(&self) -> Vec<usize>;
        fn attrib_iter(&self, loc: AttribLocation) -> Box<AttribIter>;
        fn add_attrib_f32(
            &mut self,
            loc: AttribLocation,
            name: &str,
            tuple_size: usize,
            data: &[f32],
        );
        fn add_attrib_f64(
            &mut self,
            loc: AttribLocation,
            name: &str,
            tuple_size: usize,
            data: &[f64],
        );
        fn add_attrib_i8(
            &mut self,
            loc: AttribLocation,
            name: &str,
            tuple_size: usize,
            data: &[i8],
        );
        fn add_attrib_i32(
            &mut self,
            loc: AttribLocation,
            name: &str,
            tuple_size: usize,
            data: &[i32],
        );
        fn add_attrib_i64(
            &mut self,
            loc: AttribLocation,
            name: &str,
            tuple_size: usize,
            data: &[i64],
        );
        fn add_attrib_str(
            &mut self,
            loc: AttribLocation,
            name: &str,
            tuple_size: usize,
            strings: &[&str],
            data: &[i64],
        );
    }
    extern "Rust" {
        type TetMesh;
        fn get_point_coords(&self) -> Vec<f64>;
        fn get_indices(&self) -> Vec<usize>;
        fn attrib_iter(&self, loc: AttribLocation) -> Box<AttribIter>;
        fn add_attrib_f32(
            &mut self,
            loc: AttribLocation,
            name: &str,
            tuple_size: usize,
            data: &[f32],
        );
        fn add_attrib_f64(
            &mut self,
            loc: AttribLocation,
            name: &str,
            tuple_size: usize,
            data: &[f64],
        );
        fn add_attrib_i8(
            &mut self,
            loc: AttribLocation,
            name: &str,
            tuple_size: usize,
            data: &[i8],
        );
        fn add_attrib_i32(
            &mut self,
            loc: AttribLocation,
            name: &str,
            tuple_size: usize,
            data: &[i32],
        );
        fn add_attrib_i64(
            &mut self,
            loc: AttribLocation,
            name: &str,
            tuple_size: usize,
            data: &[i64],
        );
        fn add_attrib_str(
            &mut self,
            loc: AttribLocation,
            name: &str,
            tuple_size: usize,
            strings: &[&str],
            data: &[i64],
        );
    }
    extern "Rust" {
        type PointCloud;
        fn get_point_coords(&self) -> Vec<f64>;
        fn attrib_iter(&self, loc: AttribLocation) -> Box<AttribIter>;
        fn add_attrib_f32(
            &mut self,
            loc: AttribLocation,
            name: &str,
            tuple_size: usize,
            data: &[f32],
        );
        fn add_attrib_f64(
            &mut self,
            loc: AttribLocation,
            name: &str,
            tuple_size: usize,
            data: &[f64],
        );
        fn add_attrib_i8(
            &mut self,
            loc: AttribLocation,
            name: &str,
            tuple_size: usize,
            data: &[i8],
        );
        fn add_attrib_i32(
            &mut self,
            loc: AttribLocation,
            name: &str,
            tuple_size: usize,
            data: &[i32],
        );
        fn add_attrib_i64(
            &mut self,
            loc: AttribLocation,
            name: &str,
            tuple_size: usize,
            data: &[i64],
        );
        fn add_attrib_str(
            &mut self,
            loc: AttribLocation,
            name: &str,
            tuple_size: usize,
            strings: &[&str],
            data: &[i64],
        );
    }
    #[rustfmt::skip]
    extern "Rust" {
        type AttribIter;
        fn has_next(&mut self) -> bool;
        unsafe fn next<'a>(self: &mut AttribIter<'a>) -> Result<Box<Attribute<'a>>>;
    }
    extern "Rust" {
        type TupleVecStr;
        fn len(&self) -> usize;
        fn tuple_size(&self) -> usize;
        fn at(&self, i: usize) -> &str;
    }
    #[rustfmt::skip]
    extern "Rust" {
        type Attribute<'a>;
        unsafe fn name<'b>(&'b self) -> &'b str;
        fn data_type(&self) -> DataType;
        fn get_data_i8(&self) -> TupleVecI8;
        fn get_data_i32(&self) -> TupleVecI32;
        fn get_data_i64(&self) -> TupleVecI64;
        fn get_data_f32(&self) -> TupleVecF32;
        fn get_data_f64(&self) -> TupleVecF64;
        fn get_data_str(&self) -> Box<TupleVecStr>;
    }
    extern "Rust" {
        pub type Mesh;
        fn is_tetmesh(&self) -> bool;
        fn is_polymesh(&self) -> bool;
        fn is_pointcloud(&self) -> bool;
        fn tag(&self) -> MeshTag;
        fn add_to_detail(&self, detail: Pin<&mut GU_Detail>);
        fn into_tetmesh(mesh: Box<Mesh>) -> Box<TetMesh>;
        fn into_polymesh(mesh: Box<Mesh>) -> Box<PolyMesh>;
        fn into_pointcloud(mesh: Box<Mesh>) -> Box<PointCloud>;
    }
    extern "Rust" {
        fn make_pointcloud(coords: &[f64]) -> Box<PointCloud>;
        fn make_polymesh(coords: &[f64], indices: &[usize]) -> Box<PolyMesh>;
        fn make_tetmesh(coords: &[f64], indices: &[usize]) -> Box<TetMesh>;
    }

    #[derive(Debug)]
    pub struct TupleVecI8 {
        vec: Vec<i8>,
        tuple_size: usize,
    }
    #[derive(Debug)]
    pub struct TupleVecI32 {
        vec: Vec<i32>,
        tuple_size: usize,
    }
    #[derive(Debug)]
    pub struct TupleVecI64 {
        vec: Vec<i64>,
        tuple_size: usize,
    }
    #[derive(Debug)]
    pub struct TupleVecF32 {
        vec: Vec<f32>,
        tuple_size: usize,
    }
    #[derive(Debug)]
    pub struct TupleVecF64 {
        vec: Vec<f64>,
        tuple_size: usize,
    }
    #[derive(Debug)]
    pub enum CookResultTag {
        SUCCESS,
        WARNING,
        ERROR,
    }

    #[derive(Debug)]
    pub struct CookResult {
        pub message: String,
        pub tag: CookResultTag,
    }

    #[derive(Debug)]
    pub enum AttribLocation {
        VERTEX,
        FACE,
        CELL,
        FACEVERTEX,
        CELLVERTEX,
    }

    #[derive(Debug)]
    pub enum DataType {
        I8,
        I32,
        I64,
        F32,
        F64,
        STR,
        UNSUPPORTED,
    }

    #[derive(Debug)]
    pub enum MeshTag {
        TetMesh,
        PolyMesh,
        PointCloud,
        None,
    }
}

use self::ffi::*;

unsafe impl Send for InterruptChecker {}
unsafe impl Sync for InterruptChecker {}

/// A Rust polygon mesh struct.
#[derive(Clone, PartialEq, Debug)]
#[repr(transparent)]
pub struct PolyMesh(pub mesh::PolyMesh<f64>);

impl From<mesh::PolyMesh<f64>> for PolyMesh {
    fn from(m: mesh::PolyMesh<f64>) -> Self {
        PolyMesh(m)
    }
}

/// A Rust tetmesh struct.
#[derive(Clone, PartialEq, Debug)]
#[repr(transparent)]
pub struct TetMesh(pub mesh::TetMesh<f64>);

impl From<mesh::TetMesh<f64>> for TetMesh {
    fn from(m: mesh::TetMesh<f64>) -> Self {
        TetMesh(m)
    }
}

/// A Rust pointcloud struct.
#[derive(Clone, PartialEq, Debug)]
#[repr(transparent)]
pub struct PointCloud(pub mesh::PointCloud<f64>);

impl From<mesh::PointCloud<f64>> for PointCloud {
    fn from(m: mesh::PointCloud<f64>) -> Self {
        PointCloud(m)
    }
}

// TODO: Arc is not currently supported by cxx. Move this into extern "Rust".
pub struct TupleVecStr {
    vec: Vec<std::sync::Arc<String>>,
    tuple_size: usize,
}

impl TupleVecStr {
    fn len(&self) -> usize {
        self.vec.len()
    }
    fn tuple_size(&self) -> usize {
        self.tuple_size
    }
    fn at(&self, i: usize) -> &str {
        self.vec[i].as_str()
    }
}

pub struct TupleVec<T> {
    vec: Vec<T>,
    tuple_size: usize,
}

macro_rules! impl_tuple_vec {
    ($ty:ty, $tuple_vec:ident) => {
        impl From<TupleVec<$ty>> for $tuple_vec {
            fn from(v: TupleVec<$ty>) -> Self {
                $tuple_vec {
                    vec: v.vec,
                    tuple_size: v.tuple_size,
                }
            }
        }
    };
}

impl_tuple_vec!(i8, TupleVecI8);
impl_tuple_vec!(i32, TupleVecI32);
impl_tuple_vec!(i64, TupleVecI64);
impl_tuple_vec!(f32, TupleVecF32);
impl_tuple_vec!(f64, TupleVecF64);
impl_tuple_vec!(std::sync::Arc<String>, TupleVecStr);

impl PointCloud {
    pub fn get_point_coords(&self) -> Vec<f64> {
        bytemuck::cast_slice(self.0.vertex_positions()).to_vec()
    }
    pub fn attrib_iter(&self, loc: AttribLocation) -> Box<AttribIter<'_>> {
        Box::new(match loc {
            AttribLocation::VERTEX => {
                AttribIter::Vertex(self.0.attrib_dict::<topo::VertexIndex>().iter().peekable())
            }
            _ => AttribIter::None,
        })
    }
}

impl TetMesh {
    pub fn get_point_coords(&self) -> Vec<f64> {
        bytemuck::cast_slice(self.0.vertex_positions()).to_vec()
    }
    pub fn get_indices(&self) -> Vec<usize> {
        let mut indices = Vec::new();
        for cell in self.0.cell_iter() {
            for &idx in cell.iter() {
                indices.push(idx);
            }
        }

        indices
    }

    pub fn attrib_iter(&self, loc: AttribLocation) -> Box<AttribIter<'_>> {
        Box::new(match loc {
            AttribLocation::VERTEX => {
                AttribIter::Vertex(self.0.attrib_dict::<topo::VertexIndex>().iter().peekable())
            }
            AttribLocation::CELL => {
                AttribIter::Cell(self.0.attrib_dict::<topo::CellIndex>().iter().peekable())
            }
            AttribLocation::CELLVERTEX => AttribIter::CellVertex(
                self.0
                    .attrib_dict::<topo::CellVertexIndex>()
                    .iter()
                    .peekable(),
            ),
            _ => AttribIter::None,
        })
    }
}

impl PolyMesh {
    pub fn get_point_coords(&self) -> Vec<f64> {
        bytemuck::cast_slice(self.0.vertex_positions()).to_vec()
    }
    /// Polygon mesh indices is a contiguous set of polygon indices, each in the form:
    /// `n, i_1, i_2, ..., i_n` where `n` is the number of sides on a polygon.
    pub fn get_indices(&self) -> Vec<usize> {
        let mut indices = Vec::new();

        for poly in self.0.face_iter() {
            indices.push(poly.len());
            for &idx in poly.iter() {
                indices.push(idx);
            }
        }

        indices
    }
    pub fn attrib_iter(&self, loc: AttribLocation) -> Box<AttribIter<'_>> {
        Box::new(match loc {
            AttribLocation::VERTEX => {
                AttribIter::Vertex(self.0.attrib_dict::<topo::VertexIndex>().iter().peekable())
            }
            AttribLocation::FACE => {
                AttribIter::Face(self.0.attrib_dict::<topo::FaceIndex>().iter().peekable())
            }
            AttribLocation::FACEVERTEX => AttribIter::FaceVertex(
                self.0
                    .attrib_dict::<topo::FaceVertexIndex>()
                    .iter()
                    .peekable(),
            ),
            _ => AttribIter::None,
        })
    }
}

pub enum AttribIter<'a> {
    Vertex(Peekable<Iter<'a, String, attrib::Attribute<topo::VertexIndex>>>),
    Face(Peekable<Iter<'a, String, attrib::Attribute<topo::FaceIndex>>>),
    Cell(Peekable<Iter<'a, String, attrib::Attribute<topo::CellIndex>>>),
    FaceVertex(Peekable<Iter<'a, String, attrib::Attribute<topo::FaceVertexIndex>>>),
    CellVertex(Peekable<Iter<'a, String, attrib::Attribute<topo::CellVertexIndex>>>),
    None,
}

impl<'a> AttribIter<'a> {
    pub fn has_next(&mut self) -> bool {
        match self {
            AttribIter::Vertex(ref mut iter) => iter.peek().is_some(),
            AttribIter::Face(ref mut iter) => iter.peek().is_some(),
            AttribIter::Cell(ref mut iter) => iter.peek().is_some(),
            AttribIter::FaceVertex(ref mut iter) => iter.peek().is_some(),
            AttribIter::CellVertex(ref mut iter) => iter.peek().is_some(),
            AttribIter::None => false,
        }
    }
    // TODO: Refactor this to return Option when cxx supports it.
    /// Returns the next available attribute.
    ///
    /// If the iterator is at the end or `AttribIter` is `None`, then this function will panic,
    /// so make sure to check with `has_next`.
    pub fn next(&mut self) -> Result<Box<Attribute<'a>>, Error> {
        match self {
            AttribIter::Vertex(ref mut iter) => iter.next().map(|(k, v)| Attribute {
                name: k.clone(),
                data: AttribData::Vertex(v),
            }),
            AttribIter::Face(ref mut iter) => iter.next().map(|(k, v)| Attribute {
                name: k.clone(),
                data: AttribData::Face(v),
            }),
            AttribIter::Cell(ref mut iter) => iter.next().map(|(k, v)| Attribute {
                name: k.clone(),
                data: AttribData::Cell(v),
            }),
            AttribIter::FaceVertex(ref mut iter) => iter.next().map(|(k, v)| Attribute {
                name: k.clone(),
                data: AttribData::FaceVertex(v),
            }),
            AttribIter::CellVertex(ref mut iter) => iter.next().map(|(k, v)| Attribute {
                name: k.clone(),
                data: AttribData::CellVertex(v),
            }),
            AttribIter::None => None,
        }
        .map(|a| Box::new(a))
        .ok_or(Error::AttribNotFound)
    }
}

/// Wrapper around the `attrib::Attribute` struct to eliminate generics for ffi.
#[derive(Debug)]
pub enum AttribData<'a> {
    Vertex(&'a attrib::Attribute<topo::VertexIndex>),
    Face(&'a attrib::Attribute<topo::FaceIndex>),
    Cell(&'a attrib::Attribute<topo::CellIndex>),
    FaceVertex(&'a attrib::Attribute<topo::FaceVertexIndex>),
    CellVertex(&'a attrib::Attribute<topo::CellVertexIndex>),
    None,
}

impl<'a> AttribData<'a> {
    pub fn data_type(&self) -> DataType {
        match self {
            AttribData::Vertex(a) => attrib_type_id(a),
            AttribData::Face(a) => attrib_type_id(a),
            AttribData::Cell(a) => attrib_type_id(a),
            AttribData::FaceVertex(a) => attrib_type_id(a),
            AttribData::CellVertex(a) => attrib_type_id(a),
            AttribData::None => DataType::UNSUPPORTED,
        }
    }
}

/// Opaque type to store data about a particular attribute. This struct owns the string it
/// contains thus it must be freed when done.
#[derive(Debug)]
pub struct Attribute<'a> {
    name: String,
    data: AttribData<'a>,
}

impl<'a> Attribute<'a> {
    pub fn data_type(&self) -> DataType {
        self.data.data_type()
    }
    pub fn name(&self) -> &str {
        self.name.as_str()
    }
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

fn attrib_type_id<I>(attrib: &attrib::Attribute<I>) -> DataType {
    match attrib.data.element_type_id() {
        x if impl_supported_types!(
            x, i8, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16
        ) =>
        {
            DataType::I8
        }
        x if impl_supported_types!(
            x, i32, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16
        ) =>
        {
            DataType::I32
        }
        x if impl_supported_types!(
            x, i64, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16
        ) =>
        {
            DataType::I64
        }
        x if impl_supported_types!(
            x, f32, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16
        ) =>
        {
            DataType::F32
        }
        x if impl_supported_types!(
            x, f64, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16
        ) =>
        {
            DataType::F64
        }
        // Currently we only support a single string, supporting tuples would require a refactor of
        // the gut attribute system.
        x if impl_supported_types!(x, Arc<String>) => DataType::STR,
        _ => DataType::UNSUPPORTED,
    }
}

trait AttribFlatArray {
    fn maybe_str_array<I>(
        _attrib: &attrib::Attribute<I>,
        _tuple_size: usize,
    ) -> Option<TupleVec<Self>>
    where
        Self: Sized,
    {
        None
    }
    fn attrib_flat_array<I>(attrib: &attrib::Attribute<I>) -> TupleVec<Self>
    where
        Self: Sized + Clone + 'static,
    {
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

        // Strings are stored in indirect attributes and cannot be directly cast.
        if let Some(str_array) = Self::maybe_str_array(attrib, tuple_size) {
            return str_array;
        }

        let flat_vec = match tuple_size {
            1 => cast_to_vec!(Self, attrib),
            2 => cast_to_vec!(Self, attrib, 2),
            3 => cast_to_vec!(Self, attrib, 3),
            4 => cast_to_vec!(Self, attrib, 4),
            5 => cast_to_vec!(Self, attrib, 5),
            6 => cast_to_vec!(Self, attrib, 6),
            7 => cast_to_vec!(Self, attrib, 7),
            8 => cast_to_vec!(Self, attrib, 8),
            9 => cast_to_vec!(Self, attrib, 9),
            10 => cast_to_vec!(Self, attrib, 10),
            11 => cast_to_vec!(Self, attrib, 11),
            12 => cast_to_vec!(Self, attrib, 12),
            13 => cast_to_vec!(Self, attrib, 13),
            14 => cast_to_vec!(Self, attrib, 14),
            15 => cast_to_vec!(Self, attrib, 15),
            16 => cast_to_vec!(Self, attrib, 16),
            _ => Vec::new(),
        };

        TupleVec {
            vec: flat_vec,
            tuple_size,
        }
    }
}
impl AttribFlatArray for i8 {}
impl AttribFlatArray for i32 {}
impl AttribFlatArray for i64 {}
impl AttribFlatArray for f32 {}
impl AttribFlatArray for f64 {}
impl AttribFlatArray for std::sync::Arc<String> {
    fn maybe_str_array<I>(
        attrib: &attrib::Attribute<I>,
        tuple_size: usize,
    ) -> Option<TupleVec<Self>> {
        if let Ok(indirect_data_slice) = attrib
            .data
            .indirect_data()
            .and_then(|d| d.as_rc_slice::<String>())
        {
            return Some(TupleVec {
                vec: indirect_data_slice
                    .iter()
                    .map(|s| std::sync::Arc::clone(s))
                    .collect(),
                tuple_size,
            });
        }
        None
    }
}

macro_rules! impl_get_attrib_data {
    ($attrib_data:ident) => {{
        match $attrib_data.data {
            AttribData::Vertex(data) => AttribFlatArray::attrib_flat_array(data),
            AttribData::Face(data) => AttribFlatArray::attrib_flat_array(data),
            AttribData::Cell(data) => AttribFlatArray::attrib_flat_array(data),
            AttribData::FaceVertex(data) => AttribFlatArray::attrib_flat_array(data),
            AttribData::CellVertex(data) => AttribFlatArray::attrib_flat_array(data),
            AttribData::None => TupleVec {
                vec: Vec::new(),
                tuple_size: 0,
            },
        }
    }};
}

impl<'a> Attribute<'a> {
    pub fn get_data_i8(&self) -> TupleVecI8 {
        impl_get_attrib_data!(self).into()
    }
    pub fn get_data_i32(&self) -> TupleVecI32 {
        impl_get_attrib_data!(self).into()
    }
    pub fn get_data_i64(&self) -> TupleVecI64 {
        impl_get_attrib_data!(self).into()
    }
    pub fn get_data_f32(&self) -> TupleVecF32 {
        impl_get_attrib_data!(self).into()
    }
    pub fn get_data_f64(&self) -> TupleVecF64 {
        impl_get_attrib_data!(self).into()
    }
    pub fn get_data_str(&self) -> Box<TupleVecStr> {
        Box::new(impl_get_attrib_data!(self).into())
    }
}

pub fn make_pointcloud(coords: &[f64]) -> Box<PointCloud> {
    use std::convert::TryInto;
    // check invariants
    assert!(
        coords.len() % 3 == 0,
        "Given coordinate array size is not a multiple of 3."
    );
    let verts: Vec<[f64; 3]> = coords
        .chunks_exact(3)
        .map(|chunk| chunk.try_into().unwrap())
        .collect();
    Box::new(PointCloud(mesh::PointCloud::new(verts)))
}

macro_rules! make_mesh_impl {
    ($hr_mesh:ident, $mesh_ty:ident, $coords:ident, $convert:expr) => {{
        use std::convert::TryInto;
        // check invariants
        assert!(
            $coords.len() % 3 == 0,
            "Given coordinate array size is not a multiple of 3."
        );

        let indices = $convert;
        let verts: Vec<[f64; 3]> = $coords
            .chunks_exact(3)
            .map(|chunk| chunk.try_into().unwrap())
            .collect();

        Box::new($hr_mesh(mesh::$mesh_ty::new(verts, indices)))
    }};
}

pub fn make_polymesh(coords: &[f64], indices: &[usize]) -> Box<PolyMesh> {
    make_mesh_impl!(PolyMesh, PolyMesh, coords, indices)
}

pub fn make_tetmesh(coords: &[f64], indices: &[usize]) -> Box<TetMesh> {
    make_mesh_impl!(
        TetMesh,
        TetMesh,
        coords,
        bytemuck::cast_slice(indices).to_vec()
    )
}

#[derive(Debug, PartialEq)]
pub enum Error {
    Attrib(attrib::Error),
    Internal,
    MeshMismatch,
    AttribNotFound,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Attrib(e) => write!(f, "Attribute: {}", e),
            Error::AttribNotFound => write!(f, "Attribute not found"),
            Error::Internal => write!(f, "Internal error"),
            Error::MeshMismatch => write!(f, "Mesh mismatch error"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Attrib(e) => Some(e),
            _ => None,
        }
    }
}

impl From<attrib::Error> for Error {
    fn from(a: attrib::Error) -> Self {
        Error::Attrib(a)
    }
}

macro_rules! impl_add_attrib {
    (_impl PointCloud, $data_type:ty, $mesh:ident,
     $data:ident, $name:ident, $loc:ident) => {
        let vec = $data.to_vec();
        impl_add_attrib!(_impl_points $mesh, $loc, $name, vec);
    };
    (_impl PolyMesh, $data_type:ty, $mesh:ident,
     $data:ident, $name:ident, $loc:ident) => {
        let vec = $data.to_vec();
        impl_add_attrib!(_impl_surface $mesh, $loc, $name, vec);
    };
    (_impl TetMesh, $data_type:ty, $mesh:ident,
     $data:ident, $name:ident, $loc:ident) => {
        let vec = $data.to_vec();
        impl_add_attrib!(_impl_volume $mesh, $loc, $name, vec);
    };
    (_impl PointCloud, $data_type:ty, $tuple_size:expr, $mesh:ident,
     $data:ident, $name:ident, $loc:ident) => {
        let vec = bytemuck::cast_slice::<_, [$data_type; $tuple_size]>($data).to_vec();
        impl_add_attrib!(_impl_points $mesh, $loc, $name, vec);
    };
    (_impl PolyMesh, $data_type:ty, $tuple_size:expr, $mesh:ident,
     $data:ident, $name:ident, $loc:ident) => {
        let vec = bytemuck::cast_slice::<_, [$data_type; $tuple_size]>($data).to_vec();
        impl_add_attrib!(_impl_surface $mesh, $loc, $name, vec);
    };
    (_impl TetMesh, $data_type:ty, $tuple_size:expr, $mesh:ident,
     $data:ident, $name:ident, $loc:ident) => {
        let vec = bytemuck::cast_slice::<_, [$data_type; $tuple_size]>($data).to_vec();
        impl_add_attrib!(_impl_volume $mesh, $loc, $name, vec);
    };
    // Points only attributes
    (_impl_points $mesh:ident, $loc:ident, $name:ident, $vec:ident) => {
        {
            if let AttribLocation::VERTEX = $loc {
                if let Err(error) = $mesh.0.add_attrib_data::<_,topo::VertexIndex>($name, $vec) {
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
                AttribLocation::VERTEX => {
                    if let Err(error) = $mesh.0.add_attrib_data::<_,topo::VertexIndex>($name, $vec) {
                        println!("Warning: failed to add attribute \"{}\" at {:?}, with error: {:?}", $name, $loc, error);
                    }
                },
                AttribLocation::FACE => {
                    if let Err(error) = $mesh.0.add_attrib_data::<_,topo::FaceIndex>($name, $vec) {
                        println!("Warning: failed to add attribute \"{}\" at {:?}, with error: {:?}", $name, $loc, error);
                    }
                },
                AttribLocation::FACEVERTEX => {
                    if let Err(error) = $mesh.0.add_attrib_data::<_,topo::FaceVertexIndex>($name, $vec) {
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
                AttribLocation::VERTEX => {
                    if let Err(error) = $mesh.0.add_attrib_data::<_,topo::VertexIndex>($name, $vec) {
                        println!("Warning: failed to add attribute \"{}\" at {:?}, with error: {:?}", $name, $loc, error);
                    }
                },
                AttribLocation::CELL => {
                    if let Err(error) = $mesh.0.add_attrib_data::<_,topo::CellIndex>($name, $vec) {
                        println!("Warning: failed to add attribute \"{}\" at {:?}, with error: {:?}", $name, $loc, error);
                    }
                },
                AttribLocation::CELLVERTEX => {
                    if let Err(error) = $mesh.0.add_attrib_data::<_,topo::CellVertexIndex>($name, $vec) {
                        println!("Warning: failed to add attribute \"{}\" at {:?}, with error: {:?}", $name, $loc, error);
                    }
                },
                _ => (),
            };
        }
    };
    // Main implementation of the add attribute function.
    ($mtype:ident, $mesh:ident,
     $loc:ident, $name:ident, $tuple_size:ident, $data:ident: $dty:ty) => {
        match $tuple_size {
            1 => {impl_add_attrib!(_impl $mtype, $dty, $mesh, $data, $name, $loc);},
            2 => {impl_add_attrib!(_impl $mtype, $dty, 2, $mesh, $data, $name, $loc);},
            3 => {impl_add_attrib!(_impl $mtype, $dty, 3, $mesh, $data, $name, $loc);},
            4 => {impl_add_attrib!(_impl $mtype, $dty, 4, $mesh, $data, $name, $loc);},
            5 => {impl_add_attrib!(_impl $mtype, $dty, 5, $mesh, $data, $name, $loc);},
            6 => {impl_add_attrib!(_impl $mtype, $dty, 6, $mesh, $data, $name, $loc);},
            7 => {impl_add_attrib!(_impl $mtype, $dty, 7, $mesh, $data, $name, $loc);},
            8 => {impl_add_attrib!(_impl $mtype, $dty, 8, $mesh, $data, $name, $loc);},
            9 => {impl_add_attrib!(_impl $mtype, $dty, 9, $mesh, $data, $name, $loc);},
            10 => {impl_add_attrib!(_impl $mtype, $dty, 10, $mesh, $data, $name, $loc);},
            11 => {impl_add_attrib!(_impl $mtype, $dty, 11, $mesh, $data, $name, $loc);},
            12 => {impl_add_attrib!(_impl $mtype, $dty, 12, $mesh, $data, $name, $loc);},
            13 => {impl_add_attrib!(_impl $mtype, $dty, 13, $mesh, $data, $name, $loc);},
            14 => {impl_add_attrib!(_impl $mtype, $dty, 14, $mesh, $data, $name, $loc);},
            15 => {impl_add_attrib!(_impl $mtype, $dty, 15, $mesh, $data, $name, $loc);},
            16 => {impl_add_attrib!(_impl $mtype, $dty, 16, $mesh, $data, $name, $loc);},
            _ => (),
        }
    };
    // *** String Attributes ***
    // Points only attributes
    (_impl_str_points $mesh:ident, $loc:ident, $name:ident, $update_fn:expr) => {
        {
            if let AttribLocation::VERTEX = $loc {
                $mesh.0.add_indirect_attrib::<_, topo::VertexIndex>($name, String::new())
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
                AttribLocation::VERTEX => {
                    $mesh.0.add_indirect_attrib::<_, topo::VertexIndex>($name, String::new())
                        .and_then(|(attrib, cache)| { attrib.indirect_update_with($update_fn, cache)?; Ok(()) })
                        .map_err(Error::from)
                },
                AttribLocation::FACE => {
                    $mesh.0.add_indirect_attrib::<_, topo::FaceIndex>($name, String::new())
                        .and_then(|(attrib, cache)| { attrib.indirect_update_with($update_fn, cache)?; Ok(()) })
                        .map_err(Error::from)
                },
                AttribLocation::FACEVERTEX => {
                    $mesh.0.add_indirect_attrib::<_, topo::FaceVertexIndex>($name, String::new())
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
                AttribLocation::VERTEX => {
                    $mesh.0.add_indirect_attrib::<_, topo::VertexIndex>($name, String::new())
                        .and_then(|(attrib, cache)| { attrib.indirect_update_with($update_fn, cache)?; Ok(()) })
                        .map_err(Error::from)
                },
                AttribLocation::CELL => {
                    $mesh.0.add_indirect_attrib::<_, topo::CellIndex>($name, String::new())
                        .and_then(|(attrib, cache)| { attrib.indirect_update_with($update_fn, cache)?; Ok(()) })
                        .map_err(Error::from)
                },
                AttribLocation::CELLVERTEX => {
                    $mesh.0.add_indirect_attrib::<_, topo::CellVertexIndex>($name, String::new())
                        .and_then(|(attrib, cache)| { attrib.indirect_update_with($update_fn, cache)?; Ok(()) })
                        .map_err(Error::from)
                },
                _ => Err(Error::Internal),
            }
        }
    };
    // Helpers for the implementation for string attributes below.
    (_impl_str PointCloud, $mesh:ident, $loc:ident, $name:ident, $update_fn:expr) => {
        impl_add_attrib!(_impl_str_points $mesh, $loc, $name, $update_fn)
    };
    (_impl_str PolyMesh, $mesh:ident, $loc:ident, $name:ident, $update_fn:expr) => {
        impl_add_attrib!(_impl_str_surface $mesh, $loc, $name, $update_fn)
    };
    (_impl_str TetMesh, $mesh:ident, $loc:ident, $name:ident, $update_fn:expr) => {
        impl_add_attrib!(_impl_str_volume $mesh, $loc, $name, $update_fn)
    };
    // Implementation for string attributes
    ($mesh_type:ident, $mesh:ident,
     $loc:ident, $name:ident, $tuple_size:ident,
     $strings:ident, $data:ident) => {
        debug_assert!($tuple_size == 1, "Only 1 dimensional string attributes currently supported.");
        if $tuple_size != 1 {
            return;
        }

        let update = |i: usize, _: &Arc<_>| {
            let idx = $data[i];
            if idx >= 0 {
                let s = $strings[idx as usize];
                return Some(std::sync::Arc::new(String::from(s)));
            }
            None
        };

        let res = impl_add_attrib!(_impl_str $mesh_type, $mesh, $loc, $name, update);
        match res {
            Err(error) =>
                println!("Warning: failed to add string attribute \"{}\" at {:?}, with error: {:?}", $name, $loc, error),
            _ => {}
        }
    }
}

impl PointCloud {
    pub fn add_attrib_f32(
        &mut self,
        loc: AttribLocation,
        name: &str,
        tuple_size: usize,
        data: &[f32],
    ) {
        impl_add_attrib!(PointCloud, self, loc, name, tuple_size, data: f32);
    }

    pub fn add_attrib_f64(
        &mut self,
        loc: AttribLocation,
        name: &str,
        tuple_size: usize,
        data: &[f64],
    ) {
        impl_add_attrib!(PointCloud, self, loc, name, tuple_size, data: f64);
    }

    pub fn add_attrib_i8(
        &mut self,
        loc: AttribLocation,
        name: &str,
        tuple_size: usize,
        data: &[i8],
    ) {
        impl_add_attrib!(PointCloud, self, loc, name, tuple_size, data: i8);
    }

    pub fn add_attrib_i32(
        &mut self,
        loc: AttribLocation,
        name: &str,
        tuple_size: usize,
        data: &[i32],
    ) {
        impl_add_attrib!(PointCloud, self, loc, name, tuple_size, data: i32);
    }

    pub fn add_attrib_i64(
        &mut self,
        loc: AttribLocation,
        name: &str,
        tuple_size: usize,
        data: &[i64],
    ) {
        impl_add_attrib!(PointCloud, self, loc, name, tuple_size, data: i64);
    }

    pub fn add_attrib_str(
        &mut self,
        loc: AttribLocation,
        name: &str,
        tuple_size: usize,
        strings: &[&str],
        data: &[i64],
    ) {
        impl_add_attrib!(PointCloud, self, loc, name, tuple_size, strings, data);
    }
}

impl PolyMesh {
    pub fn add_attrib_f32(
        &mut self,
        loc: AttribLocation,
        name: &str,
        tuple_size: usize,
        data: &[f32],
    ) {
        impl_add_attrib!(PolyMesh, self, loc, name, tuple_size, data: f32);
    }

    pub fn add_attrib_f64(
        &mut self,
        loc: AttribLocation,
        name: &str,
        tuple_size: usize,
        data: &[f64],
    ) {
        impl_add_attrib!(PolyMesh, self, loc, name, tuple_size, data: f64);
    }

    pub fn add_attrib_i8(
        &mut self,
        loc: AttribLocation,
        name: &str,
        tuple_size: usize,
        data: &[i8],
    ) {
        impl_add_attrib!(PolyMesh, self, loc, name, tuple_size, data: i8);
    }

    pub fn add_attrib_i32(
        &mut self,
        loc: AttribLocation,
        name: &str,
        tuple_size: usize,
        data: &[i32],
    ) {
        impl_add_attrib!(PolyMesh, self, loc, name, tuple_size, data: i32);
    }

    pub fn add_attrib_i64(
        &mut self,
        loc: AttribLocation,
        name: &str,
        tuple_size: usize,
        data: &[i64],
    ) {
        impl_add_attrib!(PolyMesh, self, loc, name, tuple_size, data: i64);
    }
    pub fn add_attrib_str(
        &mut self,
        loc: AttribLocation,
        name: &str,
        tuple_size: usize,
        strings: &[&str],
        data: &[i64],
    ) {
        impl_add_attrib!(PolyMesh, self, loc, name, tuple_size, strings, data);
    }
}

impl TetMesh {
    pub fn add_attrib_f32(
        &mut self,
        loc: AttribLocation,
        name: &str,
        tuple_size: usize,
        data: &[f32],
    ) {
        impl_add_attrib!(TetMesh, self, loc, name, tuple_size, data: f32);
    }

    pub fn add_attrib_f64(
        &mut self,
        loc: AttribLocation,
        name: &str,
        tuple_size: usize,
        data: &[f64],
    ) {
        impl_add_attrib!(TetMesh, self, loc, name, tuple_size, data: f64);
    }

    pub fn add_attrib_i8(
        &mut self,
        loc: AttribLocation,
        name: &str,
        tuple_size: usize,
        data: &[i8],
    ) {
        impl_add_attrib!(TetMesh, self, loc, name, tuple_size, data: i8);
    }

    pub fn add_attrib_i32(
        &mut self,
        loc: AttribLocation,
        name: &str,
        tuple_size: usize,
        data: &[i32],
    ) {
        impl_add_attrib!(TetMesh, self, loc, name, tuple_size, data: i32);
    }

    pub fn add_attrib_i64(
        &mut self,
        loc: AttribLocation,
        name: &str,
        tuple_size: usize,
        data: &[i64],
    ) {
        impl_add_attrib!(TetMesh, self, loc, name, tuple_size, data: i64);
    }
    pub fn add_attrib_str(
        &mut self,
        loc: AttribLocation,
        name: &str,
        tuple_size: usize,
        strings: &[&str],
        data: &[i64],
    ) {
        impl_add_attrib!(TetMesh, self, loc, name, tuple_size, strings, data);
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum Mesh {
    TetMesh(TetMesh),
    PolyMesh(PolyMesh),
    PointCloud(PointCloud),
    None,
}

impl<M: Into<Mesh>> From<Option<M>> for Mesh {
    fn from(m: Option<M>) -> Self {
        m.map(|x| x.into()).unwrap_or(Mesh::None)
    }
}

impl From<mesh::TetMesh<f64>> for Mesh {
    fn from(m: mesh::TetMesh<f64>) -> Self {
        Mesh::TetMesh(m.into())
    }
}

impl From<mesh::PolyMesh<f64>> for Mesh {
    fn from(m: mesh::PolyMesh<f64>) -> Self {
        Mesh::PolyMesh(m.into())
    }
}

impl From<mesh::PointCloud<f64>> for Mesh {
    fn from(m: mesh::PointCloud<f64>) -> Self {
        Mesh::PointCloud(m.into())
    }
}

impl From<TetMesh> for Mesh {
    fn from(m: TetMesh) -> Self {
        Mesh::TetMesh(m)
    }
}

impl From<PolyMesh> for Mesh {
    fn from(m: PolyMesh) -> Self {
        Mesh::PolyMesh(m)
    }
}

impl From<PointCloud> for Mesh {
    fn from(m: PointCloud) -> Self {
        Mesh::PointCloud(m)
    }
}

impl Mesh {
    #[inline]
    pub fn or_else<F: FnOnce() -> Mesh>(self, f: F) -> Mesh {
        match self {
            Mesh::None => f(),
            _ => self,
        }
    }
    #[inline]
    pub fn or(self, b: Mesh) -> Mesh {
        match self {
            Mesh::None => b,
            _ => self,
        }
    }
    /// Add this mesh to the given detail.
    pub fn add_to_detail(&self, detail: std::pin::Pin<&mut GU_Detail>) {
        match self {
            Mesh::TetMesh(m) => add_tetmesh(detail, m),
            Mesh::PolyMesh(m) => add_polymesh(detail, m),
            Mesh::PointCloud(m) => add_pointcloud(detail, m),
            Mesh::None => {}
        }
    }
    pub fn is_tetmesh(&self) -> bool {
        matches!(self, Mesh::TetMesh(_))
    }
    pub fn is_polymesh(&self) -> bool {
        matches!(self, Mesh::PolyMesh(_))
    }
    pub fn is_pointcloud(&self) -> bool {
        matches!(self, Mesh::PointCloud(_))
    }
    pub fn tag(&self) -> MeshTag {
        match self {
            Mesh::TetMesh(_) => MeshTag::TetMesh,
            Mesh::PolyMesh(_) => MeshTag::PolyMesh,
            Mesh::PointCloud(_) => MeshTag::PointCloud,
            Mesh::None => MeshTag::None,
        }
    }
}

pub fn into_tetmesh(mesh: Box<Mesh>) -> Box<TetMesh> {
    match *mesh {
        Mesh::TetMesh(m) => Box::new(m),
        _ => panic!("Mesh mismatch"),
    }
}
pub fn into_polymesh(mesh: Box<Mesh>) -> Box<PolyMesh> {
    match *mesh {
        Mesh::PolyMesh(m) => Box::new(m),
        _ => panic!("Mesh mismatch"),
    }
}
pub fn into_pointcloud(mesh: Box<Mesh>) -> Box<PointCloud> {
    match *mesh {
        Mesh::PointCloud(m) => Box::new(m),
        _ => panic!("Mesh mismatch"),
    }
}
