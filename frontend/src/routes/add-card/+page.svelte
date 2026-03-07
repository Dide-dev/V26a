<script lang="ts">
    import NodeViewer from '../tree-viewer/NodeViewer.svelte';
    import { exampleTree } from '../tree-viewer/example_tree';
    import type { Node } from '../tree-viewer/example_tree';
    import { addChild, insertChild } from './AddCard';

    //TODO, replaces the example with real tree in future
    let tree = $state(exampleTree); 

    //rerenders the tree when adding nodes
    //TODO, send updated tree or new card to backend
    function onAdd() {
        tree = {...tree};
    }
</script>

<h1 class="m-auto ml-1.5 text-3xl font-bold">{tree.name}</h1>

<div class="mt-1 mr-1 ml-1 flex grow flex-col gap-7 overflow-auto">
    {#each tree.roots as root (root.label)}
        <NodeViewer node={root} addChild={(p) => addChild(p, onAdd)} insertChild={(p, c) => insertChild(p, c, onAdd)} />
    {/each}
</div>

<style>
    :global(body) {
        min-height: 100vh;
        display: flex;
        flex-direction: column;
    }
</style>