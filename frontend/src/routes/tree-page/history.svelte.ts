/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

import { addCard, updateCard, type CardGetData, type CardId } from '$lib/backend/cards';
import {
    type TreeStructure,
    type Node,
    type TreeId,
    removeCardFromTree,
    updateTreeStructure
} from '$lib/backend/trees';

export type TreeHistory = {
    // Used for the API requests
    treeId: TreeId;

    // The cardId's in these two properties may not exist anymore!
    listBack: [StateTreeStructure, CardAction][];
    listForward: [StateTreeStructure, CardAction][];

    // The cardId's in these two properties must always exist in the database
    currentStructure: StateTreeStructure;
    currentCards: Record<CardId, CardGetData>;

    cardIdMapper: CardIdMapper;
};

export function initializeHistory(
    treeId: TreeId,
    structure: StateTreeStructure,
    cards: Record<CardId, CardGetData>
): TreeHistory {
    return {
        treeId,
        currentStructure: structure,
        currentCards: cards,
        listBack: [],
        listForward: [],
        cardIdMapper: new CardIdMapper()
    };
}

/** Performs action and structure update, and records it. */
function commitActionLocal(
    history: TreeHistory,
    newStructure: StateTreeStructure,
    action: CardAction
) {
    history.listForward = [];
    history.listBack.push([history.currentStructure, action]);
    history.currentStructure = newStructure;
    performActionLocal(history, action);
}

/** Only performs the action locally on the cardList, doesn't record anything.  */
function performActionLocal(history: TreeHistory, action: CardAction) {
    switch (action.type) {
        case 'none': {
            break;
        }
        case 'cardRemove': {
            delete history.currentCards[action.oldCardId];
            break;
        }
        case 'cardAdd': {
            history.currentCards[action.cardId] = action.card;
            break;
        }
        case 'cardUpdate': {
            history.currentCards[action.cardId] = action.newCard;
            break;
        }
    }
}

// TODO: I don't think we need to keep track of this in the history, right?
export function setRevealed(history: TreeHistory, index: number[], revealed: boolean) {
    const node = followIndexNode(history.currentStructure, index);
    node.revealed = revealed;
}

export async function doMove(history: TreeHistory, nodeToMove: number[], newLocation: number[]) {
    const newStructure: StateTreeStructure = $state.snapshot(history.currentStructure);

    const node = followIndexNode(newStructure, nodeToMove);

    const parent = followIndex(newStructure, nodeToMove.slice(0, -1));
    const parentIndex = nodeToMove[nodeToMove.length - 1];
    const placementParent = followIndex(newStructure, newLocation.slice(0, -1));
    const placementIndex = newLocation[newLocation.length - 1];

    parent.children.splice(parentIndex, 1);
    placementParent.children.splice(placementIndex, 0, node);

    await updateTreeStructure({ id: history.treeId, newStructure });

    commitActionLocal(history, newStructure, { type: 'none' });
}

export async function doAddCard(history: TreeHistory, location: number[], card: CardGetData) {
    const cardId = await addCard({
        treeId: history.treeId,
        front: card.front,
        back: card.back
    });

    const newStructure: StateTreeStructure = $state.snapshot(history.currentStructure);
    const parent = followIndex(newStructure, location.slice(0, -1));
    parent.children.splice(location[location.length - 1], 0, {
        revealed: true,
        cardId,
        children: []
    });

    await updateTreeStructure({ id: history.treeId, newStructure });

    const action: CardActionAdd = {
        type: 'cardAdd',
        cardId,
        card
    };
    commitActionLocal(history, newStructure, action);
}

export async function doEditCard(history: TreeHistory, cardId: CardId, newCard: CardGetData) {
    const oldCard = history.currentCards[cardId];

    await updateCard({ cardId, newBack: newCard.back, newFront: newCard.front });
    const newStructure: StateTreeStructure = $state.snapshot(history.currentStructure);
    const action: CardActionUpdate = {
        type: 'cardUpdate',
        cardId,
        newCard,
        oldCard
    };

    commitActionLocal(history, newStructure, action);
}

