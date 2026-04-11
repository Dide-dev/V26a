/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

import { test } from '@playwright/test';

test('home page translation', async ({ page }) => {
    await page.goto('/');
    await page.getByText('Hallo, wortel vanuit nl!');
    await page.getByText('en').click();
    await page.getByText('Hello, wortel from en!');
});
