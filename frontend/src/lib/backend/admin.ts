import { apiCallGet } from './index';

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

/** Get the user's own data */
export function getAllUserData(): Promise<UserGetAllData[]> {
    return apiCallGetUser('all-user-data');
}
