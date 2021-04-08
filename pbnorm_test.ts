import { pbnorm } from "./pbnorm.ts";

Deno.test("pbnorm", () => {
    for (const file of [
        "PB_Kontoauszug_KtoNr_1234567890_06-01-2020_043030.pdf",
        "PB_Kontoauszug_KtoNr_1111111111_06-02-2020_043030.pdf",
        "PB_Kontoauszug_KtoNr_2222222222_06-03-2020_043030.pdf",
        "PB_Kontoauszug_KtoNr_3333333333_06-04-2020_043030.pdf",
        "PB_Kontoauszug_KtoNr_4444444444_06-05-2020_043030.pdf",
    ]) {
        Deno.createSync(file).close();
    }

    pbnorm();

    for (const file of [
        "Postbank-2020-01-06.pdf",
        "Postbank-2020-02-06.pdf",
        "Postbank-2020-03-06.pdf",
        "Postbank-2020-04-06.pdf",
        "Postbank-2020-05-06.pdf",
    ]) {
        Deno.removeSync(file);
    }
});
