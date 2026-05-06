<!--
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
-->

<script lang="ts">
    import { onMount } from 'svelte';
    import ImageUploadButton from './ImageUploadButton.svelte';

    let {
        maxLength,
        height,
        hasMenubar,
        valueText = $bindable()
    }: {
        maxLength: number;
        height: number | 'full';
        hasMenubar: boolean;
        valueText: string;
    } = $props();
    // We ignore these warnings here because these variables are not actually used
    // reactively, but only in event handlers.
    // svelte-ignore non_reactive_update
    let input: HTMLTextAreaElement;

    onMount(() => {
        input.focus();
    });
</script>

<!-- Menubar lookalike -->
{#if hasMenubar}
    <div class="flex justify-end gap-2 bg-blue-200">
        <ImageUploadButton bind:valueText bind:input></ImageUploadButton>
    </div>
{/if}
<textarea
    class="w-full resize-none border"
    class:h-full={height === 'full'}
    rows={height !== 'full' ? height : null}
    maxlength={maxLength}
    bind:value={valueText}
    bind:this={input}
></textarea>
<p class="text-xs">{valueText.length}/{maxLength}</p>
