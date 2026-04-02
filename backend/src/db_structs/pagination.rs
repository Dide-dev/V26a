/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

const GLOBAL_MAX_PAGE_COUNT: i32 = 1000;
const GLOBAL_MAX_PAGE_SIZE: i32 = 1000;

/// A struct representing the entries a user wants to receive on an endpoint with pagination.
/// The input should be validated to make sure neither field is 0 or negative
#[derive(serde::Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PaginationOptions {
    pub page: i32,
    pub size: i32,
}

/// A struct representing the entries a user wants to receive on an endpoint with pagination.
/// The input should be validated to make sure neither field is 0 or negative
#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OptionalPaginationOptions {
    pub page: Option<i32>,
    pub size: Option<i32>,
}

impl OptionalPaginationOptions {
    /// Convert from [`OptionalPaginationOptions`] to [`PaginationOptions`], providing default
    /// values for potentially optional fields.
    ///
    /// # Arguments
    /// `default_page`: the page index to set if no page is given.
    ///
    /// `default_size`: the page size to set if no size is given.
    ///
    /// # Returns
    /// A [`PaginationOptions`] that contains the page and size that were passed, or the defaults
    /// entered in this function if not.
    pub fn to_some_with_default(&self, default_page: i32, default_size: i32) -> PaginationOptions {
        PaginationOptions {
            page: self.page.unwrap_or(default_page),
            size: self.size.unwrap_or(default_size),
        }
    }
}

impl PaginationOptions {
    /// Returns true if the page number and size are both greater than 0,
    /// and if both do not exceed 1000.
    pub fn is_valid(&self) -> bool {
        self.page > 0 && self.size > 0
        // global maxima that should never be exceeded.
        && self.page <= GLOBAL_MAX_PAGE_COUNT && self.size <= GLOBAL_MAX_PAGE_SIZE
    }
}

impl Default for PaginationOptions {
    fn default() -> Self {
        PaginationOptions { page: 1, size: 50 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pagination_options_conversion() {
        let correct = PaginationOptions { page: 67, size: 67 };

        let empty_optional = OptionalPaginationOptions {
            page: None,
            size: None,
        };
        let empty_some = empty_optional.to_some_with_default(67, 67);
        assert_eq!(empty_some, correct);

        let page_only_optional = OptionalPaginationOptions {
            page: Some(67),
            size: None,
        };
        let page_only_some = page_only_optional.to_some_with_default(0, 67);
        assert_eq!(page_only_some, correct);

        let size_only_optional = OptionalPaginationOptions {
            page: None,
            size: Some(67),
        };
        let size_only_some = size_only_optional.to_some_with_default(67, 0);
        assert_eq!(size_only_some, correct);

        let filled_optional = OptionalPaginationOptions {
            page: Some(67),
            size: Some(67),
        };
        let filled_some = filled_optional.to_some_with_default(0, 0);
        assert_eq!(filled_some, correct);
    }

    #[test]
    fn test_pagination_options_valid() {
        let one = PaginationOptions { page: 1, size: 1 };
        assert!(one.is_valid());

        let big = PaginationOptions {
            page: 500,
            size: 500,
        };
        assert!(big.is_valid());

        let zero = PaginationOptions { page: 0, size: 0 };
        assert!(!zero.is_valid());

        let single_zero = PaginationOptions { page: 0, size: 1 };
        assert!(!single_zero.is_valid());

        let page_too_big = PaginationOptions {
            page: 10_000,
            size: 0,
        };
        assert!(!page_too_big.is_valid());

        let size_too_big = PaginationOptions {
            page: 0,
            size: 100_000,
        };
        assert!(!size_too_big.is_valid());
    }
}
