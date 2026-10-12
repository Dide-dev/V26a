<!--
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
-->

<!--
    @component
    Render a sub-tree with its children. All cards will be fully displayed for printing purposes.

    ## Params
    - `node: RootNode | Node` - The contents of the current node, and its children.
    - `cards: Record<CardId, CardData>` - The contents of all cards.
    - `color: BranchColor` - The name of the color.
-->

<script lang="ts">
    import type { CardData, CardId } from '$lib/api-wrapper/cards';
    import type { Node, RootNode } from '$lib/api-wrapper/trees';
    import CardViewer from './CardViewer.svelte';
    import SubtreeViewer from './SubtreeViewer.svelte';
    import LineDrawings from '../LineDrawer.svelte';
    import type { BranchColor } from '$lib/treePage';

    let {
        node,
        cards,
        color
    }: {
        node: RootNode | Node;
        cards: Record<CardId, CardData>;
        color: BranchColor;
    } = $props();
</script>

<div class="flex">
    {#if 'label' in node}
        {#if node.children.length > 0}
            <!-- Root Node -->
            <div
                class={[
                    'root mr-1 mb-5 ml-1 flex h-fit w-card-width-md shrink-0 flex-row gap-2 text-right wrap-anywhere justify-end'
                ]}
            >
                <p class="font-semibold">{node.label}</p>
            </div>
        {/if}
    {:else}
        <!-- Standard Node -->
        <!-- The 'flex' here prevents some weird vertical whitespace -->
        <div class="mb-2 flex">
            <CardViewer card={cards[node.cardId]} {color} />
        </div>
    {/if}

    <LineDrawings alwaysFullWidth={false}>
        <!-- Here the html gets recursive as we render the 'node.children' -->
        {#each node.children as child_node (child_node)}
            <SubtreeViewer node={child_node} {cards} {color} />
        {/each}
    </LineDrawings>
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
        }
    }
</style>
