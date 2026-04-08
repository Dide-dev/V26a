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
        if (response.succeeded) {
            loginFailed = false;
            // Redirect to given url if valid
            let redirect_url = page.url.searchParams.get('redirect');
            if (!redirect_url) {
                goto(appLink('/'));
            } else {
                // redirect_url should be just a relative path, not a complete url
                if (!URL.canParse(redirect_url)) {
                    goto(appLink(redirect_url));
                } else {
                    goto(appLink('/'));
                }
            }
        } else {
            loginFailed = true;
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
