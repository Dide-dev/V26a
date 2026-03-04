<script lang="ts">
    import NodeViewer from '../tree-viewer/NodeViewer.svelte';
    import AddCard from './AddCard.svelte';
    import { exampleTree } from '../tree-viewer/example_tree';
    import type { Node } from '../tree-viewer/example_tree';

    //TODO, replaces the example with real tree in future
    let tree = $state(exampleTree); 
    let selectedNode = $state<Node | null>(null);
    let secondSelectedNode = $state<Node | null>(null);

    //register when user selects nodes or deselects
    function onSelect(node: Node) {
        if (secondSelectedNode === node) {
            secondSelectedNode = null;
        } else if (selectedNode === node) {
            selectedNode = null;
        } else if (selectedNode) {
            secondSelectedNode = node;
        } else {
            selectedNode = node;
        }
    }

    //rerenders the tree when adding nodes
    //TODO, send updated tree or new card to backend
    function onAdd() {
        tree = {...tree};
        selectedNode = null;
        secondSelectedNode = null;
    }
</script>

<AddCard {selectedNode} {secondSelectedNode} {onAdd} />

<div class="mt-1 mr-1 ml-1 flex grow flex-col gap-7 overflow-auto">
    {#each tree.roots as root (root.label)}
        <NodeViewer node={root} {selectedNode} {secondSelectedNode} {onSelect} />
    {/each}
</div>

<style>
    :global(body) {
        min-height: 100vh;
        display: flex;
        flex-direction: column;
    }
</style>