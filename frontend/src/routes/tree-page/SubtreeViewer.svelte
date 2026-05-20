<!--
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
-->

<script lang="ts">
    import type { CardGetData, CardId } from '$lib/api-wrapper/cards';
    import type { Node, RootNode } from '$lib/api-wrapper/trees';
    import CardViewer from './CardViewer.svelte';
    import SubtreeViewer from './SubtreeViewer.svelte';
    import LineDrawings from './LineDrawer.svelte';

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
            <CardViewer bind:revealed {parentRevealed} card={cards[node.cardId]} />
        </div>
    {/if}

    <LineDrawings>
        <!-- Here the html gets recursive as we render the 'node.children' -->
        {#each node.children as child_node (child_node)}
            <SubtreeViewer node={child_node} {cards} parentRevealed={revealed} />
        {/each}
    </LineDrawings>
</div>
