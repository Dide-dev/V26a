<!--
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
-->

<script lang="ts">
    import { m } from '$lib/paraglide/messages';
    import { onMount } from 'svelte';

    let {
        text = $bindable(),
        editDoneCallback,
        editCancelCallback
    }: {
        text: string;
        editDoneCallback: () => void;
        editCancelCallback: () => void;
    } = $props();

    let textareaElement: HTMLTextAreaElement;
    function editFinish() {
        text = text.trim();
        if (text.length === 0) {
            editCancelCallback();
        } else {
            editDoneCallback();
        }
    }
    function editKeydown(event: KeyboardEvent) {
        // Prevent ctrl+z from undoing last full tree action,
        // instead only undoing anything in this textarea.
        event.stopPropagation();

        // We don't want newlines in the rootnodes.
        // Instead we will just save it for the user
        if (event.key === 'Enter') {
            event.preventDefault();
            editFinish();
        }
    }

    onMount(() => {
        textareaElement.focus();
    });
</script>

<div class="mb-5 flex w-32 flex-col gap-1">
    <textarea
        class="h-fit shrink-0 p-0 font-bold"
        rows="3"
        bind:this={textareaElement}
        bind:value={text}
        onkeydown={editKeydown}
        onfocusout={() => {
            // In firefox this function does not get called when this component is destroyed,
            // but in chromium it does get called... So we manually check here just in case.
            // (This only occurs when pressing enter to save, instead of using the onfocusout directly)
            if (text === null) return;

            editFinish();
        }}
        maxlength="48"></textarea>
    <button class="cursor-pointer rounded bg-blue-500 px-3 py-1 text-white">{m.save()}</button>
</div>
