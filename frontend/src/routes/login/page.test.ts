import { render, screen } from '@testing-library/svelte';
import { test, vi, afterEach, expect } from 'vitest';
import userEvent from '@testing-library/user-event';
import page from './+page.svelte';
import { m } from '$lib/paraglide/messages.js';
import { login } from '$lib/backend/auth';

vi.mock('$lib/backend/auth', () => {
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
    const user = userEvent.setup({ delay: 0 });
    render(page);

    const emailInput = screen.getByLabelText('E-mail');
    await user.type(emailInput, 'test@test.nl');
    await user.click(screen.getByRole('button', { name: m.login() }));

    expect(login).not.toHaveBeenCalled();
});

test('No submission without email', async () => {
    const user = userEvent.setup({ delay: 0 });
    render(page);

    const passwordInput = screen.getByLabelText('Wachtwoord');
    await user.type(passwordInput, 'ThisIsAVerySecureP@ssword123');

    await user.click(screen.getByRole('button', { name: m.login() }));

    expect(login).not.toHaveBeenCalled();
});

test('No submission when e-mail is invalid', async () => {
    const user = userEvent.setup({ delay: 0 });
    render(page);

    const emailInput = screen.getByLabelText('E-mail');
    await user.type(emailInput, 'test');
    const passwordInput = screen.getByLabelText('Wachtwoord');
    await user.type(passwordInput, 'ThisIsAVerySecureP@ssword123');

    await user.click(screen.getByRole('button', { name: m.login() }));

    expect(login).not.toHaveBeenCalled();

    userEvent.clear(emailInput);
    await user.type(emailInput, 'test@');
    await user.click(screen.getByRole('button', { name: m.login() }));
    expect(login).not.toHaveBeenCalled();

    userEvent.clear(emailInput);
    await user.type(emailInput, 'test@te;st.nl');
    await user.click(screen.getByRole('button', { name: m.login() }));
    expect(login).not.toHaveBeenCalled();
});

test('Submission with valid e-mail', async () => {
    const user = userEvent.setup({ delay: 0 });
    render(page);

    const emailInput = screen.getByLabelText('E-mail');
    await user.type(emailInput, 'test@test.nl');
    const passwordInput = screen.getByLabelText('Wachtwoord');
    await user.type(passwordInput, 'ThisIsAVerySecureP@ssword123');

    await user.click(screen.getByRole('button', { name: m.login() }));

    expect(login).toHaveBeenCalled();
});
