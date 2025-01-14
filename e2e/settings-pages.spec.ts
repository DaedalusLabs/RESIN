import { test, expect } from "@playwright/test";
import { authenticatedStoreState } from "./fixtures/store-state";

test.describe("settings pages", () => {
   test.beforeEach(async ({ page }) => {
      // Set up the initial localStorage state
      await page.goto("/");
      await page.evaluate((state) => {
         // Set each store in localStorage
         for (const [key, value] of Object.entries(state)) {
            localStorage.setItem(key, JSON.stringify(value));
         }
      }, authenticatedStoreState);
   });

   test("general settings functionality", async ({ page }) => {
      // Navigate to settings page
      await page.goto("/settings");
      await page.waitForLoadState("networkidle");

      // Verify page title
      await expect(
         page.getByRole("heading", { name: "Settings" }),
      ).toBeVisible();

      // Test currency selector
      const currencySelect = page.locator("select").first();
      await expect(currencySelect).toBeDisabled();
      await expect(currencySelect).toHaveValue("USD");

      // Test language selector
      const languageSelect = page.locator("select").nth(1);
      await expect(languageSelect).toBeEnabled();
      await expect(languageSelect).toHaveValue("en-US");

      //   // Test push notifications toggle
      //   const pushNotificationsSwitch = page.getByTestId("push-notifications-switch");
      //   await expect(pushNotificationsSwitch.locator("input")).toBeVisible();
      //   const initialState = await pushNotificationsSwitch.locator("input").isChecked();
      //   await pushNotificationsSwitch.locator("input").click();
      //   await expect(pushNotificationsSwitch).toBeChecked({ checked: !initialState });
   });

   test("profile settings functionality", async ({ page }) => {
      // Navigate to profile settings
      await page.goto("/settings/profile");
      await page.waitForLoadState("networkidle");

      const randomSuffix = Math.random().toString(36).substring(2, 15);
      const randomName = `Test User ${randomSuffix}`;
      const randomAbout = `This is a test bio ${randomSuffix}`;

      // Verify page title
      await expect(
         page.getByRole("heading", { name: "Profile" }),
      ).toBeVisible();

      // Test profile picture section
      const profilePicture = page.locator("img").first();
      await expect(profilePicture).toBeVisible();

      // Test display name input
      const nameInput = page.getByRole("textbox").first();
      await nameInput.click();
      await nameInput.fill(randomName);

      // Test about textarea
      const aboutTextarea = page.locator("textarea");
      await aboutTextarea.click();
      await aboutTextarea.fill(randomAbout);

      // Test save functionality
      await page.getByRole("button", { name: "Save Changes" }).click();
      await expect(page.getByText("Profile saved")).toBeVisible();

      // Come back to profile page
      // Navigate to profile settings
      await page.goto("/settings/profile");
      await page.waitForLoadState("networkidle");
      await page.getByRole("heading", { name: "Profile" }).click();
      await expect(page).toHaveURL("/settings/profile");

      // Verify Display name is updated
      await expect(nameInput).toHaveValue(randomName);

      // Verify about is updated
      await expect(aboutTextarea).toHaveValue(randomAbout);
   });

   test("nostr keys functionality", async ({ page, browserName }) => {
      // Navigate to nostr keys page
      await page.goto("/settings/nostr-keys");
      await page.waitForLoadState("networkidle");

      // Verify page title
      await expect(
         page.getByRole("heading", { name: "Nostr Keys" }),
      ).toBeVisible();

      // Test npub section
      const npubValue = page.locator(".font-mono").first();
      await expect(npubValue).toBeVisible();

      if (browserName === "chromium") {
         const npubCopyButton = page.getByRole("button").first();
         await npubCopyButton.click();
         await expect(page.getByText("Copied")).toBeVisible();
      }

      // Test public key section
      const publicKeyValue = page.locator(".font-mono").nth(1);
      await expect(publicKeyValue).toBeVisible();

      if (browserName === "chromium") {
         const publicKeyCopyButton = page.getByRole("button").nth(1);
         await publicKeyCopyButton.click();
         await expect(page.getByText("Copied")).toBeVisible();
      }

      // Test recovery phrase section
      const showPhraseButton = page.getByRole("button", { name: "Show" });
      await expect(showPhraseButton).toBeVisible();
      await showPhraseButton.click();

      // Verify recovery phrase is shown
      const recoveryPhraseGrid = page.locator(".grid-cols-2");
      await expect(recoveryPhraseGrid).toBeVisible();
      await expect(page.locator("text=ring")).toBeVisible();

      // Test hiding recovery phrase
      const hidePhraseButton = page.getByRole("button", { name: "Hide" });
      await hidePhraseButton.click();
      await expect(page.locator("text=ring")).not.toBeVisible();
   });

   test("navigation between settings pages", async ({ page }) => {
      // Start from main settings
      await page.goto("/settings");
      await page.waitForLoadState("networkidle");

      // Navigate to profile and verify
      await page.goto("/settings/profile");
      await expect(page).toHaveURL("/settings/profile");
      await expect(
         page.getByRole("heading", { name: "Profile" }),
      ).toBeVisible();

      // Navigate to nostr keys and verify
      await page.goto("/settings/nostr-keys");
      await expect(page).toHaveURL("/settings/nostr-keys");
      await expect(
         page.getByRole("heading", { name: "Nostr Keys" }),
      ).toBeVisible();

      // Test back navigation
      await page.getByRole("link").first().click();
      await expect(page).not.toHaveURL("/settings/nostr-keys");
   });
});
