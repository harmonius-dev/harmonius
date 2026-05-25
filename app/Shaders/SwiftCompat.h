#pragma once

#ifndef __SLANG__
#include <simd/simd.h>
typedef float2 simd_float2;
typedef float4 simd_float4;
typedef uint2 simd_uint2;
#endif

#ifndef NS_ENUM
#ifdef __SLANG__
#define NS_ENUM(_type, _name) enum _name : _type
#else
#define NS_ENUM(_type, _name)                                                  \
  typedef enum _name : _type _name;                                            \
  enum _name : _type
#endif
#endif
