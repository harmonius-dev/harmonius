//! Catalog query encoding and in-memory pagination helpers.

use serde::{Deserialize, Serialize};

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

/// Encode `offset` as an opaque cursor token (stable for tests).
#[must_use]
pub fn offset_cursor(offset: usize) -> String {
    format!("o:{offset}")
}

/// Decode cursor from [`offset_cursor`]; `None` means start.
pub fn parse_offset_cursor(cursor: Option<&String>) -> usize {
    let Some(c) = cursor else {
        return 0;
    };
    c.strip_prefix("o:")
        .and_then(|n| n.parse().ok())
        .unwrap_or(0)
}

/// Return the next page of `all` listings for a query (stable ordering: by id).
#[must_use]
pub fn paginate_catalog(all: &[Listing], query: &CatalogQuery) -> Page<Listing> {
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
    let offset = parse_offset_cursor(query.cursor.as_ref());
    let limit = query.limit.max(1) as usize;
    let total = filtered.len();
    let slice: Vec<Listing> = filtered.into_iter().skip(offset).take(limit).collect();
    let advanced = offset + slice.len();
    let next_cursor = if advanced < total {
        Some(offset_cursor(advanced))
    } else {
        None
    };
    Page {
        items: slice,
        next_cursor,
    }
}
