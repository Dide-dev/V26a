/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

import { render, screen } from '@testing-library/svelte';
import type { CardGetData, CardId } from '$lib/backend/cards';
import NodeViewer from './NodeViewer.svelte';
import type { Node, RootNode, TreeWithCards } from '$lib/backend/trees';
import { userEvent } from '@testing-library/user-event/dist/cjs/index.js';
import { expect, test } from 'vitest';

function renderTree(): TreeWithCards {
    /* eslint-disable @typescript-eslint/naming-convention */
    const cards: Record<CardId, CardGetData> = {
        1: { front: 'foo', back: 'bar' },
        2: { front: 'baz', back: 'quux' },
        3: { front: 'Visible', back: 'bar' },
        4: { front: 'Visible', back: 'quux' },
        5: { front: 'Visible', back: 'bar' },
        6: { front: 'Visible', back: 'quux' },
        7: { front: 'Extra Hidden', back: 'bar' },
        8: { front: 'Extra Hidden', back: 'quux' },
        9: { front: 'Extra Hidden', back: 'bar' },
        10: { front: 'Extra Hidden', back: 'quux' },
        11: { front: 'Extra Hidden', back: 'bar' },
        12: { front: 'Extra Hidden', back: 'quux' },
        13: { front: 'Hidden Gem', back: 'bar' },
        14: { front: 'Extra Hidden', back: 'quux' }
    };

    /* eslint-enable @typescript-eslint/naming-convention */
    const node7: Node = { cardId: 7, children: [] };
    const node8: Node = { cardId: 8, children: [] };
    const node9: Node = { cardId: 9, children: [] };
    const node10: Node = { cardId: 10, children: [] };
    const node11: Node = { cardId: 11, children: [] };
    const node12: Node = { cardId: 12, children: [] };
    const node13: Node = { cardId: 13, children: [] };
    const node14: Node = { cardId: 14, children: [] };
    const node3: Node = { cardId: 3, children: [node7, node8] };
    const node4: Node = { cardId: 4, children: [node9, node10] };
    const node5: Node = { cardId: 5, children: [node11, node12] };
    const node6: Node = { cardId: 6, children: [node13, node14] };
    const node1: Node = { cardId: 1, children: [node3, node4] };
    const node2: Node = { cardId: 2, children: [node5, node6] };

    /* Node structure
     *             / 7
     *           3 - 8
     *       1     / 9
     *     /     4 - 10
     * rt
     *     \     5 - 11
     *       2     \ 12
     *           6 - 13
     *             \ 14
     */

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
    render(NodeViewer, { tree, parent: rootNode, node: rootNode, editingPage: editingPage });
    return tree;
}

test('CardViewer viewer', async () => {
    const user = userEvent.setup();
    const tree = renderTree();

    expect(tree.structure.rootNodes[0].children.length === 14);
    expect((await screen.findAllByText('Visible')).length === 0);
    expect((await screen.findAllByText('Extra Hidden')).length === 0);
    expect(await screen.findByText('foo'));
    await user.click(await screen.findByText('foo'));
    expect((await screen.findAllByText('Visible')).length === 2);
    await user.click(await screen.findByText('bar'));
    await user.click((await screen.findAllByText('Visible'))[4]);
    expect((await screen.findAllByText('Visible')).length === 3);
    expect((await screen.findAllByText('Extra Hidden')).length === 1);
    expect(await screen.findByText('Hidden Gem'));
});
