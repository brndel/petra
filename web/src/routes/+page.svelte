<script lang="ts">
    import MonthButton from "$lib/components/MonthButton.svelte";
    import PaymentView from "$lib/components/PaymentView.svelte";
    import MoneySpan from "$lib/components/MoneySpan.svelte";
    import Headerbar from "$lib/components/Headerbar.svelte";
    import MonthGraph from "$lib/components/MonthGraph.svelte";
    import { getMonthName } from "$lib/text";
    import Icon from "$lib/components/Icon.svelte";
    import MonthOverview from "$lib/components/MonthOverview.svelte";
    import { monthArray, sumMonth } from "$lib/db/month";
    import { paymentArray, selectedMonth } from "$lib/db/payment";

    $: overview = $sumMonth;

    $: selected = $selectedMonth;
</script>

<Headerbar
    loader_data={[
        "current_user",
        "user",
        "month_index",
        "category",
        "category_group",
    ]}
/>

<div class="col side scroll">
    {#if overview != undefined}
        <MonthOverview month={overview} />
    {/if}
    <div class="col scroll">
        {#each $monthArray as month}
            <MonthButton {month} />
        {/each}
    </div>
</div>
<div class="col main">
    {#if selected !== null}
        <MonthGraph payments={$paymentArray} />
        {#each $paymentArray as payment}
            <PaymentView {payment} />
        {/each}
    {:else}
        <div class="col center">
            <h2>Bitte irgendwas auswählen</h2>
        </div>
    {/if}
</div>
{#if selected !== null}
    <div class="col side scroll">
        <MonthOverview month={selected} />
    </div>
{/if}
