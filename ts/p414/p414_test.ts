import { assertEquals } from "https://deno.land/std@0.135.0/testing/asserts.ts";

function thirdMax(nums: number[]): number {
  let first: number = nums[0];
  let second: number | null = null;
  let third: number | null = null;

  for (const num of nums) {
    if (num > first) {
      third = second;
      second = first;
      first = num;
      continue;
    }

    if (num < first && (second == null || num > second)) {
      third = second;
      second = num;
      continue;
    }

    if (second != null && num < second && (third == null || num > third)) {
      third = num;
      continue;
    }
  }

  return (third != null ? third : first);
}

Deno.test("p414 smoke test", () => {
  const nums = [2, 2, 3, 1];
  assertEquals(thirdMax(nums), 1);
});

Deno.test("should return max when no third", () => {
  const nums = [1, 2];
  assertEquals(thirdMax(nums), 2);
});

Deno.test("p414 smoke test2", () => {
  const nums = [1, 2, -2147483648];
  assertEquals(thirdMax(nums), -2147483648);
});
