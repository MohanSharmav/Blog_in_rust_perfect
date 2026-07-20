//! Pure pagination business rules — no I/O, no framework types.

/// Default number of items shown per listing page across the app.
pub const DEFAULT_PAGE_SIZE: i64 = 3;

/// A resolved, in-range page position.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Page {
    pub current: usize,
    pub total: usize,
}

/// A page of items, together with the resolved page position and the total
/// item count across all pages.
#[derive(Debug, Clone, PartialEq)]
pub struct Listing<T> {
    pub items: Vec<T>,
    pub page: Page,
    pub total_items: i64,
}

/// Total number of pages needed to show `total_items` at `per_page` items per page.
pub fn total_pages(total_items: i64, per_page: i64) -> i64 {
    let pages = total_items / per_page;
    if total_items % per_page != 0 {
        pages + 1
    } else {
        pages
    }
}

/// Validates a 1-indexed `page_param` against the item/page-size counts.
///
/// Returns the resolved page on success, or `None` if the requested page is
/// out of range. When `clamp_min_one_page` is set, a listing with zero items
/// still counts as having one (empty) page rather than never resolving.
pub fn resolve_page(
    page_param: i64,
    total_items: i64,
    per_page: i64,
    clamp_min_one_page: bool,
) -> Option<Page> {
    let mut total = total_pages(total_items, per_page) as usize;
    if clamp_min_one_page {
        total = total.max(1);
    }
    let current = page_param as usize;

    if current == 0 || current > total {
        None
    } else {
        Some(Page { current, total })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn total_pages_rounds_up_on_remainder() {
        assert_eq!(total_pages(7, 3), 3);
        assert_eq!(total_pages(6, 3), 2);
        assert_eq!(total_pages(0, 3), 0);
    }

    #[test]
    fn resolve_page_rejects_out_of_range() {
        assert_eq!(resolve_page(0, 7, 3, false), None);
        assert_eq!(resolve_page(4, 7, 3, false), None);
        assert_eq!(
            resolve_page(2, 7, 3, false),
            Some(Page {
                current: 2,
                total: 3
            })
        );
    }

    #[test]
    fn resolve_page_can_clamp_to_one_page_when_empty() {
        assert_eq!(resolve_page(1, 0, 3, false), None);
        assert_eq!(
            resolve_page(1, 0, 3, true),
            Some(Page {
                current: 1,
                total: 1
            })
        );
    }
}
