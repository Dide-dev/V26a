<!--
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
-->

<script lang="ts">
    import iconEdit from '$lib/assets/icons/edit.svg?raw';
    import iconTrash from '$lib/assets/icons/trash.svg?raw';
    import iconMove from '$lib/assets/icons/move.svg?raw';
    import iconUpload from '$lib/assets/icons/upload.svg?raw';
    import { m } from '$lib/paraglide/messages.js';
    import type { DragState } from './SubtreeEditor.svelte';
    import { doDeleteCard, followIndexNode, type TreeHistory } from './history.svelte';
    import type { SideBar } from './sidebar/Sidebar.svelte';

    /** Variables used to contain data about the entire tree that is needed for API calls. */
    let {
        history,
        location,
        dragState = $bindable(),
        sidebar = $bindable()
    }: {
        history: TreeHistory;
        location: number[];
        dragState: null | DragState;
        sidebar: SideBar;
    } = $props();

    async function removeButtonClick() {
        sidebar = {
            kind: 'Menu',
            type: 'IconsOnly'
        };
        await doDeleteCard(history, location);
    }

    function startDrag(event: MouseEvent) {
        dragState = {
            type: 'dragging',
            nodeToMove: location,
            mouseX: event.clientX,
            mouseY: event.clientY
        };
        sidebar = {
            kind: 'Menu',
            type: 'IconsOnly'
        };
    }

    let cardId = $derived.by(() => {
        let node = followIndexNode(history.currentStructure, location);
        return node.cardId;
    });
    let card = $derived(history.currentCards[cardId]);

    function editClick(e: Event) {
        e.stopPropagation();

        sidebar = {
            kind: 'CardEditor',
            type: 'EditCard',
            location,
            editingVersion: $state.snapshot(card)
        };
    }

    function uploadClick(e: Event) {
        e.stopPropagation();

        sidebar = {
            kind: 'CardEditor',
            type: 'UploadToLibrary',
            location,
            editingVersion: $state.snapshot(card)
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
    <button class="cursor-pointer" title={m.editThisCard()} onclick={editClick}>
        <!-- eslint-disable-next-line svelte/no-at-html-tags -->
        {@html iconEdit}
    </button>
    <button class="cursor-pointer" title={m.deleteThisCard()} onclick={removeButtonClick}>
        <!-- eslint-disable-next-line svelte/no-at-html-tags -->
        {@html iconTrash}
    </button>
    <button class="cursor-pointer" title={m.uploadThisCard()} onclick={uploadClick}>
        <!-- eslint-disable-next-line svelte/no-at-html-tags -->
        {@html iconUpload}
    </button>
</div>
