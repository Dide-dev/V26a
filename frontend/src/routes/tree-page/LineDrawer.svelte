<!--
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
-->

<!--component
    This component expects te be in a flex row, and creates two elements:
    one is the svg with lines, the other is a flex column with the children.
 -->

<script lang="ts">
    import { onMount } from 'svelte';

    import type { Snippet } from 'svelte';
    import { lineOffset, strokeWidth, svgWidth } from './lineConstants';

    let { children }: { children: Snippet } = $props();

    let svgHeight = $state(0);

    let container: Element;
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

        if (container.children.length === 0) return;

        let top = container.getBoundingClientRect().top;

        // Vertical line from lineOffset to last child
        {
            let lastChildBounding =
                container.children[container.children.length - 1].getBoundingClientRect();
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
        for (let child of container.children) {
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
        // We set everything up so that each time any child element resizes,
        // the `redrawLines` function is called.
        // Any time the list of children changes, we also need to point the ResizeObserver to the new list of children.
        let resizeObserver = new ResizeObserver(redrawLines);
        function setupResizeObservers() {
            resizeObserver.disconnect();
            for (let child of container.children) {
                resizeObserver.observe(child);
            }
            redrawLines();
        }

        let mutationObserver = new MutationObserver(setupResizeObservers);
        mutationObserver.observe(container, { childList: true });
        setupResizeObservers();

        return () => {
            mutationObserver.disconnect();
            resizeObserver.disconnect();
        };
    });
</script>

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
<div bind:this={container} class="flex flex-col">
    {@render children()}
</div>
