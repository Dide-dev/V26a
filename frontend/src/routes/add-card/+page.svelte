<script lang="ts">
    import NodeViewer from '../tree-viewer/NodeViewer.svelte';
    import AddCard from './AddCard.svelte';
    import { exampleTree } from '../tree-viewer/example_tree';
    import type { Node } from '../tree-viewer/example_tree';

    //initalizing a reactive state 
    let tree = $state(exampleTree);
    let selectedNode = $state<Node | null>(null);
    let secondSelectedNode = $state<Node | null>(null);
</script>

<AddCard bind:selectedNode bind:secondSelectedNode />

<div class="mt-1 mr-1 ml-1 flex grow flex-col gap-7 overflow-auto">
    {#each tree.roots as root (root.label)}
        <NodeViewer node={root} bind:selectedNode bind:secondSelectedNode />
    {/each}
</div>

<style>
    :global(body) {
        min-height: 100vh;
        display: flex;
        flex-direction: column;
    }
</style>