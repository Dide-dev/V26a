<!--
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
-->

<script lang="ts">
    import { login, type LoginData } from '$lib/backend/auth';
    import { goto } from '$app/navigation';
    import { appLink } from '$lib';
    import { m } from '$lib/paraglide/messages.js';
    import { page } from '$app/state';

    let loginData: LoginData = $state({
        email: '',
        password: ''
    });
    let loginFailed: boolean = $state(false);

    async function handleSubmit(event: SubmitEvent) {
        event.preventDefault();
        let response = await login(loginData);

        if (!response.succeeded) {
            loginFailed = true;
            return;
        }

        loginFailed = false;

        // Redirect to given url if valid
        let redirectUrl = page.url.searchParams.get('redirect');
        if (!redirectUrl) {
            await goto(appLink('/'));
            return;
        }

        // redirect_url should be just a relative path, not a complete url
        if (!URL.canParse(redirectUrl)) {
            await goto(appLink(redirectUrl));
        } else {
            await goto(appLink('/'));
        }
    }
</script>

<div class="flex w-full justify-center">
    <form class="flex w-md flex-col gap-2" onsubmit={handleSubmit}>
        <label>
            {m.email()}
            <input
                class="rounded-md border-2"
                bind:value={loginData.email}
                name="email"
                type="email"
                data-testid="email"
                required
            />
        </label>
        <label>
            {m.password()}
            <input
                class="rounded-md border-2"
                bind:value={loginData.password}
                name="password"
                type="password"
                data-testid="password"
                required
            />
        </label>
        <button class="rounded-sm bg-emerald-500 p-1">{m.login()}</button>
        {#if loginFailed}
            <p class="text-red-600">{m.invalidCredentials()}</p>
        {/if}
    </form>
</div>
