---
marp: true
theme: uncover
paginate: true
_class: invert
style: |
    
---
<!-- _paginate: false-->
<!-- _footer: oct.25 2024-->

<!--
Le tournoi de oort.rs s'approche. Le mode du tournoi sera le suivant: 4
groupes de 3 fois 3 équipes et un groupe de 4 en mode round-robin. Les
deux premières équipes de chaque groupe passent dans le matchs à
élimination directe: quart, demi et finale (les deux perdants de la demi
seront 3e ex-aequo). Le score des combats et le ratio victoires /
défaites sur 10 combats aléatoires (comme sur le site de oort.rs) en cas
d'égalité, on recommence jusqu'à avoir une équipe victorieuse.

Une fois le tournoi terminé, vous devrez effectuer vos présentations
orales d'une durée de 20min. L'objectif de votre présentation est
d'expliquer le fonctionnement de vos vaisseaux, (p.ex. quels sont les
moyens de déplacement, les armes disponibles, les radars, la radio, etc.
et comment tout cela fonctionne en pratique: comment on vise, etc),
d'expliquer aussi comment fonctionne votre IA: quels mécanismes
d'attaque, de détection, de communication, et de défense vous utilisez,
par exemple. Il n'est pas nécessaire de présenter le code en tant que
tel, sauf si vous jugez que c'est mortellement important. Hésitez pas à
répéter la présentation chez vous.

N'oubliez pas de mettre les liens vers vos git sur cyberlearn, et les
slides au format PDF. Dans votre git, mettez le code qu'on puisse le
copier-coller directement dans oort.rs.
-->

# :artificial_satellite: **OORT.RS** :rocket:
## Squadron Bêta
Marty Hugo & Rouiller Cyril

---
# Introduction

- [oort.rs](https://oort.rs) est un jeu de programmation qui utilise le language Rust afin de contrôler une flotte de vaisseaux dans différents défis et duels.

---
# Introduction

- Le tournois se déroulant sur le duel de fighter.

- Nous avons donc utilisé son programme par défaut comme base, puis nous l'avons amélioré jusqu'à l'amener dans le leaderboard.


---
# Sommaire
- Introduction
- Équipements
- R&D
- Comportements
    - Modes / États
    - Mouvements
- Conclusion

---
# Équipements
- Notre vaisseau dispose de cinq éléments:
    - des réacteurs
    - un radar
    - une radio
    - une mitrailleuse
    - des missiles

---
### Équipements : *Réacteurs*
Les réacteurs permettent au vaisseau de se déplacer dans l'espace ainsi que d'effectuer des rotations sur lui même.
![bg right:40% 80%](Reacteurs.png)

---
### Équipements : *Radar*
Le radar est un outil qui donne la possibilité de scanner dans une direction donnée. Il renvoie ensuite les coordonées et la vélocité du contact ennemi.
![bg right:40% 80%](Radar.png)

---
### Équipements : *Radio*
La radio est un élément invisible qui nous permet d'envoyer et de recevoir des messages sur un canal choisi.
Les canaux sont communs au deux équipes.

---
### Équipements : *Mitrailleuse*
La mitrailleuse permet au vaisseau de tirer des balles qui sont des projectiles, elles ne sont plus controlables une fois tirées.
![bg right:40% 80%](Mitrailleuse.png)

---
### Équipements : *Missiles*
Les missiles agissent comme des mini vaisseaux, ils sont dirigeables et comportent la radio ainsi que le radar.
![bg right:40% 80%](Missile.png)

---
# R&D
Á partir du code de base, nous avons amélioré les éléments suivants:
    - Déplacements
    - Radar
    - Radio
    - Missiles

---
### R&D : *Radar*
- Le radar a été amélioré afin de se redimentionner de manière à être le plus large tout en gardant une distance de sécutité derrière l'ennemi
![bg right:40% 80%](Radar_up.png)

---
### R&D : *Radio*
- La radio quand à elle change desormais de canal à chaque tick afin de ne pas se faire écraser par un potentiel ennemi qui utiliserai le même.

---
### R&D : *Missiles*
- Les missiles font des calculs pour exploser cent millisecondes avant de toucher l'ennemi.
- En faisant cela les débris augmentent la chances de toucher l'ennemi en lui infligeant tout autant de dégats.
![bg right:40% 80%](Shrapnell.png)

---
# Comportement
Le vaisseau comporte trois états.

![w:20cm](Machine_etat.svg)

---
### Attaque
- En mode **attaque**, le vaisseau fait rotationner son radar afin de balayer l'horizon et se dirige vers la dernière position connue de l'ennemi.
- Si il y trouve un ennemi de type *"fighter"* il lui lance un missile et communique sa position par radio à tous les missiles.
- Tous les 30 ticks le vaisseau passe en mode **recherche**

---
### Recherche
- Le vaisseau va mettre son radar le plus large possible (un quart) et va faire un tour de scan en 4 ticks
- Si un missile est détecté, le vaisseau passe en mode **défense**
- Sinon il retourne en mode **attaque**

---
### Défense
- En mode **défense**, le vaisseau va cibler le missile trouvé en recherche et essayer de le détruire
- Si le missile est détruit, perdu de vue ou nous passe à coté *(plus de 90° de différense avec l'ennemi)* on retourne en **recherche**

---
### Mouvements
- Le vaisseau suit trois comportements :
    - en **recherche** et **défense** on se laisse porter
    - en **attaque** on utilise la fonction ***unpredictible_trajectory***

---
### *Unpredictible trajectory*
- Cette fonction est basée sur *"seek"* à la différence que nous utilisont comme point où se diriger notre position actuelle additionnée à :
- (en x) le cosinus de la distance entre nous et l'ennemi
- (en y) le sinus de la distance à laquelle nous souhaitons passer
![bg right:35% 90%](trigo.png)

---
# Conclusion
- Codage de vaisseau interessant
- Meilleures compréhension de la trigonométrie
- Quand-même trop de maths
- Classement pas mauvais :wink:

---
# Questions ?