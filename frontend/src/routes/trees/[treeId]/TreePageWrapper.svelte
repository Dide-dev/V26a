<!--
    Necessary for testing the tree-page with context.
-->

<script lang="ts">
    import Page from './+page.svelte';
    import { setUIState, UIState } from '$lib/context/ui.svelte.js';

    let { params }: { params: { treeId: string } } = $props();

    const uiState = new UIState();
    setUIState(uiState);
</script>

<Page {params} />
