<script lang="ts">
    export let icon: string;
    export let tooltip: string | null = null;
    export let tooltipLocation: "top" | "bottom" = "top";
</script>

<span
    class="material-symbols-rounded"
    data-tooltip={tooltip}
    data-tooltip-location={tooltipLocation}>{icon}</span
>

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

                transform: scale(0);
                opacity: 0;

                transition-property: transform, opacity, bottom, top;
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
                    opacity: 1;
                    transition-delay: 250ms;
                }
            }
        }

        &[data-tooltip-location="top"] {
            &::after {
                bottom: 50%;
            }

            &:hover::after {
                bottom: calc(100% + var(--small));
            }
        }

        &[data-tooltip-location="bottom"] {
            &::after {
                top: 50%;
            }

            &:hover::after {
                top: calc(100% + var(--small));
            }
        }
    }
</style>
