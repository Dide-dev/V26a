<!--
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
-->

<script lang="ts">
    import NodeViewer from './NodeViewer.svelte';
    import type { TreeWithCards, Node, RootNode } from '$lib/backend/trees';
    import TreeEditButtons from './TreeEditButtons.svelte';
    import AddAndDrop from './AddAndDrop.svelte';
    import { onMount } from 'svelte';
    import CardViewer from './CardViewer.svelte';

    /** Variables used to contain data about the entire tree that is needed for API calls. */
    let {
        tree = $bindable(),
        dragState = $bindable(),
        parent,
        node,
        parentIsVisible = false,
        editingPage
    }: {
        tree: TreeWithCards;
        dragState: DragState;
        parent: Node | RootNode;
        node: Node | RootNode;
        parentIsVisible?: boolean;
        editingPage: boolean;
    } = $props();

    export type DragState = null | DragStateDragging | DragStateDismissed;
    export type DragStateDragging = {
        type: 'dragging';

        parent: Node | RootNode;
        index: number;

        mouseX: number;
        mouseY: number;
    };
    /**
     * Used when the user is dragging, but the specific AddAndDrop instance is not a valid drop box.
     * This is to prevent a node from being dragged into itself and disappearing from the tree.
     */
    export type DragStateDismissed = {
        type: 'dismissed';
    };

    // You cannot drag a parent into it's children.
    // So if this component is being dragged, don't tell my children.
    let iAmBeingDragged = $derived(
        dragState?.type === 'dragging' && dragState.parent.children[dragState.index] === node
    );
    // If next node is dragged there is no need to activate our own AddAndDrop under the card,
    // as it would not change anything for the user to drop it there.
    let nextIsBeingDragged = $derived(
        dragState?.type === 'dragging' && dragState.parent.children[dragState.index - 1] === node
    );
    function dragStateChildrenSet(v: DragState) {
        /// The AddAndDrop will check if it's DragState is correct before calling this 'set' function.
        dragState = v;
    }
    function dragStateChildrenGetSelf(): DragState {
        return iAmBeingDragged || nextIsBeingDragged ? { type: 'dismissed' } : dragState;
    }
    function dragStateChildrenGetChildren(): DragState {
        return iAmBeingDragged ? { type: 'dismissed' } : dragState;
    }

    /** Whether this card is visible. This is provided to the child to determine their clickability. */
    let visible = $derived('label' in node);

    /** Width of the standard generating svg. Also defines how far apart the nodes are displayed. */
    const svgWidth = 50;
    let svgHeight = $state(0);
    let totalHeight = $state(0);

    /** Y-offset from the top of the nodecards, used for the horizontal lines connecting the nodes. */
    const lineOffset = 15;

    /** Stroke width to set the thickness of the lines connecting the nodes. Also used to calculate the svgHeight */
    const strokeWidth = 4;

    let domChildren: Element;
    let outermostDiv: Element;
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

        totalHeight = outermostDiv.getBoundingClientRect().height;

        if (node.children.length === 0) return;

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
        observer.observe(domChildren, { subtree: true, childList: true });
        if (cardViewerContainer !== null) {
            observer.observe(cardViewerContainer, { childList: true });
        }
        return () => observer.disconnect();
    });
</script>

<!--
    Padding on the top is to make room for a AddAndDrop button above here,
    only if this is a root node or I am not the first child.
-->
<div
    class="relative flex"
    bind:this={outermostDiv}
    class:mt-7={editingPage && ('label' in node || parent.children[0] !== node)}
>
    <!--
        Render copy of self (but transparent and smaller) where the mouse currently is.
        We check again for the `dragState.type`, because otherwise typescript doesn't know
        that `dragState.mouseX` and `dragState.mouseY` exists.
    -->
    {#if iAmBeingDragged && dragState?.type === 'dragging'}
        <div
            class="transform:scale(0.7) pointer-events-none fixed z-20 opacity-70"
            style="left: {dragState.mouseX}px; top: {dragState.mouseY}px;"
        >
            <NodeViewer
                {tree}
                dragState={null}
                {parent}
                {node}
                editingPage={false}
                parentIsVisible={true}
            />
        </div>
    {/if}

    {#if editingPage && parent.children[0] === node && 'cardId' in node}
        <AddAndDrop
            bind:dragState={dragStateChildrenGetChildren, dragStateChildrenSet}
            {tree}
            node={parent}
            index={0}
            lineLayout={{ type: 'over', lineOffset, svgWidth }}
        />
    {/if}

    {#if 'label' in node}
        <!-- Root Node -->
        <div class="mb-5 h-fit w-32 shrink-0 cursor-default">
            <strong>{node.label}</strong>
        </div>
    {:else}
        <!-- Standard Node -->
        {#if editingPage}
            <TreeEditButtons bind:tree bind:parent bind:node bind:dragState />
        {/if}
        <div class="flex flex-col">
            <div class="shrink-0" bind:this={cardViewerContainer}>
                <CardViewer {tree} {node} {parentIsVisible} bind:visible {editingPage} />
            </div>
            {#if !editingPage}
                <!--
                    The gap-5 on the children was removed, so that we can fill it with the add button.
                    In view mode, add this gap back.
                -->
                <div class="h-5"></div>
            {:else}
                <AddAndDrop
                    bind:dragState={dragStateChildrenGetSelf, dragStateChildrenSet}
                    {tree}
                    node={parent}
                    index={1 + parent.children.indexOf(node)}
                    lineLayout={{ type: 'under', lineHeight: totalHeight, lineOffset, svgWidth }}
                />
            {/if}
        </div>
    {/if}

    {#if node.children.length !== 0}
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
            <NodeViewer
                bind:tree
                bind:dragState={dragStateChildrenGetChildren, dragStateChildrenSet}
                parent={node}
                node={child_node}
                parentIsVisible={visible}
                {editingPage}
            />
        {/each}
    </div>

    {#if editingPage && node.children.length === 0}
        <div class="flex h-9 items-center">
            <AddAndDrop
                bind:dragState={dragStateChildrenGetChildren, dragStateChildrenSet}
                {tree}
                {node}
                index={0}
                lineLayout={{ type: 'right' }}
            />
        </div>
    {/if}
</div>
