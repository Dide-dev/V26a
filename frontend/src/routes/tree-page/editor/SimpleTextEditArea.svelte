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
        id,
        maxLength,
        height,
        valueText = $bindable(),
        placeholder = 'Enter text...'
    }: {
        id: string;
        maxLength: number;
        height: number | 'full';
        valueText: string;
        placeholder?: string;
    } = $props();
</script>

<textarea
    {id}
    class="input__long_text"
    class:h-full={height === 'full'}
    rows={height !== 'full' ? height : null}
    maxlength={maxLength}
    bind:value={valueText}
    {placeholder}
></textarea>
<p class="text-xs">{valueText.length}/{maxLength}</p>
