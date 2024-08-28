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
#ifdef USINGZ
    int64_t z;
#endif
  } ClipperPointD;

  typedef struct ClipperPoint64
  {
    int64_t x;
    int64_t y;
#ifdef USINGZ
    int64_t z;
#endif
  } ClipperPoint64;

#ifdef USINGZ
  typedef void (*ClipperZCallback64)(
    void* user_data,
    const ClipperPoint64* e1bot,
    const ClipperPoint64* e1top,
    const ClipperPoint64* e2bot,
    const ClipperPoint64* e2top,
    ClipperPoint64* pt
  );

  typedef void (*ClipperZCallbackD)(
    void* user_data,
    const ClipperPointD* e1bot,
    const ClipperPointD* e1top,
    const ClipperPointD* e2bot,
    const ClipperPointD* e2top,
    ClipperPointD* pt
  );
#endif

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
