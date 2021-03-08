#include <optional>
#include <vector>
#include <cassert>
#include <string>
#include <stdexcept>

#include <UT/UT_Debug.h>
#include <GU/GU_Detail.h>
#include <GEO/GEO_PrimTetrahedron.h>
#include <GEO/GEO_PrimPoly.h>
#include <GEO/GEO_PolyCounts.h>
#include <GA/GA_PageHandle.h>

#include <rust/cxx.h>
#include "hdkrs/src/lib.rs.h"

#include "mesh.h"

using namespace hdkrs;

inline std::ostream& operator<<(std::ostream& out, AttribLocation where) {
    switch (where) {
        case AttribLocation::VERTEX: out << "Vertex"; break;
        case AttribLocation::FACE: out << "Face"; break;
        case AttribLocation::CELL: out << "Cell"; break;
        case AttribLocation::FACEVERTEX: out << "FaceVertex"; break;
        case AttribLocation::CELLVERTEX: out << "CellVertex"; break;
        default: break;
    }
    return out;
}

// The following items define various version of the add_attrib function which
// adds an attribute of a given type to a given mesh or point cloud.

#define HR_ADD_NUM_ATTRIB_IMPL(MTYPE, TYPE, FN) \
    void add_attrib( \
            MTYPE *m, \
            AttribLocation where, \
            const char *name, \
            std::size_t tuple_size, \
            const std::vector<TYPE> &data) \
    { \
        m->FN( where, rust::Str(name), tuple_size, rust::Slice(data.data(), data.size()) ); \
    }

HR_ADD_NUM_ATTRIB_IMPL(PointCloud, int8, add_attrib_i8)
HR_ADD_NUM_ATTRIB_IMPL(PointCloud, int32, add_attrib_i32)
HR_ADD_NUM_ATTRIB_IMPL(PointCloud, int64_t, add_attrib_i64)
HR_ADD_NUM_ATTRIB_IMPL(PointCloud, fpreal32, add_attrib_f32)
HR_ADD_NUM_ATTRIB_IMPL(PointCloud, fpreal64, add_attrib_f64)

HR_ADD_NUM_ATTRIB_IMPL(PolyMesh, int8, add_attrib_i8)
HR_ADD_NUM_ATTRIB_IMPL(PolyMesh, int32, add_attrib_i32)
HR_ADD_NUM_ATTRIB_IMPL(PolyMesh, int64_t, add_attrib_i64)
HR_ADD_NUM_ATTRIB_IMPL(PolyMesh, fpreal32, add_attrib_f32)
HR_ADD_NUM_ATTRIB_IMPL(PolyMesh, fpreal64, add_attrib_f64)

HR_ADD_NUM_ATTRIB_IMPL(TetMesh, int8, add_attrib_i8)
HR_ADD_NUM_ATTRIB_IMPL(TetMesh, int32, add_attrib_i32)
HR_ADD_NUM_ATTRIB_IMPL(TetMesh, int64_t, add_attrib_i64)
HR_ADD_NUM_ATTRIB_IMPL(TetMesh, fpreal32, add_attrib_f32)
HR_ADD_NUM_ATTRIB_IMPL(TetMesh, fpreal64, add_attrib_f64)

#undef ADD_NUM_ATTRIB_IMPL

void add_attrib(
        PointCloud *ptcloud,
        AttribLocation where,
        const char *name,
        std::size_t tuple_size,
        const std::vector<rust::Str> &strings,
        const std::vector<int64_t> &indices)
{
    ptcloud->add_attrib_str(
            where, rust::Str(name), tuple_size,
            rust::Slice(strings.data(), strings.size()),
            rust::Slice(indices.data(), indices.size()));
}

void add_attrib(
        PolyMesh *polymesh,
        AttribLocation where,
        const char *name,
        std::size_t tuple_size,
        const std::vector<rust::Str> &strings,
        const std::vector<int64_t> &indices)
{
    polymesh->add_attrib_str(
            where, rust::Str(name), tuple_size,
            rust::Slice(strings.data(), strings.size()),
            rust::Slice(indices.data(), indices.size()));
}

