<script lang="ts">
    import { m } from '$lib/paraglide/messages.js';
    import { locales, getLocale } from '$lib/paraglide/runtime.js';
    import { addUser, type UserData } from '$lib/backend/users';
    import { goto } from '$app/navigation';
    import { appLink } from '$lib';
    let formData: UserData = $state({
        email: '',
        password: '',
        name: '',
        nickname: null,
        aboutMe: null,
        emailVisible: false,
        language: getLocale()
    });
    let error: null | Error = $state(null);
    async function handleSubmit(event: SubmitEvent) {
        event.preventDefault();
        try {
            await addUser(formData);
            // this should redirect to the login page
            goto(appLink('/'));
        } catch (e: any) {
            error = e;
        }
    }
</script>

<div class="flex w-full justify-center">
    <form class="flex w-md flex-col gap-2" onsubmit={handleSubmit}>
        <label>
            Email
            <input
                class="rounded-md border-2"
                bind:value={formData.email}
                name="email"
                type="email"
            />
        </label>
        <label>
            name
            <input class="rounded-md border-2" bind:value={formData.name} name="name" type="text" />
        </label>
        <label>
            Password
            <input
                class="rounded-md border-2"
                bind:value={formData.password}
                name="password"
                type="password"
            />
        </label>
        <label>
            language
            <select class="rounded-md border-2" bind:value={formData.language}>
                {#each locales as value}<option {value}>{value}</option>{/each}
            </select>
        </label>
        <button class="rounded-sm bg-emerald-500 p-1">Register</button>
        <p class="text-red-600">{error?.message}</p>
    </form>
</div>
