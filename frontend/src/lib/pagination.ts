/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/
import type { Option } from '$lib/api-wrapper/types';

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
