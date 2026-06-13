<!--
    This program has been developed by students from the bachelor Computer Science at Utrecht
    University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
-->

<!--
    @component
    Render the data of a user for in a list.

    ## Params
    - `user: AllUserGetData` - The data of the user to display.
-->

<script lang="ts">
    import type { AllUsersGetData } from '$lib/api-wrapper/admin';
    import { m } from '$lib/paraglide/messages.js';
    import { appLink } from '$lib';
    import { EditOutline, LockSolid } from 'flowbite-svelte-icons';
    import AnimatedUnderline from '$lib/AnimatedUnderline.svelte';

    let {
        user
    }: {
        user: AllUsersGetData;
    } = $props();
</script>

<div class="list__entry">
    <div class="flex w-full flex-row justify-between gap-2">
        <div class="flex max-w-2/3 flex-col">
            <span class="flex gap-1 *:*:inline">
                <AnimatedUnderline --weight-underline="calc(var(--spacing) / 2)">
                    <a
                        href={appLink(`/user-manager?id=${user.id}`)}
                        target="_blank"
                        class={[
                            !user.enabled && 'text-neutral-600 italic',
                            'line-clamp-1 w-fit flex-1 break-all text-ellipsis'
                        ]}
                        title={user.enabled ? m.editUser() : m.disabledUser()}
                    >
                        <strong>{user.name}</strong>
                    </a>
                </AnimatedUnderline>
                {#if !user.enabled}
                    <span title={m.disabledUser()}>
                        <LockSolid class="mb-1 h-5 w-5 shrink-0" />
                    </span>
                {/if}
                <span class="font-semibold text-neutral-500">#{user.id}</span>
                <a
                    href={appLink(`/user-manager?id=${user.id}`)}
                    aria-label="edit"
                    target="_blank"
                    title={m.editUser()}
                >
                    <EditOutline class="mb-1 h-5 w-5 shrink-0" />
                </a>
                <span class="flex-auto grow-100"></span>
            </span>
            <em class="-mt-1 text-neutral-700">{m.email()}: {user.email}</em>
        </div>
    </div>
</div>
