<!--
    This program has been developed by students from the bachelor Computer Science at Utrecht
    University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
-->
<script lang="ts">
    import { m } from '$lib/paraglide/messages.js';
    import { getFeedback, type FeedbackListResponse } from '$lib/api-wrapper/admin';
    import { onMount } from 'svelte';
    import { calcTotalPages, type PaginationOptions } from '$lib/pagination';
    import ListPagination from '$lib/lists/ListPagination.svelte';
    import FeedbackListEntry from '$lib/lists/FeedbackListEntry.svelte';

    let feedbackListResponse = $state(null as null | FeedbackListResponse);

    let pageSize = $state(10);
    let currentPage = $state(1);
    let totalPages = $derived(calcTotalPages(feedbackListResponse?.feedbackTotal ?? 0, pageSize));

    let entriesExpanded = $state(Array.from({ length: 10 }, () => false));

    let paginationOptions: PaginationOptions = $derived({
        page: currentPage,
        size: pageSize
    });

    onMount(async () => {
        await loadFeedback();
    });

    async function loadFeedback() {
        feedbackListResponse = await getFeedback(paginationOptions);
    }

    function handlePageChange() {
        loadFeedback();
    }

    function handlePageSizeChange() {
        currentPage = 1;
        loadFeedback();
    }
</script>

<ListPagination
    bind:currentPage
    {totalPages}
    {handlePageChange}
    bind:pageSize
    {handlePageSizeChange}
>
    {#snippet title()}
        <h1 class="page__title">{m.feedbackList()}</h1>
    {/snippet}

    {#if feedbackListResponse?.feedback === null}
        <p>{m.loading()}</p>
    {:else if feedbackListResponse?.feedback.length === 0}
        <p>{m.noFeedbackFound()}</p>
    {:else}
        <div class="list__container">
            {#each feedbackListResponse?.feedback as feedback, i (feedback)}
                <FeedbackListEntry {feedback} expanded={entriesExpanded[i]} />
            {/each}
        </div>
    {/if}
</ListPagination>
