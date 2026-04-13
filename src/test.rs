use std::os::raw::{c_int, c_void};
use std::ptr;

use crate::*;

extern "C" {
    fn clipper_rect64_size() -> usize;
    fn clipper_rectd_size() -> usize;
    fn clipper_rect64(mem: *mut c_void, left: i64, top: i64, right: i64, bottom: i64)
        -> *mut ClipperRect64;
    fn clipper_rectd(
        mem: *mut c_void,
        left: f64,
        top: f64,
        right: f64,
        bottom: f64,
    ) -> *mut ClipperRectD;
    fn clipper_rect64_width(r: *mut ClipperRect64) -> i64;
    fn clipper_rect64_height(r: *mut ClipperRect64) -> i64;
    fn clipper_rect64_as_path(mem: *mut c_void, r: *mut ClipperRect64) -> *mut ClipperPath64;
    fn clipper_rect64_scale(
        mem: *mut c_void,
        r: *mut ClipperRect64,
        scale: f64,
    ) -> *mut ClipperRect64;
    fn clipper_rectd_width(r: *mut ClipperRectD) -> f64;
    fn clipper_rectd_height(r: *mut ClipperRectD) -> f64;
    fn clipper_rectd_scale(mem: *mut c_void, r: *mut ClipperRectD, scale: f64) -> *mut ClipperRectD;
    fn clipper_delete_rect64(p: *mut ClipperRect64);
    fn clipper_delete_rectd(p: *mut ClipperRectD);
    fn clipper_svgwriter_size() -> usize;
    fn clipper_svgreader_size() -> usize;
    fn clipper_svgwriter(mem: *mut c_void, precision: c_int) -> *mut ClipperSvgWriter;
    fn clipper_svgreader(mem: *mut c_void) -> *mut ClipperSvgReader;
    fn clipper_svgwriter_add_paths64(
        w: *mut ClipperSvgWriter,
        paths: *mut ClipperPaths64,
        is_open: c_int,
        fillrule: ClipperFillRule,
        brush_color: u32,
        pen_color: u32,
        pen_width: f64,
        show_coords: c_int,
    );
    fn clipper_svgwriter_save_to_file(
        w: *mut ClipperSvgWriter,
        filename: *const i8,
        max_width: c_int,
        max_height: c_int,
        margin: c_int,
    ) -> c_int;
    fn clipper_svgwriter_clear(w: *mut ClipperSvgWriter);
    fn clipper_svgreader_load_from_file(r: *mut ClipperSvgReader, filename: *const i8);
    fn clipper_svgreader_get_pathsd(mem: *mut c_void, r: *mut ClipperSvgReader) -> *mut ClipperPathsD;
    fn clipper_svgreader_clear(r: *mut ClipperSvgReader);
    fn clipper_delete_svgwriter(p: *mut ClipperSvgWriter);
    fn clipper_delete_svgreader(p: *mut ClipperSvgReader);
    fn clipper_path64_to_points(mem: *mut c_void, path: *mut ClipperPath64) -> *mut ClipperPoint64;
    fn clipper_pathd_to_points(mem: *mut c_void, path: *mut ClipperPathD) -> *mut ClipperPointD;
    fn clipper_path64_minkowski_sum(
        mem: *mut c_void,
        pattern: *mut ClipperPath64,
        path: *mut ClipperPath64,
        is_closed: c_int,
    ) -> *mut ClipperPaths64;
    fn clipper_paths64_minkowski_sum(
        mem: *mut c_void,
        pattern: *mut ClipperPath64,
        paths: *mut ClipperPaths64,
        is_closed: c_int,
        fillrule: ClipperFillRule,
    ) -> *mut ClipperPaths64;
    fn clipper_path64_ramer_douglas_peucker(
        mem: *mut c_void,
        path: *mut ClipperPath64,
        epsilon: f64,
    ) -> *mut ClipperPath64;
    fn clipper_path64_strip_near_equal(
        mem: *mut c_void,
        path: *mut ClipperPath64,
        max_dist_sqrd: f64,
        is_closed_path: c_int,
    ) -> *mut ClipperPath64;
    fn clipper_path64_trim_collinear(
        mem: *mut c_void,
        path: *mut ClipperPath64,
        is_open_path: c_int,
    ) -> *mut ClipperPath64;
    fn clipper_path64_translate(
        mem: *mut c_void,
        path: *mut ClipperPath64,
        dx: i64,
        dy: i64,
    ) -> *mut ClipperPath64;
    fn clipper_path64_scale(
        mem: *mut c_void,
        path: *mut ClipperPath64,
        sx: f64,
        sy: f64,
        error_code: *mut c_int,
    ) -> *mut ClipperPath64;
    fn clipper_path64_bounds(mem: *mut c_void, path: *mut ClipperPath64) -> *mut ClipperRect64;
    fn clipper_scale_pathd_to_path64(
        mem: *mut c_void,
        path: *mut ClipperPathD,
        sx: f64,
        sy: f64,
        error_code: *mut c_int,
    ) -> *mut ClipperPath64;
    fn clipper_scale_path64_to_pathd(
        mem: *mut c_void,
        path: *mut ClipperPath64,
        sx: f64,
        sy: f64,
        error_code: *mut c_int,
    ) -> *mut ClipperPathD;
}

