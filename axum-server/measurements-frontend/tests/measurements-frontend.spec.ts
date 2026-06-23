import { test, expect } from '@playwright/test';
import { config } from './constants';

test('has title', async ({ page }) => {
  await page.goto(config.homeUrl);
  await expect(page).toHaveTitle(/Measurements/);
});

test('has navigation', async ({ page }) => {
  await page.goto(config.homeUrl);
  await expect(page.locator('nav').getByText('Measurements')).toBeVisible();
});
