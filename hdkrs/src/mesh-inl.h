#pragma once

#include <GU/GU_Detail.h>
#include <GEO/GEO_PrimTetrahedron.h>
#include <GEO/GEO_PrimPoly.h>
#include <GEO/GEO_PolyCounts.h>
#include <GA/GA_PageHandle.h>

#include <vector>
#include <cassert>

namespace hdkrs {

// Implement OwnedPtr specializations

template<>
inline OwnedPtr<HR_PolyMesh>::~OwnedPtr() {
    hr_free_polymesh(_ptr);
}

template<>
inline OwnedPtr<HR_TetMesh>::~OwnedPtr() {
    hr_free_tetmesh(_ptr);
}

template<>
inline OwnedPtr<HR_PointCloud>::~OwnedPtr() {
    hr_free_pointcloud(_ptr);
}

namespace mesh {

inline std::ostream& operator<<(std::ostream& out, HRAttribLocation where) {
    switch (where) {
        case HRAttribLocation::HR_VERTEX: out << "Vertex"; break;
        case HRAttribLocation::HR_FACE: out << "Face"; break;
        case HRAttribLocation::HR_CELL: out << "Cell"; break;
        case HRAttribLocation::HR_FACEVERTEX: out << "FaceVertex"; break;
        case HRAttribLocation::HR_CELLVERTEX: out << "CellVertex"; break;
        default: break;
    }
    return out;
}

namespace { // Implementation details

#define HR_ADD_NUM_ATTRIB_IMPL(MTYPE, TYPE, FN) \
    void add_attrib( \
            MTYPE *m, \
            HRAttribLocation where, \
            const char *name, \
            std::size_t tuple_size, \
            const std::vector<TYPE> &data) \
    { \
        FN( m, where, name, tuple_size, data.size(), data.data() ); \
    }

HR_ADD_NUM_ATTRIB_IMPL(HR_PointCloud, int8, hr_add_pointcloud_attrib_i8)
HR_ADD_NUM_ATTRIB_IMPL(HR_PointCloud, int32, hr_add_pointcloud_attrib_i32)
HR_ADD_NUM_ATTRIB_IMPL(HR_PointCloud, int64_t, hr_add_pointcloud_attrib_i64)
HR_ADD_NUM_ATTRIB_IMPL(HR_PointCloud, fpreal32, hr_add_pointcloud_attrib_f32)
HR_ADD_NUM_ATTRIB_IMPL(HR_PointCloud, fpreal64, hr_add_pointcloud_attrib_f64)

HR_ADD_NUM_ATTRIB_IMPL(HR_PolyMesh, int8, hr_add_polymesh_attrib_i8)
HR_ADD_NUM_ATTRIB_IMPL(HR_PolyMesh, int32, hr_add_polymesh_attrib_i32)
HR_ADD_NUM_ATTRIB_IMPL(HR_PolyMesh, int64_t, hr_add_polymesh_attrib_i64)
HR_ADD_NUM_ATTRIB_IMPL(HR_PolyMesh, fpreal32, hr_add_polymesh_attrib_f32)
HR_ADD_NUM_ATTRIB_IMPL(HR_PolyMesh, fpreal64, hr_add_polymesh_attrib_f64)

HR_ADD_NUM_ATTRIB_IMPL(HR_TetMesh, int8, hr_add_tetmesh_attrib_i8)
HR_ADD_NUM_ATTRIB_IMPL(HR_TetMesh, int32, hr_add_tetmesh_attrib_i32)
HR_ADD_NUM_ATTRIB_IMPL(HR_TetMesh, int64_t, hr_add_tetmesh_attrib_i64)
HR_ADD_NUM_ATTRIB_IMPL(HR_TetMesh, fpreal32, hr_add_tetmesh_attrib_f32)
HR_ADD_NUM_ATTRIB_IMPL(HR_TetMesh, fpreal64, hr_add_tetmesh_attrib_f64)

#undef ADD_NUM_ATTRIB_IMPL

void add_attrib(
        HR_PointCloud *ptcloud,
        HRAttribLocation where,
        const char *name,
        std::size_t tuple_size,
        const std::vector<const char *> &strings,
        const std::vector<int64_t> &indices)
{
    hr_add_pointcloud_attrib_str(
            ptcloud, where, name, tuple_size, strings.size(),
            strings.data(), indices.size(), indices.data());
}


void add_attrib(
        HR_PolyMesh *polymesh,
        HRAttribLocation where,
        const char *name,
        std::size_t tuple_size,
        const std::vector<const char *> &strings,
        const std::vector<int64_t> &indices)
{
    hr_add_polymesh_attrib_str(
            polymesh, where, name, tuple_size, strings.size(),
            strings.data(), indices.size(), indices.data());
}

void add_attrib(
        HR_TetMesh *tetmesh,
        HRAttribLocation where,
        const char *name,
        std::size_t tuple_size,
        const std::vector<const char *> &strings,
        const std::vector<int64_t> &indices)
{
    hr_add_tetmesh_attrib_str(
            tetmesh, where, name, tuple_size, strings.size(),
            strings.data(), indices.size(), indices.data());
}

template<typename T>
GA_PrimitiveTypeId mesh_prim_type_id();

template<>
GA_PrimitiveTypeId mesh_prim_type_id<HR_PolyMesh>() { return GA_PRIMPOLY; }

template<>
GA_PrimitiveTypeId mesh_prim_type_id<HR_TetMesh>() { return GA_PRIMTETRAHEDRON; }

template<typename T>
HRAttribLocation mesh_prim_attrib_location();

template<>
HRAttribLocation mesh_prim_attrib_location<HR_PolyMesh>() { return HRAttribLocation::HR_FACE; }

template<>
HRAttribLocation mesh_prim_attrib_location<HR_TetMesh>() { return HRAttribLocation::HR_CELL; }

template<typename T>
HRAttribLocation mesh_vertex_attrib_location();

template<>
HRAttribLocation mesh_vertex_attrib_location<HR_PolyMesh>() { return HRAttribLocation::HR_FACEVERTEX; }

template<>
HRAttribLocation mesh_vertex_attrib_location<HR_TetMesh>() { return HRAttribLocation::HR_CELLVERTEX; }

// Mark all points and vectors in the given detail that intersect the primitives of interest.
std::tuple<std::vector<bool>, std::size_t>
mark_points_and_count_vertices(
        const GU_Detail *detail,
        GA_PrimitiveTypeId prim_type_id)
{
    std::vector<bool> points(detail->getNumPointOffsets(), false);
    std::size_t num_vertices = 0;
    for ( GA_Offset prim_off : detail->getPrimitiveRange() )
    {
        const GEO_Primitive *prim = detail->getGEOPrimitive(prim_off);
        if (prim->getTypeId() == prim_type_id)
        {
            GA_Size num_prim_verts = detail->getPrimitiveVertexCount(prim_off);
            num_vertices += num_prim_verts;
            for ( GA_Size idx = 0; idx < num_prim_verts; ++idx ) {
                auto vtx_off = detail->getPrimitiveVertexOffset(prim_off, idx);
                points[detail->vertexPoint(vtx_off)] = true;
            }
        }
    }

    return std::make_pair(std::move(points), num_vertices);
}

template<typename T, typename M, typename S = T>
void
fill_prim_attrib(
        const GU_Detail *detail,
        const GA_AIFTuple *aif,
        const GA_Attribute *attrib,
        std::size_t tuple_size,
        std::size_t num_elem,
        M *mesh)
{
    std::vector<T> data(tuple_size*num_elem);
    int i = 0;
    for ( GA_Offset prim_off : detail->getPrimitiveRange() )
    {
        const GEO_Primitive *prim = detail->getGEOPrimitive(prim_off);
        if (prim->getTypeId() != mesh_prim_type_id<M>()) continue;
        for ( int k = 0, k_end = tuple_size; k < k_end; ++k ) {
            S val;
            aif->get(attrib, prim_off, val, k);
            data[tuple_size*i + k] = val;
        }
        i += 1;
    }

    auto name = attrib->getName().c_str();
    add_attrib(mesh, mesh_prim_attrib_location<M>(), name, tuple_size, data);
}

template<typename T, typename M, typename S = T>
void fill_point_attrib(
        const GU_Detail *detail,
        const GA_AIFTuple *aif,
        const GA_Attribute *attrib,
        std::size_t tuple_size,
        std::size_t num_elem,
        const std::vector<bool> &group,
        M *mesh)
{
    std::vector<T> data(tuple_size*num_elem);
    int i = 0;
    for ( GA_Offset pt_off : detail->getPointRange() )
    {
        if (!group[pt_off]) continue;
        for ( int k = 0, k_end = tuple_size; k < k_end; ++k ) {
            S val;
            aif->get(attrib, pt_off, val, k);
            data[tuple_size*i + k] = val;
        }
        i += 1;
    }

    auto name = attrib->getName().c_str();
    add_attrib(mesh, HRAttribLocation::HR_VERTEX, name, tuple_size, data);
}

template<typename T, typename M, typename S = T>
void fill_vertex_attrib(
        const GU_Detail *detail,
        const GA_AIFTuple *aif,
        const GA_Attribute *attrib,
        std::size_t tuple_size,
        std::size_t num_elem,
        M *mesh)
{
    std::vector<T> data(tuple_size*num_elem);
    int i = 0;
    for ( GA_Offset prim_off : detail->getPrimitiveRange() ) {
        const GEO_Primitive *prim = detail->getGEOPrimitive(prim_off);
        if (prim->getTypeId() != mesh_prim_type_id<M>()) continue;
        GA_Size num_prim_verts = detail->getPrimitiveVertexCount(prim_off);
        for (GA_Size idx = 0; idx < num_prim_verts; ++idx) {
            auto vtx_off = detail->getPrimitiveVertexOffset(prim_off, idx);
            for (int k = 0, k_end = tuple_size; k < k_end; ++k) {
                S val;
                aif->get(attrib, vtx_off, val, k);
                data[tuple_size * i + k] = val;
            }
            i += 1;
        }
    }

    auto name = attrib->getName().c_str();
    add_attrib(mesh, mesh_vertex_attrib_location<M>(), name, tuple_size, data);
}

template<typename M>
void fill_prim_str_attrib(
        const GU_Detail *detail,
        const GA_AIFSharedStringTuple *aif,
        const GA_Attribute *attrib,
        std::size_t tuple_size,
        std::size_t num_elem,
        M *mesh)
{
    // Try with different types
    std::vector<int64_t> ids(aif->getTableEntries(attrib), -1);
    std::vector<const char *> strings;
    for (auto it = aif->begin(attrib); !it.atEnd(); ++it) {
        ids[it.getHandle()] = strings.size();
        strings.push_back( it.getString() );
    }

    std::vector<int64_t> indices(tuple_size*num_elem, -1);

    int i = 0;
    for ( GA_Offset prim_off : detail->getPrimitiveRange() )
    {
        const GEO_Primitive *prim = detail->getGEOPrimitive(prim_off);
        if (prim->getTypeId() == mesh_prim_type_id<M>())
        {
            for ( int k = 0, k_end = tuple_size; k < k_end; ++k ) {
                GA_StringIndexType handle = aif->getHandle(attrib, prim_off, k);
                indices[tuple_size*i + k] = handle > -1 ? ids[handle] : -1;
            }
            i += 1;
        }
    }

    auto name = attrib->getName().c_str();
    add_attrib(mesh, mesh_prim_attrib_location<M>(), name, tuple_size, strings, indices);
}

template<typename M>
void fill_point_str_attrib(
        const GU_Detail *detail,
        const GA_AIFSharedStringTuple *aif,
        const GA_Attribute *attrib,
        std::size_t tuple_size,
        std::size_t num_elem,
        const std::vector<bool> &group,
        M *mesh)
{
    // Try with different types
    std::vector<int64_t> ids(aif->getTableEntries(attrib), -1);
    std::vector<const char *> strings;
    for (auto it = aif->begin(attrib); !it.atEnd(); ++it) {
        ids[it.getHandle()] = strings.size();
        strings.push_back( it.getString() );
    }

    std::vector<int64_t> indices(tuple_size*num_elem, -1);

    int i = 0;
    for ( GA_Offset pt_off : detail->getPointRange() )
    {
        if (!group[pt_off]) continue;
        for ( int k = 0, k_end = tuple_size; k < k_end; ++k ) {
            GA_StringIndexType handle = aif->getHandle(attrib, pt_off, k);
            indices[tuple_size*i + k] = handle > -1 ? ids[handle] : -1;
        }
        i += 1;
    }

    auto name = attrib->getName().c_str();
    add_attrib(mesh, HRAttribLocation::HR_VERTEX, name, tuple_size, strings, indices);
}

template<typename M>
void fill_vertex_str_attrib(
        const GU_Detail *detail,
        const GA_AIFSharedStringTuple *aif,
        const GA_Attribute *attrib,
        std::size_t tuple_size,
        std::size_t num_elem,
        M *mesh)
{
    // Try with different types
    std::vector<int64_t> ids(aif->getTableEntries(attrib), -1);
    std::vector<const char *> strings;
    for (auto it = aif->begin(attrib); !it.atEnd(); ++it) {
        ids[it.getHandle()] = strings.size();
        strings.push_back( it.getString() );
    }

    std::vector<int64_t> indices(tuple_size*num_elem, -1);

    int i = 0;
    for ( GA_Offset prim_off : detail->getPrimitiveRange() ) {
        const GEO_Primitive *prim = detail->getGEOPrimitive(prim_off);
        if (prim->getTypeId() != mesh_prim_type_id<M>()) continue;
        GA_Size num_prim_verts = detail->getPrimitiveVertexCount(prim_off);
        for (GA_Size idx = 0; idx < num_prim_verts; ++idx) {
            auto vtx_off = detail->getPrimitiveVertexOffset(prim_off, idx);
            for (int k = 0, k_end = tuple_size; k < k_end; ++k) {
                GA_StringIndexType handle = aif->getHandle(attrib, vtx_off, k);
                indices[tuple_size * i + k] = handle > -1 ? ids[handle] : -1;
            }
            i += 1;
        }
    }

    auto name = attrib->getName().c_str();
    add_attrib(mesh, mesh_vertex_attrib_location<M>(), name, tuple_size, strings, indices);
}

template<typename M>
void transfer_primitive_attributes(const GU_Detail* detail, M* mesh, std::size_t num_prims) {
    // Get prim data attributes
    for (auto it = detail->getAttributeDict(GA_ATTRIB_PRIMITIVE).begin(GA_SCOPE_PUBLIC); !it.atEnd(); ++it)
    {
        GA_Attribute *attrib = it.attrib();
        std::size_t tuple_size = attrib->getTupleSize();
        {
            auto aif = attrib->getAIFTuple(); // array of data
            if ( aif )
            {
                switch (aif->getStorage(attrib)) {
                    case GA_STORE_BOOL:
                        fill_prim_attrib<int8, M, int32>(detail, aif, attrib, tuple_size, num_prims, mesh); break;
                    case GA_STORE_INT8:
                        fill_prim_attrib<int8, M, int32>(detail, aif, attrib, tuple_size, num_prims, mesh); break;
                    case GA_STORE_INT32:
                        fill_prim_attrib<int32>(detail, aif, attrib, tuple_size, num_prims, mesh); break;
                    case GA_STORE_INT64:
                        fill_prim_attrib<int64_t>(detail, aif, attrib, tuple_size, num_prims, mesh); break;
                    case GA_STORE_REAL32:
                        fill_prim_attrib<fpreal32>(detail, aif, attrib, tuple_size, num_prims, mesh); break;
                    case GA_STORE_REAL64:
                        fill_prim_attrib<fpreal64>(detail, aif, attrib, tuple_size, num_prims, mesh); break;
                    default: break; // do nothing
                }
            }
        }

        {
            auto aif = attrib->getAIFSharedStringTuple(); // array of strings
            if ( aif ) {
                aif->compactStorage(attrib);
                fill_prim_str_attrib(detail, aif, attrib, tuple_size, num_prims, mesh);
            }
        }

        // don't know how to handle these yet.
        //aif = attrib->getAIFNumericArray(); // variable sized array
        //aif = attrib->getAIFSharedStringArray(); // variable sized array of strings
    }
}

// Transfer attributes from points marked in the given pt_grp
template<typename M>
void transfer_point_attributes(const GU_Detail* detail, M* mesh, const std::vector<bool>& pt_grp)
{
    std::size_t num_points = std::count(pt_grp.begin(), pt_grp.end(), true);
    for (auto it = detail->getAttributeDict(GA_ATTRIB_POINT).begin(GA_SCOPE_PUBLIC); !it.atEnd(); ++it)
    {
        GA_Attribute *attrib = it.attrib();
        if (attrib->getTypeInfo() == GA_TYPE_POINT) // ignore position attribute
            continue;
        std::size_t tuple_size = attrib->getTupleSize();
        {
            auto aif = attrib->getAIFTuple(); // array of data
            if ( aif )
            {
                switch (aif->getStorage(attrib)) {
                    case GA_STORE_BOOL:
                        fill_point_attrib<int8, M, int32>(detail, aif, attrib, tuple_size, num_points, pt_grp, mesh); break;
                    case GA_STORE_INT8:
                        fill_point_attrib<int8, M, int32>(detail, aif, attrib, tuple_size, num_points, pt_grp, mesh); break;
                    case GA_STORE_INT32:
                        fill_point_attrib<int32>(detail, aif, attrib, tuple_size, num_points, pt_grp, mesh); break;
                    case GA_STORE_INT64:
                        fill_point_attrib<int64_t>(detail, aif, attrib, tuple_size, num_points, pt_grp, mesh); break;
                    case GA_STORE_REAL32:
                        fill_point_attrib<fpreal32>(detail, aif, attrib, tuple_size, num_points, pt_grp, mesh); break;
                    case GA_STORE_REAL64:
                        fill_point_attrib<fpreal64>(detail, aif, attrib, tuple_size, num_points, pt_grp, mesh); break;
                    default: break; // do nothing
                }
            }
        }


        {
            auto aif = attrib->getAIFSharedStringTuple(); // array of strings
            if ( aif ) {
                aif->compactStorage(attrib);
                fill_point_str_attrib(detail, aif, attrib, tuple_size, num_points, pt_grp, mesh);
            }
        }

        // don't know how to handle these yet.
        //aif = attrib->getAIFNumericArray(); // variable sized array
        //aif = attrib->getAIFSharedStringArray(); // variable sized array of strings
    }
}

// Transfer attributes from vertices marked in the given vtx_grp
template<typename M>
void transfer_vertex_attributes(const GU_Detail* detail, M* mesh, std::size_t num_vertices)
{
    for (auto it = detail->getAttributeDict(GA_ATTRIB_VERTEX).begin(GA_SCOPE_PUBLIC); !it.atEnd(); ++it)
    {
        GA_Attribute *attrib = it.attrib();
        std::size_t tuple_size = attrib->getTupleSize();
        {
            auto aif = attrib->getAIFTuple(); // array of data
            if ( aif )
            {
                switch (aif->getStorage(attrib)) {
                    case GA_STORE_BOOL:
                        fill_vertex_attrib<int8, M, int32>(detail, aif, attrib, tuple_size, num_vertices, mesh); break;
                    case GA_STORE_INT8:
                        fill_vertex_attrib<int8, M, int32>(detail, aif, attrib, tuple_size, num_vertices, mesh); break;
                    case GA_STORE_INT32:
                        fill_vertex_attrib<int32>(detail, aif, attrib, tuple_size, num_vertices, mesh); break;
                    case GA_STORE_INT64:
                        fill_vertex_attrib<int64_t>(detail, aif, attrib, tuple_size, num_vertices, mesh); break;
                    case GA_STORE_REAL32:
                        fill_vertex_attrib<fpreal32>(detail, aif, attrib, tuple_size, num_vertices, mesh); break;
                    case GA_STORE_REAL64:
                        fill_vertex_attrib<fpreal64>(detail, aif, attrib, tuple_size, num_vertices, mesh); break;
                    default: break; // do nothing
                }
            }
        }


        {
            auto aif = attrib->getAIFSharedStringTuple(); // array of strings
            if ( aif ) {
                aif->compactStorage(attrib);
                fill_vertex_str_attrib(detail, aif, attrib, tuple_size, num_vertices, mesh);
            }
        }

        // don't know how to handle these yet.
        //aif = attrib->getAIFNumericArray(); // variable sized array
        //aif = attrib->getAIFSharedStringArray(); // variable sized array of strings
    }
}

template<typename M>
void transfer_attributes(const GU_Detail* detail, M* mesh, std::size_t num_prims)
{
    transfer_primitive_attributes(detail, mesh, num_prims);

    std::size_t num_vertices = 0;
    std::vector<bool> pt_grp;
    std::tie(pt_grp, num_vertices) = mark_points_and_count_vertices(detail, mesh_prim_type_id<M>());

    transfer_point_attributes(detail, mesh, pt_grp);
    transfer_vertex_attributes(detail, mesh, num_vertices);
}

template<typename HandleType, typename ArrayType>
void fill_attrib(HandleType h, ArrayType arr, GA_Offset startoff) {
    if (h.isInvalid()) return;
    std::size_t i = 0;
    auto n = startoff + (arr.size/arr.tuple_size);
    for ( GA_Offset off = startoff; off < n; ++off, ++i ) {
        for ( int j = 0; j < arr.tuple_size; ++j ) {
            h.set(off, j, arr.array[arr.tuple_size*i + j]);
        }
    }
}

/** Retrieve attributes from the mesh using the given iterator.
 */
void retrieve_attributes(GU_Detail *detail, GA_Offset startoff, HR_AttribIter *it, GA_AttributeOwner owner) {
    while ( it ) { // it could be null, but it doesn't change
        auto attrib = hr_attrib_iter_next(it);
        if (!attrib) break;
        auto name = UT_String(hr_attrib_name(attrib));
        name.forceValidVariableName();
        auto type = hr_attrib_data_type(attrib);
        if (type == HRDataType::HR_I8 ) {
            auto arr = hr_attrib_data_i8(attrib);
            auto h = GA_RWHandleC(detail->addTuple(GA_STORE_INT8, owner, name, arr.tuple_size));
            fill_attrib(h, arr, startoff);
            hr_free_attrib_data_i8(arr);
        } else if (type == HRDataType::HR_I32 ) {
            auto arr = hr_attrib_data_i32(attrib);
            auto h = GA_RWHandleI(detail->addTuple(GA_STORE_INT32, owner, name, arr.tuple_size));
            fill_attrib(h, arr, startoff);
            hr_free_attrib_data_i32(arr);
        } else if (type == HRDataType::HR_I64 ) {
            auto arr = hr_attrib_data_i64(attrib);
            auto h = GA_RWHandleID(detail->addTuple(GA_STORE_INT64, owner, name, arr.tuple_size));
            fill_attrib(h, arr, startoff);
            hr_free_attrib_data_i64(arr);
        } else if (type == HRDataType::HR_F32 ) {
            auto arr = hr_attrib_data_f32(attrib);
            auto h = GA_RWHandleF(detail->addTuple(GA_STORE_REAL32, owner, name, arr.tuple_size));
            fill_attrib(h, arr, startoff);
            hr_free_attrib_data_f32(arr);
        } else if (type == HRDataType::HR_F64 ) {
            auto arr = hr_attrib_data_f64(attrib);
            auto h = GA_RWHandleD(detail->addTuple(GA_STORE_REAL64, owner, name, arr.tuple_size));
            fill_attrib(h, arr, startoff);
            hr_free_attrib_data_f64(arr);
        } else if (type == HRDataType::HR_STR ) {
            auto arr = hr_attrib_data_str(attrib);
            auto h = GA_RWHandleS(detail->addTuple(GA_STORE_STRING, owner, name, arr.tuple_size));
            fill_attrib(h, arr, startoff);
            hr_free_attrib_data_str(arr);
        }
        hr_free_attribute(attrib);
    }
    hr_free_attrib_iter(it);
}


template<typename HandleType, typename ArrayType>
void update_attrib(HandleType h, ArrayType arr) {
    if (h.isInvalid()) return;
    int n = (arr.size/arr.tuple_size);
    for ( int j = 0; j < arr.tuple_size; ++j ) {
        h.setBlockFromIndices(GA_Index(0), GA_Size(n), arr.array, arr.tuple_size, j);
    }
}

/** Update attributes of the mesh using the given iterator.
 */
void update_attributes(GU_Detail *detail, HR_AttribIter *it, GA_AttributeOwner owner) {
    while ( it ) { // it could be null, but it doesn't change
        auto attrib = hr_attrib_iter_next(it);
        if (!attrib) break;
        auto name = UT_String(hr_attrib_name(attrib));
        name.forceValidVariableName();
        auto type = hr_attrib_data_type(attrib);
        if (type == HRDataType::HR_I8 ) {
            auto arr = hr_attrib_data_i8(attrib);
            auto h = GA_RWHandleC(detail->addTuple(GA_STORE_INT8, owner, name, arr.tuple_size));
            update_attrib(h, arr);
            hr_free_attrib_data_i8(arr);
        } else if (type == HRDataType::HR_I32 ) {
            auto arr = hr_attrib_data_i32(attrib);
            auto h = GA_RWHandleI(detail->addTuple(GA_STORE_INT32, owner, name, arr.tuple_size));
            update_attrib(h, arr);
            hr_free_attrib_data_i32(arr);
        } else if (type == HRDataType::HR_I64 ) {
            auto arr = hr_attrib_data_i64(attrib);
            auto h = GA_RWHandleID(detail->addTuple(GA_STORE_INT64, owner, name, arr.tuple_size));
            update_attrib(h, arr);
            hr_free_attrib_data_i64(arr);
        } else if (type == HRDataType::HR_F32 ) {
            auto arr = hr_attrib_data_f32(attrib);
            auto h = GA_RWHandleF(detail->addTuple(GA_STORE_REAL32, owner, name, arr.tuple_size));
            update_attrib(h, arr);
            hr_free_attrib_data_f32(arr);
        } else if (type == HRDataType::HR_F64 ) {
            auto arr = hr_attrib_data_f64(attrib);
            auto h = GA_RWHandleD(detail->addTuple(GA_STORE_REAL64, owner, name, arr.tuple_size));
            update_attrib(h, arr);
            hr_free_attrib_data_f64(arr);
        } // String attributes are not yet supported by updates
        hr_free_attribute(attrib);
    }
    hr_free_attrib_iter(it);
}

/**
 * Add a tetmesh to the current detail
 */
[[maybe_unused]]
void add_tetmesh(GU_Detail* detail, OwnedPtr<HR_TetMesh> tetmesh_ptr) {
    GA_Offset startvtxoff = GA_Offset(detail->getNumVertexOffsets());

    auto tetmesh = tetmesh_ptr.get();

    // add tets.
    if (tetmesh) {
        auto points = hr_get_tetmesh_points(tetmesh);

        auto test_indices = hr_get_tetmesh_indices(tetmesh);
        if (test_indices.size > 0) {
            std::vector<int> indices;
            for (std::size_t i = 0; i < test_indices.size; ++i) {
                indices.push_back(static_cast<int>(test_indices.array[i]));
            }

            GA_Offset startptoff = detail->appendPointBlock(points.size);
            for (exint pt_idx = 0; pt_idx < points.size; ++pt_idx) {
                GA_Offset ptoff = startptoff + pt_idx;
                detail->setPos3(ptoff, UT_Vector3(points.array[pt_idx]));
            }

            GA_Offset startprimoff = GEO_PrimTetrahedron::buildBlock(
                    detail, startptoff, detail->getNumPointOffsets(),
                    indices.size()/4, indices.data());

            retrieve_attributes(detail, startptoff, hr_tetmesh_attrib_iter(tetmesh, HRAttribLocation::HR_VERTEX, 0), GA_ATTRIB_POINT);
            retrieve_attributes(detail, startprimoff, hr_tetmesh_attrib_iter(tetmesh, HRAttribLocation::HR_CELL, 0), GA_ATTRIB_PRIMITIVE);
            retrieve_attributes(detail, startvtxoff, hr_tetmesh_attrib_iter(tetmesh, HRAttribLocation::HR_CELLVERTEX, 0), GA_ATTRIB_VERTEX);
        }
        hr_free_point_array(points);
        hr_free_index_array(test_indices);
    }
}

/**
 * Add a polymesh to the current detail
 */
[[maybe_unused]]
void add_polymesh(GU_Detail* detail, OwnedPtr<HR_PolyMesh> polymesh_ptr) {
    GA_Offset startvtxoff = GA_Offset(detail->getNumVertexOffsets());

    auto polymesh = polymesh_ptr.get();

    // add polygons
    if (polymesh) {
        auto points = hr_get_polymesh_points(polymesh);

        auto test_indices = hr_get_polymesh_indices(polymesh);
        if (test_indices.size > 0) {
            GA_Offset startptoff = detail->appendPointBlock(points.size);
            for (exint pt_idx = 0; pt_idx < points.size; ++pt_idx) {
                GA_Offset ptoff = startptoff + pt_idx;
                detail->setPos3(ptoff, UT_Vector3(points.array[pt_idx]));
            }

            GEO_PolyCounts polycounts;
            std::vector<int> poly_pt_numbers;
            int prev_n = test_indices.array[0];
            int num_polys_with_same_shape = 0;
            for (std::size_t i = 0; i < test_indices.size; ) {
                auto n = test_indices.array[i++];
                if (n != prev_n) {
                    polycounts.append(prev_n, num_polys_with_same_shape);
                    num_polys_with_same_shape = 0;
                    prev_n = n;
                }
                num_polys_with_same_shape += 1;
                for (std::size_t j = 0; j < n; ++j, ++i) {
                    poly_pt_numbers.push_back(test_indices.array[i]);
                }
            }
            polycounts.append(prev_n, num_polys_with_same_shape); // append last set

            GA_Offset startprimoff = GEO_PrimPoly::buildBlock(
                    detail, startptoff, detail->getNumPointOffsets(),
                    polycounts, poly_pt_numbers.data());

            retrieve_attributes(detail, startprimoff, hr_polymesh_attrib_iter(polymesh, HRAttribLocation::HR_FACE, 0), GA_ATTRIB_PRIMITIVE);
            retrieve_attributes(detail, startvtxoff, hr_polymesh_attrib_iter(polymesh, HRAttribLocation::HR_FACEVERTEX, 0), GA_ATTRIB_VERTEX);
            retrieve_attributes(detail, startptoff, hr_polymesh_attrib_iter(polymesh, HRAttribLocation::HR_VERTEX, 0), GA_ATTRIB_POINT);
        }

        hr_free_point_array(points);
        hr_free_index_array(test_indices);
    }
}

/**
 * Add a ptcloud to the current detail
 */
[[maybe_unused]]
void add_pointcloud(GU_Detail* detail, OwnedPtr<HR_PointCloud> ptcloud_ptr) {
    auto ptcloud = ptcloud_ptr.get();

    if (ptcloud) {
        auto points = hr_get_pointcloud_points(ptcloud);

        GA_Offset startptoff = detail->appendPointBlock(points.size);

        for (exint pt_idx = 0; pt_idx < points.size; ++pt_idx) {
            GA_Offset ptoff = startptoff + pt_idx;
            detail->setPos3(ptoff, UT_Vector3(points.array[pt_idx]));
        }

        retrieve_attributes(detail, startptoff, hr_pointcloud_attrib_iter(ptcloud, HRAttribLocation::HR_VERTEX, 0), GA_ATTRIB_POINT);
        hr_free_point_array(points);
    }
}

/**
 * Update points in the detail according to what's in the ptcloud
 */
[[maybe_unused]]
void update_points(GU_Detail* detail, OwnedPtr<HR_PointCloud> ptcloud_ptr) {
    auto ptcloud = ptcloud_ptr.get();

    if (ptcloud) {
        auto points = hr_get_pointcloud_points(ptcloud);

        for (exint pt_idx = 0; pt_idx < points.size; ++pt_idx) {
            GA_Offset ptoff = detail->pointOffset(pt_idx);
            detail->setPos3(ptoff, UT_Vector3(points.array[pt_idx]));
        }	

        update_attributes(detail, hr_pointcloud_attrib_iter(ptcloud, HRAttribLocation::HR_VERTEX, 0), GA_ATTRIB_POINT);
        hr_free_point_array(points);
    }
}

[[maybe_unused]]
OwnedPtr<HR_TetMesh> build_tetmesh(const GU_Detail *detail) {
    // Get tets for the solid from the first input
    std::vector<double> tet_vertices;
    tet_vertices.reserve(3*detail->getNumPointOffsets());
    std::vector<std::size_t> tet_indices;
    tet_indices.reserve(3*detail->getNumVertexOffsets());

    for ( GA_Offset pt_off : detail->getPointRange() )
    {
        UT_Vector3 pt = detail->getPos3(pt_off);
        tet_vertices.push_back( static_cast<double>(pt[0]) );
        tet_vertices.push_back( static_cast<double>(pt[1]) );
        tet_vertices.push_back( static_cast<double>(pt[2]) );
    }

    std::size_t num_tets = 0;
    for ( GA_Offset prim_off : detail->getPrimitiveRange() )
    {
        const GEO_Primitive *prim = detail->getGEOPrimitive(prim_off);
        if (prim->getTypeId() == GA_PRIMTETRAHEDRON) {
            num_tets += 1;
            const GEO_PrimTetrahedron *tet = static_cast<const GEO_PrimTetrahedron*>(prim);
            tet_indices.push_back(detail->pointIndex(detail->vertexPoint(tet->fastVertexOffset(0))));
            tet_indices.push_back(detail->pointIndex(detail->vertexPoint(tet->fastVertexOffset(1))));
            tet_indices.push_back(detail->pointIndex(detail->vertexPoint(tet->fastVertexOffset(2))));
            tet_indices.push_back(detail->pointIndex(detail->vertexPoint(tet->fastVertexOffset(3))));
        }
    }

    // Only creating a mesh if there are tets.
    if (num_tets > 0) {
        HR_TetMesh *tetmesh = hr_make_tetmesh(tet_vertices.size(), tet_vertices.data(),
                                        tet_indices.size(), tet_indices.data());
        assert(tetmesh);

        transfer_attributes(detail, tetmesh, num_tets);
        return OwnedPtr<HR_TetMesh>(tetmesh);
    }
    return OwnedPtr<HR_TetMesh>(nullptr);
}

[[maybe_unused]]
OwnedPtr<HR_PolyMesh> build_polymesh(const GU_Detail* detail) {
    std::vector<double> poly_vertices;
    poly_vertices.reserve(3*detail->getNumPointOffsets());
    std::vector<std::size_t> poly_indices;
    poly_indices.reserve(3*detail->getNumVertexOffsets());

    for ( GA_Offset pt_off : detail->getPointRange() )
    {
        UT_Vector3 pos = detail->getPos3(pt_off);
        poly_vertices.push_back( static_cast<double>(pos[0]) );
        poly_vertices.push_back( static_cast<double>(pos[1]) );
        poly_vertices.push_back( static_cast<double>(pos[2]) );
    }

    std::size_t num_polys = 0;
    for ( GA_Offset prim_off : detail->getPrimitiveRange() )
    {
        const GEO_Primitive *prim = detail->getGEOPrimitive(prim_off);
        if (prim->getTypeId() == GA_PRIMPOLY) {
            num_polys += 1;
            const GEO_PrimPoly *poly = static_cast<const GEO_PrimPoly*>(prim);
            std::size_t num_verts = poly->getVertexCount();
            poly_indices.push_back(num_verts);
            for ( std::size_t i = 0; i < num_verts; ++i ) {
                GA_Index idx = detail->pointIndex(detail->vertexPoint(poly->getVertexOffset(i)));
                assert(GAisValid(idx));
                poly_indices.push_back(static_cast<std::size_t>(idx));
            }
        }
    }

    // Only creating a mesh if there are polys.
    if (num_polys > 0) {
        HR_PolyMesh *polymesh = hr_make_polymesh(poly_vertices.size(), poly_vertices.data(),
                                           poly_indices.size(), poly_indices.data());
        assert(polymesh);

        transfer_attributes(detail, polymesh, num_polys);
        return OwnedPtr<HR_PolyMesh>(polymesh);
    }
    return OwnedPtr<HR_PolyMesh>(nullptr);
}

[[maybe_unused]]
OwnedPtr<HR_PointCloud> build_pointcloud(const GU_Detail* detail) {
    std::vector<double> vertex_coords(3*detail->getNumPoints());
    std::vector<bool> pt_grp(detail->getNumPointOffsets(), false);

    // We are gonna be smarter here and use block access to point data.
    GA_ROPageHandleV3 P_ph(detail->getP());

    if (P_ph.isValid()) {
        GA_Offset start, end;
        for (GA_Iterator it(detail->getPointRange()); it.blockAdvance(start, end); ) {
            P_ph.setPage(start);
            for (GA_Offset offset = start; offset < end; ++offset) {
                pt_grp[offset] = true;
                auto pos = P_ph.get(offset);
                vertex_coords[3*detail->pointIndex(offset)] = static_cast<double>( pos[0]);
                vertex_coords[3*detail->pointIndex(offset)+1] = static_cast<double>( pos[1]);
                vertex_coords[3*detail->pointIndex(offset)+2] = static_cast<double>( pos[2]);
            }
        }
    }

    HR_PointCloud *ptcloud = hr_make_pointcloud(vertex_coords.size(), vertex_coords.data());
    assert(ptcloud);

    transfer_point_attributes(detail, ptcloud, pt_grp);
    return OwnedPtr<HR_PointCloud>(ptcloud);
}

} // namespace (static)


} // namespace mesh

} // namespace hdkrs
