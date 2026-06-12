/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

import { apiCallPut } from './index';

const prefix = 'cards';

function apiCallPutCard(path: string, body: unknown) {
    return apiCallPut(prefix + path, body);
}

export type CardId = number;

export type CardData = {
    front: string;
    back: string;
};

/** Update an existing card in the database */
export function updateCard(cardId: CardId, card: CardData): Promise<void> {
    return apiCallPutCard(`/${cardId}`, card);
}
