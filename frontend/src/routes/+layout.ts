// Required for the static site generation
export const prerender = true;
// If this is not set, the static site contains files like 'abc.hmtl', instead of 'abc/index.html'.
// The `tower_http::services::ServeDir` in the backend expects the latter.
export const trailingSlash = 'always';
