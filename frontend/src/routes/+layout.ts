/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

// Required for the static site generation
export const prerender = true;
// If this is not set, the static site contains files like 'abc.hmtl', instead of 'abc/index.html'.
// The `tower_http::services::ServeDir` in the backend expects the latter.
export const trailingSlash = 'always';
