/*
    This program has been developed by students from the bachelor Computer Science at Utrecht
    University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

import { render, screen } from '@testing-library/svelte';
import { test, vi, afterEach, expect, type MockedFunction } from 'vitest';
import userEvent from '@testing-library/user-event';
import page from './+page.svelte';
import { addClassroom } from '$lib/api-wrapper/classrooms';

vi.mock('$lib/api-wrapper/classrooms', () => {
    return {
        addClassroom: vi.fn()
    };
});
vi.mock('$app/navigation', () => ({
    goto: vi.fn()
}));
afterEach(() => {
    vi.resetAllMocks();
});

test('No submission without classroom name', async () => {
    const user = userEvent.setup();
    render(page);

    const descriptionInput = screen.getByTestId('description');
    await user.type(descriptionInput, 'classroom');
    await user.click(screen.getByTestId('add-button'));

    expect(addClassroom).not.toHaveBeenCalled();
});

test('Submission with full classroom data', async () => {
    (addClassroom as MockedFunction<typeof addClassroom>).mockResolvedValue({
        id: 1,
        joinCode: 'Join12'
    });
    const user = userEvent.setup();
    render(page);

    const nameInput = screen.getByTestId('name');
    await user.type(nameInput, 'classroom');
    const descriptionInput = screen.getByTestId('description');
    await user.type(descriptionInput, 'classroom description');
    await user.click(screen.getByTestId('add-button'));

    expect(addClassroom).toHaveBeenCalledWith(
        expect.objectContaining({
            name: 'classroom',
            description: 'classroom description'
        })
    );
});
