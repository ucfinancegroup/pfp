export function dateAsInputString(d: Date) {
    let month = '' + (d.getUTCMonth() + 1),
        day = '' + d.getUTCDate(),
        year = d.getUTCFullYear();

    if (month.length < 2)
        month = '0' + month;
    if (day.length < 2)
        day = '0' + day;

    return [year, month, day].join('-');
}

export function addDays(toDate: Date, days: number) {
    const date = new Date(toDate);
    date.setDate(date.getDate() + days);
    return date;
}

export function getValue(value: number) {
    return value;
}

export function formatPrice(price: number) {
    const v = getValue(price);
    if (v < 0)
        return "-$" + (-1 * v).toFixed(2).replace(/\B(?=(\d{3})+(?!\d))/g, ",");
    return "$"+ v.toFixed(2).replace(/\B(?=(\d{3})+(?!\d))/g, ",");
}
