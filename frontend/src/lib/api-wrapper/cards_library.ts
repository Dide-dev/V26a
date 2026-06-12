/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

import { apiCallPost, apiCallGet } from './index';
import type { Option } from './types';
import { isNullOrEmpty } from './trees';
import type { Username } from '$lib/api-wrapper/users';

const prefix = 'library';

function apiCallPostCard(path: string, body: unknown) {
    return apiCallPost(prefix + path, body);
}

function apiCallGetCard(path: string) {
    return apiCallGet(prefix + path);
}

export type CardId = number;
export type LibraryCardId = number;

export type LibraryCardAddData = {
    title: string;
    tags: string[];
};

export type LibraryCardGetData = {
    cardId: number;
    title: string;
    ownerName: Username;
    createdAt: Date;
    tags: string[];
    front: string;
    back: string;
};

export type CardFilterOptions = {
    ownerName: Option<Username>;
    createdAfter: Option<Date>;
    createdBefore: Option<Date>;
    tags: string[];
};

/** Add an existing card to the cards library */
export function addCardToLibrary(card: LibraryCardAddData, cardId: CardId): Promise<LibraryCardId> {
    return apiCallPostCard(`/cards/${cardId}`, card);
}

export function getFilterParams(filterOptions: CardFilterOptions): string {
    const params = new URLSearchParams();

    Object.entries(filterOptions).forEach(([key, value]) => {
        if (!isNullOrEmpty(value)) {
            // For array values, all items need their own query parameter.
            if (Array.isArray(value)) {
                value.forEach((item) => params.append(key, String(item)));
            } else {
                // The JSON stringify is needed to get the dates in rfc3339 format instead of ISO format
                // because there is a different implementation of toJSON and toString
                params.append(key, JSON.parse(JSON.stringify(value)));
            }
        }
    });

    return String(params);
}

/** Get the list of all trees satisfying some filters at a given page. */
export function getFilteredList(filterOptions: CardFilterOptions): Promise<LibraryCardGetData[]> {
    const params: string = getFilterParams(filterOptions);
    return apiCallGetCard(`/cards?${params}`);
}