export async function doDeleteCard(history: TreeHistory, index: number[]) {
    const newStructure: StateTreeStructure = $state.snapshot(history.currentStructure);
    const card = followIndexNode(newStructure, index);

    // Perform actual structural removal locally here
    const parent = followIndex(newStructure, index.slice(0, -1));
    const parentIndex = index[index.length - 1];
    parent.children.splice(parentIndex, 1, ...card.children);

    await removeCardFromTree({
        treeId: history.treeId,
        cardId: card.cardId,
        newStructure
    });

    // Only now that the backend has accepted, commit locally
    const action: CardActionRemove = {
        type: 'cardRemove',
        oldCardId: card.cardId,
        oldCard: history.currentCards[card.cardId]
    };
    commitActionLocal(history, newStructure, action);
}

export function canUndo(history: TreeHistory): boolean {
    return history.listBack.length > 0;
}
export function canRedo(history: TreeHistory): boolean {
    return history.listForward.length > 0;
}

export async function undo(history: TreeHistory) {
    if (!canUndo(history)) return;

    const entry = history.listBack[history.listBack.length - 1];
    let action = mapActionCardId(history, entry[1]);
    let oldStructure = mapStructureCardIds(history, entry[0]);

    switch (action.type) {
        case 'none': {
            await updateTreeStructure({ id: history.treeId, newStructure: oldStructure });
            break;
        }
        case 'cardAdd': {
            await removeCardFromTree({
                treeId: history.treeId,
                cardId: action.cardId,
                newStructure: oldStructure
            });
            break;
        }
        case 'cardRemove': {
            const newCardId = await addCard({
                treeId: history.treeId,
                front: action.oldCard.front,
                back: action.oldCard.back
            });

            history.cardIdMapper.setMapping(action.oldCardId, newCardId);

            oldStructure = mapStructureCardIds(history, oldStructure);
            action = mapActionCardId(history, action);

            await updateTreeStructure({ id: history.treeId, newStructure: oldStructure });

            // TODO: if the above api call fails the local state is semi committed with the cardIdMapper. Is this fine?
            break;
        }
        case 'cardUpdate': {
            await updateCard({
                cardId: action.cardId,
                newFront: action.oldCard.front,
                newBack: action.oldCard.back
            });
            break;
        }
    }

    history.listForward.push([history.currentStructure, action]);
    history.listBack.pop();
    history.currentStructure = oldStructure;

    performActionLocal(history, actionInverse(action));
}
export async function redo(history: TreeHistory) {
    if (!canRedo(history)) return;

    const entry = history.listForward[history.listForward.length - 1];
    let action = mapActionCardId(history, entry[1]);
    let newStructure = mapStructureCardIds(history, entry[0]);

    // Here is an implemenation to do the inverse of #performActionLocal.
    switch (action.type) {
        case 'none': {
            await updateTreeStructure({ id: history.treeId, newStructure });
            break;
        }
        case 'cardAdd': {
            const newCardId = await addCard({
                treeId: history.treeId,
                front: action.card.front,
                back: action.card.back
            });

            history.cardIdMapper.setMapping(action.cardId, newCardId);

            newStructure = mapStructureCardIds(history, newStructure);
            action = mapActionCardId(history, action);

            await updateTreeStructure({ id: history.treeId, newStructure });

            // TODO: if the above api call fails the local state is semi committed with the cardIdMapper. Is this fine?
            break;
        }
        case 'cardRemove': {
            await removeCardFromTree({
                treeId: history.treeId,
                cardId: action.oldCardId,
                newStructure
            });
            break;
        }
        case 'cardUpdate': {
            await updateCard({
                cardId: action.cardId,
                newFront: action.newCard.front,
                newBack: action.newCard.back
            });
            break;
        }
    }

    history.listBack.push([history.currentStructure, action]);
    history.listForward.pop();
    history.currentStructure = newStructure;

    performActionLocal(history, action);
}

/** Maps every CardId according to this.#cardIdMapper, by creating a deep copy. */
function mapStructureCardIds(
    history: TreeHistory,
    structure: StateTreeStructure
): StateTreeStructure {
    const mapNode = (node: StateNode): StateNode => {
        return {
            revealed: node.revealed,
            cardId: history.cardIdMapper.find(node.cardId),
            children: node.children.map(mapNode)
        };
    };

    return {
        rootNodes: structure.rootNodes.map((rootNode) => ({
            label: rootNode.label,
            children: rootNode.children.map(mapNode)
        }))
    };
}

