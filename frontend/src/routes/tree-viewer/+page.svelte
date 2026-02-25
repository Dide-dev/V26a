<script lang="ts">
    import NodeViewer from './NodeViewer.svelte';
    import AddCard from '../add-card/AddCard.svelte';
    import { selectedNode } from '../add-card/selected-node.store';
    import { example_tree } from './example_tree';

    let tree = example_tree;
</script>

<h1>{tree.name}</h1>

<div class="flex grow flex-col gap-7 overflow-auto">
    {#each tree.roots as root (root)}
        <NodeViewer node={root} />
    {/each}
</div>

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
