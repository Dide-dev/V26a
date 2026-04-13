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

/** This function is for the rest of the backend to call upon, do not call from UI! */
export async function apiCallGet(path: string) {
    const response = await fetch(prefix + path, {
        method: 'GET',
        // eslint-disable-next-line @typescript-eslint/naming-convention
        headers: { Accept: 'application/json' }
    });

    if (await handleIfError(response)) {
        return;
    }

    return await response.json();
}

/** This function is for the rest of the backend to call upon, do not call from UI! */
export async function apiCallDelete(path: string) {
    const response = await fetch(prefix + path, {
        method: 'DELETE',
        // eslint-disable-next-line
        headers: { Accept: 'application/json' }
    });

    if (await handleIfError(response)) {
        return;
    }

    // Only return if some content is sent back
    if (response.headers.get('Content-Type') === 'application/json') {
        return await response.json();
    }
}

/** This function is for the rest of the backend to call upon, do not call from UI! */
export async function apiCallPost(path: string, body: unknown) {
    const response = await fetch(prefix + path, {
        method: 'POST',
        // eslint-disable-next-line @typescript-eslint/naming-convention
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(body)
    });

    if (await handleIfError(response)) {
        return;
    }

    // Only return if some content is sent back
    if (response.headers.get('Content-Type') === 'application/json') {
        return await response.json();
    } else {
        return null;
    }
}

/** Check if the response is an error and handle it in a proper way.
 * Returns true if the response is really an error, otherwise false. */
async function handleIfError(response: Response): Promise<boolean> {
    // Redirect to login page when user is not logged in while that is needed to perform action
    if (response.status === 401) {
        await redirectLogin(page.url);
        return true;
    }

    if (!response.ok) {
        throw new Error(await response.text());
    }
    return false;
}

/** Go to the login page with a redirect to the given url when successfully logged in. */
export async function redirectLogin(redirectToURL: URL) {
    const encodedURI = encodeURI(
        redirectToURL.pathname + redirectToURL.search + redirectToURL.hash
    );
    await goto(appLink(`/login?redirect=${encodedURI}`));
}
