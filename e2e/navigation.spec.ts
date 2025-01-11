import { test, expect } from '@playwright/test'
import { authenticatedStoreState } from './fixtures/store-state'

test('basic new user flow - create nostr account', async ({ page }, testInfo) => {
    // Go to the home page
    await page.goto('/')
    await page.waitForLoadState('networkidle')

    // Initial setup and authentication
    await expect(page.getByText('Rent to own')).toBeVisible()
    await expect(page).toHaveURL('/introduction')
    await page.getByRole('button', { name: 'Create Account' }).click()
    await expect(page.getByText('NOSTR account created!')).toBeVisible()
    await page.getByRole('button', { name: 'Continue with RESIN' }).click()

    // Wait for Pinia stores to be initialized
    await page.waitForFunction(() => {
        return window.localStorage.getItem('nostr-store') !== null && 
               window.localStorage.getItem('properties') !== null;
    });

    // Select commercial property option
    await page.getByText('Commercial Property').click()

    // Click choose location
    await page.getByRole('button', { name: 'Choose Location' }).click()

    // Verify redirect to map
    await expect(page).toHaveURL('/map')
    await page.waitForLoadState('networkidle')

    // Verify and interact with property finding modal
    await expect(page.getByText("Let's find a property")).toBeVisible()
    await page.getByRole('button', { name: 'Continue' }).click()

    // Wait for map and all tiles to be fully loaded
    await page.waitForFunction(() => {
        const map = document.querySelector('.maplibregl-canvas');
        const loading = document.querySelector('.maplibregl-loading');
        return map && 
               !loading && 
               window.getComputedStyle(map).opacity === '1';
    }, { timeout: 30000 });
    await page.waitForTimeout(1000);

    // Take and attach map screenshot
    const mapScreenshot = await page.screenshot();
    await testInfo.attach('map-view', { body: mapScreenshot, contentType: 'image/png' });

    // Click View Properties button (with dynamic number)
    const viewPropertiesButton = page.getByRole('button', { name: /View \d+ properties/ })
    await expect(viewPropertiesButton).toBeVisible()
    await viewPropertiesButton.click()

    // Verify redirect to properties page
    await expect(page).toHaveURL('/properties')

    // Wait until images are loaded
    await page.waitForFunction(() => {
        const images = Array.from(document.querySelectorAll('img'));
        return images.every(img => img.complete);
    });

    // Click Details button on first property
    await page.getByRole('button', { name: 'Details' }).first().click()

    // Wait until images are loaded
    await page.waitForFunction(() => {
        const images = Array.from(document.querySelectorAll('img'));
        return images.every(img => img.complete);
    });

    // Take and attach final screenshot
    const finalScreenshot = await page.screenshot();
    await testInfo.attach('property-details-view', { body: finalScreenshot, contentType: 'image/png' });
}) 

test('navigate through settings menu', async ({ page }, testInfo) => {
    // Set up the initial localStorage state
    await page.goto('/');
    await page.evaluate((state) => {
        // Set each store in localStorage
        for (const [key, value] of Object.entries(state)) {
            localStorage.setItem(key, JSON.stringify(value));
        }
    }, authenticatedStoreState);

    // Start from home page with fixture state
    await page.goto('/home')
    await page.waitForLoadState('networkidle')

    // Click the profile icon in the bottom bar (last icon)
    await page.locator('.fixed.inset-x-0.bottom-0 a').last().click()
    await expect(page.getByText('Account')).toBeVisible()

    // Test navigation through menu items
    const menuTests = [
        { name: 'Profile', expectedUrl: '/settings/profile' },
        { name: 'Messages', expectedUrl: '/messages' },
        { name: 'Nostr Keys', expectedUrl: '/settings/nostr-keys' },
        { name: 'Settings', expectedUrl: '/settings' },
        { name: 'Help', expectedUrl: '/help' }
    ]

    for (const { name, expectedUrl } of menuTests) {
        // Open menu if not already open
        await expect(page.getByText('Account')).toBeVisible()

        await page.waitForLoadState('networkidle')
         
        
        // Click menu item
        await page.getByRole('link', { name }).click()
        // Wait for page transition to complete
        // Verify URL
        await expect(page).toHaveURL(expectedUrl)
        
        await page.waitForTimeout(500) // Wait for transitions to complete

        // Take and attach screenshot of each menu page
        const screenshot = await page.screenshot();
        await testInfo.attach(`menu-${name.toLowerCase()}-view`, { body: screenshot, contentType: 'image/png' });

        await page.goto('/home')
        await page.waitForLoadState('networkidle')

        // Click the profile icon in the bottom bar (last icon)
        await page.locator('.fixed.inset-x-0.bottom-0 a').last().click()
    }

    // Test Terms & Conditions link
    await page.getByRole('link', { name: 'Terms & conditions' }).click()
    await expect(page).toHaveURL('/terms-and-conditions')

    await page.goto('/home')
    await page.waitForLoadState('networkidle')

    // Test logout functionality
    await page.locator('.fixed.inset-x-0.bottom-0 a').last().click()
    await page.locator('a').filter({ hasText: 'Log out' }).click()
    await expect(page).toHaveURL(new RegExp('/|/introduction'))
}) 

