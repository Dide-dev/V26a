<script lang="ts">
    import NodeViewer from './NodeViewer.svelte';
    import { onMount } from 'svelte';
    import { getTreeWithCards, type TreeWithCards } from '$lib/backend/trees';

    let loading = true;
    let error: string | null = null;
    let tree: TreeWithCards | null = null;

    onMount(async () => {
        try {
            const params = new URLSearchParams(window.location.search);
            const id = params.get('id');

            if (!id) {
                throw new Error('No tree id provided');
            }

            const idNumber = Number(id);

            if (Number.isNaN(idNumber)) {
                throw new Error('Invalid tree id');
            }

            tree = await getTreeWithCards(idNumber);

            if (!tree) {
                throw new Error('Tree not found');
            }
        } catch (err) {
            console.error(err);
            error = err as string;
        } finally {
            loading = false;
        }
    });
</script>

{#if loading}
    <p class="text-green-800">Tree is growing...</p>
{:else if error}
    <p class="text-red-700">{error}</p>
{:else if tree}
    <h1 class="m-auto ml-1.5 text-3xl font-bold">{tree.data.title}</h1>
    <p class="m-auto ml-1.5 pt-1 pb-1 text-sm font-bold text-gray-400">
        {tree.data.description}
    </p>

    <div class="mt-1 mr-1 ml-1 flex grow flex-col gap-7 overflow-auto">
        {#each tree.structure.rootNodes as root (root)}
            <NodeViewer node={root} cards={tree.cards} />
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
