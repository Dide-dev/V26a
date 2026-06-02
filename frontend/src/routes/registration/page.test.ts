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
import { addUser } from '$lib/api-wrapper/users';
import { goto } from '$app/navigation';
import { appLink } from '$lib';
import { getLocale } from '$lib/paraglide/runtime.js';

vi.mock('$lib/api-wrapper/users', () => {
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

test('Password without upper case letter shows error', async () => {
    const user = userEvent.setup();
    render(page);

    await user.type(screen.getByTestId('email'), 'test@test.test');
    await user.type(screen.getByTestId('name'), 'test');
    await user.type(screen.getByTestId('password'), 'aaaaaaaa1!');
    await user.type(screen.getByTestId('repeatPassword'), 'aaaaaaaa1!');
    await user.click(screen.getByTestId('agreeToTOS'));

    const button = screen.getByRole('button');
    await user.click(button);
    const error = screen.getByTestId('error');

    expect(error.textContent).toBe(m.missingCharacter());
});

test('Password without lower case letter shows error', async () => {
    const user = userEvent.setup();
    render(page);

    await user.type(screen.getByTestId('email'), 'test@test.test');
    await user.type(screen.getByTestId('name'), 'test');
    await user.type(screen.getByTestId('password'), 'AAAAAAAA1!');
    await user.type(screen.getByTestId('repeatPassword'), 'AAAAAAAA1!');
    await user.click(screen.getByTestId('agreeToTOS'));

    const button = screen.getByRole('button');
    await user.click(button);
    const error = screen.getByTestId('error');

    expect(error.textContent).toBe(m.missingCharacter());
});

test('Password without special character shows error', async () => {
    const user = userEvent.setup();
    render(page);

    await user.type(screen.getByTestId('email'), 'test@test.test');
    await user.type(screen.getByTestId('name'), 'test');
    await user.type(screen.getByTestId('password'), 'aaaaaaaA1');
    await user.type(screen.getByTestId('repeatPassword'), 'aaaaaaaA1');
    await user.click(screen.getByTestId('agreeToTOS'));

    const button = screen.getByRole('button');
    await user.click(button);
    const error = screen.getByTestId('error');

    expect(error.textContent).toBe(m.missingCharacter());
});

test('Password without number shows error', async () => {
    const user = userEvent.setup();
    render(page);

    await user.type(screen.getByTestId('email'), 'test@test.test');
    await user.type(screen.getByTestId('name'), 'test');
    await user.type(screen.getByTestId('password'), 'aaaaaaaA!');
    await user.type(screen.getByTestId('repeatPassword'), 'aaaaaaaA!');
    await user.click(screen.getByTestId('agreeToTOS'));

    const button = screen.getByRole('button');
    await user.click(button);
    const error = screen.getByTestId('error');

    expect(error.textContent).toBe(m.missingCharacter());
});

test('Differing password and repeatPassword shows error', async () => {
    const user = userEvent.setup();
    render(page);

    await user.type(screen.getByTestId('email'), 'test@test.test');
    await user.type(screen.getByTestId('name'), 'test');
    await user.type(screen.getByTestId('password'), 'aaaaaaaA1!2');
    await user.type(screen.getByTestId('repeatPassword'), 'aaaaaaaA1!');
    await user.click(screen.getByTestId('agreeToTOS'));

    const button = screen.getByRole('button');
    await user.click(button);
    const error = screen.getByTestId('error');

    expect(error.textContent).toBe(m.passwordsNotTheSame());
});

test('Redirection after successful registration', async () => {
    const user = userEvent.setup();

    (addUser as MockedFunction<typeof addUser>).mockResolvedValue({ type: 'Success', userId: 1 });

    render(page);

    const emailInput = screen.getByTestId('email');
    await user.type(emailInput, 'test@email.nl');
    const nameInput = screen.getByTestId('name');
    await user.type(nameInput, 'Tree_Maker');
    const passwordInput = screen.getByTestId('password');
    await user.type(passwordInput, 'ThisIsAVerySecureP@ssword123');
    const repeatPasswordInput = screen.getByTestId('repeatPassword');
    await user.type(repeatPasswordInput, 'ThisIsAVerySecureP@ssword123');
    await user.click(screen.getByTestId('agreeToTOS'));

    await user.click(screen.getByRole('button', { name: m.register() }));

    expect(goto).toHaveBeenCalledWith(appLink('/login'));
});

test('Submission with correct data', async () => {
    const user = userEvent.setup();

    (addUser as MockedFunction<typeof addUser>).mockResolvedValue({ type: 'Success', userId: 1 });

    render(page);

    const emailInput = screen.getByTestId('email');
    await user.type(emailInput, 'test@email.nl');
    const nameInput = screen.getByTestId('name');
    await user.type(nameInput, 'Tree_Maker');
    const passwordInput = screen.getByTestId('password');
    await user.type(passwordInput, 'ThisIsAVerySecureP@ssword123');
    const repeatPasswordInput = screen.getByTestId('repeatPassword');
    await user.type(repeatPasswordInput, 'ThisIsAVerySecureP@ssword123');
    await user.click(screen.getByTestId('agreeToTOS'));

    await user.click(screen.getByRole('button', { name: m.register() }));
    const errors = screen.queryByTestId('error');
    expect(errors).toBeFalsy();

    expect(addUser).toHaveBeenCalledWith(
        expect.objectContaining({
            data: {
                name: 'Tree_Maker',
                nickname: '',
                aboutMe: '',
                emailVisible: false,
                language: getLocale()
            },
            email: 'test@email.nl',
            password: 'ThisIsAVerySecureP@ssword123'
        })
    );
});

test('No submission without password', async () => {
    const user = userEvent.setup();
    render(page);

    const emailInput = screen.getByTestId('email');
    await user.type(emailInput, 'test@email.nl');
    const nameInput = screen.getByTestId('name');
    await user.type(nameInput, 'Tree Maker');

    await user.click(screen.getByRole('button', { name: m.register() }));
    await user.click(screen.getByTestId('agreeToTOS'));

    expect(addUser).not.toHaveBeenCalled();
});

test('No submission without repeatPassword', async () => {
    const user = userEvent.setup();
    render(page);

    const emailInput = screen.getByTestId('email');
    await user.type(emailInput, 'test@email.nl');
    const nameInput = screen.getByTestId('name');
    await user.type(nameInput, 'Tree_Maker');
    const passwordInput = screen.getByTestId('password');
    await user.type(passwordInput, 'ThisIsAVerySecureP@ssword123');
    await user.click(screen.getByTestId('agreeToTOS'));

    await user.click(screen.getByRole('button', { name: m.register() }));

    expect(addUser).not.toHaveBeenCalled();
});

test('No submission without name', async () => {
    const user = userEvent.setup();
    render(page);

    const emailInput = screen.getByTestId('email');
    await user.type(emailInput, 'test@email.nl');
    const passwordInput = screen.getByTestId('password');
    await user.type(passwordInput, 'ThisIsAVerySecureP@ssword123');
    const repeatPasswordInput = screen.getByTestId('repeatPassword');
    await user.type(repeatPasswordInput, 'ThisIsAVerySecureP@ssword123');
    await user.click(screen.getByTestId('agreeToTOS'));

    await user.click(screen.getByRole('button', { name: m.register() }));

    expect(addUser).not.toHaveBeenCalled();
});

test('No submission without email', async () => {
    const user = userEvent.setup();
    render(page);

    const nameInput = screen.getByTestId('name');
    await user.type(nameInput, 'Tree_Maker');
    const passwordInput = screen.getByTestId('password');
    await user.type(passwordInput, 'ThisIsAVerySecureP@ssword123');
    const repeatPasswordInput = screen.getByTestId('repeatPassword');
    await user.type(repeatPasswordInput, 'ThisIsAVerySecureP@ssword123');
    await user.click(screen.getByTestId('agreeToTOS'));

    await user.click(screen.getByRole('button', { name: m.register() }));

    expect(addUser).not.toHaveBeenCalled();
});

test('No submission without agreeing to TOS', async () => {
    const user = userEvent.setup();
    render(page);

    const emailInput = screen.getByTestId('email');
    await user.type(emailInput, 'test@email.nl');
    const nameInput = screen.getByTestId('name');
    await user.type(nameInput, 'Tree_Maker');
    const passwordInput = screen.getByTestId('password');
    await user.type(passwordInput, 'ThisIsAVerySecureP@ssword123');
    const repeatPasswordInput = screen.getByTestId('repeatPassword');
    await user.type(repeatPasswordInput, 'ThisIsAVerySecureP@ssword123');

    await user.click(screen.getByRole('button', { name: m.register() }));

    expect(addUser).not.toHaveBeenCalled();
});
