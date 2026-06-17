<!--
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
-->

<script lang="ts">
    import { m } from '$lib/paraglide/messages';
    import { doAddCard, doEditCard, followIndexNode, type TreeHistory } from './history.svelte';
    import type { CardEditSideBar, MenuSideBar } from './Sidebar.svelte';
    import SimpleTextEditArea from './SimpleTextEditArea.svelte';
    import Editor from '$lib/rich-text/Editor.svelte';

    let {
        sidebar = $bindable(),
        history
    }: {
        sidebar: CardEditSideBar | MenuSideBar;
        history: TreeHistory;
    } = $props();

    let isVisualEditor = $state(true);

    let cardChanged = $derived.by(() => {
        if (sidebar.kind !== 'CardEditor') return false;

        if (sidebar.editingVersion.back.length === 0) return false;

        if (sidebar.type === 'NewCard') return true;

        let node = followIndexNode(history.currentStructure, sidebar.location);
        let frontChanged = history.currentCards[node.cardId].front !== sidebar.editingVersion.front;
        let backChanged = history.currentCards[node.cardId].back !== sidebar.editingVersion.back;

        return frontChanged || backChanged;
    });

    const maxInputLengths = {
        frontMaxLength: 250,
        backMaxLength: 5000
    };

    async function finishEdit() {
        if (sidebar.kind !== 'CardEditor') return;

        let node = followIndexNode(history.currentStructure, sidebar.location);
        await doEditCard(history, node.cardId, sidebar.editingVersion);
        sidebar = {
            kind: 'Menu',
            type: 'IconsOnly'
        };
    }

    async function finishAdd() {
        if (sidebar.kind !== 'CardEditor') return;

        await doAddCard(history, sidebar.location, sidebar.editingVersion);
        sidebar = {
            kind: 'Menu',
            type: 'IconsOnly'
        };
    }

    function confirmClick(e: Event) {
        e.preventDefault();

        if (!cardChanged || sidebar.kind !== 'CardEditor') {
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
        if (event.ctrlKey) event.stopPropagation();

        if (event.ctrlKey && event.key == 'Enter') {
            confirmClick(event);
        }
    }
</script>

<svelte:body {onkeydown} />

{#if sidebar.kind === 'CardEditor'}
    <form class={[cardChanged && 'completed', 'pointer-events-auto flex w-full flex-col']}>
        <h2 class="page__heading">{m.editThisCard()}</h2>

        <div>
            <label for="front-text">{m.front()}</label>
            <SimpleTextEditArea
                id="front-text"
                maxLength={maxInputLengths.frontMaxLength}
                height={2}
                bind:valueText={sidebar.editingVersion.front}
                placeholder={m.enterFrontText()}
            />
        </div>
        <div class="flex gap-2">
            <label for="editor-toggle">{m.back()}</label>
            <div id="editor-toggle" class="grid grid-cols-2 rounded-md bg-white">
                <button
                    type="button"
                    class={[
                        isVisualEditor && 'cursor-not-allowed bg-tree-300 font-semibold',
                        !isVisualEditor && 'cursor-pointer',
                        'rounded-md p-1 transition-all duration-200 ease-in-out'
                    ]}
                    onclick={() => (isVisualEditor = true)}
                >
                    {m.visualEditor()}
                </button>
                <button
                    type="button"
                    class={[
                        isVisualEditor && 'cursor-pointer',
                        !isVisualEditor && 'cursor-not-allowed bg-tree-300 font-semibold',
                        'rounded-md p-1 transition-all duration-200 ease-in-out'
                    ]}
                    onclick={() => (isVisualEditor = false)}
                >
                    {m.codeEditor()}
                </button>
            </div>
        </div>
        {#if isVisualEditor}
            <Editor
                maxLength={maxInputLengths.backMaxLength}
                bind:valueText={sidebar.editingVersion.back}
            />
        {:else}
            <SimpleTextEditArea
                id="back-text"
                maxLength={maxInputLengths.backMaxLength}
                height="full"
                bind:valueText={sidebar.editingVersion.back}
            />
        {/if}
        {#if sidebar.errorMessage}
            <p class="text-red-600">{sidebar.errorMessage}</p>
        {/if}
        <div class="grid grid-cols-2 gap-2">
            <button
                type="reset"
                onclick={() =>
                    (sidebar = {
                        kind: 'Menu',
                        type: 'IconsOnly'
                    })}
            >
                {m.cancel()}
            </button>
            <button type="submit" onclick={confirmClick}>
                {m.save()}
            </button>
        </div>
    </form>
{/if}