#[repr(C)]
struct ClipperSvgWriter {
    _unused: [u8; 0],
}
#[repr(C)]
struct ClipperSvgReader {
    _unused: [u8; 0],
}
#[repr(C)]
struct ClipperRect64 {
    _unused: [u8; 0],
}
#[repr(C)]
struct ClipperRectD {
    _unused: [u8; 0],
}

unsafe fn alloc(size: usize) -> *mut c_void {
    clipper_allocate(size)
}

const LEAK_ITERATIONS: usize = 1_000;

#[test]
fn test_polytree64_memory() {
    let mut outer = vec![
        ClipperPoint64 { x: 0, y: 0 },
        ClipperPoint64 { x: 100, y: 0 },
        ClipperPoint64 { x: 100, y: 100 },
        ClipperPoint64 { x: 0, y: 100 },
    ];
    let mut inner = vec![
        ClipperPoint64 { x: 20, y: 20 },
        ClipperPoint64 { x: 20, y: 80 },
        ClipperPoint64 { x: 80, y: 80 },
        ClipperPoint64 { x: 80, y: 20 },
    ];

    for _ in 0..LEAK_ITERATIONS {
        unsafe {
            let outer_path_mem = alloc(clipper_path64_size());
            let outer_path =
                clipper_path64_of_points(outer_path_mem, outer.as_mut_ptr(), outer.len());
            let subjects_mem = alloc(clipper_paths64_size());
            let subjects = clipper_paths64_of_paths(subjects_mem, [outer_path].as_mut_ptr(), 1);

            let inner_path_mem = alloc(clipper_path64_size());
            let inner_path =
                clipper_path64_of_points(inner_path_mem, inner.as_mut_ptr(), inner.len());
            let clips_mem = alloc(clipper_paths64_size());
            let clips = clipper_paths64_of_paths(clips_mem, [inner_path].as_mut_ptr(), 1);

            let clipper_mem = alloc(clipper_clipper64_size());
            let clipper = clipper_clipper64(clipper_mem);
            clipper_clipper64_add_subject(clipper, subjects);
            clipper_clipper64_add_clip(clipper, clips);

            let tree_mem = alloc(clipper_polytree64_size());
            let tree = clipper_polytree64(tree_mem, ptr::null_mut());
            let open_mem = alloc(clipper_paths64_size());
            let open = clipper_paths64(open_mem);

            let success = clipper_clipper64_execute_tree_with_open(
                clipper,
                ClipperClipType_DIFFERENCE,
                ClipperFillRule_EVEN_ODD,
                tree,
                open,
            );
            assert_eq!(success, 1);

            let count = clipper_polytree64_count(tree);
            assert!(count > 0);

            let child = clipper_polytree64_get_child(tree, 0);
            assert!(!child.is_null());

            let poly_mem = alloc(clipper_path64_size());
            let poly = clipper_polytree64_polygon(poly_mem, child as *mut _);
            let poly_len = clipper_path64_length(poly);
            assert!(poly_len > 0);
            clipper_delete_path64(poly);

            let paths_mem = alloc(clipper_paths64_size());
            let paths = clipper_polytree64_to_paths(paths_mem, tree);
            let paths_len = clipper_paths64_length(paths);
            assert!(paths_len > 0);
            clipper_delete_paths64(paths);

            let area = clipper_polytree64_area(tree);
            assert!(area != 0.0);

            clipper_delete_paths64(open);
            clipper_delete_polytree64(tree);
            clipper_delete_clipper64(clipper);
            clipper_delete_path64(outer_path);
            clipper_delete_paths64(subjects);
            clipper_delete_path64(inner_path);
            clipper_delete_paths64(clips);
        }
    }
}


