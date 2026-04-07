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
        visibleParent = false,
        editingPage
    }: {
        tree: TreeWithCards;
        node: Node;
        visible: boolean;
        visibleParent: boolean;
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
        <div
            class="tree__node h-fit w-xl border-2 border-green-800 bg-green-50 p-1 text-center wrap-anywhere"
        >
            <button class="min-h-6 w-xl cursor-pointer" onclick={toggleVisible}>
                {tree.cards[node.cardId].back}
            </button>
        </div>
    {:else}
        <div
            class="tree__node h-fit w-xl border-2 border-green-800 bg-green-500 p-1 text-center wrap-anywhere"
        >
            <button class="min-h-6 w-xl cursor-pointer" onclick={toggleVisible}>
                {tree.cards[node.cardId].front}
            </button>
        </div>
    {/if}
{:else}
    <!-- Cards layout in the viewer -->
    {#if visible}
        <div class="tree__node h-fit w-xl border-2 border-green-800 bg-green-50 p-1 wrap-anywhere">
            <p>{tree.cards[node.cardId].back}</p>
        </div>
    {:else if visibleParent}
        <div
            class="tree__node h-fit w-fit max-w-xl border-2 border-green-800 bg-green-500 p-1 text-center wrap-anywhere"
        >
            <button class="min-h-6 min-w-50 cursor-pointer" onclick={setVisible}>
                {tree.cards[node.cardId].front}
            </button>
        </div>
    {:else}
        <div class="tree-node h-fit w-4 border-2 border-green-800 bg-green-800 p-1">
            <p><br /></p>
        </div>
    {/if}
{/if}
