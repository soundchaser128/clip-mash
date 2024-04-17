import { test, expect } from "@playwright/test";

test("has title", async ({ page }) => {
  await page.goto("/");
  await expect(page).toHaveTitle(/ClipMash/);
  await page.getByRole("link", { name: "Start" }).click();
  await page.getByRole("link", { name: "Add videos" }).click();
  await page.getByRole("link", { name: "Folder" }).click();
});
