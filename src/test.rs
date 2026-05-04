use std::os::raw::{c_int, c_void};
use std::ptr;

use crate::*;

unsafe extern "C" {
    fn clipper_rect64_size() -> usize;
    fn clipper_rectd_size() -> usize;
    fn clipper_rect64(
        mem: *mut c_void,
        left: i64,
        top: i64,
        right: i64,
        bottom: i64,
    ) -> *mut ClipperRect64;
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
    fn clipper_rectd_scale(mem: *mut c_void, r: *mut ClipperRectD, scale: f64)
        -> *mut ClipperRectD;
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
    fn clipper_svgreader_get_pathsd(
        mem: *mut c_void,
        r: *mut ClipperSvgReader,
    ) -> *mut ClipperPathsD;
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
    unsafe { clipper_allocate(size) }
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
            let subject =
                clipper_pathd_of_points(subject_mem, triangle.as_mut_ptr(), triangle.len());
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
        .map(|i| ClipperPoint64 {
            x: i * 10,
            y: i * 20,
        })
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
            let mresult =
                clipper_paths64_minkowski_sum(mresult_mem, pat, paths, 1, ClipperFillRule_EVEN_ODD);
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

fn write_svg_fixture(filename: &str, path_data: &str) -> std::path::PathBuf {
    use std::io::Write;
    let directory = std::path::Path::new(".tmp");
    std::fs::create_dir_all(directory).unwrap();
    let file_path = directory.join(filename);
    let mut file = std::fs::File::create(&file_path).unwrap();
    write!(
        file,
        r#"<?xml version="1.0"?><svg xmlns="http://www.w3.org/2000/svg"><path d="{}"/></svg>"#,
        path_data,
    )
    .unwrap();
    file_path
}

#[test]
fn test_svgreader_load_from_file_round_trip() {
    let fixture = write_svg_fixture("svgreader_round_trip.svg", "M0 0L10 0L10 10L0 10Z");
    let filename = std::ffi::CString::new(fixture.to_str().unwrap()).unwrap();

    unsafe {
        let reader_mem = alloc(clipper_svgreader_size());
        let reader = clipper_svgreader(reader_mem);
        clipper_svgreader_load_from_file(reader, filename.as_ptr());

        let paths_mem = alloc(clipper_pathsd_size());
        let paths = clipper_svgreader_get_pathsd(paths_mem, reader);
        assert_eq!(clipper_pathsd_length(paths), 1);

        let path_mem = alloc(clipper_pathd_size());
        let path = clipper_pathsd_get_path(path_mem, paths, 0);
        assert_eq!(clipper_pathd_length(path), 4);
        let first_point = clipper_pathd_get_point(path, 0);
        assert!((first_point.x - 0.0).abs() < 1e-6);
        assert!((first_point.y - 0.0).abs() < 1e-6);

        clipper_delete_pathd(path);
        clipper_delete_pathsd(paths);
        clipper_svgreader_clear(reader);
        clipper_delete_svgreader(reader);
    }
}

#[test]
fn test_svgreader_repeated_load_replaces_state() {
    let fixture_a = write_svg_fixture("svgreader_repeated_a.svg", "M0 0L10 0L5 10Z");
    let fixture_b = write_svg_fixture("svgreader_repeated_b.svg", "M50 50L60 50L60 60L50 60Z");
    let filename_a = std::ffi::CString::new(fixture_a.to_str().unwrap()).unwrap();
    let filename_b = std::ffi::CString::new(fixture_b.to_str().unwrap()).unwrap();

    unsafe {
        let reader_mem = alloc(clipper_svgreader_size());
        let reader = clipper_svgreader(reader_mem);

        clipper_svgreader_load_from_file(reader, filename_a.as_ptr());
        let paths_a_mem = alloc(clipper_pathsd_size());
        let paths_a = clipper_svgreader_get_pathsd(paths_a_mem, reader);
        let path_a_mem = alloc(clipper_pathd_size());
        let path_a = clipper_pathsd_get_path(path_a_mem, paths_a, 0);
        assert_eq!(clipper_pathd_length(path_a), 3);
        let first_point_a = clipper_pathd_get_point(path_a, 0);
        assert!((first_point_a.x - 0.0).abs() < 1e-6);
        assert!((first_point_a.y - 0.0).abs() < 1e-6);
        clipper_delete_pathd(path_a);
        clipper_delete_pathsd(paths_a);

        clipper_svgreader_load_from_file(reader, filename_b.as_ptr());
        let paths_b_mem = alloc(clipper_pathsd_size());
        let paths_b = clipper_svgreader_get_pathsd(paths_b_mem, reader);
        let path_b_mem = alloc(clipper_pathd_size());
        let path_b = clipper_pathsd_get_path(path_b_mem, paths_b, 0);
        assert_eq!(clipper_pathd_length(path_b), 4);
        let first_point_b = clipper_pathd_get_point(path_b, 0);
        assert!((first_point_b.x - 50.0).abs() < 1e-6);
        assert!((first_point_b.y - 50.0).abs() < 1e-6);
        clipper_delete_pathd(path_b);
        clipper_delete_pathsd(paths_b);

        clipper_svgreader_clear(reader);
        clipper_delete_svgreader(reader);
    }
}

#[test]
fn test_svgreader_clear_after_load_empties_paths() {
    let fixture = write_svg_fixture("svgreader_clear_after_load.svg", "M0 0L10 0L10 10L0 10Z");
    let filename = std::ffi::CString::new(fixture.to_str().unwrap()).unwrap();

    unsafe {
        let reader_mem = alloc(clipper_svgreader_size());
        let reader = clipper_svgreader(reader_mem);
        clipper_svgreader_load_from_file(reader, filename.as_ptr());

        let loaded_mem = alloc(clipper_pathsd_size());
        let loaded = clipper_svgreader_get_pathsd(loaded_mem, reader);
        assert_eq!(clipper_pathsd_length(loaded), 1);
        clipper_delete_pathsd(loaded);

        clipper_svgreader_clear(reader);

        let cleared_mem = alloc(clipper_pathsd_size());
        let cleared = clipper_svgreader_get_pathsd(cleared_mem, reader);
        assert_eq!(clipper_pathsd_length(cleared), 0);
        clipper_delete_pathsd(cleared);

        clipper_delete_svgreader(reader);
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

unsafe fn make_path64(points: &mut [ClipperPoint64]) -> *mut ClipperPath64 {
    unsafe {
        let mem = alloc(clipper_path64_size());
        clipper_path64_of_points(mem, points.as_mut_ptr(), points.len())
    }
}

unsafe fn make_paths64(paths: &mut [*mut ClipperPath64]) -> *mut crate::ClipperPaths64 {
    unsafe {
        let mem = alloc(clipper_paths64_size());
        clipper_paths64_of_paths(mem, paths.as_mut_ptr(), paths.len())
    }
}

unsafe fn make_pathd(points: &mut [ClipperPointD]) -> *mut ClipperPathD {
    unsafe {
        let mem = alloc(clipper_pathd_size());
        clipper_pathd_of_points(mem, points.as_mut_ptr(), points.len())
    }
}

unsafe fn make_pathsd(paths: &mut [*mut ClipperPathD]) -> *mut crate::ClipperPathsD {
    unsafe {
        let mem = alloc(clipper_pathsd_size());
        clipper_pathsd_of_paths(mem, paths.as_mut_ptr(), paths.len())
    }
}

unsafe fn collect_paths64(paths: *mut crate::ClipperPaths64) -> Vec<Vec<(i64, i64)>> {
    unsafe {
        let path_count: i32 = clipper_paths64_length(paths).try_into().unwrap();
        (0..path_count)
            .map(|i| {
                let point_count: i32 = clipper_paths64_path_length(paths, i).try_into().unwrap();
                (0..point_count)
                    .map(|j| {
                        let p = clipper_paths64_get_point(paths, i, j);
                        (p.x, p.y)
                    })
                    .collect()
            })
            .collect()
    }
}

unsafe fn collect_pathsd(paths: *mut crate::ClipperPathsD) -> Vec<Vec<(f64, f64)>> {
    unsafe {
        let path_count: i32 = clipper_pathsd_length(paths).try_into().unwrap();
        (0..path_count)
            .map(|i| {
                let point_count: i32 = clipper_pathsd_path_length(paths, i).try_into().unwrap();
                (0..point_count)
                    .map(|j| {
                        let p = clipper_pathsd_get_point(paths, i, j);
                        (p.x, p.y)
                    })
                    .collect()
            })
            .collect()
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
            let subject = make_path64(&mut triangle);
            let subjects = make_paths64(&mut [subject]);
            let clip = make_path64(&mut square);
            let clips = make_paths64(&mut [clip]);

            let clipper_mem = alloc(clipper_clipper64_size());
            let clipper_ptr = clipper_clipper64(clipper_mem);
            clipper_clipper64_add_subject(clipper_ptr, subjects);
            clipper_clipper64_add_clip(clipper_ptr, clips);

            clipper_delete_path64(subject);
            clipper_delete_paths64(subjects);
            clipper_delete_path64(clip);
            clipper_delete_paths64(clips);

            let closed = make_paths64(&mut []);
            let open = make_paths64(&mut []);

            let success = clipper_clipper64_execute(
                clipper_ptr,
                ClipperClipType_DIFFERENCE,
                ClipperFillRule_EVEN_ODD,
                closed,
                open,
            );
            assert_eq!(success, 1);

            let output = collect_paths64(closed);
            clipper_delete_paths64(closed);
            clipper_delete_paths64(open);
            clipper_delete_clipper64(clipper_ptr);
            output
        };

        assert_eq!(output, vec![vec![(5, 10), (2, 4), (4, 4), (4, 0), (10, 0)]]);
    }
}

#[test]
fn test_clipperd_intersection() {
    let mut subject = vec![
        ClipperPointD { x: 0.0, y: 0.0 },
        ClipperPointD { x: 10.0, y: 0.0 },
        ClipperPointD { x: 10.0, y: 10.0 },
        ClipperPointD { x: 0.0, y: 10.0 },
    ];
    let mut clip = vec![
        ClipperPointD { x: 5.0, y: 5.0 },
        ClipperPointD { x: 15.0, y: 5.0 },
        ClipperPointD { x: 15.0, y: 15.0 },
        ClipperPointD { x: 5.0, y: 15.0 },
    ];

    let result = unsafe {
        let subject_path = make_pathd(&mut subject);
        let subject_paths = make_pathsd(&mut [subject_path]);
        let clip_path = make_pathd(&mut clip);
        let clip_paths = make_pathsd(&mut [clip_path]);

        let clipper_mem = alloc(clipper_clipperd_size());
        let clipper_ptr = clipper_clipperd(clipper_mem, 4);
        clipper_clipperd_add_subject(clipper_ptr, subject_paths);
        clipper_clipperd_add_clip(clipper_ptr, clip_paths);

        clipper_delete_pathd(subject_path);
        clipper_delete_pathsd(subject_paths);
        clipper_delete_pathd(clip_path);
        clipper_delete_pathsd(clip_paths);

        let closed = make_pathsd(&mut []);
        let open = make_pathsd(&mut []);

        let success = clipper_clipperd_execute(
            clipper_ptr,
            ClipperClipType_INTERSECTION,
            ClipperFillRule_NON_ZERO,
            closed,
            open,
        );
        assert_eq!(success, 1);

        let output = collect_pathsd(closed);
        clipper_delete_pathsd(closed);
        clipper_delete_pathsd(open);
        clipper_delete_clipperd(clipper_ptr);
        output
    };

    assert_eq!(result.len(), 1);
    assert_eq!(result[0].len(), 4);
    let area: f64 = {
        let pts = &result[0];
        let mut signed = 0.0;
        for i in 0..pts.len() {
            let (x1, y1) = pts[i];
            let (x2, y2) = pts[(i + 1) % pts.len()];
            signed += x1 * y2 - x2 * y1;
        }
        (signed / 2.0).abs()
    };
    assert!((area - 25.0).abs() < 1e-6);
}

#[test]
fn test_clipper64_union_via_clipper() {
    let mut left = vec![
        ClipperPoint64 { x: 0, y: 0 },
        ClipperPoint64 { x: 10, y: 0 },
        ClipperPoint64 { x: 10, y: 10 },
        ClipperPoint64 { x: 0, y: 10 },
    ];
    let mut right = vec![
        ClipperPoint64 { x: 5, y: 0 },
        ClipperPoint64 { x: 15, y: 0 },
        ClipperPoint64 { x: 15, y: 10 },
        ClipperPoint64 { x: 5, y: 10 },
    ];

    let result_area = unsafe {
        let s = make_path64(&mut left);
        let ss = make_paths64(&mut [s]);
        let c = make_path64(&mut right);
        let cs = make_paths64(&mut [c]);

        let clipper_mem = alloc(clipper_clipper64_size());
        let clipper_ptr = clipper_clipper64(clipper_mem);
        clipper_clipper64_add_subject(clipper_ptr, ss);
        clipper_clipper64_add_clip(clipper_ptr, cs);

        clipper_delete_path64(s);
        clipper_delete_paths64(ss);
        clipper_delete_path64(c);
        clipper_delete_paths64(cs);

        let closed = make_paths64(&mut []);
        let open = make_paths64(&mut []);
        clipper_clipper64_execute(
            clipper_ptr,
            ClipperClipType_UNION,
            ClipperFillRule_NON_ZERO,
            closed,
            open,
        );
        let area = clipper_paths64_area(closed);
        clipper_delete_paths64(closed);
        clipper_delete_paths64(open);
        clipper_delete_clipper64(clipper_ptr);
        area
    };
    assert_eq!(result_area, 150.0);
}

#[test]
fn test_clipperoffset_inflate() {
    let mut square = vec![
        ClipperPoint64 { x: 0, y: 0 },
        ClipperPoint64 { x: 10, y: 0 },
        ClipperPoint64 { x: 10, y: 10 },
        ClipperPoint64 { x: 0, y: 10 },
    ];

    let inflated_area = unsafe {
        let path = make_path64(&mut square);
        let paths = make_paths64(&mut [path]);

        let offset_mem = alloc(clipper_clipperoffset_size());
        let offset_ptr = clipper_clipperoffset(offset_mem, 2.0, 0.25, 0, 0);
        clipper_clipperoffset_add_paths64(
            offset_ptr,
            paths,
            ClipperJoinType_ROUND_JOIN,
            ClipperEndType_POLYGON_END,
        );
        assert_eq!(clipper_clipperoffset_error_code(offset_ptr), 0);

        let result_mem = alloc(clipper_paths64_size());
        let result_ptr = clipper_clipperoffset_execute(result_mem, offset_ptr, 5.0);

        let area = clipper_paths64_area(result_ptr);

        clipper_delete_path64(path);
        clipper_delete_paths64(paths);
        clipper_delete_paths64(result_ptr);
        clipper_delete_clipperoffset(offset_ptr);
        area
    };

    assert!(
        inflated_area > 100.0 && inflated_area < 100.0 * 5.0,
        "expected inflated area > 100 and < 500; got {}",
        inflated_area
    );
}

#[test]
fn test_polytree64_round_trip() {
    let mut outer = vec![
        ClipperPoint64 { x: 0, y: 0 },
        ClipperPoint64 { x: 100, y: 0 },
        ClipperPoint64 { x: 100, y: 100 },
        ClipperPoint64 { x: 0, y: 100 },
    ];
    let mut hole = vec![
        ClipperPoint64 { x: 30, y: 30 },
        ClipperPoint64 { x: 30, y: 60 },
        ClipperPoint64 { x: 60, y: 60 },
        ClipperPoint64 { x: 60, y: 30 },
    ];

    let (count, child_is_hole, total_area) = unsafe {
        let outer_path = make_path64(&mut outer);
        let hole_path = make_path64(&mut hole);
        let subjects = make_paths64(&mut [outer_path, hole_path]);
        let clips = make_paths64(&mut []);

        let clipper_mem = alloc(clipper_clipper64_size());
        let clipper_ptr = clipper_clipper64(clipper_mem);
        clipper_clipper64_add_subject(clipper_ptr, subjects);
        clipper_clipper64_add_clip(clipper_ptr, clips);

        let tree_mem = alloc(clipper_polytree64_size());
        let tree_ptr = clipper_polytree64(tree_mem, std::ptr::null_mut());
        let open = make_paths64(&mut []);

        let success = clipper_clipper64_execute_tree_with_open(
            clipper_ptr,
            ClipperClipType_UNION,
            ClipperFillRule_EVEN_ODD,
            tree_ptr,
            open,
        );
        assert_eq!(success, 1);

        let count = clipper_polytree64_count(tree_ptr);
        let child = clipper_polytree64_get_child(tree_ptr, 0);
        let is_hole = clipper_polytree64_to_paths(alloc(clipper_paths64_size()), tree_ptr);
        let area = clipper_paths64_area(is_hole);

        let child_hole = if !child.is_null() {
            crate::clipper_polytree64_is_hole(child as *mut _) != 0
        } else {
            false
        };

        clipper_delete_path64(outer_path);
        clipper_delete_path64(hole_path);
        clipper_delete_paths64(subjects);
        clipper_delete_paths64(clips);
        clipper_delete_paths64(open);
        clipper_delete_paths64(is_hole);
        clipper_delete_polytree64(tree_ptr);
        clipper_delete_clipper64(clipper_ptr);
        (count, child_hole, area)
    };

    assert_eq!(count, 1);
    assert!(!child_is_hole);
    assert!(total_area > 0.0);
}

#[test]
fn test_path_simplify_reduces_points() {
    let mut zigzag: Vec<ClipperPoint64> = (0..50)
        .map(|i| ClipperPoint64 {
            x: i as i64,
            y: ((i % 2) as i64),
        })
        .collect();

    let (input_len, simplified_len) = unsafe {
        let path = make_path64(&mut zigzag);
        let mem = alloc(clipper_path64_size());
        let simplified = clipper_path64_simplify(mem, path, 5.0, 0);
        let input_len = crate::clipper_path64_length(path);
        let simplified_len = crate::clipper_path64_length(simplified);
        clipper_delete_path64(path);
        clipper_delete_path64(simplified);
        (input_len, simplified_len)
    };

    assert_eq!(input_len, 50);
    assert!(simplified_len < input_len);
}

#[test]
fn test_pathd_path64_round_trip() {
    let mut points_d = vec![
        ClipperPointD { x: 1.5, y: 2.5 },
        ClipperPointD { x: 10.0, y: 20.0 },
        ClipperPointD { x: 5.0, y: 30.0 },
    ];

    let recovered = unsafe {
        let pd = make_pathd(&mut points_d);
        let mem64 = alloc(clipper_path64_size());
        let p64 = clipper_pathd_to_path64(mem64, pd);
        let memd = alloc(clipper_pathd_size());
        let pd_back = clipper_path64_to_pathd(memd, p64);

        let len = crate::clipper_pathd_length(pd_back);
        let pts: Vec<(f64, f64)> = (0..len as i32)
            .map(|i| {
                let p = crate::clipper_pathd_get_point(pd_back, i);
                (p.x, p.y)
            })
            .collect();

        clipper_delete_pathd(pd);
        clipper_delete_path64(p64);
        clipper_delete_pathd(pd_back);
        pts
    };

    assert_eq!(recovered.len(), 3);
    let originals = [(1.5, 2.5), (10.0, 20.0), (5.0, 30.0)];
    for (got, want) in recovered.iter().zip(originals) {
        assert!(
            (got.0 - want.0).abs() <= 1.0,
            "x: got {} want ~{}",
            got.0,
            want.0
        );
        assert!(
            (got.1 - want.1).abs() <= 1.0,
            "y: got {} want ~{}",
            got.1,
            want.1
        );
    }
}

#[test]
fn test_paths_round_trip_d_64() {
    let mut a = vec![
        ClipperPointD { x: 0.0, y: 0.0 },
        ClipperPointD { x: 10.5, y: 0.0 },
        ClipperPointD { x: 10.5, y: 10.5 },
    ];
    let recovered = unsafe {
        let pd = make_pathd(&mut a);
        let psd = make_pathsd(&mut [pd]);
        let mem = alloc(clipper_paths64_size());
        let p64 = clipper_pathsd_to_paths64(mem, psd);
        let memd = alloc(clipper_pathsd_size());
        let psd_back = clipper_paths64_to_pathsd(memd, p64);

        let out = collect_pathsd(psd_back);

        clipper_delete_pathd(pd);
        clipper_delete_pathsd(psd);
        clipper_delete_paths64(p64);
        clipper_delete_pathsd(psd_back);
        out
    };

    assert_eq!(recovered.len(), 1);
    assert_eq!(recovered[0].len(), 3);
}

#[test]
fn test_point_in_polygon_64() {
    let mut square = vec![
        ClipperPoint64 { x: 0, y: 0 },
        ClipperPoint64 { x: 10, y: 0 },
        ClipperPoint64 { x: 10, y: 10 },
        ClipperPoint64 { x: 0, y: 10 },
    ];

    let (inside, outside) = unsafe {
        let path = make_path64(&mut square);
        let inside = clipper_point_in_path64(path, ClipperPoint64 { x: 5, y: 5 });
        let outside = clipper_point_in_path64(path, ClipperPoint64 { x: 50, y: 50 });
        clipper_delete_path64(path);
        (inside, outside)
    };

    assert_eq!(inside, ClipperPointInPolygonResult_IS_INSIDE);
    assert_eq!(outside, ClipperPointInPolygonResult_IS_OUTSIDE);
}

#[test]
fn test_area_calculations() {
    let mut square = vec![
        ClipperPoint64 { x: 0, y: 0 },
        ClipperPoint64 { x: 10, y: 0 },
        ClipperPoint64 { x: 10, y: 10 },
        ClipperPoint64 { x: 0, y: 10 },
    ];

    let mut squared = vec![
        ClipperPointD { x: 0.0, y: 0.0 },
        ClipperPointD { x: 10.5, y: 0.0 },
        ClipperPointD { x: 10.5, y: 10.5 },
        ClipperPointD { x: 0.0, y: 10.5 },
    ];

    let (area_64, area_d) = unsafe {
        let p64 = make_path64(&mut square);
        let pd = make_pathd(&mut squared);
        let a64 = clipper_path64_area(p64);
        let ad = clipper_pathd_area(pd);
        clipper_delete_path64(p64);
        clipper_delete_pathd(pd);
        (a64, ad)
    };

    assert_eq!(area_64, 100.0);
    assert!((area_d - 110.25).abs() < 1e-9);
}

#[test]
fn test_clipper64_settings_round_trip() {
    unsafe {
        let mem = alloc(clipper_clipper64_size());
        let c = clipper_clipper64(mem);
        clipper_clipper64_set_preserve_collinear(c, 1);
        clipper_clipper64_set_reverse_solution(c, 1);
        assert_eq!(clipper_clipper64_get_preserve_collinear(c), 1);
        assert_eq!(clipper_clipper64_get_reverse_solution(c), 1);
        clipper_clipper64_set_preserve_collinear(c, 0);
        assert_eq!(clipper_clipper64_get_preserve_collinear(c), 0);
        clipper_clipper64_clear(c);
        clipper_delete_clipper64(c);
    }
}

#[test]
fn test_paths_simplify() {
    let mut zigzag: Vec<ClipperPoint64> = (0..40)
        .map(|i| ClipperPoint64 {
            x: i as i64,
            y: ((i % 2) as i64),
        })
        .collect();

    unsafe {
        let path = make_path64(&mut zigzag);
        let paths = make_paths64(&mut [path]);
        let mem = alloc(clipper_paths64_size());
        let simplified = clipper_paths64_simplify(mem, paths, 2.0, 0);
        let len = clipper_paths64_path_length(simplified, 0);
        assert!(len < 40);
        clipper_delete_path64(path);
        clipper_delete_paths64(paths);
        clipper_delete_paths64(simplified);
    }
}

#[test]
fn test_clipperd_clear_then_reuse() {
    let mut subject = vec![
        ClipperPointD { x: 0.0, y: 0.0 },
        ClipperPointD { x: 5.0, y: 0.0 },
        ClipperPointD { x: 5.0, y: 5.0 },
        ClipperPointD { x: 0.0, y: 5.0 },
    ];

    unsafe {
        let mem = alloc(clipper_clipperd_size());
        let c = clipper_clipperd(mem, 2);

        let path = make_pathd(&mut subject);
        let paths = make_pathsd(&mut [path]);
        clipper_clipperd_add_subject(c, paths);

        clipper_clipperd_clear(c);
        clipper_clipperd_add_subject(c, paths);

        clipper_delete_pathd(path);
        clipper_delete_pathsd(paths);
        clipper_delete_clipperd(c);
    }
}

#[test]
fn test_paths_get_path_returns_copy() {
    let mut sq1 = vec![
        ClipperPoint64 { x: 0, y: 0 },
        ClipperPoint64 { x: 5, y: 0 },
        ClipperPoint64 { x: 5, y: 5 },
        ClipperPoint64 { x: 0, y: 5 },
    ];
    let mut sq2 = vec![
        ClipperPoint64 { x: 10, y: 10 },
        ClipperPoint64 { x: 20, y: 10 },
        ClipperPoint64 { x: 20, y: 20 },
        ClipperPoint64 { x: 10, y: 20 },
    ];

    unsafe {
        let p1 = make_path64(&mut sq1);
        let p2 = make_path64(&mut sq2);
        let paths = make_paths64(&mut [p1, p2]);

        let mem = alloc(clipper_path64_size());
        let copy = clipper_paths64_get_path(mem, paths, 1);
        let len = crate::clipper_path64_length(copy);
        assert_eq!(len, 4);

        clipper_delete_path64(p1);
        clipper_delete_path64(p2);
        clipper_delete_paths64(paths);
        clipper_delete_path64(copy);
    }
}
