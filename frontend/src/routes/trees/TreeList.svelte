<!--
    This program has been developed by students from the bachelor Computer Science at Utrecht
    University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
-->
<script lang="ts">
    import { m } from '$lib/paraglide/messages.js';
    import {
        type FilterOptions,
        type TreeListResponse,
        getFilteredList,
        getPaginationAndFilterParams
    } from '$lib/api-wrapper/trees';
    import { onMount } from 'svelte';
    import { calcTotalPages, type PaginationOptions } from '$lib/pagination';
    import TreeListEntry from '$lib/lists/TreeListEntry.svelte';
    import ListContainer from '$lib/lists/ListContainer.svelte';
    import TreeFilters from '$lib/lists/TreeFilters.svelte';
    import { page } from '$app/state';
    import { getUserInfo } from '$lib/api-wrapper/sessions';
    import { appLink, booleanFromString } from '$lib';
    import { replaceState } from '$app/navigation';
    import { PlusOutline } from 'flowbite-svelte-icons';

    let {
        filterOptions,
        titleText,
        linkToEditor = false,
        onlyOwnTrees = false
    }: {
        filterOptions: FilterOptions;
        titleText: string;
        linkToEditor?: boolean;
        onlyOwnTrees?: boolean;
    } = $props();

    let userInfo = getUserInfo();

    let treeListResponse = $state(null as null | TreeListResponse);

    let pageSize = $state(10);
    let currentPage = $state(1);
    let totalPages = $derived(calcTotalPages(treeListResponse?.totalTrees ?? 0, pageSize));

    let paginationOptions: PaginationOptions = $derived({
        page: currentPage,
        size: pageSize
    });

    onMount(() => {
        parseParams();

        loadTrees();
    });

    function parseParams() {
        let params: Record<string, string[]> = {};

        for (const field in paginationOptions) {
            params[field] = page.url.searchParams.getAll(field);
        }

        for (const field in filterOptions) {
            params[field] = page.url.searchParams.getAll(field);
        }

        // Set pagination options.
        if (params.page.length > 0) currentPage = Number(params.page[0]);
        if (params.size.length > 0) pageSize = Number(params.size[0]);

        // Set filter options.
        if (params.ownerName.length > 0) filterOptions.ownerName = params.ownerName[0];
        if (params.searchTerms.length > 0) filterOptions.searchTerms = params.searchTerms[0];
        if (params.book.length > 0) filterOptions.book = params.book;
        if (params.tag.length > 0) filterOptions.tag = params.tag;
        if (params.languageCode.length > 0) filterOptions.languageCode = params.languageCode[0];
        if (params.subject.length > 0) filterOptions.subject = params.subject[0];
        if (params.createdAfter.length > 0)
            filterOptions.createdAfter = new Date(params.createdAfter[0]);
        if (params.createdBefore.length > 0)
            filterOptions.createdBefore = new Date(params.createdBefore[0]);
        if (params.showOwnPrivate.length > 0)
            filterOptions.showOwnPrivate = booleanFromString(params.showOwnPrivate[0]);

        if (userInfo?.accountType === 'admin' && params.showPrivate.length > 0)
            filterOptions.showPrivate = booleanFromString(params.showPrivate[0]);
    }

    async function handleFilterUpdate() {
        await applyFilters();
    }

    async function applyFilters() {
        // Reset page to 1, to avoid out of range errors.
        currentPage = 1;
        await loadTrees();
        // Update search parameters.
        replaceState(`?${getPaginationAndFilterParams(paginationOptions, filterOptions)}`, {});
    }

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
        // Update search parameters.
        replaceState(`?${getPaginationAndFilterParams(paginationOptions, filterOptions)}`, {});
    }

    function handlePageChange() {
        // Reset the expanded flags to false.
        entriesExpanded.fill(false);
        loadTrees();
        // Update search parameters.
        replaceState(`?${getPaginationAndFilterParams(paginationOptions, filterOptions)}`, {});
    }

    function handleDelete() {
        if (treeListResponse?.trees.length === 1 && currentPage > 1) {
            currentPage -= 1;
        }

        loadTrees();
    }

    let entriesExpanded = $state(Array.from({ length: 10 }, () => false));
</script>

<ListContainer
    bind:currentPage
    {totalPages}
    {handlePageChange}
    bind:pageSize
    {handlePageSizeChange}
    handleApplyFilters={applyFilters}
>
    {#snippet title()}
        <h1 class="page__title flex gap-2 *:*:inline">
            {titleText}
            <a
                title={m.addTree()}
                aria-label={m.addTree()}
                class="mr-2.5 cursor-pointer transition-all duration-200 ease-in-out hover:bg-neutral-200"
                href={appLink(`/tree-generator`)}
                target="_blank"
            >
                <PlusOutline class="h-7 w-7 shrink-0" />
            </a>
        </h1>
    {/snippet}
    {#if treeListResponse?.trees === null}
        <p>{m.loading()}</p>
    {:else if treeListResponse?.trees.length === 0}
        <p class="m-4 font-semibold italic">{m.noAllTreesFound()}</p>
    {:else}
        <div class="list__container">
            {#each treeListResponse?.trees as tree, i (tree.genData.id)}
                <TreeListEntry
                    {tree}
                    bind:expanded={entriesExpanded[i]}
                    {linkToEditor}
                    {handleDelete}
                    usedForClassroom={false}
                />
            {/each}
        </div>
    {/if}
    {#snippet filterFields()}
        <TreeFilters bind:filterOptions {handleFilterUpdate} {onlyOwnTrees} />
    {/snippet}
</ListContainer>
