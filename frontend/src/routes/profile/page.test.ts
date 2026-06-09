/*
    This program has been developed by students from the bachelor Computer Science at Utrecht
    University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

import { render, screen } from '@testing-library/svelte';
import DeleteButton from './DeleteButton.svelte';
import { test, vi, afterEach, expect } from 'vitest';
import type { MockedFunction } from 'vitest';
import userEvent from '@testing-library/user-event';
import { goto } from '$app/navigation';
import page from './+page.svelte';
import { m } from '$lib/paraglide/messages.js';
import {
    updateUser,
    getOwnUserData,
    getUserInfo,
    changePassword,
    type UserGetData,
    type UserData,
    type UserInfo,
    deleteMe
} from '$lib/api-wrapper/users';

vi.mock('$lib/api-wrapper/users', () => {
    return {
        getOwnUserData: vi.fn(),
        getUserInfo: vi.fn(),
        updateUser: vi.fn(),
        changePassword: vi.fn(),
        deleteMe: vi.fn()
    };
});
afterEach(() => {
    vi.resetAllMocks();
});

vi.mock('$app/navigation', () => ({
    goto: vi.fn(),
    invalidateAll: vi.fn(),
    invalidate: vi.fn(),
    preloadData: vi.fn(),
    beforeNavigate: vi.fn(),
    afterNavigate: vi.fn()
}));

test('No submission without name', async () => {
    const userData: UserGetData = {
        email: 'john@test.com',
        data: {
            name: 'John_Test',
            nickname: 'Johnny',
            aboutMe: '',
            emailVisible: false,
            language: 'en'
        }
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
    await user.click(screen.getByTestId('confirm-button'));

    expect(updateUser).not.toHaveBeenCalled();
});

test('Submission with full data', async () => {
    const userData: UserGetData = {
        email: 'john@test.com',
        data: {
            name: 'John_Test',
            nickname: 'Johnny',
            aboutMe: '',
            emailVisible: false,
            language: 'en'
        }
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

    (updateUser as MockedFunction<typeof updateUser>).mockResolvedValue({
        type: 'Success',
        userId: 1
    });

    const user = userEvent.setup();
    render(page);

    const aboutMeInput = await screen.findByTestId('about-me');
    await user.type(aboutMeInput, 'Some interesting backstory');
    await user.click(screen.getByTestId('confirm-button'));

    const expectedUpdateData: UserData = {
        name: 'John_Test',
        nickname: 'Johnny',
        aboutMe: 'Some interesting backstory',
        emailVisible: false,
        language: 'en'
    };
    expect(updateUser).toHaveBeenCalledWith(expectedUpdateData);
});

test('New password without upper case letter shows error', async () => {
    const userData: UserGetData = {
        email: 'john@test.com',
        data: {
            name: 'John_Test',
            nickname: 'Johnny',
            aboutMe: '',
            emailVisible: false,
            language: 'en'
        }
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

    (updateUser as MockedFunction<typeof updateUser>).mockResolvedValue({
        type: 'Success',
        userId: 1
    });

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
        data: {
            name: 'John_Test',
            nickname: 'Johnny',
            aboutMe: '',
            emailVisible: false,
            language: 'en'
        }
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

    (updateUser as MockedFunction<typeof updateUser>).mockResolvedValue({
        type: 'Success',
        userId: 1
    });

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
        data: {
            name: 'John_Test',
            nickname: 'Johnny',
            aboutMe: '',
            emailVisible: false,
            language: 'en'
        }
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

    (updateUser as MockedFunction<typeof updateUser>).mockResolvedValue({
        type: 'Success',
        userId: 1
    });

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
        data: {
            name: 'John_Test',
            nickname: 'Johnny',
            aboutMe: '',
            emailVisible: false,
            language: 'en'
        }
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

    (updateUser as MockedFunction<typeof updateUser>).mockResolvedValue({
        type: 'Success',
        userId: 1
    });

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
        data: {
            name: 'John_Test',
            nickname: 'Johnny',
            aboutMe: '',
            emailVisible: false,
            language: 'en'
        }
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

    (updateUser as MockedFunction<typeof updateUser>).mockResolvedValue({
        type: 'Success',
        userId: 1
    });

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
        data: {
            name: 'John_Test',
            nickname: 'Johnny',
            aboutMe: '',
            emailVisible: false,
            language: 'en'
        }
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

    (updateUser as MockedFunction<typeof updateUser>).mockResolvedValue({
        type: 'Success',
        userId: 1
    });

    const user = userEvent.setup();
    render(page);

    await user.type(await screen.findByTestId('newPassword'), 'P@ssword123');
    await user.type(screen.getByTestId('repeatPassword'), 'P@ssword123');

    const form = await screen.findByTestId('profile-form');
    expect(form.classList).not.toContain('completed');

    expect(changePassword).not.toHaveBeenCalled();
});

test('No submission without new password', async () => {
    const userData: UserGetData = {
        email: 'john@test.com',
        data: {
            name: 'John_Test',
            nickname: 'Johnny',
            aboutMe: '',
            emailVisible: false,
            language: 'en'
        }
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

    (updateUser as MockedFunction<typeof updateUser>).mockResolvedValue({
        type: 'Success',
        userId: 1
    });

    const user = userEvent.setup();
    render(page);

    await user.type(await screen.findByTestId('oldPassword'), 'P@ssword123');

    const form = await screen.findByTestId('profile-form');
    expect(form.classList).not.toContain('completed');

    expect(changePassword).not.toHaveBeenCalled();
});

test('No submission without repeat password', async () => {
    const userData: UserGetData = {
        email: 'john@test.com',
        data: {
            name: 'John_Test',
            nickname: 'Johnny',
            aboutMe: '',
            emailVisible: false,
            language: 'en'
        }
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

    (updateUser as MockedFunction<typeof updateUser>).mockResolvedValue({
        type: 'Success',
        userId: 1
    });

    const user = userEvent.setup();
    render(page);

    await user.type(await screen.findByTestId('oldPassword'), 'P@ssword123');
    await user.type(screen.getByTestId('newPassword'), 'P@ssword123!');

    const form = await screen.findByTestId('profile-form');
    expect(form.classList).not.toContain('completed');

    expect(changePassword).not.toHaveBeenCalled();
});

test('No submission when old and new password are the same', async () => {
    const userData: UserGetData = {
        email: 'john@test.com',
        data: {
            name: 'John_Test',
            nickname: 'Johnny',
            aboutMe: '',
            emailVisible: false,
            language: 'en'
        }
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

    (updateUser as MockedFunction<typeof updateUser>).mockResolvedValue({
        type: 'Success',
        userId: 1
    });

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
        data: {
            name: 'John_Test',
            nickname: 'Johnny',
            aboutMe: '',
            emailVisible: false,
            language: 'en'
        }
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

    (updateUser as MockedFunction<typeof updateUser>).mockResolvedValue({
        type: 'Success',
        userId: 1
    });

    const user = userEvent.setup();
    render(page);

    await user.type(await screen.findByTestId('oldPassword'), 'P@ssword123');
    await user.type(screen.getByTestId('newPassword'), 'P@ssword123!');
    await user.type(screen.getByTestId('repeatPassword'), 'P@ssword123!');

    await user.click(screen.getByTestId('confirm-button'));
    const confirmation = screen.getByTestId('password-change-success');

    expect(changePassword).toHaveBeenCalled();
    expect(confirmation.textContent).toBe(m.successfulPasswordChange());
});

test('Popup opens when delete button is clicked', async () => {
    const user = userEvent.setup();

    render(DeleteButton, {});

    await user.click(screen.getByTestId('delete-button'));

    screen.getByTestId('popup');
});

test('Deletes account and renavigate after confirming deletion', async () => {
    const userData: UserGetData = {
        email: 'john@test.com',
        data: {
            name: 'John_Test',
            nickname: 'Johnny',
            aboutMe: '',
            emailVisible: false,
            language: 'en'
        }
    };
    const mockGetUser = getOwnUserData as MockedFunction<typeof getOwnUserData>;
    mockGetUser.mockResolvedValue(userData);

    const user = userEvent.setup();

    (deleteMe as MockedFunction<typeof deleteMe>).mockResolvedValue(undefined);

    render(page);
    render(DeleteButton, {});

    await user.click(screen.getByTestId('delete-button'));
    await user.click(screen.getByTestId('delete-confirm'));

    // Check that the api function is called.
    expect(deleteMe).toHaveBeenCalled();

    // After deletion user is navigated to home page.
    expect(goto).toHaveBeenCalledWith('/');
});
