<script lang="ts">
    import type { MonthData } from "$lib/data_types";
    import { loadPayments } from "$lib/db";
    import { getMonthName } from "$lib/text";
    import MoneySpan from "./MoneySpan.svelte";

    export let month: MonthData;

    $: totalAmount = month.positive + month.negative;
    // $: totalRepayAmount = totalAmount + month.repayAmount;

    function onButtonClick() {
        loadPayments(month);
    }
</script>

<button on:click={onButtonClick}>
    <span>
        {getMonthName(month)}
    </span>
    <div class="money">
        <MoneySpan amount={month.positive} />
        <MoneySpan amount={month.negative} />
        <MoneySpan
            amount={totalAmount}
            role="neutral"
        />
    </div>
    <div class="money">
        <MoneySpan amount={month.repay} role="repay" />
    </div>
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

    .money {
        display: flex;
        flex-direction: row;
        justify-content: space-between;
    }
</style>
