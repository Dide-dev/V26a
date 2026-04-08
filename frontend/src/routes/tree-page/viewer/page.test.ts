import { render, screen } from '@testing-library/svelte';
import { expect, test, vi, afterEach } from 'vitest';
import type { MockedFunction } from 'vitest';
import PageComponent from './+page.svelte';
import { page } from '$app/state';
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
        bookExercise: 'Test exercise 2091',
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
        lastModified: new Date('December 17, 1995 13:56:34'),
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
afterEach(() => {
    vi.resetAllMocks();
});

test('Page successfully loads tree', async () => {
    const tree: TreeWithCards = createTree();

    const mockGetTree = getTreeWithCards as MockedFunction<typeof getTreeWithCards>;
    mockGetTree.mockResolvedValue(tree);
    page.url.searchParams.set('id', '4');
    render(PageComponent);

    await screen.findByText(tree.data.title);

    const description = tree.data.description as string;
    screen.getByText(description);

    screen.getByText(tree.cards[1].front);
    screen.getByText(tree.cards[2].front);
});

test('No tree id provided is handled correctly', async () => {
    const tree: TreeWithCards = createTree();

    const mockGetTree = getTreeWithCards as MockedFunction<typeof getTreeWithCards>;
    mockGetTree.mockResolvedValue(tree);
    page.url.searchParams.delete('id');
    render(PageComponent);

    const error = await screen.findByTestId('error');

    expect(error.textContent).toBe('Error: No tree id provided');
});

test('Invalid tree id is handled correctly', async () => {
    const tree: TreeWithCards = createTree();

    const mockGetTree = getTreeWithCards as MockedFunction<typeof getTreeWithCards>;
    mockGetTree.mockResolvedValue(tree);
    page.url.searchParams.set('id', 'abc');
    render(PageComponent);

    const error = await screen.findByTestId('error');

    expect(error.textContent).toBe('Error: Invalid tree id');
});

test('No tree returned is handled correctly', async () => {
    const mockGetTree = getTreeWithCards as MockedFunction<typeof getTreeWithCards>;
    mockGetTree.mockRejectedValue(new Error('The tree with id 5 could not be found.'));
    page.url.searchParams.set('id', '5');
    render(PageComponent);

    const error = await screen.findByTestId('error');

    expect(error.textContent).toBe('Error: The tree with id 5 could not be found.');
});
