<!--
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
-->

<script lang="ts">
    import { locales, getLocale } from '$lib/paraglide/runtime.js';
    import { addUser, type RegisterData } from '$lib/backend/users';
    import { goto } from '$app/navigation';
    import { appLink } from '$lib';
    import { m } from '$lib/paraglide/messages.js';

    let formData: RegisterData = $state({
        email: '',
        password: '',
        name: '',
        nickname: '',
        aboutMe: null,
        emailVisible: false,
        language: getLocale()
    });
    let repeatPassword = $state('');

    let passwordMissingChar: boolean = $state(false);
    let passwordNotSame: boolean = $state(false);
    let emailAlreadyExists: boolean = $state(false);

    function checkSamePassword(): boolean {
        return repeatPassword == formData.password;
    }

    function checkLowercase(): boolean {
        let passwordChecker = formData.password.toUpperCase();
        return passwordChecker != formData.password;
    }

    function checkUppercase(): boolean {
        let passwordChecker = formData.password.toLowerCase();
        return passwordChecker != formData.password;
    }

    function checkNumber(): boolean {
        let regex = /\d/;
        return regex.test(formData.password);
    }

    function checkSpecialCharacter(): boolean {
        let regex = /[?!@#$%^&*()+=.,<>{}[\]:;"'|~`_/\\-]/;
        return regex.test(formData.password);
    }

    async function handleSubmit(event: SubmitEvent) {
        event.preventDefault();
        passwordMissingChar = false;
        passwordNotSame = false;
        emailAlreadyExists = false;

        if (!checkLowercase() || !checkUppercase() || !checkNumber() || !checkSpecialCharacter()) {
            passwordMissingChar = true;
        } else if (!checkSamePassword()) {
            passwordNotSame = true;
        } else {
            
            let addResponse = await addUser(formData);
            if (addResponse.type === 'Success') {
                emailAlreadyExists = false;
                await goto(appLink('/login'));
            } else if (addResponse.type === 'EmailAlreadyUsed') {
                emailAlreadyExists = true;
            } else {
                throw new Error('ERROR: Unexpected output from API.');
            }

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
                data-testid="email"
                placeholder={m.emailPlaceholder()}
                maxlength="256"
                required
            />
        </label>
        {#if emailAlreadyExists}
            <p class="text-red-600" data-testid="error">{m.emailAlreadyExists()}</p>
        {/if}
        <label>
            {m.name()}
            <input
                class="rounded-md border-2"
                bind:value={formData.name}
                name="name"
                type="text"
                data-testid="name"
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
                type="text"
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
                data-testid="password"
                minlength="8"
                maxlength="72"
                required
            />
        </label>
        {#if passwordMissingChar}
            <p class="text-red-600" data-testid="error">{m.missingCharacter()}</p>
        {/if}
        <label>
            {m.repeatPassword()}
            <input
                class="rounded-md border-2"
                bind:value={repeatPassword}
                name="repeat password"
                type="password"
                data-testid="repeat password"
                minlength="8"
                maxlength="72"
                required
            />
        </label>
        {#if passwordNotSame}
            <p class="text-red-600" data-testid="error">{m.passwordsNotTheSame()}</p>
        {/if}
        <label>
            {m.language()}
            <select class="rounded-md border-2" bind:value={formData.language}>
                {#each locales as value (value)}
                    <option {value}>{value}</option>
                {/each}
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
    </form>
</div>
