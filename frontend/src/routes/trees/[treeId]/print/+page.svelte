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
    import { Kbd } from 'flowbite-svelte';
    import {
        AdjustmentsHorizontalOutline,
        ArrowLeftOutline,
        InfoCircleOutline
    } from 'flowbite-svelte-icons';
    import { getUIState } from '$lib/context/ui.svelte';
    import { cubicOut } from 'svelte/easing';
    import { Tween } from 'svelte/motion';
    import { defaultPostItSizes, PostItSize } from './postItSizes';

    let { params } = $props();

    let tree: TreeWithCards | null = $state(null);

    type MenuOptions = {
        state: 'options' | 'info' | '';
        lastState: 'options' | 'info' | '';
    };

    let menu: MenuOptions = $state({
        state: '',
        lastState: ''
    });
    let menuFraction = new Tween(0, {
        duration: 200,
        easing: cubicOut
    });

    function openOptions() {
        if (menu.state === 'info') {
            menu.state = 'options';
            return;
        }

        menuFraction.target = menu.state === 'options' ? 0 : 1;
        menu.state = menu.state === 'options' ? '' : 'options';
        menu.lastState = 'options';
    }

    function openInfo() {
        if (menu.state === 'options') {
            menu.state = 'info';
            return;
        }

        menuFraction.target = menu.state === 'info' ? 0 : 1;
        menu.state = menu.state === 'info' ? '' : 'info';
        menu.lastState = 'info';
    }

    let postItSizeFreeRange = $state(false);
    let postItSizeStatus = $state(1);

    function getPostItSize() {
        if (!postItSizeFreeRange && postItSizeStatus >= defaultPostItSizes.length) {
            return new PostItSize({});
        }

        return postItSizeFreeRange
            ? new PostItSize({ width: postItSizeStatus, height: postItSizeStatus })
            : defaultPostItSizes[postItSizeStatus];
    }

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
                <div
                    class="title-grid w-full"
                    style="--options-row-height: {menuFraction.current}fr"
                >
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
                        <div class="flex flex-col justify-center">
                            <button
                                onclick={openOptions}
                                aria-label="return"
                                class:cursor-pointer={true}
                                class="page__title mt-3"
                                title={m.options()}
                            >
                                <AdjustmentsHorizontalOutline class="shrink-0 h-7 w-7" />
                            </button>
                        </div>
                        <div class="flex flex-col justify-center">
                            <button
                                onclick={openInfo}
                                aria-label="return"
                                class:cursor-pointer={true}
                                class="page__title mt-3"
                                title={m.information()}
                            >
                                <InfoCircleOutline class="shrink-0 h-7 w-7" />
                            </button>
                        </div>
                    </div>
                    <div class="overflow-y-hidden overflow-x-auto">
                        <form
                            class={[
                                menu.state === '' && menuFraction.current === 0 && 'hidden',
                                'flex flex-col pl-1.5 from-white to-neutral-50 bg-linear-180 mb-2 rounded-b-md border-neutral-200 border-2 border-t-0'
                            ]}
                        >
                            {#if menu.state === 'options' || (menu.state === '' && menu.lastState === 'options')}
                                <div class="checkbox__container">
                                    <input
                                        type="checkbox"
                                        id="free-range-checkbox"
                                        class="input__checkbox"
                                        bind:checked={postItSizeFreeRange}
                                        onchange={() => {
                                            console.log('dit werkt!');
                                            if (postItSizeFreeRange) {
                                                postItSizeStatus =
                                                    defaultPostItSizes[postItSizeStatus].width;
                                            } else {
                                                postItSizeStatus = 1;
                                            }
                                        }}
                                    />
                                    <label for="free-range-checkbox"
                                        >{m.postItPresetCheckbox()}</label
                                    >
                                </div>
                                <div class="input__container">
                                    <label for="card-width-slider">
                                        {m.postItSize()}: {getPostItSize()}
                                    </label>
                                    <input
                                        type="range"
                                        id="card-width-slider"
                                        min={postItSizeFreeRange ? 30 : 0}
                                        max={postItSizeFreeRange
                                            ? 150
                                            : defaultPostItSizes.length - 1}
                                        class="accent-tree-700 w-1/2"
                                        bind:value={postItSizeStatus}
                                    />
                                </div>
                            {:else if menu.state === 'info' || (menu.state === '' && menu.lastState === 'info')}
                                <p class="pb-1.5">
                                    {m.printScreenInfo1()}
                                    <Kbd>Ctrl</Kbd> + <Kbd>P</Kbd>
                                    {m.printScreenInfo2()}
                                </p>
                            {/if}
                        </form>
                    </div>
                </div>
            {/snippet}
        </TreeInformation>

        <div class="m-1 flex w-full grow flex-col overflow-x-auto pt-6">
            {#each tree.structure.rootNodes as root, i (root)}
                <SubtreeViewer
                    node={root}
                    cards={tree.cards}
                    color={branchColors[i % branchColors.length]}
                    cardSize={getPostItSize()}
                />
            {/each}
        </div>
    {/if}
</div>

<!-- Return to regular view upon exiting full-screen. -->
<svelte:window
    onkeydown={(event: KeyboardEvent) => {
        if (event.key === 'Escape' || event.key === 'f') {
            goto(appLink(`/trees/${tree!.metadata.genData.id}`));
        }
    }}
/>

<style>
    .title-grid {
        display: grid;
        grid-auto-flow: column;
        overflow: hidden;
        align-items: start;

        gap: var(--spacing-padding-small);
        grid-template-rows: 0.4fr var(--options-row-height);
    }
</style>
