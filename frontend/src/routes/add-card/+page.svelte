<script lang="ts">
    import NodeViewer from '../tree-viewer/NodeViewer.svelte';
    import { exampleTree } from '../tree-viewer/example_tree';
    import type { Node } from '../tree-viewer/example_tree';
    import { type CardAddData, addCard } from '$lib/backend/cards';
    import { type TreeUpdateStructureData, updateTreeStructure } from "$lib/backend/trees";

    //TODO, replaces the example with real tree in future
    let tree = $state(exampleTree);

    let inputText = $state('');
    let errorMessage = $state('');

    // Variable to have the add/insert action on hold
    let action = $state<null | { type: 'add', parent: Node } | { type: 'insert', parent: Node, child: Node }>(null);

    // Gets called when the button to add a card is clicked;
    function handlerAdd(parent: Node) {
        action = { type: 'add', parent };
    }

    // Gets called when the button to insert a card is clicked;
    // Sets the action to insert type
    function handlerInsert(parent: Node, child: Node) {
        action = { type: 'insert', parent, child };
    }

    // Gets called when the Add button in the popup is clicked;
    // Checks the action and executes the correct function
    function acceptCard(text: string) {
        if (!action) return;
        errorMessage = '';

        let card: CardAddData = {
            treeId: tree.id,
            front: '',
            back: text,
        }

        if (action.type === 'add') {
            addChild(action.parent, card);
        } else if (action.type === 'insert') {
            insertChild(action.parent, action.child, card);
        }
    }

    // Adds a new child node
    async function addChild(parent: Node, card: CardAddData) {
        const newStructure = [...parent.children, {card: card.back, children: []}];

        addCard(card)
            .then((treeId) => {
                let update: TreeUpdateStructureData = {
                    id: treeId,
                    newStructure: newStructure,
                }
                return updateTreeStructure(update);
            })

            .then(() => {
                parent.children = newStructure;
                tree = {...tree};
                action = null;
            })

            .catch((reason: Error) => {
                errorMessage = reason.toString();
                console.error('Error: ', reason);
            });
    }

    // Inserts a new child between two nodes
    async function insertChild(parent: Node, child: Node, card: CardAddData) {
        const newNode: Node = { card: card.back, children: [child] };
        const newStructure = parent.children.map(c => c === child ? newNode : c) as Node[];

        addCard(card)
            .then((treeId) => {
                let update: TreeUpdateStructureData = {
                    id: treeId,
                    newStructure: newStructure,
                }
                return updateTreeStructure(update);
            })

            .then(() => {
                parent.children = newStructure;
                tree = {...tree};
                action = null;
            })

            .catch((reason: Error) => {
                errorMessage = reason.toString();
                console.error('Error: ', reason);
            });
    }

</script>

{#if action}
    <div class="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
        <div class="bg-white rounded p-4 flex flex-col gap-2">
            <textarea class="border rounded p-1" placeholder="Enter card text..." bind:value={inputText}></textarea>
            {#if errorMessage}
                <p class="text-red-600">{errorMessage}</p>
            {/if}
            <div class="flex gap-2 justify-end">
                <button onclick={() => action = null}>Cancel</button>
                <button onclick={() => acceptCard(inputText)} disabled={!inputText}>Add</button>
            </div>
        </div>
    </div>
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
