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
    import { type TreeId } from '$lib/api-wrapper/trees';
    import { m } from '$lib/paraglide/messages';
    import Popup from '$lib/Popup.svelte';
    import iconAdd from '$lib/assets/icons/add.svg?raw';

    let { lessonId }: { lessonId: LessonId } = $props();

    let visible = $state(false);

    let treeId: TreeId | null = $state(null);

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

    async function addTree() {
        if (treeId === null) {
            return;
        }

        treeId = Number(treeId);

        await addTreeToLesson(lessonId, treeId);
        closeAdd();
    }
</script>

<button
    title={m.addTree()}
    aria-label={m.addTree()}
    class="cursor-pointer pr-2.5"
    onclick={(event) => {
        event.stopPropagation();
        addTreeButtonClick();
    }}
>
    <!-- eslint-disable-next-line svelte/no-at-html-tags -->
    {@html iconAdd}
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
            <form class="w-64">
                <label class="m-0.5" for="treeId">{m.treeId()} *</label>
                <input
                    id="treeId"
                    type="number"
                    class="input__text m-0.5"
                    bind:value={treeId}
                    placeholder={m.examplePlaceholderRequired({ value: '1' })}
                    onclick={(event) => {
                        event.stopPropagation();
                    }}
                    required
                />
                <button
                    class="m-0.5 cursor-pointer"
                    type="submit"
                    onclick={(event) => {
                        event.stopPropagation();
                        addTree();
                    }}>{m.addTree()}</button
                >
                <button
                    class="m-0.5 cursor-pointer"
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
