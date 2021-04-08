#!/usr/bin/env -S deno run --allow-read --allow-write

/*
"PB_Kontoauszug_KtoNr_1234567890_06-01-2020_043030.pdf" \
"PB_Kontoauszug_KtoNr_1111111111_06-02-2020_043030.pdf" \
"PB_Kontoauszug_KtoNr_2222222222_06-03-2020_043030.pdf" \
"PB_Kontoauszug_KtoNr_3333333333_06-04-2020_043030.pdf" \
"PB_Kontoauszug_KtoNr_4444444444_06-05-2020_043030.pdf"
*/

const re = /^PB_Kontoauszug_KtoNr_\d{10}_(\d{2})-(\d{2})-(\d{4})_\d{6}.pdf$/;

for (const entry of Deno.readDirSync(".")) {
    if (entry.isFile) {
        const match = entry.name.match(re);
        if (match) {
            const [_, day, month, year] = match;
            const to = `Postbank-${year}-${month}-${day}.pdf`
            Deno.renameSync(entry.name, to);
        }
    }
}