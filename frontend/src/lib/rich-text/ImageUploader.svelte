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
    import type { OpenChangeEvent } from 'prosekit/web/popover';

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

    let open = $state(false);
    let url = $state('');
    let file = $state<File | null>(null);
    const ariaId = $props.id();

    const editor = useEditor<ImageExtension>();

    function handleFileChange(event: Event) {
        const target = event.target as HTMLInputElement;
        const selectedFile = target.files?.[0];

        if (selectedFile) {
            file = selectedFile;
            url = '';
        } else {
            file = null;
        }
    }

    function handleUrlChange(event: Event) {
        const target = event.target as HTMLInputElement;
        const inputUrl = target.value;

        if (inputUrl) {
            url = inputUrl;
            file = null;
        } else {
            url = '';
        }
    }

    function deferResetState() {
        setTimeout(() => {
            url = '';
            file = null;
        }, 300);
    }

    function handleSubmit() {
        if (url) {
            $editor.commands.insertImage({ src: url });
        } else if (file) {
            let uploader: Uploader<string> = async (options) => {
                return await uploadImage(options.file);
            };
            $editor.commands.uploadImage({ file, uploader });
        }
        open = false;
        deferResetState();
    }

    function handleOpenChange(event: OpenChangeEvent) {
        if (!event.detail) {
            deferResetState();
        }
        open = event.detail;
    }
</script>

<PopoverRoot {open} onOpenChange={handleOpenChange}>
    <PopoverTrigger>
        <ToolbarButton pressed={open} {disabled} {tooltip}>
            {@render children?.()}
        </ToolbarButton>
    </PopoverTrigger>

    <PopoverPositioner
        placement="bottom"
        class="z-50 block h-min w-min overflow-visible transition-transform duration-100 ease-out motion-reduce:transition-none"
    >
        <PopoverPopup
            class="box-border flex w-sm origin-(--transform-origin) flex-col gap-y-4 rounded-xl border border-gray-200 bg-[canvas] p-6 text-sm shadow-lg transition-[opacity,scale] transition-discrete duration-40 data-[state=closed]:scale-95 data-[state=closed]:opacity-0 data-[state=closed]:duration-150 motion-reduce:transition-none dark:border-gray-800 starting:scale-95 starting:opacity-0"
        >
            {#if !file}
                <label for="id-link-{ariaId}">Embed Link</label>
                <input
                    id="id-link-{ariaId}"
                    class="box-border flex h-9 w-full rounded-md border border-solid border-gray-200 bg-[canvas] px-3 py-2 text-sm ring-0 ring-transparent outline-hidden transition file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-gray-500 focus-visible:ring-2 focus-visible:ring-gray-900 focus-visible:ring-offset-0 focus-visible:outline-hidden disabled:cursor-not-allowed disabled:opacity-50 dark:border-gray-800 dark:placeholder:text-gray-500 dark:focus-visible:ring-gray-300"
                    placeholder="Paste the image link..."
                    type="url"
                    value={url}
                    oninput={handleUrlChange}
                />
            {/if}

            {#if !url}
                <label for="id-upload-{ariaId}">Upload</label>
                <input
                    id="id-upload-{ariaId}"
                    class="box-border flex h-9 w-full rounded-md border border-solid border-gray-200 bg-[canvas] px-3 py-2 text-sm ring-0 ring-transparent outline-hidden transition file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-gray-500 focus-visible:ring-2 focus-visible:ring-gray-900 focus-visible:ring-offset-0 focus-visible:outline-hidden disabled:cursor-not-allowed disabled:opacity-50 dark:border-gray-800 dark:placeholder:text-gray-500 dark:focus-visible:ring-gray-300"
                    accept="image/*"
                    type="file"
                    onchange={handleFileChange}
                />
            {/if}

            {#if url}
                <button
                    class="inline-flex h-10 w-full items-center justify-center rounded-md border-0 bg-gray-900 px-4 py-2 text-sm font-medium whitespace-nowrap text-gray-50 ring-offset-white transition-colors hover:bg-gray-900/90 focus-visible:ring-2 focus-visible:ring-gray-900 focus-visible:ring-offset-2 focus-visible:outline-hidden disabled:pointer-events-none disabled:opacity-50 dark:bg-gray-50 dark:text-gray-900 dark:ring-offset-gray-950 dark:hover:bg-gray-50/90 dark:focus-visible:ring-gray-300"
                    onclick={handleSubmit}
                >
                    Insert Image
                </button>
            {/if}

            {#if file}
                <button
                    class="inline-flex h-10 w-full items-center justify-center rounded-md border-0 bg-gray-900 px-4 py-2 text-sm font-medium whitespace-nowrap text-gray-50 ring-offset-white transition-colors hover:bg-gray-900/90 focus-visible:ring-2 focus-visible:ring-gray-900 focus-visible:ring-offset-2 focus-visible:outline-hidden disabled:pointer-events-none disabled:opacity-50 dark:bg-gray-50 dark:text-gray-900 dark:ring-offset-gray-950 dark:hover:bg-gray-50/90 dark:focus-visible:ring-gray-300"
                    onclick={handleSubmit}
                >
                    Upload Image
                </button>
            {/if}
        </PopoverPopup>
    </PopoverPositioner>
</PopoverRoot>
