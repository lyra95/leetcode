import { assertEquals } from "https://deno.land/std@0.135.0/testing/asserts.ts";

type T = {
    letter: string;
    value: number;
}

const greeks: Array<T> = [
    { letter: "M", value: 1000 }, { letter: "CM", value: 900 },
    { letter: "D", value: 500 }, { letter: "CD", value: 400 },
    { letter: "C", value: 100 }, { letter: "XC", value: 90 },
    { letter: "L", value: 50 }, { letter: "XL", value: 40 },
    { letter: "X", value: 10 }, { letter: "IX", value: 9 },
    { letter: "V", value: 5 }, { letter: "IV", value: 4 },
    { letter: "I", value: 1 }
]

function intToRoman(num: number): string {
    for (const greek of greeks){
        if (num >= greek.value) {
            return greek.letter + intToRoman(num - greek.value);
        }
    }
    return "";
}

Deno.test("p12 smoke test", () => {
    assertEquals(intToRoman(27), "XXVII");
});
