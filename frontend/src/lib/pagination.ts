/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/
import { type TreeListResponse } from '$lib/backend/trees';
import { type UserListResponse } from '$lib/backend/admin';

export function calcTotalPages(total: number, pageSize: number): number {
    return Math.ceil(total / pageSize);
}

export function showTreeCount(treeListResponse: TreeListResponse | null): number {
    if (treeListResponse == null) {
        return 0;
    } else {
        return treeListResponse.totalTrees;
    }
}

export function showUserCount(userListResponse: UserListResponse | null): number {
    if (userListResponse == null) {
        return 0;
    } else {
        return userListResponse.totalUsers;
    }
}
