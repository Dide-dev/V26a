/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

import { expect, test } from 'vitest';
import { type FilterOptions, getPaginationAndFilterParams } from '$lib/api-wrapper/trees';
import type { PaginationOptions } from '$lib/pagination';

test('Pagination & Filter link generator', () => {
    const noPage: PaginationOptions = {
        page: null,
        size: null
    };
    const noFilters: FilterOptions = {
        ownerName: null,
        searchTerms: null,
        book: [],
        tag: [],
        languageCode: null,
        subject: null,
        createdAfter: null,
        createdBefore: null,
        showOwnPrivate: null,
        editable: null,
        showPrivate: null
    };

    const nothingParams = getPaginationAndFilterParams(noPage, noFilters);
    expect(nothingParams).equals('');

    const fullPage: PaginationOptions = { page: 1, size: 50 };
    const fullFilters: FilterOptions = {
        ownerName: 'Bob',
        searchTerms: 'mooi touwtje',
        book: ['test', 'test2'],
        tag: ['tag1', 'tag2'],
        languageCode: 'nl',
        subject: 'Wiskunde',
        createdAfter: new Date(Date.UTC(2012, 3, 10)),
        createdBefore: new Date(Date.UTC(2019, 11, 3)),
        showOwnPrivate: true,
        editable: true,
        showPrivate: false
    };

    const fullParams = getPaginationAndFilterParams(fullPage, fullFilters);
    expect(fullParams).equals(
        'page=1&size=50&ownerName=Bob&searchTerms=mooi+touwtje&book=test&book=test2&tag=tag1&tag=tag2&languageCode=nl&subject=Wiskunde&createdAfter=2012-04-10T00%3A00%3A00.000Z&createdBefore=2019-12-03T00%3A00%3A00.000Z&showOwnPrivate=true&editable=true&showPrivate=false'
    );

    const partialPage: PaginationOptions = { page: 1, size: null };
    const partialFilters: FilterOptions = {
        ownerName: null,
        searchTerms: null,
        book: [],
        tag: ['tag'],
        languageCode: null,
        subject: null,
        createdAfter: new Date(Date.UTC(2012, 3, 10)),
        createdBefore: null,
        showOwnPrivate: null,
        editable: null,
        showPrivate: true
    };

    const partialParams = getPaginationAndFilterParams(partialPage, partialFilters);
    expect(partialParams).equals(
        'page=1&tag=tag&createdAfter=2012-04-10T00%3A00%3A00.000Z&showPrivate=true'
    );
});
