export function dateAsInputString(d: Date) {
    let month = '' + (d.getMonth() + 1),
        day = '' + d.getDate(),
        year = d.getFullYear();

    if (month.length < 2)
        month = '0' + month;
    if (day.length < 2)
        day = '0' + day;

    return [year, month, day].join('-');
}

export function addDaysToToday(days: number) {
    const date = new Date(); // Now
    date.setDate(date.getDate() + days);
    return date;
}
