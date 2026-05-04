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

  typedef struct ClipperPointD
  {
    double x;
    double y;
  } ClipperPointD;

  typedef struct ClipperPoint64
  {
    int64_t x;
    int64_t y;
  } ClipperPoint64;

  struct ClipperRect64
  {
    int64_t left;
    int64_t top;
    int64_t right;
    int64_t bottom;
  };

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

  typedef enum ClipperJoinType
  {
    SQUARE_JOIN,
    BEVEL_JOIN,
    ROUND_JOIN,
    MITER_JOIN
  } ClipperJoinType;

  /** Open-path endpoint handling for ClipperOffset. POLYGON_END is the
   *  closed-polygon case — no endpoints to inflate. The other variants
   *  decide how the start and end of a polyline are extended: BUTT keeps
   *  the line flat at the original endpoint, SQUARE extends one delta
   *  past it, ROUND adds a half-circle cap, JOINED keeps polylines
   *  connected to neighbouring segments. */
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
