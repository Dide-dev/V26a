/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

import { apiCallGet, apiCallPost } from './index';
import {
    type UserId,
    type UserSuccess,
    type UserEmailInvalid,
    type UsernameInvalid,
    type ChangePasswordResponse,
    type UserGeneralData
} from './users';
import { type PaginationOptions, getPaginationParams } from '$lib/pagination';

const prefix = 'admin/';

function apiCallGetAdmin(path: string) {
    return apiCallGet(prefix + path);
}

function apiCallPostUser(path: string, body: unknown) {
    return apiCallPost(prefix + path, body);
}

/** The data of the user that is returned when requesting the data of all users */
export type AllUsersGetData = {
    id: number;
    email: string;
    name: string;
    enabled: boolean;
};

/** The data of the user that is returned when requesting the data of one user */
export type UserAllData = {
    email: string;
    data: UserGeneralData;
    enabled: boolean;
    accountType: string;
};

/** The data needed to change a user's password */
export type AdminChangePasswordData = {
    newPassword: string;
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
export function getUserAllData(userId: UserId): Promise<UserAllData> {
    return apiCallGetAdmin(`users/${userId}`);
}
/** Update the data of a user */
export function updateUserData(
    userId: UserId,
    newData: UserAllData
): Promise<UserSuccess | UserEmailInvalid | UsernameInvalid> {
    return apiCallPostUser(`users/${userId}/update-data`, newData);
}

/** Change a user's password */
export function adminChangePassword(
    userId: UserId,
    changePasswordData: AdminChangePasswordData
): Promise<ChangePasswordResponse> {
    return apiCallPostUser(`users/${userId}/update-password`, changePasswordData);
}