#[test]
fn test_clipper_offset_memory() {
    let mut square = vec![
        ClipperPoint64 { x: 0, y: 0 },
        ClipperPoint64 { x: 100, y: 0 },
        ClipperPoint64 { x: 100, y: 100 },
        ClipperPoint64 { x: 0, y: 100 },
    ];

    for _ in 0..LEAK_ITERATIONS {
        unsafe {
            let path_mem = alloc(clipper_path64_size());
            let path = clipper_path64_of_points(path_mem, square.as_mut_ptr(), square.len());

            let paths_mem = alloc(clipper_paths64_size());
            let paths = clipper_paths64_of_paths(paths_mem, [path].as_mut_ptr(), 1);

            let co_mem = alloc(clipper_clipperoffset_size());
            let co = clipper_clipperoffset(co_mem, 2.0, 0.25, 0, 0);

            clipper_clipperoffset_add_path64(
                co,
                path,
                ClipperJoinType_ROUND_JOIN,
                ClipperEndType_POLYGON_END,
            );

            let result_mem = alloc(clipper_paths64_size());
            let result = clipper_clipperoffset_execute(result_mem, co, 10.0);
            let len = clipper_paths64_length(result);
            assert!(len > 0);

            let result2_mem = alloc(clipper_paths64_size());
            let result2 = clipper_clipperoffset_execute(result2_mem, co, -5.0);
            let len2 = clipper_paths64_length(result2);
            assert!(len2 > 0);

            clipper_delete_paths64(result);
            clipper_delete_paths64(result2);
            clipper_delete_clipperoffset(co);
            clipper_delete_path64(path);
            clipper_delete_paths64(paths);
        }
    }
}


#[test]
fn test_rect64_scale_correctness() {
    unsafe {
        let mem = alloc(clipper_rect64_size());
        let rect = clipper_rect64(mem, 10, 20, 30, 40);

        assert_eq!(clipper_rect64_width(rect), 20);
        assert_eq!(clipper_rect64_height(rect), 20);

        let scaled_mem = alloc(clipper_rect64_size());
        let scaled = clipper_rect64_scale(scaled_mem, rect, 2.0);

        assert_eq!(clipper_rect64_width(scaled), 40);
        assert_eq!(clipper_rect64_height(scaled), 40);

        assert_eq!(clipper_rect64_width(rect), 20);
        assert_eq!(clipper_rect64_height(rect), 20);

        clipper_delete_rect64(scaled);
        clipper_delete_rect64(rect);
    }
}

#[test]
fn test_rectd_scale_correctness() {
    unsafe {
        let mem = alloc(clipper_rectd_size());
        let rect = clipper_rectd(mem, 10.0, 20.0, 30.0, 40.0);

        assert!((clipper_rectd_width(rect) - 20.0).abs() < 1e-10);
        assert!((clipper_rectd_height(rect) - 20.0).abs() < 1e-10);

        let scaled_mem = alloc(clipper_rectd_size());
        let scaled = clipper_rectd_scale(scaled_mem, rect, 2.0);

        assert!((clipper_rectd_width(scaled) - 40.0).abs() < 1e-10);
        assert!((clipper_rectd_height(scaled) - 40.0).abs() < 1e-10);

        assert!((clipper_rectd_width(rect) - 20.0).abs() < 1e-10);

        clipper_delete_rectd(scaled);
        clipper_delete_rectd(rect);
    }
}

