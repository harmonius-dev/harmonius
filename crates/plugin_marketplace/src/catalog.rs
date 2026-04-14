//! Catalog query encoding and in-memory pagination helpers.

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Client-side catalog search and pagination parameters (JSON on the wire).
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CatalogQuery {
    /// Free-text search string.
    pub query: String,
    /// Maximum listings per page.
    pub limit: u32,
    /// Opaque cursor from a previous page.
    pub cursor: Option<String>,
}

/// One row in a catalog listing response.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Listing {
    /// Plugin id.
    pub id: String,
    /// Latest or matched version string.
    pub version: String,
    /// Short summary.
    pub summary: String,
}

/// Paginated slice of listings plus optional next cursor.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Page<T> {
    /// Items for this page.
    pub items: Vec<T>,
    /// Cursor for the following page, if any.
    pub next_cursor: Option<String>,
}

/// Invalid catalog cursor or query state.
#[derive(Clone, Debug, Eq, PartialEq, Error)]
pub enum CatalogError {
    /// Cursor string was present but not a valid `o:<usize>` token.
    #[error("invalid pagination cursor")]
    InvalidCursor,
}

/// Encode `offset` as an opaque cursor token (stable for tests).
#[must_use]
pub fn offset_cursor(offset: usize) -> String {
    format!("o:{offset}")
}

/// Decode cursor from [`offset_cursor`]; `None` or empty means offset zero.
pub fn parse_offset_cursor(cursor: Option<&str>) -> Result<usize, CatalogError> {
    match cursor {
        None | Some("") => Ok(0),
        Some(c) => {
            let rest = c.strip_prefix("o:").ok_or(CatalogError::InvalidCursor)?;
            if rest.is_empty() {
                return Err(CatalogError::InvalidCursor);
            }
            rest
                .parse()
                .map_err(|_| CatalogError::InvalidCursor)
        }
    }
}

/// Return the next page of `all` listings for a query (stable ordering: by id).
pub fn paginate_catalog(
    all: &[Listing],
    query: &CatalogQuery,
) -> Result<Page<Listing>, CatalogError> {
    let mut filtered: Vec<Listing> = all
        .iter()
        .filter(|l| {
            query.query.is_empty()
                || l.id.contains(&query.query)
                || l.summary.contains(&query.query)
        })
        .cloned()
        .collect();
    filtered.sort_by(|a, b| a.id.cmp(&b.id));
    let offset = parse_offset_cursor(query.cursor.as_deref())?;
    let limit = query.limit.max(1) as usize;
    let total = filtered.len();
    let slice: Vec<Listing> = filtered.into_iter().skip(offset).take(limit).collect();
    let advanced = offset + slice.len();
    let next_cursor = if advanced < total {
        Some(offset_cursor(advanced))
    } else {
        None
    };
    Ok(Page {
        items: slice,
        next_cursor,
    })
}
