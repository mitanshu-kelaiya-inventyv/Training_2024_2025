import { Given, Then, When, After } from '@cucumber/cucumber';
import { chromium } from 'playwright';

let page, context, browser;

Given('I open {string}', async function (url) {
  // Launch the browser and open a new context and page
  browser = await chromium.launch({ headless: false });
  context = await browser.newContext();
  page = await context.newPage();
  await page.goto(url);
});

Then('the page title should be {string}', async function (expectedTitle) {
  // Get the actual title of the page
  const actualTitle = await page.title();
  
  // Check if the actual title matches the expected title
  if (actualTitle !== expectedTitle) {
    throw new Error(`Expected title to be "${expectedTitle}", but got "${actualTitle}"`);
  }
});

When('I navigate to {string}', async function (url) {
  // Navigate to the specified URL
  await page.goto(url);
  //await browser.close();
});

