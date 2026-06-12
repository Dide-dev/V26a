<!--
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
-->

<script lang="ts">
    import { login, type LoginData } from '$lib/api-wrapper/auth';
    import { goto } from '$app/navigation';
    import { appLink } from '$lib';
    import { m } from '$lib/paraglide/messages.js';
    import { page } from '$app/state';

    let loginData: LoginData = $state({
        email: '',
        password: ''
    });
    let loginFailed: boolean = $state(false);

    let formCompleted: boolean = $derived(loginData.email !== '' && loginData.password !== '');

    const minInputLengths = {
        password: 1 // In case an old account has a password shorter than the current imposed minimum.
    };

    const maxInputLengths = {
        email: 128
    };

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

<div>
    <form class={[formCompleted && 'completed']} onsubmit={handleSubmit}>
        <div>
            <h1 class="page__title">{m.login()}</h1>
        </div>
        <div>
            <label for="email">
                {m.email()}
            </label>
            <input
                id="email"
                class={[loginFailed && 'error', 'input__text']}
                bind:value={loginData.email}
                name="email"
                type="email"
                data-testid="email"
                placeholder={m.yourEmail()}
                maxlength={maxInputLengths.email}
                autocomplete="email"
                required
            />
        </div>
        <div>
            <label for="password">
                {m.password()}
            </label>
            <input
                id="password"
                class={[loginFailed && 'error', 'input__text']}
                bind:value={loginData.password}
                name="password"
                type="password"
                data-testid="password"
                placeholder={m.yourPassword()}
                minlength={minInputLengths.password}
                autocomplete="current-password"
                required
            />
        </div>
        <button class="mt-3" type="submit">{m.login()}</button>
        {#if loginFailed}
            <p class="text__error">{m.invalidCredentials()}</p>
        {/if}
        <a class="text__link mt-2 italic" href={appLink('/registration')}>{m.noAccount()}</a>
        <a class="text__link italic" href={appLink('/forgot-password')}>
            {m.forgotPassword()}
        </a>
    </form>
</div>
