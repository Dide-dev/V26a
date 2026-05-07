/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

import { apiCallGet } from './index';
import { type UserId } from './users';

const prefix = 'admin/';

function apiCallGetUser(path: string) {
    return apiCallGet(prefix + path);
}

/** The data of the user that is returned when requesting the data of all users */
export type AllUsersGetData = {
    id: number;
    email: string;
    name: string;
    enabled: boolean;
};

export type UserGetAllData = {
    email: string;
    name: string;
    enabled: boolean;
    nickname: string;
    aboutMe: string;
    emailVisible: boolean;
    accountType: string;
    language: string;
};
/** Get the data of all users */
export function getAllUsersData(): Promise<AllUsersGetData[]> {
    return apiCallGetUser('all-user-data');
}

/** Get the data of a specific user */
export function getUserAllData(userId: UserId): Promise<UserGetAllData> {
    return apiCallGetUser(`users/${userId}`);
}
