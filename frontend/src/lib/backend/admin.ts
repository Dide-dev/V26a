import { apiCallGet, apiCallPost } from './index';
import {
    type UserId,
    type UserSuccess,
    type UserEmailInvalid,
    type UsernameInvalid
} from './users';

const prefix = 'admin/';

function apiCallGetUser(path: string) {
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
    name: string;
    enabled: boolean;
    nickname: string;
    aboutMe: string;
    emailVisible: boolean;
    accountType: string;
    language: string;
};

/** Get the user's own data */
export function getAllUserData(): Promise<AllUsersGetData[]> {
    return apiCallGetUser('all-user-data');
}

/** Update the data of a user */
export function updateUserData(
    userId: UserId,
    newData: UserAllData
): Promise<UserSuccess | UserEmailInvalid | UsernameInvalid> {
    return apiCallPostUser(`users/${userId}/update-data`, newData);
}
