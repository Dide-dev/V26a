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
        email: Option<string>;
        positive: Option<string>;
        negative: Option<string>;
        additional: Option<string>;
        grade: Option<number>;
    };

    let feedbackData: FeedbackData = $state({
        email: '',
        positive: '',
        negative: '',
        additional: '',
        grade: null as number | null
    });

    const hasFeedback = $derived(
        feedbackData.positive || feedbackData.negative || feedbackData.additional
    );

    async function handleSubmit(event: SubmitEvent) {
        event.preventDefault();

        if (!feedbackData.positive && !feedbackData.negative && !feedbackData.additional) {
            // error handling
            return;
        }

        //await provideFeedback(feedbackData);

        // Redirection needed?
    }
</script>

<br />
<div class="flex w-full justify-center">
    <form class="flex w-md flex-col gap-6 rounded-xl bg-white p-6 shadow" onsubmit={handleSubmit}>
        <p class="flex w-full justify-center">{m.feedbackExplanation()}</p>
        <label class="flex flex-col gap-1 text-sm">
            {m.email()}
            <input
                class="rounded-md border-2"
                bind:value={feedbackData.email}
                name="email"
                type="email"
                data-testid="email"
            />
        </label>
        <label class="flex flex-col gap-1 text-sm">
            {m.positiveFeedback()}
            <textarea
                rows="3"
                class="rounded-md border-2 p-2"
                bind:value={feedbackData.positive}
                name="positive"
                data-testid="positive"
            ></textarea>
        </label>
        <label class="flex flex-col gap-1 text-sm">
            {m.negativeFeedback()}
            <textarea
                rows="3"
                class="rounded-md border-2 p-2"
                bind:value={feedbackData.negative}
                name="negative"
                data-testid="negative"
            ></textarea>
        </label>
        <label class="flex flex-col gap-1 text-sm">
            {m.additionalFeedback()}
            <textarea
                rows="3"
                class="rounded-md border-2 p-2"
                bind:value={feedbackData.additional}
                name="additional"
                data-testid="additional"
            ></textarea>
        </label>

        <label>
            {m.grade()}
            <div class="grid grid-cols-10 gap-5">
                <label>
                    <input
                        class="rounded-md border-2"
                        value={1}
                        bind:group={feedbackData.grade}
                        name="grade"
                        type="radio"
                        data-testid="grade"
                    />
                    1
                </label>
                <label>
                    <input
                        class="rounded-md border-2"
                        value={2}
                        bind:group={feedbackData.grade}
                        name="grade"
                        type="radio"
                        data-testid="grade"
                    />
                    2
                </label>
                <label>
                    <input
                        class="rounded-md border-2"
                        value={3}
                        bind:group={feedbackData.grade}
                        name="grade"
                        type="radio"
                        data-testid="grade"
                    />
                    3
                </label>
                <label>
                    <input
                        class="rounded-md border-2"
                        value={4}
                        bind:group={feedbackData.grade}
                        name="grade"
                        type="radio"
                        data-testid="grade"
                    />
                    4
                </label>
                <label>
                    <input
                        class="rounded-md border-2"
                        value={5}
                        bind:group={feedbackData.grade}
                        name="grade"
                        type="radio"
                        data-testid="grade"
                    />
                    5
                </label>
                <label>
                    <input
                        class="rounded-md border-2"
                        value={6}
                        bind:group={feedbackData.grade}
                        name="grade"
                        type="radio"
                        data-testid="grade"
                    />
                    6
                </label>
                <label>
                    <input
                        class="rounded-md border-2"
                        value={7}
                        bind:group={feedbackData.grade}
                        name="grade"
                        type="radio"
                        data-testid="grade"
                    />
                    7
                </label>
                <label>
                    <input
                        class="rounded-md border-2"
                        value={8}
                        bind:group={feedbackData.grade}
                        name="grade"
                        type="radio"
                        data-testid="grade"
                    />
                    8
                </label>
                <label>
                    <input
                        class="rounded-md border-2"
                        value={9}
                        bind:group={feedbackData.grade}
                        name="grade"
                        type="radio"
                        data-testid="grade"
                    />
                    9
                </label>
                <label>
                    <input
                        class="rounded-md border-2"
                        value={10}
                        bind:group={feedbackData.grade}
                        name="grade"
                        type="radio"
                        data-testid="grade"
                    />
                    10
                </label>
            </div>
        </label>
        <button class="rounded-sm bg-emerald-500 p-1">{m.submitFeedback()}</button>
        {#if !hasFeedback}
            <p class="text-red-600">{m.noFeedback()}</p>
        {/if}
    </form>
</div>
<br />
<br />
