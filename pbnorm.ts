#!/usr/bin/env -S deno run --allow-read --allow-write

const re = /^PB_Kontoauszug_KtoNr_\d{10}_(\d{2})-(\d{2})-(\d{4})_\d{6}.pdf$/;

export function pbnorm() {
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
}

if (import.meta.main) {
    pbnorm();
}