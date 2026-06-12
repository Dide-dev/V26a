/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

import { render, screen } from '@testing-library/svelte';
import { userEvent } from '@testing-library/user-event';
import { beforeEach, expect, test, vi } from 'vitest';
import MetaData from './TreeEditor.svelte';
import { m } from '$lib/paraglide/messages.js';

// mock API calls
vi.mock('$lib/api-wrapper/trees', async (importOriginal) => {
    return {
        ...(await importOriginal()),
        getTreeWithCards: vi.fn().mockResolvedValue({
            metadata: {
                data: {
                    title: 'Test123',
                    description: 'gerdgreg',
                    tags: ['tag1', 'tag2', 'tag3'],
                    languageCode: 'nl',
                    subjectId: 1,
                    bookName: 'Monkey',
                    bookAuthor: 'John Test',
                    bookEdition: '13th',
                    bookIsbn: '9780439023481',
                    bookChapter: '10',
                    bookExercise: '3',
                    assignmentText: 'I am a banabana',
                    published: false
                },
                genData: { id: 1, ownerId: 1 },
                linkedData: { ownerName: 'Foo', subjectName: 'Bar' }
            },
            structure: { rootNodes: [] },
            cards: new Map()
        }),
        getAllLanguages: vi.fn().mockResolvedValue(['Nederlands', 'English']),
        getAllSubjects: vi.fn().mockResolvedValue({
            nl: [
                { id: 1, data: { languageCode: 'nl', name: 'Nederlands' } },
                { id: 2, data: { languageCode: 'nl', name: 'Wiskunde' } }
            ]
        }),
        updateTreeMetaData: vi.fn().mockResolvedValue(undefined)
    };
});

// mock state
vi.mock('$app/state', () => ({
    page: { url: { searchParams: { get: () => '1' } } }
}));

beforeEach(() => {
    Element.prototype.getAnimations = () => [];
});

test('dropdown is hidden by default', () => {
    render(MetaData);
    expect(screen.queryByTestId('metadata-popup')).toBeNull();
});

test('dropdown opens when edit metadata button is clicked', async () => {
    const user = userEvent.setup();
    render(MetaData);

    await screen.findByText(m.editMetadata());
    await user.click(screen.getByText(m.editMetadata()));
    screen.getByTestId('metadata-popup');
});

test('fields are prefilled with tree data', async () => {
    const user = userEvent.setup();
    render(MetaData);

    await screen.findByText(m.editMetadata());
    await user.click(screen.getByText(m.editMetadata()));
    expect((screen.getByLabelText(m.bookName()) as HTMLInputElement).value).toBe('Monkey');
    expect((screen.getByLabelText(m.isbn()) as HTMLInputElement).value).toBe('9780439023481');
});

test('cancel restores original values', async () => {
    const user = userEvent.setup();
    render(MetaData);

    await screen.findByText(m.editMetadata());
    await user.click(screen.getByText(m.editMetadata()));
    await user.clear(screen.getByLabelText(m.description()));
    await user.type(screen.getByLabelText(m.description()), 'Changed Description');
    await user.click(screen.getByText(m.cancel()));

    await user.click(screen.getByText(m.editMetadata()));
    expect((screen.getByLabelText(m.description()) as HTMLInputElement).value).toBe('gerdgreg');
});

test('save calls updateTreeMetaData with correct data', async () => {
    const { updateTreeMetaData } = await import('$lib/api-wrapper/trees');
    const user = userEvent.setup();
    render(MetaData);

    await screen.findByText(m.editMetadata());
    await user.click(screen.getByText(m.editMetadata()));
    await user.clear(screen.getByLabelText(m.description()));
    await user.type(screen.getByLabelText(m.description()), 'New Apple');
    await user.click(screen.getByText(m.confirmChanges()));

    expect(updateTreeMetaData).toHaveBeenCalledWith(
        1,
        expect.objectContaining({ description: 'New Apple' })
    );
});

test('dropdown closes after save', async () => {
    const user = userEvent.setup();
    render(MetaData);

    await screen.findByText(m.editMetadata());
    await user.click(screen.getByText(m.editMetadata()));
    // Change one of the fields so that the confirm button is enabled.
    await user.type(screen.getByLabelText(m.description()), 'Changed Description');
    await user.click(screen.getByText(m.confirmChanges()));
    expect(screen.queryByTestId('metadata-popup')).toBeNull();
});

test('dropdown closes after cancel', async () => {
    const user = userEvent.setup();
    render(MetaData);

    await screen.findByText(m.editMetadata());
    await user.click(screen.getByText(m.editMetadata()));
    await user.click(screen.getByText(m.cancel()));

    expect(screen.queryByTestId('metadata-popup')).toBeNull();
});
