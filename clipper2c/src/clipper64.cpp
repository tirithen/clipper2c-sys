#include <clipper2/clipper.h>

#include "conv.h"
#include "types.h"
#include <clipper2c.h>
#include <stdint.h>

using namespace Clipper2Lib;

#ifdef __cplusplus
extern "C" {
#endif

// Constructors

ClipperClipper64 *clipper_clipper64(void *mem) {
  return to_c(new (mem) Clipper64());
}

// Setters / Getters

void clipper_clipper64_set_preserve_collinear(ClipperClipper64 *c, int t) {
  from_c(c)->PreserveCollinear(t);
}

void clipper_clipper64_set_reverse_solution(ClipperClipper64 *c, int t) {
  from_c(c)->ReverseSolution(t);
}

int clipper_clipper64_get_preserve_collinear(ClipperClipper64 *c) {
  return from_c(c)->PreserveCollinear();
}

int clipper_clipper64_get_reverse_solution(ClipperClipper64 *c) {
  return from_c(c)->ReverseSolution();
}

void clipper_clipper64_clear(ClipperClipper64 *c) { from_c(c)->Clear(); }

#ifdef USINGZ
void clipper_clipper64_set_z_callback(ClipperClipper64 *c, void* user_data, ClipperZCallback64 cb) {
  from_c(c)->SetZCallback([cb, user_data](const Point64 &p1, const Point64 &p2,
                               const Point64 &p3, const Point64 &p4,
                               Point64 &p5) {
    cb(user_data,
       reinterpret_cast<const ClipperPoint64 *>(&p1),
       reinterpret_cast<const ClipperPoint64 *>(&p2),
       reinterpret_cast<const ClipperPoint64 *>(&p3),
       reinterpret_cast<const ClipperPoint64 *>(&p4),
       reinterpret_cast<ClipperPoint64 *>(&p5));
  });
}
#endif

// Methods

void clipper_clipper64_add_subject(ClipperClipper64 *c,
                                   ClipperPaths64 *subjects) {
  from_c(c)->AddSubject(*from_c(subjects));
}

void clipper_clipper64_add_open_subject(ClipperClipper64 *c,
                                        ClipperPaths64 *open_subjects) {
  from_c(c)->AddOpenSubject(*from_c(open_subjects));
}

void clipper_clipper64_add_clip(ClipperClipper64 *c, ClipperPaths64 *clips) {
  from_c(c)->AddClip(*from_c(clips));
}

int clipper_clipper64_execute(ClipperClipper64 *c64, ClipperClipType ct,
                              ClipperFillRule fr, ClipperPaths64 *closed,
                              ClipperPaths64 *open) {
  return from_c(c64)->Execute(from_c(ct), from_c(fr), *from_c(closed),
                              *from_c(open));
}

int clipper_clipper64_execute_tree(ClipperClipper64 *c64, ClipperClipType ct,
                                   ClipperFillRule fr,
                                   ClipperPolyTree64 *tree) {
  return from_c(c64)->Execute(from_c(ct), from_c(fr), *from_c(tree));
}

int clipper_clipper64_execute_tree_with_open(ClipperClipper64 *c64,
                                             ClipperClipType ct,
                                             ClipperFillRule fr,
                                             ClipperPolyTree64 *tree,
                                             ClipperPaths64 *open) {
  return from_c(c64)->Execute(from_c(ct), from_c(fr), *from_c(tree),
                              *from_c(open));
}

#ifdef __cplusplus
}
#endif
