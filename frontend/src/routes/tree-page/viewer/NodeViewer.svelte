<!--
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
-->

<script lang="ts">
    import type { CardGetData, CardId } from '$lib/backend/cards';
    import type { Node, RootNode } from '$lib/backend/trees';
    import CardViewer from './CardViewer.svelte';
    import NodeViewer from './NodeViewer.svelte';
    import { onMount } from 'svelte';

    let {
        node,
        cards,
        parentRevealed = true
    }: {
        node: RootNode | Node;
        cards: Record<CardId, CardGetData>;
        parentRevealed?: boolean;
    } = $props();

    let revealed = $derived('label' in node);

    /** Width of the standard generating svg. Also defines how far apart the nodes are displayed. */
    const svgWidth = 50;
    let svgHeight = $state(0);

    /** Y-offset from the top of the nodecards, used for the horizontal lines connecting the nodes. */
    const lineOffset = 15;

    /** Stroke width to set the thickness of the lines connecting the nodes. Also used to calculate the svgHeight */
    const strokeWidth = 4;

    let domChildren: Element;
    let cardViewerContainer: Element | null = $state(null);
    let svgLines: SvgLine[] = $state([]);
    type SvgLine = {
        x1: number;
        y1: number;
        x2: number;
        y2: number;
    };

    function redrawLines() {
        svgLines = [];
        svgHeight = 0;

        // It is possible that node.children.length === 0, but domChildren.children.length > 0.
        // This happens when adding a new Card and a sidebar is open
        if (domChildren.children.length === 0) return;

        let top = domChildren.getBoundingClientRect().top;

        // Vertical line from lineOffset to last child
        {
            let lastChildBounding =
                domChildren.children[domChildren.children.length - 1].getBoundingClientRect();
            let childTop = lastChildBounding.top;
            svgLines.push({
                x1: svgWidth / 2,
                y1: lineOffset,
                x2: svgWidth / 2,
                y2: childTop - top + lineOffset
            });
            svgHeight = strokeWidth + childTop - top + lineOffset;
        }

        // Right horizontal line from vertical line to child
        for (let child of domChildren.children) {
            let childBounding = child.getBoundingClientRect();
            let childTop = childBounding.top;

            svgLines.push({
                x1: svgWidth / 2,
                y1: childTop + lineOffset - top,
                x2: svgWidth,
                y2: childTop + lineOffset - top
            });
        }

        // Left horizontal line from parent to vertical line
        svgLines.push({
            x1: 0,
            y1: lineOffset,
            x2: svgWidth / 2,
            y2: lineOffset
        });
    }

    onMount(() => {
        redrawLines();

        let observer = new MutationObserver(redrawLines);
        observer.observe(domChildren, { subtree: true, childList: true, characterData: true });
        if (cardViewerContainer !== null) {
            observer.observe(cardViewerContainer, {
                subtree: true,
                childList: true,
                characterData: true
            });
        }
        return () => observer.disconnect();
    });
</script>

<div class="flex">
    {#if 'label' in node}
        <!-- Root Node -->
        <div class="mb-5 h-fit w-32 shrink-0 wrap-anywhere">
            <strong>{node.label}</strong>
        </div>
    {:else}
        <!-- Standard Node -->
        <div class="mb-2">
            <!-- TODO: figure out why browsers insert random vertical whitespace in this division -->
            <CardViewer bind:revealed {parentRevealed} card={cards[node.cardId]} />
        </div>
    {/if}

    {#if svgLines.length !== 0}
        <svg width={svgWidth} height={svgHeight} class="shrink-0">
            {#each svgLines as line (line)}
                <line
                    {...line}
                    class="stroke-yellow-900"
                    stroke-linecap="round"
                    stroke-width={strokeWidth}
                />
            {/each}
        </svg>
    {/if}

    <!-- Here the html gets recursive as we render the 'node.children' -->
    <div bind:this={domChildren} class="flex flex-col">
        {#each node.children as child_node (child_node)}
            <NodeViewer node={child_node} {cards} parentRevealed={revealed} />
        {/each}
    </div>
</div>
