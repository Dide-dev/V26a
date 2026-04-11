<!--
    This program has been developed by students from the bachelor Computer Science at Utrecht
    University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
-->
<script lang="ts">
    import { m } from '$lib/paraglide/messages.js';
    import {
        type PaginationOptions,
        type FilterOptions,
        getFilteredList
    } from '$lib/backend/trees';
    import icon from '$lib/assets/editIcon.svg?raw';
    import { appLink } from '$lib/index';

    //I need to give page number and page size
    //I will get next page, page size(same as I give), total trees, trees (TreeMetaData(TreeData en TreeGeneratedData))

    function booleanToString(value: boolean) {
        if (value) return m.yes();
        return m.no();
    }
    let pageSize = $state(20);
    let thisPage = $state(1);
    let paginationOptions: PaginationOptions = $derived({
        page: thisPage,
        size: pageSize
    });
    let filterOptions: FilterOptions = $state({
        ownerId: null,
        book: [],
        createdAfter: null,
        createdBefore: null,
        showOwnPrivate: null
    });
    let treesPromise = $derived(getFilteredList(paginationOptions, filterOptions));

    // export type TreeList = {
    //     totalTrees: number;
    //     list: TreeMetadata[];
    // };

    // export type TreeMetadata = {
    //     data: TreeData;
    //     genData: TreeGeneratedData;
    // };

    // export type TreeGeneratedData = {
    //     id: TreeId;
    //     ownerId: UserId;
    //     createdAt: Date;
    //     lastModified: Date;
    //     timesVisited: number;
    //     lastVisited: Option<Date>;
    // };

    // export type TreeData = {
    //     title: string;
    //     description: Option<string>;
    //     bookName: Option<string>;
    //     bookEdition: Option<string>;
    //     bookIsbn: Option<string>;
    //     bookChapter: Option<string>;
    //     bookExercise: Option<string>;
    //     assignmentText: Option<string>;
    //     published: boolean;
    // };

    // let totalPages = $derived(calcTotalPages(totalTrees, pageSize));
    function prevPage() {
        if (thisPage > 1) {
            thisPage -= 1;
        }
    }
    function calcTotalPages(totalTrees: number, pageSize: number): number {
        return Math.ceil(totalTrees / pageSize);
    }
    function nextPage(totalPages: number) {
        if (thisPage < totalPages) {
            thisPage += 1;
        }
    }
</script>

{#await treesPromise}
    <p>{m.loading()}</p>
{:then trees}
    {#if trees.list.length > 0}
        <!-- For dynamically formatting the entire css has to be redone. -->
        <div class="relative left-1/4 mx-4 h-24 max-w-200">
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
                    {#each trees.list as tree (tree.genData.id)}
                        <tr>
                            <td class="max-w-96 pr-4 hover:text-blue-800 hover:underline">
                                <a
                                    href={appLink(`/tree-page/viewer?id=${tree.genData.id}`)}
                                    target="_blank">{tree.data.title}</a
                                >
                            </td>
                            <td class="pr-4"
                                >{new Date(
                                    tree.genData.lastModified.toString()
                                ).toLocaleString()}</td
                            >
                            <!-- This formats the date in the local timezone in the form: dd-mm-yyyy, hh:mm:ss -->
                            <td class="pr-4">{booleanToString(tree.data.published)}</td>
                            <td class="min-w-2 hover:fill-blue-800">
                                <a
                                    href={appLink(`/tree-page/editor?id=${tree.genData.id}`)}
                                    target="_blank"
                                    aria-label="edit"
                                >
                                    <!-- eslint-disable-next-line svelte/no-at-html-tags -->
                                    {@html icon}
                                </a>
                            </td>
                        </tr>
                    {/each}
                </tbody>
            </table>
        </div>
        <div class="fixed bottom-2 left-1/2 -translate-x-1/2">
            <button onclick={prevPage} class="mx-5">Prev</button>
            {thisPage}/{calcTotalPages(trees.totalTrees, pageSize)}
            <button
                onclick={() => nextPage(calcTotalPages(trees.totalTrees, pageSize))}
                class="mx-5">Next</button
            >
        </div>
    {:else}
        <div class="text-center">
            <p class="flex justify-center text-2xl">{m.noTreesFound()}</p>
            <button class="m-2 rounded-sm bg-emerald-500 p-1"
                ><a href={appLink('/tree-generator')}>{m.createTree()}</a></button
            >
        </div>
    {/if}
{:catch error}
    <p style="color:red">{error.message}</p>
{/await}
