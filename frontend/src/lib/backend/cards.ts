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
}

/** Add a card to the database */
export function addCard(card: CardAddData): Promise<CardId> {
    return apiCallPostCard('add', card);
}

/** Update an existing card in the database */
export function updateCard(card: CardUpdateData): Promise<void> {
    return apiCallPostCard('update', card);
}
