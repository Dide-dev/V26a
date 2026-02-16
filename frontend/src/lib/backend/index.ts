/** This can be changed locally if api lives somewhere else */
const prefix = '/api/v1/';

/** This function is for the rest of the backend to call upon, do not call from UI! */
export async function api_call_get(path: string) {
	const response = await fetch(prefix + path, {
		method: 'GET',
		headers: { Accept: 'application/json' }
	});
	return await response.json();
}

/** This function is for the rest of the backend to call upon, do not call from UI! */
export async function api_call_post(path: string, body: unknown) {
	const response = await fetch(prefix + path, {
		method: 'POST',
		headers: { 'Content-Type': 'application/json' },
		body: JSON.stringify(body)
	});
	return await response.json();
}