#[test]
fn test_rect64_memory() {
    for _ in 0..LEAK_ITERATIONS {
        unsafe {
            let mem = alloc(clipper_rect64_size());
            let rect = clipper_rect64(mem, 0, 0, 100, 100);

            let path_mem = alloc(clipper_path64_size());
            let path = clipper_rect64_as_path(path_mem, rect);
            assert_eq!(clipper_path64_length(path), 4);

            let scaled_mem = alloc(clipper_rect64_size());
            let scaled = clipper_rect64_scale(scaled_mem, rect, 3.0);

            clipper_delete_path64(path);
            clipper_delete_rect64(scaled);
            clipper_delete_rect64(rect);
        }
    }
}


#[test]
fn test_pathd_memory() {
    let mut points: Vec<ClipperPointD> = (0..100)
        .map(|i| ClipperPointD {
            x: i as f64,
            y: (i * 2) as f64,
        })
        .collect();

    for _ in 0..LEAK_ITERATIONS {
        unsafe {
            let mem = alloc(clipper_pathd_size());
            let path = clipper_pathd_of_points(mem, points.as_mut_ptr(), points.len());

            assert_eq!(clipper_pathd_length(path), 100);

            let pt = clipper_pathd_get_point(path, 5);
            assert!((pt.x - 5.0).abs() < 1e-10);
            assert!((pt.y - 10.0).abs() < 1e-10);

            clipper_delete_pathd(path);
        }
    }
}

#[test]
fn test_clipperd_execute_memory() {
    let mut triangle = vec![
        ClipperPointD { x: 0.0, y: 0.0 },
        ClipperPointD { x: 10.0, y: 0.0 },
        ClipperPointD { x: 5.0, y: 10.0 },
    ];
    let mut square = vec![
        ClipperPointD { x: 0.0, y: 0.0 },
        ClipperPointD { x: 4.0, y: 0.0 },
        ClipperPointD { x: 4.0, y: 4.0 },
        ClipperPointD { x: 0.0, y: 4.0 },
    ];

    for _ in 0..LEAK_ITERATIONS {
        unsafe {
            let subject_mem = alloc(clipper_pathd_size());
            let subject = clipper_pathd_of_points(subject_mem, triangle.as_mut_ptr(), triangle.len());
            let subjects_mem = alloc(clipper_pathsd_size());
            let subjects = clipper_pathsd_of_paths(subjects_mem, [subject].as_mut_ptr(), 1);

            let clip_mem = alloc(clipper_pathd_size());
            let clip = clipper_pathd_of_points(clip_mem, square.as_mut_ptr(), square.len());
            let clips_mem = alloc(clipper_pathsd_size());
            let clips = clipper_pathsd_of_paths(clips_mem, [clip].as_mut_ptr(), 1);

            let clipper_mem = alloc(clipper_clipperd_size());
            let clipper = clipper_clipperd(clipper_mem, 2);
            clipper_clipperd_add_subject(clipper, subjects);
            clipper_clipperd_add_clip(clipper, clips);

            let closed_mem = alloc(clipper_pathsd_size());
            let closed = clipper_pathsd(closed_mem);
            let open_mem = alloc(clipper_pathsd_size());
            let open = clipper_pathsd(open_mem);

            let success = clipper_clipperd_execute(
                clipper,
                ClipperClipType_DIFFERENCE,
                ClipperFillRule_EVEN_ODD,
                closed,
                open,
            );
            assert_eq!(success, 1);
            assert!(clipper_pathsd_length(closed) > 0);

            clipper_delete_pathsd(closed);
            clipper_delete_pathsd(open);
            clipper_delete_clipperd(clipper);
            clipper_delete_pathd(subject);
            clipper_delete_pathsd(subjects);
            clipper_delete_pathd(clip);
            clipper_delete_pathsd(clips);
        }
    }
}


#[test]
fn test_path_conversion_memory() {
    let mut points: Vec<ClipperPoint64> = (0..500)
        .map(|i| ClipperPoint64 { x: i, y: i * 2 })
        .collect();

    for _ in 0..LEAK_ITERATIONS {
        unsafe {
            let mem = alloc(clipper_path64_size());
            let path64 = clipper_path64_of_points(mem, points.as_mut_ptr(), points.len());

            // path64 -> pathd
            let pathd_mem = alloc(clipper_pathd_size());
            let pathd = clipper_path64_to_pathd(pathd_mem, path64);
            assert_eq!(clipper_pathd_length(pathd), 500);

            // pathd -> path64
            let back_mem = alloc(clipper_path64_size());
            let back = clipper_pathd_to_path64(back_mem, pathd);
            assert_eq!(clipper_path64_length(back), 500);

            clipper_delete_path64(back);
            clipper_delete_pathd(pathd);
            clipper_delete_path64(path64);
        }
    }
}

