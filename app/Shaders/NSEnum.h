#pragma once

#ifndef NS_ENUM
#if defined(__METAL_VERSION__)
#define NS_ENUM(_type, _name) enum _name : _type
#else
#define NS_ENUM(_type, _name) enum _name : _type _name; enum _name : _type
#endif
#endif
