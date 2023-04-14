import type { MonthData } from "./data_types";

export const repay = "Rückzahlung"

export function getMonthName(month: MonthData): string {
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
