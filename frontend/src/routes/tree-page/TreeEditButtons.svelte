<!--
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
-->

<script lang="ts">
    import iconEdit from '$lib/assets/icons/edit.svg?raw';
    import iconTrash from '$lib/assets/icons/trash.svg?raw';
    import iconMove from '$lib/assets/icons/move.svg?raw';
    import type { CardGetData } from '$lib/backend/cards';
    import { m } from '$lib/paraglide/messages.js';
    import type { DragState } from './NodeViewer.svelte';
    import CardEditor from './CardEditor.svelte';
    import { doDeleteCard, doEditCard, followIndexNode, type TreeHistory } from './history.svelte';

    /** Variables used to contain data about the entire tree that is needed for API calls. */
    let {
        history,
        index,
        dragState = $bindable(),
        numPopupsOpen = $bindable()
    }: {
        history: TreeHistory;
        index: number[];
        dragState: null | DragState;
        numPopupsOpen: number;
    } = $props();

    /** In this component the card editor popup is only used with the 'edit' button */
    let showEditPopup = $state(false);

    async function removeButtonClick() {
        await doDeleteCard(history, index);
    }

    async function finishEdit(newCard: CardGetData) {
        await doEditCard(history, cardId, newCard);
        showEditPopup = false;
    }

    function startDrag(event: MouseEvent) {
        dragState = {
            type: 'dragging',
            nodeToMove: index,
            mouseX: event.clientX,
            mouseY: event.clientY
        };
    }

    let cardId = $derived.by(() => {
        let node = followIndexNode(history.currentStructure, index);
        return node.cardId;
    });
    let card = $derived(history.currentCards[cardId]);
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
        card={{ ...card }}
        confirmText={m.edit()}
        confirmCallback={finishEdit}
        cancelCallback={() => (showEditPopup = false)}
        bind:numPopupsOpen
    />
{/if}
