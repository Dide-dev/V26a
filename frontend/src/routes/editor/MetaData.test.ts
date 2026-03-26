import { render, screen, waitFor } from '@testing-library/svelte';
import { userEvent } from '@testing-library/user-event';
import { expect, test, vi } from 'vitest';
import MetaData from './+page.svelte';

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

// mock the en-language
vi.mock('$lib/paraglide/messages.js', () => ({
    m: {
        title: () => 'Title',
        description: () => 'Description',
        bookname: () => 'Book name',
        bookEdition: () => 'Book edition',
        bookIsbn: () => 'ISBN',
        bookChapter: () => 'Chapter',
        bookExercise: () => 'Exercise',
        assignmentText: () => 'Assignment',
        published: () => 'Published',
        save: () => 'Save',
        cancel: () => 'Cancel',
        editMetadata: () => 'Edit metadata'
    }
}));

test('dropdown is hidden by default', () => {
    render(MetaData);
    expect(document.getElementsByClassName('meta_dropdown').length).not.toBeGreaterThan(0);
});

test('dropdown opens when edit metadata button is clicked', async () => {
    const user = userEvent.setup();
    render(MetaData);

    await waitFor(() => screen.getByText('Edit metadata'));
    await user.click(screen.getByText('Edit metadata'));
    await waitFor(() => {
        expect(document.getElementsByClassName('meta_dropdown').length).toBeGreaterThan(0);
    });
});

test('fields are prefilled with tree data', async () => {
    const user = userEvent.setup();
    render(MetaData);

    await waitFor(() => screen.getByText('Edit metadata'));
    await user.click(screen.getByText('Edit metadata'));
    expect((screen.getByLabelText('Book name') as HTMLInputElement).value).toBe('Monkey');
    expect((screen.getByLabelText('ISBN') as HTMLInputElement).value).toBe('123-456');
});

test('cancel restores original values', async () => {
    const user = userEvent.setup();
    render(MetaData);

    await waitFor(() => screen.getByText('Edit metadata'));
    await user.click(screen.getByText('Edit metadata'));
    await user.clear(screen.getByLabelText('Book name'));
    await user.type(screen.getByLabelText('Book name'), 'Changed Book');
    await user.click(screen.getByText('Cancel'));

    await user.click(screen.getByText('Edit metadata'));
    expect((screen.getByLabelText('Book name') as HTMLInputElement).value).toBe('Monkey');
});

test('save calls updateTreeMetaData with correct data', async () => {
    const { updateTreeMetaData } = await import('$lib/backend/trees');
    const user = userEvent.setup();
    render(MetaData);

    await waitFor(() => screen.getByText('Edit metadata'));
    await user.click(screen.getByText('Edit metadata'));
    await user.clear(screen.getByLabelText('Book name'));
    await user.type(screen.getByLabelText('Book name'), 'New Apple');
    await user.click(screen.getByText('Save'));

    expect(updateTreeMetaData).toHaveBeenCalledWith({
        id: 1,
        newData: expect.objectContaining({ bookName: 'New Apple' })
    });
});

test('dropdown closes after save', async () => {
    const user = userEvent.setup();
    render(MetaData);

    await waitFor(() => screen.getByText('Edit metadata'));
    await user.click(screen.getByText('Edit metadata'));
    await user.click(screen.getByText('Save'));
    await waitFor(() => {
        expect(document.getElementsByClassName('meta_dropdown').length).not.toBeGreaterThan(0);
    });
});

test('dropdown closes after cancel', async () => {
    const user = userEvent.setup();
    render(MetaData);

    await waitFor(() => screen.getByText('Edit metadata'));
    await user.click(screen.getByText('Edit metadata'));
    await user.click(screen.getByText('Cancel'));
    await waitFor(() => {
        expect(document.getElementsByClassName('meta_dropdown').length).not.toBeGreaterThan(0);
    });
});
