/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/
import { type TreeListResponse } from '$lib/backend/trees';

export function calcTotalPages(totalTrees: number, pageSize: number): number {
    return Math.ceil(totalTrees / pageSize);
}

export function showTreeCount(treeListResponse: TreeListResponse | null): number {
    if (treeListResponse == null) {
        return 0;
    } else {
        return treeListResponse.totalTrees;
    }
}
