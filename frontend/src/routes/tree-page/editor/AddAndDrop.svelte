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
    import iconAdd from '$lib/assets/icons/add.svg?raw';
    import type { CardGetData } from '$lib/backend/cards';
    import { doMove, type TreeHistory } from './history.svelte';
    import type { DragState } from './SubtreeEditor.svelte';
    import type { SideBar } from './Sidebar.svelte';
    import { lineOffset, svgWidth } from '../lineConstants';

    let {
        history,
        location,
        dragState = $bindable(),
        lineLayout,
        sidebar = $bindable()
    }: {
        history: TreeHistory;
        location: number[];
        dragState: null | DragState;
        lineLayout: Layout;
        sidebar: null | SideBar;
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
    };
    /** Used when this component is above a node. */
    export type LayoutOver = {
        type: 'over';
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

    // Clicking for adding cards
    async function onclick() {
        if (dragState !== null) {
            return;
        }
        let editingVersion: CardGetData = { front: '', back: '' };
        if (sidebar?.type === 'NewCard') {
            // Don't forget what the user typed if the user clicks a different add card.
            editingVersion = sidebar.editingVersion;
        }
        sidebar = {
            type: 'NewCard',
            location: location,
            editingVersion
        };
    }

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
        class="flex w-2xs items-center justify-center rounded-2xl border-2 border-dashed"
        style="height: {buttonHeight}px"
        class:bg-amber-200={isHovered && dragState?.type == 'dragging'}
        class:fill-gray-300={!highlight}
        class:fill-gray-800={highlight}
        class:border-gray-300={!highlight}
        class:border-gray-800={highlight}
        class:cursor-pointer={highlight}
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
                style:left="{-svgWidth - 2}px"
                style:top="{-lineLayout.lineHeight + buttonHeight}px"
                width={svgWidth + layoutUnderOffsetRight}
                height={lineLayout.lineHeight}
            >
                <line
                    x1={svgWidth / 2}
                    y1={lineOffset}
                    x2={svgWidth / 2}
                    y2={lineLayout.lineHeight - buttonHeight / 2}
                    stroke-width="2"
                    stroke-dasharray="5,5"
                />
                <line
                    x1={svgWidth / 2}
                    y1={lineLayout.lineHeight - buttonHeight / 2}
                    x2={svgWidth + layoutUnderOffsetRight}
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
                style:left="{-svgWidth - 2}px"
                style:top="0px"
                width={svgWidth + layoutUnderOffsetRight}
                height={buttonHeight + lineOffset}
            >
                <line
                    x1={svgWidth / 2}
                    y1={buttonHeight + lineOffset}
                    x2={svgWidth / 2}
                    y2={buttonHeight / 2}
                    stroke-width="2"
                    stroke-dasharray="5,5"
                />
                <line
                    x1={svgWidth / 2}
                    y1={buttonHeight / 2}
                    x2={svgWidth + layoutUnderOffsetRight}
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
