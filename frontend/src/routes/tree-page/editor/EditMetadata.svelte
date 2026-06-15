<!--
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
-->

<script lang="ts">
    import { onMount } from 'svelte';
    import {
        getAllLanguages,
        getAllSubjects,
        minInputLengths,
        maxInputLengths,
        updateTreeMetaData,
        type Language,
        type Subject,
        type TreeData,
        type TreeId,
        type TreeLinkedData
    } from '$lib/api-wrapper/trees';
    import Editor from '$lib/rich-text/Editor.svelte';
    import { tryParseISBN, tryParseTags } from '$lib/treeMetadata';
    import { m } from '$lib/paraglide/messages';
    import type { ManageSideBar, MenuSideBar } from './Sidebar.svelte';

    let {
        treeId,
        prevTreeData = $bindable(),
        treeLinkedData,
        sidebar = $bindable()
    }: {
        treeId: TreeId;
        prevTreeData: TreeData;
        treeLinkedData: TreeLinkedData;
        sidebar: ManageSideBar | MenuSideBar;
    } = $props();

    let visible = $state(false);
    let languages: Language[] | null = $state(null);
    let subjects: Record<string, Subject[]> | null = $state(null);

    onMount(async () => {
        languages = await getAllLanguages();
        subjects = await getAllSubjects();
    });

    // This is used to work around the fact that the Editor component does not support
    // oninput handlers like regular html elements.
    $effect(() => {
        if (treeData.assignmentText !== prevTreeData.assignmentText)
            inputsChanged.assignmentChanged = true;
    });

    let inputsChanged = $state({
        titleChanged: false,
        languageChanged: false,
        descriptionChanged: false,
        tagsChanged: false,
        subjectChanged: false,
        bookIsbnChanged: false,
        bookNameChanged: false,
        bookAuthorChanged: false,
        bookEditionChanged: false,
        bookChapterChanged: false,
        bookExerciseChanged: false,
        assignmentChanged: false,
        publishedChanged: false
    });

    let formChanged: boolean = $derived(
        inputsChanged.titleChanged ||
            inputsChanged.languageChanged ||
            inputsChanged.descriptionChanged ||
            inputsChanged.tagsChanged ||
            inputsChanged.subjectChanged ||
            inputsChanged.bookIsbnChanged ||
            inputsChanged.bookNameChanged ||
            inputsChanged.bookAuthorChanged ||
            inputsChanged.bookEditionChanged ||
            inputsChanged.bookChapterChanged ||
            inputsChanged.bookExerciseChanged ||
            inputsChanged.assignmentChanged ||
            inputsChanged.publishedChanged
    );

    let isbnInvalid: boolean = $state(false);
    let tagsInputError: string = $state('');
    let tagsInput: string = $state(prevTreeData.tags.join(', '));
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

    function resetInputStates() {
        inputsChanged.titleChanged = false;
        inputsChanged.languageChanged = false;
        inputsChanged.descriptionChanged = false;
        inputsChanged.tagsChanged = false;
        inputsChanged.subjectChanged = false;
        inputsChanged.bookIsbnChanged = false;
        inputsChanged.bookNameChanged = false;
        inputsChanged.bookAuthorChanged = false;
        inputsChanged.bookEditionChanged = false;
        inputsChanged.bookChapterChanged = false;
        inputsChanged.bookExerciseChanged = false;
        inputsChanged.assignmentChanged = false;
        inputsChanged.publishedChanged = false;
        isbnInvalid = false;
        tagsInputError = '';
    }

    async function saveData(e: SubmitEvent) {
        // This prevents the default event handler from clearing the page's search parameters.
        e.preventDefault();

        if (!formChanged) return;

        let newData: TreeData = { ...treeData };
        const isbn = tryParseISBN(treeData.bookIsbn);
        if (isbn === null) {
            isbnInvalid = true;
            return;
        }
        newData.bookIsbn = isbn === '' ? null : isbn;

        const result = tryParseTags(tagsInput);
        if (typeof result === 'string') {
            tagsInputError = result;
            return;
        }
        newData.tags = result;

        await updateTreeMetaData(treeId, newData);
        visible = false;
        prevTreeData = { ...newData, bookIsbn: newData.bookIsbn === null ? '' : newData.bookIsbn };
        treeData = { ...prevTreeData };
        resetInputStates();

        sidebar = {
            kind: 'Menu',
            type: 'IconsOnly'
        };
    }

    // Restores treeData to its original value.
    function cancelEdit(e: MouseEvent) {
        e.preventDefault();
        treeData = { ...prevTreeData };
        visible = false;
        resetInputStates();
        sidebar = {
            kind: 'Menu',
            type: 'IconsOnly'
        };
    }

    function onkeydown(event: KeyboardEvent) {
        if (!visible) return;

        // When ctrl+z is pressed, we only want that event to be handled in the relevant textbox,
        // do not let it propagate to the undo/redo buttons of the tree editor page.
        event.stopPropagation();

        // The form submit keyboard shortcut
        if (event.ctrlKey && event.key == 'Enter') {
            saveData(new SubmitEvent('submit'));
        }
    }
