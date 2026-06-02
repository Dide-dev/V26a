<!--
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
-->

<!--
    @component
    Render an animated underline effect below the first child.

    Wrapper object automatically grows to fill its container.

    ## Params
    - `--color-underline` - The color to render the underline with. Default color is black.
    - `--weight-underline` - Height to render the the underline with. Must be provided.
                             Example value might be `calc(var(--spacing) / 3)`.
-->
<script lang="ts">
    let { children } = $props();
</script>

<div class="animated_underline flex flex-auto">
    {@render children()}
</div>

<style>
    .animated_underline {
        /* Must be global to affect the rendered children. */
        > :global(*:first-child) {
            position: relative;

            /* The underline */
            &::after {
                content: '';
                /* Layout */
                position: absolute;
                left: 0;
                bottom: 0;
                width: 0;
                height: var(--weight-underline);
                /* Default color is var(--color-black). */
                background-color: var(--color-underline);
                /* Animation */
                transition: width 0.3s ease-out;
            }
        }
        /* Must be global to affect the rendered children. */
        &:hover > :global(*:first-child::after) {
            /* Grow to full width. */
            width: 100%;
        }
    }
</style>
