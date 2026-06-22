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

<button data-testid="delete-button" type="reset" class="danger" onclick={openPopup}>
    {m.deleteAccount()}
</button>

{#if popupVisible}
    <Popup>
        <form class="completed">
            <h3 class="page__subheading">
                {m.deleteAccountConfirmation()}
            </h3>

            <em>{m.dangerNotice()}</em>

            <div class="flex flex-row items-center gap-2">
                <input
                    id="delete-trees"
                    type="checkbox"
                    data-testid="delete-checkbox"
                    class="input__checkbox"
                    bind:checked={deleteTrees}
                />
                <label for="delete-trees">{m.deleteOwnedTrees()}</label>
            </div>

            <div class="grid grid-cols-2 gap-1">
                <button type="reset" onclick={cancelDelete}>
                    {m.cancel()}
                </button>
                <button
                    data-testid="delete-confirm"
                    type="submit"
                    class="danger"
                    onclick={doDelete}
                >
                    {m.yes()}
                </button>
            </div>
        </form>
    </Popup>
{/if}
