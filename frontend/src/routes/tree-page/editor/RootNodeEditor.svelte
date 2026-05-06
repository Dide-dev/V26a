<!--
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
-->

<script lang="ts">
    import iconArrowUp from '$lib/assets/icons/arrowUp.svg?raw';
    import iconArrowDown from '$lib/assets/icons/arrowDown.svg?raw';
    import iconEdit from '$lib/assets/icons/edit.svg?raw';
    import iconTrash from '$lib/assets/icons/trash.svg?raw';
    import {
        doRootNodeMove,
        doRootNodeDelete,
        type TreeHistory,
        doRootNodeEdit
    } from './history.svelte';
    import RootNodeTextarea from './RootNodeTextarea.svelte';
    import { m } from '$lib/paraglide/messages';

    /** Variables used to contain data about the entire tree that is needed for API calls. */
    let {
        history,
        index
    }: {
        history: TreeHistory;
        index: number;
    } = $props();

    /**
     * If this is null no editing box is shown.
     * If not null, contains the currently being edited label text.
     */
    let inlineEditing: null | string = $state(null);

    let rootNode = $derived(history.currentStructure.rootNodes[index]);
    let canDelete = $derived(rootNode.children.length === 0);
    let canMoveUp = $derived(index > 0);
    let canMoveDown = $derived(index < history.currentStructure.rootNodes.length - 1);

    async function moveDownClick() {
        if (!canMoveDown) return;

        await doRootNodeMove(history, index, index + 1);
    }
    async function moveUpClick() {
        if (!canMoveUp) return;

        await doRootNodeMove(history, index, index - 1);
    }
    async function deleteClick() {
        if (!canDelete) return;

        await doRootNodeDelete(history, index);
    }

    function editClick() {
        inlineEditing = rootNode.label;
    }
    function editCancel() {
        inlineEditing = null;
    }
    async function editFinish() {
        if (inlineEditing === null) return;

        if (inlineEditing === rootNode.label) {
            editCancel();
            return;
        }

        // If successfull, this will edit the history, which will update the label.
        // So we don't have to do that here locally.
        await doRootNodeEdit(history, index, inlineEditing);
    }
</script>

<!-- The row of buttons above the . -->
<div class="absolute -top-6 left-1 flex h-7 border border-fuchsia-600 bg-fuchsia-300">
    <button class="cursor-pointer" title={m.editRootnodeLabel()} onclick={editClick}>
        <!-- eslint-disable-next-line svelte/no-at-html-tags -->
        {@html iconEdit}
    </button>
    <button
        class={[canMoveDown && 'cursor-pointer text-black!', !canMoveDown && 'text-gray-500!']}
        title={m.moveDown()}
        onclick={moveDownClick}
    >
        <!-- eslint-disable-next-line svelte/no-at-html-tags -->
        {@html iconArrowDown}
    </button>
    <button
        class={[canMoveUp && 'cursor-pointer text-black!', !canMoveUp && 'text-gray-500']}
        title={m.moveUp()}
        onclick={moveUpClick}
    >
        <!-- eslint-disable-next-line svelte/no-at-html-tags -->
        {@html iconArrowUp}
    </button>
    <button
        class={[canDelete && 'cursor-pointer text-black!', !canDelete && 'text-gray-500!']}
        title={canDelete ? m.deleteRootnode() : m.deleteRootnodeBlocked()}
        onclick={deleteClick}
    >
        <!-- eslint-disable-next-line svelte/no-at-html-tags -->
        {@html iconTrash}
    </button>
</div>

{#if inlineEditing === null}
    <div class="mb-5 h-fit w-32 shrink-0 cursor-default wrap-anywhere">
        <strong>{rootNode.label}</strong>
    </div>
{:else}
    <RootNodeTextarea
        bind:text={inlineEditing}
        editDoneCallback={editFinish}
        editCancelCallback={editCancel}
    />
{/if}
