/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

import { render, screen } from '@testing-library/svelte';
import { afterEach, expect, type MockedFunction, test, vi } from 'vitest';
import NodeViewer from './NodeViewer.svelte';
import {
    type RootNode,
    type Node,
    type TreeWithCards,
    updateTreeStructure
} from '$lib/backend/trees';
import { addCard, type CardGetData, type CardId } from '$lib/backend/cards';
import { userEvent } from '@testing-library/user-event/dist/cjs/index.js';

function renderTree(): TreeWithCards {
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
    render(NodeViewer, { tree, parent: rootNode, node: rootNode, editingPage });
    return tree;
}

vi.mock('$lib/backend/cards', () => {
    return {
        addCard: vi.fn()
    };
});
vi.mock('$lib/backend/trees', () => {
    return {
        updateTreeStructure: vi.fn()
    };
});
afterEach(() => {
    vi.resetAllMocks();
});

test('NodeViewer with correct amount of add/insert buttons', async () => {
    renderTree();
    expect(document.getElementsByClassName('card__add_button')).toHaveLength(3); // Count the number of add buttons
    expect(document.getElementsByClassName('card__insert_button')).toHaveLength(2); // Count the number of insert buttons
});

test('Add card successfully', async () => {
    const user = userEvent.setup();
    const tree = renderTree();
    (addCard as MockedFunction<typeof addCard>).mockResolvedValue(3);
    (updateTreeStructure as MockedFunction<typeof updateTreeStructure>).mockResolvedValue();

    await screen.findByText('Root');
    expect(tree.structure.rootNodes[0].children.length === 2);

    await user.click(document.getElementsByClassName('card__add_button')[0]);
    await screen.findByText('Add');
    const backTextBox = document.getElementsByClassName(
        'card__back_text_box'
    )[0] as HTMLTextAreaElement;
    await user.type(backTextBox, 'Kevin was here');
    await user.click(document.getElementsByClassName('card__confirm_button')[0]);
    expect(tree.structure.rootNodes[0].children.length === 3);
});

test('Insert card successfully', async () => {
    const user = userEvent.setup();
    const tree = renderTree();
    (addCard as MockedFunction<typeof addCard>).mockResolvedValue(3);
    (updateTreeStructure as MockedFunction<typeof updateTreeStructure>).mockResolvedValue();

    await screen.findByText('foo');
    expect(tree.structure.rootNodes[0].children[0].children.length === 0);

    await user.click(document.getElementsByClassName('card__insert_button')[0]);
    await screen.findByText('Add');
    const backTextBox = document.getElementsByClassName('card__back_text_box')[0];
    await user.type(backTextBox, 'Kevin was here');
    await user.click(document.getElementsByClassName('card__confirm_button')[0]);
    expect(tree.structure.rootNodes[0].children[0].children.length === 1);
});
