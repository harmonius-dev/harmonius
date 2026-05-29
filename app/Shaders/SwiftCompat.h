#pragma once

#if defined(__SLANG__)
// Slang provides float2, float4, and uint2.
#else
#include <hlsl++.h>
using float2 = hlslpp::float2;
using float4 = hlslpp::float4;
using uint2 = hlslpp::uint2;
#endif

#ifndef NS_ENUM
#define NS_ENUM(_type, _name) enum _name : _type
#endif
