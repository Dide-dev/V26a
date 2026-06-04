/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

import { apiCallPost, apiCallGet, apiCallPut } from './index';
import type { Option } from './types';
import { isNullOrEmpty } from './trees';

const prefix = 'cards';

function apiCallPostCard(path: string, body: unknown) {
    return apiCallPost(prefix + path, body);
}

function apiCallPutCard(path: string, body: unknown) {
    return apiCallPut(prefix + path, body);
}

function apiCallGetCard(path: string) {
    return apiCallGet(prefix + path);
}

export type CardId = number;
export type LibraryCardId = number;

export type CardData = {
    front: string;
    back: string;
};

export type LibraryCardAddData = {
    title: string;
    tags: string[];
};

export type LibraryCardGetData = {
    cardId: number;
    title: string;
    ownerId: number;
    createdAt: Date;
    tags: string[];
    front: string;
    back: string;
};

export type CardFilterOptions = {
    ownerId: Option<number>;
    createdAfter: Option<Date>;
    createdBefore: Option<Date>;
    tags: string[];
};

/** Update an existing card in the database */
export function updateCard(cardId: CardId, card: CardData): Promise<void> {
    return apiCallPutCard(`/${cardId}`, card);
}

/** Add an existing card to the cards library */
export function addCardToLibrary(card: LibraryCardAddData, cardId: CardId): Promise<LibraryCardId> {
    return apiCallPostCard(`/${cardId}`, card);
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
    return apiCallGetCard(`/library?${params}`);
}
