<!--
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
-->

<!--component
    This component is intended to be under all the rootnodes,
    shows an add button that will turn into a textarea once clicked.
 -->

<script lang="ts">
    import iconAdd from '$lib/assets/icons/add.svg?raw';
    import { doRootNodeAdd, type TreeHistory } from './history.svelte';
    import RootNodeTextarea from './RootNodeTextarea.svelte';

    let {
        history,
        /**
         * When the user is dragging cards, this component should not light up on hover,
         * as dropping it does not do anything here.
         */
        enabled
    }: {
        history: TreeHistory;
        enabled: boolean;
    } = $props();

    /**
     * If this is null no editing box is shown.
     * If not null, contains the currently being edited label text.
     */
    let text: null | string = $state(null);

    function onclick() {
        text = '';
    }
    async function editDoneCallback() {
        if (text === null) return;

        const index = history.currentStructure.rootNodes.length;
        await doRootNodeAdd(history, index, text);
        text = null;
    }
    function editCancelCallback() {
        text = null;
    }
</script>

<div class="h-5"></div>

{#if text === null}
    <button
        class={[
            'flex w-32 cursor-pointer items-center justify-center border-2 border-dashed',
            'border-gray-300  fill-gray-300',
            enabled && 'hover:border-gray-800 hover:fill-gray-800',
            !enabled && '*:invisible'
        ]}
        {onclick}
    >
        <!-- eslint-disable-next-line svelte/no-at-html-tags -->
        {@html iconAdd}
    </button>
{:else}
    <RootNodeTextarea bind:text {editDoneCallback} {editCancelCallback} />
{/if}
