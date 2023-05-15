export type User = {
    id: number,
    name: string,
    userName: string,
    icon: string,
}

export type CategoryGroup = {
    id: number,
    name: string,
    icon: string,
}

export type Category = {
    id: number,
    name: string,
    icon: string,
    group: CategoryGroup
}

export type Rule = {
    id: number,
    name: string,
    categories: Category[]
    keywords: string[],
    shareRule: boolean | null,
}

export type Payment = {
    id: number,
    name: string,
    realAmount: number,
    amount: number,
    repayAmount: number,
    timestamp: Date,
    isOwner: boolean,
    owner: User,
    users: User[],
    categories: Category[]
}

export type EditablePayment = {
    name: string,
    amount: number,
    date: Date,
    users: User[],
    categories: Category[],
    rule?: Rule,

    importInfo?: EditablePaymentImportInfo,
}

export type EditablePaymentImportInfo = {
    nameRaw: string,
    refHash: string,
}

export type MonthData = {
    // "2023-12"
    month: string,
    positive: number,
    negative: number,
    repay: number
}

export type TinkImportInfo = {
    new: number,
    pending: number,
    listed: number
}