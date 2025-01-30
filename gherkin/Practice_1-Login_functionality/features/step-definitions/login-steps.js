const { Given, When, Then , After, Before} = require('@cucumber/cucumber');
const { chromium } = require('playwright');

let browser, page;

// Given('I open the login page', async function () {
//   browser = await chromium.launch({ headless: false });
//   const context = await browser.newContext();
//   page = await context.newPage();
//   await page.goto('http://localhost:3000');
// });

// When('I enter valid credentials', async function () {
//   // Fill the username field
//   await page.fill('input[name="username"]', 'user');
  
//   // Fill the password field
//   await page.fill('input[name="password"]', 'pass');
  
//   // Wait for 2 seconds (you can adjust the time as needed)
//   await page.waitForTimeout(2000);  // Wait for 2 seconds
  
//   // Click the submit button
//   await page.click('button[type="submit"]');
// });



// When('I enter invalid credentials', async function () {
//   await page.fill('input[name="username"]', 'invalidUser');
//   await page.fill('input[name="password"]', 'invalidPass');
//   await page.click('button[type="submit"]');
// });

// Then('I should be redirected to the home page', async function () {
//   await page.waitForSelector('h1', { text: 'Welcome to the Home Page!' });
//   await browser.close();
// });

// Then('I should see an error message', async function () {
//   await page.waitForSelector('p', { text: 'Invalid credentials' });
//   await browser.close();
// });

Before(async function () {
  browser = await chromium.launch({ headless: false });
  const context = await browser.newContext();
  page = await context.newPage();
});

Given('I open the login page', async function () {
  await page.goto('http://localhost:3000'); // Replace with your actual login URL
});

When('I enter {string} and {string}', async function (username, password) {
  await page.fill('input[name="username"]', username);
  await page.fill('input[name="password"]', password);
  await page.click('button[type="submit"]');
});


Then('I should see {string}', async function (expectedMessage) {
  if (expectedMessage === 'Welcome to the Home Page!') {
    await page.waitForSelector('h1', { hasText: expectedMessage });
  } else {
    await page.waitForSelector('p', { hasText: expectedMessage });
  }
});

After(async function () {
  await browser.close();
});
