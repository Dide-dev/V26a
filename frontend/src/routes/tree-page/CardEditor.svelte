<!--
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
-->

<!--component
    This is a popup with 2 text editors for the front & back of the card.
    This component will check for front & back length before the `confirmCallback` is called.
 -->
<script lang="ts">
    import type { CardGetData } from '$lib/backend/cards';
    import { uploadImage } from '$lib/backend/images';
    import { m } from '$lib/paraglide/messages';
    import Popup from '$lib/Popup.svelte';
    import { htmlFromMarkdown } from '$lib/markdown';

    let {
        card,
        /** This string must already be translated, as it is shown directly to the user. */
        confirmText,
        confirmCallback,
        cancelCallback
    }: {
        card: CardGetData;
        confirmText: string;
        confirmCallback: (cardNew: CardGetData) => void;
        cancelCallback: () => void;
    } = $props();

    const frontMaxLength = 1024;
    const backMaxLength = 5000;

    const validImageTypes = ['image/png', 'image/jpeg', 'image/avif', 'image/gif', 'image/webp'];

    let focusedField: 'front' | 'back' = $state('front');
    // This warning is ignored here because we just want to make a copy of card as a
    // new state variable, so that editing the text also updates the input fields.
    // svelte-ignore state_referenced_locally
    let cardData: CardGetData = $state(card);
    let errorMessage = $state('');
    let previewView = $state(false);

    let imageInput: HTMLInputElement;
    // We ignore these warnings here because these variables are not actually used
    // reactively, but only in event handlers.
    // svelte-ignore non_reactive_update
    let frontInput: HTMLTextAreaElement;
    // svelte-ignore non_reactive_update
    let backInput: HTMLTextAreaElement;

    async function handleImageUpload(file: File) {
        const url = await uploadImage(file);

        const markdown = '![' + file.name + '](' + url + ')';
        if (focusedField === 'front') {
            const end = frontInput.selectionEnd;
            cardData.front = cardData.front.slice(0, end) + markdown + cardData.front.slice(end);
        } else {
            const end = backInput.selectionEnd;
            cardData.back = cardData.back.slice(0, end) + markdown + cardData.back.slice(end);
        }
    }

    async function promptImageUpload() {
        if (imageInput.files!.length !== 1) {
            return;
        }
        await handleImageUpload(imageInput.files![0]);
    }

    function confirmClick() {
        cardData.front = cardData.front.trim();
        cardData.back = cardData.back.trim();

        if (cardData.back.length === 0) {
            errorMessage = m.backTextRequired();
            return;
        }

        if (cardData.front.length > frontMaxLength) {
            errorMessage = m.frontTextCharacterLimit({ limit: frontMaxLength });
            return;
        }
        if (cardData.back.length > backMaxLength) {
            errorMessage = m.backTextCharacterLimit({ limit: backMaxLength });
            return;
        }

        confirmCallback(cardData);
    }
</script>

<Popup>
    <div class="flex flex-col gap-2">
        <div class="flex justify-end gap-2">
            <form class={previewView ? 'hidden' : ''}>
                <input
                    bind:this={imageInput}
                    class="hidden"
                    type="file"
                    id="image-upload"
                    accept={validImageTypes.join(',')}
                    onchange={promptImageUpload}
                />
                <label for="image-upload" class="mr-2 cursor-pointer">{m.uploadImage()}</label>
            </form>
            <button
                class="cursor-pointer"
                onclick={() => {
                    previewView = !previewView;
                }}>{previewView ? 'Markdown' : m.preview()}</button
            >
        </div>
        {#if !previewView}
            <textarea
                class="w-xl rounded border p-1"
                placeholder={m.enterFrontText()}
                bind:this={frontInput}
                bind:value={cardData.front}
                onfocus={() => {
                    focusedField = 'front';
                }}
            ></textarea>
            <textarea
                class="w-xl rounded border p-1"
                placeholder={m.enterBackText()}
                bind:this={backInput}
                bind:value={cardData.back}
                onfocus={() => {
                    focusedField = 'back';
                }}
            ></textarea>
        {:else}
            <div class="w-xl rounded border p-1">
                <!-- eslint-disable-next-line svelte/no-at-html-tags -->
                {@html htmlFromMarkdown(cardData.front)}
            </div>
            <div class="w-xl rounded border p-1">
                <!-- eslint-disable-next-line svelte/no-at-html-tags -->
                {@html htmlFromMarkdown(cardData.back)}
            </div>
        {/if}
        {#if errorMessage}
            <p class="text-red-600">{errorMessage}</p>
        {/if}
        <div class="flex justify-end gap-2">
            <button class="cursor-pointer" onclick={cancelCallback}>
                {m.cancel()}
            </button>
            <button class="cursor-pointer" onclick={confirmClick}>
                {confirmText}
            </button>
        </div>
    </div>
</Popup>

<svelte:window
    ondragover={(e) => {
        const fileItems = [...e.dataTransfer!.items].filter((item) => item.kind === 'file');
        if (fileItems.length > 0) {
            e.preventDefault();
            if (!previewView && fileItems.some((item) => validImageTypes.includes(item.type))) {
                e.dataTransfer!.dropEffect = 'copy';
            } else {
                e.dataTransfer!.dropEffect = 'none';
            }
        }
    }}
    ondrop={async (e) => {
        e.preventDefault();

        if (!previewView) {
            for (const item of e.dataTransfer!.items) {
                const file = item.getAsFile();
                if (file !== null) {
                    await handleImageUpload(file);
                }
            }
        }
    }}
/>
