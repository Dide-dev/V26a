/*
    This program has been developed by students from the bachelor Computer Science at Utrecht
    University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

import { render, screen, waitFor } from '@testing-library/svelte';
import { test, vi, afterEach, expect } from 'vitest';
import { m } from '$lib/paraglide/messages.js';
import type { MockedFunction } from 'vitest';
import userEvent from '@testing-library/user-event';
import page from './+page.svelte';
import DeleteButton from './DeleteButton.svelte';
import {
    updateUser,
    getOwnUserData,
    getUserInfo,
    type UserGetData,
    type UserUpdateData,
    type UserInfo
} from '$lib/backend/users';

vi.mock('$lib/backend/users', () => {
    return {
        getOwnUserData: vi.fn(),
        getUserInfo: vi.fn(),
        updateUser: vi.fn()
    };
});
afterEach(() => {
    vi.resetAllMocks();
});

test('No submission without name', async () => {
    const userData: UserGetData = {
        email: 'john@test.com',
        name: 'John Test',
        nickname: 'Johnny',
        aboutMe: '',
        emailVisible: false,
        language: 'en'
    };
    const mockGetUser = getOwnUserData as MockedFunction<typeof getOwnUserData>;
    mockGetUser.mockResolvedValue(userData);
    const userInfo: UserInfo = {
        sub: 1,
        exp: 42,
        accountType: 'author'
    };
    const mockGetInfo = getUserInfo as MockedFunction<typeof getUserInfo>;
    mockGetInfo.mockReturnValue(userInfo);

    const user = userEvent.setup();
    render(page);

    const nameInput = await screen.findByTestId('name');
    await user.clear(nameInput);
    await user.click(screen.getByRole('button'));

    expect(updateUser).not.toHaveBeenCalled();
});

test('Submission with full data', async () => {
    const userData: UserGetData = {
        email: 'john@test.com',
        name: 'John Test',
        nickname: 'Johnny',
        aboutMe: '',
        emailVisible: false,
        language: 'en'
    };
    const mockGetUser = getOwnUserData as MockedFunction<typeof getOwnUserData>;
    mockGetUser.mockResolvedValue(userData);
    const userInfo: UserInfo = {
        sub: 1,
        exp: 42,
        accountType: 'author'
    };
    const mockGetInfo = getUserInfo as MockedFunction<typeof getUserInfo>;
    mockGetInfo.mockReturnValue(userInfo);

    const user = userEvent.setup();
    render(page);

    const aboutMeInput = await screen.findByTestId('about-me');
    await user.type(aboutMeInput, 'Some interesting backstory');
    await user.click(await screen.findByRole('button'));

    const expectedUpdateData: UserUpdateData = {
        name: 'John Test',
        nickname: 'Johnny',
        aboutMe: 'Some interesting backstory',
        emailVisible: false,
        language: 'en',
        accountType: null,
        enabled: null
    };
    expect(updateUser).toHaveBeenCalledWith(userInfo.sub, expectedUpdateData);
});

test('popup opens when delete button is clicked', async () => {
    const user = userEvent.setup();

    render(DeleteButton, {
        props: { user: {
            sub: 1,
            exp: 123,
            accountType: 'author'
        } }
    });

    await user.click(screen.getByRole('link'));

    await waitFor(() => {
        expect(screen.getAllByTestId('popup').length).toBeGreaterThan(0);
    });
});

test('popup closes when cancel is clicked', async () => {
    const user = userEvent.setup();

    render(DeleteButton, {
        props: { user: {
            sub: 1,
            exp: 123,
            accountType: 'author'
        } }
    });

    await user.click(screen.getByRole('link'));
    await user.click(screen.getByText(m.cancel()));

    await waitFor(() => {
        expect(screen.getAllByTestId('popup').length).toBe(0);
    });
});

test('checkbox toggles', async () => {
    const user = userEvent.setup();

    render(DeleteButton, {
        props: {
            user: {
                sub: 1,
                exp: 123,
                accountType: 'author'
            }
        }
    });

    await user.click(screen.getByRole('link'));
    const checkbox = await screen.findByTestId('delete-checkbox');

    expect((checkbox as HTMLInputElement).checked).toBe(false);
    await user.click(checkbox);
    expect((checkbox as HTMLInputElement).checked).toBe(true);
});





