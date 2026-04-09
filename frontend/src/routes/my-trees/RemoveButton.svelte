<!--
    This program has been developed by students from the bachelor Computer Science at Utrecht
    University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
-->

<script lang="ts">
    import { page } from '$app/state';
    import trashIcon from '$lib/assets/trashIcon.svg?raw';
    import { removeTree, type TreeMetadata } from '$lib/backend/trees';
    import { m } from '$lib/paraglide/messages';
    import Popup from '$lib/Popup.svelte';

    let { thisTree, allTrees = $bindable() }: { thisTree: TreeMetadata; allTrees: TreeMetadata[] } =
        $props();

    let popupVisible = $state(false);

    function openPopup(event: Event) {
        event.preventDefault();
        popupVisible = true;
    }

    function cancelRemoval() {
        popupVisible = false;
    }

    async function doRemoval() {
        await removeTree(thisTree.genData.id);
        allTrees = allTrees!.filter((tree) => tree.genData.id !== thisTree.genData.id);
        // This popup doesn't need to be closed, because this component will
        // be automatically removed with the table row of this tree.
    }
</script>

<a href={page.url.href} onclick={openPopup}>
    <!-- eslint-disable-next-line svelte/no-at-html-tags -->
    {@html trashIcon}
</a>

{#if popupVisible}
    <Popup>
        <p class="text-lg font-bold">
            {m.deleteTreeConfirmation({ title: thisTree.data.title })}
        </p>
        <div class="flex justify-end gap-2">
            <button class="rounded-md bg-gray-400 p-2" onclick={cancelRemoval}>{m.cancel()}</button>
            <button class="rounded-md bg-red-400 p-2" onclick={doRemoval}>{m.yes()}</button>
        </div>
    </Popup>
{/if}
