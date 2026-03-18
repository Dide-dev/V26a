import { render, screen, waitFor } from '@testing-library/svelte';
import { expect, test, vi } from 'vitest';
import type { MockedFunction } from 'vitest';
import PageComponent from './+page.svelte';
import type {
    RootNode,
    Node,
    TreeData,
    TreeGeneratedData,
    TreeStructure,
    TreeWithCards
} from '$lib/backend/trees';
import type { CardGetData, CardId } from '$lib/backend/cards';
import { getTreeWithCards } from '$lib/backend/trees';

function createTree(): TreeWithCards {
    // eslint-disable as CardId's are seen as object literal property, and should therefore follow camelCase.
    /* eslint-disable @typescript-eslint/naming-convention */
    const cards: Record<CardId, CardGetData> = {
        1: { front: 'front 1', back: 'back 1' },
        2: { front: 'front 2', back: 'back 2' }
    };
    /* eslint-enable @typescript-eslint/naming-convention */

    const treeData: TreeData = {
        title: 'Test Tree',
        description: 'Test Tree Description',
        bookName: 'Test book name',
        bookEdition: 'Test edition',
        bookIsbn: '12345679',
        bookChapter: 'Test chapter 4',
        bookExercise: 'Test excersise 2091',
        assignmentText: 'Test assignment text',
        published: true
    };

    const node1: Node = { cardId: 1, children: [] };
    const node2: Node = { cardId: 2, children: [] };
    const rootNode: RootNode = {
        label: 'Root',
        children: [node1, node2]
    };

    const treeStructure: TreeStructure = {
        rootNodes: [rootNode]
    };

    const treeGeneratedData: TreeGeneratedData = {
        id: 4,
        ownerId: 2098,
        createdAt: new Date('December 17, 1995 03:24:00'),
        lastEdited: new Date('December 17, 1995 13:56:34'),
        timesVisited: 5,
        lastVisited: new Date('December 17, 1995 15:09:11')
    };

    const treeWithCards: TreeWithCards = {
        data: treeData,
        genData: treeGeneratedData,
        structure: treeStructure,
        cards: cards
    };

    return treeWithCards;
}

vi.mock('$lib/backend/trees', () => {
    return { getTreeWithCards: vi.fn() };
});

test('Page succesfully loads tree', async () => {
    const tree: TreeWithCards = createTree();

    const mockGetTree = getTreeWithCards as MockedFunction<typeof getTreeWithCards>;
    mockGetTree.mockResolvedValue(tree);
    Object.defineProperty(window, 'location', {
        writable: true,
        value: { search: '?id=4' }
    });
    render(PageComponent);

    await waitFor(() => expect(screen.findByText(tree.data.title)).toBeTruthy());

    const description = tree.data.description as string;
    await waitFor(() => expect(screen.getByText(description)).toBeTruthy());

    await waitFor(() => expect(screen.getByText(tree.cards[1].front)).toBeTruthy());
    await waitFor(() => expect(screen.getByText(tree.cards[2].front)).toBeTruthy());
});

test('No tree-id provided is handled correctly', async () => {
    const tree: TreeWithCards = createTree();

    const mockGetTree = getTreeWithCards as MockedFunction<typeof getTreeWithCards>;
    mockGetTree.mockResolvedValue(tree);
    Object.defineProperty(window, 'location', {
        writable: true,
        value: { search: ' ' }
    });
    render(PageComponent);

    await waitFor(() => expect(screen.findByText('Error: No tree id provided')).toBeTruthy());
});

test('Invalid tree-id is handled correctly', async () => {
    const tree: TreeWithCards = createTree();

    const mockGetTree = getTreeWithCards as MockedFunction<typeof getTreeWithCards>;
    mockGetTree.mockResolvedValue(tree);
    Object.defineProperty(window, 'location', {
        writable: true,
        value: { search: '?id=abc' }
    });
    render(PageComponent);

    await waitFor(() => expect(screen.findByText('Error: Invalid tree id')).toBeTruthy());
});

test('No tree returned is handled correctly', async () => {
    Object.defineProperty(window, 'location', {
        writable: true,
        value: { search: '?id=abc' }
    });
    render(PageComponent);

    await waitFor(() => expect(screen.findByText('Error: Tree not found')).toBeTruthy());
});
