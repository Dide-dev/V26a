<!--
    This program has been developed by students from the bachelor Computer Science at Utrecht
    University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
-->

<script lang="ts">
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
    import TreeListEntry from '$lib/lists/TreeListEntry.svelte';
    import ListPagination from '$lib/lists/ListPagination.svelte';

    // This cannot be done with the {#await ...} in svelte,
    // because then updating it browser-side for tree deletion is not possible.
    let treeListResponse: TreeListResponse = $state({ totalTrees: 0, trees: [] });

    let pageSize = $state(10);
    let currentPage = $state(1);
    let totalPages = $derived(calcTotalPages(treeListResponse?.totalTrees ?? 0, pageSize));

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
            size: pageSize
        };
        if (userInfo !== null) {
            treeListResponse = await getFilteredList(paginationOptions, filterOptions);
        } else {
            await redirectLogin(page.url);
        }
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
    {#if treeListResponse?.trees === null}
        <p>{m.loading()}</p>
    {:else if treeListResponse?.trees.length === 0}
        <div class="text-center">
            <p class="flex justify-center text-2xl">{m.noAllTreesFound()}</p>
        </div>
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
