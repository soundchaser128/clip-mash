import { test, expect } from "@playwright/test";

const videoUrls = [
  "https://rule34video.com/video/3073401/wattson-on-a-date-dzooworks-4k/",
  "https://rule34video.com/video/3096413/wattson-on-holiday-break-dzooworks/",
  "https://rule34video.com/video/3057656/wattson-yeero/",
  "https://rule34video.com/video/3154125/caustic-fucking-wraith-wattson-dzooworks/",
  "https://rule34video.com/video/3053449/wraith-dzooworks/",
];

test("download some videos", async ({ page }) => {
  await page.goto("/");
  await expect(page).toHaveTitle(/ClipMash/);
  await page.getByRole("link", { name: "Start" }).click();
  await page.getByRole("link", { name: "Add videos" }).click();
  await page.getByRole("link", { name: "Folder" }).click();
  await page.getByRole("link", { name: "Download" }).click();
  await page
    .getByPlaceholder("Enter URLs separated by line")
    .fill(videoUrls.join("\n"));
  await page.getByRole("button", { name: "Download" }).click();
  await expect(page.getByText("Downloading 5 videos...")).toBeVisible();

  const images = await page.getByRole("img").all();
  expect(images).toHaveLength(videoUrls.length);
});
