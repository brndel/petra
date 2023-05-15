<script lang="ts">
    import type { MonthData } from "$lib/data_types";
    import { getMonthName } from "$lib/text";
    import Icon from "./Icon.svelte";
    import MoneySpan from "./MoneySpan.svelte";

    export let month: MonthData;
</script>

<div class="overview col">
    {#if month != undefined}
        <h2>{getMonthName(month)}</h2>
        <div class="col end group">
            <MoneySpan amount={month.positive} />
            <MoneySpan amount={month.negative} />
            <div class="row">
                <Icon icon="keyboard_arrow_right" />
                <MoneySpan
                    amount={month.positive + month.negative}
                    role="neutral"
                />
            </div>
        </div>
        <MoneySpan amount={month.repay} role="repay" />
        <div class="row group">
            <Icon icon="keyboard_double_arrow_right" />
            <MoneySpan amount={month.positive + month.negative + month.repay} />
        </div>
    {/if}
</div>

<style>
    .group {
        background-color: var(--surface-light);
        padding: var(--small);
        border-radius: var(--small);
    }
</style>
