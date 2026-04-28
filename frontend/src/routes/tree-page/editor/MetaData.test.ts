import { render, screen } from '@testing-library/svelte';
import { userEvent } from '@testing-library/user-event';
import { expect, test, vi } from 'vitest';
import MetaData from './+page.svelte';
import { m } from '$lib/paraglide/messages.js';

// mock API calls
vi.mock('$lib/backend/trees', () => ({
    getTreeWithCards: vi.fn().mockResolvedValue({
        data: {
            title: 'Test123',
            description: 'gerdgreg',
            bookName: 'Monkey',
            bookEdition: '13th',
            bookIsbn: '123-456',
            bookChapter: '10',
            bookExercise: '3',
            assignmentText: 'I am a banabana',
            published: false
        },
        genData: { id: 1, ownerId: 1 },
        structure: { rootNodes: [] },
        cards: new Map()
    }),
    updateTreeMetaData: vi.fn().mockResolvedValue(undefined)
}));

// mock state
vi.mock('$app/state', () => ({
    page: { url: { searchParams: { get: () => '1' } } }
}));

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
    expect((screen.getByLabelText(m.bookname()) as HTMLInputElement).value).toBe('Monkey');
    expect((screen.getByLabelText(m.bookIsbn()) as HTMLInputElement).value).toBe('123-456');
});

test('cancel restores original values', async () => {
    const user = userEvent.setup();
    render(MetaData);

    await screen.findByText(m.editMetadata());
    await user.click(screen.getByText(m.editMetadata()));
    await user.clear(screen.getByLabelText(m.bookname()));
    await user.type(screen.getByLabelText(m.bookname()), 'Changed Book');
    await user.click(screen.getByText(m.cancel()));

    await user.click(screen.getByText(m.editMetadata()));
    expect((screen.getByLabelText(m.bookname()) as HTMLInputElement).value).toBe('Monkey');
});

test('save calls updateTreeMetaData with correct data', async () => {
    const { updateTreeMetaData } = await import('$lib/backend/trees');
    const user = userEvent.setup();
    render(MetaData);

    await screen.findByText(m.editMetadata());
    await user.click(screen.getByText(m.editMetadata()));
    await user.clear(screen.getByLabelText(m.bookname()));
    await user.type(screen.getByLabelText(m.bookname()), 'New Apple');
    await user.click(screen.getByText(m.save()));

    expect(updateTreeMetaData).toHaveBeenCalledWith({
        id: 1,
        newData: expect.objectContaining({ bookName: 'New Apple' })
    });
});

test('dropdown closes after save', async () => {
    const user = userEvent.setup();
    render(MetaData);

    await screen.findByText(m.editMetadata());
    await user.click(screen.getByText(m.editMetadata()));
    await user.click(screen.getByText(m.save()));
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
