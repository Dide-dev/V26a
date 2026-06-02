/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

import { goto } from '$app/navigation';
import { page } from '$app/state';
import { appLink } from '$lib';

/** This can be changed locally if api lives somewhere else */
const prefix = '/api/v1/';

/** This function is for the rest of the api-wrapper to call upon, do not call from UI! */
export async function apiCallGet(path: string) {
    const response = await fetch(prefix + path, {
        method: 'GET',
        // eslint-disable-next-line @typescript-eslint/naming-convention
        headers: { Accept: 'application/json' }
    });

    if (!(await checkResponse(response))) {
        return;
    }

    return await response.json();
}

/** This function is for the rest of the api-wrapper to call upon, do not call from UI! */
export async function apiCallEmptyDelete(path: string) {
    const response = await fetch(prefix + path, {
        method: 'DELETE',
        // eslint-disable-next-line
        headers: { Accept: 'application/json' }
    });

    if (!(await checkResponse(response))) {
        return;
    }

    // Only return if some content is sent back
    if (response.headers.get('Content-Type') === 'application/json') {
        return await response.json();
    }
}

/** This function is for the rest of the api-wrapper to call upon, do not call from UI! */
export async function apiCallEmptyPost(path: string) {
    const response = await fetch(prefix + path, {
        method: 'POST'
    });

    if (!(await checkResponse(response))) {
        return;
    }

    // Only return if some content is sent back
    if (response.headers.get('Content-Type') === 'application/json') {
        return await response.json();
    } else {
        return null;
    }
}

/** This function is for the rest of the api-wrapper to call upon, do not call from UI! */
export async function apiCallDelete(path: string, body: unknown) {
    const response = await fetch(prefix + path, {
        method: 'DELETE',
        // eslint-disable-next-line @typescript-eslint/naming-convention
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(body)
    });

    if (!(await checkResponse(response))) {
        return;
    }

    // Only return if some content is sent back
    if (response.headers.get('Content-Type') === 'application/json') {
        return await response.json();
    } else {
        return null;
    }
}

/** This function is for the rest of the api-wrapper to call upon, do not call from UI! */
export async function apiCallPost(path: string, body: unknown) {
    const response = await fetch(prefix + path, {
        method: 'POST',
        // eslint-disable-next-line @typescript-eslint/naming-convention
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(body)
    });

    if (!(await checkResponse(response))) {
        return;
    }

    // Only return if some content is sent back
    if (response.headers.get('Content-Type') === 'application/json') {
        return await response.json();
    } else {
        return null;
    }
}

/** This function is for the rest of the api-wrapper to call upon, do not call from UI! */
export async function apiCallPut(path: string, body: unknown) {
    const response = await fetch(prefix + path, {
        method: 'PUT',
        // eslint-disable-next-line @typescript-eslint/naming-convention
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(body)
    });

    if (!(await checkResponse(response))) {
        return;
    }

    // Only return if some content is sent back
    if (response.headers.get('Content-Type') === 'application/json') {
        return await response.json();
    } else {
        return null;
    }
}

export async function apiCallPostFile(path: string, file: File) {
    // Content-Type is inferred by the browser.
    const response = await fetch(prefix + path, {
        method: 'POST',
        body: file
    });

    if (!(await checkResponse(response))) {
        return;
    }

    return await response.text();
}

/** Check if the response is successful and handle any errors in a proper way.
 *
 * @return `false` if the response is an error, otherwise `true`. */
async function checkResponse(response: Response): Promise<boolean> {
    // Redirect to login page when user is not logged in while that is needed to perform action
    if (response.status === 401) {
        await redirectLogin(page.url);
        return false;
    }

    if (!response.ok) {
        throw new Error(await response.text());
    }
    return true;
}

/** Go to the login page with a redirect to the given url when successfully logged in. */
export async function redirectLogin(redirectToURL: URL) {
    const encodedURI = encodeURI(
        redirectToURL.pathname + redirectToURL.search + redirectToURL.hash
    );
    await goto(appLink(`/login?redirect=${encodedURI}`));
}
