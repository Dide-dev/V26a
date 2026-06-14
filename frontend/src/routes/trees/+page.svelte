<!--
    This program has been developed by students from the bachelor Computer Science at Utrecht
    University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
-->
<script lang="ts">
    import { Datepicker } from 'flowbite-svelte';
    import { m } from '$lib/paraglide/messages.js';
    import {
        type FilterOptions,
        type TreeListResponse,
        getFilteredList
    } from '$lib/api-wrapper/trees';
    import { timeFormatString } from '$lib';
    import { onMount } from 'svelte';
    import { calcTotalPages, type PaginationOptions } from '$lib/pagination';
    import TreeListEntry from '$lib/lists/TreeListEntry.svelte';
    import ListPagination from '$lib/lists/ListPagination.svelte';

    let treeListResponse = $state(null as null | TreeListResponse);

    let pageSize = $state(10);
    let currentPage = $state(1);
    let totalPages = $derived(calcTotalPages(treeListResponse?.totalTrees ?? 0, pageSize));

    let filterOptions: FilterOptions = $state({
        searchTerms: null,
        ownerId: null,
        book: [],
        createdAfter: null,
        createdBefore: null,
        showOwnPrivate: true,
        editable: null
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

<div class="flex w-7/8 max-w-xl flex-col">
    <h1 class="page__title">{m.allTrees()}</h1>

    <div class="mb-4 flex-col">
        <!-- Filter options -->
        <form class="completed flex w-full flex-col gap-2" onsubmit={loadTrees}>
            <b>{m.creationDate()}</b>
            <div class="flex">
                <label class="m-1 flex-1/2">
                    {m.from()}
                    <Datepicker
                        bind:value={
                            () => filterOptions.createdAfter ?? undefined,
                            (v) => (filterOptions.createdAfter = v ?? null)
                        }
                        locale={timeFormatString}
                        translationLocale={timeFormatString}
                        placeholder={m.selectDate()}
                    />
                </label>
                <label class="m-1 flex-1/2">
                    {m.through()}
                    <Datepicker
                        bind:value={
                            () => filterOptions.createdBefore ?? undefined,
                            (v) =>
                                (filterOptions.createdBefore = v
                                    ? new Date(v.getTime() + 24 * 60 * 60 * 1000 - 1) // Add 23:59:59.999 to make this date inclusive
                                    : null)
                        }
                        locale={timeFormatString}
                        translationLocale={timeFormatString}
                        placeholder={m.selectDate()}
                    />
                </label>
            </div>
            <label class="m-1">
                {m.ownerId()}
                <input
                    class="rounded-md border-2"
                    type="number"
                    bind:value={filterOptions.ownerId}
                />
            </label>
            <label class="m-1">
                {m.searchTerms()}
                <input class="rounded-md border-2" bind:value={filterOptions.searchTerms} />
            </label>
            <button type="submit">{m.applyFilters()}</button>
        </form>
    </div>

    {#if treeListResponse?.trees === null}
        <p>{m.loading()}</p>
    {:else if treeListResponse?.trees.length === 0}
        <p class="mt-4 font-semibold italic">{m.noAllTreesFound()}</p>
    {:else}
        <div class="flex w-full flex-col items-center justify-center overflow-auto">
            <ListPagination
                bind:currentPage
                {totalPages}
                {handlePageChange}
                bind:pageSize
                {handlePageSizeChange}
            >
                <div class="list__container">
                    {#each treeListResponse?.trees as tree, i (tree.genData.id)}
                        <TreeListEntry {tree} bind:expanded={entriesExpanded[i]} />
                    {/each}
                </div>
            </ListPagination>
        </div>
    {/if}
</div>
