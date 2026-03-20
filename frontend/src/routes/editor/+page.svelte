<script lang="ts">
    import { page } from '$app/state';
    import { onMount } from 'svelte';
    import Dropdown from './dropdown.svelte'

    let treeId: string | null = null;

    //metadata
    let treeData = $state({
        bookName: '',
        bookIsbn: '',
        bookChapter: '',
        bookExercise: '',
    });

    //temporary storage
    let oldData = { ...treeData };
    let dropDown = $state(false);

    onMount(() => {
        treeId = page.url.searchParams.get('id');
        //TODO: get treedata
    });

    //save changes
     function saveData() {
        oldData = { ...treeData };
        dropDown = false;
        //TODO: send data to backend
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

<div>
    {treeId}
</div>
