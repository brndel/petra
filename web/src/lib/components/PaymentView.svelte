<script lang="ts">
    import type { Payment } from "$lib/data_types";
    import Icon from "./Icon.svelte";
    import MoneySpan from "./MoneySpan.svelte";
    import UserStack from "./UserStack.svelte";
    import UserView from "./UserView.svelte";

    export let payment: Payment;

    $: showUsers = !(
        payment.users.length == 1 && payment.users[0] == payment.owner
    );

    $: date = `${payment.timestamp.getDate()}.${
        payment.timestamp.getMonth() + 1
    } ${payment.timestamp.getHours()}:${payment.timestamp
        .getMinutes()
        .toString()
        .padStart(2, "0")}`;
</script>

<div class="payment row">
    <div class="col start">
        <span>
            {payment.name}
            &#x2219;
            {date}
        </span>
        <div class="row center">
            <UserView user={payment.owner} />
            {#if showUsers}
                <Icon icon="arrow_right" />
                <UserStack users={payment.users} />
            {/if}
        </div>
    </div>
    <div class="row center">
        {#each payment.categories as category}
            <Icon icon={category.icon} tooltip={category.name} />
        {/each}
    </div>
    <div class="col end">
        <MoneySpan amount={payment.amount} role="auto" />
        {#if showUsers}
            <MoneySpan amount={payment.repayAmount} role="repay" />
            <MoneySpan
                amount={payment.amount + payment.repayAmount}
                role="neutral"
            />
        {/if}
    </div>
</div>

<style>
    .payment {
        background-color: var(--surface);
        border-radius: var(--small);
        padding: var(--small);
    }

    .col,
    .row {
        gap: var(--small);
        justify-content: space-between;
        align-items: stretch;
    }

    .col {
        display: flex;
        flex-direction: column;
    }

    .row {
        display: flex;
        flex-direction: row;
    }

    .end {
        align-items: flex-end;
    }

    .center {
        align-items: center;
    }

    .start {
        align-items: flex-start;
    }
</style>
