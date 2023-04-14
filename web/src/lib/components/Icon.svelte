<script lang="ts">
    export let icon: string;
    export let tooltip: string | null = null;
</script>

<span class="material-symbols-rounded" data-tooltip={tooltip}>{icon}</span>

<style lang="scss">
    span {
        user-select: none;
        position: relative;
        display: flex;
        justify-content: center;

        &[data-tooltip] {
            border-radius: 4px;
            transition-property: background-color, color, border-radius;
            transition-duration: 200ms;
            &::after {
                font-family: "Segoe UI", Tahoma, Geneva, Verdana, sans-serif;
                content: attr(data-tooltip);
                font-size: 16px;
                font-weight: bold;
                background-color: var(--primary);
                color: var(--primary-on);
                border-radius: var(--small);
                padding: var(--small);
                position: absolute;

                bottom: 100%;
                transform: scale(0);
                opacity: 0;

                transition-property: transform, opacity, bottom;
                transition-duration: 200ms;
                transition-timing-function: cubic-bezier(0.5, 0, 0.5, 1);
            }
            &:hover {
                background-color: var(--primary);
                color: var(--primary-on);
                border-radius: 50%;
                box-shadow: 0 0 0 4px var(--primary);
                transition-delay: 250ms;
                &::after {
                    transform: scale(1);
                    bottom: calc(100% + var(--small));
                    opacity: 1;
                    transition-delay: 250ms;
                }
            }
        }
    }
</style>
