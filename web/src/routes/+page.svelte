<script lang="ts">
    import MonthButton from "$lib/components/MonthButton.svelte";
    import PaymentView from "$lib/components/PaymentView.svelte";
    import {
        monthsArray,
        paymentsArray,
        sumMonth,
    } from "../lib/db";
    import MoneySpan from "$lib/components/MoneySpan.svelte";
    import DbLoader from "$lib/components/DBLoader.svelte";

    $: overview = $sumMonth;
</script>

<header>
    <DbLoader data={["current_user", "month_index", "user"]}/>
</header>
<main>
    <div class="sidebar">
        <div class="overview">
            {#if overview != undefined}
                <h3>Gesamt</h3>
                <div class="sub">
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
    <div class="content">
        {#each $paymentsArray as payment}
            <PaymentView {payment} />
        {/each}
    </div>
</main>

<style>
    :global(:root) {
        font-family: "Segoe UI", Tahoma, Geneva, Verdana, sans-serif;

        --background: #202020;
        --background-on: #ffffff;
        --background-dark: #1b1b1b;
        --background-light: #333333;

        --surface: #2c2c2c;
        --surface-on: #ffffff;
        --surface-dark: #1d1d1d;
        --surface-light: #3f3f3f;

        --primary: #003cff;
        --primary-on: #ffffff;
        --primary-dark: #0032d6;
        --primary-light: #3363ff;

        --button: #2c2c2c;
        --button-on: #ffffff;
        --button-dark: #1d1d1d;
        --button-light: #3f3f3f;
        --button-shadow: 4px 4px 8px 4px rgba(15, 15, 15, 0.425);

        --small: 8px;
        --user-size: 42px;
    }

    :global(*) {
        box-sizing: border-box;
        margin: 0;
        padding: 0;
    }

    :global(html) {
        height: 100%;
    }

    :global(body) {
        height: 100%;
        background-color: var(--background);
        color: var(--background-on);
        display: flex;
        flex-direction: column;
    }

    header {
        background-color: var(--surface);
        color: var(--surface-on);
        padding: var(--small);
    }

    main {
        flex: 1;
        display: flex;
        flex-direction: row;
    }

    .sidebar {
        width: 256px;
        display: flex;
        flex-direction: column;
        padding: var(--small);
        gap: var(--small);
        background-color: var(--background-dark);
    }

    .overview {
        aspect-ratio: 1;
        border-radius: var(--small);
        background-color: var(--surface);
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: space-around;
        gap: var(--small);
    }

    .sub {
        display: flex;
        flex-direction: column;
        align-items: center;
        gap: var(--small);
    }

    .content {
        flex: 1;
        display: flex;
        flex-direction: column;
        padding: var(--small);
        gap: var(--small);
    }
</style>
