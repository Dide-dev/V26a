/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

/** This can be changed locally if api lives somewhere else */
const prefix = '/api/v1/';

/** This function is for the rest of the backend to call upon, do not call from UI! */
export async function apiCallGet(path: string) {
    const response = await fetch(prefix + path, {
        method: 'GET',
        // eslint-disable-next-line
        headers: { Accept: 'application/json' }
    });

    if (!response.ok) {
        throw new Error(await response.text());
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

    if (!response.ok) {
        throw new Error(await response.text());
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
        // eslint-disable-next-line
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(body)
    });

    if (!response.ok) {
        throw new Error(await response.text());
    }

    // Only return if some content is sent back
    if (response.headers.get('Content-Type') === 'application/json') {
        return await response.json();
    }
}
