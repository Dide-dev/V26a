import { userEvent } from '@testing-library/user-event';
import { render, waitFor, screen } from '@testing-library/svelte';
import { expect, test, vi } from 'vitest';
import page from './+page.svelte';

test('fetching counter', async () => {
    global.fetch = vi.fn().mockImplementation(() => {
        return Promise.resolve(Response.json({ a: '', b: 67 }));
    });

    render(page);
    await waitFor(() => {
        expect(() => screen.getByText('Current count: 67')).not.toThrow();
    });
});

test('incrementing counter', async () => {
    let counter = 0;
    global.fetch = vi.fn().mockImplementation((path: string) => {
        if (path === '/api/v1/demo/json') {
            return Promise.resolve(Response.json({ a: '', b: counter }));
        }
        if (path === '/api/v1/demo/increment') {
            counter++;
            return Promise.resolve(Response.json({}));
        }
    });

    const user = userEvent.setup();

    render(page);

    let button = await waitFor(() => screen.getByText('Increment!'));
    await user.click(button);
    await waitFor(() => {
        expect(() => screen.getByText('Current count: 1')).not.toThrow();
    });
});

test('adding to counter', async () => {
    let counter = 0;
    global.fetch = vi.fn().mockImplementation((path: string, options: any) => {
        if (path === '/api/v1/demo/json') {
            return Promise.resolve(Response.json({ a: '', b: counter }));
        }
        if (path === '/api/v1/demo/add-to-counter') {
            const body = JSON.parse(options.body);
            counter += body.add;
            return Promise.resolve(Response.json({}));
        }
    });

    const user = userEvent.setup();

    render(page);

    let button = await waitFor(() => screen.getByText('Increment by 100_000_000!'));
    await user.click(button);
    await waitFor(() => {
        expect(() => screen.getByText('Current count: 100000000')).not.toThrow();
    });
});
