#pragma once

#ifdef __cplusplus
extern "C" {
#endif

typedef struct HSRSlangReflectionOutput {
  int status;
  const char* json;
  const char* diagnostics;
} HSRSlangReflectionOutput;

HSRSlangReflectionOutput hsrCompileSlangReflection(
  const char* sourcePath,
  const char* metallibPath,
  const char* includeDirectory
);

void hsrFreeSlangReflectionOutput(HSRSlangReflectionOutput output);

#ifdef __cplusplus
}
#endif
