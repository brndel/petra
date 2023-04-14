<script lang="ts">
    import { repay } from "$lib/text";
    import Icon from "./Icon.svelte";

    export let amount: number;
    export let role: "auto" | "neutral" | "repay" = "auto";

    $: money = (amount > 0 ? "+" : "") + (amount / 100).toFixed(2);

    function getRoleClass(role: string, amount: number): string {
        switch (role) {
            case "auto":
                if (amount > 0) {
                    return "positive";
                } else if (amount < 0) {
                    return "negative";
                } else {
                    return "netural";
                }
            default:
                return role;
        }
    }

    $: roleClass = getRoleClass(role, amount);
</script>

<span class={roleClass}>
    {#if role === "repay"}
        <Icon icon="currency_exchange" tooltip={repay}/>
    {/if}
    {money}
</span>

<style>
    span {
        font-size: 16px;
        font-weight: bold;
        display: flex;
        align-items: center;
        gap: var(--small);
    }

    .positive {
        color: hsl(120, 100%, 50%);
    }

    .negative {
        color: hsl(0, 100%, 50%);
    }

    .neutral {
        color: hsl(0, 0%, 80%);
    }

    .repay {
        color: hsl(0, 0%, 50%);
    }
</style>
