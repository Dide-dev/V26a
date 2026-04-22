<!--
    This program has been developed by students from the bachelor Computer Science at Utrecht
    University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
-->

<script lang="ts">
    import type { TreeWithCards, Node } from '$lib/backend/trees';
    let {
        tree,
        node,
        visible = $bindable(),
        parentIsVisible = false,
        editingPage
    }: {
        tree: TreeWithCards;
        node: Node;
        visible: boolean;
        parentIsVisible: boolean;
        editingPage: boolean;
    } = $props();

    function setVisible() {
        visible = true;
    }
    function toggleVisible() {
        visible = !visible;
    }
</script>

{#if editingPage}
    <!-- Cards layout in the editor -->
    {#if visible}
        <button
            class="h-fit min-h-9 w-xl cursor-pointer border-2 border-green-800 bg-green-50 p-1 text-left wrap-anywhere"
            onclick={toggleVisible}
        >
            {tree.cards[node.cardId].back}
        </button>
    {:else}
        <button
            class="h-fit min-h-9 w-xl cursor-pointer border-2 border-green-800 bg-green-500 p-1 text-center wrap-anywhere"
            onclick={toggleVisible}
        >
            {tree.cards[node.cardId].front}
        </button>
    {/if}
{:else}
    <!-- Cards layout in the viewer -->
    {#if visible}
        <p class="h-fit min-h-9 w-xl border-2 border-green-800 bg-green-50 p-1 wrap-anywhere">
            {tree.cards[node.cardId].back}
        </p>
    {:else if parentIsVisible}
        <button
            class="h-fit min-h-9 w-fit max-w-xl min-w-50 cursor-pointer border-2 border-green-800 bg-green-500 p-1 text-center wrap-anywhere"
            onclick={setVisible}
        >
            {tree.cards[node.cardId].front}
        </button>
    {:else}
        <p class="tree-node h-fit min-h-9 w-4 border-2 border-green-800 bg-green-800 p-1"><br /></p>
    {/if}
{/if}
