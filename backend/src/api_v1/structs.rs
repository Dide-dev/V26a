/// A struct representing the entries a user wants to receive on an endpoint with pagination.
/// The input should be validated to make sure neither field is 0 or negative
#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PaginationOptions {
    pub page: i32,
    pub size: i32,
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
