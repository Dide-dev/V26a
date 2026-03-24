/// A struct representing the entries a user wants to receive on an endpoint with pagination.
/// The input should be validated to make sure neither field is 0 or negative
#[derive(serde::Deserialize, Debug)]
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
    pub fn input_is_valid(&self) -> bool {
        self.page > 0 && self.size > 0
        // global maxima that should never be exceeded.
        && self.page <= 1000 && self.size <= 1000
    }
}

impl Default for PaginationOptions {
    fn default() -> Self {
        PaginationOptions { page: 1, size: 50 }
    }
}
