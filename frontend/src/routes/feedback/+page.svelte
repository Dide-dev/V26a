<!--
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
-->

<script lang="ts">
    import { provideFeedback, type FeedbackData } from '$lib/backend/feedback';
    import { m } from '$lib/paraglide/messages.js';

    let feedbackData: FeedbackData = $state({
        email: '',
        feedback: ''
    });

    let successfulSubmission: boolean = $state(false);

    async function handleSubmit(event: SubmitEvent) {
        event.preventDefault();
        successfulSubmission = false;

        await provideFeedback(feedbackData);
        successfulSubmission = true;

        feedbackData.email = '';
        feedbackData.feedback = '';
    }
</script>

<div>
    <form class={[feedbackData.feedback && 'completed']} onsubmit={handleSubmit}>
        <div>
            <h1 class="page__title">Feedback</h1>
            <p>{m.feedbackExplanation()}</p>
        </div>
        <div>
            <div class="label__container">
                <label for="email">{m.email()}</label>
                <div class="info">
                    <sup>🛈</sup>
                    <div class="info__content transform-none">{m.emailFeedbackInfo()}</div>
                </div>
            </div>
            <input
                id="email"
                class="input__text"
                bind:value={feedbackData.email}
                name="email"
                type="email"
                autocomplete="email"
                placeholder={m.optionalPlaceholder({ value: m.yourEmail() })}
                data-testid="email"
            />
        </div>
        <div>
            <label for="feedback">{m.feedback()} *</label>
            <textarea
                id="feedback"
                rows="3"
                class="input__long_text"
                bind:value={feedbackData.feedback}
                name="feedback"
                data-testid="feedback"
                placeholder={m.yourFeedback()}
                required
            ></textarea>
            {#if successfulSubmission}
                <p class="text__success" data-testid="submission-success">
                    {m.succesfulFeedbackSubmission()}
                </p>
            {/if}
        </div>
        <button data-testid="submit-feedback-button">{m.submitFeedback()}</button>
    </form>
</div>
<br />
<br />