#[test]
fn test_path_to_points_memory() {
    let mut points: Vec<ClipperPoint64> = (0..500)
        .map(|i| ClipperPoint64 { x: i, y: i * 3 })
        .collect();

    for _ in 0..LEAK_ITERATIONS {
        unsafe {
            let mem = alloc(clipper_path64_size());
            let path = clipper_path64_of_points(mem, points.as_mut_ptr(), points.len());

            let len = clipper_path64_length(path);
            let pts_mem = libc::malloc(len * std::mem::size_of::<ClipperPoint64>());
            let pts = clipper_path64_to_points(pts_mem, path);

            assert_eq!((*pts).x, 0);
            assert_eq!((*pts.add(1)).x, 1);
            assert_eq!((*pts.add(499)).x, 499);
            assert_eq!((*pts.add(499)).y, 499 * 3);

            libc::free(pts_mem);
            clipper_delete_path64(path);
        }
    }
}

#[test]
fn test_pathd_to_points_memory() {
    let mut points: Vec<ClipperPointD> = (0..500)
        .map(|i| ClipperPointD {
            x: i as f64,
            y: (i * 3) as f64,
        })
        .collect();

    for _ in 0..LEAK_ITERATIONS {
        unsafe {
            let mem = alloc(clipper_pathd_size());
            let path = clipper_pathd_of_points(mem, points.as_mut_ptr(), points.len());

            let len = clipper_pathd_length(path);
            let pts_mem = libc::malloc(len * std::mem::size_of::<ClipperPointD>());
            let pts = clipper_pathd_to_points(pts_mem, path);

            assert!(((*pts).x - 0.0).abs() < 1e-10);
            assert!(((*pts.add(499)).x - 499.0).abs() < 1e-10);
            assert!(((*pts.add(499)).y - 1497.0).abs() < 1e-10);

            libc::free(pts_mem);
            clipper_delete_pathd(path);
        }
    }
}

#[test]
fn test_scale_path_conversion_memory() {
    let mut points: Vec<ClipperPoint64> = (0..100)
        .map(|i| ClipperPoint64 { x: i * 10, y: i * 20 })
        .collect();

    for _ in 0..LEAK_ITERATIONS {
        unsafe {
            let mem = alloc(clipper_path64_size());
            let path64 = clipper_path64_of_points(mem, points.as_mut_ptr(), points.len());

            let mut err: c_int = 0;
            let pathd_mem = alloc(clipper_pathd_size());
            let pathd = clipper_scale_path64_to_pathd(pathd_mem, path64, 0.5, 0.5, &mut err);
            assert_eq!(err, 0);
            assert_eq!(clipper_pathd_length(pathd), 100);

            let back_mem = alloc(clipper_path64_size());
            let back = clipper_scale_pathd_to_path64(back_mem, pathd, 2.0, 2.0, &mut err);
            assert_eq!(err, 0);
            assert_eq!(clipper_path64_length(back), 100);

            clipper_delete_path64(back);
            clipper_delete_pathd(pathd);
            clipper_delete_path64(path64);
        }
    }
}


