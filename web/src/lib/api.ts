function createUrl(method: string): string {
    return `api/${method}`;
}

export async function apiGet(method: string, args: {} | null = null): Promise<any> {
    let url = createUrl(method);
    if (args !== null) {
        url += "?" + new URLSearchParams(args);
    }
    let response = await fetch(url, { method: "GET" });
    return await response.json();
}

export async function apiPost(method: string, args: {} | null = null): Promise<any> {
    let url = createUrl(method);
    let response = await fetch(url, { method: "POST", body: JSON.stringify(args) });
    return await response.json();
}