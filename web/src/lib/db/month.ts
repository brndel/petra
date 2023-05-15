import { readable, writable } from "svelte/store";
import { rawToMap, type RawData } from "./helpers";
import type { MonthData } from "$lib/data_types";

export let rawMonths = writable<RawData>(undefined);

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

let monthMap = readable<Map<string, MonthData> | undefined>(undefined, (set) => {
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

export let monthArray = readable<MonthData[]>([], (set) => {
    monthMap.subscribe((monthMap) => {
        if (monthMap === undefined) {
            set([]);
            return;
        }

        set(Array.from(monthMap.values()));
    })
});