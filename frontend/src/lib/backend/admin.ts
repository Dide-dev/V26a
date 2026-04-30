import { apiCallGet } from './index';
import { type PaginationOptions, getPaginationParams } from './trees';

const prefix = 'admin/';

function apiCallGetUser(path: string) {
    return apiCallGet(prefix + path);
}

/** The data of the user that is returned when requesting the data of all users */
export type UserGetAllData = {
    id: number;
    email: string;
    name: string;
    enabled: boolean;
};

export type UserListResponse = {
    totalTrees: number;
    users: UserGetAllData[];
};

/** Get the data of all user's */
export function getAllUserData(pageOptions: PaginationOptions): Promise<UserListResponse> {
    const params: string = getPaginationParams(pageOptions);
    return apiCallGetUser(`all-user-data?${params}`);
}
