<!--
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
-->

<!--
    @component
    Render a pop-up allowing the user to add new lessons to a classroom.

    ## Params
    - `classroomId` - The id of the classroom to which the lesson will be added, corresponding to the classroom dashboard currently open.
-->

<script lang="ts">
    import { type ClassroomId } from '$lib/api-wrapper/classrooms';
    import { addLesson, type LessonAddData } from '$lib/api-wrapper/classrooms';
    import { m } from '$lib/paraglide/messages';
    import Popup from '$lib/Popup.svelte';

    let {
        classroomId,
        onLessonAdded
    }: { classroomId: ClassroomId; onLessonAdded: () => Promise<void> } = $props();

    let visible = $state(false);

    let lessonAddData: LessonAddData = $state({
        name: '',
        description: '',
        published: true
    });

    function addLessonButtonClick() {
        visible = true;
    }

    function closeEdit() {
        lessonAddData.name = '';
        lessonAddData.description = '';
        lessonAddData.published = true;
        visible = false;
    }

    function onkeydown(event: KeyboardEvent) {
        if (!visible) return;

        // When ctrl+z is pressed, we only want that event to be handled in the relevant textbox,
        // do not let it propagate to the undo/redo buttons of the tree editor page.
        event.stopPropagation();

        // The form submit keyboard shortcut
        if (event.ctrlKey && event.key == 'Enter') {
            createLesson();
        }
    }

    async function createLesson() {
        await addLesson(lessonAddData, classroomId);
        await onLessonAdded();
        closeEdit();
    }
</script>

<button
    class="cursor-pointer rounded-md border-2 border-current bg-neutral-200 p-1 text-lg font-bold text-neutral-600 transition-all duration-200"
    onclick={addLessonButtonClick}
>
    {m.addLesson()}
</button>

<!--
    We bind this event to the body instead of the window,
    so that this one fires before the onkeydown in the +page.svelte,
    so it can possibly stop the propagation.
-->
<svelte:body {onkeydown} />

{#if visible}
    <Popup>
        <div class="mb-3 flex w-125 flex-col" data-testid="add-lesson-popup">
            <form>
                <label for="lessonName">{m.lessonName()} *</label>
                <input
                    id="lessonName"
                    class="input__text"
                    bind:value={lessonAddData.name}
                    placeholder={m.examplePlaceholderRequired({ value: m.exampleLessonName() })}
                    required
                />
                <label for="lessonDescription">{m.description()}</label>
                <textarea
                    id="lessonDescription"
                    rows="3"
                    class="input__long_text"
                    bind:value={lessonAddData.description}
                    placeholder={m.examplePlaceholder({ value: m.exampleLessonDescription() })}
                ></textarea>
            </form>
        </div>
        <button
            class="cursor-pointer rounded-md border-2 border-current bg-neutral-200 p-1 text-lg font-bold text-neutral-600 transition-all duration-200"
            onclick={createLesson}>{m.addLesson()}</button
        >
        <button
            class="cursor-pointer rounded-md border-2 border-current bg-neutral-200 p-1 text-lg font-bold text-neutral-600 transition-all duration-200"
            onclick={closeEdit}>{m.close()}</button
        >
    </Popup>
{/if}
