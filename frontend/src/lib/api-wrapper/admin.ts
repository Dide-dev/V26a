/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

import { apiCallGet } from './index';
import { type UserId } from './users';
import { type PaginationOptions, getPaginationParams } from '$lib/pagination';

const prefix = 'admin/';

function apiCallGetAdmin(path: string) {
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

export type UserListResponse = {
    totalUsers: number;
    users: AllUsersGetData[];
};

/** Get the data of all users */
export function getAllUserData(pageOptions: PaginationOptions): Promise<UserListResponse> {
    const params: string = getPaginationParams(pageOptions);
    return apiCallGetAdmin(`all-user-data?${params}`);
}

/** Get the data of a specific user */
export function getUserAllData(userId: UserId): Promise<UserGetAllData> {
    return apiCallGetAdmin(`users/${userId}`);
}
