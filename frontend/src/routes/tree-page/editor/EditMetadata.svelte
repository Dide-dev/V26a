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
        title: prevTreeData.title,
        languageCode: prevTreeData.languageCode,
        description: prevTreeData.description,
        tags: prevTreeData.tags,
        subjectId: prevTreeData.subjectId,
        bookName: prevTreeData.bookName,
        bookEdition: prevTreeData.bookEdition,
        bookIsbn: prevTreeData.bookIsbn ?? '',
        bookAuthor: prevTreeData.bookAuthor,
        bookChapter: prevTreeData.bookChapter,
        bookExercise: prevTreeData.bookExercise,
        assignmentText: prevTreeData.assignmentText,
        published: prevTreeData.published
    });

    async function saveData() {
        await updateTreeMetaData({
            id: treeId,
            newData: { ...treeData, bookIsbn: treeData.bookIsbn === '' ? null : treeData.bookIsbn }
        });
        visible = false;
        prevTreeData = { ...treeData };
    }

    // Restores treeData to its original value.
    function cancelEdit() {
        treeData = { ...prevTreeData };
        visible = false;
    }

    function onEditMetadataButtonClick() {
        visible = true;
    }

    function onkeydown(event: KeyboardEvent) {
        if (!visible) return;

        // When ctrl+z is pressed, we only want that event to be handled in the relevant textbox,
        // do not let it propagate to the undo/redo buttons of the tree editor page.
        event.stopPropagation();

        // The form submit keyboard shortcut
        if (event.ctrlKey && event.key == 'Enter') {
            saveData();
        }
    }
</script>

<button class="cursor-pointer rounded-sm bg-emerald-500 p-1" onclick={onEditMetadataButtonClick}>
    {m.editMetadata()}
</button>

<!--
    We bind this event to the body instead of the window,
    so that this one fires before the onkeydown in the +page.svelte,
    so it can possibly stop the propagation.
-->
<svelte:body {onkeydown} />

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
                    >{m.bookName()}<input
                        class="w-full rounded border p-1"
                        bind:value={treeData.bookName}
                    /></label
                >
                <label
                    >{m.edition()}<input
                        class="w-full rounded border p-1"
                        bind:value={treeData.bookEdition}
                    /></label
                >
            </div>
            <div class="w-64">
                <label
                    >{m.isbn()}<input
                        class="w-full rounded border p-1"
                        bind:value={treeData.bookIsbn}
                    /></label
                >
                <label
                    >{m.chapter()}<input
                        class="w-full rounded border p-1"
                        bind:value={treeData.bookChapter}
                    /></label
                >
                <label
                    >{m.exercise()}<input
                        class="w-full rounded border p-1"
                        bind:value={treeData.bookExercise}
                    /></label
                >
                <label
                    >{m.assignment()}<input
                        class="w-full rounded border p-1"
                        bind:value={treeData.assignmentText}
                    /></label
                >
                <div>
                    <label for="published">{m.published()}</label>
                    <input id="published" type="checkbox" bind:checked={treeData.published} />
                </div>
            </div>
        </div>
        <button class="cursor-pointer rounded bg-blue-500 px-3 py-1 text-white" onclick={saveData}
            >{m.save()}</button
        >
        <button class="cursor-pointer" onclick={cancelEdit}>{m.cancel()}</button>
    </Popup>
{/if}
