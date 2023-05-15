import type { MonthData } from "./data_types";

export const repay = "Rückzahlung"

export function getMonthList(): { id: number, name: string }[] {
    return [
        { id: 1, name: "Januar" },
        { id: 2, name: "Februar" },
        { id: 3, name: "März" },
        { id: 4, name: "April" },
        { id: 5, name: "Mai" },
        { id: 6, name: "Juni" },
        { id: 7, name: "Juli" },
        { id: 8, name: "August" },
        { id: 9, name: "September" },
        { id: 10, name: "Oktober" },
        { id: 11, name: "November" },
        { id: 12, name: "Dezember" }
    ]

}

export function getMonthName(month: MonthData | null): string {
    if (month === null) {
        return ""
    }
    
    if (month.month === "*") {
        return "Gesamt"
    }

    let parts = month.month.split("-");
    let yearPart: string = parts[0];
    let monthPart: string = parts[1];
    let monthNumber = parseInt(monthPart);

    function getName(monthNumber: number): string {
        switch (monthNumber) {
            case 1: return "Januar";
            case 2: return "Februar";
            case 3: return "März";
            case 4: return "April";
            case 5: return "Mai";
            case 6: return "Juni";
            case 7: return "Juli";
            case 8: return "August";
            case 9: return "September";
            case 10: return "Oktober";
            case 11: return "November";
            case 12: return "Dezember";
        }
        return "?"
    }

    let monthName = getName(monthNumber);

    return `${monthName} ${yearPart}`;
}

export function formatDate(date: Date): string {
    return `${date.getDate()}.${date.getMonth() + 1}`;
    // ${date.getHours()}:${date
    //     .getMinutes()
    //     .toString()
    //     .padStart(2, "0")}
}