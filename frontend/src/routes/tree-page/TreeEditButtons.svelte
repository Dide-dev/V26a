<!--
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
-->

<script lang="ts">
    import iconEdit from '$lib/assets/iconEdit.svg?raw';
    import iconTrash from '$lib/assets/iconTrash.svg?raw';
    import iconMove from '$lib/assets/iconMove.svg?raw';
    import type { Node, RootNode, TreeRemoveCardData, TreeWithCards } from '$lib/backend/trees';
    import { removeCardFromTree } from '$lib/backend/trees';
    import { updateCard, type CardUpdateData, type CardGetData } from '$lib/backend/cards';
    import { m } from '$lib/paraglide/messages.js';
    import type { DragState } from './NodeViewer.svelte';
    import CardEditor from './CardEditor.svelte';

    /** Variables used to contain data about the entire tree that is needed for API calls. */
    let {
        tree = $bindable(),
        parent = $bindable(),
        node = $bindable(),
        dragState = $bindable()
    }: {
        tree: TreeWithCards;
        parent: Node | RootNode;
        node: Node;
        dragState: null | DragState;
    } = $props();

    /** In this component the card editor popup is only used with the 'edit' button */
    let showEditPopup = $state(false);

    async function removeButtonClick() {
        const index = parent.children.indexOf(node);
        parent.children.splice(index, 1, ...node.children);

        const remove: TreeRemoveCardData = {
            treeId: tree.genData.id,
            cardId: node.cardId,
            newStructure: tree.structure
        };
        await removeCardFromTree(remove);
    }

    async function finishEdit(newCard: CardGetData) {
        let update: CardUpdateData = {
            cardId: node.cardId,
            newFront: newCard.front,
            newBack: newCard.back
        };
        await updateCard(update);

        tree.cards[update.cardId] = {
            front: update.newFront,
            back: update.newBack
        };

        showEditPopup = false;
    }

    function startDrag(event: MouseEvent) {
        dragState = {
            type: 'dragging',
            parent,
            index: parent.children.indexOf(node),
            mouseX: event.clientX,
            mouseY: event.clientY
        };
    }
</script>

<div class="absolute -top-6 left-2 h-7 border border-fuchsia-600 bg-fuchsia-300">
    <button
        class="inline-block cursor-move"
        title={m.moveCardWithSubtree()}
        onmousedown={startDrag}
    >
        <!-- eslint-disable-next-line svelte/no-at-html-tags -->
        {@html iconMove}
    </button>
    <button class="cursor-pointer" title={m.editThisCard()} onclick={() => (showEditPopup = true)}>
        <!-- eslint-disable-next-line svelte/no-at-html-tags -->
        {@html iconEdit}
    </button>
    <button class="cursor-pointer" title={m.deleteThisCard()} onclick={removeButtonClick}>
        <!-- eslint-disable-next-line svelte/no-at-html-tags -->
        {@html iconTrash}
    </button>
</div>

<!-- Edit card popup -->
{#if showEditPopup}
    <!--
        Very important to make a copy of the card here.
        We want the user to use the cancel/confirm buttons, not to directly edit the card itself.
    -->
    <CardEditor
        card={{ ...tree.cards[node.cardId] }}
        confirmText={m.edit()}
        confirmCallback={finishEdit}
        cancelCallback={() => (showEditPopup = false)}
    />
{/if}
