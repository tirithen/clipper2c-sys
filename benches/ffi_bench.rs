use clipper2c_sys::*;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::os::raw::c_void;

unsafe fn alloc(size: usize) -> *mut c_void {
    clipper_allocate(size)
}

fn bench_boolean_difference(c: &mut Criterion) {
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

    c.bench_function("boolean_difference", |b| {
        b.iter(|| unsafe {
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

            let closed_mem = alloc(clipper_paths64_size());
            let closed_ptr = clipper_paths64(closed_mem);
            let open_mem = alloc(clipper_paths64_size());
            let open_ptr = clipper_paths64(open_mem);

            clipper_clipper64_execute(
                clipper_ptr,
                ClipperClipType_DIFFERENCE,
                ClipperFillRule_EVEN_ODD,
                closed_ptr,
                open_ptr,
            );

            let result = clipper_paths64_length(closed_ptr);
            clipper_delete_paths64(closed_ptr);
            clipper_delete_paths64(open_ptr);
            clipper_delete_clipper64(clipper_ptr);
            black_box(result)
        })
    });
}

fn bench_path64_of_points(c: &mut Criterion) {
    let mut points: Vec<ClipperPoint64> = (0..1000)
        .map(|i| ClipperPoint64 { x: i, y: i * 2 })
        .collect();

    c.bench_function("path64_of_points_1000", |b| {
        b.iter(|| unsafe {
            let mem = alloc(clipper_path64_size());
            let path = clipper_path64_of_points(mem, points.as_mut_ptr(), points.len());
            let len = clipper_path64_length(path);
            clipper_delete_path64(path);
            black_box(len)
        })
    });
}

fn bench_pathd_of_points(c: &mut Criterion) {
    let mut points: Vec<ClipperPointD> = (0..1000)
        .map(|i| ClipperPointD {
            x: i as f64,
            y: i as f64 * 2.0,
        })
        .collect();

    c.bench_function("pathd_of_points_1000", |b| {
        b.iter(|| unsafe {
            let mem = alloc(clipper_pathd_size());
            let path = clipper_pathd_of_points(mem, points.as_mut_ptr(), points.len());
            let len = clipper_pathd_length(path);
            clipper_delete_pathd(path);
            black_box(len)
        })
    });
}

fn bench_path64_simplify(c: &mut Criterion) {
    let mut points: Vec<ClipperPoint64> = (0..500)
        .map(|i| ClipperPoint64 {
            x: i * 10,
            y: (i % 7) * 3,
        })
        .collect();

    let path = unsafe {
        let mem = alloc(clipper_path64_size());
        clipper_path64_of_points(mem, points.as_mut_ptr(), points.len())
    };

    c.bench_function("path64_simplify_500", |b| {
        b.iter(|| unsafe {
            let out_mem = alloc(clipper_path64_size());
            let simplified = clipper_path64_simplify(out_mem, path, 2.0, 0);
            let len = clipper_path64_length(simplified);
            clipper_delete_path64(simplified);
            black_box(len)
        })
    });

    unsafe { clipper_delete_path64(path) };
}

fn bench_inflate(c: &mut Criterion) {
    let mut points = vec![
        ClipperPoint64 { x: 0, y: 0 },
        ClipperPoint64 { x: 100, y: 0 },
        ClipperPoint64 { x: 100, y: 100 },
        ClipperPoint64 { x: 0, y: 100 },
    ];

    let (path, paths) = unsafe {
        let path_mem = alloc(clipper_path64_size());
        let path = clipper_path64_of_points(path_mem, points.as_mut_ptr(), points.len());
        let paths_mem = alloc(clipper_paths64_size());
        let paths = clipper_paths64_of_paths(paths_mem, [path].as_mut_ptr(), 1);
        (path, paths)
    };

    c.bench_function("inflate_square", |b| {
        b.iter(|| unsafe {
            let out_mem = alloc(clipper_paths64_size());
            let inflated = clipper_paths64_inflate(
                out_mem,
                paths,
                10.0,
                ClipperJoinType_ROUND_JOIN,
                ClipperEndType_POLYGON_END,
                2.0,
            );
            let len = clipper_paths64_length(inflated);
            clipper_delete_paths64(inflated);
            black_box(len)
        })
    });

    unsafe {
        clipper_delete_path64(path);
        clipper_delete_paths64(paths);
    }
}

