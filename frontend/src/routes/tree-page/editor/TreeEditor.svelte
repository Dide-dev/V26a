<!--
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
-->

<!--component
    This is the top level component, after the Lock.svelte and +page.svelte.
 -->

<script lang="ts">
    import { onMount } from 'svelte';
    import { m } from '$lib/paraglide/messages.js';
    import { getTreeWithCards, type TreeId, type TreeMetadata } from '$lib/api-wrapper/trees';
    import { appLink } from '$lib';
    import SubtreeEditor, { type DragState } from './SubtreeEditor.svelte';
    import {
        type TreeHistory,
        undo,
        redo,
        initializeHistory,
        canUndo,
        canRedo
    } from './history.svelte';
    import type { SideBar } from './Sidebar.svelte';
    import SidebarComponent from './Sidebar.svelte';
    import RootNodeAddButton from './RootNodeAddButton.svelte';
    import { branchColors, duplicateThisTree } from '$lib/treePage';
    import { getUserInfo } from '$lib/api-wrapper/sessions';
    import TreeInformation from '../TreeInformation.svelte';
    import {
        ArrowUpRightFromSquareOutline,
        FileCloneOutline,
        RedoOutline,
        UndoOutline
    } from 'flowbite-svelte-icons';
    import { Tween } from 'svelte/motion';
    import { cubicOut } from 'svelte/easing';

    let { id }: { id: TreeId } = $props();

    let userInfo = $state(getUserInfo());
    let hasOwnerPermissions = $state(false);

    let sideBarFraction = new Tween(300, {
        duration: 400,
        easing: cubicOut
    });

    let dragState: DragState = $state(null);
    let scrollArea: Element | null = $state(null);
    // We scroll the page for the user if they are dragging some cards and the mouse is near the edge.
    $effect(() => {
        if (dragState?.type !== 'dragging') return;

        const scrollAreaSize = 70;
        const scrollFps = 30;
        const scrollSpeed = 900;

        let intervalId = setInterval(() => {
            if (dragState?.type !== 'dragging') return;

            let scrollX = 0;
            let scrollY = 0;

            let scrollAmount = scrollSpeed / scrollFps;

            // In of the 4 directions calculate how far into the scrollArea we are
            let x1 = scrollAreaSize - dragState.mouseX;
            let x2 = scrollAreaSize + dragState.mouseX - window.innerWidth;
            let y1 = scrollAreaSize - dragState.mouseY;
            let y2 = scrollAreaSize + dragState.mouseY - window.innerHeight;
            if (x1 > 0) {
                scrollX = -scrollAmount;
            } else if (x2 > 0) {
                scrollX = scrollAmount;
            }
            if (y1 > 0) {
                scrollY = -scrollAmount;
            } else if (y2 > 0) {
                scrollY = scrollAmount;
            }

            if (scrollX !== 0) {
                scrollArea?.scrollBy(scrollX, 0);
            }
            if (scrollY !== 0) {
                window.scrollBy(0, scrollY);
            }
        }, 1000 / scrollFps);

        return () => clearInterval(intervalId);
    });

    $effect(() => {
        if (tree?.sidebar !== null) sideBarFraction.target = 600;
        else sideBarFraction.target = 300;
    });

    let tree: {
        metadata: TreeMetadata;
        history: TreeHistory;
        sidebar: null | SideBar;
        sidebarElement: null | SidebarComponent;
    } | null = $state(null);

    onMount(async () => {
        const fetchedTree = await getTreeWithCards(id);
        tree = {
            metadata: fetchedTree.metadata,
            history: initializeHistory(
                fetchedTree.metadata.genData.id,
                fetchedTree.structure,
                fetchedTree.cards
            ),
            sidebar: null,
            sidebarElement: null
        };

        const isOwner = fetchedTree.metadata.genData.ownerId === userInfo?.sub;
        const isAdmin = userInfo?.accountType === 'admin';

        if (isOwner || isAdmin) {
            hasOwnerPermissions = true;
        }
    });

    function undoButtonClick() {
        tree!.sidebar = null;
        undo(tree!.history);
    }
    function redoButtonClick() {
        tree!.sidebar = null;
        redo(tree!.history);
    }
    // The undo & redo keyboard shortcuts.
    function onkeydown(event: KeyboardEvent) {
        if (event.ctrlKey && event.key == 'z') {
            undoButtonClick();
            event.stopPropagation();
        }
        if (event.ctrlKey && event.key == 'y') {
            redoButtonClick();
            event.stopPropagation();
        }
        if (event.ctrlKey && event.key == 'Z') {
            redoButtonClick();
            event.stopPropagation();
        }
    }
