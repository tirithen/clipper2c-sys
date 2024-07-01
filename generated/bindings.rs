/* automatically generated by rust-bindgen 0.69.4 */

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ClipperClipper64 {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ClipperClipperD {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ClipperClipperOffset {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ClipperPath64 {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ClipperPathD {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ClipperPaths64 {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ClipperPathsD {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ClipperPolyTree64 {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ClipperPolyTreeD {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct ClipperPointD {
    pub x: f64,
    pub y: f64,
}
#[test]
fn bindgen_test_layout_ClipperPointD() {
    const UNINIT: ::std::mem::MaybeUninit<ClipperPointD> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<ClipperPointD>(),
        16usize,
        concat!("Size of: ", stringify!(ClipperPointD))
    );
    assert_eq!(
        ::std::mem::align_of::<ClipperPointD>(),
        8usize,
        concat!("Alignment of ", stringify!(ClipperPointD))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).x) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ClipperPointD),
            "::",
            stringify!(x)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).y) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(ClipperPointD),
            "::",
            stringify!(y)
        )
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
#[cfg_attr(
	feature = "serde",
	derive(serde::Serialize, serde::Deserialize)
)]
pub struct ClipperPoint64 {
    pub x: i64,
    pub y: i64,
}
#[test]
fn bindgen_test_layout_ClipperPoint64() {
    const UNINIT: ::std::mem::MaybeUninit<ClipperPoint64> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<ClipperPoint64>(),
        16usize,
        concat!("Size of: ", stringify!(ClipperPoint64))
    );
    assert_eq!(
        ::std::mem::align_of::<ClipperPoint64>(),
        8usize,
        concat!("Alignment of ", stringify!(ClipperPoint64))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).x) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ClipperPoint64),
            "::",
            stringify!(x)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).y) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(ClipperPoint64),
            "::",
            stringify!(y)
        )
    );
}
pub const ClipperFillRule_EVEN_ODD: ClipperFillRule = 0;
pub const ClipperFillRule_NON_ZERO: ClipperFillRule = 1;
pub const ClipperFillRule_POSITIVE: ClipperFillRule = 2;
pub const ClipperFillRule_NEGATIVE: ClipperFillRule = 3;
pub type ClipperFillRule = ::std::os::raw::c_uint;
pub const ClipperClipType_NONE: ClipperClipType = 0;
pub const ClipperClipType_INTERSECTION: ClipperClipType = 1;
pub const ClipperClipType_UNION: ClipperClipType = 2;
pub const ClipperClipType_DIFFERENCE: ClipperClipType = 3;
pub const ClipperClipType_XOR: ClipperClipType = 4;
pub type ClipperClipType = ::std::os::raw::c_uint;
pub const ClipperPathType_SUBJECT: ClipperPathType = 0;
pub const ClipperPathType_CLIP: ClipperPathType = 1;
pub type ClipperPathType = ::std::os::raw::c_uint;
pub const ClipperJoinType_SQUARE_JOIN: ClipperJoinType = 0;
pub const ClipperJoinType_BEVEL_JOIN: ClipperJoinType = 1;
pub const ClipperJoinType_ROUND_JOIN: ClipperJoinType = 2;
pub const ClipperJoinType_MITER_JOIN: ClipperJoinType = 3;
pub type ClipperJoinType = ::std::os::raw::c_uint;
pub const ClipperEndType_POLYGON_END: ClipperEndType = 0;
pub const ClipperEndType_JOINED_END: ClipperEndType = 1;
pub const ClipperEndType_BUTT_END: ClipperEndType = 2;
pub const ClipperEndType_SQUARE_END: ClipperEndType = 3;
pub const ClipperEndType_ROUND_END: ClipperEndType = 4;
pub type ClipperEndType = ::std::os::raw::c_uint;
pub const ClipperPointInPolygonResult_IS_ON: ClipperPointInPolygonResult = 0;
pub const ClipperPointInPolygonResult_IS_INSIDE: ClipperPointInPolygonResult = 1;
pub const ClipperPointInPolygonResult_IS_OUTSIDE: ClipperPointInPolygonResult = 2;
pub type ClipperPointInPolygonResult = ::std::os::raw::c_uint;
extern "C" {
    pub fn clipper_paths64_inflate(
        mem: *mut ::std::os::raw::c_void,
        paths: *mut ClipperPaths64,
        delta: f64,
        jt: ClipperJoinType,
        et: ClipperEndType,
        miter_limit: f64,
    ) -> *mut ClipperPaths64;
}
extern "C" {
    pub fn clipper_pathsd_inflate(
        mem: *mut ::std::os::raw::c_void,
        paths: *mut ClipperPathsD,
        delta: f64,
        jt: ClipperJoinType,
        et: ClipperEndType,
        miter_limit: f64,
        precision: ::std::os::raw::c_int,
    ) -> *mut ClipperPathsD;
}
extern "C" {
    pub fn clipper_path64(mem: *mut ::std::os::raw::c_void) -> *mut ClipperPath64;
}
extern "C" {
    pub fn clipper_pathd(mem: *mut ::std::os::raw::c_void) -> *mut ClipperPathD;
}
extern "C" {
    pub fn clipper_path64_of_points(
        mem: *mut ::std::os::raw::c_void,
        pts: *mut ClipperPoint64,
        len_pts: usize,
    ) -> *mut ClipperPath64;
}
extern "C" {
    pub fn clipper_pathd_of_points(
        mem: *mut ::std::os::raw::c_void,
        pts: *mut ClipperPointD,
        len_pts: usize,
    ) -> *mut ClipperPathD;
}
extern "C" {
    pub fn clipper_path64_add_point(path: *mut ClipperPath64, pt: ClipperPoint64);
}
extern "C" {
    pub fn clipper_pathd_add_point(path: *mut ClipperPathD, pt: ClipperPointD);
}
extern "C" {
    pub fn clipper_paths64(mem: *mut ::std::os::raw::c_void) -> *mut ClipperPaths64;
}
extern "C" {
    pub fn clipper_pathsd(mem: *mut ::std::os::raw::c_void) -> *mut ClipperPathsD;
}
extern "C" {
    pub fn clipper_paths64_of_paths(
        mem: *mut ::std::os::raw::c_void,
        paths: *mut *mut ClipperPath64,
        len_paths: usize,
    ) -> *mut ClipperPaths64;
}
extern "C" {
    pub fn clipper_pathsd_of_paths(
        mem: *mut ::std::os::raw::c_void,
        paths: *mut *mut ClipperPathD,
        len_paths: usize,
    ) -> *mut ClipperPathsD;
}
extern "C" {
    pub fn clipper_paths64_add_path(paths: *mut ClipperPaths64, p: *mut ClipperPath64);
}
extern "C" {
    pub fn clipper_pathsd_add_path(paths: *mut ClipperPathsD, p: *mut ClipperPathD);
}
extern "C" {
    pub fn clipper_paths64_add_paths(a: *mut ClipperPaths64, b: *mut ClipperPaths64);
}
extern "C" {
    pub fn clipper_pathsd_add_paths(a: *mut ClipperPathsD, b: *mut ClipperPathsD);
}
extern "C" {
    pub fn clipper_path64_length(path: *mut ClipperPath64) -> usize;
}
extern "C" {
    pub fn clipper_pathd_length(path: *mut ClipperPathD) -> usize;
}
extern "C" {
    pub fn clipper_path64_get_point(
        path: *mut ClipperPath64,
        idx: ::std::os::raw::c_int,
    ) -> ClipperPoint64;
}
extern "C" {
    pub fn clipper_pathd_get_point(
        path: *mut ClipperPathD,
        idx: ::std::os::raw::c_int,
    ) -> ClipperPointD;
}
extern "C" {
    pub fn clipper_paths64_length(paths: *mut ClipperPaths64) -> usize;
}
extern "C" {
    pub fn clipper_pathsd_length(paths: *mut ClipperPathsD) -> usize;
}
extern "C" {
    pub fn clipper_paths64_path_length(
        paths: *mut ClipperPaths64,
        idx: ::std::os::raw::c_int,
    ) -> usize;
}
extern "C" {
    pub fn clipper_pathsd_path_length(
        paths: *mut ClipperPathsD,
        idx: ::std::os::raw::c_int,
    ) -> usize;
}
extern "C" {
    pub fn clipper_paths64_get_path(
        mem: *mut ::std::os::raw::c_void,
        paths: *mut ClipperPaths64,
        idx: ::std::os::raw::c_int,
    ) -> *mut ClipperPath64;
}
extern "C" {
    pub fn clipper_pathsd_get_path(
        mem: *mut ::std::os::raw::c_void,
        paths: *mut ClipperPathsD,
        idx: ::std::os::raw::c_int,
    ) -> *mut ClipperPathD;
}
extern "C" {
    pub fn clipper_paths64_get_point(
        paths: *mut ClipperPaths64,
        path_idx: ::std::os::raw::c_int,
        point_idx: ::std::os::raw::c_int,
    ) -> ClipperPoint64;
}
extern "C" {
    pub fn clipper_pathsd_get_point(
        paths: *mut ClipperPathsD,
        path_idx: ::std::os::raw::c_int,
        point_idx: ::std::os::raw::c_int,
    ) -> ClipperPointD;
}
extern "C" {
    pub fn clipper_path64_simplify(
        mem: *mut ::std::os::raw::c_void,
        path: *mut ClipperPath64,
        epsilon: f64,
        is_open_path: ::std::os::raw::c_int,
    ) -> *mut ClipperPath64;
}
extern "C" {
    pub fn clipper_pathd_simplify(
        mem: *mut ::std::os::raw::c_void,
        path: *mut ClipperPathD,
        epsilon: f64,
        is_open_path: ::std::os::raw::c_int,
    ) -> *mut ClipperPathD;
}
extern "C" {
    pub fn clipper_paths64_simplify(
        mem: *mut ::std::os::raw::c_void,
        paths: *mut ClipperPaths64,
        epsilon: f64,
        is_open_paths: ::std::os::raw::c_int,
    ) -> *mut ClipperPaths64;
}
extern "C" {
    pub fn clipper_pathsd_simplify(
        mem: *mut ::std::os::raw::c_void,
        paths: *mut ClipperPathsD,
        epsilon: f64,
        is_open_paths: ::std::os::raw::c_int,
    ) -> *mut ClipperPathsD;
}
extern "C" {
    pub fn clipper_path64_to_pathd(
        mem: *mut ::std::os::raw::c_void,
        path: *mut ClipperPath64,
    ) -> *mut ClipperPathD;
}
extern "C" {
    pub fn clipper_pathd_to_path64(
        mem: *mut ::std::os::raw::c_void,
        path: *mut ClipperPathD,
    ) -> *mut ClipperPath64;
}
extern "C" {
    pub fn clipper_paths64_to_pathsd(
        mem: *mut ::std::os::raw::c_void,
        paths: *mut ClipperPaths64,
    ) -> *mut ClipperPathsD;
}
extern "C" {
    pub fn clipper_pathsd_to_paths64(
        mem: *mut ::std::os::raw::c_void,
        paths: *mut ClipperPathsD,
    ) -> *mut ClipperPaths64;
}
extern "C" {
    pub fn clipper_pathd_area(path: *mut ClipperPathD) -> f64;
}
extern "C" {
    pub fn clipper_path64_area(path: *mut ClipperPath64) -> f64;
}
extern "C" {
    pub fn clipper_pathsd_area(paths: *mut ClipperPathsD) -> f64;
}
extern "C" {
    pub fn clipper_paths64_area(paths: *mut ClipperPaths64) -> f64;
}
extern "C" {
    pub fn clipper_point_in_path64(
        path: *mut ClipperPath64,
        pt: ClipperPoint64,
    ) -> ClipperPointInPolygonResult;
}
extern "C" {
    pub fn clipper_point_in_pathd(
        path: *mut ClipperPathD,
        pt: ClipperPointD,
    ) -> ClipperPointInPolygonResult;
}
extern "C" {
    pub fn clipper_polytree64(
        mem: *mut ::std::os::raw::c_void,
        parent: *mut ClipperPolyTree64,
    ) -> *mut ClipperPolyTree64;
}
extern "C" {
    pub fn clipper_polytreed(
        mem: *mut ::std::os::raw::c_void,
        parent: *mut ClipperPolyTreeD,
    ) -> *mut ClipperPolyTreeD;
}
extern "C" {
    pub fn clipper_polytree64_parent(pt: *mut ClipperPolyTree64) -> *const ClipperPolyTree64;
}
extern "C" {
    pub fn clipper_polytree64_get_child(
        pt: *mut ClipperPolyTree64,
        idx: usize,
    ) -> *const ClipperPolyTree64;
}
extern "C" {
    pub fn clipper_polytree64_add_child(
        pt: *mut ClipperPolyTree64,
        path: *mut ClipperPath64,
    ) -> *mut ClipperPolyTree64;
}
extern "C" {
    pub fn clipper_polytree64_count(pt: *mut ClipperPolyTree64) -> usize;
}
extern "C" {
    pub fn clipper_polytree64_is_hole(pt: *mut ClipperPolyTree64) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn clipper_polytree64_polygon(
        mem: *mut ::std::os::raw::c_void,
        pt: *mut ClipperPolyTree64,
    ) -> *mut ClipperPath64;
}
extern "C" {
    pub fn clipper_polytree64_area(pt: *mut ClipperPolyTree64) -> f64;
}
extern "C" {
    pub fn clipper_polytree64_to_paths(
        mem: *mut ::std::os::raw::c_void,
        pt: *mut ClipperPolyTree64,
    ) -> *mut ClipperPaths64;
}
extern "C" {
    pub fn clipper_polytreed_parent(pt: *mut ClipperPolyTreeD) -> *const ClipperPolyTreeD;
}
extern "C" {
    pub fn clipper_polytreed_get_child(
        pt: *mut ClipperPolyTreeD,
        idx: usize,
    ) -> *const ClipperPolyTreeD;
}
extern "C" {
    pub fn clipper_polytreed_set_inv_scale(pt: *mut ClipperPolyTreeD, value: f64);
}
extern "C" {
    pub fn clipper_polytreed_inv_scale(pt: *mut ClipperPolyTreeD) -> f64;
}
extern "C" {
    pub fn clipper_polytreed_add_child(
        pt: *mut ClipperPolyTreeD,
        path: *mut ClipperPath64,
    ) -> *mut ClipperPolyTreeD;
}
extern "C" {
    pub fn clipper_polytreed_count(pt: *mut ClipperPolyTreeD) -> usize;
}
extern "C" {
    pub fn clipper_polytreed_is_hole(pt: *mut ClipperPolyTreeD) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn clipper_polytreed_polygon(
        mem: *mut ::std::os::raw::c_void,
        pt: *mut ClipperPolyTreeD,
    ) -> *mut ClipperPathD;
}
extern "C" {
    pub fn clipper_polytreed_area(pt: *mut ClipperPolyTreeD) -> f64;
}
extern "C" {
    pub fn clipper_polytreed_to_paths(
        mem: *mut ::std::os::raw::c_void,
        pt: *mut ClipperPolyTreeD,
    ) -> *mut ClipperPathsD;
}
extern "C" {
    pub fn clipper_clipper64(mem: *mut ::std::os::raw::c_void) -> *mut ClipperClipper64;
}
extern "C" {
    pub fn clipper_clipperd(
        mem: *mut ::std::os::raw::c_void,
        precision: ::std::os::raw::c_int,
    ) -> *mut ClipperClipperD;
}
extern "C" {
    pub fn clipper_clipper64_set_preserve_collinear(
        c: *mut ClipperClipper64,
        t: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn clipper_clipper64_set_reverse_solution(
        c: *mut ClipperClipper64,
        t: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn clipper_clipper64_get_preserve_collinear(
        c: *mut ClipperClipper64,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn clipper_clipper64_get_reverse_solution(
        c: *mut ClipperClipper64,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn clipper_clipper64_clear(c: *mut ClipperClipper64);
}
extern "C" {
    pub fn clipper_clipperd_set_preserve_collinear(
        c: *mut ClipperClipperD,
        t: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn clipper_clipperd_set_reverse_solution(c: *mut ClipperClipperD, t: ::std::os::raw::c_int);
}
extern "C" {
    pub fn clipper_clipperd_get_preserve_collinear(
        c: *mut ClipperClipperD,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn clipper_clipperd_get_reverse_solution(c: *mut ClipperClipperD) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn clipper_clipperd_clear(c: *mut ClipperClipperD);
}
extern "C" {
    pub fn clipper_clipper64_add_subject(c: *mut ClipperClipper64, subjects: *mut ClipperPaths64);
}
extern "C" {
    pub fn clipper_clipper64_add_open_subject(
        c: *mut ClipperClipper64,
        open_subjects: *mut ClipperPaths64,
    );
}
extern "C" {
    pub fn clipper_clipper64_add_clip(c: *mut ClipperClipper64, clips: *mut ClipperPaths64);
}
extern "C" {
    pub fn clipper_clipper64_execute(
        c64: *mut ClipperClipper64,
        ct: ClipperClipType,
        fr: ClipperFillRule,
        closed: *mut ClipperPaths64,
        open: *mut ClipperPaths64,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn clipper_clipper64_execute_tree_with_open(
        c64: *mut ClipperClipper64,
        ct: ClipperClipType,
        fr: ClipperFillRule,
        tree: *mut ClipperPolyTree64,
        open: *mut ClipperPaths64,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn clipper_clipperd_add_subject(c: *mut ClipperClipperD, subjects: *mut ClipperPathsD);
}
extern "C" {
    pub fn clipper_clipperd_add_open_subject(
        c: *mut ClipperClipperD,
        open_subjects: *mut ClipperPathsD,
    );
}
extern "C" {
    pub fn clipper_clipperd_add_clip(c: *mut ClipperClipperD, clips: *mut ClipperPathsD);
}
extern "C" {
    pub fn clipper_clipperd_execute(
        cD: *mut ClipperClipperD,
        ct: ClipperClipType,
        fr: ClipperFillRule,
        closed: *mut ClipperPathsD,
        open: *mut ClipperPathsD,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn clipper_clipperd_execute_tree_with_open(
        cD: *mut ClipperClipperD,
        ct: ClipperClipType,
        fr: ClipperFillRule,
        tree: *mut ClipperPolyTreeD,
        open: *mut ClipperPathsD,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn clipper_clipperoffset(
        mem: *mut ::std::os::raw::c_void,
        miter_limit: f64,
        arc_tolerance: f64,
        preserve_collinear: ::std::os::raw::c_int,
        reverse_solution: ::std::os::raw::c_int,
    ) -> *mut ClipperClipperOffset;
}
extern "C" {
    pub fn clipper_clipperoffset_set_miter_limit(c: *mut ClipperClipperOffset, l: f64);
}
extern "C" {
    pub fn clipper_clipperoffset_set_arc_tolerance(c: *mut ClipperClipperOffset, t: f64);
}
extern "C" {
    pub fn clipper_clipperoffset_set_preserve_collinear(
        c: *mut ClipperClipperOffset,
        t: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn clipper_clipperoffset_set_reverse_solution(
        c: *mut ClipperClipperOffset,
        t: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn clipper_clipperoffset_get_miter_limit(c: *mut ClipperClipperOffset) -> f64;
}
extern "C" {
    pub fn clipper_clipperoffset_get_arc_tolerance(c: *mut ClipperClipperOffset) -> f64;
}
extern "C" {
    pub fn clipper_clipperoffset_get_preserve_collinear(
        c: *mut ClipperClipperOffset,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn clipper_clipperoffset_get_reverse_solution(
        c: *mut ClipperClipperOffset,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn clipper_clipperoffset_error_code(c: *mut ClipperClipperOffset) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn clipper_clipperoffset_clear(c: *mut ClipperClipperOffset);
}
extern "C" {
    pub fn clipper_clipperoffset_add_path64(
        c: *mut ClipperClipperOffset,
        p: *mut ClipperPath64,
        jt: ClipperJoinType,
        et: ClipperEndType,
    );
}
extern "C" {
    pub fn clipper_clipperoffset_add_paths64(
        c: *mut ClipperClipperOffset,
        p: *mut ClipperPaths64,
        jt: ClipperJoinType,
        et: ClipperEndType,
    );
}
extern "C" {
    pub fn clipper_clipperoffset_execute(
        mem: *mut ::std::os::raw::c_void,
        c: *mut ClipperClipperOffset,
        delta: f64,
    ) -> *mut ClipperPaths64;
}
extern "C" {
    pub fn clipper_path64_size() -> usize;
}
extern "C" {
    pub fn clipper_pathd_size() -> usize;
}
extern "C" {
    pub fn clipper_paths64_size() -> usize;
}
extern "C" {
    pub fn clipper_pathsd_size() -> usize;
}
extern "C" {
    pub fn clipper_polytree64_size() -> usize;
}
extern "C" {
    pub fn clipper_polytreed_size() -> usize;
}
extern "C" {
    pub fn clipper_clipper64_size() -> usize;
}
extern "C" {
    pub fn clipper_clipperd_size() -> usize;
}
extern "C" {
    pub fn clipper_clipperoffset_size() -> usize;
}
extern "C" {
    pub fn clipper_delete_path64(p: *mut ClipperPath64);
}
extern "C" {
    pub fn clipper_delete_pathd(p: *mut ClipperPathD);
}
extern "C" {
    pub fn clipper_delete_paths64(p: *mut ClipperPaths64);
}
extern "C" {
    pub fn clipper_delete_pathsd(p: *mut ClipperPathsD);
}
extern "C" {
    pub fn clipper_delete_polytree64(p: *mut ClipperPolyTree64);
}
extern "C" {
    pub fn clipper_delete_polytreed(p: *mut ClipperPolyTreeD);
}
extern "C" {
    pub fn clipper_delete_clipper64(p: *mut ClipperClipper64);
}
extern "C" {
    pub fn clipper_delete_clipperd(p: *mut ClipperClipperD);
}
extern "C" {
    pub fn clipper_delete_clipperoffset(p: *mut ClipperClipperOffset);
}
