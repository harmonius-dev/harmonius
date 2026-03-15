# Scene Rendering Pipeline User Stories

## US-2.10.1 Efficient ECS-to-GPU Data Flow

**As a** graphics programmer, **I want** decoupled render proxy extraction with dirty-flag
incremental updates, **so that** only changed entities are re-uploaded each frame, keeping
CPU and bus bandwidth proportional to changes rather than total entity count.

## US-2.10.2 Rendering Debug Diagnostics

**As a** graphics programmer, **I want** buffer visualization modes (depth, normals, motion
vectors, roughness, overdraw heat maps) and an immediate-mode debug drawing API for gizmos,
**so that** I can rapidly diagnose rendering issues during development.
