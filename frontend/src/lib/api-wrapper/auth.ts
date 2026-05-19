/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

import { apiCallPost } from './index';

const prefix = 'auth/';

function apiCallPostAuth(path: string, body: unknown) {
    return apiCallPost(prefix + path, body);
}

/** The data required to login */
export type LoginData = {
    email: string;
    password: string;
};

export type LoginResponse = {
    succeeded: boolean;
};

/** Log in with the provided credentials */
export function login(credentials: LoginData): Promise<LoginResponse> {
    return apiCallPostAuth('login', credentials);
}
