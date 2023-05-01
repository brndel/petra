<script lang="ts">
    import { apiPost } from "$lib/api";
    import EditablePaymentView from "$lib/components/EditablePaymentView.svelte";
    import Headerbar from "$lib/components/Headerbar.svelte";
    import Icon from "$lib/components/Icon.svelte";
    import type { EditablePayment } from "$lib/data_types";
    import {
        addEditablePayment,
        editablePayments,
        loadTinkPayments,
        tinkTokenTimeout,
    } from "$lib/db";
    import { getMonthList } from "$lib/text";
    import { paymentHasError } from "$lib/util";

    let now = new Date();

    $: tokenTimeout = $tinkTokenTimeout;
    $: minuteDiff =
        tokenTimeout === undefined || tokenTimeout === null
            ? tokenTimeout
            : (tokenTimeout.getTime() - now.getTime()) / (1000 * 60);

    let tinkPaymentMonth = now.getMonth() + 1;
    let tinkPaymentYear = now.getFullYear();

    let errorCount: number = 0;

    function updateErrorCount() {
        errorCount = payments.filter((p) => paymentHasError(p)).length;
    }

    function importPayments() {
        if (tokenTimeout === undefined || tokenTimeout === null) {
            return;
        }

        loadTinkPayments(`${tinkPaymentYear}-${tinkPaymentMonth}`);
    }

    function submitPayments() {
        updateErrorCount();

        if (errorCount === 0 && payments.length >= 1) {
            let postPayments = [];


            for(const payment of payments) {
                postPayments.push({
                    name: payment.name,
                    amount: payment.amount,
                    timestamp: payment.date,
                    users: payment.users.map((p) => p.id),
                    categories: payment.categories.map((c) => c.id),
                    tink_ref: payment.importInfo?.refHash
                });
            }

            apiPost("payment", postPayments).then(() => {
                payments = [];
                window.location.href = "/";
            })

        }
    }

    function updateEditablePayments(p: EditablePayment[]): EditablePayment[] {
        errorCount = p.filter((p) => paymentHasError(p)).length;
        return p;
    }

    $: payments = updateEditablePayments($editablePayments);
</script>

<Headerbar
    loader_data={[
        "current_user",
        "user",
        "tink/token",
        "category",
        "category_group",
    ]}
/>
<main>
    <div class="col side space">
        <div class="overview col">
            <h3>Tink</h3>
            {#if minuteDiff === undefined}
                ...
            {:else if minuteDiff === null}
                <span class="center">
                    Aktuell bist du nicht mit Tink verbunden
                </span>
                <a
                    class="tink"
                    href="https://link.tink.com/1.0/transactions/connect-accounts/?client_id=54e8e5d65f5e4339ad76321d45c0f990&redirect_uri=http%3A%2F%2Flocalhost%3A8187%2Fapi%2Ftink%2Ftoken_callback&market=DE&locale=de_DE"
                >
                    Verbinden
                </a>
            {:else}
                <div class="text-center">
                    <span>Aktuelle Verbindung hält noch</span>
                    <b>
                        {Math.floor(minuteDiff / 60)}
                        h
                        {Math.floor(minuteDiff % 60)
                            .toString()
                            .padStart(2, "0")}
                        min
                    </b>
                </div>
                <div class="col month-select">
                    <div class="row">
                        <select
                            name="month"
                            id="month-select"
                            bind:value={tinkPaymentMonth}
                        >
                            {#each getMonthList() as month}
                                <option value={month.id}>{month.name}</option>
                            {/each}
                        </select>
                        <input
                            type="number"
                            name="year"
                            id="year-input"
                            bind:value={tinkPaymentYear}
                        />
                    </div>
                    <button on:click={importPayments}>Importieren</button>
                </div>
            {/if}
        </div>
        <button on:click={addEditablePayment}> Von Hand hinzufügen </button>
    </div>
    <div class="col">
        {#if payments.length === 0}
            <span> Bitte neue zahlung erstellen oder importieren </span>
        {:else}
            {#each payments as payment}
                <EditablePaymentView
                    {payment}
                    highlightError={errorCount !== undefined}
                    onChange={updateErrorCount}
                />
            {/each}
        {/if}
    </div>
    <div class="col side">
        <div class="overview col">
            <h2>Übersicht</h2>
            <span>{payments.length} Zahlungen</span>
            {#if errorCount !== 0}
                <div class="row center error-counter">
                    <Icon icon="error" />
                    <span>
                        Noch <b>{errorCount}</b> Fehler
                    </span>
                </div>
            {/if}
            <button class="row center" on:click={submitPayments}
            disabled={errorCount !== 0}
                >Abschicken <Icon icon="send" /></button
            >
        </div>
    </div>
</main>

<style lang="scss">
    button,
    a.tink {
        background-color: var(--primary);
        color: var(--primary-on);
        text-decoration: none;
        padding: var(--small);
        border-radius: var(--small);
        border: none;
        font-weight: bold;
        cursor: pointer;
    }

    button:disabled {
        opacity: 0.5;
        cursor: not-allowed;
    }

    .month-select {
        display: flex;

        .row > * {
            width: 50%;
        }
    }

    .error-counter {
        background-color: var(--error);
        color: var(--error-on);
        padding: var(--small);
        border-radius: var(--small);
    }
</style>
