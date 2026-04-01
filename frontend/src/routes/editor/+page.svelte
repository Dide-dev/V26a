<!--
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
-->

<script lang="ts">
    import { page } from '$app/state';
    import { onMount } from 'svelte';
    import { m } from '$lib/paraglide/messages.js';
    import { updateTreeMetaData, getTreeWithCards, type TreeWithCards } from '$lib/backend/trees';
    import NodeViewer from '../tree-viewer/NodeViewer.svelte';

    let treeData = $state({
        title: '',
        description: '',
        bookName: '',
        bookEdition: '',
        bookIsbn: '',
        bookChapter: '',
        bookExercise: '',
        assignmentText: '',
        published: false
    });

    let loading = $state(true);
    let error: Error | null = $state(null);
    let id: number | null = null;
    let tree: TreeWithCards | null = $state(null);

    // The old tree data that is restored when canceling the edit.
    let oldData: typeof treeData;
    let dropDown = $state(false);

    onMount(async () => {
        const pageId = page.url.searchParams.get('id');

        if (!pageId) {
            error = new Error('No tree id provided');
            return;
        }

        id = Number(pageId);

        if (Number.isNaN(id)) {
            error = new Error('Invalid tree id');
            return;
        }

        tree = await getTreeWithCards(id).then(
            (treeWithCards) => treeWithCards,
            (e) => {
                error = e;
                return null;
            }
        );

        if (tree) {
            treeData = {
                title: tree.data.title ?? '',
                description: tree.data.description ?? '',
                bookName: tree.data.bookName ?? '',
                bookEdition: tree.data.bookEdition ?? '',
                bookIsbn: tree.data.bookIsbn ?? '',
                bookChapter: tree.data.bookChapter ?? '',
                bookExercise: tree.data.bookExercise ?? '',
                assignmentText: tree.data.assignmentText ?? '',
                published: tree.data.published ?? false
            };
            oldData = $state.snapshot(treeData);
        }

        loading = false;
    });

    async function saveData() {
        if (!id || !tree) return;
        oldData = $state.snapshot(treeData);
        dropDown = false;
        await updateTreeMetaData({
            id: id,
            newData: { ...treeData }
        });
    }

    // Restores treeData to its original value.
    function cancelEdit() {
        treeData = { ...oldData };
        dropDown = false;
    }
</script>

{#if error == null && loading}
    <p class="text-green-800">{m.loadingTree()}</p>
{:else if error != null}
    <p class="text-red-700" data-testid="error">{error}</p>
{:else if tree}
    <h1 class="m-auto ml-1.5 text-3xl font-bold">{tree.data.title}</h1>
    <p class="m-auto ml-1.5 pt-1 pb-1 text-sm font-bold text-gray-400">
        {tree.data.description}
    </p>

    <div class="relative ml-4">
        <button class="rounded-sm bg-emerald-500 p-1" onclick={() => (dropDown = !dropDown)}
            >{m.editMetadata()}</button
        >

        {#if dropDown}
            <div
                class="meta-dropdown absolute z-10 flex w-64 flex-col gap-2 rounded bg-white p-4 shadow"
            >
                <label
                    >{m.title()}<input
                        class="w-full rounded border p-1"
                        bind:value={treeData.title}
                    /></label
                >
                <label
                    >{m.description()}<textarea
                        class="w-full rounded border p-1"
                        bind:value={treeData.description}
                    ></textarea></label
                >
                <label
                    >{m.bookname()}<input
                        class="w-full rounded border p-1"
                        bind:value={treeData.bookName}
                    /></label
                >
                <label
                    >{m.bookEdition()}<input
                        class="w-full rounded border p-1"
                        bind:value={treeData.bookEdition}
                    /></label
                >
                <label
                    >{m.bookIsbn()}<input
                        class="w-full rounded border p-1"
                        bind:value={treeData.bookIsbn}
                    /></label
                >
                <label
                    >{m.bookChapter()}<input
                        class="w-full rounded border p-1"
                        bind:value={treeData.bookChapter}
                    /></label
                >
                <label
                    >{m.bookExercise()}<input
                        class="w-full rounded border p-1"
                        bind:value={treeData.bookExercise}
                    /></label
                >
                <label
                    >{m.assignmentText()}<input
                        class="w-full rounded border p-1"
                        bind:value={treeData.assignmentText}
                    /></label
                >
                <label
                    >{m.published()}<input
                        type="checkbox"
                        bind:checked={treeData.published}
                    /></label
                >
                <button class="rounded bg-blue-500 px-3 py-1 text-white" onclick={saveData}
                    >{m.save()}</button
                >
                <button onclick={cancelEdit}>{m.cancel()}</button>
            </div>
        {/if}
    </div>

    <div class="mt-1 mr-1 ml-1 flex grow flex-col gap-7 overflow-auto">
        {#each tree.structure.rootNodes as root (root)}
            <NodeViewer bind:tree parent={root} node={root} editingPage={true} />
        {/each}
    </div>
{/if}

<style>
    :global(body) {
        min-height: 100vh;
        display: flex;
        flex-direction: column;
    }
</style>
