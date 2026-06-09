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
        getFilteredList,
        duplicateTree
    } from '$lib/api-wrapper/trees';
    import { appLink, timeFormatString } from '$lib';
    import { onMount } from 'svelte';
    import { calcTotalPages, type PaginationOptions } from '$lib/pagination';
    import iconDuplicate from '$lib/assets/icons/duplicate.svg?raw';
    import { getUserInfo } from '$lib/api-wrapper/users';

    let timeFormat = new Intl.DateTimeFormat(timeFormatString);
    let userInfo = $state(getUserInfo());

    let pageSizeString = $state('10');
    let pageSizeNumber = $derived(Number(pageSizeString));
    let thisPage = $state(1);

    let filterOptions: FilterOptions = $state({
        ownerId: null,
        searchTerms: '',
        book: [],
        createdAfter: null,
        createdBefore: null,
        showOwnPrivate: true,
        editable: null
    });
    let paginationOptions: PaginationOptions = $derived({
        page: thisPage,
        size: pageSizeNumber
    });

    let treeListResponse: null | TreeListResponse = $state(null);

    let prevPageExists = $derived(thisPage > 1);
    let nextPageExists = $derived.by(() => {
        // For some reason svelte does not understand the type of treeListResponse so we have to manually cast it.
        const total = (treeListResponse as TreeListResponse | null)?.totalTrees ?? 0;
        return thisPage < calcTotalPages(total, pageSizeNumber);
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
        thisPage = 1;
        loadTrees();
    }

    function prevPage() {
        thisPage -= 1;
        loadTrees();
    }

    function nextPage() {
        thisPage += 1;
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
</script>

<div class="flex w-full justify-center">
    <div class="flex-col">
        <!-- Filter options -->
        <form class="completed flex w-md flex-col gap-2" onsubmit={loadTrees}>
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

{#if treeListResponse?.trees === null}
    <p>{m.loading()}</p>
{:else if treeListResponse?.trees.length === 0}
    <div class="text-center">
        <p class="flex justify-center text-2xl">{m.noAllTreesFound()}</p>
    </div>
{:else}
    <div class="100vw flex flex-col items-center justify-center overflow-auto">
        <table class="border-separate border-spacing-2">
            <thead>
                <tr>
                    <th class="text-left">{m.title()}</th>
                    <th class="text-left">{m.book()}</th>
                    <th class="text-left">{m.timesVisited()}</th>
                    <th class="text-left">{m.createdAt()}</th>
                </tr>
            </thead>
            <tbody>
                {#each treeListResponse?.trees as tree (tree.genData.id)}
                    <tr>
                        <td class="max-w-96 pr-4 wrap-anywhere hover:text-blue-800 hover:underline">
                            <a
                                href={appLink(`/tree-page/viewer?id=${tree.genData.id}`)}
                                target="_blank">{tree.data.title}</a
                            >
                        </td>
                        <td class="pr-4 wrap-anywhere">{showBookName(tree.data.bookName)}</td>
                        <td class="pr-4">{tree.genData.timesVisited}</td>
                        <td class="pr-4">
                            <!-- This formats the date in the local timezone in the form: dd-mm-yyyy -->
                            {timeFormat.format(new Date(tree.genData.createdAt))}
                        </td>
                        {#if userInfo !== null}
                            <td class="min-w-2 hover:fill-blue-800">
                                <button
                                    onclick={() => duplicateThisTree(tree.genData.id)}
                                    aria-label="duplicate"
                                    class:cursor-pointer={true}
                                    class="flex self-end"
                                >
                                    <!-- eslint-disable-next-line svelte/no-at-html-tags -->
                                    {@html iconDuplicate}
                                </button>
                            </td>
                        {/if}
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
            {thisPage}/{calcTotalPages(treeListResponse?.totalTrees ?? 0, pageSizeNumber)}
            <button
                onclick={nextPage}
                disabled={!nextPageExists}
                class="mx-5"
                class:cursor-pointer={nextPageExists}>Next</button
            >
        </div>
    </div>
{/if}
