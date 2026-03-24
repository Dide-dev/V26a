/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

import { render } from '@testing-library/svelte';
import { expect, test } from 'vitest';
import NodeViewer from './NodeViewer.svelte';
import type { RootNode } from './example_tree';

test('NodeViewer', async () => {
    const node: RootNode = { label: 'Root', children: [] };
    render(NodeViewer, { node });

    expect(document.getElementsByClassName('h-fit')).toHaveLength(1); // Count the number of cards
});

test('NodeViewer with child', async () => {
    const node: RootNode = { label: 'Root', children: [{ card: 'bla', children: [] }] };
    render(NodeViewer, { node });

    expect(document.getElementsByClassName('h-fit')).toHaveLength(2); // Count the number of cards
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
});
