<!--
    This program has been developed by students from the bachelor Computer Science at Utrecht
    University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
-->

<script lang="ts">
    import { deleteMe } from '$lib/api-wrapper/users';
    import { m } from '$lib/paraglide/messages';
    import Popup from '$lib/Popup.svelte';
    import { goto } from '$app/navigation';
    import { appLink } from '$lib';

    let popupVisible = $state(false);

    let deleteTrees = $state(false);

    function openPopup(event: Event) {
        event.preventDefault();
        popupVisible = true;
    }

    function cancelDelete() {
        popupVisible = false;
    }

    async function doDelete() {
        await deleteMe(deleteTrees);
        await goto(appLink('/'));
    }
</script>

<button type="reset" data-testid="delete-button" class="danger" onclick={openPopup}>
    {m.deleteAccount()}
</button>

{#if popupVisible}
    <Popup>
        <p class="text-lg font-bold">
            {m.deleteAccountConfirmation()}
        </p>

        <label class="flex items-center gap-2">
            <input type="checkbox" data-testid="delete-checkbox" bind:checked={deleteTrees} />
            {m.deleteOwnedTrees()}
        </label>

        <div class="flex justify-end gap-2">
            <button type="reset" onclick={cancelDelete}>
                {m.cancel()}
            </button>
            <button type="reset" data-testid="delete-confirm" class="danger" onclick={doDelete}>
                {m.yes()}
            </button>
        </div>
    </Popup>
{/if}
