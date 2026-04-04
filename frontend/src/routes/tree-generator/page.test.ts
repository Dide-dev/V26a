/*
    This program has been developed by students from the bachelor Computer Science at Utrecht
    University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

import { render, screen } from '@testing-library/svelte';
import { test, vi, afterEach, expect } from 'vitest';
import type { MockedFunction } from 'vitest';
import userEvent from '@testing-library/user-event';
import page from './+page.svelte';
import { m } from '$lib/paraglide/messages.js';
import { addTree } from '$lib/backend/trees';
import { goto } from '$app/navigation';

vi.mock('$lib/backend/trees', () => {
    return {
        addTree: vi.fn()
    };
});
vi.mock('$app/navigation', () => ({
    goto: vi.fn()
}));
afterEach(() => {
    vi.resetAllMocks();
});

test('No submission without tree title', async () => {
    const user = userEvent.setup();
    render(page);

    await user.click(screen.getByRole('button', { name: m.createTree() }));

    expect(addTree).not.toHaveBeenCalled();
});

test('Submission with correct data', async () => {
    const user = userEvent.setup();
    render(page);

    const titleInput = screen.getByTestId('title');
    await user.type(titleInput, 'Test tree title');
    const descriptionInput = screen.getByTestId('description');
    await user.type(descriptionInput, 'Test tree description');
    const bookNameInput = screen.getByTestId('bookname');
    await user.type(bookNameInput, 'A book name');
    const bookEditionInput = screen.getByTestId('bookedition');
    await user.type(bookEditionInput, '4th edition');
    const bookIsbnInput = screen.getByTestId('bookIsbn');
    await user.type(bookIsbnInput, '9781408883464');
    const chapterInput = screen.getByTestId('chapter');
    await user.type(chapterInput, 'chapter 1');
    const excerciseInput = screen.getByTestId('exercise');
    await user.type(excerciseInput, 'exercise 1');
    const assignmentTextInput = screen.getByTestId('assignment');
    await user.type(assignmentTextInput, 'assignment text');
    await user.click(screen.getByRole('button', { name: m.createTree() }));

    expect(addTree).toHaveBeenCalledWith(
        expect.objectContaining({
            title: 'Test tree title',
            description: 'Test tree description',
            bookName: 'A book name',
            bookEdition: '4th edition',
            bookIsbn: '9781408883464',
            bookChapter: 'chapter 1',
            bookExercise: 'exercise 1',
            assignmentText: 'assignment text',
            published: false
        })
    );
});

test('Redirection after succesful creation', async () => {
    const user = userEvent.setup();

    (addTree as MockedFunction<typeof addTree>).mockResolvedValue(1);

    render(page);

    const titleInput = screen.getByTestId('title');
    await user.type(titleInput, 'Test tree title');
    await user.click(screen.getByRole('button', { name: m.createTree() }));

    expect(goto).toHaveBeenCalledWith('/tree-page/editor?id=1');
});
