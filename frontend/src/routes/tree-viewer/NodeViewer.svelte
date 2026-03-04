<script lang="ts">
    import NodeViewer from './NodeViewer.svelte';
    import type { Node, RootNode } from './example_tree';
    import { tick } from 'svelte';

    //initialization off types
    let { node, selectedNode = null, secondSelectedNode = null, onSelect = () => {} }: {
        node: Node | RootNode;
        selectedNode?: Node | null;
        secondSelectedNode?: Node | null;
        onSelect?: (node: Node) => void;
    } = $props();

    /** Width of the standard generating svg. Also defines how far apart the nodes are displayed. */
    const svgWidth = 50;
    let svgHeight = $state(0);

    /** Y-offset from the top of the nodecards, used for the horizontal lines connecting the nodes. */
    const lineOffset = 15;

    /** Stroke width to set the thickness of the lines connecting the nodes. Also used to calculate the svgHeight*/
    const strokeWidth = 4;

    let domChildren: Element;
    let svgLines: SvgLine[] = $state([]);
    type SvgLine = {
        x1: number;
        y1: number;
        x2: number;
        y2: number;
    };

    $effect(() => {
        if (!domChildren) return;
        //handles "shifts" in tree
        const observer = new ResizeObserver(() => {
            if (node.children.length === 0) return;

            tick().then(() => {
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
        });
        observer.observe(domChildren);
        return () => observer.disconnect();
    });
</script>

<div class="flex">
    <button
        class="h-fit w-xl cursor-pointer border-2 border-green-800 p-1"
        class:bg-green-50={selectedNode !== node}
        class:bg-fuchsia-300={selectedNode === node}
        class:bg-orange-300={secondSelectedNode === node}
        onclick={() => onSelect(node as Node)}
    >
        {#if 'label' in node}
            <div class="h-fit w-32 shrink-0">
                <strong>{node.label}</strong>
            </div>
        {:else}
            <p>{node.card}</p>
        {/if}
    </button>

    {#if node.children.length !== 0}
        <svg width={svgWidth} height={svgHeight} class="shrink-0">
            {#each svgLines as line (line)}
                <line
                    {...line}
                    class="stroke-yellow-950"
                    stroke-linecap="round"
                    stroke-width={strokeWidth}
                />
            {/each}
        </svg>
    {/if}

    <div bind:this={domChildren} class="flex flex-col gap-5">
        {#each node.children as child_node (child_node)}
            <NodeViewer node={child_node} {selectedNode} {secondSelectedNode} {onSelect} />
        {/each}
    </div>
</div>
