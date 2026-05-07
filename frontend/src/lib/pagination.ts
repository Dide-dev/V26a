/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/
import { type TreeListResponse } from '$lib/backend/trees';
import { type UserListResponse } from '$lib/backend/admin';
import type { Option } from '$lib/backend/types';

export type PaginationOptions = {
    page: Option<number>;
    size: Option<number>;
};

export function getPaginationParams(pageOptions: PaginationOptions): string {
    const params = new URLSearchParams();

    Object.entries(pageOptions).forEach(([key, value]) => {
        if (value !== null) {
            params.append(key, String(value));
        }
    });
    return String(params);
}

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
