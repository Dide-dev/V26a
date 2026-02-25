import { apiCallPost } from './index';

const prefix = 'auth/';

function apiCallPostAuth(path: string, body: unknown) {
	return apiCallPost(prefix + path, body);
}

/** The data required to login */
type LoginData = {
	email: string;
	password: string;
};

/** Add a user to the database */
export function login(user: LoginData): Promise<void> {
	return apiCallPostAuth('login', user);
}
