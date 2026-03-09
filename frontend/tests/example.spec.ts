import { test, expect } from '@playwright/test';

test('demo pages', async ({ page }) => {
    async function getJsonCounterValue(): number {
        const text = await page.getByText('Received:').textContent();
        const json = JSON.parse(text.split('Received: ')[1]);
        return json.b;
    }

    async function getTextCounterValue(): number {
        const counterText = await page.getByText('Current count:').textContent();
        return parseInt(counterText.split('Current count: ')[1]);
    }

    await page.goto('/api-testing');
    const jsonCounter = await getJsonCounterValue();

    await page.goto('/counter');
    const textCounter = await getTextCounterValue();
    expect(jsonCounter).toEqual(textCounter);

    await page.getByText('Increment!').click();
    await page.getByText('Increment by 100_000_000!').click();
    expect(await getTextCounterValue()).toEqual(textCounter + 100000001);

    await page.goto('/api-testing');
    expect(await getJsonCounterValue()).toEqual(textCounter + 100000001);
});