#[test]
fn test_minkowski_sum_memory() {
    let mut pattern = vec![
        ClipperPoint64 { x: -5, y: -5 },
        ClipperPoint64 { x: 5, y: -5 },
        ClipperPoint64 { x: 5, y: 5 },
        ClipperPoint64 { x: -5, y: 5 },
    ];
    let mut path = vec![
        ClipperPoint64 { x: 0, y: 0 },
        ClipperPoint64 { x: 100, y: 0 },
        ClipperPoint64 { x: 100, y: 100 },
        ClipperPoint64 { x: 0, y: 100 },
    ];

    for _ in 0..LEAK_ITERATIONS {
        unsafe {
            let pat_mem = alloc(clipper_path64_size());
            let pat = clipper_path64_of_points(pat_mem, pattern.as_mut_ptr(), pattern.len());
            let path_mem = alloc(clipper_path64_size());
            let p = clipper_path64_of_points(path_mem, path.as_mut_ptr(), path.len());

            // Single-path Minkowski sum
            let result_mem = alloc(clipper_paths64_size());
            let result = clipper_path64_minkowski_sum(result_mem, pat, p, 1);
            assert!(clipper_paths64_length(result) > 0);

            // Multi-path Minkowski sum
            let paths_mem = alloc(clipper_paths64_size());
            let paths = clipper_paths64_of_paths(paths_mem, [p].as_mut_ptr(), 1);
            let mresult_mem = alloc(clipper_paths64_size());
            let mresult = clipper_paths64_minkowski_sum(
                mresult_mem,
                pat,
                paths,
                1,
                ClipperFillRule_EVEN_ODD,
            );
            assert!(clipper_paths64_length(mresult) > 0);

            clipper_delete_paths64(mresult);
            clipper_delete_paths64(paths);
            clipper_delete_paths64(result);
            clipper_delete_path64(p);
            clipper_delete_path64(pat);
        }
    }
}


#[test]
fn test_svgwriter_memory() {
    let mut points = vec![
        ClipperPoint64 { x: 0, y: 0 },
        ClipperPoint64 { x: 100, y: 0 },
        ClipperPoint64 { x: 100, y: 100 },
        ClipperPoint64 { x: 0, y: 100 },
    ];

    for _ in 0..LEAK_ITERATIONS {
        unsafe {
            let path_mem = alloc(clipper_path64_size());
            let path = clipper_path64_of_points(path_mem, points.as_mut_ptr(), points.len());
            let paths_mem = alloc(clipper_paths64_size());
            let paths = clipper_paths64_of_paths(paths_mem, [path].as_mut_ptr(), 1);

            let writer_mem = alloc(clipper_svgwriter_size());
            let writer = clipper_svgwriter(writer_mem, 2);

            clipper_svgwriter_add_paths64(
                writer,
                paths,
                0,
                ClipperFillRule_EVEN_ODD,
                0xFF0000FF, // brush
                0xFF000000, // pen
                1.0,
                0,
            );

            clipper_svgwriter_clear(writer);
            clipper_delete_svgwriter(writer);
            clipper_delete_path64(path);
            clipper_delete_paths64(paths);
        }
    }
}

#[test]
fn test_svgreader_memory() {
    for _ in 0..LEAK_ITERATIONS {
        unsafe {
            let reader_mem = alloc(clipper_svgreader_size());
            let reader = clipper_svgreader(reader_mem);

            let paths_mem = alloc(clipper_pathsd_size());
            let paths = clipper_svgreader_get_pathsd(paths_mem, reader);
            assert_eq!(clipper_pathsd_length(paths), 0);

            clipper_delete_pathsd(paths);
            clipper_svgreader_clear(reader);
            clipper_delete_svgreader(reader);
        }
    }
}


#[test]
fn test_path_simplify_memory() {
    let mut points: Vec<ClipperPoint64> = (0..200)
        .map(|i| ClipperPoint64 {
            x: i * 10,
            y: (i % 7) * 3,
        })
        .collect();

    for _ in 0..LEAK_ITERATIONS {
        unsafe {
            let mem = alloc(clipper_path64_size());
            let path = clipper_path64_of_points(mem, points.as_mut_ptr(), points.len());

            let simp_mem = alloc(clipper_path64_size());
            let simplified = clipper_path64_simplify(simp_mem, path, 2.0, 0);
            assert!(clipper_path64_length(simplified) > 0);
            assert!(clipper_path64_length(simplified) <= 200);

            clipper_delete_path64(simplified);
            clipper_delete_path64(path);
        }
    }
}

