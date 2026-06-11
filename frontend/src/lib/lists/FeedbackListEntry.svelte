<!--
    This program has been developed by students from the bachelor Computer Science at Utrecht
    University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
-->

<!--
    @component
    Render the data of a tree for in a list.

    ## Params
    - `tree: TreeMetadata` - The data of the tree to display.
    - `expanded: boolean` - Bindable flag whether the entry displays all its contents.
-->

<script lang="ts">
    import { m } from '$lib/paraglide/messages.js';
    import { timeFormatString } from '$lib';
    import type { FeedbackGetData } from '$lib/api-wrapper/admin';

    let timeFormat = new Intl.DateTimeFormat(timeFormatString);

    /** Approximation of a good cut-off point for feedback text, depending on window width. */
    let textLimit = $state(Math.min(50, window.innerWidth / 12));

    let {
        feedback,
        expanded = $bindable()
    }: {
        feedback: FeedbackGetData;
        expanded: boolean;
    } = $props();
</script>

<div class="list__entry">
    <div class="flex w-full flex-row justify-between gap-2">
        <div class="flex max-w-2/3 flex-col">
            <span class="flex gap-1 *:*:inline">
                <span class="font-semibold text-neutral-500">#{feedback.feedbackId}</span>
                <span class="flex grow-100"></span>
            </span>
            {#if feedback.email !== ''}
                <em class="-mt-1 text-neutral-700">
                    {m.leftBy()}
                    <a
                        href={`mailto:${feedback.email}`}
                        target="_blank"
                        rel="external"
                        class="text__link"
                    >
                        {feedback.email}
                    </a>
                </em>
            {/if}
        </div>
        <div class="max-w-1/3 text-right">
            <p class="font-semibold">{timeFormat.format(new Date(feedback.createdAt))}</p>
        </div>
    </div>
    <div
        class={[
            (expanded || feedback.feedback.length <= textLimit) && 'inline',
            !expanded && 'flex'
        ]}
    >
        <span
            id="collapsible"
            class={[!expanded && 'line-clamp-1', 'flex-1 wrap-break-word text-ellipsis']}
        >
            {feedback.feedback}
        </span>
        {#if feedback.feedback.length > textLimit}
            <button
                onclick={() => {
                    expanded = !expanded;
                }}
                class={[feedback.feedback.length <= textLimit && '', 'text__link']}
            >
                {expanded ? m.showLess() : m.showMore()}
            </button>
        {/if}
    </div>
</div>

<svelte:window
    onresize={() => {
        textLimit = Math.min(50, window.innerWidth / 12);
    }}
/>
