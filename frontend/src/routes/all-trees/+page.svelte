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
    import { appLink } from '$lib';
    import { onMount } from 'svelte';

    // nl-NL and en-GB behave in the same way; we will not support American time notation.
    const timeFormat = new Intl.DateTimeFormat('nl-NL');

    let pageSize = $state(5);
    let thisPage = $state(1);
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
</script>

{#if trees === null}
    <p>{m.loading()}</p>
{:else if trees.length === 0}
    <div class="text-center">
        <p class="flex justify-center text-2xl">{m.noTreesFound()}</p>
    </div>
{:else}
    <div class="100vw flex justify-center">
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
