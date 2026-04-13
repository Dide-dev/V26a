<!--
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
-->

<script lang="ts">
    import { page } from '$app/state';
    import { onMount } from 'svelte';
    import { m } from '$lib/paraglide/messages.js';
    import {
        updateTreeMetaData,
        getTreeWithCards,
        type TreeWithCards,
        type TreeData
    } from '$lib/backend/trees';
    import NodeViewer from '../NodeViewer.svelte';
    import { redirectLogin } from '$lib/backend';
    import { goto } from '$app/navigation';
    import { appLink } from '$lib';

    let treeData: TreeData = $state({
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

    let id: number | null = null;
    let tree: TreeWithCards | null = $state(null);
    let dropDown = $state(false);

    onMount(async () => {
        const pageId = page.url.searchParams.get('id');

        if (!pageId) {
            // TODO: redirect to tree list
            await goto(appLink('/'));
            return;
        }

        id = Number(pageId);

        if (Number.isNaN(id)) {
            // TODO: redirect to tree list
            await goto(appLink('/'));
            return;
        }

        tree = await getTreeWithCards(id);

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

        loading = false;
    });

    async function saveData() {
        if (!id || !tree) return;
        dropDown = false;
        await updateTreeMetaData({
            id: id,
            newData: { ...treeData }
        });
        tree.data = { ...treeData };
    }

    // Restores treeData to its original value.
    function cancelEdit() {
        treeData = tree!.data;
        dropDown = false;
    }
</script>

{#if loading}
    <p class="text-green-800">{m.loadingTree()}</p>
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

<!--
    Here we add a few css classes to the body,
    to make sure that the element above here with the `overflow-auto` class can be
    growing in height until the scroll bar is at the bottom of the screen.
-->
<style>
    :global(body) {
        min-height: 100vh;
        display: flex;
        flex-direction: column;
    }
</style>
