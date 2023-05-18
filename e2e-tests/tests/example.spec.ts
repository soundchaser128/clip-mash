import { test, expect } from "@playwright/test";
import path from "path";

const VIDEO_PATH = path.resolve(process.cwd(), "videos");

test("has title", async ({ page }) => {
  await page.goto('/');
  await expect(page).toHaveTitle(/ClipMash/);
  const button = page.getByRole("button", { name: "Local" });
  await button.click();
  await expect(page).toHaveURL(`/local/path`);

  await page.getByLabel("Compilation name").fill("Test compilation");
  await page.getByRole("button", { name: "Submit" }).click();

  await expect(page).toHaveURL(`/local/path`);

  await page.getByLabel("Folder containing your videos").fill(VIDEO_PATH);
  await page.getByRole("button", { name: "Submit" }).click();

  await expect(page).toHaveURL(`/local/videos`);
});
