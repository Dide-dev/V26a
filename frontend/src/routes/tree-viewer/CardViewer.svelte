<!--
    This program has been developed by students from the bachelor Computer Science at Utrecht
    University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
-->
<script lang="ts">
    import type { Node } from './example_tree';
    let { node, visibleParent = false }: { node: Node; visibleParent?: boolean } = $props();
    let visible = $state(false);
    let clickable = $state(() => visibleParent);

</script>

{#if visibleParent}
<p>{node.card}</p>
{:else}
    <p>No visible Parent</p>
{/if}
