const puppeteer = require('puppeteer');
const fs = require('fs');
const path = require('path');

// function wait(ms) { return new Promise(resolve => setTimeout(resolve, ms)); }

(async () => 
{
    const agents = 
    [
        'Mozilla/5.0 (Linux; Android 10; SM-A505FN) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/88.0.4324.152 Mobile Safari/537.36',
        'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/85.0.4183.121 Safari/537.36',
        'Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_6) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/14.0.3 Safari/605.1.15',
    ];

    const exoplanetsDict = require('./exoplanets_data.json');
    fs.mkdirSync('img', { recursive: true });

    let i = 0;

    for (let planetName in exoplanetsDict) 
    {
        console.log(`Status : loading ${planetName}`);
        const planetUrl = exoplanetsDict[planetName];
        const browser = await puppeteer.launch(
        {
            headless: true,
            args: ['--use-gl=desktop', '--enable-webgl', '--window-size=1920,1080']
        });
        const page = await browser.newPage();
        await page.setViewport({ width: 1920, height: 1080 });
        
        try 
        {
            await page.setUserAgent(agents[i % agents.length]);
            await page.goto(planetUrl, { waitUntil: 'domcontentloaded', timeout: 60000 });


            //récupération catégorie
            await page.waitForSelector('tbody');
            const planetType = await page.evaluate(() => 
            {
                const row = [...document.querySelectorAll('tr.fact_row')].find(row => row.querySelector('.title').textContent.includes('PLANET TYPE'));
                return row ? row.querySelector('.value').textContent.trim() : 'Unknown';
            });

            //gestion dossier catégorie
            const typeDir = path.join('img', planetType.replace(/ /g, '_').replace('/', '_').toLowerCase());
            fs.mkdirSync(typeDir, { recursive: true });


            //gestion iframe
            const frameHandle = await page.$('iframe');
            const frame = await frameHandle.contentFrame();

            //gestion shadow
            await frame.waitForSelector('.loading', { hidden: true });
            const footerDivs = await frame.$$('.footerSVGDiv');
            if (footerDivs.length >= 2) {
                await footerDivs[0].click();
            }

            //suppréssion de l'ui
            await frame.evaluate(() => {
                const uiElements = document.querySelectorAll('.ui');
                uiElements.forEach(el => el.style.display = 'none');
            });

            //screenshot
            const canvas = await frame.$('canvas');
            const filename = path.join(typeDir, `${planetName.replace('/', '_').replace(/ /g, '-')}.png`);
            await canvas.screenshot({ path: filename });
            

        }catch (error) { console.error(`Status : error processing ${planetName}\n${error}`); } 
        finally 
        { 
            i++;
            await browser.close(); 
        }
    }
    console.log("All captures are saved.");
})();
