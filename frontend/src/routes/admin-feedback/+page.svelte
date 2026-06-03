<!--
    This program has been developed by students from the bachelor Computer Science at Utrecht
    University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
-->
<script lang="ts">
    import { m } from '$lib/paraglide/messages.js';
    import { getFeedback, type FeedbackListResponse } from '$lib/api-wrapper/admin';
    import { timeFormatString } from '$lib';
    import { onMount } from 'svelte';
    import { calcTotalPages, type PaginationOptions } from '$lib/pagination';

    let timeFormat = new Intl.DateTimeFormat(timeFormatString);
    let pageSizeString = $state('10');
    let pageSizeNumber = $derived(Number(pageSizeString));
    let thisPage = $state(1);

    let paginationOptions: PaginationOptions = $derived({
        page: thisPage,
        size: pageSizeNumber
    });

    let feedbackListResponse: null | FeedbackListResponse = $state(null);
    let expanded = $state<Record<number, boolean>>({});

    function getFeedbackText(feedback: string, expanded: boolean) {
        return expanded || feedback.length <= 200 ? feedback : feedback.slice(0, 200) + '...';
    }

    let prevPageExists = $derived(thisPage > 1);
    let nextPageExists = $derived.by(() => {
        // For some reason svelte does not understand the type of feedbackListResponse so we have to manually cast it.
        const total = (feedbackListResponse as FeedbackListResponse | null)?.feedbackTotal ?? 0;
        return thisPage < calcTotalPages(total, pageSizeNumber);
    });

    onMount(async () => {
        await loadFeedback();
    });

    async function loadFeedback() {
        feedbackListResponse = await getFeedback(paginationOptions);
    }

    function changePageSize() {
        thisPage = 1;
        loadFeedback();
    }

    function prevPage() {
        thisPage -= 1;
        loadFeedback();
    }

    function nextPage() {
        thisPage += 1;
        loadFeedback();
    }
</script>

<div class="flex w-full justify-center">
    <div class="flex-col">
        <!-- Change the number of feedback entries per page -->
        <label>
            {m.numberPerPage({ n: m.feedbackEntries() })}
            <select
                class="m-1 w-15 rounded-sm bg-neutral-200 p-1"
                bind:value={pageSizeString}
                onchange={changePageSize}
            >
                <option value="5">5</option>
                <option value="10">10</option>
                <option value="20">20</option>
                <option value="50">50</option>
            </select>
        </label>
    </div>
</div>

{#if feedbackListResponse?.feedback === null}
    <p>{m.loading()}</p>
{:else if feedbackListResponse?.feedback.length === 0}
    <div class="text-center">
        <p class="flex justify-center text-2xl">{m.noFeedbackFound()}</p>
    </div>
{:else}
    <div class="100vw flex flex-col items-center justify-center overflow-auto">
        <table class="border-separate border-spacing-2">
            <thead>
                <tr>
                    <th class="text-left">{m.email()}</th>
                    <th class="text-center">Feedback</th>
                    <th class="text-center">{m.feedbackAddDate()}</th>
                </tr>
            </thead>
            <tbody>
                {#each feedbackListResponse?.feedback as feedback (feedback.feedbackId)}
                    <tr>
                        <td class="min-w-10 pr-4 align-text-top wrap-anywhere">
                            {#if feedback.email == '' || feedback.email == null}
                                -
                            {:else}
                                <a
                                    class="hover:text-blue-800 hover:underline"
                                    href="mailto:{feedback.email}">{feedback.email}</a
                                >
                            {/if}
                        </td>
                        <td class="max-w-125 pr-4 pl-4 align-text-top wrap-anywhere">
                            {getFeedbackText(feedback.feedback, expanded[feedback.feedbackId])}
                            {#if feedback.feedback.length > 200}
                                <button
                                    onclick={() => {
                                        expanded[feedback.feedbackId] =
                                            !expanded[feedback.feedbackId];
                                    }}
                                    class="text__link"
                                >
                                    {expanded[feedback.feedbackId] ? m.showLess() : m.showMore()}
                                </button>
                            {/if}</td
                        >
                        <td class="pr-4 align-text-top">
                            <!-- This formats the date in the local timezone in the form: dd-mm-yyyy -->
                            {timeFormat.format(new Date(feedback.createdAt))}
                        </td>
                    </tr>
                {/each}
            </tbody>
        </table>
        <div>
            <button
                onclick={prevPage}
                disabled={!prevPageExists}
                class="mx-5"
                class:cursor-pointer={prevPageExists}>Prev</button
            >
            {thisPage}/{calcTotalPages(feedbackListResponse?.feedbackTotal ?? 0, pageSizeNumber)}
            <button
                onclick={nextPage}
                disabled={!nextPageExists}
                class="mx-5"
                class:cursor-pointer={nextPageExists}>Next</button
            >
        </div>
    </div>
{/if}
