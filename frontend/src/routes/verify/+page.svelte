<!--
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
-->

<script lang="ts">
    import { page } from '$app/state';
    import { m } from '$lib/paraglide/messages';
    import { goto } from '$app/navigation';
    import { appLink } from '$lib';
    import { verifyUser } from '$lib/api-wrapper/users';
    import { onMount } from 'svelte';

    let verificationSuccess: boolean | null = $state(null);

    onMount(async () => {
        let verificationCode = page.url.searchParams.get('code');

        if (!verificationCode) {
            await goto(appLink('/registration'));
        }

        let verificationState = await verifyUser(verificationCode!);
        verificationSuccess = verificationState.type === 'Success';
    });

    async function gotoRegister(e: Event) {
        e.preventDefault();
        await goto(appLink('/registration'));
    }

    async function gotoLogin(e: Event) {
        e.preventDefault();
        await goto(appLink('/login'));
    }
</script>

<div>
    {#if verificationSuccess === null}
        <p class="text-green-800">{m.loading()}</p>
    {:else if verificationSuccess!}
        <form class="completed">
            <div>
                <h1 class="page__title">{m.verificationSuccessTitle()}</h1>
                <p>{m.verificationSuccessBody()}</p>
            </div>

            <button type="submit" class="cursor-pointer rounded px-3 py-1" onclick={gotoLogin}
                >{m.login()}</button
            >
        </form>
    {:else}
        <form class="completed">
            <div>
                <h1 class="page__title">{m.verificationFailureTitle()}</h1>
                <p>{m.verificationFailureBody()}</p>
            </div>

            <div class="mb-0 flex flex-row gap-1 *:w-1/2">
                <button
                    type="submit"
                    class="mb- cursor-pointer rounded px-3 py-1"
                    onclick={gotoLogin}>{m.login()}</button
                >
                <button
                    type="submit"
                    class="cursor-pointer rounded px-3 py-1"
                    onclick={gotoRegister}>{m.register()}</button
                >
            </div>
        </form>
    {/if}
</div>
