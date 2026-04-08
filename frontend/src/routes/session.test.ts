import { expect, test, beforeEach } from 'vitest';
import { getUserInfo } from '$lib/backend/users';

beforeEach(() => {
    // reset cookie for each test
    Object.defineProperty(document, 'cookie', {
        writable: true,
        value: ''
    });
});

test('getUserInfo returns null when cookie is empty', () => {
    Object.defineProperty(document, 'cookie', {
        writable: true,
        value: ''
    });
    const result = getUserInfo();
    expect(result).toBeNull();
});

test('getUserInfo returns null when no cookie exists', () => {
    const result = getUserInfo();
    expect(result).toBeNull();
});

test('getUserInfo returns userinfo when cookie exists', () => {
    const userInfo = { userId: 1, accountType: 'author' };
    Object.defineProperty(document, 'cookie', {
        writable: true,
        value: `user_info=${encodeURIComponent(JSON.stringify(userInfo))}`
    });
    const result = getUserInfo();
    expect(result).toEqual(userInfo);
});
