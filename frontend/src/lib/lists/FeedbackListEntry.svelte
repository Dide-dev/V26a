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
                <span class="flex-auto grow-100"></span>
            </span>
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
        </div>
        <div class="max-w-1/3 text-right">
            <p class="font-semibold">{timeFormat.format(new Date(feedback.createdAt))}</p>
        </div>
    </div>
    <div class={[(expanded || feedback.feedback.length <= 50) && 'inline', !expanded && 'flex']}>
        <span class={[!expanded && 'line-clamp-1', 'w-fit flex-1 text-ellipsis']}>
            {feedback.feedback}
        </span>
        {#if feedback.feedback.length > 50}
            <button
                onclick={() => {
                    expanded = !expanded;
                }}
                class={[feedback.feedback.length <= 50 && 'block', 'text__link']}
            >
                {expanded ? m.showLess() : m.showMore()}
            </button>
        {/if}
    </div>
</div>
