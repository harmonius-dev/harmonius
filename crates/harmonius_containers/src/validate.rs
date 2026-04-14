//! Pure transfer validation without mutating containers.

use crate::container::Container;
use crate::tags::TagSet;
use crate::transfer::TransferError;

/// Validates a linear insert into `dest` without mutating it.
pub fn validate_transfer(
    dest: &Container,
    incoming_weight: f32,
    incoming_tags: &TagSet,
) -> Result<(), TransferError> {
    if dest.len() >= dest.capacity() {
        return Err(TransferError::ContainerFull);
    }

    let attempted = dest.total_weight() + incoming_weight;
    if attempted > dest.def().weight_limit {
        return Err(TransferError::OverWeight {
            limit: dest.def().weight_limit,
            attempted,
        });
    }

    let required = dest.def().required_tags.to_vec();
    if !required.is_empty() && !incoming_tags.contains_all(&required) {
        return Err(TransferError::TagMismatch {
            required,
            provided: incoming_tags.to_vec(),
        });
    }

    Ok(())
}
