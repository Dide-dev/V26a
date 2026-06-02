/*
    This program has been developed by students from the bachelor Computer Science at Utrecht
    University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

import { render, screen } from '@testing-library/svelte';
import { test, vi, afterEach, expect } from 'vitest';
import userEvent from '@testing-library/user-event';
import page from './+page.svelte';
import { provideFeedback } from '$lib/api-wrapper/feedback';

vi.mock('$lib/api-wrapper/feedback', () => {
    return {
        provideFeedback: vi.fn()
    };
});
afterEach(() => {
    vi.resetAllMocks();
});

test('Feedback submission with feedback and correct email', async () => {
    const user = userEvent.setup();
    render(page);

    const emailInput = screen.getByTestId('email');
    await user.type(emailInput, 'test@email.nl');
    const feedbackInput = screen.getByTestId('feedback');
    await user.type(feedbackInput, 'Feedback');

    await user.click(screen.getByTestId('submit-feedback-button'));

    expect(provideFeedback).toHaveBeenCalled();
});

test('Feedback submission without email', async () => {
    const user = userEvent.setup();
    render(page);

    const feedbackInput = screen.getByTestId('feedback');
    await user.type(feedbackInput, 'Feedback');

    await user.click(screen.getByTestId('submit-feedback-button'));

    expect(provideFeedback).toHaveBeenCalled();
});

test('No feedback submission without feedback', async () => {
    const user = userEvent.setup();
    render(page);

    await user.click(screen.getByTestId('submit-feedback-button'));

    expect(provideFeedback).not.toHaveBeenCalled();
});
