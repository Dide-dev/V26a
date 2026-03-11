<script lang="ts">
    import { locales, getLocale } from '$lib/paraglide/runtime.js';
    import { addUser, type UserData } from '$lib/backend/users';
    import { goto } from '$app/navigation';
    import { appLink } from '$lib';
    import { m } from '$lib/paraglide/messages.js';

    let formData: UserData = $state({
        email: '',
        password: '',
        name: '',
        nickname: '',
        aboutMe: null,
        emailVisible: false,
        language: getLocale()
    });
    let error: null | Error = $state(null);

    async function handleSubmit(event: SubmitEvent) {
        event.preventDefault();
        try {
            await addUser(formData);
            // TODO: this should redirect to the login page
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
                bind:value={formData.email}
                name="email"
                type="email"
                placeholder={m.emailPlaceholder()}
                maxlength="256"
                required
            />
        </label>
        <label>
            {m.name()}
            <input
                class="rounded-md border-2"
                bind:value={formData.name}
                name="name"
                type="text"
                maxlength="128"
                required
            />
        </label>
        <label>
            {m.nickname()}
            <input
                class="rounded-md border-2"
                bind:value={formData.nickname}
                name="nickname"
                type="nickname"
                placeholder={m.optional()}
                maxlength="128"
            />
        </label>
        <label>
            {m.password()}
            <input
                class="rounded-md border-2"
                bind:value={formData.password}
                name="password"
                type="password"
                maxlength="128"
                required
            />
        </label>
        <label>
            {m.language()}
            <select class="rounded-md border-2" bind:value={formData.language}>
                {#each locales as value (value)}<option {value}>{value}</option>{/each}
            </select>
        </label>
        <label>
            <input
                class="rounded-md border-2"
                bind:checked={formData.emailVisible}
                name="emailVisible"
                type="checkbox"
                maxlength="128"
            />
            {m.emailVisible()}
        </label>
        <button class="rounded-sm bg-emerald-500 p-1">{m.register()}</button>
        <p class="text-red-600">{error?.message}</p>
    </form>
</div>
