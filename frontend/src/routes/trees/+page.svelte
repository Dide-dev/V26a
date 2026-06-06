<!--
    This program has been developed by students from the bachelor Computer Science at Utrecht
    University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
-->
<script lang="ts">
    import { Datepicker, PaginationNav } from 'flowbite-svelte';
    import {
        ArrowUpRightFromSquareOutline,
        ChevronLeftOutline,
        ChevronRightOutline,
        EditOutline,
        FileCloneOutline
    } from 'flowbite-svelte-icons';
    import { m } from '$lib/paraglide/messages.js';
    import {
        type FilterOptions,
        type TreeListResponse,
        getFilteredList,
        duplicateTree,
        type TreeMetadata
    } from '$lib/api-wrapper/trees';
    import { appLink, timeFormatString } from '$lib';
    import { onMount } from 'svelte';
    import { calcTotalPages, type PaginationOptions } from '$lib/pagination';
    import { getUserInfo } from '$lib/api-wrapper/users';
    import AnimatedUnderline from '$lib/AnimatedUnderline.svelte';

    let timeFormat = new Intl.DateTimeFormat(timeFormatString);
    let userInfo = $state(getUserInfo());

    let treeListResponse = $state(null as null | TreeListResponse);

    let pageSizeString = $state('10');
    let pageSizeNumber = $derived(Number(pageSizeString));
    let currentPage = $state(1);
    let totalPages = $derived(calcTotalPages(treeListResponse?.totalTrees ?? 0, pageSizeNumber));

    let filterOptions: FilterOptions = $state({
        ownerId: null,
        book: [],
        createdAfter: null,
        createdBefore: null,
        showOwnPrivate: true,
        editable: null
    });
    let paginationOptions: PaginationOptions = $derived({
        page: currentPage,
        size: pageSizeNumber
    });

    onMount(async () => {
        await loadTrees();
    });

    async function duplicateThisTree(id: number) {
        let newTreeId = await duplicateTree(id);
        await window.open(appLink(`/tree-page/editor?id=${newTreeId}`));
    }

    async function loadTrees() {
        treeListResponse = await getFilteredList(paginationOptions, filterOptions);
    }

    function changePageSize() {
        // Update the length of the description shown array, and reset values to false.
        if (pageSizeNumber > descriptionShown.length) {
            descriptionShown = Array.from({ length: pageSizeNumber }, () => false);
        }
        // Move to page 1, to prevent out-of-range page errors.
        handlePageChange(1);
    }

    function handlePageChange(page: number) {
        currentPage = page;
        // Reset the shown description to false.
        descriptionShown.fill(false);
        loadTrees();
    }

    function showBookName(bookName: string | null): string {
        if (bookName == '') {
            return '-';
        } else if (bookName == null) {
            return '-';
        } else {
            return bookName;
        }
    }

    let descriptionShown = $state(Array.from({ length: 10 }, () => false));
</script>

