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

    let error: null | Error = $state(null);

    async function handleSubmit(event: SubmitEvent) {
        event.preventDefault();
        try {
            const treeId = await addTree(treeData);
            console.log('Tree created with ID:', treeId);
            goto(appLink(`/editor?id=${treeId}`));
        } catch (e) {
            error = e as Error;
            console.error('Failed to create tree:', e);
        }
    }
</script>

<div class="flex w-full justify-center">
    <form class="flex w-md flex-col gap-2" onsubmit={handleSubmit}>
        <label>
            {m.title()}
            <input
                class="rounded-md border-2"
                bind:value={treeData.title}
                name="title"
                type="title"
                data-testid="title"
                maxlength="256"
                required
            />
        </label>
        <label>
            <p>{m.description()}</p>
            <textarea
                class="size-64 rounded-md border-2"
                bind:value={treeData.description}
                name="description"
                data-testid="description"
                maxlength="128"
                placeholder={m.optional()}
            ></textarea>
        </label>
        <label>
            {m.bookname()}
            <input
                class="rounded-md border-2"
                bind:value={treeData.bookName}
                name="bookname"
                type="bookname"
                data-testid="bookname"
                placeholder={m.optional()}
                maxlength="128"
            />
        </label>
        <label>
            {m.bookedition()}
            <input
                class="rounded-md border-2"
                bind:value={treeData.bookEdition}
                name="bookedition"
                type="bookedition"
                data-testid="bookedition"
                placeholder={m.optional()}
                maxlength="128"
            />
        </label>
        <label>
            {m.bookIsbn()}
            <input
                class="rounded-md border-2"
                bind:value={treeData.bookIsbn}
                name="bookIsbn"
                type="bookIsbn"
                data-testid="bookIsbn"
                placeholder={m.optional()}
                maxlength="128"
            />
        </label>
        <label>
            {m.bookChapter()}
            <input
                class="rounded-md border-2"
                bind:value={treeData.bookChapter}
                name="chapter"
                type="chapter"
                data-testid="chapter"
                maxlength="128"
                placeholder={m.optional()}
            />
        </label>
        <label>
            {m.bookExercise()}
            <input
                class="rounded-md border-2"
                bind:value={treeData.bookExercise}
                name="exercise"
                type="exercise"
                data-testid="exercise"
                maxlength="128"
                placeholder={m.optional()}
            />
        </label>
        <label>
            {m.assignmentText()}
            <input
                class="rounded-md border-2"
                bind:value={treeData.assignmentText}
                name="assignment"
                type="assignment"
                data-testid="assignment"
                maxlength="128"
                placeholder={m.optional()}
            />
        </label>
        <button class="rounded-sm bg-emerald-500 p-1">{m.createTree()}</button>
        <p class="text-red-600">{error?.message}</p>
    </form>
</div>
