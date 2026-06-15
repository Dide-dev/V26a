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
    import type { EventHandler } from 'svelte/elements';
    import type { Notice } from '$lib/Notices.svelte';

    let {
        sidebar = $bindable(),
        history,
        hasOwnerPermissions = true,
        treeId,
        prevTreeData = $bindable(),
        notices = $bindable(),
        treeLinkedData,
        onmouseenter,
        onmouseleave
    }: {
        sidebar: SideBar;
        history: TreeHistory;
        hasOwnerPermissions?: boolean;
        treeId: number;
        prevTreeData: TreeData;
        notices: Notice[];
        treeLinkedData: TreeLinkedData;
        onmouseenter: EventHandler;
        onmouseleave: EventHandler;
    } = $props();

    export type SideBar = MenuSideBar | CardEditSideBar | ManageSideBar;

    export type MenuSideBar = {
        kind: 'Menu';
        type: 'IconsOnly' | 'FullMenu';
    };

    export type CardEditSideBar = {
        kind: 'CardEditor';
        type: 'NewCard' | 'EditCard';
        location: number[];
        editingVersion: CardData;
        errorMessage?: string;
    };

    export type ManageSideBar = {
        kind: 'Manager';
        type: 'EditMetadata' | 'ManageCoAuthors' | 'Share';
    };
    /** Most recently recorded scroll position on window. */
    let lastScrollPosition = $state(0);
    /** Margin between nav-bar and the top of the screen in pixels. When 0, nav-bar is fully visible. */
    let topScrollMargin = $state(0);
    /** Height of a single row in the nav-bar in pixels. */
    const navRowHeight = 40;
    /** Default scroll distance in browser in pixels.*/
    const defaultScrollDistance = 100;
    /** Maximum margin between nav-bar and the top of the screen in pixels. */
    const maxScrollMargin = navRowHeight + defaultScrollDistance;

    let offset = $state(window.scrollY);
</script>

<svelte:window
    onscroll={() => {
        // We calculate the margin of the nav-bar, to account for it in the sidebar.
        topScrollMargin = Math.min(
            0,
            Math.max(-maxScrollMargin, topScrollMargin + lastScrollPosition - window.scrollY)
        );
        lastScrollPosition = window.scrollY;

        offset = Math.max(topScrollMargin, -navRowHeight) + navRowHeight;
    }}
    onkeydown={(e) => {
        if (e.key === 'Escape') {
            sidebar = {
                kind: 'Menu',
                type: 'IconsOnly'
            };
        }
    }}
/>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<div
    class={[
        sidebar.kind !== 'Menu' && 'pl-4',
        'relative flex h-full rounded-bl-2xl bg-neutral-100 px-2 pb-4 shadow-[-5px_0px_20px_0px_rgba(0_0_0/0.2)]'
    ]}
    {onmouseenter}
    {onmouseleave}
>
    <div class={['sticky h-fit w-full']} style="top: {offset}px">
        {#if sidebar.kind === 'Menu'}
            <div
                class="mt-2 flex flex-col gap-1 *:ml-2 *:flex *:cursor-pointer *:flex-row *:items-center *:gap-2 *:rounded-md *:pb-2 *:pl-2 *:hover:bg-neutral-200"
            >
                <button
                    type="button"
                    class="overflow-hidden"
                    onclick={() => {
                        sidebar = {
                            kind: 'Manager',
                            type: 'EditMetadata'
                        };
                    }}
                    data-testid="metadata-button"
                >
                    <CellAttributesOutline
                        class={[
                            sidebar.type === 'IconsOnly' && 'mt-3 mb-1',
                            'page__heading h-7 w-7 shrink-0'
                        ]}
                    />
                    {#if sidebar.type === 'FullMenu'}
                        <AnimatedUnderline --weight-underline="calc(var(--spacing) / 1.5)">
                            <span class="page__heading text-nowrap">{m.editMetadata()}</span>
                        </AnimatedUnderline>
                    {/if}
                </button>
                {#if hasOwnerPermissions}
                    <button
                        type="button"
                        class="overflow-hidden"
                        onclick={() => {
                            sidebar = {
                                kind: 'Manager',
                                type: 'ManageCoAuthors'
                            };
                        }}
                    >
                        <UserSettingsOutline
                            class={[
                                sidebar.type === 'IconsOnly' && 'mt-3 mb-1',
                                'page__heading h-7 w-7 shrink-0'
                            ]}
                        />
                        {#if sidebar.type === 'FullMenu'}
                            <AnimatedUnderline --weight-underline="calc(var(--spacing) / 1.5)">
                                <span class="page__heading text-nowrap">{m.coAuthors()}</span>
                            </AnimatedUnderline>
                        {/if}
                    </button>
                {/if}
                <button
                    type="button"
                    class="overflow-hidden"
                    onclick={() => {
                        sidebar = {
                            kind: 'Manager',
                            type: 'Share'
                        };
                    }}
                >
                    <ShareNodesOutline
                        class={[
                            sidebar.type === 'IconsOnly' && 'mt-3 mb-1',
                            'page__heading h-7 w-7 shrink-0'
                        ]}
                    />
                    {#if sidebar.type === 'FullMenu'}
                        <AnimatedUnderline --weight-underline="calc(var(--spacing) / 1.5)">
                            <span class="page__heading text-nowrap">{m.share()}</span>
                        </AnimatedUnderline>
                    {/if}
                </button>
            </div>
        {:else if sidebar.type === 'EditMetadata'}
            <EditMetadata {treeId} bind:prevTreeData {treeLinkedData} bind:sidebar />
        {:else if sidebar.type === 'ManageCoAuthors'}
            <CoAuthor {treeId} coAuthors={treeLinkedData.coAuthors} bind:sidebar />
        {:else if sidebar.type === 'Share'}
            <Share {treeId} bind:prevTreeData bind:sidebar bind:notices />
        {:else if sidebar.kind === 'CardEditor'}
            <EditCard bind:sidebar {history} />
        {/if}
    </div>
</div>
