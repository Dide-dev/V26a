<!--
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
-->

<script lang="ts">
    import SubtreeViewer from '../SubtreeViewer.svelte';
    import { onMount } from 'svelte';
    import { getTreeWithCards, type TreeWithCards, duplicateTree } from '$lib/api-wrapper/trees';
    import { page } from '$app/state';
    import { m } from '$lib/paraglide/messages.js';
    import { getUserInfo } from '$lib/api-wrapper/users';
    import { goto } from '$app/navigation';
    import { appLink } from '$lib';
    import editIcon from '$lib/assets/icons/edit.svg?raw';
    import AnimatedUnderline from '$lib/AnimatedUnderline.svelte';
    import { markdownToFinalHtml } from '$lib/rich-text/markdown';
    import iconDuplicate from '$lib/assets/icons/duplicate.svg?raw';
    import { branchColors } from '$lib/treePage';

    let userInfo = getUserInfo();

    let userAllowedToEdit = $state(false);
    let tree: TreeWithCards | null = $state(null);

    let showFullAssignment = $state(false);
    let assignmentIncludesImage = $state(false);

    /** The assignment text cut short depending on the state of `showFullAssignment`.
     *
     * @remarks If the assignment text includes an image, the text is never cut short.
     */
    let assignmentText = $derived(
        showFullAssignment ||
            assignmentIncludesImage ||
            tree!.metadata.data.assignmentText.length <= 128
            ? tree!.metadata.data.assignmentText
            : tree!.metadata.data.assignmentText.slice(0, 128) + '...'
    );

    onMount(async () => {
        const pageId = page.url.searchParams.get('id');

        if (!pageId) {
            await goto(appLink('/all-trees'));
            return;
        }

        let id = Number(pageId);

        if (Number.isNaN(id)) {
            await goto(appLink('/all-trees'));
            return;
        }

        tree = await getTreeWithCards(id);

        let htmlAssignment = markdownToFinalHtml(tree.metadata.data.assignmentText);
        assignmentIncludesImage = htmlAssignment.includes('<img');

        const isOwner = tree.metadata.genData.ownerId === userInfo?.sub;
        const isAdmin = userInfo?.accountType === 'admin';
        const isCoAuthor = tree.metadata.linkedData.coAuthors.includes(userInfo?.name ?? '');

        if (isOwner || isAdmin || isCoAuthor) {
            userAllowedToEdit = true;
        }
    });

    async function duplicateThisTree(id: number) {
        let newTreeId = await duplicateTree(id);
        await window.open(appLink(`/tree-page/editor?id=${newTreeId}`));
    }
</script>

<div class="flex h-full w-full flex-col items-start overflow-hidden">
    {#if tree === null}
        <p class="text-green-800">{m.loadingTree()}</p>
    {:else}
        <div class="ml-1.5 flex w-[calc(100%-1.5*var(--spacing))] flex-col">
            <div class="flex flex-col md:w-2/3">
                <div class="flex">
                    <h1 class="page__title mt-0">{tree.metadata.data.title}</h1>
                    {#if userAllowedToEdit}
                        <div class="flex flex-col justify-center">
                            <a
                                href={appLink(`/tree-page/editor?id=${tree!.metadata.genData.id}`)}
                                class="page__title mt-3 ml-1"
                                data-testid="edit-button"
                            >
                                <!-- eslint-disable-next-line svelte/no-at-html-tags -->
                                {@html editIcon}
                            </a>
                        </div>
                    {/if}
                    {#if userInfo !== null}
                        <div class="flex flex-col justify-center">
                            <button
                                onclick={() => duplicateThisTree(tree!.metadata.genData.id)}
                                aria-label="duplicate"
                                class:cursor-pointer={true}
                                class="page__title mt-3 ml-1"
                            >
                                <!-- eslint-disable-next-line svelte/no-at-html-tags -->
                                {@html iconDuplicate}
                            </button>
                        </div>
                    {/if}
                </div>
                <span class="-mt-2 italic *:*:inline">
                    {m.madeBy()}:
                    <AnimatedUnderline --weight-underline="calc(var(--spacing) / 4)">
                        <a
                            href={appLink(`/all-trees?owner=${tree.metadata.genData.ownerId}`)}
                            class="text-black">{tree.metadata.linkedData.ownerName}</a
                        >
                    </AnimatedUnderline>
                </span>
                <p class="font-semibold wrap-anywhere text-neutral-600">
                    {tree.metadata.data.description}
                </p>
                <div class="mt-1 flex flex-row">
                    {#each tree.metadata.data.tags as tag (tag)}
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
                            {#if tree.metadata.data.assignmentText.length > 128 && !assignmentIncludesImage}
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
                    {#if tree.metadata.data.bookIsbn !== null}
                        <a
                            href={appLink(`/all-trees?isbn=${tree.metadata.data.bookIsbn}`)}
                            class="mt-text-minor flex w-fit self-end *:*:flex-col"
                        >
                            <AnimatedUnderline --weight-underline="calc(var(--spacing) / 2)">
                                <h2 class="w-fit self-end text-lg font-semibold text-neutral-800">
                                    {tree.metadata.data.bookName}
                                </h2>
                            </AnimatedUnderline>
                        </a>
                        {#if tree.metadata.data.bookAuthor !== ''}
                            <p class="italic">
                                {m.writtenBy({ name: tree.metadata.data.bookAuthor })}
                            </p>
                        {/if}
                        <p>
                            {m.edition()}
                            {tree.metadata.data.bookEdition !== ''
                                ? tree.metadata.data.bookEdition
                                : '?'},
                            {m.chapter()}
                            {tree.metadata.data.bookChapter !== ''
                                ? tree.metadata.data.bookChapter
                                : '?'},
                            {m.exercise()}
                            {tree.metadata.data.bookExercise !== ''
                                ? tree.metadata.data.bookExercise
                                : '?'}
                        </p>
                    {/if}
                </div>
            </div>
        </div>

        <div class="m-1 flex w-full grow flex-col overflow-x-auto pt-6">
            {#each tree.structure.rootNodes as root, i (root)}
                <SubtreeViewer
                    node={root}
                    cards={tree.cards}
                    color={branchColors[i % branchColors.length]}
                    distanceFromRevealed={0}
                    isRevealed={true}
                />
            {/each}
        </div>
    {/if}
</div>

{#snippet tagBlob(tag: string)}
    <a
        href={appLink(`/all-trees?tags=${tag}`)}
        class={[
            'mr-1.5 rounded-4xl border border-neutral-400 bg-neutral-200 p-[0.25rem_0.5rem] italic',
            'transition-all duration-200',
            'hover:bg-neutral-400 hover:p-[0.25rem_1rem] hover:text-white'
        ]}
    >
        {tag}
    </a>
{/snippet}
