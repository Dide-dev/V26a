<!--
    This program has been developed by students from the bachelor Computer Science at Utrecht
    University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
-->
<script lang="ts">
    import { m } from '$lib/paraglide/messages.js';
    import { type AllUsersGetData, getAllUserData } from '$lib/backend/admin';
    import { onMount } from 'svelte';
    import iconEdit from '$lib/assets/icons/edit.svg?raw';
    import { appLink } from '$lib';
    import { booleanToString } from '$lib/general';

    onMount(async () => {
        await loadUsers();
    });

    let users: null | AllUsersGetData[] = $state(null);

    async function loadUsers() {
        users = await getAllUserData();
    }
</script>

{#if users === null}
    <p>{m.loading()}</p>
{:else}
    <div class="100vw flex justify-center">
        <table class="border-separate border-spacing-2">
            <thead>
                <tr>
                    <th class="text-left">{m.id()}</th>
                    <th class="text-left">{m.email()}</th>
                    <th class="text-left">{m.name()}</th>
                    <th class="text-left">{m.enabled()}</th>
                    <!-- Edit button -->
                    <th class="text-left"></th>
                </tr>
            </thead>
            <tbody>
                {#each users as user (user.id)}
                    <tr>
                        <td class="pr-4 wrap-anywhere">{user.id}</td>
                        <td class="pr-4 wrap-anywhere">{user.email}</td>
                        <td class="pr-4">{user.name}</td>
                        <td class="pr-4">{booleanToString(user.enabled)}</td>
                        <td class="min-w-2 hover:fill-blue-800">
                            <a href={appLink(`/user-manager?id=${user.id}`)} aria-label="edit">
                                <!-- eslint-disable-next-line svelte/no-at-html-tags -->
                                {@html iconEdit}
                            </a>
                        </td>
                    </tr>
                {/each}
            </tbody>
        </table>
    </div>
{/if}