<div class="flex w-7/8 max-w-xl flex-col">
    <div class="flex-col">
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
            <button type="submit">{m.applyFilters()}</button>
        </form>

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
                    {@render treeEntry(tree, i)}
                {/each}
            </div>

            <!--        <table class="border-separate border-spacing-2">-->
            <!--            <thead>-->
            <!--                <tr>-->
            <!--                    <th class="text-left">{m.title()}</th>-->
            <!--                    <th class="text-left">{m.book()}</th>-->
            <!--                    <th class="text-left">{m.timesVisited()}</th>-->
            <!--                    <th class="text-left">{m.createdAt()}</th>-->
            <!--                </tr>-->
            <!--            </thead>-->
            <!--            <tbody>-->
            <!--                {#each treeListResponse?.trees as tree (tree.genData.id)}-->
            <!--                    <tr>-->
            <!--                        <td class="max-w-96 pr-4 wrap-anywhere hover:text-blue-800 hover:underline">-->
            <!--                            <a-->
            <!--                                href={appLink(`/tree-page/viewer?id=${tree.genData.id}`)}-->
            <!--                                target="_blank">{tree.data.title}</a-->
            <!--                            >-->
            <!--                        </td>-->
            <!--                        <td class="pr-4 wrap-anywhere">{showBookName(tree.data.bookName)}</td>-->
            <!--                        <td class="pr-4">{tree.genData.timesVisited}</td>-->
            <!--                        <td class="pr-4">-->
            <!--                            &lt;!&ndash; This formats the date in the local timezone in the form: dd-mm-yyyy &ndash;&gt;-->
            <!--                            {timeFormat.format(new Date(tree.genData.createdAt))}-->
            <!--                        </td>-->
            <!--                        {#if userInfo !== null}-->
            <!--                            <td class="min-w-2 hover:fill-blue-800">-->
            <!--                                <button-->
            <!--                                    onclick={() => duplicateThisTree(tree.genData.id)}-->
            <!--                                    aria-label="duplicate"-->
            <!--                                    class:cursor-pointer={true}-->
            <!--                                    class="flex self-end"-->
            <!--                                >-->
            <!--                                    &lt;!&ndash; eslint-disable-next-line svelte/no-at-html-tags &ndash;&gt;-->
            <!--                                    {@html iconDuplicate}-->
            <!--                                </button>-->
            <!--                            </td>-->
            <!--                        {/if}-->
            <!--                    </tr>-->
            <!--                {/each}-->
            <!--            </tbody>-->
            <!--        </table>-->

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

{#snippet treeEntry(tree: TreeMetadata, index: number)}
    <div
        class={[
            'flex w-full flex-col gap-1',
            'border-t border-neutral-200 bg-neutral-50 p-2 transition-colors duration-200 hover:bg-neutral-100'
        ]}
    >
        <div class="flex w-full flex-row justify-between gap-2">
            <div class="flex max-w-2/3 flex-col">
                <span class="flex gap-1 *:*:inline">
                    <AnimatedUnderline --weight-underline="calc(var(--spacing) / 2)">
                        <a
                            href={appLink(`/tree-page/viewer?id=${tree.genData.id}`)}
                            target="_blank"
                            class="line-clamp-1 w-fit flex-1 break-all text-ellipsis"
                        >
                            <strong>{tree.data.title}</strong>
                        </a>
                    </AnimatedUnderline>
                    <span class="font-semibold text-neutral-500">#{tree.genData.id}</span>
                    <a
                        href={appLink(`/tree-page/viewer?id=${tree.genData.id}`)}
                        aria-label="open"
                        target="_blank"
                        title={m.viewTree()}
                    >
                        <ArrowUpRightFromSquareOutline class="mb-1 h-5 w-5 shrink-0" />
                    </a>
                    {#if userInfo?.sub === tree.genData.ownerId}
                        <a
                            href={appLink(`/tree-page/editor?id=${tree.genData.id}`)}
                            aria-label="edit"
                            target="_blank"
                            title={m.editTree()}
                        >
                            <EditOutline class="mb-1 h-5 w-5 shrink-0" />
                        </a>
                    {/if}
                    {#if userInfo !== null}
                        <button
                            onclick={() => duplicateThisTree(tree.genData.id)}
                            aria-label="duplicate"
                            title={m.duplicateTree()}
                            class="cursor-pointer"
                        >
                            <FileCloneOutline class="mb-1 h-5 w-5 shrink-0" />
                        </button>
                    {/if}
                </span>
                <em class="-mt-1 text-neutral-700">{m.madeBy()} {tree.linkedData.ownerName}</em>
            </div>

            <div class="max-w-1/3 text-right">
                <span class="text-neutral-700">
                    {tree.data.tags.map((tag) => `#${tag}`).join(' ')}
                </span>
            </div>
        </div>

        {#if tree.data.description.length > 0}
            <div class={[descriptionShown[index] && 'inline', !descriptionShown[index] && 'flex']}>
                <span
                    class={[
                        !descriptionShown[index] && 'line-clamp-1',
                        'min-w-0 flex-1 text-ellipsis'
                    ]}
                >
                    {tree.data.description}
                </span>
                {#if tree.data.description.length > 50}
                    <button
                        onclick={() => {
                            descriptionShown[index] = !descriptionShown[index];
                        }}
                        class="text__link"
                    >
                        {descriptionShown[index] ? m.showLess() : m.showMore()}
                    </button>
                {/if}
            </div>
        {/if}
    </div>
{/snippet}
