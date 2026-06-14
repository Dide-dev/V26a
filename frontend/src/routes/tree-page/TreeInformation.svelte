<!--
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
-->

<!--
    @component
    Display a tree's information.

    ## Params
    - `tree: TreeMetadata` - The data of the tree.
    - `titleSnippet: Snippet` - Title snippet to display at the top.
-->
<script lang="ts">
    import { type TreeMetadata } from '$lib/api-wrapper/trees';
    import { appLink } from '$lib';
    import { m } from '$lib/paraglide/messages.js';
    import AnimatedUnderline from '$lib/AnimatedUnderline.svelte';
    import { onMount, type Snippet } from 'svelte';
    import { markdownToFinalHtml } from '$lib/rich-text/markdown';

    let {
        tree,
        titleSnippet
    }: {
        tree: TreeMetadata;
        titleSnippet: Snippet;
    } = $props();

    let showFullAssignment = $state(false);
    let assignmentIncludesImage = $state(false);

    /** The assignment text cut short depending on the state of `showFullAssignment`.
     *
     * @remarks If the assignment text includes an image, the text is never cut short.
     */
    let assignmentText = $derived(
        showFullAssignment || assignmentIncludesImage || tree.data.assignmentText.length <= 128
            ? tree.data.assignmentText
            : tree.data.assignmentText.slice(0, 128) + '...'
    );

    onMount(() => {
        let htmlAssignment = markdownToFinalHtml(tree.data.assignmentText);
        assignmentIncludesImage = htmlAssignment.includes('<img');
    });
</script>

<div class="ml-1.5 flex w-[calc(100%-1.5*var(--spacing))] flex-col">
    <div class="flex flex-col md:w-2/3">
        {@render titleSnippet()}
        <span class="-mt-2 italic *:*:inline">
            {m.madeBy()}:
            <AnimatedUnderline --weight-underline="calc(var(--spacing) / 4)">
                <a href={appLink(`/trees?owner=${tree.linkedData.ownerName}`)} class="text-black"
                    >{tree.linkedData.ownerName}</a
                >
            </AnimatedUnderline>
        </span>
        <p class="font-semibold wrap-anywhere text-neutral-600">
            {tree.data.description}
        </p>
        <div class="mt-1 flex flex-row">
            {#each tree.data.tags as tag (tag)}
                {@render tagBlob(tag)}
            {/each}
        </div>
    </div>
    <div class="mt-text-small flex flex-auto flex-row">
        <div class="flex w-2/3 flex-col">
            {#if assignmentText !== ''}
                <p class="mt-text-minor text-lg font-semibold wrap-anywhere">
                    {m.assignment()}
                </p>
                <p>
                    <!-- Trees from the prototype may contain images. -->
                    <span class="wrap-anywhere *:last:inline">
                        <!-- eslint-disable-next-line svelte/no-at-html-tags -->
                        {@html markdownToFinalHtml(assignmentText)}
                    </span>
                    {#if tree.data.assignmentText.length > 128 && !assignmentIncludesImage}
                        <button
                            onclick={() => {
                                showFullAssignment = !showFullAssignment;
                            }}
                            class="text__link"
                        >
                            {showFullAssignment ? m.showLess() : m.showMore()}
                        </button>
                    {/if}
                </p>
            {/if}
        </div>
        <div class="mr-4 flex w-1/3 flex-col text-right">
            {#if tree.data.bookIsbn ?? '' !== ''}
                <a
                    href={appLink(`/trees?book=${tree.data.bookIsbn}`)}
                    class="mt-text-minor flex w-fit self-end *:*:flex-col"
                >
                    <AnimatedUnderline --weight-underline="calc(var(--spacing) / 2)">
                        <h2 class="w-fit self-end text-lg font-semibold text-neutral-800">
                            {tree.data.bookName}
                        </h2>
                    </AnimatedUnderline>
                </a>
                {#if tree.data.bookAuthor !== ''}
                    <p class="italic">
                        {m.writtenBy({ name: tree.data.bookAuthor })}
                    </p>
                {/if}
                <p>
                    {m.edition()}
                    {tree.data.bookEdition !== '' ? tree.data.bookEdition : '?'},
                    {m.chapter()}
                    {tree.data.bookChapter !== '' ? tree.data.bookChapter : '?'},
                    {m.exercise()}
                    {tree.data.bookExercise !== '' ? tree.data.bookExercise : '?'}
                </p>
            {/if}
        </div>
    </div>
</div>

{#snippet tagBlob(tag: string)}
    <a
        href={appLink(`/trees?tag=${tag}`)}
        class={[
            'mr-1.5 rounded-4xl border border-neutral-400 bg-neutral-200 p-[0.25rem_0.5rem] italic',
            'transition-all duration-200',
            'hover:bg-neutral-400 hover:p-[0.25rem_1rem] hover:text-white'
        ]}
    >
        {tag}
    </a>
{/snippet}
