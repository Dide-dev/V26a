/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

import { apiCallPost, apiCallGet, apiCallDelete } from './index';
import type { Option } from './types';

const prefix = 'users/';

function apiCallPostUser(path: string, body: unknown) {
    return apiCallPost(prefix + path, body);
}

function apiCallDeleteUser(path: string) {
    return apiCallDelete(prefix + path);
}

function apiCallGetUser(path: string) {
    return apiCallGet(prefix + path);
}

export type UserId = number;
export type AccountType = 'author' | 'admin';

/** The data a user can modify, except email or password
 *
 * The data needed to update a user
 * `accountType` and `enabled` are only updated if a non-null value is passed.
 * `nickname` and `aboutMe` are always updated, where null values are stored as-is in the
 * database.
 */
export type UserData = {
    name: string;
    nickname: string;
    aboutMe: string;
    emailVisible: boolean;
    language: string;
};

/** The data needed to register a new user */
export type RegisterData = {
    email: string;
    password: string;
    data: UserData;
};

export type UserSuccess = {
    type: 'Success';
    userId: UserId;
};

export type UserEmailInvalid = {
    type: 'EmailAlreadyUsed';
};

export type UsernameInvalid = {
    type: 'UsernameAlreadyUsed';
};

export type ChangePasswordResponse = {
    succeeded: boolean;
};

/** The private data of the user that is returned when requesting it */
export type UserGetData = {
    email: string;
    data: UserData;
};

/** The public data of a users profile page. */
export type PublicUserData = {
    name: string;
    nickname: string;
    aboutMe: string;
    language: string;
    accountType: AccountType;
    /** Is null when the user set the `emailVisible` to false. */
    email: Option<string>;
};

/** The data in the cookie */
export type UserInfo = {
    /** This corresponds to userid of the current login. */
    sub: UserId;
    /** Expiration datetime for the current login, represented as number of seconds since unix epoch. */
    exp: number;
    accountType: AccountType;
};

/** The data needed to change a user's password */
export type ChangePasswordData = {
    oldPassword: string;
    newPassword: string;
};

/** Add a user to the database */
export function addUser(
    user: RegisterData
): Promise<UserSuccess | UserEmailInvalid | UsernameInvalid> {
    return apiCallPostUser('add', user);
}

/** Delete a user in the database */
export function deleteUser(userId: UserId, deleteTrees: boolean): Promise<void> {
    return apiCallDeleteUser(`/${userId}/delete?deleteOwnedTrees=${deleteTrees}`);
}

/** Get the user's own data */
export function getOwnUserData(): Promise<UserGetData> {
    return apiCallGetUser('me');
}

/** Get a user's public data. Is by default cached in a global variable, to reduce server load. */
export function getPublicUserData(userId: UserId): Promise<PublicUserData> {
    return apiCallGetUser(`${userId}/public`);
}

/** Update a user's own data */
export function updateUser(
    newData: UserData
): Promise<UserSuccess | UserEmailInvalid | UsernameInvalid> {
    return apiCallPostUser(`update`, newData);
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

/** Change a user's password */
export function changePassword(
    changePasswordData: ChangePasswordData
): Promise<ChangePasswordResponse> {
    return apiCallPostUser('change-password', changePasswordData);
}
