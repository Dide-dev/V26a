<!--
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
-->

<!--
    @component
    Render a sub-tree with its children.

    ## Params
    - `node: RootNode | Node` - The contents of the current node, and its children.
    - `cards: Record<CardId, CardData>` - The contents of all cards.
    - `color: BranchColor` - The name of the color.
    - `distanceFromRevealed: number` - The distance from the closest revealed card.
                                       If `0`, the card is revealed.
    - `isRevealed: boolean` - Whether this node has been revealed.
-->

<script lang="ts">
    import type { CardData, CardId } from '$lib/api-wrapper/cards';
    import type { Node, RootNode } from '$lib/api-wrapper/trees';
    import CardViewer from './CardViewer.svelte';
    import SubtreeViewer from './SubtreeViewer.svelte';
    import LineDrawings from './LineDrawer.svelte';
    import AnimatedUnderline from '$lib/AnimatedUnderline.svelte';
    import type { BranchColor } from '$lib/treePage';
    import { m } from '$lib/paraglide/messages.js';

    let {
        node,
        cards,
        color,
        distanceFromRevealed = 0,
        isRevealed = $bindable(false)
    }: {
        node: RootNode | Node;
        cards: Record<CardId, CardData>;
        color: BranchColor;
        distanceFromRevealed: number;
        isRevealed: boolean;
    } = $props();

    let rootOpen = $state(true);

    let childrenRevealed = $state(
        (() => Array.from({ length: node.children.length }, () => false))()
    );

    let outOfFocus = $derived(node.children.length > 0 && !childrenRevealed.some((b) => !b));

    // We use an effect-rune to interact correctly with bound variables.
    $effect(() => {
        isRevealed = distanceFromRevealed === 0;
    });

    function toggleRoot() {
        if (node.children.length > 0) {
            rootOpen = !rootOpen;
        }
    }
</script>

<div class="flex">
    {#if 'label' in node}
        <!-- Root Node -->
        <button
            class={[
                !rootOpen && 'closed',
                node.children.length > 0 && 'hover:text-neutral-600',
                'root mr-1 mb-5 ml-1 flex h-fit w-card-width-md shrink-0 cursor-pointer flex-row gap-2 text-right wrap-anywhere *:*:justify-end'
            ]}
            onclick={toggleRoot}
            title={rootOpen ? m.closeBranch() : m.openBranch()}
        >
            {#if node.children.length > 0}
                <AnimatedUnderline
                    --weight-underline="calc(var(--spacing) / 3)"
                    --color-underline="var(--color-neutral-600)"
                >
                    <p class="font-semibold">{node.label}</p>
                </AnimatedUnderline>
            {:else}
                <p class="font-semibold">{node.label}</p>
            {/if}
        </button>
    {:else}
        <!-- Standard Node -->
        <!-- The 'flex' here prevents some weird vertical whitespace -->
        <div class="mb-2 flex">
            <CardViewer bind:distanceFromRevealed card={cards[node.cardId]} {color} {outOfFocus} />
        </div>
    {/if}

    {#if rootOpen}
        <LineDrawings alwaysFullWidth={false}>
            <!-- Here the html gets recursive as we render the 'node.children' -->
            {#each node.children as child_node, i (child_node)}
                <SubtreeViewer
                    node={child_node}
                    {cards}
                    {color}
                    distanceFromRevealed={distanceFromRevealed + 1}
                    bind:isRevealed={childrenRevealed[i]}
                />
            {/each}
        </LineDrawings>
    {/if}
</div>

<style>
    @layer components {
        .root {
            position: relative;
            transition: color 200ms ease-in-out;

            &::before {
                /* Layout */
                content: '';
                position: absolute;
                bottom: 5%;
                height: 87.5%;
                z-index: 1;
                left: 0;

                /* Formatting */
                background-color: var(--color-neutral-350);
                width: var(--spacing-padding-minor);
                transition: background-color 200ms ease-in-out;
            }

            &:hover::before {
                background-color: var(--color-neutral-500);
            }

            &::after {
                content: '˅';
                font-weight: var(--font-weight-bold);
                transition: transform 200ms ease-in-out;
                height: fit-content;
            }

            &.closed::after {
                transform: rotate(-90deg);
            }
        }
    }
</style>
