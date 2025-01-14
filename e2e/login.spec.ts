import { test, expect } from "@playwright/test";

test("login with NOSTR recovery phrase", async ({ page }, testInfo) => {
   // Go to the home page
   await page.goto("/");
   await page.waitForLoadState("networkidle");

   // Click the Sign in button
   await page.getByRole("button", { name: "Log in with NOSTR" }).click();

   // Click "Use recovery phrase" option
   await page.getByRole("button", { name: "Use recovery phrase" }).click();

   // Enter the recovery phrase
   const words = [
      "ring",
      "fine",
      "foot",
      "behind",
      "blouse",
      "ten",
      "brown",
      "apart",
      "tuition",
      "toddler",
      "fabric",
      "emotion",
   ];

   // Take screenshot of recovery phrase form
   const loginScreenshot = await page.screenshot();
   await testInfo.attach("recovery-phrase-form", {
      body: loginScreenshot,
      contentType: "image/png",
   });

   // Type each word into its respective input
   for (let i = 0; i < words.length; i++) {
      await page.locator(`input[placeholder="Word ${i + 1}"]`).fill(words[i]);
   }

   // Click continue
   await page.getByRole("button", { name: "Log in", exact: true }).click();

   // Wait for login to complete and redirect
   await page.waitForLoadState("networkidle");

   // Navigate to Nostr Keys page through menu
   await page.locator(".fixed.inset-x-0.bottom-0 a").last().click();
   await expect(page.getByText("Account")).toBeVisible();
   await page.getByRole("link", { name: "Nostr Keys" }).click();

   // Verify the pubkey
   const expectedPubkey =
      "2255c59bcb5b38c05f45815fc6dfbbfe6f02e714c47b68d5e1446157002f98e4";
   await expect(page.getByText(expectedPubkey)).toBeVisible();

   // Take screenshot of Nostr Keys page showing the pubkey
   const keysScreenshot = await page.screenshot();
   await testInfo.attach("nostr-keys-view", {
      body: keysScreenshot,
      contentType: "image/png",
   });
});
