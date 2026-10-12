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
    import { branchColors } from '$lib/treePage';
    import TreeInformation from '../TreeInformation.svelte';
    import { ArrowLeftOutline } from 'flowbite-svelte-icons';
    import { getUIState } from '$lib/context/ui.svelte';

    let { params } = $props();

    let tree: TreeWithCards | null = $state(null);

    const uiState = getUIState();

    onMount(async () => {
        let id = Number(params.treeId);

        if (Number.isNaN(id)) {
            await goto(appLink('/trees'));
            return;
        }

        tree = await getTreeWithCards(id);

        uiState.maximize();
    });
</script>

<div class="flex h-full w-full flex-col items-start overflow-hidden">
    {#if tree === null}
        <p class="text-green-800">{m.loadingTree()}</p>
    {:else}
        <TreeInformation tree={tree.metadata}>
            {#snippet titleSnippet()}
                <div class="flex gap-1">
                    <div class="flex flex-col justify-center">
                        <a
                            onclick={() => {
                                uiState.minimize();
                            }}
                            href={appLink(`/trees/${tree!.metadata.genData.id}`)}
                            aria-label="return"
                            class:cursor-pointer={true}
                            class="page__title mt-3"
                            title={m.return()}
                        >
                            <ArrowLeftOutline class="shrink-0 h-7 w-7" />
                        </a>
                    </div>
                    <h1 class="page__title mt-0">{tree?.metadata.data.title}</h1>
                </div>
            {/snippet}
        </TreeInformation>

        <div class="m-1 flex w-full grow flex-col overflow-x-auto pt-6">
            {#each tree.structure.rootNodes as root, i (root)}
                <SubtreeViewer
                    node={root}
                    cards={tree.cards}
                    color={branchColors[i % branchColors.length]}
                />
            {/each}
        </div>
    {/if}
</div>

<svelte:window
    onkeydown={(event: KeyboardEvent) => {
        if (event.key === 'Escape' || event.key === 'f') {
            goto(appLink(`/trees/${tree!.metadata.genData.id}`));
        }
    }}
/>
