import os
import json
import time
from selenium import webdriver
from selenium.webdriver.chrome.options import Options
from selenium.webdriver.common.by import By
from selenium.webdriver.support.ui import WebDriverWait
from selenium.webdriver.support import expected_conditions as EC

options = Options()
options.add_argument("--use-gl=desktop")  # Force l'utilisation de l'accélération matérielle OpenGL
options.add_argument("--enable-webgl")  # Active WebGL
options.headless = False  # N'affiche pas la fenêtre
options.add_argument("--window-size=1920,1080")

# options.add_argument('--disable-gpu')  # Optionnel pour certaines versions de Chrome
# options.add_argument('--disable-extensions')
# options.add_argument('--no-sandbox')
# options.add_argument('--disable-dev-shm-usage')
# options.add_argument('--disable-images') 

driver = webdriver.Chrome(executable_path="/usr/local/bin/chromedriver", options=options)

with open("exoplanets_data.json", "r") as file:
    exoplanets_dict = json.load(file)

images_dir = "img"
os.makedirs(images_dir, exist_ok=True)

for planet_name, planet_url in exoplanets_dict.items():

    try:
        print("---------------------------------------------------")
        print("Status : loading url")
        driver.get(planet_url)
        print("Status : url loaded")
        print("Status : searching iframe")
        iframe = driver.find_element(By.TAG_NAME, "iframe")
        driver.switch_to.frame(iframe)
        print("Status : iframe found")
        print("Status : loading canvas")
        driver.implicitly_wait(2)
        WebDriverWait(driver, 60).until( EC.invisibility_of_element_located((By.CSS_SELECTOR, ".loading")) )
        driver.implicitly_wait(5)
        print("Status : canvas loaded")
        print("Status : remove label and shadow")
        # driver.switch_to.default_content()
        footer_divs = driver.find_elements(By.CSS_SELECTOR, ".footerSVGDiv")
        # footer_divs = WebDriverWait(driver, 20).until(EC.visibility_of_all_elements_located((By.CSS_SELECTOR, ".footerBarRight .footerSVGDiv")) )
        # footer_divs = WebDriverWait(driver, 20).until(EC.visibility_of_all_elements_located((By.CSS_SELECTOR, ".footerBarRight .footerSVGDiv") ))
        if len(footer_divs) >= 2:
            footer_divs[0].click()
            # footer_divs[1].click()
            # switch = footer_divs[1].find_element(By.CSS_SELECTOR, ".switch:first-child")
            # switch.click()

        # driver.switch_to.frame(iframe)
        print("Status : remove ui class")
        driver.execute_script("""
        var uiElements = document.querySelectorAll('.ui');
        uiElements.forEach(function(el) {
            el.style.display = 'none';  
        });
        """)
        print("Status : class removed")

        print("Status : searching canvas")
        filename = os.path.join(images_dir, f"{planet_name.replace('/', '_').replace(' ', '-')}.png")
        canvas = driver.find_element(By.CSS_SELECTOR, "canvas")
        print("Status : canvas find")

        print("Status : screenshot initialize")
        canvas.screenshot(filename)
        driver.implicitly_wait(1)
        print(f"Status : screenshot saved for {planet_name} at {filename}")

    except Exception as e:
        print("Status : error")
        print(e)


driver.quit()
print("All captures are saved.")
