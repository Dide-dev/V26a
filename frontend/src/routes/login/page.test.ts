/*
    This program has been developed by students from the bachelor Computer Science at Utrecht
    University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

import { render, screen } from '@testing-library/svelte';
import { test, vi, afterEach, expect, type MockedFunction } from 'vitest';
import userEvent from '@testing-library/user-event';
import page from './+page.svelte';
import { m } from '$lib/paraglide/messages.js';
import { login } from '$lib/api-wrapper/auth';

vi.mock('$lib/api-wrapper/auth', () => {
    return {
        login: vi.fn()
    };
});
vi.mock('$app/navigation', () => ({
    goto: vi.fn()
}));
afterEach(() => {
    vi.resetAllMocks();
});

test('No submission without password', async () => {
    const user = userEvent.setup();
    render(page);

    const emailInput = screen.getByTestId('email');
    await user.type(emailInput, 'test@test.nl');
    await user.click(screen.getByRole('button', { name: m.login() }));

    expect(login).not.toHaveBeenCalled();
});

test('No submission without email', async () => {
    const user = userEvent.setup();
    render(page);

    const passwordInput = screen.getByTestId('password');
    await user.type(passwordInput, 'ThisIsAVerySecureP@ssword123');

    await user.click(screen.getByRole('button', { name: m.login() }));

    expect(login).not.toHaveBeenCalled();
});

test('No submission when e-mail contains no @ and no domain', async () => {
    const user = userEvent.setup();
    render(page);

    const emailInput = screen.getByTestId('email');
    await user.type(emailInput, 'test');
    const passwordInput = screen.getByTestId('password');
    await user.type(passwordInput, 'ThisIsAVerySecureP@ssword123');

    await user.click(screen.getByRole('button', { name: m.login() }));

    expect(login).not.toHaveBeenCalled();
});

test('No submission when e-mail contains no domain', async () => {
    const user = userEvent.setup();
    render(page);

    const emailInput = screen.getByTestId('email');
    await user.type(emailInput, 'test@');
    const passwordInput = screen.getByTestId('password');
    await user.type(passwordInput, 'ThisIsAVerySecureP@ssword123');

    await user.click(screen.getByRole('button', { name: m.login() }));

    expect(login).not.toHaveBeenCalled();
});

test('No submission when e-mail contains a invalid special character other than @', async () => {
    const user = userEvent.setup();
    render(page);

    const emailInput = screen.getByTestId('email');
    await user.type(emailInput, 'test@te;st.nl');
    const passwordInput = screen.getByTestId('password');
    await user.type(passwordInput, 'ThisIsAVerySecureP@ssword123');

    await user.click(screen.getByRole('button', { name: m.login() }));

    expect(login).not.toHaveBeenCalled();
});

test('Submission with valid e-mail', async () => {
    (login as MockedFunction<typeof login>).mockResolvedValue({ succeeded: true });

    const user = userEvent.setup();
    render(page);

    const emailInput = screen.getByTestId('email');
    await user.type(emailInput, 'test@test.nl');
    const passwordInput = screen.getByTestId('password');
    await user.type(passwordInput, 'ThisIsAVerySecureP@ssword123');

    await user.click(screen.getByRole('button', { name: m.login() }));

    expect(login).toHaveBeenCalled();
});
