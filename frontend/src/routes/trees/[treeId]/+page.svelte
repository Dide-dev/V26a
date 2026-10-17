<!--
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
-->

<script lang="ts">
    import SubtreeViewer from './SubtreeViewer.svelte';
    import { onMount } from 'svelte';
    import { getTreeWithCards, type TreeWithCards } from '$lib/api-wrapper/trees';
    import { m } from '$lib/paraglide/messages.js';
    import { goto } from '$app/navigation';
    import { appLink } from '$lib';
    import { branchColors, duplicateThisTree } from '$lib/treePage';
    import TreeInformation from './TreeInformation.svelte';
    import { getUserInfo } from '$lib/api-wrapper/sessions';
    import {
        EditOutline,
        ExpandOutline,
        FileCloneOutline,
        MinimizeOutline,
        PrinterOutline
    } from 'flowbite-svelte-icons';
    import { getUIState } from '$lib/context/ui.svelte';
    import { MediaQuery } from 'svelte/reactivity';

    let { params } = $props();

    let tree: TreeWithCards | null = $state(null);
    let userAllowedToEdit = $state(false);
    let userInfo = getUserInfo();

    /** Query whether the window is large or small. 'true' if large, and 'false' if not. */
    const windowIsLarge = new MediaQuery('min-width: 48rem');

    const uiState = getUIState();

    onMount(async () => {
        let id = Number(params.treeId);

        if (Number.isNaN(id)) {
            await goto(appLink('/trees'));
            return;
        }

        tree = await getTreeWithCards(id);

        const isOwner = tree.metadata.genData.ownerId === userInfo?.userId;
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
                <div class="flex gap-1">
                    <h1 class="page__title mt-0">{tree?.metadata.data.title}</h1>
                    {#if userAllowedToEdit}
                        <div class="flex flex-col justify-center">
                            <a
                                href={appLink(`/trees/${tree!.metadata.genData.id}/edit`)}
                                class="page__title mt-3"
                                data-testid="edit-button"
                                title={m.editTree()}
                            >
                                <EditOutline class="h-7 w-7 shrink-0" />
                            </a>
                        </div>
                    {/if}
                    {#if userInfo !== null}
                        <div class="flex flex-col justify-center">
                            <button
                                onclick={() => duplicateThisTree(tree!.metadata.genData.id)}
                                aria-label="duplicate"
                                class:cursor-pointer={true}
                                class="page__title mt-3"
                                title={m.duplicateTree()}
                            >
                                <FileCloneOutline class="h-7 w-7 shrink-0" />
                            </button>
                        </div>
                    {/if}
                    <div class="flex flex-col justify-center">
                        <button
                            onclick={() => {
                                uiState.toggle();
                            }}
                            aria-label={uiState.visible ? 'maximize' : 'minimize'}
                            class:cursor-pointer={true}
                            class="page__title mt-3"
                            title={uiState.visible ? m.maximizeScreen() : m.minimizeScreen()}
                        >
                            {#if uiState.visible}
                                <ExpandOutline class="h-6 w-6 shrink-0" />
                            {:else}
                                <MinimizeOutline class="h-7 w-7 shrink-0" />
                            {/if}
                        </button>
                    </div>
                    {#if windowIsLarge.current}
                        <div class="flex flex-col justify-center">
                            <a
                                href={appLink(`/trees/${tree!.metadata.genData.id}/print`)}
                                aria-label="print"
                                class:cursor-pointer={true}
                                class="page__title mt-3"
                                title={m.printMode()}
                            >
                                <PrinterOutline class="shrink-0 h-7 w-7" />
                            </a>
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
