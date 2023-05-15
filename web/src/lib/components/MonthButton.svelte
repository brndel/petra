<script lang="ts">
    import type { MonthData } from "$lib/data_types";
    import { loadPayments } from "$lib/db/payment";
    import { getMonthName } from "$lib/text";
    import Icon from "./Icon.svelte";
    import MoneySpan from "./MoneySpan.svelte";

    export let month: MonthData;

    $: totalAmount = month.positive + month.negative;
    $: calculatedAmount = totalAmount + month.repay;

    function onButtonClick() {
        loadPayments(month);
    }
</script>

<button on:click={onButtonClick}>
    <span>
        {getMonthName(month)}
    </span>
    <div class="row space">
        <MoneySpan amount={month.repay} role="repay" />
        <MoneySpan amount={calculatedAmount} role="auto" />
    </div>
    <!-- <div class="money">
    </div>
    <div class="money">
        <Icon icon="arrow_right" />
    </div> -->
</button>

<style>
    button {
        cursor: pointer;
        background-color: var(--button);
        color: var(--button-on);
        border: none;
        padding: var(--small);
        gap: var(--small);
        border-radius: var(--small);
        font-size: 16px;
        font-weight: bold;
        display: flex;
        flex-direction: column;
        box-shadow: var(--button-shadow);
        transition: background-color 100ms;
    }

    button:hover {
        background-color: var(--button-dark);
    }
</style>
