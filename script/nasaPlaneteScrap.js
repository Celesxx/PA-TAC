const puppeteer = require('puppeteer');
const fs = require('fs');
const path = require('path');


function wait(ms) { return new Promise(resolve => setTimeout(resolve, ms)); }


(async () => 
{
    
    const agents = [
        'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/85.0.4183.121 Safari/537.36',
        'Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_6) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/14.0.3 Safari/605.1.15',
    ];
    
    
    const exoplanetsDict = require('./exoplanets_data.json'); // Assurez-vous que le chemin est correct
    const imagesDir = 'img';
    fs.mkdirSync(imagesDir, { recursive: true });
    
    let i = 0;
    for (let planetName in exoplanetsDict) 
    {
        const planetUrl = exoplanetsDict[planetName];
        try 
        {
            console.log("---------------------------------------------------");
            console.log("Status : loading url");
            const browser = await puppeteer.launch(
            {
                headless: true, // Pour voir ce qui se passe, mettez à true pour mode sans tête
                args: ['--use-gl=desktop', '--enable-webgl', '--window-size=1920,1080']
            });
            const page = await browser.newPage();
            await page.setViewport({ width: 1920, height: 1080 });
            // await page.goto(planetUrl);
            // await page.waitForTimeout(8000);
            await page.setUserAgent(agents[i % 2]);
            await page.goto(planetUrl, { waitUntil: 'domcontentloaded', timeout: 60000 });
            await wait(1000);
            console.log("Status : url loaded");
            
            
            console.log("Status : searching iframe");
            const frameHandle = await page.$('iframe');
            const frame = await frameHandle.contentFrame();
            console.log("Status : iframe found");
            
            console.log("Status : loading canvas");
            await frame.waitForSelector('.loading', { hidden: true });
            console.log("Status : canvas loaded");
            
            console.log("Status : remove label and shadow");
            const footerDivs = await frame.$$('.footerSVGDiv');
            if (footerDivs.length >= 2) {
                await footerDivs[0].click(); // Simulate clicks on specified elements
            }
            
            console.log("Status : remove ui class");
            await frame.evaluate(() => {
                const uiElements = document.querySelectorAll('.ui');
                uiElements.forEach(el => el.style.display = 'none');
            });
            console.log("Status : class removed");
            
            console.log("Status : searching canvas");
            const canvas = await frame.$('canvas');
            console.log("Status : canvas find");
            
            console.log("Status : screenshot initialize");
            const filename = path.join(imagesDir, `${planetName.replace('/', '_').replace(/ /g, '-')}.png`);
            await canvas.screenshot({ path: filename });
            await wait(2000);
            console.log(`Status : screenshot saved for ${planetName} at ${filename}`);
            i++;
            await browser.close();

        } catch (e) 
        {
            await browser.close();
            console.log("Status : error", e);
        }
    }

    console.log("All captures are saved.");
})();
