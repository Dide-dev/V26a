<!--
    This program has been developed by students from the bachelor Computer Science at Utrecht
    University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
-->
<script lang="ts">
    import type { TreeWithCards, Node } from '$lib/backend/trees';
    let { tree, node, visible = $bindable(), visibleParent = false }: { tree: TreeWithCards; node: Node; visible:boolean; visibleParent:boolean; } = $props();

    function setVisible() {
        visible = true;
    }

</script>
{#if visible}
    <div class="h-fit w-xl border-2 border-green-800 bg-green-50 p-1">
        <p>{tree.cards[node.cardId].back}</p>
    </div>
{:else if visibleParent}
    <div class="h-fit w-fit border-2 border-green-800 bg-green-600 p-1">
        <button onclick={setVisible}>{tree.cards[node.cardId].front}</button>
    </div>
{:else}
    <div class="h-fit w-s border-2 border-green-800 bg-green-800 p-1">
        <p><br></p>
    </div>
{/if}
