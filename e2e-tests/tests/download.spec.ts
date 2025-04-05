import { test, expect } from "@playwright/test";
import { restartServer } from "../lib/server";

const videoUrls = [
  "https://rule34video.com/video/3057656/wattson-yeero/",
  "https://rule34video.com/video/3154125/caustic-fucking-wraith-wattson-dzooworks/",
  "https://rule34video.com/video/3053449/wraith-dzooworks/",
];

test("download some videos", async ({ page }) => {
  await restartServer();

  await page.goto("/");
  await expect(page).toHaveTitle(/ClipMash/);
  await page.getByRole("link", { name: "Start" }).click();
  await page.getByRole("link", { name: "Add videos" }).click();
  await page.getByRole("link", { name: "Download" }).click();
  await page
    .getByPlaceholder("Enter URLs separated by line")
    .fill(videoUrls.join("\n"));

  await page.getByRole("button", { name: "Add tag(s) to all videos" }).click();
  await page.getByPlaceholder("Enter new tag").fill("Rule 34");
  await page
    .getByTestId("modal-content")
    .getByRole("button", { name: "Add" })
    .click();

  await page.getByRole("button", { name: "Download" }).click();
  const loader = page.getByText(`Downloading ${videoUrls.length} videos...`);
  await expect(loader).toBeVisible();

  // wait until the loader disappears
  await expect(loader).not.toBeVisible({
    timeout: 120 * 1000,
  });

  expect(page.url()).toContain("/library");

  const images = await page.getByRole("img").all();
  expect(images).toHaveLength(videoUrls.length);
});
