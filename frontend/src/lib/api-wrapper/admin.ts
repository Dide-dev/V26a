/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

import { apiCallGet, apiCallPut } from './index';
import {
    type UserId,
    type UserSuccess,
    type UserEmailInvalid,
    type UsernameInvalid,
    type ChangePasswordResponse,
    type UserGeneralData
} from './users';
import { type PaginationOptions, getPaginationParams } from '$lib/pagination';

const prefix = 'admin';

function apiCallGetAdmin(path: string) {
    return apiCallGet(prefix + path);
}

function apiCallPutAdmin(path: string, body: unknown) {
    return apiCallPut(prefix + path, body);
}

/** The data of the user that is returned when requesting the data of all users */
export type AllUsersGetData = {
    id: number;
    email: string;
    name: string;
    enabled: boolean;
};

/** The data of the user that is returned when requesting the data of one user */
export type AdminGetUserData = {
    createdAt: Date;
    lastLogin: Date;
    updateableData: AdminUpdateData;
};

/** The data of the user that can be updated */
export type AdminUpdateData = {
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

/** The data returned when retrieving feedback. */
export type FeedbackListResponse = {
    feedbackTotal: number;
    feedback: FeedbackGetData[];
};

/** Data corresponding to a returned piece of feedback. */
export type FeedbackGetData = {
    feedbackId: number;
    email: string;
    feedback: string;
    createdAt: Date;
};

/** Get the data of all users */
export function getAllUserData(pageOptions: PaginationOptions): Promise<UserListResponse> {
    const params: string = getPaginationParams(pageOptions);
    return apiCallGetAdmin(`/users?${params}`);
}

/** Get the data of a specific user */
export function getUserAllData(userId: UserId): Promise<AdminGetUserData> {
    return apiCallGetAdmin(`/users/${userId}`);
}
/** Update the data of a user */
export function updateUserData(
    userId: UserId,
    newData: AdminUpdateData
): Promise<UserSuccess | UserEmailInvalid | UsernameInvalid> {
    return apiCallPutAdmin(`/users/${userId}`, newData);
}

/** Change a user's password */
export function adminChangePassword(
    userId: UserId,
    changePasswordData: AdminChangePasswordData
): Promise<ChangePasswordResponse> {
    return apiCallPutAdmin(`/users/${userId}/password`, changePasswordData);
}

/** Get feedback from the database. */
export function getFeedback(pageOptions: PaginationOptions): Promise<FeedbackListResponse> {
    const params: string = getPaginationParams(pageOptions);
    return apiCallGetAdmin(`/feedback?${params}`);
}
