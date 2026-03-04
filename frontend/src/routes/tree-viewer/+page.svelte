<script lang="ts">
    import NodeViewer from './NodeViewer.svelte';
    import { exampleTree } from './example_tree';

    let tree = exampleTree;
</script>

<h1 class="m-auto ml-1.5 text-3xl font-bold">{tree.name}</h1>
<p class="m-auto ml-1.5 pt-1 pb-1 text-sm font-bold text-gray-400">{tree.assignment}</p>

<div class="mt-1 mr-1 ml-1 flex grow flex-col gap-7 overflow-auto">
    {#each tree.roots as root (root.label)}
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
