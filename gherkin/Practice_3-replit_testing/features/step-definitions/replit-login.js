// loginSteps.js
import { Given, When, Then, After, Before } from '@cucumber/cucumber';
import { chromium } from 'playwright';
import { expect } from 'chai';
let browser;
let page;

Before(async () => {
    // Set up browser before each scenario
    browser = await chromium.launch({ headless: false });
    page = await browser.newPage();
  });
  
  Given('the user is on the LearnXinYMinutes homepage', { timeout: 10000 },  async () => {
    await page.goto('https://replit.com/');
  });
  
  Given('the user navigates to the login page', { timeout: 15000 }, async () => {
    await page.click('a[href="/login"]'); 
  await page.waitForNavigation({ timeout: 10000 });
  });

When('the user enters a valid {string}', { timeout: 10000 }, async (username) => {
    await page.getByRole('textbox', { name: 'Email or username' }).fill(username); 
});

When('the user enters the correct {string}', { timeout: 10000 }, async (password) => {
    await page.getByRole('textbox', { name: 'Password' }).fill(password); 
});

When('the user clicks the "Log In" button', { timeout: 30000 }, async () => {
    await page.click('button[type="submit"]'); 
    // Wait for navigation to complete after clicking login
    await page.waitForNavigation({ timeout: 15000 }); 
});

Then('the user should be redirected to the Replit dashboard',{timeout: 10000}, async () => {
    await page.waitForTimeout(5000);
    const currentUrl = await page.url();

    // Ensure the user is redirected to the correct dashboard URL
    expect(currentUrl).to.equal('https://replit.com/~');
});




After(async () => {
  // Close the browser after the test
  await browser.close();
});
