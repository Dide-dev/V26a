<script lang="ts">
    import { page } from '$app/state';
    import { onMount } from 'svelte';
    import { updateTreeMetaData, getTreeWithCards, type TreeWithCards } from '$lib/backend/trees';

    //metadata
    let treeData = $state({
        bookName: '',
        bookIsbn: '',
        bookChapter: '',
        bookExercise: '',
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
                bookName: tree.data.bookName ?? '',
                bookIsbn: tree.data.bookIsbn ?? '',
                bookChapter: tree.data.bookChapter ?? '',
                bookExercise: tree.data.bookExercise ?? '',
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
            newData: { 
                ...tree!.data,  
                bookName: treeData.bookName,
                bookIsbn: treeData.bookIsbn,
                bookChapter: treeData.bookChapter,
                bookExercise: treeData.bookExercise,
            } 
        });
    }

    //restore original data
    function cancelData() {
        treeData = { ...oldData };
        dropDown = false;
    }
</script>

<button onclick={() => dropDown = !dropDown}>
    Edit metadata
</button>

{#if dropDown}
    <div class="absolute bg-white rounded shadow p-4 flex flex-col gap-2 z-10 w-64">
        <label>Book name<input class="border rounded p-1 w-full" bind:value={treeData.bookName} /></label>
        <label>ISBN<input class="border rounded p-1 w-full" bind:value={treeData.bookIsbn} /></label>
        <label>Chapter<input class="border rounded p-1 w-full" bind:value={treeData.bookChapter} /></label>
        <label>Exercise<input class="border rounded p-1 w-full" bind:value={treeData.bookExercise} /></label>
        <button class="bg-blue-500 text-white rounded px-3 py-1" onclick={saveData}>Save</button>
        <button onclick={cancelData}>Cancel</button>
    </div>
{/if}

