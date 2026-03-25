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

test('passwordNoUpper', async () => {
    const user = userEvent.setup();
    render(page);

    await user.type(screen.getByTestId('email'), 'test@test.test');
    await user.type(screen.getByTestId('name'), 'test');
    await user.type(screen.getByTestId('password'), 'aaaaaaaa1!');
    await user.type(screen.getByTestId('repeat password'), 'aaaaaaaa1!');

    const button = screen.getByRole('button');
    await user.click(button);
    const error = screen.getByTestId('error');

    expect(error.textContent).toBe(m.missingCharacter());
});

test('passwordNoLower', async () => {
    const user = userEvent.setup();
    render(page);

    await user.type(screen.getByTestId('email'), 'test@test.test');
    await user.type(screen.getByTestId('name'), 'test');
    await user.type(screen.getByTestId('password'), 'AAAAAAAA1!');
    await user.type(screen.getByTestId('repeat password'), 'AAAAAAAA1!');

    const button = screen.getByRole('button');
    await user.click(button);
    const error = screen.getByTestId('error');

    expect(error.textContent).toBe(m.missingCharacter());
});

test('passwordNoSpecialCharacter', async () => {
    const user = userEvent.setup();
    render(page);

    await user.type(screen.getByTestId('email'), 'test@test.test');
    await user.type(screen.getByTestId('name'), 'test');
    await user.type(screen.getByTestId('password'), 'aaaaaaaA1');
    await user.type(screen.getByTestId('repeat password'), 'aaaaaaaA1');

    const button = screen.getByRole('button');
    await user.click(button);
    const error = screen.getByTestId('error');

    expect(error.textContent).toBe(m.missingCharacter());
});

test('passwordNoNumber', async () => {
    const user = userEvent.setup();
    render(page);

    await user.type(screen.getByTestId('email'), 'test@test.test');
    await user.type(screen.getByTestId('name'), 'test');
    await user.type(screen.getByTestId('password'), 'aaaaaaaA!');
    await user.type(screen.getByTestId('repeat password'), 'aaaaaaaA!');

    const button = screen.getByRole('button');
    await user.click(button);
    const error = screen.getByTestId('error');

    expect(error.textContent).toBe(m.missingCharacter());
});

test('passwordsNotTheSame', async () => {
    const user = userEvent.setup();
    render(page);

    await user.type(screen.getByTestId('email'), 'test@test.test');
    await user.type(screen.getByTestId('name'), 'test');
    await user.type(screen.getByTestId('password'), 'aaaaaaaA1!2');
    await user.type(screen.getByTestId('repeat password'), 'aaaaaaaA1!');

    const button = screen.getByRole('button');
    await user.click(button);
    const error = screen.getByTestId('error');

    expect(error.textContent).toBe(m.passwordsNotTheSame());
});

test('passwordsCorrect', async () => {
    let test = '';
    global.fetch = vi.fn().mockImplementation((path: string) => {
        if (path === '/api/v1/users/add') {
            test = 'Correct password';
        }
    });
    const user = userEvent.setup();
    render(page);

    await user.type(screen.getByTestId('email'), 'test@test.test');
    await user.type(screen.getByTestId('name'), 'test');
    await user.type(screen.getByTestId('password'), 'aaaaaaaA1!');
    await user.type(screen.getByTestId('repeat password'), 'aaaaaaaA1!');

    const button = screen.getByRole('button');
    await user.click(button);
    const error = screen.getByTestId('error');

    expect(error.textContent).not.toBe(m.missingCharacter());
    expect(error.textContent).not.toBe(m.passwordsNotTheSame());
    expect(test).toBe('Correct password');
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