<!--
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
-->

<!--component
    This simple component is used for editing the front of the card,
    or for the back when in 'code editor' instead of 'visual editor'.
    For the back of the card, see '$lib/rich-text/Editor.svelte'
 -->

<script lang="ts">
    let {
        maxLength,
        height,
        valueText = $bindable()
    }: {
        maxLength: number;
        height: number | 'full';
        valueText: string;
    } = $props();
</script>

<textarea
    class="w-full resize-none border"
    class:h-full={height === 'full'}
    rows={height !== 'full' ? height : null}
    maxlength={maxLength}
    bind:value={valueText}
></textarea>
<p class="text-xs">{valueText.length}/{maxLength}</p>
