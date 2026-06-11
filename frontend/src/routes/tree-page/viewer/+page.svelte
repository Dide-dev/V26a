<!--
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
-->

<script lang="ts">
    import SubtreeViewer from '../SubtreeViewer.svelte';
    import { onMount } from 'svelte';
    import { duplicateTree, getTreeWithCards, type TreeWithCards } from '$lib/api-wrapper/trees';
    import { page } from '$app/state';
    import { m } from '$lib/paraglide/messages.js';
    import { goto } from '$app/navigation';
    import { appLink } from '$lib';
    import { branchColors } from '$lib/treePage';
    import TreeInformation from '../TreeInformation.svelte';
    import { getUserInfo } from '$lib/api-wrapper/users';
    import editIcon from '$lib/assets/icons/edit.svg?raw';
    import iconDuplicate from '$lib/assets/icons/duplicate.svg?raw';

    let tree: TreeWithCards | null = $state(null);
    let userAllowedToEdit = $state(false);
    let userInfo = getUserInfo();

    async function duplicateThisTree(id: number) {
        let newTreeId = await duplicateTree(id);
        await window.open(appLink(`/tree-page/editor?id=${newTreeId}`));
    }

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

        const isOwner = tree.metadata.genData.ownerId === userInfo?.sub;
        const isAdmin = userInfo?.accountType === 'admin';
        const isCoAuthor = tree.metadata.linkedData.coAuthors.includes(userInfo?.name ?? '');

        if (isOwner || isAdmin || isCoAuthor) {
            userAllowedToEdit = true;
        }
    });
</script>

<div class="flex h-full w-full flex-col items-start overflow-hidden">
    {#if tree === null}
        <p class="text-green-800">{m.loadingTree()}</p>
    {:else}
        <TreeInformation tree={tree.metadata}>
            {#snippet titleSnippet()}
                <div class="flex">
                    <h1 class="page__title mt-0">{tree?.metadata.data.title}</h1>
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
            {/snippet}
        </TreeInformation>

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