</script>

<!--
    We bind this event to the body instead of the window,
    so that this one fires before the onkeydown in the +page.svelte,
    so it can possibly stop the propagation.
-->
<svelte:body {onkeydown} />

<form
    data-testid="metadata-menu"
    onsubmit={saveData}
    class={[formChanged && 'completed', 'flex h-fit w-full flex-col overflow-hidden']}
>
    <div>
        <h2 class="page__heading">{m.editMetadata()}</h2>
    </div>
    <div>
        <label for="title">{m.title()} *</label>
        <div class="input__container">
            <input
                id="title"
                class={[!inputsChanged.titleChanged && 'inactive', 'input__text']}
                type="text"
                bind:value={treeData.title}
                placeholder={m.examplePlaceholderRequired({
                    value: m.exampleTitle({ name: treeLinkedData.ownerName ?? '' })
                })}
                oninput={() => {
                    inputsChanged.titleChanged = true;
                }}
                minlength={minInputLengths.title}
                maxlength={maxInputLengths.title}
                required
            />
        </div>
    </div>
    <div>
        <label for="tags">{m.tags()}</label>
        <div class="input__container">
            <input
                id="tags"
                class={[
                    !inputsChanged.tagsChanged && 'inactive',
                    tagsInputError !== '' && 'error',
                    'input__text'
                ]}
                type="text"
                bind:value={tagsInput}
                placeholder={m.examplePlaceholder({ value: m.exampleTags() })}
                oninput={() => {
                    inputsChanged.tagsChanged = true;
                }}
            />
        </div>
        {#if tagsInputError !== ''}
            <p class="text__error" data-testid="tags-input-error">
                {tagsInputError}
            </p>
        {/if}
    </div>
    <div>
        <label for="description">{m.description()}</label>
        <textarea
            id="description"
            class={[!inputsChanged.descriptionChanged && 'inactive', 'input__long_text', 'w-full']}
            bind:value={treeData.description}
            placeholder={m.examplePlaceholder({ value: m.exampleDescription() })}
            oninput={() => {
                inputsChanged.descriptionChanged = true;
            }}
            minlength={minInputLengths.description}
            maxlength={maxInputLengths.description}
        ></textarea>
    </div>
    <div class="*:max-h-text-area">
        <label for="assignment">{m.assignment()}</label>
        <Editor
            maxLength={maxInputLengths.assignmentText}
            bind:valueText={treeData.assignmentText}
        />
    </div>
    <div>
        <label for="language">{m.language()} *</label>
        <select
            id="language"
            class="input__select block"
            bind:value={treeData.languageCode}
            oninput={() => {
                inputsChanged.languageChanged = true;
            }}
        >
            {#each languages as value (value)}
                <option value={value.code} class={['input__select_option']}>
                    {value.name}
                </option>
            {/each}
        </select>
    </div>
    <div>
        <label for="subject">{m.subject()}</label>
        <select
            id="subject"
            class="input__select block"
            bind:value={treeData.subjectId}
            oninput={() => {
                inputsChanged.subjectChanged = true;
            }}
        >
            <option value={null} selected class={['input__select_option']}>{m.none()}</option>
            {#each subjects === null ? [] : subjects[treeData.languageCode] as value (value)}
                <option value={value.id} class={['input__select_option']}>
                    {value.data.name}
                </option>
            {/each}
        </select>
    </div>
    <div class="checkbox__container">
        <input
            id="published"
            class="input__checkbox"
            type="checkbox"
            bind:checked={treeData.published}
            oninput={() => {
                inputsChanged.publishedChanged = true;
            }}
        />
        <label for="published">{m.published()}</label>
    </div>
    <div>
        <label for="isbn">{m.isbn()}</label>
        <div class="input__container">
            <input
                id="isbn"
                class={[
                    !inputsChanged.bookIsbnChanged && 'inactive',
                    isbnInvalid && 'error',
                    'input__text'
                ]}
                type="text"
                bind:value={treeData.bookIsbn}
                placeholder={m.examplePlaceholder({ value: m.exampleBookISBN() })}
                oninput={() => {
                    inputsChanged.bookIsbnChanged = true;
                }}
                minlength={minInputLengths.bookIsbn}
                maxlength={maxInputLengths.bookIsbn}
            />
        </div>
        {#if isbnInvalid}
            <p class="text__error">{m.invalidISBN()}</p>
        {/if}
    </div>
    {#if treeData.bookIsbn ?? '' !== ''}
        <div>
            <label for="bookName">{m.bookName()}</label>
            <div class="input__container">
                <input
                    id="bookName"
                    class={[!inputsChanged.bookNameChanged && 'inactive', 'input__text']}
                    type="text"
                    disabled={treeData.bookIsbn === prevTreeData.bookIsbn || null}
                    bind:value={treeData.bookName}
                    placeholder={m.examplePlaceholder({
                        value: m.exampleBookName()
                    })}
                    oninput={() => {
                        inputsChanged.bookNameChanged = true;
                    }}
                    minlength={minInputLengths.bookName}
                    maxlength={maxInputLengths.bookName}
                />
            </div>
        </div>
        <div>
            <label for="author">{m.author()}</label>
            <div class="input__container">
                <input
                    id="author"
                    class={[!inputsChanged.bookAuthorChanged && 'inactive', 'input__text']}
                    type="text"
                    disabled={treeData.bookIsbn === prevTreeData.bookIsbn || null}
                    bind:value={treeData.bookAuthor}
                    placeholder={m.examplePlaceholder({ value: m.exampleAuthor() })}
                    oninput={() => {
                        inputsChanged.bookAuthorChanged = true;
                    }}
                    minlength={minInputLengths.bookAuthor}
                    maxlength={maxInputLengths.bookAuthor}
                />
            </div>
        </div>
        <div>
            <label for="edition">{m.edition()}</label>
            <div class="input__container">
                <input
                    id="edition"
                    class={[!inputsChanged.bookEditionChanged && 'inactive', 'input__text']}
                    type="text"
                    disabled={treeData.bookIsbn === prevTreeData.bookIsbn || null}
                    bind:value={treeData.bookEdition}
                    placeholder={m.examplePlaceholder({
                        value: m.exampleBookEdition()
                    })}
                    oninput={() => {
                        inputsChanged.bookEditionChanged = true;
                    }}
                    minlength={minInputLengths.bookEdition}
                    maxlength={maxInputLengths.bookEdition}
                />
            </div>
        </div>
        <div>
            <label for="chapter">{m.chapter()}</label>
            <div class="input__container">
                <input
                    id="chapter"
                    class={[!inputsChanged.bookChapterChanged && 'inactive', 'input__text']}
                    type="text"
                    bind:value={treeData.bookChapter}
                    placeholder={m.examplePlaceholder({
                        value: m.exampleBookChapter()
                    })}
                    oninput={() => {
                        inputsChanged.bookChapterChanged = true;
                    }}
                    minlength={minInputLengths.bookChapter}
                    maxlength={maxInputLengths.bookChapter}
                />
            </div>
        </div>
        <div>
            <label for="exercise">{m.exercise()}</label>
            <div class="input__container">
                <input
                    id="exercise"
                    class={[!inputsChanged.bookExerciseChanged && 'inactive', 'input__text']}
                    type="text"
                    bind:value={treeData.bookExercise}
                    placeholder={m.examplePlaceholder({
                        value: m.exampleExercise()
                    })}
                    oninput={() => {
                        inputsChanged.bookExerciseChanged = true;
                    }}
                    minlength={minInputLengths.bookExercise}
                    maxlength={maxInputLengths.bookExercise}
                />
            </div>
        </div>
    {/if}
    <div class="mb-0 flex flex-row gap-1 *:w-1/2">
        <button type="submit" data-testid="confirm-button">{m.confirmChanges()}</button>
        <button type="reset" onclick={cancelEdit}>{m.cancel()}</button>
    </div>
</form>

<style>
    form {
        .inactive:not(.input__long_text) {
            background-color: var(--color-neutral-100);
        }
    }
</style>
