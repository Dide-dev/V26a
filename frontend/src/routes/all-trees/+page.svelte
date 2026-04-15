<!--
    This program has been developed by students from the bachelor Computer Science at Utrecht
    University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
-->
<script lang="ts">
    import { m } from '$lib/paraglide/messages.js';
    import {
        type FilterOptions,
        type PaginationOptions,
        type TreeListResponse,
        type TreeMetadata,
        getFilteredList
    } from '$lib/backend/trees';
    import { appLink } from '$lib/index';
    import { onMount } from 'svelte';

    // nl-NL and en-GB behave in the same way; we will not support American time notation.
    const timeFormat = new Intl.DateTimeFormat('nl-NL');

    let pageSize = $state(5);
    let thisPage = $state(1);

    let filterOptionsForm = $state({
        ownerId: '',
        createdAfterYear: '',
        createdAfterMonth: '',
        createdAfterDay: '',
        createdBeforeYear: '',
        createdBeforeMonth: '',
        createdBeforeDay: ''
    });

    let filterOptions: FilterOptions = $state({
        ownerId: null,
        book: [],
        createdAfter: null,
        createdBefore: null,
        showOwnPrivate: true
    });
    let paginationOptions: PaginationOptions = $derived({
        page: thisPage,
        size: pageSize
    });

    let treeListResponse: null | TreeListResponse = $state(null);
    let trees: null | TreeMetadata[] = $state(null);

    let prevPageExists = $derived(thisPage > 1);
    let nextPageExists = $derived(
        thisPage < calcTotalPages(showTreeCount(treeListResponse), pageSize)
    );

    onMount(async () => {
        await loadTrees();
    });

    async function loadTrees() {
        treeListResponse = await getFilteredList(paginationOptions, filterOptions);
        trees = treeListResponse.trees;
    }

    function prevPage() {
        thisPage -= 1;
        loadTrees();
    }

    function nextPage() {
        thisPage += 1;
        loadTrees();
    }

    function calcTotalPages(totalTrees: number, pageSize: number): number {
        return Math.ceil(totalTrees / pageSize);
    }

    function showTreeCount(treeListResponse: TreeListResponse | null): number {
        if (treeListResponse == null) {
            return 0;
        } else {
            return treeListResponse.totalTrees;
        }
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

    async function readFilters() {
        filterOptions.ownerId = !isNaN(parseInt(filterOptionsForm.ownerId))
            ? Number(filterOptionsForm.ownerId)
            : null;

        const tryCreatedAfter = new Date(
            parseInt(filterOptionsForm.createdAfterYear),
            parseInt(filterOptionsForm.createdAfterMonth) - 1,
            parseInt(filterOptionsForm.createdAfterDay)
        );

        filterOptions.createdAfter = !isNaN(tryCreatedAfter.valueOf()) ? tryCreatedAfter : null;

        const tryCreatedBefore = new Date(
            parseInt(filterOptionsForm.createdBeforeYear),
            parseInt(filterOptionsForm.createdBeforeMonth) - 1,
            parseInt(filterOptionsForm.createdBeforeDay)
        );

        filterOptions.createdBefore = !isNaN(tryCreatedBefore.valueOf()) ? tryCreatedBefore : null;

        await loadTrees();
    }
</script>

<div class="flex w-full justify-center">
    <div class="flex-col">
        <!-- Filter options -->
        <form class="flex w-md flex-col gap-2" onsubmit={readFilters}>
            <label>
                {m.createdBetween()}
                <input
                    class="w-12 rounded-md border-2"
                    bind:value={filterOptionsForm.createdAfterYear}
                />
                <input
                    class="w-6 rounded-md border-2"
                    bind:value={filterOptionsForm.createdAfterMonth}
                />
                <input
                    class="w-6 rounded-md border-2"
                    bind:value={filterOptionsForm.createdAfterDay}
                />
                {m.and()}
                <input
                    class="w-12 rounded-md border-2"
                    bind:value={filterOptionsForm.createdBeforeYear}
                />
                <input
                    class="w-6 rounded-md border-2"
                    bind:value={filterOptionsForm.createdBeforeMonth}
                />
                <input
                    class="w-6 rounded-md border-2"
                    bind:value={filterOptionsForm.createdBeforeDay}
                />
            </label>
            <label>
                {m.ownerId()}
                <input class="rounded-md border-2" bind:value={filterOptionsForm.ownerId} />
            </label>
            <button class="rounded-sm bg-emerald-500 p-1">{m.applyFilters()}</button>
        </form>

        <!-- Change the number of trees per page -->
        <label>
            {m.numberOfTrees()}
            <select
                class="rounded-sm bg-neutral-400 p-1"
                bind:value={pageSize}
                onchange={loadTrees}
            >
                <option value="5">5</option>
                <option value="10">10</option>
                <option value="20">20</option>
                <option value="50">50</option>
            </select>
        </label>
    </div>
</div>

{#if trees === null}
    <p>{m.loading()}</p>
{:else if trees.length === 0}
    <div class="text-center">
        <p class="flex justify-center text-2xl">{m.noTreesFound()}</p>
    </div>
{:else}
    <div class="100vw flex justify-center overflow-auto">
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
                {#each trees as tree (tree.genData.id)}
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
                    </tr>
                {/each}
            </tbody>
        </table>
        <div class="fixed bottom-2 left-1/2 -translate-x-1/2">
            <button
                onclick={prevPage}
                disabled={!prevPageExists}
                class="mx-5"
                class:cursor-pointer={prevPageExists}>Prev</button
            >
            {thisPage}/{calcTotalPages(showTreeCount(treeListResponse), pageSize)}
            <button
                onclick={nextPage}
                disabled={!nextPageExists}
                class="mx-5"
                class:cursor-pointer={nextPageExists}>Next</button
            >
        </div>
    </div>
{/if}
