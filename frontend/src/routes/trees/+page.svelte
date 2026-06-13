<!--
    This program has been developed by students from the bachelor Computer Science at Utrecht
    University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
-->
<script lang="ts">
    import { m } from '$lib/paraglide/messages.js';
    import { getLocale } from '$lib/paraglide/runtime';
    import {
        type FilterOptions,
        type TreeListResponse,
        getFilteredList
    } from '$lib/api-wrapper/trees';
    import { onMount } from 'svelte';
    import { calcTotalPages, type PaginationOptions } from '$lib/pagination';
    import TreeListEntry from '$lib/lists/TreeListEntry.svelte';
    import ListPagination from '$lib/lists/ListPagination.svelte';
    import TreeFilters from '$lib/lists/TreeFilters.svelte';

    let treeListResponse = $state(null as null | TreeListResponse);

    let pageSize = $state(10);
    let currentPage = $state(1);
    let totalPages = $derived(calcTotalPages(treeListResponse?.totalTrees ?? 0, pageSize));

    let filterOptions: FilterOptions = $state({
        ownerName: null,
        searchTerms: null,
        book: [],
        tag: [],
        languageCode: getLocale(),
        subject: null,
        createdAfter: null,
        createdBefore: null,
        showOwnPrivate: true,
        editable: null,
        showPrivate: false
    });
    let paginationOptions: PaginationOptions = $derived({
        page: currentPage,
        size: pageSize
    });

    onMount(async () => {
        await loadTrees();
    });

    async function loadTrees() {
        if (filterOptions.searchTerms !== null && filterOptions.searchTerms.trim().length === 0) {
            filterOptions.searchTerms = null;
        }

        treeListResponse = await getFilteredList(paginationOptions, filterOptions);
    }

    function handlePageSizeChange() {
        // Update the length of the description shown array, and reset values to false.
        if (pageSize > entriesExpanded.length) {
            entriesExpanded = Array.from({ length: pageSize }, () => false);
        }
        // Move to page 1, to prevent out-of-range page errors.
        currentPage = 1;
        loadTrees();
    }

    function handlePageChange() {
        // Reset the expanded flags to false.
        entriesExpanded.fill(false);
        loadTrees();
    }

    let entriesExpanded = $state(Array.from({ length: 10 }, () => false));
</script>

<ListPagination
    bind:currentPage
    {totalPages}
    {handlePageChange}
    bind:pageSize
    {handlePageSizeChange}
    handleApplyFilters={loadTrees}
>
    {#snippet title()}
        <h1 class="page__title">{m.allTrees()}</h1>
    {/snippet}
    {#if treeListResponse?.trees === null}
        <p>{m.loading()}</p>
    {:else if treeListResponse?.trees.length === 0}
        <p class="m-4 font-semibold italic">{m.noAllTreesFound()}</p>
    {:else}
        <div class="list__container">
            {#each treeListResponse?.trees as tree, i (tree.genData.id)}
                <TreeListEntry {tree} bind:expanded={entriesExpanded[i]} />
            {/each}
        </div>
    {/if}
    {#snippet filterFields()}
        <TreeFilters bind:filterOptions />
    {/snippet}
</ListPagination>
