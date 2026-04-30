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
    changePassword,
    type UserGetData,
    type UserUpdateData,
    type UserInfo
} from '$lib/backend/users';

vi.mock('$lib/backend/users', () => {
    return {
        getOwnUserData: vi.fn(),
        getUserInfo: vi.fn(),
        updateUser: vi.fn(),
        changePassword: vi.fn()
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
    await user.click(screen.getByTestId('confirm-profile'));

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
    await user.click(screen.getByTestId('confirm-profile'));

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
        props: {
            user: {
                sub: 1,
                exp: 123,
                accountType: 'author'
            }
        }
    });

    await user.click(screen.getByTestId('delete-button'));

    expect((await screen.findAllByTestId('popup')).length).toBeGreaterThan(0);
});

test('New password without upper case letter shows error', async () => {
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

    await user.type(await screen.findByTestId('oldPassword'), 'P@ssword123');
    await user.type(screen.getByTestId('newPassword'), '@ssword123');
    await user.type(screen.getByTestId('repeatPassword'), '@ssword123');

    const button = screen.getByTestId('confirm-button');
    await user.click(button);
    const error = screen.getByTestId('missing-char-error');

    expect(error.textContent).toBe(m.missingCharacter());
});

test('New password without lower case letter shows error', async () => {
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

    await user.type(await screen.findByTestId('oldPassword'), 'P@ssword123');
    await user.type(screen.getByTestId('newPassword'), '@SSWORD123');
    await user.type(screen.getByTestId('repeatPassword'), '@SSWORD123');

    const button = screen.getByTestId('confirm-button');
    await user.click(button);
    const error = screen.getByTestId('missing-char-error');

    expect(error.textContent).toBe(m.missingCharacter());
});

test('New password without special character shows error', async () => {
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

    await user.type(await screen.findByTestId('oldPassword'), 'P@ssword123');
    await user.type(screen.getByTestId('newPassword'), 'Password123');
    await user.type(screen.getByTestId('repeatPassword'), 'Password123');

    const button = screen.getByTestId('confirm-button');
    await user.click(button);
    const error = screen.getByTestId('missing-char-error');

    expect(error.textContent).toBe(m.missingCharacter());
});

test('New password without number shows error', async () => {
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

    await user.type(await screen.findByTestId('oldPassword'), 'P@ssword123');
    await user.type(screen.getByTestId('newPassword'), 'Password');
    await user.type(screen.getByTestId('repeatPassword'), 'Password');

    const button = screen.getByTestId('confirm-button');
    await user.click(button);
    const error = screen.getByTestId('missing-char-error');

    expect(error.textContent).toBe(m.missingCharacter());
});

test('Differing new password and repeat password shows error', async () => {
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

    await user.type(await screen.findByTestId('oldPassword'), 'P@ssword123');
    await user.type(screen.getByTestId('newPassword'), 'P@ssword123!');
    await user.type(screen.getByTestId('repeatPassword'), 'P@ssword321!');

    const button = screen.getByTestId('confirm-button');
    await user.click(button);
    const error = screen.getByTestId('differing-password-error');

    expect(error.textContent).toBe(m.passwordsNotTheSame());
});

test('No submission without old password', async () => {
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

    await user.type(await screen.findByTestId('newPassword'), 'P@ssword123');
    await user.type(screen.getByTestId('repeatPassword'), 'P@ssword123');

    await user.click(screen.getByTestId('confirm-button'));
    const error = screen.getByTestId('change-attempt-error');

    expect(changePassword).not.toHaveBeenCalled();
    expect(error.textContent).toBe(m.changePasswordAttempt());
});

test('No submission without new password', async () => {
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

    await user.type(await screen.findByTestId('oldPassword'), 'P@ssword123');
    await user.type(screen.getByTestId('repeatPassword'), 'P@ssword123');

    await user.click(screen.getByTestId('confirm-button'));
    const error = screen.getByTestId('change-attempt-error');

    expect(changePassword).not.toHaveBeenCalled();
    expect(error.textContent).toBe(m.changePasswordAttempt());
});

test('No submission without repeat password', async () => {
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

    await user.type(await screen.findByTestId('oldPassword'), 'P@ssword123');
    await user.type(screen.getByTestId('newPassword'), 'P@ssword123!');

    await user.click(screen.getByTestId('confirm-button'));
    const error = screen.getByTestId('change-attempt-error');

    expect(changePassword).not.toHaveBeenCalled();
    expect(error.textContent).toBe(m.changePasswordAttempt());
});

test('No submission when old and new password are the same', async () => {
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

    await user.type(await screen.findByTestId('oldPassword'), 'P@ssword123');
    await user.type(screen.getByTestId('newPassword'), 'P@ssword123');
    await user.type(screen.getByTestId('repeatPassword'), 'Password123');

    await user.click(screen.getByTestId('confirm-button'));
    const error = screen.getByTestId('same-old-and-new-password-error');

    expect(changePassword).not.toHaveBeenCalled();
    expect(error.textContent).toBe(m.sameOldAndNewPassword());
});

test('Submission when all password expectations have been met', async () => {
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
    (changePassword as MockedFunction<typeof changePassword>).mockResolvedValue({
        succeeded: true
    });

    const user = userEvent.setup();
    render(page);

    await user.type(await screen.findByTestId('oldPassword'), 'P@ssword123');
    await user.type(screen.getByTestId('newPassword'), 'P@ssword123!');
    await user.type(screen.getByTestId('repeatPassword'), 'P@ssword123!');

    await user.click(screen.getByTestId('confirm-button'));
    const confirmation = screen.getByTestId('password-change-success');

    expect(changePassword).toHaveBeenCalled();
    expect(confirmation.textContent).toBe(m.succesfulPasswordChange());
});