void add_attrib(
        TetMesh *tetmesh,
        AttribLocation where,
        const char *name,
        std::size_t tuple_size,
        const std::vector<rust::Str> &strings,
        const std::vector<int64_t> &indices)
{
    tetmesh->add_attrib_str(
            where, rust::Str(name), tuple_size,
            rust::Slice(strings.data(), strings.size()),
            rust::Slice(indices.data(), indices.size()));
}

template<typename T>
GA_PrimitiveTypeId mesh_prim_type_id();

template<>
GA_PrimitiveTypeId mesh_prim_type_id<PolyMesh>() { return GA_PRIMPOLY; }

template<>
GA_PrimitiveTypeId mesh_prim_type_id<TetMesh>() { return GA_PRIMTETRAHEDRON; }

template<typename T>
AttribLocation mesh_prim_attrib_location();

template<>
AttribLocation mesh_prim_attrib_location<PolyMesh>() { return AttribLocation::FACE; }

template<>
AttribLocation mesh_prim_attrib_location<TetMesh>() { return AttribLocation::CELL; }

template<typename T>
AttribLocation mesh_vertex_attrib_location();

template<>
AttribLocation mesh_vertex_attrib_location<PolyMesh>() { return AttribLocation::FACEVERTEX; }

template<>
AttribLocation mesh_vertex_attrib_location<TetMesh>() { return AttribLocation::CELLVERTEX; }

// Mark all points and vectors in the given detail that intersect the primitives of interest.
std::tuple<std::vector<bool>, std::size_t>
mark_points_and_count_vertices(
        const GU_Detail& detail,
        GA_PrimitiveTypeId prim_type_id)
{
    std::vector<bool> points(detail.getNumPointOffsets(), false);
    std::size_t num_vertices = 0;
    for ( GA_Offset prim_off : detail.getPrimitiveRange() )
    {
        const GEO_Primitive *prim = detail.getGEOPrimitive(prim_off);
        if (prim->getTypeId() == prim_type_id)
        {
            GA_Size num_prim_verts = detail.getPrimitiveVertexCount(prim_off);
            num_vertices += num_prim_verts;
            for ( GA_Size idx = 0; idx < num_prim_verts; ++idx ) {
                auto vtx_off = detail.getPrimitiveVertexOffset(prim_off, idx);
                points[detail.vertexPoint(vtx_off)] = true;
            }
        }
    }

    return std::make_pair(std::move(points), num_vertices);
}

