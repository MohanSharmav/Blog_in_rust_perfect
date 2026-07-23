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

    #[test]
    #[should_panic]
    fn total_pages_panics_on_zero_per_page() {
        // Documents a real constraint rather than hiding it: `per_page` must
        // be positive. The only caller, `DEFAULT_PAGE_SIZE`, always is; this
        // pins down what happens if a future caller ever gets it wrong.
        let _ = total_pages(10, 0);
    }
}

#[cfg(test)]
mod proptests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        /// Whatever `resolve_page` returns, it's either `None` or a page
        /// that's actually valid: 1-indexed and within its own `total`.
        #[test]
        fn resolved_page_is_always_in_range(
            page_param in any::<i64>(),
            total_items in 0i64..100_000,
            per_page in 1i64..1_000,
            clamp in any::<bool>(),
        ) {
            if let Some(page) = resolve_page(page_param, total_items, per_page, clamp) {
                prop_assert!(page.current >= 1);
                prop_assert!(page.total >= 1);
                prop_assert!(page.current <= page.total);
            }
        }

        /// A page number of zero or below is never valid, regardless of how
        /// many items there are or whether clamping is on — this also
        /// exercises the `page_param as usize` cast for every negative i64,
        /// including `i64::MIN`, to confirm the wraparound never accidentally
        /// produces an in-range `usize`.
        #[test]
        fn non_positive_page_param_is_always_rejected(
            page_param in i64::MIN..=0i64,
            total_items in 0i64..100_000,
            per_page in 1i64..1_000,
            clamp in any::<bool>(),
        ) {
            prop_assert_eq!(resolve_page(page_param, total_items, per_page, clamp), None);
        }

        /// Without clamping, an empty listing has no valid page at all —
        /// not even page 1.
        #[test]
        fn empty_listing_without_clamp_never_resolves(
            page_param in 1i64..1_000,
            per_page in 1i64..1_000,
        ) {
            prop_assert_eq!(resolve_page(page_param, 0, per_page, false), None);
        }

        /// With clamping, an empty listing always resolves to a single,
        /// empty page one — this is the guarantee `list_posts_for_category`
        /// relies on to avoid a redirect loop on an empty category.
        #[test]
        fn empty_listing_with_clamp_always_resolves_page_one(per_page in 1i64..1_000) {
            prop_assert_eq!(
                resolve_page(1, 0, per_page, true),
                Some(Page { current: 1, total: 1 })
            );
        }
    }
}
