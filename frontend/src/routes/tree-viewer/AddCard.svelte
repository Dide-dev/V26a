<!--
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
-->

<script lang="ts">
    import type { TreeId, TreeStructure, TreeUpdateStructureData, Node, RootNode } from '$lib/backend/trees'
    import { updateTreeStructure } from "$lib/backend/trees";
    import { type CardAddData, type CardId, addCard } from '$lib/backend/cards';

    let { treeId, treeStructure = $bindable(), node }:
        { treeId: TreeId; treeStructure: TreeStructure; node: Node | RootNode } = $props();

    let inputText = $state('');
    let errorMessage = $state('');

    // Variable to have the add/insert action on hold
    let action = $state<null |
        { type: 'add', parent: Node | RootNode } |
        { type: 'insert', parent: Node | RootNode, child: Node }>(null);


    // Gets called when the button to add a card is clicked;
    function handlerAdd(parent: Node | RootNode) {
        action = { type: 'add', parent };
    }

    // Gets called when the button to insert a card is clicked;
    // Sets the action to insert type
    function handlerInsert(parent: Node | RootNode, child: Node) {
        action = { type: 'insert', parent, child };
    }

    // Gets called when the Add button in the popup is clicked;
    // Checks the action and executes the correct function
    function acceptCard(text: string) {
        if (!action) return;
        errorMessage = '';

        let card: CardAddData = {
            treeId: treeId,
            front: text,
            back: '',
        }

        if (action.type === 'add') {
            addChild(action.parent, card);
        } else if (action.type === 'insert') {
            insertChild(action.parent, action.child, card);
        }
    }

    // Adds a new child node
    async function addChild(parent: Node | RootNode, card: CardAddData) {
        addCard(card)
            .then((cardId: CardId) => {
                //const newStructure = [...parent.children, {card: card.back, children: []}];
                let newStructure = treeStructure;
                let update: TreeUpdateStructureData = {
                    id: treeId,
                    newStructure: newStructure,
                }
                return updateTreeStructure(update);
            })

            .then(() => {
                //parent.children = newStructure;
                //tree = {...tree};
                action = null;
            })

            .catch((reason: Error) => {
                errorMessage = reason.toString();
                console.error('Error: ', reason);
            });
    }

    // Inserts a new child between two nodes
    async function insertChild(parent: Node | RootNode, child: Node, card: CardAddData) {
        addCard(card)
            .then((cardId: CardId) => {
                //const newNode: Node = { card: card.back, children: [child] };
                //const newStructure = parent.children.map(c => c === child ? newNode : c) as Node[];
                let newStructure = treeStructure;
                let update: TreeUpdateStructureData = {
                    id: treeId,
                    newStructure: newStructure,
                }
                return updateTreeStructure(update);
            })

            .then(() => {
                //parent.children = newStructure;
                //tree = {...tree};
                action = null;
            })

            .catch((reason: Error) => {
                errorMessage = reason.toString();
                console.error('Error: ', reason);
            });
    }
</script>

<button
    class="absolute top-2 right-2 z-10 h-5 w-5 rounded-full bg-blue-500 text-white text-xs opacity-30 hover:opacity-100 transition-opacity cursor-pointer"
    onclick={() => handlerAdd(node)}
>+</button>
{#each node.children as child_node, index (child_node)}
    <button
        class="absolute -right-20 z-10 h-5 w-5 rounded-full bg-green-500 text-white text-xs opacity-30 hover:opacity-100 transition-opacity cursor-pointer"
        style={`top: ${0.5 + 3.5 * index}rem`}
        onclick={() => handlerInsert(node, child_node)}
    >+</button>
{/each}

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
