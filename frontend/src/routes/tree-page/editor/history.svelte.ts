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
    updateTreeStructure,
    type RootNode
} from '$lib/backend/trees';

export type TreeHistory = {
    // Used for the API requests
    treeId: TreeId;

    // The cardId's in these two properties may not exist anymore!
    listBack: [TreeStructure, CardAction][];
    listForward: [TreeStructure, CardAction][];

    // The cardId's in these two properties must always exist in the database
    currentStructure: TreeStructure;
    currentCards: Record<CardId, CardGetData>;

    cardIdMapper: CardIdMapper;
};

export function initializeHistory(
    treeId: TreeId,
    structure: TreeStructure,
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
function commitActionLocal(history: TreeHistory, newStructure: TreeStructure, action: CardAction) {
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
        default: {
            // Currently this is unreachable, but don't fail silently in the future if we add another type.
            throw new Error('Unknown card action type');
        }
    }
}

export async function doMove(history: TreeHistory, nodeToMove: number[], newLocation: number[]) {
    const newStructure: TreeStructure = $state.snapshot(history.currentStructure);

    const node = followIndexNode(newStructure, nodeToMove);

    const parent = followIndex(newStructure, nodeToMove.slice(0, -1))!;
    const parentIndex = nodeToMove[nodeToMove.length - 1];
    const placementParent = followIndex(newStructure, newLocation.slice(0, -1))!;
    const placementIndex = newLocation[newLocation.length - 1];

    parent.children.splice(parentIndex, 1);
    placementParent.children.splice(placementIndex, 0, node);

    await updateTreeStructure({
        id: history.treeId,
        newStructure
    });

    commitActionLocal(history, newStructure, { type: 'none', sendStructureToBackend: true });
}

export async function doAddCard(history: TreeHistory, location: number[], card: CardGetData) {
    const cardId = await addCard({
        treeId: history.treeId,
        front: card.front,
        back: card.back
    });

    const newStructure: TreeStructure = $state.snapshot(history.currentStructure);
    const parent = followIndex(newStructure, location.slice(0, -1))!;
    parent.children.splice(location[location.length - 1], 0, {
        cardId,
        children: []
    });

    await updateTreeStructure({
        id: history.treeId,
        newStructure
    });

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
    const newStructure: TreeStructure = $state.snapshot(history.currentStructure);
    const action: CardActionUpdate = {
        type: 'cardUpdate',
        cardId,
        newCard,
        oldCard
    };

    commitActionLocal(history, newStructure, action);
}

