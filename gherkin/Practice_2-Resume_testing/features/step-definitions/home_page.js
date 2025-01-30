import { Given, When, Then } from '@cucumber/cucumber';
import  { expect } from 'chai';
import { chromium } from 'playwright';

let browser, page;

Given('I navigate to the home page', async function () {
    browser = await chromium.launch({ headless: false }); // Set to true for headless mode
    page = await browser.newPage();
    await page.goto('http://localhost:8000', { timeout: 5000 }); // Enforce 5000ms timeout for loading the page
});

Then('the background image should load correctly', async function () {
    await page.waitForTimeout(2000); // Wait for 2 seconds to let the image load
    await page.waitForSelector('.bgImage', { timeout: 5000 });  // Wait for the image with class 'bgImage' to load

    // Get the src attribute of the image
    const imgSrc = await page.locator('.bgImage').getAttribute('src');
    console.log('Image Source:', imgSrc);
    
    // Assert that the src is not empty and contains the correct URL
    expect(imgSrc).to.not.be.empty;  // Ensure the src attribute is not empty
    expect(imgSrc).to.include('http');  // Ensure the src attribute contains 'http', assuming it's a full URL

    
});

Then('the background image width should be 100vw', async function () {
    const backgroundWidth = await page.locator('.bgImage').evaluate(
        el => getComputedStyle(el).width,  // Get the width in px
        { timeout: 5000 }
    );
    
    // Convert the viewport width to pixels (this is the expected width)
    const viewportWidth = await page.viewportSize().width;  // Get the current viewport width in pixels
    
    // Compare the image width with the viewport width (with a tolerance)
    const tolerance = 10;  // Set tolerance for slight differences in rendering
    const expectedWidth = `${viewportWidth}px`;
    
    console.log('Image Width:', backgroundWidth);
    console.log('Expected Width:', expectedWidth);

    console.log(viewportWidth, expectedWidth);
    
    // Assert that the width is close to 100vw in pixels
    expect(parseInt(backgroundWidth)).to.be.closeTo(parseInt(expectedWidth), tolerance);
});


Then('the opacity of the background image should be 0.7', async function () {
    const backgroundOpacity = await page.locator('.bgImage').evaluate(
        el => getComputedStyle(el).opacity,
        { timeout: 5000 }
    );

    // Assert that the opacity is 0.7
    expect(backgroundOpacity).to.equal('0.7');
    
    // Close the browser once the test is completed
    
});


Given('I am on the home page', async function () {
    await page.goto('http://localhost:8000'); // Update with your actual URL
});

Then('the personal section should be displayed with profile image', async function () {
    const personalSection = page.locator('.personal');
    const isVisible = await personalSection.isVisible();  // Check visibility using Playwright's API
    expect(isVisible).to.be.true;  // Use Chai to assert that the element is visible
});



Then('the profile image should have a height and width of 200px', async function () {
    const profileImage = await page.locator('.personalImg');

    const imageHeight = await profileImage.evaluate(el => getComputedStyle(el).height);
    const imageWidth = await profileImage.evaluate(el => getComputedStyle(el).width);

    expect(imageHeight).to.equal('200px');
    expect(imageWidth).to.equal('200px');
});

Then('the profile image should be circular', async function () {
    const profileImage = await page.locator('.personalImg');
    const borderRadius = await profileImage.evaluate(el => getComputedStyle(el).borderRadius);
    
    expect(borderRadius).to.equal('50%');
});

Then('the personal name should be displayed in black', async function () {
    const personalName = await page.locator('.personalName');
    
    const color = await personalName.evaluate(el => getComputedStyle(el).color);
    const fontSize = await personalName.evaluate(el => getComputedStyle(el).fontSize);
    
    expect(color).to.equal('rgb(0, 0, 0)');
});

Then('the personal title should be displayed in black', async function () {
    const personalTitle = await page.locator('.personaltitle');
    
    const color = await personalTitle.evaluate(el => getComputedStyle(el).color);
    const fontSize = await personalTitle.evaluate(el => getComputedStyle(el).fontSize);
    
    expect(color).to.equal('rgb(0, 0, 0)');
});

Then('the "Download CV" button should be visible and clickable', async function () {
    const cvButton = page.locator('.cvbutton');
    
    // Check if the button is visible using Playwright
    const isVisible = await cvButton.isVisible();
    expect(isVisible).to.be.true;

    // Check if the button is enabled (clickable) using Playwright
    const isEnabled = await cvButton.isEnabled();
    expect(isEnabled).to.be.true;

    // Optionally, simulate clicking the button
    await cvButton.click();

    // Close the browser after the test completes
    await page.close();
});
