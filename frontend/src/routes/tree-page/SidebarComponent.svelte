<!--
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
-->

<!--component
    This sidebar is currently only used for the card editing, but could be used for more later.
 -->
<script lang="ts">
    import type { CardGetData } from '$lib/backend/cards';
    import { m } from '$lib/paraglide/messages';
    import { doAddCard, doEditCard, followIndexNode, type TreeHistory } from './history.svelte.ts';
    import EditAreaComponent from './EditAreaComponent.svelte';

    let {
        sidebar = $bindable(),
        history
    }: {
        sidebar: SideBar | null;
        history: TreeHistory;
    } = $props();

    export type SideBar = {
        type: 'NewCard' | 'EditCard';
        location: number[];
        editingVersion: CardGetData;
        errorMessage?: string;
    };

    const frontMaxLength = 250;
    const backMaxLength = 5000;

    async function finishEdit() {
        let node = followIndexNode(history.currentStructure, sidebar!.location);
        await doEditCard(history, node.cardId, sidebar!.editingVersion);
        sidebar = null;
    }

    async function finishAdd() {
        await doAddCard(history, sidebar!.location, sidebar!.editingVersion);
        sidebar = null;
    }

    function confirmClick() {
        if (sidebar === null) {
            return;
        }
        sidebar.editingVersion.front = sidebar.editingVersion.front.trim();
        sidebar.editingVersion.back = sidebar.editingVersion.back.trim();

        if (sidebar.editingVersion.back.length === 0) {
            sidebar.errorMessage = m.backTextRequired();
            return;
        }

        if (sidebar.editingVersion.front.length > frontMaxLength) {
            sidebar.errorMessage = m.frontTextCharacterLimit({ limit: frontMaxLength });
            return;
        }
        if (sidebar.editingVersion.back.length > backMaxLength) {
            sidebar.errorMessage = m.backTextCharacterLimit({ limit: backMaxLength });
            return;
        }
        sidebar.errorMessage = '';

        if (sidebar.type === 'NewCard') {
            finishAdd();
        } else {
            finishEdit();
        }
    }

    export const fullwidth: number = 350;
</script>

{#if sidebar !== null}
    <div class="pointer-events-none absolute top-0 right-0 h-full" style="width: {fullwidth}px;">
        <div
            class="pointer-events-auto sticky top-0 z-50 flex w-full flex-col gap-2 bg-blue-50 p-2 shadow-[-5px_5px_5px_-5px_rgba(0_0_0/0.8)]"
        >
            <h1>{m.editThisCard()}</h1>

            <p>Front</p>
            <EditAreaComponent maxLength={frontMaxLength} height={2} hasMenubar={false} bind:valueText={sidebar.editingVersion.front}></EditAreaComponent>
            <p>Back</p>
            <EditAreaComponent maxLength={backMaxLength} height={10} hasMenubar={true} bind:valueText={sidebar.editingVersion.back}></EditAreaComponent>
            {#if sidebar.errorMessage}
                <p class="text-red-600">{sidebar.errorMessage}</p>
            {/if}
            <div class="flex gap-2">
                <button class="cursor-pointer" onclick={() => (sidebar = null)}>
                    {m.cancel()}
                </button>
                <button class="cursor-pointer" onclick={confirmClick}>
                    {m.save()}
                </button>
            </div>
        </div>
    </div>
{/if}
