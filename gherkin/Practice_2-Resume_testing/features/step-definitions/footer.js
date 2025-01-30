import { Given, Then, After } from '@cucumber/cucumber';
import { chromium } from 'playwright';
import { expect } from 'chai';

let page, context, browser;

Given('I am on the {string} page', async function (pageName) {
  browser = await chromium.launch({ headless: false });
  context = await browser.newContext();
  page = await context.newPage();

  if (pageName === 'home') {
    await page.goto('http://localhost:8000'); 
  } else if (pageName === 'skill') {
    await page.goto('http://localhost:8000/skill');
  } else {
    throw new Error(`Unknown page: ${pageName}`);
  }
});

Then('the footer should be visible', async function () {
  const footer = await page.locator('.footer'); // Ensure this selector is correct

  // Use Playwright's .isVisible() to check visibility
  const isVisible = await footer.isVisible();

  // Assert the visibility using Chai
  expect(isVisible).to.equal(true); // It should be true if the footer is visible
});

After(async function () {
  await browser.close();
});
