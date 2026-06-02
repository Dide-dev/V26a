<!--
    This program has been developed by students from the bachelor Computer Science at Utrecht
    University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
-->

<script lang="ts">
    import {
        TooltipPopup,
        TooltipPositioner,
        TooltipRoot,
        TooltipTrigger
    } from 'prosekit/svelte/tooltip';

    const {
        pressed = false,
        disabled = false,
        onClick,
        tooltip,
        children
    }: {
        pressed?: boolean;
        disabled?: boolean;
        onClick?: () => void;
        tooltip?: string;
        children?: import('svelte').Snippet;
    } = $props();
</script>

<TooltipRoot>
    <TooltipTrigger class="block">
        <button
            data-state={pressed ? 'on' : 'off'}
            {disabled}
            type="button"
            class="outline-unset focus-visible:outline-unset flex min-h-8 min-w-8 items-center justify-center rounded-md bg-transparent p-1 text-sm font-medium text-gray-900 transition hover:bg-gray-100 focus-visible:ring-2 focus-visible:ring-gray-900 disabled:pointer-events-none disabled:text-gray-900/50 data-[state=on]:bg-gray-200 dark:text-gray-50 dark:hover:bg-gray-800 dark:focus-visible:ring-gray-300 dark:disabled:text-gray-50/50 dark:data-[state=on]:bg-gray-700"
            onclick={onClick}
            onmousedown={(e) => e.preventDefault()}
        >
            {@render children?.()}
            {#if tooltip}
                <span class="sr-only">{tooltip}</span>
            {/if}
        </button>
    </TooltipTrigger>
    {#if tooltip}
        <TooltipPositioner class="z-50 block h-min w-min overflow-visible">
            <TooltipPopup
                class="box-border flex origin-(--transform-origin) overflow-hidden rounded-md border border-solid bg-gray-900 px-3 py-1.5 text-xs text-nowrap text-gray-50 shadow-xs transition-[opacity,scale] transition-discrete duration-100 data-[state=closed]:scale-95 data-[state=closed]:opacity-0 data-[state=closed]:duration-150 motion-reduce:transition-none dark:bg-gray-50 dark:text-gray-900 starting:scale-95 starting:opacity-0"
            >
                {tooltip}
            </TooltipPopup>
        </TooltipPositioner>
    {/if}
</TooltipRoot>
