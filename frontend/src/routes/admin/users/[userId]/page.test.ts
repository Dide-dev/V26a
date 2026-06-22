/*
    This program has been developed by students from the bachelor Computer Science at Utrecht
    University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

import { render, screen } from '@testing-library/svelte';
import { test, vi, afterEach, expect } from 'vitest';
import userEvent from '@testing-library/user-event';
import PageComponent from './+page.svelte';
import { getUserInfo, type UserInfo } from '$lib/api-wrapper/sessions';
import {
    type AdminUpdateData,
    getUserAllData,
    updateUserData,
    adminChangePassword,
    type AdminGetUserData
} from '$lib/api-wrapper/admin';
import { m } from '$lib/paraglide/messages.js';

vi.mock('$lib/api-wrapper/sessions', () => {
    return {
        getUserInfo: vi.fn()
    };
});
vi.mock('$lib/api-wrapper/admin', () => {
    return {
        getUserAllData: vi.fn(),
        updateUserData: vi.fn(),
        adminChangePassword: vi.fn()
    };
});
vi.mock('$app/navigation', () => ({
    goto: vi.fn()
}));
afterEach(() => {
    vi.resetAllMocks();
});

function mockAdminGetUserData(userData: AdminUpdateData) {
    const getData: AdminGetUserData = {
        createdAt: new Date(),
        lastLogin: new Date(),
        updateableData: userData
    };

    const mockGetUser = vi.mocked(getUserAllData);
    mockGetUser.mockResolvedValue(getData);
}

test('No submission without name', async () => {
    const userData: AdminUpdateData = {
        email: 'john@test.com',
        data: {
            name: 'John Test',
            nickname: 'Johnny',
            aboutMe: ''
        },
        enabled: true,
        accountType: 'author'
    };

    mockAdminGetUserData(userData);
    const userInfo: UserInfo = {
        userId: 1,
        name: 'John',
        nickname: '',
        accountType: 'author'
    };
    const mockGetInfo = vi.mocked(getUserInfo);
    mockGetInfo.mockReturnValue(userInfo);

    const user = userEvent.setup();
    render(PageComponent, {
        params: {
            userId: '1'
        }
    });

    const nameInput = await screen.findByTestId('name');
    await user.clear(nameInput);
    await user.click(screen.getByTestId('confirm-button'));

    expect(updateUserData).not.toHaveBeenCalled();
});

test('Submission with full data', async () => {
    const userData: AdminUpdateData = {
        email: 'john@test.com',
        data: {
            name: 'John Test',
            nickname: 'Johnny',
            aboutMe: ''
        },
        enabled: true,
        accountType: 'author'
    };
    mockAdminGetUserData(userData);
    const userInfo: UserInfo = {
        userId: 1,
        name: 'John',
        nickname: '',
        accountType: 'author'
    };
    const mockGetInfo = vi.mocked(getUserInfo);
    mockGetInfo.mockReturnValue(userInfo);

    vi.mocked(updateUserData).mockResolvedValue({
        type: 'Success',
        userId: 1
    });

    const user = userEvent.setup();
    render(PageComponent, {
        params: {
            userId: '1'
        }
    });

    const aboutMeInput = await screen.findByTestId('about-me');
    await user.type(aboutMeInput, 'Some interesting backstory');
    await user.click(screen.getByTestId('confirm-button'));

    const expectedUpdateData: AdminUpdateData = {
        email: 'john@test.com',
        data: {
            name: 'John Test',
            nickname: 'Johnny',
            aboutMe: 'Some interesting backstory'
        },
        enabled: true,
        accountType: 'author'
    };
    expect(updateUserData).toHaveBeenCalledWith(1, expectedUpdateData);
});

test('New password without upper case letter shows error', async () => {
    const userData: AdminUpdateData = {
        email: 'john@test.com',
        data: {
            name: 'John Test',
            nickname: 'Johnny',
            aboutMe: ''
        },
        enabled: true,
        accountType: 'author'
    };
    mockAdminGetUserData(userData);
    const userInfo: UserInfo = {
        userId: 1,
        name: 'John',
        nickname: '',
        accountType: 'author'
    };
    const mockGetInfo = vi.mocked(getUserInfo);
    mockGetInfo.mockReturnValue(userInfo);

    vi.mocked(updateUserData).mockResolvedValue({
        type: 'Success',
        userId: 1
    });

    const user = userEvent.setup();
    render(PageComponent, {
        params: {
            userId: '1'
        }
    });

    await user.type(await screen.findByTestId('newPassword'), '@ssword123');
    await user.type(screen.getByTestId('repeatPassword'), '@ssword123');

    const button = screen.getByTestId('confirm-button');
    await user.click(button);
    const error = screen.getByTestId('missing-char-error');

    expect(error.textContent).toBe(m.missingCharacter());
});

test('New password without lower case letter shows error', async () => {
    const userData: AdminUpdateData = {
        email: 'john@test.com',
        data: {
            name: 'John Test',
            nickname: 'Johnny',
            aboutMe: ''
        },
        enabled: true,
        accountType: 'author'
    };
    mockAdminGetUserData(userData);
    const userInfo: UserInfo = {
        userId: 1,
        name: 'John',
        nickname: '',
        accountType: 'author'
    };
    const mockGetInfo = vi.mocked(getUserInfo);
    mockGetInfo.mockReturnValue(userInfo);

    vi.mocked(updateUserData).mockResolvedValue({
        type: 'Success',
        userId: 1
    });

    const user = userEvent.setup();
    render(PageComponent, {
        params: {
            userId: '1'
        }
    });

    await user.type(await screen.findByTestId('newPassword'), '@SSWORD123');
    await user.type(screen.getByTestId('repeatPassword'), '@SSWORD123');

    const button = screen.getByTestId('confirm-button');
    await user.click(button);
    const error = screen.getByTestId('missing-char-error');

    expect(error.textContent).toBe(m.missingCharacter());
});

test('New password without special character shows error', async () => {
    const userData: AdminUpdateData = {
        email: 'john@test.com',
        data: {
            name: 'John Test',
            nickname: 'Johnny',
            aboutMe: ''
        },
        enabled: true,
        accountType: 'author'
    };
    mockAdminGetUserData(userData);
    const userInfo: UserInfo = {
        userId: 1,
        name: 'John',
        nickname: '',
        accountType: 'author'
    };
    const mockGetInfo = vi.mocked(getUserInfo);
    mockGetInfo.mockReturnValue(userInfo);

    vi.mocked(updateUserData).mockResolvedValue({
        type: 'Success',
        userId: 1
    });

    const user = userEvent.setup();
    render(PageComponent, {
        params: {
            userId: '1'
        }
    });

    await user.type(await screen.findByTestId('newPassword'), 'Password123');
    await user.type(screen.getByTestId('repeatPassword'), 'Password123');

    const button = screen.getByTestId('confirm-button');
    await user.click(button);
    const error = screen.getByTestId('missing-char-error');

    expect(error.textContent).toBe(m.missingCharacter());
});

test('New password without number shows error', async () => {
    const userData: AdminUpdateData = {
        email: 'john@test.com',
        data: {
            name: 'John Test',
            nickname: 'Johnny',
            aboutMe: ''
        },
        enabled: true,
        accountType: 'author'
    };
    mockAdminGetUserData(userData);
    const userInfo: UserInfo = {
        userId: 1,
        name: 'John',
        nickname: '',
        accountType: 'author'
    };
    const mockGetInfo = vi.mocked(getUserInfo);
    mockGetInfo.mockReturnValue(userInfo);

    vi.mocked(updateUserData).mockResolvedValue({
        type: 'Success',
        userId: 1
    });

    const user = userEvent.setup();
    render(PageComponent, {
        params: {
            userId: '1'
        }
    });

    await user.type(await screen.findByTestId('newPassword'), 'P@ssword');
    await user.type(screen.getByTestId('repeatPassword'), 'P@ssword');

    const button = screen.getByTestId('confirm-button');
    await user.click(button);
    const error = screen.getByTestId('missing-char-error');

    expect(error.textContent).toBe(m.missingCharacter());
});

test('Differing new password and repeat password shows error', async () => {
    const userData: AdminUpdateData = {
        email: 'john@test.com',
        data: {
            name: 'John Test',
            nickname: 'Johnny',
            aboutMe: ''
        },
        enabled: true,
        accountType: 'author'
    };
    mockAdminGetUserData(userData);
    const userInfo: UserInfo = {
        userId: 1,
        name: 'John',
        nickname: '',
        accountType: 'author'
    };
    const mockGetInfo = vi.mocked(getUserInfo);
    mockGetInfo.mockReturnValue(userInfo);

    vi.mocked(updateUserData).mockResolvedValue({
        type: 'Success',
        userId: 1
    });

    const user = userEvent.setup();
    render(PageComponent, {
        params: {
            userId: '1'
        }
    });

    await user.type(await screen.findByTestId('newPassword'), 'P@ssword123!');
    await user.type(screen.getByTestId('repeatPassword'), 'P@ssword321!');

    const button = screen.getByTestId('confirm-button');
    await user.click(button);
    const error = screen.getByTestId('differing-password-error');

    expect(error.textContent).toBe(m.passwordsNotTheSame());
});

test('No submission without new password', async () => {
    const userData: AdminUpdateData = {
        email: 'john@test.com',
        data: {
            name: 'John Test',
            nickname: 'Johnny',
            aboutMe: ''
        },
        enabled: true,
        accountType: 'author'
    };
    mockAdminGetUserData(userData);
    const userInfo: UserInfo = {
        userId: 1,
        name: 'John',
        nickname: '',
        accountType: 'author'
    };
    const mockGetInfo = vi.mocked(getUserInfo);
    mockGetInfo.mockReturnValue(userInfo);

    vi.mocked(updateUserData).mockResolvedValue({
        type: 'Success',
        userId: 1
    });

    render(PageComponent, {
        params: {
            userId: '1'
        }
    });

    const form = await screen.findByTestId('profile-form');
    expect(form.classList).not.toContain('completed');

    expect(adminChangePassword).not.toHaveBeenCalled();
});

test('No submission without repeat password', async () => {
    const userData: AdminUpdateData = {
        email: 'john@test.com',
        data: {
            name: 'John Test',
            nickname: 'Johnny',
            aboutMe: ''
        },
        enabled: true,
        accountType: 'author'
    };
    mockAdminGetUserData(userData);
    const userInfo: UserInfo = {
        userId: 1,
        name: 'John',
        nickname: '',
        accountType: 'author'
    };
    const mockGetInfo = vi.mocked(getUserInfo);
    mockGetInfo.mockReturnValue(userInfo);

    vi.mocked(updateUserData).mockResolvedValue({
        type: 'Success',
        userId: 1
    });

    const user = userEvent.setup();
    render(PageComponent, {
        params: {
            userId: '1'
        }
    });

    await user.type(await screen.findByTestId('newPassword'), 'P@ssword123!');

    const form = await screen.findByTestId('profile-form');
    expect(form.classList).not.toContain('completed');

    expect(adminChangePassword).not.toHaveBeenCalled();
});

test('Submission when all password expectations have been met', async () => {
    const userData: AdminUpdateData = {
        email: 'john@test.com',
        data: {
            name: 'John Test',
            nickname: 'Johnny',
            aboutMe: ''
        },
        enabled: true,
        accountType: 'author'
    };
    mockAdminGetUserData(userData);
    const userInfo: UserInfo = {
        userId: 1,
        name: 'John',
        nickname: '',
        accountType: 'author'
    };
    const mockGetInfo = vi.mocked(getUserInfo);
    mockGetInfo.mockReturnValue(userInfo);

    vi.mocked(updateUserData).mockResolvedValue({
        type: 'Success',
        userId: 1
    });

    vi.mocked(adminChangePassword).mockResolvedValue({
        succeeded: true
    });

    const user = userEvent.setup();
    render(PageComponent, {
        params: {
            userId: '1'
        }
    });

    await user.type(await screen.findByTestId('newPassword'), 'P@ssword123!');
    await user.type(screen.getByTestId('repeatPassword'), 'P@ssword123!');

    await user.click(screen.getByTestId('confirm-button'));
    const confirmation = screen.getByTestId('password-change-success');

    expect(adminChangePassword).toHaveBeenCalled();
    expect(confirmation.textContent).toBe(m.successfulPasswordChange());
});
