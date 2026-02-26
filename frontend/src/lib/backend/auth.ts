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

/** Log in with the provided credentials */
export function login(credentials: LoginData): Promise<void> {
    return apiCallPostAuth('login', credentials);
}
