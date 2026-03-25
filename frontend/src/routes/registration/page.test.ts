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
import { addUser } from '$lib/backend/users';
import { goto } from '$app/navigation';

vi.mock('$lib/backend/users', () => {
    return {
        addUser: vi.fn()
    };
});
vi.mock('$app/navigation', () => ({
    goto: vi.fn()
}));
afterEach(() => {
    vi.resetAllMocks();
});

test('Redirection after succesful creation', async () => {
    const user = userEvent.setup();

    (addUser as MockedFunction<typeof addUser>).mockResolvedValue(1);

    render(page);

    const emailInput = screen.getByLabelText('E-mail');
    await user.type(emailInput, 'test@email.nl');
    const nameInput = screen.getByLabelText('Naam');
    await user.type(nameInput, 'Tree Maker');
    const passwordInput = screen.getByLabelText('Wachtwoord');
    await user.type(passwordInput, 'ThisIsAVerySecureP@ssword123');

    await user.click(screen.getByRole('button', { name: m.register() }));

    expect(goto).toHaveBeenCalledWith('/login');
});