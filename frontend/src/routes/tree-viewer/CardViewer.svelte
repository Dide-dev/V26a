<!--
    This program has been developed by students from the bachelor Computer Science at Utrecht
    University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
-->
<script lang="ts">
    import type { Node } from './example_tree';
    let { node, visible = $bindable(), visibleParent = false }: { node: Node; visible:boolean; visibleParent:boolean; } = $props();

    function setVisible() {
        visible = true;
    }

</script>

{#if visible}
    <p>{node.card}</p>
{:else if visibleParent}
    <button onclick={setVisible}>Clickable</button>
{:else}
    <p>Not clickable</p>
{/if}
