<!--
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
-->

<script lang="ts">
    import iconUndo from '$lib/assets/iconUndo.svg?raw';
    import iconRedo from '$lib/assets/iconRedo.svg?raw';
    import iconExternalLink from '$lib/assets/iconExternalLink.svg?raw';
    import { page } from '$app/state';
    import { onMount } from 'svelte';
    import { m } from '$lib/paraglide/messages.js';
    import { getTreeWithCards, type TreeData } from '$lib/backend/trees';
    import { goto } from '$app/navigation';
    import { appLink } from '$lib';
    import NodeViewer, { type DragState } from '../NodeViewer.svelte';
    import {
        type TreeHistory,
        undo,
        redo,
        initializeHistory,
        fillStateStructure,
        canUndo,
        canRedo
    } from '../history.svelte';
    import EditMetadata from './EditMetadata.svelte';

    let dragState: DragState = $state(null);
    let scrollArea: Element | null = $state(null);
    // We scroll the page for the user if they are dragging and the mouse is near the edge
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

    let loading = $state(true);

    let id: number | null = null;
    let tree: [TreeData, TreeHistory] | null = $state(null);

    onMount(async () => {
        const pageId = page.url.searchParams.get('id');

        if (!pageId) {
            // TODO: redirect to tree list
            await goto(appLink('/'));
            return;
        }

        id = Number(pageId);

        if (Number.isNaN(id)) {
            // TODO: redirect to tree list
            await goto(appLink('/'));
            return;
        }

        const fetchedTree = await getTreeWithCards(id);
        tree = [
            fetchedTree.data,
            initializeHistory(
                fetchedTree.genData.id,
                fillStateStructure(fetchedTree.structure, true),
                fetchedTree.cards
            )
        ];

        loading = false;
    });

    function undoButtonClick() {
        undo(tree![1]);
    }
    function redoButtonClick() {
        redo(tree![1]);
    }
    // The undo & redo keyboard shortcuts
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

{#if loading}
    <p class="text-green-800">{m.loadingTree()}</p>
{:else if tree}
    <h1 class="m-auto ml-1.5 text-3xl font-bold">{tree[0].title}</h1>
    <p class="m-auto ml-1.5 pt-1 pb-1 text-sm font-bold text-gray-400">
        {tree[0].description}
    </p>

    <div class="m-2 flex gap-2">
        <EditMetadata treeId={tree[1].treeId} bind:prevTreeData={tree[0]} />
        <button
            class={[
                'rounded-sm bg-emerald-500 p-1',
                canUndo(tree[1]) && 'cursor-pointer stroke-black p-1',
                !canUndo(tree[1]) && 'stroke-gray-500'
            ]}
            onclick={undoButtonClick}
            title={m.undo()}
        >
            <!-- eslint-disable-next-line svelte/no-at-html-tags -->
            {@html iconUndo}
        </button>
        <button
            class={[
                'rounded-sm bg-emerald-500 p-1',
                canRedo(tree[1]) && 'cursor-pointer stroke-black p-1',
                !canRedo(tree[1]) && 'stroke-gray-500'
            ]}
            onclick={redoButtonClick}
            title={m.redo()}
        >
            <!-- eslint-disable-next-line svelte/no-at-html-tags -->
            {@html iconRedo}
        </button>
        <a
            class="rounded-sm bg-emerald-500 p-1 *:inline"
            href={appLink(`/tree-page/viewer?id=${tree[1].treeId}`)}
            target="_blank"
        >
            {m.viewTree()}
            <!-- eslint-disable-next-line svelte/no-at-html-tags -->
            {@html iconExternalLink}
        </a>
    </div>

    <!--
        The padding on the bottom is to make the scrollbar not intersect with some buttons.
    -->
    <div bind:this={scrollArea} class="m-1 flex grow flex-col overflow-auto pb-7">
        {#each tree[1].currentStructure.rootNodes as root, index (root)}
            <NodeViewer
                bind:history={tree[1]}
                location={[index]}
                bind:dragState
                editingPage={true}
            />
        {/each}
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

<!--
    Here we add a few css classes to the body,
    to make sure that the element above here with the `overflow-auto` class can be
    growing in height until the scroll bar is at the bottom of the screen.
-->
<style>
    :global(body) {
        min-height: 100vh;
        display: flex;
        flex-direction: column;
    }
</style>