/** Maps the CardId according to this.#cardIdMapper, if this action has a CardId. */
function mapActionCardId(history: TreeHistory, action: CardAction): CardAction {
    switch (action.type) {
        case 'none': {
            return action;
        }
        case 'cardRemove': {
            action.oldCardId = history.cardIdMapper.find(action.oldCardId);
            return action;
        }
        case 'cardAdd':
        case 'cardUpdate': {
            action.cardId = history.cardIdMapper.find(action.cardId);
            return action;
        }
    }
}

class CardIdMapper {
    #mapping: Record<CardId, CardId> = {};

    /** Resolves a potentially outdated CardId to the current CardId. */
    find(cardId: CardId): CardId {
        const value = this.#mapping[cardId];
        if (value === undefined) {
            // Is not present in the mapping, so it has not changed
            return cardId;
        }

        // It is possible that we have to do multiple lookups if a card is deleted & recreated multiple times.
        return (this.#mapping[cardId] = this.find(value));
    }

    /**
     * Configures an old cardId to map to a new one.
     * Will make sure that any even older cardIds pointing to the given oldCardId are redirected to the newValue.
     */
    setMapping(oldCardId: CardId, newValue: CardId) {
        this.#mapping[oldCardId] = newValue;
    }
}

/** These actions only describe what happend to the cards, not to the structure. */
type CardAction = CardActionNone | CardActionRemove | CardActionAdd | CardActionUpdate;
/**
 * Actually with all the other Actions the structure also gets edited,
 * it is just that with this action type nothing else changes.
 */
type CardActionNone = {
    type: 'none';
};
type CardActionRemove = {
    type: 'cardRemove';
    oldCardId: CardId;
    oldCard: CardGetData;
};
type CardActionAdd = {
    type: 'cardAdd';
    cardId: CardId;
    card: CardGetData;
};
type CardActionUpdate = {
    type: 'cardUpdate';
    cardId: CardId;
    newCard: CardGetData;
    oldCard: CardGetData;
};

function actionInverse(action: CardAction): CardAction {
    switch (action.type) {
        case 'none': {
            return action;
        }
        case 'cardAdd': {
            return {
                type: 'cardRemove',
                oldCardId: action.cardId,
                oldCard: action.card
            };
        }
        case 'cardRemove': {
            return {
                type: 'cardAdd',
                cardId: action.oldCardId,
                card: action.oldCard
            };
        }
        case 'cardUpdate': {
            return {
                type: 'cardUpdate',
                cardId: action.cardId,
                newCard: action.oldCard,
                oldCard: action.newCard
            };
        }
    }
}

// Statefull variants of types of the same name in 'trees.ts'.
// For now the only state tracked is the 'revealed' property of Nodes, we could add more.
export type StateTreeStructure = {
    rootNodes: StateRootNode[];
};
export type StateRootNode = {
    label: string;
    children: StateNode[];
};
export type StateNode = {
    revealed: boolean;

    cardId: CardId;
    children: StateNode[];
};
/**
 * Makes a deep copy and fills in all state from StateTreeStructure
 * Currently fills the 'revealed' field with true everywhere.
 */
export function fillStateStructure(
    statelessStructure: TreeStructure,
    allRevealed: boolean
): StateTreeStructure {
    function fillStateNode(statelessNode: Node): StateNode {
        return {
            revealed: allRevealed,
            cardId: statelessNode.cardId,
            children: statelessNode.children.map(fillStateNode)
        };
    }

    return {
        rootNodes: statelessStructure.rootNodes.map((rootNode) => ({
            label: rootNode.label,
            children: rootNode.children.map(fillStateNode)
        }))
    };
}

/**
 * Returns reference to a RootNode or Node, found by following index array in the given tree.
 * Could also return undefined if the index array is invalid.
 */
export function followIndex(
    treeStructure: StateTreeStructure,
    index: number[]
): StateRootNode | StateNode {
    let node: StateRootNode | StateNode = treeStructure.rootNodes[index[0]];
    for (let i = 1; i < index.length; i++) {
        node = node.children[index[i]];
    }
    return node;
}
/**
 * Returns reference to a Node, found by following index array in the given tree.
 * Will assert that the index referes to a Node, not a RootNode.
 */
export function followIndexNode(treeStructure: StateTreeStructure, index: number[]): StateNode {
    const node = followIndex(treeStructure, index);
    if (!('cardId' in node)) {
        // This should NEVER happen, but we have to check for typescript
        throw new Error('followIndexNode was called with index referencing RootNode');
    }
    return node;
}
