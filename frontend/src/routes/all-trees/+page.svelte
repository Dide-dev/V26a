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

    onMount(async () => {
        await loadTrees();
    });

    async function loadTrees() {
        treeListResponse = await getFilteredList(paginationOptions, filterOptions);
        trees = treeListResponse.trees;
    }

    function prevPage() {
        if (thisPage > 1) {
            thisPage -= 1;
            loadTrees();
        }
    }

    function nextPage(totalPages: number) {
        if (thisPage < totalPages) {
            thisPage += 1;
            loadTrees();
        }
    }

    function calcTotalPages(totalTrees: number, pageSize: number): number {
        return Math.ceil(totalTrees / pageSize);
    }

    function checkIfNull(treeListResponse: TreeListResponse | null): number {
        if (treeListResponse == null) {
            return 0;
        } else {
            return treeListResponse.totalTrees;
        }
    }

    function checkIfHasBook(bookName: string | null): string {
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
                    <!-- Edit button -->
                    <th class="text-left"></th>
                </tr>
            </thead>
            <tbody>
                {#each trees as tree (tree.genData.id)}
                    <tr>
                        <td class="max-w-96 pr-4 hover:text-blue-800 hover:underline">
                            <a
                                href={appLink(`/tree-page/viewer?id=${tree.genData.id}`)}
                                target="_blank">{tree.data.title}</a
                            >
                        </td>
                        <td class="pr-4">{checkIfHasBook(tree.data.bookName)}</td>
                        <td class="pr-4">{tree.genData.timesVisited}</td>
                        <td class="pr-4">
                            <!-- This formats the date in the local timezone in the form: dd-mm-yyyy -->
                            {new Date(tree.genData.createdAt).getDate()}-{new Date(
                                tree.genData.createdAt
                            ).getMonth() + 1}-{new Date(tree.genData.createdAt).getFullYear()}
                        </td>
                    </tr>
                {/each}
            </tbody>
        </table>
        <div class="fixed bottom-2 left-1/2 -translate-x-1/2">
            <button onclick={prevPage} class="mx-5">Prev</button>
            {thisPage}/{calcTotalPages(checkIfNull(treeListResponse), pageSize)}
            <button
                onclick={() => nextPage(calcTotalPages(checkIfNull(treeListResponse), pageSize))}
                class="mx-5">Next</button
            >
        </div>
    </div>
{/if}