template<typename T, typename M, typename S = T>
void
fill_prim_attrib(
        const GU_Detail& detail,
        const GA_AIFTuple *aif,
        const GA_Attribute *attrib,
        std::size_t tuple_size,
        std::size_t num_elem,
        M *mesh)
{
    std::vector<T> data(tuple_size*num_elem);
    int i = 0;
    for ( GA_Offset prim_off : detail.getPrimitiveRange() )
    {
        const GEO_Primitive *prim = detail.getGEOPrimitive(prim_off);
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
        const GU_Detail &detail,
        const GA_AIFTuple *aif,
        const GA_Attribute *attrib,
        std::size_t tuple_size,
        std::size_t num_elem,
        const std::vector<bool> &group,
        M *mesh)
{
    std::vector<T> data(tuple_size*num_elem);
    int i = 0;
    for ( GA_Offset pt_off : detail.getPointRange() )
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
    add_attrib(mesh, AttribLocation::VERTEX, name, tuple_size, data);
}

template<typename T, typename M, typename S = T>
void fill_vertex_attrib(
        const GU_Detail &detail,
        const GA_AIFTuple *aif,
        const GA_Attribute *attrib,
        std::size_t tuple_size,
        std::size_t num_elem,
        M *mesh)
{
    std::vector<T> data(tuple_size*num_elem);
    int i = 0;
    for ( GA_Offset prim_off : detail.getPrimitiveRange() ) {
        const GEO_Primitive *prim = detail.getGEOPrimitive(prim_off);
        if (prim->getTypeId() != mesh_prim_type_id<M>()) continue;
        GA_Size num_prim_verts = detail.getPrimitiveVertexCount(prim_off);
        for (GA_Size idx = 0; idx < num_prim_verts; ++idx) {
            auto vtx_off = detail.getPrimitiveVertexOffset(prim_off, idx);
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
        const GU_Detail &detail,
        const GA_AIFSharedStringTuple *aif,
        const GA_Attribute *attrib,
        std::size_t tuple_size,
        std::size_t num_elem,
        M *mesh)
{
    // Try with different types
    std::vector<int64_t> ids(aif->getTableEntries(attrib), -1);
    std::vector<rust::Str> strings;
    for (auto it = aif->begin(attrib); !it.atEnd(); ++it) {
        ids[it.getHandle()] = strings.size();
        strings.push_back( rust::Str(it.getString()) );
    }

    std::vector<int64_t> indices(tuple_size*num_elem, -1);

    int i = 0;
    for ( GA_Offset prim_off : detail.getPrimitiveRange() )
    {
        const GEO_Primitive *prim = detail.getGEOPrimitive(prim_off);
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
        const GU_Detail &detail,
        const GA_AIFSharedStringTuple *aif,
        const GA_Attribute *attrib,
        std::size_t tuple_size,
        std::size_t num_elem,
        const std::vector<bool> &group,
        M *mesh)
{
    // Try with different types
    std::vector<int64_t> ids(aif->getTableEntries(attrib), -1);
    std::vector<rust::Str> strings;
    for (auto it = aif->begin(attrib); !it.atEnd(); ++it) {
        ids[it.getHandle()] = strings.size();
        strings.push_back( rust::Str(it.getString()) );
    }

    std::vector<int64_t> indices(tuple_size*num_elem, -1);

    int i = 0;
    for ( GA_Offset pt_off : detail.getPointRange() )
    {
        if (!group[pt_off]) continue;
        for ( int k = 0, k_end = tuple_size; k < k_end; ++k ) {
            GA_StringIndexType handle = aif->getHandle(attrib, pt_off, k);
            indices[tuple_size*i + k] = handle > -1 ? ids[handle] : -1;
        }
        i += 1;
    }

    auto name = attrib->getName().c_str();
    add_attrib(mesh, AttribLocation::VERTEX, name, tuple_size, strings, indices);
}

template<typename M>
void fill_vertex_str_attrib(
        const GU_Detail &detail,
        const GA_AIFSharedStringTuple *aif,
        const GA_Attribute *attrib,
        std::size_t tuple_size,
        std::size_t num_elem,
        M *mesh)
{
    // Try with different types
    std::vector<int64_t> ids(aif->getTableEntries(attrib), -1);
    std::vector<rust::Str> strings;
    for (auto it = aif->begin(attrib); !it.atEnd(); ++it) {
        ids[it.getHandle()] = strings.size();
        strings.push_back( rust::Str(it.getString()) );
    }

    std::vector<int64_t> indices(tuple_size*num_elem, -1);

    int i = 0;
    for ( GA_Offset prim_off : detail.getPrimitiveRange() ) {
        const GEO_Primitive *prim = detail.getGEOPrimitive(prim_off);
        if (prim->getTypeId() != mesh_prim_type_id<M>()) continue;
        GA_Size num_prim_verts = detail.getPrimitiveVertexCount(prim_off);
        for (GA_Size idx = 0; idx < num_prim_verts; ++idx) {
            auto vtx_off = detail.getPrimitiveVertexOffset(prim_off, idx);
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
void transfer_primitive_attributes(const GU_Detail& detail, M* mesh, std::size_t num_prims) {
    // Get prim data attributes
    for (auto it = detail.getAttributeDict(GA_ATTRIB_PRIMITIVE).begin(GA_SCOPE_PUBLIC); !it.atEnd(); ++it)
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
void transfer_point_attributes(const GU_Detail& detail, M* mesh, const std::vector<bool>& pt_grp)
{
    std::size_t num_points = std::count(pt_grp.begin(), pt_grp.end(), true);
    for (auto it = detail.getAttributeDict(GA_ATTRIB_POINT).begin(GA_SCOPE_PUBLIC); !it.atEnd(); ++it)
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
void transfer_vertex_attributes(const GU_Detail& detail, M* mesh, std::size_t num_vertices)
{
    for (auto it = detail.getAttributeDict(GA_ATTRIB_VERTEX).begin(GA_SCOPE_PUBLIC); !it.atEnd(); ++it)
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
void transfer_attributes(const GU_Detail& detail, M* mesh, std::size_t num_prims)
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
    auto n = startoff + (arr.vec.size()/arr.tuple_size);
    for ( GA_Offset off = startoff; off < n; ++off, ++i ) {
        for ( int j = 0; j < arr.tuple_size; ++j ) {
            h.set(off, j, arr.vec[arr.tuple_size*i + j]);
        }
    }
}

void fill_str_attrib(GA_RWHandleS h, const rust::box<TupleVecStr> &arr, GA_Offset startoff) {
    if (h.isInvalid()) return;
    std::size_t i = 0;
    auto n = startoff + (arr->len()/arr->tuple_size());
    for ( GA_Offset off = startoff; off < n; ++off, ++i ) {
        for ( int j = 0; j < arr->tuple_size(); ++j ) {
            h.set(off, j, arr->at(arr->tuple_size()*i + j).data());
        }
    }
}

/** Retrieve attributes from the mesh using the given iterator.
 */
void retrieve_attributes(GU_Detail *detail, GA_Offset startoff, rust::box<AttribIter> it, GA_AttributeOwner owner) {
    for ( ;; ) {
        if (!it->has_next()) break;
        auto attrib = it->next();
        auto name = UT_String(std::string(attrib->name()));
        name.forceValidVariableName();
        auto type = attrib->data_type();
        if (type == DataType::I8 ) {
            auto arr = attrib->get_data_i8();
            auto h = GA_RWHandleC(detail->addTuple(GA_STORE_INT8, owner, name, arr.tuple_size));
            fill_attrib(h, arr, startoff);
        } else if (type == DataType::I32 ) {
            auto arr = attrib->get_data_i32();
            auto h = GA_RWHandleI(detail->addTuple(GA_STORE_INT32, owner, name, arr.tuple_size));
            fill_attrib(h, arr, startoff);
        } else if (type == DataType::I64 ) {
            auto arr = attrib->get_data_i64();
            auto h = GA_RWHandleID(detail->addTuple(GA_STORE_INT64, owner, name, arr.tuple_size));
            fill_attrib(h, arr, startoff);
        } else if (type == DataType::F32 ) {
            auto arr = attrib->get_data_f32();
            auto h = GA_RWHandleF(detail->addTuple(GA_STORE_REAL32, owner, name, arr.tuple_size));
            fill_attrib(h, arr, startoff);
        } else if (type == DataType::F64 ) {
            auto arr = attrib->get_data_f64();
            auto h = GA_RWHandleD(detail->addTuple(GA_STORE_REAL64, owner, name, arr.tuple_size));
            fill_attrib(h, arr, startoff);
        } else if (type == DataType::STR ) {
            auto box_arr = attrib->get_data_str();
            auto h = GA_RWHandleS(detail->addTuple(GA_STORE_STRING, owner, name, box_arr->tuple_size()));
            fill_str_attrib(h, box_arr, startoff);
        }
    }
}


template<typename HandleType, typename ArrayType>
void update_attrib(HandleType h, ArrayType arr) {
    if (h.isInvalid()) return;
    int n = (arr.vec.size()/arr.tuple_size);
    for ( int j = 0; j < arr.tuple_size; ++j ) {
        h.setBlockFromIndices(GA_Index(0), GA_Size(n), arr.vec.data(), arr.tuple_size, j);
    }
}

/** Update attributes of the mesh using the given iterator.
 */
void update_attributes(GU_Detail *detail, rust::box<AttribIter> it, GA_AttributeOwner owner) {
    for ( ;; ) {
        if ( !it->has_next() ) break;
        auto attrib = it->next();
        auto name = UT_String(std::string(attrib->name()));
        name.forceValidVariableName();
        auto type = attrib->data_type();
        if (type == DataType::I8 ) {
            auto arr = attrib->get_data_i8();
            auto h = GA_RWHandleC(detail->addTuple(GA_STORE_INT8, owner, name, arr.tuple_size));
            update_attrib(h, arr);
        } else if (type == DataType::I32 ) {
            auto arr = attrib->get_data_i32();
            auto h = GA_RWHandleI(detail->addTuple(GA_STORE_INT32, owner, name, arr.tuple_size));
            update_attrib(h, arr);
        } else if (type == DataType::I64 ) {
            auto arr = attrib->get_data_i64();
            auto h = GA_RWHandleID(detail->addTuple(GA_STORE_INT64, owner, name, arr.tuple_size));
            update_attrib(h, arr);
        } else if (type == DataType::F32 ) {
            auto arr = attrib->get_data_f32();
            auto h = GA_RWHandleF(detail->addTuple(GA_STORE_REAL32, owner, name, arr.tuple_size));
            update_attrib(h, arr);
        } else if (type == DataType::F64 ) {
            auto arr = attrib->get_data_f64();
            auto h = GA_RWHandleD(detail->addTuple(GA_STORE_REAL64, owner, name, arr.tuple_size));
            update_attrib(h, arr);
        } // String attributes are not yet supported by updates
    }
}

/**
 * Add a mesh to the current detail.
 */
void add_mesh(GU_Detail* detail, rust::box<Mesh> mesh) {
    // No exceptions should be thrown here since we check the tag explicitly.
    switch (mesh->tag()) {
        case MeshTag::TetMesh:
            add_tetmesh(detail, into_tetmesh(std::move(mesh)));
            break;
        case MeshTag::PolyMesh:
            add_polymesh(detail, into_polymesh(std::move(mesh)));
            break;
        case MeshTag::PointCloud:
            add_pointcloud(detail, into_pointcloud(std::move(mesh)));
            break;
        default: break; // Do nothing
    }
}

/**
 * Add a tetmesh to the current detail
 */
void add_tetmesh(GU_Detail* detail, rust::box<TetMesh> tetmesh) {
    try {
        GA_Offset startvtxoff = GA_Offset(detail->getNumVertexOffsets());
        auto point_coords = tetmesh->get_point_coords();
        auto num_points = point_coords.size()/3;
        auto unsigned_indices = tetmesh->get_indices();
        if (unsigned_indices.size() > 0) {
            std::vector<int> indices(unsigned_indices.begin(), unsigned_indices.end());

            GA_Offset startptoff = detail->appendPointBlock(num_points);
            for (exint pt_idx = 0; pt_idx < num_points; ++pt_idx) {
                GA_Offset ptoff = startptoff + pt_idx;
                detail->setPos3(ptoff, UT_Vector3(&point_coords[3*pt_idx]));
            }

            GA_Offset startprimoff = GEO_PrimTetrahedron::buildBlock(
                    detail, startptoff, detail->getNumPointOffsets(),
                    indices.size()/4, indices.data());

            retrieve_attributes(detail, startptoff, tetmesh->attrib_iter(AttribLocation::VERTEX), GA_ATTRIB_POINT);
            retrieve_attributes(detail, startprimoff, tetmesh->attrib_iter(AttribLocation::CELL), GA_ATTRIB_PRIMITIVE);
            retrieve_attributes(detail, startvtxoff, tetmesh->attrib_iter(AttribLocation::CELLVERTEX), GA_ATTRIB_VERTEX);
        }
    } catch (const rust::Error &e) {
        UT_ASSERT_MSG(false, "Attribute error");
    }
}

/**
 * Add a polymesh to the current detail
 */
void add_polymesh(GU_Detail* detail, rust::box<PolyMesh> polymesh) {
    GA_Offset startvtxoff = GA_Offset(detail->getNumVertexOffsets());
    auto point_coords = polymesh->get_point_coords();
    auto num_points = point_coords.size()/3;
    auto test_indices = polymesh->get_indices();
    if (test_indices.size() > 0) {
        GA_Offset startptoff = detail->appendPointBlock(num_points);
        for (exint pt_idx = 0; pt_idx < num_points; ++pt_idx) {
            GA_Offset ptoff = startptoff + pt_idx;
            detail->setPos3(ptoff, UT_Vector3(&point_coords[3*pt_idx]));
        }

        GEO_PolyCounts polycounts;
        std::vector<int> poly_pt_numbers;
        int prev_n = test_indices[0];
        int num_polys_with_same_shape = 0;
        for (std::size_t i = 0; i < test_indices.size(); ) {
            auto n = test_indices[i++];
            if (n != prev_n) {
                polycounts.append(prev_n, num_polys_with_same_shape);
                num_polys_with_same_shape = 0;
                prev_n = n;
            }
            num_polys_with_same_shape += 1;
            for (std::size_t j = 0; j < n; ++j, ++i) {
                poly_pt_numbers.push_back(test_indices[i]);
            }
        }
        polycounts.append(prev_n, num_polys_with_same_shape); // append last set

        GA_Offset startprimoff = GEO_PrimPoly::buildBlock(
                detail, startptoff, detail->getNumPointOffsets(),
                polycounts, poly_pt_numbers.data());

        retrieve_attributes(detail, startprimoff, polymesh->attrib_iter(AttribLocation::FACE), GA_ATTRIB_PRIMITIVE);
        retrieve_attributes(detail, startvtxoff, polymesh->attrib_iter(AttribLocation::FACEVERTEX), GA_ATTRIB_VERTEX);
        retrieve_attributes(detail, startptoff, polymesh->attrib_iter(AttribLocation::VERTEX), GA_ATTRIB_POINT);
    }
}

/**
 * Add a ptcloud to the current detail
 */
void add_pointcloud(GU_Detail* detail, rust::box<PointCloud> ptcloud) {
    auto point_coords = ptcloud->get_point_coords();
    auto num_points = point_coords.size()/3;

    GA_Offset startptoff = detail->appendPointBlock(num_points);

    for (exint pt_idx = 0; pt_idx < num_points; ++pt_idx) {
        GA_Offset ptoff = startptoff + pt_idx;
        detail->setPos3(ptoff, UT_Vector3(&point_coords[3*pt_idx]));
    }

    retrieve_attributes(detail, startptoff, ptcloud->attrib_iter(AttribLocation::VERTEX), GA_ATTRIB_POINT);
}

/**
 * Update points in the detail according to what's in the ptcloud
 */
void update_points(GU_Detail* detail, rust::box<PointCloud> ptcloud) {
    auto point_coords = ptcloud->get_point_coords();
    auto num_points = point_coords.size()/3;

    for (exint pt_idx = 0; pt_idx < num_points; ++pt_idx) {
        GA_Offset ptoff = detail->pointOffset(pt_idx);
        detail->setPos3(ptoff, UT_Vector3(&point_coords[3*pt_idx]));
    }	

    update_attributes(detail, ptcloud->attrib_iter(AttribLocation::VERTEX), GA_ATTRIB_POINT);
}

rust::box<TetMesh> build_tetmesh(const GU_Detail& detail) {
    // Get tets for the solid from the first input
    std::vector<double> tet_vertices;
    tet_vertices.reserve(3*detail.getNumPointOffsets());
    std::vector<std::size_t> tet_indices;
    tet_indices.reserve(3*detail.getNumVertexOffsets());

    for ( GA_Offset pt_off : detail.getPointRange() )
    {
        UT_Vector3 pt = detail.getPos3(pt_off);
        tet_vertices.push_back( static_cast<double>(pt[0]) );
        tet_vertices.push_back( static_cast<double>(pt[1]) );
        tet_vertices.push_back( static_cast<double>(pt[2]) );
    }

    std::size_t num_tets = 0;
    for ( GA_Offset prim_off : detail.getPrimitiveRange() )
    {
        const GEO_Primitive *prim = detail.getGEOPrimitive(prim_off);
        if (prim->getTypeId() == GA_PRIMTETRAHEDRON) {
            num_tets += 1;
            const GEO_PrimTetrahedron *tet = static_cast<const GEO_PrimTetrahedron*>(prim);
            tet_indices.push_back(detail.pointIndex(detail.vertexPoint(tet->fastVertexOffset(0))));
            tet_indices.push_back(detail.pointIndex(detail.vertexPoint(tet->fastVertexOffset(1))));
            tet_indices.push_back(detail.pointIndex(detail.vertexPoint(tet->fastVertexOffset(2))));
            tet_indices.push_back(detail.pointIndex(detail.vertexPoint(tet->fastVertexOffset(3))));
        }
    }

    // Only creating a mesh if there are tets.
    if (num_tets == 0) {
        throw std::runtime_error("No tetrahedra found");
    }

    rust::box<TetMesh> tetmesh = make_tetmesh(
            rust::Slice(static_cast<const double *>(tet_vertices.data()), tet_vertices.size()),
            rust::Slice(static_cast<const uint64_t *>(tet_indices.data()), tet_indices.size()));

    TetMesh* tetmesh_ptr = tetmesh.into_raw();
    transfer_attributes(detail, tetmesh_ptr, num_tets);
    return rust::box<TetMesh>::from_raw(tetmesh_ptr);
}

rust::box<PolyMesh> build_polymesh(const GU_Detail& detail) {
    std::vector<double> poly_vertices;
    poly_vertices.reserve(3*detail.getNumPointOffsets());
    std::vector<std::size_t> poly_indices;
    poly_indices.reserve(3*detail.getNumVertexOffsets());

    for ( GA_Offset pt_off : detail.getPointRange() )
    {
        UT_Vector3 pos = detail.getPos3(pt_off);
        poly_vertices.push_back( static_cast<double>(pos[0]) );
        poly_vertices.push_back( static_cast<double>(pos[1]) );
        poly_vertices.push_back( static_cast<double>(pos[2]) );
    }

    std::size_t num_polys = 0;
    for ( GA_Offset prim_off : detail.getPrimitiveRange() )
    {
        const GEO_Primitive *prim = detail.getGEOPrimitive(prim_off);
        if (prim->getTypeId() == GA_PRIMPOLY) {
            num_polys += 1;
            const GEO_PrimPoly *poly = static_cast<const GEO_PrimPoly*>(prim);
            std::size_t num_verts = poly->getVertexCount();
            poly_indices.push_back(num_verts);
            for ( std::size_t i = 0; i < num_verts; ++i ) {
                GA_Index idx = detail.pointIndex(detail.vertexPoint(poly->getVertexOffset(i)));
                assert(GAisValid(idx));
                poly_indices.push_back(static_cast<std::size_t>(idx));
            }
        }
    }

    // Only creating a mesh if there are polys.
    if (num_polys == 0) {
        throw std::runtime_error("No polygons found");
    }

    rust::Slice vertices_slice(static_cast<const double *>(poly_vertices.data()), poly_vertices.size());
    rust::Slice indices_slice(static_cast<const uint64_t *>(poly_indices.data()), poly_indices.size());
    rust::box<PolyMesh> polymesh = make_polymesh(vertices_slice, indices_slice);

    auto polymesh_ptr = polymesh.into_raw();
    transfer_attributes(detail, polymesh_ptr, num_polys);
    return rust::box<PolyMesh>::from_raw(polymesh_ptr);
}

rust::box<PointCloud> build_pointcloud(const GU_Detail& detail) {
    std::vector<double> vertex_coords(3*detail.getNumPoints());
    std::vector<bool> pt_grp(detail.getNumPointOffsets(), false);

    // We are gonna be smarter here and use block access to point data.
    GA_ROPageHandleV3 P_ph(detail.getP());

    if (P_ph.isValid()) {
        GA_Offset start, end;
        for (GA_Iterator it(detail.getPointRange()); it.blockAdvance(start, end); ) {
            P_ph.setPage(start);
            for (GA_Offset offset = start; offset < end; ++offset) {
                pt_grp[offset] = true;
                auto pos = P_ph.get(offset);
                vertex_coords[3*detail.pointIndex(offset)] = static_cast<double>( pos[0]);
                vertex_coords[3*detail.pointIndex(offset)+1] = static_cast<double>( pos[1]);
                vertex_coords[3*detail.pointIndex(offset)+2] = static_cast<double>( pos[2]);
            }
        }
    }

    rust::Slice vertex_coords_slice(static_cast<const double *>(vertex_coords.data()), vertex_coords.size());
    rust::box<PointCloud> ptcloud = make_pointcloud(vertex_coords_slice);

    auto ptcloud_ptr = ptcloud.into_raw();
    transfer_point_attributes(detail, ptcloud_ptr, pt_grp);
    return rust::box<PointCloud>::from_raw(ptcloud_ptr);
}