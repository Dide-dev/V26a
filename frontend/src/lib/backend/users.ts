/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

import { apiCallPost, apiCallGet } from './index';
import type { Option } from './types';

const prefix = 'users/';

function apiCallPostUser(path: string, body: unknown) {
    return apiCallPost(prefix + path, body);
}

function apiCallGetUser(path: string) {
    return apiCallGet(prefix + path);
}

export type UserId = number;
export type AccountType = 'author' | 'admin';

/** The data needed to register a new user */
export type RegisterData = {
    email: string;
    password: string;
    name: string;
    nickname: Option<string>;
    aboutMe: Option<string>;
    emailVisible: boolean;
    language: string;
};

/** The data of the user that is returned when requesting it */
export type UserGetData = {
    email: string;
    name: string;
    nickname: Option<string>;
    aboutMe: Option<string>;
    emailVisible: boolean;
    language: string;
};

/**
 * The data needed to update a user
 * `account_type` is only updated if a non-null value is passed.
 * `nickname` and `about_me` are always updated, where null values are stored as-is in the
 * database.
 */
export type UserUpdateData = {
    name: string;
    nickname: Option<string>;
    aboutMe: Option<string>;
    emailVisible: boolean;
    language: string;
    accountType: Option<AccountType>;
};

/** The data in the cookie */
export type UserInfo = {
    userId: UserId;
    accountType: AccountType;
};

/** Add a user to the database */
export function addUser(user: RegisterData): Promise<UserId> {
    return apiCallPostUser('add', user);
}

/** Get the user's own data */
export function getOwnUserData(): Promise<UserGetData> {
    return apiCallGetUser('me');
}

/** Get the user's own data */
export function updateUser(id: UserId, newData: UserUpdateData): Promise<UserGetData> {
    return apiCallPostUser('update/' + id, newData);
}

/** read the userinfo from the cookie */
/** if logged in return id & type and if not logged in return null */
export function getUserInfo(): UserInfo | null {
    if (typeof document === 'undefined') return null;
    const cookie = document.cookie.split('; ').find((row) => row.startsWith('user_info='));
    if (!cookie) return null;
    try {
        return JSON.parse(decodeURIComponent(cookie.split('=')[1]));
    } catch {
        return null;
    }
}
