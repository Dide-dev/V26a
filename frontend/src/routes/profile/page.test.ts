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
