<script lang="ts">
    import MonthButton from "$lib/components/MonthButton.svelte";
    import PaymentView from "$lib/components/PaymentView.svelte";
    import { monthsArray, paymentsArray, sumMonth } from "../lib/db";
    import MoneySpan from "$lib/components/MoneySpan.svelte";
    import Headerbar from "$lib/components/Headerbar.svelte";

    $: overview = $sumMonth;
</script>

<Headerbar loader_data={["current_user", "user", "month_index"]} />
<main>
    <div class="col side">
        <div class="overview col">
            {#if overview != undefined}
                <h3>Gesamt</h3>
                <div class="col center">
                    <MoneySpan amount={overview.positive} />
                    <MoneySpan amount={overview.negative} />
                    <MoneySpan
                        amount={overview.positive + overview.negative}
                        role="neutral"
                    />
                </div>
                <MoneySpan amount={overview.repay} role="repay" />
            {/if}
        </div>
        {#each $monthsArray as month}
            <MonthButton {month} />
        {/each}
    </div>
    <div class="col">
        {#each $paymentsArray as payment}
            <PaymentView {payment} />
        {/each}
    </div>
</main>

<style>
    .overview {
        aspect-ratio: 1;
        border-radius: var(--small);
        background-color: var(--surface);
        align-items: center;
        justify-content: space-around;
        gap: var(--small);
    }
</style>
