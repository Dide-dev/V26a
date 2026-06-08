<!--
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
-->

<script lang="ts">
    import { provideFeedback, type FeedbackData } from '$lib/api-wrapper/feedback';
    import { m } from '$lib/paraglide/messages.js';

    let feedbackData: FeedbackData = $state({
        email: '',
        feedback: ''
    });

    const minInputLengths = {
        email: 3,
        feedback: 6
    };

    const maxInputLengths = {
        email: 128,
        feedback: 5000
    };

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
                    <div class="info__content transform-none">
                        {m.emailFeedbackInfo()}
                        {m.conformPrivacy()}
                    </div>
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
                maxLength={maxInputLengths.email}
                minLength={minInputLengths.email}
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
                maxLength={maxInputLengths.feedback}
                minLength={minInputLengths.feedback}
            ></textarea>
            {#if successfulSubmission}
                <p class="text__success" data-testid="submission-success">
                    {m.successfulFeedbackSubmission()}
                </p>
            {/if}
        </div>
        <button type="submit" data-testid="submit-feedback-button">{m.submitFeedback()}</button>
    </form>
</div>
<br />
<br />
