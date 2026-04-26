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

    let formCompleted: boolean = $derived(treeData.title !== '');

    async function handleSubmit(event: SubmitEvent) {
        event.preventDefault();

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
                placeholder={m.examplePlaceholder() +
                    m.exampleTitle({ name: userInfo?.sub ?? '' }) +
                    m.requiredPlaceholder()}
                maxlength="128"
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
                placeholder={m.examplePlaceholder() + m.exampleDescription()}
                maxlength="256"
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
                placeholder={m.examplePlaceholder() + m.exampleBookName()}
                maxlength="192"
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
                type="number"
                data-testid="bookEdition"
                placeholder={m.examplePlaceholder() + m.exampleBookEdition()}
                maxlength="32"
            />
        </div>
        <div>
            <label for="bookIsbn">
                {m.bookIsbn()}
            </label>
            <input
                id="bookIsbn"
                class="input__text"
                bind:value={treeData.bookIsbn}
                name="bookIsbn"
                type="number"
                data-testid="bookIsbn"
                placeholder={m.examplePlaceholder() + m.exampleBookISBN()}
                maxlength="13"
            />
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
                placeholder={m.examplePlaceholder() + m.exampleBookChapter()}
                maxlength="32"
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
                placeholder={m.examplePlaceholder() + m.exampleExercise()}
                maxlength="32"
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
                placeholder={m.examplePlaceholder() + m.exampleAssignment()}
                maxlength="5000"
            ></textarea>
        </div>
        <button>{m.createTree()}</button>
    </form>
</div>
