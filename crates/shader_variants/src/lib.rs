#![deny(clippy::all)]
#![deny(missing_docs)]
// `bundle` uses `mmap`; see `bundle.rs` SAFETY comment. DXC subprocess wiring stays out of this
// crate until the platform toolchains land in-repo.

//! Shader permutation keys, variant budgets, bundles, and runtime resolution.
//!
//! Implements the rendering shader variant design (`docs/design/rendering/shader-variants.md`).

pub mod budget;
pub mod bundle;
pub mod coverage;
pub mod metrics;
pub mod permutation;
pub mod precompile;
pub mod resolver;

pub use budget::{validate_variant_budgets, BudgetViolationKind, BuildBudgetError};
pub use bundle::{
    ShaderApi, VariantBundle, VariantBundleWriter, VariantError, VariantRecord, FORMAT_VERSION_V1,
};
pub use coverage::{coverage_report, CoverageReport};
pub use metrics::{next_precompile_from_metrics, UsageMetrics};
pub use permutation::{
    ContentHash, LodLevel, PermutationKey, RenderPath, ShaderFeatures, ShadingModel,
};
pub use precompile::{validate_post_precompile_caps, validate_precompile_subset};
pub use resolver::{OnDemandCompiler, ShaderResolver};
