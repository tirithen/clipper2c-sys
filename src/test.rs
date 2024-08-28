use crate::{
    clipper_clipper64, clipper_clipper64_add_clip, clipper_clipper64_add_subject,
    clipper_clipper64_execute, clipper_clipper64_size, clipper_delete_clipper64,
    clipper_delete_path64, clipper_delete_paths64, clipper_path64_of_points, clipper_path64_size,
    clipper_paths64_get_point, clipper_paths64_length, clipper_paths64_of_paths,
    clipper_paths64_path_length, clipper_paths64_size, ClipperClipType_DIFFERENCE,
    ClipperFillRule_EVEN_ODD, ClipperPoint64,
};

unsafe fn malloc(size: usize) -> *mut std::os::raw::c_void {
    libc::malloc(size)
}

#[test]
fn test_difference_boolean_operation() {
    let mut triangle = vec![
        ClipperPoint64 {
            x: 0,
            y: 0,
            ..Default::default()
        },
        ClipperPoint64 {
            x: 10,
            y: 0,
            ..Default::default()
        },
        ClipperPoint64 {
            x: 5,
            y: 10,
            ..Default::default()
        },
    ];

    let mut square = vec![
        ClipperPoint64 {
            x: 0,
            y: 0,
            ..Default::default()
        },
        ClipperPoint64 {
            x: 4,
            y: 0,
            ..Default::default()
        },
        ClipperPoint64 {
            x: 4,
            y: 4,
            ..Default::default()
        },
        ClipperPoint64 {
            x: 0,
            y: 4,
            ..Default::default()
        },
    ];

    for _ in 0..1_000 {
        let output = unsafe {
            let subject_mem = malloc(clipper_path64_size());
            let subject_ptr =
                clipper_path64_of_points(subject_mem, triangle.as_mut_ptr(), triangle.len());

            let subjects_mem = malloc(clipper_paths64_size());
            let subjects_ptr =
                clipper_paths64_of_paths(subjects_mem, [subject_ptr].as_mut_ptr(), 1);

            let clip_mem = malloc(clipper_path64_size());
            let clip_ptr = clipper_path64_of_points(clip_mem, square.as_mut_ptr(), square.len());

            let clips_mem = malloc(clipper_paths64_size());
            let clips_ptr = clipper_paths64_of_paths(clips_mem, [clip_ptr].as_mut_ptr(), 1);

            let clipper_mem = malloc(clipper_clipper64_size());
            let clipper_ptr = clipper_clipper64(clipper_mem);

            #[cfg(feature = "usingz")]
            {
                extern "C" fn cb(
                    _ud: *mut std::ffi::c_void,
                    _e1bot: *const ClipperPoint64,
                    _e1top: *const ClipperPoint64,
                    _e2bot: *const ClipperPoint64,
                    _e2top: *const ClipperPoint64,
                    pt: *mut ClipperPoint64,
                ) {
                    unsafe {
                        (*pt).z = 1;
                    }
                }
                crate::clipper_clipper64_set_z_callback(
                    clipper_ptr,
                    std::ptr::null_mut(),
                    Some(cb),
                );
            }

            clipper_clipper64_add_subject(clipper_ptr, subjects_ptr);
            clipper_clipper64_add_clip(clipper_ptr, clips_ptr);

            clipper_delete_path64(subject_ptr);
            clipper_delete_paths64(subjects_ptr);
            clipper_delete_path64(clip_ptr);
            clipper_delete_paths64(clips_ptr);

            let closed_path_mem = malloc(clipper_paths64_size());
            let closed_path_ptr = clipper_paths64_of_paths(closed_path_mem, [].as_mut_ptr(), 0);

            let open_path_mem = malloc(clipper_paths64_size());
            let open_path_ptr = clipper_paths64_of_paths(open_path_mem, [].as_mut_ptr(), 0);

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

                    #[cfg(feature = "usingz")]
                    {
                        assert!((0..point_len)
                            .map(|j| clipper_paths64_get_point(closed_path_ptr, i, j))
                            .map(|p| p.z)
                            .any(|z| z == 1));
                    }

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
