<!--
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
-->

<script lang="ts">
    import { goto } from '$app/navigation';
    import { appLink } from '$lib';
    import { m } from '$lib/paraglide/messages.js';
    import { addTree, type TreeData } from '$lib/backend/trees';
    import { onMount } from 'svelte';
    import { getUserInfo, type UserInfo } from '$lib/backend/users';
    import { redirectLogin } from '$lib/backend';
    import { page } from '$app/state';
    import type { Option } from '$lib/backend/types';

    let userInfo: UserInfo | null = $state(null);

    onMount(async () => {
        userInfo = getUserInfo();
        if (userInfo === null) {
            await redirectLogin(page.url);
            return;
        }
    });

    let treeData: TreeData = $state({
        title: '',
        description: '',
        bookName: '',
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
        bookName: 0,
        bookEdition: 0,
        bookIsbn: 10,
        bookChapter: 0,
        bookExercise: 0,
        assignmentText: 0
    };

    const maxInputLengths = {
        title: 128,
        description: 256,
        bookName: 192,
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
            return 11 - (s % 11) === parseInt(lastChar);
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

    async function handleSubmit(event: SubmitEvent) {
        event.preventDefault();
        isbnInvalid = false;

        const isbn = tryParseISBN();
        if (isbn === null) {
            isbnInvalid = true;
            return;
        }

        treeData.bookIsbn = isbn;
        const treeId = await addTree(treeData);
        await goto(appLink(`/tree-page/editor?id=${treeId}`));
    }
</script>

<div>
    <h1 class="page__title">{m.createTree()}</h1>
    <form class={[formCompleted && 'completed']} onsubmit={handleSubmit}>
        <div>
            <label for="title">
                {m.title()} *
            </label>
            <!-- TODO: replace `userInfo?.sub` with username -->
            <input
                id="title"
                class="input__text"
                bind:value={treeData.title}
                name="title"
                type="text"
                data-testid="title"
                placeholder={m.examplePlaceholderRequired({
                    value: m.exampleTitle({ name: userInfo?.sub ?? '' })
                })}
                minlength={minInputLengths.title}
                maxlength={maxInputLengths.title}
                required
            />
        </div>
        <div>
            <label for="description">
                {m.description()}
            </label>
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
            <label for="bookName">
                {m.bookname()}
            </label>
            <input
                id="bookName"
                class="input__text"
                bind:value={treeData.bookName}
                name="bookname"
                type="text"
                data-testid="bookname"
                placeholder={m.examplePlaceholder({ value: m.exampleBookName() })}
                minlength={minInputLengths.bookName}
                maxlength={maxInputLengths.bookName}
            />
        </div>
        <div>
            <label for="bookEdition">
                {m.bookedition()}
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
        <div>
            <label for="bookIsbn">
                {m.bookIsbn()}
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
        <div>
            <label for="bookChapter">
                {m.bookChapter()}
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
        <div>
            <label for="exercise">
                {m.bookExercise()}
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
        <div>
            <label for="assignmentText">
                {m.assignmentText()}
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
