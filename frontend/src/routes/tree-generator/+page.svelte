<!--
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
-->

<script lang="ts">
    import { goto } from '$app/navigation';
    import { appLink } from '$lib';
    import { m } from '$lib/paraglide/messages.js';
    import { tryParseTags, tryParseISBN } from '$lib/treeMetadata.js';
    import {
        addTree,
        getAllLanguages,
        getAllSubjects,
        minInputLengths,
        maxInputLengths,
        type Language,
        type Subject,
        type TreeData
    } from '$lib/api-wrapper/trees';
    import { onMount } from 'svelte';
    import { getOwnUserData, getUserInfo, type UserGetData } from '$lib/api-wrapper/users';
    import Editor from '$lib/rich-text/Editor.svelte';
    import { redirectLogin } from '$lib/api-wrapper';
    import { page } from '$app/state';

    let userData: UserGetData | null = $state(null);
    let languages: Language[] | null = $state(null);
    let subjects: Record<string, Subject[]> | null = $state(null);

    onMount(async () => {
        if (getUserInfo() === null) {
            await redirectLogin(page.url);
            return;
        }
        userData = await getOwnUserData();
        treeData.languageCode = userData.data.language;
        languages = await getAllLanguages();
        subjects = await getAllSubjects();
    });

    let tagsInput: string = $state('');
    let tagsInputError: string = $state('');
    let treeData: TreeData = $state({
        title: '',
        description: '',
        languageCode: 'nl',
        tags: [],
        subjectId: null,
        bookName: '',
        bookAuthor: '',
        bookEdition: '',
        bookIsbn: '',
        bookChapter: '',
        bookExercise: '',
        assignmentText: '',
        published: false
    });

    let isbnInvalid = $state(false);
    let formCompleted: boolean = $derived(treeData.title !== '');

    async function handleSubmit(event: SubmitEvent) {
        event.preventDefault();
        isbnInvalid = false;

        const newData: TreeData = { ...treeData };

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

        const treeId = await addTree(newData);
        await goto(appLink(`/tree-page/editor?id=${treeId}`));
    }
</script>

