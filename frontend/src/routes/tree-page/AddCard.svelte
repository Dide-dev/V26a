<!--
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
-->

<script lang="ts">
    import type {
        Node,
        RootNode,
        TreeUpdateStructureData,
        TreeWithCards
    } from '$lib/backend/trees';
    import { updateTreeStructure } from '$lib/backend/trees';
    import { addCard, type CardAddData, type CardId } from '$lib/backend/cards';
    import { m } from '$lib/paraglide/messages.js';

    /** Variables used to contain data about the entire tree that is needed for API calls. */
    let {
        tree = $bindable(),
        parent = $bindable(),
        node = $bindable()
    }: { tree: TreeWithCards; parent: Node | RootNode; node: Node | RootNode } = $props();

    let frontText = $state('');
    let backText = $state('');
    let errorMessage = $state('');

    const frontMaxLength = 1024;
    const backMaxLength = 5000;

    /** Variable to have the add/insert action on hold. */
    let action = $state<
        | null
        | { type: 'add'; parent: Node | RootNode }
        | { type: 'insert'; parent: Node | RootNode; child: Node }
    >(null);

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
            back: back
        };

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
        if (card.front.length > frontMaxLength) {
            errorMessage = `Front text cannot contain more than ${frontMaxLength} characters`;
            return;
        }
        if (card.back.length > backMaxLength) {
            errorMessage = `Back text cannot contain more than ${backMaxLength} characters`;
            return;
        }

        addCard(card)
            .then((cardId: CardId) => {
                const newNode: Node = { cardId, children: [] };
                tree.cards[cardId] = {
                    front: card.front,
                    back: card.back
                };
                parent.children.push(newNode);
                let update: TreeUpdateStructureData = {
                    id: tree.genData.id,
                    newStructure: tree.structure
                };
                return updateTreeStructure(update);
            })

            .then(() => {
                parent = { ...parent };
                parent.children[-1] = { ...parent.children[-1] };
                tree.structure = { ...tree.structure };
                reset();
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
        if (card.front.length > frontMaxLength) {
            errorMessage = m.frontTextCharacterLimit({ limit: frontMaxLength });
            return;
        }
        if (card.back.length > backMaxLength) {
            errorMessage = m.backTextCharacterLimit({ limit: backMaxLength });
            return;
        }

        addCard(card)
            .then((cardId: CardId) => {
                const newNode: Node = { cardId, children: [child] };
                tree.cards[cardId] = {
                    front: card.front,
                    back: card.back
                };
                parent.children = parent.children.map((c) => (c === child ? newNode : c)) as Node[];
                let update: TreeUpdateStructureData = {
                    id: tree.genData.id,
                    newStructure: tree.structure
                };
                return updateTreeStructure(update);
            })

            .then(() => {
                parent = { ...parent };
                parent.children[-1] = { ...parent.children[-1] };
                tree.structure = { ...tree.structure };
                reset();
            })

            .catch((reason: Error) => {
                errorMessage = reason.toString();
                console.error('Error: ', reason);
            });
    }

    /** Reset function to clear the popup */
    function reset() {
        action = null;
        frontText = '';
        backText = '';
    }
</script>

<!-- Button to add new card as a child to this card -->
<button
    class="card__add_button absolute top-2 right-2 h-5 w-5 cursor-pointer rounded-full bg-cyan-400 text-xs text-white opacity-60 transition-opacity hover:opacity-100"
    onclick={() => handlerAdd(node)}>+</button
>
<!-- Buttons to insert a new card between this card and a child to this card -->
{#if !('label' in node)}
    <button
        class="card__insert_button absolute top-2 right-[548] h-5 w-5 cursor-pointer rounded-full bg-yellow-300 text-xs text-white opacity-60 transition-opacity hover:opacity-100"
        onclick={() => handlerInsert(parent, node)}>+</button
    >
{/if}

<!-- Add/Insert card popup -->
{#if action}
    <div class="fixed inset-0 z-50 flex items-center justify-center bg-black/50">
        <div class="card__popup flex flex-col gap-2 rounded bg-white p-4">
            <textarea
                class="card__front_text_box rounded border p-1"
                placeholder={m.enterFrontText()}
                bind:value={frontText}
            ></textarea>
            <textarea
                class="card__back_text_box rounded border p-1"
                placeholder={m.enterBackText()}
                bind:value={backText}
            ></textarea>
            {#if errorMessage}
                <p class="text-red-600">{errorMessage}</p>
            {/if}
            <div class="flex justify-end gap-2">
                <button onclick={() => reset()}>Cancel</button>
                <button
                    class="card__confirm_button"
                    onclick={() => {
                        if (backText.trim()) acceptCard(frontText.trim(), backText.trim());
                        else errorMessage = m.backTextRequired();
                    }}>Add</button
                >
            </div>
        </div>
    </div>
{/if}
