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

    let loginData: LoginData = $state({
        email: '',
        password: ''
    });
    let error: null | Error = $state(null);

    async function handleSubmit(event: SubmitEvent) {
        event.preventDefault();
        try {
            await login(loginData);
            // TODO: this should redirect to the previous page after login.
            goto(appLink('/'));
        } catch (e) {
            error = e as Error;
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
        <p class="text-red-600">{error?.message}</p>
    </form>
</div>
