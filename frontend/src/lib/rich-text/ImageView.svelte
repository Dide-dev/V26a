<script lang="ts">
    import type { ProseMirrorNode } from 'prosekit/pm/model';
    import { UploadTask } from 'prosekit/extensions/file';
    import type { ImageAttrs } from './imageProsekit';
    import type { SvelteNodeViewProps } from 'prosekit/svelte';
    import { ResizableHandle, ResizableRoot } from 'prosekit/svelte/resizable';
    import { fromStore } from 'svelte/store';

    type Props = SvelteNodeViewProps;

    const props: Props = $props();
    const node: ProseMirrorNode = $derived(fromStore(props.node).current);
    const selected: boolean = $derived(fromStore(props.selected).current);

    const attrs = $derived(node.attrs as ImageAttrs);
    const url = $derived(attrs.src || '');
    const uploading = $derived(url.startsWith('blob:'));

    let aspectRatio = $state<number | undefined>(undefined);
    let error = $state<string | undefined>(undefined);
    let progress = $state(0);

    $effect(() => {
        if (!uploading) {
            return;
        }

        const uploadTask = UploadTask.get<string>(url);
        if (!uploadTask) return;

        let canceled = false;

        uploadTask.finished.catch((err) => {
            if (canceled) return;
            error = String(err);
        });
        const unsubscribeProgress = uploadTask.subscribeProgress(({ loaded, total }) => {
            if (canceled) return;
            progress = total ? loaded / total : 0;
        });

        return () => {
            canceled = true;
            unsubscribeProgress();
        };
    });

    function handleImageLoad(event: Event) {
        const img = event.target as HTMLImageElement;
        const { naturalWidth, naturalHeight } = img;
        const ratio = naturalWidth / naturalHeight;
        if (ratio && Number.isFinite(ratio)) {
            aspectRatio = ratio;
        }
        if (naturalWidth && naturalHeight && (!attrs.width || !attrs.height)) {
            props.setAttrs({ width: naturalWidth, height: naturalHeight });
        }
    }
</script>

<ResizableRoot
    width={attrs.width ?? undefined}
    height={attrs.height ?? undefined}
    {aspectRatio}
    data-selected={selected ? '' : undefined}
    class="group relative my-2 box-border flex max-h-150 min-h-16 max-w-full min-w-16 items-center justify-center overflow-hidden outline-2 outline-transparent outline-solid data-selected:outline-blue-500"
    onResizeEnd={(event) => props.setAttrs(event.detail)}
>
    {#if url && !error}
        <img
            src={url}
            alt="upload preview"
            class="h-full max-h-full w-full max-w-full object-contain"
            onload={handleImageLoad}
        />
    {/if}
    {#if uploading && !error}
        <div
            class="absolute bottom-0 left-0 m-1 flex content-center items-center gap-2 rounded-sm bg-gray-800/60 p-1.5 text-xs text-white/80 transition"
        >
            <div class="i-lucide-loader-circle block size-4 animate-spin"></div>
            <div>{Math.round(progress * 100)}%</div>
        </div>
    {/if}
    {#if error}
        <div
            class="@container absolute top-0 right-0 bottom-0 left-0 flex flex-col items-center justify-center gap-4 bg-gray-200 p-2 text-sm dark:bg-gray-800"
        >
            <div class="i-lucide-image-off block size-8"></div>
            <div class="hidden opacity-80 @xs:block">Failed to upload image</div>
        </div>
    {/if}
    <ResizableHandle
        class="absolute right-0 bottom-0 m-1.5 rounded-sm bg-gray-900/30 p-1 text-white/50 opacity-0 transition group-hover:opacity-100 group-data-resizing:opacity-100 hover:bg-gray-800/60 hover:opacity-100 active:translate-x-0.5 active:translate-y-0.5 active:bg-gray-800/60 active:text-white/80"
        position="bottom-right"
    >
        <div class="i-lucide-move-diagonal-2 block size-4"></div>
    </ResizableHandle>
</ResizableRoot>
