<!--
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
-->

<!--component
    This sidebar is currently only used for the card editing, but could be used for more later.
 -->
<script lang="ts">
    import { type CardData } from '$lib/api-wrapper/cards';
    import { m } from '$lib/paraglide/messages';
    import { type TreeHistory } from './history.svelte.ts';
    import {
        CellAttributesOutline,
        ShareNodesOutline,
        UserSettingsOutline
    } from 'flowbite-svelte-icons';
    import AnimatedUnderline from '$lib/AnimatedUnderline.svelte';
    import EditMetadata from './EditMetadata.svelte';
    import EditCard from './EditCard.svelte';
    import type { TreeData, TreeLinkedData } from '$lib/api-wrapper/trees';
    import CoAuthor from './CoAuthor.svelte';
    import Share from './Share.svelte';
    import CardsLibrary from './CardsLibrary.svelte';

    let {
        sidebar = $bindable(),
        history,
        hasOwnerPermissions = true,
        treeId,
        prevTreeData = $bindable(),
        treeLinkedData
    }: {
        sidebar: SideBar | null;
        history: TreeHistory;
        hasOwnerPermissions?: boolean;
        treeId: number;
        prevTreeData: TreeData;
        treeLinkedData: TreeLinkedData;
    } = $props();

    export type SideBar = CardEditSideBar | ManageSideBar;

    export type CardEditSideBar = {
        card: true;
        type: 'NewCard' | 'EditCard';
        location: number[];
        editingVersion: CardData;
        errorMessage?: string;
    };

    export type ManageSideBar = {
        card: false;
        type: 'EditMetadata' | 'ManageCoAuthors' | 'Share';
    };
</script>

<div
    class="h-full overflow-x-hidden overflow-y-auto rounded-bl-2xl bg-neutral-100 p-2 pb-4 shadow-[-5px_0px_20px_0px_rgba(0_0_0/0.2)]"
>
    {#if sidebar === null}
        <div
            class="flex flex-col gap-1 *:ml-2 *:flex *:cursor-pointer *:flex-row *:items-center *:gap-2 *:rounded-md *:pb-2 *:pl-2 *:hover:bg-neutral-200"
        >
            <div class="m-1 flex gap-2 rounded">
                <button
                    class="cursor-pointer rounded p-1"
                    class:bg-green-500={!isLibrary}
                    onclick={() => (isLibrary = false)}
                >
                    <h1>{m.editThisCard()}</h1>
                </button>
                <button
                    class="cursor-pointer rounded p-1"
                    class:bg-green-500={isLibrary}
                    onclick={async () => {
                        isLibrary = true;
                    }}
                >
                    <h1>{m.cardLibrary()}</h1>
                </button>
            </div>
            <button
                type="button"
                onclick={() => {
                    sidebar = {
                        card: false,
                        type: 'EditMetadata'
                    };
                }}
            >
                <CellAttributesOutline class="page__heading h-7 w-7 shrink-0" />
                <AnimatedUnderline --weight-underline="calc(var(--spacing) / 1.5)">
                    <span class="page__heading">{m.editMetadata()}</span>
                </AnimatedUnderline>
            </button>
            {#if hasOwnerPermissions}
                <button
                    type="button"
                    onclick={() => {
                        sidebar = {
                            card: false,
                            type: 'ManageCoAuthors'
                        };
                    }}
                >
                    <UserSettingsOutline class="page__heading h-7 w-7 shrink-0" />
                    <AnimatedUnderline --weight-underline="calc(var(--spacing) / 1.5)">
                        <span class="page__heading">{m.coAuthors()}</span>
                    </AnimatedUnderline>
                </button>
            {/if}
            <button
                type="button"
                onclick={() => {
                    sidebar = {
                        card: false,
                        type: 'Share'
                    };
                }}
            >
                <ShareNodesOutline class="page__heading h-7 w-7 shrink-0" />
                <AnimatedUnderline --weight-underline="calc(var(--spacing) / 1.5)">
                    <span class="page__heading">{m.share()}</span>
                </AnimatedUnderline>
            </button>
        </div>
    {:else if sidebar.type === 'EditMetadata'}
        <EditMetadata {treeId} bind:prevTreeData {treeLinkedData} bind:sidebar />
    {:else if sidebar.type === 'ManageCoAuthors'}
        <CoAuthor {treeId} coAuthors={treeLinkedData.coAuthors} bind:sidebar />
    {:else if sidebar.type === 'Share'}
        <Share bind:sidebar />
    {:else if sidebar.card}
        <EditCard bind:sidebar {history} />
    {/if}
</div>
