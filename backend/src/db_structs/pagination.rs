/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/
use crate::errors::ApiError;
use crate::errors::ApiError::{InvalidPage, MaxPageNumberExceeded};
use utoipa::ToSchema;

/// A struct representing the entries a user wants to receive on an endpoint with pagination.
/// The input should be validated to make sure neither field is 0 or negative
#[derive(serde::Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PaginationOptions {
    page: i32,
    size: i32,
}

/// A struct representing the entries a user wants to receive on an endpoint with pagination.
/// The input should be validated to make sure neither field is 0 or negative
#[derive(serde::Deserialize, ToSchema, Debug)]
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
    pub fn to_some_with_default(
        &self,
        default_page: i32,
        default_size: i32,
        max_page: i32,
        max_size: i32,
    ) -> Result<PaginationOptions, ApiError> {
        PaginationOptions::new(
            self.page.unwrap_or(default_page),
            self.size.unwrap_or(default_size),
            max_page,
            max_size,
        )
    }
}

impl PaginationOptions {
    /// Returns true if the page number and size are both greater than 0,
    /// and if both do not exceed 1000.
    pub fn new(page: i32, size: i32, max_page: i32, max_size: i32) -> Result<Self, ApiError> {
        if page < 1 || size < 1 {
            return Err(InvalidPage);
        }
        if page > max_page {
            return Err(MaxPageNumberExceeded(max_page));
        }
        if size > max_size {
            return Err(MaxPageNumberExceeded(max_size));
        }
        Ok(Self { page, size })
    }

    pub fn get_page(&self) -> i32 {
        self.page
    }

    pub fn get_size(&self) -> i32 {
        self.size
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
        let empty_some = empty_optional
            .to_some_with_default(67, 67, 100, 500)
            .unwrap();
        assert_eq!(empty_some, correct);

        let page_only_optional = OptionalPaginationOptions {
            page: Some(67),
            size: None,
        };
        let page_only_some = page_only_optional
            .to_some_with_default(0, 67, 100, 500)
            .unwrap();
        assert_eq!(page_only_some, correct);

        let size_only_optional = OptionalPaginationOptions {
            page: None,
            size: Some(67),
        };
        let size_only_some = size_only_optional
            .to_some_with_default(67, 0, 100, 500)
            .unwrap();
        assert_eq!(size_only_some, correct);

        let filled_optional = OptionalPaginationOptions {
            page: Some(67),
            size: Some(67),
        };
        let filled_some = filled_optional
            .to_some_with_default(0, 0, 100, 500)
            .unwrap();
        assert_eq!(filled_some, correct);
    }

    #[test]
    fn test_pagination_options_valid() {
        let one = PaginationOptions::new(1, 1, 1000, 50);
        assert!(one.is_ok());

        let big = PaginationOptions::new(1000, 500, 1000, 500);
        assert!(big.is_ok());

        let page_zero = PaginationOptions::new(0, 1, 1000, 50);
        assert!(page_zero.is_err());

        let size_zero = PaginationOptions::new(1, 0, 1000, 50);
        assert!(size_zero.is_err());

        let page_too_big = PaginationOptions::new(10_000, 50, 1000, 50);
        assert!(page_too_big.is_err());

        let size_too_big = PaginationOptions::new(1, 500, 1000, 50);
        assert!(size_too_big.is_err());
    }
}
