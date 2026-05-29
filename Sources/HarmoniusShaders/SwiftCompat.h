#pragma once

#if defined(__SLANG__)
#define HarmoniusFloat2 float2
#define HarmoniusFloat4 float4
#define HarmoniusUInt2 uint2
#else
#include <hlsl++.h>

using HarmoniusFloat2 = hlslpp::float2;
using HarmoniusFloat4 = hlslpp::float4;
using HarmoniusUInt2 = hlslpp::uint2;

inline HarmoniusFloat2 HarmoniusMakeFloat2(float x, float y) {
  return HarmoniusFloat2(x, y);
}

inline HarmoniusFloat4 HarmoniusMakeFloat4(float x, float y, float z, float w) {
  return HarmoniusFloat4(x, y, z, w);
}

inline HarmoniusUInt2 HarmoniusMakeUInt2(uint32_t x, uint32_t y) {
  return HarmoniusUInt2(x, y);
}

inline float HarmoniusFloat2Component(const HarmoniusFloat2& value, int index) {
  return value[index];
}

inline float HarmoniusFloat4Component(const HarmoniusFloat4& value, int index) {
  return value[index];
}
#endif

#ifndef NS_ENUM
#define NS_ENUM(_type, _name) enum _name : _type
#endif