export async function doDeleteCard(history: TreeHistory, index: number[]) {
    const newStructure: TreeStructure = $state.snapshot(history.currentStructure);
    const card = followIndexNode(newStructure, index);

    // Perform actual structural removal locally here
    const parent = followIndex(newStructure, index.slice(0, -1))!;
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

export async function doRootNodeMove(history: TreeHistory, index: number, newIndex: number) {
    const newStructure: TreeStructure = $state.snapshot(history.currentStructure);
    const rootnode = newStructure.rootNodes[index];

    newStructure.rootNodes.splice(index, 1);
    newStructure.rootNodes.splice(newIndex, 0, rootnode);

    await updateTreeStructure({
        id: history.treeId,
        newStructure
    });

    commitActionLocal(history, newStructure, { type: 'none', sendStructureToBackend: true });
}

export async function doRootNodeEdit(history: TreeHistory, index: number, newLabel: string) {
    const newStructure: TreeStructure = $state.snapshot(history.currentStructure);
    newStructure.rootNodes[index].label = newLabel;

    await updateTreeStructure({
        id: history.treeId,
        newStructure
    });

    commitActionLocal(history, newStructure, { type: 'none', sendStructureToBackend: true });
}

export async function doRootNodeDelete(history: TreeHistory, index: number) {
    const newStructure: TreeStructure = $state.snapshot(history.currentStructure);

    // This should NEVER happen, but just in case a sanity check
    if (newStructure.rootNodes[index].children.length !== 0) {
        throw new Error('Tried to doRootNodeRemove on rootnode with children');
    }

    newStructure.rootNodes.splice(index, 1);

    await updateTreeStructure({
        id: history.treeId,
        newStructure
    });

    commitActionLocal(history, newStructure, { type: 'none', sendStructureToBackend: true });
}

export async function doRootNodeAdd(history: TreeHistory, index: number, label: string) {
    const newStructure: TreeStructure = $state.snapshot(history.currentStructure);

    newStructure.rootNodes.splice(index, 0, { label, children: [] });

    await updateTreeStructure({
        id: history.treeId,
        newStructure
    });

    commitActionLocal(history, newStructure, { type: 'none', sendStructureToBackend: true });
}

export function canUndo(history: TreeHistory): boolean {
    return history.listBack.length > 0;
}
export function canRedo(history: TreeHistory): boolean {
    return history.listForward.length > 0;
}

/**
 * Makes api calls to backend to perform the action and update the structure.
 * Will also make sure to map the CardId in the structure and action, and returns the updated versions.
 * Only local state that this function changes is the cardIdMapper.
 */
async function performActionToBackend(
    history: TreeHistory,
    newStructure: TreeStructure,
    action: CardAction
): Promise<[TreeStructure, CardAction]> {
    action = mapActionCardId(history, action);
    newStructure = mapStructureCardIds(history, newStructure);

    switch (action.type) {
        case 'none': {
            if (action.sendStructureToBackend) {
                await updateTreeStructure({
                    id: history.treeId,
                    newStructure
                });
            }
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

            await updateTreeStructure({
                id: history.treeId,
                newStructure
            });
            // If the above api call fails the database and local state will both be consistent,
            // there will just be an unused card in the database.

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
        default: {
            // Currently this is unreachable, but don't fail silently in the future if we add another type.
            throw new Error('Unknown card action type');
        }
    }
    return [newStructure, action];
}

export async function undo(history: TreeHistory) {
    if (!canUndo(history)) return;

    const entry = history.listBack[history.listBack.length - 1];
    const [oldStructure, actionInversed] = await performActionToBackend(
        history,
        entry[0],
        actionInverse(entry[1])
    );

    // The action is inversed twice so it is like the original again.
    history.listForward.push([history.currentStructure, actionInverse(actionInversed)]);
    history.listBack.pop();
    history.currentStructure = oldStructure;

    performActionLocal(history, actionInversed);
}
export async function redo(history: TreeHistory) {
    if (!canRedo(history)) return;

    const entry = history.listForward[history.listForward.length - 1];
    const [newStructure, action] = await performActionToBackend(history, entry[0], entry[1]);

    history.listBack.push([history.currentStructure, action]);
    history.listForward.pop();
    history.currentStructure = newStructure;

    performActionLocal(history, action);
}

/** Maps every CardId according to history.#cardIdMapper, by creating a deep copy. */
function mapStructureCardIds(history: TreeHistory, structure: TreeStructure): TreeStructure {
    const mapNode = (node: Node): Node => {
        return {
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

/**
 * Maps the CardId according to history.#cardIdMapper, if this action has a CardId.
 * The action will be modified in place, and a reference to the same object is also returned.
 */
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
        default: {
            // Currently this is unreachable, but don't fail silently in the future if we add another type.
            throw new Error('Unknown card action type');
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

/** These actions only describe what happens to the cards, not to the structure. */
type CardAction = CardActionNone | CardActionRemove | CardActionAdd | CardActionUpdate;
/**
 * This action specifies that only the tree structure is edited.
 * This is in contrast to the other actions which also edit the set of cards in some way.
 */
type CardActionNone = {
    type: 'none';
    /**
     * If the `revealed` property changes, there is no need to send the structure to the backend with undo/redo.
     * But if a card was moved, we do need to inform the backend.
     * This field indicates which of these two cases we are in.
     */
    sendStructureToBackend: boolean;
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
        default: {
            // Currently this is unreachable, but don't fail silently in the future if we add another type.
            throw new Error('Unknown card action type');
        }
    }
}

/**
 * Returns reference to a RootNode or Node, found by following index array in the given tree.
 * Could also return null if the index array is invalid.
 */
export function followIndex(treeStructure: TreeStructure, index: number[]): RootNode | Node | null {
    let node: RootNode | Node | null = treeStructure.rootNodes[index[0]];
    for (let i = 1; i < index.length; i++) {
        node = node.children[index[i]];

        if (node === undefined) {
            return null;
        }
    }
    return node;
}
/**
 * Returns reference to a Node, found by following index array in the given tree.
 * Will assert that the index refers to a Node, not a RootNode.
 */
export function followIndexNode(treeStructure: TreeStructure, index: number[]): Node {
    const node = followIndex(treeStructure, index);
    if (node === null || !('cardId' in node)) {
        // This should NEVER happen, but we have to check for typescript
        throw new Error(
            'followIndexNode was called with index referencing RootNode or non-existing node'
        );
    }
    return node;
}
