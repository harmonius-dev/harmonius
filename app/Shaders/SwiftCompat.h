#pragma once

#if defined(__SLANG__)
#define HarmoniusFloat2 float2
#define HarmoniusFloat4 float4
#define HarmoniusUInt2 uint2
#else
#include <stdint.h>

typedef struct __attribute__((aligned(8))) {
  float x;
  float y;
} HarmoniusFloat2;

typedef struct __attribute__((aligned(16))) {
  float x;
  float y;
  float z;
  float w;
} HarmoniusFloat4;

typedef struct __attribute__((aligned(8))) {
  uint32_t x;
  uint32_t y;
} HarmoniusUInt2;
#endif

#ifndef NS_ENUM
#define NS_ENUM(_type, _name) enum _name : _type
#endif
