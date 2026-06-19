<!--
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
-->

<script lang="ts">
    import { m } from '$lib/paraglide/messages.js';
    import { requestResetPassword, type ResetPasswordRequestData } from '$lib/api-wrapper/users';
    import type { Snippet } from 'svelte';
    import { ParaglideMessage } from '@inlang/paraglide-js-svelte';
    import { goto } from '$app/navigation';
    import { appLink } from '$lib';

    let resetPasswordData: ResetPasswordRequestData = $state({
        email: ''
    });
    let formCompleted: boolean = $derived(resetPasswordData.email !== '');
    let formSent: boolean = $state(false);

    const maxInputLengths = {
        email: 128
    };

    async function handleSubmit(event: SubmitEvent) {
        event.preventDefault();
        await requestResetPassword(resetPasswordData);

        formSent = true;
    }

    async function gotoLogin(e: Event) {
        e.preventDefault();
        await goto(appLink('/login'));
    }
</script>

<div>
    <form class={[formCompleted && 'completed']} onsubmit={handleSubmit}>
        <div>
            <h1 class="page__title">{m.resetPassword()}</h1>
        </div>
        {#if !formSent}
            <p>{m.resetPasswordInstructions()}</p>
            <div>
                <label for="email">
                    {m.email()}
                </label>
                <input
                    id="email"
                    class="input__text"
                    bind:value={resetPasswordData.email}
                    name="email"
                    type="email"
                    data-testid="email"
                    placeholder={m.yourEmail()}
                    maxlength={maxInputLengths.email}
                    autocomplete="email"
                    required
                />
            </div>
            <button class="mt-3" type="submit">{m.send()}</button>
        {:else}
            <p>
                <ParaglideMessage
                    message={m.resetPasswordRequestSent}
                    inputs={{ mail: resetPasswordData.email }}
                >
                    {#snippet link({ children }: { children?: Snippet })}
                        <a class="text__link" href="mailto:{resetPasswordData.email}">
                            {@render children?.()}
                        </a>
                    {/snippet}
                </ParaglideMessage>
                {m.checkSpam()}
            </p>

            <button type="submit" class="mt-4 cursor-pointer rounded px-3 py-1" onclick={gotoLogin}
                >{m.login()}
            </button>
        {/if}
    </form>
</div>
