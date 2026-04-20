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
    nickname: string;
    aboutMe: string;
    emailVisible: boolean;
    language: string;
};

export type AddUserSuccess = {
    type: 'Success';
    userId: UserId;
};

export type AddUserEmailInvalid = {
    type: 'EmailAlreadyUsed';
};

/** The data of the user that is returned when requesting it */
export type UserGetData = {
    email: string;
    name: string;
    nickname: string;
    aboutMe: string;
    emailVisible: boolean;
    language: string;
};

/** The data of the user that is returned when requesting the data of all users */
export type UserGetAllData = {
    email: string;
    name: string;
    nickname: string;
    aboutMe: string;
    emailVisible: boolean;
    language: string;
    accountType: AccountType;
    enabled: boolean;
};

/**
 * The data needed to update a user
 * `accountType` and `enabled` are only updated if a non-null value is passed.
 * `nickname` and `aboutMe` are always updated, where null values are stored as-is in the
 * database.
 */
export type UserUpdateData = {
    name: string;
    nickname: string;
    aboutMe: string;
    emailVisible: boolean;
    language: string;
    accountType: Option<AccountType>;
    enabled: Option<boolean>;
};

/** The data in the cookie */
export type UserInfo = {
    /** This corresponds to userid of the current login. */
    sub: UserId;
    /** Expiration datetime for the current login, represented as number of seconds since unix epoch. */
    exp: number;
    accountType: AccountType;
};

/** Add a user to the database */
export function addUser(user: RegisterData): Promise<AddUserSuccess | AddUserEmailInvalid> {
    return apiCallPostUser('add', user);
}

/** Get the user's own data */
export function getOwnUserData(): Promise<UserGetData> {
    return apiCallGetUser('me');
}

/** Get the user's own data */
export function getAllUserData(): Promise<UserGetAllData[]> {
    return apiCallGetUser('admin/all-user-data');
}

/** Update a user's data */
export function updateUser(userId: UserId, newData: UserUpdateData): Promise<void> {
    return apiCallPostUser(`update/${userId}`, newData);
}

/**
 * Read the userinfo from the cookie.
 * If logged in return id & type and if not logged in return null.
 */
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
