<!--
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
-->

<script lang="ts">
    import { onMount } from 'svelte';

    let {
        text = $bindable(),
        numPopupsOpen = $bindable(),
        editDoneCallback,
        editCancelCallback
    }: {
        text: string;
        numPopupsOpen: number;
        editDoneCallback: () => void;
        editCancelCallback: () => void;
    } = $props();

    let textareaElement: HTMLTextAreaElement;
    async function editFinish() {
        if (text.length === 0) {
            editCancelCallback();
        } else {
            editDoneCallback();
        }
    }
    async function editKeypress(event: KeyboardEvent) {
        if (event.key === 'Enter') {
            event.preventDefault();
            event.stopPropagation();
            editFinish();
        }
    }

    onMount(() => {
        numPopupsOpen += 1;
        textareaElement.focus();

        return () => (numPopupsOpen -= 1);
    });
</script>

<textarea
    class="mb-5 h-fit w-32 shrink-0 p-0 font-bold"
    rows="3"
    bind:this={textareaElement}
    bind:value={text}
    onkeypress={editKeypress}
    onfocusout={editFinish}
    maxlength="48"
></textarea>
