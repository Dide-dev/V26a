import { apiCallPost, apiCallGet } from './index';
import type { Option } from './types';

const prefix = 'users/';

function apiCallPostUser(path: string, body: unknown) {
    return apiCallPost(prefix + path, body);
}

// TODO: remove next line when the function is used
// eslint-disable-next-line
function apiCallGetUser(path: string) {
    return apiCallGet(prefix + path);
}

export type UserId = number;

/** The data of the user */
export type UserData = {
    email: string;
    password: string;
    name: string;
    nickname: Option<string>;
    aboutMe: Option<string>;
    emailVisible: boolean;
    language: string;
};

/** Add a user to the database */
export function addUser(user: UserData): Promise<UserId> {
    return apiCallPostUser('add', user);
}
