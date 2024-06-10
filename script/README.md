# Projet d'Exoplanètes - README

Ce projet a pour objectif de capturer des images de diverses exoplanètes sur le site de la nasa `https://exoplanets.nasa.gov/discovery/exoplanet-catalog/`en utilisant des scripts Python et JavaScript.

## Installation

### Environnement Python

Pour configurer l'environnement Python, suivez ces étapes :

- Installer Anaconda
- Créer un nouvel envrionnement à partir du 
```bash
conda create --name chooseEnvName --file requirements.txt
```
- Activez l'environnement
```bash
conda activate chooseEnvName
```

- Lancer le script nasaPlaneteScrap.py

### Environnement JavaScript

Pour configurer l'environnement JavaScript, suivez ces étapes :

- Installer node.js
- Installer les dépendances 

```bash
npm install
```

- Lancer le script nasaPlaneteScrap2.js

## Structure du Projet
La structure de ce projet est la suivante :

```graphql
PA-TAC/script/
│
├── requirements.txt      # Dépendances Python
├── package.json          # Dépendances JavaScript
├── nasaPlaneteScrap.py             # Script Python
├── nasaPlaneteScrap.js             # Ancien script JavaScript
└── nasaPlaneteScrap2.js             # Script JavaScript
```


## Resize script

```bash
python resize.py crop 580 580 ../data/dataset/resized/gas_giant ../data/dataset/resized/gas_giant
python resize.py resize 200 200 ../data/dataset/resized/gas_giant ../data/dataset/resized/gas_giant
```