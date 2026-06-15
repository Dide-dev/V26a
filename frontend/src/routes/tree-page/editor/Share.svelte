<!--
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
-->

<script lang="ts">
    import { m } from '$lib/paraglide/messages';
    import { Alert } from 'flowbite-svelte';
    import { CheckOutline } from 'flowbite-svelte-icons';
    import type { ManageSideBar, MenuSideBar } from './Sidebar.svelte';
    import { type TreeData, updateTreeMetaData } from '$lib/api-wrapper/trees';
    import { page } from '$app/state';

    let {
        sidebar = $bindable(),
        treeId,
        prevTreeData = $bindable()
    }: {
        treeId: number;
        sidebar: ManageSideBar | MenuSideBar;
        prevTreeData: TreeData;
    } = $props();

    let notices: string[] = $state([]);

    function onkeydown(event: KeyboardEvent) {
        // When ctrl+z is pressed, we only want that event to be handled in the relevant textbox,
        // do not let it propagate to the undo/redo buttons of the tree editor page.
        if (event.key !== 'Escape') event.stopPropagation();
    }

    async function onsubmit(event: Event) {
        event.preventDefault();

        prevTreeData.published = !prevTreeData.published;

        await updateTreeMetaData(treeId, prevTreeData);

        sidebar = {
            kind: 'Menu',
            type: 'IconsOnly'
        };
    }

    /** Copy link to tree viewer page to user's clipboard. */
    function copyLink(event: Event) {
        event.preventDefault();

        navigator.clipboard.writeText(
            `${page.url.protocol}//${page.url.hostname}/tree-page/viewer?id=${treeId}`
        );

        if (notices.length < 5) notices.push(m.linkCopied());
    }

    function dismissNotice(index: number) {
        notices.splice(index, 1);
    }
</script>

<svelte:body {onkeydown} />

<form class="completed flex w-full flex-col gap-1">
    <h2 class="page__heading">{m.share()}</h2>

    {#if prevTreeData.published}
        <p>{m.publishedTreeNotice()}</p>
        <p>{m.publishedTreeSharingNotice()}</p>
        <div class="grid grid-cols-2 gap-1">
            <button type="reset" onclick={copyLink}>
                {m.copyLink()}
            </button>
            <button type="submit" onclick={onsubmit}>
                {m.unpublish()}
            </button>
        </div>
    {:else}
        <p>{m.unpublishedTreeSharingNotice()}</p>
        <button type="submit" onclick={onsubmit}>
            {m.publish()}
        </button>
    {/if}
</form>

<div class="fixed right-2 bottom-2 z-50 flex flex-col gap-1">
    {#each notices as notice, i (i)}
        <Alert
            dismissable
            onclick={() => dismissNotice(i)}
            class="rounded-md border bg-tree-100 text-tree-610"
        >
            {#snippet icon()}
                <CheckOutline class="h-5 w-5 shrink-0" />
            {/snippet}
            <p>{notice}</p>
        </Alert>
    {/each}
</div>
