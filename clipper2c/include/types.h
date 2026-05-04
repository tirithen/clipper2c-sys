#ifndef TYPES_H
#define TYPES_H
#pragma once
#include <stddef.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C"
{
#endif

  typedef struct ClipperClipper64 ClipperClipper64;
  typedef struct ClipperClipperD ClipperClipperD;
  typedef struct ClipperClipperOffset ClipperClipperOffset;
  typedef struct ClipperPath64 ClipperPath64;
  typedef struct ClipperPathD ClipperPathD;
  typedef struct ClipperPaths64 ClipperPaths64;
  typedef struct ClipperPathsD ClipperPathsD;
  typedef struct ClipperRect64 ClipperRect64;
  typedef struct ClipperRectD ClipperRectD;
  typedef struct ClipperPolyTree64 ClipperPolyTree64;
  typedef struct ClipperPolyTreeD ClipperPolyTreeD;
  typedef struct ClipperSvgWriter ClipperSvgWriter;
  typedef struct ClipperSvgReader ClipperSvgReader;

  /** Coordinate pair in floating-point (f64) space. The clipping engine
   *  itself runs on integers; ClipperClipperD multiplies each component
   *  by its scale factor (s = 128 at the default precision = 2) and
   *  rounds to ClipperPoint64 before feeding the engine, so |x × s| and
   *  |y × s| must fit ClipperPoint64's range. Outputs are divided back
   *  and quantised to the scaling grid. */
  typedef struct ClipperPointD
  {
    double x;
    double y;
  } ClipperPointD;

  /** Coordinate pair in the engine's native integer (i64) space. Each
   *  component must satisfy |c| ≤ INT64_MAX/4 (~2.3 × 10¹⁸); values
   *  outside this range produce a range error during a clipping
   *  operation. */
  typedef struct ClipperPoint64
  {
    int64_t x;
    int64_t y;
  } ClipperPoint64;

  /** Axis-aligned bounding rectangle in i64 space. Returned by
   *  clipper_path64_bounds / clipper_paths64_bounds; consumed by
   *  clipper_*_rect_clip and clipper_*_rect_clip_line. */
  struct ClipperRect64
  {
    int64_t left;
    int64_t top;
    int64_t right;
    int64_t bottom;
  };

  /** Axis-aligned bounding rectangle in f64 space. The f64 counterpart
   *  to ClipperRect64 — same role, scaled-coordinate constraint inherits
   *  from ClipperPointD. */
  struct ClipperRectD
  {
    double left;
    double top;
    double right;
    double bottom;
  };

  /** Determines which subregions are 'inside' when paths self-intersect
   *  or overlap. EVEN_ODD toggles inside/outside on every edge crossing.
   *  NON_ZERO uses the winding-number sign — the conventional rule for
   *  polygons-with-holes data where holes wind opposite to their outer
   *  contour. POSITIVE and NEGATIVE only count windings of the matching
   *  sign, useful when direction has been baked into the input. */
  typedef enum ClipperFillRule
  {
    EVEN_ODD,
    NON_ZERO,
    POSITIVE,
    NEGATIVE
  } ClipperFillRule;

  /** Boolean operation kind. NONE is the sentinel default for an
   *  unconfigured engine — passing it to clipper_*_execute is undefined.
   *  Use INTERSECTION / UNION / DIFFERENCE / XOR for the standard four
   *  set operations. */
  typedef enum ClipperClipType
  {
    NONE,
    INTERSECTION,
    UNION,
    DIFFERENCE,
    XOR
  } ClipperClipType;

  /** Subject paths are the input geometry being clipped. Clip paths are
   *  the mask. The clipping engine combines the two sets according to
   *  the chosen ClipperClipType and ClipperFillRule. */
  typedef enum ClipperPathType
  {
    SUBJECT,
    CLIP
  } ClipperPathType;

  /** Corner-handling style for ClipperOffset; same shapes as SVG and
   *  Cairo stroke joins. SQUARE squares off perpendicular to the
   *  corner. BEVEL flattens the corner with a straight cut. ROUND
   *  replaces it with a circular arc. MITER extends the offset edges
   *  to their intersection, clipped at the configured miter limit. */
  typedef enum ClipperJoinType
  {
    SQUARE_JOIN,
    BEVEL_JOIN,
    ROUND_JOIN,
    MITER_JOIN
  } ClipperJoinType;

  /** Open-path endpoint handling for ClipperOffset. POLYGON_END is the
   *  closed-polygon case — no endpoints to inflate. The other variants
   *  decide how the start and end of a polyline are extended (listed in
   *  enum order): JOINED keeps polylines connected to neighbouring
   *  segments, BUTT keeps the line flat at the original endpoint,
   *  SQUARE extends one delta past it, ROUND adds a half-circle cap. */
  typedef enum ClipperEndType
  {
    POLYGON_END,
    JOINED_END,
    BUTT_END,
    SQUARE_END,
    ROUND_END
  } ClipperEndType;

  typedef enum ClipperPointInPolygonResult
  {
    IS_ON,
    IS_INSIDE,
    IS_OUTSIDE
  } ClipperPointInPolygonResult;

#ifdef __cplusplus
}
#endif

#endif
