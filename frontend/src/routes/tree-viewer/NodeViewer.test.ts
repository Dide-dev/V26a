import { render } from '@testing-library/svelte';
import { expect, test } from 'vitest';
import NodeViewer from './NodeViewer.svelte';
import type { RootNode } from './example_tree';

test('NodeViewer', async () => {
    const node: RootNode = { label: 'Root', children: [] };
    render(NodeViewer, { node });

    expect(document.getElementsByClassName('h-fit')).toHaveLength(1); // Count the number of cards
    expect(document.getElementsByTagName('line')).toHaveLength(0);
});

test('NodeViewer with child', async () => {
    const node: RootNode = { label: 'Root', children: [{ card: 'bla', children: [] }] };
    render(NodeViewer, { node });

    expect(document.getElementsByClassName('h-fit')).toHaveLength(2); // Count the number of cards
    expect(document.getElementsByTagName('line')).toHaveLength(3);
});

test('NodeViewer with children', async () => {
    const node: RootNode = {
        label: 'Root',
        children: [
            { card: 'foo', children: [] },
            { card: 'bar', children: [] }
        ]
    };
    render(NodeViewer, { node });

    expect(document.getElementsByClassName('h-fit')).toHaveLength(3); // Count the number of cards
    expect(document.getElementsByTagName('line')).toHaveLength(4);
});
