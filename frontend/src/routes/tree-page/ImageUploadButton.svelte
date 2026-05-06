<!--
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
-->

<script lang="ts">
    import { uploadImage } from '$lib/backend/images';
    import { m } from '$lib/paraglide/messages';

    let {
        valueText = $bindable(),
        input = $bindable()
    }: { valueText: string; input: HTMLTextAreaElement } = $props();

    const validImageTypes = ['images/png', 'image/jpeg', 'image/avif', 'image/gif', 'image/webp'];

    let imageInput: HTMLInputElement;

    async function handleImageUpload(file: File) {
        const url = await uploadImage(file);

        const markdown = '![' + file.name + '](' + url + ')';
        const end = input.selectionEnd;
        valueText = valueText.slice(0, end) + markdown + valueText.slice(end);
    }
    async function promptImageUpload() {
        if (imageInput.files!.length !== 1) {
            return;
        }
        await handleImageUpload(imageInput.files![0]);
    }
</script>

<input
    bind:this={imageInput}
    class="hidden"
    type="file"
    id="image-upload"
    accept={validImageTypes.join(',')}
    onchange={promptImageUpload}
/>
<label for="image-upload" class="mr-2 cursor-pointer">{m.uploadImage()}</label>
