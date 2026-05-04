<!--
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
-->

<script lang="ts">

    import ImageUploadButton from "./ImageUploadButton.svelte";

    let { maxLength, height, hasMenubar, valueText = $bindable() }: { maxLength: number; height: number; hasMenubar: boolean; valueText: string; } = $props();
    // We ignore these warnings here because these variables are not actually used
    // reactively, but only in event handlers.
    // svelte-ignore non_reactive_update
    let input: HTMLTextAreaElement;

</script>


<div class="w-full">
    <!-- Menubar lookalike -->
    {#if hasMenubar}
        <div class="flex justify-end gap-2">
            <ImageUploadButton bind:valueText={valueText} bind:input={input} ></ImageUploadButton>
        </div>
    {/if}
    <textarea
        class="border w-full"
        rows={height}
        maxlength={maxLength}
        bind:value={valueText}
        bind:this={input}
    ></textarea>
    <p class="text-xs">{valueText.length}/{maxLength}</p>
</div>
