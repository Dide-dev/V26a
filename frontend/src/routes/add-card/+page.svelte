<script lang="ts">
    import NodeViewer from '../tree-viewer/NodeViewer.svelte';
    import { exampleTree } from '../tree-viewer/example_tree';
    import PopupWindow from './PopupWindow.svelte';
    import type { Node } from '../tree-viewer/example_tree';
    import { addChild, insertChild } from './AddCard';
    import { type CardData, addCard } from '$lib/backend/cards';

    //TODO, replaces the example with real tree in future
    let tree = $state(exampleTree);

    //rerenders the tree when adding nodes
    //TODO, send updated tree or new card to backend
    function onAdd(front: string, back: string) {
        tree = {...tree};
        let card: CardData = {
            treeId: tree.id,
            front: front,
            back: back,
        }
        addCard(card);
    }

    //variable to have the add/insert action on hold
    let action = $state<null | { type: 'add', parent: Node } | { type: 'insert', parent: Node, child: Node }>(null);


    function handlerAdd(parent: Node) {
        action = { type: 'add', parent };
    }

    function handlerInsert(parent: Node, child: Node) {
        action = { type: 'insert', parent, child };
    }

    //check the action and execute it if necessarily
    function acceptCard(text: string) {
    if (!action) return;
        if (action.type === 'add') {
            addChild(action.parent, '', text, onAdd);
        } else {
            insertChild(action.parent, action.child, '', text, onAdd);
        }
        action = null;
    }
</script>

{#if action}
    <PopupWindow acceptCard={acceptCard} declineCard={() => action = null} />
{/if}

<h1 class="m-auto ml-1.5 text-3xl font-bold">{tree.name}</h1>

<div class="mt-1 mr-1 ml-1 flex grow flex-col gap-7 overflow-auto">
    {#each tree.roots as root (root.label)}
        <NodeViewer node={root} addChild={handlerAdd} insertChild={handlerInsert} />
    {/each}
</div>

<style>
    :global(body) {
        min-height: 100vh;
        display: flex;
        flex-direction: column;
    }
</style>
