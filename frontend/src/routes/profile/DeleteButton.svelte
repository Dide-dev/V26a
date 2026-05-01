<!--
    This program has been developed by students from the bachelor Computer Science at Utrecht
    University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
-->

<script lang="ts">
    import { deleteUser, type UserInfo } from '$lib/backend/users';
    import { m } from '$lib/paraglide/messages';
    import Popup from '$lib/Popup.svelte';
    import { goto } from '$app/navigation';
    import { appLink } from '$lib';

    let { user }: { user: UserInfo } = $props();

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
        await deleteUser(user.sub, deleteTrees);
        await goto(appLink('/'));
    }
</script>

<button data-testid="delete-button" class="rounded-md bg-red-800 p-2" onclick={openPopup}>
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
            <button class="rounded-md bg-gray-400 p-2" onclick={cancelDelete}>
                {m.cancel()}
            </button>
            <button class="rounded-md bg-red-400 p-2" onclick={doDelete}>
                {m.yes()}
            </button>
        </div>
    </Popup>
{/if}
