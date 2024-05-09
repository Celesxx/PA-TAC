const puppeteer = require('puppeteer');
const fs = require('fs');
const path = require('path');

function wait(ms) { return new Promise(resolve => setTimeout(resolve, ms)); }

(async () => 
{
    const agents = 
    [
        'Mozilla/5.0 (Linux; Android 10; SM-A505FN) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/88.0.4324.152 Mobile Safari/537.36',
        'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/85.0.4183.121 Safari/537.36',
        'Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_6) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/14.0.3 Safari/605.1.15',
    ];

    const exoplanetsDict = require('../data/listExoplanetsLinkCurrent.json');
    fs.mkdirSync('../data/dataset', { recursive: true });

    let i = 0;

    for (let planetName in exoplanetsDict) 
    {
        console.log(`Status : loading ${planetName}`);
        const planetUrl = exoplanetsDict[planetName];
        const browser = await puppeteer.launch(
        {
            headless: true,
            args: ['--use-gl=desktop', '--enable-webgl', '--window-size=1280,800',
            '--start-fullscreen',  // Lance le navigateur en plein écran
            '--high-dpi-support=1',  // Active le support haute DPI
            '--device-scale-factor=2'
            ]
        });
        const page = await browser.newPage();
        await page.setViewport({ width: 1280, height: 800, deviceScaleFactor: 1 });
        
        
        
        try 
        {
            await page.setUserAgent(agents[i % agents.length]);
            await page.goto(planetUrl, { waitUntil: 'domcontentloaded', timeout: 60000 });

            //récupération catégorie
            // await page.waitForSelector('tbody');
            // const planetType = await page.evaluate(() => 
            // {
            //     const row = [...document.querySelectorAll('tr.fact_row')].find(row => row.querySelector('.title').textContent.includes('PLANET TYPE'));
            //     return row ? row.querySelector('.value').textContent.trim() : 'Unknown';
            // });

            await page.waitForSelector('.smd-acf-grid-layout');
            const planetType = await page.evaluate(() => 
            {
                const row = [...document.querySelectorAll('.smd-acf-grid-col')].find(row => row.querySelector('.text-bold').textContent.includes('Planet Type'));
                return row ? row.querySelector('ul li span').textContent.trim() : 'Unknown';
            });

            //gestion dossier catégorie
            const typeDir = path.join('../data/dataset', planetType.replace(/ /g, '_').replace('/', '_').toLowerCase());
            fs.mkdirSync(typeDir, { recursive: true });


            //gestion iframe
            const frameHandle = await page.$('iframe');
            const frame = await frameHandle.contentFrame();

            //gestion shadow pour 1920x1080
            // await frame.waitForSelector('.loading', { hidden: true });
            // const footerDivs = await frame.$$('.footerSVGDiv');
            // if (footerDivs.length >= 2) {
            //     await footerDivs[0].click();
            // }
      
            
            //reset position
            await frame.waitForSelector('.headerButton');
            await frame.click('.headerButton');
            await frame.evaluate(() => {
                const menuItems = Array.from(document.querySelectorAll('nav.siteNav ul > li'));
                const settingsItem = menuItems.find(item => item.textContent.includes('Settings'));
                if (settingsItem) {
                    settingsItem.click();
                }
            });
            await frame.evaluate(() => {
                const settingsEntries = Array.from(document.querySelectorAll('.settingsEntry'));
                if (settingsEntries.length >= 4) { // Assurez-vous qu'il y a au moins quatre entrées
                    const floodLightingSwitch = settingsEntries[3].querySelector('.switch input[type="checkbox"]'); // Sélectionnez le commutateur dans le quatrième élément
                    if (floodLightingSwitch) {
                        floodLightingSwitch.click(); // Cliquez sur le commutateur
                    }
                }
            });
            await frame.click('.settings .headerBar .headerRight .closeButton'); 
            await frame.click('.mainMenu .headerBar .headerRight .headerButton');
            await frame.waitForSelector('#compareBtnObj');
            await frame.click('#compareBtnObj');
            await frame.waitForSelector('.menuOptions', { visible: true });
            await frame.click('.menuItem.planet');

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