#[test]
fn test_path_ramer_douglas_peucker_memory() {
    let mut points: Vec<ClipperPoint64> = (0..200)
        .map(|i| ClipperPoint64 {
            x: i * 10,
            y: (i % 5) * 2,
        })
        .collect();

    for _ in 0..LEAK_ITERATIONS {
        unsafe {
            let mem = alloc(clipper_path64_size());
            let path = clipper_path64_of_points(mem, points.as_mut_ptr(), points.len());

            let rdp_mem = alloc(clipper_path64_size());
            let rdp = clipper_path64_ramer_douglas_peucker(rdp_mem, path, 5.0);
            assert!(clipper_path64_length(rdp) > 0);

            clipper_delete_path64(rdp);
            clipper_delete_path64(path);
        }
    }
}

#[test]
fn test_path_strip_near_equal_memory() {
    let mut points = vec![
        ClipperPoint64 { x: 0, y: 0 },
        ClipperPoint64 { x: 1, y: 0 },
        ClipperPoint64 { x: 100, y: 0 },
        ClipperPoint64 { x: 101, y: 0 },
        ClipperPoint64 { x: 100, y: 100 },
    ];

    for _ in 0..LEAK_ITERATIONS {
        unsafe {
            let mem = alloc(clipper_path64_size());
            let path = clipper_path64_of_points(mem, points.as_mut_ptr(), points.len());

            let strip_mem = alloc(clipper_path64_size());
            let stripped = clipper_path64_strip_near_equal(strip_mem, path, 4.0, 0);
            assert!(clipper_path64_length(stripped) > 0);

            clipper_delete_path64(stripped);
            clipper_delete_path64(path);
        }
    }
}

#[test]
fn test_path_trim_collinear_memory() {
    let mut points = vec![
        ClipperPoint64 { x: 0, y: 0 },
        ClipperPoint64 { x: 50, y: 0 },
        ClipperPoint64 { x: 100, y: 0 },
        ClipperPoint64 { x: 100, y: 100 },
        ClipperPoint64 { x: 0, y: 100 },
    ];

    for _ in 0..LEAK_ITERATIONS {
        unsafe {
            let mem = alloc(clipper_path64_size());
            let path = clipper_path64_of_points(mem, points.as_mut_ptr(), points.len());

            let trim_mem = alloc(clipper_path64_size());
            let trimmed = clipper_path64_trim_collinear(trim_mem, path, 0);
            assert!(clipper_path64_length(trimmed) > 0);
            assert!(clipper_path64_length(trimmed) < clipper_path64_length(path));

            clipper_delete_path64(trimmed);
            clipper_delete_path64(path);
        }
    }
}

#[test]
fn test_path_translate_memory() {
    let mut points = vec![
        ClipperPoint64 { x: 0, y: 0 },
        ClipperPoint64 { x: 100, y: 0 },
        ClipperPoint64 { x: 100, y: 100 },
    ];

    for _ in 0..LEAK_ITERATIONS {
        unsafe {
            let mem = alloc(clipper_path64_size());
            let path = clipper_path64_of_points(mem, points.as_mut_ptr(), points.len());

            let trans_mem = alloc(clipper_path64_size());
            let translated = clipper_path64_translate(trans_mem, path, 50, 50);
            assert_eq!(clipper_path64_length(translated), 3);

            let pt = clipper_path64_get_point(translated, 0);
            assert_eq!(pt.x, 50);
            assert_eq!(pt.y, 50);

            clipper_delete_path64(translated);
            clipper_delete_path64(path);
        }
    }
}

#[test]
fn test_path_scale_memory() {
    let mut points = vec![
        ClipperPoint64 { x: 0, y: 0 },
        ClipperPoint64 { x: 100, y: 0 },
        ClipperPoint64 { x: 100, y: 100 },
    ];

    for _ in 0..LEAK_ITERATIONS {
        unsafe {
            let mem = alloc(clipper_path64_size());
            let path = clipper_path64_of_points(mem, points.as_mut_ptr(), points.len());

            let mut err: c_int = 0;
            let scaled_mem = alloc(clipper_path64_size());
            let scaled = clipper_path64_scale(scaled_mem, path, 2.0, 2.0, &mut err);
            assert_eq!(err, 0);
            assert_eq!(clipper_path64_length(scaled), 3);

            let pt = clipper_path64_get_point(scaled, 1);
            assert_eq!(pt.x, 200);

            clipper_delete_path64(scaled);
            clipper_delete_path64(path);
        }
    }
}

