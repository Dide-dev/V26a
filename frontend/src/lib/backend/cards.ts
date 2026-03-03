import { apiCallPost, apiCallGet } from './index';

const prefix = 'cards/';

function apiCallPostCard(path: string, body: unknown) {
    return apiCallPost(prefix + path, body);
}

// TODO: remove next line when the function is used
// eslint-disable-next-line
function apiCallGetCard(path: string) {
    return apiCallGet(prefix + path);
}

/** The basic data of the card, including the tree it belongs to. */
type CardData = {
    treeId: number,
    front: string,
    back: string
};

type CardId = number;

/** Add a card to the database */
export function addCard(card: CardData): Promise<CardId> {
    return apiCallPostCard('add', card);
}
