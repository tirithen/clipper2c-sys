if(EXISTS "/home/tirithen/Programmering/Clipper2/CPP/ClipperTests[1]_tests.cmake")
  include("/home/tirithen/Programmering/Clipper2/CPP/ClipperTests[1]_tests.cmake")
else()
  add_test(ClipperTests_NOT_BUILT ClipperTests_NOT_BUILT)
endif()