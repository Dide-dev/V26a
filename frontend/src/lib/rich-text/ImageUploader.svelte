<!--
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
-->

<script lang="ts">
    import type { Uploader } from 'prosekit/extensions/file';
    import type { ImageExtension } from 'prosekit/extensions/image';
    import { useEditor } from 'prosekit/svelte';
    import {
        PopoverPopup,
        PopoverPositioner,
        PopoverRoot,
        PopoverTrigger
    } from 'prosekit/svelte/popover';
    import { m } from '$lib/paraglide/messages';

    import ToolbarButton from './ToolbarButton.svelte';
    import { uploadImage } from '$lib/api-wrapper/images';

    let {
        tooltip,
        disabled,
        children
    }: {
        tooltip: string;
        disabled: boolean;
        children?: import('svelte').Snippet;
    } = $props();

    const validImageTypes = ['images/png', 'image/jpeg', 'image/avif', 'image/gif', 'image/webp'];

    let open = $state(false);
    let file = $state<File | null>(null);
    let fileInput: HTMLInputElement;
    const ariaId = $props.id();

    const editor = useEditor<ImageExtension>();

    function handleFileChange() {
        file = fileInput.files?.[0] ?? null;
    }

    function handleSubmit() {
        if (file !== null) {
            let uploader: Uploader<string> = async (options) => {
                return await uploadImage(options.file);
            };
            $editor.commands.uploadImage({ file, uploader });
        }
        open = false;
        file = null;
        fileInput.value = '';
    }
</script>

<PopoverRoot {open} onOpenChange={(event) => (open = event.detail)}>
    <PopoverTrigger>
        <ToolbarButton pressed={open} {disabled} {tooltip}>
            {@render children?.()}
        </ToolbarButton>
    </PopoverTrigger>

    <PopoverPositioner placement="bottom" class="z-50 block h-min w-min overflow-visible">
        <PopoverPopup
            class="box-border flex w-sm origin-(--transform-origin) flex-col gap-y-4 rounded-xl border border-gray-200 bg-[canvas] p-6 text-sm shadow-lg transition-[opacity,scale] transition-discrete duration-40 data-[state=closed]:scale-95 data-[state=closed]:opacity-0 data-[state=closed]:duration-150 motion-reduce:transition-none dark:border-gray-800 starting:scale-95 starting:opacity-0"
        >
            <label for="id-upload-{ariaId}">{m.upload()}</label>
            <input
                id="id-upload-{ariaId}"
                bind:this={fileInput}
                class="flex w-full rounded-md border border-solid border-gray-200 bg-[canvas] text-sm outline-hidden"
                accept={validImageTypes.join(',')}
                type="file"
                onchange={handleFileChange}
            />

            {#if file !== null}
                <button
                    class="inline-flex h-10 w-full items-center justify-center rounded-md border-0 bg-gray-900 px-4 py-2 text-sm font-medium whitespace-nowrap text-gray-50 ring-offset-white transition-colors hover:bg-gray-900/90 focus-visible:ring-2 focus-visible:ring-gray-900 focus-visible:ring-offset-2 focus-visible:outline-hidden disabled:pointer-events-none disabled:opacity-50 dark:bg-gray-50 dark:text-gray-900 dark:ring-offset-gray-950 dark:hover:bg-gray-50/90 dark:focus-visible:ring-gray-300"
                    onclick={handleSubmit}
                >
                    {m.uploadImage()}
                </button>
            {/if}
        </PopoverPopup>
    </PopoverPositioner>
</PopoverRoot>
