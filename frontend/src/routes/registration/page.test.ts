import { userEvent } from '@testing-library/user-event';
import { render, screen } from '@testing-library/svelte';
import { expect, test, vi } from 'vitest';
import page from './+page.svelte';
import { m } from '$lib/paraglide/messages.js';

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
