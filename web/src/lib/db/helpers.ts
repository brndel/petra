import type { Readable, Subscriber } from "svelte/store";

export type RawData = { [key: string]: any };

export function rawToMap<K, V>(rawData: RawData[], mapValue: (raw: RawData) => V, mapKey: (raw: RawData) => K): Map<K, V> | undefined {
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

export function rawToMapId<V>(rawData: RawData[], mapValue: (raw: RawData) => V): Map<number, V> | undefined {
    return rawToMap(rawData, mapValue, (raw) => raw.id as number);
}

export function multiSubscribe2<T,
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

export function multiSubscribe3<T,
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

export function multiSubscribe4<T,
    D0, D1, D2, D3,
    D = [D0, D1, D2, D3]
>(
    readables: [Readable<D0 | undefined>, Readable<D1 | undefined>, Readable<D2 | undefined>, Readable<D3 | undefined>],
    set: Subscriber<T | undefined>,
    update: (data: D) => T | undefined) {
    let data: [D0 | undefined, D1 | undefined, D2 | undefined, D3 | undefined] = [undefined, undefined, undefined, undefined];

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