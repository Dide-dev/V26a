<!--
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
-->

<!--component
    This is a box on the editing page with 2 purposes:
    - Add card button
    - Destination for moving cards with drag & drop
 -->
<script lang="ts">
    import iconAdd from '$lib/assets/iconAdd.svg?raw';
    import { doAddCard, doMove, type TreeHistory } from './history.svelte';
    import type { DragState } from './NodeViewer.svelte';
    import type { SideBar } from './Sidebar.svelte';

    let {
        history,
        location,
        dragState = $bindable(),
        lineLayout,
        sidebar = $bindable(),
        numPopupsOpen = $bindable()
    }: {
        history: TreeHistory;
        location: number[];
        dragState: null | DragState;
        lineLayout: Layout;
        sidebar: null | SideBar;
        numPopupsOpen: number;
    } = $props();

    export type Layout = LayoutRight | LayoutUnder | LayoutOver;
    /** Used when this component is to the right of a node. */
    export type LayoutRight = {
        type: 'right';
    };
    /** Used when this component is below a node. */
    export type LayoutUnder = {
        type: 'under';
        lineHeight: number;

        // These correspond to private variables or constants in the NodeViewer.svelte.
        svgWidth: number;
        lineOffset: number;
    };
    /** Used when this component is above a node. */
    export type LayoutOver = {
        type: 'over';

        // These correspond to private variables or constants in the NodeViewer.svelte.
        svgWidth: number;
        lineOffset: number;
    };

    /**
     * How much the add button must be shifted over to the right,
     * so that it is not behind the TreeEditButtons
     */
    const layoutUnderOffsetRight = 130;

    /** This size must be known to draw the line, so it cannot be a tailwindcss height. */
    const buttonHeight = 24;

    let isHovered = $state(false);
    let highlight = $derived((dragState == null && isHovered) || dragState?.type === 'dragging');


    // Clicking for adding cards // TODO remove card when cancel button is pressed (or a new card is added when no new text is added)
    async function onclick() {
        if (dragState !== null) {
            return;
        }
        await doAddCard(history, location, {front: '', back: ''});
        // This does the same as the editClick() functionality in TreeEditButtons.svelte
        sidebar = {
            location: location,
            editingVersion: {front: '', back: ''}
        };
    }

    /*async function finishAddCard(newCard: CardGetData) {
        await doAddCard(history, location, newCard);
        showAddCardPopup = false;
    }*/

    // This mouse event is for drag & drop
    async function onmouseup(event: MouseEvent) {
        if (dragState?.type !== 'dragging') {
            return;
        }

        event.stopPropagation();

        await doMove(history, dragState.nodeToMove, location);

        dragState = null;
    }
</script>

{#snippet buttonArea()}
    <button
        class="flex w-2xs cursor-pointer items-center justify-center rounded-2xl border-2 border-dashed"
        style="height: {buttonHeight}px"
        class:bg-amber-200={isHovered && dragState?.type == 'dragging'}
        class:fill-gray-300={!highlight}
        class:fill-gray-800={highlight}
        class:border-gray-300={!highlight}
        class:border-gray-800={highlight}
        {onclick}
        {onmouseup}
        onmouseenter={() => (isHovered = true)}
        onmouseleave={() => (isHovered = false)}
    >
        {#if dragState === null}
            <!-- eslint-disable-next-line svelte/no-at-html-tags -->
            {@html iconAdd}
        {/if}
    </button>
{/snippet}

{#if lineLayout.type === 'right'}
    <div class="flex items-center">
        <div
            class="w-12 border-t-2 border-dashed border-gray-300"
            class:border-gray-800={highlight}
        ></div>
        {@render buttonArea()}
    </div>
{:else if lineLayout.type === 'under'}
    <div class="m-0.5 flex h-full min-h-7 items-end justify-start">
        <div class="relative">
            <svg
                class="absolute -z-10 stroke-gray-300"
                class:stroke-gray-800={highlight}
                style:left="{-lineLayout.svgWidth - 2}px"
                style:top="{-lineLayout.lineHeight + buttonHeight}px"
                width={lineLayout.svgWidth + layoutUnderOffsetRight}
                height={lineLayout.lineHeight}
            >
                <line
                    x1={lineLayout.svgWidth / 2}
                    y1={lineLayout.lineOffset}
                    x2={lineLayout.svgWidth / 2}
                    y2={lineLayout.lineHeight - buttonHeight / 2}
                    stroke-width="2"
                    stroke-dasharray="5,5"
                />
                <line
                    x1={lineLayout.svgWidth / 2}
                    y1={lineLayout.lineHeight - buttonHeight / 2}
                    x2={lineLayout.svgWidth + layoutUnderOffsetRight}
                    y2={lineLayout.lineHeight - buttonHeight / 2}
                    stroke-width="2"
                    stroke-dasharray="5,5"
                />
            </svg>
            <div style:margin-left="{layoutUnderOffsetRight}px">
                {@render buttonArea()}
            </div>
        </div>
    </div>
{:else}
    <div class="absolute -top-7 m-0.5 h-7">
        <div class="relative">
            <svg
                class="absolute -z-10 stroke-gray-300"
                class:stroke-gray-800={highlight}
                style:left="{-lineLayout.svgWidth - 2}px"
                style:top="0px"
                width={lineLayout.svgWidth + layoutUnderOffsetRight}
                height={buttonHeight + lineLayout.lineOffset}
            >
                <line
                    x1={lineLayout.svgWidth / 2}
                    y1={buttonHeight + lineLayout.lineOffset}
                    x2={lineLayout.svgWidth / 2}
                    y2={buttonHeight / 2}
                    stroke-width="2"
                    stroke-dasharray="5,5"
                />
                <line
                    x1={lineLayout.svgWidth / 2}
                    y1={buttonHeight / 2}
                    x2={lineLayout.svgWidth + layoutUnderOffsetRight}
                    y2={buttonHeight / 2}
                    stroke-width="2"
                    stroke-dasharray="5,5"
                />
            </svg>
            <div style:margin-left="{layoutUnderOffsetRight}px">
                {@render buttonArea()}
            </div>
        </div>
    </div>
{/if}

