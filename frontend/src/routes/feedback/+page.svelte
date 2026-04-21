<!--
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
-->

<script lang="ts">
    // WHEN FEEDBACK BACKEND HAS BEEN MERGED!!!
    // import { provideFeedback, type FeedbackData } from '$lib/backend/feedback';
    import { goto } from '$app/navigation';
    import { appLink } from '$lib';
    import { m } from '$lib/paraglide/messages.js';
    import { page } from '$app/state';
    import type { Option } from '$lib/backend/types';

    type FeedbackData = {
        email: string;
        feedback: string;
    };

    let feedbackData: FeedbackData = $state({
        email: '',
        feedback: ''
    });

    async function handleSubmit(event: SubmitEvent) {
        event.preventDefault();

        //await provideFeedback(feedbackData);

        // Redirection needed?
    }
</script>

<br />
<div class="flex w-full justify-center">
    <form class="flex w-md flex-col gap-6 rounded-xl bg-white p-6 shadow" onsubmit={handleSubmit}>
        <h1 class="mb-0 text-3xl font-bold">Feedback</h1>
        <p class="flex w-full justify-center">{m.feedbackExplanation()}</p>
        <label class="flex flex-col gap-1 text-sm">
            <p class="font-bold">{m.email()}</p>
            <input
                class="rounded-md border-2"
                bind:value={feedbackData.email}
                name="email"
                type="email"
                data-testid="email"
            />
        </label>
        <label class="flex flex-col gap-1 text-sm">
            <p class="font-bold">Feedback</p>
            <textarea
                rows="3"
                class="rounded-md border-2 p-2"
                bind:value={feedbackData.feedback}
                name="feedback"
                data-testid="feedback"
                required
            ></textarea>
        </label>
        <button class="rounded-sm bg-emerald-500 p-1">{m.submitFeedback()}</button>
    </form>
</div>
<br />
<br />
