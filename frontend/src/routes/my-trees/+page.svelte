<!--
    This program has been developed by students from the bachelor Computer Science at Utrecht
    University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
-->

<script lang="ts">
    import { m } from '$lib/paraglide/messages.js';
    import {
        getUsersOwnTrees,
        type TreeMetadata,
        type TreeListResponse,
        type PaginationOptions
    } from '$lib/backend/trees';
    import iconEdit from '$lib/assets/icons/edit.svg?raw';
    import { appLink } from '$lib';
    import RemoveButton from './RemoveButton.svelte';
    import { onMount } from 'svelte';
    import { calcTotalPages, showTreeCount } from '$lib/pagination';
    import { booleanToString } from '$lib/general';

    // This cannot be done with the {#await ...} in svelte,
    // because then updating it browser-side for tree deletion is not possible.
    let treeListResponse: null | TreeListResponse = $state(null);
    let trees: null | TreeMetadata[] = $state(null);

    let pageSizeString = $state('10');
    let pageSizeNumber = $derived(Number(pageSizeString));
    let thisPage = $state(1);

    let paginationOptions: PaginationOptions = $derived({
        page: thisPage,
        size: pageSizeNumber
    });

    onMount(async () => {
        await loadTrees();
    });

    async function loadTrees() {
        treeListResponse = await getUsersOwnTrees(paginationOptions);
        trees = treeListResponse.trees;
    }

    let prevPageExists = $derived(thisPage > 1);
    let nextPageExists = $derived(
        thisPage < calcTotalPages(showTreeCount(treeListResponse), pageSizeNumber)
    );

    function prevPage() {
        thisPage -= 1;
        loadTrees();
    }

    function nextPage() {
        thisPage += 1;
        loadTrees();
    }
</script>

<div class="flex w-full justify-center">
    <div class="flex-col">
        <!-- Change the number of trees per page -->
        <label>
            {m.numberOfTrees()}
            <select
                class="m-1 w-15 rounded-sm bg-neutral-200 p-1"
                bind:value={pageSizeString}
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
        <p class="flex justify-center text-2xl">{m.noMyTreesFound()}</p>
        <button class="m-2 rounded-sm bg-emerald-500 p-1"
            ><a href={appLink('/tree-generator')}>{m.createTree()}</a></button
        >
    </div>
{:else}
    <div class="100vw flex flex-col items-center justify-center overflow-auto">
        <table class="border-separate border-spacing-2">
            <thead>
                <tr>
                    <th class="text-left">{m.title()}</th>
                    <th class="text-left">{m.lastEdited()}</th>
                    <th class="text-left">{m.published()}</th>
                    <!-- Edit button -->
                    <th class="text-left"></th>
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
                        <td class="pr-4">
                            <!-- This formats the date in the local timezone in the form: dd-mm-yyyy, hh:mm:ss -->
                            {new Date(tree.genData.lastModified.toString()).toLocaleString()}
                        </td>
                        <td class="pr-4">{booleanToString(tree.data.published)}</td>
                        <td class="min-w-2 hover:fill-blue-800">
                            <a
                                href={appLink(`/tree-page/editor?id=${tree.genData.id}`)}
                                target="_blank"
                                aria-label="edit"
                            >
                                <!-- eslint-disable-next-line svelte/no-at-html-tags -->
                                {@html iconEdit}
                            </a>
                        </td>
                        <td class="min-w-2 hover:fill-blue-800">
                            <RemoveButton thisTree={tree} bind:allTrees={trees} />
                        </td>
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
            {thisPage}/{calcTotalPages(showTreeCount(treeListResponse), pageSizeNumber)}
            <button
                onclick={nextPage}
                disabled={!nextPageExists}
                class="mx-5"
                class:cursor-pointer={nextPageExists}>Next</button
            >
        </div>
    </div>
{/if}
