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
    import { m } from '$lib/paraglide/messages';
    import Popup from '$lib/Popup.svelte';

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

    let errorMessage = $state('');

    function confirmClick() {
        card.front = card.front.trim();
        card.back = card.back.trim();

        if (card.back.length === 0) {
            errorMessage = m.backTextRequired();
            return;
        }

        if (card.front.length > frontMaxLength) {
            errorMessage = m.frontTextCharacterLimit({ limit: frontMaxLength });
            return;
        }
        if (card.back.length > backMaxLength) {
            errorMessage = m.backTextCharacterLimit({ limit: backMaxLength });
            return;
        }

        confirmCallback(card);
    }
</script>

<Popup>
    <div class="flex flex-col gap-2">
        <textarea
            class="rounded border p-1"
            placeholder={m.enterFrontText()}
            bind:value={card.front}
        ></textarea>
        <textarea class="rounded border p-1" placeholder={m.enterBackText()} bind:value={card.back}
        ></textarea>
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
