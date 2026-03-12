<script lang="ts">
    import NodeViewer from './NodeViewer.svelte';
    import { onMount } from 'svelte';
    import { getTreeWithCards } from '$lib/backend/trees';
    import type { TreeWithCards } from '$lib/backend/trees';
    import type { HeuristicTree, RootNode } from './example_tree';

    let tree: TreeWithCards | null = null;
    let loading = true;
    let error: string | null = null;
    // let heuristicTree : HeuristicTree = $state();

    onMount(async () => {
        try {
            const params = new URLSearchParams(window.location.search);
            const id = params.get('id');
            const idNumber = Number(id);

            if (Number.isNaN(idNumber)) {
                throw new Error('Invalid tree id');
            }

            if (!id) {
                console.error('No tree id provided');
                return;
            }

            tree = await getTreeWithCards(idNumber);

            if (!tree) {
                throw new Error('Tree not found');
            }
        } catch (err) {
            console.error(err);
            error = 'Falied to load tree';
        } finally {
            loading = false;
        }
    });

    // function convertToHeuristicTree(tree: TreeWithCards){
    //     heuristicTree.name = tree.data.title;
    //     heuristicTree.assignment = tree.data.assignmentText;
    //     heuristicTree.roots = tree.cards;
    // }
</script>

{#if loading}
    <p class="text-green-800">Tree is growing...</p>
{:else if error}
    <p class="text-red-700">{error}</p>
{:else if tree}
    <h1 class="m-auto ml-1.5 text-3xl font-bold">{tree.data.title}</h1>
    <p class="m-auto ml-1.5 pt-1 pb-1 text-sm font-bold text-gray-400">
        {tree.data.assignmentText}
    </p>
{/if}

<!-- <div class="mt-1 mr-1 ml-1 flex grow flex-col gap-7 overflow-auto">
    {#each tree.roots as root (root)}
        <NodeViewer node={root} />
    {/each}
</div> -->

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
