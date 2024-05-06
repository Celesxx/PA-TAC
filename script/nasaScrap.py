from selenium import webdriver
from selenium.webdriver.chrome.options import Options
from selenium.webdriver.common.by import By
from selenium.webdriver.support.ui import WebDriverWait
from selenium.webdriver.support import expected_conditions as EC
import json
import time

options = Options()
options.headless = True  # N'affiche pas la fenêtre
options.add_argument("--window-size=1920,1080") 
driver = webdriver.Chrome(executable_path="/usr/local/bin/chromedriver", options=options)
driver.get("https://exoplanets.nasa.gov/discovery/exoplanet-catalog/")  # Url du site
print("status : loading page")
driver.implicitly_wait(10)
print("status : page loaded")

exoplanets_dict = {}
page_number = 1

try:
    while True:
        print(f"status : scrapping page : {page_number}")
        exoplanets_data = driver.find_elements(By.CSS_SELECTOR, "div#results > ul.exoplanet")
        for planet in exoplanets_data:
            name_element = planet.find_element(By.CSS_SELECTOR, "li.display_name > a")
            planet_name = name_element.text
            planet_url = name_element.get_attribute('href')
            exoplanets_dict[planet_name] = planet_url

        next_button = driver.find_element(By.CSS_SELECTOR, "span.next > a")
        if "disabled" in next_button.get_attribute("class") or page_number == 226:
            break
        else:
            next_button.click()
            time.sleep(2)
            WebDriverWait(driver, 10).until(EC.visibility_of_element_located((By.CSS_SELECTOR, "div#results > ul.exoplanet")))
            page_number += 1 

except Exception as e:
    print(f"An error occurred: {e}")
finally:
    driver.quit()
    print("status : saving data")
    with open("exoplanets_data.json", "w") as file:
        json.dump(exoplanets_dict, file, indent=4)
    print("Les données ont été sauvegardées dans 'exoplanets_data.json'")