#[test]
fn test_inflate_memory() {
    let mut points = vec![
        ClipperPoint64 { x: 0, y: 0 },
        ClipperPoint64 { x: 100, y: 0 },
        ClipperPoint64 { x: 100, y: 100 },
        ClipperPoint64 { x: 0, y: 100 },
    ];

    for _ in 0..LEAK_ITERATIONS {
        unsafe {
            let path_mem = alloc(clipper_path64_size());
            let path = clipper_path64_of_points(path_mem, points.as_mut_ptr(), points.len());
            let paths_mem = alloc(clipper_paths64_size());
            let paths = clipper_paths64_of_paths(paths_mem, [path].as_mut_ptr(), 1);

            let result_mem = alloc(clipper_paths64_size());
            let result = clipper_paths64_inflate(
                result_mem,
                paths,
                10.0,
                ClipperJoinType_ROUND_JOIN,
                ClipperEndType_POLYGON_END,
                2.0,
            );
            assert!(clipper_paths64_length(result) > 0);

            clipper_delete_paths64(result);
            clipper_delete_path64(path);
            clipper_delete_paths64(paths);
        }
    }
}


#[test]
fn test_difference_boolean_operation() {
    let mut triangle = vec![
        ClipperPoint64 { x: 0, y: 0 },
        ClipperPoint64 { x: 10, y: 0 },
        ClipperPoint64 { x: 5, y: 10 },
    ];

    let mut square = vec![
        ClipperPoint64 { x: 0, y: 0 },
        ClipperPoint64 { x: 4, y: 0 },
        ClipperPoint64 { x: 4, y: 4 },
        ClipperPoint64 { x: 0, y: 4 },
    ];

    for _ in 0..1_000 {
        let output = unsafe {
            let subject_mem = alloc(clipper_path64_size());
            let subject_ptr =
                clipper_path64_of_points(subject_mem, triangle.as_mut_ptr(), triangle.len());

            let subjects_mem = alloc(clipper_paths64_size());
            let subjects_ptr =
                clipper_paths64_of_paths(subjects_mem, [subject_ptr].as_mut_ptr(), 1);

            let clip_mem = alloc(clipper_path64_size());
            let clip_ptr = clipper_path64_of_points(clip_mem, square.as_mut_ptr(), square.len());

            let clips_mem = alloc(clipper_paths64_size());
            let clips_ptr = clipper_paths64_of_paths(clips_mem, [clip_ptr].as_mut_ptr(), 1);

            let clipper_mem = alloc(clipper_clipper64_size());
            let clipper_ptr = clipper_clipper64(clipper_mem);

            clipper_clipper64_add_subject(clipper_ptr, subjects_ptr);
            clipper_clipper64_add_clip(clipper_ptr, clips_ptr);

            clipper_delete_path64(subject_ptr);
            clipper_delete_paths64(subjects_ptr);
            clipper_delete_path64(clip_ptr);
            clipper_delete_paths64(clips_ptr);

            let closed_path_mem = alloc(clipper_paths64_size());
            let closed_path_ptr = clipper_paths64(closed_path_mem);

            let open_path_mem = alloc(clipper_paths64_size());
            let open_path_ptr = clipper_paths64(open_path_mem);

            let success = clipper_clipper64_execute(
                clipper_ptr,
                ClipperClipType_DIFFERENCE,
                ClipperFillRule_EVEN_ODD,
                closed_path_ptr,
                open_path_ptr,
            );

            assert_eq!(success, 1);

            let len: i32 = clipper_paths64_length(closed_path_ptr).try_into().unwrap();

            let output = (0..len)
                .map(|i| {
                    let point_len: i32 = clipper_paths64_path_length(closed_path_ptr, i)
                        .try_into()
                        .unwrap();
                    (0..point_len)
                        .map(|j| clipper_paths64_get_point(closed_path_ptr, i, j))
                        .map(|p| (p.x, p.y))
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();

            clipper_delete_paths64(closed_path_ptr);
            clipper_delete_paths64(open_path_ptr);

            clipper_delete_clipper64(clipper_ptr);

            output
        };

        let expected_output = vec![vec![(5, 10), (2, 4), (4, 4), (4, 0), (10, 0)]];
        assert_eq!(output, expected_output);
    }
}
