<!--
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
-->

<script lang="ts">
    import type { Node, RootNode, TreeUpdateStructureData, TreeWithCards } from '$lib/backend/trees';
    import { updateTreeStructure } from "$lib/backend/trees";
    import { addCard, type CardAddData, type CardId } from '$lib/backend/cards';

    /** Variables used to contain data about the entire tree that is needed for API calls. */
    let { tree = $bindable(), node = $bindable() }:
        { tree: TreeWithCards, node: Node | RootNode } = $props();

    let frontText = $state('');
    let backText = $state('');
    let errorMessage = $state('');

    /** Variable to have the add/insert action on hold. */
    let action = $state<null |
        { type: 'add', parent: Node | RootNode } |
        { type: 'insert', parent: Node | RootNode, child: Node }>(null);


    /** Gets called when the button to add a card is clicked;
     * Sets the action to the 'add' type.
     *
     * @param parent The node that will have the new node as a child.
     */
    function handlerAdd(parent: Node | RootNode) {
        action = { type: 'add', parent };
    }

    /** Gets called when the button to insert a card is clicked;
     * Sets the action to 'insert' type.
     *
     * @param parent The node that will have the new node as a child.
     * @param child The node that will now be a child of the new node instead of parent.
     */
    function handlerInsert(parent: Node | RootNode, child: Node) {
        action = { type: 'insert', parent, child };
    }

    /** Gets called when the Add button in the popup is clicked;
     * Checks the action and executes the correct function.
     *
     * @param front The front text of the card.
     * @param back The back text of the card.
     */
    function acceptCard(front: string, back: string) {
        if (!action) return;
        errorMessage = '';

        let card: CardAddData = {
            treeId: tree.genData.id,
            front: front,
            back: back,
        }

        if (action.type === 'add') {
            addChild(action.parent, card);
        } else if (action.type === 'insert') {
            insertChild(action.parent, action.child, card);
        }
    }

    /** Adds a new child node
     *
     * @param parent The node that will have the new node as a child.
     * @param card The data used to construct the new node.
     */
    async function addChild(parent: Node | RootNode, card: CardAddData) {
        addCard(card)
            .then((cardId: CardId) => {
                const newNode: Node = {cardId, children: []};
                tree.cards[cardId] = {
                    front: card.front,
                    back: card.back
                };
                parent.children.push(newNode);
                let update: TreeUpdateStructureData = {
                    id: tree.genData.id,
                    newStructure: tree.structure,
                }
                return updateTreeStructure(update);
            })

            .then(() => {
                action = null;
                parent = {...parent}
                parent.children[-1] = {...parent.children[-1]}
                tree.structure = {...tree.structure};
            })

            .catch((reason: Error) => {
                errorMessage = reason.toString();
                console.error('Error: ', reason);
            });
    }

    /** Inserts a new child between two nodes
     *
     * @param parent The node that will have the new node as a child.
     * @param child The node that will now be a child of the new node instead of parent.
     * @param card The data used to construct the new node.
     */
    async function insertChild(parent: Node | RootNode, child: Node, card: CardAddData) {
        addCard(card)
            .then((cardId: CardId) => {
                const newNode: Node = {cardId, children: [child]};
                tree.cards[cardId] = {
                    front: card.front,
                    back: card.back
                };
                parent.children = parent.children.map(c => c === child ? newNode : c) as Node[];
                let update: TreeUpdateStructureData = {
                    id: tree.genData.id,
                    newStructure: tree.structure,
                }
                return updateTreeStructure(update);
            })

            .then(() => {
                action = null;
                parent = {...parent}
                parent.children[-1] = {...parent.children[-1]}
                tree.structure = {...tree.structure};
            })

            .catch((reason: Error) => {
                errorMessage = reason.toString();
                console.error('Error: ', reason);
            });
    }
</script>

<!-- Button to add new card as a child to this card -->
<button
    class="absolute top-2 right-2 z-10 h-5 w-5 rounded-full bg-blue-500 text-white text-xs opacity-30 hover:opacity-100 transition-opacity cursor-pointer"
    onclick={() => handlerAdd(node)}
>+</button>
<!-- Buttons to insert a new card between this card and a child to this card -->
{#each node.children as child_node, index (child_node)}
    <button
        class="absolute -right-20 z-10 h-5 w-5 rounded-full bg-green-500 text-white text-xs opacity-30 hover:opacity-100 transition-opacity cursor-pointer"
        style={`top: ${0.5 + 3.5 * index}rem`}
        onclick={() => handlerInsert(node, child_node)}
    >+</button>
{/each}

<!-- Add/Insert card popup -->
{#if action}
    <div class="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
        <div class="bg-white rounded p-4 flex flex-col gap-2">
            <textarea
                class="border rounded p-1"
                placeholder="Enter card front text..."
                bind:value={frontText}
            ></textarea>
            <textarea
                class="border rounded p-1"
                placeholder="Enter card back text..."
                bind:value={backText}
            ></textarea>
            {#if errorMessage}
                <p class="text-red-600">{errorMessage}</p>
            {/if}
            <div class="flex gap-2 justify-end">
                <button onclick={() => action = null}>Cancel</button>
                <button onclick={() => acceptCard(frontText, backText)} disabled={!frontText || !backText}>Add</button>
            </div>
        </div>
    </div>
{/if}
