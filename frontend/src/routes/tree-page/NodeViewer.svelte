<!--
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
-->

<script lang="ts">
    import NodeViewer from './NodeViewer.svelte';
    import type { TreeWithCards, Node, RootNode } from '$lib/backend/trees';
    import TreeEditButtons from './TreeEditButtons.svelte';
    import { onMount } from 'svelte';
    import CardViewer from './CardViewer.svelte';

    /** Variables used to contain data about the entire tree that is needed for API calls. */
    let {
        tree = $bindable(),
        parent,
        node,
        parentIsVisible = false,
        editingPage
    }: {
        tree: TreeWithCards;
        parent: Node | RootNode;
        node: Node | RootNode;
        parentIsVisible?: boolean;
        editingPage: boolean;
    } = $props();

    /** Whether this card is visible. This is provided to the child to determine their clickability. */
    let visible = $derived('label' in node);

    /** Width of the standard generating svg. Also defines how far apart the nodes are displayed. */
    const svgWidth = 50;
    let svgHeight = $state(0);

    /** Y-offset from the top of the nodecards, used for the horizontal lines connecting the nodes. */
    const lineOffset = 15;

    /** Stroke width to set the thickness of the lines connecting the nodes. Also used to calculate the svgHeight */
    const strokeWidth = 4;

    let domChildren: Element;
    let svgLines: SvgLine[] = $state([]);
    type SvgLine = {
        x1: number;
        y1: number;
        x2: number;
        y2: number;
    };

    onMount(() => {
        if (!domChildren) return;
        //handles "shifts" in tree
        const observer = new ResizeObserver(() => {
            if (node.children.length === 0) return;

            svgLines = [];
            svgHeight = 0;

            let top = domChildren.getBoundingClientRect().top;

            // Vertical line from lineOffset to last child
            {
                let lastChildBounding =
                    domChildren.children[
                        domChildren.children.length - 1
                    ].children[0].getBoundingClientRect();
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
                let childBounding = child.children[0].getBoundingClientRect();
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
        });
        observer.observe(domChildren);
        return () => observer.disconnect();
    });
</script>

<div class="flex">
    <!-- Root Node -->
    {#if 'label' in node}
        <div class="tree__node h-fit w-32 shrink-0 cursor-default">
            <strong>{node.label}</strong>
        </div>
        <!-- Standard Node -->
    {:else}
        <CardViewer {tree} {node} {parentIsVisible} bind:visible {editingPage} />
    {/if}

    <!-- Adds buttons to manage the cards of the tree if in editing mode -->
    {#if editingPage}
        <div class="relative">
            <TreeEditButtons bind:tree bind:parent bind:node />
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

    <!-- Create children Nodes -->
    <div bind:this={domChildren} class="flex flex-col gap-5">
        {#each node.children as child_node (child_node)}
            <div class="relative">
                <NodeViewer
                    bind:tree
                    parent={node}
                    node={child_node}
                    parentIsVisible={visible}
                    {editingPage}
                />
            </div>
        {/each}
    </div>
</div>
