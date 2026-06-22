<!--
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
-->

<!--
    @component
    Render a pop-up allowing the user to add new lessons to a classroom.

    ## Params
    - `classroomId` - The id of the classroom to which the lesson will be added, corresponding to the classroom [classroom_id] currently open.
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

    const minInputLengths = {
        name: 1,
        description: 0
    };

    const maxInputLengths = {
        name: 32,
        description: 128
    };

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
    class="cursor-pointer rounded-md border-2 border-neutral-600 bg-neutral-200 p-1 text-lg font-bold text-black transition-all duration-200 ease-in-out hover:bg-neutral-600 hover:text-white"
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
        <div class="mb-3 flex w-100 flex-col" data-testid="add-lesson-popup">
            <form
                class={[lessonAddData.name !== '' && 'completed', 'w-100']}
                onsubmit={createLesson}
            >
                <label for="lessonName">{m.lessonName()} *</label>
                <input
                    id="lessonName"
                    class="input__text"
                    type="text"
                    minlength={minInputLengths.name}
                    maxlength={maxInputLengths.name}
                    bind:value={lessonAddData.name}
                    placeholder={m.examplePlaceholderRequired({ value: m.exampleLessonName() })}
                    required
                />
                <label for="lessonDescription">{m.description()}</label>
                <textarea
                    id="lessonDescription"
                    rows="3"
                    class="input__long_text"
                    minlength={minInputLengths.description}
                    maxlength={maxInputLengths.description}
                    bind:value={lessonAddData.description}
                    placeholder={m.examplePlaceholder({ value: m.exampleLessonDescription() })}
                ></textarea>
                <button class="mt-0.5" type="submit">{m.addLesson()}</button>
                <button class="mt-0.5" type="reset" onclick={closeEdit}>{m.close()}</button>
            </form>
        </div>
    </Popup>
{/if}
