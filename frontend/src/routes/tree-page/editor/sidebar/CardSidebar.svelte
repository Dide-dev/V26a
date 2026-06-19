<!--
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
-->

<script lang="ts">
    import type { CardSideBar, ManageSideBar } from './Sidebar.svelte';
    import CardEditor from './CardEditor.svelte';
    import InsertFromLibrary from './InsertFromLibrary.svelte';
    import UploadToLibrary from './UploadToLibrary.svelte';
    import type { TreeHistory } from '../history.svelte.ts';
    import type { Notice } from '$lib/Notices.svelte';

    let {
        sidebar = $bindable(),
        history,
        notices = $bindable()
    }: {
        sidebar: CardSideBar | ManageSideBar;
        history: TreeHistory;
        notices: Notice[];
    } = $props();
</script>

{#if sidebar.type === 'NewCard' || sidebar.type === 'EditCard'}
    {#if sidebar.insertingFromLibrary}
        <InsertFromLibrary bind:sidebar />
    {:else}
        <CardEditor bind:sidebar {history} />
    {/if}
{:else if sidebar.type === 'UploadToLibrary'}
    <UploadToLibrary bind:sidebar {history} bind:notices />
{/if}
