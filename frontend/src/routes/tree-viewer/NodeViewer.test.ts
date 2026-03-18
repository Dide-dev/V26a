import { render } from '@testing-library/svelte';
import { expect, test } from 'vitest';
import NodeViewer from './NodeViewer.svelte';
import type { RootNode, Node } from '$lib/backend/trees';
import type { CardGetData, CardId } from '$lib/backend/cards';

test('NodeViewer', async () => {
    const node: RootNode = { label: 'Root', children: [] };
    // eslint-disable as CardId's are seen as object literal property, and should therefore follow camelCase.
    /* eslint-disable @typescript-eslint/naming-convention */
    const cards: Record<CardId, CardGetData> = {
        1: { front: 'foo', back: 'bar' },
        2: { front: 'baz', back: 'quux' }
    };
    /* eslint-enable @typescript-eslint/naming-convention */
    render(NodeViewer, { node, cards });

    expect(document.getElementsByClassName('h-fit')).toHaveLength(1); // Count the number of cards
});

test('NodeViewer with child', async () => {
    // eslint-disable as CardId's are seen as object literal property, and should therefore follow camelCase.
    /* eslint-disable @typescript-eslint/naming-convention */
    const cards: Record<CardId, CardGetData> = {
        1: { front: 'foo', back: 'bar' },
        2: { front: 'baz', back: 'quux' }
    };
    /* eslint-enable @typescript-eslint/naming-convention */
    const node: Node = { cardId: 1, children: [] };
    const rootNode: RootNode = { label: 'Root', children: [node] };
    render(NodeViewer, { node: rootNode, cards });

    expect(document.getElementsByClassName('h-fit')).toHaveLength(2); // Count the number of cards
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

    expect(document.getElementsByClassName('h-fit')).toHaveLength(3); // Count the number of cards
});
