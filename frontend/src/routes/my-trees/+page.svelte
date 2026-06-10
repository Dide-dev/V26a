<!--
    This program has been developed by students from the bachelor Computer Science at Utrecht
    University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
-->

<script lang="ts">
    import { PaginationNav } from 'flowbite-svelte';
    import { ChevronLeftOutline, ChevronRightOutline } from 'flowbite-svelte-icons';
    import { m } from '$lib/paraglide/messages.js';
    import {
        getFilteredList,
        type TreeListResponse,
        type FilterOptions
    } from '$lib/api-wrapper/trees';
    import { onMount } from 'svelte';
    import { calcTotalPages, type PaginationOptions } from '$lib/pagination';
    import { getUserInfo } from '$lib/api-wrapper/users';
    import { redirectLogin } from '$lib/api-wrapper';
    import { page } from '$app/state';
    import TreeListEntry from '$lib/TreeListEntry.svelte';

    // This cannot be done with the {#await ...} in svelte,
    // because then updating it browser-side for tree deletion is not possible.
    let treeListResponse: TreeListResponse = $state({ totalTrees: 0, trees: [] });

    let pageSizeString = $state('10');
    let pageSizeNumber = $derived(Number(pageSizeString));
    let currentPage = $state(1);
    let totalPages = $derived(calcTotalPages(treeListResponse?.totalTrees ?? 0, pageSizeNumber));

    onMount(async () => {
        await loadTrees();
    });

    async function loadTrees() {
        let userInfo = getUserInfo();

        let filterOptions: FilterOptions = {
            ownerId: null,
            book: [],
            createdAfter: null,
            createdBefore: null,
            showOwnPrivate: true,
            editable: true
        };
        let paginationOptions: PaginationOptions = {
            page: currentPage,
            size: pageSizeNumber
        };
        if (userInfo !== null) {
            treeListResponse = await getFilteredList(paginationOptions, filterOptions);
        } else {
            await redirectLogin(page.url);
        }
    }

    function changePageSize() {
        // Update the length of the description shown array, and reset values to false.
        if (pageSizeNumber > entriesExpanded.length) {
            entriesExpanded = Array.from({ length: pageSizeNumber }, () => false);
        }
        // Move to page 1, to prevent out-of-range page errors.
        handlePageChange(1);
    }

    function handlePageChange(page: number) {
        currentPage = page;
        // Reset the shown description to false.
        entriesExpanded.fill(false);
        loadTrees();
    }

    let entriesExpanded = $state(Array.from({ length: 10 }, () => false));
</script>

<div class="flex w-full justify-center">
    <div class="flex-col">
        <!-- Change the number of trees per page -->
        <label>
            {m.numberPerPage({ n: m.treesUncapitalized() })}
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

<div class="flex w-7/8 max-w-xl flex-col">
    {#if treeListResponse?.trees === null}
        <p>{m.loading()}</p>
    {:else if treeListResponse?.trees.length === 0}
        <div class="text-center">
            <p class="flex justify-center text-2xl">{m.noAllTreesFound()}</p>
        </div>
    {:else}
        <div class="flex w-full flex-col items-center justify-center overflow-auto">
            <div class="mb-4 flex w-full flex-col border-b border-neutral-200">
                {#each treeListResponse?.trees as tree, i (tree.genData.id)}
                    <TreeListEntry {tree} index={i} bind:entriesExpanded />
                {/each}
            </div>

            <PaginationNav {currentPage} {totalPages} onPageChange={handlePageChange}>
                {#snippet prevContent()}
                    <span class="sr-only">{m.previousPage()}</span>
                    <ChevronLeftOutline class="h-5 w-5" />
                {/snippet}
                {#snippet nextContent()}
                    <span class="sr-only">{m.nextPage()}</span>
                    <ChevronRightOutline class="h-5 w-5" />
                {/snippet}
            </PaginationNav>
        </div>
    {/if}
</div>
