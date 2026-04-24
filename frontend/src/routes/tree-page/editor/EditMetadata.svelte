<!--
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
-->

<script lang="ts">
    import { updateTreeMetaData, type TreeData, type TreeId } from '$lib/backend/trees';
    import { m } from '$lib/paraglide/messages';
    import Popup from '$lib/Popup.svelte';

    let { treeId, prevTreeData = $bindable() }: { treeId: TreeId; prevTreeData: TreeData } =
        $props();

    let visible = $state(false);
    let treeData: TreeData = $state({
        title: prevTreeData.title ?? '',
        description: prevTreeData.description ?? '',
        bookName: prevTreeData.bookName ?? '',
        bookEdition: prevTreeData.bookEdition ?? '',
        bookIsbn: prevTreeData.bookIsbn ?? '',
        bookChapter: prevTreeData.bookChapter ?? '',
        bookExercise: prevTreeData.bookExercise ?? '',
        assignmentText: prevTreeData.assignmentText ?? '',
        published: prevTreeData.published ?? false
    });

    async function saveData() {
        await updateTreeMetaData({
            id: treeId,
            newData: { ...treeData }
        });
        visible = false;
        prevTreeData = { ...treeData };
    }

    // Restores treeData to its original value.
    function cancelEdit() {
        treeData = { ...prevTreeData };
        visible = false;
    }

    // The form submit keyboard shortcut
    function onkeydown(event: KeyboardEvent) {
        if (event.ctrlKey && event.key == 'Enter') {
            saveData();
            event.stopPropagation();
        }
    }
</script>

<button class="cursor-pointer rounded-sm bg-emerald-500 p-1" onclick={() => (visible = !visible)}>
    {m.editMetadata()}
</button>

<svelte:window {onkeydown} />

{#if visible}
    <Popup>
        <div class="mb-3 flex gap-3" data-testid="metadata-popup">
            <div class="w-64">
                <label
                    >{m.title()}<input
                        class="w-full rounded border p-1"
                        bind:value={treeData.title}
                    /></label
                >
                <label
                    >{m.description()}<textarea
                        class="w-full rounded border p-1"
                        bind:value={treeData.description}
                    ></textarea></label
                >
                <label
                    >{m.bookname()}<input
                        class="w-full rounded border p-1"
                        bind:value={treeData.bookName}
                    /></label
                >
                <label
                    >{m.bookEdition()}<input
                        class="w-full rounded border p-1"
                        bind:value={treeData.bookEdition}
                    /></label
                >
            </div>
            <div class="w-64">
                <label
                    >{m.bookIsbn()}<input
                        class="w-full rounded border p-1"
                        bind:value={treeData.bookIsbn}
                    /></label
                >
                <label
                    >{m.bookChapter()}<input
                        class="w-full rounded border p-1"
                        bind:value={treeData.bookChapter}
                    /></label
                >
                <label
                    >{m.bookExercise()}<input
                        class="w-full rounded border p-1"
                        bind:value={treeData.bookExercise}
                    /></label
                >
                <label
                    >{m.assignmentText()}<input
                        class="w-full rounded border p-1"
                        bind:value={treeData.assignmentText}
                    /></label
                >
                <label
                    >{m.published()}<input
                        type="checkbox"
                        bind:checked={treeData.published}
                    /></label
                >
            </div>
        </div>
        <button class="cursor-pointer rounded bg-blue-500 px-3 py-1 text-white" onclick={saveData}
            >{m.save()}</button
        >
        <button class="cursor-pointer" onclick={cancelEdit}>{m.cancel()}</button>
    </Popup>
{/if}
