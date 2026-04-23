/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

import { apiCallPost, apiCallGet } from './index';
import type { TreeId } from './trees';

const prefix = 'cards/';

function apiCallPostCard(path: string, body: unknown) {
    return apiCallPost(prefix + path, body);
}

// TODO: remove next line when the function is used
// eslint-disable-next-line
function apiCallGetCard(path: string) {
    return apiCallGet(prefix + path);
}

export type CardId = number;
export type LibraryCardId = number;

/** The basic data of the card, including the tree it belongs to. */
export type CardAddData = {
    treeId: TreeId;
    front: string;
    back: string;
};

/** The basic data required to update an existing card. */
export type CardUpdateData = {
    cardId: number;
    newFront: string;
    newBack: string;
};

export type CardGetData = {
    front: string;
    back: string;
};

export type LibraryCardAddData = {
    title: string;
    tags: string[];
}

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
    return apiCallPostCard(cardId + '/add-to-library', card);
}
