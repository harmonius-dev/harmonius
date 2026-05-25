#pragma once

#if defined(__SLANG__)
// Slang provides float2, float4, and uint2.
#else
#include <simd/simd.h>
typedef simd_float2 float2;
typedef simd_float4 float4;
typedef simd_uint2 uint2;
#endif

#ifndef NS_ENUM
#if defined(__SLANG__)
#define NS_ENUM(_type, _name) enum _name : _type
#else
#define NS_ENUM(_type, _name)                                                  \
  typedef enum _name : _type _name;                                            \
  enum _name : _type
#endif
#endif