<div>
    <h1 class="page__title">{m.createTree()}</h1>
    <form class={[formCompleted && 'completed']} onsubmit={handleSubmit}>
        <h2 class="page__heading">{m.general()}</h2>
        <div>
            <label for="title">{m.title()} *</label>
            <input
                id="title"
                class="input__text"
                bind:value={treeData.title}
                name="title"
                type="text"
                data-testid="title"
                placeholder={m.examplePlaceholderRequired({
                    value: m.exampleTitle({ name: userData?.data.name ?? '' })
                })}
                minlength={minInputLengths.title}
                maxlength={maxInputLengths.title}
                required
            />
        </div>
        <div>
            <label for="description">{m.description()}</label>
            <textarea
                id="description"
                class="input__long_text min-h-20"
                bind:value={treeData.description}
                name="description"
                data-testid="description"
                placeholder={m.examplePlaceholderOptional({ value: m.exampleDescription() })}
                minlength={minInputLengths.description}
                maxlength={maxInputLengths.description}
            ></textarea>
        </div>
        <div>
            <label for="tags">{m.tags()}</label>
            <input
                id="tags"
                class={[tagsInputError !== '' && 'error', 'input__text']}
                bind:value={tagsInput}
                name="tags"
                type="text"
                data-testid="tags"
                placeholder={m.examplePlaceholderOptional({ value: m.exampleTags() })}
            />
            {#if tagsInputError !== ''}
                <p class="text__error" data-testid="tags-input-error">{tagsInputError}</p>
            {/if}
        </div>
        <div>
            <label for="language">{m.language()} *</label>
            <select id="language" class="input__select" bind:value={treeData.languageCode}>
                {#each languages as value (value)}
                    <option value={value.code} class={['input__select_option']}>
                        {value.name}
                    </option>
                {/each}
            </select>
        </div>
        <div>
            <label for="subject">{m.subject()}</label>
            <select id="subject" class="input__select" bind:value={treeData.subjectId}>
                <option value={null} selected class={['input__select_option']}>{m.none()}</option>
                {#each subjects === null ? [] : subjects[treeData.languageCode] as value (value)}
                    <option value={value.id} class={['input__select_option']}>
                        {value.data.name}
                    </option>
                {/each}
            </select>
        </div>
        <h2 class="page__heading">{m.book()}</h2>
        <div>
            <label for="bookIsbn">
                {m.isbn()}
            </label>
            <input
                id="bookIsbn"
                class={[isbnInvalid && 'error', 'input__text']}
                bind:value={treeData.bookIsbn}
                name="bookIsbn"
                type="text"
                data-testid="bookIsbn"
                placeholder={m.examplePlaceholderOptional({ value: m.exampleBookISBN() })}
                minlength={minInputLengths.bookIsbn}
                maxlength={maxInputLengths.bookIsbn}
            />
            {#if isbnInvalid}
                <p class="text__error" data-testid="error">{m.invalidISBN()}</p>
            {/if}
        </div>
        {#if treeData.bookIsbn !== ''}
            <div>
                <label for="bookName">
                    {m.title()}
                </label>
                <input
                    id="bookName"
                    class="input__text"
                    bind:value={treeData.bookName}
                    name="bookName"
                    type="text"
                    data-testid="bookname"
                    placeholder={m.examplePlaceholder({ value: m.exampleBookName() })}
                    minlength={minInputLengths.bookName}
                    maxlength={maxInputLengths.bookName}
                />
            </div>
            <div>
                <label for="bookAuthor">
                    {m.author()}
                </label>
                <input
                    id="bookAuthor"
                    class="input__text"
                    bind:value={treeData.bookAuthor}
                    name="bookAuthor"
                    type="text"
                    data-testid="bookAuthor"
                    placeholder={m.examplePlaceholder({ value: m.exampleAuthor() })}
                    minlength={minInputLengths.bookAuthor}
                    maxlength={maxInputLengths.bookAuthor}
                />
            </div>
            <div>
                <label for="bookEdition">
                    {m.edition()}
                </label>
                <input
                    id="bookEdition"
                    class="input__text"
                    bind:value={treeData.bookEdition}
                    name="bookEdition"
                    type="text"
                    data-testid="bookEdition"
                    placeholder={m.examplePlaceholderOptional({ value: m.exampleBookEdition() })}
                    minlength={minInputLengths.bookEdition}
                    maxlength={maxInputLengths.bookEdition}
                />
            </div>
            <div>
                <label for="bookChapter">
                    {m.chapter()}
                </label>
                <input
                    id="bookChapter"
                    class="input__text"
                    bind:value={treeData.bookChapter}
                    name="chapter"
                    type="text"
                    data-testid="chapter"
                    placeholder={m.examplePlaceholderOptional({ value: m.exampleBookChapter() })}
                    minlength={minInputLengths.bookChapter}
                    maxlength={maxInputLengths.bookChapter}
                />
            </div>
            <div>
                <label for="exercise">
                    {m.exercise()}
                </label>
                <input
                    id="exercise"
                    class="input__text"
                    bind:value={treeData.bookExercise}
                    name="exercise"
                    type="text"
                    data-testid="exercise"
                    placeholder={m.examplePlaceholderOptional({ value: m.exampleExercise() })}
                    minlength={minInputLengths.bookExercise}
                    maxlength={maxInputLengths.bookExercise}
                />
            </div>
        {/if}
        <h2 class="page__heading">{m.assignment()}</h2>
        <div>
            <label for="assignmentText">
                {m.description()}
            </label>
            <Editor
                maxLength={maxInputLengths.assignmentText}
                bind:valueText={treeData.assignmentText}
            />
        </div>
        <button type="submit">{m.createTree()}</button>
    </form>
</div>
