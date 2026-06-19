<!--
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
-->

<script lang="ts">
    import { m } from '$lib/paraglide/messages';
    import type { ManageSideBar, MenuSideBar } from './Sidebar.svelte';
    import { removeTree } from '$lib/api-wrapper/trees';
    import { goto } from '$app/navigation';
    import { appLink } from '$lib';

    let {
        sidebar = $bindable(),
        treeId,
        title
    }: {
        sidebar: ManageSideBar | MenuSideBar;
        treeId: number;
        title: string;
    } = $props();

    function onkeydown(event: KeyboardEvent) {
        // When ctrl+z is pressed, we only want that event to be handled in the relevant textbox,
        // do not let it propagate to the undo/redo buttons of the tree editor page.
        if (event.ctrlKey) event.stopPropagation();
    }

    async function onsubmit(event: Event) {
        event.preventDefault();

        await removeTree(treeId);

        await goto(appLink('/my-trees'));
    }

    function oncancel(event: Event) {
        event.preventDefault();

        sidebar = {
            kind: 'Menu',
            type: 'IconsOnly'
        };
    }
</script>

<svelte:body {onkeydown} />

<form class="completed flex w-full flex-col gap-1">
    <h2 class="page__heading">{m.deleteTree()}</h2>
    <p>{m.deleteTreeConfirmation({ title })}</p>
    <div class="grid grid-cols-2 gap-1">
        <button type="reset" onclick={oncancel}>
            {m.cancel()}
        </button>
        <button type="submit" onclick={onsubmit} class="danger">
            {m.confirm()}
        </button>
    </div>
</form>
