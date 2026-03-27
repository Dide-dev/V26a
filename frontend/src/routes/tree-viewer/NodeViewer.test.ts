/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

import { render } from '@testing-library/svelte';
import { expect, test, vi } from 'vitest';
import NodeViewer from './NodeViewer.svelte';
import type { RootNode, Node } from '$lib/backend/trees';
import type { CardGetData, CardId } from '$lib/backend/cards';
import type { UserId } from '$lib/backend/users';
import TreePage from './+page.svelte';

test('NodeViewer', async () => {
    const node: RootNode = { label: 'Root', children: [] };
    const cards: Record<CardId, CardGetData> = {};
    render(NodeViewer, { node, cards });

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
    render(NodeViewer, { node: rootNode, cards });

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
    const node: RootNode = {
        label: 'Root',
        children: [node1, node2]
    };
    render(NodeViewer, { node, cards });

    expect(document.getElementsByClassName('tree__node')).toHaveLength(3); // Count the number of cards
});

test('shows Edit button when user is logged in', async () => {
    render(TreePage, { currentUser: 12 });

    await new Promise((r) => setTimeout(r));

     expect(document.getElementsByClassName('edit_button').length).toBeGreaterThan(0);
});

test('does not show Edit button when user is not logged in', async () => {
    render(TreePage, { currentUser: null });

    await new Promise((r) => setTimeout(r)); 

    expect(document.getElementsByClassName('edit_button')).toHaveLength(0);
});
