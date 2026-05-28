/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

import { apiCallPost, apiCallGet } from './index';
import type { TreeId } from './trees';
import type { Option } from './types';
import { isNullOrEmpty } from './trees';

const prefix = 'cards/';

function apiCallPostCard(path: string, body: unknown) {
    return apiCallPost(prefix + path, body);
}

function apiCallGetCard(path: string) {
    return apiCallGet(prefix + path);
}

export type CardId = number;
export type LibraryCardId = number;

/** The basic data of the card, including the tree it belongs to. */
export type CardAddData = {
    treeId: TreeId;
    data: CardData;
};

/** The basic data required to update an existing card. */
export type CardUpdateData = {
    cardId: number;
    data: CardData;
};

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

/** Add a card to the database */
export function addCard(card: CardAddData): Promise<CardId> {
    return apiCallPostCard('add', card);
}

/** Update an existing card in the database */
export function updateCard(card: CardUpdateData): Promise<void> {
    return apiCallPostCard('update', card);
}

/** Add an existing card to the cards library */
export function addCardToLibrary(card: LibraryCardAddData, cardId: CardId): Promise<LibraryCardId> {
    return apiCallPostCard(`${cardId}/add-to-library`, card);
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
    return apiCallGetCard(`get-library?${params}`);
}
