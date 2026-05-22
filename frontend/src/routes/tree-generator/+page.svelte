<!--
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
-->

<script lang="ts">
    import { goto } from '$app/navigation';
    import { appLink } from '$lib';
    import { m } from '$lib/paraglide/messages.js';
    import {
        addTree,
        getAllLanguages,
        getAllSubjects,
        type Language,
        type Subject,
        type TreeData
    } from '$lib/backend/trees';
    import { onMount } from 'svelte';
    import {
        getOwnUserData,
        getUserInfo,
        type UserGetData,
        type UserInfo
    } from '$lib/backend/users';
    import { redirectLogin } from '$lib/backend';
    import { page } from '$app/state';
    import type { Option } from '$lib/backend/types';

    let userInfo: UserInfo | null = $state(null);
    let userData: UserGetData | null = $state(null);
    let languages: Language[] | null = $state(null);
    let subjects: Record<string, Subject[]> | null = $state(null);

    onMount(async () => {
        userInfo = getUserInfo();
        if (userInfo === null) {
            await redirectLogin(page.url);
            return;
        }
        userData = await getOwnUserData();
        treeData.languageCode = userData.language;
        languages = await getAllLanguages();
        subjects = await getAllSubjects();
    });

    let tagsInput: string = $state('');
    let tagsInputError: string | null = $state(null);
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

    const minInputLengths = {
        title: 4,
        description: 0,
        tag: 2,
        bookName: 0,
        bookAuthor: 0,
        bookEdition: 0,
        bookIsbn: 10,
        bookChapter: 0,
        bookExercise: 0,
        assignmentText: 0
    };

    const maxInputLengths = {
        title: 128,
        description: 256,
        tag: 32,
        bookName: 192,
        bookAuthor: 192,
        bookEdition: 32,
        bookIsbn: 17,
        bookChapter: 32,
        bookExercise: 32,
        assignmentText: 5000
    };

    /* Returns either the normalized value of the ISBN if it is valid,
     * or `null` if it is not. */
    function tryParseISBN(): Option<string> {
        if (!treeData.bookIsbn) {
            return '';
        }

        const isbn = treeData.bookIsbn.trim().replaceAll('-', '');

        if (isbn.length === 10 && checkISBN10(isbn)) {
            return isbn;
        } else if (isbn.length === 13 && checkISBN13(isbn)) {
            return isbn;
        } else {
            return null;
        }
    }

    function checkISBN10(str: string): boolean {
        let s = 0,
            t = 0;

        for (let i = 0; i < 9; i++) {
            let d = parseInt(str[i]);

            if (isNaN(d)) {
                return false;
            }

            t += d;
            s += t;
        }

        s += t;

        const lastChar = str[9];
        if (lastChar === 'X') {
            // X is used to indicate a checksum digit value of '10'
            return 11 - (s % 11) === 10;
        } else {
            return (11 - (s % 11)) % 11 === parseInt(lastChar);
        }
    }

    function checkISBN13(str: string): boolean {
        let w = 1,
            s = 0;

        for (let c of str) {
            const d = parseInt(c);

            if (isNaN(d)) {
                return false;
            }

            s += w * d;
            w = (w + 2) % 4;
        }

        return s % 10 === 0;
    }

    function tryParseTags(): string[] | null {
        if (tagsInput.trim() === '') {
            return [];
        }
        const list = tagsInput.split(',').map((t) => t.trim());
        if (list.some((t) => t.length < minInputLengths.tag || t.length > maxInputLengths.tag)) {
            tagsInputError = m.tagLengthRequirements();
            return null;
        }
        if (list.some((t) => /\s/.test(t))) {
            tagsInputError = m.noWhitespaceInTags();
            return null;
        }
        return list;
    }

    async function handleSubmit(event: SubmitEvent) {
        event.preventDefault();
        isbnInvalid = false;

        const isbn = tryParseISBN();
        if (isbn === null) {
            isbnInvalid = true;
            return;
        }
        treeData.bookIsbn = isbn === '' ? null : isbn;

        const tags = tryParseTags();
        if (tags === null) {
            return;
        }
        treeData.tags = tags;

        const treeId = await addTree(treeData);
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
                    value: m.exampleTitle({ name: userData?.name ?? '' })
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
                placeholder={m.examplePlaceholder({ value: m.exampleDescription() })}
                minlength={minInputLengths.description}
                maxlength={maxInputLengths.description}
            ></textarea>
        </div>
        <div>
            <label for="tags">{m.tags()}</label>
            <input
                id="tags"
                class="input__text"
                bind:value={tagsInput}
                name="tags"
                type="text"
                data-testid="tags"
                placeholder={m.examplePlaceholder({ value: m.exampleTags() })}
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
                        {value.name}
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
                placeholder={m.examplePlaceholder({ value: m.exampleBookISBN() })}
                minlength={minInputLengths.bookIsbn}
                maxlength={maxInputLengths.bookIsbn}
            />
            {#if isbnInvalid}
                <p class="text__error" data-testid="error">{m.invalidISBN()}</p>
            {/if}
        </div>
        <div class={[treeData.bookIsbn === '' && 'hidden']}>
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
        <div class={[treeData.bookIsbn === '' && 'hidden']}>
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
                placeholder={m.examplePlaceholder({ value: m.exampleName() })}
                minlength={minInputLengths.bookAuthor}
                maxlength={maxInputLengths.bookAuthor}
            />
        </div>
        <div class={[treeData.bookIsbn === '' && 'hidden']}>
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
                placeholder={m.examplePlaceholder({ value: m.exampleBookEdition() })}
                minlength={minInputLengths.bookEdition}
                maxlength={maxInputLengths.bookEdition}
            />
        </div>
        <div class={[treeData.bookIsbn === '' && 'hidden']}>
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
                placeholder={m.examplePlaceholder({ value: m.exampleBookChapter() })}
                minlength={minInputLengths.bookChapter}
                maxlength={maxInputLengths.bookChapter}
            />
        </div>
        <div class={[treeData.bookIsbn === '' && 'hidden']}>
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
                placeholder={m.examplePlaceholder({ value: m.exampleExercise() })}
                minlength={minInputLengths.bookExercise}
                maxlength={maxInputLengths.bookExercise}
            />
        </div>
        <h2 class="page__heading">{m.assignment()}</h2>
        <div>
            <label for="assignmentText">
                {m.description()}
            </label>
            <textarea
                id="assignmentText"
                class="input__long_text min-h-25"
                bind:value={treeData.assignmentText}
                name="assignment"
                data-testid="assignment"
                placeholder={m.examplePlaceholder({ value: m.exampleAssignment() })}
                minlength={minInputLengths.assignmentText}
                maxlength={maxInputLengths.assignmentText}
            ></textarea>
        </div>
        <button>{m.createTree()}</button>
    </form>
</div>
