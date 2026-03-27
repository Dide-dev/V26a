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

    let treeId: number | null = null;
    let tree: TreeWithCards | null = null;
    let error: null | Error = $state(null);

    // The old tree data that is restored when canceling the edit.
    let oldData: typeof treeData;
    let dropDown = $state(false);

    onMount(async () => {
        try {
            treeId = Number(page.url.searchParams.get('id'));

            tree = await getTreeWithCards(treeId);

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
        } catch (err) {
            console.error(err);
            error = err as Error;
        }
    });

    async function saveData() {
        if (!treeId || !tree) return;
        oldData = $state.snapshot(treeData);
        dropDown = false;
        await updateTreeMetaData({
            id: treeId,
            newData: { ...treeData }
        });
    }

    // Restores treeData to its original value.
    function cancelEdit() {
        treeData = { ...oldData };
        dropDown = false;
    }
</script>

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
            <label>{m.published()}<input type="checkbox" bind:checked={treeData.published} /></label
            >
            <button class="rounded bg-blue-500 px-3 py-1 text-white" onclick={saveData}
                >{m.save()}</button
            >
            <button onclick={cancelEdit}>{m.cancel()}</button>
            <p class="text-red-600">{error?.message}</p>
        </div>
    {/if}
</div>
