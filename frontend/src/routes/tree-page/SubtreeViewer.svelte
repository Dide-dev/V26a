<!--
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
-->

<script lang="ts">
    import type { CardData, CardId } from '$lib/api-wrapper/cards';
    import type { Node, RootNode } from '$lib/api-wrapper/trees';
    import CardViewer from './CardViewer.svelte';
    import SubtreeViewer from './SubtreeViewer.svelte';
    import LineDrawings from './LineDrawer.svelte';

    let {
        node,
        cards,
        distanceFromRevealed,
        isRevealed = $bindable()
    }: {
        node: RootNode | Node;
        cards: Record<CardId, CardData>;
        distanceFromRevealed: number;
        isRevealed: boolean;
    } = $props();

    let revealed = $derived('label' in node);

    let distance = $derived(revealed ? 0 : distanceFromRevealed);

    let childrenRevealed = $state(
        (() => Array.from({ length: node.children.length }, () => false))()
    );

    let outOfFocus = $derived(node.children.length > 0 && !childrenRevealed.some((b) => !b));

    $effect(() => {
        isRevealed = distance === 0;
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
        <!-- The 'flex' here prevents some weird vertical whitespace -->
        <div class="mb-2 flex">
            <CardViewer
                bind:distanceFromRevealed={distance}
                card={cards[node.cardId]}
                baseColor="tree"
                bind:outOfFocus
            />
        </div>
    {/if}

    <LineDrawings alwaysFullWidth={false}>
        <!-- Here the html gets recursive as we render the 'node.children' -->
        {#each node.children as child_node, i (child_node)}
            <SubtreeViewer
                node={child_node}
                {cards}
                distanceFromRevealed={distance + 1}
                bind:isRevealed={childrenRevealed[i]}
            />
        {/each}
    </LineDrawings>
</div>
