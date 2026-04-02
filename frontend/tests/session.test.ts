/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

import { test, expect, type Page } from '@playwright/test';

/** Will only use ascii alphanumerical characters in resulting string. */
function generateRandomString(length: number): string {
    let result = '';
    const chars = 'abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789';
    for (let i = 0; i < length; i++) {
        result += chars[Math.floor(Math.random() * chars.length)];
    }
    return result;
}

test('check that login creates user_info cookie', async ({ page }) => {
    // The email is randomized, because playwright runs multiple browsers for the tests.
    // Between tests the database is not reset so that would result in failure to register with the same email.
    const email = `test123+${generateRandomString(20)}@gmail.com`;
    const password = 'Monkey123!';

    // Function that fills in the login UI
    async function login(page: Page, email: string, password: string) {
        await page.goto('/login');
        await page.getByTestId('email').fill(email);
        await page.getByTestId('password').fill(password);
        await page.getByRole('button').click();
        await page.waitForLoadState('networkidle');
    }

    // Function that fills in the register UI
    async function register(page: Page, email: string, password: string) {
        await page.goto('/registration');

        await page.getByTestId('email').fill(email);
        await page.getByTestId('name').fill('Test User');
        await page.getByTestId('password').fill(password);
        await page.getByTestId('repeat password').fill(password);
        await page.getByRole('button').click();

        await page.goto('/login');
    }

    // Set the cookie
    async function getUserInfoCookie(page: Page) {
        const cookies = await page.context().cookies();
        return cookies.find((c) => c.name === 'user_info');
    }

    await register(page, email, password);
    await login(page, email, password);
    const cookie = await getUserInfoCookie(page);

    // Validate that the cookie exists
    expect(cookie).toBeDefined();

    // Checks if the content of the cookie is the expected default (author)
    const userInfo = JSON.parse(decodeURIComponent(cookie!.value));
    expect(userInfo.sub).toBeDefined();
    expect(userInfo.accountType).toBe('author');
});
