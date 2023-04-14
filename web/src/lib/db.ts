import { readable, writable, type Readable, type Subscriber, type Writable } from "svelte/store";
import type { Category, CategoryGroup, MonthData, Payment, User } from "./data_types.js";
import { apiGet } from "./api.js";

type RawData = { [key: string]: any };

// Stores internal

let rawUsers = writable<RawData[]>(undefined);
let rawMonths = writable<RawData>(undefined);

let rawCurrentUser = writable<RawData | null>(undefined);
let rawPayments = writable<RawData[]>(undefined);

// Stores external

export let sumMonth = readable<MonthData | undefined>(undefined, (set) => {
    rawMonths.subscribe((rawMonths) => {
        let raw = rawMonths == undefined ? undefined : rawMonths["sum"];
        if (raw == undefined) {
            set(undefined);
            return;
        }

        let data: MonthData = {
            month: raw.month,
            positive: raw.positive,
            negative: raw.negative,
            repay: raw.repay
        }

        set(data)
    });
});

export let months = readable<Map<string, MonthData> | undefined>(undefined, (set) => {
    rawMonths.subscribe((rawMonths) => {
        let months = rawMonths == undefined ? undefined : rawMonths["months"];
        set(
            rawToMap(months, (raw) => {
                let data: MonthData = {
                    month: raw.month,
                    positive: raw.positive,
                    negative: raw.negative,
                    repay: raw.repay
                }
                return data;
            }, (raw) => raw.month)
        );
    });
});

export let users = readable<Map<number, User> | undefined>(undefined, (set) => {
    rawUsers.subscribe((rawUsers) => {
        set(
            rawToMapId(rawUsers, (raw) => {
                let data: User = {
                    id: raw.id,
                    name: raw.name,
                    displayName: raw.display_name
                }
                return data;
            })
        );
    });
});

export let currentUser = readable<User | null | undefined>(undefined, (set) => {
    multiSubscribe2([rawCurrentUser, users], set, ([rawCurrentUser, users]) => {
        if (rawCurrentUser == null) {
            return null;
        }
        return users.get(rawCurrentUser["id"]);
    })
});

export let payments = readable<Map<number, Payment> | undefined>(undefined, (set) => {
    multiSubscribe2([rawPayments, users], set, ([rawPayments, users]) => {
        let map = rawToMap(rawPayments, (raw) => {

            let data: Payment = {
                id: raw.id,
                name: raw.name,
                amount: raw.amount,
                repayAmount: raw.repay_amount,
                timestamp: new Date(raw.timestamp),
                owner: users.get(raw.owner_id)!,
                users: raw.users.map((id: number) => users.get(id)!),
                categories: []
            }


            return data;
        }, (raw) => raw.id as number);

        // console.log("payments", map);

        return map;
    })
})

// Array readables

export let monthsArray = readable<MonthData[]>([], (set) => {
    months.subscribe((months) => {
        if (months === undefined) {
            set([]);
            return;
        }

        set(Array.from(months.values()));
    })
});


export let paymentsArray = readable<Payment[]>([], (set) => {
    payments.subscribe((payments) => {
        if (payments === undefined) {
            set([]);
            return;
        }

        set(Array.from(payments.values()));
    })
});

// Loaders

function getSetter(method: string): Writable<any> | undefined {
    switch (method) {
        case "current_user": return rawCurrentUser;
        case "user": return rawUsers;
        case "month_index": return rawMonths;
        case "payment": return rawPayments;
    }
}
export async function loadDB(method: string, args?: {}) {
    let setter = getSetter(method);
    if (setter === undefined) {
        console.log("invalid setter '" + method + "'");
        return;
    }
    await loadRawData(method, setter, args);
}

async function loadRawData(method: string, setter: Writable<any>, args?: {}) {
    let rawData = await apiGet(method, args);
    setter.set(rawData);
}

// export async function loadCurrentUser() {
//     await loadDB("current_user");
// }

// export async function loadUsers() {
//     await loadDB("user");
// }

// export async function loadMonthData() {
//     await loadDB("month_index");
// }

export async function loadPayments(month: MonthData) {
    await loadDB("payment", { "month": month.month });
}

// Subscribe helpers


function rawToMap<K, V>(rawData: RawData[], mapValue: (raw: RawData) => V, mapKey: (raw: RawData) => K): Map<K, V> | undefined {
    if (rawData === undefined) {
        return undefined;
    }
    let map = new Map<K, V>();

    for (let rawKey in rawData) {
        let rawValue = rawData[rawKey];
        let value = mapValue(rawValue);
        let key = mapKey(rawValue);
        map.set(key, value);
    }

    return map;
}

function rawToMapId<V>(rawData: RawData[], mapValue: (raw: RawData) => V): Map<number, V> | undefined {
    return rawToMap(rawData, mapValue, (raw) => raw.id as number);
}

function multiSubscribe2<T,
    D0, D1,
    D = [D0, D1]
>(
    readables: [Readable<D0 | undefined>, Readable<D1 | undefined>],
    set: Subscriber<T | undefined>,
    update: (data: D) => T | undefined) {
    let data: [D0 | undefined, D1 | undefined] = [undefined, undefined];

    function onChange() {
        for (let value of data) {
            if (value == undefined) {
                set(undefined);
                return;
            }
        }

        set(update(data as D));
    }

    for (let i = 0; i < readables.length; i++) {
        let readable = readables[i];
        readable.subscribe((v) => { data[i] = v; onChange(); });
    }
}