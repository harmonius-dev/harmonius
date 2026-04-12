# Visual Editors Test Cases

Companion test cases for [visual-editors.md](visual-editors.md).

Consolidates test cases from the following former files:

- logic-graph-test-cases.md
- material-animation-test-cases.md
- specialized-editors-test-cases.md

See those files for the full test case definitions until migration is complete.

## Unit Tests

| ID | Requirement | Description |
|----|-------------|-------------|
| TC-15.8.1.1 | R-15.8.1 | Graph IR node add/remove |
| TC-15.8.1.2 | R-15.8.1 | Graph IR edge connect |
| TC-15.8.1.3 | R-15.8.1 | Graph IR variable CRUD |
| TC-15.8.2.1 | R-15.8.2 | Type inference bind generic |
| TC-15.8.2.2 | R-15.8.2 | Type inference wildcard |
| TC-15.8.2.3 | R-15.8.2 | Type mismatch diagnostic |
| TC-15.8.3.1 | R-15.8.3 | Validation cycle detect |
| TC-15.8.3.2 | R-15.8.3 | Validation dangling pin |
| TC-15.8.3.3 | R-15.8.3 | Validation missing input |
| TC-15.8.4.1 | R-15.8.4 | Coroutine lowering |
| TC-15.8.4.2 | R-15.8.4 | Coroutine state variants |
| TC-15.8.12.1 | R-15.8.12 | Dead node elimination |
| TC-15.8.12.2 | R-15.8.12 | Constant folding |
| TC-15.8.12.3 | R-15.8.12 | Subgraph inlining |
| TC-15.8.12.4 | R-15.8.12 | Monomorphization |
| TC-15.8.5.1 | R-15.8.5a | Shader graph HLSL emit |
| TC-15.8.5.2 | R-15.8.5b | Shader compile DXC |
| TC-15.8.5.3 | R-15.8.5c | Material PBR template |
| TC-15.8.13.1 | R-15.8.13 | Graph two-way diff |
| TC-15.8.13.2 | R-15.8.13 | Graph three-way merge |
| TC-15.8.14.1 | R-15.8.14 | Find usages |
| TC-15.8.14.2 | R-15.8.14 | Rename propagation |
| TC-15.3.1.1 | R-15.3.1 | Material node connect |
| TC-15.3.1.2 | R-15.3.1 | Material pin type check |
| TC-15.3.2.1 | R-15.3.2 | Material function inline |
| TC-15.3.4.1 | R-15.3.4 | Parameter tweak no recompile |
| TC-15.3.5.1 | R-15.3.5 | Material instance override |
| TC-15.3.6.1 | R-15.3.6 | Material library search |
| TC-15.4.1.1 | R-15.4.1 | Timeline add keyframe |
| TC-15.4.1.2 | R-15.4.1 | Timeline move keyframe |
| TC-15.4.2.1 | R-15.4.2 | Curve tangent modes |
| TC-15.4.2.2 | R-15.4.2 | Curve preset apply |
| TC-15.4.3.1 | R-15.4.3 | Skeleton bone select |
| TC-15.4.5.1 | R-15.4.5 | Blend space 1D sample |
| TC-15.4.5.2 | R-15.4.5 | Blend space 2D sample |
| TC-15.4.6.1 | R-15.4.6 | State machine transition |
| TC-15.4.7.1 | R-15.4.7 | Retarget bone mapping |
| TC-15.15.1.1 | R-15.15.1 | Graph widget add node |
| TC-15.15.1.2 | R-15.15.1 | Graph widget connect |
| TC-15.15.2.1 | R-15.15.2 | Table widget add row |
| TC-15.15.2.2 | R-15.15.2 | Table widget sort |

## Integration Tests

| ID | Requirement | Description |
|----|-------------|-------------|
| TC-15.8.I.1 | R-15.8.1--R-15.8.4 | Compile-to-ECS |
| TC-15.8.I.2 | R-15.8.5a--R-15.8.5c | Shader pipeline |
| TC-15.8.I.3 | R-15.8.11 | Debug breakpoint hit |
| TC-15.3.I.1 | R-15.3.1--R-15.3.6 | Material edit-to-preview |
| TC-15.4.I.1 | R-15.4.1--R-15.4.7 | Anim edit-to-preview |
| TC-15.SE.I.1 | R-15.13.1 | Behavior tree execution |
| TC-15.SE.I.2 | R-15.15.1 | Loot table simulation |

## Benchmarks

| ID | Requirement | Description |
|----|-------------|-------------|
| TC-15.8.B.1 | R-15.8.12 | 500-node compile time |
| TC-15.8.B.2 | R-15.8.2 | Type inference latency |
| TC-15.3.B.1 | R-15.3.4 | Parameter update latency |
| TC-15.3.B.2 | R-15.3.1 | HLSL generation throughput |
| TC-15.4.B.1 | R-15.4.2 | Curve evaluate throughput |
