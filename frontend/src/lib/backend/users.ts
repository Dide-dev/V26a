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
export type UserData = {
    email: string;
    name: string;
    nickname: Option<string>;
    aboutMe: Option<string>;
    emailVisible: boolean;
    language: string;
};

/** The data in the cookie */
export type UserInfo = {
    /** This corresponds to userid of the current login. */
    sub: UserId;
    /** Experation datetime for the current login, represented as number of seconds since unix epoch. */
    exp: number;
    accountType: 'author' | 'admin';
};

/** Add a user to the database */
export function addUser(user: RegisterData): Promise<UserId> {
    return apiCallPostUser('add', user);
}

/** Get the user's own data */
export function getOwnUserData(): Promise<UserData> {
    return apiCallGetUser('me');
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
