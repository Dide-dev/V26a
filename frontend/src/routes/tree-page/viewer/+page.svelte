<!--
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
-->

<script lang="ts">
    import SubtreeViewer from '../SubtreeViewer.svelte';
    import { onMount } from 'svelte';
    import { getTreeWithCards, type TreeWithCards } from '$lib/backend/trees';
    import { page } from '$app/state';
    import { m } from '$lib/paraglide/messages.js';
    import { getUserInfo } from '$lib/backend/users';
    import { goto } from '$app/navigation';
    import { appLink } from '$lib';

    let userInfo = getUserInfo();

    let userAllowedToEdit = $state(false);
    let tree: TreeWithCards | null = $state(null);

    onMount(async () => {
        const pageId = page.url.searchParams.get('id');

        if (!pageId) {
            // TODO: redirect to tree list
            await goto(appLink('/'));
            return;
        }

        let id = Number(pageId);

        if (Number.isNaN(id)) {
            // TODO: redirect to tree list
            await goto(appLink('/'));
            return;
        }

        tree = await getTreeWithCards(id);

        const isOwner = tree.genData.ownerId === userInfo?.sub;
        const isAdmin = userInfo?.accountType === 'admin';

        if (isOwner || isAdmin) {
            userAllowedToEdit = true;
        }
    });
</script>

<div class="flex h-full w-full flex-col items-start overflow-hidden">
    {#if tree === null}
        <p class="text-green-800">{m.loadingTree()}</p>
    {:else}
        {#if userAllowedToEdit}
            <button
                data-testid="edit-button"
                class="fixed top-16 right-4 z-50 m-2 cursor-pointer rounded-sm bg-emerald-500 px-4 py-2 shadow"
                onclick={() => goto(appLink(`/tree-page/editor?id=${tree!.genData.id}`))}
            >
                {m.edit()}
            </button>
        {/if}

        <h1 class="m-auto ml-1.5 text-3xl font-bold">{tree.data.title}</h1>
        <p class="m-auto ml-1.5 pt-1 pb-1 text-sm font-bold text-gray-400">
            {tree.data.description}
        </p>

        <div class="m-1 flex w-full grow flex-col overflow-x-auto pt-6">
            {#each tree.structure.rootNodes as root (root)}
                <SubtreeViewer node={root} cards={tree.cards} />
            {/each}
        </div>
    {/if}
</div>
