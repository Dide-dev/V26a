<!--
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
-->

<!--component
    This sidebar is currently only used for the card editing, but could be used for more later.
 -->
<script lang="ts">
    import type { CardData } from '$lib/api-wrapper/cards';
    import { m } from '$lib/paraglide/messages';
    import Editor from '$lib/rich-text/Editor.svelte';
    import { doAddCard, doEditCard, followIndexNode, type TreeHistory } from './history.svelte.ts';
    import SimpleTextEditArea from './SimpleTextEditArea.svelte';

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
        editingVersion: CardData;
        errorMessage?: string;
    };

    const maxInputLengths = {
        frontMaxLength: 250,
        backMaxLength: 5000
    };

    let isVisualEditor = $state(true);

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

        if (sidebar.editingVersion.back.length === 0) {
            sidebar.errorMessage = m.backTextRequired();
            return;
        }

        if (sidebar.editingVersion.front.length > maxInputLengths.frontMaxLength) {
            sidebar.errorMessage = m.frontTextCharacterLimit({
                limit: maxInputLengths.frontMaxLength
            });
            return;
        }
        if (sidebar.editingVersion.back.length > maxInputLengths.backMaxLength) {
            sidebar.errorMessage = m.backTextCharacterLimit({
                limit: maxInputLengths.backMaxLength
            });
            return;
        }
        sidebar.errorMessage = '';

        if (sidebar.type === 'NewCard') {
            finishAdd();
        } else {
            finishEdit();
        }
    }

    // Prevent any ctrl+z or the like in the sidebar from undo-ing actions in the tree.
    function onkeydown(event: KeyboardEvent) {
        event.stopPropagation();

        if (event.ctrlKey && event.key == 'Enter') {
            confirmClick();
        }
    }

    export const htmlWidth: number = 600;
    export const htmlHeight: number = 650;
</script>

{#if sidebar !== null}
    <div style="width: {htmlWidth}px;" class="pointer-events-none absolute -top-12 right-0 h-full">
        <!-- svelte-ignore a11y_no_static_element_interactions -->
        <div
            style="height: {htmlHeight}px;"
            class="pointer-events-auto sticky top-0 z-30 flex w-full flex-col bg-blue-50 p-2 shadow-[-5px_5px_5px_-5px_rgba(0_0_0/0.8)]"
            {onkeydown}
        >
            <h1>{m.editThisCard()}</h1>

            <p>Front</p>
            <SimpleTextEditArea
                maxLength={maxInputLengths.frontMaxLength}
                height={2}
                bind:valueText={sidebar.editingVersion.front}
            />
            <div class="h-5"></div>
            <p>Back</p>
            <div class="m-1 flex gap-2 rounded bg-white p-1">
                <button
                    class="cursor-pointer rounded p-1"
                    class:bg-green-500={isVisualEditor === true}
                    onclick={() => (isVisualEditor = true)}
                >
                    {m.visualEditor()}
                </button>
                <button
                    class="cursor-pointer rounded p-1"
                    class:bg-green-500={isVisualEditor === false}
                    onclick={() => (isVisualEditor = false)}
                >
                    {m.codeEditor()}
                </button>
            </div>
            {#if isVisualEditor}
                <Editor
                    maxLength={maxInputLengths.backMaxLength}
                    bind:valueText={sidebar.editingVersion.back}
                />
            {:else}
                <SimpleTextEditArea
                    maxLength={maxInputLengths.backMaxLength}
                    height="full"
                    bind:valueText={sidebar.editingVersion.back}
                />
            {/if}
            <div class="h-5"></div>
            {#if sidebar.errorMessage}
                <p class="text-red-600">{sidebar.errorMessage}</p>
            {/if}
            <div class="flex gap-2">
                <button
                    class="cursor-pointer rounded bg-red-300 p-1"
                    onclick={() => (sidebar = null)}
                >
                    {m.cancel()}
                </button>
                <button class="cursor-pointer rounded bg-green-300 p-1" onclick={confirmClick}>
                    {m.save()}
                </button>
            </div>
        </div>
    </div>
{/if}
