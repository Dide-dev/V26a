/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

import { expect, test } from 'vitest';
import { type FilterOptions, getPaginationAndFilterParams } from '$lib/api-calls/trees';
import type { PaginationOptions } from '$lib/pagination';

test('Pagination & Filter link generator', () => {
    const noPage: PaginationOptions = {
        page: null,
        size: null
    };
    const noFilters: FilterOptions = {
        book: [],
        createdAfter: null,
        createdBefore: null,
        ownerId: null,
        showOwnPrivate: null
    };

    const nothingParams = getPaginationAndFilterParams(noPage, noFilters);
    expect(nothingParams).equals('');

    const fullPage: PaginationOptions = { page: 1, size: 50 };
    const fullFilters: FilterOptions = {
        ownerId: 123,
        book: ['test', 'test2'],
        createdAfter: new Date(Date.UTC(2012, 3, 10)),
        createdBefore: new Date(Date.UTC(2019, 11, 3)),
        showOwnPrivate: true
    };

    const fullParams = getPaginationAndFilterParams(fullPage, fullFilters);
    expect(fullParams).equals(
        'page=1&size=50&ownerId=123&book=test&book=test2&createdAfter=2012-04-10T00%3A00%3A00.000Z&createdBefore=2019-12-03T00%3A00%3A00.000Z&showOwnPrivate=true'
    );

    const partialPage: PaginationOptions = { page: 1, size: null };
    const partialFilters: FilterOptions = {
        ownerId: null,
        book: ['test'],
        createdAfter: new Date(Date.UTC(2012, 3, 10)),
        createdBefore: null,
        showOwnPrivate: null
    };

    const partialParams = getPaginationAndFilterParams(partialPage, partialFilters);
    expect(partialParams).equals('page=1&book=test&createdAfter=2012-04-10T00%3A00%3A00.000Z');
});
