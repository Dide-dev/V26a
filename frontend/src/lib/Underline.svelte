<!--
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
-->

<!--
    @component
    Render an animated underline effect below the children.

    ## Params
    - `--color-underline` - The color to render the underline. Default color is black.
-->
<script lang="ts">
    let { children } = $props();
</script>

<div class="animated_underline flex flex-auto">
    {@render children()}
</div>

<style>
    /* Underline formatting. */
    .animated_underline {
        > :global(*) {
            position: relative;

            &::after {
                content: '';
                position: absolute;
                left: 0;
                bottom: 0;
                width: 0;
                height: calc(var(--spacing) * 0.5);
                background-color: var(--color-underline);

                transition: width 0.3s ease-out;
            }
        }

        &:hover > :global(*::after) {
            width: 100%;
        }
    }
</style>
