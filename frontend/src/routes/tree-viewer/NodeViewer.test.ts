/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

import { render } from '@testing-library/svelte';
import { expect, test } from 'vitest';
import NodeViewer from './NodeViewer.svelte';
import type { RootNode, Node, TreeWithCards } from '$lib/backend/trees';
import type { CardGetData, CardId } from '$lib/backend/cards';

test('NodeViewer', async () => {
    const rootNode: RootNode = { label: 'Root', children: [] };
    const cards: Record<CardId, CardGetData> = {};
    const tree: TreeWithCards = {
        data: {
            title: 'Test',
            description: null,
            bookName: null,
            bookEdition: null,
            bookIsbn: null,
            bookChapter: null,
            bookExercise: null,
            assignmentText: null,
            published: true
        },
        genData: {
            id: 0,
            ownerId: 0,
            createdAt: new Date(),
            lastModified: new Date(),
            timesVisited: 0,
            lastVisited: null
        },
        structure: {
            rootNodes: [rootNode]
        },
        cards: cards
    };
    const editingPage = false;
    render(NodeViewer, { tree, node:rootNode, editingPage });

    expect(document.getElementsByClassName('tree__node')).toHaveLength(1); // Count the number of cards
});

test('NodeViewer with child', async () => {
    // eslint-disable as CardId's are seen as object literal property, and should therefore follow camelCase.
    /* eslint-disable @typescript-eslint/naming-convention */
    const cards: Record<CardId, CardGetData> = {
        1: { front: 'foo', back: 'bar' }
    };
    /* eslint-enable @typescript-eslint/naming-convention */
    const node: Node = { cardId: 1, children: [] };
    const rootNode: RootNode = { label: 'Root', children: [node] };
    const tree: TreeWithCards = {
        data: {
            title: 'Test',
            description: null,
            bookName: null,
            bookEdition: null,
            bookIsbn: null,
            bookChapter: null,
            bookExercise: null,
            assignmentText: null,
            published: true
        },
        genData: {
            id: 0,
            ownerId: 0,
            createdAt: new Date(),
            lastModified: new Date(),
            timesVisited: 0,
            lastVisited: null
        },
        structure: {
            rootNodes: [rootNode]
        },
        cards: cards
    };
    const editingPage = false;

    render(NodeViewer, { tree, node:rootNode, editingPage });

    expect(document.getElementsByClassName('tree__node')).toHaveLength(2); // Count the number of cards
});

test('NodeViewer with children', async () => {
    // eslint-disable as CardId's are seen as object literal property, and should therefore follow camelCase.
    /* eslint-disable @typescript-eslint/naming-convention */
    const cards: Record<CardId, CardGetData> = {
        1: { front: 'foo', back: 'bar' },
        2: { front: 'baz', back: 'quux' }
    };
    /* eslint-enable @typescript-eslint/naming-convention */
    const node1: Node = { cardId: 1, children: [] };
    const node2: Node = { cardId: 2, children: [] };
    const rootNode: RootNode = {
        label: 'Root',
        children: [node1, node2]
    };
    const tree: TreeWithCards = {
        data: {
            title: 'Test',
            description: null,
            bookName: null,
            bookEdition: null,
            bookIsbn: null,
            bookChapter: null,
            bookExercise: null,
            assignmentText: null,
            published: true
        },
        genData: {
            id: 0,
            ownerId: 0,
            createdAt: new Date(),
            lastModified: new Date(),
            timesVisited: 0,
            lastVisited: null
        },
        structure: {
            rootNodes: [rootNode]
        },
        cards: cards
    };
    const editingPage = false;
    render(NodeViewer, { tree, node:rootNode, editingPage });

    expect(document.getElementsByClassName('tree__node')).toHaveLength(3); // Count the number of cards
});

test('NodeViewer with children and add/insert buttons', async () => {
    // eslint-disable as CardId's are seen as object literal property, and should therefore follow camelCase.
    /* eslint-disable @typescript-eslint/naming-convention */
    const cards: Record<CardId, CardGetData> = {
        1: { front: 'foo', back: 'bar' },
        2: { front: 'baz', back: 'quux' }
    };
    /* eslint-enable @typescript-eslint/naming-convention */
    const node1: Node = { cardId: 1, children: [] };
    const node2: Node = { cardId: 2, children: [] };
    const rootNode: RootNode = {
        label: 'Root',
        children: [node1, node2]
    };
    const tree: TreeWithCards = {
        data: {
            title: 'Test',
            description: null,
            bookName: null,
            bookEdition: null,
            bookIsbn: null,
            bookChapter: null,
            bookExercise: null,
            assignmentText: null,
            published: true
        },
        genData: {
            id: 0,
            ownerId: 0,
            createdAt: new Date(),
            lastModified: new Date(),
            timesVisited: 0,
            lastVisited: null
        },
        structure: {
            rootNodes: [rootNode]
        },
        cards: cards
    };
    const editingPage = true;
    render(NodeViewer, { tree, node: rootNode, editingPage });

    expect(document.getElementsByClassName('card__add_button')).toHaveLength(3); // Count the number of add buttons
    expect(document.getElementsByClassName('card__insert_button')).toHaveLength(2); // Count the number of insert buttons
});
