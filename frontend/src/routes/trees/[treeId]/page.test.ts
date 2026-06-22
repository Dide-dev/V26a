/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

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
    TreeLinkedData,
    TreeMetadata,
    TreeStructure,
    TreeWithCards
} from '$lib/api-wrapper/trees';
import type { CardData, CardId } from '$lib/api-wrapper/cards';
import { getTreeWithCards } from '$lib/api-wrapper/trees';
import { getUserInfo } from '$lib/api-wrapper/sessions';

// Mock the logging check.
vi.mock('$lib/api-wrapper/sessions', () => ({
    getUserInfo: vi.fn()
}));

function createTree(): TreeWithCards {
    // eslint-disable as CardId's are seen as object literal property, and should therefore follow camelCase.
    /* eslint-disable @typescript-eslint/naming-convention */
    const cards: Record<CardId, CardData> = {
        1: { front: 'front 1', back: 'back 1' },
        2: { front: 'front 2', back: 'back 2' }
    };
    /* eslint-enable @typescript-eslint/naming-convention */

    const treeData: TreeData = {
        title: 'Test Tree',
        description: 'Test Tree Description',
        languageCode: 'en',
        tags: [],
        subjectId: 1,
        bookName: 'Test book name',
        bookAuthor: 'Test author',
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

    const treeLinkedData: TreeLinkedData = {
        ownerName: 'Test owner name',
        subjectName: 'Test subject name',
        coAuthors: [],
        languageName: 'English'
    };

    const treeMetadata: TreeMetadata = {
        data: treeData,
        genData: treeGeneratedData,
        linkedData: treeLinkedData
    };

    const treeWithCards: TreeWithCards = {
        metadata: treeMetadata,
        structure: treeStructure,
        cards: cards
    };

    return treeWithCards;
}

vi.mock('$lib/api-wrapper/trees', () => {
    return { getTreeWithCards: vi.fn() };
});
afterEach(() => {
    vi.resetAllMocks();
});

test('Page successfully loads tree', async () => {
    const tree: TreeWithCards = createTree();

    const mockGetTree = getTreeWithCards as MockedFunction<typeof getTreeWithCards>;
    mockGetTree.mockResolvedValue(tree);
    render(PageComponent, {
        params: {
            treeId: '4'
        }
    });

    await screen.findByText(tree.metadata.data.title);

    const description = tree.metadata.data.description as string;
    screen.getByText(description);

    screen.getByText(tree.cards[1].front);
    screen.getByText(tree.cards[2].front);
});

test('shows Edit button when user is logged in', async () => {
    const tree: TreeWithCards = createTree();

    const mockGetTree = getTreeWithCards as MockedFunction<typeof getTreeWithCards>;
    mockGetTree.mockResolvedValue(tree);

    vi.mocked(getUserInfo).mockReturnValue({
        accountType: 'author',
        sub: 2098,
        name: 'John',
        nickname: ''
    });

    render(PageComponent, {
        params: {
            treeId: '4'
        }
    });

    await screen.findByTestId('edit-button');
});

test('does not show Edit button when user is not logged in', async () => {
    const tree: TreeWithCards = createTree();

    const mockGetTree = getTreeWithCards as MockedFunction<typeof getTreeWithCards>;
    mockGetTree.mockResolvedValue(tree);
    page.url.searchParams.set('id', '4');

    vi.mocked(getUserInfo).mockReturnValue(null);

    await expect(() => screen.findByTestId('edit-button')).rejects.toThrow();
});