fn bench_path_conversion(c: &mut Criterion) {
    let mut points: Vec<ClipperPoint64> = (0..1000)
        .map(|i| ClipperPoint64 { x: i, y: i * 2 })
        .collect();

    let path = unsafe {
        let mem = alloc(clipper_path64_size());
        clipper_path64_of_points(mem, points.as_mut_ptr(), points.len())
    };

    c.bench_function("path64_to_pathd_1000", |b| {
        b.iter(|| unsafe {
            let out_mem = alloc(clipper_pathd_size());
            let converted = clipper_path64_to_pathd(out_mem, path);
            let len = clipper_pathd_length(converted);
            clipper_delete_pathd(converted);
            black_box(len)
        })
    });

    unsafe { clipper_delete_path64(path) };
}

fn bench_clipper_offset(c: &mut Criterion) {
    let mut points = vec![
        ClipperPoint64 { x: 0, y: 0 },
        ClipperPoint64 { x: 100, y: 0 },
        ClipperPoint64 { x: 100, y: 100 },
        ClipperPoint64 { x: 0, y: 100 },
    ];

    let path = unsafe {
        let mem = alloc(clipper_path64_size());
        clipper_path64_of_points(mem, points.as_mut_ptr(), points.len())
    };

    c.bench_function("clipper_offset_execute", |b| {
        b.iter(|| unsafe {
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
            clipper_delete_paths64(result);
            clipper_delete_clipperoffset(co);
            black_box(len)
        })
    });

    unsafe { clipper_delete_path64(path) };
}

fn bench_polytree(c: &mut Criterion) {
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

    c.bench_function("polytree_execute_and_extract", |b| {
        b.iter(|| unsafe {
            let outer_mem = alloc(clipper_path64_size());
            let outer_path = clipper_path64_of_points(outer_mem, outer.as_mut_ptr(), outer.len());
            let subjects_mem = alloc(clipper_paths64_size());
            let subjects = clipper_paths64_of_paths(subjects_mem, [outer_path].as_mut_ptr(), 1);

            let inner_mem = alloc(clipper_path64_size());
            let inner_path = clipper_path64_of_points(inner_mem, inner.as_mut_ptr(), inner.len());
            let clips_mem = alloc(clipper_paths64_size());
            let clips = clipper_paths64_of_paths(clips_mem, [inner_path].as_mut_ptr(), 1);

            let clipper_mem = alloc(clipper_clipper64_size());
            let clipper = clipper_clipper64(clipper_mem);
            clipper_clipper64_add_subject(clipper, subjects);
            clipper_clipper64_add_clip(clipper, clips);

            let tree_mem = alloc(clipper_polytree64_size());
            let tree = clipper_polytree64(tree_mem, std::ptr::null_mut());
            let open_mem = alloc(clipper_paths64_size());
            let open = clipper_paths64(open_mem);

            clipper_clipper64_execute_tree_with_open(
                clipper,
                ClipperClipType_DIFFERENCE,
                ClipperFillRule_EVEN_ODD,
                tree,
                open,
            );

            let paths_mem = alloc(clipper_paths64_size());
            let paths = clipper_polytree64_to_paths(paths_mem, tree);
            let len = clipper_paths64_length(paths);

            clipper_delete_paths64(paths);
            clipper_delete_paths64(open);
            clipper_delete_polytree64(tree);
            clipper_delete_clipper64(clipper);
            clipper_delete_path64(outer_path);
            clipper_delete_paths64(subjects);
            clipper_delete_path64(inner_path);
            clipper_delete_paths64(clips);
            black_box(len)
        })
    });
}

criterion_group!(
    benches,
    bench_boolean_difference,
    bench_path64_of_points,
    bench_pathd_of_points,
    bench_path64_simplify,
    bench_inflate,
    bench_path_conversion,
    bench_clipper_offset,
    bench_polytree,
);
criterion_main!(benches);
