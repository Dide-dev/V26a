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

    //metadata
    let treeData = $state({
            title: '',
            description: '',
            bookName: '',
            bookEdition: '',
            bookIsbn: '',
            bookChapter: '',
            bookExercise: '',
            assignmentText: '',
            published: false,
    });

    //variables 
    let treeId: number | null = null;
    let tree: TreeWithCards | null = null;
    let error: string | null = null;
    

    //temporary storage
    let oldData = { ...treeData};
    let dropDown = $state(false);

    onMount(async () => {
        try{
            treeId = Number(page.url.searchParams.get('id'));

        //get treedata
            tree = await getTreeWithCards(treeId);
        
        //prefill data
            treeData = {
                title: tree.data.title ?? '',
                description: tree.data.description ?? '',
                bookName: tree.data.bookName ?? '',
                bookEdition: tree.data.bookEdition ?? '',
                bookIsbn: tree.data.bookIsbn ?? '',
                bookChapter: tree.data.bookChapter ?? '',
                bookExercise: tree.data.bookExercise ?? '',
                assignmentText: tree.data.assignmentText ?? '',
                published: tree.data.published ?? false,
            };
            oldData = { ...treeData};
        } catch (err) {
            console.error(err);
            error = err as string;
        }
    });

    //save changes (to backend)
    async function saveData() {
        if (!treeId || !tree) return;
        oldData = { ...treeData };
        dropDown = false;
        await updateTreeMetaData({ 
              id: treeId,
              newData: { ...treeData } 
        });
    }

    //restore original data
    function cancelData() {
        treeData = { ...oldData };
        dropDown = false;
    }
</script>

<button class="rounded-sm bg-emerald-500 p-1" onclick={() => dropDown = !dropDown}>{m.editMetadata()}</button>

{#if dropDown}
    <div class= "meta_dropdown absolute bg-white rounded shadow p-4 flex flex-col gap-2 z-10 w-64">
        <label>{m.title()}<input class="border rounded p-1 w-full" bind:value={treeData.title} /></label>
        <label>{m.description()}<textarea class="border rounded p-1 w-full" bind:value={treeData.description}></textarea></label>
        <label>{m.bookname()}<input class="border rounded p-1 w-full" bind:value={treeData.bookName} /></label>
        <label>{m.bookEdition()}<input class="border rounded p-1 w-full" bind:value={treeData.bookEdition} /></label>
        <label>{m.bookIsbn()}<input class="border rounded p-1 w-full" bind:value={treeData.bookIsbn} /></label>
        <label>{m.bookChapter()}<input class="border rounded p-1 w-full" bind:value={treeData.bookChapter} /></label>
        <label>{m.bookExercise()}<input class="border rounded p-1 w-full" bind:value={treeData.bookExercise} /></label>
        <label>{m.assignmentText()}<input class="border rounded p-1 w-full" bind:value={treeData.assignmentText} /></label>
        <label>{m.published}<input type="checkbox" bind:checked={treeData.published} /></label>
        <button class="bg-blue-500 text-white rounded px-3 py-1" onclick={saveData}>{m.save}</button>
        <button onclick={cancelData}>{m.cancel}</button>
    </div>
{/if}

