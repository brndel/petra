import { readable, writable, type Readable, type Subscriber, type Writable } from "svelte/store";
import type { Category, CategoryGroup, EditablePayment, MonthData, Payment, TinkImportInfo, User } from "./data_types.js";
import { apiGet } from "./api.js";

type RawData = { [key: string]: any };

// Stores internal

let rawUsers = writable<RawData[]>(undefined);
let rawMonths = writable<RawData>(undefined);
let rawCategoryGroups = writable<RawData[]>(undefined);
let rawCategories = writable<RawData[]>(undefined);

let rawCurrentUser = writable<RawData | null>(undefined);
let rawPayments = writable<RawData[]>(undefined);

let rawTinkToken = writable<RawData | null>(undefined);
let rawTinkPayments = writable<RawData>(undefined);

rawTinkPayments.subscribe((rawTinkPayments) => {
    if (rawTinkPayments === undefined) {
        return;
    }

    tinkImportInfoWrite.set({
        listed: rawTinkPayments.listed,
        pending: rawTinkPayments.pending,
    });

    let payments: EditablePayment[] = [];
    for (const payment of rawTinkPayments.new) {
        payments.push({
            name: payment.name,
            amount: payment.amount,
            date: new Date(payment.date),
            users: [],
            categories: [],
            rule: null,
            importInfo: {
                nameRaw: payment.name_raw,
                refHash: payment.ref_hash
            }
        })
    }
    editablePaymentsWrite.set(payments);
});

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

let monthIndex = readable<Map<string, MonthData> | undefined>(undefined, (set) => {
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

let users = readable<Map<number, User> | undefined>(undefined, (set) => {
    rawUsers.subscribe((rawUsers) => {
        set(
            rawToMapId(rawUsers, (raw) => {
                let data: User = {
                    id: raw.id,
                    userName: raw.name,
                    name: raw.display_name,
                    icon: "person"
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
        return users.get(rawCurrentUser["user"]);
    })
});

let categoryGroups = readable<Map<number, CategoryGroup> | undefined>(undefined, (set) => {
    rawCategoryGroups.subscribe((rawCategoryGroups) => {
        if (rawCategoryGroups === undefined) {
            set(undefined);
            return;
        }
        set(
            rawToMapId(rawCategoryGroups, (raw) => {
                let data: CategoryGroup = {
                    id: raw.id,
                    name: raw.name,
                    icon: raw.icon,
                }
                return data;
            })
        );
    });
});

let categories = readable<Map<number, Category> | undefined>(undefined, (set) => {
    multiSubscribe2([rawCategories, categoryGroups], set, ([rawCategories, categoryGroups]) => {
        let map = rawToMapId(rawCategories, (raw) => {
            let data: Category = {
                id: raw.id,
                name: raw.name,
                icon: raw.icon,
                group: categoryGroups.get(raw.group_id)!
            };
            return data;
        });

        return map;
    });
});

let payments = readable<Map<number, Payment> | undefined>(undefined, (set) => {
    multiSubscribe3([rawPayments, users, categories], set, ([rawPayments, users, categories]) => {
        let map = rawToMapId(rawPayments, (raw) => {

            let data: Payment = {
                id: raw.id,
                name: raw.name,
                amount: raw.amount,
                repayAmount: raw.repay_amount,
                timestamp: new Date(raw.timestamp),
                owner: users.get(raw.owner_id)!,
                users: raw.users.map((id: number) => users.get(id)!),
                categories: raw.categories.map((id: number) => categories.get(id)!),
            }


            return data;
        });

        // console.log("payments", map);

        return map;
    })
})

let tinkImportInfoWrite = writable<TinkImportInfo | undefined>(undefined);

export let tinkImportInfo = tinkImportInfoWrite as Readable<TinkImportInfo | undefined>;

let editablePaymentsWrite = writable<EditablePayment[]>([]);

export let editablePayments = editablePaymentsWrite as Readable<EditablePayment[]>;

export function addEditablePayment() {
    let payment: EditablePayment = {
        name: "",
        amount: 0,
        date: new Date(),
        users: [],
        categories: [],
        rule: null,
        importInfo: undefined,
    }
    editablePaymentsWrite.update((payments) => payments.concat(payment))
}

// Tink

export let tinkTokenTimeout = readable<Date | null | undefined>(undefined, (set) => {
    rawTinkToken.subscribe((rawTinkToken) => {
        // console.log("rawTinkToken", rawTinkToken);

        if (rawTinkToken === null || rawTinkToken === undefined) {
            set(rawTinkToken);
        } else {
            let timestamp = rawTinkToken.expires_timestamp;
            set(new Date(timestamp));
        }
    })
});

// Array readables

export let monthIndexArray = readable<MonthData[]>([], (set) => {
    monthIndex.subscribe((months) => {
        if (months === undefined) {
            set([]);
            return;
        }

        set(Array.from(months.values()));
    })
});

export let usersArray = readable<User[]>([], (set) => {
    users.subscribe((users) => {
        if (users === undefined) {
            set([]);
            return;
        }

        set(Array.from(users.values()))
    })
});


export let categoriesArray = readable<Category[]>([], (set) => {
    categories.subscribe((categories) => {
        if (categories === undefined) {
            set([]);
            return;
        }

        set(Array.from(categories.values()))
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

export type LoaderMethod = "current_user" | "user" | "category_group" | "category" | "month_index" | "payment" | "tink/token" | "tink/payment";

function getSetter(method: LoaderMethod): Writable<any> | undefined {
    switch (method) {
        case "current_user": return rawCurrentUser;
        case "user": return rawUsers;
        case "category_group": return rawCategoryGroups;
        case "category": return rawCategories;
        case "month_index": return rawMonths;
        case "payment": return rawPayments;
        case "tink/token": return rawTinkToken;
        case "tink/payment": return rawTinkPayments;
    }
}
export async function loadDB(method: LoaderMethod, args?: {}) {
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

export async function loadPayments(month: MonthData) {
    await loadDB("payment", { "month": month.month });
}

export async function loadTinkPayments(month: string) {
    await loadDB("tink/payment", { "month": month });
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

function multiSubscribe3<T,
    D0, D1, D2,
    D = [D0, D1, D2]
>(
    readables: [Readable<D0 | undefined>, Readable<D1 | undefined>, Readable<D2 | undefined>],
    set: Subscriber<T | undefined>,
    update: (data: D) => T | undefined) {
    let data: [D0 | undefined, D1 | undefined, D2 | undefined] = [undefined, undefined, undefined];

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