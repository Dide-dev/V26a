<!--
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
-->

<!--
    @component
    Render a pop-up allowing the user to add trees to lessons.

    ## Params
    - `lessonId` - The id of the lesson to which the tree will be added, corresponding to the lesson on which the add button has been pressed.
-->

<script lang="ts">
    import { addTreeToLesson, type LessonId } from '$lib/api-wrapper/lessons';
    import { getTreeWithCards, type TreeId, type TreeWithCards } from '$lib/api-wrapper/trees';
    import { m } from '$lib/paraglide/messages';
    import Popup from '$lib/Popup.svelte';
    import { PlusOutline } from 'flowbite-svelte-icons';

    let { lessonId, onTreeAdded }: { lessonId: LessonId; onTreeAdded: () => Promise<void> } =
        $props();

    let visible = $state(false);

    let treeId: TreeId | null = $state(null);
    let firstUnpublishedNoticeGiven: boolean = $state(false);
    let isTreePublished: boolean = $state(false);
    let tree: TreeWithCards | null = $state(null);

    function addTreeButtonClick() {
        visible = true;
    }

    function closeAdd() {
        treeId = null;
        visible = false;
    }

    function onkeydown(event: KeyboardEvent) {
        if (!visible) return;

        // When ctrl+z is pressed, we only want that event to be handled in the relevant textbox,
        // do not let it propagate to the undo/redo buttons of the tree editor page.
        event.stopPropagation();

        // The form submit keyboard shortcut
        if (event.ctrlKey && event.key == 'Enter') {
            addTree();
        }
    }

    async function submitAddTree(event: SubmitEvent) {
        event.stopPropagation();
        event.preventDefault();
        await addTree();
    }

    async function addTree() {
        if (treeId === null) {
            return;
        }

        tree = await getTreeWithCards(treeId);
        if (!(tree === null)) {
            isTreePublished = tree.metadata.data.published;
        }

        if (!firstUnpublishedNoticeGiven && !isTreePublished) {
            firstUnpublishedNoticeGiven = true;
            return;
        }
        firstUnpublishedNoticeGiven = false;

        await addTreeToLesson(lessonId, treeId);
        await onTreeAdded();
        closeAdd();
    }
</script>

<button
    title={m.addTree()}
    aria-label={m.addTree()}
    class="mr-2.5 cursor-pointer transition-all duration-200 ease-in-out hover:bg-neutral-200"
    onclick={(event) => {
        event.stopPropagation();
        addTreeButtonClick();
    }}
>
    <PlusOutline class="h-5 w-5 shrink-0" />
</button>

<!--
    We bind this event to the body instead of the window,
    so that this one fires before the onkeydown in the +page.svelte,
    so it can possibly stop the propagation.
-->
<svelte:body {onkeydown} />

{#if visible}
    <Popup>
        <div class="mb-3 flex w-64 flex-col" data-testid="add-tree-popup">
            <form class={[treeId !== null && 'completed', 'w-64']} onsubmit={submitAddTree}>
                <label class="m-0.5" for="treeId">{m.treeId()} *</label>
                <input
                    id="treeId"
                    type="number"
                    class="input__text"
                    bind:value={treeId}
                    placeholder={m.examplePlaceholderRequired({ value: '1' })}
                    onclick={(event) => {
                        event.stopPropagation();
                    }}
                    required
                />
                {#if firstUnpublishedNoticeGiven}
                    <p class="w-full pt-0.5 pb-0.5 text-sm text-amber-600">
                        {m.unpublishedTreeLesson()}
                    </p>
                {/if}
                <button
                    class="mt-0.5"
                    type="submit"
                    onclick={(event) => {
                        event.stopPropagation();
                    }}>{m.addTree()}</button
                >
                <button
                    class="mt-0.5"
                    type="reset"
                    onclick={(event) => {
                        event.stopPropagation();
                        closeAdd();
                    }}>{m.close()}</button
                >
            </form>
        </div>
    </Popup>
{/if}
