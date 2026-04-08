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

export type AddUserSuccess = {
    type: 'Success';
    userId: UserId;
};

export type AddUserEmailInvalid = {
    type: 'EmailAlreadyUsed';
};

/** Add a user to the database */
export function addUser(user: UserData): Promise<AddUserSuccess | AddUserEmailInvalid> {
    return apiCallPostUser('add', user);
}
