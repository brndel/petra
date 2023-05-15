import { readable, writable, type Readable } from "svelte/store";
import { multiSubscribe4, rawToMapId, type RawData } from "./helpers";
import { currentUser, userMap } from "./user";
import { categoryMap } from "./category";
import type { MonthData, Payment } from "$lib/data_types";
import { loadDB } from "./loader";

export let rawPayments = writable<RawData[]>(undefined);

let selectedMonthWrite = writable<MonthData | null>(null);

export let selectedMonth = selectedMonthWrite as Readable<MonthData | null>;

export let paymentMap = readable<Map<number, Payment> | undefined>(undefined, (set) => {
    multiSubscribe4([rawPayments, userMap, categoryMap, currentUser], set, ([rawPayments, users, categories, currentUser]) => {
        let map = rawToMapId(rawPayments, (raw) => {

            let data: Payment = {
                id: raw.id,
                name: raw.name,
                realAmount: raw.real_amount,
                amount: raw.user_amount,
                repayAmount: raw.repay_amount,
                timestamp: new Date(raw.timestamp),
                isOwner: raw.owner_id == currentUser?.id,
                owner: users.get(raw.owner_id)!,
                users: raw.users.map((id: number) => users.get(id)!),
                categories: raw.categories.map((id: number) => categories.get(id)!),
            }


            return data;
        });

        // console.log("payments", map);

        return map;
    })
});



export let paymentArray = readable<Payment[]>([], (set) => {
    paymentMap.subscribe((paymentMap) => {
        if (paymentMap === undefined) {
            set([]);
            return;
        }

        set(Array.from(paymentMap.values()));
    })
});

export async function loadPayments(month: MonthData) {
    selectedMonthWrite.set(month);
    await loadDB("payment", { "month": month.month });
}