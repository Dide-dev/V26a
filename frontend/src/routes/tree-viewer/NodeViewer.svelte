<!--
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
-->

<script lang="ts">
    import NodeViewer from './NodeViewer.svelte';
    import type { Node, RootNode } from './example_tree';
    import { tick } from 'svelte';

    //initialization off types
    let { node, addChild = null, insertChild = null }: {
        node: Node | RootNode;
        addChild?: ((node: Node) => void) | null;
        insertChild?: ((parent: Node, child: Node) => void) | null;
    } = $props();

    /** Width of the standard generating svg. Also defines how far apart the nodes are displayed. */
    const svgWidth = 90;
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
    {#if 'label' in node}
        <div class="h-fit w-32 shrink-0 cursor-default">
            <strong>{node.label}</strong>
        </div>
    {:else}
        <div class="relative h-fit group">
        <button
            class="h-fit w-xl border-2 border-green-800 p-1 bg-green-50"
        >
            <p>{node.card}</p>
        </button>
        {#if addChild}
            <button
                class="absolute top-1.5 right-1 h-5 w-5 rounded-full bg-blue-500 text-white text-xs opacity-20 group-hover:opacity-100 transition-opacity cursor-pointer"
                onclick={() => addChild!(node as Node)}
            >+</button>
        {/if}
    </div>
    {/if}

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
        <div class="relative"> 
            {#if insertChild}
               <button
                    class="absolute -left-8 z-10 h-5 w-5 rounded-full bg-green-500 text-white text-xs opacity-20 hover:opacity-100 transition-opacity cursor-pointer"
                    onclick={() => insertChild!(node as Node, child_node as Node)}
                >+</button>
            {/if}
            <NodeViewer node={child_node} {addChild} {insertChild} />
        </div>
    {/each}
</div>
</div>