</script>

{#if tree === null}
    <p class="w-full text-center text-green-800">{m.loadingTree()}</p>
{:else}
    <div
        class="grid h-full w-full"
        style={`grid-template-columns: 1fr ${sideBarFraction.current}px;`}
    >
        <!-- svelte-ignore a11y_click_events_have_key_events -->
        <!-- svelte-ignore a11y_no_static_element_interactions -->
        <div
            class="overflow-x-auto"
            onclick={() => {
                tree!.sidebar = null;
            }}
        >
            <TreeInformation tree={tree.metadata}>
                {#snippet titleSnippet()}
                    <div class="flex gap-1">
                        <h1 class="page__title mt-0">{tree?.metadata.data.title}</h1>
                        <div class="flex flex-col justify-center">
                            <a
                                href={appLink(`/tree-page/viewer?id=${tree!.metadata.genData.id}`)}
                                aria-label="view"
                                class="page__title mt-3"
                                title={m.viewTree()}
                            >
                                <ArrowUpRightFromSquareOutline class="h-7 w-7 shrink-0" />
                            </a>
                        </div>
                        <div class="flex flex-col justify-center">
                            <button
                                onclick={() => duplicateThisTree(tree!.metadata.genData.id)}
                                aria-label="duplicate"
                                class="page__title mt-3 cursor-pointer"
                                title={m.duplicateTree()}
                            >
                                <FileCloneOutline class="h-7 w-7 shrink-0" />
                            </button>
                        </div>
                    </div>
                {/snippet}
            </TreeInformation>

            <div class="m-2 flex gap-2">
                <button
                    class={[
                        'ml-1 flex cursor-pointer items-center justify-center rounded-md border-2 p-1',
                        'border-gray-500 transition-colors duration-200 ease-in-out',
                        canUndo(tree.history) && 'cursor-pointer bg-white hover:bg-neutral-200',
                        !canUndo(tree.history) && 'bg-neutral-300'
                    ]}
                    onclick={undoButtonClick}
                    title={m.undo()}
                >
                    <UndoOutline
                        class={[!canUndo(tree.history) && 'text-neutral-700', 'h-6 w-6 shrink-0']}
                    />
                </button>
                <button
                    class={[
                        'ml-1 flex cursor-pointer items-center justify-center rounded-md border-2 p-1',
                        'border-gray-500 transition-colors duration-200 ease-in-out',
                        canRedo(tree.history) && 'cursor-pointer bg-white hover:bg-neutral-200',
                        !canRedo(tree.history) && 'bg-neutral-300'
                    ]}
                    onclick={redoButtonClick}
                    title={m.redo()}
                >
                    <RedoOutline
                        class={[!canRedo(tree.history) && 'text-neutral-700', 'h-6 w-6 shrink-0']}
                    />
                </button>
            </div>

            <!--
                The padding on the bottom is to make the scrollbar not intersect with some buttons.
            -->
            <div bind:this={scrollArea} class="m-1 flex overflow-x-auto pb-7">
                <div class="flex grow flex-col">
                    {#each tree.history.currentStructure.rootNodes as root, location (root)}
                        <SubtreeEditor
                            bind:history={tree.history}
                            location={[location]}
                            bind:dragState
                            bind:sidebar={tree.sidebar}
                            color={branchColors[location % branchColors.length]}
                        />
                    {/each}
                    <RootNodeAddButton history={tree.history} enabled={dragState === null} />
                </div>
            </div>
        </div>
        <SidebarComponent
            bind:this={tree.sidebarElement}
            bind:sidebar={tree.sidebar}
            history={tree.history}
            {hasOwnerPermissions}
            treeId={tree.history.treeId}
            bind:prevTreeData={tree.metadata.data}
            treeLinkedData={tree.metadata.linkedData}
        />
    </div>
{/if}

<!--
    We need to listen on the entire body to detect if drag & drop did not find target.
    Also here we update the position of the 'ghost' while dragging.
-->
<svelte:window
    onmouseup={() => (dragState = null)}
    onmouseenter={(event: MouseEvent) => {
        if (event.buttons === 0) {
            dragState = null;
        }
    }}
    onmousemove={(event: MouseEvent) => {
        if (dragState?.type !== 'dragging') return;

        dragState.mouseX = event.clientX;
        dragState.mouseY = event.clientY;
    }}
    {onkeydown}
/>
{#if dragState?.type === 'dragging'}
    <style>
        * {
            cursor: move;
        }
    </style>
{/if}

<style>
    :global(nav),
    :global(footer) {
        margin: 0;
    }
</style>
